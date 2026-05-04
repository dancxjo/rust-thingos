#![no_std]
#![no_main]

acpi_common::declare_acpi_driver!(
    b"acpi_thermal",
    b"drv.AcpiThermal",
    abi::driver_interface::DriverClass::Other,
    "ACPI thermal service online",
    "Waiting for thermal zones and cooling-device support"
);
