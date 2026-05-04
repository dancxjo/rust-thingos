//! # acpi_power — ACPI power and button handling service
//!
//! Handles power button and sleep button events via the PM1 fixed event
//! registers (parsed from the FADT), lid open/close state via the EC query
//! event stream, and exposes normalised events at `/run/power/`.
//!
//! ## VFS layout
//!
//! ```text
//! /run/power/
//!   events   ← non-blocking: read 1 byte per event, EAGAIN when empty
//!   lid      ← "open\n" or "closed\n"
//!   action   ← writable: "reboot\n" or "shutdown\n"
//! ```
//!
//! ## Event byte codes
//!
//! * `0x01` — Power button pressed
//! * `0x02` — Sleep button pressed
//! * `0x03` — Lid closed
//! * `0x04` — Lid opened
//!
//! ## Graceful degradation
//!
//! When the FADT is absent (non-ACPI hardware or early boot without acpid),
//! PM1 polling is skipped.  The VFS provider still mounts and serves stale
//! state so consumers do not crash.
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
extern crate alloc;

use acpi_common::{
    AcpiEvent, KIND_LID_CLOSE, KIND_LID_OPEN, KIND_PM1_POWER_BTN, KIND_PM1_SLEEP_BTN,
    SOURCE_PM1,
};
use abi::errors::Errno;
use abi::vfs_rpc::VfsRpcOp;
use ipc_helpers::provider::{ProviderLoop, ProviderResponse};
use stem::syscall::vfs::{vfs_close, vfs_mount, vfs_open, vfs_read, vfs_write};
use stem::syscall::{ioport_read, ioport_write, irq_subscribe};
use stem::{info, trace, warn};

// ── Driver manifest glue (mirrors acpi_ec) ───────────────────────────────────

use abi::driver_interface::{
    DRIVER_DESCRIPTOR_ABI_VERSION, DeviceInfo, DriverClass, DriverDescriptor, DriverEntryCtx,
    ProbeResult, Status,
};
use stem::abi::module_manifest::{MANIFEST_MAGIC, ManifestHeader, ModuleKind, device_kind_bytes};

const THINGOS_DRIVER_NAME: &[u8] = b"acpi_power";
const KIND_DRV_POWER: &str = "drv.AcpiPower";

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
    if out.is_null() {
        return Status::InvalidArgument;
    }
    let out = &mut *out;
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
    device_kind: device_kind_bytes(KIND_DRV_POWER.as_bytes()),
    version: 1,
    _reserved: 0,
};

// ── Constants ─────────────────────────────────────────────────────────────────

const MOUNT_PATH: &str = "/run/power";
/// Path to the normalized ACPI event bus maintained by `acpid`.
const ACPI_EVENT_BUS: &str = "/services/acpi/events";
const PORT_CAPACITY: usize = 4096;

/// SCI IRQ vector (ACPI SCI is typically GSI 9, vector 0x29 in Thing-OS).
const SCI_VECTOR: u8 = 0x29;

/// Background-loop polling interval (milliseconds).
const POLL_MS: u64 = 100;

/// Maximum retries waiting for acpid and acpi_ec to mount their services.
const SERVICE_WAIT_RETRIES: usize = 50;

// ── VFS handles & inodes ──────────────────────────────────────────────────────

const HANDLE_ROOT: u64   = 1;
const HANDLE_EVENTS: u64 = 2;
const HANDLE_LID: u64    = 3;
const HANDLE_ACTION: u64 = 4;

const INO_ROOT:   u64 = 0xac10_0001;
const INO_EVENTS: u64 = 0xac10_0002;
const INO_LID:    u64 = 0xac10_0003;
const INO_ACTION: u64 = 0xac10_0004;

// ── Power event byte codes ────────────────────────────────────────────────────

pub const EVENT_POWER_BUTTON: u8 = 0x01;
pub const EVENT_SLEEP_BUTTON: u8 = 0x02;
pub const EVENT_LID_CLOSE:    u8 = 0x03;
pub const EVENT_LID_OPEN:     u8 = 0x04;

/// Capacity of the in-process event ring buffer.
const EVENTS_CAP: usize = 32;

// ── PM1 event status register bit masks ──────────────────────────────────────

const PM1_PWRBTN_STS: u16 = 1 << 8;
const PM1_SLPBTN_STS: u16 = 1 << 9;

