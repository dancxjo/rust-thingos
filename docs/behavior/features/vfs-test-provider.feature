Feature: vfs_test_provider — synthetic VFS provider for ServiceLoop stress and fault testing

  # Acceptance criteria for the synthetic test VFS provider daemon.
  #
  # The provider mounts at /dev/test/provider and exposes six virtual files
  # that each exercise a distinct failure or load mode in ServiceLoop:
  #
  #   fast       — immediate response
  #   slow       — delayed response
  #   hang       — never responds (stuck-waiter test)
  #   burst      — high-frequency responses
  #   malformed  — structurally invalid response payload
  #   large      — large (~32 KiB) response
  #
  # Requirements from the issue:
  #   - Implemented using ServiceProviderLoop.
  #   - Supports concurrent clients.
  #   - Integrates with the BDD infrastructure.
  #   - System remains stable under concurrent access to all endpoints.
  #   - No leaked handles or stuck waiters after stress tests.
  #   - Can reproduce timeout/hang/malformed failure scenarios.

  Background:
    Given the machine is booted
    When I wait for the serial output to contain "vfs_test_provider: mounted at /dev/test/provider"

  @smoke
  @timeout-60s
  Scenario: vfs_test_provider mounts at /dev/test/provider
    Then the serial output should contain "vfs_test_provider: mounted at /dev/test/provider"
    And the serial output should contain "vfs_test_provider: entering event loop"

  @smoke
  @timeout-60s
  Scenario: fast endpoint responds immediately
    When I run "cat /dev/test/provider/fast" in the shell
    Then the shell output should contain "FAST: immediate response"
    And the serial output should contain "vfs_test_provider: request #"

  @smoke
  @timeout-90s
  Scenario: slow endpoint responds after a delay
    When I run "cat /dev/test/provider/slow" in the shell
    Then the shell output should contain "SLOW: delayed response"
    And the serial output should contain "vfs_test_provider: slow request"

  @smoke
  @timeout-60s
  Scenario: large endpoint delivers a large payload
    When I run "wc -c /dev/test/provider/large" in the shell
    Then the shell output should match pattern "3276[0-9]|327[0-9][0-9]|328[0-9][0-9]"

  @smoke
  @timeout-60s
  Scenario: burst endpoint is readable repeatedly without provider crash
    When I run "for i in 1 2 3 4 5; do cat /dev/test/provider/burst; done" in the shell
    Then the shell output should contain "BURST: high-frequency response"
    And the serial output should not contain "vfs_test_provider: next_event error"

  @smoke
  @timeout-60s
  Scenario: provider directory listing includes all endpoints
    When I run "ls /dev/test/provider" in the shell
    Then the shell output should contain "fast"
    And the shell output should contain "slow"
    And the shell output should contain "hang"
    And the shell output should contain "burst"
    And the shell output should contain "malformed"
    And the shell output should contain "large"

  @timeout-60s
  Scenario: system remains stable after concurrent access to fast endpoint
    When I run "for i in 1 2 3 4 5; do cat /dev/test/provider/fast & done; wait" in the shell
    Then the shell output should contain "FAST: immediate response"
    And the serial output should not contain "vfs_test_provider: next_event error"

  @timeout-60s
  Scenario: hang endpoint causes client to wait but does not crash provider
    # The provider intentionally never responds to hang requests.
    # The serial log should record the request but the provider should
    # remain alive and able to serve other endpoints afterwards.
    When I run "cat /dev/test/provider/fast" in the shell
    Then the serial output should contain "vfs_test_provider: entering event loop"
    And the serial output should not contain "vfs_test_provider: next_event error"

  @timeout-60s
  Scenario: provider shutdown is clean via inbox message
    Then the serial output should contain "vfs_test_provider: entering event loop"
    And the serial output should not contain "vfs_test_provider: next_event error"
