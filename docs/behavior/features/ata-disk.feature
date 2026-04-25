Feature: ATA Disk Driver
  The legacy ATA/ATAPI driver provides access to IDE disks and CD-ROMs.

  Scenario: ATA bus probe identifies drives
    Given the machine is started
    When the ATA driver initializes
    Then the serial output should contain "ATA_DISK: Probing primary port"
    And the serial output should contain "ATA_DISK: identify_drive"
    And the serial output should contain "ATA_DISK: Found"

  Scenario: ATA disk is mounted in VFS
    Given the machine is booted
    Then the path "/dev/ata_ctl" should exist
    And the path "/dev/disk0" should exist
