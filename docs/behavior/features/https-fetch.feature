Feature: HTTPS Fetch

  Scenario: Fetch Example Domain page
    Given the machine is booted
    When I wait for the shell prompt
    And I wait for the serial output to contain "NETD: Network ready"
    And I type "ping -c 1 example.com" on the serial console
    And I wait for the serial output to contain "1 packets transmitted"
    And I type "mount -t https example.com /https/ex" on the serial console
    And I wait for the serial output to contain "HTTPSD_READY"
    And I wait for the serial output to contain "mounted type=https"
    And I type "cat /https/ex/@index" on the serial console
    Then the serial output should contain "Example Domain"
    And the serial output should contain "documentation examples"
    And the serial output should contain "permission"
