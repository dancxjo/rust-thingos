// ============================================================================
// Pointer Events
// ============================================================================

/// Mouse button identifier.
///
/// Used in [`MouseMessage::Button`] to identify which physical button changed.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MouseButton {
    Left = 0,
    Right = 1,
    Middle = 2,
}

impl MouseButton {
    /// Convert a raw button index to a [`MouseButton`].
    ///
    /// Returns `None` for unknown indices.
    pub fn from_raw(value: u8) -> Option<Self> {
        match value {
            0 => Some(MouseButton::Left),
            1 => Some(MouseButton::Right),
            2 => Some(MouseButton::Middle),
            _ => None,
        }
    }
}

/// Canonical typed mouse event.
///
/// This is the logical view of mouse input used throughout the new
/// message-passing path.  The wire representation is the `BristleEventHeader`
/// + one of the `Pointer*Payload` structs; this enum is the decoded,
/// strongly-typed form that producers build and consumers pattern-match on.
///
/// # Variants
/// * `Move`   — relative pointer motion in device units.
/// * `Button` — button press or release.
/// * `Scroll` — wheel or trackpad scroll in device units.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MouseMessage {
    /// Relative pointer movement.
    Move { dx: i16, dy: i16 },
    /// Button press (`pressed = true`) or release (`pressed = false`).
    Button { button: MouseButton, pressed: bool },
    /// Wheel / trackpad scroll.
    Scroll { dx: i16, dy: i16 },
}

/// Pointer move payload (4 bytes)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PointerMovePayload {
    pub dx: i16, // Relative X movement
    pub dy: i16, // Relative Y movement
}

impl PointerMovePayload {
    pub const SIZE: usize = core::mem::size_of::<Self>();

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut buf = [0u8; Self::SIZE];
        buf[0..2].copy_from_slice(&self.dx.to_le_bytes());
        buf[2..4].copy_from_slice(&self.dy.to_le_bytes());
        buf
    }

    pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Self {
        Self {
            dx: i16::from_le_bytes(bytes[0..2].try_into().unwrap()),
            dy: i16::from_le_bytes(bytes[2..4].try_into().unwrap()),
        }
    }
}

/// Pointer button payload (2 bytes)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PointerButtonPayload {
    pub button: u8, // 0=left, 1=right, 2=middle
    pub _pad: u8,
}

impl PointerButtonPayload {
    pub const SIZE: usize = core::mem::size_of::<Self>();

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        [self.button, self._pad]
    }

    pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Self {
        Self {
            button: bytes[0],
            _pad: bytes[1],
        }
    }
}

/// Scroll payload (4 bytes)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct ScrollPayload {
    pub dx: i16, // Horizontal scroll
    pub dy: i16, // Vertical scroll
}

impl ScrollPayload {
    pub const SIZE: usize = core::mem::size_of::<Self>();

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut buf = [0u8; Self::SIZE];
        buf[0..2].copy_from_slice(&self.dx.to_le_bytes());
        buf[2..4].copy_from_slice(&self.dy.to_le_bytes());
        buf
    }

    pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Self {
        Self {
            dx: i16::from_le_bytes(bytes[0..2].try_into().unwrap()),
            dy: i16::from_le_bytes(bytes[2..4].try_into().unwrap()),
        }
    }
}
