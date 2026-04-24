Feature: httpsd concurrent streaming
  Verify that multiple open handles stream response bodies concurrently
  without head-of-line blocking.  Each handle runs in its own worker thread
  so a slow or large response on one handle does not stall reads on
  unrelated handles.

  Scenario: Two simultaneous large fetches complete concurrently
    Given the machine is booted
    When I wait for the shell prompt
    And I type "ping -c 1 example.com" on the serial console
    And I wait for the serial output to contain "1 packets transmitted, 1 received"
    And I type "mount -t https none /https" on the serial console
    And I wait for the serial output to contain "HTTPSD_READY"
    And I type "cat /https/example.com/@index & cat /https/www.iana.org/@index & wait" on the serial console
    And I wait for the shell prompt
    Then the command output should contain "Example Domain"
    And the command output should contain "Internet Assigned Numbers Authority"

  Scenario: Close of a streaming handle cancels its worker cleanly
    Given the machine is booted
    When I wait for the shell prompt
    And I type "ping -c 1 example.com" on the serial console
    And I wait for the serial output to contain "1 packets transmitted, 1 received"
    And I type "mount -t https none /https" on the serial console
    And I wait for the serial output to contain "HTTPSD_READY"
    And I type "cat /https/example.com/@index > /dev/null" on the serial console
    And I wait for the shell prompt
    And I type "cat /https/example.com/@index" on the serial console
    And I wait for the serial output to contain "Example Domain"
    And I wait for the shell prompt
    Then the serial output should not contain "PANIC"
