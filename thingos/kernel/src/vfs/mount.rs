//! VFS mount table.
//!
//! Maintains a list of (`mount_point`, `Arc<dyn VfsDriver>`) pairs sorted
//! longest-prefix-first so that the most specific mount is tried first.
//!
//! # Thread safety
//! The table is protected by a reader-writer spin-lock.  Mounts happen once
//! at boot; reads happen on every `open(2)` syscall, so a shared read lock
//! greatly reduces contention on the hot lookup path.

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;

use abi::errors::{Errno, SysResult};
use abi::syscall::mount_flags;
use spin::RwLock;

use super::VfsDriver;

#[derive(Clone)]
struct MountLayer {
    driver: Arc<dyn VfsDriver>,
    /// Stable per-mount identifier, assigned once at mount time.
    id: u64,
    /// Flags applied to this layer (e.g. MCREATE, MOUNT_COR).
    flags: u32,
}

#[derive(Clone)]
struct MountEntry {
    /// The canonical mount point, e.g. `"/dev"` (no trailing slash).
    prefix: String,
    stack: Vec<MountLayer>,
}

const GLOBAL_NAMESPACE_ID: u64 = 1;

static MOUNT_TABLES: RwLock<BTreeMap<u64, Vec<MountEntry>>> = RwLock::new(BTreeMap::new());
static INIT_DONE: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);
static NEXT_MOUNT_ID: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(1);

fn current_namespace_id() -> u64 {
    if let Some(pinfo) = crate::sched::process_info_current() {
        pinfo.lock().namespace.id()
    } else {
        GLOBAL_NAMESPACE_ID
    }
}

fn ensure_namespace_table_exists_locked(tables: &mut BTreeMap<u64, Vec<MountEntry>>, ns_id: u64) {
    if tables.contains_key(&ns_id) {
        return;
    }

    let initial_mount_table = tables.get(&GLOBAL_NAMESPACE_ID).cloned().unwrap_or_default();
    tables.insert(ns_id, initial_mount_table);
}

fn ensure_namespace(ns_id: u64) {
    let mut tables = MOUNT_TABLES.write();
    ensure_namespace_table_exists_locked(&mut tables, ns_id);
}

/// initialize the mount table storage.  Must be called once before any
/// [`mount`] or [`lookup`] call.
pub fn init() {
    let mut tables = MOUNT_TABLES.write();
    tables.clear();
    tables.insert(GLOBAL_NAMESPACE_ID, Vec::new());
    // The static tables are now ready.
    INIT_DONE.store(true, core::sync::atomic::Ordering::SeqCst);
}

/// Mount a filesystem driver at `mount_point` (e.g. `"/dev"`).
///
/// Thread-safe.
pub fn mount(mount_point: &str, driver: Arc<dyn VfsDriver>, flags: u32) {
    mount_for_namespace(current_namespace_id(), mount_point, driver, flags);
}

/// Mount a filesystem driver at `mount_point` in `ns_id`.
pub fn mount_for_namespace(ns_id: u64, mount_point: &str, driver: Arc<dyn VfsDriver>, flags: u32) {
    let prefix = normalise(mount_point);
    let id = NEXT_MOUNT_ID.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    let mut tables = MOUNT_TABLES.write();
    ensure_namespace_table_exists_locked(&mut tables, ns_id);
    let table = tables.get_mut(&ns_id).expect("namespace table should exist after ensure");

    let layer = MountLayer { driver, id, flags };

    if let Some(pos) = table.iter().position(|e| e.prefix == prefix) {
        if flags & mount_flags::MBEFORE != 0 {
            table[pos].stack.insert(0, layer);
        } else if flags & mount_flags::MAFTER != 0 {
            table[pos].stack.push(layer);
        } else {
            table[pos].stack.clear();
            table[pos].stack.push(layer);
        }
    } else {
        table.push(MountEntry { prefix, stack: alloc::vec![layer] });
        // Keep longest-prefix first so that `/dev/pts` beats `/dev`.
        table.sort_by(|a, b| b.prefix.len().cmp(&a.prefix.len()));
    }
}

/// Unmount the filesystem at `mount_point`.
///
/// Returns `Err(ENOENT)` if nothing is mounted there.
pub fn umount(mount_point: &str) -> SysResult<()> {
    umount_for_namespace(current_namespace_id(), mount_point)
}

