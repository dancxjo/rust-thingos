# Mouse Input Migration: Legacy Port Mechanics → VFS-First Message Path

## Summary

This document describes the migration of the PS/2 mouse input path from legacy
port/channel mechanics to the canonical VFS-first message-passing system.

---

## Old Flow (Legacy Port Mechanics)

```
[Hardware PS/2]
      │ IRQ12 / polling
      ▼
[ps2_mouse driver]
  channel_send_all(write_handle, raw_bytes)
      │ raw ChannelHandle integer passed as boot arg
      ▼
[Bristle HID broker]
  ws.add_port_readable(mouse_read as u64)   ← legacy port wait
  channel_recv(handle, &mut buf)            ← raw byte blob recv
      │ channel_send_all to consumers
      ▼
[bloom / input_echo / other consumers]
```

**Legacy dependencies in the mouse path:**
- `channel_send_all` in `drivers/ps2_mouse/src/main.rs` — raw port send
- `add_port_readable(mouse_read as u64)` in `userspace/bristle/src/main.rs` —
  waits on a raw kernel port handle rather than a VFS file descriptor
- `channel_recv(handle, ...)` in `bristle` for mouse data — raw byte blob recv

---

## New Flow (VFS-First Message Path)

```
[Hardware PS/2]
      │ IRQ12 / polling
      ▼
[ps2_mouse driver]
  vfs_fd_from_handle(write_handle) → write_fd
  vfs_write(write_fd, event_bytes)          ← VFS write
      │ same channel, now via VFS layer
      ▼
[Bristle HID broker]
  vfs_fd_from_handle(mouse_read) → mouse_fd
  ws.add_fd_readable(mouse_fd)              ← FD-based readiness
  vfs_read(mouse_fd, &mut buf)              ← VFS read
      │ forwarding to consumers (unchanged in this migration)
      ▼
[bloom / input_echo / other consumers]
```

**Changed in this migration:**
- `drivers/ps2_mouse/src/main.rs`:
  - Replaced `channel_send_all(handle, bytes)` with `vfs_write(fd, bytes)`.
  - The write channel handle is bridged to a VFS FD via `vfs_fd_from_handle`
    immediately after startup; all subsequent I/O flows through the FD.
- `userspace/bristle/src/main.rs` (mouse path only):
  - `add_port_readable(mouse_read as u64)` → `add_fd_readable(mouse_fd)`.
  - `channel_recv(handle, buf)` → `vfs_read(mouse_fd, buf)`.
  - Mouse read handle is bridged to a VFS FD via `vfs_fd_from_handle` at
    startup; keyboard path is **not** changed in this task.
- `abi/src/hid/pointer.rs`:
  - Added `MouseButton` enum and `MouseMessage` typed enum to document the
    canonical logical mouse event shape for the new message path.

---

## Legacy Dependencies Removed

| Location | Legacy API Removed | Replacement |
|---|---|---|
| `drivers/ps2_mouse/src/main.rs` | `channel_send_all` | `vfs_write` on a channel FD |
| `userspace/bristle/src/main.rs` | `add_port_readable` (mouse) | `add_fd_readable` (mouse FD) |
| `userspace/bristle/src/main.rs` | `channel_recv` (mouse) | `vfs_read` (mouse FD) |

---

## Remaining Compatibility Residue

The following items were intentionally **not** changed in this task:

- **Bristle keyboard path**: keyboard still uses `add_port_readable` +
  `channel_recv`. Migration follows the same pattern as mouse.
- **Bristle output path**: events are still forwarded to `bloom_evt_write` and
  `evt_input_echo_write` via `channel_send_all`. This is shared with the
  keyboard path and should be migrated as a separate task.
- **Sprout channel setup**: `sprout` still creates the `mouse_raw` channel with
  `channel_create` and passes the write handle as an integer boot arg to
  `ps2_mouse`, and the read handle as part of a packed u64 to `bristle`. This
  could be further improved by using `channel_create_fds` + `spawn_process_ex`
  with FD inheritance, but is not required for the semantic migration.

---

## Canonical Mouse Event Shape

The `abi::hid::MouseMessage` enum documents the typed, logical view of mouse
events used throughout the new message path:

```rust
pub enum MouseMessage {
    /// Relative pointer movement.
    Move { dx: i16, dy: i16 },
    /// Button press (`pressed = true`) or release (`pressed = false`).
    Button { button: MouseButton, pressed: bool },
    /// Wheel / trackpad scroll.
    Scroll { dx: i16, dy: i16 },
}
```

The wire encoding remains the `BristleEventHeader` + `PointerMovePayload` /
`PointerButtonPayload` / `ScrollPayload` structs (unchanged). `MouseMessage`
is the decoded form that consumers should pattern-match on.

---

## Follow-Up Tasks

- Migrate **keyboard** input path using the same FD bridge pattern.
- Migrate **bristle output** path (bloom_evt_write, evt_input_echo_write) to
  FD-based writes once downstream consumers are updated.
- Consider migrating **sprout channel setup** to `channel_create_fds` +
  `spawn_process_ex` FD inheritance to remove the last handle-as-integer
  startup plumbing.
- Explore publishing mouse events at a VFS path (e.g. `/dev/input/mouse0`) to
  enable direct open/read by any consumer without going through Bristle.
