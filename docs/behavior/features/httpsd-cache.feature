Feature: httpsd header/response cache
  Verify that httpsd surfaces the HTTP response metadata of cached fetches
  as xattrs on the /https/<host>/<path> file, as browsable files under
  /run/httpsd/cache, and (for 3xx responses) as symlinks.

  Scenario: Response headers are published as xattrs
    Given the machine is booted
    When I wait for the shell prompt
    And I type "ping -c 1 en.wikipedia.org" on the serial console
    And I wait for the serial output to contain "1 packets transmitted, 1 received"
    And I type "cat /https/en.wikipedia.org/wiki/Dormouse > /dev/null" on the serial console
    And I wait for 2 seconds
    And I type "attr_list /https/en.wikipedia.org/wiki/Dormouse" on the serial console
    Then the serial output should contain "user.http.status_code"
    And the serial output should contain "user.http.content_type"
    And the serial output should contain "user.http.url"
    When I type "attr_get /https/en.wikipedia.org/wiki/Dormouse user.http.content_type" on the serial console
    Then the serial output should contain "text/html"

  Scenario: Cache mount exposes the raw response head and index
    Given the machine is booted
    When I wait for the shell prompt
    And I type "ping -c 1 en.wikipedia.org" on the serial console
    And I wait for the serial output to contain "1 packets transmitted, 1 received"
    And I type "cat /https/en.wikipedia.org/wiki/Dormouse > /dev/null" on the serial console
    And I wait for 2 seconds
    And I type "cat /run/httpsd/cache/index" on the serial console
    Then the serial output should contain "en.wikipedia.org"
    When I type "cat /run/httpsd/cache/en.wikipedia.org/wiki/Dormouse/headers" on the serial console
    Then the serial output should contain "HTTP/"
    And the serial output should contain "Content-Type:"
    When I type "cat /run/httpsd/cache/en.wikipedia.org/wiki/Dormouse/status" on the serial console
    Then the serial output should contain "200"

  Scenario: 3xx redirects surface as symlinks after first fetch
    Given the machine is booted
    When I wait for the shell prompt
    And I type "ping -c 1 en.wikipedia.org" on the serial console
    And I wait for the serial output to contain "1 packets transmitted, 1 received"
    # First fetch populates the cache with the 301 entry so the kernel can
    # follow the symlink on subsequent path resolutions.
    And I type "cat /https/en.wikipedia.org/wiki/ > /dev/null" on the serial console
    And I wait for 2 seconds
    And I type "attr_get /https/en.wikipedia.org/wiki/ user.http.location" on the serial console
    Then the serial output should contain "Main_Page"
    When I type "cat /https/en.wikipedia.org/wiki/" on the serial console
    Then the serial output should contain "Main Page"
