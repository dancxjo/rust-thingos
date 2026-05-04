#![no_std]
#![no_main]

acpi_common::declare_acpi_driver!(
    b"acpi_gpio_keys",
    b"drv.AcpiGpioKeys",
    abi::driver_interface::DriverClass::Input,
    "ACPI GPIO keys service online",
    "Waiting for GPIO event and ACPI notification support"
);
