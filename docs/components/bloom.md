# Bloom

**Bloom** is ThingOS’s compositor service boundary: it turns client surfaces into pixels on `/dev/display/cardX`.

## What Bloom owns

- Display output enumeration and presentation (`/dev/display/card0` in the current implementation)
- Surface registration and lifecycle
- Buffer import/release and commit-time surface state promotion
- Composition order and visibility
- Damage-driven redraw loop (event-driven)
- Input focus and dispatch based on normalized Bristle events
- Compositor-authored visuals (background/cursor/overlays) via shared **pistil** helpers

## What Bloom does not own

- Shell policy (tiling/floating/layout strategy)
- Launcher/panel/wallpaper policy
- Toolkit/widget abstractions
- Input hardware driver logic

## Runtime service interfaces

### 1) Display-facing interface (Bloom → `/dev/display/cardX`)

Bloom uses the display ABI operations:
- `DISPLAY_OP_GET_INFO`
- `DISPLAY_OP_IMPORT_BUFFER`
- `DISPLAY_OP_RELEASE_BUFFER`
- `DISPLAY_OP_COMMIT`

Internal Bloom operations map to:
- `enumerate_outputs() -> [OutputInfo]`
- `import_buffer(handle, metadata) -> BufferId`
- `release_buffer(BufferId)`
- `present(output, composition_list, damage) -> PresentResult`

### 2) Input-facing interface (Bristle → Bloom)

Bloom consumes Bristle’s normalized HID event stream (`BristleEventHeader` + payloads), including:
- pointer move/button
- keyboard press/release (+ modifiers/repeat)

Bloom performs:
- hit testing
- pointer focus transitions (enter/leave/motion)
- keyboard focus assignment on pointer click
- focused event dispatch to the owning surface client

### 3) Client-facing interface (native Bloom protocol)

Bloom publishes a channel handle at:
- `/services/bloom`

Clients connect over channels and use a surface-centric protocol with double-buffered pending/current state.

Client → Bloom requests:
- `CONNECT(reply_channel, event_channel)`
- `CONNECT_INBOX(reply_channel, event_channel, input_pid)`
- `CREATE_SURFACE(reply_channel, client_id)`
- `DESTROY_SURFACE(reply_channel, client_id, surface_id)`
- `ATTACH_BUFFER(reply_channel, client_id, surface_id, handle, width, height, stride, format, modifier)`
- `DAMAGE(reply_channel, client_id, surface_id, rect)`
- `SET_INPUT_REGION(reply_channel, client_id, surface_id, rect)`
- `SET_OPAQUE_REGION(reply_channel, client_id, surface_id, rect)`
- `SET_DEST_RECT(reply_channel, client_id, surface_id, rect)`
- `SET_Z_ORDER(reply_channel, client_id, surface_id, z)`
- `COMMIT(reply_channel, client_id, surface_id)`

Bloom → Client events:
- `ACK(status, value, serial)`
- `FRAME_DONE(surface_id, serial, timestamp_ns)`
- `POINTER_ENTER/LEAVE/MOTION` (typed inbox when `CONNECT_INBOX` is used)
- `POINTER_BUTTON` (typed inbox when `CONNECT_INBOX` is used)
- `KEYBOARD_ENTER/LEAVE` (typed inbox when `CONNECT_INBOX` is used)
- `KEYBOARD_KEY` (typed inbox when `CONNECT_INBOX` is used)

## Internal module split

Bloom is organized around:
- `display.rs` — display card interaction and atomic present path
- `scene.rs` — clients, surfaces, pending/current state, hit testing, composition list
- `protocol.rs` — client protocol wire types and event encoding
- `input.rs` — Bristle ingestion and focus routing
- `render.rs` — compositor visuals and pistil-backed blitting
- `damage.rs` — per-output dirty tracking
- `frame_clock.rs` — frame-pacing clock (readiness-driven; no sleep-based polling)
- `world.rs` — `BloomWorld`: single owner of all mutable compositor state
- `loop_types.rs` — `BloomService` trait, `BloomLoop`, `Interest`, `LoopEvent`, `LoopAction`
- `services/` — focused service implementations:
  - `wayland.rs` — `WaylandService`: native Bloom protocol client dispatch
  - `input_service.rs` — `InputService`: bristle HID event ingestion
  - `wallpaper.rs` — `WallpaperService`: wallpaper watch-path reactions
- `main.rs` — startup, `BloomWorld` construction, service registration, `BloomLoop::run`

## Loop contract

Bloom's compositor loop is **readiness-driven, not sleep-driven**:

```text
loop {
    wait(next_frame_deadline)   ← blocks until FD ready or timeout
    for each ready FD → dispatch to owning BloomService
    poll wallpaper worker (poll_ready_background)
    if frame_clock.repaint_due() && damage.is_dirty() → present + frame callbacks
}
```

Services implement `BloomService` and communicate only through `BloomWorld`
mutations and `LoopAction` return values — never by calling each other directly.

## Pistil boundary

Bloom uses pistil for compositor-owned drawing support only.
Client content is still imported as buffers and composed by Bloom.
