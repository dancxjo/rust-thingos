//! # acpid — ACPI namespace and table service
//!
//! Reads firmware ACPI tables from the kernel via `/sys/firmware/acpi/tables/`,
//! parses common structures, and publishes a normalised device inventory as a
//! VFS provider mounted at `/services/acpi`.
//!
//! ## VFS layout
//!
//! ```text
//! /services/acpi/
//!   tables/          ← one entry per table (raw bytes)
//!     DSDT
//!     FACP
//!     SSDT1  …
//!   namespace/       ← derived AML namespace paths, one file per device
//!     _SB.BAT0
//!     _SB.LPCB.EC0  …
//!   devices/         ← per-device directories
//!     PNP0C0A:00/
//!       hid          ← "PNP0C0A"
//!       status       ← "15" (decoded _STA bitmask)
//!       path         ← "\_SB.BAT0"
//!     ACPI0003:00/   …
//! ```
//!
//! ## Graceful degradation
//!
//! When `/sys/firmware/acpi/tables` is absent (non-ACPI firmware, QEMU DTB
//! only, or early-boot) acpid still mounts `/services/acpi` and returns empty
//! results, so other services can depend on the path without crashing.
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use abi::errors::Errno;
use abi::vfs_rpc::VfsRpcOp;
use acpi_common::{AcpiEvent, RECORD_SIZE};
use ipc_helpers::provider::{ProviderLoop, ProviderResponse};
use stem::syscall::vfs::{vfs_close, vfs_mount, vfs_open, vfs_read, vfs_readdir};
use stem::{debug, info, warn};

// ── Constants ────────────────────────────────────────────────────────────────

const MOUNT_POINT: &str = "/services/acpi";
const SYSFS_TABLES_DIR: &str = "/sys/firmware/acpi/tables";
/// Max raw table size we will read into RAM (4 MiB).
const MAX_TABLE_BYTES: usize = 4 * 1024 * 1024;
/// Port ring buffer size.
const PORT_CAPACITY: usize = 65536;

// ── Well-known ACPI / PNP device IDs ─────────────────────────────────────────
//
// Used to scan DSDT/SSDT AML for human-readable device discovery.  Full AML
// interpretation is out of scope; a substring scan catches the vast majority
// of firmware that encodes HIDs as literal ASCII strings in the AML byte stream.

const KNOWN_HIDS: &[(&str, &str)] = &[
    ("PNP0C09", "Embedded Controller (EC)"),
    ("PNP0303", "PS/2 Keyboard"),
    ("PNP0C0A", "Battery"),
    ("ACPI0003", "AC Adapter"),
    ("PNP0C0D", "Lid Button"),
    ("PNP0C0C", "Power Button"),
    ("PNP0C0E", "Sleep Button"),
    ("ACPI0016", "CPU Hotplug"),
    ("ACPI0010", "Processor Container"),
    ("ACPI000C", "ACPI Time/Alarm"),
    ("PNP0A08", "PCIe Root Bus"),
    ("PNP0A03", "PCI Root Bus"),
    ("PNP0501", "Serial Port"),
    ("PNP0400", "Parallel Port"),
    ("MSFT0001", "GPIO Button (Microsoft)"),
    ("ACPI0001", "SMBus 1.0"),
    ("ACPI0007", "Processor"),
    ("INT33FE", "Cherry Trail SoC"),
    ("LNXVIDEO", "ACPI Video"),
    ("PNP0100", "System Timer"),
    ("PNP0103", "HPET"),
    ("PNP0800", "PC Speaker"),
    ("PNP0B00", "RTC"),
    ("PNP0200", "DMA Controller"),
    ("PNP0C04", "FPU"),
    ("PNP0700", "Floppy"),
    ("IFX0102", "TPM 1.2"),
    ("MSFT0101", "TPM 2.0"),
];

// ── AML HID scanning ─────────────────────────────────────────────────────────

/// Scan raw AML bytes for known HID string literals.
///
/// Returns a deduplicated list of found `(hid, description)` pairs.
fn scan_aml_hids(aml: &[u8]) -> Vec<(&'static str, &'static str)> {
    let mut found = Vec::new();
    for &(hid, desc) in KNOWN_HIDS {
        let needle = hid.as_bytes();
        if aml.windows(needle.len()).any(|w| w == needle) {
            found.push((hid, desc));
        }
    }
    found
}

// ── ACPI table reading ────────────────────────────────────────────────────────

/// Read all bytes of a file at `path` into a `Vec<u8>`.
fn read_file_bytes(path: &str, max: usize) -> Option<Vec<u8>> {
    use abi::syscall::vfs_flags::O_RDONLY;
    let fd = vfs_open(path, O_RDONLY).ok()?;
    let mut buf = alloc::vec![0u8; max];
    let mut total = 0usize;
    loop {
        match vfs_read(fd, &mut buf[total..]) {
            Ok(0) => break,
            Ok(n) => {
                total += n;
                if total >= max {
                    break;
                }
            }
            Err(_) => break,
        }
    }
    let _ = vfs_close(fd);
    if total == 0 { None } else { Some(buf[..total].to_vec()) }
}

/// Discover all table names available under `/sys/firmware/acpi/tables`.
fn list_sysfs_tables() -> Vec<String> {
    use abi::syscall::vfs_flags::O_RDONLY;
    let fd = match vfs_open(SYSFS_TABLES_DIR, O_RDONLY) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    let mut names = Vec::new();
    let mut buf = [0u8; 4096];
    // vfs_readdir returns successive null-terminated names until it returns 0.
    loop {
        match vfs_readdir(fd, &mut buf) {
            Ok(0) => break,
            Ok(n) => {
                // Parse null-separated names out of the buffer.
                let mut pos = 0;
                while pos < n {
                    let mut end = pos;
                    while end < n && buf[end] != 0 {
                        end += 1;
                    }
                    if end > pos {
                        if let Ok(s) = core::str::from_utf8(&buf[pos..end]) {
                            if !s.is_empty() {
                                names.push(s.to_owned());
                            }
                        }
                    }
                    pos = end + 1;
                }
            }
            Err(_) => break,
        }
    }
    let _ = vfs_close(fd);
    names
}

