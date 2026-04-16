//! Driver catalog: scans `/bin` (and `/drivers` when present) for binaries
//! that export the `THING_DRIVER_V1` marker symbol and caches their metadata.
//!
//! # Discovery model
//!
//! On each scan pass `devd` reads every regular file from the known search
//! paths.  For each candidate binary the catalog:
//!
//! 1. Checks for the `THING_DRIVER_V1` ELF symbol (presence = driver-capable).
//! 2. Reads the [`DriverInterfaceV1`] struct from the symbol's virtual address
//!    inside the loaded data to extract matching criteria and the entrypoint
//!    symbol name.
//! 3. Caches a [`DriverEntry`] record keyed by binary path.
//!
//! The catalog is rescanned periodically so that newly installed driver
//! binaries are picked up without restarting `devd`.

#![allow(dead_code)]

extern crate alloc;

use abi::driver_interface::{
    DriverEntryCtx, DriverInterfaceV1, DRIVER_ENTRY_SYMBOL, DRIVER_INTERFACE_ABI_VERSION,
    DRIVER_MARKER_SYMBOL, DRIVER_MATCH_ANY_CLASS, DRIVER_MATCH_ANY_ID,
};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use stem::debug;
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read, vfs_readdir, vfs_seek};

use abi::syscall::vfs_flags::O_RDONLY;

/// Maximum ELF binary size the catalog will read into memory for symbol
/// inspection.  Binaries larger than this are silently skipped.
const MAX_BINARY_INSPECT_BYTES: usize = 8 * 1024 * 1024; // 8 MiB

/// Directories searched for driver-capable binaries, in priority order.
const SEARCH_PATHS: &[&str] = &["/bin", "/drivers"];

/// Cached metadata extracted from a driver-capable binary.
#[derive(Clone)]
pub struct DriverEntry {
    /// Absolute VFS path to the binary (e.g. `/bin/rtc_cmos`).
    pub path: String,
    /// ABI version read from the marker symbol.
    pub abi_version: u32,
    /// PCI vendor ID to match (0 = any).
    pub vendor_id: u16,
    /// PCI device ID to match (0 = any).
    pub device_id: u16,
    /// PCI class code to match (0 = any).
    pub class_code: u32,
    /// Class code bitmask (0 = exact match or no-mask).
    pub class_mask: u32,
    /// Name of the driver entrypoint symbol.
    pub entry_symbol: String,
}

impl DriverEntry {
    /// Returns `true` when this driver matches the given PCI identifiers.
    pub fn matches_pci(&self, vendor: u16, device: u16, class: u32) -> bool {
        if self.vendor_id != DRIVER_MATCH_ANY_ID && self.vendor_id != vendor {
            return false;
        }
        if self.device_id != DRIVER_MATCH_ANY_ID && self.device_id != device {
            return false;
        }
        if self.class_code != DRIVER_MATCH_ANY_CLASS {
            let mask = if self.class_mask != 0 { self.class_mask } else { 0xFFFFFF };
            if (class & mask) != (self.class_code & mask) {
                return false;
            }
        }
        true
    }
}

/// In-memory driver catalog populated by [`scan`].
pub struct Catalog {
    entries: Vec<DriverEntry>,
}

impl Catalog {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    /// (Re-)scan all known search paths and rebuild the catalog.
    pub fn scan(&mut self) {
        self.entries.clear();
        for &dir in SEARCH_PATHS {
            self.scan_dir(dir);
        }
        debug!("DEVD CATALOG: {} driver(s) found", self.entries.len());
    }

    /// Return the first driver entry whose PCI match criteria cover the given
    /// device.  Returns `None` when no registered driver matches.
    pub fn find_for_pci(&self, vendor: u16, device: u16, class: u32) -> Option<&DriverEntry> {
        self.entries.iter().find(|e| e.matches_pci(vendor, device, class))
    }

    /// All catalogued driver entries.
    pub fn entries(&self) -> &[DriverEntry] {
        &self.entries
    }

    // ── Private helpers ──────────────────────────────────────────────────────

