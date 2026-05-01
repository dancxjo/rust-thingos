//! xHCI USB host controller driver.
//!
//! This first userspace slice owns controller discovery, MMIO mapping, DMA
//! bootstrap rings, controller start, IRQ subscription, and root-port reset.
#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::mem::size_of;
use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{Ordering, fence};

use abi::driver_interface::{
    BusKind, DRIVER_DESCRIPTOR_ABI_VERSION, DeviceInfo, DriverClass, DriverDescriptor,
    DriverEntryCtx, ProbeResult, Status,
};
use abi::vm::{VmBacking, VmMapFlags, VmMapReq, VmProt};
use stem::abi::module_manifest::{MANIFEST_MAGIC, ManifestHeader, ModuleKind, device_kind_bytes};
use stem::syscall::{
    device_alloc_dma, device_claim, device_dma_phys, device_irq_subscribe, device_irq_wait,
    device_map_mmio,
};
use stem::{debug, error, info, trace, warn};

const THINGOS_DRIVER_NAME: &[u8] = b"xhci";

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
    driver_class: DriverClass::Other,
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
    let class = dev.class_code & 0x00ff_ffff;
    let is_match = dev.bus == BusKind::Pci as u32 && class == 0x0c0330;
    out.matched = if is_match { 1 } else { 0 };
    out.score = if is_match { 925 } else { 0 };
    out.claimed_class = DriverClass::Other;
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
    device_kind: device_kind_bytes(b"dev.usb.Xhci"),
    version: 1,
    _reserved: 0,
};

const PAGE_SIZE: usize = 4096;
const COMMAND_RING_TRBS: usize = 256;
const EVENT_RING_TRBS: usize = 256;

const CAP_CAPLENGTH: usize = 0x00;
const CAP_HCIVERSION: usize = 0x02;
const CAP_HCSPARAMS1: usize = 0x04;
const CAP_HCSPARAMS2: usize = 0x08;
const CAP_HCCPARAMS1: usize = 0x10;
const CAP_DBOFF: usize = 0x14;
const CAP_RTSOFF: usize = 0x18;

const OP_USBCMD: usize = 0x00;
const OP_USBSTS: usize = 0x04;
const OP_PAGESIZE: usize = 0x08;
const OP_CRCR: usize = 0x18;
const OP_DCBAAP: usize = 0x30;
const OP_CONFIG: usize = 0x38;
const OP_PORT_REGS: usize = 0x400;
const PORT_REG_STRIDE: usize = 0x10;
const PORTSC: usize = 0x00;

const RT_IR0: usize = 0x20;
const IR_STRIDE: usize = 0x20;
const IR_IMAN: usize = 0x00;
const IR_ERSTSZ: usize = 0x08;
const IR_ERSTBA: usize = 0x10;
const IR_ERDP: usize = 0x18;

const USBCMD_RS: u32 = 1 << 0;
const USBCMD_HCRST: u32 = 1 << 1;
const USBCMD_INTE: u32 = 1 << 2;
const USBSTS_HCH: u32 = 1 << 0;
const USBSTS_EINT: u32 = 1 << 3;
const USBSTS_PCD: u32 = 1 << 4;
const USBSTS_CNR: u32 = 1 << 11;

const IMAN_IP: u32 = 1 << 0;
const IMAN_IE: u32 = 1 << 1;

const PORTSC_CCS: u32 = 1 << 0;
const PORTSC_PED: u32 = 1 << 1;
const PORTSC_PR: u32 = 1 << 4;
const PORTSC_SPEED_SHIFT: u32 = 10;
const PORTSC_SPEED_MASK: u32 = 0x0f << PORTSC_SPEED_SHIFT;
const PORTSC_CSC: u32 = 1 << 17;
const PORTSC_PEC: u32 = 1 << 18;
const PORTSC_WRC: u32 = 1 << 19;
const PORTSC_OCC: u32 = 1 << 20;
const PORTSC_PRC: u32 = 1 << 21;
const PORTSC_PLC: u32 = 1 << 22;
const PORTSC_CEC: u32 = 1 << 23;
const PORTSC_CHANGE_BITS: u32 =
    PORTSC_CSC | PORTSC_PEC | PORTSC_WRC | PORTSC_OCC | PORTSC_PRC | PORTSC_PLC | PORTSC_CEC;

const TRB_TYPE_SHIFT: u32 = 10;
const TRB_TYPE_LINK: u32 = 6;

