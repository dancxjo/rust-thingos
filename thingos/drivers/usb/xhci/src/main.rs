//! xHCI USB host controller driver.
//!
//! First vertical slice:
//! - initialize xHCI from userspace through SYS_DEVICE_*;
//! - enumerate devices on root ports;
//! - read basic USB descriptors;
//! - configure USB Mass Storage Bulk-Only devices;
//! - expose LUN0 read-only as `/dev/block/usb0`.
#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::{format, vec};
use core::cmp::min;
use core::mem::size_of;
use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{Ordering, fence};

use abi::driver_interface::{
    BusKind, DRIVER_DESCRIPTOR_ABI_VERSION, DRIVER_FLAG_PCI, DRIVER_INTERFACE_ABI_VERSION,
    DeviceInfo, DriverClass, DriverDescriptor, DriverEntryCtx, DriverInterfaceV1, ProbeResult,
    Status,
};
use abi::errors::Errno;
use abi::vfs_rpc::VfsRpcOp;
use abi::vm::{VmBacking, VmMapFlags, VmMapReq, VmProt};
use ipc_helpers::provider::{ProviderLoop, ProviderRequest, ProviderResponse};
use stem::abi::module_manifest::{MANIFEST_MAGIC, ManifestHeader, ModuleKind, device_kind_bytes};
use stem::syscall::vfs::{vfs_mkdir, vfs_mount};
use stem::syscall::{
    device_alloc_dma, device_claim, device_dma_phys, device_irq_subscribe, device_irq_wait,
    device_map_mmio, port_create,
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
    flags: DRIVER_FLAG_PCI,
    vendor_id: 0,
    device_id: 0,
    class_code: 0x0c0330,
    class_mask: 0x00ff_ffff,
    entry_symbol: [0u8; 32],
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
    device_kind: device_kind_bytes(b"dev.usb.Xhci"),
    version: 1,
    _reserved: 0,
};

const PAGE_SIZE: usize = 4096;
const COMMAND_RING_TRBS: usize = 256;
const EVENT_RING_TRBS: usize = 256;
const TRANSFER_RING_TRBS: usize = 256;
const DMA_BUFFER_BYTES: usize = 64 * 1024;
const MAX_ENDPOINT_ID: usize = 31;

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
const ERDP_EHB: u64 = 1 << 3;

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

const TRB_CYCLE: u32 = 1 << 0;
const TRB_TC: u32 = 1 << 1;
const TRB_IOC: u32 = 1 << 5;
const TRB_IDT: u32 = 1 << 6;
const TRB_DIR: u32 = 1 << 16;
const TRB_TYPE_SHIFT: u32 = 10;
const TRB_SLOT_SHIFT: u32 = 24;
const TRB_EP_SHIFT: u32 = 16;

const TRB_TYPE_NORMAL: u32 = 1;
const TRB_TYPE_SETUP_STAGE: u32 = 2;
const TRB_TYPE_DATA_STAGE: u32 = 3;
const TRB_TYPE_STATUS_STAGE: u32 = 4;
const TRB_TYPE_LINK: u32 = 6;
const TRB_TYPE_NO_OP: u32 = 23;
const TRB_TYPE_ENABLE_SLOT: u32 = 9;
const TRB_TYPE_ADDRESS_DEVICE: u32 = 11;
const TRB_TYPE_CONFIGURE_ENDPOINT: u32 = 12;
const TRB_TYPE_TRANSFER_EVENT: u32 = 32;
const TRB_TYPE_COMMAND_COMPLETION_EVENT: u32 = 33;
const TRB_TYPE_PORT_STATUS_CHANGE_EVENT: u32 = 34;

const CC_SUCCESS: u32 = 1;
const CC_SHORT_PACKET: u32 = 13;

const EP_TYPE_BULK_OUT: u32 = 2;
const EP_TYPE_CONTROL: u32 = 4;
const EP_TYPE_BULK_IN: u32 = 6;

const USB_REQ_GET_DESCRIPTOR: u8 = 0x06;
const USB_REQ_SET_CONFIGURATION: u8 = 0x09;
const USB_DESC_DEVICE: u8 = 0x01;
const USB_DESC_CONFIGURATION: u8 = 0x02;
const USB_DESC_INTERFACE: u8 = 0x04;
const USB_DESC_ENDPOINT: u8 = 0x05;
const USB_DIR_IN: u8 = 0x80;

const S_IFREG: u32 = 0o100000;
const CBW_SIGNATURE: u32 = 0x4342_5355;
const CSW_SIGNATURE: u32 = 0x5342_5355;

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
    bytes: usize,
}

impl DmaAlloc {
    fn alloc(claim: usize, bytes: usize) -> Result<Self, abi::errors::Errno> {
        let page_count = pages_for(bytes);
        let virt = device_alloc_dma(claim, page_count)?;
        let phys = device_dma_phys(virt)?;
        let bytes = page_count * PAGE_SIZE;
        unsafe {
            core::ptr::write_bytes(virt as *mut u8, 0, bytes);
        }
        Ok(Self { virt, phys, bytes })
    }

    fn zero(&self) {
        unsafe {
            core::ptr::write_bytes(self.virt as *mut u8, 0, self.bytes);
        }
    }

    fn as_mut_ptr<T>(&self) -> *mut T {
        self.virt as *mut T
    }

    fn write_bytes(&self, data: &[u8]) {
        let len = min(data.len(), self.bytes);
        unsafe {
            core::ptr::copy_nonoverlapping(data.as_ptr(), self.virt as *mut u8, len);
        }
    }

    fn read_bytes(&self, len: usize, out: &mut [u8]) {
        let len = min(min(len, self.bytes), out.len());
        unsafe {
            core::ptr::copy_nonoverlapping(self.virt as *const u8, out.as_mut_ptr(), len);
        }
    }
}

struct Ring {
    dma: DmaAlloc,
    trbs: usize,
    enqueue: usize,
    cycle: bool,
}