    fn scan_dir(&mut self, dir: &str) {
        let fd = match vfs_open(dir, O_RDONLY) {
            Ok(fd) => fd,
            Err(_) => return,
        };

        let mut buf = [0u8; 4096];
        let n = match vfs_readdir(fd, &mut buf) {
            Ok(n) => n,
            Err(_) => {
                let _ = vfs_close(fd);
                return;
            }
        };
        let _ = vfs_close(fd);

        let mut offset = 0usize;
        while offset < n {
            let mut end = offset;
            while end < n && buf[end] != 0 {
                end += 1;
            }
            if end > offset {
                if let Ok(name) = core::str::from_utf8(&buf[offset..end]) {
                    if name != "." && name != ".." {
                        let path = alloc::format!("{}/{}", dir, name);
                        self.inspect_binary(&path);
                    }
                }
            }
            offset = end.saturating_add(1);
        }
    }

    fn inspect_binary(&mut self, path: &str) {
        // Read the binary into memory for symbol inspection.
        let bytes = match read_file(path, MAX_BINARY_INSPECT_BYTES) {
            Some(b) => b,
            None => return,
        };

        // Check for the driver marker symbol.
        let sym_vaddr = match resolve_elf64_symbol_from_bytes(&bytes, DRIVER_MARKER_SYMBOL) {
            Some(v) => v,
            None => return,
        };

        // The symbol value is the file-relative VMA.  Read the
        // DriverInterfaceV1 struct from the file at the segment offset that
        // corresponds to sym_vaddr.
        let iface = match read_driver_interface_v1(&bytes, sym_vaddr) {
            Some(i) => i,
            None => {
                debug!("DEVD CATALOG: {} has THING_DRIVER_V1 but could not read struct", path);
                return;
            }
        };

        if iface.abi_version != DRIVER_INTERFACE_ABI_VERSION {
            debug!(
                "DEVD CATALOG: {} has unknown abi_version {} (expected {}), skipping",
                path, iface.abi_version, DRIVER_INTERFACE_ABI_VERSION
            );
            return;
        }

        let entry_symbol = iface.entry_symbol_name().to_string();
        debug!(
            "DEVD CATALOG: registered driver '{}' vendor=0x{:04x} device=0x{:04x} \
             class=0x{:06x} entry='{}'",
            path, iface.vendor_id, iface.device_id, iface.class_code, entry_symbol
        );

        self.entries.push(DriverEntry {
            path: path.to_string(),
            abi_version: iface.abi_version,
            vendor_id: iface.vendor_id,
            device_id: iface.device_id,
            class_code: iface.class_code,
            class_mask: iface.class_mask,
            entry_symbol,
        });
    }
}

// ── ELF helpers (userspace-side, reads from a byte slice) ───────────────────

/// Resolve an ELF64 symbol by name from a binary byte slice; returns the
/// file-relative virtual address (not adjusted for load bias).
fn resolve_elf64_symbol_from_bytes(bytes: &[u8], target: &str) -> Option<u64> {
    if bytes.len() < 64 {
        return None;
    }
    if &bytes[0..4] != b"\x7fELF" || bytes[4] != 2 || bytes[5] != 1 {
        return None;
    }

    let e_shoff = read_u64(bytes, 40)? as usize;
    let e_shentsize = read_u16(bytes, 58)? as usize;
    let e_shnum = read_u16(bytes, 60)? as usize;

    if e_shoff == 0 || e_shentsize < 64 || e_shnum == 0 {
        return None;
    }

    for i in 0..e_shnum {
        let sh_off = e_shoff.saturating_add(i.saturating_mul(e_shentsize));
        if sh_off + e_shentsize > bytes.len() {
            break;
        }
        let sh_type = read_u32(bytes, sh_off + 4)?;
        if sh_type != 2 && sh_type != 11 {
            continue; // SHT_SYMTAB=2, SHT_DYNSYM=11
        }

        let sh_link = read_u32(bytes, sh_off + 40)? as usize;
        let strtab_sh_off = e_shoff.saturating_add(sh_link.saturating_mul(e_shentsize));
        if strtab_sh_off + e_shentsize > bytes.len() {
            continue;
        }
        let strtab_off = read_u64(bytes, strtab_sh_off + 24)? as usize;
        let strtab_size = read_u64(bytes, strtab_sh_off + 32)? as usize;
        if strtab_off + strtab_size > bytes.len() {
            continue;
        }

        let sym_off = read_u64(bytes, sh_off + 24)? as usize;
        let sym_size = read_u64(bytes, sh_off + 32)? as usize;
        const SYM_ENTRY: usize = 24; // Elf64_Sym
        if sym_size == 0 || sym_off + sym_size > bytes.len() {
            continue;
        }

        for s in 0..(sym_size / SYM_ENTRY) {
            let se = sym_off + s * SYM_ENTRY;
            if se + SYM_ENTRY > bytes.len() {
                break;
            }
            let st_name = read_u32(bytes, se)? as usize;
            let st_value = read_u64(bytes, se + 8)?;
            if st_value == 0 {
                continue;
            }
            let name_off = strtab_off + st_name;
            if name_off >= bytes.len() {
                continue;
            }
            let name_end = bytes[name_off..]
                .iter()
                .position(|&b| b == 0)
                .map(|n| name_off + n)
                .unwrap_or(bytes.len());
            if let Ok(name) = core::str::from_utf8(&bytes[name_off..name_end]) {
                if name == target {
                    return Some(st_value);
                }
            }
        }
    }
    None
}