/// Unmount the filesystem at `mount_point` in `ns_id`.
pub fn umount_for_namespace(ns_id: u64, mount_point: &str) -> SysResult<()> {
    let prefix = normalise(mount_point);
    let mut tables = MOUNT_TABLES.write();
    ensure_namespace_table_exists_locked(&mut tables, ns_id);
    let table = tables.get_mut(&ns_id).expect("namespace table should exist after ensure");
    let before = table.len();
    table.retain(|e| e.prefix != prefix);
    if table.len() == before { Err(Errno::ENOENT) } else { Ok(()) }
}

/// Resolve `path` to a VFS node by finding the best-matching mount and
/// calling its `lookup` with the remaining path component(s).
///
/// `path` must be absolute (start with `/`).
pub fn lookup(path: &str) -> SysResult<alloc::sync::Arc<dyn super::VfsNode>> {
    lookup_for_namespace(current_namespace_id(), path)
}

/// Resolve `path` in `ns_id`.
pub fn lookup_for_namespace(
    ns_id: u64,
    path: &str,
) -> SysResult<alloc::sync::Arc<dyn super::VfsNode>> {
    ensure_namespace(ns_id);
    if !path.starts_with('/') {
        return Err(Errno::ENOENT);
    }

    // Capture matching drivers into a local list to avoid holding the read
    // lock during potentially blocking driver lookups.
    let matches: Vec<(String, Vec<Arc<dyn VfsDriver>>)> = {
        let tables = MOUNT_TABLES.read();
        let Some(table) = tables.get(&ns_id) else {
            return Err(Errno::ENOENT);
        };
        table
            .iter()
            .filter_map(|entry| {
                strip_prefix(path, &entry.prefix).map(|rel| {
                    let drivers = entry.stack.iter().map(|l| Arc::clone(&l.driver)).collect();
                    (rel.to_string(), drivers)
                })
            })
            .collect()
    };

    for (rel, stack) in matches {
        let mut dir_layers = Vec::new();
        for driver in stack {
            match driver.lookup(&rel) {
                Ok(node) => {
                    if node.stat()?.is_dir() {
                        dir_layers.push(node);
                        continue;
                    }
                    return Ok(node);
                }
                Err(Errno::ENOENT) => {
                    // Specialized fallback for fb0 if the driver doesn't have it.
                    // This is a legacy hack for early-boot framebuffer access.
                    if rel == "fb0" && path.starts_with("/dev") {
                        if let Ok(node) = crate::vfs::devfs::DevFs::new().lookup("fb0") {
                            return Ok(node);
                        }
                    }
                    continue;
                }
                Err(err) => return Err(err),
            }
        }
        if !dir_layers.is_empty() {
            if dir_layers.len() == 1 {
                return Ok(dir_layers.pop().unwrap());
            }
            return Ok(Arc::new(crate::vfs::union::UnionDirNode::new(dir_layers)));
        }
    }

    Err(Errno::ENOENT)
}

/// Return all mount points that are immediate children of `parent_path`.
///
/// For example, if we have mounts at `/dev`, `/dev/display/card0`, and `/sys`,
/// `get_mounts_under("/dev/display")` would return `["card0"]`.
pub fn get_mounts_under(parent_path: &str) -> Vec<String> {
    get_mounts_under_for_namespace(current_namespace_id(), parent_path)
}

/// Return mount points immediately under `parent_path` in `ns_id`.
pub fn get_mounts_under_for_namespace(ns_id: u64, parent_path: &str) -> Vec<String> {
    ensure_namespace(ns_id);
    let prefix = normalise(parent_path);
    let tables = MOUNT_TABLES.read();
    let Some(table) = tables.get(&ns_id) else {
        return Vec::new();
    };
    let mut results = Vec::new();

    for entry in table.iter() {
        if entry.prefix == prefix {
            continue;
        }

        if let Some(rel) = strip_prefix(&entry.prefix, &prefix) {
            // Check if it's an immediate child (no further slashes).
            let rel = rel.trim_start_matches('/');
            if !rel.is_empty() && !rel.contains('/') {
                results.push(rel.to_string());
            }
        }
    }

    results
}