#[repr(C, align(16))]
#[derive(Clone, Copy, Default)]
struct Trb {
    parameter: u64,
    status: u32,
    control: u32,
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Default)]
struct EventRingSegmentTableEntry {
    ring_segment_base: u64,
    ring_segment_size: u16,
    reserved0: u16,
    reserved1: u32,
}

#[derive(Clone)]
struct DmaAlloc {
    virt: u64,
    phys: u64,
}

impl DmaAlloc {
    fn alloc(claim: usize, bytes: usize) -> Result<Self, abi::errors::Errno> {
        let page_count = pages_for(bytes);
        let virt = device_alloc_dma(claim, page_count)?;
        let phys = device_dma_phys(virt)?;
        unsafe {
            core::ptr::write_bytes(virt as *mut u8, 0, page_count * PAGE_SIZE);
        }
        Ok(Self { virt, phys })
    }

    fn as_mut_ptr<T>(&self) -> *mut T {
        self.virt as *mut T
    }
}

struct XhciRegs {
    cap_len: usize,
    op: u64,
    runtime: u64,
    doorbells: u64,
    version: u16,
    hcs1: u32,
    hcs2: u32,
    hcc1: u32,
}

impl XhciRegs {
    fn map(mmio: u64) -> Self {
        let cap_len = read8(mmio, CAP_CAPLENGTH) as usize;
        let runtime = mmio + (read32(mmio, CAP_RTSOFF) & !0x1f) as u64;
        let doorbells = mmio + (read32(mmio, CAP_DBOFF) & !0x3) as u64;
        Self {
            cap_len,
            op: mmio + cap_len as u64,
            runtime,
            doorbells,
            version: read16(mmio, CAP_HCIVERSION),
            hcs1: read32(mmio, CAP_HCSPARAMS1),
            hcs2: read32(mmio, CAP_HCSPARAMS2),
            hcc1: read32(mmio, CAP_HCCPARAMS1),
        }
    }

    fn max_slots(&self) -> u8 {
        (self.hcs1 & 0xff) as u8
    }

    fn max_ports(&self) -> u8 {
        ((self.hcs1 >> 24) & 0xff) as u8
    }

    fn scratchpad_count(&self) -> u16 {
        let lo = (self.hcs2 >> 21) & 0x1f;
        let hi = (self.hcs2 >> 27) & 0x1f;
        ((hi << 5) | lo) as u16
    }

    fn supports_64_bit_addresses(&self) -> bool {
        (self.hcc1 & 1) != 0
    }

    fn uses_64_byte_contexts(&self) -> bool {
        (self.hcc1 & (1 << 2)) != 0
    }

    fn portsc_offset(&self, port_index: usize) -> usize {
        OP_PORT_REGS + (port_index * PORT_REG_STRIDE) + PORTSC
    }

    fn ir_offset(&self, interrupter: usize, reg: usize) -> usize {
        RT_IR0 + interrupter * IR_STRIDE + reg
    }
}

struct XhciController {
    claim: usize,
    regs: XhciRegs,
    dcbaa: DmaAlloc,
    command_ring: DmaAlloc,
    event_ring: DmaAlloc,
    erst: DmaAlloc,
    scratchpad_array: Option<DmaAlloc>,
    scratchpads: Vec<DmaAlloc>,
    irq_enabled: bool,
    max_slots_enabled: u8,
}

impl XhciController {
    fn new(claim: usize, mmio: u64) -> Result<Self, &'static str> {
        let regs = XhciRegs::map(mmio);
        info!("xhci: hci version {}.{}", (regs.version >> 8) & 0xff, (regs.version >> 4) & 0x0f);
        info!("xhci: max slots {}", regs.max_slots());
        info!("xhci: max ports {}", regs.max_ports());
        info!("xhci: scratchpad buffers {}", regs.scratchpad_count());
        debug!(
            "xhci: cap_len={} op=0x{:x} runtime=0x{:x} doorbells=0x{:x} ac64={} csz={}",
            regs.cap_len,
            regs.op,
            regs.runtime,
            regs.doorbells,
            regs.supports_64_bit_addresses(),
            if regs.uses_64_byte_contexts() { 64 } else { 32 }
        );

        let max_slots = regs.max_slots().max(1);
        let dcbaa = DmaAlloc::alloc(claim, (max_slots as usize + 1) * size_of::<u64>())
            .map_err(|_| "DCBAA allocation failed")?;
        let command_ring = DmaAlloc::alloc(claim, COMMAND_RING_TRBS * size_of::<Trb>())
            .map_err(|_| "command ring allocation failed")?;
        let event_ring = DmaAlloc::alloc(claim, EVENT_RING_TRBS * size_of::<Trb>())
            .map_err(|_| "event ring allocation failed")?;
        let erst = DmaAlloc::alloc(claim, size_of::<EventRingSegmentTableEntry>())
            .map_err(|_| "ERST allocation failed")?;

