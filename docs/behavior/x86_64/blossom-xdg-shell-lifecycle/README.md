# Feature: blossom xdg-shell lifecycle

> Last run: 2026-04-25 13:10:40

## Scenarios

| Scenario | Steps | Status | Link |
|----------|-------|--------|------|
| create xdg_surface for a wl_surface succeeds | 0/0 | ✅ | [View Details](create-xdg-surface-for-a-wl-surface-succeeds/README.md) |
| creating a second xdg_surface for the same wl_surface errors | 0/0 | ✅ | [View Details](creating-a-second-xdg-surface-for-the-same-wl-surface-errors/README.md) |
| get_toplevel assigns the toplevel role and emits initial configure | 0/0 | ✅ | [View Details](get-toplevel-assigns-the-toplevel-role-and-emits-initial-configure/README.md) |
| calling get_toplevel a second time on the same xdg_surface errors | 0/0 | ✅ | [View Details](calling-get-toplevel-a-second-time-on-the-same-xdg-surface-errors/README.md) |
| committing a buffer before ack_configure is rejected | 0/0 | ✅ | [View Details](committing-a-buffer-before-ack-configure-is-rejected/README.md) |
| ack_configure with an unknown serial is rejected | 0/0 | ✅ | [View Details](ack-configure-with-an-unknown-serial-is-rejected/README.md) |
| ack_configure with the pending serial marks the surface as configured | 0/0 | ✅ | [View Details](ack-configure-with-the-pending-serial-marks-the-surface-as-configured/README.md) |
| first valid commit after ack_configure is accepted | 0/0 | ✅ | [View Details](first-valid-commit-after-ack-configure-is-accepted/README.md) |
| set_title and set_app_id update toplevel state | 0/0 | ✅ | [View Details](set-title-and-set-app-id-update-toplevel-state/README.md) |
| ping/pong round trip is accepted | 0/0 | ✅ | [View Details](ping-pong-round-trip-is-accepted/README.md) |
| destroying xdg_toplevel then xdg_surface cleans up state | 0/0 | ✅ | [View Details](destroying-xdg-toplevel-then-xdg-surface-cleans-up-state/README.md) |
| get_popup is explicitly unsupported in v1 | 0/0 | ✅ | [View Details](get-popup-is-explicitly-unsupported-in-v1/README.md) |
| frame callback fires after surface commit | 0/0 | ✅ | [View Details](frame-callback-fires-after-surface-commit/README.md) |
