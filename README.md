# Thing-OS

Thing-OS is a Rust operating-system workspace and Rust compiler fork. It builds
a bootable kernel, a VFS-first userspace, userland hardware drivers, a
Wayland-capable desktop stack, and a Thing-OS target implementation of Rust
`std` from one repository.

The project is not trying to be "Unix with different names." Its long-term
model is a typed world: system meaning belongs to first-class concepts such as
`Thing`, `Kind`, `Form`, `Place`, `Authority`, `Presence`, `Task`, `Job`,
`Group`, `Space`, and `Message`. Unix and POSIX shapes still matter because
they make the system usable and portable, but they are projections over that
model, not the root ontology.

The practical rule is:

```text
typed-world truth first; Unix, VFS, fd, and protocol projections second
```

At the same time, Thing-OS is deliberately pragmatic. The running system boots,
spawns services, mounts providers, launches drivers, paints a desktop, speaks a
useful subset of Wayland, runs shell utilities, and exercises the platform with
BDD scenarios. The architecture is only useful if it keeps producing a system
that can boot and be tested.

## Current Snapshot

As of May 1, 2026:

- The repository is the source of truth for both Thing-OS and its Rust fork.
  `compiler/` and `library/` live at the root, and Thing-OS `std` support is
  implemented in-tree rather than carried as an external patch set.
- `just iso` builds bootable images for Thing-OS targets. `just run` boots the
  default x86_64 system in QEMU.
- The kernel remains `no_std`, scheduler-first, and spawn-plus-exec based.
  There is intentionally no `fork` syscall.
- Userspace, drivers, services, tests, and tools may use Thing-OS `std` where
  that improves correctness or maintainability.
- Display, input, storage, networking, audio, service discovery, and session
  state are exposed through mounted files, device files, service paths, and
  file descriptors.
- Hardware drivers are userland programs. The kernel exposes controlled device
  primitives such as MMIO mapping, DMA allocation, I/O port access, and IRQ
  waiting.
- `sprout` supervises system bring-up, `cambium` orchestrates drivers,
  `bristle` normalizes input, and `bloom` composes the desktop.
- Bloom reaches first paint from VFS/device/session state. It must not depend
  on graph discovery, `UI_CROWN`, or `ThingId` lookup for core display bring-up.
- The display path includes `/dev/display/cardN`, boot framebuffer and
  virtio-gpu providers, hardware cursor support, damage-based presentation,
  DMABUF import, ACCEL2D command batches, CPU fallback, and virgl-backed GPU
  acceleration for eligible operations.
- The Wayland surface is actively growing: `wl_output`, seat/pointer/keyboard,
  XKB keymap memfd, `wl_subcompositor`, `wl_data_device_manager`, xdg-shell
  toplevel/popup behavior, layer shell, regions, frame callbacks, drag and drop,
  and client-visible desktop scenarios are covered.
- USB xHCI and mass-storage work can enumerate devices, expose block media,
  scan partitions, and mount read-only FAT16/FAT32 media through `fatd`.
- The image includes a useful shell and many small utilities: `sh`, `ls`, `cat`,
  `grep`, `ps`, `top`, `cp`, `mv`, `rm`, `mkdir`, `mount`, `ping`, `nslookup`,
  `lspci`, `lsusb`, `leaf`, `wayland_hello`, and more under `thingos/utils/`.
- Behavior coverage is first-class. Gherkin features in
  `docs/behavior/features/` cover compositor behavior, shell pipelines,
  ServiceLoop contracts, scheduler diagnostics, USB/FAT, PCI, audio, networking,
  boot progress, and process reaping.

This is still an experimental system. POSIX compatibility, hosted Rust
toolchain work, multi-architecture parity, and typed-world promotion are active
work, not finished claims.

## The Core Ideas

### Typed World, Practical Projections

Thing-OS design starts with canonical owners:

- `Task` owns schedulable execution.
- `Job` owns lifecycle, exit, wait, and reaping semantics.
- `Space` owns address-space semantics.
- `Authority` owns permissions and capability context.
- `Place` owns visibility, namespace, and where things are reachable.
- `Message` owns typed coordination and event delivery.

Unix-shaped concepts are compatibility views:

- process = `Job + Space + Authority + Place + Task(s)`
- fd = one handle form for a `Thing`
- path = a route through a `Place` projection
- signal = compatibility projection over typed coordination and messages
- session/process group = compatibility projection over `Group` and `Presence`

This distinction matters in code review. New meaning should land in the
canonical owner first, then be projected into VFS, POSIX, procfs, Wayland, or
other compatibility surfaces as needed.

