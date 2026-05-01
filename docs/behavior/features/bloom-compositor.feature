Feature: Bloom compositor service loop and responsiveness

  The bloom compositor is readiness-driven: a single service loop waits on
  all I/O sources (Wayland client port, bristle HID events, wallpaper/theme watch)
  and dispatches to focused service objects.  Wallpaper changes are loaded
  after the loop is live so first paint is not blocked by image decoding.

  Scenario: bloom compositor service starts and publishes its port
    Given the machine is booted
    Then the serial output should contain "bloom: compositor service starting" within 60s
    And the serial output should contain "bloom: output0" within 60s

  Scenario: bloom links the pistil wallpaper renderer
    Given the machine is booted
    Then the serial output should contain "bloom: pistil background renderer loaded from /lib/libpistil.so" within 60s

  Scenario: bloom links the pistil Inter text renderer
    Given the machine is booted
    Then the serial output should contain "bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf" within 60s

  Scenario: bloom configures the default Solarized Warm theme
    Given the machine is booted
    Then the serial output should contain "bloom: initial theme configured Solarized Warm" within 60s
    And the serial output should contain "bloom: watching theme config /session/desktop/theme" within 60s

  Scenario: bloom compositor reacts to theme watch path
    Given the machine is booted
    Then the serial output should contain "bloom: service loop started" within 60s
    When I wait for the shell prompt
    And I type "echo Solarized Warm > /session/desktop/theme" on the serial console
    Then the serial output should contain "bloom: reacting to theme change: Solarized Warm -> Solarized Warm" within 60s

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

  Scenario: virtio GPU driver discovers host scanout dimensions
    # Regression coverage for host-driven QEMU display sizing.  The virtio
    # display path must query virtio-gpu scanout metadata instead of freezing
    # the compositor to the boot framebuffer dimensions forever.
    Given the machine is booted
    Then the serial output should contain "display_virtio_gpu: host scanout" within 60s
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
    And the serial output should contain "size=96x96" within 60s
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
  Scenario: pointer motion schedules a paced cursor repaint
    Given the machine is booted
    Then the pointer debug overlay should update after mouse movement

  @pointer-debug
  Scenario: pointer movement commits bounded damage
    # Pointer movement drives a burst of small DISPLAY_OP_COMMIT calls through
    # display_virtio_gpu.  The virtio control queue must not wedge or fault
    # while processing the transfer/flush sequence for those commits.
    Given the machine is booted
    Then the pointer debug overlay should update after mouse movement
    And the serial output should contain "bloom: committing bounded damage rects=" within 60s
    And the serial output should not contain "controlq faulted"

  @pointer-debug
  Scenario: pointer animation releases imported display buffers cleanly
    # The compositor replaces pointer/chrome buffers during pointer animation.
    # display_virtio_gpu must tear down the provider-side import mapping and
    # translated FD when Bloom releases the old buffer.
    Given the machine is booted
    Then the pointer debug overlay should update after mouse movement
    And the serial output should not contain "failed to unmap released buffer"
    And the serial output should not contain "failed to close released buffer"

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

  Scenario: synchronous display driver does not advertise VBLANK capability
    # The current display providers answer DISPLAY_OP_COMMIT synchronously over
    # VFS RPC. They must not advertise VBLANK because software-vsync sleeps in
    # that path keep bloom from returning to its input loop.
    Given the machine is booted
    Then the serial output should contain "bloom: output0" within 60s
    And the serial output should contain "bloom: display driver does not support VBLANK" within 60s

  Scenario: virtio GPU driver advertises GPU blit and direct scanout capabilities
    # display_virtio_gpu uses GPU transfer/flush commands (GPU_BLIT) and
    # sets up a scanout resource (DIRECT_SCANOUT). Bloom must log both
    # after reading DISPLAY_OP_GET_INFO so capability-gated code paths can
    # be selected at runtime.
    Given the machine is booted
    Then the serial output should contain "bloom: output0" within 60s
    And the serial output should contain "bloom: display driver supports GPU blit (hardware transfer/flush)" within 60s
    And the serial output should contain "bloom: display driver supports direct scanout (zero-copy path to display)" within 60s

  Scenario: virtio GPU driver advertises partial flush and resource cache capabilities
    # display_virtio_gpu processes client damage rects (PARTIAL_FLUSH) and
    # maintains a pre-allocated frame pool (RESOURCE_CACHE). Bloom logs both
    # after reading DISPLAY_OP_GET_INFO.
    Given the machine is booted
    Then the serial output should contain "bloom: output0" within 60s
    And the serial output should contain "bloom: display driver supports partial flush (damage regions)" within 60s
    And the serial output should contain "bloom: display driver supports resource cache (pre-allocated buffer pool)" within 60s

  @bootfb
  Scenario: boot framebuffer driver advertises partial flush capability
    # display_bootfb blits only the client-supplied damage rectangles on each
    # commit, so it correctly advertises PARTIAL_FLUSH. It must not advertise
    # any GPU_* capabilities because all rendering is done in software on the CPU.
    Given the machine is booted with framebuffer display
    Then the serial output should contain "bloom: output0" within 60s
    And the serial output should contain "bloom: display driver supports partial flush (damage regions)" within 60s

  Scenario: desktop clock runs below compositor input priority
    # The clock is a decorative Wayland client. It should not compete with
    # Bloom's cursor/input path at normal scheduler priority.
    Given the machine is booted
    Then the serial output should contain "clock: running at low scheduler priority" within 60s

  @pointer-debug
  Scenario: pointer motion events are coalesced to latest-per-frame
    # Multiple PointerMove samples arriving between frames must be aggregated
    # into a single focus lookup and a single client delivery.  The compositor
    # logs coalesce stats (pre= post=) on the first few motion flushes.
    Given the machine is booted
    Then the pointer debug overlay should update after mouse movement
    And the serial output should contain "bloom: motion coalesce pre=" within 60s

  Scenario: wl_output global is advertised in wl_registry
    # Verifies that bloom advertises wl_output so clients can discover display
    # properties such as resolution, scale, and refresh rate.
    Given the machine is booted
    Then the serial output should contain "bloom: output0" within 60s
    And the serial output should contain "wayland-server: listening on /run/wayland-0" within 60s

  Scenario: wl_subcompositor global is advertised so clients can build subsurface trees
    # Verifies that bloom advertises wl_subcompositor (v1) alongside the other
    # wl_registry globals.  Clients use it to give wl_surfaces the subsurface
    # role for nested UI hierarchies (e.g. titlebars, popovers, or video
    # overlays composited inside a parent toplevel).
    Given the machine is booted
    Then the serial output should contain "wayland-server: listening on /run/wayland-0" within 60s
    And the serial output should contain "wayland-server: advertising globals" within 60s
    And the serial output should contain "wl_subcompositor" within 60s
  Scenario: wl_data_device_manager is advertised in wl_registry
    # Verifies that bloom advertises wl_data_device_manager so Wayland clients
    # can use clipboard (copy/paste) functionality.
    Given the machine is booted
    Then the serial output should contain "wayland-server: listening on /run/wayland-0" within 60s
    And the serial output should contain "wl_data_device_manager" within 60s

  Scenario: linux dmabuf global is advertised for GPU-backed display import
    # Verifies that bloom exposes zwp_linux_dmabuf_v1 when the display driver
    # can import linear FD-backed buffers, and that the default Wayland smoke
    # client can create and commit wl_buffers through the dmabuf path.
    Given the machine is booted
    Then the serial output should contain "bloom: display driver supports linear dmabuf import" within 60s
    And the serial output should contain "zwp_linux_dmabuf_v1" within 60s
    And the serial output should contain "wayland_hello: using zwp_linux_dmabuf_v1 buffers" within 60s
    And the serial output should contain "wayland-server: imported dmabuf wl_buffer=" within 60s

  Scenario: clipboard selection is broadcast to other clients
    # Verifies that wl_data_device.set_selection causes bloom to log the
    # clipboard ownership change and relay the offer to other clients.
    Given the machine is booted
    Then the serial output should contain "wayland-server: listening on /run/wayland-0" within 60s
    When I wait for the shell prompt
    And I type "wayland_clipboard_test" on the serial console
    Then the serial output should contain "wayland-server: clipboard selection set" within 60s

  Scenario: wl_data_device.start_drag initiates a drag-and-drop session
    # Verifies that calling wl_data_device.start_drag causes bloom to record
    # the drag owner and MIME types, transitioning into DnD-active state.
    Given the machine is booted
    Then the serial output should contain "wayland-server: listening on /run/wayland-0" within 60s
    When I wait for the shell prompt
    And I type "wayland_dnd_test" on the serial console
    Then the serial output should contain "wayland-server: DnD started" within 60s
    And the serial output should contain "wayland_dnd_test: start_drag acknowledged by compositor" within 60s

  Scenario: virtio-GPU display driver advertises hardware cursor capability
    # When bloom connects to display_virtio_gpu and the cursor virtqueue is
    # available, DISPLAY_OP_GET_INFO must return DisplayCaps::HARDWARE_CURSOR so
    # bloom can take the cursor-only fast path.
    Given the machine is booted
    Then the serial output should contain "virtio_gpu: cursor queue (queue 1) configured" within 60s
    And the serial output should contain "bloom: hw cursor image set buffer=" within 60s
    And the serial output should contain "hw_size=64x64" within 60s
    And the bloom cursor should be visible

  Scenario: cursor-only motion does not trigger full scene recomposition
    # Moving the pointer when no window content has changed must skip the full
    # software compose path.  Bloom logs a trace message on each hardware cursor
    # move; a full commit would instead log the normal present path.
    Given the machine is booted
    Then the serial output should contain "bloom: output0" within 60s
    When the user moves the mouse pointer
    Then the serial output should contain "bloom: hw cursor move" within 60s
    And the pointer debug overlay should update after mouse movement

  Scenario: display_bootfb does not advertise hardware cursor
    # The boot framebuffer driver has no cursor queue and must not advertise
    # DisplayCaps::HARDWARE_CURSOR.  When bloom connects to display_bootfb it
    # must fall back to the software cursor composition path.
    Given the machine is booted with framebuffer display
    Then the serial output should contain "bloom: output0" within 60s
    And the serial output should not contain "virtio_gpu: cursor queue (queue 1) configured" within 10s

  Scenario: cursor hotspot is respected during hardware cursor placement
    # SET_CURSOR carries a hotspot offset.  The hardware cursor must be placed
    # so that the hotspot—not the top-left corner of the cursor image—aligns
    # with the pointer position reported by the input subsystem.
    Given the machine is booted
    Then the serial output should contain "bloom: hw cursor image set buffer=" within 60s
    And the serial output should contain "hotspot=" within 60s

  Scenario: virtio GPU driver announces GPU and CPU composition paths at startup
    # display_virtio_gpu must log that both the GPU fast-copy (opaque) path and
    # the CPU alpha-blend fallback path are available so that operations with
    # visibility into the driver's capabilities can confirm the feature is present.
    Given the machine is booted
    Then the serial output should contain "display_virtio_gpu: GPU initialized successfully" within 60s
    And the serial output should contain "display_virtio_gpu: composition paths: gpu_opaque_copy=enabled cpu_alpha_blend=enabled" within 60s

  Scenario: virtio GPU driver advertises GPU_ALPHA_BLEND when virgl 3D is available
    # When the virtio-gpu device supports virgl 3D, the driver initialises the
    # GPU alpha-blend pipeline and logs the extended composition path message.
    # Bloom uses supports_gpu_alpha_blend() to branch on this capability and
    # avoid per-pixel CPU blend loops on the client side.
    Given the machine is booted
    Then the serial output should contain "display_virtio_gpu: GPU initialized successfully" within 60s
    And the serial output should contain "display_virtio_gpu: virgl GPU alpha blend ready" within 60s
    And the serial output should contain "gpu_alpha_blend=enabled" within 60s
    And the serial output should contain "bloom: display driver supports GPU alpha blending" within 120s

  Scenario: virtio GPU driver advertises ACCEL2D_GPU when virgl 3D is available
    # When the virtio-gpu device supports virgl 3D, the driver enables the
    # GPU-backed ACCEL2D execution path and logs a confirmation message.
    # ACCEL2D_CMD_COPY_RECT and ACCEL2D_CMD_ALPHA_BLIT are routed to the virgl
    # GPU pipeline; other commands use the CPU fallback path.
    Given the machine is booted
    Then the serial output should contain "display_virtio_gpu: GPU initialized successfully" within 60s
    And the serial output should contain "display_virtio_gpu: accel2d_gpu=enabled" within 60s

  Scenario: virtio GPU driver records GPU vs CPU plane composition counts on first commit
    # After the first successful DISPLAY_OP_COMMIT the driver logs how many
    # planes went through the GPU fast-copy path and how many required CPU
    # alpha blending.  Both counters must appear in the first-commit log line.
    Given the machine is booted
    Then the serial output should contain "First frame rendered" within 60s
    And the serial output should contain "display_virtio_gpu: first commit copied buffer=" within 60s
    And the serial output should contain "gpu_planes=" within 60s
    And the serial output should contain "cpu_planes=" within 60s



  Scenario: wl_keyboard keymap is advertised as us-intl XKB layout
    # Verifies that bloom sends a proper XKB keymap (WL_KEYBOARD_KEYMAP_FORMAT_XKB_V1)
    # when a client binds wl_keyboard via wl_seat.  The keymap should use the
    # us-intl layout so that clients can decode key symbols including dead keys
    # for accented characters.
    Given the machine is booted
    Then the serial output should contain "wayland-server: listening on /run/wayland-0" within 60s
    And the serial output should contain "wayland-server: sending us-intl XKB keymap" within 120s

  Scenario: wl_compositor.create_region produces a live region object
    # Verifies that wl_compositor.create_region no longer registers a destroyed/
    # no-op tombstone but instead allocates a live wl_region object.  The
    # wayland_hello smoke client calls create_region at startup and the
    # compositor must log the creation.
    Given the machine is booted
    Then the serial output should contain "wayland-server: listening on /run/wayland-0" within 60s
    And the serial output should contain "wayland-server: wl_compositor create_region id=" within 60s

  Scenario: wl_region.add is handled by the compositor
    # Verifies that wl_region.add accumulates a rectangle into the region's
    # rect list rather than crashing or producing a protocol error.
    Given the machine is booted
    Then the serial output should contain "wayland-server: listening on /run/wayland-0" within 60s
    And the serial output should contain "wayland-server: wl_region obj=" within 60s
    And the serial output should contain "add x=" within 60s

  Scenario: wl_region lifecycle completes without protocol error
    # Verifies the full wl_region lifecycle: create → add → set_opaque_region →
    # destroy all succeed without bloom emitting a protocol error.  The
    # wayland_hello client logs a specific message on completion.
    Given the machine is booted
    Then the serial output should contain "wayland-server: listening on /run/wayland-0" within 60s
    And the serial output should contain "wayland_hello: wl_region smoke test: create+add+set_opaque+destroy" within 60s

  Scenario: wl_surface.set_opaque_region is committed to the scene
    # Verifies that the opaque region set by a Wayland client via
    # wl_surface.set_opaque_region is resolved from the wl_region object and
    # propagated to the compositor's scene at commit time.  bloom logs a debug
    # message including the resolved bounding rect when the IPC command is
    # emitted.
    Given the machine is booted
    Then the serial output should contain "wayland-server: listening on /run/wayland-0" within 60s
    And the serial output should contain "wayland-server: wl_surface obj=" within 60s
    And the serial output should contain "commit opaque_region=Some(" within 60s

  Scenario: wl_surface.set_input_region is committed to the scene
    # Verifies that the input region set by a Wayland client via
    # wl_surface.set_input_region is resolved from the wl_region object and
    # propagated to the compositor's scene at commit time so that pointer
    # hit-testing respects the declared input area.  bloom logs a debug message
    # including the resolved bounding rect when the IPC command is emitted.
    # wayland_hello exercises the full lifecycle: create_region → add →
    # set_input_region → destroy → commit.
    Given the machine is booted
    Then the serial output should contain "wayland-server: listening on /run/wayland-0" within 60s
    And the serial output should contain "wayland_hello: wl_region smoke test: create+add+set_input+destroy" within 60s
    And the serial output should contain "commit input_region=Some(" within 60s
