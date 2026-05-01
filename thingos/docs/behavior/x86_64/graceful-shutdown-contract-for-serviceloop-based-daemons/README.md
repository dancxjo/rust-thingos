# Feature: Graceful shutdown contract for ServiceLoop-based daemons

> Last run: 2026-05-01 15:28:03

## Scenarios

| Scenario | Steps | Status | Link |
|----------|-------|--------|------|
| ServiceProviderLoop.shutdown_sequence unmounts registered paths | 0/0 | ⏭️ | [View Details](serviceproviderloop-shutdown-sequence-unmounts-registered-paths/README.md) |
| ServiceProviderLoop.shutdown_sequence is idempotent | 0/0 | ⏭️ | [View Details](serviceproviderloop-shutdown-sequence-is-idempotent/README.md) |
| ServiceProviderLoop.run_until_shutdown triggers on InboxClosed | 0/0 | ⏭️ | [View Details](serviceproviderloop-run-until-shutdown-triggers-on-inboxclosed/README.md) |
| Restarting virtio_netd does not leave stale mounts | 0/0 | ⏭️ | [View Details](restarting-virtio-netd-does-not-leave-stale-mounts/README.md) |
| virtio_netd logs shutdown progression | 0/0 | ⏭️ | [View Details](virtio-netd-logs-shutdown-progression/README.md) |
| virtio_netd provider thread stops after shutdown | 0/0 | ⏭️ | [View Details](virtio-netd-provider-thread-stops-after-shutdown/README.md) |
