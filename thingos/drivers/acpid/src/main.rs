//! # acpid — ACPI namespace and table service
//!
//! Reads firmware ACPI tables from the kernel via `/sys/firmware/acpi_tables/`,
//! parses common structures, and publishes a normalised device inventory as a
//! VFS provider mounted at `/services/acpi`.
//!
//! ## VFS layout
//!
//! ```text
//! /services/acpi/
//!   tables/          ← directory, one entry per table
//!     APIC           ← raw bytes of the MADT
//!     FACP           ← raw bytes of the FADT
//!     DSDT           ← raw bytes of the DSDT
//!     SSDT           ← first SSDT (SSDT1, SSDT2 … for duplicates)
//!     …
//!   devices          ← plain-text inventory of known ACPI device IDs
//! ```
//!
//! ## Graceful degradation
//!
//! When `/sys/firmware/acpi_tables` is absent (non-ACPI firmware, QEMU DTB
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
use ipc_helpers::provider::{ProviderLoop, ProviderResponse};
use stem::syscall::vfs::{vfs_close, vfs_mount, vfs_open, vfs_read, vfs_readdir};
use stem::{debug, info, warn};

// ── Constants ────────────────────────────────────────────────────────────────

const MOUNT_POINT: &str = "/services/acpi";
const SYSFS_TABLES_DIR: &str = "/sys/firmware/acpi_tables";
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

/// Discover all table names available under `/sys/firmware/acpi_tables`.
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

struct AcpiContext {
    tables: Vec<AcpiTable>,
    devices_text: String,
}

impl AcpiContext {
    fn build() -> Self {
        let table_names = list_sysfs_tables();
        let mut tables = Vec::new();
        let mut all_aml: Vec<u8> = Vec::new();

        for name in &table_names {
            let path = alloc::format!("{}/{}", SYSFS_TABLES_DIR, name);
            if let Some(data) = read_file_bytes(&path, MAX_TABLE_BYTES) {
                debug!("acpid: loaded table {} ({} bytes)", name, data.len());
                // Accumulate DSDT/SSDT AML for device scanning.
                let sig = name.trim_start_matches(|c: char| !c.is_ascii_alphabetic());
                if sig.starts_with("DSDT") || sig.starts_with("SSDT") {
                    // AML body starts after the 36-byte SDT header.
                    if data.len() > 36 {
                        all_aml.extend_from_slice(&data[36..]);
                    }
                }
                tables.push(AcpiTable { name: name.clone(), data });
            } else {
                warn!("acpid: failed to read table {}", name);
            }
        }

        // Build device inventory.
        let devices = scan_aml_hids(&all_aml);
        let mut devices_text = String::new();
        if devices.is_empty() && table_names.is_empty() {
            devices_text.push_str("# No ACPI tables found\n");
        } else if devices.is_empty() {
            devices_text.push_str("# No known device IDs found in AML\n");
        } else {
            for (hid, desc) in &devices {
                devices_text.push_str(hid);
                devices_text.push(' ');
                devices_text.push_str(desc);
                devices_text.push('\n');
            }
        }

        // Log summary.
        if tables.is_empty() {
            info!("ACPI tables not found; running without ACPI");
        } else {
            let names: Vec<&str> = tables.iter().map(|t| t.name.as_str()).collect();
            // Log in chunks of ≤8 names to avoid very long info lines.
            let mut i = 0;
            while i < names.len() {
                let end = (i + 8).min(names.len());
                let chunk = &names[i..end];
                // Format chunk as space-separated list
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
                    debug!("acpid: tables (cont): {}", line);
                }
                i = end;
            }
            if !devices.is_empty() {
                info!("ACPI devices found: {} device IDs", devices.len());
                for (hid, desc) in &devices {
                    debug!("acpid: device {} — {}", hid, desc);
                }
            }
        }

        AcpiContext { tables, devices_text }
    }
}

// ── VFS provider ──────────────────────────────────────────────────────────────

/// Stable inode numbers.
const INO_ROOT: u64 = 0xa000_0001;
const INO_TABLES_DIR: u64 = 0xa000_0002;
const INO_DEVICES: u64 = 0xa000_0003;
/// Base inode for table files: INO_TABLE_BASE + index.
const INO_TABLE_BASE: u64 = 0xa000_0100;

/// Provider handle allocations:
///   1  → root directory "/"
///   2  → "tables" directory
///   3  → "devices" file
///   4+ → table files (4 + table_index)
const HANDLE_ROOT: u64 = 1;
const HANDLE_TABLES_DIR: u64 = 2;
const HANDLE_DEVICES: u64 = 3;
const HANDLE_TABLE_BASE: u64 = 4;

