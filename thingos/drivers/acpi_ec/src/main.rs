//! # acpi_ec — ACPI Embedded Controller core driver
//!
//! Owns the EC I/O ports (0x62 / 0x66), implements the IBF/OBF handshake
//! protocol, subscribes to the SCI interrupt for query-event logging, and
//! mounts a composable VFS service at `/services/ec/`.
//!
//! ## VFS layout
//!
//! ```text
//! /services/ec/
//!   status   ← 1-byte EC status register (port 0x66), read-only
//!   data     ← 1-byte raw output-buffer byte (port 0x62), read-only
//!   query    ← 1-byte CMD_QUERY result; each read issues one CMD_QUERY
//!   read     ← write 1-byte register address, then read 1-byte result
//!   write    ← write 2-byte (reg, val) to perform CMD_WRITE
//!   events   ← stream of pending query-event codes (EAGAIN when empty)
//! ```
//!
//! ## Clients
//!
//! * `ec_kbd` — reads `status` + `data` to decode keyboard scancodes.
//! * `acpi_battery` (future) — uses `read`/`write` for BST/BIF registers.
//! * `acpi_power` (future) — reads `events` stream for lid/button events.
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
extern crate alloc;

use abi::errors::Errno;
use abi::vfs_rpc::VfsRpcOp;
use ipc_helpers::provider::{ProviderLoop, ProviderResponse};
use stem::syscall::vfs::vfs_mount;
use stem::syscall::{ioport_read, ioport_write, irq_subscribe};
use stem::{debug, info, trace, warn};

// ── EC hardware ───────────────────────────────────────────────────────────────

/// EC data port (read keyboard / query bytes from here).
const DATA_PORT: usize = 0x62;
/// EC command/status port.
const COMMAND_PORT: usize = 0x66;

/// Output-Buffer Full — byte ready in DATA_PORT.
const STATUS_OBF: u8 = 1 << 0;
/// Input-Buffer Full — EC still processing last command; wait before writing.
const STATUS_IBF: u8 = 1 << 1;
/// SCI Event pending — issue CMD_QUERY to retrieve the event code.
const STATUS_SCI_EVT: u8 = 1 << 5;

/// Read an EC register (two-byte protocol: CMD_READ + reg → reply in OBF).
const CMD_READ: u8 = 0x80;
/// Write an EC register (three-byte: CMD_WRITE + reg + val).
const CMD_WRITE: u8 = 0x81;
/// Query pending event code (one-byte reply in OBF).
const CMD_QUERY: u8 = 0x84;

/// Maximum IBF/OBF busy-wait iterations before declaring a timeout.
const WAIT_SPINS: usize = 1024;

/// SCI IRQ vector (ACPI SCI is typically GSI 9, vector 0x29 in Thing-OS).
const SCI_VECTOR: u8 = 0x29;

// ── VFS layout ────────────────────────────────────────────────────────────────

const MOUNT_PATH: &str = "/services/ec";
const PORT_CAPACITY: usize = 4096;

const HANDLE_ROOT: u64   = 1;
const HANDLE_STATUS: u64 = 2;
const HANDLE_DATA: u64   = 3;
const HANDLE_QUERY: u64  = 4;
const HANDLE_READ: u64   = 5;
const HANDLE_WRITE: u64  = 6;
const HANDLE_EVENTS: u64 = 7;

const INO_ROOT: u64   = 0xec00_0001;
const INO_STATUS: u64 = 0xec00_0002;
const INO_DATA: u64   = 0xec00_0003;
const INO_QUERY: u64  = 0xec00_0004;
const INO_READ: u64   = 0xec00_0005;
const INO_WRITE: u64  = 0xec00_0006;
const INO_EVENTS: u64 = 0xec00_0007;

/// Capacity of the in-process EC query-event ring buffer.
const EVENTS_CAP: usize = 32;

/// Background-loop polling interval (milliseconds).
const POLL_MS: u64 = 5;

// ── Driver manifest & glue ────────────────────────────────────────────────────

