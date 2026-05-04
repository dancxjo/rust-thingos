# Thing-OS Changelog and History

This file records the project story behind the current codebase. It is not a
release-note dump. It highlights architectural pivots, subsystem milestones,
and the design pressure that shaped the repository.

The current repository also contains upstream Rust history, so the native
Thing-OS timeline below is based on commits touching `thingos/`, `xtask/`,
`targets/`, `docs/`, behavior tests, and the top-level project files.

## Unreleased - May 1, 2026

### Current System Shape

- Thing-OS is now a unified Rust fork plus OS workspace. The Rust compiler and
  standard library live at repository root, and Thing-OS `std` work is edited
  directly in `library/` rather than applied from an external patch flow.
- The architecture is documented as typed-world first, with Unix/POSIX, VFS,
  fd, procfs, and protocol surfaces treated as explicit projections.
- The running system is VFS-first in practice: devices, services, processes,
  display, networking, media, and session settings are reachable through paths
  and file descriptors.
- Kernel execution remains scheduler-first. Recent work reduced global
  scheduler lock coupling, added CPU-local diagnostics, refined wait queues,
  hardened reaping, and exposed scheduler/service state through procfs and BDD.
- Long-lived services are converging on inbox-backed `ServiceLoop` actors,
  where inbox control messages are multiplexed with fd, provider, timer, IRQ,
  and child-exit readiness.

### Desktop, Display, and Wayland

- Bloom is the default compositor service. It discovers display devices through
  `/dev/display/cardN`, watches session files under `/session/desktop`, and
  reaches first paint without graph discovery.
- Display drivers expose a stable v1 device-file ABI with `DISPLAY_OP_GET_INFO`,
  buffer import/release, atomic commit, hardware cursor operations, and ACCEL2D
  command batches.
- The boot framebuffer and virtio-gpu display providers support damage regions,
  resource caching, hardware cursor movement, DMABUF import, and capability
  negotiation.
- ACCEL2D was introduced as a shared 2D acceleration model with CPU fallback,
  conformance tests, strict buffer validation, and virgl-backed GPU execution
  for eligible virtio-gpu copy and alpha-blend paths.
- Bloom now emits ACCEL2D batches instead of always doing CPU composition, while
  preserving CPU paths for commands and formats that are not GPU-eligible.
- Wayland support expanded substantially: output discovery, seat/pointer/
  keyboard input, XKB keymaps over memfd, frame callbacks, subcompositors,
  clipboard/data-device setup, drag and drop, xdg-shell toplevels and popups,
  layer-shell surfaces, input/opaque regions, popup dismissal, pointer axis
  events, and z-order/window overlays.
- Desktop behavior is covered by BDD scenarios with serial assertions and
  screenshots for first paint, pointer responsiveness, hardware cursor motion,
  Wayland clients, Alt+Tab cycling, wallpaper/theme watches, and accelerated
  virtio-gpu presentation.

### Drivers, Devices, and Media

- Cambium became the driver orchestrator for matching devices, spawning drivers,
  watching `/sys/devices`, and handling readiness notifications.
- Userland driver coverage includes PCI stub/device discovery, PS/2 keyboard
  and mouse, RTC CMOS, hardware RNG, ATA/AHCI, HDAudio, VirtIO sound, VirtIO
  net, VirtIO GPU/display, RTL8168, and xHCI USB.
- xHCI work now proves command/event-ring progress, slot enable, EP0
  enumeration, mass-storage enumeration, SCSI REQUEST SENSE, and block-device
  publication through BDD.
- USB mass storage is integrated with the block layer, partition scanning, and
  `/dev/block` aliases.
- A FAT16/FAT32 parser library and `fatd` daemon mount read-only USB FAT media
  at `/media/usb`, with parser unit tests and BDD scenarios for mounted files.
- Audio work added early stack setup, VFS-facing PCM/ring paths, virtio-sound
  and HDAudio driver behavior, startup chime behavior, and BDD coverage for
  absent-device and readiness cases.

### Networking, VFS Providers, and Utilities

- `netd`, `virtio_netd`, socket paths, ICMP, DNS, UDP/TCP behavior, and polling
  were pushed toward readiness-driven file-descriptor semantics instead of
  busy waiting.
- HTTPS became a VFS provider path: mounting `/https` lets shell programs read
  remote content through ordinary file operations.
- VFS provider behavior was hardened with layered mounts, provider request
  concurrency fixes, bounded response buffering, `ReadIntoFd`, file attributes,
  and watch semantics.
- The userland tool set grew into a practical shell environment: file tools,
  process tools, networking tools, PCI/USB inspection, terminal demos,
  protocol demos, and Wayland clients.
- Leaf was added as a terminal application on top of a shared `terminal_core`
  library.

### Rust, Platform, and Build

- The platform contract was clarified: kernel and intentional PAL crates remain
  `no_std`; non-kernel userspace, drivers, services, tests, and tools may use
  Thing-OS `std`.
