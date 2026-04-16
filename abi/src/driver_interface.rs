/// Driver Interface ABI v1
///
/// A binary is classified as a driver-capable image by the system if it
/// exports a globally-visible symbol named `THING_DRIVER_V1` whose type is
/// [`DriverInterfaceV1`].  `devd` scans candidate binaries for this symbol,
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
///   are independent: `devd` spawns a **new process instance** that enters the
///   driver entrypoint directly; `main` is not called.
/// - Driver logic runs as a normal kernel-scheduled task; there is no
///   in-process function jump from `devd`.
/// - Arguments are passed via the stable [`DriverEntryCtx`] payload whose
///   address is passed in `ctx_ptr`.

/// ABI version constant embedded in every [`DriverInterfaceV1`] record.
pub const DRIVER_INTERFACE_ABI_VERSION: u32 = 1;

/// Name of the well-known global symbol that marks a binary as driver-capable.
pub const DRIVER_MARKER_SYMBOL: &str = "THING_DRIVER_V1";

/// Name of the default driver entrypoint invoked by `devd`.
pub const DRIVER_ENTRY_SYMBOL: &str = "thing_driver_entry_v1";

/// Match-any sentinel for `vendor_id` and `device_id`.
pub const DRIVER_MATCH_ANY_ID: u16 = 0;

/// Match-any sentinel for `class_code`.
pub const DRIVER_MATCH_ANY_CLASS: u32 = 0;

/// Flag: the binary can serve as a driver for PCI devices.
pub const DRIVER_FLAG_PCI: u32 = 1 << 0;

/// Flag: the binary can serve as a driver for platform/non-PCI devices.
pub const DRIVER_FLAG_PLATFORM: u32 = 1 << 1;

/// Marker struct exported as `THING_DRIVER_V1` by driver-capable binaries.
///
/// `devd` locates this symbol by walking the ELF symbol table of each binary
/// in `/bin` (and `/drivers` when present).  The presence of the symbol alone
/// is sufficient to classify the binary as a driver; the fields are used only
/// for device matching and entrypoint resolution.
///
/// ## ABI stability
///
/// This struct is `#[repr(C)]` and its size/layout is part of the v1 ABI.
/// Fields marked `_reserved` must be zeroed by the declaring binary; `devd`
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
    /// bytes.  When the first byte is NUL `devd` falls back to the default
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
            let masked_class = class & if self.class_mask != 0 { self.class_mask } else { 0xFFFFFF };
            let masked_expected = self.class_code & if self.class_mask != 0 { self.class_mask } else { 0xFFFFFF };
            if masked_class != masked_expected {
                return false;
            }
        }
        true
    }
}

/// Context block passed to a driver entrypoint (`thing_driver_entry_v1`).
///
/// `devd` serialises this struct into a memfd page, then passes its address
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
#[derive(Debug, Clone, Copy, Default)]
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
        assert_eq!(core::mem::size_of::<DriverInterfaceV1>(), 48);
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