use abi::driver_interface::{
    DRIVER_DESCRIPTOR_ABI_VERSION, DeviceInfo, DriverClass, DriverDescriptor, DriverEntryCtx,
    ProbeResult, Status,
};
use stem::abi::module_manifest::{MANIFEST_MAGIC, ManifestHeader, ModuleKind, device_kind_bytes};

const THINGOS_DRIVER_NAME: &[u8] = b"acpi_ec";
const KIND_DRV_EC: &str = "drv.AcpiEc";

#[unsafe(no_mangle)]
#[used]
pub static THINGOS_DRIVER: DriverDescriptor = DriverDescriptor {
    abi_version: DRIVER_DESCRIPTOR_ABI_VERSION,
    driver_name_ptr: THINGOS_DRIVER_NAME.as_ptr(),
    driver_name_len: THINGOS_DRIVER_NAME.len(),
    driver_class: DriverClass::Other,
    flags: 0,
    probe: thingos_driver_probe,
    #[cfg(target_arch = "x86_64")]
    start: thingos_driver_start_safe as unsafe extern "C" fn(*const DriverEntryCtx) -> Status,
    #[cfg(not(target_arch = "x86_64"))]
    start: thingos_driver_start,
};

#[cfg(target_arch = "x86_64")]
core::arch::global_asm!(
    r#"
    .section .text
    .global thingos_driver_start_safe
    thingos_driver_start_safe:
        sub rsp, 8
        push rdi
        call thingos_runtime_setup
        pop rdi
        add rsp, 8
        call thingos_driver_start_rust
        ret
"#
);

#[cfg(target_arch = "x86_64")]
unsafe extern "C" {
    fn thingos_driver_start_safe(ctx: *const DriverEntryCtx) -> Status;
}

#[unsafe(no_mangle)]
unsafe extern "C" fn thingos_driver_start_rust(ctx: *const DriverEntryCtx) -> Status {
    thingos_driver_start(ctx)
}

unsafe extern "C" fn thingos_driver_probe(
    _dev: *const DeviceInfo,
    out: *mut ProbeResult,
) -> Status {
    if out.is_null() { return Status::InvalidArgument; }
    let out = unsafe { &mut *out };
    out.matched = 0;
    out.score = 0;
    out.claimed_class = DriverClass::Other;
    out.flags = 0;
    Status::NoMatch
}

unsafe extern "C" fn thingos_driver_start(_ctx: *const DriverEntryCtx) -> Status {
    main(0)
}

#[unsafe(link_section = ".thing_manifest")]
#[unsafe(no_mangle)]
#[used]
pub static MANIFEST: ManifestHeader = ManifestHeader {
    magic: MANIFEST_MAGIC,
    kind: ModuleKind::Driver,
    device_kind: device_kind_bytes(KIND_DRV_EC.as_bytes()),
    version: 1,
    _reserved: 0,
};

// ── EC hardware primitives ────────────────────────────────────────────────────

fn ec_read_status() -> u8 {
    ioport_read(COMMAND_PORT, 1) as u8
}

fn ec_read_data() -> u8 {
    ioport_read(DATA_PORT, 1) as u8
}

/// Spin until IBF (input-buffer full) clears.  Returns `false` on timeout.
fn ec_wait_ibf_clear() -> bool {
    for _ in 0..WAIT_SPINS {
        let s = ec_read_status();
        if s == 0xff { return false; } // EC absent
        if s & STATUS_IBF == 0 { return true; }
        core::hint::spin_loop();
    }
    false
}

/// Spin until OBF (output-buffer full) is set.  Returns `false` on timeout.
fn ec_wait_obf_set() -> bool {
    for _ in 0..WAIT_SPINS {
        let s = ec_read_status();
        if s == 0xff { return false; }
        if s & STATUS_OBF != 0 { return true; }
        core::hint::spin_loop();
    }
    false
}

/// Issue CMD_QUERY and return the one-byte event code, or `None` on timeout.
fn ec_do_query() -> Option<u8> {
    if !ec_wait_ibf_clear() { return None; }
    ioport_write(COMMAND_PORT, CMD_QUERY as usize, 1);
    if !ec_wait_obf_set() { return None; }
    Some(ec_read_data())
}