/// Return the stable mount ID for the best-matching mount covering `path`.
///
/// Returns `0` if no mount covers `path` (which should not happen for valid
/// absolute paths after the VFS is initialized).
pub fn mount_id_for_path(path: &str) -> u64 {
    mount_id_for_path_in_namespace(current_namespace_id(), path)
}

/// Return the stable mount ID for `path` in `ns_id`.
pub fn mount_id_for_path_in_namespace(ns_id: u64, path: &str) -> u64 {
    ensure_namespace(ns_id);
    if !path.starts_with('/') {
        return 0;
    }
    let tables = MOUNT_TABLES.read();
    let Some(table) = tables.get(&ns_id) else {
        return 0;
    };
    // The table is sorted longest-prefix first, so the first match is the
    // most specific mount. We return the topmost layer's ID.
    for entry in table.iter() {
        if strip_prefix(path, &entry.prefix).is_some() {
            if let Some(top) = entry.stack.first() {
                return top.id;
            }
        }
    }
    0
}

/// Return the VFS driver associated with `path`.
pub fn get_driver_for_path(path: &str) -> SysResult<Arc<dyn VfsDriver>> {
    get_driver_for_path_in_namespace(current_namespace_id(), path)
}

/// Return the VFS driver associated with `path` in `ns_id`.
pub fn get_driver_for_path_in_namespace(ns_id: u64, path: &str) -> SysResult<Arc<dyn VfsDriver>> {
    get_driver_and_relative_for_path_in_namespace(ns_id, path).map(|(driver, _)| driver)
}

/// Return the VFS driver associated with `path` plus the path relative to that mount.
pub fn get_driver_and_relative_for_path(path: &str) -> SysResult<(Arc<dyn VfsDriver>, String)> {
    get_driver_and_relative_for_path_in_namespace(current_namespace_id(), path)
}

/// Return the VFS driver associated with `path` plus the path relative to that mount.
pub fn get_driver_and_relative_for_path_in_namespace(
    ns_id: u64,
    path: &str,
) -> SysResult<(Arc<dyn VfsDriver>, String)> {
    ensure_namespace(ns_id);
    if !path.starts_with('/') {
        return Err(Errno::ENOENT);
    }
    let tables = MOUNT_TABLES.read();
    let Some(table) = tables.get(&ns_id) else {
        return Err(Errno::ENOENT);
    };
    for entry in table.iter() {
        if let Some(rel) = strip_prefix(path, &entry.prefix) {
            if let Some(top) = entry.stack.first() {
                return Ok((Arc::clone(&top.driver), rel.to_string()));
            }
        }
    }
    Err(Errno::ENOENT)
}

/// Create a new regular file at `path` by finding the best-matching mount.
///
/// `path` must be absolute.  Returns the new open node on success.
pub fn create(path: &str) -> SysResult<alloc::sync::Arc<dyn super::VfsNode>> {
    create_in_namespace(current_namespace_id(), path)
}

/// Create a regular file at `path` in `ns_id`.
pub fn create_in_namespace(
    ns_id: u64,
    path: &str,
) -> SysResult<alloc::sync::Arc<dyn super::VfsNode>> {
    ensure_namespace(ns_id);
    if !path.starts_with('/') {
        return Err(Errno::ENOENT);
    }
    let (rel, driver): (String, Arc<dyn VfsDriver>) = {
        let tables = MOUNT_TABLES.read();
        let Some(table) = tables.get(&ns_id) else {
            return Err(Errno::ENOENT);
        };
        table
            .iter()
            .find_map(|entry| {
                strip_prefix(path, &entry.prefix).and_then(|rel| {
                    let layer = entry
                        .stack
                        .iter()
                        .find(|l| (l.flags & mount_flags::MCREATE) != 0)
                        .or_else(|| entry.stack.first())?;
                    Some((rel.to_string(), Arc::clone(&layer.driver)))
                })
            })
            .ok_or(Errno::ENOENT)?
    };
    driver.create(&rel)
}

/// Create a directory at `path` by finding the best-matching mount.
///
/// `path` must be absolute.
pub fn mkdir(path: &str) -> SysResult<()> {
    mkdir_in_namespace(current_namespace_id(), path)
}

