use spin::RwLock;

use crate::{BootModuleDesc, FramebufferInfo, PhysRange};

pub static BOOT_INFO: RwLock<Option<BootSyscallInfo>> = RwLock::new(None);

#[derive(Clone, Copy)]
pub struct BootSyscallInfo {
    pub memory_map: &'static [PhysRange],
    pub modules: &'static [BootModuleDesc],
    pub framebuffer: Option<FramebufferInfo>,
    pub hhdm_offset: u64,
    pub acpi_rsdp: Option<u64>,
    pub dtb_ptr: Option<u64>,
}

pub fn set(info: BootSyscallInfo) {
    if crate::is_runtime_initialized() {
        crate::runtime_base().serial_putbuf(b"[kernel:boot_info] set begin\r\n");
    }
    *BOOT_INFO.write() = Some(info);
    if crate::is_runtime_initialized() {
        crate::runtime_base().serial_putbuf(b"[kernel:boot_info] set ok\r\n");
    }
}

pub fn get() -> Option<BootSyscallInfo> {
    *BOOT_INFO.read()
}
