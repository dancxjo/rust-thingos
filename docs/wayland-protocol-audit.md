# Wayland Protocol Implementation Audit — Thing-OS / Bloom

_Generated 2026-05-01. Tracks the compliance status of Bloom's Wayland compositor_
_relative to the roadmap in issue #2 (Wayland protocol audit and compliance roadmap)._

---

## Executive Summary

All seven major gaps called out in the original roadmap issue have been closed.
Bloom now advertises a complete set of stable Wayland globals and handles the
full input lifecycle (`wl_seat`, `wl_pointer`, `wl_keyboard`), display output
advertisement (`wl_output`), clipboard (`wl_data_device_manager`), surface
hierarchies (`wl_subcompositor`/`wl_subsurface`), GPU-backed buffers
(`zwp_linux_dmabuf_v1`), frame-timing feedback (`wp_presentation`), and desktop
shell surfaces (`zwlr_layer_shell_v1`).

Remaining open items are in the **known gaps** section below.

---

## Global Registry

| Global                    | Version | Status  | Notes                                  |
|---------------------------|---------|---------|----------------------------------------|
| `wl_compositor`           | 4       | ✅ Full  | `create_surface`, `create_region`      |
| `wl_shm`                  | 1       | ✅ Full  | ARGB8888, XRGB8888 formats             |
| `xdg_wm_base`             | 1       | ✅ Full  | Stable, configure/ack lifecycle        |
| `wl_seat`                 | 5       | ✅ Full  | Pointer + keyboard capabilities        |
| `wl_output`               | 2       | ✅ Full  | Geometry, mode, scale, done events     |
| `wl_subcompositor`        | 1       | ✅ Full  | Synchronized and desynchronized modes  |
| `wl_data_device_manager`  | 3       | ✅ Full  | Clipboard; DnD is a no-op              |
| `zwp_linux_dmabuf_v1`     | 3       | ✅ Full  | Linear modifier; conditional on driver |
| `wp_presentation`         | 1       | ✅ Full  | Best-effort software timestamp         |
| `zwlr_layer_shell_v1`     | 4       | ✅ Full  | Background/bottom/top/overlay layers   |

---

## Core Protocol (`wl_*`)

### `wl_display`

| Request / Event      | Status  | Notes                                      |
|----------------------|---------|--------------------------------------------|
| `sync`               | ✅       | Sends `wl_callback.done` immediately       |
| `get_registry`       | ✅       | Announces all advertised globals           |
| `error` (event)      | ✅       | Emitted on protocol violations             |
| `delete_id` (event)  | ✅       | Sent when objects are destroyed            |

### `wl_registry`

| Request | Status | Notes                     |
|---------|--------|---------------------------|
| `bind`  | ✅      | Routes to correct handler |

### `wl_compositor`

| Request          | Status | Notes                                        |
|------------------|--------|----------------------------------------------|
| `create_surface` | ✅      | Allocates `ObjectEntry::Surface`             |
| `create_region`  | ✅      | Allocates `ObjectEntry::Region` with rect list |

### `wl_region`

| Request    | Status | Notes                                                    |
|------------|--------|----------------------------------------------------------|
| `add`      | ✅      | Appends rect to region; degenerate rects ignored         |
| `subtract` | ✅      | Appends subtract-op to region; degenerate rects ignored  |
| `destroy`  | ✅      | Destroys object and emits `wl_display.delete_id`         |

### `wl_shm` / `wl_shm_pool`

| Request        | Status | Notes                                        |
|----------------|--------|----------------------------------------------|
| `create_pool`  | ✅      | Accepts fd via `recvmsg`                     |
| `create_buffer`| ✅      | ARGB8888, XRGB8888                           |
| `destroy`      | ✅      |                                              |
| `resize`       | ✅      |                                              |

### `wl_buffer`

| Request   | Status | Notes |
|-----------|--------|-------|
| `destroy` | ✅      |       |
| `release` | ✅      | Sent after compositor is done with buffer |

