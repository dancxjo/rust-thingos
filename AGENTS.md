# Thing-OS Agent Notes

This file is a quick map of the repository so agents (and humans) can orient fast.

## What this repo is
- Thing-OS is a Rust workspace that builds a VFS-first OS kernel plus userland apps.
- The old kernel graph/ThingId model is legacy. Do not introduce new boot, display, or UI dependencies on `stem::thing`, `ThingId`, `UI_CROWN`, or graph discovery for core system bring-up.
- Build/test automation lives in `xtask` and is surfaced via `just`.

## Common commands
- Build ISO: `just iso` (override arch with `KARCH=aarch64`, `riscv64`, `loongarch64`)
- Run QEMU: `just run`
- Run BDD tests: `just behave` (see `tools/bdd`)
- Clean: `just clean`
- Audit platform boundary: `python3 scripts/audit_platform_boundary.py`

## BDD expectations for agents
- When adding a feature, add new BDD coverage and ensure the new BDD tests pass.
- When fixing a bug, add or update scenarios in the appropriate existing `.feature` file for that behavior.
- For bug fixes, follow fail-first workflow: make the scenario fail first, then fix the bug, then run the BDD suite again and confirm it passes.

## Top-level layout (what's what)
- `abi/`: shared ABI types and syscalls between kernel/userspace.
- `bran/`: core kernel runtime (boot/runtime abstraction).
- `kernel/`: kernel crate and core kernel logic.
- `drivers/`: hardware driver crates.
- `utils/`: most user programs and demos (each subdir is a crate).
- `bloom/`, `bristle/`, `pistil/`, `sprout/`: top-level UI/runtime userspace crates.
- `bloom/`, `blossom/`, `display/`: graphics/compositor-related crates.
- `stem/`, `stem-macros/`: internal libs and proc-macros.
- `targets/`: custom JSON target specs for bare metal builds.
- `tools/`: auxiliary tooling (BDD, pciids, etc).
- `xtask/`: build orchestration used by `just`.
- `docs/`: documentation and test reports (`docs/behavior/` is generated).
- `compiler/`, `library/`: Rust compiler and standard library source (this repo is a Rust fork).
- `vendor/`: vendored dependencies (Limine, OVMF).

## Where to start when changing behavior
- Kernel interfaces: `abi/` and `kernel/`
- Syscall surface: `abi/src/syscall.rs`
- User apps: `utils/` plus top-level userspace crates (`bloom/`, `bristle/`, `pistil/`, `sprout/`)
- Build/config: `justfile`, `xtask/`, `targets/`

## UI / Display Contract

- Display discovery and presentation are filesystem-driven.
- The kernel exposes the boot framebuffer at `/dev/fb0`; display drivers and the compositor must bind through files and file descriptors, not graph nodes.
- Bloom should boot and paint with only VFS/device state available. Do not require `UI_CROWN`, graph watches, or `ThingId` lookups to reach first paint.
- Runtime UI coordination should happen through mounted services and session/runtime files such as `/services`, `/run`, and `/session`.
- Desktop background configuration lives at `/session/desktop/{wallpaper,mode,background_color}` and Bloom is expected to watch and react to those files.

## Platform Layer Contract (std + stem PAL)

**Thing-OS is a Rust fork that builds and customizes `std` for Thing-OS targets.**

- The repository includes Rust `compiler/` and `library/` sources specifically so Thing-OS can evolve its own `std` behavior.
- `std` is allowed for non-kernel Thing-OS code. Use it in userspace, drivers, services, tests, and tooling when it improves correctness, compatibility, or maintainability.
- `stem::pal` remains the explicit low-level/no_std platform abstraction for the kernel-adjacent runtime, PAL itself, and crates that intentionally stay `no_std`.
- Build tools (`xtask`, `tools/*`) can use host `std` freely.

**Key rules:**
- Do not add new blanket `no_std` requirements outside the kernel. Existing non-kernel `no_std` crates may migrate to Thing-OS `std` when convenient.
- Kernel code must remain `no_std`; PAL/no_std crates should keep explicit platform boundaries via `stem::pal`.
- Avoid host-only assumptions in target userspace. Prefer Thing-OS `std` APIs where available, and extend `library/std/src/sys/pal/thingos/` when the standard library needs more OS support.

**See `docs/platform.md` for the complete platform layer contract.**

## Rust source of truth

This repository is itself a fork of the Rust compiler and standard library, customized for Thing-OS.

- **Fork Repository**: [dancxjo/thingos](https://github.com/dancxjo/thingos) (unified with [dancxjo/rust-thingos](https://github.com/dancxjo/rust-thingos))
- **Source Layout**: The Rust `compiler/` and `library/` (standard library) directories are at the root of this workspace.
- **Modifications**: Edit Rust source files directly in the root (e.g., `library/std/src/sys/pal/thingos/...`). Commit and push changes as part of the main repository history.

Important implications:
- All Rust source code is tracked directly in this repository.
- `git status` shows all changes to the compiler and standard library.
- The `xtask` build system hashes the relevant root directories and configuration files to detect when the compiler needs to be rebuilt.

## Architecture Guardrails

Four non-negotiable design rules govern all kernel and userspace changes:

1. **Scheduler-first** — every unit of execution is a kernel-scheduled task.
2. **Userland drivers** — hardware logic lives in userspace; kernel exposes only `SYS_DEVICE_*` primitives.
3. **VFS-first** — all system resources are reachable through mounted filesystem paths.
4. **Spawn + exec** — new processes use `SYS_SPAWN_PROCESS[_EX]` + `SYS_TASK_EXEC`; there is no `SYS_FORK`.

**See `docs/concepts/thingos-guardrails.md` for the full reference and PR review checklist.**

## Notes
- Workspace members are listed in `Cargo.toml`.
- `target/` is build output and can be ignored in reviews.
- Reminder: use the `apply_patch` tool directly for file edits (avoid running it via exec). 
- Reminder: hashing helpers must use the correct byte width for each integer type (u32/i32 = 4 bytes).
- The `stem` build script expects `assets/pci/pci.ids`; ensure it exists (or skip builds that trigger `stem`'s build.rs) when running `cargo test`.
- Reminder: avoid clearing all UI caches on watch events; prefer targeted invalidation once event payloads provide node IDs.
