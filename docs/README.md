# ThingOS Documentation Index

This directory contains design documents, reference material, and
architecture notes for the ThingOS kernel and userland.

> **Quick orientation**: see `AGENTS.md` at the repo root for a summary of
> key design rules, common commands, and important invariants.

---

## Top-Level Documents

| File | What it covers |
|---|---|
| [`platform.md`](platform.md) | Platform layer contract — Thing-OS `std`, kernel `no_std`, and `stem::pal` |
| [`../CHANGELOG.md`](../CHANGELOG.md) | Project Heritage & Changes Log (History from 2025 – Present) |

---

## Subdirectories

### `abi/` — ABI Definitions

Low-level contracts between kernel and userspace.

- [`task_entry.md`](abi/task_entry.md) — task entry registers, stack layout, ARG0 semantics

### `architecture/` — VM and Address Space

- [`how-to-think-in-thingos.md`](architecture/how-to-think-in-thingos.md) — contributor/agent mental model (typed truth vs Unix projection)
- [`job.md`](architecture/job.md) — Job as canonical lifecycle object for creation/exit/wait/reap
- [`process-projection.md`](architecture/process-projection.md) — Process as compatibility projection (not root ontology)
- [`space.md`](architecture/space.md) — Space as the canonical address-space object
- [`space-audit.md`](architecture/space-audit.md) — inventory of VM coupling in kernel code

### `build/` — Build System and Toolchain

- [`bootstrapping.md`](build/bootstrapping.md) — Rust cross-compiler bootstrap (`just fetch-rust`, `cargo xtask rustc-thingos`)
- [`kind-schema-workflow.md`](build/kind-schema-workflow.md) — `kindc` schema compiler workflow and drift detection
- [`status/rustc_build.md`](build/status/rustc_build.md) — current rustc bootstrap build status
- [`status/semantic_self_hosting.md`](build/status/semantic_self_hosting.md) — milestone audit for semantic self-hosting (current gaps and in-place capabilities)

### `components/` — Per-Component Guides

One document per major crate/binary:

| File | Component |
|---|---|
| [`bloom.md`](components/bloom.md) | Bloom — compositor |
| [`blossom.md`](components/blossom.md) | Blossom — graphics layer |
| [`bran.md`](components/bran.md) | Bran — kernel runtime abstraction |
| [`bristle.md`](components/bristle.md) | Bristle — HID broker |
| [`flytrap.md`](components/flytrap.md) | Flytrap — security monitor |
| [`kernel.md`](components/kernel.md) | Kernel |
| [`root.md`](components/root.md) | Root — graph/layout engine |
| [`sprout.md`](components/sprout.md) | Sprout — init/process launcher |
| [`stem.md`](components/stem.md) | Stem — low-level platform library and PAL helpers |
| [`virtio_sound.md`](components/virtio_sound.md) | VirtIO Sound — `cambium` marker/export and VFS PCM path |

### `concepts/` — Design Concepts and Doctrine

Higher-level design decisions, cross-cutting concerns, and reference specs.

**IPC and Messaging**
- [`ipc.md`](concepts/ipc.md) — IPC doctrine; the six primitives
- [`channel_semantics.md`](concepts/channel_semantics.md) — channel specification
- [`channels_vs_pipes.md`](concepts/channels_vs_pipes.md) — when to use which
- [`ipc_cookbook.md`](concepts/ipc_cookbook.md) — practical IPC recipes
- [`ipc_diag.md`](concepts/ipc_diag.md) — IPC health diagnostics via `/proc/ipc/`

**UI and Display**
- [`ui_architecture.md`](concepts/ui_architecture.md) — compositor architecture
- [`ui_intent_contract.md`](concepts/ui_intent_contract.md) — app/service boundary
- [`damage-tracking.md`](concepts/damage-tracking.md) — damage region tracking
- [`render_graph_pipeline.md`](concepts/render_graph_pipeline.md) — render pipeline
- [`root_batching.md`](concepts/root_batching.md) — Root draw-call batching
- [`simd_glyph_rendering.md`](concepts/simd_glyph_rendering.md) — SIMD glyph rendering
- [`viewport-physics.md`](concepts/viewport-physics.md) — viewport physics
- [`vir_pipeline.md`](concepts/vir_pipeline.md) — Vector Intermediate Representation pipeline
- [`font-graph.md`](concepts/font-graph.md) — font graph and discovery
- [`filesystem_window_model.md`](concepts/filesystem_window_model.md) — filesystem-backed window model
- [`photosynthesis_layout.md`](concepts/photosynthesis_layout.md) — Photosynthesis layout algorithm

**Platform and Runtime**
- [`userland.md`](concepts/userland.md) — userspace threading and runtime model
- [`pal-examples.md`](concepts/pal-examples.md) — PAL usage examples
- [`thingos-pal-abi-sync.md`](concepts/thingos-pal-abi-sync.md) — checklist for ThingOS std PAL syscall/ABI mirror sync
- [`unix-compat.md`](concepts/unix-compat.md) — Unix compatibility layer

