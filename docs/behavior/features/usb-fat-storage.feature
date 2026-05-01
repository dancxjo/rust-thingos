Feature: Reading files from a USB FAT volume
  A USB mass-storage device with a FAT partition should appear as ordinary
  read-only media. Users should not need to care that xHCI, partition scanning,
  and fatd are separate userland services.

  Background:
    Given the machine is booted with a USB FAT image

  Scenario: The USB FAT partition is discovered and mounted
    When I wait for the shell prompt
    And I type "ls /dev/block/usb0" on the serial console
    Then the command output should not contain "No such file"
    And the log should match pattern "ums: MBR partition 1 type="
    When I type "ls /dev/block/usb0p1" on the serial console
    Then the command output should not contain "No such file"
    And the log should match pattern "fatd: found Fat(16|32) filesystem on /dev/block/usb0p1"
    And the log should match pattern "fatd: mounted /dev/block/usb0p1 at /media/usb"

  Scenario: Files on the USB FAT volume can be listed and read
    When I wait for the shell prompt
    And I type "ls /media/usb" on the serial console
    Then the command output should not contain "No such file"
    When I type "cat /media/usb/hello.txt" on the serial console
    Then the latest command output should contain "hello"
