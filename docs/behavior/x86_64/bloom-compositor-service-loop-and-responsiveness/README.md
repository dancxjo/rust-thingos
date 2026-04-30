# Feature: Bloom compositor service loop and responsiveness

> Last run: 2026-04-30 10:54:48

## Scenarios

| Scenario | Steps | Status | Link |
|----------|-------|--------|------|
| bloom compositor service starts and publishes its port | 3/3 | ✅ | [View Details](bloom-compositor-service-starts-and-publishes-its-port/README.md) |
| bloom links the pistil wallpaper renderer | 2/2 | ✅ | [View Details](bloom-links-the-pistil-wallpaper-renderer/README.md) |
| bloom links the pistil Noto Sans text renderer | 2/2 | ✅ | [View Details](bloom-links-the-pistil-noto-sans-text-renderer/README.md) |
| libpistil exports the vector renderer | 4/4 | ✅ | [View Details](libpistil-exports-the-vector-renderer/README.md) |
| bloom compositor remains observable after startup | 3/3 | ✅ | [View Details](bloom-compositor-remains-observable-after-startup/README.md) |
| bloom compositor reacts to wallpaper watch path | 5/5 | ✅ | [View Details](bloom-compositor-reacts-to-wallpaper-watch-path/README.md) |
| bloom uses the wallpaper watch without steady-state polling | 6/6 | ✅ | [View Details](bloom-uses-the-wallpaper-watch-without-steady-state-polling/README.md) |
| bloom service loop starts | 2/2 | ✅ | [View Details](bloom-service-loop-starts/README.md) |
| bloom service loop processes multiple wallpaper events | 7/7 | ✅ | [View Details](bloom-service-loop-processes-multiple-wallpaper-events/README.md) |
| wallpaper reload happens after the service loop is live | 5/5 | ✅ | [View Details](wallpaper-reload-happens-after-the-service-loop-is-live/README.md) |
| bloom display driver presents a unified device interface | 2/2 | ✅ | [View Details](bloom-display-driver-presents-a-unified-device-interface/README.md) |
| virtio GPU driver accepts display device calls via VFS | 3/3 | ✅ | [View Details](virtio-gpu-driver-accepts-display-device-calls-via-vfs/README.md) |
| bloom service loop paints the first frame without client connections | 6/6 | ✅ | [View Details](bloom-service-loop-paints-the-first-frame-without-client-connections/README.md) |
| boot framebuffer driver paints the first compositor commit | 5/5 | ✅ | [View Details](boot-framebuffer-driver-paints-the-first-compositor-commit/README.md) |
| bloom first frame includes the cached cursor plane | 4/4 | ✅ | [View Details](bloom-first-frame-includes-the-cached-cursor-plane/README.md) |
| bloom first frame leaves the pointer debug overlay off by default | 3/3 | ✅ | [View Details](bloom-first-frame-leaves-the-pointer-debug-overlay-off-by-default/README.md) |
| Alt F7 toggles the pointer debug overlay | 8/8 | ✅ | [View Details](alt-f7-toggles-the-pointer-debug-overlay/README.md) |
| enabled pointer debug overlay includes the cursor svg | 1/2 | ❌ | [View Details](enabled-pointer-debug-overlay-includes-the-cursor-svg/README.md) |
| pointer debug overlay updates after mouse movement | 1/2 | ❌ | [View Details](pointer-debug-overlay-updates-after-mouse-movement/README.md) |
| pointer movement commits bounded damage | 1/2 | ❌ | [View Details](pointer-movement-commits-bounded-damage/README.md) |
| delayed pointer samples animate the visible cursor toward the latest position | 1/2 | ❌ | [View Details](delayed-pointer-samples-animate-the-visible-cursor-toward-the-latest-position/README.md) |
| failed wallpaper decode leaves previous wallpaper active | 5/5 | ✅ | [View Details](failed-wallpaper-decode-leaves-previous-wallpaper-active/README.md) |
| display driver reports VBLANK capability | 3/3 | ✅ | [View Details](display-driver-reports-vblank-capability/README.md) |
| pointer motion events are coalesced to latest-per-frame | 1/2 | ❌ | [View Details](pointer-motion-events-are-coalesced-to-latest-per-frame/README.md) |
