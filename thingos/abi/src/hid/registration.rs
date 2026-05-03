//! Bristle control-plane registration protocol.
//!
//! Consumers (bloom, echo) send [`KIND_BRISTLE_REGISTER_SINK`] inbox messages
//! to bristle's PID to subscribe for normalized input events.  Bristle writes
//! its PID to `/run/bristle/pid` at startup so consumers can discover it.

// ── RegisterSink message ──────────────────────────────────────────────────────

/// `KindId` for the `RegisterSink` inbox message sent to bristle.
///
/// Payload layout v1 (5 bytes, still accepted):
/// ```text
/// byte 0      : tag  — BRISTLE_SINK_TAG_BLOOM or BRISTLE_SINK_TAG_ECHO
/// bytes 1..=4 : port write handle as u32 little-endian
/// ```
///
/// Payload layout v2 (6 bytes):
/// ```text
/// byte 0      : tag
/// bytes 1..=4 : port write handle as u32 little-endian
/// byte 5      : interest mask, see `BRISTLE_EVENT_CLASS_*`
/// ```
/// Bristle bridges the handle to a VFS FD and writes normalized bristle
/// events matching the interest mask whenever input arrives from a device.
pub const KIND_BRISTLE_REGISTER_SINK: [u8; 16] = [
    0xb4, 0x1c, 0xe8, 0x5a, 0x0f, 0x3d, 0x72, 0x9e, 0xc2, 0x58, 0x4a, 0x17, 0xd6, 0x83, 0xf0, 0x21,
];

/// `KindId` for raw device events sent by HID drivers to bristle.
///
/// Payload layout is one complete bristle event: [`BristleEventHeader`]
/// followed by the event-specific payload.
pub const KIND_BRISTLE_DEVICE_EVENT: [u8; 16] = [
    0x45, 0x89, 0x1a, 0x74, 0x8c, 0x2e, 0x4d, 0xa1, 0x97, 0x36, 0x5f, 0xc0, 0xe7, 0x18, 0xb2, 0x6d,
];

/// `KindId` for display bounds updates sent by the compositor to bristle.
///
/// Payload layout (8 bytes):
/// ```text
/// bytes 0..=3 : output width as u32 little-endian
/// bytes 4..=7 : output height as u32 little-endian
/// ```
///
/// Bristle uses this to track the active output dimensions after host-driven
/// display resizes.
pub const KIND_BRISTLE_SET_DISPLAY_BOUNDS: [u8; 16] = [
    0x2f, 0x91, 0x6b, 0x44, 0xa7, 0xc8, 0x4d, 0x1f, 0x92, 0x50, 0xd3, 0x7a, 0x65, 0x0e, 0x18, 0xbb,
];

/// Sink tag identifying the bloom compositor as the target.
pub const BRISTLE_SINK_TAG_BLOOM: u8 = 0;

/// Sink tag identifying the echo / input-echo consumer as the target.
pub const BRISTLE_SINK_TAG_ECHO: u8 = 1;

/// Sink interest bit for keyboard events.
pub const BRISTLE_EVENT_CLASS_KEYBOARD: u8 = 1 << 0;
/// Sink interest bit for pointer and scroll events.
pub const BRISTLE_EVENT_CLASS_POINTER: u8 = 1 << 1;
/// Subscribe to every currently defined input event class.
pub const BRISTLE_EVENT_CLASS_ALL: u8 = BRISTLE_EVENT_CLASS_KEYBOARD | BRISTLE_EVENT_CLASS_POINTER;

// ── RegisterSink payload helpers ──────────────────────────────────────────────

/// Encode a `RegisterSink` payload.
///
/// `tag` should be [`BRISTLE_SINK_TAG_BLOOM`] or [`BRISTLE_SINK_TAG_ECHO`].
/// `handle` is the port write handle that bristle should forward events to.
#[inline]
pub fn encode_register_sink(tag: u8, handle: u32) -> [u8; 5] {
    let h = handle.to_le_bytes();
    [tag, h[0], h[1], h[2], h[3]]
}

/// Encode a v2 `RegisterSink` payload with an explicit event-interest mask.
#[inline]
pub fn encode_register_sink_with_mask(tag: u8, handle: u32, event_mask: u8) -> [u8; 6] {
    let h = handle.to_le_bytes();
    [tag, h[0], h[1], h[2], h[3], event_mask]
}

/// Decode a `RegisterSink` payload, returning `(tag, handle)`.
///
/// Returns `None` if `bytes` is shorter than 5 bytes.
#[inline]
pub fn decode_register_sink(bytes: &[u8]) -> Option<(u8, u32)> {
    decode_register_sink_with_mask(bytes).map(|(tag, handle, _)| (tag, handle))
}

/// Decode a `RegisterSink` payload, returning `(tag, handle, event_mask)`.
///
/// Old 5-byte payloads default to [`BRISTLE_EVENT_CLASS_ALL`].
#[inline]
pub fn decode_register_sink_with_mask(bytes: &[u8]) -> Option<(u8, u32, u8)> {
    if bytes.len() < 5 {
        return None;
    }
    let tag = bytes[0];
    let handle = u32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
    let event_mask = bytes.get(5).copied().unwrap_or(BRISTLE_EVENT_CLASS_ALL);
    Some((tag, handle, event_mask))
}

// ── Display bounds payload helpers ───────────────────────────────────────────

/// Encode a display-bounds payload for [`KIND_BRISTLE_SET_DISPLAY_BOUNDS`].
#[inline]
pub fn encode_display_bounds(width: u32, height: u32) -> [u8; 8] {
    let w = width.to_le_bytes();
    let h = height.to_le_bytes();
    [w[0], w[1], w[2], w[3], h[0], h[1], h[2], h[3]]
}

/// Decode a display-bounds payload, returning `(width, height)`.
///
/// Returns `None` if `bytes` is shorter than 8 bytes.
#[inline]
pub fn decode_display_bounds(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.len() < 8 {
        return None;
    }
    let width = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    let height = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    Some((width, height))
}
