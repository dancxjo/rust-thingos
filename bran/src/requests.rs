use kernel::{BootModuleDesc, BootModuleKind};
use limine::BaseRevision;
use limine::request::{
    DeviceTreeBlobRequest, FramebufferRequest, HhdmRequest, ExecutableFileRequest, MemoryMapRequest,
    ModuleRequest, RsdpRequest,
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
static mut MODULES_LEN: usize = 0;
static mut MODULES_INIT: bool = false;

pub fn get_modules() -> &'static [BootModuleDesc] {
    unsafe {
        if !MODULES_INIT {
            if let Some(response) = MODULE_REQUEST.get_response() {
                let files = response.modules();
                let count = core::cmp::min(files.len(), MAX_MODULES);
                kernel::kdebug!("Limine: Found {} boot modules", files.len());
                for i in 0..count {
                    let file = files[i];

                    // Name
                    let name = file.path().to_str().unwrap_or("unknown");
                    let cmdline = core::str::from_utf8(file.cmdline()).unwrap_or("");
                    kernel::ktrace!(
                        "  [{}] {} (cmdline='{}') size={}",
                        i,
                        name,
                        cmdline,
                        file.size()
                    );

                    // Data
                    let ptr = file.addr();
                    let len = file.size() as usize;
                    let bytes = core::slice::from_raw_parts(ptr, len);

                    // Physical address
                    let hhdm = HHDM_REQUEST.get_response().map(|r| r.offset()).unwrap_or(0);
                    let virt_addr = ptr as u64;
                    let phys_start = if virt_addr >= hhdm {
                        virt_addr - hhdm
                    } else {
                        virt_addr
                    };

                    MODULES_CACHE[i] = BootModuleDesc {
                        name,
                        cmdline,
                        bytes,
                        phys_start,
                        phys_end: phys_start + len as u64,
                        kind: BootModuleKind::Unknown,
                    };
                }
                MODULES_LEN = count;
            }
            MODULES_INIT = true;
        }
        &MODULES_CACHE[..MODULES_LEN]
    }
}

pub fn get_kernel_cmdline() -> &'static str {
    EXECUTABLE_FILE_REQUEST
        .get_response()
        .and_then(|r: &limine::response::ExecutableFileResponse| {
            core::str::from_utf8(r.file().cmdline()).ok()
        })
        .unwrap_or("")
}