References:

- [`docs/architecture/how-to-think-in-thingos.md`](docs/architecture/how-to-think-in-thingos.md)
- [`docs/architecture/ontology.md`](docs/architecture/ontology.md)
- [`docs/architecture/unix-projection.md`](docs/architecture/unix-projection.md)
- [`docs/architecture/vfs-as-place-projection.md`](docs/architecture/vfs-as-place-projection.md)

### VFS-First, But Not File-Ontology

The running system is VFS-first because paths and file descriptors are the
simplest common substrate for bootstrapping services, drivers, tests, and
portable programs. Device state appears under `/dev`, process state under
`/proc`, service rendezvous under `/services`, runtime state under `/run`, and
session state under `/session`.

That does not mean "the file is the object." A VFS node is one access form for
an underlying Thing in a Place. This keeps the system debuggable and testable
while leaving room for richer typed handles and messages.

### Userland Drivers

Hardware policy lives outside the kernel. Drivers such as `display_virtio_gpu`,
`virtio_netd`, `virtio_sound`, `hdaudio`, `rtc_cmos`, `ps2_kbd`, `ps2_mouse`,
`ata_disk`, `ahci_disk`, `rtl8168d`, and `usb/xhci` are userspace crates under
`thingos/drivers/`.

The kernel provides the narrow boundary: claim a device, map MMIO, allocate DMA,
translate DMA addresses, access I/O ports, subscribe to IRQs, and wait for IRQ
readiness. Drivers publish useful state back through VFS and service paths.

### Scheduler-First Execution

Every unit of execution is a kernel-scheduled task. The scheduler owns CPU time,
run queues, blocking, wakeups, and preemption. Work over April 2026 focused
heavily on SMP correctness, lock-order reduction, wait queue behavior, reaping,
CPU-local diagnostics, anti-starvation behavior, and procfs-visible scheduler
telemetry.

### Spawn Plus Exec

Thing-OS intentionally avoids `fork`. New processes are created with
`SYS_SPAWN_PROCESS[_EX]` and then execute a program with `SYS_TASK_EXEC`.
This fits a system where address spaces, VFS namespaces, driver handles, DMA,
and capabilities should have explicit ownership rather than implicit
copy-on-write inheritance.

### Inbox-Backed Services

Long-running services are moving toward inbox-backed actor loops. Their control
plane is an inbox; file descriptors, IRQs, provider ports, child exits, and
timers are readiness sources multiplexed beside it. The shared implementation
is `stem::service_loop::ServiceLoop`.

This is visible in services such as Cambium and in behavior tests that check
ServiceLoop state under `/proc/<pid>/serviceloop`.

### Rust Is Part Of The OS

Thing-OS carries a Rust compiler and standard-library fork in this repository.
The kernel stays `no_std`, but non-kernel Thing-OS code can use Thing-OS `std`.
When a normal Rust API is missing, the preferred fix is usually in
`library/std/src/sys/pal/thingos/` or in `stem::pal`, not a one-off workaround
inside each app.

The current `rustc-thingos` flow primarily builds a Linux-hosted stage-1 cross
compiler. A Thing-OS-native compiler path exists as an opt-in experiment, but
full in-guest self-hosting is not yet the default validated path.

References:

- [`docs/platform.md`](docs/platform.md)
- [`docs/build/bootstrapping.md`](docs/build/bootstrapping.md)
- [`docs/build/status/rustc_build.md`](docs/build/status/rustc_build.md)
- [`docs/build/status/semantic_self_hosting.md`](docs/build/status/semantic_self_hosting.md)

## Repository Map

The root is a combined Rust and Thing-OS workspace.

- `thingos/abi/` - shared ABI types, syscall numbers, display/audio/HID/VFS
  contracts, schema IDs, and wire formats.
- `thingos/kernel/` - kernel crate and kernel subsystems.
- `thingos/bran/` - boot runtime abstraction between bootloader/architecture
  setup and the kernel.
- `thingos/stem/` - low-level userspace platform library, syscall wrappers,
  PAL helpers, wait sets, and ServiceLoop support.
- `thingos/sprout/` - init and supervisor.
- `thingos/cambium/` - driver orchestrator.
- `thingos/bristle/` - input broker and normalized HID path.
- `thingos/bloom/` - compositor, Wayland bridge, scene, damage, input, and
  display presentation logic.
- `thingos/blossom/` - client-side shell/protocol state helpers.
- `thingos/pistil/` - shared rendering, text, gradient, mask, and visual
  primitives.
