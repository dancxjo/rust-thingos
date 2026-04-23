Feature: ISO9660 boot filesystem mount

  Scenario: iso9660d mounts the boot filesystem at /mnt/iso
    Given the machine is booted
    Then the log should match pattern "iso9660d: found ISO9660 on device (atapi|ata_|ahci)"
    And I should see "iso9660d: mounted at /mnt/iso" after "iso9660d: found ISO9660 on device"
    When I wait for the shell prompt
    And I type "cat /mnt/iso/etc/hostname" on the serial console
    Then the latest command output should contain "thingos"

  Scenario: iso9660d serves multiple sequential reads correctly via parallel dispatch
    Given the machine is booted
    When I wait for the shell prompt
    And I type "cat /mnt/iso/etc/hostname" on the serial console
    Then the latest command output should contain "thingos"
    When I type "cat /mnt/iso/etc/hostname" on the serial console
    Then the latest command output should contain "thingos"
