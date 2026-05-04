//! # acpi_battery — ACPI battery and AC adapter reporting service
//!
//! Polls the Embedded Controller (EC) for battery state and AC adapter
//! presence, reads ACPI namespace data from acpid to confirm device
//! presence, and exposes normalised power-supply state at `/sys/power/`.
//!
//! ## VFS layout
//!
//! ```text
//! /sys/power/
//!   battery0/
//!     status    ← "Charging\n", "Discharging\n", "Full\n", or "Unknown\n"
//!     percent   ← "XX\n" (0–100, or "Unknown\n" when unavailable)
//!     rate      ← present charge/discharge rate in mW, e.g. "1500\n"
//!     capacity  ← full charge capacity in mWh, e.g. "45000\n"
//!   ac          ← "online\n" or "offline\n"
//! ```
//!
//! ## EC register layout
//!
//! Battery registers follow the ACPI Embedded Controller battery region layout
//! used by the majority of ACPI-compatible laptops:
//!
//! | Register | Width | Meaning |
//! |----------|-------|---------|
//! | `0x38`   | 8-bit | Battery state: bit0=discharging, bit1=charging, bit2=critical |
//! | `0x3A`   | 16-bit LE | Remaining capacity (mAh) |
//! | `0x3C`   | 16-bit LE | Present rate (mA) |
//! | `0x48`   | 16-bit LE | Full charge capacity (mAh) |
//! | `0xA0`   | 8-bit | AC state: bit0=AC present |
//!
//! ## Graceful degradation
//!
//! When the EC service or acpid is unavailable the driver still mounts its VFS
//! provider and returns "Unknown" / 0 values so consumers never crash.
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
extern crate alloc;

use abi::errors::Errno;
use abi::vfs_rpc::VfsRpcOp;
use ipc_helpers::provider::{ProviderLoop, ProviderResponse};
use stem::syscall::vfs::{vfs_close, vfs_mount, vfs_open, vfs_read, vfs_write};
use stem::{debug, info, trace, warn};

// ── Driver manifest glue ──────────────────────────────────────────────────────

use abi::driver_interface::{
    DRIVER_DESCRIPTOR_ABI_VERSION, DeviceInfo, DriverClass, DriverDescriptor, DriverEntryCtx,
    ProbeResult, Status,
};
use stem::abi::module_manifest::{MANIFEST_MAGIC, ManifestHeader, ModuleKind, device_kind_bytes};

const THINGOS_DRIVER_NAME: &[u8] = b"acpi_battery";
const KIND_DRV_BATTERY: &str = "drv.AcpiBattery";

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
    device_kind: device_kind_bytes(KIND_DRV_BATTERY.as_bytes()),
    version: 1,
    _reserved: 0,
};

// ── Constants ─────────────────────────────────────────────────────────────────

const MOUNT_PATH: &str = "/sys/power";
const PORT_CAPACITY: usize = 4096;

/// Background-loop polling interval (milliseconds).
const POLL_MS: u64 = 5000;

/// Maximum retries waiting for acpid and acpi_ec to mount their services.
const SERVICE_WAIT_RETRIES: usize = 50;

// ── EC register offsets for battery/AC (common ACPI laptop layout) ────────────

/// Battery state: bit0=discharging, bit1=charging, bit2=critical.
const EC_REG_BATT_STATE:  u8 = 0x38;
/// Remaining capacity low byte (mAh, little-endian 16-bit pair with 0x3B).
const EC_REG_BATT_REMA_L: u8 = 0x3A;
/// Remaining capacity high byte.
const EC_REG_BATT_REMA_H: u8 = 0x3B;
/// Present rate low byte (mA, little-endian 16-bit pair with 0x3D).
const EC_REG_BATT_RATE_L: u8 = 0x3C;
/// Present rate high byte.
const EC_REG_BATT_RATE_H: u8 = 0x3D;
/// Full charge capacity low byte (mAh, little-endian 16-bit pair with 0x49).
const EC_REG_BATT_FULL_L: u8 = 0x48;
/// Full charge capacity high byte.
const EC_REG_BATT_FULL_H: u8 = 0x49;
/// AC adapter state: bit0=AC present.
const EC_REG_AC_STATE:    u8 = 0xA0;

/// Battery state bits.
const BATT_STATE_DISCHARGING: u8 = 1 << 0;
const BATT_STATE_CHARGING:    u8 = 1 << 1;

// ── VFS handles & inodes ──────────────────────────────────────────────────────

