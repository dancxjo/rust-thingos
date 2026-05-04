#![no_std]

#[macro_export]
macro_rules! declare_acpi_driver {
    ($driver_name:expr, $device_kind:expr, $driver_class:expr, $online_message:expr, $wait_message:expr) => {
        use abi::driver_interface::{
            DRIVER_DESCRIPTOR_ABI_VERSION, DeviceInfo, DriverClass, DriverDescriptor,
            DriverEntryCtx, ProbeResult, Status,
        };
        use stem::abi::module_manifest::{
            MANIFEST_MAGIC, ManifestHeader, ModuleKind, device_kind_bytes,
        };

        const THINGOS_DRIVER_NAME: &[u8] = $driver_name;
        const THINGOS_DEVICE_KIND: &[u8] = $device_kind;

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
            driver_class: $driver_class,
            flags: 0,
            probe: thingos_driver_probe,
            #[cfg(target_arch = "x86_64")]
            start: thingos_driver_start_safe
                as unsafe extern "C" fn(ctx: *const DriverEntryCtx) -> Status,
            #[cfg(not(target_arch = "x86_64"))]
            start: thingos_driver_start,
        };

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

        #[cfg(target_arch = "x86_64")]
        #[unsafe(no_mangle)]
        unsafe extern "C" fn thingos_driver_start_rust(ctx: *const DriverEntryCtx) -> Status {
            thingos_driver_start(ctx)
        }

        unsafe extern "C" fn thingos_driver_probe(
            _dev: *const DeviceInfo,
            out: *mut ProbeResult,
        ) -> Status {
            if out.is_null() {
                return Status::InvalidArgument;
            }
            let out = &mut *out;
            out.matched = 0;
            out.score = 0;
            out.claimed_class = $driver_class;
            out.flags = 0;
            Status::NoMatch
        }

        unsafe extern "C" fn thingos_driver_start(_ctx: *const DriverEntryCtx) -> Status {
            main(0)
        }

        #[unsafe(link_section = ".thing_manifest")]
        #[unsafe(no_mangle)]
        #[used]
        pub static MANIFEST: ManifestHeader = ManifestHeader {
            magic: MANIFEST_MAGIC,
            kind: ModuleKind::Driver,
            device_kind: device_kind_bytes(THINGOS_DEVICE_KIND),
            version: 1,
            _reserved: 0,
        };

        #[stem::main]
        fn main(_raw_arg: usize) -> ! {
            stem::info!($online_message);
            stem::debug!($wait_message);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };
}
