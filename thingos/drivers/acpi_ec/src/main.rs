#![no_std]
#![no_main]

acpi_common::declare_acpi_driver!(
    b"acpi_ec",
    b"drv.AcpiEc",
    abi::driver_interface::DriverClass::Other,
    "ACPI EC service online",
    "Waiting for EC operation-region and GPE routing support"
);