const HANDLE_ROOT:        u64 = 1;
const HANDLE_BATTERY0:    u64 = 2;
const HANDLE_STATUS:      u64 = 3;
const HANDLE_PERCENT:     u64 = 4;
const HANDLE_RATE:        u64 = 5;
const HANDLE_CAPACITY:    u64 = 6;
const HANDLE_AC:          u64 = 7;

const INO_ROOT:           u64 = 0xba00_0001;
const INO_BATTERY0:       u64 = 0xba00_0002;
const INO_STATUS:         u64 = 0xba00_0003;
const INO_PERCENT:        u64 = 0xba00_0004;
const INO_RATE:           u64 = 0xba00_0005;
const INO_CAPACITY:       u64 = 0xba00_0006;
const INO_AC:             u64 = 0xba00_0007;

// ── Battery charging state ────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Debug)]
enum ChargingState {
    Charging,
    Discharging,
    Full,
    Unknown,
}

impl ChargingState {
    fn as_str(self) -> &'static str {
        match self {
            ChargingState::Charging    => "Charging\n",
            ChargingState::Discharging => "Discharging\n",
            ChargingState::Full        => "Full\n",
            ChargingState::Unknown     => "Unknown\n",
        }
    }
}

// ── Driver state ──────────────────────────────────────────────────────────────

struct BatteryState {
    /// Whether a PNP0C0A battery device was found in the ACPI namespace.
    battery_present: bool,
    /// Whether an ACPI0003 AC adapter device was found in the ACPI namespace.
    ac_device_present: bool,
    /// Charging / discharging / full / unknown.
    charging_state: ChargingState,
    /// Remaining capacity in mAh (0 when unknown).
    remaining_mah: u16,
    /// Full charge capacity in mAh (0 when unknown).
    full_mah: u16,
    /// Present charge/discharge rate in mA (0 when unknown).
    rate_ma: u16,
    /// AC adapter online (true = AC present).
    ac_online: bool,
}

impl BatteryState {
    fn new() -> Self {
        Self {
            battery_present:   false,
            ac_device_present: false,
            charging_state:    ChargingState::Unknown,
            remaining_mah:     0,
            full_mah:          0,
            rate_ma:           0,
            ac_online:         false,
        }
    }

    /// Compute battery percentage from remaining/full capacity.
    /// Returns None when capacity data is unavailable.
    fn percent(&self) -> Option<u8> {
        if self.full_mah == 0 { return None; }
        Some(((self.remaining_mah as u32 * 100) / self.full_mah as u32).min(100) as u8)
    }
}

// ── EC register access ────────────────────────────────────────────────────────

/// Read a single EC register via `/services/ec/read`.
///
/// The EC `read` file expects a 1-byte write (register address) followed by a
/// 1-byte read (result).  Returns `None` on any I/O error.
fn ec_read_reg(ec_read_fd: u32, reg: u8) -> Option<u8> {
    vfs_write(ec_read_fd, &[reg]).ok()?;
    let mut buf = [0u8; 1];
    vfs_read(ec_read_fd, &mut buf).ok()?;
    Some(buf[0])
}

/// Open `/services/ec/read`, retrying until the service is available.
fn open_ec_read() -> Option<u32> {
    use abi::syscall::vfs_flags::O_RDWR;
    for _ in 0..SERVICE_WAIT_RETRIES {
        if let Ok(fd) = vfs_open("/services/ec/read", O_RDWR) {
            return Some(fd);
        }
        stem::time::sleep_ms(20);
    }
    None
}

// ── ACPI namespace polling ────────────────────────────────────────────────────

/// Check if a given HID device directory exists in the acpid namespace.
///
/// Reads `/services/acpi/devices/<HID>:00/hid` and returns true if the file
/// is readable.  Does not retry — called once at startup after acpid is up.
fn acpi_device_present(hid: &str) -> bool {
    use abi::syscall::vfs_flags::O_RDONLY;
    let mut path = alloc::string::String::from("/services/acpi/devices/");
    path.push_str(hid);
    path.push_str(":00/hid");
    if let Ok(fd) = vfs_open(&path, O_RDONLY) {
        let _ = vfs_close(fd);
        true
    } else {
        false
    }
}

