Feature: Bloom compositor desktop behavior

  Bloom is the VFS-driven compositor for the default Thing-OS desktop session.
  These scenarios describe behaviors users and clients depend on: first paint,
  live desktop settings, pointer responsiveness, display backends, and Wayland
  client integration. Implementation telemetry appears only as supporting
  evidence inside those stories.

  Scenario: The desktop reaches first paint using only display and session files
    Given the machine is booted
    Then the serial output should contain "bloom: compositor service starting" within 60s
    And the serial output should contain "bloom: creating service port" within 60s
    And the serial output should contain "bloom: service loop started" within 60s
    And the serial output should contain "bloom: output0" within 60s
    And the serial output should contain "bloom: pistil background renderer loaded from /lib/libpistil.so" within 60s
    And the serial output should contain "bloom: pistil font text renderer loaded with default /public/fonts/Inter-Regular.ttf" within 60s
    And the serial output should contain "bloom: initial theme configured SolarisWarm" within 60s
    And the serial output should contain "bloom: watching theme config /session/desktop/theme" within 60s
    And the serial output should contain "bloom: watching wallpaper config /session/desktop/wallpaper" within 60s
    And the serial output should contain "bloom: built-in busy spinner ready" within 60s
    And the serial output should contain "bloom: busy spinner serviceLoop started" within 60s
    And the serial output should contain "bloom: cursor ready" within 60s
    And the serial output should contain "Wallpaper and cursor are ready." within 60s
    And the serial output should contain "Deferred visual resources are ready." within 60s
    And the serial output should contain "bloom: busy spinner waiting for stable scene" within 60s
    And the serial output should contain "bloom: busy spinner scene stable after" within 60s
    And the serial output should contain "bloom: busy spinner fading out" within 60s
    And the serial output should contain "bloom: busy spinner handoff complete; real cursor active" within 60s
    And the serial output should contain "bloom: deferred default font became available" within 90s
    And the serial output should contain "bloom: deferred symbol font became available" within 90s
    And the serial output should contain "size=96x96" within 60s
    And the serial output should contain "First frame rendered" within 60s
    And the serial output should not contain "Freed node"
    And the bloom first frame should contain visible pixels
    And the bloom cursor should be visible
    And the serial output should not contain "bloom: pointer debug overlay ready"

  Scenario: Desktop settings update through watched session files
    Given the machine is booted
    Then the serial output should contain "bloom: service loop started" within 60s
    When I wait for the shell prompt
    And I type "echo SolarisWarm > /session/desktop/theme" on the serial console
    Then the serial output should contain "Applying theme SolarisWarm" within 60s
    When I type "echo /public/wallpapers/flower.png > /session/desktop/wallpaper" on the serial console
    Then the serial output should contain "bloom: reacting to wallpaper change" within 60s
    When I type "echo /public/wallpapers/clouds.bmp > /session/desktop/wallpaper" on the serial console
    Then the serial output should contain "/public/wallpapers/clouds.bmp" within 60s
    When I type "echo /share/wallpapers/flower.png > /session/desktop/wallpaper" on the serial console
    Then the serial output should contain "bloom: migrated wallpaper path /share/wallpapers/flower.png -> /public/wallpapers/flower.png" within 60s
    When I type "echo /nonexistent/bad.bmp > /session/desktop/wallpaper" on the serial console
    Then the serial output should contain "bloom: wallpaper decode failed" within 60s
    When I type "echo wallpaper-watch-idle" on the serial console
    And I wait for 2 seconds
    Then the latest serial output should not contain "VFS: sys_fs_open path='/session/desktop/wallpaper'"

  Scenario: Built-in themes can be selected through the session theme file
    Given the machine is booted
    Then the serial output should contain "bloom: service loop started" within 60s
    When I wait for the shell prompt
    And I type "echo solaris-warm > /session/desktop/theme" on the serial console
    Then the serial output should contain "Applying theme SolarisWarm" within 60s
    And the serial output should contain "clock: applying theme SolarisWarm" within 60s
    And I type "echo aurora-glass > /session/desktop/theme" on the serial console
    Then the serial output should contain "Applying theme Aurora Glass" within 60s
    And the serial output should contain "clock: applying theme Aurora Glass" within 60s
    When I type "echo obsidian-bloom > /session/desktop/theme" on the serial console
    Then the serial output should contain "Applying theme Obsidian Bloom" within 60s
    And the serial output should contain "clock: applying theme Obsidian Bloom" within 60s

  Scenario: The Themes app lists and swaps built-in desktop themes
    Given the machine is booted
    Then the serial output should contain "bloom: service loop started" within 60s
    When I wait for the shell prompt
    And I type "themes --list" on the serial console
    Then the command output should contain "SolarisWarm"
    And the command output should contain "Aurora Glass"
    And the command output should contain "Obsidian Bloom"
    When I type "themes --set aurora-glass" on the serial console
    Then the command output should contain "Selected theme Aurora Glass"
    And the serial output should contain "Applying theme Aurora Glass" within 60s
    When I type "themes --current" on the serial console
    Then the command output should strictly be "Aurora Glass"

  @wallpaper-fade
  Scenario: Wallpaper appears with a fade from black
    Given the machine is booted
    Then the serial output should contain "First frame rendered" within 180s
    And the serial output should contain "Fading in wallpaper..." within 60s
    And the serial output should contain "Wallpaper fade-in complete" within 60s

  @bootfb
  Scenario: The boot framebuffer backend can present the desktop
    Given the machine is booted
    Then the serial output should contain "display_bootfb: recovered Cambium DriverEntryCtx" within 60s
    And the serial output should contain "display_bootfb: mounted VFS provider at /dev/display/card0 via cambium" within 60s
    And the serial output should not contain "bloom: failed to connect to any /dev/display/cardN after retries"
    And the serial output should contain "bloom: output0" within 60s
    And the serial output should contain "display_bootfb: imported buffer" within 60s
    And the serial output should contain "bloom: display driver supports partial flush (damage regions)" within 60s
    And the serial output should contain "First frame rendered" within 60s
    And the bloom first frame should contain visible pixels
    And the serial output should not contain "task='display_bootfb'"
    And the serial output should not contain "virtio_gpu: cursor queue (queue 1) configured"

  Scenario: The virtio GPU backend exposes accelerated presentation capabilities
    Given the machine is booted
    Then the serial output should contain "display_virtio_gpu: recovered Cambium DriverEntryCtx" within 60s
    And the serial output should contain "display_virtio_gpu: mounted VFS provider at /dev/display/card0 via cambium" within 60s
    And the serial output should not contain "bloom: failed to connect to any /dev/display/cardN after retries"
    And the serial output should contain "display_virtio_gpu: GPU initialized successfully" within 60s
    And the serial output should contain "display_virtio_gpu: host scanout" within 60s
    And the serial output should contain "bloom: output0" within 60s
    And the serial output should contain "bloom: display driver does not support VBLANK" within 60s
    And the serial output should contain "bloom: display driver supports GPU blit (hardware transfer/flush)" within 60s
    And the serial output should contain "bloom: display driver supports direct scanout (zero-copy path to display)" within 60s
    And the serial output should contain "bloom: display driver supports partial flush (damage regions)" within 60s
    And the serial output should contain "bloom: display driver supports resource cache (pre-allocated buffer pool)" within 60s
    And the serial output should contain "bloom: display driver supports linear dmabuf import" within 60s
    And the serial output should contain "display_virtio_gpu: composition paths: gpu_opaque_copy=enabled cpu_alpha_blend=enabled" within 60s
    And the serial output should contain "display_virtio_gpu: virgl GPU alpha blend ready" within 60s
    And the serial output should contain "gpu_alpha_blend=enabled" within 60s
    And the serial output should contain "bloom: display driver supports GPU alpha blending" within 120s
    And the serial output should contain "display_virtio_gpu: accel2d_gpu=enabled" within 60s
    And the serial output should contain "First frame rendered" within 60s
    And the serial output should contain "display_virtio_gpu: first commit copied buffer=" within 60s
    And the serial output should contain "gpu_planes=" within 60s
    And the serial output should contain "cpu_planes=" within 60s

  Scenario: Hardware cursor motion stays responsive without full scene recomposition
    Given the machine is booted
    Then the serial output should contain "virtio_gpu: cursor queue (queue 1) configured" within 60s
    And the serial output should contain "bloom: hw cursor image set buffer=" within 60s
    And the serial output should contain "hw_size=64x64" within 60s
    And the serial output should contain "hotspot=" within 60s
    And the bloom cursor should be visible
    When I move the mouse
    Then the serial output should contain "bloom: hw cursor move" within 60s
    And the pointer debug overlay should update after mouse movement

  @pointer-debug
  Scenario: Pointer debug mode exposes cursor, damage, and motion behavior
    Given the machine is booted
    Then the serial output should contain "bloom: registered bristle pointer sink" within 60s
    And the serial output should contain "ps2_kbd: bristle pid=" within 60s
    When I press Alt+F7
    Then the serial output should contain "bloom: pointer debug overlay enabled" within 60s
    And the serial output should contain "bloom: pointer debug overlay ready" within 60s
    And the pointer debug overlay should include the cursor svg
    And the pointer debug overlay should update after mouse movement
    And the serial output should contain "bloom: committing bounded damage rects=" within 60s
    And the serial output should contain "bloom: cursor smoothing" within 60s
    And the serial output should contain "bloom: motion coalesce pre=" within 60s
    And the serial output should not contain "controlq faulted"
    And the serial output should not contain "failed to unmap released buffer"
    And the serial output should not contain "failed to close released buffer"
    When I press Alt+F7
    Then the serial output should contain "bloom: pointer debug overlay disabled" within 60s

  Scenario: Default Wayland clients receive a complete desktop protocol surface
    Given the machine is booted
    Then the serial output should contain "wayland-server: listening on /run/wayland-0" within 60s
    And the serial output should contain "wayland-server: advertising globals" within 60s
    And the serial output should contain "bloom: output0" within 60s
    And the serial output should contain "wl_subcompositor" within 60s
    And the serial output should contain "wl_data_device_manager" within 60s
    And the serial output should contain "wayland-server: sending us-intl XKB keymap" within 120s
    And the serial output should contain "wayland_hello: connected to /run/wayland-0" within 60s
    And the serial output should contain "wayland-server: xdg_surface obj=" within 60s
    And the serial output should contain "wayland-server: xdg_toplevel obj=" within 60s
    And the serial output should contain "wayland-server: surface " within 60s
    And the serial output should contain "wayland-server: frame callback done" within 60s
    And the serial output should contain "wayland_hello: frame callback done" within 60s
    And the serial output should contain "clock: running at low scheduler priority" within 60s

  Scenario: Wayland clients can use dmabuf, regions, clipboard, and drag-and-drop
    Given the machine is booted
    Then the serial output should contain "wayland-server: listening on /run/wayland-0" within 60s
    And the serial output should contain "zwp_linux_dmabuf_v1" within 60s
    And the serial output should contain "wayland_hello: using zwp_linux_dmabuf_v1 buffers" within 60s
    And the serial output should contain "wayland-server: imported dmabuf wl_buffer=" within 60s
    And the serial output should contain "wayland-server: wl_compositor create_region id=" within 60s
    And the serial output should contain "wayland-server: wl_region obj=" within 60s
    And the serial output should contain "add x=" within 60s
    And the serial output should contain "wayland_hello: wl_region smoke test: create+add+set_opaque+destroy" within 60s
    And the serial output should contain "commit opaque_region=Some(" within 60s
    And the serial output should contain "wayland_hello: wl_region smoke test: create+add+set_input+destroy" within 60s
    And the serial output should contain "commit input_region=Some(" within 60s
    When I wait for the shell prompt
    And I type "wayland_clipboard_test" on the serial console
    Then the serial output should contain "wayland-server: clipboard selection set" within 60s
    When I type "wayland_dnd_test" on the serial console
    Then the serial output should contain "wayland-server: DnD started" within 60s
    And the serial output should contain "wayland_dnd_test: start_drag acknowledged by compositor" within 60s

  Scenario: libpistil exposes rendering entry points used by Bloom and clients
    Given the machine is booted
    When I wait for the shell prompt
    And I type "test_dlopen" on the serial console
    Then the serial output should contain "[test_dlopen] pistil_draw_vector_smoke: PASS" within 60s

  Scenario: Bloom startup emits an ordered phase trace through desktop_ready
    Given the machine is booted
    Then the serial output should contain "bloom.phase=start" within 60s
    And the serial output should contain "bloom.phase=open_display" within 60s
    And the serial output should contain "bloom.phase=get_info" within 60s
    And the serial output should contain "bloom.phase=import_primary_buffer" within 60s
    And the serial output should contain "bloom.phase=load_wallpaper" within 60s
    And the serial output should contain "bloom.phase=init_cursor" within 60s
    And the serial output should contain "bloom.phase=first_damage" within 60s
    And the serial output should contain "bloom.phase=first_commit_begin" within 60s
    And the serial output should contain "bloom.phase=first_commit_done" within 60s
    And the serial output should contain "bloom.phase=desktop_ready" within 60s

  Scenario: Display RPC watchdog does not fire on a healthy desktop
    Given the machine is booted
    Then the serial output should contain "First frame rendered" within 180s
    And the serial output should not contain "rpc.stall"
    And the serial output should not contain "rpc.slow"
    And the serial output should not contain "VFS_RPC: display device_call.stall"
    And the serial output should not contain "VFS_RPC: display device_call.slow"
    And the serial output should not contain "VFS_RPC: TIMEOUT"