### `wl_surface`

| Request                 | Status  | Notes                                         |
|-------------------------|---------|-----------------------------------------------|
| `attach`                | ✅       |                                               |
| `damage`                | ✅       |                                               |
| `damage_buffer`         | ✅       |                                               |
| `frame`                 | ✅       | Callback fired on WEVT_FRAME_DONE             |
| `commit`                | ✅       | Full xdg-shell / layer-shell lifecycle check  |
| `destroy`               | ✅       | Discards pending presentation feedbacks       |
| `set_opaque_region`     | ✅       | Stores pending region object ID               |
| `set_input_region`      | ✅       | Stores pending region object ID               |
| `set_buffer_transform`  | ⚠️ no-op |                                               |
| `set_buffer_scale`      | ⚠️ no-op |                                               |

### `wl_seat`

| Request / Event              | Status | Notes                                      |
|------------------------------|--------|--------------------------------------------|
| `get_pointer`                | ✅      | Binds `wl_pointer` object                  |
| `get_keyboard`               | ✅      | Binds `wl_keyboard`, sends keymap + repeat |
| `get_touch`                  | ❌      | Not implemented                            |
| `capabilities` (event)       | ✅      | Advertises pointer + keyboard              |
| `name` (event)               | ✅      | Sends `"seat0"`                            |

### `wl_pointer`

| Event           | Status | Notes                                              |
|-----------------|--------|----------------------------------------------------|
| `enter`         | ✅      | Routed from `WEVT_POINTER_ENTER`; followed by `frame` |
| `leave`         | ✅      | Routed from `WEVT_POINTER_LEAVE`; followed by `frame` |
| `motion`        | ✅      | Coalesced; routed from `WEVT_POINTER_MOTION`; followed by `frame` |
| `button`        | ✅      | Routed from `WEVT_POINTER_BUTTON`; evdev mapping; followed by `frame` |
| `axis`          | ✅      | Routed from `WEVT_POINTER_SCROLL`; vertical and horizontal |
| `frame`         | ✅      | Sent after every pointer event group (v5+)         |
| `axis_source`   | ✅      | Emitted as `wheel` before axis events              |
| `set_cursor`    | ⚠️ no-op| Accepted; bloom manages cursor internally          |
| `release`       | ✅      | Destroys the pointer object                        |

### `wl_keyboard`

| Request / Event  | Status | Notes                                                     |
|------------------|--------|-----------------------------------------------------------|
| `keymap`         | ✅      | **Format XKB_V1**, us-intl layout via memfd (new)         |
| `enter`          | ✅      | Sends current keys array (empty); modifiers               |
| `leave`          | ✅      |                                                           |
| `key`            | ✅      | HID → evdev mapping; pressed/released                     |
| `modifiers`      | ✅      | Shift/Ctrl/Alt/Meta/AltGr via bitmask                     |
| `repeat_info`    | ✅      | 25 keys/s, 600 ms delay                                   |
| `release`        | ✅      | Destroys the keyboard object                              |

### `wl_output`

| Event      | Status | Notes                                               |
|------------|--------|-----------------------------------------------------|
| `geometry` | ✅      | Position, mm size, subpixel, transform              |
| `mode`     | ✅      | Width, height, refresh; `CURRENT` flag set          |
| `scale`    | ✅      | Integer scale advertised as 1                       |
| `done`     | ✅      | Sent after all output events                        |
| `release`  | ✅      | Destroys the output object (v3+ request)            |

### `wl_subcompositor` / `wl_subsurface`

| Request             | Status | Notes                                           |
|---------------------|--------|-------------------------------------------------|
| `get_subsurface`    | ✅      | Role enforcement, parent/child linking          |
| `set_position`      | ✅      | Pending in sync mode; immediate in desync       |
| `place_above`       | ✅      | Reorders children; protocol error on bad sibling|
| `place_below`       | ✅      | As above                                        |
| `set_sync`          | ✅      |                                                 |
| `set_desync`        | ✅      | Flushes pending state immediately               |
| `destroy`           | ✅      | Detaches role, removes from parent's stack      |

