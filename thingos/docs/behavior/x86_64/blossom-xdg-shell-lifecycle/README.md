# Feature: blossom xdg-shell lifecycle

> Last run: 2026-05-01 15:28:03

## Scenarios

| Scenario | Steps | Status | Link |
|----------|-------|--------|------|
| create xdg_surface for a wl_surface succeeds | 0/0 | ⏭️ | [View Details](create-xdg-surface-for-a-wl-surface-succeeds/README.md) |
| creating a second xdg_surface for the same wl_surface errors | 0/0 | ⏭️ | [View Details](creating-a-second-xdg-surface-for-the-same-wl-surface-errors/README.md) |
| get_toplevel assigns the toplevel role and emits initial configure | 0/0 | ⏭️ | [View Details](get-toplevel-assigns-the-toplevel-role-and-emits-initial-configure/README.md) |
| calling get_toplevel a second time on the same xdg_surface errors | 0/0 | ⏭️ | [View Details](calling-get-toplevel-a-second-time-on-the-same-xdg-surface-errors/README.md) |
| committing a buffer before ack_configure is rejected | 0/0 | ⏭️ | [View Details](committing-a-buffer-before-ack-configure-is-rejected/README.md) |
| ack_configure with an unknown serial is rejected | 0/0 | ⏭️ | [View Details](ack-configure-with-an-unknown-serial-is-rejected/README.md) |
| ack_configure with the pending serial marks the surface as configured | 0/0 | ⏭️ | [View Details](ack-configure-with-the-pending-serial-marks-the-surface-as-configured/README.md) |
| first valid commit after ack_configure is accepted | 0/0 | ⏭️ | [View Details](first-valid-commit-after-ack-configure-is-accepted/README.md) |
| first valid commit is visible above the compositor background | 0/0 | ⏭️ | [View Details](first-valid-commit-is-visible-above-the-compositor-background/README.md) |
| higher z-order content obscures lower window chrome | 0/0 | ⏭️ | [View Details](higher-z-order-content-obscures-lower-window-chrome/README.md) |
| displayed Wayland windows are visible through the session filesystem | 0/0 | ⏭️ | [View Details](displayed-wayland-windows-are-visible-through-the-session-filesystem/README.md) |
| wl_seat delivers pointer and keyboard events to the focused client | 0/0 | ⏭️ | [View Details](wl-seat-delivers-pointer-and-keyboard-events-to-the-focused-client/README.md) |
| dragging the title bar moves a toplevel window | 0/0 | ⏭️ | [View Details](dragging-the-title-bar-moves-a-toplevel-window/README.md) |
| dragging the frame resizes a toplevel window | 0/0 | ⏭️ | [View Details](dragging-the-frame-resizes-a-toplevel-window/README.md) |
| clicking the maximize chrome button sends a maximized configure | 0/0 | ⏭️ | [View Details](clicking-the-maximize-chrome-button-sends-a-maximized-configure/README.md) |
| clicking the minimize chrome button minimizes through compositor chrome | 0/0 | ⏭️ | [View Details](clicking-the-minimize-chrome-button-minimizes-through-compositor-chrome/README.md) |
| clicking the close chrome button sends xdg_toplevel.close | 0/0 | ⏭️ | [View Details](clicking-the-close-chrome-button-sends-xdg-toplevel-close/README.md) |
| set_title and set_app_id update toplevel state | 0/0 | ⏭️ | [View Details](set-title-and-set-app-id-update-toplevel-state/README.md) |
| ping/pong round trip is accepted | 0/0 | ⏭️ | [View Details](ping-pong-round-trip-is-accepted/README.md) |
| destroying xdg_toplevel then xdg_surface cleans up state | 0/0 | ⏭️ | [View Details](destroying-xdg-toplevel-then-xdg-surface-cleans-up-state/README.md) |
| get_popup assigns the popup role and emits configure | 0/0 | ⏭️ | [View Details](get-popup-assigns-the-popup-role-and-emits-configure/README.md) |
| clicking outside a popup causes the compositor to send popup_done | 0/0 | ⏭️ | [View Details](clicking-outside-a-popup-causes-the-compositor-to-send-popup-done/README.md) |
| clicking the close chrome button on a toplevel also dismisses any open popup | 0/0 | ⏭️ | [View Details](clicking-the-close-chrome-button-on-a-toplevel-also-dismisses-any-open-popup/README.md) |
| frame callback fires after surface commit | 0/0 | ⏭️ | [View Details](frame-callback-fires-after-surface-commit/README.md) |
| registry advertises wp_presentation | 0/0 | ⏭️ | [View Details](registry-advertises-wp-presentation/README.md) |
| wp_presentation_feedback receives presented after commit | 0/0 | ⏭️ | [View Details](wp-presentation-feedback-receives-presented-after-commit/README.md) |
