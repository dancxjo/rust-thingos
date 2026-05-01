Feature: lsusb utility
  lsusb reports USB devices and USB host controllers from sysfs.

  Scenario: lsusb runs from the serial shell
    Given the machine is booted
    When I wait for the shell prompt
    And the shell command "lsusb" succeeds
    Then the latest command output should contain "USB"
