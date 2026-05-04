//! sysfs — read-only kernel device discovery metadata mounted at `/sys`.
//!
//! The initial implementation exposes claimable PCI devices from the kernel
//! device registry under `/sys/devices`. This is enough for `cambium` to discover
//! hardware, match a userspace driver, and decide whether a restart is valid.

use alloc::sync::Arc;
use alloc::vec::Vec;
use alloc::{format, vec};

use abi::errors::{Errno, SysResult};

use super::{VfsDriver, VfsNode, VfsStat};
use crate::device_registry::{DeviceEntry, REGISTRY};

pub struct SysFs;

impl SysFs {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SysFs {
    fn default() -> Self {
        Self::new()
    }
}

impl VfsDriver for SysFs {
    fn lookup(&self, path: &str) -> SysResult<Arc<dyn VfsNode>> {
        crate::ktrace!("Looking up sysfs path '{}'", path);
        match SysPath::parse(path)? {
            SysPath::Root => Ok(Arc::new(StaticDirNode::new(
                300,
                &["boot", "devices", "device_snapshot", "firmware"],
            ))),
            SysPath::Boot => Ok(Arc::new(StaticDirNode::new(308, &["cmdline"]))),
            SysPath::BootFile("cmdline") => {
                let mut text = crate::vfs::devfs::cmdline().unwrap_or_default().into_bytes();
                if !text.ends_with(b"\n") {
                    text.push(b'\n');
                }
                Ok(Arc::new(StaticTextNode::new(text, 309)))
            }
            SysPath::BootFile(_) => Err(Errno::ENOENT),
            SysPath::Devices => Ok(Arc::new(DevicesDirNode)),
            SysPath::DeviceSnapshot => {
                Ok(Arc::new(StaticTextNode::new(device_snapshot_text(), 307)))
            }
            SysPath::DeviceDir(name) => {
                let (idx, entry) = find_device_by_slot(name)?;
                crate::ktrace!("Matched sysfs device dir '{}' at index {}", name, idx);
                Ok(Arc::new(DeviceDirNode::new(idx, entry)))
            }
            SysPath::DeviceFile(name, file) => {
                let (idx, entry) = find_device_by_slot(name)?;
                let node = lookup_device_file(idx, entry, file)?;
                crate::ktrace!("Matched sysfs device file '{}/{}'", name, file);
                Ok(Arc::new(node))
            }
            SysPath::VirtioDir(name) => {
                let (idx, entry) = find_device_by_slot(name)?;
                if entry.vendor_id != 0x1af4 {
                    return Err(Errno::ENOENT);
                }
                Ok(Arc::new(VirtioDirNode::new(idx, entry)))
            }
            SysPath::VirtioFile(name, file) => {
                let (idx, entry) = find_device_by_slot(name)?;
                if entry.vendor_id != 0x1af4 {
                    return Err(Errno::ENOENT);
                }
                let node = lookup_virtio_file(idx, entry, file)?;
                Ok(Arc::new(node))
            }
            SysPath::Firmware => {
                Ok(Arc::new(StaticDirNode::new(302, &["acpi", "dtb", "hhdm", "framebuffer"])))
            }
            // /sys/firmware/acpi/ — directory with rsdp file and tables subdirectory.
            SysPath::AcpiDir => Ok(Arc::new(StaticDirNode::new(0x5ffd, &["rsdp", "tables"]))),
            SysPath::AcpiRsdpFile => {
                if let Some(rsdp) = crate::boot_info::get().and_then(|i| i.acpi_rsdp) {
                    let text = format!("0x{:016x}\n", rsdp);
                    Ok(Arc::new(StaticTextNode::new(text.into_bytes(), 303)))
                } else {
                    Err(Errno::ENOENT)
                }
            }
            SysPath::FirmwareFile("dtb") => {
                if let Some(dtb) = crate::boot_info::get().and_then(|i| i.dtb_ptr) {
                    let text = format!("0x{:016x}\n", dtb);
                    Ok(Arc::new(StaticTextNode::new(text.into_bytes(), 304)))
                } else {
                    Err(Errno::ENOENT)
                }
            }
            SysPath::FirmwareFile("hhdm") => {
                if let Some(hhdm) = crate::boot_info::get().map(|i| i.hhdm_offset) {
                    let text = format!("0x{:016x}\n", hhdm);
                    Ok(Arc::new(StaticTextNode::new(text.into_bytes(), 305)))
                } else {
                    Err(Errno::ENOENT)
                }
            }
            SysPath::FirmwareFile("framebuffer") => {
                if let Some(fb) = crate::boot_info::get().and_then(|i| i.framebuffer) {
                    let text = format!(
                        "width={}\nheight={}\nstride={}\nformat={:?}\naddr=0x{:x}\n",
                        fb.width, fb.height, fb.pitch, fb.format, fb.addr
                    );
                    Ok(Arc::new(StaticTextNode::new(text.into_bytes(), 306)))
                } else {
                    Err(Errno::ENOENT)
                }
            }
            SysPath::FirmwareFile(_) => Err(Errno::ENOENT),
            SysPath::AcpiTablesDir => Ok(Arc::new(AcpiTablesDirNode)),
            SysPath::AcpiTableFile(name) => {
                let data = acpi_table_data(name).ok_or(Errno::ENOENT)?;
                Ok(Arc::new(StaticTextNode::new(data, acpi_table_ino(name))))
            }
        }
    }
}

enum SysPath<'a> {
    Root,
    Boot,
    BootFile(&'a str),
    Devices,
    DeviceSnapshot,
    DeviceDir(&'a str),
    DeviceFile(&'a str, &'a str),
    VirtioDir(&'a str),
    VirtioFile(&'a str, &'a str),
    Firmware,
    FirmwareFile(&'a str),
    /// `/sys/firmware/acpi/` — directory node.
    AcpiDir,
    /// `/sys/firmware/acpi/rsdp` — hex RSDP physical address.
    AcpiRsdpFile,
    AcpiTablesDir,
    AcpiTableFile(&'a str),
}

impl<'a> SysPath<'a> {
    fn parse(path: &'a str) -> SysResult<Self> {
        if path.is_empty() {
            return Ok(Self::Root);
        }

        let mut parts = path.split('/').filter(|part| !part.is_empty());
        match (parts.next(), parts.next(), parts.next(), parts.next()) {
            (Some("boot"), None, None, None) => Ok(Self::Boot),
            (Some("boot"), Some(file), None, None) => Ok(Self::BootFile(file)),
            (Some("devices"), None, None, None) => Ok(Self::Devices),
            (Some("device_snapshot"), None, None, None) => Ok(Self::DeviceSnapshot),
            (Some("devices"), Some(dev), None, None) => Ok(Self::DeviceDir(dev)),
            (Some("devices"), Some(dev), Some("virtio"), None) => Ok(Self::VirtioDir(dev)),
            (Some("devices"), Some(dev), Some("virtio"), Some(file)) => {
                Ok(Self::VirtioFile(dev, file))
            }
            (Some("devices"), Some(dev), Some(file), None) => Ok(Self::DeviceFile(dev, file)),
            (Some("firmware"), None, None, None) => Ok(Self::Firmware),
            // New canonical path: /sys/firmware/acpi/{rsdp,tables/…}
            (Some("firmware"), Some("acpi"), None, None) => Ok(Self::AcpiDir),
            (Some("firmware"), Some("acpi"), Some("rsdp"), None) => Ok(Self::AcpiRsdpFile),
            (Some("firmware"), Some("acpi"), Some("tables"), None) => Ok(Self::AcpiTablesDir),
            (Some("firmware"), Some("acpi"), Some("tables"), Some(name)) => {
                Ok(Self::AcpiTableFile(name))
            }
            // Legacy alias kept for compatibility: /sys/firmware/acpi_tables/…
            (Some("firmware"), Some("acpi_tables"), None, None) => Ok(Self::AcpiTablesDir),
            (Some("firmware"), Some("acpi_tables"), Some(name), None) => {
                Ok(Self::AcpiTableFile(name))
            }
            (Some("firmware"), Some(file), None, None) => Ok(Self::FirmwareFile(file)),
            _ => Err(Errno::ENOENT),
        }
    }
}

struct StaticDirNode {
    ino: u64,
    entries: &'static [&'static str],
}

impl StaticDirNode {
    const fn new(ino: u64, entries: &'static [&'static str]) -> Self {
        Self { ino, entries }
    }
}

impl VfsNode for StaticDirNode {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }

    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }

    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat { mode: VfsStat::S_IFDIR | 0o555, size: 0, ino: self.ino, ..Default::default() })
    }

    fn readdir(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        super::write_readdir_entries(self.entries.iter().copied(), offset, buf)
    }
}

