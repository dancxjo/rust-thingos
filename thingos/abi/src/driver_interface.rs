/// Driver Interface ABI v1
///
/// A binary is classified as a driver-capable image by the system if it
/// exports a globally-visible symbol named `THING_DRIVER_V1` whose type is
/// [`DriverInterfaceV1`].  `cambium` scans candidate binaries for this symbol,
/// reads the metadata, and uses it to match devices and locate the driver
/// entrypoint.
///
/// # Example
///
/// ```rust,no_run
/// use abi::driver_interface::{DriverInterfaceV1, DRIVER_INTERFACE_ABI_VERSION};
///
/// #[no_mangle]
/// #[used]
/// pub static THING_DRIVER_V1: DriverInterfaceV1 = DriverInterfaceV1 {
///     abi_version: DRIVER_INTERFACE_ABI_VERSION,
///     flags: 0,
///     vendor_id: 0x1AF4,        // match only this PCI vendor (0 = any)
///     device_id: 0,             // match any device from this vendor
///     class_code: 0,            // match any class (0 = any)
///     class_mask: 0,
///     entry_symbol: *b"thing_driver_entry_v1\0\0\0\0\0\0\0\0\0\0\0",
/// };
///
/// #[no_mangle]
/// pub extern "C" fn thing_driver_entry_v1(ctx_ptr: u64, ctx_len: u32) -> i32 {
///     // driver activation logic
///     0
/// }
/// ```
///
/// # Lifecycle
///
/// - Normal process execution (`main`) and driver entry (`thing_driver_entry_v1`)
///   are independent: `cambium` spawns a **new process instance** that enters the
///   driver entrypoint directly; `main` is not called.
/// - Driver logic runs as a normal kernel-scheduled task; there is no
///   in-process function jump from `cambium`.
/// - Arguments are passed via the stable [`DriverEntryCtx`] payload whose
///   address is passed in `ctx_ptr`.

/// ABI version constant embedded in every [`DriverInterfaceV1`] record.
pub const DRIVER_INTERFACE_ABI_VERSION: u32 = 1;

/// ABI version for [`DriverDescriptor`] exported as [`DRIVER_DESCRIPTOR_SYMBOL`].
pub const DRIVER_DESCRIPTOR_ABI_VERSION: u32 = 1;

/// Name of the well-known global symbol that marks a binary as driver-capable.
pub const DRIVER_MARKER_SYMBOL: &str = "THING_DRIVER_V1";

/// Name of the stable descriptor symbol exported by v2-style drivers.
pub const DRIVER_DESCRIPTOR_SYMBOL: &str = "THINGOS_DRIVER";

/// Name of the default driver entrypoint invoked by `cambium`.
pub const DRIVER_ENTRY_SYMBOL: &str = "thing_driver_entry_v1";

/// Match-any sentinel for `vendor_id` and `device_id`.
pub const DRIVER_MATCH_ANY_ID: u16 = 0;

/// Match-any sentinel for `class_code`.
pub const DRIVER_MATCH_ANY_CLASS: u32 = 0;

/// Flag: the binary can serve as a driver for PCI devices.
pub const DRIVER_FLAG_PCI: u32 = 1 << 0;

/// Flag: the binary can serve as a driver for platform/non-PCI devices.
pub const DRIVER_FLAG_PLATFORM: u32 = 1 << 1;

/// Opaque host/kernel-owned capability handle.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Handle(pub u64);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Ok = 0,
    NotDriver = -1,
    NoMatch = -2,
    Unsupported = -3,
    InvalidArgument = -4,
    BindFailed = -5,
    PublishFailed = -6,
    InternalError = -7,
}

