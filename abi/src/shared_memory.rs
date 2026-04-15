//! Shared memory descriptor types for the control-vs-bulk IPC split.
//!
//! # Doctrine
//!
//! Thing-OS uses a two-tier IPC model:
//!
//! - **Control plane** – small, latency-sensitive messages sent over channels
//!   (e.g. `channel_send` / `channel_recv`). Maximum size is the channel
//!   ring capacity (typically 4 KiB). Use this for commands, ACKs, events, and
//!   any metadata that directs how bulk data should be interpreted.
//!
//! - **Bulk plane** – large, throughput-sensitive data shared via memory-mapped
//!   things (`shared_memory`). The sender creates a shared memory thing, writes
//!   data into it (or keeps it as a persistent shared ring), and passes the
//!   thing to the receiver via `sendmsg`. The receiver calls
//!   `vm_map` to obtain a writable or read-only view without copying.
//!
//! # Shared memory lifecycle
//!
//! ```text
//! Creator                         Receiver
//! -------                         --------
//! shared_memory_create("name", size) -> thing
//! vm_map(thing, READ|WRITE)  -> ptr   (optional, if creator also writes)
//! [fill data at ptr]
//! sendmsg(thing, b"", &[thing]) --------->  recvmsg(thing) -> (b"", [new_thing])
//!                                        vm_map(new_thing, READ) -> ptr
//!                                        [read data at ptr]
//!                                        vm_unmap(ptr, size)
//!                                        vfs_close(new_thing)
//! vm_unmap(ptr, size)
//! vfs_close(thing)                    (thing dropped → physical memory freed)
//! ```
//!
//! The physical memory is reference-counted by the kernel: it is released only
//! when **all** things and all `vm_map` mappings that reference it have been
//! dropped.
//!
//! # Revocation
//!
//! A sender may stop sharing a memory region by closing its own thing. Existing
//! mappings in other processes remain valid until those processes call
//! `vm_unmap` or close their own thing copy. There is no forced-unmap
//! primitive; processes are expected to honour the protocol and unmap promptly
//! when the companion control message signals that the buffer is done.
//!
//! # Wire format
//!
//! [`SharedMemoryRef`] is the canonical inline descriptor used in control-plane
//! messages to identify a bulk-data region.  It encodes as 16 bytes
//! (little-endian):
//!
//! ```text
//! offset  size  field
//!      0     4  thing    – thing number (sender-local when embedded in a
//!                          message; the receiver obtains its own thing via
//!                          recvmsg before mapping)
//!      4     4  _pad     – reserved, must be zero
//!      8     8  length   – byte length of the valid data window
//! ```
//!
//! For display / pixel-buffer use, callers should use [`BufferHandle`] in
//! `abi::display::types` which extends this with width, height, stride, and
//! format fields.
//!
//! [`BufferHandle`]: crate::display::types::BufferHandle

/// Canonical inline descriptor for a bulk-data shared memory region.
///
/// Embed this in any control-plane message that accompanies a shared memory transfer.
/// The `thing` field is the sender's local thing number; the physical backing is
/// transferred by passing the thing over the channel with `sendmsg`.
///
/// Wire size: [`SHARED_MEMORY_REF_WIRE_SIZE`] bytes (little-endian).
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SharedMemoryRef {
    /// Sender-local thing number.
    pub thing: u32,
    /// Reserved padding (must be zero).
    pub _pad: u32,
    /// Byte length of the valid data window within the shared memory.
    pub length: u64,
}

/// Byte size of the [`SharedMemoryRef`] wire encoding.
pub const SHARED_MEMORY_REF_WIRE_SIZE: usize = 16;

impl SharedMemoryRef {
    /// Create a new descriptor.
    #[inline]
    pub const fn new(thing: u32, length: u64) -> Self {
        Self {
            thing,
            _pad: 0,
            length,
        }
    }

