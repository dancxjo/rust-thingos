//! AHCI (SATA) Disk Driver (VFS-native)
//!
//! Userspace driver for AHCI SATA controllers. Detects SATA devices
//! and exposes them as VFS files in `/dev/storage/`.
#![no_std]
#![no_main]
extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;

use abi::driver_interface::{
    BusKind, DRIVER_DESCRIPTOR_ABI_VERSION, DeviceInfo, DriverClass, DriverDescriptor,
    DriverEntryCtx, ProbeResult, Status,
};
use abi::errors::Errno;
use abi::vfs_rpc::VfsRpcOp;
use abi::vm::{VmBacking, VmMapFlags, VmMapReq, VmProt};
use ipc_helpers::provider::{ProviderLoop, ProviderRequest, ProviderResponse};
use stem::abi::module_manifest::{MANIFEST_MAGIC, ManifestHeader, ModuleKind};
use stem::block::{BlockDevice, BlockError};
use stem::syscall::vfs::vfs_mount;
use stem::syscall::{
    device_alloc_dma, device_claim, device_dma_phys, device_map_mmio, port_create,
};
use stem::{error, info, trace, warn, yield_now};

const THINGOS_DRIVER_NAME: &[u8] = b"ahci_disk";

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

unsafe extern "C" fn thingos_driver_probe(dev: *const DeviceInfo, out: *mut ProbeResult) -> Status {
    if dev.is_null() || out.is_null() {
        return Status::InvalidArgument;
    }
    let dev = &*dev;
    let out = &mut *out;
    let is_match = dev.bus == BusKind::Pci as u32 && (dev.class_code & 0x00ff_ffff) == 0x010601;
    out.matched = if is_match { 1 } else { 0 };
    out.score = if is_match { 900 } else { 0 };
    out.claimed_class = DriverClass::Block;
    out.flags = 0;
    if is_match { Status::Ok } else { Status::NoMatch }
}

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

#[unsafe(link_section = ".thing_manifest")]
#[unsafe(no_mangle)]
#[used]
pub static MANIFEST: ManifestHeader = ManifestHeader {
    magic: MANIFEST_MAGIC,
    kind: ModuleKind::Driver,
    device_kind: *b"dev.storage.Ahci\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    version: 1,
    _reserved: 0,
};

// AHCI Registers
const HBA_CAP: usize = 0x00;
const HBA_GHC: usize = 0x04;
const HBA_PI: usize = 0x0C;
const HBA_PORT_BASE: usize = 0x100;

const PORT_CLB: usize = 0x00;
const PORT_CLBU: usize = 0x04;
const PORT_FB: usize = 0x08;
const PORT_FBU: usize = 0x0C;
const PORT_IS: usize = 0x10;
const PORT_CMD: usize = 0x18;
const PORT_TFD: usize = 0x20;
const PORT_SIG: usize = 0x24;
const PORT_SSTS: usize = 0x28;
const PORT_SERR: usize = 0x30;
const PORT_SACT: usize = 0x34;
const PORT_CI: usize = 0x38;

const S_IFREG: u32 = 0o100000;
const S_IFDIR: u32 = 0o040000;

const SATA_SIG_ATA: u32 = 0x00000101;
const SATA_SIG_ATAPI: u32 = 0xEB140101;

const PORT_CMD_ST: u32 = 1 << 0;
const PORT_CMD_FRE: u32 = 1 << 4;
const PORT_CMD_FR: u32 = 1 << 14;
const PORT_CMD_CR: u32 = 1 << 15;
const FIS_TYPE_REG_H2D: u8 = 0x27;
const ATA_CMD_PACKET: u8 = 0xA0;

const OFFSET_CMD_LIST: usize = 0x000;
const OFFSET_FIS: usize = 0x400;
const OFFSET_CMD_TABLE: usize = 0x500;
const OFFSET_DATA: usize = 0x600;

#[repr(C, packed)]
struct CommandHeader {
    cfl: u8,
    pm: u8,
    prdtl: u16,
    prdbc: u32,
    ctba: u32,
    ctbau: u32,
    reserved: [u32; 4],
}