- The `std` implementation and `stem::pal` boundary are documented as the place
  to add platform capabilities instead of scattering app-local workarounds.
- `rustc-thingos` builds and caches a Linux-hosted cross compiler by default.
  Thing-OS-native rustc/cargo staging is opt-in and gated by cache validation.
- Dynamic linking for x86_64 Thing-OS userspace is wired far enough to stage
  shared libraries such as `libstd.so` and `libpistil.so`.
- `xtask` owns ISO/HDD image building, QEMU launch, BDD, freeze hunting,
  platform audits, Rust toolchain caching, OVMF/Limine fetching, and schema
  generation checks.

### Behavior Testing and Documentation

- BDD was migrated toward native async execution, JSON reporting, screenshot
  artifacts, QMP-driven input, and feature-level coverage for observable
  behavior.
- Behavior specs now cover Bloom, Wayland, graphics orchestration, shell
  pipelines, ServiceLoop, scheduler diagnostics, boot progress, reaping, USB,
  FAT, PCI, audio, entropy, networking, shutdown, and debug log levels.
- Documentation was reorganized around architecture, concepts, IPC, kernel
  contracts, components, build status, migration notes, and audits.

## Current Repository Evolution

### April 14, 2026 - Import and Typed-World Baseline

The current repository history begins its Thing-OS-specific arc by importing
core OS code into the Rust tree:

- `0bcf6920d06` initialized the `thingos` crate with core definitions.
- `6ed0164b395` added Limine bootloader setup.
- `f2591f12f0d` established core kernel architecture, ABI definitions, and
  initial userspace utilities.
- `039a41d58fb` introduced `kindc`, the schema compiler for canonical Kind
  definitions.

The early post-import work immediately pushed away from "process/thread as the
root model" and toward explicit concepts:

- `Task` and `TaskState` became canonical vocabulary.
- `JobExit`, `JobWaitResult`, `Group`, `Authority`, `Place`, and `Space`
  bridges were introduced.
- Unix-shaped fields were quarantined under explicit compatibility boundaries
  such as `ProcessUnixCompat`.
- Job-exit delivery was routed through Message/Inbox paths.

This period established the main architectural theme: Unix compatibility is
useful, but it should not be allowed to own new meaning.

### April 15-16, 2026 - Guardrails, Projection Docs, and Platform Boundaries

The project then made the typed-world model reviewable:

- `docs/architecture/ontology.md` became the canonical ontology reference.
- `docs/architecture/unix-projection.md` explained how Unix concepts project
  from typed-world owners.
- `docs/architecture/how-to-think-in-thingos.md` gave contributors a short
  review model.
- `docs/architecture/vfs-as-place-projection.md` clarified that VFS is a
  practical Place projection, not the whole ontology.
- Guardrail and schema drift checks were added through `just`, CI, and xtask.

At the same time, the platform layer became more pragmatic:

- Thing-OS `std` was allowed for non-kernel code.
- `stem::pal` remained the explicit low-level/no_std boundary.
- POSIX crate-porting guidance was documented for `cfg(unix)` ecosystem crates.
- Baseline pthread, TLS, dynamic linking, and shared-library staging work began.

### April 16-19, 2026 - From Conceptual Kernel To Usable System Surface

The next wave focused on making the system useful:

- Many traditional utilities were added or wired into the image.
- Driver binaries were separated into `/drivers`.
- `Seed`/driver entry descriptors replaced earlier motor naming.
- Pistil moved from a font/rendering service shape toward shared rendering
  libraries and dynamic loading.
- Bloom was split into clearer compositor modules.
- Sprout began orchestrating a dynamic graphics stack.
- Cambium became the driver orchestrator.
- IPC/channel terminology was cleaned up toward ports and handles.
- Networking grew ICMP, DNS, TCP/UDP improvements, socket polling, net device
  VFS publication, and the HTTPS VFS provider.
- Scheduler work attacked real boot-time freezes and stalls by reducing global
  critical sections, bounding scans, cleaning wait queues, and deferring work
  out of hot locks.

This is the period where the project became visibly VFS-first in practice:
networking, providers, drivers, display, shell tools, and tests began composing
through paths, fds, poll readiness, and mounted namespaces.

### April 20-26, 2026 - Desktop Bring-Up and Runtime Hardening

Work shifted toward the graphical session and boot stability:

- Bloom first-paint behavior became a tested contract.
- Session files under `/session/desktop` became the control surface for theme,
  wallpaper, background, and desktop state.
- Bristle and PS/2 input paths fed normalized HID events to the compositor.
- Display commit paths gained fixed-size planes, damage rectangles, alpha
  blending, buffer management, and bootfb/virtio-gpu parity work.
- Audio stack initialization and VFS-facing audio diagnostics were added.
- VFS page cache support improved executable loading.
- ServiceLoop and supervisor behavior became visible through diagnostics and
  behavior tests.

