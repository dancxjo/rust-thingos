Feature: blossom xdg-shell lifecycle

  Background:
    Given the bloom compositor is running with blossom support
    And a Wayland client has connected via /run/wayland-0
    And the client has bound wl_compositor and xdg_wm_base

  # ── Surface creation ────────────────────────────────────────────────────────

  Scenario: create xdg_surface for a wl_surface succeeds
    Given the client has created a wl_surface
    When the client calls xdg_wm_base.get_xdg_surface for that wl_surface
    Then a new xdg_surface object is registered successfully
    And the wl_surface now has the xdg role

  Scenario: creating a second xdg_surface for the same wl_surface errors
    Given the client has created a wl_surface
    And the client has already called get_xdg_surface for that surface
    When the client calls get_xdg_surface again for the same wl_surface
    Then the compositor sends a wl_display.error
    And the error code indicates role conflict

  # ── Toplevel assignment ──────────────────────────────────────────────────────

  Scenario: get_toplevel assigns the toplevel role and emits initial configure
    Given the client has an xdg_surface
    When the client calls xdg_surface.get_toplevel
    Then the compositor emits xdg_toplevel.configure with width=0 height=0 and empty states
    And the compositor then emits xdg_surface.configure with a fresh serial
    And the serial from xdg_surface.configure is greater than zero
    And the compositor does not reserve titlebar chrome for the toplevel
    And the compositor exposes a move handle for the toplevel

  Scenario: calling get_toplevel a second time on the same xdg_surface errors
    Given the client has an xdg_surface that already has a toplevel role
    When the client calls xdg_surface.get_toplevel again
    Then the compositor sends a wl_display.error
    And the error code indicates the surface already has a role

  # ── Configure / ack lifecycle ────────────────────────────────────────────────

  Scenario: committing a buffer before ack_configure is rejected
    Given the client has an xdg_toplevel that has received configure events
    And the client has NOT yet sent ack_configure
    When the client attaches a shm buffer and calls wl_surface.commit
    Then the compositor sends an xdg_surface error for unconfigured_buffer

  Scenario: ack_configure with an unknown serial is rejected
    Given the client has an xdg_toplevel
    When the client sends xdg_surface.ack_configure with serial 99999
    Then the compositor sends a wl_display.error
    And the error indicates an invalid configure serial

  Scenario: ack_configure with the pending serial marks the surface as configured
    Given the client has received an xdg_surface.configure with a known serial
    When the client sends xdg_surface.ack_configure with that serial
    Then no error is sent
    And the surface is now permitted to commit buffers

  # ── First commit ─────────────────────────────────────────────────────────────

  Scenario: first valid commit after ack_configure is accepted
    Given the client has ack_configured its xdg_surface
    And the client has attached a valid wl_shm buffer
    When the client calls wl_surface.commit
    Then no protocol error is sent
    And the compositor marks the surface as eligible for mapping

  @wayland-visible
  Scenario: first valid commit is visible above the compositor background
    Then the Wayland hello client should be visible
    And the compositor should render themed window bodies
    And active window chrome button glyphs should be centered inside their buttons
    And active window chrome should be rendered with beveled gradient borders
    And active window chrome should include facet frame focus accents

  @pointer-debug @wayland-visible
  Scenario: higher z-order content obscures lower window chrome
    Given the client has an xdg_toplevel
    Then the Wayland hello client should be visible
    When I drag the Clock window over the Wayland hello title bar
    Then higher z-order window content should obscure lower window chrome

  @wayland-visible @wayland-fs
  Scenario: displayed Wayland windows are visible through the session filesystem
    Then the Wayland hello client should be visible
    When I wait for the shell prompt
    And I type "find /session/wayland/windows" on the serial console
    Then the latest serial output should contain "/session/wayland/windows/index"
    When I type "cat /session/wayland/windows/index" on the serial console
    Then the latest serial output should contain "Thing-OS Wayland Lab"
    And the latest serial output should contain "Clock"
    And the latest serial output should contain "title="
    When I type "cat /session/wayland/components" on the serial console
    Then the latest serial output should contain "xdg_toplevel"
    When I type "cat /session/wayland/events/latest" on the serial console
    Then the latest serial output should contain "surface_committed"

  @pointer-debug @wayland-input
  Scenario: wl_seat delivers pointer and keyboard events to the focused client
    Given the client has an xdg_toplevel
    Then the Wayland hello client should be visible
    When I click inside the Wayland hello client and press A
    Then the Wayland hello client should receive pointer and keyboard input

  @pointer-debug @window-handle-drag
  Scenario: dragging the window handle moves a toplevel window
    Given the client has an xdg_toplevel
    Then the Wayland hello client should be visible
    When I drag the Wayland hello window handle
    Then the compositor should move the toplevel window
    And the dragged window should keep a stable cursor offset

  @pointer-debug
  Scenario: dragging the frame resizes a toplevel window
    Given the client has an xdg_toplevel
    Then the Wayland hello client should be visible
    When I drag the Wayland hello frame
    Then the compositor should resize the toplevel window

  @pointer-debug
  Scenario: clicking the maximize chrome button sends a maximized configure
    Given the client has an xdg_toplevel
    Then the Wayland hello client should be visible
    When I click the Wayland hello maximize button
    Then the compositor should send a maximized toplevel configure

  @pointer-debug
  Scenario: clicking the minimize chrome button minimizes through compositor chrome
    Given the client has an xdg_toplevel
    Then the Wayland hello client should be visible
    When I click the Wayland hello minimize button
    Then the compositor should minimize the toplevel window

  @pointer-debug
  Scenario: clicking the close chrome button sends xdg_toplevel.close
    Given the client has an xdg_toplevel
    Then the Wayland hello client should be visible
    When I click the Wayland hello close button
    Then the compositor should send xdg_toplevel.close

  # ── State updates ────────────────────────────────────────────────────────────

  Scenario: set_title and set_app_id update toplevel state
    Given the client has an xdg_toplevel
    When the client sends xdg_toplevel.set_title "My App"
    And the client sends xdg_toplevel.set_app_id "org.example.myapp"
    Then no error is sent
    And the compositor records the title as "My App"
    And the compositor records the app_id as "org.example.myapp"

  # ── Ping / pong ──────────────────────────────────────────────────────────────

  Scenario: ping/pong round trip is accepted
    Given the compositor has sent an xdg_wm_base.ping with a serial
    When the client responds with xdg_wm_base.pong using the same serial
    Then no error is sent

  # ── Destroy / cleanup ────────────────────────────────────────────────────────

  Scenario: destroying xdg_toplevel then xdg_surface cleans up state
    Given the client has an xdg_toplevel
    When the client sends xdg_toplevel.destroy
    And the client sends xdg_surface.destroy
    Then no error is sent
    And the compositor no longer tracks any state for those objects

  Scenario: crashed clients leave windows that close dramatically
    Given the client has an xdg_toplevel
    Then the Wayland hello client should be visible
    When I type "killall wayland_hello" on the serial console
    Then the serial output should contain "requested dramatic close" within 30s
    And the serial output should contain "bloom: dramatic close started surface=" within 30s
    And the serial output should contain "bloom: dramatic close finished surface=" within 30s

  # ── Popup lifecycle ──────────────────────────────────────────────────────────

  Scenario: get_popup assigns the popup role and emits configure
    Given the client has an xdg_surface
    When the client calls xdg_surface.get_popup
    Then a new xdg_popup object is registered successfully
    And the compositor emits xdg_popup.configure before xdg_surface.configure

  @pointer-debug @wayland-popup
  Scenario: clicking outside a popup causes the compositor to send popup_done
    Given the client has an xdg_toplevel
    Then the Wayland hello client should be visible
    When I click inside the Wayland hello client popup area
    Then the Wayland hello popup should be visible
    When I click outside the Wayland hello popup
    Then the compositor should send xdg_popup.popup_done
    And the Wayland hello client should log "compositor dismissed popup"

  @pointer-debug @wayland-popup
  Scenario: clicking the close chrome button on a toplevel also dismisses any open popup
    Given the client has an xdg_toplevel
    Then the Wayland hello client should be visible
    When I click inside the Wayland hello client popup area
    Then the Wayland hello popup should be visible
    When I click the Wayland hello close button
    Then the compositor should send xdg_popup.popup_done
    And the compositor should send xdg_toplevel.close

  # ── Frame callbacks ───────────────────────────────────────────────────────────

  Scenario: frame callback fires after surface commit
    Given the client has a fully configured xdg_toplevel
    And the client has called wl_surface.frame to register a callback
    When the client commits the surface with a valid shm buffer
    And the compositor presents the next frame
    Then the client receives wl_callback.done for that callback object

  # ── wp_presentation (Presentation Time) ─────────────────────────────────────

  Scenario: registry advertises wp_presentation
    Given a Wayland client has connected via /run/wayland-0
    When the client requests the wl_registry global list
    Then wl_registry advertises wp_presentation version 1

  Scenario: wp_presentation_feedback receives presented after commit
    Given the client has a fully configured xdg_toplevel
    And the client has requested wp_presentation.feedback for the surface
    When the client commits the surface with a valid shm buffer
    And the compositor presents the next frame
    Then the client receives wp_presentation_feedback.presented for that feedback object