// ── FADT structure offsets ────────────────────────────────────────────────────

const FADT_PM1A_EVT_BLK:  usize = 56;
const FADT_PM1B_EVT_BLK:  usize = 60;
const FADT_PM1A_CNT_BLK:  usize = 64;
const FADT_PM1_EVT_LEN:   usize = 88;
const FADT_RESET_REG:     usize = 116; // 12-byte GAS
const FADT_RESET_VALUE:   usize = 128;
const FADT_MIN_LEN:       usize = 129;

/// Minimum number of FADT bytes we need to read.
const FADT_READ_SIZE: usize = 256;

/// Generic Address Structure (GAS) from ACPI spec.
#[derive(Copy, Clone, Debug)]
struct Gas {
    space_id:    u8,
    bit_width:   u8,
    #[allow(dead_code)]
    bit_offset:  u8,
    #[allow(dead_code)]
    access_size: u8,
    address:     u64,
}

impl Gas {
    fn from_bytes(b: &[u8]) -> Option<Self> {
        if b.len() < 12 { return None; }
        Some(Self {
            space_id:    b[0],
            bit_width:   b[1],
            bit_offset:  b[2],
            access_size: b[3],
            address:     u64::from_le_bytes(b[4..12].try_into().ok()?),
        })
    }

    /// Write `value` to this register via I/O port (space_id == 1).
    ///
    /// Used for the FADT reset path: writing `reset_value` here triggers a
    /// platform reset on systems that implement the ACPI reset register.
    fn write_io(&self, value: u8) {
        if self.space_id == 1 && self.address != 0 {
            let width = if self.bit_width == 0 { 1 } else { ((self.bit_width + 7) / 8).max(1) as usize };
            ioport_write(self.address as usize, value as usize, width);
        }
    }
}

/// Parsed FADT fields needed by acpi_power.
struct FadtInfo {
    pm1a_evt_blk: u32,
    pm1b_evt_blk: u32,
    pm1a_cnt_blk: u32,
    pm1_evt_half: usize, // PM1_EVT_LEN / 2 (status register half)
    reset_reg:    Gas,
    reset_value:  u8,
}

impl FadtInfo {
    fn pm1_sts_port(&self) -> Option<usize> {
        if self.pm1a_evt_blk == 0 || self.pm1_evt_half == 0 {
            None
        } else {
            Some(self.pm1a_evt_blk as usize)
        }
    }

    fn pm1b_sts_port(&self) -> Option<usize> {
        if self.pm1b_evt_blk == 0 || self.pm1_evt_half == 0 {
            None
        } else {
            Some(self.pm1b_evt_blk as usize)
        }
    }

    /// Attempt a platform reset via the FADT reset register.
    ///
    /// Returns without resetting if the reset register is absent or not an
    /// I/O port register (memory-mapped reset is not yet supported).
    fn try_reset(&self) {
        self.reset_reg.write_io(self.reset_value);
    }

    /// Attempt S5 (soft-off) via the PM1a control register.
    ///
    /// Uses SLP_TYP=5, SLP_EN as a best-effort value when the DSDT \_S5
    /// object is not available from userspace.  Falls back to the stem
    /// shutdown syscall regardless.
    fn try_shutdown(&self) {
        if self.pm1a_cnt_blk != 0 {
            // SLP_EN (bit 13) | SLP_TYP=5 (bits 10–12).
            const SLP_TYP5: u16 = 5 << 10;
            const SLP_EN:   u16 = 1 << 13;
            ioport_write(self.pm1a_cnt_blk as usize, (SLP_TYP5 | SLP_EN) as usize, 2);
        }
    }
}