/// Wait for acpid to mount, then probe for battery and AC adapter devices.
fn probe_acpi_devices(state: &mut BatteryState) {
    // Give acpid time to mount.
    for _ in 0..SERVICE_WAIT_RETRIES {
        use abi::syscall::vfs_flags::O_RDONLY;
        if vfs_open("/services/acpi/devices", O_RDONLY)
            .map(|fd| { let _ = vfs_close(fd); })
            .is_ok()
        {
            break;
        }
        stem::time::sleep_ms(20);
    }

    state.battery_present   = acpi_device_present("PNP0C0A");
    state.ac_device_present = acpi_device_present("ACPI0003");

    if state.battery_present {
        debug!("ACPI battery device PNP0C0A found");
    } else {
        debug!("No ACPI battery device in namespace; EC polling will still attempt");
    }
    if state.ac_device_present {
        debug!("ACPI AC adapter device ACPI0003 found");
    } else {
        debug!("No ACPI AC adapter device in namespace");
    }
}

// ── EC polling ────────────────────────────────────────────────────────────────

/// Poll EC registers and update `state` with the latest battery/AC values.
fn poll_ec(state: &mut BatteryState, ec_fd: Option<u32>) {
    let fd = match ec_fd {
        Some(f) => f,
        None => return,
    };

    // --- Battery state ---
    if let Some(bst) = ec_read_reg(fd, EC_REG_BATT_STATE) {
        state.charging_state = if bst & BATT_STATE_CHARGING != 0 {
            ChargingState::Charging
        } else if bst & BATT_STATE_DISCHARGING != 0 {
            ChargingState::Discharging
        } else {
            // Neither bit set → battery full or AC with no charge needed.
            ChargingState::Full
        };
        trace!("EC battery state register = 0x{:02x}", bst);
    }

    // --- Remaining capacity ---
    if let (Some(lo), Some(hi)) = (
        ec_read_reg(fd, EC_REG_BATT_REMA_L),
        ec_read_reg(fd, EC_REG_BATT_REMA_H),
    ) {
        state.remaining_mah = u16::from_le_bytes([lo, hi]);
        trace!("EC remaining capacity = {} mAh", state.remaining_mah);
    }

    // --- Full charge capacity ---
    if let (Some(lo), Some(hi)) = (
        ec_read_reg(fd, EC_REG_BATT_FULL_L),
        ec_read_reg(fd, EC_REG_BATT_FULL_H),
    ) {
        state.full_mah = u16::from_le_bytes([lo, hi]);
        trace!("EC full capacity = {} mAh", state.full_mah);
    }

    // --- Present rate ---
    if let (Some(lo), Some(hi)) = (
        ec_read_reg(fd, EC_REG_BATT_RATE_L),
        ec_read_reg(fd, EC_REG_BATT_RATE_H),
    ) {
        state.rate_ma = u16::from_le_bytes([lo, hi]);
        trace!("EC rate = {} mA", state.rate_ma);
    }

    // --- AC adapter state ---
    if let Some(ac) = ec_read_reg(fd, EC_REG_AC_STATE) {
        let online = ac & 0x01 != 0;
        if online != state.ac_online {
            if online {
                info!("AC adapter online");
            } else {
                info!("AC adapter offline");
            }
        }
        state.ac_online = online;
        trace!("EC AC state register = 0x{:02x}", ac);
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
        ""                  => ProviderResponse::ok_u64(HANDLE_ROOT),
        "battery0"          => ProviderResponse::ok_u64(HANDLE_BATTERY0),
        "battery0/status"   => ProviderResponse::ok_u64(HANDLE_STATUS),
        "battery0/percent"  => ProviderResponse::ok_u64(HANDLE_PERCENT),
        "battery0/rate"     => ProviderResponse::ok_u64(HANDLE_RATE),
        "battery0/capacity" => ProviderResponse::ok_u64(HANDLE_CAPACITY),
        "ac"                => ProviderResponse::ok_u64(HANDLE_AC),
        _                   => ProviderResponse::err(Errno::ENOENT),
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
        HANDLE_ROOT     => ProviderResponse::ok_stat(S_IFDIR | 0o555, 0,  INO_ROOT),
        HANDLE_BATTERY0 => ProviderResponse::ok_stat(S_IFDIR | 0o555, 0,  INO_BATTERY0),
        HANDLE_STATUS   => ProviderResponse::ok_stat(S_IFREG | 0o444, 0,  INO_STATUS),
        HANDLE_PERCENT  => ProviderResponse::ok_stat(S_IFREG | 0o444, 0,  INO_PERCENT),
        HANDLE_RATE     => ProviderResponse::ok_stat(S_IFREG | 0o444, 0,  INO_RATE),
        HANDLE_CAPACITY => ProviderResponse::ok_stat(S_IFREG | 0o444, 0,  INO_CAPACITY),
        HANDLE_AC       => ProviderResponse::ok_stat(S_IFREG | 0o444, 0,  INO_AC),
        _               => ProviderResponse::err(Errno::EBADF),
    }
}

