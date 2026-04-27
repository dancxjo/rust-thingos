#[cfg(all(feature = "rt", not(feature = "std")))]
extern "C" {
    fn stem_user_main(arg: usize) -> !;
}

#[cfg(all(feature = "rt", not(feature = "std")))]
#[no_mangle]
pub unsafe extern "C" fn thingos_runtime_setup() {
    if let Some(info) = crate::tls::read_tls_info() {
        let _ = crate::tls::setup_thread_tls(&info);
    }
}

#[cfg(all(feature = "rt", not(feature = "std")))]
#[no_mangle]
pub unsafe extern "C" fn entry_impl(arg: usize) -> ! {
    thingos_runtime_setup();
    stem_user_main(arg)
}

#[cfg(all(
    target_arch = "x86_64",
    feature = "rt",
    any(target_os = "none", any(target_os = "thingos", target_env = "thingos"))
))]
core::arch::global_asm!(
    r#"
    .section .text.entry
    .global _start
    _start:
        // Kernel jumps here. RSP is 16-byte aligned (e.g. 0x400000).
        // RDI holds the argument for stem_user_main.
        // CALL instruction pushes 8 bytes, so RSP becomes aligned-8.
        call entry_impl
        ud2
"#
);

#[cfg(all(
    target_arch = "aarch64",
    feature = "rt",
    any(target_os = "none", any(target_os = "thingos", target_env = "thingos"))
))]
core::arch::global_asm!(
    r#"
    .section .text.entry
    .global _start
    _start:
        // x0 holds arg0
        bl entry_impl
        brk #1
    "#
);

#[cfg(all(
    target_arch = "riscv64",
    feature = "rt",
    any(target_os = "none", any(target_os = "thingos", target_env = "thingos"))
))]
core::arch::global_asm!(
    r#"
    .section .text.entry
    .global _start
    _start:
        // a0 holds arg0
        call entry_impl
        unimp
    "#
);

#[cfg(all(
    target_arch = "loongarch64",
    feature = "rt",
    any(target_os = "none", any(target_os = "thingos", target_env = "thingos"))
))]
core::arch::global_asm!(
    r#"
    .section .text.entry
    .global _start
    _start:
        // a0 holds arg0
        bl entry_impl
        break 0
    "#
);
