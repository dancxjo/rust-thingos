use alloc::sync::Arc;
use alloc::vec::Vec;

use abi::errors::SysResult;

use super::{VfsDriver, VfsNode};
use crate::vfs::ramfs::RamFs;
use crate::vfs::union::UnionDirNode;

/// A generic copy-on-read overlay filesystem driver.
/// It wraps an underlying driver, checking an ephemeral `RamFs` layer first.
/// If a file is missing in the upper layer but exists in the lower layer,
/// it is read fully from the lower layer and copied into the upper layer.
pub struct OverlayFs {
    lower: Arc<dyn VfsDriver>,
    upper: Arc<RamFs>,
}

impl OverlayFs {
    pub fn new(lower: Arc<dyn VfsDriver>) -> Self {
        Self { lower, upper: Arc::new(RamFs::new()) }
    }

    /// Read the entire file from the lower node and write it into the upper node.
    fn populate_upper(
        lower_node: &Arc<dyn VfsNode>,
        upper_node: &Arc<dyn VfsNode>,
    ) -> SysResult<()> {
        let mut offset = 0;
        let mut buf = alloc::vec![0u8; 4096];
        loop {
            let n = lower_node.read(offset, &mut buf)?;
            if n == 0 {
                break;
            }
            upper_node.write(offset, &buf[..n])?;
            offset += n as u64;
        }
        Ok(())
    }
}

impl VfsDriver for OverlayFs {
    fn lookup(&self, path: &str) -> SysResult<Arc<dyn VfsNode>> {
        let upper_node = self.upper.lookup(path).ok();
        let lower_node = match self.lower.lookup(path) {
            Ok(node) => node,
            Err(abi::errors::Errno::ENOENT) => {
                if let Some(upper_node) = upper_node {
                    return Ok(upper_node);
                }
                return Err(abi::errors::Errno::ENOENT);
            }
            Err(err) => return Err(err),
        };

        let stat = lower_node.stat()?;
        if stat.is_dir() {
            if let Some(upper_node) = upper_node {
                if upper_node.stat()?.is_dir() {
                    return Ok(Arc::new(UnionDirNode::new(alloc::vec![upper_node, lower_node])));
                }
                return Ok(upper_node);
            }
            return Ok(lower_node);
        }

        if let Some(upper_node) = upper_node {
            return Ok(upper_node);
        }

        // Create the file in upper.
        // Ensure parent directories exist in the upper layer.
        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        if parts.len() > 1 {
            let mut current_path = alloc::string::String::new();
            for part in &parts[..parts.len() - 1] {
                if !current_path.is_empty() {
                    current_path.push('/');
                }
                current_path.push_str(part);
                let _ = self.upper.mkdir(&current_path); // Ignore EEXIST
            }
        }

        let upper_node = match self.upper.create(path) {
            Ok(n) => n,
            Err(_) => return Ok(lower_node), // Fallback to lower on creation failure
        };

        // 4. Copy data from lower to upper.
        if Self::populate_upper(&lower_node, &upper_node).is_err() {
            // If copy fails, fallback to lower
            return Ok(lower_node);
        }

        Ok(upper_node)
    }

    fn create(&self, path: &str) -> SysResult<Arc<dyn VfsNode>> {
        self.upper.create(path)
    }

    fn mkdir(&self, path: &str) -> SysResult<()> {
        self.upper.mkdir(path)
    }

    fn unlink(&self, path: &str) -> SysResult<()> {
        // We only unlink from upper for now. True overlay whiteouts are omitted for simplicity.
        self.upper.unlink(path)
    }

    fn rename(&self, old_path: &str, new_path: &str) -> SysResult<()> {
        self.upper.rename(old_path, new_path)
    }

    fn symlink(&self, target: &str, link_path: &str) -> SysResult<()> {
        self.upper.symlink(target, link_path)
    }

    fn link(&self, src_path: &str, dst_path: &str) -> SysResult<()> {
        self.upper.link(src_path, dst_path)
    }
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vfs::VfsStat;
    use crate::vfs::ramfs::RamFs;

    #[test]
    fn test_overlay_copy_on_read() {
        let lower = Arc::new(RamFs::new());
        let lower_node = lower.create("/test.txt").unwrap();
        lower_node.write(0, b"hello world").unwrap();

        let overlay = OverlayFs::new(lower.clone());

        // Initial lookup should find it and copy it to upper
        let node = overlay.lookup("/test.txt").expect("failed to lookup");
        let mut buf = [0u8; 11];
        let n = node.read(0, &mut buf).unwrap();
        assert_eq!(n, 11);
        assert_eq!(&buf, b"hello world");

        // Now modify upper
        node.write(0, b"HELLO").unwrap();

        // Reading again from overlay should give modified upper
        let mut buf2 = [0u8; 11];
        node.read(0, &mut buf2).unwrap();
        assert_eq!(&buf2, b"HELLO world");

        // But lower should be untouched
        let mut buf3 = [0u8; 11];
        lower_node.read(0, &mut buf3).unwrap();
        assert_eq!(&buf3, b"hello world");
    }

    #[test]
    fn test_overlay_create_new() {
        let lower = Arc::new(RamFs::new());
        let overlay = OverlayFs::new(lower.clone());

        let node = overlay.create("/new.txt").unwrap();
        node.write(0, b"new file").unwrap();

        // Should exist in overlay
        assert!(overlay.lookup("/new.txt").is_ok());

        // Should NOT exist in lower
        assert!(lower.lookup("/new.txt").is_err());
    }

    #[test]
    fn test_overlay_merges_upper_and_lower_directories() {
        let lower = Arc::new(RamFs::new());
        lower.mkdir("/bin").unwrap();
        lower.mkdir("/etc").unwrap();

        let overlay = OverlayFs::new(lower.clone());
        overlay.mkdir("/run").unwrap();

        let root = overlay.lookup("/").expect("root lookup");
        let mut buf = [0u8; 128];
        let n = root.readdir(0, &mut buf).expect("root readdir");
        let text = core::str::from_utf8(&buf[..n]).unwrap();

        assert!(text.contains("bin"));
        assert!(text.contains("etc"));
        assert!(text.contains("run"));
    }
}