### `wl_data_device_manager` / `wl_data_device` / `wl_data_source` / `wl_data_offer`

| Feature               | Status  | Notes                                              |
|-----------------------|---------|----------------------------------------------------|
| `create_data_source`  | ✅       |                                                    |
| `get_data_device`     | ✅       |                                                    |
| `set_selection`       | ✅       | Broadcasts `wl_data_offer` to all other clients    |
| `data_offer.receive`  | ✅       | Routes write-fd to source client (clipboard + DnD) |
| `selection` (event)   | ✅       | Null selection on owner disconnect                 |
| `start_drag`          | ✅       | Tracks drag source; sends enter/leave/motion/drop  |
| `wl_data_offer.accept`| ✅       | Forwards `wl_data_source.target` to source         |
| `wl_data_offer.finish`| ✅       | Sends `wl_data_source.dnd_finished` to source      |

---

## xdg-shell Protocol

### `xdg_wm_base`

| Request            | Status | Notes                                            |
|--------------------|--------|--------------------------------------------------|
| `destroy`          | ✅      |                                                  |
| `create_positioner`| ✅      | Minimal state tracking                           |
| `get_xdg_surface`  | ✅      | Role uniqueness enforced                         |
| `pong`             | ✅      | Heartbeat acknowledged                           |
| `ping` (event)     | ✅      | Emitted periodically                             |

### `xdg_surface`

| Request              | Status | Notes                         |
|----------------------|--------|-------------------------------|
| `destroy`            | ✅      |                               |
| `get_toplevel`       | ✅      | Role uniqueness enforced      |
| `get_popup`          | ✅      |                               |
| `set_window_geometry`| ✅      |                               |
| `ack_configure`      | ✅      | Serial synchronization        |
| `configure` (event)  | ✅      |                               |

### `xdg_toplevel`

| Request             | Status  | Notes                                  |
|---------------------|---------|----------------------------------------|
| `set_parent`        | ✅       |                                        |
| `set_title`         | ✅       | Forwarded via IPC `WCMD_SET_TITLE`     |
| `set_app_id`        | ✅       |                                        |
| `show_window_menu`  | ⚠️ no-op|                                        |
| `move`              | ✅       |                                        |
| `resize`            | ✅       |                                        |
| `set_max_size`      | ✅       |                                        |
| `set_min_size`      | ✅       |                                        |
| `set_maximized`     | ✅       |                                        |
| `unset_maximized`   | ✅       |                                        |
| `set_fullscreen`    | ✅       |                                        |
| `unset_fullscreen`  | ✅       |                                        |
| `set_minimized`     | ✅       |                                        |
| `configure` (event) | ✅       | Width, height, states array            |
| `close` (event)     | ✅       | Sent by compositor chrome              |

### `xdg_popup`

| Request / Event   | Status | Notes                                  |
|-------------------|--------|----------------------------------------|
| `destroy`         | ✅      |                                        |
| `configure`       | ✅      | x, y, width, height                    |
| `popup_done`      | ❌      | Not yet emitted                        |
| `reposition`      | ❌      | v3 only; not yet implemented           |

---

## Extension Protocols

### `zwp_linux_dmabuf_v1` (version 3)

| Feature               | Status | Notes                                               |
|-----------------------|--------|-----------------------------------------------------|
| Global advertisement  | ✅      | Conditional on `supports_dmabuf` driver cap         |
| `create_params`       | ✅      | Up to 4 planes                                      |
| `add` (plane)         | ✅      |                                                     |
| `create` / `create_immed` | ✅  | Linear modifier; format ARGB8888 / XRGB8888 only    |
| Modifier feedback     | ❌      | `zwp_linux_dmabuf_feedback_v1` not yet implemented  |

