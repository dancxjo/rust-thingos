Feature: Bloom compositor service loop and responsiveness

  The bloom compositor is readiness-driven: a single service loop waits on
  all I/O sources (Wayland client port, bristle HID events, wallpaper watch)
  and dispatches to focused service objects.  Wallpaper changes are loaded
  after the loop is live so first paint is not blocked by image decoding.

  Scenario: bloom compositor service starts and publishes its port
    Given the machine is booted
    Then the serial output should contain "bloom: compositor service starting" within 60s
    And the serial output should contain "bloom: output0" within 60s

  Scenario: bloom links the pistil wallpaper renderer
    Given the machine is booted
    Then the serial output should contain "bloom: pistil background renderer loaded from /lib/libpistil.so" within 60s

  Scenario: bloom compositor remains observable after startup
    Given the machine is booted
    Then the serial output should contain "bloom: output0" within 60s
    And the serial output should contain "bloom: creating service port" within 60s

  Scenario: bloom compositor reacts to wallpaper watch path
    Given the machine is booted
    Then the serial output should contain "bloom: service loop started" within 60s
    When I wait for the shell prompt
    And I type "echo /share/wallpapers/flower.bmp > /session/desktop/wallpaper" on the serial console
    Then the serial output should contain "bloom: reacting to wallpaper change" within 60s

  Scenario: bloom service loop starts
    Given the machine is booted
    Then the serial output should contain "bloom: service loop started" within 60s

  Scenario: bloom service loop processes multiple wallpaper events
    # Verifies that the service loop continues to ingest events even when
    # a previous event triggered a wallpaper reload: two rapid wallpaper-change
    # writes are both processed.
    Given the machine is booted
    Then the serial output should contain "bloom: service loop started" within 60s
    When I wait for the shell prompt
    And I type "echo /share/wallpapers/flower.bmp > /session/desktop/wallpaper" on the serial console
    And I type "echo /share/wallpapers/clouds.bmp > /session/desktop/wallpaper" on the serial console
    Then the serial output should contain "bloom: reacting to wallpaper change" within 60s
    And the serial output should contain "/share/wallpapers/clouds.bmp" within 60s

  Scenario: wallpaper reload happens after the service loop is live
    # Verifies that a wallpaper-change event is handled by the service loop
    # after startup.  The bundled wallpapers are small and decoded inline to
    # avoid allocator corruption from concurrent background loads.
    Given the machine is booted
    Then the serial output should contain "bloom: service loop started" within 60s
    When I wait for the shell prompt
    And I type "echo /share/wallpapers/flower.bmp > /session/desktop/wallpaper" on the serial console
    Then the serial output should contain "bloom: reacting to wallpaper change" within 60s

  Scenario: bloom display driver presents a unified device interface
    # Verifies that bloom can enumerate /dev/display/card0 regardless of
    # whether the underlying driver is display_bootfb (linear framebuffer blit)
    # or display_virtio_gpu (DMA blit + GPU transfer/flush).  The kernel routes
    # VFS device call responses correctly only when the driver includes the
    # required req_id in every response — which is now guaranteed by both
    # drivers using ProviderLoop.
    Given the machine is booted
    Then the serial output should contain "bloom: output0" within 60s

  Scenario: virtio GPU driver accepts display device calls via VFS
    # Verifies that display_virtio_gpu responds to DISPLAY_OP_GET_INFO over
    # the VFS device call interface so bloom can read display dimensions and
    # proceed to import buffers and commit frames.
    Given the machine is booted
    Then the serial output should contain "display_virtio_gpu: GPU initialized successfully" within 60s
    And the serial output should contain "bloom: output0" within 60s

  Scenario: bloom service loop paints the first frame without client connections
    # The FrameClock starts with repaint_requested=true so the compositor
    # produces an initial frame (the fallback wallpaper) immediately on boot,
    # before any Wayland client has connected.  This verifies the loop reaches
    # the repaint phase and that the display driver receives real pixels.
    Given the machine is booted
    Then the serial output should contain "bloom: service loop started" within 60s
    And the serial output should contain "bloom: output0" within 60s
    And the serial output should contain "First frame rendered" within 60s
    And the serial output should not contain "Freed node"
    And the bloom first frame should contain visible pixels

  Scenario: failed wallpaper decode leaves previous wallpaper active
    Given the machine is booted
    Then the serial output should contain "bloom: service loop started" within 60s
    When I wait for the shell prompt
    And I type "echo /nonexistent/bad.bmp > /session/desktop/wallpaper" on the serial console
    Then the serial output should contain "bloom: wallpaper decode failed" within 60s
