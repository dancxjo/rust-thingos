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
const EARLY_BOOT_TRACE_ENABLED: bool = false;

fn early_serial_write(msg: &[u8]) {
    if !EARLY_BOOT_TRACE_ENABLED {
        let _ = msg;
        return;
    }
    arch::early_serial_write(msg);
}

fn init_onscreen_terminal() {
    let Some(response) = FRAMEBUFFER_REQUEST.get_response() else {
        return;
    };
    let Some(fb) = response.framebuffers().into_iter().next() else {
        return;
    };
    crate::console::init(Framebuffer::new(&fb));
}

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    arch::early_serial_write(b"KMAIN\r\n");
    early_serial_write(b"[bran] kmain enter\r\n");

    // Architecture-specific early initialization (e.g., stack mode switching on AArch64)
    unsafe {
        RUNTIME.early_init();
    }
    early_serial_write(b"[bran] early_init ok\r\n");

    assert!(BASE_REVISION.is_supported());
    early_serial_write(b"[bran] base revision ok\r\n");

    let framebuffer_present = FRAMEBUFFER_REQUEST.get_response().is_some();
    if !framebuffer_present {
        early_serial_write(b"[bran] no framebuffer; headless mode\r\n");
    }

    // Initialize paging and UART before logging
    let hhdm_offset = requests::HHDM_REQUEST.get_response().map(|r| r.offset()).unwrap_or(0);
    early_serial_write(b"[bran] runtime init...\r\n");
    RUNTIME.init(hhdm_offset);
    early_serial_write(b"[bran] runtime init ok\r\n");
    init_onscreen_terminal();
    kernel::syscall::handlers::register_console_disable(crate::console::disable);

    early_serial_write(b"[bran] logging init begin\r\n");
    unsafe {
        kernel::logging::init(&RUNTIME);
    }
    early_serial_write(b"[bran] logging init ok\r\n");

    if !framebuffer_present {
        early_serial_write(b"[bran] no framebuffer; serial-only mode\r\n");
    }

    // Avoid logger macro calls here while isolating early boot hangs.
    early_serial_write(b"[bran] post-logging marker\r\n");
    early_serial_write(b"[bran] calling kernel::start\r\n");

    kernel::start(&RUNTIME);
}

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout);
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    kernel::kerror!("{}", info);

    // Dump the progress ring so post-mortem analysis can see recent scheduler
    // and IRQ activity before the panic.
    kernel::trace::progress_ring::dump();

    crate::console::flush_sync();
    crate::console::serial_flush_sync();

    arch::early_serial_write(b"FAULT\r\n");

    hcf();
}