/// Create a directory at `path` in `ns_id`.
pub fn mkdir_in_namespace(ns_id: u64, path: &str) -> SysResult<()> {
    ensure_namespace(ns_id);
    if !path.starts_with('/') {
        return Err(Errno::ENOENT);
    }
    let (rel, driver): (String, Arc<dyn VfsDriver>) = {
        let tables = MOUNT_TABLES.read();
        let Some(table) = tables.get(&ns_id) else {
            return Err(Errno::ENOENT);
        };
        table
            .iter()
            .find_map(|entry| {
                strip_prefix(path, &entry.prefix).and_then(|rel| {
                    let layer = entry
                        .stack
                        .iter()
                        .find(|l| (l.flags & mount_flags::MCREATE) != 0)
                        .or_else(|| entry.stack.first())?;
                    Some((rel.to_string(), Arc::clone(&layer.driver)))
                })
            })
            .ok_or(Errno::ENOENT)?
    };
    driver.mkdir(&rel)
}

/// Remove the file or empty directory at `path`.
///
/// `path` must be absolute.
pub fn unlink(path: &str) -> SysResult<()> {
    unlink_in_namespace(current_namespace_id(), path)
}

/// Remove the file or empty directory at `path` in `ns_id`.
pub fn unlink_in_namespace(ns_id: u64, path: &str) -> SysResult<()> {
    ensure_namespace(ns_id);
    if !path.starts_with('/') {
        return Err(Errno::ENOENT);
    }
    let (rel, driver): (String, Arc<dyn VfsDriver>) = {
        let tables = MOUNT_TABLES.read();
        let Some(table) = tables.get(&ns_id) else {
            return Err(Errno::ENOENT);
        };
        table
            .iter()
            .find_map(|entry| {
                strip_prefix(path, &entry.prefix).and_then(|rel| {
                    let layer = entry
                        .stack
                        .iter()
                        .find(|l| (l.flags & mount_flags::MCREATE) != 0)
                        .or_else(|| entry.stack.first())?;
                    Some((rel.to_string(), Arc::clone(&layer.driver)))
                })
            })
            .ok_or(Errno::ENOENT)?
    };
    driver.unlink(&rel)
}

/// Create a symbolic link at `link_path` pointing to `target`.
///
/// `link_path` must be absolute.
pub fn symlink(target: &str, link_path: &str) -> SysResult<()> {
    symlink_in_namespace(current_namespace_id(), target, link_path)
}

/// Create a symbolic link in `ns_id`.
pub fn symlink_in_namespace(ns_id: u64, target: &str, link_path: &str) -> SysResult<()> {
    ensure_namespace(ns_id);
    if !link_path.starts_with('/') {
        return Err(Errno::ENOENT);
    }
    let (rel, driver): (String, Arc<dyn VfsDriver>) = {
        let tables = MOUNT_TABLES.read();
        let Some(table) = tables.get(&ns_id) else {
            return Err(Errno::ENOENT);
        };
        table
            .iter()
            .find_map(|entry| {
                strip_prefix(link_path, &entry.prefix).and_then(|rel| {
                    let layer = entry
                        .stack
                        .iter()
                        .find(|l| (l.flags & mount_flags::MCREATE) != 0)
                        .or_else(|| entry.stack.first())?;
                    Some((rel.to_string(), Arc::clone(&layer.driver)))
                })
            })
            .ok_or(Errno::ENOENT)?
    };
    driver.symlink(target, &rel)
}

/// Create a hard link at `dst_path` referring to the same file as `src_path`.
///
/// Both paths must be absolute and within the same mount point.
/// Returns `EXDEV` if the paths are on different mount points, and `ENOENT`
/// if either path has no matching mount.
pub fn link(src_path: &str, dst_path: &str) -> SysResult<()> {
    link_in_namespace(current_namespace_id(), src_path, dst_path)
}