struct DevicesDirNode;

impl VfsNode for DevicesDirNode {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }

    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }

    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat { mode: VfsStat::S_IFDIR | 0o555, size: 0, ino: 301, ..Default::default() })
    }

    fn readdir(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let slots = device_slot_names();
        crate::ktrace!("Sysfs readdir found {} slot(s)", slots.len());
        let n = super::write_readdir_entries(slots.iter().map(|s| s.as_str()), offset, buf)?;
        crate::ktrace!("Sysfs readdir wrote {} bytes", n);
        Ok(n)
    }
}

struct DeviceDirNode {
    ino: u64,
    device_index: usize,
}

impl DeviceDirNode {
    fn new(device_index: usize, _entry: DeviceEntry) -> Self {
        Self { ino: 0x1000 + device_index as u64, device_index }
    }
}

impl VfsNode for DeviceDirNode {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }

    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }

    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat { mode: VfsStat::S_IFDIR | 0o555, size: 0, ino: self.ino, ..Default::default() })
    }

    fn readdir(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let mut entries = vec![
            "vendor", "device", "class", "status", "bar0", "bar1", "bar2", "bar3", "bar4", "bar5",
        ];

        // Check if it's a VirtIO device
        let reg = REGISTRY.lock();
        let is_virtio =
            reg.entry_copy(self.device_index).map(|e| e.vendor_id == 0x1af4).unwrap_or(false);
        drop(reg);

        if is_virtio {
            entries.push("virtio");
        }

        entries.push("kind");
        entries.push("ioports");

        super::write_readdir_entries(entries.into_iter(), offset, buf)
    }
}

