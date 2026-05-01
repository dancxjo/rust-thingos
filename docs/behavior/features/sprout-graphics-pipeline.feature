Feature: Sprout desktop session orchestration
  Sprout brings up the graphical session by starting the compositor, input
  broker, default Wayland clients, and realtime clock source.

  Scenario: The graphical session reaches an interactive desktop
    Given the machine is booted
    Then the serial output should contain "SPROUT: Starting full pipeline (graphics + input)" within 60s
    And the serial output should contain "SPROUT: Spawned bristle" within 60s
    And the serial output should contain "SPROUT: Spawned bloom" within 120s
    And the serial output should contain "bloom: compositor service starting" within 120s
    And the serial output should contain "bloom: service loop started" within 180s
    And the serial output should contain "First frame rendered" within 180s
    And the serial output should contain "wayland-server: listening on /run/wayland-0" within 180s
    And the serial output should contain "SPROUT: Spawned wayland_hello" within 180s
    And the serial output should contain "wayland_hello: connected to /run/wayland-0" within 180s
    And the serial output should contain "wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf" within 180s
    And the serial output should contain "wayland-server: xdg_surface obj=" within 180s
    And the serial output should contain "wayland-server: xdg_toplevel obj=" within 180s
    And the serial output should contain "wayland-server: surface " within 180s
    And the serial output should contain "wayland-server: frame callback done" within 180s
    And the serial output should contain "wayland_hello: frame callback done" within 180s

  Scenario: The default desktop apps start with clock time anchored by RTC
    Given the machine is booted
    Then the serial output should contain "SPROUT: Spawned clock" within 180s
    And the serial output should contain "clock: connected to /run/wayland-0" within 180s
    And the serial output should contain "clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf" within 180s
    And the serial output should contain "RTC: claimed /sys/devices/isa-0070" within 180s
    And the serial output should contain "RTC: System clock anchored" within 180s
    And the serial output should contain "System clock anchored:" within 180s
    And the serial output should contain "System clock tick:" within 180s
    And the serial output should contain "clock: tick local=" within 180s
    And the serial output should contain "SPROUT: Spawned leaf" within 180s
    And the serial output should contain "leaf: connected to /run/wayland-0" within 180s
    And the serial output should contain "leaf: spawned shell pid=" within 180s
    And the serial output should contain "leaf: pipe-backed shell input uses local echo" within 180s
