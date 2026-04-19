# ThingOS

ThingOS is a Rust OS workspace and Rust fork focused on bringing up a practical
system with a custom target, growing `std` support, and a path toward
self-hosting.

## Current status (April 2026)

- **Serial terminal is up**: `sprout` launches an early serial shell on
  `/dev/console`.
- **Shell and utility set are up**: the image includes many `/bin` tools
  (`sh`, `ls`, `cat`, `grep`, `ps`, `top`, `cp`, `mv`, `rm`, `mkdir`, etc.)
  plus additional demo/test utilities under `thingos/utils/`.
- **Custom Rust targets are in use**: build flows target
  `targets/*-unknown-thingos.json` (including `x86_64-unknown-thingos`).
- **Large portion of `std` is implemented for ThingOS**: we are building with
  `-Z build-std=core,alloc,std,panic_abort` and exercising that surface through
  in-tree programs/tests.
- **Std programs compile and run on target**: e.g. `hello_std`,
  `hello_stdio`, and thread/std tests are part of the workspace.
- **Ecosystem std crate experiments are in progress**: including `runa`-style
  terminal/TUI bring-up work.
- **Rust bootstrap works today as host-cross**:
  `cargo xtask rustc-thingos` builds a Linux-hosted stage-1 compiler that
  targets ThingOS.
- **Self-hosting is the goal**: we are actively working toward compiling and
  running `rustc` and `cargo` inside ThingOS itself.
- **Busybox-style shell experiment added**: `just busybox` builds and stages a
  tiny busybox-compatible shell (`armybox` as `/bin/busybox` and `/bin/ash`) to
  validate that workflow.

## Useful commands

- `just iso` — build ISO
- `just run` — boot in QEMU
- `just rustc-thingos` — build/cache the current cross compiler
- `just busybox` — build and boot with the tiny busybox-compatible shell

## Fetching a web page from the shell

ThingOS exposes an HTTPS-backed virtual filesystem at `/https`.

Boot the image and fetch a page directly from the shell:

```sh
just run
# in the ThingOS shell:
cat /https/en.wikipedia.org/wiki/Dormouse
```

How this works at runtime:

- `cat` reads a path under `/https/...`
- `httpsd` translates that path into an HTTPS URL
- `http` opens `/net/tcp/*` sockets via `netd`, performs TLS, and returns body bytes

Notes:

- The first read may take a moment while DNS/TCP/TLS are established.
- Requests include a ThingOS `User-Agent` so sites with bot policies can identify traffic.

## Architecture direction

ThingOS keeps a typed-world architecture as the long-term canonical model, with
Unix-compatible surfaces treated as projection/compatibility layers.

References:

- [`docs/architecture/ontology.md`](docs/architecture/ontology.md)
- [`docs/architecture/unix-projection.md`](docs/architecture/unix-projection.md)
- [`docs/architecture/concept-classification.md`](docs/architecture/concept-classification.md)
- [`docs/concepts/thingos-guardrails.md`](docs/concepts/thingos-guardrails.md)