struct StaticTextNode {
    data: Vec<u8>,
    ino: u64,
}

impl StaticTextNode {
    fn new(data: Vec<u8>, ino: u64) -> Self {
        Self { data, ino }
    }
}

impl VfsNode for StaticTextNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let off = offset as usize;
        if off >= self.data.len() {
            return Ok(0);
        }
        let avail = &self.data[off..];
        let n = avail.len().min(buf.len());
        buf[..n].copy_from_slice(&avail[..n]);
        Ok(n)
    }

    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EROFS)
    }

    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFREG | 0o444,
            size: self.data.len() as u64,
            ino: self.ino,
            ..Default::default()
        })
    }
}

fn device_slot_names() -> Vec<alloc::string::String> {
    let reg = REGISTRY.lock();
    let mut out = Vec::new();
    for index in 0..reg.len() {
        let Some(entry) = reg.entry_copy(index) else {
            continue;
        };
        out.push(slot_name(entry));
    }
    out
}

fn device_snapshot_text() -> Vec<u8> {
    use core::fmt::Write;

    let reg = REGISTRY.lock();
    let mut out = alloc::string::String::new();
    for index in 0..reg.len() {
        let Some(entry) = reg.entry_copy(index) else {
            continue;
        };
        let _ = writeln!(
            out,
            "{}\t{}\t0x{:04x}\t0x{:04x}\t0x{:02x}{:02x}{:02x}\tpresent",
            slot_name(entry),
            entry.kind,
            entry.vendor_id,
            entry.device_id,
            entry.class_code,
            entry.subclass,
            entry.prog_if
        );
    }
    out.into_bytes()
}

fn find_device_by_slot(slot: &str) -> SysResult<(usize, DeviceEntry)> {
    let reg = REGISTRY.lock();
    for index in 0..reg.len() {
        let Some(entry) = reg.entry_copy(index) else {
            continue;
        };
        if slot_name(entry) == slot {
            return Ok((index, entry));
        }
    }
    Err(Errno::ENOENT)
}

fn slot_name(entry: DeviceEntry) -> alloc::string::String {
    if let Some(loc) = entry.pci_location {
        format!("pci-0000:{:02x}:{:02x}.{}", loc.bus, loc.dev, loc.func)
    } else if !entry.ioport_ranges.is_empty() {
        format!("isa-{:04x}", entry.resource_id as u16)
    } else {
        format!("platform-{:08x}", entry.resource_id)
    }
}

