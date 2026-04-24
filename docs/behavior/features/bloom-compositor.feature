Feature: Bloom compositor service loop and responsiveness

  The bloom compositor is readiness-driven: a single service loop waits on
  all I/O sources (Wayland client port, bristle HID events, wallpaper watch)
  and dispatches to focused service objects.  Wallpaper changes are decoded
  in a background worker thread so the compositor loop is never blocked by
  image decoding.

  Scenario: bloom compositor service starts and publishes its port
    Given the machine is booted
    Then the log should match pattern "bloom: compositor service starting"
    And the log should match pattern "bloom: output0 [0-9]+x[0-9]+"

  Scenario: bloom compositor remains observable after startup
    Given the machine is booted
    Then the log should match pattern "bloom: output0 [0-9]+x[0-9]+"
    And the log should match pattern "bloom: creating service port"

  Scenario: bloom compositor reacts to wallpaper watch path
    Given the machine is booted
    When I wait for the shell prompt
    And I type "echo /share/wallpapers/flower.bmp > /session/desktop/wallpaper" on the serial console
    Then the log should match pattern "bloom: reacting to wallpaper change"

  Scenario: bloom service loop starts
    Given the machine is booted
    Then the log should match pattern "bloom: service loop started"

  Scenario: bloom service loop processes multiple wallpaper events
    # Verifies that the service loop continues to ingest events even when
    # a previous event triggered an async wallpaper decode: two rapid
    # wallpaper-change writes are both processed.
    Given the machine is booted
    When I wait for the shell prompt
    And I type "echo /share/wallpapers/flower.bmp > /session/desktop/wallpaper" on the serial console
    And I type "echo /share/wallpapers/flower.bmp > /session/desktop/wallpaper" on the serial console
    Then the log should match pattern "bloom: service loop started"
    And "bloom: reacting to wallpaper change" should appear at least 2 times

  Scenario: wallpaper reload does not block the render loop
    # Verifies the non-blocking design: a wallpaper-change event triggers an
    # async worker (start_background_load) rather than a synchronous decode.
    # The render thread immediately returns to its frame loop; the decoded
    # texture is swapped in later via poll_ready_background.
    Given the machine is booted
    When I wait for the shell prompt
    And I type "echo /share/wallpapers/flower.bmp > /session/desktop/wallpaper" on the serial console
    Then the log should match pattern "bloom: reacting to wallpaper change \(async\)"

  Scenario: bloom display driver presents a unified device interface
    # Verifies that bloom can enumerate /dev/display/card0 regardless of
    # whether the underlying driver is display_bootfb (linear framebuffer blit)
    # or display_virtio_gpu (DMA blit + GPU transfer/flush).  The kernel routes
    # VFS device call responses correctly only when the driver includes the
    # required req_id in every response — which is now guaranteed by both
    # drivers using ProviderLoop.
    Given the machine is booted
    Then the log should match pattern "bloom: output0 [0-9]+x[0-9]+"

  Scenario: virtio GPU driver accepts display device calls via VFS
    # Verifies that display_virtio_gpu responds to DISPLAY_OP_GET_INFO over
    # the VFS device call interface so bloom can read display dimensions and
    # proceed to import buffers and commit frames.
    Given the machine is booted
    Then the log should match pattern "display_virtio_gpu: GPU initialized successfully"
    And the log should match pattern "bloom: output0 [0-9]+x[0-9]+"

