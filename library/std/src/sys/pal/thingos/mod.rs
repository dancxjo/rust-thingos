#![deny(unsafe_op_in_unsafe_fn)]

// ThingOS targets are currently abort-only for panics.
// Emit a clear compile-time error for any attempt to build std with unwind.
#[cfg(panic = "unwind")]
compile_error!(
    "ThingOS std PAL is abort-only: panic=unwind is not supported for target_os=thingos"
);

pub(crate) mod abi;
pub mod common;
pub mod futex;
pub mod os;
pub mod time;

pub use common::*;

#[cfg(not(test))]
static mut BOOT_ARG: usize = 0;

/// Returns the raw scheduler-provided process argument captured at entry.
///
/// Regular std programs should use ordinary `fn main()` signatures. Driver and
/// seed-aware programs that need the boot/module context can read this value
/// through `std::os::thingos::boot_arg()`.
pub(crate) fn boot_arg() -> usize {
    #[cfg(not(test))]
    unsafe {
        BOOT_ARG
    }
    #[cfg(test)]
    {
        0
    }
}

#[cfg(not(test))]
#[unsafe(no_mangle)]
extern "C" fn thingos_std_runtime_setup() {
    // ThingOS std startup currently needs no extra runtime initialization.
}

#[cfg(not(test))]
#[unsafe(no_mangle)]
extern "C" fn thingos_start(boot_arg: usize) -> ! {
    unsafe extern "C" {
        fn main(_: isize, _: *const *const u8, _: u8) -> i32;
    }

    unsafe {
        BOOT_ARG = boot_arg;
    }
    unsafe {
        thingos_std_runtime_setup();
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
        // RDI holds the raw scheduler-provided process argument.
        // CALL creates the standard SysV entry stack shape for Rust code.
        call thingos_start
        ud2

    // Unified driver entry trampoline for Catalog-mode spawns.
    // Preserves SysV alignment (call pushes 8 bytes) and ensures TLS is ready.
    .global thingos_driver_trampoline
    thingos_driver_trampoline:
        // RSP is 16-byte aligned on entry to the process.
        // We need 16n + 8 for the C entry point.
        sub rsp, 8
        // Call runtime setup (TLS, etc)
        call thingos_std_runtime_setup
        // Restore stack for the actual start logic
        add rsp, 8
        // Jump to the driver's handle_start logic (must be provided or jumped from here)
        // For simplicity, we assume RDI still holds the context pointer.
        // But we need to know WHERE the driver's start is.
        // Better: let the driver's start symbol BE the trampoline target.
        ret
"#
);

#[cfg(all(target_arch = "aarch64", not(test)))]
crate::arch::global_asm!(
    r#"
    .section .text.entry
    .global _start
    _start:
        // x0 holds the raw scheduler-provided process argument.
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
        // a0 holds the raw scheduler-provided process argument.
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
        // a0 holds the raw scheduler-provided process argument.
        bl thingos_start
        break 0
"#
);
