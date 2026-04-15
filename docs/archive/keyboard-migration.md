# Keyboard Input Migration: Legacy Port Mechanics → VFS-First Message Path

## Summary

This document describes the migration of the PS/2 keyboard input path from
legacy port/channel mechanics to the canonical VFS-first message-passing
system.  It follows the same pattern established by the mouse migration
(`docs/archive/mouse-migration.md`).

---

## Old Flow (Legacy Port Mechanics)

```
[Hardware PS/2]
      │ IRQ1 / polling
      ▼
[ps2_kbd driver]
  KeyboardState::process_ps2(scancode) → KeyEdge
  channel_send_all(write_handle, raw_bytes)   ← raw ChannelHandle integer
      │ raw ChannelHandle integer passed as boot arg
      ▼
[Bristle HID broker]
  ws.add_port_readable(kbd_read as u64)       ← legacy port wait
  channel_recv(kbd_read, &mut buf)            ← raw byte blob recv
      │ channel_send_all to consumers
      ▼
[bloom / input_echo / other consumers]
```

**Legacy dependencies in the keyboard path:**
- `channel_send_all` in `drivers/ps2_kbd/src/main.rs` — raw port send
- `add_port_readable(kbd_read as u64)` in `userspace/bristle/src/main.rs`
- `channel_recv(kbd_read, buf)` in `bristle` for keyboard data

---

## New Flow (VFS-First Message Path)

```
[Hardware PS/2]
      │ IRQ1 / polling
      ▼
[ps2_kbd driver]
  KeyboardState::process_ps2(scancode) → KeyEdge
  vfs_fd_from_handle(write_handle) → write_fd
  vfs_write(write_fd, event_bytes)            ← VFS write
      │ same channel, now via VFS layer
      ▼
[Bristle HID broker]
  vfs_fd_from_handle(kbd_read) → kbd_fd
  ws.add_fd_readable(kbd_fd)                  ← FD-based readiness
  vfs_read(kbd_fd, &mut buf)                  ← VFS read
      │ forwarding to consumers (channel_send_all, unchanged in this task)
      ▼
[bloom / input_echo / other consumers]
```

**Changed in this migration:**
- `drivers/ps2_kbd/src/main.rs`:
  - Replaced `channel_send_all(handle, bytes)` with `vfs_write(fd, bytes)`.
  - Write channel handle is bridged to a VFS FD via `vfs_fd_from_handle` at
    startup; all subsequent I/O flows through the FD.
  - Removed `ChannelHandle` and `channel_send_all` imports entirely.
- `userspace/bristle/src/main.rs` (keyboard path):
  - Removed `add_port_readable(kbd_read)` — replaced by `add_fd_readable(kbd_fd)`.
  - Replaced `channel_recv(kbd_read, buf)` with `vfs_read(kbd_fd, buf)`.
  - Removed `channel_recv` from imports.
- `abi/src/hid/bristle.rs` + `abi/src/hid/mod.rs`:
  - Added `KeyboardMessage` typed enum documenting the canonical logical
    keyboard event shape for the new message path.

---

## Physical-Key vs Text Semantics

Thing-OS currently uses a **single unified stream of physical key events**.

Each event carries:
- `Key` — the logical key identity (HID Usage ID equivalent)
- `pressed` — true for key-down, false for key-up
- `modifiers` — current bitmask of shift, ctrl, alt, altgr, meta
- `repeat` — true when the key was already held (auto-repeat)

**Text generation is intentionally left to consumers.** Bristle does not
perform layout translation; applications receive `Key` + `Mods` and apply
their own mapping.  A `Text` variant can be added to `KeyboardMessage` if a
text-production layer is introduced above the physical event stream, but it is
not added now to avoid implying that Bristle translates scancodes to Unicode.

---

## Legacy Dependencies Removed

| Location | Legacy API Removed | Replacement |
|---|---|---|
| `drivers/ps2_kbd/src/main.rs` | `channel_send_all` | `vfs_write` on channel FD |
| `drivers/ps2_kbd/src/main.rs` | `ChannelHandle` import | plain `u32` FD |
| `userspace/bristle/src/main.rs` | `add_port_readable` (kbd) | `add_fd_readable` (kbd FD) |
| `userspace/bristle/src/main.rs` | `channel_recv` (kbd) | `vfs_read` (kbd FD) |

---

## Remaining Compatibility Residue

- **Bristle output path**: events are still forwarded to `bloom_evt_write` and
  `evt_input_echo_write` via `channel_send_all`.  This is shared output and
  should be migrated as a separate task.
- **Sprout channel setup**: `sprout` still creates `kbd_raw` via
  `channel_create` and passes the write handle as an integer boot arg.
  Replacing with `channel_create_fds` + `spawn_process_ex` FD inheritance is
  a follow-up improvement.

---

## Follow-Up Tasks

- Migrate **bristle output** path to FD-based writes once downstream consumers
  are updated.
- Consider `channel_create_fds` + FD inheritance in `sprout` for both
  keyboard and mouse setup.
- Explore publishing keyboard events at a VFS path (e.g.
  `/dev/input/keyboard0`) to allow direct consumer access without Bristle.
- If a text/input abstraction layer is added above Bristle, document it as
  a `Text` variant in `KeyboardMessage` at that time.