/// Create a hard link in `ns_id`.
pub fn link_in_namespace(ns_id: u64, src_path: &str, dst_path: &str) -> SysResult<()> {
    ensure_namespace(ns_id);
    if !src_path.starts_with('/') || !dst_path.starts_with('/') {
        return Err(Errno::ENOENT);
    }
    let (src_rel, dst_rel, driver): (String, String, Arc<dyn VfsDriver>) = {
        let tables = MOUNT_TABLES.read();
        let Some(table) = tables.get(&ns_id) else {
            return Err(Errno::ENOENT);
        };
        let src_entry = table
            .iter()
            .find_map(|entry| {
                strip_prefix(src_path, &entry.prefix).and_then(|rel| {
                    let layer = entry
                        .stack
                        .iter()
                        .find(|l| (l.flags & mount_flags::MCREATE) != 0)
                        .or_else(|| entry.stack.first())?;
                    Some((rel.to_string(), Arc::clone(&layer.driver)))
                })
            })
            .ok_or(Errno::ENOENT)?;
        let dst_entry = table
            .iter()
            .find_map(|entry| {
                strip_prefix(dst_path, &entry.prefix).and_then(|rel| {
                    let layer = entry
                        .stack
                        .iter()
                        .find(|l| (l.flags & mount_flags::MCREATE) != 0)
                        .or_else(|| entry.stack.first())?;
                    Some((rel.to_string(), Arc::clone(&layer.driver)))
                })
            })
            .ok_or(Errno::ENOENT)?;
        if !Arc::ptr_eq(&src_entry.1, &dst_entry.1) {
            return Err(Errno::EXDEV);
        }
        (src_entry.0, dst_entry.0, src_entry.1)
    };
    driver.link(&src_rel, &dst_rel)
}

/// Rename a file or directory from `old_path` to `new_path`.
///
/// Both paths must be absolute and within the same mount point.
pub fn rename(old_path: &str, new_path: &str) -> SysResult<()> {
    rename_in_namespace(current_namespace_id(), old_path, new_path)
}

/// Rename a file or directory in `ns_id`.
pub fn rename_in_namespace(ns_id: u64, old_path: &str, new_path: &str) -> SysResult<()> {
    ensure_namespace(ns_id);
    if !old_path.starts_with('/') || !new_path.starts_with('/') {
        return Err(Errno::ENOENT);
    }
    let (old_rel, new_rel, driver): (String, String, Arc<dyn VfsDriver>) = {
        let tables = MOUNT_TABLES.read();
        let Some(table) = tables.get(&ns_id) else {
            return Err(Errno::ENOENT);
        };
        table
            .iter()
            .find_map(|entry| {
                let old_rel = strip_prefix(old_path, &entry.prefix)?;
                let new_rel = strip_prefix(new_path, &entry.prefix)?;
                let layer = entry
                    .stack
                    .iter()
                    .find(|l| (l.flags & mount_flags::MCREATE) != 0)
                    .or_else(|| entry.stack.first())?;
                Some((old_rel.to_string(), new_rel.to_string(), Arc::clone(&layer.driver)))
            })
            .ok_or(Errno::EXDEV)?
    };
    driver.rename(&old_rel, &new_rel)
}

/// Return a human-readable text listing of all active mount points.
///
/// Format:
/// ```text
/// <mount_point> <type> rw 0 0
/// ```
/// Used by `/proc/mounts`.
pub fn mounts_text() -> alloc::string::String {
    mounts_text_for_namespace(current_namespace_id())
}

/// Return text listing of mounts in `ns_id`.
pub fn mounts_text_for_namespace(ns_id: u64) -> alloc::string::String {
    ensure_namespace(ns_id);
    let tables = MOUNT_TABLES.read();
    let Some(table) = tables.get(&ns_id) else {
        return alloc::string::String::new();
    };
    let mut out = alloc::string::String::new();
    for entry in table.iter() {
        out.push_str(&entry.prefix);
        out.push_str(" vfs rw 0 0\n");
    }
    out
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn normalise(p: &str) -> String {
    // Strip trailing slash unless it is the root itself.
    let s = p.trim_end_matches('/');
    if s.is_empty() { String::from("/") } else { String::from(s) }
}

/// Returns the relative portion of `path` after `prefix`, if `path` starts
/// with that prefix followed by `/` or is exactly equal.
fn strip_prefix<'a>(path: &'a str, prefix: &str) -> Option<&'a str> {
    if prefix == "/" {
        // Root mount: everything after the leading slash.
        return Some(&path[1..]);
    }
    if path == prefix {
        return Some("");
    }
    // Avoid allocation: check if path starts with prefix followed by '/'.
    let prefix_with_slash = prefix.trim_end_matches('/');
    let rest = path.strip_prefix(prefix_with_slash)?;
    rest.strip_prefix('/')
}

