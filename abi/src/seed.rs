//! # Seed/Shoot binary model
//!
//! A **Seed** is a compiled ELF binary with typed interface descriptors embedded
//! in it.  It is inert and inspectable — a package of code and metadata that is
//! not yet executing.
//!
//! A **Shoot** is the live, running realization that grows from a Seed: a process,
//! driver binding, or service instance that the system creates when it germinates
//! the Seed into an execution context.

/// ABI version this descriptor was compiled against.
pub const SEED_ABI_VERSION: u32 = 1;

/// Name of the ELF symbol exported by every Seed binary.
///
/// The system discovers Seeds by scanning for this symbol and reading the
/// embedded [`Seed`] descriptor to determine what interfaces the binary
/// implements and how it may be hosted.
pub const SEED_SYMBOL: &str = "THINGOS_SEED";

// ── Built-in interface identifiers ───────────────────────────────────────────

/// Interface id for `ProgramV1` — a conventional one-shot executable program.
///
/// Required entry: `main(args: ArgVec) -> ExitCode`
pub const INTERFACE_PROGRAM_V1: u32 = 1;

/// Interface id for `LifecycleV1` — a Seed that can be started/stopped by the host.
///
/// Required entries: `start(ctx: HostContext) -> Status`,
/// `stop(ctx: HostContext) -> Status`
pub const INTERFACE_LIFECYCLE_V1: u32 = 2;

/// Interface id for `DriverV1` — a Seed that can be probed and bound as a driver.
///
/// Required entries: `probe(ctx, dev) -> ProbeDisposition`,
/// `bind(ctx, dev) -> BindResult`, `unbind(ctx, dev) -> Status`
pub const INTERFACE_DRIVER_V1: u32 = 3;

// ── Hosting mode bits ─────────────────────────────────────────────────────────

/// Hosting mode: Seed can be germinated as a regular program (ProgramV1).
pub const HOST_PROGRAM: u64 = 1 << 0;
/// Hosting mode: Seed can be hosted as a resident lifecycle service (LifecycleV1).
pub const HOST_LIFECYCLE: u64 = 1 << 1;
/// Hosting mode: Seed can be probed/bound as a driver (DriverV1).
pub const HOST_DRIVER: u64 = 1 << 2;

// ── Core types ────────────────────────────────────────────────────────────────

/// One declared interface inside a [`Seed`] descriptor.
///
/// Each entry names an interface by `(interface_id, interface_version)` and
/// optionally supplies the entry surface symbol the system calls to enter the
/// Seed through that interface.
///
/// ### Entry symbol semantics
///
/// - For [`INTERFACE_PROGRAM_V1`]: an empty (`len == 0`) entry symbol means
///   "use the ELF default entry", preserving plain `main`/crt-style program
///   flow unchanged.
/// - For [`INTERFACE_DRIVER_V1`] and [`INTERFACE_LIFECYCLE_V1`]: the host
///   requires a non-empty entry symbol; binaries must provide one.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct SeedInterface {
    pub interface_id: u32,
    pub interface_version: u32,
    pub flags: u32,
    pub reserved: u32,
    pub entry_symbol_ptr: *const u8,
    pub entry_symbol_len: usize,
}

/// Immutable descriptor for a compiled Seed binary.
///
/// Every Seed binary must export this as a `#[used] #[unsafe(no_mangle)]`
/// static named `THINGOS_SEED` so it is visible in the ELF symbol table at
/// load time.  The system reads the descriptor to determine the Seed's name,
/// hosting modes, and declared interfaces before germinating it into a Shoot.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Seed {
    /// Must equal [`SEED_ABI_VERSION`].
    pub abi_version: u32,
    /// Number of valid entries in `interfaces` (must be ≤ 4).
    pub interface_count: u32,
    /// Bitmask of `HOST_*` constants declaring how this Seed may be hosted.
    pub hosting_modes: u64,
    /// Reserved capability bitmask; set to 0.
    pub capabilities: u64,
    /// Pointer/length of the human-readable Seed name (UTF-8, no null terminator).
    pub name_ptr: *const u8,
    pub name_len: usize,
    /// Declared interface table; unused slots must be zeroed.
    pub interfaces: [SeedInterface; 4],
}

unsafe impl Sync for Seed {}
unsafe impl Send for Seed {}

impl Seed {
    /// Look up a declared interface by `(id, version)`.
    ///
    /// Returns `None` if the Seed does not declare that interface.
    #[inline]
    pub fn interface(&self, interface_id: u32, interface_version: u32) -> Option<SeedInterface> {
        let limit = core::cmp::min(self.interface_count as usize, self.interfaces.len());
        self.interfaces.iter().copied().take(limit).find(|iface| {
            iface.interface_id == interface_id && iface.interface_version == interface_version
        })
    }

    /// Returns `true` if this Seed declares [`INTERFACE_DRIVER_V1`].
    #[inline]
    pub fn implements_driver_v1(&self) -> bool {
        self.interface(INTERFACE_DRIVER_V1, 1).is_some()
    }

    /// Returns `true` if this Seed declares [`INTERFACE_PROGRAM_V1`].
    #[inline]
    pub fn implements_program_v1(&self) -> bool {
        self.interface(INTERFACE_PROGRAM_V1, 1).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_interface_lookup_finds_declared_interfaces() {
        let seed = Seed {
            abi_version: SEED_ABI_VERSION,
            interface_count: 2,
            hosting_modes: HOST_PROGRAM,
            capabilities: 0,
            name_ptr: core::ptr::null(),
            name_len: 0,
            interfaces: [
                SeedInterface {
                    interface_id: INTERFACE_PROGRAM_V1,
                    interface_version: 1,
                    flags: 0,
                    reserved: 0,
                    entry_symbol_ptr: core::ptr::null(),
                    entry_symbol_len: 0,
                },
                SeedInterface {
                    interface_id: INTERFACE_DRIVER_V1,
                    interface_version: 1,
                    flags: 0,
                    reserved: 0,
                    entry_symbol_ptr: core::ptr::null(),
                    entry_symbol_len: 0,
                },
                SeedInterface::default(),
                SeedInterface::default(),
            ],
        };

        assert!(seed.implements_program_v1());
        assert!(seed.implements_driver_v1());
        assert!(seed.interface(INTERFACE_LIFECYCLE_V1, 1).is_none());
    }
}
