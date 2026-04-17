//! Driver catalog: scans `/drivers` for binaries that advertise DriverV1 via
//! the canonical Motor descriptor and caches their metadata.
//!
//! # Discovery model
//!
//! On each scan pass `devd` reads every regular file from the known search
//! paths.  For each candidate binary the catalog:
//!
//! 1. Checks for the `THINGOS_SEED` ELF symbol and confirms `DriverV1`.
//! 2. Reads descriptor metadata and optional legacy hints.
//! 3. Caches a [`DriverEntry`] record keyed by binary path.
//!
//! The catalog is rescanned periodically so that newly installed driver
//! binaries are picked up without restarting `devd`.

#![allow(dead_code)]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use abi::driver_interface::{
    DRIVER_DESCRIPTOR_ABI_VERSION, DRIVER_DESCRIPTOR_SYMBOL, DRIVER_ENTRY_SYMBOL,
    DRIVER_INTERFACE_ABI_VERSION, DRIVER_MARKER_SYMBOL, DRIVER_MATCH_ANY_CLASS,
    DRIVER_MATCH_ANY_ID, DriverClass, DriverDescriptor, DriverInterfaceV1,
};
use abi::seed::{INTERFACE_DRIVER_V1, SEED_ABI_VERSION, SEED_SYMBOL, Seed};
use abi::syscall::vfs_flags::O_RDONLY;
use stem::debug;
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read, vfs_readdir, vfs_seek};

/// Maximum ELF binary size the catalog will read into memory for symbol
/// inspection.  Binaries larger than this are silently skipped.
const MAX_BINARY_INSPECT_BYTES: usize = 8 * 1024 * 1024; // 8 MiB

/// Directories searched for driver-capable binaries, in priority order.
/// Only `/drivers` is scanned — driver binaries must live there, not in `/bin`.
const SEARCH_PATHS: &[&str] = &["/drivers"];

/// Cached metadata extracted from a driver-capable binary.
#[derive(Clone)]
pub struct DriverEntry {
    /// Absolute VFS path to the binary (e.g. `/bin/rtc_cmos`).
    pub path: String,
    /// ABI version read from the marker symbol.
    pub abi_version: u32,
    /// Human-readable driver name (from THINGOS_SEED/THINGOS_DRIVER when present).
    pub driver_name: String,
    /// Driver class declared in the descriptor.
    pub driver_class: DriverClass,
    /// PCI vendor ID to match (legacy hint, 0 = any).
    pub vendor_id: u16,
    /// PCI device ID to match (legacy hint, 0 = any).
    pub device_id: u16,
    /// PCI class code to match (legacy hint, 0 = any).
    pub class_code: u32,
    /// Class code bitmask (legacy hint, 0 = exact match or no-mask).
    pub class_mask: u32,
    /// Name of the driver start symbol.
    pub start_symbol: String,
    /// Canonical device kind matched by this driver (e.g. `dev.rtc.Cmos`).
    pub device_kind: String,
}

impl DriverEntry {
    /// Return match score for this driver against a discovered PCI device.
    /// Zero means no match.
    pub fn pci_match_score(&self, vendor: u16, device: u16, class: u32) -> u32 {
        // Prefer explicit legacy match hints when available.
        if self.vendor_id != 0 || self.device_id != 0 || self.class_code != 0 {
            let mut score = 0u32;
            if self.vendor_id != DRIVER_MATCH_ANY_ID && self.vendor_id != vendor {
                return 0;
            }
            if self.vendor_id != DRIVER_MATCH_ANY_ID {
                score += 200;
            }
            if self.device_id != DRIVER_MATCH_ANY_ID && self.device_id != device {
                return 0;
            }
            if self.device_id != DRIVER_MATCH_ANY_ID {
                score += 400;
            }
            if self.class_code != DRIVER_MATCH_ANY_CLASS {
                let mask = if self.class_mask != 0 { self.class_mask } else { 0xFFFFFF };
                if (class & mask) != (self.class_code & mask) {
                    return 0;
                }
                score += 300;
            }
            // small tie-break preference on abi version
            return score.saturating_add(self.abi_version.min(50));
        }

        // Class-only fallback for descriptor-only drivers.
        let major = (class >> 16) as u8;
        match self.driver_class {
            DriverClass::Net if major == 0x02 => 100,
            DriverClass::Block if major == 0x01 => 100,
            DriverClass::Display if major == 0x03 => 100,
            DriverClass::Input if major == 0x09 => 100,
            DriverClass::Audio if major == 0x04 => 100,
            DriverClass::Serial if major == 0x07 => 100,
            DriverClass::Unknown | DriverClass::Other => 0,
            _ => 0,
        }
    }