fn dispatch(ctx: &AcpiContext, op: VfsRpcOp, payload: &[u8]) -> ProviderResponse {
    match op {
        VfsRpcOp::Lookup => handle_lookup(ctx, payload),
        VfsRpcOp::Stat => handle_stat(ctx, payload),
        VfsRpcOp::Read => handle_read(ctx, payload),
        VfsRpcOp::Readdir => handle_readdir(ctx, payload),
        VfsRpcOp::Close => ProviderResponse::ok_empty(),
        VfsRpcOp::ReadIntoFd => handle_read_into_fd(ctx, payload),
        _ => ProviderResponse::err(Errno::ENOSYS),
    }
}

fn parse_u64_le(buf: &[u8]) -> Option<u64> {
    if buf.len() < 8 { return None; }
    Some(u64::from_le_bytes(buf[..8].try_into().ok()?))
}

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
    // Strip leading slash.
    let path = path.trim_start_matches('/');

    match path {
        "" => ProviderResponse::ok_u64(HANDLE_ROOT),
        "tables" => ProviderResponse::ok_u64(HANDLE_TABLES_DIR),
        "devices" => ProviderResponse::ok_u64(HANDLE_DEVICES),
        other => {
            // Could be "tables/<name>" or just "<name>" if mounted subdirectory.
            let name =
                if let Some(rest) = other.strip_prefix("tables/") { rest } else { return ProviderResponse::err(Errno::ENOENT) };
            for (i, t) in ctx.tables.iter().enumerate() {
                if t.name == name {
                    return ProviderResponse::ok_u64(HANDLE_TABLE_BASE + i as u64);
                }
            }
            ProviderResponse::err(Errno::ENOENT)
        }
    }
}

fn handle_stat(ctx: &AcpiContext, payload: &[u8]) -> ProviderResponse {
    let handle = match parse_u64_le(payload) {
        Some(h) => h,
        None => return ProviderResponse::err(Errno::EINVAL),
    };
    let s_ifreg: u32 = 0o100000;
    let s_ifdir: u32 = 0o040000;
    match handle {
        HANDLE_ROOT => ProviderResponse::ok_stat(s_ifdir | 0o555, 0, INO_ROOT),
        HANDLE_TABLES_DIR => ProviderResponse::ok_stat(s_ifdir | 0o555, 0, INO_TABLES_DIR),
        HANDLE_DEVICES => ProviderResponse::ok_stat(
            s_ifreg | 0o444,
            ctx.devices_text.len() as u64,
            INO_DEVICES,
        ),
        h if h >= HANDLE_TABLE_BASE => {
            let idx = (h - HANDLE_TABLE_BASE) as usize;
            if let Some(t) = ctx.tables.get(idx) {
                ProviderResponse::ok_stat(
                    s_ifreg | 0o444,
                    t.data.len() as u64,
                    INO_TABLE_BASE + idx as u64,
                )
            } else {
                ProviderResponse::err(Errno::EBADF)
            }
        }
        _ => ProviderResponse::err(Errno::EBADF),
    }
}

fn handle_read(ctx: &AcpiContext, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 20 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
    let offset = u64::from_le_bytes(payload[8..16].try_into().unwrap_or([0; 8])) as usize;
    let len = u32::from_le_bytes(payload[16..20].try_into().unwrap_or([0; 4])) as usize;

    let data: &[u8] = match handle {
        HANDLE_DEVICES => ctx.devices_text.as_bytes(),
        h if h >= HANDLE_TABLE_BASE => {
            let idx = (h - HANDLE_TABLE_BASE) as usize;
            match ctx.tables.get(idx) {
                Some(t) => &t.data,
                None => return ProviderResponse::err(Errno::EBADF),
            }
        }
        _ => return ProviderResponse::err(Errno::EISDIR),
    };

    if offset >= data.len() {
        return ProviderResponse::ok_read(&[]);
    }
    let end = (offset + len).min(data.len());
    ProviderResponse::ok_read(&data[offset..end])
}

fn handle_read_into_fd(ctx: &AcpiContext, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 24 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
    let offset = u64::from_le_bytes(payload[8..16].try_into().unwrap_or([0; 8])) as usize;
    let len = u32::from_le_bytes(payload[16..20].try_into().unwrap_or([0; 4])) as usize;
    let dest_fd = u32::from_le_bytes(payload[20..24].try_into().unwrap_or([0; 4]));

    let data: &[u8] = match handle {
        HANDLE_DEVICES => ctx.devices_text.as_bytes(),
        h if h >= HANDLE_TABLE_BASE => {
            let idx = (h - HANDLE_TABLE_BASE) as usize;
            match ctx.tables.get(idx) {
                Some(t) => &t.data,
                None => return ProviderResponse::err(Errno::EBADF),
            }
        }
        _ => return ProviderResponse::err(Errno::EISDIR),
    };

    if offset >= data.len() {
        return ProviderResponse::ok_written(0);
    }
    let end = (offset + len).min(data.len());
    let slice = &data[offset..end];
    match stem::syscall::vfs::vfs_write(dest_fd, slice) {
        Ok(n) => ProviderResponse::ok_written(n as u32),
        Err(e) => ProviderResponse::err(e),
    }
}

