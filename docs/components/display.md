# Display Driver Contract

This document specifies the stable **v1** buffer and command semantics for
Thing-OS display drivers, covering both `display_bootfb` and
`display_virtio_gpu`.  Bloom and any future compositor must follow these rules
to produce correct output.

## Overview

Display drivers expose a device-file interface under `/dev/display/cardN`.
Clients interact via `DISPLAY_OP_*` ioctls:

| Operation | Purpose |
|---|---|
| `DISPLAY_OP_GET_INFO` | Query display resolution, capabilities, and supported formats |
| `DISPLAY_OP_IMPORT_BUFFER` | Map a client `memfd` into the driver as a `BufferId` |
| `DISPLAY_OP_RELEASE_BUFFER` | Release a previously imported buffer |
| `DISPLAY_OP_COMMIT` | Atomic plane-list present |
| `DISPLAY_OP_ACCEL2D` | Batch 2D acceleration commands |
| `DISPLAY_OP_SET_CURSOR` | Upload and configure the hardware cursor image |
| `DISPLAY_OP_MOVE_CURSOR` | Reposition the hardware cursor |

---

## v1 Buffer Contract

The following rules are **stable ABI**.  All current drivers enforce them and
return deterministic errors for violations — no silent pixel clipping, no
undefined behaviour.

### 1. Destination buffer (`BufferId`)

- **Only `BufferId(0)` is writable** in the current implementation.
  `BufferId(0)` refers to the driver-owned output framebuffer (the frame-pool
  slot currently being composited into).
- Any ACCEL2D command that names a non-zero destination buffer ID returns
  `EINVAL`.
- Imported client buffers (`BufferId > 0`) are **read-only** sources in v1.

### 2. Source buffer pixel format

- Accepted formats: `Bgra8888` and `Bgrx8888` (4 bytes per pixel, linear
  in-memory BGRA order; as a little-endian `u32` this is `0xAARRGGBB`).
- Any other format (e.g. `Rgb565`, unknown formats) causes the command to
  return `EINVAL`.
- The driver validates the format when the buffer is imported
  (`DISPLAY_OP_IMPORT_BUFFER`); ACCEL2D handlers re-validate at dispatch time
  to guard against future format-agnostic import paths.

### 3. Stride

- `stride >= width × 4` must hold for all imported buffers.
- The driver validates this at import time.
- Padded strides (e.g. alignment to 64 bytes) are accepted.
- A stride smaller than `width × 4` causes import to fail with `EINVAL`.

### 4. Memory layout (modifier)

- Only **linear** layout (`modifier = 0`) is accepted in v1.
- Hardware-tiled or compressed layouts are rejected at import time with
  `EINVAL`.

### 5. `DisplayBufferDesc`

The `abi::display::types::DisplayBufferDesc` struct encodes rules 2–4 in a
single validatable descriptor.  Use `DisplayBufferDesc::is_valid_accel2d_src()`
to pre-check a descriptor before submitting commands.

---

## ACCEL2D Command Semantics

Batch submitted via `DISPLAY_OP_ACCEL2D`.  Each batch contains an
`Accel2dBatch` header followed by up to `MAX_ACCEL2D_BATCH_CMDS` (4096)
`Accel2dCommand` entries.

### Command summary

| Command | Description | Dst buffer | Src format |
|---|---|---|---|
| `ACCEL2D_CMD_CLEAR_RECT` | Fill rect with solid colour | `BufferId(0)` only | — |
| `ACCEL2D_CMD_COPY_RECT` | Pixel-exact rect copy (no scale) | `BufferId(0)` only | 4 bpp |
| `ACCEL2D_CMD_STRETCH_BLIT` | Scaled copy (nearest-neighbour) | `BufferId(0)` only | 4 bpp |
| `ACCEL2D_CMD_ALPHA_BLIT` | Over-blend with global alpha | `BufferId(0)` only | 4 bpp |
| `ACCEL2D_CMD_MASKED_BLIT` | Over-blend with per-pixel mask | `BufferId(0)` only | 4 bpp (src + mask) |
| `ACCEL2D_CMD_ROUNDED_CLIP_BLIT` | Blit clipped to rounded rect | `BufferId(0)` only | 4 bpp |
| `ACCEL2D_CMD_FLUSH_DAMAGE` | Present dirty regions to display | — | — |

### Error codes

| Condition | Error |
|---|---|
| `dst_buffer != BufferId(0)` | `EINVAL` — non-zero destination is invalid in v1 |
| Source buffer not found | `ENOENT` — buffer was never imported or already released |
| Source format not 4 bpp | `EINVAL` — explicit format contract violation |
| Frame pool empty | `ENXIO` — driver not yet initialized |
| GPU transfer failure | `EIO` |
| Unknown command kind | `ENOSYS` |

---

## Capabilities (`DisplayCaps`)

Query via `DISPLAY_OP_GET_INFO`.  Relevant capability flags:

| Flag | Meaning |
|---|---|
| `ACCEL2D_CLEAR` | Driver implements `ACCEL2D_CMD_CLEAR_RECT` |
| `ACCEL2D_COPY` | Driver implements `ACCEL2D_CMD_COPY_RECT` |
| `ACCEL2D_STRETCH` | Driver implements `ACCEL2D_CMD_STRETCH_BLIT` |
| `ACCEL2D_ALPHA_BLIT` | Driver implements `ACCEL2D_CMD_ALPHA_BLIT` |
| `ACCEL2D_MASKED_BLIT` | Driver implements `ACCEL2D_CMD_MASKED_BLIT` |
| `ACCEL2D_ROUNDED_CLIP_BLIT` | Driver implements `ACCEL2D_CMD_ROUNDED_CLIP_BLIT` |
| `ACCEL2D_FLUSH_DAMAGE` | Driver processes `ACCEL2D_CMD_FLUSH_DAMAGE` |
| `GPU_ALPHA_BLEND` | Driver uses GPU hardware for alpha blending (virgl path) |

---

## Future Direction: Multi-Buffer GPU Path

The v1 rules are intentionally conservative.  The following extensions are
**planned** but not yet implemented:

- **Non-zero destination `BufferId`s**: Off-screen compositing targets for
  GPU-accelerated multi-layer rendering.
- **GPU-native tiling modifiers**: Allow source buffers with hardware-tiled
  layouts (e.g. Intel X-tiled, Arm AFBC) for zero-copy GPU texture upload.
- **Fence-based synchronisation**: Producer/consumer sync fences
  (`DisplayCaps::FENCES`) to replace the current implicit CPU barrier.
- **Multi-plane formats**: YUV/YCbCr source buffers for video compositing.

When these extensions land, the `DisplayBufferDesc` struct will be extended
and the `is_valid_accel2d_src()` predicate will be updated to reflect the
new rules.  Drivers that implement multi-buffer support must set the
appropriate `DisplayCaps` bits to signal the extended contract to clients.

---

## Bloom Integration Notes

- Bloom must call `DISPLAY_OP_GET_INFO` at startup and check `DisplayCaps`
  before submitting ACCEL2D commands.
- For the CPU-fallback path, Bloom should import all source surfaces as
  `Bgra8888` with linear stride.
- Bloom must not assume `ACCEL2D_CMD_FLUSH_DAMAGE` implies vsync; use
  `DISPLAY_OP_COMMIT` with `CommitFlags::VSYNC` for synchronised presentation.
