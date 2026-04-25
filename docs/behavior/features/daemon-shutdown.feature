Feature: Graceful shutdown contract for ServiceLoop-based daemons
  # Executable spec for the canonical daemon shutdown sequence described in
  # the issue "Define and implement graceful shutdown contract for ServiceLoop
  # daemons".
  #
  # The contract (in order):
  #   1. Stop accepting new work
  #   2. Unmount any VFS provider paths
  #   3. Close ports/handles
  #   4. Flush logs or buffered state
  #   5. Exit cleanly with status
  #
  # ServiceProviderLoop.shutdown_sequence() and
  # ServiceProviderLoop.run_until_shutdown() implement this sequence.
  # virtio_netd is the reference implementation.

  Background:
    Given the machine is booted

  Scenario: ServiceProviderLoop.shutdown_sequence unmounts registered paths
    Given a VFS provider is mounted at "/run/test_provider"
    And a ServiceProviderLoop has registered "/run/test_provider" as a mount path
    When shutdown_sequence is called on the ServiceProviderLoop
    Then "/run/test_provider" should no longer be accessible in the VFS
    And the shutdown log should contain "initiating graceful shutdown"
    And the shutdown log should contain "unmounted /run/test_provider"
    And the shutdown log should contain "shutdown complete"

  Scenario: ServiceProviderLoop.shutdown_sequence is idempotent
    Given a VFS provider is mounted at "/run/test_provider2"
    And a ServiceProviderLoop has registered "/run/test_provider2" as a mount path
    When shutdown_sequence is called twice on the ServiceProviderLoop
    Then no errors or panics occur
    And the unmount log line appears exactly once

  Scenario: ServiceProviderLoop.run_until_shutdown triggers on InboxClosed
    Given a ServiceProviderLoop is running with a provider mounted at "/run/test_provider3"
    When the inbox of the provider daemon is closed
    Then run_until_shutdown should call shutdown_sequence
    And "/run/test_provider3" should no longer be accessible in the VFS
    And the daemon should exit cleanly

  Scenario: Restarting virtio_netd does not leave stale mounts
    # Key acceptance criterion from the issue.
    Given virtio_netd is running and has mounted its provider at "/dev/net/virtio0"
    When the kernel closes the virtio_netd inbox (simulating restart)
    And virtio_netd is started again
    Then the new virtio_netd instance should mount successfully at "/dev/net/virtio0"
    And no "ghost mount" error should appear in the kernel log

  Scenario: virtio_netd logs shutdown progression
    Given virtio_netd is running and has mounted its provider at "/dev/net/virtio0"
    When the kernel closes the virtio_netd inbox
    Then the kernel log should contain "VIRTIO_NETD: shutdown initiated"
    And the kernel log should contain "VIRTIO_NETD: unmounting /dev/net/virtio0"
    And the kernel log should contain "VIRTIO_NETD: shutdown complete"

  Scenario: virtio_netd provider thread stops after shutdown
    Given virtio_netd is running
    When shutdown is triggered
    Then the provider thread should stop polling within 100ms
    And no further VFS RPC requests should be dispatched after the mount is removed