#[repr(C, packed)]
struct CommandTable {
    cfis: [u8; 64],
    acmd: [u8; 16],
    reserved: [u8; 48],
    prdt: [PrdtEntry; 1],
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct PrdtEntry {
    dba: u32,
    dbau: u32,
    reserved: u32,
    dbc: u32,
}

fn mmio_read32(base: u64, offset: usize) -> u32 {
    unsafe { core::ptr::read_volatile((base as usize + offset) as *const u32) }
}
fn mmio_write32(base: u64, offset: usize, val: u32) {
    unsafe { core::ptr::write_volatile((base as usize + offset) as *mut u32, val) }
}
fn port_base(hba_base: u64, port: u32) -> u64 {
    hba_base + HBA_PORT_BASE as u64 + (port as u64 * 0x80)
}

fn wait_port_cmd_clear(pb: u64, mask: u32, iterations: u32) -> bool {
    for i in 0..iterations {
        if mmio_read32(pb, PORT_CMD) & mask == 0 {
            return true;
        }
        if i % 1000 == 0 {
            yield_now();
        }
    }
    false
}

fn stop_port_engine(pb: u64) -> bool {
    let cmd = mmio_read32(pb, PORT_CMD);
    mmio_write32(pb, PORT_CMD, cmd & !PORT_CMD_ST);
    if !wait_port_cmd_clear(pb, PORT_CMD_CR, 100000) {
        return false;
    }

    let cmd = mmio_read32(pb, PORT_CMD);
    mmio_write32(pb, PORT_CMD, cmd & !PORT_CMD_FRE);
    wait_port_cmd_clear(pb, PORT_CMD_FR, 100000)
}

fn start_port_engine(pb: u64) {
    let cmd = mmio_read32(pb, PORT_CMD);
    mmio_write32(pb, PORT_CMD, cmd | PORT_CMD_FRE);
    let cmd = mmio_read32(pb, PORT_CMD);
    mmio_write32(pb, PORT_CMD, cmd | PORT_CMD_ST);
}

struct AhciDevice {
    mmio_base: u64,
    port: u32,
    dma_virt: u64,
    dma_phys: u64,
    is_atapi: bool,
}

impl BlockDevice for AhciDevice {
    fn sector_size(&self) -> u64 {
        if self.is_atapi { 2048 } else { 512 }
    }
    fn sector_count(&self) -> Option<u64> {
        Some(0)
    } // Unknown for now

