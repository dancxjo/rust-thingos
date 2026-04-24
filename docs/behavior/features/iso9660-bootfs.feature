Feature: ISO9660 boot filesystem mount

  Scenario: iso9660d mounts the boot filesystem at /media/cdrom
    Given the machine is booted
    Then the log should match pattern "iso9660d: found ISO9660 on device (atapi|ata_|ahci)"
    And I should see "iso9660d: mounted at /media/cdrom" after "iso9660d: found ISO9660 on device"
    When I wait for the shell prompt
    And I type "cat /media/cdrom/etc/hostname" on the serial console
    Then the latest command output should contain "thingos"

  Scenario: iso9660d serves multiple sequential reads correctly via parallel dispatch
    Given the machine is booted
    When I wait for the shell prompt
    And I type "cat /media/cdrom/etc/hostname" on the serial console
    Then the latest command output should contain "thingos"
    When I type "cat /media/cdrom/etc/hostname" on the serial console
    Then the latest command output should contain "thingos"

  Scenario: iso9660d handles concurrent reads and readdir without response mis-correlation
    Given the machine is booted
    When I wait for the shell prompt
    And I type "ls /media/cdrom && cat /media/cdrom/etc/hostname" on the serial console
    Then the latest command output should contain "thingos"
    And I type "ls /media/cdrom/etc && cat /media/cdrom/etc/hostname" on the serial console
    Then the latest command output should contain "thingos"

  Scenario: boot ISO files are accessible at /media/cdrom via QEMU -cdrom
    Given the machine is booted
    When I wait for the shell prompt
    And I type "cd /media/cdrom" on the serial console
    And I type "ls" on the serial console
    Then the latest command output should contain "etc"
