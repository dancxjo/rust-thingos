#![deny(unsafe_op_in_unsafe_fn)]

// ThingOS targets are currently abort-only for panics.
// Emit a clear compile-time error for any attempt to build std with unwind.
#[cfg(panic = "unwind")]
compile_error!(
    "ThingOS std PAL is abort-only: panic=unwind is not supported for target_os=thingos"
);

pub mod common;
pub mod futex;
pub mod os;
pub mod time;

pub use common::*;

#[cfg(not(test))]
#[unsafe(no_mangle)]
extern "C" fn thingos_start() -> ! {
    use crate::sys::thingos_syscall_numbers::SYS_TASK_SET_TLS_BASE;

    unsafe extern "C" {
        fn main(_: isize, _: *const *const u8, _: u8) -> i32;
    }

    // Allocate and configure main thread TLS if present
    let tls_base = crate::sys::thread::thingos::allocate_tls_block();
    if tls_base != 0 {
        unsafe {
            crate::sys::pal::raw_syscall6(SYS_TASK_SET_TLS_BASE, tls_base, 0, 0, 0, 0, 0);
        }
    }

    let code = unsafe { main(0, core::ptr::null(), 0) };
    os::exit(code)
}

#[cfg(all(target_arch = "x86_64", not(test)))]
crate::arch::global_asm!(
    r#"
    .section .text.entry
    .global _start
    _start:
        // Kernel jumps here with RSP 16-byte aligned.
        // CALL creates the standard SysV entry stack shape for Rust code.
        call thingos_start
        ud2
"#
);

#[cfg(all(target_arch = "aarch64", not(test)))]
crate::arch::global_asm!(
    r#"
    .section .text.entry
    .global _start
    _start:
        bl thingos_start
        brk #1
"#
);

#[cfg(all(target_arch = "riscv64", not(test)))]
crate::arch::global_asm!(
    r#"
    .section .text.entry
    .global _start
    _start:
        call thingos_start
        unimp
"#
);

#[cfg(all(target_arch = "loongarch64", not(test)))]
crate::arch::global_asm!(
    r#"
    .section .text.entry
    .global _start
    _start:
        bl thingos_start
        break 0
"#
);
