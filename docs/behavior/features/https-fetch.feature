Feature: HTTPS Fetch

  Scenario: Fetch Wikipedia Dormouse page
    Given the machine is booted
    When I wait for the shell prompt
    And I type "ping -c 1 en.wikipedia.org" on the serial console
    And I wait for the serial output to contain "1 packets transmitted, 1 received"
    And I type "cat /https/en.wikipedia.org/wiki/Dormouse" on the serial console
    Then the serial output should contain "Dormouse"
    And the serial output should contain "Gliridae"
    And the serial output should contain "nocturnal"
