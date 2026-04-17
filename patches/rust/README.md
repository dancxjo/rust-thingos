Rust and LLVM snapshot patches for the integrated Rust fork.

```bash
just clean
cargo xtask rustc-thingos
```

## Layout

- `patches/rust/` — patches applied to the top-level repository.
- `patches/rust/llvm-project/` — patches applied to `src/llvm-project`.

These directories contain historical patches used to bootstrap the fork.

## Current applied patches

### `vendor-rust/`

| File | Description |
|---|---|
| `0001-bootstrap-use-thingos-cmake-system-name.patch` | Rust bootstrap (`src/bootstrap/src/core/build_steps/llvm.rs`) now sets `CMAKE_SYSTEM_NAME=ThingOS` and `LLVM_ON_UNIX=ON` when building for `x86_64-unknown-thingos`, instead of the `Generic` fallback that caused `EnvPathSeparator`/`getSize` errors. Resolves #714. |

### `llvm-project/`

| File | Description |
|---|---|
| `0001-llvm-classify-thingos-as-unix.patch` | LLVM CMake (`config-ix.cmake`, `HandleLLVMOptions.cmake`) classifies `ThingOS` as a Unix-like platform so that POSIX-flavoured support headers and libraries are selected during host-tool compilation. Resolves #714. |

The numbered `patches/rust/*.patch` files (`00-core-prelude.patch` through
`99-llvm-thingos.patch`) are historical snapshots of changes that were once
applied manually.

The PAL implementation for ThingOS (`library/std/src/sys/pal/thingos/`) that
these patches describe is committed directly to the
[`dancxjo/rust-thingos`](https://github.com/dancxjo/rust-thingos) fork.

## Source of truth

The source of truth for all Rust/LLVM modifications is this repository itself.
The patches here are historical artifacts preserved for documentation purposes.
