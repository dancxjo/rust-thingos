/// Align value upwards to the given alignment.
///
/// `align` must be a power of two.
///
/// ```
/// use stem::utils::align_up;
/// assert_eq!(align_up(0, 4096), 0);
/// assert_eq!(align_up(1, 4096), 4096);
/// ```
pub const fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

/// Align value downwards to the given alignment.
///
/// `align` must be a power of two.
///
/// ```
/// use stem::utils::align_down;
/// assert_eq!(align_down(1, 4096), 0);
/// assert_eq!(align_down(4096, 4096), 4096);
/// ```
pub const fn align_down(addr: usize, align: usize) -> usize {
    addr & !(align - 1)
}

/// Check if value is aligned to the given alignment.
///
/// `align` must be a power of two.
///
/// ```
/// use stem::utils::is_aligned;
/// assert!(is_aligned(4096, 4096));
/// assert!(!is_aligned(4097, 4096));
/// ```
pub const fn is_aligned(addr: usize, align: usize) -> bool {
    (addr & (align - 1)) == 0
}

pub trait RangeExt {
    fn length(&self) -> usize;
    fn contains_val(&self, val: usize) -> bool;
    fn overlaps(&self, other: &Self) -> bool;
    fn intersection(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;
}

impl RangeExt for core::ops::Range<usize> {
    fn length(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    fn contains_val(&self, val: usize) -> bool {
        val >= self.start && val < self.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.start < other.end && other.start < self.end
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        let start = core::cmp::max(self.start, other.start);
        let end = core::cmp::min(self.end, other.end);
        if start < end { Some(start..end) } else { None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_ext() {
        let r1 = 10..20;
        let r2 = 15..25;
        let r3 = 20..30;

        assert_eq!(r1.length(), 10);
        assert!(r1.contains_val(10));
        assert!(r1.contains_val(19));
        assert!(!r1.contains_val(20));

        assert!(r1.overlaps(&r2));
        assert!(!r1.overlaps(&r3)); // [10, 20) and [20, 30) do not overlap

        assert_eq!(r1.intersection(&r2), Some(15..20));
        assert_eq!(r1.intersection(&r3), None);
    }

    #[test]
    fn test_align_up() {
        assert_eq!(align_up(0, 4096), 0);
        assert_eq!(align_up(1, 4096), 4096);
        assert_eq!(align_up(4095, 4096), 4096);
        assert_eq!(align_up(4096, 4096), 4096);
        assert_eq!(align_up(4097, 4096), 8192);
    }

    #[test]
    fn test_align_down() {
        assert_eq!(align_down(0, 4096), 0);
        assert_eq!(align_down(1, 4096), 0);
        assert_eq!(align_down(4095, 4096), 0);
        assert_eq!(align_down(4096, 4096), 4096);
        assert_eq!(align_down(4097, 4096), 4096);
    }

    #[test]
    fn test_is_aligned() {
        assert!(is_aligned(0, 4096));
        assert!(!is_aligned(1, 4096));
        assert!(is_aligned(4096, 4096));
        assert!(!is_aligned(4097, 4096));
    }
}

/// A simple `core::fmt::Write` implementation that writes into a fixed byte
/// slice.  Bytes beyond the end of the slice are silently dropped.
///
/// Used by [`LoopMetricsSnapshot::write_text`] to format diagnostic output
/// without heap allocation.
pub struct SliceWriter<'a> {
    buf: &'a mut [u8],
    pos: usize,
}

impl<'a> SliceWriter<'a> {
    /// Create a new `SliceWriter` wrapping `buf`.
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    /// Returns the number of bytes written so far.
    pub fn written(&self) -> usize {
        self.pos
    }
}

impl core::fmt::Write for SliceWriter<'_> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let remaining = self.buf.len().saturating_sub(self.pos);
        let n = bytes.len().min(remaining);
        self.buf[self.pos..self.pos + n].copy_from_slice(&bytes[..n]);
        self.pos += n;
        Ok(())
    }
}
///
/// The format is: `[count: u32 LE] [len0: u32 LE] [arg0...] [len1: u32 LE] [arg1...]`
pub fn parse_argv(buf: &[u8]) -> alloc::vec::Vec<&[u8]> {
    let mut args = alloc::vec::Vec::new();
    if buf.len() < 4 {
        return args;
    }
    let count = u32::from_le_bytes(buf[0..4].try_into().unwrap_or([0; 4])) as usize;
    let mut offset = 4;
    for _ in 0..count {
        if offset + 4 > buf.len() {
            break;
        }
        let len = u32::from_le_bytes(buf[offset..offset + 4].try_into().unwrap_or([0; 4])) as usize;
        offset += 4;
        if offset + len > buf.len() {
            break;
        }
        args.push(&buf[offset..offset + len]);
        offset += len;
    }
    args
}
