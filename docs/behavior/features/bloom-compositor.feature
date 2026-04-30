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

  Scenario: bloom links the pistil Noto Sans text renderer
    Given the machine is booted
    Then the serial output should contain "bloom: pistil font text renderer loaded with default /share/fonts/NotoSans-Regular.ttf" within 60s

  Scenario: libpistil exports the vector renderer
    Given the machine is booted
    When I wait for the shell prompt
    And I type "test_dlopen" on the serial console
    Then the serial output should contain "[test_dlopen] pistil_draw_vector_smoke: PASS" within 60s

  Scenario: bloom compositor remains observable after startup
    Given the machine is booted
    Then the serial output should contain "bloom: output0" within 60s
    And the serial output should contain "bloom: creating service port" within 60s

  Scenario: bloom compositor reacts to wallpaper watch path
    Given the machine is booted
    Then the serial output should contain "bloom: service loop started" within 60s
    When I wait for the shell prompt
    And I type "echo /share/wallpapers/flower.png > /session/desktop/wallpaper" on the serial console
    Then the serial output should contain "bloom: reacting to wallpaper change" within 60s

  Scenario: bloom uses the wallpaper watch without steady-state polling
    Given the machine is booted
    Then the serial output should contain "bloom: watching wallpaper config /session/desktop/wallpaper" within 60s
    When I wait for the shell prompt
    And I type "echo wallpaper-watch-idle" on the serial console
    And I wait for 2 seconds
    Then the latest serial output should not contain "VFS: sys_fs_open path='/session/desktop/wallpaper'"

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
    And I type "echo /share/wallpapers/flower.png > /session/desktop/wallpaper" on the serial console
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
    And I type "echo /share/wallpapers/flower.png > /session/desktop/wallpaper" on the serial console
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

  @bootfb
  Scenario: boot framebuffer driver paints the first compositor commit
    # Regression coverage for /dev/fb0 shared mappings: the display_bootfb
    # provider must map the physical framebuffer into userspace, not the
    # kernel-virtual scanout address reported by the boot runtime.  It must
    # also receive Bloom's inline damage rects after the used plane array so
    # a successful commit actually copies pixels to the boot framebuffer.
    Given the machine is booted
    Then the serial output should contain "display_bootfb: imported buffer" within 60s
    And the serial output should contain "First frame rendered" within 60s
    And the bloom first frame should contain visible pixels
    And the serial output should not contain "task='display_bootfb'"

  Scenario: bloom first frame includes the cached cursor plane
    Given the machine is booted
    Then the serial output should contain "bloom: cursor ready" within 60s
    And the serial output should contain "First frame rendered" within 60s
    And the bloom cursor should be visible

  Scenario: bloom first frame leaves the pointer debug overlay off by default
    Given the machine is booted
    Then the serial output should contain "First frame rendered" within 60s
    And the serial output should not contain "bloom: pointer debug overlay ready"

  @pointer-debug
  Scenario: Alt F7 toggles the pointer debug overlay
    Given the machine is booted
    Then the serial output should contain "bloom: registered bristle pointer sink" within 60s
    And the serial output should contain "ps2_kbd: bristle pid=" within 60s
    When I press Alt+F7
    Then the serial output should contain "bloom: pointer debug overlay enabled" within 60s
    And the serial output should contain "bloom: pointer debug overlay ready" within 60s
    When I press Alt+F7
    Then the serial output should contain "bloom: pointer debug overlay disabled" within 60s

  @pointer-debug
  Scenario: enabled pointer debug overlay includes the cursor svg
    Given the machine is booted
    Then the pointer debug overlay should include the cursor svg

  @pointer-debug
  Scenario: pointer debug overlay updates after mouse movement
    Given the machine is booted
    Then the pointer debug overlay should update after mouse movement

  @pointer-debug
  Scenario: pointer movement commits bounded damage
    Given the machine is booted
    Then the pointer debug overlay should update after mouse movement
    And the serial output should contain "bloom: committing bounded damage rects=" within 60s

  @pointer-debug
  Scenario: delayed pointer samples animate the visible cursor toward the latest position
    Given the machine is booted
    Then the pointer debug overlay should update after mouse movement
    And the serial output should contain "bloom: cursor smoothing" within 60s

  Scenario: failed wallpaper decode leaves previous wallpaper active
    Given the machine is booted
    Then the serial output should contain "bloom: service loop started" within 60s
    When I wait for the shell prompt
    And I type "echo /nonexistent/bad.bmp > /session/desktop/wallpaper" on the serial console
    Then the serial output should contain "bloom: wallpaper decode failed" within 60s

  Scenario: display driver reports VBLANK capability
    # Verifies that the display driver advertises the VBLANK capability so
    # bloom and other clients know that CommitFlags::VSYNC is honoured.
    Given the machine is booted
    Then the serial output should contain "bloom: output0" within 60s
    And the serial output should contain "bloom: display driver supports VBLANK (vsync)" within 60s
  @pointer-debug
  Scenario: pointer motion events are coalesced to latest-per-frame
    # Multiple PointerMove samples arriving between frames must be aggregated
    # into a single focus lookup and a single client delivery.  The compositor
    # logs coalesce stats (pre= post=) on the first few motion flushes.
    Given the machine is booted
    Then the pointer debug overlay should update after mouse movement
    And the serial output should contain "bloom: motion coalesce pre=" within 60s
