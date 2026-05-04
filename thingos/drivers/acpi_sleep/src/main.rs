#![no_std]
#![no_main]

acpi_common::declare_acpi_driver!(
    b"acpi_sleep",
    b"drv.AcpiSleep",
    abi::driver_interface::DriverClass::Other,
    "ACPI sleep service online",
    "Waiting for S-state and wake-source support"
);
