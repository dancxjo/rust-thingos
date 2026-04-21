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
    When I type "ls" on the serial console
    Then "motd" should appear at least 1 times
    When I type "ls" on the serial console
    Then "motd" should appear at least 2 times
    When I type "ls" on the serial console
    Then "motd" should appear at least 3 times
    When I type "ls" on the serial console
    Then "motd" should appear at least 4 times
    When I type "ls" on the serial console
    Then "motd" should appear at least 5 times