**Kernel Internals**
- [`scheduling.md`](concepts/scheduling.md) — scheduler design
- [`namespaces.md`](concepts/namespaces.md) — namespace semantics
- [`memfd.md`](concepts/memfd.md) — anonymous memory files
- [`process-lifecycle.md`](concepts/process-lifecycle.md) — process lifecycle
- [`process-object.md`](concepts/process-object.md) — Process object internals
- [`vfs_rpc_provider.md`](concepts/vfs_rpc_provider.md) — VFS RPC provider protocol
- [`port_waiting.md`](concepts/port_waiting.md) — port wait semantics
- [`readiness.md`](concepts/readiness.md) — readiness polling model
- [`syscalls.md`](concepts/syscalls.md) — **full syscall ABI reference**

**Architecture and Governance**
- [`thingos-guardrails.md`](concepts/thingos-guardrails.md) — non-negotiable design rules (read this first)
- [`service_contract.md`](concepts/service_contract.md) — service contract system
- [`supervisor_protocol.md`](concepts/supervisor_protocol.md) — supervisor/child protocol
- [`content_provider.md`](concepts/content_provider.md) — content provider pattern
- [`i18n_system.md`](concepts/i18n_system.md) — internationalisation
- [`presence.md`](concepts/presence.md) — presence and availability

### `ipc/` — IPC Migration and Analysis

Active IPC migration tracking and semantic analysis.

- [`ipc_migration_status.md`](ipc/ipc_migration_status.md) — overall migration status
- [`convergence_strategy.md`](ipc/convergence_strategy.md) — inbox/port convergence strategy
- [`inbox_vs_port_semantics.md`](ipc/inbox_vs_port_semantics.md) — semantic analysis
- [`vfs_rpc_ipc_audit.md`](ipc/vfs_rpc_ipc_audit.md) — VFS RPC transport audit
- [`event-vs-message.md`](ipc/event-vs-message.md) — events vs messages distinction
- [`delivery-semantics-matrix.md`](ipc/delivery-semantics-matrix.md) — delivery semantics matrix
- [`group-broadcast.md`](ipc/group-broadcast.md) — typed group-broadcast design

### `audits/` — Focused Architecture Audits

- [`ipc-typed-world-audit.md`](audits/ipc-typed-world-audit.md) — canonical vs compatibility IPC semantics through typed-world lens

### `kernel/` — Kernel Internals Reference

Implementation details for kernel subsystems.

- [`fs-semantics.md`](kernel/fs-semantics.md) — POSIX filesystem semantics status and gaps
- [`posix-checklist.toml`](kernel/posix-checklist.toml) — machine-readable POSIX feature tracker
- [`signals.md`](kernel/signals.md) — signal subsystem (POSIX signals, ABI, gaps)
- [`scheduler-anti-starvation.md`](kernel/scheduler-anti-starvation.md) — priority aging invariants
- [`wait_many.md`](kernel/wait_many.md) — `SYS_WAIT_MANY` design
- [`waitables.md`](kernel/waitables.md) — waitable handle shapes (`FsWatch`, `OpHandle`, `ReadyCondition`)
- [`watch.md`](kernel/watch.md) — VFS watch system (`SYS_FS_WATCH_FD` / `SYS_FS_WATCH_PATH`)
- [`network_waitable_events.md`](kernel/network_waitable_events.md) — network blocking via `SYS_FS_POLL`
- [`logging_levels.md`](kernel/logging_levels.md) — log level definitions
- [`services-layout.md`](kernel/services-layout.md) — VFS namespace layout for services

### `migration/` — Active Migration Control Documents

Inventory and guidance for the ongoing kernel restructuring (Phase 9).

- [`concept-mapping.md`](migration/concept-mapping.md) — canonical lexicon: legacy Unix → ThingOS
- [`review-guidelines.md`](migration/review-guidelines.md) — PR review checklist derived from concept mapping
- [`boundary_audit.md`](migration/boundary_audit.md) — kernel boundary type audit
- [`authority_inventory.md`](migration/authority_inventory.md) — process credential/permission inventory
- [`bridge_architecture.md`](migration/bridge_architecture.md) — old→new semantic translation layer
- [`process_responsibility_map.md`](migration/process_responsibility_map.md) — process responsibility decomposition
- [`process_execution_context_inventory.md`](migration/process_execution_context_inventory.md) — execution context inventory

### `archive/` — Historical / Completed Work

Implementation summaries and migration guides for completed work.
These are kept for reference but describe past states, not current behavior.

- Input migration: [`keyboard-migration.md`](archive/keyboard-migration.md), [`mouse-migration.md`](archive/mouse-migration.md)
- Cursor: `CURSOR_BUTTER_SMOOTH_*.md` — cursor overlay composition implementation
- Platform: [`platform-migration.md`](archive/platform-migration.md) — guide for migrating to `stem::pal`
- Other: `IMPLEMENTATION_SUMMARY.md`, `OPTIMIZATION_SUMMARY.md`, etc.

---

## Key Cross-Cutting References

| Topic | Primary Document |
|---|---|
| Design rules / PR checklist | [`concepts/thingos-guardrails.md`](concepts/thingos-guardrails.md) |
| Syscall numbers | [`concepts/syscalls.md`](concepts/syscalls.md) or `abi/src/numbers.rs` |
| Platform layer contract | [`platform.md`](platform.md) |
| IPC doctrine | [`concepts/ipc.md`](concepts/ipc.md) |
| VFS semantics | [`kernel/fs-semantics.md`](kernel/fs-semantics.md) |
| Signals | [`kernel/signals.md`](kernel/signals.md) |
| Scheduler | [`concepts/scheduling.md`](concepts/scheduling.md) |
