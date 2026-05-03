#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::{String, ToString};

use abi::driver_interface::{
    BusKind, DRIVER_DESCRIPTOR_ABI_VERSION, DRIVER_INTERFACE_ABI_VERSION, DeviceInfo, DriverClass,
    DriverDescriptor, DriverEntryCtx, DriverInterfaceV1, ProbeResult, Status,
};
use abi::vfs_rpc::VFS_RPC_MAX_REQ;
use display_bootfb::driver::BootFbDriver;
use display_bootfb::vfs_provider::dispatch_vfs_rpc;
use ipc_helpers::provider::ProviderLoop;
use stem::abi::module_manifest::{MANIFEST_MAGIC, ManifestHeader, ModuleKind, device_kind_bytes};
use stem::syscall::vfs::vfs_mount;
use stem::syscall::{device_claim, port_create};
use stem::{error, info, warn};

const THINGOS_DRIVER_NAME: &[u8] = b"display_amd_gpu";
const DRIVER_DEVPATH_ENV: &[u8] = b"THINGOS_DRIVER_DEVPATH";
const AMD_VENDOR_ID: u16 = 0x1002;
const AMD_REMBRANDT_680M_DEVICE_ID: u16 = 0x1681;
const PCI_CLASS_DISPLAY: u32 = 0x030000;
const PCI_CLASS_MASK: u32 = 0xff0000;

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
    driver_class: DriverClass::Display,
    flags: 0,
    probe: thingos_driver_probe,
    #[cfg(target_arch = "x86_64")]
    start: thingos_driver_start_safe,
    #[cfg(not(target_arch = "x86_64"))]
    start: thingos_driver_start,
};

#[unsafe(no_mangle)]
#[used]
pub static THING_DRIVER_V1: DriverInterfaceV1 = DriverInterfaceV1 {
    abi_version: DRIVER_INTERFACE_ABI_VERSION,
    flags: 0,
    vendor_id: AMD_VENDOR_ID,
    device_id: AMD_REMBRANDT_680M_DEVICE_ID,
    class_code: PCI_CLASS_DISPLAY,
    class_mask: PCI_CLASS_MASK,
    entry_symbol: [0u8; 32],
};

unsafe extern "C" fn thingos_driver_probe(dev: *const DeviceInfo, out: *mut ProbeResult) -> Status {
    if dev.is_null() || out.is_null() {
        return Status::InvalidArgument;
    }
    let dev = &*dev;
    let out = &mut *out;
    let is_display = (dev.class_code & PCI_CLASS_MASK) == PCI_CLASS_DISPLAY;
    let is_match = dev.bus == BusKind::Pci as u32
        && dev.vendor_id as u16 == AMD_VENDOR_ID
        && dev.device_id as u16 == AMD_REMBRANDT_680M_DEVICE_ID
        && is_display;
    out.matched = if is_match { 1 } else { 0 };
    out.score = if is_match { 1700 } else { 0 };
    out.claimed_class = DriverClass::Display;
    out.flags = 0;
    if is_match { Status::Ok } else { Status::NoMatch }
}

unsafe extern "C" fn thingos_driver_start(ctx: *const DriverEntryCtx) -> Status {
    main(ctx as usize)
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
    thingos_driver_start(ctx)
}

#[unsafe(link_section = ".thing_manifest")]
#[unsafe(no_mangle)]
#[used]
pub static MANIFEST: ManifestHeader = ManifestHeader {
    magic: MANIFEST_MAGIC,
    kind: ModuleKind::Driver,
    device_kind: device_kind_bytes(b"dev.display.Gpu"),
    version: 1,
    _reserved: 0,
};

#[stem::main]
fn main(boot_fd: usize) -> ! {
    info!("Starting AMD Rembrandt display driver...");

    let Some(ctx) = read_driver_ctx(boot_fd) else {
        error!("AMD display driver did not receive a driver context");
        stem::syscall::exit(1);
    };
    let device_path = ctx.device_path_str();
    if device_path.is_empty() {
        error!("AMD display driver received an empty device path");
        stem::syscall::exit(1);
    }
    info!(
        "Recovered driver context for {} vendor=0x{:04x} device=0x{:04x} class=0x{:06x}",
        device_path, ctx.vendor_id, ctx.device_id, ctx.class_code
    );

    if let Err(e) = device_claim(device_path) {
        error!("Failed to claim AMD display device {}: {:?}", device_path, e);
        stem::syscall::exit(1);
    }

    let Some(dev_path) = assigned_dev_path() else {
        error!("Cambium did not assign a display device path");
        stem::syscall::exit(1);
    };

    let mut driver = match BootFbDriver::new() {
        Some(driver) => driver,
        None => {
            error!("AMD display initialization failed: boot framebuffer is unavailable");
            stem::syscall::exit(1);
        }
    };
    driver.driver_name = "display_amd_gpu";
    info!(
        "Using firmware-initialized AMD scanout {}x{} stride={}",
        driver.fb.width, driver.fb.height, driver.fb.stride
    );
    warn!("Native AMD modesetting is not implemented");

    let (vfs_write, vfs_read) = match port_create(VFS_RPC_MAX_REQ * 8) {
        Ok(handles) => handles,
        Err(e) => {
            error!("Failed to create display provider port: {:?}", e);
            stem::syscall::exit(1);
        }
    };

    match vfs_mount(vfs_write, &dev_path) {
        Ok(()) => info!("AMD display service mounted at {}", dev_path),
        Err(e) => {
            error!("Failed to mount AMD display service at {}: {:?}", dev_path, e);
            stem::syscall::exit(1);
        }
    }

    info!("AMD display provider loop online");
    let mut lp = ProviderLoop::new(vfs_read);
    loop {
        let req = match lp.next_request() {
            Ok(r) => r,
            Err(e) => {
                warn!("AMD display provider loop stopped: {:?}", e);
                break;
            }
        };
        let resp = dispatch_vfs_rpc(&mut driver, &req);
        if let Err(e) = lp.send_response(&req, resp) {
            warn!("AMD display RPC response failed: {:?}", e);
        }
    }

    stem::syscall::exit(0);
}

fn read_driver_ctx(boot_fd: usize) -> Option<DriverEntryCtx> {
    if boot_fd == 0 {
        return None;
    }
    let req = abi::vm::VmMapReq {
        addr_hint: 0,
        len: core::mem::size_of::<DriverEntryCtx>(),
        prot: abi::vm::VmProt::READ | abi::vm::VmProt::USER,
        flags: abi::vm::VmMapFlags::empty(),
        backing: abi::vm::VmBacking::File { thing: boot_fd as u32, offset: 0 },
    };
    let resp = stem::syscall::vm_map(&req).ok()?;
    let ctx = unsafe { core::ptr::read_unaligned(resp.addr as *const DriverEntryCtx) };
    if ctx.version == 1 { Some(ctx) } else { None }
}

fn assigned_dev_path() -> Option<String> {
    let mut buf = [0u8; 128];
    let len = stem::syscall::env_get(DRIVER_DEVPATH_ENV, &mut buf).ok()?;
    if len == 0 || len > buf.len() {
        return None;
    }
    let path = core::str::from_utf8(&buf[..len]).ok()?.trim();
    if path.is_empty() { None } else { Some(path.to_string()) }
}
