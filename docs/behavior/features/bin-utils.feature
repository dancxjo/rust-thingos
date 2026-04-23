Feature: Bin Utils Traditional Functionality

  Scenario: echo, pipe, and wc work together
    Given the machine is booted
    When I wait for the shell prompt
    And I type "echo hello world | wc -w" on the serial console
    Then the command output should strictly be "2"

  Scenario: ls, grep, and wc pipe line counting
    Given the machine is booted
    When I wait for the shell prompt
    And I type "ls /bin | grep sh | wc -l" on the serial console
    Then the command output should strictly be "2"

  Scenario: POSIX behavior - grep -v for inverted match (expected to fail if not implemented)
    Given the machine is booted
    When I wait for the shell prompt
    And I type "echo -e 'a\nb\nc' | grep -v b" on the serial console
    Then the serial output should contain "a" within 5s
    And the serial output should contain "c" within 5s
    And the latest serial output should not contain "b"

  Scenario: POSIX behavior - cat -n for line numbers (expected to fail if not implemented)
    Given the machine is booted
    When I wait for the shell prompt
    And I type "echo hello | cat -n" on the serial console
    Then the command output should contain "1  hello"

  Scenario: POSIX behavior - head -n and tail -n (expected to fail if not fully POSIX)
    Given the machine is booted
    When I wait for the shell prompt
    And I type "echo -e '1\n2\n3' | head -n 2 | tail -n 1" on the serial console
    Then the command output should strictly be "2"
