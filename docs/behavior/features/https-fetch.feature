Feature: HTTPS Fetch

  Scenario: Fetch Example Domain page
    Given the machine is booted
    When I wait for the shell prompt
    And I wait for the serial output to contain "NETD: Network ready"
    And I type "ping -c 1 example.com" on the serial console
    And I wait for the serial output to contain "1 packets transmitted"
    And I type "mount -t https none /https" on the serial console
    And I wait for the serial output to contain "HTTPSD_READY"
    And I wait for the serial output to contain "mounted type=https"
    And I type "cat /https/example.com/@index" on the serial console
    Then the command output should contain "Example Domain"
    And the command output should contain "documentation examples"
    And the command output should contain "permission"
