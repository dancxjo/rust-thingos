#![no_std]
#![no_main]

acpi_common::declare_acpi_driver!(
    b"acpi_vendor_hotkeys",
    b"drv.AcpiVendorHotkeys",
    abi::driver_interface::DriverClass::Input,
    "ACPI vendor hotkeys service online",
    "Waiting for vendor-specific ACPI/WMI hotkey support"
);
