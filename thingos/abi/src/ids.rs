use crate::wire::ThingId;

/// Extension methods for converting between internal u64 thing numbers and ABI ThingIds.
/// This bridges the gap while the kernel uses u64 internally but the ABI uses UUIDs.
impl ThingId {
    /// Creates a ThingId from a u64 thing number by zero-padding.
    /// Layout: [thing_bytes(8) | 0...0]
    pub fn from_u64(val: u64) -> Self {
        let mut bytes = [0u8; 16];
        bytes[0..8].copy_from_slice(&val.to_le_bytes());
        ThingId(bytes)
    }

    /// Extracts the u64 thing number from a ThingId.
    /// This is lossy/unsafe if the ThingId was not created via `from_u64`.
    /// For the bridge, we assume it is valid.
    pub fn to_u64_lossy(&self) -> u64 {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.0[0..8]);
        u64::from_le_bytes(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thing_id_round_trip() {
        let values = [0u64, 1, 42, u64::MAX, 0xDEADBEEFCAFEBABE];

        for &val in &values {
            let thing_id = ThingId::from_u64(val);
            let result = thing_id.to_u64_lossy();
            assert_eq!(result, val, "Round trip failed for value: {}", val);

            // Verify the upper 8 bytes are zero (padding)
            // The layout is [thing_bytes(8) | 0...0]
            assert_eq!(&thing_id.0[8..16], &[0u8; 8], "Upper bytes must be zero-padded");
        }
    }

    #[test]
    fn test_thing_id_default_is_zero_padded() {
        let id = ThingId::default();
        assert_eq!(id.to_u64_lossy(), 0);
        assert_eq!(&id.0[8..16], &[0u8; 8], "Upper bytes must be zero-padded");
    }

    #[test]
    #[cfg(feature = "kernel-id-gen")]
    fn test_debug_nonce_violates_bridge() {
        let _ = ThingId::new_debug_nonce();
        let id = ThingId::new_debug_nonce();
        // A debug nonce should have non-zero upper bytes (it uses them for sequence)
        // This confirms it would be "lossy" or "cursed" if used as a thing number.
        assert_ne!(
            &id.0[8..16],
            &[0u8; 8],
            "Debug nonce must NOT be zero-padded (violations are documented)"
        );
    }
}
