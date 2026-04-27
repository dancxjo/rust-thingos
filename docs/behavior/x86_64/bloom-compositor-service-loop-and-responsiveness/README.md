# Feature: Bloom compositor service loop and responsiveness

> Last run: 2026-04-27 09:53:39

## Scenarios

| Scenario | Steps | Status | Link |
|----------|-------|--------|------|
| bloom compositor service starts and publishes its port | 3/3 | ✅ | [View Details](bloom-compositor-service-starts-and-publishes-its-port/README.md) |
| bloom links the pistil wallpaper renderer | 2/2 | ✅ | [View Details](bloom-links-the-pistil-wallpaper-renderer/README.md) |
| libpistil exports the vector renderer | 4/4 | ✅ | [View Details](libpistil-exports-the-vector-renderer/README.md) |
| bloom compositor remains observable after startup | 3/3 | ✅ | [View Details](bloom-compositor-remains-observable-after-startup/README.md) |
| bloom compositor reacts to wallpaper watch path | 1/2 | ❌ | [View Details](bloom-compositor-reacts-to-wallpaper-watch-path/README.md) |
| bloom service loop starts | 1/2 | ❌ | [View Details](bloom-service-loop-starts/README.md) |
| bloom service loop processes multiple wallpaper events | 1/2 | ❌ | [View Details](bloom-service-loop-processes-multiple-wallpaper-events/README.md) |
| wallpaper reload happens after the service loop is live | 1/2 | ❌ | [View Details](wallpaper-reload-happens-after-the-service-loop-is-live/README.md) |
| bloom display driver presents a unified device interface | 2/2 | ✅ | [View Details](bloom-display-driver-presents-a-unified-device-interface/README.md) |
| virtio GPU driver accepts display device calls via VFS | 3/3 | ✅ | [View Details](virtio-gpu-driver-accepts-display-device-calls-via-vfs/README.md) |
| bloom service loop paints the first frame without client connections | 1/2 | ❌ | [View Details](bloom-service-loop-paints-the-first-frame-without-client-connections/README.md) |
| bloom first frame includes the cached cursor plane | 3/4 | ❌ | [View Details](bloom-first-frame-includes-the-cached-cursor-plane/README.md) |
| bloom first frame includes the pointer debug overlay | 1/2 | ❌ | [View Details](bloom-first-frame-includes-the-pointer-debug-overlay/README.md) |
| pointer debug overlay updates after mouse movement | 2/2 | ✅ | [View Details](pointer-debug-overlay-updates-after-mouse-movement/README.md) |
| failed wallpaper decode leaves previous wallpaper active | 1/2 | ❌ | [View Details](failed-wallpaper-decode-leaves-previous-wallpaper-active/README.md) |