        let mut controller = Self {
            claim,
            regs,
            dcbaa,
            command_ring,
            event_ring,
            erst,
            scratchpad_array: None,
            scratchpads: Vec::new(),
            irq_enabled: false,
            max_slots_enabled: max_slots,
        };

        controller.stop_and_reset()?;
        controller.allocate_scratchpads()?;
        controller.init_command_ring();
        controller.init_event_ring();
        controller.program_controller()?;
        controller.try_enable_irq();
        controller.run()?;
        Ok(controller)
    }

    fn stop_and_reset(&self) -> Result<(), &'static str> {
        let cmd = read32(self.regs.op, OP_USBCMD) & !USBCMD_RS;
        write32(self.regs.op, OP_USBCMD, cmd);
        self.wait_status_set(USBSTS_HCH, 250_000, "controller halt")?;

        write32(self.regs.op, OP_USBCMD, read32(self.regs.op, OP_USBCMD) | USBCMD_HCRST);
        self.wait_cmd_clear(USBCMD_HCRST, 250_000, "controller reset")?;
        self.wait_status_clear(USBSTS_CNR, 250_000, "controller not ready")?;
        Ok(())
    }

    fn allocate_scratchpads(&mut self) -> Result<(), &'static str> {
        let count = self.regs.scratchpad_count() as usize;
        if count == 0 {
            return Ok(());
        }

        let page_size = self.controller_page_size();
        let array = DmaAlloc::alloc(self.claim, count * size_of::<u64>())
            .map_err(|_| "scratchpad pointer array allocation failed")?;
        for i in 0..count {
            let buf = DmaAlloc::alloc(self.claim, page_size)
                .map_err(|_| "scratchpad buffer allocation failed")?;
            unsafe {
                let entries = array.as_mut_ptr::<u64>();
                write_volatile(entries.add(i), buf.phys);
            }
            self.scratchpads.push(buf);
        }

        unsafe {
            let dcbaa = self.dcbaa.as_mut_ptr::<u64>();
            write_volatile(dcbaa, array.phys);
        }
        self.scratchpad_array = Some(array);
        info!("xhci: scratchpad DMA ready count={} page_size={}", count, page_size);
        Ok(())
    }

    fn init_command_ring(&self) {
        unsafe {
            let trbs = self.command_ring.as_mut_ptr::<Trb>();
            let link = trbs.add(COMMAND_RING_TRBS - 1);
            write_volatile(
                link,
                Trb {
                    parameter: self.command_ring.phys,
                    status: 0,
                    control: (TRB_TYPE_LINK << TRB_TYPE_SHIFT) | (1 << 1) | 1,
                },
            );
        }
        fence(Ordering::SeqCst);
    }

    fn init_event_ring(&self) {
        unsafe {
            let entry = self.erst.as_mut_ptr::<EventRingSegmentTableEntry>();
            write_volatile(
                entry,
                EventRingSegmentTableEntry {
                    ring_segment_base: self.event_ring.phys,
                    ring_segment_size: EVENT_RING_TRBS as u16,
                    reserved0: 0,
                    reserved1: 0,
                },
            );
        }
        fence(Ordering::SeqCst);
    }

    fn program_controller(&self) -> Result<(), &'static str> {
        let max_slots = self.max_slots_enabled as u32;
        write64(self.regs.op, OP_DCBAAP, self.dcbaa.phys);
        write64(self.regs.op, OP_CRCR, self.command_ring.phys | 1);

        let ir0_erstsz = self.regs.ir_offset(0, IR_ERSTSZ);
        let ir0_erstba = self.regs.ir_offset(0, IR_ERSTBA);
        let ir0_erdp = self.regs.ir_offset(0, IR_ERDP);
        write32(self.regs.runtime, ir0_erstsz, 1);
        write64(self.regs.runtime, ir0_erstba, self.erst.phys);
        write64(self.regs.runtime, ir0_erdp, self.event_ring.phys);

        write32(self.regs.op, OP_CONFIG, max_slots);
        let observed = read32(self.regs.op, OP_CONFIG) & 0xff;
        if observed == 0 {
            return Err("CONFIG rejected max slot enable");
        }
        info!("xhci: configured {} device slots", observed);
        Ok(())
    }

    fn try_enable_irq(&mut self) {
        match device_irq_subscribe(self.claim, 0) {
            Ok(()) => {
                let off = self.regs.ir_offset(0, IR_IMAN);
                write32(self.regs.runtime, off, read32(self.regs.runtime, off) | IMAN_IE);
                write32(self.regs.op, OP_USBCMD, read32(self.regs.op, OP_USBCMD) | USBCMD_INTE);
                self.irq_enabled = true;
                info!("xhci: irq subscribed");
            }
            Err(e) => {
                self.irq_enabled = false;
                warn!("xhci: irq subscribe failed: {:?}; polling controller", e);
            }
        }
    }

    fn run(&self) -> Result<(), &'static str> {
        write32(self.regs.op, OP_USBSTS, USBSTS_EINT | USBSTS_PCD);
        write32(self.regs.op, OP_USBCMD, read32(self.regs.op, OP_USBCMD) | USBCMD_RS);
        self.wait_status_clear(USBSTS_HCH, 250_000, "controller run")?;
        info!("xhci: controller running");
        doorbell(self.regs.doorbells, 0, 0);
        Ok(())
    }

    fn scan_root_ports(&self) {
        for port in 0..self.regs.max_ports() as usize {
            let port_number = port + 1;
            let portsc = read32(self.regs.op, self.regs.portsc_offset(port));
            let connected = (portsc & PORTSC_CCS) != 0;
            let enabled = (portsc & PORTSC_PED) != 0;
            let speed = port_speed_name(((portsc & PORTSC_SPEED_MASK) >> PORTSC_SPEED_SHIFT) as u8);
            trace!(
                "xhci: port {} status=0x{:08x} connected={} enabled={} speed={}",
                port_number, portsc, connected, enabled, speed
            );
            if !connected {
                continue;
            }
            info!("xhci: port {} connected, speed {}", port_number, speed);
            match self.reset_port(port) {
                Ok(reset_speed) => {
                    info!("xhci: port {} reset complete, speed {}", port_number, reset_speed)
                }
                Err(e) => warn!("xhci: port {} reset failed: {}", port_number, e),
            }
        }
    }

    fn reset_port(&self, port: usize) -> Result<&'static str, &'static str> {
        let off = self.regs.portsc_offset(port);
        clear_port_changes(self.regs.op, off);
        let before = read32(self.regs.op, off);
        if (before & PORTSC_CCS) == 0 {
            return Err("not connected");
        }

        write32(self.regs.op, off, (before & !PORTSC_CHANGE_BITS) | PORTSC_PR);
        for tick in 0..250_000 {
            let val = read32(self.regs.op, off);
            if (val & PORTSC_PR) == 0 && (val & PORTSC_PRC) != 0 {
                let speed =
                    port_speed_name(((val & PORTSC_SPEED_MASK) >> PORTSC_SPEED_SHIFT) as u8);
                clear_port_changes(self.regs.op, off);
                return Ok(speed);
            }
            if tick % 2_000 == 0 {
                stem::yield_now();
            }
        }
        Err("timeout")
    }

    fn wait_for_events_or_poll_delay(&self) {
        if self.irq_enabled {
            let _ = device_irq_wait(self.claim, 0);
            let off = self.regs.ir_offset(0, IR_IMAN);
            let iman = read32(self.regs.runtime, off);
            if (iman & IMAN_IP) != 0 {
                write32(self.regs.runtime, off, iman);
            }
        } else {
            stem::time::sleep_ms(250);
        }
    }

    fn controller_page_size(&self) -> usize {
        let page_bits = read32(self.regs.op, OP_PAGESIZE);
        for bit in 0..16 {
            if (page_bits & (1 << bit)) != 0 {
                return 1usize << (12 + bit);
            }
        }
        PAGE_SIZE
    }

    fn wait_cmd_clear(
        &self,
        mask: u32,
        spins: usize,
        label: &'static str,
    ) -> Result<(), &'static str> {
        for tick in 0..spins {
            if (read32(self.regs.op, OP_USBCMD) & mask) == 0 {
                return Ok(());
            }
            if tick % 2_000 == 0 {
                stem::yield_now();
            }
        }
        warn!("xhci: timeout waiting for {}", label);
        Err(label)
    }

    fn wait_status_set(
        &self,
        mask: u32,
        spins: usize,
        label: &'static str,
    ) -> Result<(), &'static str> {
        for tick in 0..spins {
            if (read32(self.regs.op, OP_USBSTS) & mask) == mask {
                return Ok(());
            }
            if tick % 2_000 == 0 {
                stem::yield_now();
            }
        }
        warn!("xhci: timeout waiting for {}", label);
        Err(label)
    }

    fn wait_status_clear(
        &self,
        mask: u32,
        spins: usize,
        label: &'static str,
    ) -> Result<(), &'static str> {
        for tick in 0..spins {
            if (read32(self.regs.op, OP_USBSTS) & mask) == 0 {
                return Ok(());
            }
            if tick % 2_000 == 0 {
                stem::yield_now();
            }
        }
        warn!("xhci: timeout waiting for {}", label);
        Err(label)
    }
}