    /// Returns `true` when this driver matches the given PCI identifiers.
    pub fn matches_pci(&self, vendor: u16, device: u16, class: u32) -> bool {
        self.pci_match_score(vendor, device, class) != 0
    }

    /// Returns `true` when this driver matches the given device kind.
    pub fn matches_kind(&self, kind: &str) -> bool {
        self.device_kind == kind
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
        let mut best: Option<(&DriverEntry, u32)> = None;
        for entry in &self.entries {
            let score = entry.pci_match_score(vendor, device, class);
            if score == 0 {
                continue;
            }
            if best.map_or(true, |(_, s)| score > s) {
                best = Some((entry, score));
            }
        }
        best.map(|(e, _)| e)
    }

    /// Return the first driver entry that matches the given device kind.
    pub fn find_for_kind(&self, kind: &str) -> Option<&DriverEntry> {
        self.entries.iter().find(|e| e.matches_kind(kind))
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

    /// Inspect a single binary at an explicit path and, if it is a valid
    /// driver-capable binary, add it to the catalog.
    ///
    /// Returns `Some(&DriverEntry)` when the binary was accepted, `None` when
    /// it is not driver-capable or cannot be read.
    pub fn inspect_binary_path(&mut self, path: &str) -> Option<&DriverEntry> {
        let before = self.entries.len();
        self.inspect_binary(path);
        if self.entries.len() > before { Some(&self.entries[before]) } else { None }
    }

    fn inspect_binary(&mut self, path: &str) {
        // Read the binary into memory for symbol inspection.
        let bytes = match read_file(path, MAX_BINARY_INSPECT_BYTES) {
            Some(b) => b,
            None => return,
        };

        let seed_descriptor = resolve_elf64_symbol_from_bytes(&bytes, SEED_SYMBOL)
            .and_then(|sym_vaddr| read_seed_descriptor(&bytes, sym_vaddr));

        let mut iface_legacy: Option<DriverInterfaceV1> = None;
        if let Some(sym_vaddr) = resolve_elf64_symbol_from_bytes(&bytes, DRIVER_MARKER_SYMBOL) {
            iface_legacy = read_driver_interface_v1(&bytes, sym_vaddr);
        }

        // Canonical path: Seed descriptor declares DriverV1 interface.
        if let Some(seed) = seed_descriptor {
            if seed.abi_version != SEED_ABI_VERSION {
                debug!(
                    "DEVD CATALOG: {} has unknown seed abi_version {} (expected {}), skipping",
                    path, seed.abi_version, SEED_ABI_VERSION
                );
                return;
            }
            if !seed.implements_driver_v1() {
                return;
            }
        }

        let descriptor = match resolve_elf64_symbol_from_bytes(&bytes, DRIVER_DESCRIPTOR_SYMBOL)
            .and_then(|sym_vaddr| read_driver_descriptor(&bytes, sym_vaddr))
        {
            Some(d) => d,
            None => {
                // Transitional compatibility path: legacy marker-only drivers.
                if seed_descriptor.is_some() {
                    debug!(
                        "DEVD CATALOG: {} declares DriverV1 in its Seed but lacks legacy THINGOS_DRIVER descriptor; skipping",
                        path
                    );
                    return;
                }
                let Some(iface) = iface_legacy else {
                    return;
                };
                if iface.abi_version != DRIVER_INTERFACE_ABI_VERSION {
                    debug!(
                        "DEVD CATALOG: {} has unknown legacy abi_version {} (expected {}), skipping",
                        path, iface.abi_version, DRIVER_INTERFACE_ABI_VERSION
                    );
                    return;
                }
                let start_symbol = iface.entry_symbol_name().to_string();
                debug!(
                    "DEVD CATALOG: registered legacy driver '{}' vendor=0x{:04x} device=0x{:04x} \
                     class=0x{:06x} entry='{}'",
                    path, iface.vendor_id, iface.device_id, iface.class_code, start_symbol
                );
                self.entries.push(DriverEntry {
                    path: path.to_string(),
                    abi_version: iface.abi_version,
                    driver_name: path.to_string(),
                    driver_class: DriverClass::Unknown,
                    vendor_id: iface.vendor_id,
                    device_id: iface.device_id,
                    class_code: iface.class_code,
                    class_mask: iface.class_mask,
                    start_symbol,
                });
                return;
            }
        };

        if descriptor.abi_version != DRIVER_DESCRIPTOR_ABI_VERSION {
            debug!(
                "DEVD CATALOG: {} has unknown descriptor abi_version {} (expected {}), skipping",
                path, descriptor.abi_version, DRIVER_DESCRIPTOR_ABI_VERSION
            );
            return;
        }

        let driver_name =
            read_driver_name(&bytes, descriptor.driver_name_ptr as u64, descriptor.driver_name_len)
                .unwrap_or_else(|| path.to_string());
        let start_symbol =
            resolve_elf64_symbol_name_by_value(&bytes, descriptor.start as usize as u64)
                .unwrap_or_else(|| DRIVER_ENTRY_SYMBOL.to_string());
        let legacy = iface_legacy.unwrap_or(DriverInterfaceV1 {
            abi_version: 0,
            flags: 0,
            vendor_id: 0,
            device_id: 0,
            class_code: 0,
            class_mask: 0,
            entry_symbol: [0u8; 32],
        });
        let manifest = resolve_elf64_section_from_bytes(&bytes, abi::module_manifest::SECTION_NAME)
            .and_then(|info| read_manifest(&bytes, info.offset));

        let device_kind = manifest
            .map(|m| {
                let bytes = &m.device_kind;
                let len = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
                core::str::from_utf8(&bytes[..len])
                    .unwrap_or("unknown")
                    .to_string()
            })
            .unwrap_or_else(|| "unknown".into());

        debug!(
            "DEVD CATALOG: registered driver '{}' name='{}' class={:?} kind='{}' start='{}'",
            path, driver_name, descriptor.driver_class, device_kind, start_symbol
        );

        self.entries.push(DriverEntry {
            path: path.to_string(),
            abi_version: descriptor.abi_version,
            driver_name,
            driver_class: descriptor.driver_class,
            vendor_id: legacy.vendor_id,
            device_id: legacy.device_id,
            class_code: legacy.class_code,
            class_mask: legacy.class_mask,
            start_symbol,
            device_kind,
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

struct SectionInfo {
    offset: usize,
    size: usize,
}

fn resolve_elf64_section_from_bytes(bytes: &[u8], target: &str) -> Option<SectionInfo> {
    if bytes.len() < 64 {
        return None;
    }
    let e_shoff = read_u64(bytes, 40)? as usize;
    let e_shentsize = read_u16(bytes, 58)? as usize;
    let e_shnum = read_u16(bytes, 60)? as usize;
    let e_shstrndx = read_u16(bytes, 62)? as usize;

    if e_shoff == 0 || e_shentsize < 64 || e_shnum == 0 {
        return None;
    }

    let shstrtab_sh_off = e_shoff.saturating_add(e_shstrndx.saturating_mul(e_shentsize));
    let shstrtab_off = read_u64(bytes, shstrtab_sh_off + 24)? as usize;
    let shstrtab_size = read_u64(bytes, shstrtab_sh_off + 32)? as usize;

    for i in 0..e_shnum {
        let sh_off = e_shoff.saturating_add(i.saturating_mul(e_shentsize));
        let sh_name_off = read_u32(bytes, sh_off)? as usize;
        let name_off = shstrtab_off + sh_name_off;
        if name_off >= shstrtab_off + shstrtab_size || name_off >= bytes.len() {
            continue;
        }

        let name_end = bytes[name_off..]
            .iter()
            .position(|&b| b == 0)
            .map(|n| name_off + n)
            .unwrap_or(bytes.len());
        if let Ok(name) = core::str::from_utf8(&bytes[name_off..name_end]) {
            if name == target {
                return Some(SectionInfo {
                    offset: read_u64(bytes, sh_off + 24)? as usize,
                    size: read_u64(bytes, sh_off + 32)? as usize,
                });
            }
        }
    }
    None
}

/// Resolve an ELF64 symbol name by symbol value.
fn resolve_elf64_symbol_name_by_value(bytes: &[u8], target_value: u64) -> Option<String> {
    if bytes.len() < 64 || target_value == 0 {
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
            continue;
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
        const SYM_ENTRY: usize = 24;
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
            if st_value != target_value {
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
                if !name.is_empty() {
                    return Some(name.to_string());
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

fn read_driver_descriptor(bytes: &[u8], sym_vaddr: u64) -> Option<DriverDescriptor> {
    let file_off = vaddr_to_file_offset(bytes, sym_vaddr)?;
    let size = core::mem::size_of::<DriverDescriptor>();
    if file_off + size > bytes.len() {
        return None;
    }
    let desc: DriverDescriptor = unsafe {
        let mut tmp = core::mem::MaybeUninit::<DriverDescriptor>::uninit();
        core::ptr::copy_nonoverlapping(
            bytes.as_ptr().add(file_off),
            tmp.as_mut_ptr() as *mut u8,
            size,
        );
        tmp.assume_init()
    };
    Some(desc)
}

fn read_seed_descriptor(bytes: &[u8], sym_vaddr: u64) -> Option<Seed> {
    let file_off = vaddr_to_file_offset(bytes, sym_vaddr)?;
    let size = core::mem::size_of::<Seed>();
    if file_off + size > bytes.len() {
        return None;
    }
    let desc: Seed = unsafe {
        let mut tmp = core::mem::MaybeUninit::<Seed>::uninit();
        core::ptr::copy_nonoverlapping(
            bytes.as_ptr().add(file_off),
            tmp.as_mut_ptr() as *mut u8,
            size,
        );
        tmp.assume_init()
    };
    Some(desc)
}

fn read_manifest(bytes: &[u8], offset: usize) -> Option<abi::module_manifest::ManifestHeader> {
    let size = core::mem::size_of::<abi::module_manifest::ManifestHeader>();
    if offset + size > bytes.len() {
        return None;
    }
    let manifest: abi::module_manifest::ManifestHeader = unsafe {
        let mut tmp = core::mem::MaybeUninit::<abi::module_manifest::ManifestHeader>::uninit();
        core::ptr::copy_nonoverlapping(
            bytes.as_ptr().add(offset),
            tmp.as_mut_ptr() as *mut u8,
            size,
        );
        tmp.assume_init()
    };
    if manifest.magic != abi::module_manifest::MANIFEST_MAGIC {
        return None;
    }
    Some(manifest)
}

fn read_driver_name(bytes: &[u8], vaddr: u64, len: usize) -> Option<String> {
    if len == 0 {
        return None;
    }
    let file_off = vaddr_to_file_offset(bytes, vaddr)?;
    if file_off + len > bytes.len() {
        return None;
    }
    let raw = &bytes[file_off..file_off + len];
    core::str::from_utf8(raw).ok().map(|s| s.to_string())
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
        return None;
    }
    Some(out)
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
