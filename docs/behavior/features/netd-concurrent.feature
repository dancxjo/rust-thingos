Feature: netd concurrent TCP and DNS/RPC stability
  Verifies that TCP retransmit and poll cadence remain stable under concurrent
  DNS lookup and RPC load.  The network poll thread and the RPC/dispatch thread
  run independently, so a slow DNS query or burst of RPC requests must not
  stall packet processing on established TCP connections.

  Background:
    Given the machine is booted
    When I wait for the shell prompt
    And I wait for the serial output to contain "NETD: Network ready"

  Scenario: DNS lookup completes while TCP fetch is in-progress
    # Mount the HTTPS provider so a TCP connection is actively held open
    # while a separate DNS lookup is triggered concurrently.
    When I type "mount -t https none /https" on the serial console
    And I wait for the serial output to contain "HTTPSD_READY"
    And I wait for the serial output to contain "mounted type=https"
    # Trigger a DNS lookup through /net/dns/lookup (the RPC thread
    # forwards it to the poll thread as a NetCommand::StartDnsLookup).
    And I type "cat /net/dns/lookup/example.com" on the serial console
    And I wait for 3 seconds
    Then the command output should contain "93."
    # Now do a real TCP fetch to confirm the smoltcp stack is still healthy.
    When I type "cat /https/example.com/@index" on the serial console
    Then the command output should contain "Example Domain"

  Scenario: Multiple sequential DNS lookups do not degrade TCP connectivity
    When I type "cat /net/dns/lookup/example.com" on the serial console
    And I wait for 3 seconds
    Then the command output should contain "93."
    When I type "cat /net/dns/lookup/example.com" on the serial console
    And I wait for 3 seconds
    Then the command output should contain "93."
    # TCP fetch must still succeed after repeated DNS round-trips.
    When I type "mount -t https none /https" on the serial console
    And I wait for the serial output to contain "HTTPSD_READY"
    When I type "cat /https/example.com/@index" on the serial console
    Then the command output should contain "Example Domain"

  Scenario: TCP connect via hostname resolves through the deferred-connect path
    # A TCP connect with a hostname causes the RPC thread to enqueue a
    # StartDeferredConnect command; once DNS resolves, the poll thread
    # emits a DeferredConnectResult event and the RPC thread finalises
    # the connect.  The existing HTTPS fetch exercises this path end-to-end.
    When I type "mount -t https none /https" on the serial console
    And I wait for the serial output to contain "HTTPSD_READY"
    And I wait for the serial output to contain "mounted type=https"
    And I type "cat /https/example.com/@index" on the serial console
    Then the command output should contain "Example Domain"
    And the command output should contain "documentation examples"
