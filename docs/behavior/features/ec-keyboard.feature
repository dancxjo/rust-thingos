Feature: Embedded controller keyboard input

  Embedded-controller keyboards are exposed as a userland input driver and
  publish normalized key events through Bristle when classic ACPI EC ports are
  present.

  Background:
    Given the machine is booted

  Scenario: ec_kbd starts and degrades cleanly when no EC controller is present
    Then the log should match pattern "ec_kbd.*EC keyboard controller unavailable|ec_kbd.*Using (SCI-assisted|EC keyboard polling) .*loop"
