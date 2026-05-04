#![no_std]
#![no_main]

acpi_common::declare_acpi_driver!(
    b"acpi_power",
    b"drv.AcpiPower",
    abi::driver_interface::DriverClass::Other,
    "ACPI power service online",
    "Waiting for power-button, lid, and fixed-event support"
);
