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

  @smoke
  @timeout-90s
  Scenario: xHCI command and event ring path completes No-Op command
    Then the serial output should contain "xhci: controller running" within 90s
    And the serial output should contain "xhci: command completion type=NO_OP success" within 90s

  @smoke
  @timeout-90s
  Scenario: xHCI Enable Slot command returns a valid slot ID
    Then the serial output should contain "xhci: controller running" within 90s
    And the serial output should contain "xhci: enable slot -> slot_id=" within 90s

  @smoke
  @timeout-120s
  Scenario: EP0 port reset completes for a connected device
    Then the serial output should contain "xhci: port " within 120s
    And the serial output should contain "connected, speed" within 120s
    And the serial output should contain "reset complete, speed" within 120s

  @smoke
  @timeout-120s
  Scenario: USB device is addressed and assigned a slot
    Then the serial output should contain "usb: device attached on xhci0 port" within 120s
    And the serial output should contain "xhci: address device slot=" within 120s

  @smoke
  @timeout-120s
  Scenario: USB device descriptor is fetched and VID/PID are logged
    Then the serial output should contain "usb: vid=" within 120s

  @smoke
  @timeout-120s
  Scenario: USB device class, subclass, and protocol are logged
    Then the serial output should contain "usb: class=" within 120s
    And the serial output should contain "subclass=" within 120s
    And the serial output should contain "protocol=" within 120s

  @smoke
  @timeout-120s
  Scenario: USB configuration count is logged
    Then the serial output should contain "usb: configurations=" within 120s

  @smoke
  @timeout-120s
  Scenario: USB configuration descriptor is parsed with interface and endpoint descriptors
    Then the serial output should contain "usb: interface " within 120s
    And the serial output should contain "usb: endpoint addr=" within 120s

  @smoke
  @timeout-120s
  Scenario: SET_CONFIGURATION completes successfully
    Then the serial output should contain "usb: set configuration" within 120s