    fn read_sectors(&self, lba: u64, count: u64, buf: &mut [u8]) -> Result<(), BlockError> {
        if count == 0 {
            return Ok(());
        }
        let mut current_lba = lba;
        let mut offset = 0;
        let ss = self.sector_size() as usize;
        for _ in 0..count {
            if self.is_atapi {
                self.read_atapi_sector(current_lba, &mut buf[offset..offset + ss])?;
            } else {
                // ATA implementation omitted for brevity or placeholder
                return Err(BlockError::NotSupported);
            }
            current_lba += 1;
            offset += ss;
        }
        Ok(())
    }
}

impl AhciDevice {
    fn read_atapi_sector(&self, lba: u64, buf: &mut [u8]) -> Result<(), BlockError> {
        let pb = port_base(self.mmio_base, self.port);
        mmio_write32(pb, PORT_IS, 0xFFFFFFFF);
        mmio_write32(pb, PORT_SERR, 0xFFFFFFFF);
        mmio_write32(pb, PORT_SACT, 0);

        let ds = 2048;
        let prdt = PrdtEntry {
            dba: (self.dma_phys as usize + OFFSET_DATA) as u32,
            dbau: ((self.dma_phys as usize + OFFSET_DATA) >> 32) as u32,
            reserved: 0,
            dbc: (ds - 1) as u32,
        };

        unsafe {
            let tbl = &mut *((self.dma_virt as usize + OFFSET_CMD_TABLE) as *mut CommandTable);
            tbl.cfis.fill(0);
            tbl.cfis[0] = FIS_TYPE_REG_H2D;
            tbl.cfis[1] = 0x80;
            tbl.cfis[2] = ATA_CMD_PACKET;
            tbl.cfis[3] = 1;
            tbl.cfis[5] = (ds as u32 & 0xFF) as u8;
            tbl.cfis[6] = ((ds as u32 >> 8) & 0xFF) as u8;

            tbl.acmd.fill(0);
            let lba32 = lba as u32;
            tbl.acmd[0] = 0x28; // READ(10)
            tbl.acmd[2] = (lba32 >> 24) as u8;
            tbl.acmd[3] = (lba32 >> 16) as u8;
            tbl.acmd[4] = (lba32 >> 8) as u8;
            tbl.acmd[5] = lba32 as u8;
            tbl.acmd[8] = 1; // 1 sector
            tbl.prdt[0] = prdt;

            let hdr = &mut *((self.dma_virt as usize + OFFSET_CMD_LIST) as *mut CommandHeader);
            hdr.cfl = 5 | 0x20; // 5 dwords + ATAPI bit
            hdr.pm = 0;
            hdr.prdtl = 1;
            hdr.prdbc = 0;
            hdr.ctba = (self.dma_phys as usize + OFFSET_CMD_TABLE) as u32;
            hdr.ctbau = ((self.dma_phys as usize + OFFSET_CMD_TABLE) >> 32) as u32;
        }

        let mut timeout = 200000;
        while (mmio_read32(pb, PORT_TFD) & 0x80) != 0 && timeout > 0 {
            timeout -= 1;
            if timeout % 1000 == 0 {
                yield_now();
            }
        }
        if timeout == 0 {
            let is = mmio_read32(pb, PORT_IS);
            let tfd = mmio_read32(pb, PORT_TFD);
            error!(
                "AHCI: port {} not ready for ATAPI packet is={:#x} tfd={:#x}",
                self.port, is, tfd
            );
            return Err(BlockError::NotReady);
        }

        if mmio_read32(pb, PORT_CI) & 1 != 0 {
            mmio_write32(pb, PORT_CI, 0);
        }

        trace!("AHCI: sending command for LBA {}", lba);
        mmio_write32(pb, PORT_CI, 1);
        let mut loop_timeout = 200000;
        loop {
            let ci = mmio_read32(pb, PORT_CI);
            if ci & 1 == 0 {
                break;
            }
            if mmio_read32(pb, PORT_IS) & (1 << 30) != 0 {
                error!("AHCI: IoError on port {}", self.port);
                return Err(BlockError::IoError);
            }
            loop_timeout -= 1;
            if loop_timeout % 1000 == 0 {
                yield_now();
            }
            if loop_timeout == 0 {
                let is = mmio_read32(pb, PORT_IS);
                let tfd = mmio_read32(pb, PORT_TFD);
                error!("AHCI: command timeout on port {} is={:#x} tfd={:#x}", self.port, is, tfd);
                return Err(BlockError::NotReady);
            }
        }
        trace!("AHCI: command completed for LBA {}", lba);

        let src = unsafe {
            core::slice::from_raw_parts((self.dma_virt as usize + OFFSET_DATA) as *const u8, 2048)
        };
        buf[0..2048].copy_from_slice(src);
        Ok(())
    }
}

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
                let offset = u64::from_le_bytes(req.payload[8..16].try_into().unwrap());
                let len = u32::from_le_bytes(req.payload[16..20].try_into().unwrap()) as usize;
                info!("AHCI: Read RPC offset={} len={}", offset, len);
                let sector_size = self.device.sector_size();
                let start_lba = offset / sector_size;
                let end_lba = if len > 0 {
                    (offset + len as u64 - 1) / sector_size
                } else {
                    start_lba.saturating_sub(1)
                };
                let count = if len > 0 { end_lba - start_lba + 1 } else { 0 };
                let mut bounce = Vec::with_capacity((count * sector_size) as usize);
                bounce.resize((count * sector_size) as usize, 0);
                match self.device.read_sectors(start_lba, count, &mut bounce) {
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

fn resolve_device_path(boot_fd: usize) -> String {
    if boot_fd != 0 {
        let req = VmMapReq {
            addr_hint: 0,
            len: 4096,
            prot: VmProt::READ | VmProt::USER,
            flags: VmMapFlags::empty(),
            backing: VmBacking::File { thing: boot_fd as u32, offset: 0 },
        };
        if let Ok(resp) = stem::syscall::vm_map(&req) {
            let path = {
                let ctx = unsafe { &*(resp.addr as *const DriverEntryCtx) };
                if ctx.version == 1 {
                    let s = ctx.device_path_str();
                    if !s.is_empty() { Some(s.to_string()) } else { None }
                } else {
                    None
                }
            };
            let _ = stem::syscall::vm_unmap(resp.addr, 4096);
            if let Some(p) = path {
                return p;
            }
        }
    }
    "pci-0000:00:1f.2".to_string()
}

#[stem::main]
fn main(boot_fd: usize) -> ! {
    info!("AHCI: Starting AHCI VFS driver");
    let device_path = resolve_device_path(boot_fd);
    let claim = match device_claim(&device_path) {
        Ok(c) => c,
        Err(_) => {
            warn!("AHCI: failed to claim device {}", device_path);
            stem::syscall::exit(Status::NoMatch as i32);
        }
    };

    info!("AHCI: Claimed device {}", device_path);
    let mmio = match device_map_mmio(claim, 5) {
        Ok(m) => m,
        Err(_) => {
            error!("AHCI: failed to map MMIO for {}", device_path);
            stem::syscall::exit(Status::BindFailed as i32);
        }
    };

    // Enable AHCI
    mmio_write32(mmio, HBA_GHC, mmio_read32(mmio, HBA_GHC) | (1 << 31));

    let pi = mmio_read32(mmio, HBA_PI);

    for port_num in 0..32 {
        if pi & (1 << port_num) == 0 {
            continue;
        }
        let pb = port_base(mmio, port_num);
        let ssts = mmio_read32(pb, PORT_SSTS);
        if (ssts & 0x0F) != 0x03 || (ssts & 0x0F00) != 0x0100 {
            continue;
        }

        let sig = mmio_read32(pb, PORT_SIG);
        let is_atapi = sig == SATA_SIG_ATAPI;
        let name =
            if is_atapi { format!("atapi{}", port_num) } else { format!("ahci{}", port_num) };

        // Allocate unique DMA buffer for this device
        let dma_virt = match device_alloc_dma(claim, 1) {
            Ok(v) => v,
            Err(e) => {
                error!("AHCI: failed to alloc DMA for port {}: {:?}", port_num, e);
                continue;
            }
        };
        let dma_phys = device_dma_phys(dma_virt).expect("AHCI: dma_phys failed");

        if !stop_port_engine(pb) {
            warn!("AHCI: failed to stop port {} command engine", port_num);
            continue;
        }

        unsafe {
            core::ptr::write_bytes(dma_virt as *mut u8, 0, 4096);
        }

        // Setup Port
        let clb = dma_phys + OFFSET_CMD_LIST as u64;
        let fb = dma_phys + OFFSET_FIS as u64;
        mmio_write32(pb, PORT_CLB, clb as u32);
        mmio_write32(pb, PORT_CLBU, (clb >> 32) as u32);
        mmio_write32(pb, PORT_FB, fb as u32);
        mmio_write32(pb, PORT_FBU, (fb >> 32) as u32);
        mmio_write32(pb, PORT_SACT, 0);
        mmio_write32(pb, PORT_CI, 0);
        mmio_write32(pb, PORT_SERR, 0xFFFFFFFF);
        mmio_write32(pb, PORT_IS, 0xFFFFFFFF);
        start_port_engine(pb);

        let device =
            Arc::new(AhciDevice { mmio_base: mmio, port: port_num, dma_virt, dma_phys, is_atapi });
        let provider = Arc::new(StorageProvider { device });
        let path = format!("/dev/storage/{}", name);
        let (v_w, v_r) = port_create(65536).unwrap();
        vfs_mount(v_w, &path).unwrap();
        info!("AHCI: Mounted {} at {}", name, path);

        info!("AHCI: Provider loop online at {}", path);
        let mut ploop = ProviderLoop::new(v_r);
        loop {
            let req = match ploop.next_request() {
                Ok(req) => req,
                Err(err) => {
                    warn!("AHCI: provider loop closed for {}: {:?}", path, err);
                    break;
                }
            };
            let resp = provider.handle_rpc(&req);
            let _ = ploop.send_response(&req, resp);
        }
    }

    loop {
        stem::syscall::sleep_ms(60000);
    }
}