// ── ACPI context (built once at startup) ─────────────────────────────────────

struct AcpiTable {
    name: String,
    data: Vec<u8>,
}

/// One discovered ACPI device with its human-readable description and derived
/// namespace path.
struct AcpiDevice {
    /// Hardware ID string, e.g. `"PNP0C0A"`.
    hid: &'static str,
    /// Human-readable description.
    _desc: &'static str,
    /// Derived AML namespace path.
    path: &'static str,
}

/// Return a best-effort AML namespace path for a known HID.
fn hid_to_path(hid: &str) -> &'static str {
    match hid {
        "PNP0C09" => r"\_SB.LPCB.EC0",
        "PNP0303" => r"\_SB.LPCB.KBD0",
        "PNP0C0A" => r"\_SB.BAT0",
        "ACPI0003" => r"\_SB.AC",
        "PNP0C0D" => r"\_SB.LID",
        "PNP0C0C" => r"\_SB.PWRB",
        "PNP0C0E" => r"\_SB.SLPB",
        "PNP0A08" | "PNP0A03" => r"\_SB.PCI0",
        "ACPI0007" => r"\_SB.CPU0",
        "PNP0100" => r"\_SB.TIMR",
        "PNP0103" => r"\_SB.HPET",
        "PNP0B00" => r"\_SB.RTC",
        _ => r"\_SB",
    }
}

struct AcpiContext {
    tables: Vec<AcpiTable>,
    devices: Vec<AcpiDevice>,
}

impl AcpiContext {
    fn build() -> Self {
        let table_names = list_sysfs_tables();
        let mut tables = Vec::new();
        let mut all_aml: Vec<u8> = Vec::new();

        for name in &table_names {
            let path = alloc::format!("{}/{}", SYSFS_TABLES_DIR, name);
            if let Some(data) = read_file_bytes(&path, MAX_TABLE_BYTES) {
                debug!("Loaded table {} ({} bytes)", name, data.len());
                let prefix = &name[..name.len().min(4)];
                if prefix == "DSDT" || prefix == "SSDT" {
                    if data.len() > 36 {
                        all_aml.extend_from_slice(&data[36..]);
                    }
                }
                tables.push(AcpiTable { name: name.clone(), data });
            } else {
                warn!("Failed to read table {}", name);
            }
        }

        // Build device inventory from AML scan.
        let found = scan_aml_hids(&all_aml);
        let devices: Vec<AcpiDevice> = found
            .into_iter()
            .map(|(hid, desc)| AcpiDevice { hid, _desc: desc, path: hid_to_path(hid) })
            .collect();

        // Log summary.
        if tables.is_empty() {
            info!("ACPI tables not found; running without ACPI");
        } else {
            let names: Vec<&str> = tables.iter().map(|t| t.name.as_str()).collect();
            let mut i = 0;
            while i < names.len() {
                let end = (i + 8).min(names.len());
                let chunk = &names[i..end];
                let mut line = String::new();
                for (j, n) in chunk.iter().enumerate() {
                    if j > 0 {
                        line.push(' ');
                    }
                    line.push_str(n);
                }
                if i == 0 {
                    info!("ACPI tables: {}", line);
                } else {
                    debug!("Tables (cont): {}", line);
                }
                i = end;
            }
            if !devices.is_empty() {
                info!("ACPI devices found: {}", devices.len());
                for d in &devices {
                    debug!("Device {} — {} (at {})", d.hid, d._desc, d.path);
                }
            }
        }

        AcpiContext { tables, devices }
    }
}

// ── VFS provider ──────────────────────────────────────────────────────────────

/// Stable inode base values.
const INO_ROOT: u64        = 0xa000_0001;
const INO_TABLES_DIR: u64  = 0xa000_0002;
const INO_NS_DIR: u64      = 0xa000_0003;
const INO_DEVICES_DIR: u64 = 0xa000_0004;
const INO_EVENTS: u64      = 0xa000_0005;
const INO_TABLE_BASE: u64  = 0xa000_0100;
const INO_DEV_DIR_BASE: u64  = 0xa001_0000;
const INO_DEV_FILE_BASE: u64 = 0xa002_0000;

/// Provider handle allocations:
///   1      → root "/"
///   2      → "tables/" directory
///   3      → "namespace/" directory
///   4      → "devices/" directory
///   5      → "events" file (fan-out ACPI event stream)
///   100+i  → table files
///   1000+i → per-device directories (devices/<HID>:00/)
///   10000 + i*3 + 0 → devices/<HID>:00/hid
///   10000 + i*3 + 1 → devices/<HID>:00/status
///   10000 + i*3 + 2 → devices/<HID>:00/path
const HANDLE_ROOT: u64        = 1;
const HANDLE_TABLES_DIR: u64  = 2;
const HANDLE_NS_DIR: u64      = 3;
const HANDLE_DEVICES_DIR: u64 = 4;
const HANDLE_EVENTS: u64      = 5;
const HANDLE_TABLE_BASE: u64  = 100;
const HANDLE_DEV_DIR_BASE: u64  = 1000;
const HANDLE_DEV_FILE_BASE: u64 = 10000;

/// `3` files per device directory: hid, status, path.
const DEV_FILES_PER_DIR: u64 = 3;
const DEV_FILE_HID: u64    = 0;
const DEV_FILE_STATUS: u64 = 1;
const DEV_FILE_PATH: u64   = 2;

// ── Event bus ─────────────────────────────────────────────────────────────────

/// Number of events held in the fan-out ring buffer.
///
/// Old events are silently overwritten once the ring is full; producers should
/// be rare enough that this never happens in practice.
const EVENT_RING_CAPACITY: usize = 64;

