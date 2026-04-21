Feature: httpsd header/response cache
  Verify that httpsd surfaces the HTTP response metadata of cached fetches
  as xattrs on the /https/<host>/<path> file, as browsable files under
  /run/httpsd/cache, and (for 3xx responses) as symlinks.

  Scenario: Response headers are published as xattrs
    Given the machine is booted
    When I wait for the shell prompt
    And I type "ping -c 1 example.com" on the serial console
    And I wait for the serial output to contain "1 packets transmitted, 1 received"
    And I type "cat /https/example.com > /dev/null" on the serial console
    And I wait for the shell prompt
    And I type "attr_list /https/example.com" on the serial console
    Then the serial output should contain "user.http.status_code" within 30s
    And the serial output should contain "user.http.content_type"
    And the serial output should contain "user.http.url"
    When I type "attr_get /https/example.com user.http.content_type" on the serial console
    And I wait for the shell prompt
    Then the serial output should contain "text/html"

  Scenario: Cache mount exposes the raw response head and index
    Given the machine is booted
    When I wait for the shell prompt
    And I type "ping -c 1 example.com" on the serial console
    And I wait for the serial output to contain "1 packets transmitted, 1 received"
    And I type "cat /https/example.com > /dev/null" on the serial console
    And I wait for the shell prompt
    And I type "cat /run/httpsd/cache/index" on the serial console
    Then the serial output should contain "example.com"
    When I type "cat /run/httpsd/cache/example.com/headers" on the serial console
    Then the serial output should contain "HTTP/"
    And the serial output should contain "Content-Type:"
    When I type "cat /run/httpsd/cache/example.com/status" on the serial console
    Then the serial output should contain "200"

  Scenario: 3xx redirects surface as symlinks after first fetch
    Given the machine is booted
    When I wait for the shell prompt
    And I type "ping -c 1 example.org" on the serial console
    And I wait for the serial output to contain "1 packets transmitted, 1 received"
    And I type "attr_get /https/example.org user.http.location" on the serial console
    Then the serial output should contain "example.com" within 30s
    When I type "cat /https/example.org" on the serial console
    And I wait for the shell prompt
    Then the serial output should contain "Example Domain"