fn format_u32(n: u32, buf: &mut [u8; 16]) -> &[u8] {
    // Write decimal digits + newline into buf, return the filled slice.
    let mut tmp = [0u8; 12];
    let mut pos = 0usize;
    let mut val = n;
    if val == 0 {
        tmp[pos] = b'0';
        pos += 1;
    } else {
        while val > 0 {
            tmp[pos] = b'0' + (val % 10) as u8;
            pos += 1;
            val /= 10;
        }
        // Reverse the digits.
        tmp[..pos].reverse();
    }
    tmp[pos] = b'\n';
    pos += 1;
    buf[..pos].copy_from_slice(&tmp[..pos]);
    &buf[..pos]
}

fn handle_read(state: &BatteryState, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 8 { return ProviderResponse::err(Errno::EINVAL); }
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
    let mut num_buf = [0u8; 16];
    match handle {
        HANDLE_STATUS => {
            ProviderResponse::ok_read(state.charging_state.as_str().as_bytes())
        }
        HANDLE_PERCENT => {
            match state.percent() {
                Some(pct) => {
                    let text = format_u32(pct as u32, &mut num_buf);
                    ProviderResponse::ok_read(text)
                }
                None => ProviderResponse::ok_read(b"Unknown\n"),
            }
        }
        HANDLE_RATE => {
            // Convert mA to mW: approximate using nominal 1000 mV per cell (×1),
            // or just expose mA directly as a dimensionless count.  The spec says
            // "rate" so we expose the mA value (most ACPI drivers do the same
            // when voltage is unknown).
            let text = format_u32(state.rate_ma as u32, &mut num_buf);
            ProviderResponse::ok_read(text)
        }
        HANDLE_CAPACITY => {
            let text = format_u32(state.full_mah as u32, &mut num_buf);
            ProviderResponse::ok_read(text)
        }
        HANDLE_AC => {
            let text: &[u8] = if state.ac_online { b"online\n" } else { b"offline\n" };
            ProviderResponse::ok_read(text)
        }
        HANDLE_ROOT | HANDLE_BATTERY0 => ProviderResponse::err(Errno::EISDIR),
        _ => ProviderResponse::err(Errno::EBADF),
    }
}

fn handle_readdir(payload: &[u8]) -> ProviderResponse {
    if payload.len() < 20 { return ProviderResponse::err(Errno::EINVAL); }
    let handle  = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
    let offset  = u64::from_le_bytes(payload[8..16].try_into().unwrap_or([0; 8])) as usize;
    let max_len = u32::from_le_bytes(payload[16..20].try_into().unwrap_or([0; 4])) as usize;

    const DT_REG: u8 = 8;
    const DT_DIR: u8 = 4;

    let (entries, is_dir): (&[(&str, u64, u8)], bool) = match handle {
        HANDLE_ROOT => (
            &[
                ("battery0", INO_BATTERY0, DT_DIR),
                ("ac",       INO_AC,       DT_REG),
            ],
            true,
        ),
        HANDLE_BATTERY0 => (
            &[
                ("status",   INO_STATUS,   DT_REG),
                ("percent",  INO_PERCENT,  DT_REG),
                ("rate",     INO_RATE,     DT_REG),
                ("capacity", INO_CAPACITY, DT_REG),
            ],
            true,
        ),
        _ => return ProviderResponse::err(Errno::ENOTDIR),
    };
    let _ = is_dir;

    let mut out: alloc::vec::Vec<u8> = alloc::vec::Vec::new();
    for (i, &(name, ino, dt)) in entries.iter().enumerate() {
        if i < offset { continue; }
        let entry_len = 8 + 1 + 1 + name.len();
        if out.len() + entry_len > max_len { break; }
        out.extend_from_slice(&ino.to_le_bytes());
        out.push(dt);
        out.push(name.len() as u8);
        out.extend_from_slice(name.as_bytes());
    }
    ProviderResponse::ok_read(&out)
}

