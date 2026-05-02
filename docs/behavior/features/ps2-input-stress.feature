Feature: PS/2 input stress diagnostics

  The PS/2 input path should leave enough serial evidence to identify whether
  a freeze occurred in the hardware IRQ handler, the scancode FIFO, Bristle
  dispatch, or Bloom event handling.

  @timeout=180s
  Scenario: High-frequency PS/2 input emits end-to-end diagnostics
    Given the machine is booted
    Then the serial output should contain "bloom: registered bristle pointer sink" within 60s
    And the serial output should contain "ps2_kbd: bristle pid=" within 60s
    And the serial output should contain "ps2_mouse: bristle pid=" within 60s
    When I stress PS/2 input with 240 high-frequency events
    Then the serial output should contain "PS/2 IRQ entry:" within 60s
    And the serial output should contain "PS/2 IRQ exit:" within 60s
    And the serial output should contain "PS/2 IRQ wake entry:" within 60s
    And the serial output should contain "PS/2 take_scancode entry:" within 60s
    And the serial output should contain "bristle: dispatch entry" within 60s
    And the serial output should contain "bloom: input event entry" within 60s
    And the serial output should contain "bloom: handle_bristle_event entry" within 60s
    And the serial output should contain "input_rate" within 60s
