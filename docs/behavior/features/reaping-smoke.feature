Feature: Process Reaping Smoke Test

  @smoke
  Scenario: Launch ps and verify reaping
    Given the machine is booted
    When I wait for the shell prompt
    And I type "ps" on the serial console
    Then the serial output should contain "PID"
    And the serial output should contain "sh"
    And the serial output should contain "ps"