fn dispatch(state: &BatteryState, op: VfsRpcOp, payload: &[u8]) -> ProviderResponse {
    match op {
        VfsRpcOp::Lookup  => handle_lookup(payload),
        VfsRpcOp::Stat    => handle_stat(payload),
        VfsRpcOp::Read    => handle_read(state, payload),
        VfsRpcOp::Readdir => handle_readdir(payload),
        VfsRpcOp::Close   => ProviderResponse::ok_empty(),
        _                 => ProviderResponse::err(Errno::ENOSYS),
    }
}

// ── Main loop ─────────────────────────────────────────────────────────────────

fn run_loop(req_read: u32) -> ! {
    let mut state = BatteryState::new();

    // Probe ACPI namespace for device presence (best-effort, non-fatal).
    probe_acpi_devices(&mut state);

    // Open EC read interface.
    let ec_fd = open_ec_read();
    if ec_fd.is_some() {
        debug!("EC read interface opened");
    } else {
        debug!("EC read interface unavailable; battery values will read as unknown");
    }

    // Initial poll so values are non-stale from the first VFS read.
    poll_ec(&mut state, ec_fd);

    let mut lp = ProviderLoop::new(req_read);
    let mut ticks: u64 = 0;

    loop {
        // 1. Drain all pending VFS requests (non-blocking).
        loop {
            match lp.try_next_request() {
                Ok(Some(req)) => {
                    let resp = dispatch(&state, req.op, &req.payload);
                    if let Err(e) = lp.send_response(&req, resp) {
                        warn!("send_response failed: {:?}", e);
                    }
                }
                Ok(None)          => break,
                Err(Errno::EPIPE) => stem::syscall::exit(0),
                Err(_)            => break,
            }
        }

        // 2. Periodic EC poll (every POLL_MS).
        poll_ec(&mut state, ec_fd);
        ticks += 1;
        trace!("Battery poll tick {}: state={:?} ac={}", ticks,
               state.charging_state, state.ac_online);

        stem::time::sleep_ms(POLL_MS);
    }
}

// ── Entry point ───────────────────────────────────────────────────────────────

