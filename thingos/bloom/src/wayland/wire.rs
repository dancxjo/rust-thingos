//! Wayland wire protocol codec.
//!
//! Encodes and decodes the Wayland binary wire format:
//!
//! ```text
//! +--------+--------+--------+
//! |object_id (u32)  |sz | op |   header (8 bytes, little-endian)
//! +--------+--------+--------+
//! | payload bytes …          |   (sz - 8) bytes
//! +--------+--------+--------+
//! ```
//!
//! `sz` is the total message size in bytes (header + payload), encoded in the
//! upper 16 bits of the second u32.  `op` is the opcode in the lower 16 bits.
//!
//! All integers in the payload are native-endian (host byte order), matching
//! how `wayland_hello` encodes them.

use alloc::vec::Vec;

/// A fully decoded Wayland message.
#[derive(Debug, Clone)]
pub struct WireMsg {
    pub object_id: u32,
    pub opcode: u16,
    /// Payload bytes (does NOT include the 8-byte header).
    pub data: Vec<u8>,
}

/// Encode a Wayland message to bytes (header + payload).
pub fn encode(object_id: u32, opcode: u16, payload: &[u8]) -> Vec<u8> {
    let size = (8 + payload.len()) as u16;
    let mut buf = Vec::with_capacity(8 + payload.len());
    buf.extend_from_slice(&object_id.to_ne_bytes());
    let size_op: u32 = ((size as u32) << 16) | (opcode as u32);
    buf.extend_from_slice(&size_op.to_ne_bytes());
    buf.extend_from_slice(payload);
    buf
}

/// Encode a string as a Wayland `string` argument (length-prefixed, NUL-terminated, 4-byte-aligned).
pub fn encode_string(s: &str) -> Vec<u8> {
    let len = s.len() as u32 + 1; // +1 for NUL
    let mut bytes: Vec<u8> = Vec::with_capacity(4 + len as usize);
    bytes.extend_from_slice(&len.to_ne_bytes());
    bytes.extend_from_slice(s.as_bytes());
    bytes.push(0u8); // NUL terminator
    // Pad to 4-byte boundary.
    while bytes.len() % 4 != 0 {
        bytes.push(0u8);
    }
    bytes
}

/// Encode a Wayland `array` argument (length-prefixed, 4-byte-aligned).
pub fn encode_array(data: &[u8]) -> Vec<u8> {
    let len = data.len() as u32;
    let mut bytes: Vec<u8> = Vec::with_capacity(4 + data.len());
    bytes.extend_from_slice(&len.to_ne_bytes());
    bytes.extend_from_slice(data);
    while bytes.len() % 4 != 0 {
        bytes.push(0u8);
    }
    bytes
}

/// Try to parse one complete Wayland message from the front of `buf`.
///
/// Returns `Some((msg, bytes_consumed))` when a complete message is available,
/// or `None` if the buffer contains fewer bytes than the message header
/// indicates.
pub fn decode_one(buf: &[u8]) -> Option<(WireMsg, usize)> {
    if buf.len() < 8 {
        return None;
    }
    let object_id = u32::from_ne_bytes(buf[0..4].try_into().ok()?);
    let size_op = u32::from_ne_bytes(buf[4..8].try_into().ok()?);
    let size = (size_op >> 16) as usize;
    let opcode = (size_op & 0xFFFF) as u16;

    if size < 8 || buf.len() < size {
        return None;
    }

    let data = buf[8..size].to_vec();
    Some((WireMsg { object_id, opcode, data }, size))
}

/// Read a u32 from `buf` at `offset`.
#[inline]
pub fn read_u32(buf: &[u8], offset: usize) -> Option<u32> {
    buf.get(offset..offset + 4).and_then(|b| b.try_into().ok()).map(u32::from_ne_bytes)
}

/// Read an i32 from `buf` at `offset`.
#[inline]
pub fn read_i32(buf: &[u8], offset: usize) -> Option<i32> {
    buf.get(offset..offset + 4).and_then(|b| b.try_into().ok()).map(i32::from_ne_bytes)
}

/// Read a length-prefixed Wayland string from `buf` at `offset`.
/// Returns `(string_bytes_without_nul, total_bytes_consumed)`.
pub fn read_string(buf: &[u8], offset: usize) -> Option<(&[u8], usize)> {
    let len = read_u32(buf, offset)? as usize;
    let start = offset + 4;
    let end = start + len;
    if buf.len() < end {
        return None;
    }
    // Strip NUL if present.
    let s = if len > 0 && buf[end - 1] == 0 { &buf[start..end - 1] } else { &buf[start..end] };
    // Total consumed = 4 (length) + len rounded up to 4-byte boundary.
    let padded = (len + 3) & !3;
    Some((s, 4 + padded))
}
