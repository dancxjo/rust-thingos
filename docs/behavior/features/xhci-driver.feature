Feature: xHCI userspace driver
  xHCI controllers are handled by a userspace driver that binds through the
  standard PCI device claim, MMIO, DMA, and IRQ device syscalls.

  Background:
    Given the machine is booted

  @smoke
  @timeout-60s
  Scenario: xHCI driver is available to Cambium
    Then the serial output should contain "DEVD CATALOG: registered driver '/drivers/xhci'"
