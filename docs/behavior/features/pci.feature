Feature: PCI Bus and Discovery
  The kernel scans the PCI bus at boot and matches discovered devices
  with userspace drivers via Sprout and Cambium.

  Scenario: PCI bus scan identifies hardware
    Given the machine is started
    When the kernel initializes the PCI subsystem
    Then the serial output should contain "PCI: Scanning bus 0"
    And the serial output should contain "PCI: Found device"

  Scenario: PCI device nodes are published in VFS
    Given the machine is booted
    Then the directory "/dev/pci" should contain device nodes
    And each node should have "vendor_id" and "device_id" attributes

  Scenario: Sysfs publishes an atomic device discovery snapshot
    Given the machine is booted
    When I wait for the shell prompt
    And the shell command "cat /sys/device_snapshot" succeeds
    Then the latest command output should contain "pci-0000:"
    And the latest command output should contain "present"
