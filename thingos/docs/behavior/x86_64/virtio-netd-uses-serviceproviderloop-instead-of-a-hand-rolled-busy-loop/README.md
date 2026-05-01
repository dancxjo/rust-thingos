# Feature: virtio_netd uses ServiceProviderLoop instead of a hand-rolled busy loop

> Last run: 2026-05-01 15:28:03

## Scenarios

| Scenario | Steps | Status | Link |
|----------|-------|--------|------|
| virtio_netd mounts successfully at /dev/net/virtio0 | 0/0 | ⏭️ | [View Details](virtio-netd-mounts-successfully-at-dev-net-virtio0/README.md) |
| virtio_netd logs MAC address and initial link state during init | 0/0 | ⏭️ | [View Details](virtio-netd-logs-mac-address-and-initial-link-state-during-init/README.md) |
| netd can read MAC address from /dev/net/virtio0 | 0/0 | ⏭️ | [View Details](netd-can-read-mac-address-from-dev-net-virtio0/README.md) |
| virtio_netd does not emit a poll storm at idle | 0/0 | ⏭️ | [View Details](virtio-netd-does-not-emit-a-poll-storm-at-idle/README.md) |