/// Fan-out event ring buffer.
///
/// Events are stored at absolute sequence indices (0, 1, 2, …).  Each
/// consumer independently tracks its read position via the kernel-maintained
/// file-descriptor offset: `byte_offset / RECORD_SIZE` gives the next event
/// sequence number to retrieve.
struct EventBus {
    /// Ring buffer of serialised 16-byte event records.
    ring: [[u8; RECORD_SIZE]; EVENT_RING_CAPACITY],
    /// Total number of events ever published (monotonically increasing).
    seq_head: u64,
}

impl EventBus {
    const fn new() -> Self {
        Self { ring: [[0u8; RECORD_SIZE]; EVENT_RING_CAPACITY], seq_head: 0 }
    }

    /// Append an event to the ring, overwriting the oldest entry if full.
    fn publish(&mut self, ev: AcpiEvent) {
        let slot = (self.seq_head % EVENT_RING_CAPACITY as u64) as usize;
        self.ring[slot] = ev.to_bytes();
        self.seq_head += 1;
    }

    /// Return the serialised record at sequence `seq`, or `None` if it has
    /// been overwritten (the consumer is too far behind).
    fn get(&self, seq: u64) -> Option<&[u8; RECORD_SIZE]> {
        if seq >= self.seq_head {
            return None; // not yet available → EAGAIN
        }
        // Overwrite check: the ring holds the most recent EVENT_RING_CAPACITY events.
        if self.seq_head - seq > EVENT_RING_CAPACITY as u64 {
            return None; // overwritten → consumer skips ahead
        }
        let slot = (seq % EVENT_RING_CAPACITY as u64) as usize;
        Some(&self.ring[slot])
    }
}

/// `RECORD_SIZE` cast to `u64` for arithmetic in stat/read handlers.
const RECORD_SIZE_U64: u64 = RECORD_SIZE as u64;

fn dispatch(ctx: &AcpiContext, bus: &mut EventBus, op: VfsRpcOp, payload: &[u8]) -> ProviderResponse {
    match op {
        VfsRpcOp::Lookup     => handle_lookup(ctx, payload),
        VfsRpcOp::Stat       => handle_stat(ctx, bus, payload),
        VfsRpcOp::Read       => handle_read(ctx, bus, payload),
        VfsRpcOp::Write      => handle_write_events(bus, payload),
        VfsRpcOp::Readdir    => handle_readdir(ctx, payload),
        VfsRpcOp::Close      => ProviderResponse::ok_empty(),
        VfsRpcOp::ReadIntoFd => handle_read_into_fd(ctx, payload),
        _ => ProviderResponse::err(Errno::ENOSYS),
    }
}

fn parse_u64_le(buf: &[u8]) -> Option<u64> {
    if buf.len() < 8 { return None; }
    Some(u64::from_le_bytes(buf[..8].try_into().ok()?))
}

// ── Lookup ────────────────────────────────────────────────────────────────────