impl Ring {
    fn new(claim: usize, trbs: usize, with_link: bool) -> Result<Self, abi::errors::Errno> {
        let dma = DmaAlloc::alloc(claim, trbs * size_of::<Trb>())?;
        let ring = Self { dma, trbs, enqueue: 0, cycle: true };
        if with_link {
            ring.write_link(true);
        }
        Ok(ring)
    }

    fn phys(&self) -> u64 {
        self.dma.phys
    }

    fn push(&mut self, mut trb: Trb) -> u64 {
        if self.enqueue >= self.trbs - 1 {
            self.write_link(self.cycle);
            self.enqueue = 0;
            self.cycle = !self.cycle;
        }
        if self.cycle {
            trb.control |= TRB_CYCLE;
        } else {
            trb.control &= !TRB_CYCLE;
        }
        let ptr = self.dma.phys + (self.enqueue * size_of::<Trb>()) as u64;
        unsafe {
            write_volatile(self.dma.as_mut_ptr::<Trb>().add(self.enqueue), trb);
        }
        fence(Ordering::SeqCst);
        self.enqueue += 1;
        if self.enqueue == self.trbs - 1 {
            self.write_link(self.cycle);
            self.enqueue = 0;
            self.cycle = !self.cycle;
        }
        ptr
    }

    fn write_link(&self, cycle: bool) {
        let mut control = trb_type(TRB_TYPE_LINK) | TRB_TC;
        if cycle {
            control |= TRB_CYCLE;
        }
        let link = Trb { parameter: self.dma.phys, status: 0, control };
        unsafe {
            write_volatile(self.dma.as_mut_ptr::<Trb>().add(self.trbs - 1), link);
        }
        fence(Ordering::SeqCst);
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

    fn context_size(&self) -> usize {
        if self.uses_64_byte_contexts() { 64 } else { 32 }
    }

    fn portsc_offset(&self, port_index: usize) -> usize {
        OP_PORT_REGS + (port_index * PORT_REG_STRIDE) + PORTSC
    }

    fn ir_offset(&self, interrupter: usize, reg: usize) -> usize {
        RT_IR0 + interrupter * IR_STRIDE + reg
    }
}

#[derive(Clone, Copy)]
struct EndpointDescriptor {
    address: u8,
    attributes: u8,
    max_packet_size: u16,
    interval: u8,
}

impl EndpointDescriptor {
    fn endpoint_id(&self) -> usize {
        let number = (self.address & 0x0f) as usize;
        let in_dir = (self.address & USB_DIR_IN) != 0;
        number * 2 + if in_dir { 1 } else { 0 }
    }

    fn is_bulk(&self) -> bool {
        (self.attributes & 0x03) == 0x02
    }
}

#[derive(Clone, Copy)]
struct InterfaceDescriptor {
    number: u8,
    class: u8,
    subclass: u8,
    protocol: u8,
}

struct UsbDevice {
    slot_id: u8,
    port: u8,
    speed: u8,
    config_value: u8,
    ep0_ring: Ring,
    device_context: DmaAlloc,
    input_context: DmaAlloc,
    setup_buffer: DmaAlloc,
}

struct UsbMassStorage {
    device: UsbDevice,
    bulk_in: EndpointDescriptor,
    bulk_out: EndpointDescriptor,
    bulk_in_ring: Ring,
    bulk_out_ring: Ring,
    cbw: DmaAlloc,
    data: DmaAlloc,
    csw: DmaAlloc,
    tag: u32,
    sector_count: u64,
    sector_size: u32,
}

struct XhciController {
    claim: usize,
    regs: XhciRegs,
    dcbaa: DmaAlloc,
    command_ring: Ring,
    event_ring: DmaAlloc,
    erst: DmaAlloc,
    scratchpad_array: Option<DmaAlloc>,
    scratchpads: Vec<DmaAlloc>,
    event_dequeue: usize,
    event_cycle: bool,
    irq_enabled: bool,
    max_slots_enabled: u8,
    enumerated_ports: Vec<bool>,
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
            regs.context_size()
        );

        let max_slots = regs.max_slots().max(1);
        let dcbaa = DmaAlloc::alloc(claim, (max_slots as usize + 1) * size_of::<u64>())
            .map_err(|_| "DCBAA allocation failed")?;
        let command_ring = Ring::new(claim, COMMAND_RING_TRBS, true)
            .map_err(|_| "command ring allocation failed")?;
        let event_ring = DmaAlloc::alloc(claim, EVENT_RING_TRBS * size_of::<Trb>())
            .map_err(|_| "event ring allocation failed")?;
        let erst = DmaAlloc::alloc(claim, size_of::<EventRingSegmentTableEntry>())
            .map_err(|_| "ERST allocation failed")?;

        let ports = regs.max_ports() as usize;
        let mut controller = Self {
            claim,
            regs,
            dcbaa,
            command_ring,
            event_ring,
            erst,
            scratchpad_array: None,
            scratchpads: Vec::new(),
            event_dequeue: 0,
            event_cycle: true,
            irq_enabled: false,
            max_slots_enabled: max_slots,
            enumerated_ports: vec![false; ports],
        };

