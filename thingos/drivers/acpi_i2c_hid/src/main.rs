#![no_std]
#![no_main]

acpi_common::declare_acpi_driver!(
    b"acpi_i2c_hid",
    b"drv.AcpiI2cHid",
    abi::driver_interface::DriverClass::Input,
    "ACPI I2C HID discovery service online",
    "Waiting for ACPI _CRS I2C/GPIO interrupt decoding"
);
