# Seed/Shoot binary model

## Overview

Every executable artifact in Thing-OS is a **Seed** — a loadable ELF with a
canonical metadata descriptor embedded in it.  The Seed model replaces the
legacy split between "programs" (entered via `main`) and "drivers" (recognized
by ad-hoc convention) with a single uniform abstraction.

## Seed and Shoot

| Term | Meaning |
|---|---|
| **Seed** | A compiled ELF binary with typed interface descriptors embedded in it.  Inert and inspectable; not yet executing. |
| **Shoot** | A live, running realization of a Seed: a process, driver binding, or service instance created when the system germinates the Seed. |

> **Key rule**: Seeds are inert and inspectable. Shoots are active and
> scheduled/bound. Never conflate the two in APIs or architecture.

### Lifecycle vocabulary

| Operation | Rust/system analogue | Meaning |
|---|---|---|
| `plant(seed)` | install to image / present in `/bin` or `/drivers` | Register a Seed so the system knows it exists |
| `germinate(seed, ctx)` | load + start / probe + bind | Load and initialize a Seed into a Shoot |
| `shoot` | running process / driver binding | The active instance |
| `propagate(seed, n)` | spawn N processes | Create multiple Shoots from the same Seed |
| `transplant(shoot, ctx)` | move to new CPU / place / device association | Relocate a running Shoot |
| `wither(shoot)` | exit / teardown / unbind | Terminate and clean up a Shoot |

## Seed descriptor

Every Seed binary exports a `#[used] #[unsafe(no_mangle)]` static named
`THINGOS_SEED` of type `abi::seed::Seed`.  The system discovers Seeds by
scanning for this symbol.

### `abi::seed::Seed` layout (ABI v1)

```
Seed {
    abi_version: u32,       // must equal SEED_ABI_VERSION (1)
    interface_count: u32,   // number of valid entries in interfaces[]
    hosting_modes: u64,     // bitmask of HOST_* flags
    capabilities: u64,      // reserved, set to 0
    name_ptr: *const u8,
    name_len: usize,        // UTF-8 Seed name, no null terminator
    interfaces: [SeedInterface; 4],
}

SeedInterface {
    interface_id: u32,
    interface_version: u32,
    flags: u32,
    reserved: u32,
    entry_symbol_ptr: *const u8,
    entry_symbol_len: usize,    // 0 = use ELF default entry (ProgramV1 only)
}
```

### Hosting modes

| Constant | Meaning |
|---|---|
| `HOST_PROGRAM` | Seed can be germinated as a program (ProgramV1) |
| `HOST_LIFECYCLE` | Seed can be hosted as a resident service (LifecycleV1) |
| `HOST_DRIVER` | Seed can be probed/bound as a driver (DriverV1) |

## Built-in interfaces

### ProgramV1 (`INTERFACE_PROGRAM_V1 = 1`)

Represents a conventional one-shot executable program.

- **Required entry**: `main(args: ArgVec) -> ExitCode`
- **Entry symbol**: empty (`len == 0`) means "use ELF default entry" —
  preserving plain `main`/crt-style programs with zero extra ceremony.
- **Hosting mode**: `HOST_PROGRAM`

### LifecycleV1 (`INTERFACE_LIFECYCLE_V1 = 2`)

Represents a Seed that can be started and stopped by the host/runtime.

- **Required entries**: `start(ctx: HostContext) -> Status`, `stop(ctx: HostContext) -> Status`
- **Entry symbol**: must be non-empty; names the `start` entry surface.
- **Hosting mode**: `HOST_LIFECYCLE`

### DriverV1 (`INTERFACE_DRIVER_V1 = 3`)

Represents a Seed that can be probed and bound as a driver.

- **Required entries**: `probe(ctx, dev) -> ProbeDisposition`, `bind(ctx, dev) -> BindResult`, `unbind(ctx, dev) -> Status`
- **Entry symbol**: must be non-empty; names the driver entry surface.
- **Hosting mode**: `HOST_DRIVER`

## Runtime discovery

The driver orchestrator (`cambium`) prefers the canonical Seed path:

1. Read ELF symbol table; look for `THINGOS_SEED` (= `SEED_SYMBOL`).
2. Parse embedded `Seed` descriptor; verify `abi_version == SEED_ABI_VERSION`.
3. Check that the Seed declares `DriverV1` (`INTERFACE_DRIVER_V1`).
4. Also read legacy `THINGOS_DRIVER` descriptor for matching metadata during
   the transitional period.

For program execution, the loader detects `ProgramV1` in the Seed descriptor.
When the entry symbol is empty the loader uses the ELF default entry, which
keeps plain `main`-based programs working without any change.

## Transitional compatibility

Legacy binary formats continue to work while the codebase migrates:

- **Legacy driver path** (`THING_DRIVER_V1`, `THINGOS_DRIVER`): explicitly
  supported and labelled `// Transitional compatibility path` in `cambium` catalog.
- **Legacy program path** (plain ELF entry, no Seed descriptor): still launched
  normally; the Seed descriptor is additive.
- **Canonical path**: binaries that export `THINGOS_SEED` are handled via the
  Seed-native code path; legacy fallbacks are only reached when the Seed
  descriptor is absent.

Do not bury transitional logic inside new Seed/Shoot APIs.  The codebase must
make it obvious which path is canonical and which is scaffolding.

## Migrated examples

| Binary | Interface | Notes |
|---|---|---|
| `drivers/hwrng` | `DriverV1` | Reference driver Seed migration |
| `userspace/hello_stdio` | `ProgramV1` | Reference program Seed migration (plain `main` entry) |

## Plain `main` programs

Regular programs that use a plain `fn main()` entry remain fully supported.
They can optionally declare `ProgramV1` in their Seed descriptor with an empty
entry symbol, which signals "use ELF default entry".  No `#[stem::main]` macro
or extra boilerplate is required for programs that already use `fn main()`.

## Naming conventions

- Seed binary name: use the program/driver name without any suffix (e.g. `b"hwrng"`, not `b"hwrng.driver"`).
- Exported descriptor symbol: always `THINGOS_SEED` (the `SEED_SYMBOL` constant).
- Types: `abi::seed::Seed`, `abi::seed::SeedInterface`.
