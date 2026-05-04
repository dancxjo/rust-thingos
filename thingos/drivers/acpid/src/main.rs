#![no_std]
#![no_main]

acpi_common::declare_acpi_driver!(
    b"acpid",
    b"drv.AcpiNamespace",
    abi::driver_interface::DriverClass::Other,
    "ACPI namespace service online",
    "Waiting for AML namespace evaluation support"
);
