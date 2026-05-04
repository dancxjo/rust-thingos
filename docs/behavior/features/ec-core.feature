Feature: EC core driver — /services/ec

  The Embedded Controller (EC) core driver owns hardware ports 0x62/0x66,
  implements the IBF/OBF handshake protocol, and exposes a composable VFS
  service at /services/ec/.  All laptop-subsystem drivers (keyboard, battery,
  thermal) must access the EC through this service rather than touching I/O
  ports directly.

  Background:
    Given the machine is booted

  Scenario: EC core driver comes online
    Then the log should match pattern "acpi_ec.*ACPI EC service online"

  Scenario: EC service is mounted in the filesystem
    Then the path "/services/ec" should exist

  Scenario: EC service exposes status file
    Then the path "/services/ec/status" should exist

  Scenario: EC service exposes data file
    Then the path "/services/ec/data" should exist

  Scenario: EC service exposes query file
    Then the path "/services/ec/query" should exist

  Scenario: EC service exposes read and write files
    Then the path "/services/ec/read" should exist
    And the path "/services/ec/write" should exist

  Scenario: EC service exposes events stream
    Then the path "/services/ec/events" should exist

  Scenario: EC keyboard driver depends on EC service, not direct ports
    Then the log should not match pattern "ec_kbd.*ioport"
    And the log should match pattern "ec_kbd.*EC keyboard"
