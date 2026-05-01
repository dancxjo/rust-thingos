Feature: xHCI userspace driver
  xHCI controllers are handled by a userspace driver that binds through the
  standard PCI device claim, MMIO, DMA, and IRQ device syscalls.

  Background:
    Given the machine is booted with qemu-xhci

  @smoke
  @timeout-60s
  Scenario: xHCI PCI controller binds and starts the userspace driver
    Then the serial output should contain "DEVD CATALOG: registered driver '/drivers/xhci'"
    And the serial output should contain "CAMBIUM: discovered xHCI PCI device" within 60s
    And the serial output should contain "class=0x0c0330" within 60s
    And the serial output should contain "CAMBIUM: matched driver '/drivers/xhci' for xHCI PCI device" within 60s
    And the serial output should contain "xhci: starting userspace xHCI driver" within 60s
    And the serial output should contain "xhci: BAR0 mapped" within 60s