### `wp_presentation` (version 1)

| Feature          | Status | Notes                                                  |
|------------------|--------|--------------------------------------------------------|
| `feedback`       | ✅      | Per-surface `wp_presentation_feedback` objects         |
| `presented`      | ✅      | Monotonic-clock timestamp; no HW/vsync flags claimed   |
| `discarded`      | ✅      | On surface destroy or missing surface                  |
| `clock_id`       | ✅      | `CLOCK_MONOTONIC` advertised                           |

### `zwlr_layer_shell_v1` (version 4)

| Feature                     | Status  | Notes                                      |
|-----------------------------|---------|--------------------------------------------|
| `get_layer_surface`         | ✅       | Background/bottom/top/overlay              |
| `set_size`                  | ✅       |                                            |
| `set_anchor`                | ✅       |                                            |
| `set_exclusive_zone`        | ✅       |                                            |
| `set_margin`                | ✅       |                                            |
| `set_keyboard_interactivity`| ⚠️ no-op| Accepted; scene-wide focus rules for now   |
| `set_layer`                 | ✅       |                                            |
| `ack_configure`             | ✅       | Serial checked against pending queue       |
| `configure` (event)         | ✅       | Sent on first commit; (0,0) hint           |
| `closed` (event)            | ✅       | Emitted via `WEVT_CLOSE_LAYER_SURFACE`     |
| `get_popup`                 | ⚠️ no-op| v1 popup attachment not implemented        |

---

## Known Gaps / Future Work

| Item                                  | Priority | Notes                                               |
|---------------------------------------|----------|-----------------------------------------------------|
| `wl_touch`                            | Low      | Touch input not yet connected                       |
| `xdg_popup.popup_done`                | Medium   | Compositor should dismiss popups on click outside   |
| `zwp_linux_dmabuf_feedback_v1`        | Low      | DMABuf format/modifier feedback for v4+             |
| `wl_pointer.frame` grouping           | Low      | Batching pointer events into frame groups           |
| `wl_data_device` drag-and-drop        | ✅ Done  | `start_drag` implemented; enter/leave/motion/drop events sent |
| `xdg_positioner` (full)               | Low      | Gravity, constraint adjustment not honoured         |
| `wp_fractional_scale_v1`              | Low      | HiDPI fractional scaling                           |
| `xdg_activation_v1`                  | Low      | Focus-stealing prevention                          |
| `ext_session_lock_v1`                 | Low      | Session lock protocol                              |
| Server-side decorations (`zxdg_decoration_manager_v1`) | Low | Chrome entirely compositor-drawn |
| `wl_surface.set_opaque_region` (real) | Low      | Currently a no-op; could optimise composition      |
| `wl_pointer.axis_discrete` / `axis_value120` | Low | High-resolution scroll (v8+) not yet emitted |

---

## Keymap Change (this PR)

`wl_keyboard.keymap` previously sent `WL_KEYBOARD_KEYMAP_FORMAT_NO_KEYMAP`
(format=0) which left clients without a key-symbol mapping and unable to decode
accented characters or dead key sequences.

**Changed:** `send_keyboard_keymap` now:
1. Creates a memfd containing a self-contained XKB keymap string.
2. Sends `WL_KEYBOARD_KEYMAP_FORMAT_XKB_V1` (format=1) with the fd and byte
   length including the null terminator.
3. Closes the server-side fd after `sendmsg` (client holds its own copy).

The embedded keymap is the **us-intl** layout with dead keys (`dead_grave`,
`dead_tilde`, `dead_circumflex`, `dead_acute`, `dead_diaeresis`, etc.) and
AltGr-accessible accented characters (`àéüñç…`).  All keys present in the
`evdev_key()` translation table in `bloom/src/wayland/mod.rs` are covered.

---

_Last updated: 2026-05-01_