        controller.stop_and_reset()?;
        controller.allocate_scratchpads()?;
        controller.init_event_ring();
        controller.program_controller()?;
        controller.try_enable_irq();
        controller.run()?;
        controller.send_no_op()?;
        Ok(controller)
    }

    fn stop_and_reset(&self) -> Result<(), &'static str> {
        write32(self.regs.op, OP_USBCMD, read32(self.regs.op, OP_USBCMD) & !USBCMD_RS);
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
                write_volatile(array.as_mut_ptr::<u64>().add(i), buf.phys);
            }
            self.scratchpads.push(buf);
        }
        unsafe {
            write_volatile(self.dcbaa.as_mut_ptr::<u64>(), array.phys);
        }
        self.scratchpad_array = Some(array);
        info!("xhci: scratchpad DMA ready count={} page_size={}", count, page_size);
        Ok(())
    }

    fn init_event_ring(&self) {
        unsafe {
            write_volatile(
                self.erst.as_mut_ptr::<EventRingSegmentTableEntry>(),
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
        write64(self.regs.op, OP_DCBAAP, self.dcbaa.phys);
        write64(self.regs.op, OP_CRCR, self.command_ring.phys() | 1);

        write32(self.regs.runtime, self.regs.ir_offset(0, IR_ERSTSZ), 1);
        write64(self.regs.runtime, self.regs.ir_offset(0, IR_ERSTBA), self.erst.phys);
        write64(
            self.regs.runtime,
            self.regs.ir_offset(0, IR_ERDP),
            self.event_ring.phys | ERDP_EHB,
        );

        write32(self.regs.op, OP_CONFIG, self.max_slots_enabled as u32);
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
        Ok(())
    }

    fn scan_and_enumerate(&mut self) -> Option<UsbMassStorage> {
        for port in 0..self.regs.max_ports() as usize {
            if self.enumerated_ports.get(port).copied().unwrap_or(false) {
                continue;
            }

            let port_number = port + 1;
            let portsc = read32(self.regs.op, self.regs.portsc_offset(port));
            let connected = (portsc & PORTSC_CCS) != 0;
            let enabled = (portsc & PORTSC_PED) != 0;
            let speed = ((portsc & PORTSC_SPEED_MASK) >> PORTSC_SPEED_SHIFT) as u8;
            trace!(
                "xhci: port {} status=0x{:08x} connected={} enabled={} speed={}",
                port_number,
                portsc,
                connected,
                enabled,
                port_speed_name(speed)
            );
            if !connected {
                continue;
            }

            info!("xhci: port {} connected, speed {}", port_number, port_speed_name(speed));
            let reset_speed = match self.reset_port(port) {
                Ok(s) => s,
                Err(e) => {
                    warn!("xhci: port {} reset failed: {}", port_number, e);
                    continue;
                }
            };
            info!(
                "xhci: port {} reset complete, speed {}",
                port_number,
                port_speed_name(reset_speed)
            );

            match self.enumerate_device(port_number as u8, reset_speed) {
                Ok(Some(storage)) => {
                    if let Some(p) = self.enumerated_ports.get_mut(port) {
                        *p = true;
                    }
                    return Some(storage);
                }
                Ok(None) => {
                    if let Some(p) = self.enumerated_ports.get_mut(port) {
                        *p = true;
                    }
                }
                Err(e) => warn!("xhci: port {} enumeration failed: {}", port_number, e),
            }
        }
        None
    }

    fn reset_port(&self, port: usize) -> Result<u8, &'static str> {
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
                let speed = ((val & PORTSC_SPEED_MASK) >> PORTSC_SPEED_SHIFT) as u8;
                clear_port_changes(self.regs.op, off);
                return Ok(speed);
            }
            if tick % 2_000 == 0 {
                stem::yield_now();
            }
        }
        Err("timeout")
    }

    fn enumerate_device(
        &mut self,
        port: u8,
        speed: u8,
    ) -> Result<Option<UsbMassStorage>, &'static str> {
        let slot_id = self.enable_slot()?;
        info!("usb: device attached on xhci0 port {}", port);

        let csz = self.regs.context_size();
        let input_context = DmaAlloc::alloc(self.claim, (MAX_ENDPOINT_ID + 2) * csz)
            .map_err(|_| "input context allocation failed")?;
        let device_context = DmaAlloc::alloc(self.claim, (MAX_ENDPOINT_ID + 1) * csz)
            .map_err(|_| "device context allocation failed")?;
        let ep0_ring = Ring::new(self.claim, TRANSFER_RING_TRBS, true)
            .map_err(|_| "ep0 ring allocation failed")?;
        let setup_buffer = DmaAlloc::alloc(self.claim, DMA_BUFFER_BYTES)
            .map_err(|_| "control buffer allocation failed")?;

        unsafe {
            write_volatile(
                self.dcbaa.as_mut_ptr::<u64>().add(slot_id as usize),
                device_context.phys,
            );
        }

        fill_address_input_context(
            &input_context,
            csz,
            port,
            speed,
            ep0_max_packet(speed),
            ep0_ring.phys(),
        );
        self.address_device(slot_id, input_context.phys)?;

        let mut dev = UsbDevice {
            slot_id,
            port,
            speed,
            config_value: 0,
            ep0_ring,
            device_context,
            input_context,
            setup_buffer,
        };
        debug!(
            "xhci: slot {} device_context=0x{:x} input_context=0x{:x}",
            dev.slot_id, dev.device_context.phys, dev.input_context.phys
        );

        let first = self.get_descriptor(&mut dev, USB_DESC_DEVICE, 0, 8)?;
        if first.len() < 8 {
            return Err("short device descriptor header");
        }
        let full_len = first[0].max(18) as usize;
        let device_desc = self.get_descriptor(&mut dev, USB_DESC_DEVICE, 0, full_len)?;
        if device_desc.len() < 18 {
            return Err("short device descriptor");
        }
        let usb_bcd = le16(&device_desc[2..4]);
        let class = device_desc[4];
        let subclass = device_desc[5];
        let protocol = device_desc[6];
        let vid = le16(&device_desc[8..10]);
        let pid = le16(&device_desc[10..12]);
        let config_count = device_desc[17];
        info!("usb: vid={:04x} pid={:04x}", vid, pid);
        info!(
            "usb: usb={}.{:02x}",
            (usb_bcd >> 8) & 0xff,
            usb_bcd & 0xff,
        );
        info!("usb: class={:02x} subclass={:02x} protocol={:02x}", class, subclass, protocol);
        info!("usb: configurations={}", config_count);

        let config_head = self.get_descriptor(&mut dev, USB_DESC_CONFIGURATION, 0, 9)?;
        if config_head.len() < 9 {
            return Err("short configuration descriptor header");
        }
        let total_len = le16(&config_head[2..4]) as usize;
        let config_value = config_head[5];
        let config = self.get_descriptor(&mut dev, USB_DESC_CONFIGURATION, 0, total_len)?;
        dev.config_value = config_value;
        let parsed = parse_configuration(&config);
        for iface in &parsed.interfaces {
            info!(
                "usb: interface {} class={:02x} subclass={:02x} protocol={:02x}",
                iface.number, iface.class, iface.subclass, iface.protocol
            );
        }
        for ep in &parsed.endpoints {
            info!(
                "usb: endpoint addr=0x{:02x} attrs=0x{:02x} max_packet={} interval={}",
                ep.address, ep.attributes, ep.max_packet_size, ep.interval
            );
        }

        let Some(ms_iface) = parsed
            .interfaces
            .iter()
            .copied()
            .find(|i| i.class == 0x08 && i.subclass == 0x06 && i.protocol == 0x50)
        else {
            self.set_configuration(&mut dev, config_value)?;
            info!("usb: no mass-storage BOT interface on port {}", port);
            return Ok(None);
        };

        let Some(bulk_in) =
            parsed.endpoints.iter().copied().find(|e| e.is_bulk() && (e.address & USB_DIR_IN) != 0)
        else {
            return Err("mass-storage bulk IN endpoint missing");
        };
        let Some(bulk_out) =
            parsed.endpoints.iter().copied().find(|e| e.is_bulk() && (e.address & USB_DIR_IN) == 0)
        else {
            return Err("mass-storage bulk OUT endpoint missing");
        };

        info!(
            "usb: mass-storage interface {} class={:02x} subclass={:02x} protocol={:02x}",
            ms_iface.number, ms_iface.class, ms_iface.subclass, ms_iface.protocol
        );

        let bulk_in_ring = Ring::new(self.claim, TRANSFER_RING_TRBS, true)
            .map_err(|_| "bulk IN ring allocation failed")?;
        let bulk_out_ring = Ring::new(self.claim, TRANSFER_RING_TRBS, true)
            .map_err(|_| "bulk OUT ring allocation failed")?;
        self.configure_bulk_endpoints(
            &dev,
            bulk_in,
            bulk_in_ring.phys(),
            bulk_out,
            bulk_out_ring.phys(),
        )?;
        self.set_configuration(&mut dev, config_value)?;

        let cbw = DmaAlloc::alloc(self.claim, 64).map_err(|_| "CBW allocation failed")?;
        let data = DmaAlloc::alloc(self.claim, DMA_BUFFER_BYTES)
            .map_err(|_| "BOT data allocation failed")?;
        let csw = DmaAlloc::alloc(self.claim, 64).map_err(|_| "CSW allocation failed")?;
        let mut storage = UsbMassStorage {
            device: dev,
            bulk_in,
            bulk_out,
            bulk_in_ring,
            bulk_out_ring,
            cbw,
            data,
            csw,
            tag: 1,
            sector_count: 0,
            sector_size: 512,
        };

        storage.init(self)?;
        Ok(Some(storage))
    }

    fn send_no_op(&mut self) -> Result<(), &'static str> {
        let ptr = self.command_ring.push(Trb {
            parameter: 0,
            status: 0,
            control: trb_type(TRB_TYPE_NO_OP),
        });
        doorbell(self.regs.doorbells, 0, 0);
        self.wait_command_completion(ptr)?;
        info!("xhci: command completion type=NO_OP success");
        Ok(())
    }

    fn enable_slot(&mut self) -> Result<u8, &'static str> {
        let ptr = self.command_ring.push(Trb {
            parameter: 0,
            status: 0,
            control: trb_type(TRB_TYPE_ENABLE_SLOT),
        });
        doorbell(self.regs.doorbells, 0, 0);
        let ev = self.wait_command_completion(ptr)?;
        let slot_id = ((ev.control >> TRB_SLOT_SHIFT) & 0xff) as u8;
        info!("xhci: enable slot -> slot_id={}", slot_id);
        Ok(slot_id)
    }

    fn address_device(&mut self, slot_id: u8, input_phys: u64) -> Result<(), &'static str> {
        let ptr = self.command_ring.push(Trb {
            parameter: input_phys,
            status: 0,
            control: trb_type(TRB_TYPE_ADDRESS_DEVICE) | ((slot_id as u32) << TRB_SLOT_SHIFT),
        });
        doorbell(self.regs.doorbells, 0, 0);
        let _ = self.wait_command_completion(ptr)?;
        info!("xhci: address device slot={} ok", slot_id);
        Ok(())
    }

    fn configure_bulk_endpoints(
        &mut self,
        dev: &UsbDevice,
        bulk_in: EndpointDescriptor,
        bulk_in_ring_phys: u64,
        bulk_out: EndpointDescriptor,
        bulk_out_ring_phys: u64,
    ) -> Result<(), &'static str> {
        dev.input_context.zero();
        let csz = self.regs.context_size();
        let in_id = bulk_in.endpoint_id();
        let out_id = bulk_out.endpoint_id();
        let max_id = in_id.max(out_id);
        set_ctx_dword(&dev.input_context, csz, 0, 1, (1 << 0) | (1 << in_id) | (1 << out_id));
        set_slot_context(&dev.input_context, csz, dev.port, dev.speed, max_id as u8);
        set_endpoint_context(
            &dev.input_context,
            csz,
            in_id,
            EP_TYPE_BULK_IN,
            bulk_in.max_packet_size,
            bulk_in_ring_phys,
            0x4000,
        );
        set_endpoint_context(
            &dev.input_context,
            csz,
            out_id,
            EP_TYPE_BULK_OUT,
            bulk_out.max_packet_size,
            bulk_out_ring_phys,
            0x4000,
        );
        let ptr = self.command_ring.push(Trb {
            parameter: dev.input_context.phys,
            status: 0,
            control: trb_type(TRB_TYPE_CONFIGURE_ENDPOINT)
                | ((dev.slot_id as u32) << TRB_SLOT_SHIFT),
        });
        doorbell(self.regs.doorbells, 0, 0);
        let _ = self.wait_command_completion(ptr)?;
        info!("xhci: configured bulk endpoints in={} out={}", in_id, out_id);
        Ok(())
    }

    fn get_descriptor(
        &mut self,
        dev: &mut UsbDevice,
        desc_type: u8,
        index: u8,
        len: usize,
    ) -> Result<Vec<u8>, &'static str> {
        let value = ((desc_type as u16) << 8) | index as u16;
        let setup = [
            0x80,
            USB_REQ_GET_DESCRIPTOR,
            value as u8,
            (value >> 8) as u8,
            0,
            0,
            len as u8,
            (len >> 8) as u8,
        ];
        let actual = self.control_transfer(dev, &setup, len, true)?;
        let mut out = vec![0u8; actual];
        dev.setup_buffer.read_bytes(actual, &mut out);
        Ok(out)
    }

    fn set_configuration(&mut self, dev: &mut UsbDevice, value: u8) -> Result<(), &'static str> {
        let setup = [0x00, USB_REQ_SET_CONFIGURATION, value, 0, 0, 0, 0, 0];
        let _ = self.control_transfer(dev, &setup, 0, false)?;
        info!("usb: set configuration {}", value);
        Ok(())
    }

    fn control_transfer(
        &mut self,
        dev: &mut UsbDevice,
        setup: &[u8; 8],
        data_len: usize,
        data_in: bool,
    ) -> Result<usize, &'static str> {
        dev.setup_buffer.zero();
        let mut setup_param = 0u64;
        for (i, b) in setup.iter().copied().enumerate() {
            setup_param |= (b as u64) << (i * 8);
        }
        let trt = if data_len == 0 {
            0
        } else if data_in {
            3
        } else {
            2
        };
        dev.ep0_ring.push(Trb {
            parameter: setup_param,
            status: 8,
            control: trb_type(TRB_TYPE_SETUP_STAGE) | TRB_IDT | ((trt as u32) << 16),
        });
        if data_len != 0 {
            dev.ep0_ring.push(Trb {
                parameter: dev.setup_buffer.phys,
                status: data_len as u32,
                control: trb_type(TRB_TYPE_DATA_STAGE) | if data_in { TRB_DIR } else { 0 },
            });
        }
        let status_dir = if data_len == 0 || !data_in { TRB_DIR } else { 0 };
        dev.ep0_ring.push(Trb {
            parameter: 0,
            status: 0,
            control: trb_type(TRB_TYPE_STATUS_STAGE) | status_dir | TRB_IOC,
        });
        doorbell(self.regs.doorbells, dev.slot_id as usize, 1);
        let ev = self.wait_transfer_event(dev.slot_id, 1)?;
        let residual = ev.status & 0x00ff_ffff;
        Ok(data_len.saturating_sub(residual as usize))
    }

    fn normal_transfer(
        &mut self,
        slot_id: u8,
        endpoint_id: usize,
        ring: &mut Ring,
        dma: &DmaAlloc,
        len: usize,
    ) -> Result<usize, &'static str> {
        ring.push(Trb {
            parameter: dma.phys,
            status: len as u32,
            control: trb_type(TRB_TYPE_NORMAL) | TRB_IOC,
        });
        doorbell(self.regs.doorbells, slot_id as usize, endpoint_id as u32);
        let ev = self.wait_transfer_event(slot_id, endpoint_id as u8)?;
        let residual = ev.status & 0x00ff_ffff;
        Ok(len.saturating_sub(residual as usize))
    }

    fn wait_command_completion(&mut self, command_ptr: u64) -> Result<Trb, &'static str> {
        for _ in 0..250_000 {
            while let Some(ev) = self.next_event() {
                match event_type(ev.control) {
                    TRB_TYPE_COMMAND_COMPLETION_EVENT if ev.parameter == command_ptr => {
                        let cc = completion_code(ev.status);
                        if cc == CC_SUCCESS {
                            return Ok(ev);
                        }
                        warn!("xhci: command failed cc={}", cc);
                        return Err("command completion error");
                    }
                    TRB_TYPE_PORT_STATUS_CHANGE_EVENT => self.log_port_event(ev),
                    _ => trace!("xhci: ignored event type={}", event_type(ev.control)),
                }
            }
            self.wait_for_event_delay();
        }
        Err("command timeout")
    }

    fn wait_transfer_event(&mut self, slot_id: u8, endpoint_id: u8) -> Result<Trb, &'static str> {
        for _ in 0..500_000 {
            while let Some(ev) = self.next_event() {
                match event_type(ev.control) {
                    TRB_TYPE_TRANSFER_EVENT => {
                        let ev_slot = ((ev.control >> TRB_SLOT_SHIFT) & 0xff) as u8;
                        let ev_ep = ((ev.control >> TRB_EP_SHIFT) & 0x1f) as u8;
                        if ev_slot == slot_id && ev_ep == endpoint_id {
                            let cc = completion_code(ev.status);
                            if cc == CC_SUCCESS || cc == CC_SHORT_PACKET {
                                return Ok(ev);
                            }
                            warn!("xhci: transfer failed slot={} ep={} cc={}", ev_slot, ev_ep, cc);
                            return Err("transfer completion error");
                        }
                    }
                    TRB_TYPE_PORT_STATUS_CHANGE_EVENT => self.log_port_event(ev),
                    _ => trace!("xhci: ignored event type={}", event_type(ev.control)),
                }
            }
            self.wait_for_event_delay();
        }
        Err("transfer timeout")
    }

    fn next_event(&mut self) -> Option<Trb> {
        let ptr = unsafe { self.event_ring.as_mut_ptr::<Trb>().add(self.event_dequeue) };
        let ev = unsafe { read_volatile(ptr) };
        let cycle = (ev.control & TRB_CYCLE) != 0;
        if cycle != self.event_cycle {
            return None;
        }
        self.event_dequeue += 1;
        if self.event_dequeue >= EVENT_RING_TRBS {
            self.event_dequeue = 0;
            self.event_cycle = !self.event_cycle;
        }
        let erdp = self.event_ring.phys + (self.event_dequeue * size_of::<Trb>()) as u64;
        write64(self.regs.runtime, self.regs.ir_offset(0, IR_ERDP), erdp | ERDP_EHB);
        Some(ev)
    }

    fn log_port_event(&self, ev: Trb) {
        let port = ((ev.parameter >> 24) & 0xff) as u8;
        trace!("xhci: port status change event port={}", port);
    }

    fn wait_for_event_delay(&self) {
        if self.irq_enabled {
            let _ = device_irq_wait(self.claim, 0);
            let off = self.regs.ir_offset(0, IR_IMAN);
            let iman = read32(self.regs.runtime, off);
            if (iman & IMAN_IP) != 0 {
                write32(self.regs.runtime, off, iman);
            }
        } else {
            stem::time::sleep_ms(1);
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

impl UsbMassStorage {
    fn init(&mut self, xhci: &mut XhciController) -> Result<(), &'static str> {
        info!("ums: device attached");
        let inquiry = self.scsi_inquiry(xhci)?;
        let vendor = ascii_field(&inquiry, 8, 8);
        let product = ascii_field(&inquiry, 16, 16);
        info!("ums: vendor=\"{}\" product=\"{}\"", vendor, product);
        if let Err(e) = self.scsi_test_unit_ready(xhci) {
            info!("ums: TEST UNIT READY failed ({}), issuing REQUEST SENSE", e);
            let _ = self.scsi_request_sense(xhci);
        }
        let (sectors, sector_size) = self.scsi_read_capacity_10(xhci)?;
        self.sector_count = sectors;
        self.sector_size = sector_size;
        info!("ums: capacity {} sectors, sector_size={}", sectors, sector_size);
        let mut first = vec![0u8; sector_size as usize];
        self.read_sectors(xhci, 0, 1, &mut first)?;
        info!("ums: READ(10) lba=0 count=1 ok");
        log_hex_prefix("ums: lba0", &first[..min(first.len(), 64)]);
        Ok(())
    }

    fn read_sectors(
        &mut self,
        xhci: &mut XhciController,
        lba: u64,
        count: u64,
        out: &mut [u8],
    ) -> Result<(), &'static str> {
        let bytes = (count as usize).saturating_mul(self.sector_size as usize);
        if bytes > out.len() || bytes > DMA_BUFFER_BYTES {
            return Err("read buffer too small");
        }
        let mut cdb = [0u8; 16];
        cdb[0] = 0x28;
        cdb[2] = (lba >> 24) as u8;
        cdb[3] = (lba >> 16) as u8;
        cdb[4] = (lba >> 8) as u8;
        cdb[5] = lba as u8;
        cdb[7] = (count >> 8) as u8;
        cdb[8] = count as u8;
        let got = self.bot_command(xhci, &cdb, 10, bytes, true)?;
        if got < bytes {
            return Err("short READ(10)");
        }
        self.data.read_bytes(bytes, out);
        Ok(())
    }

    fn scsi_inquiry(&mut self, xhci: &mut XhciController) -> Result<Vec<u8>, &'static str> {
        let mut cdb = [0u8; 16];
        cdb[0] = 0x12;
        cdb[4] = 36;
        let got = self.bot_command(xhci, &cdb, 6, 36, true)?;
        let mut out = vec![0u8; got];
        self.data.read_bytes(got, &mut out);
        Ok(out)
    }

    fn scsi_test_unit_ready(&mut self, xhci: &mut XhciController) -> Result<(), &'static str> {
        let mut cdb = [0u8; 16];
        cdb[0] = 0x00;
        self.bot_command(xhci, &cdb, 6, 0, false).map(|_| ())
    }

    /// SCSI REQUEST SENSE (opcode 0x03).
    ///
    /// Retrieves up to 18 bytes of sense data from the device and logs the
    /// sense key, ASC, and ASCQ fields.  Returns `Ok(())` on success; the
    /// caller does not need to inspect the sense data to continue.
    fn scsi_request_sense(&mut self, xhci: &mut XhciController) -> Result<(), &'static str> {
        const SENSE_LEN: usize = 18;
        // Fixed-format sense data field offsets (SPC-4 Table 28).
        const SENSE_KEY_OFFSET: usize = 2;
        const SENSE_ASC_OFFSET: usize = 12;
        const SENSE_ASCQ_OFFSET: usize = 13;
        // ASCQ is at offset 13, so we need at least 14 bytes to read it.
        const MIN_SENSE_DATA_LEN: usize = SENSE_ASCQ_OFFSET + 1;
        const SCSI_CDB_6_LEN: u8 = 6;

        let mut cdb = [0u8; 16];
        cdb[0] = 0x03; // REQUEST SENSE
        cdb[4] = SENSE_LEN as u8;
        // Use bot_transfer_raw to avoid recursive autosense on sense failure.
        let (got, _) = self.bot_transfer_raw(xhci, &cdb, SCSI_CDB_6_LEN, SENSE_LEN, true)?;
        let valid = got.min(SENSE_LEN);
        if valid < MIN_SENSE_DATA_LEN {
            warn!("ums: REQUEST SENSE returned only {} bytes", valid);
            return Ok(());
        }
        let mut sense = [0u8; SENSE_LEN];
        self.data.read_bytes(valid, &mut sense[..valid]);
        let sense_key = sense[SENSE_KEY_OFFSET] & 0x0f;
        let asc = sense[SENSE_ASC_OFFSET];
        let ascq = sense[SENSE_ASCQ_OFFSET];
        info!(
            "ums: REQUEST SENSE key=0x{:02x} asc=0x{:02x} ascq=0x{:02x}",
            sense_key, asc, ascq
        );
        Ok(())
    }

    fn scsi_read_capacity_10(
        &mut self,
        xhci: &mut XhciController,
    ) -> Result<(u64, u32), &'static str> {
        let mut cdb = [0u8; 16];
        cdb[0] = 0x25;
        let got = self.bot_command(xhci, &cdb, 10, 8, true)?;
        if got < 8 {
            return Err("short READ CAPACITY(10)");
        }
        let mut buf = [0u8; 8];
        self.data.read_bytes(8, &mut buf);
        let last_lba = be32(&buf[0..4]) as u64;
        let sector_size = be32(&buf[4..8]);
        Ok((last_lba + 1, sector_size))
    }

    fn bot_command(
        &mut self,
        xhci: &mut XhciController,
        cdb: &[u8; 16],
        cdb_len: u8,
        data_len: usize,
        data_in: bool,
    ) -> Result<usize, &'static str> {
        let (transferred, csw_status) =
            self.bot_transfer_raw(xhci, cdb, cdb_len, data_len, data_in)?;
        if csw_status == 0x01 {
            // CHECK CONDITION – retrieve sense data before returning the error.
            let _ = self.scsi_request_sense(xhci);
            return Err("CSW command failed");
        }
        if csw_status != 0x00 {
            return Err("CSW command failed");
        }
        Ok(transferred)
    }

    /// Raw BOT transfer: CBW → (optional data phase) → CSW.
    ///
    /// Returns `(bytes_transferred, csw_status)` where `csw_status` is the
    /// bCSWStatus field (0=Good, 1=Check Condition, 2=Phase Error).
    fn bot_transfer_raw(
        &mut self,
        xhci: &mut XhciController,
        cdb: &[u8; 16],
        cdb_len: u8,
        data_len: usize,
        data_in: bool,
    ) -> Result<(usize, u8), &'static str> {
        let tag = self.next_tag();
        let mut cbw = [0u8; 31];
        cbw[0..4].copy_from_slice(&CBW_SIGNATURE.to_le_bytes());
        cbw[4..8].copy_from_slice(&tag.to_le_bytes());
        cbw[8..12].copy_from_slice(&(data_len as u32).to_le_bytes());
        cbw[12] = if data_in { 0x80 } else { 0x00 };
        cbw[13] = 0;
        cbw[14] = cdb_len;
        cbw[15..31].copy_from_slice(cdb);
        self.cbw.write_bytes(&cbw);
        let slot = self.device.slot_id;
        let out_id = self.bulk_out.endpoint_id();
        let in_id = self.bulk_in.endpoint_id();
        let sent = xhci.normal_transfer(slot, out_id, &mut self.bulk_out_ring, &self.cbw, 31)?;
        if sent != 31 {
            return Err("short CBW");
        }

        let mut transferred = 0;
        if data_len != 0 {
            transferred = if data_in {
                xhci.normal_transfer(slot, in_id, &mut self.bulk_in_ring, &self.data, data_len)?
            } else {
                xhci.normal_transfer(slot, out_id, &mut self.bulk_out_ring, &self.data, data_len)?
            };
        }

        self.csw.zero();
        let got_csw = xhci.normal_transfer(slot, in_id, &mut self.bulk_in_ring, &self.csw, 13)?;
        if got_csw < 13 {
            return Err("short CSW");
        }
        let mut csw = [0u8; 13];
        self.csw.read_bytes(13, &mut csw);
        if le32(&csw[0..4]) != CSW_SIGNATURE {
            return Err("bad CSW signature");
        }
        if le32(&csw[4..8]) != tag {
            return Err("CSW tag mismatch");
        }
        Ok((transferred, csw[12]))
    }

    fn next_tag(&mut self) -> u32 {
        let tag = self.tag;
        self.tag = self.tag.wrapping_add(1).max(1);
        tag
    }
}