/// Given a file-relative virtual address `vaddr`, find the PT_LOAD segment
/// that covers it and return the corresponding file offset.
fn vaddr_to_file_offset(bytes: &[u8], vaddr: u64) -> Option<usize> {
    if bytes.len() < 64 {
        return None;
    }
    let e_phoff = read_u64(bytes, 32)? as usize;
    let e_phentsize = read_u16(bytes, 54)? as usize;
    let e_phnum = read_u16(bytes, 56)? as usize;

    for i in 0..e_phnum {
        let off = e_phoff + i * e_phentsize;
        if off + e_phentsize > bytes.len() {
            break;
        }
        let p_type = read_u32(bytes, off)?;
        if p_type != 1 {
            // PT_LOAD
            continue;
        }
        let p_offset = read_u64(bytes, off + 8)?;
        let p_vaddr = read_u64(bytes, off + 16)?;
        let p_filesz = read_u64(bytes, off + 32)?;
        if vaddr >= p_vaddr && vaddr < p_vaddr + p_filesz {
            let delta = vaddr - p_vaddr;
            return Some((p_offset + delta) as usize);
        }
    }
    None
}

/// Read a [`DriverInterfaceV1`] struct from the binary at the file offset
/// corresponding to `sym_vaddr`.
fn read_driver_interface_v1(bytes: &[u8], sym_vaddr: u64) -> Option<DriverInterfaceV1> {
    let file_off = vaddr_to_file_offset(bytes, sym_vaddr)?;
    let size = core::mem::size_of::<DriverInterfaceV1>();
    if file_off + size > bytes.len() {
        return None;
    }
    // SAFETY: we verified that [file_off, file_off+size) is in bounds.
    // DriverInterfaceV1 is repr(C) with no padding requirements beyond alignment.
    let iface: DriverInterfaceV1 = unsafe {
        let mut tmp = core::mem::MaybeUninit::<DriverInterfaceV1>::uninit();
        core::ptr::copy_nonoverlapping(
            bytes.as_ptr().add(file_off),
            tmp.as_mut_ptr() as *mut u8,
            size,
        );
        tmp.assume_init()
    };
    Some(iface)
}

// ── VFS read helper ──────────────────────────────────────────────────────────

fn read_file(path: &str, max_bytes: usize) -> Option<Vec<u8>> {
    let fd = vfs_open(path, O_RDONLY).ok()?;

    // Read up to max_bytes in chunks.
    let mut buf = alloc::vec![0u8; max_bytes.min(1024 * 1024)];
    let mut total = 0usize;
    let mut out = Vec::new();
    loop {
        let room = buf.len().min(max_bytes.saturating_sub(total));
        if room == 0 {
            break;
        }
        match vfs_read(fd, &mut buf[..room]) {
            Ok(0) => break,
            Ok(n) => {
                out.extend_from_slice(&buf[..n]);
                total += n;
            }
            Err(_) => break,
        }
    }
    let _ = vfs_close(fd);
    if out.len() < 4 {
        None
    } else {
        Some(out)
    }
}

// ── Byte readers ─────────────────────────────────────────────────────────────

fn read_u16(b: &[u8], off: usize) -> Option<u16> {
    let s = b.get(off..off + 2)?;
    Some(u16::from_le_bytes([s[0], s[1]]))
}

fn read_u32(b: &[u8], off: usize) -> Option<u32> {
    let s = b.get(off..off + 4)?;
    Some(u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
}

fn read_u64(b: &[u8], off: usize) -> Option<u64> {
    let s = b.get(off..off + 8)?;
    Some(u64::from_le_bytes([s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]]))
}
