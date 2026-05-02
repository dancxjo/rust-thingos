Feature: Shell Command Repetition

  Scenario: Repeat cd into https and cat example content three times
    Given the machine is booted
    When I wait for the shell prompt
    And I type "mount -t https example.com /https/ex" on the serial console
    And I wait for the serial output to contain "mounted type=https"
    And I type "cat /https/ex/@index" on the serial console
    And I type "cat /https/ex/@index" on the serial console
    And I type "cat /https/ex/@index" on the serial console
    Then the command output should contain "<html"

  Scenario: Repeat ls five times
    Given the machine is booted
    When I wait for the shell prompt
    And I type "ls" on the serial console
    And I type "ls" on the serial console
    And I type "ls" on the serial console
    And I type "ls" on the serial console
    And I type "ls" on the serial console
    Then the command output should contain "bin"
    And the latest serial output should not contain "spawn failed: connection timed out"
    And the latest serial output should not contain "VFS RPC: TIMEOUT"
