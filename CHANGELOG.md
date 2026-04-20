# Thing-OS Complete Heritage & Changes Log

This document provides a comprehensive history of the Thing-OS project, tracing its lineage through three major development repositories: `basic-os` (2025), `janix` (Early 2026), and the current `thingos` implementation (Mid 2026).

## 🏗️ Architectural Eras

1.  **The Genesis Era (Early – Mid 2025) [`basic-os`]**: Exploration of "graph-native" concepts. Initial POCs centered on memory-backed graph systems (Alocasia/Begonia) and early Wasm/ELF execution environments.
2.  **Userland & Desktop Migration (Late 2025) [`basic-os`]**: The shift of core components (like the compositor) to userspace. Development of the first widget libraries, capability-based security, and early desktop environments.
3.  **The Foundation Era (January 2026) [`janix`]**: Re-implementation for multi-arch stability (x86_64, aarch64, riscv64, loongarch64). Focus on Limine bootloader support and BDD testing frameworks.
4.  **The Universal Graph Era (February – March 2026) [`janix`]**: High-water mark of the "Everything is a Node" philosophy. Introduction of the Root service, 128-bit `ThingId`, and GQL (Graph Query Language).
5.  **The VFS Pivot (Early April 2026) [`janix`]**: Transition to the "Everything is a File" architecture. A massive refactor (Acts III–V) moved system discovery to a Virtual File System, removing 16,000+ lines of graph code.
6.  **The Modern Era (Mid April 2026 – Present) [`thingos`]**: POSIX compliance, system hardening, and toolchain integration. Implementation of POSIX signals, job management, and self-hosting toolchain stability.

---

## 🗓️ Significant Milestones

### Phase 0: Genesis & Core Concepts (Early 2025)
- **POC Graph**: Introduced Alocasia and Begonia graph systems with UUID-based identity.
- **Execution**: Early support for executing WebAssembly (via `wasmi`) and ELF binaries.
- **Memory Persona**: Reversible persona names for human-readable graph navigation.

### Phase 1: Kernel Foundation & Multitasking (Mid 2025)
- **Scheduler**: Transitioned to a full task scheduler with support for kernel and user tasks.
- **HAL**: Refactored GDT/IDT/APIC and encapsulated system components into a core `OS` struct.
- **Isolation**: Implemented page table mirroring and hardened context switching to ensure stack isolation.

### Phase 2: Graphical Evolution & Userland Migration (Late 2025)
- **Compositor**: Moved the compositor from the kernel to a standalone userland process.
- **Telemetry**: Implemented journal structures for event recording and replay.
- **Capabilities**: Introduced a capability-based security model for bundle permission delegation.

### Phase 3: Desktop Experience & Hardening (Dec 2025)
- **Integrated Desktop**: Introduction of `kernel_standalone` mode where the init program acts as the desktop runner.
- **Safety**: Systematic elimination of `static mut` in favor of `UnsafeCell` and safe wrappers.
- **Widgets**: Developed a comprehensive widget library (ListBox, Scrollbar, Button, etc.) with a Flexbox-inspired layout engine.

### Phase 4: Multi-Arch & POSIX Foundation (Jan 2026)
- **Jan 1**: Initial commit with Limine bootloader support.
- **Jan 5**: Multi-Arch Support: Aarch64, Riscv64, and Loongarch64 bring-up.
- **Jan 15**: Memory Milestone: Frame allocator with contiguous allocation and HHDM paging support.

### Phase 5: The System Graph (Feb 2026)
- **Feb 2**: Root Service: Introduction of the System Graph and 128-bit `ThingId`.
- **Feb 15**: Networking (Netd): Userspace network stack with `smoltcp` integration.
- **Feb 20**: Anther & Phloem: Graphical explorer and GQL engine for system introspection.

### Phase 6: The VFS Revolution (Early April 2026)
- **Apr 7**: Act III: Birth of the VFS: Introduced `VfsNode`, `FdTable`, and the mount system.
- **Apr 8**: Act IV: Kernel Filesystems: Implemented `tmpfs`, `devfs`, and `bootfs`.
- **Apr 9**: The Great De-Graphing: Removed `anther`, `phloem`, and `boltd`. Migrated `netd` and drivers to VFS-first communication (`/dev/net/virtio0`).
- **Apr 10**: VFS-Driven Audio: Complete rewrite of the audio subsystem to use file-based buffers instead of graph nodes.

