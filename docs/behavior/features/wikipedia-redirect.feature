Feature: Wikipedia Redirect
  Scenario: Fetch Wikipedia /wiki path should follow redirect
    Given the machine is booted
    When I wait for the shell prompt
    And I type "cat /https/en.wikipedia.org/wiki/" on the serial console
    Then the serial output should contain "Main Page"
