# Rustc on ThingOS: Build Status Audit (April 2026)

## Summary

The Linux-hosted stage-1 cross-compiler bootstrap **succeeds** as of April 12,
2026.  Running `cargo xtask rustc-thingos` (or
`just rustc-thingos`) drives `x.py` through all three stages
successfully and caches the result under `target/rustc-thingos/`.

The default produced compiler is a **Linux-hosted cross-compiler** — it runs on
`x86_64-unknown-linux-gnu` and can cross-compile code targeting
`x86_64-unknown-thingos`.  It is **not yet** a ThingOS-native compiler and is
therefore **not staged into the ISO by default** unless a cached ThingOS-native
toolchain is present and ISO toolchain staging is explicitly requested.

---

## Blocking issues: resolved

Three issues were tracked as blockers in issue #713.  All three have been
closed:

### #714 — LLVM platform shims (`EnvPathSeparator`, `getSize`)

**Status: resolved** via CMake patches.

The original LLVM errors occurred because the bootstrap mapped
`x86_64-unknown-thingos` to `CMAKE_SYSTEM_NAME=Generic`, causing LLVM to omit
Unix-specific declarations.

Two structured patches fix this:

| Patch | Target repo | What it does |
|---|---|---|
| `patches/rust/0001-bootstrap-use-thingos-cmake-system-name.patch` | Root | Changes `llvm.rs` to emit `CMAKE_SYSTEM_NAME=ThingOS` and `LLVM_ON_UNIX=ON` instead of `Generic` |
| `patches/rust/llvm-project/0001-llvm-classify-thingos-as-unix.patch` | `src/llvm-project` | Classifies `ThingOS` as a Unix-like platform in `config-ix.cmake` and `HandleLLVMOptions.cmake` |

These patches were used to bootstrap the fork and are now integrated or applied via direct commits.

### #716 — Refine `llvm-target` in target JSON

**Status: resolved** without changing the JSON field.

Issue #716 asked whether `"llvm-target": "x86_64-unknown-none"` should be
changed to something more Unix-flavoured.  The CMake patch approach adopted for
#714 makes the `llvm-target` value irrelevant for the LLVM host-tool build: the
`CMAKE_SYSTEM_NAME=ThingOS` override is applied unconditionally when the target
triple contains `thingos`.

`targets/x86_64-unknown-thingos.json` retains `"llvm-target":
"x86_64-unknown-none"` to preserve the bare-metal code-generation profile for
user binaries.

### #717 — Fix rustlib staging path and re-enable rustc-thingos in ISO build

**Status: resolved with explicit gating and validation.**

- `xtask/src/image.rs` calls `stage_rustc_for_iso(sh, iso_root)` (line ~638).
- `xtask/src/main.rs` calls `rustc_thingos::build_rustc_thingos(&sh, &env)` in
  all ISO/run paths.
- `build_rustc_thingos` runs by default and respects the `SKIP_RUSTC_THINGOS=1`
  env-var opt-out gate.
- `stage_rustc_for_iso` is gated behind `INCLUDE_RUST_TOOLCHAIN=1` and only
  stages cached ThingOS-native artifacts (`thingos-rustc`, optional
  `thingos-cargo`, and ThingOS rustlib tree).
- After staging, `stage_rustc_for_iso` validates expected ISO artifacts:
  `bin/rustc`, optional `bin/cargo`, and
  `lib/rustlib/x86_64-unknown-thingos/lib` when ThingOS rustlib cache exists.

If the ThingOS-native cache is missing, staging is skipped with an advisory
message and the ISO continues without a bundled Rust toolchain.

---

## Patch inventory

### Integrated Patches

| File | Applies to | Description |
|---|---|---|
| `patches/rust/0001-bootstrap-use-thingos-cmake-system-name.patch` | Root | Bootstrap uses `ThingOS` CMake system name |
| `patches/rust/llvm-project/0001-llvm-classify-thingos-as-unix.patch` | `src/llvm-project` | Classify ThingOS as Unix in LLVM CMake |

### Legacy flat patches (documentation only — NOT applied automatically)

The numbered `patches/rust/*.patch` files (`00-core-prelude.patch` through
`95-net.patch`) are historical snapshots of what was once applied manually.
They are **not** replayed by `just rust-apply-patches`.  The PAL
implementation they document (`library/std/src/sys/pal/thingos/`) is committed
directly to the `dancxjo/rust-thingos` fork.

---

## Current artifact layout

After a successful `cargo xtask rustc-thingos`, the bootstrap
writes artifacts to the repository-root `build/` tree:

```text
build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/rustc-main
build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-*.rlib
build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-thingos/release/deps/*.rlib
build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/...
```

The `xtask` wrapper then assembles a developer-friendly cache at:

```text
target/rustc-thingos/rustc              ← stage-1 rustc binary (Linux ELF)
target/rustc-thingos/rustc-wrapper      ← wrapper script (sets sysroot + LD_LIBRARY_PATH)
target/rustc-thingos/lib/rustlib/       ← host + ThingOS target sysroot
target/rustc-thingos/.cache-key         ← invalidation hash
```

The cache is keyed on `targets/x86_64-unknown-thingos.json`,
`rust-toolchain.toml`, and the git HEAD.

---

## Important limitation: Linux-hosted cross-compiler only

The current bootstrap config:

```toml
[build]
host  = ["x86_64-unknown-linux-gnu"]
target = ["x86_64-unknown-linux-gnu", "x86_64-unknown-thingos"]
```

produces a **Linux-hosted cross-compiler** (`ELF 64-bit LSB pie executable,
dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2`).

Shipping this binary into the ThingOS ISO would put an unusable Linux ELF at
`/bin/rustc`, so ISO staging is intentionally disabled pending a
ThingOS-native compiler.

---

## Known remaining issues

### 1. `unexpected_cfgs` warnings

The std build emits `unexpected_cfgs` warnings for `#[cfg(target_os =
"thingos")]` in the PAL modules.  The warnings do not block the build but
indicate `thingos` is not yet declared in `library/std/build.rs` as a known
`target_os` value.

`library/std/build.rs` (or equivalent in the compiler's check-cfg list).

### 2. ThingOS-hosted compiler

To ship `rustc` inside the ThingOS image, the bootstrap `host` must be changed
to `x86_64-unknown-thingos`:

```toml
[build]
host = ["x86_64-unknown-thingos"]
target = ["x86_64-unknown-thingos"]
```

This requires the full ThingOS userspace runtime (process spawn, VFS, signal
handling) to be stable enough to host a compiler process.

### 3. Canonical rustlib sysroot

ThingOS target `.rlib` files currently come from the cargo output directory
`stage1-std/x86_64-unknown-thingos/release/deps/` rather than the canonical
`stage1/lib/rustlib/x86_64-unknown-thingos/lib/` path.  The xtask wrapper
reconstructs a synthetic sysroot from the deps directory.  A cleaner approach
is to promote the ThingOS sysroot to the canonical path via a bootstrap patch.

### 4. ISO staging policy and constraints

ISO staging is implemented but intentionally **opt-in**:

1. Set `INCLUDE_RUST_TOOLCHAIN=1` to request staging.
2. A cached ThingOS-native `target/rustc-thingos/thingos-rustc` must exist.
3. Validation fails the staging step if expected staged artifacts are missing.

Owner: build/toolchain maintainers (`xtask` rustc-thingos flow).

---

---

*Last updated: April 12, 2026*
*Status: Linux-hosted cross-compiler bootstrap succeeds; LLVM blockers resolved;
ISO staging implemented with explicit gates; ThingOS-native compiler availability
still determines whether a toolchain can be included.*
