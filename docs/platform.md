# Platform Layer Contract

This document defines the formal boundary between Thing-OS applications and the underlying platform for a repository that **builds and customizes Rust `std`** while also maintaining an explicit **stem PAL** for low-level/no_std platform primitives.

## Overview

Thing-OS is a fork of Rust that includes `compiler/` and `library/` at the repository root so we can ship Thing-OS-specific `std` behavior. Non-kernel Thing-OS code may use that `std` whenever it is the clearest tool. In parallel, the **stem** crate provides a `core`/`alloc` platform abstraction layer (`stem::pal`) for code that intentionally stays `no_std`.

### Why This Matters

- **Sustainability**: We can evolve both Thing-OS `std` and PAL interfaces in-repo
- **Pragmatism**: Userspace can use `std` directly when it improves the code
- **Clarity**: Low-level platform capabilities are explicit and documented
- **Incremental**: New platform features are added intentionally, either in `std` or in PAL as appropriate

## The Platform Abstraction Layer (PAL)

The `stem::pal` module provides the **explicit contract** for low-level platform-specific functionality used by `no_std` crates and by the `std` implementation itself:

```rust
pub mod pal {
    pub mod log;      // Logging primitives
    pub mod clock;    // Time and monotonic clock
    pub mod abort;    // Panic and process termination
    pub mod alloc;    // Memory allocator hooks
    pub mod net;      // Network device access
}
```

## Panic/Unwind Policy (Thing-OS Targets)

Thing-OS targets are currently **abort-only** for panics.

- All Thing-OS target specs set `panic-strategy = "abort"`.
- Stack unwinding ABI/runtime (`panic_unwind`, personality routines, unwinder integration) is not part of the supported target contract.
- A panic in any thread aborts the process; panic payload propagation through `JoinHandle::join` is therefore not available on Thing-OS today.

To keep this failure mode explicit, Thing-OS runtime code emits a compile-time diagnostic if built with `panic = "unwind"`.

### Design Principles

1. **Use `std` where it helps**: Non-kernel userspace, drivers, services, and tests may use Thing-OS `std`.
2. **Keep the kernel small**: Kernel code remains `no_std`.
3. **Keep PAL minimal**: PAL exposes the low-level primitives needed by no_std code and by `std`.
4. **Keep OS behavior target-aware**: Extend `library/std/src/sys/pal/thingos/` or `stem::pal` when the platform surface needs to grow.

## What's Allowed Where

### Kernel and Userspace (Runtime Code)

**Preferred building blocks:**
- `core` - Rust's core library (no allocator, no platform)
- `alloc` - Rust's allocator library (requires our global allocator)
- Thing-OS `std` - Preferred for non-kernel code when available and convenient
- `stem` - Our platform layer (includes `stem::pal`)
- `abi` - Shared types and syscall interfaces

**Boundary guidance:**
- Kernel code must remain `no_std`.
- Non-kernel runtime crates are not required to stay `no_std`; migrate or start them on Thing-OS `std` when that makes the implementation better.
- PAL itself and crates intentionally shared with the kernel can stay `no_std` and should use explicit `stem::pal` boundaries.
- Avoid host-only assumptions in target userspace; if a `std` API is missing or wrong, fix the Thing-OS `std` PAL instead of working around it in every app.

**How it's enforced:**
- CI/project checks may enforce kernel-only or crate-specific constraints.
- Boundary audits, when present, should reject host-only leaks and kernel `std` usage, not `std` usage in normal userspace.

### Build Tools (Compile-time Code)

**CAN use std:**
- `xtask` - Build orchestration
- `tools/*` - Build-time utilities (pciids, bdd, unifont-gen, etc.)
- `*-macros` - Proc-macro crates (run at compile time)

These crates are clearly separated and never linked into the kernel. They may use host `std` freely.

## Porting Standard for Third-Party Rust Crates (`std` + `cfg(unix)`)

