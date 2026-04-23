//! UnionNode — layered (union) mount support.
//!
//! Allows multiple [`VfsDriver`] providers to be stacked at the same path
//! prefix.  Resolution order: **last mounted wins** unless [`UnionNode`] is
//! created with `fallthrough = true`, in which case the first provider that
//! returns a node (i.e. does *not* return `ENOENT`) wins.
//!
//! # Example
//! ```text
//! /bin = base_bin_provider + user_bin_provider
//! ```
//! With `fallthrough = false` (default): `user_bin_provider` is tried first;
//! only if it returns `ENOENT` is `base_bin_provider` tried (last → first).
//!
//! # Use cases
//! - Overlay user tools over system tools.
//! - Compose testing environments (mock `/services/net` overlaid for a test
//!   process).
//! - Fallback chains without graph merging.
//!
//! [`VfsDriver`]: super::VfsDriver

use alloc::collections::BTreeSet;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

use abi::errors::{Errno, SysResult};

use super::{VfsDriver, VfsNode};

// ── UnionNode ─────────────────────────────────────────────────────────────────

/// A [`VfsDriver`] that layers multiple drivers at the same mount point.
///
/// Drivers are stored in the order they were added.  Lookup walks the list
/// from the **end** (most-recently added) toward the beginning, returning the
/// first successful result.  If all drivers return `ENOENT` the union returns
/// `ENOENT`.  Any other error from a driver is propagated immediately.
pub struct UnionFs {
    /// Ordered list of backing drivers (append-only in normal usage).
    layers: Vec<Arc<dyn VfsDriver>>,
    /// When `true`, lookup falls through to the next layer even on success
    /// (layers are merged; later layers take precedence for duplicate names).
    ///
    /// When `false` (the default), the first successful lookup wins.
    pub fallthrough: bool,
}

impl UnionFs {
    /// Create an empty union with no layers.
    pub fn new() -> Self {
        Self { layers: Vec::new(), fallthrough: false }
    }

    /// Create a union with `fallthrough` behaviour enabled.
    pub fn new_fallthrough() -> Self {
        Self { layers: Vec::new(), fallthrough: true }
    }

    /// Add `driver` as the **topmost** (highest-priority) layer.
    ///
    /// Returns `&mut self` so calls can be chained.
    pub fn push(&mut self, driver: Arc<dyn VfsDriver>) -> &mut Self {
        self.layers.push(driver);
        self
    }

    /// Number of layers currently in this union.
    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }
}

impl Default for UnionFs {
    fn default() -> Self {
        Self::new()
    }
}

impl VfsDriver for UnionFs {
    /// Resolve `path` by trying layers from most-recently added to oldest.
    ///
    /// If multiple directories are found (and `fallthrough` is active), they are merged.
    fn lookup(&self, path: &str) -> SysResult<Arc<dyn VfsNode>> {
        let mut found_nodes = Vec::new();

        for driver in self.layers.iter().rev() {
            match driver.lookup(path) {
                Ok(node) => {
                    let stat = node.stat()?;
                    let is_dir = (stat.mode & crate::vfs::VfsStat::S_IFDIR) != 0;

                    found_nodes.push(node);

                    // If it's a file, it always shadows everything below.
                    if !is_dir || !self.fallthrough {
                        break;
                    }
                }
                Err(Errno::ENOENT) => continue,
                Err(e) => return Err(e),
            }
        }

        if found_nodes.is_empty() {
            return Err(Errno::ENOENT);
        }

        if found_nodes.len() == 1 {
            return Ok(found_nodes.pop().unwrap());
        }

        // Multiple nodes found (must all be directories because of the !is_dir break above).
        Ok(Arc::new(UnionDirNode { layers: found_nodes }))
    }
}

// ── UnionDirNode ──────────────────────────────────────────────────────────────

/// A directory node that merges entries from multiple underlying directory nodes.
pub struct UnionDirNode {
    layers: Vec<Arc<dyn VfsNode>>,
}