/// Read EC register `reg` via the CMD_READ protocol.
fn ec_read_register(reg: u8) -> Option<u8> {
    if !ec_wait_ibf_clear() { return None; }
    ioport_write(COMMAND_PORT, CMD_READ as usize, 1);
    if !ec_wait_ibf_clear() { return None; }
    ioport_write(DATA_PORT, reg as usize, 1);
    if !ec_wait_obf_set() { return None; }
    Some(ec_read_data())
}

/// Write `val` to EC register `reg` via the CMD_WRITE protocol.
fn ec_write_register(reg: u8, val: u8) -> bool {
    if !ec_wait_ibf_clear() { return false; }
    ioport_write(COMMAND_PORT, CMD_WRITE as usize, 1);
    if !ec_wait_ibf_clear() { return false; }
    ioport_write(DATA_PORT, reg as usize, 1);
    if !ec_wait_ibf_clear() { return false; }
    ioport_write(DATA_PORT, val as usize, 1);
    true
}

// ── Driver state ──────────────────────────────────────────────────────────────

struct EcState {
    /// Ring buffer of pending query-event codes.
    events: [u8; EVENTS_CAP],
    ev_head: usize,
    ev_tail: usize,
    /// Result of the last CMD_READ operation (for the `read` file protocol).
    read_result: u8,
}

impl EcState {
    const fn new() -> Self {
        Self { events: [0u8; EVENTS_CAP], ev_head: 0, ev_tail: 0, read_result: 0 }
    }

    fn enqueue_event(&mut self, code: u8) {
        let next = (self.ev_tail + 1) % EVENTS_CAP;
        if next == self.ev_head {
            warn!("EC event buffer full, dropping code=0x{:02x}", code);
            return;
        }
        self.events[self.ev_tail] = code;
        self.ev_tail = next;
    }

    fn dequeue_event(&mut self) -> Option<u8> {
        if self.ev_head == self.ev_tail { return None; }
        let code = self.events[self.ev_head];
        self.ev_head = (self.ev_head + 1) % EVENTS_CAP;
        Some(code)
    }
}

// ── VFS dispatch ──────────────────────────────────────────────────────────────

fn parse_u64_le(buf: &[u8]) -> Option<u64> {
    if buf.len() < 8 { return None; }
    Some(u64::from_le_bytes(buf[..8].try_into().ok()?))
}

fn handle_lookup(payload: &[u8]) -> ProviderResponse {
    if payload.len() < 4 { return ProviderResponse::err(Errno::EINVAL); }
    let path_len = u32::from_le_bytes(payload[..4].try_into().unwrap_or([0; 4])) as usize;
    if payload.len() < 4 + path_len { return ProviderResponse::err(Errno::EINVAL); }
    let path = match core::str::from_utf8(&payload[4..4 + path_len]) {
        Ok(s) => s,
        Err(_) => return ProviderResponse::err(Errno::EINVAL),
    };
    let path = path.trim_start_matches('/');
    match path {
        ""       => ProviderResponse::ok_u64(HANDLE_ROOT),
        "status" => ProviderResponse::ok_u64(HANDLE_STATUS),
        "data"   => ProviderResponse::ok_u64(HANDLE_DATA),
        "query"  => ProviderResponse::ok_u64(HANDLE_QUERY),
        "read"   => ProviderResponse::ok_u64(HANDLE_READ),
        "write"  => ProviderResponse::ok_u64(HANDLE_WRITE),
        "events" => ProviderResponse::ok_u64(HANDLE_EVENTS),
        _        => ProviderResponse::err(Errno::ENOENT),
    }
}