fn pages_for(bytes: usize) -> usize {
    bytes.saturating_add(PAGE_SIZE - 1) / PAGE_SIZE
}

fn read8(base: u64, offset: usize) -> u8 {
    unsafe { read_volatile((base as usize + offset) as *const u8) }
}

fn read16(base: u64, offset: usize) -> u16 {
    unsafe { read_volatile((base as usize + offset) as *const u16) }
}

fn read32(base: u64, offset: usize) -> u32 {
    unsafe { read_volatile((base as usize + offset) as *const u32) }
}

fn write32(base: u64, offset: usize, value: u32) {
    unsafe { write_volatile((base as usize + offset) as *mut u32, value) }
}

fn write64(base: u64, offset: usize, value: u64) {
    unsafe { write_volatile((base as usize + offset) as *mut u64, value) }
}

fn doorbell(doorbells: u64, index: usize, value: u32) {
    write32(doorbells, index * 4, value);
}

fn clear_port_changes(op: u64, portsc_offset: usize) {
    let val = read32(op, portsc_offset);
    write32(op, portsc_offset, (val & !PORTSC_CHANGE_BITS) | (val & PORTSC_CHANGE_BITS));
}

fn port_speed_name(speed: u8) -> &'static str {
    match speed {
        1 => "full",
        2 => "low",
        3 => "high",
        4 => "super",
        5 => "superplus",
        _ => "unknown",
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
    "/sys/devices/pci-0000:00:14.0".to_string()
}