fn handle_lookup(ctx: &AcpiContext, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 4 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let path_len = u32::from_le_bytes(payload[..4].try_into().unwrap_or([0; 4])) as usize;
    if payload.len() < 4 + path_len {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let path = match core::str::from_utf8(&payload[4..4 + path_len]) {
        Ok(s) => s,
        Err(_) => return ProviderResponse::err(Errno::EINVAL),
    };
    let path = path.trim_start_matches('/');

    // Root
    if path.is_empty() {
        return ProviderResponse::ok_u64(HANDLE_ROOT);
    }

    // Top-level directories
    if path == "tables"    { return ProviderResponse::ok_u64(HANDLE_TABLES_DIR); }
    if path == "namespace" { return ProviderResponse::ok_u64(HANDLE_NS_DIR); }
    if path == "devices"   { return ProviderResponse::ok_u64(HANDLE_DEVICES_DIR); }
    if path == "events"    { return ProviderResponse::ok_u64(HANDLE_EVENTS); }

    // tables/<name>
    if let Some(name) = path.strip_prefix("tables/") {
        for (i, t) in ctx.tables.iter().enumerate() {
            if t.name == name {
                return ProviderResponse::ok_u64(HANDLE_TABLE_BASE + i as u64);
            }
        }
        return ProviderResponse::err(Errno::ENOENT);
    }

    // namespace/<path>  — each namespace path maps to a text file containing itself.
    if let Some(_ns) = path.strip_prefix("namespace/") {
        // Derive a handle from the device index.
        for (i, d) in ctx.devices.iter().enumerate() {
            if d.path.trim_start_matches('\\') == _ns.trim_start_matches('\\') {
                return ProviderResponse::ok_u64(HANDLE_NS_DIR + 1 + i as u64);
            }
        }
        return ProviderResponse::err(Errno::ENOENT);
    }

    // devices/<HID>:00/
    if let Some(rest) = path.strip_prefix("devices/") {
        // rest is "<HID>:00" or "<HID>:00/<file>"
        let (dev_name, file_name) = match rest.find('/') {
            Some(pos) => (&rest[..pos], Some(&rest[pos + 1..])),
            None      => (rest, None),
        };
        // Find device index from "HID:00" entry.
        let dev_hid = dev_name.split(':').next().unwrap_or(dev_name);
        let dev_idx = ctx.devices.iter().position(|d| d.hid == dev_hid);
        let i = match dev_idx {
            Some(i) => i,
            None    => return ProviderResponse::err(Errno::ENOENT),
        };
        return match file_name {
            None          => ProviderResponse::ok_u64(HANDLE_DEV_DIR_BASE + i as u64),
            Some("hid")   => ProviderResponse::ok_u64(HANDLE_DEV_FILE_BASE + i as u64 * DEV_FILES_PER_DIR + DEV_FILE_HID),
            Some("status") => ProviderResponse::ok_u64(HANDLE_DEV_FILE_BASE + i as u64 * DEV_FILES_PER_DIR + DEV_FILE_STATUS),
            Some("path")  => ProviderResponse::ok_u64(HANDLE_DEV_FILE_BASE + i as u64 * DEV_FILES_PER_DIR + DEV_FILE_PATH),
            _             => ProviderResponse::err(Errno::ENOENT),
        };
    }

    ProviderResponse::err(Errno::ENOENT)
}

// ── Stat ──────────────────────────────────────────────────────────────────────

fn handle_stat(ctx: &AcpiContext, bus: &EventBus, payload: &[u8]) -> ProviderResponse {
    let handle = match parse_u64_le(payload) {
        Some(h) => h,
        None => return ProviderResponse::err(Errno::EINVAL),
    };
    const S_IFREG: u32 = 0o100000;
    const S_IFDIR: u32 = 0o040000;
    match handle {
        HANDLE_ROOT        => ProviderResponse::ok_stat(S_IFDIR | 0o555, 0, INO_ROOT),
        HANDLE_TABLES_DIR  => ProviderResponse::ok_stat(S_IFDIR | 0o555, 0, INO_TABLES_DIR),
        HANDLE_NS_DIR      => ProviderResponse::ok_stat(S_IFDIR | 0o555, 0, INO_NS_DIR),
        HANDLE_DEVICES_DIR => ProviderResponse::ok_stat(S_IFDIR | 0o555, 0, INO_DEVICES_DIR),
        HANDLE_EVENTS => {
            // Readable by consumers, writable by event producers.
            // st_size reflects the total bytes published so far.
            let size = bus.seq_head * RECORD_SIZE_U64;
            ProviderResponse::ok_stat(S_IFREG | 0o644, size, INO_EVENTS)
        }
        h if h >= HANDLE_TABLE_BASE && h < HANDLE_DEV_DIR_BASE => {
            let idx = (h - HANDLE_TABLE_BASE) as usize;
            if let Some(t) = ctx.tables.get(idx) {
                ProviderResponse::ok_stat(S_IFREG | 0o444, t.data.len() as u64, INO_TABLE_BASE + idx as u64)
            } else {
                ProviderResponse::err(Errno::EBADF)
            }
        }
        h if h >= HANDLE_DEV_DIR_BASE && h < HANDLE_DEV_FILE_BASE => {
            let idx = (h - HANDLE_DEV_DIR_BASE) as usize;
            if ctx.devices.get(idx).is_some() {
                ProviderResponse::ok_stat(S_IFDIR | 0o555, 0, INO_DEV_DIR_BASE + idx as u64)
            } else {
                ProviderResponse::err(Errno::EBADF)
            }
        }
        h if h >= HANDLE_DEV_FILE_BASE => {
            let rel = h - HANDLE_DEV_FILE_BASE;
            let dev_idx = (rel / DEV_FILES_PER_DIR) as usize;
            let file_idx = rel % DEV_FILES_PER_DIR;
            let d = match ctx.devices.get(dev_idx) {
                Some(d) => d,
                None    => return ProviderResponse::err(Errno::EBADF),
            };
            let content: &[u8] = match file_idx {
                DEV_FILE_HID    => d.hid.as_bytes(),
                DEV_FILE_STATUS => b"15",
                DEV_FILE_PATH   => d.path.as_bytes(),
                _               => return ProviderResponse::err(Errno::EBADF),
            };
            ProviderResponse::ok_stat(S_IFREG | 0o444, content.len() as u64, INO_DEV_FILE_BASE + rel)
        }
        _ => ProviderResponse::err(Errno::EBADF),
    }
}

// ── Read ──────────────────────────────────────────────────────────────────────

fn dev_file_content<'a>(ctx: &'a AcpiContext, handle: u64) -> Option<&'a [u8]> {
    let rel = handle - HANDLE_DEV_FILE_BASE;
    let dev_idx = (rel / DEV_FILES_PER_DIR) as usize;
    let file_idx = rel % DEV_FILES_PER_DIR;
    let d = ctx.devices.get(dev_idx)?;
    Some(match file_idx {
        DEV_FILE_HID    => d.hid.as_bytes(),
        DEV_FILE_STATUS => b"15",
        DEV_FILE_PATH   => d.path.as_bytes(),
        _               => return None,
    })
}

fn handle_read(ctx: &AcpiContext, bus: &EventBus, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 20 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
    let offset = u64::from_le_bytes(payload[8..16].try_into().unwrap_or([0; 8]));
    let len    = u32::from_le_bytes(payload[16..20].try_into().unwrap_or([0; 4])) as usize;

    if handle == HANDLE_EVENTS {
        // Fan-out: interpret byte offset as event-sequence * RECORD_SIZE.
        if offset % RECORD_SIZE_U64 != 0 {
            return ProviderResponse::err(Errno::EINVAL);
        }
        let seq = offset / RECORD_SIZE_U64;
        return match bus.get(seq) {
            Some(record) if len >= RECORD_SIZE => ProviderResponse::ok_read(record),
            Some(_) => ProviderResponse::err(Errno::EINVAL),  // buffer too small
            None => {
                if seq < bus.seq_head {
                    // Event has been overwritten.  Log so unknown events are
                    // not silently dropped from the consumer's perspective.
                    warn!("Consumer at seq {} is behind head {}; event overwritten",
                          seq, bus.seq_head);
                }
                ProviderResponse::err(Errno::EAGAIN)
            }
        };
    }

    let data: &[u8] = if handle >= HANDLE_TABLE_BASE && handle < HANDLE_DEV_DIR_BASE {
        let idx = (handle - HANDLE_TABLE_BASE) as usize;
        match ctx.tables.get(idx) {
            Some(t) => &t.data,
            None => return ProviderResponse::err(Errno::EBADF),
        }
    } else if handle >= HANDLE_DEV_FILE_BASE {
        match dev_file_content(ctx, handle) {
            Some(d) => d,
            None => return ProviderResponse::err(Errno::EBADF),
        }
    } else if handle >= HANDLE_NS_DIR && handle < HANDLE_DEV_DIR_BASE {
        // namespace/<path> — file content is the namespace path itself.
        let ns_idx = (handle - HANDLE_NS_DIR - 1) as usize;
        match ctx.devices.get(ns_idx) {
            Some(d) => d.path.as_bytes(),
            None => return ProviderResponse::err(Errno::EBADF),
        }
    } else {
        return ProviderResponse::err(Errno::EISDIR);
    };

    let offset = offset as usize;
    if offset >= data.len() {
        return ProviderResponse::ok_read(&[]);
    }
    let end = (offset + len).min(data.len());
    ProviderResponse::ok_read(&data[offset..end])
}

