#![no_std]

// ── Normalized ACPI event record ──────────────────────────────────────────────

/// Fixed-size (16-byte) normalized ACPI event record.
///
/// Produced by ACPI event sources (acpi_ec, acpi_power) and consumed by
/// subscribers via `/services/acpi/events`.  The file uses simple fan-out
/// semantics: each reader tracks its own file position (in bytes), which the
/// kernel maintains per file-descriptor.  Position `N` corresponds to event
/// sequence `N / RECORD_SIZE`; the provider returns `EAGAIN` when the reader
/// is caught up to the current head.
///
/// ## Wire layout
///
/// | bytes  | field        | meaning                                          |
/// |--------|--------------|--------------------------------------------------|
/// | 0..8   | timestamp_ms | monotonic ms since boot                          |
/// | 8      | kind         | event kind discriminant (see `KIND_*` constants) |
/// | 9      | raw_code     | raw hardware byte (EC query, GPE number, …)      |
/// | 10     | source       | event source (see `SOURCE_*` constants)          |
/// | 11     | flags        | reserved — must be zero                          |
/// | 12..16 | extra        | optional extra data (LE u32)                     |
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AcpiEvent {
    /// Monotonic timestamp in milliseconds since boot.
    pub timestamp_ms: u64,
    /// Event kind discriminant (see `KIND_*` constants).
    pub kind: u8,
    /// Raw hardware code (EC query byte, GPE number, etc.).
    pub raw_code: u8,
    /// Event source (see `SOURCE_*` constants).
    pub source: u8,
    /// Flags / reserved — must be zero.
    pub flags: u8,
    /// Optional extra data (LE-encoded u32).
    pub extra: u32,
}

/// Serialized size of one [`AcpiEvent`] record in bytes.
pub const RECORD_SIZE: usize = 16;

// ── Event kind constants ──────────────────────────────────────────────────────

/// EC query event (SCI/query code from the embedded controller).
pub const KIND_EC_QUERY: u8      = 0x00;
/// PM1 fixed event: power button pressed.
pub const KIND_PM1_POWER_BTN: u8 = 0x01;
/// PM1 fixed event: sleep button pressed.
pub const KIND_PM1_SLEEP_BTN: u8 = 0x02;
/// Lid closed.
pub const KIND_LID_CLOSE: u8     = 0x03;
/// Lid opened.
pub const KIND_LID_OPEN: u8      = 0x04;
/// General-purpose event (GPE) notification.
pub const KIND_GPE: u8           = 0x05;
/// Unknown or unrecognised event — preserved rather than dropped.
pub const KIND_UNKNOWN: u8       = 0xFF;

// ── Event source constants ────────────────────────────────────────────────────

/// Event originated from the ACPI Embedded Controller.
pub const SOURCE_EC: u8        = 0x00;
/// Event originated from the PM1 fixed hardware registers.
pub const SOURCE_PM1: u8       = 0x01;
/// Event originated from a GPE block.
pub const SOURCE_GPE: u8       = 0x02;
/// Synthetic/injected event (tests, software simulation).
pub const SOURCE_SYNTHETIC: u8 = 0xFF;

// ── Serialization ─────────────────────────────────────────────────────────────

impl AcpiEvent {
    /// Serialise this record into a 16-byte little-endian buffer.
    pub fn to_bytes(self) -> [u8; RECORD_SIZE] {
        let mut buf = [0u8; RECORD_SIZE];
        buf[0..8].copy_from_slice(&self.timestamp_ms.to_le_bytes());
        buf[8]  = self.kind;
        buf[9]  = self.raw_code;
        buf[10] = self.source;
        buf[11] = self.flags;
        buf[12..16].copy_from_slice(&self.extra.to_le_bytes());
        buf
    }

    /// Deserialise from a byte slice (must be at least [`RECORD_SIZE`] bytes).
    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < RECORD_SIZE { return None; }
        Some(Self {
            timestamp_ms: u64::from_le_bytes(buf[0..8].try_into().ok()?),
            kind:     buf[8],
            raw_code: buf[9],
            source:   buf[10],
            flags:    buf[11],
            extra: u32::from_le_bytes(buf[12..16].try_into().ok()?),
        })
    }
}

#[cfg(test)]
mod event_tests {
    use super::*;

    #[test]
    fn round_trip_ec_query() {
        let ev = AcpiEvent {
            timestamp_ms: 12345,
            kind:     KIND_EC_QUERY,
            raw_code: 0x81,
            source:   SOURCE_EC,
            flags:    0,
            extra:    0,
        };
        let bytes = ev.to_bytes();
        assert_eq!(bytes.len(), RECORD_SIZE);
        let decoded = AcpiEvent::from_bytes(&bytes).unwrap();
        assert_eq!(decoded, ev);
    }

