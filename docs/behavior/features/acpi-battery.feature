Feature: ACPI battery and AC adapter service — /sys/power

  The ACPI battery service polls the Embedded Controller (EC) for battery
  state and AC adapter presence, and exposes normalised power-supply data at
  /sys/power/.

  Background:
    Given the machine is booted

  Scenario: ACPI battery service comes online
    Then the log should match pattern "acpi_battery.*ACPI battery service online"

  Scenario: Battery service is mounted in the filesystem
    Then the path "/sys/power" should exist

  Scenario: Battery service exposes battery0 directory
    Then the path "/sys/power/battery0" should exist

  Scenario: Battery service exposes battery status file
    Then the path "/sys/power/battery0/status" should exist

  Scenario: Battery service exposes battery percent file
    Then the path "/sys/power/battery0/percent" should exist

  Scenario: Battery service exposes battery rate file
    Then the path "/sys/power/battery0/rate" should exist

  Scenario: Battery service exposes battery capacity file
    Then the path "/sys/power/battery0/capacity" should exist

  Scenario: Battery service exposes AC adapter state file
    Then the path "/sys/power/ac" should exist

  Scenario: Battery status is readable and returns a known state
    When I read the file "/sys/power/battery0/status"
    Then the output should match pattern "Charging|Discharging|Full|Unknown"

  Scenario: Battery percent is readable
    When I read the file "/sys/power/battery0/percent"
    Then the output should match pattern "[0-9]+|Unknown"

  Scenario: AC adapter state is readable
    When I read the file "/sys/power/ac"
    Then the output should match pattern "online|offline"
