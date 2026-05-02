Feature: Shell text pipelines
  The bundled command-line tools should compose through pipes for common text
  workflows instead of only succeeding as isolated binaries.

  Scenario: Basic filters compose in a pipeline
    Given the machine is booted
    When I wait for the shell prompt
    And I type "echo hello world | wc -w" on the serial console
    Then the command output should strictly be "2"
    When I type "echo hello | grep hello | wc -l" on the serial console
    Then the command output should strictly be "1"
    When I type "printf 'a\nb\nc\n' | cat | wc -l" on the serial console
    Then the command output should strictly be "3"

  Scenario: POSIX-style filter options work inside pipelines
    Given the machine is booted
    When I wait for the shell prompt
    And I type "echo -e 'a\nb\nc' | grep -v b" on the serial console
    Then the command output should contain "a"
    And the command output should contain "c"
    And the command output should not contain "b"
    When I type "echo hello | cat -n" on the serial console
    Then the command output should contain "1  hello"
    When I type "echo -e '1\n2\n3' | head -n 2 | tail -n 1" on the serial console
    Then the command output should strictly be "2"

  Scenario: Tree displays command directories
    Given the machine is booted
    When I wait for the shell prompt
    And I type "tree /bin" on the serial console
    Then the command output should contain "/bin"
    And the command output should contain "tree"
    And the command output should contain "|--"

  Scenario: Less can page text received from a pipeline
    Given the machine is booted
    When I wait for the shell prompt
    And I start "printf 'alpha\nbeta\n' | less" on the serial console
    Then the command output should contain "alpha"
    And the command output should contain "beta"
    When I send "q" to the serial console
    Then the shell prompt should return
