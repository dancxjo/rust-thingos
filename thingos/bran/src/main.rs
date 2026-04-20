#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

mod arch;
pub mod console;
mod framebuffer;
mod mem;
mod requests;
pub mod runtime;
mod theme;

use arch::hcf;
use core::assert;
use framebuffer::Framebuffer;
use kernel::{BootRuntime, BootTasking};

use requests::{BASE_REVISION, FRAMEBUFFER_REQUEST};

pub static RUNTIME: arch::CurrentRuntime = arch::create_runtime();

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    // Ultra-early Proof of Life (semihosting)
    unsafe {
        for &b in b"KMAIN\r\n" {
            core::arch::asm!("hlt #0xF000", in("w0") 0x03, in("x1") &b);
        }
    }

    // Architecture-specific early initialization (e.g., stack mode switching on AArch64)
    unsafe {
        RUNTIME.early_init();
    }

    assert!(BASE_REVISION.is_supported());

    if FRAMEBUFFER_REQUEST.get_response().is_none() {
        hcf();
    }

    // Initialize paging and UART before logging
    let hhdm_offset = requests::HHDM_REQUEST.get_response().map(|r| r.offset()).unwrap_or(0);
    RUNTIME.init(hhdm_offset);

    unsafe {
        kernel::logging::init(&RUNTIME);
    }

    // Post-logging_init marker
    kernel::kinfo!("ALIVE (UART/LOGGING READY)");

    kernel::start(&RUNTIME);
}

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout);
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    kernel::kerror!("{}", info);
    
    // Semihosting FAULT marker
    unsafe {
        for &b in b"FAULT\r\n" {
            core::arch::asm!("hlt #0xF000", in("w0") 0x03, in("x1") &b);
        }
    }

    hcf();
}