fn handle_stat(payload: &[u8]) -> ProviderResponse {
    let handle = match parse_u64_le(payload) {
        Some(h) => h,
        None => return ProviderResponse::err(Errno::EINVAL),
    };
    const S_IFREG: u32 = 0o100000;
    const S_IFDIR: u32 = 0o040000;
    match handle {
        HANDLE_ROOT   => ProviderResponse::ok_stat(S_IFDIR | 0o555, 0, INO_ROOT),
        HANDLE_STATUS => ProviderResponse::ok_stat(S_IFREG | 0o444, 1, INO_STATUS),
        HANDLE_DATA   => ProviderResponse::ok_stat(S_IFREG | 0o444, 1, INO_DATA),
        HANDLE_QUERY  => ProviderResponse::ok_stat(S_IFREG | 0o444, 1, INO_QUERY),
        HANDLE_READ   => ProviderResponse::ok_stat(S_IFREG | 0o644, 1, INO_READ),
        HANDLE_WRITE  => ProviderResponse::ok_stat(S_IFREG | 0o200, 0, INO_WRITE),
        HANDLE_EVENTS => ProviderResponse::ok_stat(S_IFREG | 0o444, 0, INO_EVENTS),
        _             => ProviderResponse::err(Errno::EBADF),
    }
}

fn handle_read(state: &mut EcState, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 20 { return ProviderResponse::err(Errno::EINVAL); }
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));

    match handle {
        HANDLE_STATUS => {
            let s = ec_read_status();
            trace!("EC status=0x{:02x}", s);
            ProviderResponse::ok_read(&[s])
        }
        HANDLE_DATA => {
            let b = ec_read_data();
            trace!("EC data=0x{:02x}", b);
            ProviderResponse::ok_read(&[b])
        }
        HANDLE_QUERY => {
            match ec_do_query() {
                Some(code) => {
                    debug!("EC query result=0x{:02x}", code);
                    ProviderResponse::ok_read(&[code])
                }
                None => ProviderResponse::err(Errno::ETIMEDOUT),
            }
        }
        HANDLE_READ => {
            // Return the result cached by the last Write to this handle.
            ProviderResponse::ok_read(&[state.read_result])
        }
        HANDLE_EVENTS => {
            match state.dequeue_event() {
                Some(code) => ProviderResponse::ok_read(&[code]),
                None       => ProviderResponse::err(Errno::EAGAIN),
            }
        }
        HANDLE_ROOT => ProviderResponse::err(Errno::EISDIR),
        _           => ProviderResponse::err(Errno::EBADF),
    }
}

fn handle_write(state: &mut EcState, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 20 { return ProviderResponse::err(Errno::EINVAL); }
    let handle   = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
    let data_len = u32::from_le_bytes(payload[16..20].try_into().unwrap_or([0; 4])) as usize;
    if payload.len() < 20 + data_len { return ProviderResponse::err(Errno::EINVAL); }
    let data = &payload[20..20 + data_len];

    match handle {
        HANDLE_READ => {
            // Write protocol: client writes register address, then reads result.
            if data.is_empty() { return ProviderResponse::err(Errno::EINVAL); }
            let reg = data[0];
            match ec_read_register(reg) {
                Some(val) => {
                    trace!("EC read reg=0x{:02x} val=0x{:02x}", reg, val);
                    state.read_result = val;
                    ProviderResponse::ok_written(data_len as u32)
                }
                None => ProviderResponse::err(Errno::ETIMEDOUT),
            }
        }
        HANDLE_WRITE => {
            if data.len() < 2 { return ProviderResponse::err(Errno::EINVAL); }
            let (reg, val) = (data[0], data[1]);
            if ec_write_register(reg, val) {
                trace!("EC write reg=0x{:02x} val=0x{:02x}", reg, val);
                ProviderResponse::ok_written(data_len as u32)
            } else {
                ProviderResponse::err(Errno::ETIMEDOUT)
            }
        }
        _ => ProviderResponse::err(Errno::ENOSYS),
    }
}