fn lookup_device_file(
    device_index: usize,
    entry: DeviceEntry,
    file: &str,
) -> SysResult<StaticTextNode> {
    let ino_base = 0x2000 + (device_index as u64) * 16;
    let text = match file {
        "vendor" => format!("0x{:04x}\n", entry.vendor_id),
        "device" => format!("0x{:04x}\n", entry.device_id),
        "class" => {
            format!("0x{:02x}{:02x}{:02x}\n", entry.class_code, entry.subclass, entry.prog_if)
        }
        "status" => "present\n".into(),
        "bar0" => format!("0x{:x} 0x{:x}\n", entry.mmio_bars[0], entry.mmio_sizes[0]),
        "bar1" => format!("0x{:x} 0x{:x}\n", entry.mmio_bars[1], entry.mmio_sizes[1]),
        "bar2" => format!("0x{:x} 0x{:x}\n", entry.mmio_bars[2], entry.mmio_sizes[2]),
        "bar3" => format!("0x{:x} 0x{:x}\n", entry.mmio_bars[3], entry.mmio_sizes[3]),
        "bar4" => format!("0x{:x} 0x{:x}\n", entry.mmio_bars[4], entry.mmio_sizes[4]),
        "bar5" => format!("0x{:x} 0x{:x}\n", entry.mmio_bars[5], entry.mmio_sizes[5]),
        "kind" => format!("{}\n", entry.kind),
        "ioports" => {
            let mut s = alloc::string::String::new();
            for &(start, end) in entry.ioport_ranges {
                use core::fmt::Write;
                let _ = write!(s, "0x{:x}-0x{:x}\n", start, end);
            }
            s
        }
        _ => return Err(Errno::ENOENT),
    };
    Ok(StaticTextNode::new(text.into_bytes(), ino_base))
}

struct VirtioDirNode {
    ino: u64,
}

impl VirtioDirNode {
    fn new(device_index: usize, _entry: DeviceEntry) -> Self {
        Self { ino: 0x3000 + device_index as u64 }
    }
}

impl VfsNode for VirtioDirNode {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }

    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }

    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat { mode: VfsStat::S_IFDIR | 0o555, size: 0, ino: self.ino, ..Default::default() })
    }

    fn readdir(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        super::write_readdir_entries(
            [
                "common_bar",
                "common_offset",
                "notify_bar",
                "notify_offset",
                "notify_multiplier",
                "isr_bar",
                "isr_offset",
                "device_bar",
                "device_offset",
            ]
            .into_iter(),
            offset,
            buf,
        )
    }
}

