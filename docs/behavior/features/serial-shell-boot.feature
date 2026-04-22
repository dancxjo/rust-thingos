Feature: Serial Shell Boot

  @smoke
  Scenario: Boot reaches an interactive serial shell before background bring-up continues
    Given the machine is booted
    When I wait for the shell prompt
    And I type "echo shell-ready" on the serial console
    Then the serial output should contain "shell-ready"
