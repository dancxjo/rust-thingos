#![no_std]
#![no_main]

acpi_common::declare_acpi_driver!(
    b"acpi_backlight",
    b"drv.AcpiBacklight",
    abi::driver_interface::DriverClass::Display,
    "ACPI backlight service online",
    "Waiting for ACPI video/backlight method support"
);
