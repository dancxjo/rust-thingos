# VirtIO Sound Driver (`virtio_sound`)

This document explains how the VirtIO sound driver is discovered by `devd`,
how it exposes a PCM device through VFS, and how `beeper` can stream piped PCM
data into that device.

## Driver Discovery (`devd` + `THINGOS_DRIVER`)

`devd` catalog/manual mode only treats a binary as driver-capable if the ELF
exports a global symbol named `THINGOS_DRIVER` with type
`DriverDescriptor`.

`virtio_sound` now exports:

- `THINGOS_DRIVER` with:
  - `driver_class = DriverClass::Audio`
  - `probe(dev, out)` matching VirtIO PCI audio devices
  - `start(ctx)` long-lived service body
- (compatibility) legacy `THING_DRIVER_V1` / `thing_driver_entry_v1` exports

At spawn time, `devd` resolves the descriptor start entrypoint symbol and
launches that service for the selected device.

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