    /// Encode to little-endian bytes.  Returns the number of bytes written, or
    /// `None` if `out` is too small.
    pub fn encode_le(&self, out: &mut [u8]) -> Option<usize> {
        if out.len() < SHARED_MEMORY_REF_WIRE_SIZE {
            return None;
        }
        out[0..4].copy_from_slice(&self.thing.to_le_bytes());
        out[4..8].copy_from_slice(&self._pad.to_le_bytes());
        out[8..16].copy_from_slice(&self.length.to_le_bytes());
        Some(SHARED_MEMORY_REF_WIRE_SIZE)
    }

    /// Decode from little-endian bytes.  Returns `None` if `src` is too small.
    pub fn decode_le(src: &[u8]) -> Option<Self> {
        if src.len() < SHARED_MEMORY_REF_WIRE_SIZE {
            return None;
        }
        // SAFETY: the length check above guarantees these sub-slices are
        // exactly 4 / 4 / 8 bytes, so the fixed-size array conversions below
        // cannot fail.
        let thing = u32::from_le_bytes(src[0..4].try_into().unwrap());
        let _pad = u32::from_le_bytes(src[4..8].try_into().unwrap());
        let length = u64::from_le_bytes(src[8..16].try_into().unwrap());
        Some(Self { thing, _pad, length })
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    #[test]
    fn wire_size_constant_matches_struct() {
        assert_eq!(SHARED_MEMORY_REF_WIRE_SIZE, core::mem::size_of::<SharedMemoryRef>());
    }

    #[test]
    fn round_trip_encode_decode() {
        let desc = SharedMemoryRef::new(7, 0x0001_0000);
        let mut buf = [0u8; SHARED_MEMORY_REF_WIRE_SIZE];
        let written = desc.encode_le(&mut buf).unwrap();
        assert_eq!(written, SHARED_MEMORY_REF_WIRE_SIZE);

        let decoded = SharedMemoryRef::decode_le(&buf).unwrap();
        assert_eq!(decoded.thing, 7);
        assert_eq!(decoded._pad, 0);
        assert_eq!(decoded.length, 0x0001_0000);
    }

    #[test]
    fn golden_bytes() {
        // thing=1, length=4096 (0x1000)
        let desc = SharedMemoryRef::new(1, 0x1000);
        let mut buf = [0u8; SHARED_MEMORY_REF_WIRE_SIZE];
        desc.encode_le(&mut buf).unwrap();
        assert_eq!(
            buf,
            [
                0x01, 0x00, 0x00, 0x00, // thing = 1
                0x00, 0x00, 0x00, 0x00, // _pad = 0
                0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // length = 0x1000
            ]
        );
    }

    #[test]
    fn decode_too_short_returns_none() {
        let buf = [0u8; SHARED_MEMORY_REF_WIRE_SIZE - 1];
        assert!(SharedMemoryRef::decode_le(&buf).is_none());
    }

    #[test]
    fn encode_too_small_buffer_returns_none() {
        let desc = SharedMemoryRef::new(3, 8192);
        let mut buf = [0u8; SHARED_MEMORY_REF_WIRE_SIZE - 1];
        assert!(desc.encode_le(&mut buf).is_none());
    }

    /// Verify that a descriptor survives a channel-message round-trip
    /// (control header + SharedMemoryRef payload, then back).
    #[test]
    fn control_message_payload_round_trip() {
        // Simulate embedding SharedMemoryRef inside a larger control message.
        // Layout: tag(1) + SharedMemoryRef(16) = 17 bytes.
        const MSG_BULK_OFFER: u8 = 0x42;
        let desc = SharedMemoryRef::new(5, 1920 * 1080 * 4);

        let mut msg = [0u8; 17];
        msg[0] = MSG_BULK_OFFER;
        desc.encode_le(&mut msg[1..]).unwrap();

        // Deserialise
        assert_eq!(msg[0], MSG_BULK_OFFER);
        let decoded = SharedMemoryRef::decode_le(&msg[1..]).unwrap();
        assert_eq!(decoded.thing, 5);
        assert_eq!(decoded.length, 1920 * 1080 * 4);
    }
}
