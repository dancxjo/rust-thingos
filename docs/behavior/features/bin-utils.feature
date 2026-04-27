Feature: Bin Utils Traditional Functionality

  Scenario: echo, pipe, and wc work together
    Given the machine is booted
    When I wait for the shell prompt
    And I type "echo hello world | wc -w" on the serial console
    Then the command output should strictly be "2"

  Scenario: grep filters piped input correctly
    Given the machine is booted
    When I wait for the shell prompt
    And I type "echo hello | grep hello | wc -l" on the serial console
    Then the command output should strictly be "1"

  Scenario: 3-command pipeline with cat as middle filter
    Given the machine is booted
    When I wait for the shell prompt
    And I type "printf 'a\nb\nc\n' | cat | wc -l" on the serial console
    Then the command output should strictly be "3"

  Scenario: POSIX behavior - grep -v for inverted match
    Given the machine is booted
    When I wait for the shell prompt
    And I type "echo -e 'a\nb\nc' | grep -v b" on the serial console
    Then the command output should contain "a"
    And the command output should contain "c"
    And the command output should not contain "b"

  Scenario: POSIX behavior - cat -n for line numbers
    Given the machine is booted
    When I wait for the shell prompt
    And I type "echo hello | cat -n" on the serial console
    Then the command output should contain "1  hello"

  Scenario: POSIX behavior - head -n and tail -n
    Given the machine is booted
    When I wait for the shell prompt
    And I type "echo -e '1\n2\n3' | head -n 2 | tail -n 1" on the serial console
    Then the command output should strictly be "2"

  Scenario: user-visible utility text defaults to Esperanto
    Given the machine is booted
    When I wait for the shell prompt
    And I type "file /does-not-exist" on the serial console
    Then the command output should contain "/does-not-exist: ne povas malfermi"