struct ParsedConfiguration {
    interfaces: Vec<InterfaceDescriptor>,
    endpoints: Vec<EndpointDescriptor>,
}

fn parse_configuration(buf: &[u8]) -> ParsedConfiguration {
    let mut interfaces = Vec::new();
    let mut endpoints = Vec::new();
    let mut offset = 0;
    while offset + 2 <= buf.len() {
        let len = buf[offset] as usize;
        let desc_type = buf[offset + 1];
        if len < 2 || offset + len > buf.len() {
            break;
        }
        let d = &buf[offset..offset + len];
        match desc_type {
            USB_DESC_INTERFACE if len >= 9 => interfaces.push(InterfaceDescriptor {
                number: d[2],
                class: d[5],
                subclass: d[6],
                protocol: d[7],
            }),
            USB_DESC_ENDPOINT if len >= 7 => endpoints.push(EndpointDescriptor {
                address: d[2],
                attributes: d[3],
                max_packet_size: le16(&d[4..6]),
                interval: d[6],
            }),
            _ => {}
        }
        offset += len;
    }
    ParsedConfiguration { interfaces, endpoints }
}

struct UsbBlockProvider {
    controller: XhciController,
    storage: UsbMassStorage,
}

impl UsbBlockProvider {
    fn handle_rpc(&mut self, req: &ProviderRequest) -> ProviderResponse {
        match req.op {
            VfsRpcOp::Lookup => ProviderResponse::ok_u64(1),
            VfsRpcOp::Stat => {
                let size = self.storage.sector_count * self.storage.sector_size as u64;
                ProviderResponse::ok_stat(S_IFREG | 0o444, size, 1)
            }
            VfsRpcOp::Read => {
                if req.payload.len() < 12 {
                    return ProviderResponse::err(Errno::EINVAL);
                }
                let offset = u64::from_le_bytes(req.payload[0..8].try_into().unwrap());
                let len = u32::from_le_bytes(req.payload[8..12].try_into().unwrap()) as usize;
                if len == 0 {
                    return ProviderResponse::ok_bytes(&[]);
                }
                let sector_size = self.storage.sector_size as u64;
                let start_lba = offset / sector_size;
                let end_lba = (offset + len as u64 - 1) / sector_size;
                let count = end_lba - start_lba + 1;
                let mut bounce = vec![0u8; (count * sector_size) as usize];
                match self.storage.read_sectors(&mut self.controller, start_lba, count, &mut bounce)
                {
                    Ok(()) => {
                        let inner = (offset % sector_size) as usize;
                        ProviderResponse::ok_bytes(&bounce[inner..inner + len])
                    }
                    Err(e) => {
                        warn!("ums: read failed: {}", e);
                        ProviderResponse::err(Errno::EIO)
                    }
                }
            }
            _ => ProviderResponse::err(Errno::ENOSYS),
        }
    }
}

