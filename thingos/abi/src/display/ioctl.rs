//! Display Device Ops (IOCTLs)

/// Retrieve display device information and capabilities.
/// Output: DisplayInfo
pub const DISPLAY_OP_GET_INFO: u32 = 1;

/// Import a shared memory buffer.
/// Input: BufferHandle
/// Output: BufferId (via u32 return value or out_ptr)
pub const DISPLAY_OP_IMPORT_BUFFER: u32 = 2;

/// Release an imported buffer.
/// Input: BufferId
pub const DISPLAY_OP_RELEASE_BUFFER: u32 = 3;

/// Atomic presentation commit.
/// Input: CommitRequest
pub const DISPLAY_OP_COMMIT: u32 = 4;

/// Set display mode.
/// Input: DisplayMode
pub const DISPLAY_OP_SET_MODE: u32 = 5;

/// Upload a hardware cursor image and configure its hotspot.
///
/// Only valid when `DisplayCaps::HARDWARE_CURSOR` is set.
/// Input: SetCursorRequest
pub const DISPLAY_OP_SET_CURSOR: u32 = 6;

/// Move the hardware cursor to a new screen position without triggering a
/// full scene recomposition.
///
/// Only valid when `DisplayCaps::HARDWARE_CURSOR` is set.  The position
/// corresponds to the cursor *hotspot* on screen; the driver uses the hotspot
/// configured by the last `DISPLAY_OP_SET_CURSOR` call to compute where to
/// paint the cursor image.
/// Input: MoveCursorRequest
pub const DISPLAY_OP_MOVE_CURSOR: u32 = 7;

/// Submit a batch of 2D acceleration commands.
///
/// The payload begins with an [`super::accel2d::Accel2dBatch`] header
/// immediately followed by `cmd_count` ×
/// [`super::accel2d::Accel2dCommand`] records (each
/// [`super::accel2d::ACCEL2D_COMMAND_SIZE`] bytes).
///
/// Supported command kinds are indicated by the `ACCEL2D_*` bits in
/// [`super::types::DisplayCaps`].  The driver returns `ENOSYS` for any
/// command kind it does not implement.
pub const DISPLAY_OP_ACCEL2D: u32 = 8;
