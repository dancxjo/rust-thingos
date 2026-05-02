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

  @relative-path
  Scenario: Relative executable paths run from the current directory
    Given the machine is booted
    When I wait for the shell prompt
    And I type "cd /bin" on the serial console
    And I type "./echo relative-path-ok" on the serial console
    Then the command output should strictly be "relative-path-ok"
    And the command output should not contain "spawn failed"

  @completion
  Scenario: Tab completes a command name before execution
    Given the machine is booted
    When I wait for the shell prompt
    And I type "cd /bin" on the serial console
    And I use serial Tab completion from "ec" to run "echo tab-completion-ok"
    Then the latest serial output should contain "tab-completion-ok"
    And the latest serial output should not contain "spawn failed"
