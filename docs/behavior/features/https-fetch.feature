Feature: HTTPS Fetch

  Scenario: Fetch Wikipedia Dormouse page
    Given the machine is booted
    When I wait for the shell prompt
    And I type "cat /https/en.wikipedia.org/wiki/Dormouse" on the serial console
    Then the serial output should contain "Gliridae"
    And the serial output should contain "nocturnal"