The important design shift was that the desktop was no longer a graph-discovery
demo. It became a set of services bound by VFS paths, device fds, shared
libraries, and session files.

### April 27-29, 2026 - Input, Watchers, and Testable Desktop Behavior

The system then moved from "it paints" toward "it behaves":

- Pointer debug overlays, QMP input automation, mouse movement assertions, and
  screenshot artifacts made display/input behavior testable.
- Low-level keyboard hotkeys such as Ctrl+Alt+Del and Alt+F7 were added to the
  behavior suite.
- Wayland demo clients were spawned by the supervisor.
- Compositor behavior documentation moved toward user-visible scenarios instead
  of purely implementation logs.
- Voluntary-yield task migration and cross-CPU synchronization bugs were fixed.

This phase made BDD a practical design tool for graphical behavior rather than
only a boot smoke-test harness.

### April 30, 2026 - Wayland and Compositor Maturity

April 30 was a large Wayland/compositor push:

- Bloom gained Wayland seat, pointer, keyboard, output, subcompositor,
  subsurface, data-device, clipboard/drag-and-drop base, xdg shell, popup, and
  layer-shell behavior.
- Window management gained title handling, chrome buttons, dragging, resizing,
  fullscreen/shade toggles, shadows, rounded clipping, and surface cleanup.
- Display pacing and cursor responsiveness improved through hardware cursor
  support, cursor DMA, smarter damage coalescing, opaque-region culling,
  direct-present detection, and removal of frame-count smoothing.
- `clock`, `hello`, and desktop orchestration scenarios exercised the
  client-visible protocol surface.
- TCP/UDP, RTC, entropy, PCI snapshots, and scheduler/service diagnostics
  continued to improve alongside the desktop stack.

The project was no longer just proving that a framebuffer could be drawn. It
was building a small but real desktop protocol environment.

### May 1, 2026 - Acceleration, USB/FAT, and Behavior Consolidation

The May 1 history is dominated by acceleration, storage, and broader behavior
coverage:

- Display ABI capability negotiation landed.
- ACCEL2D was defined as a command model with CPU fallback.
- `accel2d_cpu` added conformance tests and later SIMD/tiled optimization.
- `display_virtio_gpu` gained GPU-backed plane composition, alpha blending, and
  virgl-backed ACCEL2D execution.
- Bloom began emitting ACCEL2D batches for composition.
- xHCI gained command/event, EP0, mass-storage, SCSI, block-layer, partition,
  and USB FAT coverage.
- `fat` and `fatd` landed with FAT16/FAT32 read-only media support.
- Wayland grew input regions, opaque regions, pointer axis events, layer-surface
  close events, drag-and-drop start paths, and richer xdg-positioner behavior.
- Leaf and terminal-core work added a real terminal app path.
- The BDD system gained JSON reporting, screenshots, QMP input, and rewritten
  behavior documentation for the actual implemented system.

## Older Heritage Notes

The previous changelog described a longer lineage through earlier projects
named `basic-os` and `janix`. That older history is useful context, but it is
not directly verifiable from the current repository's Git object graph in the
same way as the April-May 2026 timeline above.

Kept as heritage context:

- Early experiments explored graph-native OS ideas, memory-backed object graphs,
  Wasm/ELF execution, human-readable object naming, and UI/widget prototypes.
- Later work moved graphics and drivers out of the kernel, experimented with
  capability-style authority, and developed desktop/runtime concepts.
- A multi-architecture foundation grew around Limine, BDD tests, and x86_64,
  aarch64, riscv64, and loongarch64 target bring-up.
- The graph model reached a high-water mark with Root, GQL-style introspection,
  and 128-bit `ThingId` concepts.
- The decisive pivot was away from graph discovery as the boot/runtime control
  plane and toward a VFS-first system surface. In the current codebase, that
  means core bring-up must not depend on `UI_CROWN`, graph walks, or ThingId
  lookups.

The modern Thing-OS code still preserves typed-world ambition, but the path
changed: bootable, testable, VFS-first services now carry the system while
typed handles, Kinds, messages, Places, and authority boundaries mature beneath
and beside those projections.

## Design Lessons So Far

- A deep ontology is only helpful when each concept has an owner in code.
- VFS paths are excellent for bootstrapping, testing, and human inspection, but
  they should remain projections rather than identity itself.
- Userland drivers force cleaner ownership, but only if the kernel boundary is
  narrow and the readiness model is disciplined.
- Scheduler bugs become architecture bugs. Lock order, wake routing, reaping,
  and wait queues need tests and telemetry as much as features do.
- A desktop is a system integration test. Display, input, file watching, shared
  memory, dynamic linking, services, and scheduling all show their weaknesses
  when the pointer has to move smoothly.
- BDD has been most valuable when scenarios describe what a user or client can
  observe, while unit tests cover protocol parsers, batching, placement, and
  other internal invariants.