/// Attempt to read and parse the FADT from the acpid service.
///
/// Returns `None` gracefully if acpid has no FACP table or if the table
/// is shorter than `FADT_MIN_LEN` bytes (pre-ACPI 2.0 or non-ACPI hardware).
fn read_fadt() -> Option<FadtInfo> {
    use abi::syscall::vfs_flags::O_RDONLY;
    let path = "/services/acpi/tables/FACP";
    let fd = vfs_open(path, O_RDONLY).ok()?;
    let mut buf = [0u8; FADT_READ_SIZE];
    let mut total = 0usize;
    loop {
        match vfs_read(fd, &mut buf[total..]) {
            Ok(0)  => break,
            Ok(n)  => { total += n; if total >= buf.len() { break; } }
            Err(_) => break,
        }
    }
    let _ = vfs_close(fd);
    if total < FADT_MIN_LEN {
        return None;
    }

    let pm1a_evt_blk = u32::from_le_bytes(buf[FADT_PM1A_EVT_BLK..FADT_PM1A_EVT_BLK + 4].try_into().ok()?);
    let pm1b_evt_blk = u32::from_le_bytes(buf[FADT_PM1B_EVT_BLK..FADT_PM1B_EVT_BLK + 4].try_into().ok()?);
    let pm1a_cnt_blk = u32::from_le_bytes(buf[FADT_PM1A_CNT_BLK..FADT_PM1A_CNT_BLK + 4].try_into().ok()?);
    let pm1_evt_len  = buf[FADT_PM1_EVT_LEN];
    let reset_reg    = Gas::from_bytes(&buf[FADT_RESET_REG..FADT_RESET_REG + 12])?;
    let reset_value  = buf[FADT_RESET_VALUE];

    Some(FadtInfo {
        pm1a_evt_blk,
        pm1b_evt_blk,
        pm1a_cnt_blk,
        pm1_evt_half: (pm1_evt_len / 2) as usize,
        reset_reg,
        reset_value,
    })
}

// ── Deferred action ───────────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Debug)]
enum DeferredAction {
    Reboot,
    Shutdown,
}

// ── Driver state ──────────────────────────────────────────────────────────────

struct PowerState {
    events:          [u8; EVENTS_CAP],
    ev_head:         usize,
    ev_tail:         usize,
    lid_open:        bool,
    fadt:            Option<FadtInfo>,
    pending_action:  Option<DeferredAction>,
}

impl PowerState {
    fn new(fadt: Option<FadtInfo>) -> Self {
        Self {
            events:         [0u8; EVENTS_CAP],
            ev_head:        0,
            ev_tail:        0,
            lid_open:       true,
            fadt,
            pending_action: None,
        }
    }

    fn enqueue(&mut self, code: u8) {
        let next = (self.ev_tail + 1) % EVENTS_CAP;
        if next == self.ev_head {
            warn!("Event buffer full, dropping code=0x{:02x}", code);
            return;
        }
        self.events[self.ev_tail] = code;
        self.ev_tail = next;
    }

    fn dequeue(&mut self) -> Option<u8> {
        if self.ev_head == self.ev_tail { return None; }
        let code = self.events[self.ev_head];
        self.ev_head = (self.ev_head + 1) % EVENTS_CAP;
        Some(code)
    }
}

// ── ACPI event bus publishing ─────────────────────────────────────────────────

/// Attempt to open the normalized ACPI event bus for writing.
fn open_acpi_bus() -> Option<u32> {
    use abi::syscall::vfs_flags::O_WRONLY;
    vfs_open(ACPI_EVENT_BUS, O_WRONLY).ok()
}

/// Publish one event to the ACPI event bus (best-effort, non-fatal on failure).
fn publish_to_bus(bus_fd: Option<u32>, kind: u8, raw_code: u8) {
    let fd = match bus_fd {
        Some(f) => f,
        None => return,
    };
    let ts_ms = stem::time::monotonic_ns() / 1_000_000;
    let ev = AcpiEvent { timestamp_ms: ts_ms, kind, raw_code, source: SOURCE_PM1, flags: 0, extra: 0 };
    let _ = vfs_write(fd, &ev.to_bytes());
}

// ── PM1 event polling ─────────────────────────────────────────────────────────

fn poll_pm1_port(state: &mut PowerState, port: usize, bus_fd: Option<u32>) {
    let pm1_sts = ioport_read(port, 2) as u16;
    if pm1_sts == 0 || pm1_sts == 0xFFFF {
        return;
    }

    if pm1_sts & PM1_PWRBTN_STS != 0 {
        // Write-1-to-clear the status bit.
        ioport_write(port, PM1_PWRBTN_STS as usize, 2);
        state.enqueue(EVENT_POWER_BUTTON);
        publish_to_bus(bus_fd, KIND_PM1_POWER_BTN, 0);
        info!("Power button pressed");
    }

    if pm1_sts & PM1_SLPBTN_STS != 0 {
        ioport_write(port, PM1_SLPBTN_STS as usize, 2);
        state.enqueue(EVENT_SLEEP_BUTTON);
        publish_to_bus(bus_fd, KIND_PM1_SLEEP_BTN, 0);
        info!("Sleep button pressed");
    }
}

