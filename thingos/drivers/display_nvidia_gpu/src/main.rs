#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::{String, ToString};

use abi::display_driver_protocol::{FB_INFO_PAYLOAD_SIZE, FbInfoPayload};
use abi::driver_interface::{
    BusKind, DRIVER_DESCRIPTOR_ABI_VERSION, DRIVER_INTERFACE_ABI_VERSION, DeviceInfo, DriverClass,
    DriverDescriptor, DriverEntryCtx, DriverInterfaceV1, ProbeResult, Status,
};
use abi::errors::Errno;
use abi::syscall::vfs_flags::{O_RDONLY, O_RDWR};
use abi::vfs_rpc::VFS_RPC_MAX_REQ;
use display_bootfb::driver::{BootFbDriver, Framebuffer};
use display_bootfb::vfs_provider::dispatch_vfs_rpc;
use ipc_helpers::provider::ProviderLoop;
use stem::abi::module_manifest::{MANIFEST_MAGIC, ManifestHeader, ModuleKind, device_kind_bytes};
use stem::syscall::vfs::{vfs_close, vfs_mount, vfs_open, vfs_read};
use stem::syscall::{device_claim, device_map_mmio, port_create};
use stem::{error, info, warn};

const THINGOS_DRIVER_NAME: &[u8] = b"display_nvidia_gpu";
const DRIVER_DEVPATH_ENV: &[u8] = b"THINGOS_DRIVER_DEVPATH";
const NVIDIA_VENDOR_ID: u16 = 0x10de;
const PCI_CLASS_DISPLAY: u32 = 0x030000;
const PCI_CLASS_MASK: u32 = 0xff0000;
const NVIDIA_VRAM_BAR: usize = 1;

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
    vendor_id: NVIDIA_VENDOR_ID,
    device_id: 0,
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
    let is_match =
        dev.bus == BusKind::Pci as u32 && dev.vendor_id as u16 == NVIDIA_VENDOR_ID && is_display;
    out.matched = if is_match { 1 } else { 0 };
    out.score = if is_match { 1500 } else { 0 };
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
    info!("Starting NVIDIA display driver...");

    let Some(ctx) = read_driver_ctx(boot_fd) else {
        error!("NVIDIA display driver did not receive a driver context");
        stem::syscall::exit(1);
    };
    let device_path = ctx.device_path_str();
    if device_path.is_empty() {
        error!("NVIDIA display driver received an empty device path");
        stem::syscall::exit(1);
    }
    info!(
        "Recovered driver context for {} vendor=0x{:04x} device=0x{:04x} class=0x{:06x}",
        device_path, ctx.vendor_id, ctx.device_id, ctx.class_code
    );

    let Some(dev_path) = assigned_dev_path() else {
        error!("Cambium did not assign a display device path");
        stem::syscall::exit(1);
    };

    let mut driver = match init_nvidia_bar_framebuffer(device_path) {
        Ok(driver) => driver,
        Err(e) => {
            error!("NVIDIA display initialization failed: {:?}", e);
            stem::syscall::exit(1);
        }
    };

    let (vfs_write, vfs_read) = match port_create(VFS_RPC_MAX_REQ * 8) {
        Ok(handles) => handles,
        Err(e) => {
            error!("Failed to create display provider port: {:?}", e);
            stem::syscall::exit(1);
        }
    };

    match vfs_mount(vfs_write, &dev_path) {
        Ok(()) => info!("NVIDIA display service mounted at {}", dev_path),
        Err(e) => {
            error!("Failed to mount NVIDIA display service at {}: {:?}", dev_path, e);
            stem::syscall::exit(1);
        }
    }

    info!("NVIDIA display provider loop online");
    let mut lp = ProviderLoop::new(vfs_read);
    loop {
        let req = match lp.next_request() {
            Ok(r) => r,
            Err(e) => {
                warn!("NVIDIA display provider loop stopped: {:?}", e);
                break;
            }
        };
        let resp = dispatch_vfs_rpc(&mut driver, &req);
        if let Err(e) = lp.send_response(&req, resp) {
            warn!("NVIDIA display RPC response failed: {:?}", e);
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

fn init_nvidia_bar_framebuffer(device_path: &str) -> Result<BootFbDriver, Errno> {
    let fb_info = read_bootfb_info()?;
    if fb_info.bpp != 32 || fb_info.width == 0 || fb_info.height == 0 || fb_info.stride == 0 {
        warn!(
            "Unsupported firmware framebuffer geometry {}x{} stride={} bpp={}",
            fb_info.width, fb_info.height, fb_info.stride, fb_info.bpp
        );
        return Err(Errno::ENOSYS);
    }

    let fb_bytes =
        (fb_info.height as usize).checked_mul(fb_info.stride as usize).ok_or(Errno::EINVAL)?;
    let bar_size = read_bar_size(device_path, NVIDIA_VRAM_BAR).unwrap_or(0);
    if bar_size != 0 && fb_bytes > bar_size {
        warn!(
            "NVIDIA BAR{} is too small for firmware scanout: need={} size={}",
            NVIDIA_VRAM_BAR, fb_bytes, bar_size
        );
        return Err(Errno::ENOMEM);
    }

    let claim = device_claim(device_path)?;
    let bar = device_map_mmio(claim, NVIDIA_VRAM_BAR)?;
    info!(
        "Mapped NVIDIA BAR{} aperture for firmware scanout {}x{} stride={} bytes={}",
        NVIDIA_VRAM_BAR, fb_info.width, fb_info.height, fb_info.stride, fb_bytes
    );
    warn!("Using firmware-initialized NVIDIA scanout; native modesetting is not implemented");

    Ok(BootFbDriver::with_framebuffer(
        Framebuffer {
            base: bar as *mut u8,
            width: fb_info.width,
            height: fb_info.height,
            stride: fb_info.stride,
            bpp: fb_info.bpp,
        },
        "display_nvidia_gpu",
    ))
}

fn read_bootfb_info() -> Result<FbInfoPayload, Errno> {
    let fd = vfs_open("/dev/fb0", O_RDWR)?;
    let mut payload = FbInfoPayload {
        device_handle: 0,
        width: 0,
        height: 0,
        stride: 0,
        bpp: 0,
        format: 0,
        _reserved: 0,
    };
    let slice = unsafe {
        core::slice::from_raw_parts_mut(&mut payload as *mut _ as *mut u8, FB_INFO_PAYLOAD_SIZE)
    };
    let n = vfs_read(fd, slice);
    let _ = vfs_close(fd);
    if n? < FB_INFO_PAYLOAD_SIZE {
        return Err(Errno::EIO);
    }
    Ok(payload)
}

fn read_bar_size(device_path: &str, bar: usize) -> Result<usize, Errno> {
    let text = read_text(&alloc::format!("{}/bar{}", device_path, bar))?;
    let mut parts = text.split_whitespace();
    let _base = parts.next().ok_or(Errno::EINVAL)?;
    let size = parts.next().ok_or(Errno::EINVAL)?;
    parse_u64(size).map(|v| v as usize)
}

fn read_text(path: &str) -> Result<String, Errno> {
    let fd = vfs_open(path, O_RDONLY)?;
    let mut buf = [0u8; 96];
    let n = vfs_read(fd, &mut buf);
    let _ = vfs_close(fd);
    let n = n?;
    core::str::from_utf8(&buf[..n]).map(|s| s.trim().to_string()).map_err(|_| Errno::EINVAL)
}

fn parse_u64(s: &str) -> Result<u64, Errno> {
    let (radix, digits) = if let Some(hex) = s.strip_prefix("0x") { (16, hex) } else { (10, s) };
    let mut out = 0u64;
    for b in digits.bytes() {
        let val = match b {
            b'0'..=b'9' => (b - b'0') as u64,
            b'a'..=b'f' if radix == 16 => (b - b'a' + 10) as u64,
            b'A'..=b'F' if radix == 16 => (b - b'A' + 10) as u64,
            _ => return Err(Errno::EINVAL),
        };
        if val >= radix {
            return Err(Errno::EINVAL);
        }
        out = out.checked_mul(radix).and_then(|v| v.checked_add(val)).ok_or(Errno::EINVAL)?;
    }
    Ok(out)
}