fn handle_readdir(ctx: &AcpiContext, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 20 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
    let offset =
        u64::from_le_bytes(payload[8..16].try_into().unwrap_or([0; 8])) as usize;
    let max_len = u32::from_le_bytes(payload[16..20].try_into().unwrap_or([0; 4])) as usize;

    // abi::vfs_rpc::DirentWire: [ino:u64][file_type:u8][name_len:u8][name...]
    const DT_DIR: u8 = 4;
    const DT_REG: u8 = 8;

    let mut out: Vec<u8> = Vec::new();
    let mut emitted = 0usize;

    let append = |out: &mut Vec<u8>, ino: u64, ft: u8, name: &str| {
        out.extend_from_slice(&ino.to_le_bytes());
        out.push(ft);
        out.push(name.len() as u8);
        out.extend_from_slice(name.as_bytes());
    };

    match handle {
        HANDLE_ROOT => {
            let entries: &[(&str, u8, u64)] = &[
                ("tables", DT_DIR, INO_TABLES_DIR),
                ("devices", DT_REG, INO_DEVICES),
            ];
            for (i, &(name, ft, ino)) in entries.iter().enumerate() {
                if i < offset {
                    continue;
                }
                if out.len() + 10 + name.len() > max_len {
                    break;
                }
                append(&mut out, ino, ft, name);
                emitted += 1;
            }
        }
        HANDLE_TABLES_DIR => {
            for (i, t) in ctx.tables.iter().enumerate() {
                if i < offset {
                    continue;
                }
                let name = t.name.as_str();
                if out.len() + 10 + name.len() > max_len {
                    break;
                }
                append(&mut out, INO_TABLE_BASE + i as u64, DT_REG, name);
                emitted += 1;
            }
        }
        _ => return ProviderResponse::err(Errno::ENOTDIR),
    }

    let _ = emitted;
    ProviderResponse::ok_read(&out)
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn run_provider(ctx: AcpiContext, req_read: u32) -> ! {
    let mut lp = ProviderLoop::new(req_read);
    loop {
        match lp.next_request() {
            Ok(req) => {
                let resp = dispatch(&ctx, req.op, &req.payload);
                if let Err(e) = lp.send_response(&req, resp) {
                    warn!("acpid: send_response failed: {:?}", e);
                }
            }
            Err(Errno::EPIPE) => {
                info!("acpid: provider port closed, restarting");
                stem::syscall::exit(0);
            }
            Err(e) => {
                warn!("acpid: provider loop error: {:?}", e);
                stem::time::sleep_ms(100);
            }
        }
    }
}

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("ACPI namespace service starting");

    let ctx = AcpiContext::build();

    // Create provider port pair.
    let (req_write, req_read) = match stem::syscall::port::port_create(PORT_CAPACITY) {
        Ok(p) => p,
        Err(e) => {
            warn!("acpid: port_create failed: {:?}", e);
            stem::syscall::exit(1);
        }
    };

    // Mount at /services/acpi.
    if let Err(e) = vfs_mount(req_write, MOUNT_POINT) {
        warn!("acpid: mount at {} failed: {:?}", MOUNT_POINT, e);
        // Do not exit — other services may depend on us being alive.
    } else {
        debug!("acpid: mounted at {}", MOUNT_POINT);
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
        AcpiContext { tables: ts, devices_text: "# test\n".to_string() }
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
        let handle = HANDLE_TABLE_BASE;
        let offset: u64 = 0;
        let len: u32 = 16;
        let mut payload = handle.to_le_bytes().to_vec();
        payload.extend_from_slice(&offset.to_le_bytes());
        payload.extend_from_slice(&len.to_le_bytes());
        let resp = handle_read(&ctx, &payload);
        assert_eq!(resp.status, 0);
        // ok_read prepends a u32 byte count
        let n = u32::from_le_bytes(resp.payload[..4].try_into().unwrap()) as usize;
        assert_eq!(&resp.payload[4..4 + n], data);
    }

    #[test]
    fn read_beyond_end_returns_empty() {
        let data = b"short";
        let ctx = make_ctx(&[("FACP", data)]);
        let handle = HANDLE_TABLE_BASE;
        let offset: u64 = 1000;
        let len: u32 = 100;
        let mut payload = handle.to_le_bytes().to_vec();
        payload.extend_from_slice(&offset.to_le_bytes());
        payload.extend_from_slice(&len.to_le_bytes());
        let resp = handle_read(&ctx, &payload);
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
        // Check raw entry bytes contain "tables" and "devices"
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
    }
}

