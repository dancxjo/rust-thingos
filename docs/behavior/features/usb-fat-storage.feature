Feature: Reading files from a USB FAT volume
  A USB mass-storage device with a FAT partition should appear as ordinary
  read-only media. Users should not need to care that xHCI, partition scanning,
  and fatd are separate userland services.

  Background:
    Given the machine is booted with a USB FAT image

  Scenario: The USB FAT partition is discovered and mounted
    When I wait for the shell prompt
    Then the serial output should contain "ums: whole-disk service provider online for /dev/block/usb0" within 120s
    And the serial output should contain "ums: MBR partition 1 type=" within 120s
    And the serial output should contain "ums: mounted partition 1 at /dev/block/usb0p1" within 120s
    And the serial output should not contain "USB partition scan failed to open /dev/block/usb0"
    And the serial output should contain "fatd: found Fat16 filesystem on /dev/block/usb0p1" within 120s
    And the serial output should contain "fatd: mounted /dev/block/usb0p1 at /media/usb" within 120s

  Scenario: Files on the USB FAT volume can be listed and read
    When I wait for the shell prompt
    Then the serial output should contain "fatd: mounted /dev/block/usb0p1 at /media/usb" within 120s
    When I wait for the shell prompt
    When I type "ls /media/usb" on the serial console
    Then the command output should not contain "No such file"
    When I type "cat /media/usb/hello.txt" on the serial console
    Then the latest command output should contain "hello"
    When I type "cat /media/usb/long-file-name.txt" on the serial console
    Then the latest command output should contain "long filename"
