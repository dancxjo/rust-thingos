# Feature: ServiceLoop — inbox-backed control plane for ThingOS services

> Last run: 2026-04-23 19:52:41

## Scenarios

| Scenario | Steps | Status | Link |
|----------|-------|--------|------|
| inbox messages are dispatched as control-plane events | 0/0 | ✅ | [View Details](inbox-messages-are-dispatched-as-control-plane-events/README.md) |
| when the inbox and a pipe both fire, the inbox wins | 0/0 | ✅ | [View Details](when-the-inbox-and-a-pipe-both-fire-the-inbox-wins/README.md) |
| secondary FD readiness still dispatches when the inbox is idle | 0/0 | ✅ | [View Details](secondary-fd-readiness-still-dispatches-when-the-inbox-is-idle/README.md) |
| an IRQ-registered service wakes from add_irq | 0/0 | ✅ | [View Details](an-irq-registered-service-wakes-from-add-irq/README.md) |
| closing the inbox surfaces a clean shutdown event | 0/0 | ✅ | [View Details](closing-the-inbox-surfaces-a-clean-shutdown-event/README.md) |
| drain_inbox batches multiple messages in one wake | 0/0 | ✅ | [View Details](drain-inbox-batches-multiple-messages-in-one-wake/README.md) |
| removing a secondary token does not detach the inbox | 0/0 | ✅ | [View Details](removing-a-secondary-token-does-not-detach-the-inbox/README.md) |
