#![no_std]
#![no_main]

acpi_common::declare_acpi_driver!(
    b"acpi_battery",
    b"drv.AcpiBattery",
    abi::driver_interface::DriverClass::Other,
    "ACPI battery service online",
    "Waiting for battery and AC-adapter namespace devices"
);
