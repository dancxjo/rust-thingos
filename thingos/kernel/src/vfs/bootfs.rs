//! bootfs — minimal static boot filesystem.
//!
//! Provides a read-only filesystem whose contents are compiled into the kernel
//! binary at link time, or passed as boot modules by the loader.
//!
//! The boot filesystem is mounted at `/boot` by [`crate::vfs::init`].

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;

use abi::errors::{Errno, SysResult};

use super::{VfsDriver, VfsNode, VfsStat};
use crate::BootModuleDesc;

// ── Embedded file contents ────────────────────────────────────────────────────

const VERSION_DATA: &[u8] = b"Thing-OS v0.1\n";
const MOTD_DATA: &[u8] = b"\x1B[1;32m\n        .-.\n       /   \\        \x1B[1;36mTHING-OS\x1B[1;32m\n      |     |       \x1B[0;36m\"People, places, things.\"\x1B[1;32m\n       \\   /        \n        `-'        \n       /   \\        v0.1  \xE2\x80\xA2  \n      |     |       2026-04-16\n       \\   /\n        `-'\n\x1B[0m\n\x1B[2m--------------------------------------------------------------\x1B[0m\n\x1B[1m sprout has taken root. the system is awake.\x1B[0m\n\n  try:\n    \x1B[36mls /bin\x1B[0m       browse available shoots\n    \x1B[36mps\x1B[0m            observe living processes\n    \x1B[36mcat /version\x1B[0m  inspect the genome\n\n\x1B[2m--------------------------------------------------------------\x1B[0m\n";

// ── BootFs driver ─────────────────────────────────────────────────────────────

/// The boot filesystem driver.  Mounted at `/boot` by `vfs::init`.
///
/// An index mapping clean module names (no leading slash, no null padding) to
/// their module-slice index is built once in [`BootFs::new`].  This makes
/// every [`VfsDriver::lookup`] call O(log n) instead of O(n), which matters
/// when 100+ modules are loaded and every `open()` syscall hits this path.
pub struct BootFs {
    modules: &'static [BootModuleDesc],
    /// O(log n) name → index lookup, built once at construction time.
    index: BTreeMap<&'static str, usize>,
    /// Pre-cleaned module names (leading slash stripped, null/whitespace trimmed),
    /// shared with `BootDirNode` so `readdir` avoids per-call string normalisation.
    clean_names: Arc<Vec<&'static str>>,
}

impl BootFs {
    pub fn new(modules: &'static [BootModuleDesc]) -> Self {
        let mut index = BTreeMap::new();
        let mut clean_names = Vec::with_capacity(modules.len());
        for (i, m) in modules.iter().enumerate() {
            let name = m.name.trim_matches('\0').trim();
            let clean = name.strip_prefix('/').unwrap_or(name);
            clean_names.push(clean);
            if !clean.is_empty() {
                // Preserve first-match semantics: if two modules share a name the
                // first one wins, matching the behaviour of the old linear scan.
                index.entry(clean).or_insert(i);
            }
        }
        Self { modules, index, clean_names: Arc::new(clean_names) }
    }
}

impl VfsDriver for BootFs {
    fn lookup(&self, path: &str) -> SysResult<Arc<dyn VfsNode>> {
        let path = path.strip_prefix('/').unwrap_or(path);
        if path.is_empty() {
            return Ok(Arc::new(BootDirNode {
                prefix: String::new(),
                clean_names: Arc::clone(&self.clean_names),
            }));
        }

        if path == "version" {
            return Ok(Arc::new(StaticFileNode::new(VERSION_DATA, 10)));
        }
        if path == "motd" {
            return Ok(Arc::new(StaticFileNode::new(MOTD_DATA, 11)));
        }

        // O(log n) exact match via pre-built index.
        if let Some(&i) = self.index.get(path) {
            return Ok(Arc::new(StaticFileNode::new(self.modules[i].bytes, 100 + i as u64)));
        }

        // Check if `path` is a directory prefix (less frequent; linear over
        // pre-cleaned names avoids repeated trim/strip_prefix per entry).
        let dir_prefix =
            if path.ends_with('/') { path.to_string() } else { alloc::format!("{}/", path) };
        let found_subdir = self.clean_names.iter().any(|cn| cn.starts_with(dir_prefix.as_str()));

        if found_subdir {
            return Ok(Arc::new(BootDirNode {
                prefix: path.to_string(),
                clean_names: Arc::clone(&self.clean_names),
            }));
        }

        Err(Errno::ENOENT)
    }
}

// ── /boot directory node ──────────────────────────────────────────────────────

struct BootDirNode {
    prefix: String,
    /// Shared reference to pre-cleaned module names from [`BootFs`].
    clean_names: Arc<Vec<&'static str>>,
}

impl VfsNode for BootDirNode {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat { mode: VfsStat::S_IFDIR | 0o555, size: 0, ino: 9, ..Default::default() })
    }
    fn readdir(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let mut components = BTreeSet::new();

        // Standard files at root
        if self.prefix.is_empty() {
            components.insert("version".to_string());
            components.insert("motd".to_string());
        }

        let prefix_with_slash = if self.prefix.is_empty() {
            String::new()
        } else if self.prefix.ends_with('/') {
            self.prefix.clone()
        } else {
            alloc::format!("{}/", self.prefix)
        };

        // Use the pre-cleaned names — no per-entry trim/strip_prefix overhead.
        for &clean_name in self.clean_names.iter() {
            if clean_name.starts_with(&prefix_with_slash) {
                let rest = &clean_name[prefix_with_slash.len()..];
                if let Some(slash_idx) = rest.find('/') {
                    // Directory component
                    components.insert(rest[..slash_idx].to_string());
                } else if !rest.is_empty() {
                    // File component
                    components.insert(rest.to_string());
                }
            }
        }

        super::write_readdir_entries(components.iter().map(|s| s.as_str()), offset, buf)
    }
}

// ── Static file node ──────────────────────────────────────────────────────────

struct StaticFileNode {
    data: &'static [u8],
    ino: u64,
}

impl StaticFileNode {
    const fn new(data: &'static [u8], ino: u64) -> Self {
        Self { data, ino }
    }
}

impl VfsNode for StaticFileNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let off = offset as usize;
        if off >= self.data.len() {
            return Ok(0);
        }
        let avail = &self.data[off..];
        let n = avail.len().min(buf.len());
        buf[..n].copy_from_slice(&avail[..n]);
        Ok(n)
    }

    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EROFS)
    }

    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFREG | 0o444,
            size: self.data.len() as u64,
            ino: self.ino,
            ..Default::default()
        })
    }
}
