#![no_std]
#![no_main]

acpi_common::declare_acpi_driver!(
    b"acpi_irq_routing",
    b"drv.AcpiIrqRouting",
    abi::driver_interface::DriverClass::Other,
    "ACPI IRQ routing service online",
    "Waiting for PCI _PRT and GSI routing support"
);
