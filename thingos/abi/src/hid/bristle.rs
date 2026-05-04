use super::{BRISTLE_EVENT_MAGIC, BRISTLE_EVENT_VERSION, EventType, HidParseError, Key, Mods};

/// Bristle event header (20 bytes).
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct BristleEventHeader {
    pub magic: u32,        // BRISTLE_EVENT_MAGIC
    pub version: u16,      // BRISTLE_EVENT_VERSION
    pub event_type: u16,   // EventType discriminant
    pub timestamp_ns: u64, // Monotonic timestamp
    pub payload_len: u32,  // Bytes following header
}

impl BristleEventHeader {
    pub const SIZE: usize = core::mem::size_of::<Self>();

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut buf = [0u8; Self::SIZE];
        buf[0..4].copy_from_slice(&self.magic.to_le_bytes());
        buf[4..6].copy_from_slice(&self.version.to_le_bytes());
        buf[6..8].copy_from_slice(&self.event_type.to_le_bytes());
        buf[8..16].copy_from_slice(&self.timestamp_ns.to_le_bytes());
        buf[16..20].copy_from_slice(&self.payload_len.to_le_bytes());
        buf
    }

    pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Result<Self, HidParseError> {
        let header = Self {
            magic: u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            version: u16::from_le_bytes(bytes[4..6].try_into().unwrap()),
            event_type: u16::from_le_bytes(bytes[6..8].try_into().unwrap()),
            timestamp_ns: u64::from_le_bytes(bytes[8..16].try_into().unwrap()),
            payload_len: u32::from_le_bytes(bytes[16..20].try_into().unwrap()),
        };

        if header.magic != BRISTLE_EVENT_MAGIC {
            return Err(HidParseError::BadMagic);
        }
        if header.version != BRISTLE_EVENT_VERSION {
            return Err(HidParseError::BadVersion);
        }
        EventType::from_raw(header.event_type)?;
        Ok(header)
    }
}

/// KeyDown/KeyUp payload (4 bytes)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct KeyEventPayload {
    pub key: u16,  // Key enum value
    pub mods: u8,  // Mods bitmask
    pub flags: u8, // bit0 = repeat
}

impl KeyEventPayload {
    pub const SIZE: usize = core::mem::size_of::<Self>();

    pub fn key(&self) -> Key {
        Key::from_raw(self.key)
    }
    pub fn mods(&self) -> Mods {
        Mods(self.mods)
    }
    pub fn is_repeat(&self) -> bool {
        self.flags & 1 != 0
    }

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut buf = [0u8; Self::SIZE];
        buf[0..2].copy_from_slice(&self.key.to_le_bytes());
        buf[2] = self.mods;
        buf[3] = self.flags;
        buf
    }

    pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Self {
        Self {
            key: u16::from_le_bytes(bytes[0..2].try_into().unwrap()),
            mods: bytes[2],
            flags: bytes[3],
        }
    }
}

// ============================================================================
// Canonical Typed Keyboard Event
// ============================================================================

/// Canonical typed keyboard event.
///
/// This is the logical, strongly-typed view of keyboard input used throughout
/// the new message-passing path.  The wire encoding remains the
/// `BristleEventHeader` + `KeyEventPayload` structs (unchanged); this enum is
/// the decoded form that producers build and consumers pattern-match on.
///
/// # Physical-key vs Text semantics
///
/// Thing-OS currently uses a **single unified stream** of physical key events.
/// Text generation (character mapping, IME) is intentionally left to
/// consumers: they receive `Key` + `Mods` and apply their own layout logic.
/// A future `Text` variant can be added to this enum if a text-production
/// layer is introduced upstream of consumers, but it is **not** added here to
/// avoid implying that Bristle performs layout translation today.
///
/// # Variants
/// * `Key` — a physical key press or release with full modifier and repeat state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyboardMessage {
    /// Physical key event.
    ///
    /// `pressed = true` for key-down, `false` for key-up.
    /// `repeat = true` when the key was already held (auto-repeat).
    Key { key: Key, pressed: bool, modifiers: Mods, repeat: bool },
}