fn publish_status(path: &str) {
    use abi::syscall::vfs_flags::{O_CREAT, O_RDWR};
    use stem::syscall::vfs::{vfs_close, vfs_mkdir, vfs_open, vfs_write};

    let _ = vfs_mkdir("/services");
    let _ = vfs_mkdir("/services/usb");
    let _ = vfs_mkdir("/services/usb/xhci0");
    if let Ok(fd) = vfs_open("/services/usb/xhci0/device", O_CREAT | O_RDWR) {
        let _ = vfs_write(fd, path.as_bytes());
        let _ = vfs_close(fd);
    }
    if let Ok(fd) = vfs_open("/services/usb/xhci0/state", O_CREAT | O_RDWR) {
        let _ = vfs_write(fd, b"running\n");
        let _ = vfs_close(fd);
    }
}

#[stem::main]
fn main(boot_fd: usize) -> ! {
    info!("xhci: starting userspace xHCI driver");
    let device_path = resolve_device_path(boot_fd);

    let claim = match device_claim(&device_path) {
        Ok(claim) => claim,
        Err(e) => {
            warn!("xhci: failed to claim device {}: {:?}", device_path, e);
            stem::syscall::exit(Status::NoMatch as i32);
        }
    };
    info!("xhci: claimed {}", device_path);

    let mmio = match device_map_mmio(claim, 0) {
        Ok(mmio) => mmio,
        Err(e) => {
            error!("xhci: failed to map BAR0 for {}: {:?}", device_path, e);
            stem::syscall::exit(Status::BindFailed as i32);
        }
    };
    info!("xhci: BAR0 mapped at 0x{:x}", mmio);

    let controller = match XhciController::new(claim, mmio) {
        Ok(controller) => controller,
        Err(e) => {
            error!("xhci: controller init failed: {}", e);
            stem::syscall::exit(Status::BindFailed as i32);
        }
    };

    publish_status(&device_path);
    controller.scan_root_ports();

    loop {
        controller.wait_for_events_or_poll_delay();
        let status = read32(controller.regs.op, OP_USBSTS);
        if (status & (USBSTS_EINT | USBSTS_PCD)) != 0 {
            write32(controller.regs.op, OP_USBSTS, status & (USBSTS_EINT | USBSTS_PCD));
            controller.scan_root_ports();
        }
    }
}