- `thingos/drivers/` - userland drivers.
- `thingos/utils/` - shell, utilities, demos, daemons, protocol clients, and
  test programs.
- `thingos/libs/` - shared userspace libraries such as `accel2d_cpu`,
  `terminal_core`, SVG helpers, HTTP, and IPC helpers.
- `compiler/`, `library/`, `src/` - Rust compiler, standard library, bootstrap,
  and Rust tooling sources.
- `targets/` - Thing-OS target JSON specs.
- `xtask/` - build, image, QEMU, BDD, audit, and toolchain orchestration.
- `tools/bdd/` - behavior-test runner and QEMU automation.
- `docs/` - architecture, subsystem docs, behavior specs, status audits, and
  generated behavior reports.
- `assets/` - fonts, cursors, wallpapers, icons, themes, and PCI metadata.
- `vendor/` - vendored dependencies such as firmware and bootloader assets.

## Common Commands

```sh
just iso                         # Build the default x86_64 ISO
KARCH=aarch64 just iso           # Build another architecture
just run                         # Boot the default image in QEMU
just run x86_64 -i               # Boot interactively
just hdd                         # Build an HDD image
just run-hdd                     # Boot the HDD image
just behave                      # Run BDD behavior tests
just behave --arch x86_64        # Run BDD for one architecture
just smoke                       # Run smoke-tagged BDD tests
just audit-platform              # Check platform/no_std boundaries
just kindc-check                 # Check generated Kind schema drift
just rustc-thingos               # Build/cache the Thing-OS Rust toolchain path
just freeze-hunter               # Run QEMU freeze/stress automation
just clean                       # Clean build artifacts and generated images
```

Useful environment knobs:

```sh
RUST_PROFILE=release just iso
QEMUFLAGS="-m 4G -smp 8" just run
SCHED_TELEMETRY=1 just run
SKIP_RUSTC_THINGOS=1 just iso
INCLUDE_RUST_TOOLCHAIN=1 just iso
BUILD_THINGOS_NATIVE_RUSTC=1 just rustc-thingos
```

## In-Guest Examples

Fetch through the HTTPS VFS provider:

```sh
mount -t https none /https
cat /https/example.com/@index
```

Inspect devices and media:

```sh
lspci
lsusb
ls /dev/block
ls /media/usb
cat /media/usb/hello.txt
```

Exercise the desktop and Wayland clients:

```sh
leaf
wayland_hello
clock
echo /share/wallpapers/flower.png > /session/desktop/wallpaper
echo Solarized Warm > /session/desktop/theme
```

## Testing Philosophy

Behavior tests are part of the design process, not a reporting afterthought.
When a feature changes visible behavior, add or update a Gherkin scenario under
`docs/behavior/features/`. Bug fixes should usually follow a fail-first flow:
make the scenario expose the bug, fix it, then rerun the relevant BDD slice.

Lower-level invariants belong in focused unit tests. For example, Bloom damage
and ACCEL2D batching have unit coverage, while BDD asserts user-visible cursor,
presentation, and Wayland behavior. FAT parser logic has host-testable library
coverage, while BDD checks mounted USB media as the user sees it.

References:

- [`docs/behavior/test-inventory.md`](docs/behavior/test-inventory.md)
- [`docs/behavior/features/`](docs/behavior/features/)
- [`tools/bdd/`](tools/bdd/)

## Architecture Guardrails

These rules are the high-friction review points:

1. Scheduler-first: every execution unit is a scheduled task.
2. Userland drivers: hardware logic lives outside the kernel.
3. VFS-first: system resources are reachable through mounted paths.
4. Spawn plus exec: no `fork`.
5. Inbox-backed services: long-lived service control planes use inboxes and
   readiness multiplexing.

Read [`docs/concepts/thingos-guardrails.md`](docs/concepts/thingos-guardrails.md)
before changing kernel, ABI, driver, display, IPC, or process behavior.

## Where To Start

- Architecture mental model:
  [`docs/architecture/how-to-think-in-thingos.md`](docs/architecture/how-to-think-in-thingos.md)
- Guardrails and PR checklist:
  [`docs/concepts/thingos-guardrails.md`](docs/concepts/thingos-guardrails.md)
- Platform layer contract:
  [`docs/platform.md`](docs/platform.md)
- Display/compositor contract:
  [`docs/components/display.md`](docs/components/display.md) and
  [`docs/components/bloom.md`](docs/components/bloom.md)
- IPC and ServiceLoop:
  [`docs/ipc/service_loop.md`](docs/ipc/service_loop.md)
- History:
  [`CHANGELOG.md`](CHANGELOG.md)