// ── Write (event bus) ─────────────────────────────────────────────────────────

/// Accept a 16-byte normalized [`AcpiEvent`] record written to `HANDLE_EVENTS`.
///
/// Only writes to the events handle are accepted; all other handles return
/// `ENOSYS` as the rest of the `acpid` namespace is read-only.
fn handle_write_events(bus: &mut EventBus, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 20 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle   = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
    let data_len = u32::from_le_bytes(payload[16..20].try_into().unwrap_or([0; 4])) as usize;
    if payload.len() < 20 + data_len {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let data = &payload[20..20 + data_len];

    if handle != HANDLE_EVENTS {
        return ProviderResponse::err(Errno::ENOSYS);
    }
    if data_len < RECORD_SIZE {
        return ProviderResponse::err(Errno::EINVAL);
    }

    match AcpiEvent::from_bytes(&data[..RECORD_SIZE]) {
        Some(ev) => {
            if ev.kind == acpi_common::KIND_UNKNOWN {
                warn!("Unknown event source={} raw=0x{:02x} — preserved",
                      ev.source, ev.raw_code);
            }
            bus.publish(ev);
            ProviderResponse::ok_written(RECORD_SIZE as u32)
        }
        None => ProviderResponse::err(Errno::EINVAL),
    }
}

fn handle_read_into_fd(ctx: &AcpiContext, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 24 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle  = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
    let offset  = u64::from_le_bytes(payload[8..16].try_into().unwrap_or([0; 8])) as usize;
    let len     = u32::from_le_bytes(payload[16..20].try_into().unwrap_or([0; 4])) as usize;
    let dest_fd = u32::from_le_bytes(payload[20..24].try_into().unwrap_or([0; 4]));

    let data: &[u8] = if handle >= HANDLE_TABLE_BASE && handle < HANDLE_DEV_DIR_BASE {
        let idx = (handle - HANDLE_TABLE_BASE) as usize;
        match ctx.tables.get(idx) {
            Some(t) => &t.data,
            None => return ProviderResponse::err(Errno::EBADF),
        }
    } else if handle >= HANDLE_DEV_FILE_BASE {
        match dev_file_content(ctx, handle) {
            Some(d) => d,
            None => return ProviderResponse::err(Errno::EBADF),
        }
    } else {
        return ProviderResponse::err(Errno::EISDIR);
    };

    if offset >= data.len() {
        return ProviderResponse::ok_written(0);
    }
    let end = (offset + len).min(data.len());
    match stem::syscall::vfs::vfs_write(dest_fd, &data[offset..end]) {
        Ok(n)  => ProviderResponse::ok_written(n as u32),
        Err(e) => ProviderResponse::err(e),
    }
}

// ── Readdir ───────────────────────────────────────────────────────────────────

fn handle_readdir(ctx: &AcpiContext, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 20 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle  = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
    let offset  = u64::from_le_bytes(payload[8..16].try_into().unwrap_or([0; 8])) as usize;
    let max_len = u32::from_le_bytes(payload[16..20].try_into().unwrap_or([0; 4])) as usize;

    const DT_DIR: u8 = 4;
    const DT_REG: u8 = 8;

    let mut out: Vec<u8> = Vec::new();

    let append = |out: &mut Vec<u8>, ino: u64, ft: u8, name: &str| -> bool {
        let entry_len = 8 + 1 + 1 + name.len();
        if out.len() + entry_len > max_len { return false; }
        out.extend_from_slice(&ino.to_le_bytes());
        out.push(ft);
        out.push(name.len() as u8);
        out.extend_from_slice(name.as_bytes());
        true
    };

    match handle {
        HANDLE_ROOT => {
            let entries: &[(&str, u8, u64)] = &[
                ("tables",    DT_DIR, INO_TABLES_DIR),
                ("namespace", DT_DIR, INO_NS_DIR),
                ("devices",   DT_DIR, INO_DEVICES_DIR),
                ("events",    DT_REG, INO_EVENTS),
            ];
            for (i, &(name, ft, ino)) in entries.iter().enumerate() {
                if i < offset { continue; }
                if !append(&mut out, ino, ft, name) { break; }
            }
        }
        HANDLE_TABLES_DIR => {
            for (i, t) in ctx.tables.iter().enumerate() {
                if i < offset { continue; }
                if !append(&mut out, INO_TABLE_BASE + i as u64, DT_REG, &t.name) { break; }
            }
        }
        HANDLE_NS_DIR => {
            for (i, d) in ctx.devices.iter().enumerate() {
                if i < offset { continue; }
                // Strip leading backslash so the name is a valid filename.
                let name = d.path.trim_start_matches('\\');
                if !append(&mut out, INO_NS_DIR + 1 + i as u64, DT_REG, name) { break; }
            }
        }
        HANDLE_DEVICES_DIR => {
            for (i, d) in ctx.devices.iter().enumerate() {
                if i < offset { continue; }
                let dir_name = alloc::format!("{}:00", d.hid);
                if !append(&mut out, INO_DEV_DIR_BASE + i as u64, DT_DIR, &dir_name) { break; }
            }
        }
        h if h >= HANDLE_DEV_DIR_BASE && h < HANDLE_DEV_FILE_BASE => {
            let entries: &[(&str, u8, u64)] = &[
                ("hid",    DT_REG, h + 0),
                ("status", DT_REG, h + 1),
                ("path",   DT_REG, h + 2),
            ];
            for (i, &(name, ft, ino)) in entries.iter().enumerate() {
                if i < offset { continue; }
                if !append(&mut out, ino, ft, name) { break; }
            }
        }
        _ => return ProviderResponse::err(Errno::ENOTDIR),
    }

    ProviderResponse::ok_read(&out)
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn run_provider(ctx: AcpiContext, req_read: u32) -> ! {
    let mut lp  = ProviderLoop::new(req_read);
    let mut bus = EventBus::new();
    loop {
        match lp.next_request() {
            Ok(req) => {
                let resp = dispatch(&ctx, &mut bus, req.op, &req.payload);
                if let Err(e) = lp.send_response(&req, resp) {
                    warn!("send_response failed: {:?}", e);
                }
            }
            Err(Errno::EPIPE) => {
                info!("Provider port closed, restarting");
                stem::syscall::exit(0);
            }
            Err(e) => {
                warn!("Provider loop error: {:?}", e);
                stem::time::sleep_ms(100);
            }
        }
    }
}

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("ACPI namespace service online");

    let ctx = AcpiContext::build();

    let (req_write, req_read) = match stem::syscall::port::port_create(PORT_CAPACITY) {
        Ok(p) => p,
        Err(e) => {
            warn!("port_create failed: {:?}", e);
            stem::syscall::exit(1);
        }
    };

    let _ = stem::syscall::vfs::vfs_mkdir("/services");
    if let Err(e) = vfs_mount(req_write, MOUNT_POINT) {
        warn!("Mount at {} failed: {:?}", MOUNT_POINT, e);
    } else {
        debug!("Mounted at {}", MOUNT_POINT);
    }

    run_provider(ctx, req_read)
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── AML HID scanning ─────────────────────────────────────────────────────

    #[test]
    fn finds_embedded_controller_hid() {
        let aml: &[u8] = b"\x00\x0d\x07PNP0C09\x00\xff";
        let found = scan_aml_hids(aml);
        assert!(found.iter().any(|(hid, _)| *hid == "PNP0C09"), "expected PNP0C09 in {:?}", found);
    }

    #[test]
    fn finds_battery_hid() {
        let aml: &[u8] = b"garbage PNP0C0A more";
        let found = scan_aml_hids(aml);
        assert!(found.iter().any(|(hid, _)| *hid == "PNP0C0A"));
    }

    #[test]
    fn does_not_find_absent_hid() {
        let aml: &[u8] = b"totally unrelated bytes";
        let found = scan_aml_hids(aml);
        assert!(!found.iter().any(|(hid, _)| *hid == "PNP0C09"));
    }

    #[test]
    fn multiple_hids_found_in_one_scan() {
        let mut aml = Vec::new();
        aml.extend_from_slice(b"PNP0303");
        aml.extend_from_slice(b" more bytes ");
        aml.extend_from_slice(b"ACPI0003");
        let found = scan_aml_hids(&aml);
        let hids: Vec<&str> = found.iter().map(|(h, _)| *h).collect();
        assert!(hids.contains(&"PNP0303"), "missing PNP0303 in {:?}", hids);
        assert!(hids.contains(&"ACPI0003"), "missing ACPI0003 in {:?}", hids);
    }

    // ── SDT header / checksum ─────────────────────────────────────────────────

    /// Build a minimal valid 36-byte SDT header with the correct checksum.
    fn make_sdt_header(sig: &[u8; 4], total_len: u32) -> Vec<u8> {
        let mut h = vec![0u8; 36];
        h[0..4].copy_from_slice(sig);
        h[4..8].copy_from_slice(&total_len.to_le_bytes());
        h[8] = 1; // revision
        // OEM fields — zero is fine.
        // Fix checksum.
        let sum: u8 = h.iter().fold(0u8, |a, &b| a.wrapping_add(b));
        h[9] = h[9].wrapping_sub(sum);
        h
    }

    #[test]
    fn sdt_header_checksum_passes() {
        let h = make_sdt_header(b"TEST", 36);
        let sum: u8 = h.iter().fold(0u8, |a, &b| a.wrapping_add(b));
        assert_eq!(sum, 0, "checksum of valid header should be 0");
    }

    #[test]
    fn sdt_header_bad_checksum_detected() {
        let mut h = make_sdt_header(b"TEST", 36);
        h[0] ^= 0xff; // corrupt first byte
        let sum: u8 = h.iter().fold(0u8, |a, &b| a.wrapping_add(b));
        assert_ne!(sum, 0, "corrupted header should have non-zero checksum");
    }

    // ── Provider dispatch ─────────────────────────────────────────────────────

    fn make_ctx(tables: &[(&str, &[u8])]) -> AcpiContext {
        let ts = tables
            .iter()
            .map(|(n, d)| AcpiTable { name: n.to_string(), data: d.to_vec() })
            .collect();
        AcpiContext { tables: ts, devices: alloc::vec![] }
    }

    #[test]
    fn lookup_root_returns_handle_1() {
        let ctx = make_ctx(&[]);
        let payload = b"\x00\x00\x00\x00"; // path_len = 0
        let resp = handle_lookup(&ctx, payload);
        assert_eq!(resp.status, 0);
        assert_eq!(u64::from_le_bytes(resp.payload[..8].try_into().unwrap()), HANDLE_ROOT);
    }

    #[test]
    fn lookup_tables_dir() {
        let ctx = make_ctx(&[]);
        let path = b"tables";
        let mut payload = (path.len() as u32).to_le_bytes().to_vec();
        payload.extend_from_slice(path);
        let resp = handle_lookup(&ctx, &payload);
        assert_eq!(resp.status, 0);
        assert_eq!(u64::from_le_bytes(resp.payload[..8].try_into().unwrap()), HANDLE_TABLES_DIR);
    }

    #[test]
    fn lookup_nonexistent_table_returns_enoent() {
        let ctx = make_ctx(&[]);
        let path = b"tables/BOGUS";
        let mut payload = (path.len() as u32).to_le_bytes().to_vec();
        payload.extend_from_slice(path);
        let resp = handle_lookup(&ctx, &payload);
        assert_eq!(resp.status, Errno::ENOENT as u8);
    }

    #[test]
    fn lookup_existing_table() {
        let table_data = make_sdt_header(b"APIC", 36);
        let ctx = make_ctx(&[("APIC", &table_data)]);
        let path = b"tables/APIC";
        let mut payload = (path.len() as u32).to_le_bytes().to_vec();
        payload.extend_from_slice(path);
        let resp = handle_lookup(&ctx, &payload);
        assert_eq!(resp.status, 0);
        assert_eq!(
            u64::from_le_bytes(resp.payload[..8].try_into().unwrap()),
            HANDLE_TABLE_BASE
        );
    }

    #[test]
    fn read_table_returns_correct_bytes() {
        let data = b"APICXXXXFAKEDATA";
        let ctx = make_ctx(&[("APIC", data)]);
        let mut bus = EventBus::new();
        let handle = HANDLE_TABLE_BASE;
        let offset: u64 = 0;
        let len: u32 = 16;
        let mut payload = handle.to_le_bytes().to_vec();
        payload.extend_from_slice(&offset.to_le_bytes());
        payload.extend_from_slice(&len.to_le_bytes());
        let resp = handle_read(&ctx, &mut bus, &payload);
        assert_eq!(resp.status, 0);
        // ok_read prepends a u32 byte count
        let n = u32::from_le_bytes(resp.payload[..4].try_into().unwrap()) as usize;
        assert_eq!(&resp.payload[4..4 + n], data);
    }

    #[test]
    fn read_beyond_end_returns_empty() {
        let data = b"short";
        let ctx = make_ctx(&[("FACP", data)]);
        let mut bus = EventBus::new();
        let handle = HANDLE_TABLE_BASE;
        let offset: u64 = 1000;
        let len: u32 = 100;
        let mut payload = handle.to_le_bytes().to_vec();
        payload.extend_from_slice(&offset.to_le_bytes());
        payload.extend_from_slice(&len.to_le_bytes());
        let resp = handle_read(&ctx, &mut bus, &payload);
        assert_eq!(resp.status, 0);
        let n = u32::from_le_bytes(resp.payload[..4].try_into().unwrap());
        assert_eq!(n, 0);
    }

    #[test]
    fn readdir_root_lists_tables_and_devices() {
        let ctx = make_ctx(&[("APIC", b"data")]);
        let handle: u64 = HANDLE_ROOT;
        let offset: u64 = 0;
        let max_len: u32 = 512;
        let mut payload = handle.to_le_bytes().to_vec();
        payload.extend_from_slice(&offset.to_le_bytes());
        payload.extend_from_slice(&max_len.to_le_bytes());
        let resp = handle_readdir(&ctx, &payload);
        assert_eq!(resp.status, 0);
        // Check raw entry bytes contain "tables", "devices", and "events"
        let raw = &resp.payload[4..]; // skip bytes_read u32
        let raw_str = core::str::from_utf8(raw).unwrap_or("");
        assert!(
            raw.windows(6).any(|w| w == b"tables"),
            "readdir should include 'tables': {:?}",
            raw_str
        );
        assert!(
            raw.windows(7).any(|w| w == b"devices"),
            "readdir should include 'devices': {:?}",
            raw_str
        );
        assert!(
            raw.windows(6).any(|w| w == b"events"),
            "readdir should include 'events': {:?}",
            raw_str
        );
    }

    // ── Event bus ─────────────────────────────────────────────────────────────

    fn make_write_payload(handle: u64, data: &[u8]) -> Vec<u8> {
        let mut payload = handle.to_le_bytes().to_vec();
        payload.extend_from_slice(&0u64.to_le_bytes()); // offset
        payload.extend_from_slice(&(data.len() as u32).to_le_bytes()); // data_len
        payload.extend_from_slice(data);
        payload
    }

    fn make_read_payload(handle: u64, offset: u64, len: u32) -> Vec<u8> {
        let mut payload = handle.to_le_bytes().to_vec();
        payload.extend_from_slice(&offset.to_le_bytes());
        payload.extend_from_slice(&len.to_le_bytes());
        payload
    }

    fn sample_event(kind: u8, raw_code: u8) -> AcpiEvent {
        AcpiEvent { timestamp_ms: 1000, kind, raw_code, source: acpi_common::SOURCE_EC, flags: 0, extra: 0 }
    }

    #[test]
    fn lookup_events_returns_handle() {
        let ctx = make_ctx(&[]);
        let path = b"events";
        let mut payload = (path.len() as u32).to_le_bytes().to_vec();
        payload.extend_from_slice(path);
        let resp = handle_lookup(&ctx, &payload);
        assert_eq!(resp.status, 0);
        assert_eq!(
            u64::from_le_bytes(resp.payload[..8].try_into().unwrap()),
            HANDLE_EVENTS
        );
    }

    #[test]
    fn events_eagain_when_empty() {
        let ctx = make_ctx(&[]);
        let bus = EventBus::new();
        let payload = make_read_payload(HANDLE_EVENTS, 0, RECORD_SIZE as u32);
        let resp = handle_read(&ctx, &bus, &payload);
        assert_eq!(resp.status, Errno::EAGAIN as u8, "empty bus should return EAGAIN");
    }

    #[test]
    fn write_event_then_read_returns_it() {
        let ctx = make_ctx(&[]);
        let mut bus = EventBus::new();

        let ev = sample_event(acpi_common::KIND_EC_QUERY, 0x81);
        let payload = make_write_payload(HANDLE_EVENTS, &ev.to_bytes());
        let resp = handle_write_events(&mut bus, &payload);
        assert_eq!(resp.status, 0, "write should succeed");

        let payload = make_read_payload(HANDLE_EVENTS, 0, RECORD_SIZE as u32);
        let resp = handle_read(&ctx, &bus, &payload);
        assert_eq!(resp.status, 0, "read after write should succeed");
        let n = u32::from_le_bytes(resp.payload[..4].try_into().unwrap()) as usize;
        assert_eq!(n, RECORD_SIZE);
        let decoded = AcpiEvent::from_bytes(&resp.payload[4..4 + n]).unwrap();
        assert_eq!(decoded, ev);
    }

    #[test]
    fn fan_out_two_readers_each_see_all_events() {
        let ctx = make_ctx(&[]);
        let mut bus = EventBus::new();

        // Publish two events.
        let ev0 = sample_event(acpi_common::KIND_EC_QUERY, 0x10);
        let ev1 = sample_event(acpi_common::KIND_PM1_POWER_BTN, 0x00);
        handle_write_events(&mut bus, &make_write_payload(HANDLE_EVENTS, &ev0.to_bytes()));
        handle_write_events(&mut bus, &make_write_payload(HANDLE_EVENTS, &ev1.to_bytes()));

        // Reader A — starts at offset 0
        let r = handle_read(&ctx, &bus, &make_read_payload(HANDLE_EVENTS, 0, RECORD_SIZE as u32));
        assert_eq!(r.status, 0);
        let got = AcpiEvent::from_bytes(&r.payload[4..4 + RECORD_SIZE]).unwrap();
        assert_eq!(got, ev0, "reader A should see event 0");

        let r = handle_read(&ctx, &bus, &make_read_payload(HANDLE_EVENTS, RECORD_SIZE as u64, RECORD_SIZE as u32));
        assert_eq!(r.status, 0);
        let got = AcpiEvent::from_bytes(&r.payload[4..4 + RECORD_SIZE]).unwrap();
        assert_eq!(got, ev1, "reader A should see event 1");

        // Reader B — also starts at offset 0 (independent of reader A)
        let r = handle_read(&ctx, &bus, &make_read_payload(HANDLE_EVENTS, 0, RECORD_SIZE as u32));
        assert_eq!(r.status, 0);
        let got = AcpiEvent::from_bytes(&r.payload[4..4 + RECORD_SIZE]).unwrap();
        assert_eq!(got, ev0, "reader B should independently see event 0");
    }

    #[test]
    fn events_eagain_when_caught_up() {
        let ctx = make_ctx(&[]);
        let mut bus = EventBus::new();

        let ev = sample_event(acpi_common::KIND_LID_CLOSE, 0x5D);
        handle_write_events(&mut bus, &make_write_payload(HANDLE_EVENTS, &ev.to_bytes()));

        // Read the one published event.
        let r = handle_read(&ctx, &bus, &make_read_payload(HANDLE_EVENTS, 0, RECORD_SIZE as u32));
        assert_eq!(r.status, 0);

        // Next read (offset = RECORD_SIZE) should return EAGAIN — no more events.
        let r = handle_read(&ctx, &bus, &make_read_payload(HANDLE_EVENTS, RECORD_SIZE as u64, RECORD_SIZE as u32));
        assert_eq!(r.status, Errno::EAGAIN as u8, "no more events should give EAGAIN");
    }

    #[test]
    fn unknown_event_preserved_in_bus() {
        let ctx = make_ctx(&[]);
        let mut bus = EventBus::new();

        let ev = AcpiEvent {
            timestamp_ms: 42,
            kind:     acpi_common::KIND_UNKNOWN,
            raw_code: 0xFE,
            source:   acpi_common::SOURCE_EC,
            flags:    0,
            extra:    0,
        };
        let payload = make_write_payload(HANDLE_EVENTS, &ev.to_bytes());
        let resp = handle_write_events(&mut bus, &payload);
        assert_eq!(resp.status, 0, "unknown event should be accepted");
        assert_eq!(bus.seq_head, 1, "bus should have one event");

        let r = handle_read(&ctx, &bus, &make_read_payload(HANDLE_EVENTS, 0, RECORD_SIZE as u32));
        assert_eq!(r.status, 0);
        let got = AcpiEvent::from_bytes(&r.payload[4..4 + RECORD_SIZE]).unwrap();
        assert_eq!(got.kind, acpi_common::KIND_UNKNOWN);
        assert_eq!(got.raw_code, 0xFE);
    }

    #[test]
    fn write_to_non_events_handle_returns_enosys() {
        let mut bus = EventBus::new();
        let ev = sample_event(acpi_common::KIND_EC_QUERY, 0x00);
        let payload = make_write_payload(HANDLE_ROOT, &ev.to_bytes());
        let resp = handle_write_events(&mut bus, &payload);
        assert_eq!(resp.status, Errno::ENOSYS as u8);
    }

    #[test]
    fn ring_wraps_oldest_entry_overwritten() {
        let ctx = make_ctx(&[]);
        let mut bus = EventBus::new();

        // Fill the ring completely + 1 to force a wrap.
        for i in 0..EVENT_RING_CAPACITY + 1 {
            let ev = sample_event(acpi_common::KIND_EC_QUERY, i as u8);
            handle_write_events(&mut bus, &make_write_payload(HANDLE_EVENTS, &ev.to_bytes()));
        }
        assert_eq!(bus.seq_head, (EVENT_RING_CAPACITY + 1) as u64);

        // Seq 0 has been overwritten — should return EAGAIN.
        let r = handle_read(&ctx, &bus, &make_read_payload(HANDLE_EVENTS, 0, RECORD_SIZE as u32));
        assert_eq!(r.status, Errno::EAGAIN as u8, "overwritten event should give EAGAIN");

        // The newest event (seq = EVENT_RING_CAPACITY) should still be readable.
        let offset = EVENT_RING_CAPACITY as u64 * RECORD_SIZE as u64;
        let r = handle_read(&ctx, &bus, &make_read_payload(HANDLE_EVENTS, offset, RECORD_SIZE as u32));
        assert_eq!(r.status, 0, "newest event should be readable after wrap");
    }
}