fn poll_pm1_events(state: &mut PowerState, bus_fd: Option<u32>) {
    // We can't hold a shared reference to state.fadt while mutably borrowing
    // state, so extract the port values first.
    let pm1a_port = state.fadt.as_ref().and_then(|f| f.pm1_sts_port());
    let pm1b_port = state.fadt.as_ref().and_then(|f| f.pm1b_sts_port());
    if let Some(port) = pm1a_port { poll_pm1_port(state, port, bus_fd); }
    if let Some(port) = pm1b_port { poll_pm1_port(state, port, bus_fd); }
}

// ── EC event polling ──────────────────────────────────────────────────────────

/// Common EC query codes for lid events seen across laptop firmware.
/// Lid-close codes are system-specific; these match common ThinkPad/Lenovo
/// patterns and serve as a best-effort baseline.
const EC_LID_CLOSE_CODES: &[u8] = &[0x80, 0x5D];
const EC_LID_OPEN_CODES:  &[u8] = &[0x81, 0x5C];

fn poll_ec_events(state: &mut PowerState, ec_fd: Option<u32>, bus_fd: Option<u32>) {
    let fd = match ec_fd {
        Some(f) => f,
        None => return,
    };
    let mut code = [0u8; 1];
    match vfs_read(fd, &mut code) {
        Ok(1) => {
            let c = code[0];
            if EC_LID_CLOSE_CODES.contains(&c) {
                state.lid_open = false;
                state.enqueue(EVENT_LID_CLOSE);
                publish_to_bus(bus_fd, KIND_LID_CLOSE, c);
                info!("Lid closed");
                trace!("Lid closed via EC query=0x{:02x}", c);
            } else if EC_LID_OPEN_CODES.contains(&c) {
                state.lid_open = true;
                state.enqueue(EVENT_LID_OPEN);
                publish_to_bus(bus_fd, KIND_LID_OPEN, c);
                info!("Lid opened");
                trace!("Lid opened via EC query=0x{:02x}", c);
            }
        }
        _ => {}
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
    match path.trim_start_matches('/') {
        ""       => ProviderResponse::ok_u64(HANDLE_ROOT),
        "events" => ProviderResponse::ok_u64(HANDLE_EVENTS),
        "lid"    => ProviderResponse::ok_u64(HANDLE_LID),
        "action" => ProviderResponse::ok_u64(HANDLE_ACTION),
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
        HANDLE_ROOT   => ProviderResponse::ok_stat(S_IFDIR | 0o555, 0,  INO_ROOT),
        HANDLE_EVENTS => ProviderResponse::ok_stat(S_IFREG | 0o444, 0,  INO_EVENTS),
        HANDLE_LID    => ProviderResponse::ok_stat(S_IFREG | 0o444, 7,  INO_LID),
        HANDLE_ACTION => ProviderResponse::ok_stat(S_IFREG | 0o200, 0,  INO_ACTION),
        _             => ProviderResponse::err(Errno::EBADF),
    }
}

fn handle_read(state: &mut PowerState, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 8 { return ProviderResponse::err(Errno::EINVAL); }
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
    match handle {
        HANDLE_EVENTS => {
            match state.dequeue() {
                Some(code) => ProviderResponse::ok_read(&[code]),
                None       => ProviderResponse::err(Errno::EAGAIN),
            }
        }
        HANDLE_LID => {
            let text: &[u8] = if state.lid_open { b"open\n" } else { b"closed\n" };
            ProviderResponse::ok_read(text)
        }
        HANDLE_ROOT   => ProviderResponse::err(Errno::EISDIR),
        HANDLE_ACTION => ProviderResponse::err(Errno::EACCES),
        _             => ProviderResponse::err(Errno::EBADF),
    }
}

fn handle_write(state: &mut PowerState, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 20 { return ProviderResponse::err(Errno::EINVAL); }
    let handle   = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
    let data_len = u32::from_le_bytes(payload[16..20].try_into().unwrap_or([0; 4])) as usize;
    if payload.len() < 20 + data_len { return ProviderResponse::err(Errno::EINVAL); }
    let data = &payload[20..20 + data_len];

    match handle {
        HANDLE_ACTION => {
            let cmd = match core::str::from_utf8(data) {
                Ok(s) => s.trim_end_matches(['\n', '\r', ' ']),
                Err(_) => return ProviderResponse::err(Errno::EINVAL),
            };
            match cmd {
                "reboot" => {
                    info!("Reboot requested via /run/power/action");
                    state.pending_action = Some(DeferredAction::Reboot);
                    ProviderResponse::ok_written(data_len as u32)
                }
                "shutdown" => {
                    info!("Shutdown requested via /run/power/action");
                    state.pending_action = Some(DeferredAction::Shutdown);
                    ProviderResponse::ok_written(data_len as u32)
                }
                _ => ProviderResponse::err(Errno::EINVAL),
            }
        }
        HANDLE_ROOT | HANDLE_EVENTS | HANDLE_LID => ProviderResponse::err(Errno::EACCES),
        _ => ProviderResponse::err(Errno::EBADF),
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
        ("events", INO_EVENTS),
        ("lid",    INO_LID),
        ("action", INO_ACTION),
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

fn dispatch(state: &mut PowerState, op: VfsRpcOp, payload: &[u8]) -> ProviderResponse {
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

fn open_ec_events() -> Option<u32> {
    use abi::syscall::vfs_flags::O_RDONLY;
    for _ in 0..SERVICE_WAIT_RETRIES {
        if let Ok(fd) = vfs_open("/services/ec/events", O_RDONLY) {
            return Some(fd);
        }
        stem::time::sleep_ms(20);
    }
    None
}

fn run_loop(req_read: u32) -> ! {
    // Parse FADT (retry briefly to let acpid finish mounting its VFS).
    let mut fadt_opt: Option<FadtInfo> = None;
    for _ in 0..SERVICE_WAIT_RETRIES {
        match read_fadt() {
            Some(f) => { fadt_opt = Some(f); break; }
            None    => stem::time::sleep_ms(20),
        }
    }

    match &fadt_opt {
        Some(f) if f.pm1a_evt_blk != 0 => {
            trace!("PM1a event block at port 0x{:04x} (half={} bytes)",
                   f.pm1a_evt_blk, f.pm1_evt_half);
        }
        _ => info!("PM1 polling disabled (FADT not available)"),
    }

    // Try to open the EC events stream.
    let ec_fd = open_ec_events();
    if ec_fd.is_some() {
        info!("EC events stream opened");
    } else {
        info!("EC events stream unavailable; lid detection disabled");
    }

    // Try to open the normalized ACPI event bus (non-fatal if acpid not ready).
    let bus_fd = open_acpi_bus();
    if bus_fd.is_some() {
        info!("Connected to ACPI event bus");
    }

    let mut lp    = ProviderLoop::new(req_read);
    let mut state = PowerState::new(fadt_opt);

    loop {
        // 1. Drain all pending VFS requests (non-blocking).
        loop {
            match lp.try_next_request() {
                Ok(Some(req)) => {
                    let resp = dispatch(&mut state, req.op, &req.payload);
                    if let Err(e) = lp.send_response(&req, resp) {
                        warn!("send_response failed: {:?}", e);
                    }
                    // Execute any deferred action AFTER sending the response.
                    // Try FADT hardware paths first, then fall back to the
                    // kernel syscall which handles the platform reset/shutdown.
                    if let Some(action) = state.pending_action.take() {
                        match action {
                            DeferredAction::Reboot => {
                                if let Some(ref fadt) = state.fadt {
                                    fadt.try_reset();
                                }
                                stem::syscall::reboot()
                            }
                            DeferredAction::Shutdown => {
                                if let Some(ref fadt) = state.fadt {
                                    fadt.try_shutdown();
                                }
                                stem::syscall::shutdown()
                            }
                        }
                    }
                }
                Ok(None)          => break,
                Err(Errno::EPIPE) => stem::syscall::exit(0),
                Err(_)            => break,
            }
        }

        // 2. Poll PM1 event status registers for fixed hardware events.
        poll_pm1_events(&mut state, bus_fd);

        // 3. Drain EC query-event codes for lid state.
        poll_ec_events(&mut state, ec_fd, bus_fd);

        stem::time::sleep_ms(POLL_MS);
    }
}

// ── Entry point ───────────────────────────────────────────────────────────────

#[stem::main]
fn main(_raw_arg: usize) -> ! {
    info!("ACPI power service online");

    // Subscribe to the ACPI SCI so we wake promptly on fixed hardware events.
    match irq_subscribe(SCI_VECTOR) {
        Ok(())   => trace!("Subscribed to SCI vector 0x{:02x}", SCI_VECTOR),
        Err(err) => {
            trace!("SCI subscription failed: {:?}", err);
            info!("SCI IRQ unavailable; relying on polling");
        }
    }

    let (req_write, req_read) = match stem::syscall::port::port_create(PORT_CAPACITY) {
        Ok(p) => p,
        Err(e) => {
            warn!("port_create failed: {:?}", e);
            stem::syscall::exit(1);
        }
    };

    let _ = stem::syscall::vfs::vfs_mkdir("/run");
    if let Err(e) = vfs_mount(req_write, MOUNT_PATH) {
        warn!("Mount at {} failed: {:?}", MOUNT_PATH, e);
    } else {
        info!("Mounted at {}", MOUNT_PATH);
    }

    run_loop(req_read)
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_root() {
        let payload = b"\x00\x00\x00\x00";
        let resp = handle_lookup(payload);
        assert_eq!(resp.status, 0);
        assert_eq!(u64::from_le_bytes(resp.payload[..8].try_into().unwrap()), HANDLE_ROOT);
    }

    #[test]
    fn lookup_known_files() {
        for (name, expected) in &[
            ("events", HANDLE_EVENTS),
            ("lid",    HANDLE_LID),
            ("action", HANDLE_ACTION),
        ] {
            let mut payload = (name.len() as u32).to_le_bytes().to_vec();
            payload.extend_from_slice(name.as_bytes());
            let resp = handle_lookup(&payload);
            assert_eq!(resp.status, 0, "lookup failed for {}", name);
            let handle = u64::from_le_bytes(resp.payload[..8].try_into().unwrap());
            assert_eq!(handle, *expected, "wrong handle for {}", name);
        }
    }

    #[test]
    fn lookup_unknown_returns_enoent() {
        let name = "nonexistent";
        let mut payload = (name.len() as u32).to_le_bytes().to_vec();
        payload.extend_from_slice(name.as_bytes());
        let resp = handle_lookup(&payload);
        assert_eq!(resp.status, Errno::ENOENT as u8);
    }

    #[test]
    fn event_ring_buffer_enqueue_dequeue() {
        let mut state = PowerState::new(None);
        assert_eq!(state.dequeue(), None);
        state.enqueue(EVENT_POWER_BUTTON);
        state.enqueue(EVENT_LID_CLOSE);
        assert_eq!(state.dequeue(), Some(EVENT_POWER_BUTTON));
        assert_eq!(state.dequeue(), Some(EVENT_LID_CLOSE));
        assert_eq!(state.dequeue(), None);
    }

    #[test]
    fn event_ring_buffer_full_drops() {
        let mut state = PowerState::new(None);
        // Fill completely (EVENTS_CAP - 1 items fit in a ring of EVENTS_CAP).
        for _ in 0..EVENTS_CAP - 1 {
            state.enqueue(EVENT_POWER_BUTTON);
        }
        // This one should be silently dropped (buffer full warning path).
        state.enqueue(EVENT_SLEEP_BUTTON);
        // Drain and verify no sleep button in the output.
        let mut count = 0usize;
        while let Some(code) = state.dequeue() {
            assert_eq!(code, EVENT_POWER_BUTTON);
            count += 1;
        }
        assert_eq!(count, EVENTS_CAP - 1);
    }

    #[test]
    fn read_events_returns_eagain_when_empty() {
        let mut state = PowerState::new(None);
        // Build a minimal Read payload: 8-byte handle, rest zeros.
        let mut payload = [0u8; 20];
        payload[0..8].copy_from_slice(&HANDLE_EVENTS.to_le_bytes());
        let resp = handle_read(&mut state, &payload);
        assert_eq!(resp.status, Errno::EAGAIN as u8);
    }

    #[test]
    fn read_events_returns_event_byte() {
        let mut state = PowerState::new(None);
        state.enqueue(EVENT_LID_OPEN);
        let mut payload = [0u8; 20];
        payload[0..8].copy_from_slice(&HANDLE_EVENTS.to_le_bytes());
        let resp = handle_read(&mut state, &payload);
        assert_eq!(resp.status, 0);
        // ok_read prepends a 4-byte length, then the data byte.
        assert_eq!(resp.payload.len(), 5);
        assert_eq!(resp.payload[4], EVENT_LID_OPEN);
    }

    #[test]
    fn read_lid_state() {
        let mut state = PowerState::new(None);
        let mut payload = [0u8; 20];
        payload[0..8].copy_from_slice(&HANDLE_LID.to_le_bytes());

        let resp = handle_read(&mut state, &payload);
        assert_eq!(resp.status, 0);
        let data = &resp.payload[4..]; // skip 4-byte length prefix
        assert_eq!(data, b"open\n");

        state.lid_open = false;
        let resp = handle_read(&mut state, &payload);
        assert_eq!(resp.status, 0);
        let data = &resp.payload[4..];
        assert_eq!(data, b"closed\n");
    }

    #[test]
    fn write_invalid_action_returns_einval() {
        let mut state = PowerState::new(None);
        let cmd = b"suspend";
        let mut payload = alloc::vec![0u8; 20 + cmd.len()];
        payload[0..8].copy_from_slice(&HANDLE_ACTION.to_le_bytes());
        payload[16..20].copy_from_slice(&(cmd.len() as u32).to_le_bytes());
        payload[20..20 + cmd.len()].copy_from_slice(cmd);
        let resp = handle_write(&mut state, &payload);
        assert_eq!(resp.status, Errno::EINVAL as u8);
    }

    #[test]
    fn write_reboot_sets_deferred_action() {
        let mut state = PowerState::new(None);
        let cmd = b"reboot";
        let mut payload = alloc::vec![0u8; 20 + cmd.len()];
        payload[0..8].copy_from_slice(&HANDLE_ACTION.to_le_bytes());
        payload[16..20].copy_from_slice(&(cmd.len() as u32).to_le_bytes());
        payload[20..20 + cmd.len()].copy_from_slice(cmd);
        let resp = handle_write(&mut state, &payload);
        assert_eq!(resp.status, 0);
        assert_eq!(state.pending_action, Some(DeferredAction::Reboot));
    }

    #[test]
    fn write_shutdown_sets_deferred_action() {
        let mut state = PowerState::new(None);
        let cmd = b"shutdown\n"; // with trailing newline
        let mut payload = alloc::vec![0u8; 20 + cmd.len()];
        payload[0..8].copy_from_slice(&HANDLE_ACTION.to_le_bytes());
        payload[16..20].copy_from_slice(&(cmd.len() as u32).to_le_bytes());
        payload[20..20 + cmd.len()].copy_from_slice(cmd);
        let resp = handle_write(&mut state, &payload);
        assert_eq!(resp.status, 0);
        assert_eq!(state.pending_action, Some(DeferredAction::Shutdown));
    }

    #[test]
    fn gas_from_bytes_parses_correctly() {
        let mut buf = [0u8; 12];
        buf[0] = 1; // I/O space
        buf[1] = 8; // 8-bit
        buf[4] = 0xB2; // low byte of address
        let gas = Gas::from_bytes(&buf).unwrap();
        assert_eq!(gas.space_id, 1);
        assert_eq!(gas.bit_width, 8);
        assert_eq!(gas.address, 0xB2);
    }

    #[test]
    fn readdir_returns_three_entries() {
        let mut payload = [0u8; 20];
        payload[0..8].copy_from_slice(&HANDLE_ROOT.to_le_bytes());
        payload[16..20].copy_from_slice(&4096u32.to_le_bytes());
        let resp = handle_readdir(&payload);
        assert_eq!(resp.status, 0);
        // Count entries: each is 8 + 1 + 1 + name_len bytes.
        // events=6, lid=3, action=6
        let expected_bytes = (8+1+1+6) + (8+1+1+3) + (8+1+1+6); // 16+13+16 = 45
        assert_eq!(resp.payload.len(), 4 + expected_bytes); // 4-byte length prefix
    }
}

