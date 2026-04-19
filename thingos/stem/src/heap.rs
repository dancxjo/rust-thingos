//! Heap management - delegates to platform abstraction layer.

use abi::errors::Errno;

use crate::pal;

/// Grow the heap by at least `min_bytes`.
///
/// This is a convenience wrapper around the PAL primitive.
pub fn grow_heap(min_bytes: usize) -> Result<(), Errno> {
    pal::alloc::grow_heap(min_bytes)
}
