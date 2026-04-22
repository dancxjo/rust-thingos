Feature: ISO9660 boot filesystem mount

  Scenario: iso9660d mounts the boot filesystem at /mnt/iso
    Given the machine is booted
    Then the serial output should contain "iso9660d: found ISO9660 on device"
    And I should see "iso9660d: mounted at /mnt/iso" after "iso9660d: found ISO9660 on device"
    When I wait for the shell prompt
    And I type "ls /mnt/iso" on the serial console
    Then the latest serial output should not contain "No such file or directory"