fn fill_address_input_context(
    ctx: &DmaAlloc,
    csz: usize,
    port: u8,
    speed: u8,
    ep0_mps: u16,
    ep0_ring_phys: u64,
) {
    ctx.zero();
    set_ctx_dword(ctx, csz, 0, 1, (1 << 0) | (1 << 1));
    set_slot_context(ctx, csz, port, speed, 1);
    set_endpoint_context(ctx, csz, 1, EP_TYPE_CONTROL, ep0_mps, ep0_ring_phys, 8);
}

fn set_slot_context(ctx: &DmaAlloc, csz: usize, port: u8, speed: u8, context_entries: u8) {
    let slot = 1;
    set_ctx_dword(ctx, csz, slot, 0, ((speed as u32) << 20) | ((context_entries as u32) << 27));
    set_ctx_dword(ctx, csz, slot, 1, (port as u32) << 16);
}

fn set_endpoint_context(
    ctx: &DmaAlloc,
    csz: usize,
    endpoint_id: usize,
    ep_type: u32,
    max_packet_size: u16,
    ring_phys: u64,
    avg_trb_len: u32,
) {
    let index = 1 + endpoint_id;
    let ep_dword1 = (3 << 1) | (ep_type << 3) | ((max_packet_size as u32) << 16);
    set_ctx_dword(ctx, csz, index, 1, ep_dword1);
    let dequeue = ring_phys | 1;
    set_ctx_dword(ctx, csz, index, 2, dequeue as u32);
    set_ctx_dword(ctx, csz, index, 3, (dequeue >> 32) as u32);
    set_ctx_dword(ctx, csz, index, 4, avg_trb_len & 0xffff);
}

