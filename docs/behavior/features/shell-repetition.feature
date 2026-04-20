Feature: Shell Command Repetition

  Scenario: Repeat cd into https and cat example content three times
    Given the machine is booted
    When I wait for the shell prompt
    And I type "cd /https" on the serial console
    And I type "cat www.example.com" on the serial console
    And I type "cd /https" on the serial console
    And I type "cat www.example.com" on the serial console
    And I type "cd /https" on the serial console
    And I type "cat www.example.com" on the serial console

  Scenario: Repeat ls five times
    Given the machine is booted
    When I wait for the shell prompt
    And I type "ls" on the serial console
    And I type "ls" on the serial console
    And I type "ls" on the serial console
    And I type "ls" on the serial console
    And I type "ls" on the serial console
