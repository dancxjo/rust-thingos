Feature: ACPI platform driver modules

  ACPI support is split into early userland drivers and services so laptop
  firmware support can grow without moving policy into the kernel.

  Background:
    Given the machine is booted

  Scenario: ACPI platform modules launch from boot modules
    Then the log should match pattern "acpid.*ACPI namespace service online"
    And the log should match pattern "acpi_ec.*ACPI EC service online"
    And the log should match pattern "acpi_power.*ACPI power service online"
    And the log should match pattern "acpi_battery.*ACPI battery service online"
    And the log should match pattern "acpi_thermal.*ACPI thermal service online"
    And the log should match pattern "acpi_backlight.*ACPI backlight service online"
    And the log should match pattern "acpi_i2c_hid.*ACPI I2C HID discovery service online"
    And the log should match pattern "acpi_gpio_keys.*ACPI GPIO keys service online"
    And the log should match pattern "acpi_irq_routing.*ACPI IRQ routing service online"
    And the log should match pattern "acpi_sleep.*ACPI sleep service online"
    And the log should match pattern "acpi_vendor_hotkeys.*ACPI vendor hotkeys service online"