fn set_ctx_dword(ctx: &DmaAlloc, csz: usize, context_index: usize, dword: usize, value: u32) {
    unsafe {
        write_volatile((ctx.virt as usize + context_index * csz + dword * 4) as *mut u32, value);
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

fn trb_type(ty: u32) -> u32 {
    ty << TRB_TYPE_SHIFT
}

fn event_type(control: u32) -> u32 {
    (control >> TRB_TYPE_SHIFT) & 0x3f
}

fn completion_code(status: u32) -> u32 {
    (status >> 24) & 0xff
}

fn ep0_max_packet(speed: u8) -> u16 {
    match speed {
        1 => 64,
        2 => 8,
        3 => 64,
        4 | 5 => 512,
        _ => 64,
    }
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

fn le16(bytes: &[u8]) -> u16 {
    u16::from_le_bytes([bytes[0], bytes[1]])
}

fn le32(bytes: &[u8]) -> u32 {
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

fn be32(bytes: &[u8]) -> u32 {
    u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

fn ascii_field(bytes: &[u8], start: usize, len: usize) -> String {
    if start >= bytes.len() {
        return String::new();
    }
    let end = min(bytes.len(), start + len);
    let s = core::str::from_utf8(&bytes[start..end]).unwrap_or("");
    s.trim().to_string()
}

fn log_hex_prefix(prefix: &str, bytes: &[u8]) {
    let mut line = String::new();
    for b in bytes {
        line.push_str(&format!("{:02x} ", b));
    }
    info!("{}: {}", prefix, line.trim());
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
    use stem::syscall::vfs::{vfs_close, vfs_open, vfs_write};

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

fn serve_usb_block(controller: XhciController, storage: UsbMassStorage) -> ! {
    let _ = vfs_mkdir("/dev/block");
    let (v_w, v_r) = match port_create(65536) {
        Ok(p) => p,
        Err(e) => {
            error!("ums: failed to create provider port: {:?}", e);
            loop {
                stem::time::sleep_ms(60_000);
            }
        }
    };
    if let Err(e) = vfs_mount(v_w, "/dev/block/usb0") {
        error!("ums: failed to mount /dev/block/usb0: {:?}", e);
        loop {
            stem::time::sleep_ms(60_000);
        }
    }
    info!("ums: mounted read-only block device at /dev/block/usb0");
    let mut provider = UsbBlockProvider { controller, storage };
    let mut ploop = ProviderLoop::new(v_r);
    loop {
        let req = match ploop.next_request() {
            Ok(req) => req,
            Err(e) => {
                warn!("ums: provider loop closed: {:?}", e);
                stem::time::sleep_ms(1000);
                continue;
            }
        };
        let resp = provider.handle_rpc(&req);
        let _ = ploop.send_response(&req, resp);
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

    let mut controller = match XhciController::new(claim, mmio) {
        Ok(controller) => controller,
        Err(e) => {
            error!("xhci: controller init failed: {}", e);
            stem::syscall::exit(Status::BindFailed as i32);
        }
    };

    publish_status(&device_path);
    loop {
        if let Some(storage) = controller.scan_and_enumerate() {
            serve_usb_block(controller, storage);
        }
        controller.wait_for_event_delay();
        let status = read32(controller.regs.op, OP_USBSTS);
        if (status & (USBSTS_EINT | USBSTS_PCD)) != 0 {
            write32(controller.regs.op, OP_USBSTS, status & (USBSTS_EINT | USBSTS_PCD));
        }
    }
}
