Feature: USB FAT storage read-only mount

  Background:
    Given the machine is booted with a USB FAT image

  Scenario: USB block device appears in /dev/block
    When I wait for the shell prompt
    Then the path "/dev/block/usb0" should exist

  Scenario: Partition scanner detects the MBR partition table
    When I wait for the shell prompt
    Then the log should match pattern "ums: MBR partition 1 type="

  Scenario: Partition block device is exposed in /dev/block
    When I wait for the shell prompt
    Then the path "/dev/block/usb0p1" should exist

  Scenario: fatd mounts the FAT partition at /media/usb
    When I wait for the shell prompt
    Then the log should match pattern "fatd: found Fat(16|32) filesystem on /dev/block/usb0p1"
    And the log should match pattern "fatd: mounted /dev/block/usb0p1 at /media/usb"

  Scenario: ls shows directory contents of the mounted FAT volume
    When I wait for the shell prompt
    And I type "ls /media/usb" on the serial console
    Then the latest command output should not contain "No such file"

  Scenario: cat reads a known file from the FAT volume
    When I wait for the shell prompt
    And I type "cat /media/usb/hello.txt" on the serial console
    Then the latest command output should contain "hello"