fn append_readdir_entry(
    name: &str,
    offset: u64,
    virtual_pos: &mut u64,
    buf: &mut [u8],
    written: &mut usize,
) -> bool {
    let entry_len = u64::try_from(name.len())
        .unwrap_or(u64::MAX)
        .saturating_add(1);
    if *virtual_pos + entry_len > offset {
        let start_in_entry = if offset > *virtual_pos {
            usize::try_from(offset - *virtual_pos).unwrap_or(usize::MAX)
        } else {
            0
        };
        if start_in_entry < name.len() {
            let part = &name.as_bytes()[start_in_entry..];
            let copy_n = part.len().min(buf.len() - *written);
            buf[*written..*written + copy_n].copy_from_slice(&part[..copy_n]);
            *written += copy_n;
            if copy_n == part.len() && *written < buf.len() {
                buf[*written] = 0;
                *written += 1;
            }
        } else if start_in_entry == name.len() && *written < buf.len() {
            buf[*written] = 0;
            *written += 1;
        }
    }
    *virtual_pos = virtual_pos.saturating_add(entry_len);
    *written == buf.len()
}

impl super::VfsNode for UnionDirNode {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }

    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }

    fn stat(&self) -> SysResult<super::VfsStat> {
        // Use the stat of the topmost layer.
        self.layers[0].stat()
    }

    fn readdir(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        if buf.is_empty() {
            return Ok(0);
        }

        let mut seen = BTreeSet::new();
        // Keep this reasonably small so a provider-backed layer can yield early,
        // while still large enough to amortize IPC/syscall overhead.
        let mut scratch = alloc::vec![0u8; 2048];
        let mut virtual_pos = 0u64;
        let mut written = 0usize;

        for layer in &self.layers {
            let mut layer_offset = 0u64;
            let mut pending_name = String::new();

            loop {
                let n = match layer.readdir(layer_offset, &mut scratch) {
                    Ok(n) => n,
                    Err(_) => break,
                };
                if n == 0 {
                    if !pending_name.is_empty() {
                        let name = core::mem::take(&mut pending_name);
                        if seen.insert(name.clone()) {
                            if append_readdir_entry(
                                &name,
                                offset,
                                &mut virtual_pos,
                                buf,
                                &mut written,
                            ) {
                                return Ok(written);
                            }
                        }
                    }
                    break;
                }

                layer_offset = layer_offset.saturating_add(n as u64);
                let mut start = 0usize;
                for i in 0..n {
                    if scratch[i] != 0 {
                        continue;
                    }

                    let part = core::str::from_utf8(&scratch[start..i]).map_err(|_| {
                        crate::kwarn!("UnionDirNode::readdir: invalid UTF-8 in layer stream");
                        Errno::EIO
                    })?;
                    pending_name.push_str(part);

                    if !pending_name.is_empty() {
                        let name = core::mem::take(&mut pending_name);
                        if seen.insert(name.clone()) {
                            if append_readdir_entry(
                                &name,
                                offset,
                                &mut virtual_pos,
                                buf,
                                &mut written,
                            ) {
                                return Ok(written);
                            }
                        }
                    }

                    start = i + 1;
                }

                if start < n {
                    let part = core::str::from_utf8(&scratch[start..n]).map_err(|_| {
                        crate::kwarn!("UnionDirNode::readdir: invalid UTF-8 in trailing layer bytes");
                        Errno::EIO
                    })?;
                    pending_name.push_str(part);
                }
            }
        }

        Ok(written)
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use core::sync::atomic::{AtomicUsize, Ordering};

    use alloc::sync::Arc;
    use alloc::vec::Vec;

    use abi::errors::Errno;

    use super::*;
    use crate::vfs::{VfsNode, VfsStat};

    // ── Helpers ─────────────────────────────────────────────────────────────

    struct SingleFileFs {
        name: &'static str,
        content: &'static [u8],
        ino: u64,
    }

    impl SingleFileFs {
        fn new(name: &'static str, content: &'static [u8], ino: u64) -> Arc<Self> {
            Arc::new(Self { name, content, ino })
        }
    }

    impl VfsDriver for SingleFileFs {
        fn lookup(&self, path: &str) -> SysResult<Arc<dyn VfsNode>> {
            if path == self.name {
                Ok(Arc::new(StaticNode { content: self.content, ino: self.ino }))
            } else {
                Err(Errno::ENOENT)
            }
        }
    }

    struct StaticNode {
        content: &'static [u8],
        ino: u64,
    }

    impl VfsNode for StaticNode {
        fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
            let off = offset as usize;
            if off >= self.content.len() {
                return Ok(0);
            }
            let avail = &self.content[off..];
            let n = avail.len().min(buf.len());
            buf[..n].copy_from_slice(&avail[..n]);
            Ok(n)
        }
        fn write(&self, _: u64, _: &[u8]) -> SysResult<usize> {
            Err(Errno::EROFS)
        }
        fn stat(&self) -> SysResult<VfsStat> {
            Ok(VfsStat {
                mode: VfsStat::S_IFREG | 0o444,
                size: self.content.len() as u64,
                ino: self.ino,
                ..Default::default()
            })
        }
    }

    struct StaticDirNode {
        names: Vec<&'static str>,
        calls: Arc<AtomicUsize>,
        ino: u64,
    }

    impl VfsNode for StaticDirNode {
        fn read(&self, _: u64, _: &mut [u8]) -> SysResult<usize> {
            Err(Errno::EISDIR)
        }

        fn write(&self, _: u64, _: &[u8]) -> SysResult<usize> {
            Err(Errno::EISDIR)
        }

        fn stat(&self) -> SysResult<VfsStat> {
            Ok(VfsStat { mode: VfsStat::S_IFDIR | 0o555, size: 0, ino: self.ino, ..Default::default() })
        }

        fn readdir(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
            self.calls.fetch_add(1, Ordering::Relaxed);
            super::super::write_readdir_entries(self.names.iter().copied(), offset, buf)
        }
    }

    struct StaticDirFs {
        names: Vec<&'static str>,
        calls: Arc<AtomicUsize>,
        ino: u64,
    }

    impl VfsDriver for StaticDirFs {
        fn lookup(&self, path: &str) -> SysResult<Arc<dyn VfsNode>> {
            if path == "dir" {
                Ok(Arc::new(StaticDirNode {
                    names: self.names.clone(),
                    calls: self.calls.clone(),
                    ino: self.ino,
                }))
            } else {
                Err(Errno::ENOENT)
            }
        }
    }

    // Error-producing driver.
    struct ErrFs(Errno);
    impl VfsDriver for ErrFs {
        fn lookup(&self, _: &str) -> SysResult<Arc<dyn VfsNode>> {
            Err(self.0)
        }
    }

    // ── Tests ────────────────────────────────────────────────────────────────

    #[test]
    fn test_empty_union_returns_enoent() {
        let union = UnionFs::new();
        assert!(matches!(union.lookup("anything"), Err(Errno::ENOENT)));
    }

    #[test]
    fn test_single_layer_hit() {
        let mut union = UnionFs::new();
        union.push(SingleFileFs::new("hello", b"world", 1));
        let node = union.lookup("hello").unwrap();
        let mut buf = [0u8; 5];
        node.read(0, &mut buf).unwrap();
        assert_eq!(&buf, b"world");
    }

    #[test]
    fn test_single_layer_miss() {
        let mut union = UnionFs::new();
        union.push(SingleFileFs::new("hello", b"world", 1));
        assert!(matches!(union.lookup("nope"), Err(Errno::ENOENT)));
    }

    #[test]
    fn test_upper_layer_shadows_lower() {
        let mut union = UnionFs::new();
        // base layer: "tool" → "base"
        union.push(SingleFileFs::new("tool", b"base", 1));
        // upper layer: "tool" → "user" (added last = highest priority)
        union.push(SingleFileFs::new("tool", b"user", 2));

        let node = union.lookup("tool").unwrap();
        let stat = node.stat().unwrap();
        // Should come from the upper (ino=2).
        assert_eq!(stat.ino, 2);
        let mut buf = [0u8; 4];
        node.read(0, &mut buf).unwrap();
        assert_eq!(&buf, b"user");
    }

    #[test]
    fn test_fallthrough_to_lower_when_upper_misses() {
        let mut union = UnionFs::new();
        // base layer has "base_tool"
        union.push(SingleFileFs::new("base_tool", b"bt", 10));
        // upper layer has only "upper_tool"
        union.push(SingleFileFs::new("upper_tool", b"ut", 20));

        // "base_tool" not in upper → should fall through to base.
        let node = union.lookup("base_tool").unwrap();
        let stat = node.stat().unwrap();
        assert_eq!(stat.ino, 10);
    }

    #[test]
    fn test_hard_error_propagates_without_falling_through() {
        let mut union = UnionFs::new();
        union.push(Arc::new(ErrFs(Errno::EIO)));
        // Hard error should propagate, not fall to lower layers.
        assert!(matches!(union.lookup("x"), Err(Errno::EIO)));
    }

    #[test]
    fn test_layer_count() {
        let mut union = UnionFs::new();
        assert_eq!(union.layer_count(), 0);
        union.push(SingleFileFs::new("a", b"a", 1));
        assert_eq!(union.layer_count(), 1);
        union.push(SingleFileFs::new("b", b"b", 2));
        assert_eq!(union.layer_count(), 2);
    }

    #[test]
    fn test_union_readdir_merges_and_deduplicates() {
        let upper_calls = Arc::new(AtomicUsize::new(0));
        let lower_calls = Arc::new(AtomicUsize::new(0));
        let mut union = UnionFs::new_fallthrough();
        union.push(Arc::new(StaticDirFs {
            names: vec!["a", "b"],
            calls: lower_calls.clone(),
            ino: 1,
        }));
        union.push(Arc::new(StaticDirFs {
            names: vec!["b", "c"],
            calls: upper_calls.clone(),
            ino: 2,
        }));

        let node = union.lookup("dir").unwrap();
        let mut buf = [0u8; 64];
        let n = node.readdir(0, &mut buf).unwrap();
        assert_eq!(&buf[..n], b"b\0c\0a\0");
        assert!(upper_calls.load(Ordering::Relaxed) > 0);
        assert!(lower_calls.load(Ordering::Relaxed) > 0);
    }

    #[test]
    fn test_union_readdir_can_return_without_touching_lower_layers() {
        let upper_calls = Arc::new(AtomicUsize::new(0));
        let lower_calls = Arc::new(AtomicUsize::new(0));
        let mut union = UnionFs::new_fallthrough();
        union.push(Arc::new(StaticDirFs {
            names: vec!["lower-entry"],
            calls: lower_calls.clone(),
            ino: 11,
        }));
        union.push(Arc::new(StaticDirFs {
            names: vec!["abcd", "efgh", "ijkl"],
            calls: upper_calls.clone(),
            ino: 12,
        }));

        let node = union.lookup("dir").unwrap();
        let mut buf = [0u8; 5];
        let n = node.readdir(0, &mut buf).unwrap();
        assert_eq!(n, 5);
        assert_eq!(&buf, b"abcd\0");
        assert!(upper_calls.load(Ordering::Relaxed) > 0);
        assert_eq!(lower_calls.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_union_readdir_offset_inside_stream() {
        let mut union = UnionFs::new_fallthrough();
        union.push(Arc::new(StaticDirFs {
            names: vec!["a", "b"],
            calls: Arc::new(AtomicUsize::new(0)),
            ino: 21,
        }));
        union.push(Arc::new(StaticDirFs {
            names: vec!["b", "c"],
            calls: Arc::new(AtomicUsize::new(0)),
            ino: 22,
        }));

        let node = union.lookup("dir").unwrap();
        let mut buf = [0u8; 64];
        let n = node.readdir(2, &mut buf).unwrap();
        assert_eq!(&buf[..n], b"c\0a\0");
    }
}
