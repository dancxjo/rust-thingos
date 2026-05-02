Feature: USB storage through the xHCI userspace driver
  USB controllers are driven from userspace through the PCI, MMIO, DMA, IRQ,
  and VFS device interfaces. A user should experience this as removable media
  appearing as read-only block devices without kernel-resident USB logic.

  Background:
    Given the machine is booted with qemu-xhci

  @smoke
  @timeout-120s
  Scenario: A USB mass-storage device is discovered and configured by userland
    Then the serial output should contain "DEVD CATALOG: registered driver '/drivers/xhci'"
    And the serial output should contain "CAMBIUM: discovered xHCI PCI device" within 60s
    And the serial output should contain "class=0x0c0330" within 60s
    And the serial output should contain "CAMBIUM: matched driver '/drivers/xhci' for xHCI PCI device" within 60s
    And the serial output should contain "xhci: starting userspace xHCI driver" within 60s
    And the serial output should contain "xhci: BAR0 mapped" within 60s
    And the serial output should contain "xhci: IRQ enabled" within 90s
    And the serial output should not contain "xhci: irq subscribe failed"
    And the serial output should contain "xhci: controller running" within 90s
    And the serial output should contain "xhci: command completion type=NO_OP success" within 90s
    And the serial output should contain "xhci: port " within 120s
    And the serial output should contain "connected, speed" within 120s
    And the serial output should contain "reset complete, speed" within 120s
    And the serial output should contain "xhci: enable slot -> slot_id=" within 90s
    And the serial output should contain "usb: device attached on xhci0 port" within 120s
    And the serial output should contain "xhci: address device slot=" within 120s
    And the serial output should contain "usb: vid=" within 120s
    And the serial output should contain "usb: class=" within 120s
    And the serial output should contain "subclass=" within 120s
    And the serial output should contain "protocol=" within 120s
    And the serial output should contain "usb: configurations=" within 120s
    And the serial output should contain "usb: interface " within 120s
    And the serial output should contain "usb: endpoint addr=" within 120s
    And the serial output should contain "usb: set configuration" within 120s
    And the serial output should contain "ums: device attached" within 120s

  @smoke
  @timeout-120s
  Scenario: The mass-storage device can be read and published as block media
    Then the serial output should contain "ums: vendor=" within 120s
    And the serial output should contain "product=" within 120s
    And the serial output should contain "ums: capacity " within 120s
    And the serial output should contain "sector_size=512" within 120s
    And the serial output should contain "ums: REQUEST SENSE key=" within 120s
    And the serial output should contain "ums: READ(10) lba=0 count=1 ok" within 120s
    And the serial output should contain "ums: mounted read-only block device at /dev/block/usb0" within 120s
    And the serial output should contain "ums: created /dev/disk/usb0" within 120s
    And the serial output should contain "ums: created /dev/disk/by-bus/usb0" within 120s
    And the serial output should contain "ums: partition scan: read LBA 0 ok" within 120s
    And the serial output should contain "ums: mounted partition 1 at /dev/block/usb0p1" within 120s