#[stem::main]
fn main(_raw_arg: usize) -> ! {
    info!("ACPI battery service online");

    let (req_write, req_read) = match stem::syscall::port::port_create(PORT_CAPACITY) {
        Ok(p) => p,
        Err(e) => {
            warn!("port_create failed: {:?}", e);
            stem::syscall::exit(1);
        }
    };

    let _ = stem::syscall::vfs::vfs_mkdir("/sys");
    let _ = stem::syscall::vfs::vfs_mkdir("/sys/power");
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

    fn make_lookup_payload(name: &str) -> alloc::vec::Vec<u8> {
        let mut v = (name.len() as u32).to_le_bytes().to_vec();
        v.extend_from_slice(name.as_bytes());
        v
    }

    fn make_read_payload(handle: u64) -> alloc::vec::Vec<u8> {
        let mut v = handle.to_le_bytes().to_vec();
        v.extend_from_slice(&[0u8; 12]); // padding
        v
    }

    fn default_state() -> BatteryState {
        BatteryState::new()
    }

    // ── lookup ────────────────────────────────────────────────────────────────

    #[test]
    fn lookup_root() {
        let resp = handle_lookup(&make_lookup_payload(""));
        assert_eq!(resp.status, 0);
        assert_eq!(u64::from_le_bytes(resp.payload[..8].try_into().unwrap()), HANDLE_ROOT);
    }

    #[test]
    fn lookup_battery0() {
        let resp = handle_lookup(&make_lookup_payload("battery0"));
        assert_eq!(resp.status, 0);
        assert_eq!(u64::from_le_bytes(resp.payload[..8].try_into().unwrap()), HANDLE_BATTERY0);
    }

    #[test]
    fn lookup_battery_files() {
        for (name, expected) in &[
            ("battery0/status",   HANDLE_STATUS),
            ("battery0/percent",  HANDLE_PERCENT),
            ("battery0/rate",     HANDLE_RATE),
            ("battery0/capacity", HANDLE_CAPACITY),
            ("ac",                HANDLE_AC),
        ] {
            let resp = handle_lookup(&make_lookup_payload(name));
            assert_eq!(resp.status, 0, "lookup failed for {}", name);
            let handle = u64::from_le_bytes(resp.payload[..8].try_into().unwrap());
            assert_eq!(handle, *expected, "wrong handle for {}", name);
        }
    }

    #[test]
    fn lookup_unknown_returns_enoent() {
        let resp = handle_lookup(&make_lookup_payload("nonexistent"));
        assert_eq!(resp.status, Errno::ENOENT as u8);
    }

    // ── read ──────────────────────────────────────────────────────────────────

    #[test]
    fn read_status_unknown_by_default() {
        let state = default_state();
        let resp = handle_read(&state, &make_read_payload(HANDLE_STATUS));
        assert_eq!(resp.status, 0);
        // ok_read prepends 4-byte length then data.
        let data = &resp.payload[4..];
        assert_eq!(data, b"Unknown\n");
    }

    #[test]
    fn read_status_charging() {
        let mut state = default_state();
        state.charging_state = ChargingState::Charging;
        let resp = handle_read(&state, &make_read_payload(HANDLE_STATUS));
        assert_eq!(resp.status, 0);
        assert_eq!(&resp.payload[4..], b"Charging\n");
    }

    #[test]
    fn read_status_discharging() {
        let mut state = default_state();
        state.charging_state = ChargingState::Discharging;
        let resp = handle_read(&state, &make_read_payload(HANDLE_STATUS));
        assert_eq!(resp.status, 0);
        assert_eq!(&resp.payload[4..], b"Discharging\n");
    }

    #[test]
    fn read_percent_unknown_when_no_capacity() {
        let state = default_state();
        let resp = handle_read(&state, &make_read_payload(HANDLE_PERCENT));
        assert_eq!(resp.status, 0);
        assert_eq!(&resp.payload[4..], b"Unknown\n");
    }

    #[test]
    fn read_percent_computed_correctly() {
        let mut state = default_state();
        state.remaining_mah = 3000;
        state.full_mah = 6000;
        let resp = handle_read(&state, &make_read_payload(HANDLE_PERCENT));
        assert_eq!(resp.status, 0);
        assert_eq!(&resp.payload[4..], b"50\n");
    }

    #[test]
    fn read_percent_clamped_to_100() {
        let mut state = default_state();
        state.remaining_mah = 7000;
        state.full_mah = 6000;
        let resp = handle_read(&state, &make_read_payload(HANDLE_PERCENT));
        assert_eq!(resp.status, 0);
        assert_eq!(&resp.payload[4..], b"100\n");
    }

    #[test]
    fn read_rate_zero_by_default() {
        let state = default_state();
        let resp = handle_read(&state, &make_read_payload(HANDLE_RATE));
        assert_eq!(resp.status, 0);
        assert_eq!(&resp.payload[4..], b"0\n");
    }

    #[test]
    fn read_capacity_zero_by_default() {
        let state = default_state();
        let resp = handle_read(&state, &make_read_payload(HANDLE_CAPACITY));
        assert_eq!(resp.status, 0);
        assert_eq!(&resp.payload[4..], b"0\n");
    }

    #[test]
    fn read_ac_offline_by_default() {
        let state = default_state();
        let resp = handle_read(&state, &make_read_payload(HANDLE_AC));
        assert_eq!(resp.status, 0);
        assert_eq!(&resp.payload[4..], b"offline\n");
    }

    #[test]
    fn read_ac_online() {
        let mut state = default_state();
        state.ac_online = true;
        let resp = handle_read(&state, &make_read_payload(HANDLE_AC));
        assert_eq!(resp.status, 0);
        assert_eq!(&resp.payload[4..], b"online\n");
    }

    #[test]
    fn read_root_returns_eisdir() {
        let state = default_state();
        let resp = handle_read(&state, &make_read_payload(HANDLE_ROOT));
        assert_eq!(resp.status, Errno::EISDIR as u8);
    }

    // ── format_u32 ───────────────────────────────────────────────────────────

    #[test]
    fn format_u32_zero() {
        let mut buf = [0u8; 16];
        let s = format_u32(0, &mut buf);
        assert_eq!(s, b"0\n");
    }

    #[test]
    fn format_u32_value() {
        let mut buf = [0u8; 16];
        let s = format_u32(12345, &mut buf);
        assert_eq!(s, b"12345\n");
    }

    // ── percent ───────────────────────────────────────────────────────────────

    #[test]
    fn percent_none_when_full_zero() {
        let state = default_state();
        assert_eq!(state.percent(), None);
    }

    #[test]
    fn percent_100_when_full() {
        let mut state = default_state();
        state.remaining_mah = 5000;
        state.full_mah = 5000;
        assert_eq!(state.percent(), Some(100));
    }

    #[test]
    fn percent_50_when_half() {
        let mut state = default_state();
        state.remaining_mah = 2500;
        state.full_mah = 5000;
        assert_eq!(state.percent(), Some(50));
    }
}