#[cfg(test)]
mod tests {
    use alloc::sync::Arc;
    use alloc::vec;

    use abi::errors::Errno;

    use super::*;
    use crate::vfs::{VfsDriver, VfsNode, VfsStat};

    struct DummyNode;
    impl VfsNode for DummyNode {
        fn read(&self, _: u64, _: &mut [u8]) -> abi::errors::SysResult<usize> {
            Ok(0)
        }
        fn write(&self, _: u64, _: &[u8]) -> abi::errors::SysResult<usize> {
            Ok(0)
        }
        fn stat(&self) -> abi::errors::SysResult<VfsStat> {
            Ok(VfsStat { mode: VfsStat::S_IFCHR | 0o666, size: 0, ino: 99, ..Default::default() })
        }
    }

    struct DummyFs;
    impl VfsDriver for DummyFs {
        fn lookup(&self, path: &str) -> abi::errors::SysResult<Arc<dyn VfsNode>> {
            if path == "thing" { Ok(Arc::new(DummyNode)) } else { Err(Errno::ENOENT) }
        }
    }

    fn fresh_table() {
        let mut tables = MOUNT_TABLES.write();
        tables.clear();
        tables.insert(GLOBAL_NAMESPACE_ID, Vec::new());
        INIT_DONE.store(true, core::sync::atomic::Ordering::SeqCst);
    }

    #[test]
    fn test_mount_and_lookup() {
        fresh_table();
        mount("/test", Arc::new(DummyFs), mount_flags::MREPL);
        assert!(lookup("/test/thing").is_ok());
    }

    #[test]
    fn test_lookup_unknown_path_returns_enoent() {
        fresh_table();
        mount("/test", Arc::new(DummyFs), mount_flags::MREPL);
        assert!(matches!(lookup("/other/thing"), Err(Errno::ENOENT)));
    }

    #[test]
    fn test_umount_removes_mount() {
        fresh_table();
        mount("/rm", Arc::new(DummyFs), mount_flags::MREPL);
        assert!(lookup("/rm/thing").is_ok());
        umount("/rm").unwrap();
        assert!(matches!(lookup("/rm/thing"), Err(Errno::ENOENT)));
    }

    #[test]
    fn test_longer_prefix_wins() {
        fresh_table();

        struct Short;
        impl VfsDriver for Short {
            fn lookup(&self, _: &str) -> abi::errors::SysResult<Arc<dyn VfsNode>> {
                Err(Errno::EIO)
            }
        }
        mount("/a", Arc::new(Short), mount_flags::MREPL);
        mount("/a/b", Arc::new(DummyFs), mount_flags::MREPL);

        // "/a/b/thing" should match the longer prefix "/a/b".
        assert!(lookup("/a/b/thing").is_ok());
        // "/a/thing" hits the short driver which returns EIO (not ENOENT).
        assert!(matches!(lookup("/a/thing"), Err(Errno::EIO)));
    }

    #[test]
    fn test_strip_prefix_helper() {
        assert_eq!(strip_prefix("/dev/console", "/dev"), Some("console"));
        assert_eq!(strip_prefix("/dev", "/dev"), Some(""));
        assert_eq!(strip_prefix("/other/path", "/dev"), None);
        assert_eq!(strip_prefix("/hello", "/"), Some("hello"));
    }

    #[test]
    fn test_isolated_namespace_starts_with_global_mounts_but_diverges() {
        fresh_table();
        let global_ns = GLOBAL_NAMESPACE_ID;
        let isolated_ns = GLOBAL_NAMESPACE_ID + 7;
        mount_for_namespace(global_ns, "/shared", Arc::new(DummyFs), mount_flags::MREPL);

        assert!(lookup_for_namespace(global_ns, "/shared/thing").is_ok());
        assert!(lookup_for_namespace(isolated_ns, "/shared/thing").is_ok());

        mount_for_namespace(isolated_ns, "/private", Arc::new(DummyFs), mount_flags::MREPL);
        assert!(lookup_for_namespace(isolated_ns, "/private/thing").is_ok());
        assert!(matches!(lookup_for_namespace(global_ns, "/private/thing"), Err(Errno::ENOENT)));
    }
}