For ecosystem crates like terminal UIs (`runa`) and file tools (`lsv`) that
select Unix backends (`cfg(unix)`), ThingOS uses this default policy:

1. **Default path (transparent)**: keep Unix-family compatibility and grow a
   **thin libc POSIX shim** over existing ThingOS kernel/stem primitives.
2. **Fallback path (targeted backend)**: add `cfg(target_os = "thingos")`
   backends only for crates that require semantics we intentionally do not
   emulate or where a native backend is materially better.
3. **Do not fork by default**: prefer upstreamable backend additions over
   long-lived private forks.

### Option tradeoff summary

| Option | Runtime overhead | Engineering overhead | Ecosystem compatibility |
|---|---|---|---|
| Full libc POSIX emulation | Moderate code-size growth; thin syscall translation cost; pthread/TLS bookkeeping in userspace | Lower per-crate maintenance after baseline lands | High (many `cfg(unix)` crates compile unchanged) |
| Pure ThingOS-specific backends | Lowest runtime overhead for each crate | High ongoing maintenance across many crates | Medium/low unless each crate gains a native backend |

### Minimum libc surface for `crossterm`-class crates

To unblock common terminal stacks, prioritize this libc compatibility subset:

- **`unistd` constants/functions**: `STDIN_FILENO`, `STDOUT_FILENO`,
  `STDERR_FILENO`, `isatty`.
- **`termios` basics**: `termios` struct + `tcgetattr`, `tcsetattr`,
  `TCSANOW`, canonical/raw-mode flag bits used by raw-mode toggling.
- **TTY ioctl substrate**: `ioctl(TCGETS/TCSETS/TCSETSW/TCSETSF)`,
  `ioctl(TIOCGPGRP/TIOCSPGRP)`, and `ioctl(TIOCGWINSZ)` for sizing.
- **pthread baseline** (for std/threading consumers): `pthread_create`,
  `pthread_join`, `pthread_detach`, `pthread_self`, basic attrs.

This keeps the compatibility layer narrow while covering the most common
`cfg(unix)` assumptions in CLI/TUI dependencies.

## Adding New Platform Capabilities

When you need a new platform capability (e.g., file I/O, networking), first decide where it belongs:

- Add or fix Thing-OS `std` behavior in `library/std/src/sys/pal/thingos/` when ordinary Rust userspace should get the capability through standard APIs.
- Add or extend `stem::pal` when the capability is needed by `no_std` crates, PAL internals, or code shared with the kernel.

For PAL additions, follow this process:

### 1. Define the PAL Interface

Add a new module under `stem/src/pal/`:

```rust
// stem/src/pal/fs.rs

/// Read bytes from a file.
pub fn read(fd: usize, buf: &mut [u8]) -> Result<usize, Errno> {
    crate::syscall::read(fd, buf)
}

/// Write bytes to a file.
pub fn write(fd: usize, buf: &[u8]) -> Result<usize, Errno> {
    crate::syscall::write(fd, buf)
}
```

### 2. Update PAL Module

Export the new module in `stem/src/pal/mod.rs`:

```rust
pub mod fs;  // New!
pub mod log;
pub mod clock;
// ...
```

### 3. Provide High-Level Wrappers

Add ergonomic wrappers in stem's public API:

```rust
// stem/src/fs.rs

pub use crate::pal::fs::{read, write};

pub struct File {
    fd: usize,
}

impl File {
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, Errno> {
        pal::fs::read(self.fd, buf)
    }
}
```

### 4. Document the Contract

- Document the PAL function's semantics
- Explain syscall behavior and error conditions
- Note any platform-specific considerations

### 5. Test the Boundary

The platform boundary must remain auditable where project checks exist:

```bash
# Run tests
cargo test -p stem
```

## Escape Hatches

### Host-Dependent Code

If you need host-specific functionality for development/testing, use the `xtask` crate or create a new crate under `tools/`. These are **never** linked into the kernel.

