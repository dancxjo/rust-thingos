Feature: HTTPS Fetch

  Scenario: Fetch Example Domain page
    Given the machine is booted
    When I wait for the shell prompt
    And I type "ping -c 1 example.com" on the serial console
    And I wait for the serial output to contain "1 packets transmitted, 1 received"
    And I type "cat /https/example.com" on the serial console
    Then the serial output should contain "Example Domain"
    And the serial output should contain "documentation examples"
    And the serial output should contain "permission"
