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

  @pointer-debug
  Scenario: dragging the title bar moves a toplevel window
    Given the client has an xdg_toplevel
    Then the Wayland hello client should be visible
    When I drag the Wayland hello title bar
    Then the compositor should move the toplevel window

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

  # ── Unsupported paths ────────────────────────────────────────────────────────

  Scenario: get_popup is explicitly unsupported in v1
    Given the client has an xdg_surface
    When the client calls xdg_surface.get_popup
    Then the compositor sends a wl_display.error
    And the error message indicates popups are not supported in v1

  # ── Frame callbacks ───────────────────────────────────────────────────────────

  Scenario: frame callback fires after surface commit
    Given the client has a fully configured xdg_toplevel
    And the client has called wl_surface.frame to register a callback
    When the client commits the surface with a valid shm buffer
    And the compositor presents the next frame
    Then the client receives wl_callback.done for that callback object
