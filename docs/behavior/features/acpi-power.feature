Feature: ACPI power and button handling service — /run/power

  The ACPI power service handles power button and sleep button events via the
  PM1 fixed-event registers, tracks lid open/close state via the EC query
  event stream, and exposes normalised events at /run/power/.

  Background:
    Given the machine is booted

  Scenario: ACPI power service comes online
    Then the log should match pattern "acpi_power.*ACPI power service online"

  Scenario: Power service is mounted in the filesystem
    Then the path "/run/power" should exist

  Scenario: Power service exposes events stream
    Then the path "/run/power/events" should exist

  Scenario: Power service exposes lid state
    Then the path "/run/power/lid" should exist

  Scenario: Power service exposes action file
    Then the path "/run/power/action" should exist

  Scenario: Lid state reads as open on a freshly booted system
    When I read the file "/run/power/lid"
    Then the output should contain "open"

  Scenario: Events stream returns EAGAIN when no events are pending
    When I read the file "/run/power/events"
    Then the read returns EAGAIN or an empty result