### Phase 7: Modern Stability (Mid April 2026)
- **Apr 13**: POSIX Signals: Full support for `sigaction`, `kill`, and signal frame injection on x86_64.
- **Apr 14**: **The Great Import**: Core kernel architecture, ABI definitions, and initial userspace utilities established in the current repository.
- **Apr 15**: Process Groups: Support for `tcsetpgrp`, process groups, and sessions for job control.
- **Apr 18**: Self-Hosting Status: Integration of ThingOS-patched `vendor/rust`.

---

## 🛠️ Subsystem Deep Dives

### Kernel & Scheduler
- **Evolution**: From simple kthreads in `basic-os` to a modular SMP scheduler with atomic task reaping.
- **Safety**: Transitioned from loose graph-based permissions to a strict capability model and validated syscall boundaries.
- **Ghost Storage**: Refactored scheduler params to prevent panics when outgoing tasks are reaped mid-switch.

### Graphics (Bloom/Blossom/Petals)
- **Evolution**: Started as a basic blitter, evolved into a declarative UI system (`Blossom`), then refactored into the `Petals` graphics library.
- **Layout**: Modern Flexbox/Grid-inspired engine evolved from early fixed-coordinate widgets.
- **Damage Tracking**: High-efficiency rendering only updates "damaged" screen regions.

### Networking & VFS
- **Networking**: Decoupled drivers (virtio_netd) from protocol logic (netd) via VFS-first IPC.
- **Socket API**: Transitioned from graph-based "NetClient" nodes to standard file descriptors and `poll()` semantics.
- **VFS**: Supports userland providers (e.g., `iso9660`) mounting directly into the system tree.

---

## 🔍 Hidden Gems & Resolved Hazards

- **Clock Calibration**: Ensuring accurate monotonic time across architectures via calibrated delays.
- **Atomic Task Reaping**: Fixed persistent race conditions in process cleanup by ensuring atomic state transitions.
- **Zero-Copy Framebuffers**: Hardened 4K performance through efficient buffer negotiation.
- **Logging Deadlocks**: Implementation of the "Copy-then-Emit" pattern to prevent UART lock contention.
- **ABI Alignment**: Strict 16-byte stack alignment for stable syscall entry.

---

## 📜 Repository-Native Detailed Log (Post-Import)

### 1. Toolchain & Standard Library Foundation
**Commits:** `e16466f` to `f956e64`
- **Rust Fork Integration**: Established `thingos` as a first-class target in the Rust compiler.
- **Sync Primitives**: Implemented `futex`-based synchronization for `std::sync`.
- **System Call Bindings**: Connected `std::fs`, `std::net`, and `std::thread` to the initial Thing-OS syscall surface.

### 2. Task & Job Migration
**Commits:** `bfb00dc` to `54da729`
- **Terminology Shift**: Formally deprecated "Process" and "Thread" in favor of "Task" and "Job".
- **Process Decomposition**: Broke the `Process` struct into subdivisions like `ProcessAddressSpace` and `ProcessLifecycle`.
- **Unix Quarantine**: Created `ProcessUnixCompat` to isolate legacy Unix assumptions from the core kernel logic.

### 3. VFS-First & Messaging Refactor
**Commits:** `1bc7659`, `ffe996c`, `7a8e8ce`
- **Input Migration**: Moved HID paths to VFS-first message systems and introduced the `Inbox` primitive.
- **VFS Discovery**: Display drivers and the compositor (`bloom`) migrated to binding through files and file descriptors (e.g., `/dev/fb0`).

### 4. SMP Scheduler & Resource Optimization
**Commits:** `393cd78`, `e06d51a`, `bf833e8`
- **SMP Stability**: Resolved lock contention and fixed `waitpid` consistency for resource reaping.
- **Resource Flush**: Added logic to flush pipes, sockets, and file descriptors on task exit.