Example:
```
tools/
  my-analyzer/     # Uses std, never linked into runtime
    Cargo.toml     # Does NOT have #![no_std]
    src/
      main.rs      # Uses std::fs, std::io, etc.
```

### Testing

Tests can use `std`. For crates that intentionally remain `no_std`, conditional compilation remains useful:

```rust
#![cfg_attr(not(test), no_std)]

#[cfg(test)]
mod tests {
    use std::vec::Vec;  // OK in tests
    // ...
}
```

## Compliance Verification

### Automated Audit

Run platform boundary audits when they exist and when you are working on boundary-sensitive crates. Audits should focus on kernel `std` usage, host-only assumptions, and crate-specific no_std promises:

```bash
just audit-platform
```

If no audit command exists in the current checkout, rely on targeted builds/tests and manual review.

### Manual Review

When reviewing code:
- ✅ New `use stem::pal::*` in low-level/platform code - Good, explicit boundary
- ✅ New `use std::*` in non-kernel Thing-OS userspace - Acceptable, and often preferred
- ❌ Host-only assumptions leaking into Thing-OS runtime code - **REJECT**
- ❌ `std` linked into kernel code - **REJECT**
- ❌ Platform behavior hidden behind unrelated abstractions - **REJECT**

## Current Platform Surface

As of this writing, `stem::pal` provides:

> **Doc maintenance note:** Keep this table in sync with `stem/src/pal/mod.rs` whenever PAL modules are added, removed, or renamed.

| Module | Purpose | Key Functions |
|--------|---------|---------------|
| `pal::log` | Logging | `write()`, `write_with_provenance()` |
| `pal::clock` | Time | `monotonic_ns()`, `sleep_ns()`, `unix_time_ns()` |
| `pal::abort` | Termination | `abort()`, `debug_write_str()` |
| `pal::alloc` | Heap management | `grow_heap()` |
| `pal::net` | Networking | `tcp_connect()`, `tcp_bind()`, `udp_bind()`, `resolve_hostname()` |

Higher-level APIs in stem build on these primitives:
- `stem::println!()` → `pal::log`
- `stem::sleep()` → `pal::clock`
- Panic handler → `pal::abort`
- Global allocator → `pal::alloc`

## Evolution Strategy

The PAL is intentionally minimal. Expand it **incrementally**:

1. Start with stubs or errors for unimplemented features
2. Add syscalls only when actually needed
3. Keep the PAL layer thin - business logic goes above it
4. Prefer composition over feature creep in PAL

Example progression:
- Phase 1: `pal::fs::read()` returns `Err(ENOSYS)`
- Phase 2: Implement basic read syscall
- Phase 3: Add buffering in `stem::io::BufReader` (not in PAL)

## FAQ

**Q: Can I use std in my userspace app?**  
A: Yes. Use Thing-OS `std` wherever it makes non-kernel code clearer, more compatible, or easier to maintain.

**Q: What if I need threading/async/sockets?**  
A: Prefer Thing-OS `std` APIs for normal userspace. If the API is missing or incomplete, extend `library/std/src/sys/pal/thingos/`. Add or extend `stem::pal` only when no_std or kernel-adjacent code needs the primitive.

**Q: Can build tools use std?**  
A: Yes! `xtask` and crates under `tools/` can use std freely.

**Q: How do I know if my crate is compliant?**  
A: Kernel crates must stay no_std. For non-kernel crates, `std` is compliant unless the crate explicitly promises no_std compatibility or uses host-only behavior that Thing-OS cannot support.

**Q: What if a dependency pulls in std?**  
A: That is fine for non-kernel code. For kernel/PAL/shared no_std crates, choose a no_std-compatible dependency or disable default features.

## References

- Source: `stem/src/pal/`
- Audit command: `just audit-platform`
- CI integration: `.github/workflows/` (when added)
- Issue: [Original task for platform boundary formalization]
