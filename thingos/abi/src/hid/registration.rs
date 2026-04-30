//! Bristle control-plane registration protocol.
//!
//! Consumers (bloom, echo) send [`KIND_BRISTLE_REGISTER_SINK`] inbox messages
//! to bristle's PID to subscribe for normalized input events.  Bristle writes
//! its PID to `/run/bristle/pid` at startup so consumers can discover it.

// ── RegisterSink message ──────────────────────────────────────────────────────

/// `KindId` for the `RegisterSink` inbox message sent to bristle.
///
/// Payload layout (5 bytes):
/// ```text
/// byte 0      : tag  — BRISTLE_SINK_TAG_BLOOM or BRISTLE_SINK_TAG_ECHO
/// bytes 1..=4 : port write handle as u32 little-endian
/// ```
/// Bristle bridges the handle to a VFS FD and writes normalized bristle
/// events to it whenever input arrives from a device.
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

/// Sink tag identifying the bloom compositor as the target.
pub const BRISTLE_SINK_TAG_BLOOM: u8 = 0;

/// Sink tag identifying the echo / input-echo consumer as the target.
pub const BRISTLE_SINK_TAG_ECHO: u8 = 1;

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

/// Decode a `RegisterSink` payload, returning `(tag, handle)`.
///
/// Returns `None` if `bytes` is shorter than 5 bytes.
#[inline]
pub fn decode_register_sink(bytes: &[u8]) -> Option<(u8, u32)> {
    if bytes.len() < 5 {
        return None;
    }
    let tag = bytes[0];
    let handle = u32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
    Some((tag, handle))
}
