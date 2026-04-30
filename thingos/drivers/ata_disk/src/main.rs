//! ATA Disk Driver (VFS-native)
//!
//! Userspace driver that detects ATA/ATAPI devices on IDE controllers
//! and exposes them as VFS files in `/dev/storage/`.
#![no_std]
#![no_main]
extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

use abi::driver_interface::{
    BusKind, DRIVER_DESCRIPTOR_ABI_VERSION, DRIVER_INTERFACE_ABI_VERSION, DeviceInfo, DriverClass,
    DriverDescriptor, DriverEntryCtx, DriverInterfaceV1, ProbeResult, Status,
};
use abi::errors::{Errno, SysResult};
use abi::vfs_rpc::VfsRpcOp;
use abi::vm::{VmBacking, VmMapFlags, VmMapReq, VmProt};
use ipc_helpers::provider::{ProviderLoop, ProviderRequest, ProviderResponse};
use stem::abi::module_manifest::{MANIFEST_MAGIC, ManifestHeader, ModuleKind};
use stem::block::{BlockDevice, BlockError};
use stem::syscall::vfs::{vfs_handle_from_port, vfs_mount};
use stem::syscall::{device_claim, ioport_read, ioport_write, port_create};
use stem::{debug, error, info, warn};

const THINGOS_DRIVER_NAME: &[u8] = b"ata_disk";

#[cfg(target_arch = "x86_64")]
unsafe extern "C" {
    fn thingos_driver_start_safe(ctx: *const DriverEntryCtx) -> Status;
}

#[unsafe(no_mangle)]
#[used]
pub static THINGOS_DRIVER: DriverDescriptor = DriverDescriptor {
    abi_version: DRIVER_DESCRIPTOR_ABI_VERSION,
    driver_name_ptr: THINGOS_DRIVER_NAME.as_ptr(),
    driver_name_len: THINGOS_DRIVER_NAME.len(),
    driver_class: DriverClass::Block,
    flags: 0,
    probe: thingos_driver_probe,
    #[cfg(target_arch = "x86_64")]
    start: thingos_driver_start_safe,
    #[cfg(not(target_arch = "x86_64"))]
    start: thingos_driver_start_rust,
};