    #[test]
    fn round_trip_pm1_power_button() {
        let ev = AcpiEvent {
            timestamp_ms: 999_999,
            kind:     KIND_PM1_POWER_BTN,
            raw_code: 0,
            source:   SOURCE_PM1,
            flags:    0,
            extra:    0x1000,
        };
        let bytes = ev.to_bytes();
        let decoded = AcpiEvent::from_bytes(&bytes).unwrap();
        assert_eq!(decoded, ev);
    }

    #[test]
    fn from_bytes_rejects_short_slice() {
        assert!(AcpiEvent::from_bytes(&[0u8; 15]).is_none());
    }

    #[test]
    fn from_bytes_accepts_exact_size() {
        let buf = [0u8; RECORD_SIZE];
        assert!(AcpiEvent::from_bytes(&buf).is_some());
    }

    #[test]
    fn unknown_event_kind_preserved() {
        let ev = AcpiEvent {
            timestamp_ms: 0,
            kind:     KIND_UNKNOWN,
            raw_code: 0xAB,
            source:   SOURCE_EC,
            flags:    0,
            extra:    0,
        };
        let bytes = ev.to_bytes();
        let decoded = AcpiEvent::from_bytes(&bytes).unwrap();
        assert_eq!(decoded.kind, KIND_UNKNOWN);
        assert_eq!(decoded.raw_code, 0xAB);
    }
}

#[macro_export]
macro_rules! declare_acpi_driver {
    ($driver_name:expr, $device_kind:expr, $driver_class:expr, $online_message:expr, $wait_message:expr) => {
        use abi::driver_interface::{
            DRIVER_DESCRIPTOR_ABI_VERSION, DeviceInfo, DriverClass, DriverDescriptor,
            DriverEntryCtx, ProbeResult, Status,
        };
        use stem::abi::module_manifest::{
            MANIFEST_MAGIC, ManifestHeader, ModuleKind, device_kind_bytes,
        };

        const THINGOS_DRIVER_NAME: &[u8] = $driver_name;
        const THINGOS_DEVICE_KIND: &[u8] = $device_kind;

        #[cfg(target_arch = "x86_64")]
        unsafe extern "C" {
            fn thingos_driver_start_safe(ctx: *const DriverEntryCtx) -> Status;
        }

        #[unsafe(no_mangle)]
        #[used]
        pub static THINGOS_DRIVER: DriverDescriptor = DriverDescriptor {
            abi_version: DRIVER_DESCRIPTOR_ABI_VERSION,
            driver_name_ptr: THINGOS_DRIVER_NAME.as_ptr(),
            driver_name_len: THINGOS_DRIVER_NAME.len(),
            driver_class: $driver_class,
            flags: 0,
            probe: thingos_driver_probe,
            #[cfg(target_arch = "x86_64")]
            start: thingos_driver_start_safe
                as unsafe extern "C" fn(ctx: *const DriverEntryCtx) -> Status,
            #[cfg(not(target_arch = "x86_64"))]
            start: thingos_driver_start,
        };

        #[cfg(target_arch = "x86_64")]
        core::arch::global_asm!(
            r#"
    .section .text
    .global thingos_driver_start_safe
    thingos_driver_start_safe:
        sub rsp, 8
        push rdi
        call thingos_runtime_setup
        pop rdi
        add rsp, 8
        call thingos_driver_start_rust
        ret
"#
        );

        #[cfg(target_arch = "x86_64")]
        #[unsafe(no_mangle)]
        unsafe extern "C" fn thingos_driver_start_rust(ctx: *const DriverEntryCtx) -> Status {
            thingos_driver_start(ctx)
        }

        unsafe extern "C" fn thingos_driver_probe(
            _dev: *const DeviceInfo,
            out: *mut ProbeResult,
        ) -> Status {
            if out.is_null() {
                return Status::InvalidArgument;
            }
            let out = &mut *out;
            out.matched = 0;
            out.score = 0;
            out.claimed_class = $driver_class;
            out.flags = 0;
            Status::NoMatch
        }

        unsafe extern "C" fn thingos_driver_start(_ctx: *const DriverEntryCtx) -> Status {
            main(0)
        }

        #[unsafe(link_section = ".thing_manifest")]
        #[unsafe(no_mangle)]
        #[used]
        pub static MANIFEST: ManifestHeader = ManifestHeader {
            magic: MANIFEST_MAGIC,
            kind: ModuleKind::Driver,
            device_kind: device_kind_bytes(THINGOS_DEVICE_KIND),
            version: 1,
            _reserved: 0,
        };

        #[stem::main]
        fn main(_raw_arg: usize) -> ! {
            stem::info!($online_message);
            stem::debug!($wait_message);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };
}
