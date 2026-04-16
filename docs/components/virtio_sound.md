# VirtIO Sound Driver (`virtio_sound`)

This document explains how the VirtIO sound driver is discovered by `devd`,
how it exposes a PCM device through VFS, and how `beeper` can stream piped PCM
data into that device.

## Driver Discovery (`devd` + `THING_DRIVER_V1`)

`devd` catalog/manual mode only treats a binary as driver-capable if the ELF
exports a global symbol named `THING_DRIVER_V1` with type
`DriverInterfaceV1`.

`virtio_sound` now exports:

- `THING_DRIVER_V1` with:
  - `flags = DRIVER_FLAG_PCI`
  - `vendor_id = 0x1af4` (VirtIO vendor)
  - `class_code = 0x040100` and `class_mask = 0xffff00` (multimedia audio)
- `thing_driver_entry_v1(ctx_ptr, ctx_len)` entrypoint

At spawn time, `devd` passes `DriverEntryCtx` to `thing_driver_entry_v1`. The
driver reads `device_path` from that context and claims the exact device,
instead of relying on fallback probing.

## Runtime VFS Contract

After initialization, `virtio_sound` mounts:

- `/dev/audio/card0/ctl`
- `/dev/audio/card0/out0`

`out0` is the playback PCM stream node:

- `AUDIO_SET_PARAMS` configures stream format/rate/channels.
- `AUDIO_START` transitions playback to running.
- `write()` pushes PCM bytes into the driver ring.
- `poll(POLLOUT)` reports writable space/backpressure.

Internally, the driver dequeues from its software ring and submits TX buffers
to the VirtIO sound queue.

## `beeper` Piped-PCM Behavior

`beeper` now supports two input modes:

1. Piped stdin PCM data (preferred when stdin is readable)
2. Generated tone/chime fallback (when no stdin data is available)

Implementation behavior:

- It polls fd `0` with `POLLIN` and a zero timeout.
- If readable, it drains stdin bytes and streams them to
  `/dev/audio/card0/out0`.
- If not readable, it generates its built-in waveform/chime and streams that.

## Quick Usage

Start the driver through `devd`:

```sh
devd virtio_sound
```

Play generated chime:

```sh
beeper
```

Play raw piped PCM (S16LE, stereo, 44100 Hz expected by current beeper path):

```sh
cat sample.raw | beeper
```

## Relevant Source Files

- `drivers/virtio_sound/src/main.rs`
- `drivers/beeper/src/main.rs`
- `abi/src/driver_interface.rs`
- `drivers/devd/src/catalog.rs`