fn handle_readdir(payload: &[u8]) -> ProviderResponse {
    if payload.len() < 20 { return ProviderResponse::err(Errno::EINVAL); }
    let handle  = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
    let offset  = u64::from_le_bytes(payload[8..16].try_into().unwrap_or([0; 8])) as usize;
    let max_len = u32::from_le_bytes(payload[16..20].try_into().unwrap_or([0; 4])) as usize;

    if handle != HANDLE_ROOT { return ProviderResponse::err(Errno::ENOTDIR); }

    const DT_REG: u8 = 8;
    let entries: &[(&str, u64)] = &[
        ("status", INO_STATUS),
        ("data",   INO_DATA),
        ("query",  INO_QUERY),
        ("read",   INO_READ),
        ("write",  INO_WRITE),
        ("events", INO_EVENTS),
    ];

    let mut out: alloc::vec::Vec<u8> = alloc::vec::Vec::new();
    for (i, &(name, ino)) in entries.iter().enumerate() {
        if i < offset { continue; }
        let entry_len = 8 + 1 + 1 + name.len();
        if out.len() + entry_len > max_len { break; }
        out.extend_from_slice(&ino.to_le_bytes());
        out.push(DT_REG);
        out.push(name.len() as u8);
        out.extend_from_slice(name.as_bytes());
    }
    ProviderResponse::ok_read(&out)
}

fn dispatch(state: &mut EcState, op: VfsRpcOp, payload: &[u8]) -> ProviderResponse {
    match op {
        VfsRpcOp::Lookup  => handle_lookup(payload),
        VfsRpcOp::Stat    => handle_stat(payload),
        VfsRpcOp::Read    => handle_read(state, payload),
        VfsRpcOp::Write   => handle_write(state, payload),
        VfsRpcOp::Readdir => handle_readdir(payload),
        VfsRpcOp::Close   => ProviderResponse::ok_empty(),
        _                 => ProviderResponse::err(Errno::ENOSYS),
    }
}

// ── Main loop ─────────────────────────────────────────────────────────────────

fn run_loop(req_read: u32) -> ! {
    let mut lp    = ProviderLoop::new(req_read);
    let mut state = EcState::new();

    loop {
        // 1. Drain all pending VFS requests (non-blocking).
        loop {
            match lp.try_next_request() {
                Ok(Some(req)) => {
                    let resp = dispatch(&mut state, req.op, &req.payload);
                    if let Err(e) = lp.send_response(&req, resp) {
                        warn!("send_response failed: {:?}", e);
                    }
                }
                Ok(None)           => break,
                Err(Errno::EPIPE)  => stem::syscall::exit(0),
                Err(_)             => break,
            }
        }

        // 2. Poll EC for pending query events.
        //    Only issue CMD_QUERY when SCI_EVT is set AND OBF is clear,
        //    so we never clobber keyboard data waiting in the output buffer.
        let status = ec_read_status();
        if status != 0xff && (status & STATUS_SCI_EVT != 0) && (status & STATUS_OBF == 0) {
            if let Some(code) = ec_do_query() {
                debug!("EC query event=0x{:02x}", code);
                state.enqueue_event(code);
            }
        }

        stem::time::sleep_ms(POLL_MS);
    }
}

// ── Entry point ───────────────────────────────────────────────────────────────