impl Status {
    #[inline]
    pub const fn from_i32(code: i32) -> Self {
        match code {
            0 => Self::Ok,
            -1 => Self::NotDriver,
            -2 => Self::NoMatch,
            -3 => Self::Unsupported,
            -4 => Self::InvalidArgument,
            -5 => Self::BindFailed,
            -6 => Self::PublishFailed,
            _ => Self::InternalError,
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverClass {
    Unknown = 0,
    Net = 1,
    Block = 2,
    Display = 3,
    Input = 4,
    Audio = 5,
    Serial = 6,
    Other = 0xffff_ffff,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusKind {
    Unknown = 0,
    Pci = 1,
    Virtio = 2,
    Platform = 3,
    Usb = 4,
    Isa = 5,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportKind {
    Unknown = 0,
    Mmio = 1,
    Pio = 2,
    VirtioPci = 3,
    VirtioMmio = 4,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct DeviceLocator {
    pub bus: u32,
    pub slot: u32,
    pub function: u32,
    pub instance: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct MmioRegion {
    pub base: u64,
    pub length: u64,
    pub flags: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct DeviceCapability {
    pub key: u32,
    pub value: u32,
    pub data0: u64,
    pub data1: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DeviceInfo {
    pub bus: u32,
    pub locator: DeviceLocator,
    pub vendor_id: u32,
    pub device_id: u32,
    pub subsystem_vendor_id: u32,
    pub subsystem_device_id: u32,
    pub class_code: u32,
    pub transport: u32,
    pub irq: u32,
    pub mmio_regions_ptr: *const MmioRegion,
    pub mmio_regions_len: usize,
    pub caps_ptr: *const DeviceCapability,
    pub caps_len: usize,
}

impl Default for DeviceInfo {
    fn default() -> Self {
        Self {
            bus: BusKind::Unknown as u32,
            locator: DeviceLocator::default(),
            vendor_id: 0,
            device_id: 0,
            subsystem_vendor_id: 0,
            subsystem_device_id: 0,
            class_code: 0,
            transport: TransportKind::Unknown as u32,
            irq: 0,
            mmio_regions_ptr: core::ptr::null(),
            mmio_regions_len: 0,
            caps_ptr: core::ptr::null(),
            caps_len: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ProbeResult {
    pub matched: u32,
    pub score: u32,
    pub claimed_class: DriverClass,
    pub flags: u64,
}

impl Default for ProbeResult {
    fn default() -> Self {
        Self { matched: 0, score: 0, claimed_class: DriverClass::Unknown, flags: 0 }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DriverHostVtable {
    pub version: u32,
    pub log: unsafe extern "C" fn(level: LogLevel, ptr: *const u8, len: usize),
    pub publish_devnode: unsafe extern "C" fn(
        devfs_root: Handle,
        path_ptr: *const u8,
        path_len: usize,
        service_handle: Handle,
    ) -> Status,
    pub create_port:
        unsafe extern "C" fn(out_server: *mut Handle, out_client: *mut Handle) -> Status,
    pub create_stream_pair: unsafe extern "C" fn(out_a: *mut Handle, out_b: *mut Handle) -> Status,
    pub wait: unsafe extern "C" fn(
        handles_ptr: *const Handle,
        handles_len: usize,
        timeout_ms: u64,
        out_index: *mut usize,
    ) -> Status,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DriverStartContext {
    pub abi_version: u32,
    pub device: DeviceInfo,
    pub device_handle: Handle,
    pub devfs_root: Handle,
    pub event_port: Handle,
    pub host: DriverHostVtable,
    pub publish_name_ptr: *const u8,
    pub publish_name_len: usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DriverDescriptor {
    pub abi_version: u32,
    pub driver_name_ptr: *const u8,
    pub driver_name_len: usize,
    pub driver_class: DriverClass,
    pub flags: u64,
    pub probe: unsafe extern "C" fn(dev: *const DeviceInfo, out: *mut ProbeResult) -> Status,
    pub start: unsafe extern "C" fn(ctx: *const DriverStartContext) -> Status,
}

unsafe impl Sync for DriverDescriptor {}
unsafe impl Send for DriverDescriptor {}

/// Marker struct exported as `THING_DRIVER_V1` by driver-capable binaries.
///
/// `cambium` locates this symbol by walking the ELF symbol table of each binary
/// in `/bin` (and `/drivers` when present).  The presence of the symbol alone
/// is sufficient to classify the binary as a driver; the fields are used only
/// for device matching and entrypoint resolution.
///
/// ## ABI stability
///
/// This struct is `#[repr(C)]` and its size/layout is part of the v1 ABI.
/// Fields marked `_reserved` must be zeroed by the declaring binary; `cambium`
/// ignores them.  New fields will be added in a v2 struct.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DriverInterfaceV1 {
    /// Must equal [`DRIVER_INTERFACE_ABI_VERSION`] (= 1).
    pub abi_version: u32,

    /// Bitmask of [`DRIVER_FLAG_*`] constants.  Zero means "no specific flags".
    pub flags: u32,

    /// PCI vendor ID to match (`DRIVER_MATCH_ANY_ID` = 0 matches all vendors).
    pub vendor_id: u16,

    /// PCI device ID to match (`DRIVER_MATCH_ANY_ID` = 0 matches any device
    /// from the selected vendor).
    pub device_id: u16,

    /// PCI class code to match (bits 23:16 = class, 15:8 = subclass, 7:0 =
    /// prog-if).  `DRIVER_MATCH_ANY_CLASS` = 0 matches any class.
    pub class_code: u32,

    /// Bitmask applied to `class_code` before comparison.  Zero means apply
    /// no masking (equivalent to an exact match when `class_code` is non-zero).
    pub class_mask: u32,

    /// Null-terminated name of the driver entrypoint symbol, padded to 32
    /// bytes.  When the first byte is NUL `cambium` falls back to the default
    /// symbol name [`DRIVER_ENTRY_SYMBOL`].
    pub entry_symbol: [u8; 32],
}

impl DriverInterfaceV1 {
    /// Return the driver entrypoint symbol name as a `&str`, falling back to
    /// [`DRIVER_ENTRY_SYMBOL`] when the field is empty.
    pub fn entry_symbol_name(&self) -> &str {
        let end = self.entry_symbol.iter().position(|&b| b == 0).unwrap_or(32);
        if end == 0 {
            return DRIVER_ENTRY_SYMBOL;
        }
        core::str::from_utf8(&self.entry_symbol[..end]).unwrap_or(DRIVER_ENTRY_SYMBOL)
    }

    /// Returns `true` if this record matches the given PCI device identifiers.
    ///
    /// The matching rules are:
    /// - `vendor_id == DRIVER_MATCH_ANY_ID` → accept any vendor.
    /// - `device_id == DRIVER_MATCH_ANY_ID` → accept any device from the matched vendor.
    /// - `class_code == DRIVER_MATCH_ANY_CLASS` → accept any class code.
    /// - Otherwise each field must match exactly (after masking for `class_code`).
    pub fn matches_pci(&self, vendor: u16, device: u16, class: u32) -> bool {
        if self.vendor_id != DRIVER_MATCH_ANY_ID && self.vendor_id != vendor {
            return false;
        }
        if self.device_id != DRIVER_MATCH_ANY_ID && self.device_id != device {
            return false;
        }
        if self.class_code != DRIVER_MATCH_ANY_CLASS {
            let masked_class =
                class & if self.class_mask != 0 { self.class_mask } else { 0xFFFFFF };
            let masked_expected =
                self.class_code & if self.class_mask != 0 { self.class_mask } else { 0xFFFFFF };
            if masked_class != masked_expected {
                return false;
            }
        }
        true
    }
}

/// Context block passed to a driver entrypoint (`thing_driver_entry_v1`).
///
/// `cambium` serialises this struct into a memfd page, then passes its address
/// as `ctx_ptr` and its size as `ctx_len` when spawning the driver task via
/// `SYS_SPAWN_PROCESS_EX`.  The driver maps the memfd and reads its context
/// from there.
///
/// ## Versioning
///
/// `version` will always be the first field.  Future versions will increment
/// this field.  Drivers should check `version` before accessing any fields
/// beyond those present in the version they were compiled against.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DriverEntryCtx {
    /// Struct version.  Currently always 1.
    pub version: u32,

    /// PCI vendor ID of the device being served.
    pub vendor_id: u16,

    /// PCI device ID of the device being served.
    pub device_id: u16,

    /// PCI class code (class:subclass:prog-if packed as `u32`).
    pub class_code: u32,

    pub _reserved0: u32,

    /// Null-terminated VFS path to the sysfs device directory
    /// (e.g. `/sys/devices/pci-0000:00:01.0`), padded to 128 bytes.
    pub device_path: [u8; 128],
}

impl Default for DriverEntryCtx {
    fn default() -> Self {
        Self {
            version: 0,
            vendor_id: 0,
            device_id: 0,
            class_code: 0,
            _reserved0: 0,
            device_path: [0u8; 128],
        }
    }
}

impl DriverEntryCtx {
    /// Return the device path as a `&str`, or `""` on invalid UTF-8.
    pub fn device_path_str(&self) -> &str {
        let end = self.device_path.iter().position(|&b| b == 0).unwrap_or(128);
        core::str::from_utf8(&self.device_path[..end]).unwrap_or("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn driver_interface_v1_size_is_stable() {
        // This test enforces ABI stability: if you add fields you must bump
        // the version to v2 and add a new struct.
        assert_eq!(core::mem::size_of::<DriverInterfaceV1>(), 52);
    }

    #[test]
    fn driver_entry_ctx_size_is_stable() {
        assert_eq!(core::mem::size_of::<DriverEntryCtx>(), 144);
    }

    #[test]
    fn entry_symbol_name_returns_default_when_empty() {
        let iface = DriverInterfaceV1 {
            abi_version: DRIVER_INTERFACE_ABI_VERSION,
            flags: 0,
            vendor_id: 0,
            device_id: 0,
            class_code: 0,
            class_mask: 0,
            entry_symbol: [0u8; 32],
        };
        assert_eq!(iface.entry_symbol_name(), DRIVER_ENTRY_SYMBOL);
    }

    #[test]
    fn entry_symbol_name_returns_custom() {
        let mut sym = [0u8; 32];
        sym[..6].copy_from_slice(b"my_drv");
        let iface = DriverInterfaceV1 {
            abi_version: DRIVER_INTERFACE_ABI_VERSION,
            flags: 0,
            vendor_id: 0,
            device_id: 0,
            class_code: 0,
            class_mask: 0,
            entry_symbol: sym,
        };
        assert_eq!(iface.entry_symbol_name(), "my_drv");
    }

    #[test]
    fn matches_pci_any_vendor() {
        let iface = DriverInterfaceV1 {
            abi_version: DRIVER_INTERFACE_ABI_VERSION,
            flags: 0,
            vendor_id: 0,
            device_id: 0,
            class_code: 0,
            class_mask: 0,
            entry_symbol: [0u8; 32],
        };
        assert!(iface.matches_pci(0x1AF4, 0x1000, 0x020000));
        assert!(iface.matches_pci(0x8086, 0x0000, 0x000000));
    }

    #[test]
    fn matches_pci_specific_vendor() {
        let iface = DriverInterfaceV1 {
            abi_version: DRIVER_INTERFACE_ABI_VERSION,
            flags: DRIVER_FLAG_PCI,
            vendor_id: 0x1AF4,
            device_id: 0,
            class_code: 0,
            class_mask: 0,
            entry_symbol: [0u8; 32],
        };
        assert!(iface.matches_pci(0x1AF4, 0x1000, 0x000000));
        assert!(!iface.matches_pci(0x8086, 0x1000, 0x000000));
    }

    #[test]
    fn matches_pci_vendor_and_device() {
        let iface = DriverInterfaceV1 {
            abi_version: DRIVER_INTERFACE_ABI_VERSION,
            flags: DRIVER_FLAG_PCI,
            vendor_id: 0x1AF4,
            device_id: 0x1050,
            class_code: 0,
            class_mask: 0,
            entry_symbol: [0u8; 32],
        };
        assert!(iface.matches_pci(0x1AF4, 0x1050, 0x000000));
        assert!(!iface.matches_pci(0x1AF4, 0x1000, 0x000000));
    }

    #[test]
    fn matches_pci_class_code() {
        let iface = DriverInterfaceV1 {
            abi_version: DRIVER_INTERFACE_ABI_VERSION,
            flags: DRIVER_FLAG_PCI,
            vendor_id: 0,
            device_id: 0,
            class_code: 0x020000,
            class_mask: 0xFF0000,
            entry_symbol: [0u8; 32],
        };
        assert!(iface.matches_pci(0x8086, 0x0000, 0x020000));
        assert!(iface.matches_pci(0x1AF4, 0xFFFF, 0x020010));
        assert!(!iface.matches_pci(0x8086, 0x0000, 0x010000));
    }
}
