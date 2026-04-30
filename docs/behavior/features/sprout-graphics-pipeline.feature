Feature: Sprout graphics pipeline orchestration

  Sprout is responsible for bringing up the graphics and input stack during
  early boot. This feature verifies that sprout correctly launches bloom,
  bristle, and the necessary drivers, and that they reach a functional state.

  Scenario: sprout launches the full graphics pipeline
    Given the machine is booted
    Then the serial output should contain "SPROUT: Starting full pipeline (graphics + input)" within 60s
    And the serial output should contain "SPROUT: Spawned bristle" within 60s
    And the serial output should contain "SPROUT: Spawned bloom" within 120s
    And the serial output should contain "bloom: compositor service starting" within 120s
    And the serial output should contain "bloom: service loop started" within 180s
    And the serial output should contain "First frame rendered" within 180s

  Scenario: sprout launches the default Wayland client
    Given the machine is booted
    Then the serial output should contain "wayland-server: listening on /run/wayland-0" within 180s
    And the serial output should contain "SPROUT: Spawned wayland_hello" within 180s
    And the serial output should contain "wayland_hello: connected to /run/wayland-0" within 180s
    And the serial output should contain "wayland_hello: pistil text renderer loaded with default /share/fonts/NotoSans-Regular.ttf" within 180s
    And the serial output should contain "wayland-server: xdg_surface obj=" within 180s
    And the serial output should contain "wayland-server: xdg_toplevel obj=" within 180s
    And the serial output should contain "wayland-server: surface " within 180s
    And the serial output should contain "wayland-server: frame callback done" within 180s
    And the serial output should contain "wayland_hello: frame callback done" within 180s

  Scenario: sprout launches the Wayland clock client
    Given the machine is booted
    Then the serial output should contain "SPROUT: Spawned clock" within 180s
    And the serial output should contain "clock: connected to /run/wayland-0" within 180s
    And the serial output should contain "clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf" within 180s
    And the serial output should contain "clock: tick local=" within 180s

  Scenario: rtc anchors the system clock before the Wayland clock displays realtime
    Given the machine is booted
    Then the serial output should contain "RTC: claimed /sys/devices/isa-0070" within 180s
    And the serial output should contain "RTC: System clock anchored" within 180s
    And the serial output should contain "clock: tick local=" within 180s
    And the serial output should not contain "WAITING FOR RTC"

  Scenario: Cambium matches the RTC CMOS device by canonical kind
    Given the machine is booted
    Then the serial output should contain "RTC: System clock anchored" within 180s
    And the serial output should contain "System clock anchored:" within 180s
