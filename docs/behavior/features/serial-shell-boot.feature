Feature: Serial Shell Boot

  @smoke
  Scenario: Boot reaches an interactive serial shell before background bring-up continues
    Given the machine is booted
    When I wait for the shell prompt
    And I type "echo shell-ready" on the serial console
    Then the command output should contain "shell-ready"

  @smoke
  Scenario: Serial shell remains responsive after supervisor startup idles
    Given the machine is booted
    When I wait for the shell prompt
    And I wait for the serial output to contain "SPROUT: Continuing supervisor startup"
    Then the serial shell is still responsive