#[unsafe(no_mangle)]
#[used]
pub static THING_DRIVER_V1: DriverInterfaceV1 = DriverInterfaceV1 {
    abi_version: DRIVER_INTERFACE_ABI_VERSION,
    flags: 1, // DRIVER_FLAG_PCI
    vendor_id: 0,
    device_id: 0,
    class_code: 0x010100, // Mass Storage : IDE Controller
    class_mask: 0xFFFF00, // Match Class and Subclass
    entry_symbol: [0u8; 32],
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

#[unsafe(no_mangle)]
unsafe extern "C" fn thingos_driver_start_rust(ctx: *const DriverEntryCtx) -> Status {
    main(ctx as usize)
}

unsafe extern "C" fn thingos_driver_probe(dev: *const DeviceInfo, out: *mut ProbeResult) -> Status {
    if dev.is_null() || out.is_null() {
        return Status::InvalidArgument;
    }
    let dev = &*dev;
    let out = &mut *out;

    let is_pci_ide = dev.bus == BusKind::Pci as u32 && (dev.class_code & 0xFFFF00) == 0x010100;
    let is_isa = dev.bus == BusKind::Isa as u32 || dev.bus == BusKind::Unknown as u32;

    if is_pci_ide || is_isa {
        out.matched = 1;
        out.score = if is_pci_ide { 800 } else { 700 };
        out.claimed_class = DriverClass::Block;
        out.flags = 0;
        Status::Ok
    } else {
        out.matched = 0;
        out.score = 0;
        out.claimed_class = DriverClass::Block;
        out.flags = 0;
        Status::NoMatch
    }
}

#[unsafe(link_section = ".thing_manifest")]
#[unsafe(no_mangle)]
#[used]
pub static MANIFEST: ManifestHeader = ManifestHeader {
    magic: MANIFEST_MAGIC,
    kind: ModuleKind::Driver,
    device_kind: *b"dev.storage.Ide\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    version: 1,
    _reserved: 0,
};

// ATA constants
const ATA_PRIMARY_IO: u16 = 0x1F0;
const ATA_PRIMARY_CTRL: u16 = 0x3F6;
const ATA_SECONDARY_IO: u16 = 0x170;
const ATA_SECONDARY_CTRL: u16 = 0x376;

const ATA_REG_DATA: u16 = 0;
const ATA_REG_SECCOUNT: u16 = 2;
const ATA_REG_LBA_LO: u16 = 3;
const ATA_REG_LBA_MID: u16 = 4;
const ATA_REG_LBA_HI: u16 = 5;
const ATA_REG_DRIVE: u16 = 6;
const ATA_REG_STATUS: u16 = 7;
const ATA_REG_COMMAND: u16 = 7;

const ATA_CMD_IDENTIFY: u8 = 0xEC;
const ATA_CMD_IDENTIFY_PACKET: u8 = 0xA1;
const ATA_CMD_PACKET: u8 = 0xA0;
const ATA_CMD_READ_SECTORS: u8 = 0x20;
const ATA_CMD_READ_SECTORS_EXT: u8 = 0x24;

const ATAPI_SIG_MID: u8 = 0x14;
const ATAPI_SIG_HI: u8 = 0xEB;

const ATA_SR_BSY: u8 = 0x80;
const ATA_SR_DRQ: u8 = 0x08;
const ATA_SR_ERR: u8 = 0x01;

const S_IFREG: u32 = 0o100000;
const S_IFDIR: u32 = 0o040000;

// ── Hardware Helpers ─────────────────────────────────────────────────────────

fn ata_inb(port: u16) -> u8 {
    ioport_read(port as usize, 1) as u8
}
fn ata_inw(port: u16) -> u16 {
    ioport_read(port as usize, 2) as u16
}
fn ata_outb(port: u16, val: u8) {
    ioport_write(port as usize, val as usize, 1);
}
fn ata_outw(port: u16, val: u16) {
    ioport_write(port as usize, val as usize, 2);
}

fn wait_bsy_clear(io_base: u16) -> bool {
    for _ in 0..100000 {
        if ata_inb(io_base + ATA_REG_STATUS) & ATA_SR_BSY == 0 {
            return true;
        }
    }
    false
}

// ── ATA Disk Implementation ──────────────────────────────────────────────────

struct AtaDisk {
    io_base: u16,
    is_slave: bool,
    sector_count: u64,
    sector_size: u32,
    supports_lba48: bool,
}

impl BlockDevice for AtaDisk {
    fn sector_size(&self) -> u64 {
        self.sector_size as u64
    }
    fn sector_count(&self) -> Option<u64> {
        Some(self.sector_count)
    }

    fn read_sectors(&self, lba: u64, count: u64, buf: &mut [u8]) -> Result<(), BlockError> {
        if count == 0 {
            return Ok(());
        }
        if !self.supports_lba48 && lba > 0x0FFFFFFF {
            return Err(BlockError::OutOfRange);
        }

        let drive_sel = if self.is_slave { 0xF0 } else { 0xE0 };

        // For simplicity, handle multiple sectors by calling hardware for each or batching
        // Here we'll do the LBA setup once and loop DRQ.
        if self.supports_lba48 {
            ata_outb(self.io_base + ATA_REG_DRIVE, drive_sel);
            ata_outb(self.io_base + ATA_REG_SECCOUNT, ((count >> 8) & 0xFF) as u8);
            ata_outb(self.io_base + ATA_REG_LBA_LO, ((lba >> 24) & 0xFF) as u8);
            ata_outb(self.io_base + ATA_REG_LBA_MID, ((lba >> 32) & 0xFF) as u8);
            ata_outb(self.io_base + ATA_REG_LBA_HI, ((lba >> 40) & 0xFF) as u8);
            ata_outb(self.io_base + ATA_REG_SECCOUNT, (count & 0xFF) as u8);
            ata_outb(self.io_base + ATA_REG_LBA_LO, (lba & 0xFF) as u8);
            ata_outb(self.io_base + ATA_REG_LBA_MID, ((lba >> 8) & 0xFF) as u8);
            ata_outb(self.io_base + ATA_REG_LBA_HI, ((lba >> 16) & 0xFF) as u8);
            ata_outb(self.io_base + ATA_REG_COMMAND, ATA_CMD_READ_SECTORS_EXT);
        } else {
            let lba28 = lba as u32;
            ata_outb(self.io_base + ATA_REG_DRIVE, drive_sel | ((lba28 >> 24) & 0x0F) as u8);
            ata_outb(self.io_base + ATA_REG_SECCOUNT, count as u8);
            ata_outb(self.io_base + ATA_REG_LBA_LO, (lba28 & 0xFF) as u8);
            ata_outb(self.io_base + ATA_REG_LBA_MID, ((lba28 >> 8) & 0xFF) as u8);
            ata_outb(self.io_base + ATA_REG_LBA_HI, ((lba28 >> 16) & 0xFF) as u8);
            ata_outb(self.io_base + ATA_REG_COMMAND, ATA_CMD_READ_SECTORS);
        }

        let mut offset = 0;
        for _ in 0..count {
            loop {
                let status = ata_inb(self.io_base + ATA_REG_STATUS);
                if status & ATA_SR_ERR != 0 {
                    return Err(BlockError::IoError);
                }
                if status & ATA_SR_DRQ != 0 {
                    break;
                }
            }
            for _ in 0..256 {
                let word = ata_inw(self.io_base + ATA_REG_DATA);
                if offset + 1 < buf.len() {
                    buf[offset] = (word & 0xFF) as u8;
                    buf[offset + 1] = (word >> 8) as u8;
                    offset += 2;
                }
            }
        }
        Ok(())
    }
}

// ── ATAPI Device Implementation ──────────────────────────────────────────────

struct AtapiDevice {
    io_base: u16,
    ctrl_base: u16,
    is_slave: bool,
}

impl BlockDevice for AtapiDevice {
    fn sector_size(&self) -> u64 {
        2048
    }

    fn read_sectors(&self, lba: u64, count: u64, buf: &mut [u8]) -> Result<(), BlockError> {
        if count == 0 {
            return Ok(());
        }
        let mut offset = 0;
        for i in 0..count {
            self.read_one_sector(lba + i, &mut buf[offset..offset + 2048])?;
            offset += 2048;
        }
        Ok(())
    }
}

impl AtapiDevice {
    fn read_one_sector(&self, lba: u64, buf: &mut [u8]) -> Result<(), BlockError> {
        let drive_sel = if self.is_slave { 0xB0 } else { 0xA0 };
        ata_outb(self.io_base + ATA_REG_DRIVE, drive_sel);
        for _ in 0..4 {
            ata_inb(self.ctrl_base);
        }
        if !wait_bsy_clear(self.io_base) {
            return Err(BlockError::NotReady);
        }

        let byte_count = 2048u16;
        ata_outb(self.io_base + ATA_REG_LBA_MID, (byte_count & 0xFF) as u8);
        ata_outb(self.io_base + ATA_REG_LBA_HI, ((byte_count >> 8) & 0xFF) as u8);
        ata_outb(self.io_base + ATA_REG_COMMAND, ATA_CMD_PACKET);

        for _ in 0..100000 {
            let status = ata_inb(self.io_base + ATA_REG_STATUS);
            if status & ATA_SR_ERR != 0 {
                return Err(BlockError::IoError);
            }
            if status & ATA_SR_DRQ != 0 {
                break;
            }
        }

        let lba32 = lba as u32;
        let packet: [u16; 6] = [
            0x00A8, // READ(12)
            ((lba32 >> 24) as u16) << 8 | ((lba32 >> 16) as u16 & 0xFF),
            ((lba32 >> 8) as u16 & 0xFF) << 8 | (lba32 as u16 & 0xFF),
            0, // length high
            1, // length low (1 sector)
            0,
        ];
        for word in packet {
            ata_outw(self.io_base + ATA_REG_DATA, word);
        }

        loop {
            let status = ata_inb(self.io_base + ATA_REG_STATUS);
            if status & ATA_SR_ERR != 0 {
                return Err(BlockError::IoError);
            }
            if status & ATA_SR_BSY == 0 && status & ATA_SR_DRQ != 0 {
                break;
            }
        }

        for i in 0..1024 {
            let word = ata_inw(self.io_base + ATA_REG_DATA);
            buf[i * 2] = (word & 0xFF) as u8;
            buf[i * 2 + 1] = (word >> 8) as u8;
        }
        Ok(())
    }
}

// ── VFS Provider ─────────────────────────────────────────────────────────────

struct StorageProvider {
    device: Arc<dyn BlockDevice>,
}

impl StorageProvider {
    fn handle_rpc(&self, req: &ProviderRequest) -> ProviderResponse {
        match req.op {
            VfsRpcOp::Lookup => ProviderResponse::ok_u64(1),
            VfsRpcOp::Stat => {
                let size = self.device.sector_count().unwrap_or(0) * self.device.sector_size();
                ProviderResponse::ok_stat(S_IFREG | 0o444, size, 1)
            }
            VfsRpcOp::Read => {
                let offset = u64::from_le_bytes(req.payload[0..8].try_into().unwrap());
                let len = u32::from_le_bytes(req.payload[8..12].try_into().unwrap()) as usize;

                let sector_size = self.device.sector_size();
                let mut data = Vec::with_capacity(len);
                data.resize(len, 0);

                // Handle simple sector-aligned read for now (good enough for iso9660d)
                let lba = offset / sector_size;
                let count = (len as u64 + sector_size - 1) / sector_size;

                // If not aligned, we'd need a bounce buffer, but let's assume alignment for iso9660
                let mut bounce = Vec::with_capacity((count * sector_size) as usize);
                bounce.resize((count * sector_size) as usize, 0);

                match self.device.read_sectors(lba, count, &mut bounce) {
                    Ok(_) => {
                        let inner_off = (offset % sector_size) as usize;
                        ProviderResponse::ok_bytes(&bounce[inner_off..inner_off + len])
                    }
                    Err(_) => ProviderResponse::err(Errno::EIO),
                }
            }
            _ => ProviderResponse::err(Errno::ENOSYS),
        }
    }
}

// ── Main Entry ───────────────────────────────────────────────────────────────

#[stem::main]
fn main(boot_fd: usize) -> ! {
    info!("ATA_DISK: Starting Generic IDE/ATAPI VFS driver");

    let primary_io = ATA_PRIMARY_IO;
    let primary_ctrl = ATA_PRIMARY_CTRL;
    let secondary_io = ATA_SECONDARY_IO;
    let secondary_ctrl = ATA_SECONDARY_CTRL;

    if boot_fd != 0 {
        let req = VmMapReq {
            addr_hint: 0,
            len: 4096,
            prot: VmProt::READ | VmProt::USER,
            flags: VmMapFlags::empty(),
            backing: VmBacking::File { thing: boot_fd as u32, offset: 0 },
        };
        if let Ok(resp) = stem::syscall::vm_map(&req) {
            let path_owned = {
                let ctx = unsafe { &*(resp.addr as *const DriverEntryCtx) };
                if ctx.version == 1 {
                    let path = ctx.device_path_str();
                    if !path.is_empty() { Some(path.to_string()) } else { None }
                } else {
                    None
                }
            };
            let _ = stem::syscall::vm_unmap(resp.addr, 4096);
            if let Some(path) = path_owned {
                let _ = device_claim(&path);
                // In a real IDE controller we'd read the BARs here if not using legacy ports,
                // but for now we stick to defaults if they work.
            }
        }
    }

    let mut devices = Vec::new();

    // Primary Master
    if let Some(disk) = identify_ata(primary_io, primary_ctrl, false) {
        devices.push((Arc::new(disk) as Arc<dyn BlockDevice>, "ide_0_0"));
    } else if let Some(dev) = identify_atapi_probe(primary_io, primary_ctrl, false) {
        devices.push((Arc::new(dev) as Arc<dyn BlockDevice>, "atapi_0_0"));
    }
    // Primary Slave
    if let Some(disk) = identify_ata(primary_io, primary_ctrl, true) {
        devices.push((Arc::new(disk) as Arc<dyn BlockDevice>, "ide_0_1"));
    } else if let Some(dev) = identify_atapi_probe(primary_io, primary_ctrl, true) {
        devices.push((Arc::new(dev) as Arc<dyn BlockDevice>, "atapi_0_1"));
    }
    // Secondary Master
    if let Some(disk) = identify_ata(secondary_io, secondary_ctrl, false) {
        devices.push((Arc::new(disk) as Arc<dyn BlockDevice>, "ide_1_0"));
    } else if let Some(dev) = identify_atapi_probe(secondary_io, secondary_ctrl, false) {
        devices.push((Arc::new(dev) as Arc<dyn BlockDevice>, "atapi_1_0"));
    }
    // Secondary Slave
    if let Some(disk) = identify_ata(secondary_io, secondary_ctrl, true) {
        devices.push((Arc::new(disk) as Arc<dyn BlockDevice>, "ide_1_1"));
    } else if let Some(dev) = identify_atapi_probe(secondary_io, secondary_ctrl, true) {
        devices.push((Arc::new(dev) as Arc<dyn BlockDevice>, "atapi_1_1"));
    }

    info!("ATA_DISK: Found {} storage device(s)", devices.len());

    for (device, name) in devices {
        let path = format!("/dev/storage/{}", name);
        let provider = Arc::new(StorageProvider { device });

        let (vfs_write, vfs_read) = port_create(65536).expect("ata_disk: port_create failed");
        vfs_mount(vfs_write, &path).expect("ata_disk: vfs_mount failed");
        info!("ATA_DISK: Mounted {} at {}", name, path);

        stem::thread::spawn_task_detached(move || {
            let mut ploop = ProviderLoop::new(vfs_read);
            loop {
                let req = match ploop.next_request() {
                    Ok(req) => req,
                    Err(err) => {
                        warn!("ATA_DISK: provider loop closed for {}: {:?}", name, err);
                        break;
                    }
                };
                let resp = provider.handle_rpc(&req);
                let _ = ploop.send_response(&req, resp);
            }
        })
        .expect("ata_disk: thread spawn failed");
    }

    loop {
        stem::syscall::sleep_ms(60000);
    }
}

fn identify_ata(io: u16, ctrl: u16, slave: bool) -> Option<AtaDisk> {
    let drive_sel = if slave { 0xB0 } else { 0xA0 };
    ata_outb(io + ATA_REG_DRIVE, drive_sel);
    for _ in 0..4 {
        ata_inb(ctrl);
    }
    ata_outb(io + ATA_REG_SECCOUNT, 0);
    ata_outb(io + ATA_REG_LBA_LO, 0);
    ata_outb(io + ATA_REG_LBA_MID, 0);
    ata_outb(io + ATA_REG_LBA_HI, 0);
    ata_outb(io + ATA_REG_COMMAND, ATA_CMD_IDENTIFY);

    let status = ata_inb(io + ATA_REG_STATUS);
    if status == 0 || status == 0xFF {
        return None;
    }
    if !wait_bsy_clear(io) {
        return None;
    }

    let lba_mid = ata_inb(io + ATA_REG_LBA_MID);
    let lba_hi = ata_inb(io + ATA_REG_LBA_HI);
    if lba_mid != 0 || lba_hi != 0 {
        return None;
    }

    loop {
        let s = ata_inb(io + ATA_REG_STATUS);
        if s & ATA_SR_DRQ != 0 {
            break;
        }
        if s & ATA_SR_ERR != 0 {
            return None;
        }
    }

    let mut ident = [0u16; 256];
    for i in 0..256 {
        ident[i] = ata_inw(io + ATA_REG_DATA);
    }

    let lba48 = (ident[83] & (1 << 10)) != 0;
    let count = if lba48 {
        (ident[100] as u64)
            | ((ident[101] as u64) << 16)
            | ((ident[102] as u64) << 32)
            | ((ident[103] as u64) << 48)
    } else {
        (ident[60] as u64) | ((ident[61] as u64) << 16)
    };

    Some(AtaDisk {
        io_base: io,
        is_slave: slave,
        sector_count: count,
        sector_size: 512,
        supports_lba48: lba48,
    })
}

fn identify_atapi_probe(io: u16, ctrl: u16, slave: bool) -> Option<AtapiDevice> {
    let drive_sel = if slave { 0xB0 } else { 0xA0 };
    ata_outb(io + ATA_REG_DRIVE, drive_sel);
    for _ in 0..4 {
        ata_inb(ctrl);
    }
    ata_outb(io + ATA_REG_COMMAND, ATA_CMD_IDENTIFY);
    let status = ata_inb(io + ATA_REG_STATUS);
    if status == 0 || status == 0xFF {
        return None;
    }
    if !wait_bsy_clear(io) {
        return None;
    }

    let lba_mid = ata_inb(io + ATA_REG_LBA_MID);
    let lba_hi = ata_inb(io + ATA_REG_LBA_HI);
    if lba_mid == ATAPI_SIG_MID && lba_hi == ATAPI_SIG_HI {
        Some(AtapiDevice { io_base: io, ctrl_base: ctrl, is_slave: slave })
    } else {
        None
    }
}
