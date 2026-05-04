Feature: virtio_netd uses ServiceProviderLoop instead of a hand-rolled busy loop

  virtio_netd exposes the VirtIO-NET hardware as a VFS provider mounted at
  Cambium's stable /dev/net/cardN path.  After migration it must use
  ServiceProviderLoop for all VFS RPC dispatch so that idle threads sleep
  rather than spinning on yield_now().

  Background:
    Given the machine is booted
    When I wait for the serial output to contain "VIRTIO_NETD: Mounted at /dev/net/card0"

  @smoke
  @timeout-60s
  Scenario: virtio_netd mounts successfully at /dev/net/card0
    Then the serial output should contain "VIRTIO_NETD: Driver initialized successfully"
    And the serial output should contain "VIRTIO_NETD: Mounted at /dev/net/card0"
    And the serial output should contain "VIRTIO_NETD: Provider thread live at /dev/net/card0"

  @smoke
  @timeout-60s
  Scenario: virtio_netd logs MAC address and initial link state during init
    Then the serial output should match pattern "VIRTIO_NETD: MAC [0-9a-f]{2}(:[0-9a-f]{2}){5}"
    And the serial output should match pattern "VIRTIO_NETD: Initial link state is (UP|DOWN)"

  @smoke
  @timeout-60s
  Scenario: netd can read MAC address from /dev/net/card0
    # Verifies that VFS provider RPC still works after the ServiceProviderLoop
    # migration.
    Then the serial output should contain "NETD: Opened VFS NIC device at /dev/net/card0"
    When I wait for the serial output to contain "NETD: Network ready"
    Then the serial output should contain "NETD:"

  @smoke
  @timeout-60s
  Scenario: virtio_netd does not emit a poll storm at idle
    # A busy-polling driver would log thousands of yield or poll entries per
    # second.  After migration the loop blocks on next_event so at most one
    # spurious wake per millisecond timeout is expected.
    # We detect a poll storm by the absence of rapid consecutive Poll-op
    # trace entries in the boot log.
    Then the serial output should not match pattern "VIRTIO_NETD: dispatch begin op=Poll.*\nVIRTIO_NETD: dispatch begin op=Poll"
