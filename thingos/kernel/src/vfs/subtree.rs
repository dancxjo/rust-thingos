//! Bind a subtree of one filesystem driver at another mount point.

use alloc::string::{String, ToString};
use alloc::sync::Arc;

use abi::errors::SysResult;

use super::{VfsDriver, VfsNode};

pub struct SubtreeFs {
    lower: Arc<dyn VfsDriver>,
    prefix: String,
}

impl SubtreeFs {
    pub fn new(lower: Arc<dyn VfsDriver>, prefix: &str) -> Self {
        Self { lower, prefix: prefix.trim_matches('/').to_string() }
    }

    fn map_path(&self, path: &str) -> String {
        let path = path.trim_matches('/');
        match (self.prefix.is_empty(), path.is_empty()) {
            (true, _) => path.to_string(),
            (_, true) => self.prefix.clone(),
            _ => {
                let mut mapped = String::with_capacity(self.prefix.len() + 1 + path.len());
                mapped.push_str(&self.prefix);
                mapped.push('/');
                mapped.push_str(path);
                mapped
            }
        }
    }
}

impl VfsDriver for SubtreeFs {
    fn lookup(&self, path: &str) -> SysResult<Arc<dyn VfsNode>> {
        self.lower.lookup(&self.map_path(path))
    }

    fn create(&self, path: &str) -> SysResult<Arc<dyn VfsNode>> {
        self.lower.create(&self.map_path(path))
    }

    fn mkdir(&self, path: &str) -> SysResult<()> {
        self.lower.mkdir(&self.map_path(path))
    }

    fn unlink(&self, path: &str) -> SysResult<()> {
        self.lower.unlink(&self.map_path(path))
    }

    fn rename(&self, old_path: &str, new_path: &str) -> SysResult<()> {
        self.lower.rename(&self.map_path(old_path), &self.map_path(new_path))
    }

    fn symlink(&self, target: &str, link_path: &str) -> SysResult<()> {
        self.lower.symlink(target, &self.map_path(link_path))
    }

    fn link(&self, src_path: &str, dst_path: &str) -> SysResult<()> {
        self.lower.link(&self.map_path(src_path), &self.map_path(dst_path))
    }
}
