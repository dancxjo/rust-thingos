use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use kernel::{BootModuleDesc, BootModuleKind};
use limine::BaseRevision;
use limine::request::{
    DeviceTreeBlobRequest, ExecutableFileRequest, FramebufferRequest, HhdmRequest,
    MemoryMapRequest, ModuleRequest, RsdpRequest,
};

#[unsafe(no_mangle)]
#[used]
#[unsafe(link_section = ".limine_reqs")]
pub static BASE_REVISION: BaseRevision = BaseRevision::new();

#[unsafe(no_mangle)]
#[used]
#[unsafe(link_section = ".limine_reqs")]
pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[unsafe(no_mangle)]
#[used]
#[unsafe(link_section = ".limine_reqs")]
pub static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

#[unsafe(no_mangle)]
#[used]
#[unsafe(link_section = ".limine_reqs")]
pub static MODULE_REQUEST: ModuleRequest = ModuleRequest::new();

#[unsafe(no_mangle)]
#[used]
#[unsafe(link_section = ".limine_reqs")]
pub static RSDP_REQUEST: RsdpRequest = RsdpRequest::new();

#[unsafe(no_mangle)]
#[used]
#[unsafe(link_section = ".limine_reqs")]
pub static DTB_REQUEST: DeviceTreeBlobRequest = DeviceTreeBlobRequest::new();

#[unsafe(no_mangle)]
#[used]
#[unsafe(link_section = ".limine_reqs")]
pub static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

#[unsafe(no_mangle)]
#[used]
#[unsafe(link_section = ".limine_reqs")]
pub static EXECUTABLE_FILE_REQUEST: ExecutableFileRequest = ExecutableFileRequest::new();

const MAX_MODULES: usize = 256;

static mut MODULES_CACHE: [BootModuleDesc; MAX_MODULES] = [BootModuleDesc {
    name: "",
    cmdline: "",
    bytes: &[],
    phys_start: 0,
    phys_end: 0,
    kind: BootModuleKind::Unknown,
}; MAX_MODULES];

/// Number of valid entries written into `MODULES_CACHE`.
static MODULES_LEN: AtomicUsize = AtomicUsize::new(0);

/// Set to `true` (with `Release`) once `MODULES_CACHE` and `MODULES_LEN` are
/// fully written.  Readers load this with `Acquire` to observe the completed
/// writes.
static MODULES_INIT: AtomicBool = AtomicBool::new(false);

#[cfg(target_arch = "x86_64")]
fn early_serial_write(msg: &[u8]) {
    unsafe {
        let port = 0x3f8u16;
        for &b in msg {
            let mut spins = 0u32;
            loop {
                let lsr: u8;
                core::arch::asm!(
                    "in al, dx",
                    out("al") lsr,
                    in("dx") port + 5,
                    options(nostack, preserves_flags)
                );
                if (lsr & 0x20) != 0 || spins >= 100_000 {
                    break;
                }
                spins = spins.saturating_add(1);
                core::hint::spin_loop();
            }
            core::arch::asm!(
                "out dx, al",
                in("dx") port,
                in("al") b,
                options(nostack, preserves_flags)
            );
        }
    }
}

#[cfg(not(target_arch = "x86_64"))]
fn early_serial_write(_msg: &[u8]) {}

pub fn get_modules() -> &'static [BootModuleDesc] {
    // Fast path: already initialized.  The Acquire load synchronises with the
    // Release store below, guaranteeing visibility of MODULES_CACHE contents.
    if MODULES_INIT.load(Ordering::Acquire) {
        let len = MODULES_LEN.load(Ordering::Relaxed);
        return unsafe { &MODULES_CACHE[..len] };
    }

    let count = if let Some(response) = MODULE_REQUEST.get_response() {
        let files = response.modules();
        let total = files.len();

        let count = core::cmp::min(total, MAX_MODULES);
        for i in 0..count {
            let file = files[i];

            // Name
            let name = file.path().to_str().unwrap_or("unknown");
            let cmdline = core::str::from_utf8(file.cmdline()).unwrap_or("");

            // Data
            let ptr = file.addr();
            let len = file.size() as usize;
            let bytes = unsafe { core::slice::from_raw_parts(ptr, len) };

            // Physical address
            let hhdm = HHDM_REQUEST.get_response().map(|r| r.offset()).unwrap_or(0);
            let virt_addr = ptr as u64;
            let phys_start = if virt_addr >= hhdm { virt_addr - hhdm } else { virt_addr };

            unsafe {
                MODULES_CACHE[i] = BootModuleDesc {
                    name,
                    cmdline,
                    bytes,
                    phys_start,
                    phys_end: phys_start + len as u64,
                    kind: BootModuleKind::Unknown,
                };
            }
        }
        count
    } else {
        0
    };

    // Publish count (Relaxed — the Release on MODULES_INIT below provides the
    // ordering guarantee to paired Acquire readers).
    MODULES_LEN.store(count, Ordering::Relaxed);
    // Release: ensures all writes to MODULES_CACHE and MODULES_LEN happen-before
    // any Acquire load of MODULES_INIT in another CPU.
    MODULES_INIT.store(true, Ordering::Release);

    unsafe { &MODULES_CACHE[..count] }
}

pub fn get_kernel_cmdline() -> &'static str {
    EXECUTABLE_FILE_REQUEST
        .get_response()
        .and_then(|r: &limine::response::ExecutableFileResponse| {
            core::str::from_utf8(r.file().cmdline()).ok()
        })
        .unwrap_or("")
}