fn lookup_virtio_file(
    device_index: usize,
    entry: DeviceEntry,
    file: &str,
) -> SysResult<StaticTextNode> {
    use crate::virtio::pci::VirtioCapabilityType;

    let loc = entry.pci_location.ok_or(Errno::ENODEV)?;
    let runtime = crate::runtime_base();
    let mut virtio_caps = Vec::new();

    // Scan capabilities to find VirtIO ones
    let cap_ptr_initial = {
        let status = runtime.pci_cfg_read32(loc.bus, loc.dev, loc.func, 0x04)? >> 16;
        let cap_ptr = (runtime.pci_cfg_read32(loc.bus, loc.dev, loc.func, 0x34)? & 0xFF) as u8;
        if (status & 0x10) == 0 {
            return Err(Errno::ENOSYS);
        }
        cap_ptr
    };

    let mut cap_ptr = cap_ptr_initial;
    while cap_ptr != 0 {
        let cap_header = runtime.pci_cfg_read32(loc.bus, loc.dev, loc.func, cap_ptr)?;
        let cap_id = (cap_header & 0xFF) as u8;
        let next_ptr = ((cap_header >> 8) & 0xFF) as u8;
        if cap_id == 0x09 {
            // Vendor specific (VirtIO)
            let len = ((cap_header >> 16) & 0xFF) as u8;
            let cap_type = ((cap_header >> 24) & 0xFF) as u8;
            let cap_info = runtime.pci_cfg_read32(loc.bus, loc.dev, loc.func, cap_ptr + 4)?;
            let bar = (cap_info & 0xFF) as u8;

            let mut notify_off_multiplier = 0u32;
            let offset = runtime.pci_cfg_read32(loc.bus, loc.dev, loc.func, cap_ptr + 8)?;
            let length = runtime.pci_cfg_read32(loc.bus, loc.dev, loc.func, cap_ptr + 12)?;

            if cap_type == VirtioCapabilityType::NotifyCfg as u8 && len >= 20 {
                notify_off_multiplier =
                    runtime.pci_cfg_read32(loc.bus, loc.dev, loc.func, cap_ptr + 16)?;
            }

            virtio_caps.push(crate::virtio::pci::VirtioCapability {
                cap_type,
                bar,
                offset,
                length,
                notify_off_multiplier,
            });
        }
        cap_ptr = next_ptr;
    }

    let find_cap = |t: VirtioCapabilityType| virtio_caps.iter().find(|c| c.cap_type == t as u8);

    let text = match file {
        "common_bar" => format!(
            "{}\n",
            find_cap(VirtioCapabilityType::CommonCfg).map(|c| c.bar).unwrap_or(0xFF)
        ),
        "common_offset" => format!(
            "0x{:x}\n",
            find_cap(VirtioCapabilityType::CommonCfg).map(|c| c.offset).unwrap_or(0)
        ),
        "notify_bar" => format!(
            "{}\n",
            find_cap(VirtioCapabilityType::NotifyCfg).map(|c| c.bar).unwrap_or(0xFF)
        ),
        "notify_offset" => format!(
            "0x{:x}\n",
            find_cap(VirtioCapabilityType::NotifyCfg).map(|c| c.offset).unwrap_or(0)
        ),
        "notify_multiplier" => format!(
            "{}\n",
            find_cap(VirtioCapabilityType::NotifyCfg).map(|c| c.notify_off_multiplier).unwrap_or(0)
        ),
        "isr_bar" => {
            format!("{}\n", find_cap(VirtioCapabilityType::IsrCfg).map(|c| c.bar).unwrap_or(0xFF))
        }
        "isr_offset" => format!(
            "0x{:x}\n",
            find_cap(VirtioCapabilityType::IsrCfg).map(|c| c.offset).unwrap_or(0)
        ),
        "device_bar" => format!(
            "{}\n",
            find_cap(VirtioCapabilityType::DeviceCfg).map(|c| c.bar).unwrap_or(0xFF)
        ),
        "device_offset" => format!(
            "0x{:x}\n",
            find_cap(VirtioCapabilityType::DeviceCfg).map(|c| c.offset).unwrap_or(0)
        ),
        _ => return Err(Errno::ENOENT),
    };

    let ino_base = 0x4000 + (device_index as u64) * 32;
    Ok(StaticTextNode::new(text.into_bytes(), ino_base))
}

// ── ACPI table sysfs helpers ─────────────────────────────────────────────────

/// Enumerate ACPI tables from boot_info and return them with their sysfs names.
fn acpi_entries() -> alloc::vec::Vec<(usize, alloc::string::String, crate::acpi_tables::AcpiEntry)>
{
    let info = match crate::boot_info::get() {
        Some(i) => i,
        None => return alloc::vec![],
    };
    let rsdp_phys = match info.acpi_rsdp {
        Some(p) => p,
        None => return alloc::vec![],
    };
    let entries = crate::acpi_tables::enumerate(rsdp_phys, info.hhdm_offset);
    let names = crate::acpi_tables::sysfs_names(&entries);
    names.into_iter().zip(entries).map(|((idx, name), entry)| (idx, name, entry)).collect()
}

/// Get raw bytes for a specific ACPI table by its sysfs name (e.g. `APIC`,
/// `DSDT`, `SSDT1`).
fn acpi_table_data(name: &str) -> Option<alloc::vec::Vec<u8>> {
    for (_idx, sname, entry) in acpi_entries() {
        if sname == name {
            return Some(entry.data);
        }
    }
    None
}

/// Stable inode number derived from the table name.
fn acpi_table_ino(name: &str) -> u64 {
    let mut h = 0x5000u64;
    for &b in name.as_bytes() {
        h = h.wrapping_mul(31).wrapping_add(b as u64);
    }
    h
}

/// Dynamic directory node listing available ACPI tables.
struct AcpiTablesDirNode;

impl VfsNode for AcpiTablesDirNode {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }

    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }

    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat { mode: VfsStat::S_IFDIR | 0o555, size: 0, ino: 0x5ffe, ..Default::default() })
    }

    fn readdir(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let entries = acpi_entries();
        let names: alloc::vec::Vec<alloc::string::String> =
            entries.into_iter().map(|(_, name, _)| name).collect();
        super::write_readdir_entries(names.iter().map(|s| s.as_str()), offset, buf)
    }
}