#[stem::main]
fn main(_raw_arg: usize) -> ! {
    info!("ACPI EC service online");

    // Log whether EC hardware is reachable.
    let present = ec_read_status() != 0xff;
    if present {
        debug!("EC hardware detected at ports 0x{:02x}/0x{:02x}", DATA_PORT, COMMAND_PORT);
    } else {
        debug!("No EC hardware at ports 0x{:02x}/0x{:02x}; service idle", DATA_PORT, COMMAND_PORT);
    }

    // Subscribe to SCI so we wake promptly on EC events.
    match irq_subscribe(SCI_VECTOR) {
        Ok(()) => debug!("Subscribed to SCI vector 0x{:02x}", SCI_VECTOR),
        Err(_) => debug!("SCI IRQ unavailable; relying on polling"),
    }

    let (req_write, req_read) = match stem::syscall::port::port_create(PORT_CAPACITY) {
        Ok(p) => p,
        Err(e) => {
            warn!("port_create failed: {:?}", e);
            stem::syscall::exit(1);
        }
    };

    let _ = stem::syscall::vfs::vfs_mkdir("/services");
    if let Err(e) = vfs_mount(req_write, MOUNT_PATH) {
        warn!("Mount at {} failed: {:?}", MOUNT_PATH, e);
    } else {
        debug!("Mounted at {}", MOUNT_PATH);
    }

    run_loop(req_read)
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_root_returns_handle_1() {
        let payload = b"\x00\x00\x00\x00";
        let resp = handle_lookup(payload);
        assert_eq!(resp.status, 0);
        assert_eq!(u64::from_le_bytes(resp.payload[..8].try_into().unwrap()), HANDLE_ROOT);
    }

    #[test]
    fn lookup_known_files() {
        for (name, expected_handle) in &[
            ("status", HANDLE_STATUS),
            ("data",   HANDLE_DATA),
            ("query",  HANDLE_QUERY),
            ("read",   HANDLE_READ),
            ("write",  HANDLE_WRITE),
            ("events", HANDLE_EVENTS),
        ] {
            let mut payload = (name.len() as u32).to_le_bytes().to_vec();
            payload.extend_from_slice(name.as_bytes());
            let resp = handle_lookup(&payload);
            assert_eq!(resp.status, 0, "status lookup failed for {}", name);
            let h = u64::from_le_bytes(resp.payload[..8].try_into().unwrap());
            assert_eq!(h, *expected_handle, "wrong handle for {}", name);
        }
    }

    #[test]
    fn lookup_unknown_returns_enoent() {
        let path = b"bogus";
        let mut payload = (path.len() as u32).to_le_bytes().to_vec();
        payload.extend_from_slice(path);
        let resp = handle_lookup(&payload);
        assert_eq!(resp.status, Errno::ENOENT as u8);
    }

    #[test]
    fn events_returns_eagain_when_empty() {
        let mut state = EcState::new();
        let mut payload = HANDLE_EVENTS.to_le_bytes().to_vec();
        payload.extend_from_slice(&0u64.to_le_bytes()); // offset
        payload.extend_from_slice(&64u32.to_le_bytes()); // len
        let resp = handle_read(&mut state, &payload);
        assert_eq!(resp.status, Errno::EAGAIN as u8);
    }

    #[test]
    fn events_returns_queued_code() {
        let mut state = EcState::new();
        state.enqueue_event(0x71);
        let mut payload = HANDLE_EVENTS.to_le_bytes().to_vec();
        payload.extend_from_slice(&0u64.to_le_bytes());
        payload.extend_from_slice(&64u32.to_le_bytes());
        let resp = handle_read(&mut state, &payload);
        assert_eq!(resp.status, 0);
        let n = u32::from_le_bytes(resp.payload[..4].try_into().unwrap());
        assert_eq!(n, 1);
        assert_eq!(resp.payload[4], 0x71);
        // Second read drains the buffer.
        let resp2 = handle_read(&mut state, &payload);
        assert_eq!(resp2.status, Errno::EAGAIN as u8);
    }

    #[test]
    fn event_ring_buffer_wraps_without_panic() {
        let mut state = EcState::new();
        for i in 0..EVENTS_CAP + 8 {
            state.enqueue_event(i as u8);
        }
        // Drain all; should not panic or loop forever.
        let mut count = 0usize;
        while state.dequeue_event().is_some() {
            count += 1;
            assert!(count <= EVENTS_CAP, "dequeue ran past capacity");
        }
    }

    #[test]
    fn readdir_root_lists_all_files() {
        let mut payload = HANDLE_ROOT.to_le_bytes().to_vec();
        payload.extend_from_slice(&0u64.to_le_bytes()); // offset
        payload.extend_from_slice(&512u32.to_le_bytes()); // max_len
        let resp = handle_readdir(&payload);
        assert_eq!(resp.status, 0);
        let raw = &resp.payload[4..]; // skip bytes_read u32
        for name in &["status", "data", "query", "read", "write", "events"] {
            assert!(
                raw.windows(name.len()).any(|w| w == name.as_bytes()),
                "missing '{}' in readdir output",
                name
            );
        }
    }
}

