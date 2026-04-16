//! # Motor Seed/Shoot Model
//!
//! The Motor binary model uses plant terminology to separate artifact from instance:
//!
//! - **[`Seed`]** — the compiled Motor artifact: an ELF image plus its embedded
//!   [`Seed`] descriptor. Inert and inspectable; not yet executing.
//! - **Shoot** — a live, running realization of a Seed: a process, driver binding,
//!   or service instance created when the system enters or binds a Seed.
//!
//! ## Lifecycle vocabulary
//!
//! | Operation | Meaning |
//! |---|---|
//! | `plant(seed)` | Register / install a Seed into the system (present in image or available for loading) |
//! | `germinate(seed, ctx)` | Load and initialize a Seed into a Shoot (allocate resources, prepare execution) |
//! | `shoot` | The active instance (process, driver binding, or service instance) |
//! | `propagate(seed, n)` | Create multiple Shoots from the same Seed |
//! | `transplant(shoot, ctx)` | Move a Shoot to a different execution context |
//! | `wither(shoot)` | Terminate and clean up a Shoot (release resources, detach from system) |
//!
//! Seeds are inert and inspectable. Shoots are active and scheduled/bound.
//! Never conflate the two in APIs or architecture.

/// Canonical Seed (Motor artifact) ABI version.
pub const SEED_ABI_VERSION: u32 = 1;

/// Back-compat alias — prefer [`SEED_ABI_VERSION`].
pub const MOTOR_DESCRIPTOR_ABI_VERSION: u32 = SEED_ABI_VERSION;

/// Name of the ELF symbol exported by every Seed binary.
///
/// The system discovers Seeds by scanning for this symbol, then reads the
/// embedded [`Seed`] descriptor to determine what interfaces the binary
/// implements and how it can be hosted.
pub const SEED_SYMBOL: &str = "THINGOS_MOTOR";

/// Back-compat alias — prefer [`SEED_SYMBOL`].
pub const MOTOR_DESCRIPTOR_SYMBOL: &str = SEED_SYMBOL;

// ── Built-in interface identifiers ───────────────────────────────────────────

/// Interface id for `ProgramV1` — conventional one-shot executable program.
///
/// Required entry: `main(args: ArgVec) -> ExitCode`
pub const MOTOR_INTERFACE_PROGRAM_V1: u32 = 1;

/// Interface id for `LifecycleV1` — Motor that can be started/stopped by the host.
///
/// Required entries: `start(ctx: HostContext) -> Status`,
/// `stop(ctx: HostContext) -> Status`
pub const MOTOR_INTERFACE_LIFECYCLE_V1: u32 = 2;

/// Interface id for `DriverV1` — Motor that can be probed and bound as a driver.
///
/// Required entries: `probe(ctx, dev) -> ProbeDisposition`,
/// `bind(ctx, dev) -> BindResult`, `unbind(ctx, dev) -> Status`
pub const MOTOR_INTERFACE_DRIVER_V1: u32 = 3;

// ── Hosting mode bits ─────────────────────────────────────────────────────────

/// Hosting mode: Seed can be germinated as a regular program (ProgramV1).
pub const MOTOR_HOST_PROGRAM: u64 = 1 << 0;
/// Hosting mode: Seed can be hosted as a resident lifecycle service (LifecycleV1).
pub const MOTOR_HOST_LIFECYCLE: u64 = 1 << 1;
/// Hosting mode: Seed can be probed/bound as a driver (DriverV1).
pub const MOTOR_HOST_DRIVER: u64 = 1 << 2;

// ── Core types ────────────────────────────────────────────────────────────────

/// One declared interface inside a [`Seed`] descriptor.
///
/// Each entry names an interface by `(interface_id, interface_version)` and
/// optionally supplies the entry surface symbol the system calls to enter the
/// Seed through that interface.
///
/// ### Entry symbol semantics
///
/// - For [`MOTOR_INTERFACE_PROGRAM_V1`]: an empty (`len == 0`) entry symbol
///   means "use the ELF default entry", preserving plain `main`/crt-style
///   program flow unchanged.
/// - For [`MOTOR_INTERFACE_DRIVER_V1`] and [`MOTOR_INTERFACE_LIFECYCLE_V1`]:
///   the host requires a non-empty entry symbol; binaries must provide one.
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

/// Back-compat alias — prefer [`SeedInterface`].
pub type MotorInterfaceDescriptor = SeedInterface;

/// Canonical immutable descriptor for a Motor artifact (**Seed**).
///
/// A Seed is a compiled ELF plus this descriptor exported as [`SEED_SYMBOL`]
/// (`THINGOS_MOTOR`). It is inert until the system *germinates* it into a
/// running **Shoot**.
///
/// The descriptor must be embedded in the binary as a `#[used]` static with
/// `#[unsafe(no_mangle)]` so it is visible in the ELF symbol table at load time.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Seed {
    /// Must equal [`SEED_ABI_VERSION`].
    pub abi_version: u32,
    /// Number of valid entries in `interfaces` (must be ≤ 4).
    pub interface_count: u32,
    /// Bitmask of `MOTOR_HOST_*` constants declaring how this Seed may be hosted.
    pub hosting_modes: u64,
    /// Reserved capability bitmask; set to 0.
    pub capabilities: u64,
    /// Pointer/length of the human-readable Seed name (UTF-8, no null terminator).
    pub motor_name_ptr: *const u8,
    pub motor_name_len: usize,
    /// Declared interface table; unused slots must be zeroed.
    pub interfaces: [SeedInterface; 4],
}

unsafe impl Sync for Seed {}
unsafe impl Send for Seed {}

/// Back-compat alias — prefer [`Seed`].
pub type MotorDescriptor = Seed;

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

    /// Returns `true` if this Seed declares [`MOTOR_INTERFACE_DRIVER_V1`].
    #[inline]
    pub fn implements_driver_v1(&self) -> bool {
        self.interface(MOTOR_INTERFACE_DRIVER_V1, 1).is_some()
    }

    /// Returns `true` if this Seed declares [`MOTOR_INTERFACE_PROGRAM_V1`].
    #[inline]
    pub fn implements_program_v1(&self) -> bool {
        self.interface(MOTOR_INTERFACE_PROGRAM_V1, 1).is_some()
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
            hosting_modes: MOTOR_HOST_PROGRAM,
            capabilities: 0,
            motor_name_ptr: core::ptr::null(),
            motor_name_len: 0,
            interfaces: [
                SeedInterface {
                    interface_id: MOTOR_INTERFACE_PROGRAM_V1,
                    interface_version: 1,
                    flags: 0,
                    reserved: 0,
                    entry_symbol_ptr: core::ptr::null(),
                    entry_symbol_len: 0,
                },
                SeedInterface {
                    interface_id: MOTOR_INTERFACE_DRIVER_V1,
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
        assert!(seed.interface(MOTOR_INTERFACE_LIFECYCLE_V1, 1).is_none());
    }

    #[test]
    fn backcompat_type_aliases_compile() {
        // Verify that the MotorDescriptor/MotorInterfaceDescriptor aliases
        // are still usable while callers migrate to Seed/SeedInterface.
        let _: MotorDescriptor = Seed {
            abi_version: MOTOR_DESCRIPTOR_ABI_VERSION,
            interface_count: 0,
            hosting_modes: 0,
            capabilities: 0,
            motor_name_ptr: core::ptr::null(),
            motor_name_len: 0,
            interfaces: [MotorInterfaceDescriptor::default(); 4],
        };
    }
}
