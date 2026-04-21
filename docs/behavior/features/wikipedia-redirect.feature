Feature: Wikipedia Redirect
  Scenario: Fetch Wikipedia /wiki path should follow redirect
    Given the machine is booted
    When I wait for the shell prompt
    And I type "mount -t https en.wikipedia.org /https/wp" on the serial console
    And I wait for the serial output to contain "mounted type=https"
    And I type "cat /https/wp/wiki/@index" on the serial console
    Then the serial output should contain "Main Page"
