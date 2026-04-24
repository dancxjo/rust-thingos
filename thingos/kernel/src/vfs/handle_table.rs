//! Per-process VFS handle table.
//!
//! Each process that uses VFS syscalls has an `HandleTable` embedded inside its
//! `ProcessInfo`.  Kernel threads do not have an `HandleTable` and all VFS
//! syscalls return `ENOENT` for them.
//!
//! Handles are non-negative integers starting at 0.  Descriptors 0,
//! 1, and 2 are **stdio** (stdin / stdout / stderr) and are pre-populated at
//! process spawn time via [`HandleTable::insert_at`].  All other descriptors are
//! allocated by [`HandleTable::open`] which scans for the lowest free slot.
//!
//! # Limits
//! `MAX_HANDLES` open handles per process.  This is intentionally small for now.

use alloc::string::String;
use alloc::sync::Arc;

use abi::errors::{Errno, SysResult};
use spin::Mutex;

use super::{OpenFlags, VfsNode};

pub const MAX_HANDLES: usize = 256;
pub const HANDLE_CLOEXEC: u32 = abi::syscall::handle_flags::HANDLE_CLOEXEC;

/// A single open-handle entry in the FD table.
///
/// The `offset` is wrapped in `Arc<Mutex<u64>>` so that handles
/// created by `dup` or `dup2` share the same file position, matching POSIX
/// open-handle-description semantics.
pub struct OpenHandle {
    pub node: Arc<dyn VfsNode>,
    pub status_flags: Arc<Mutex<OpenFlags>>,
    pub handle_flags: u32,
    /// Absolute path of this open file (if known).
    pub path: Arc<String>,
    /// Shared read/write position — cloned (not copied) on dup/dup2.
    pub offset: Arc<Mutex<u64>>,
}

impl Clone for OpenHandle {
    fn clone(&self) -> Self {
        self.node.on_dup();
        Self {
            node: self.node.clone(),
            status_flags: self.status_flags.clone(),
            handle_flags: self.handle_flags,
            path: self.path.clone(),
            offset: self.offset.clone(),
        }
    }
}

/// Per-process handle table.
#[derive(Clone)]
pub struct HandleTable {
    entries: alloc::vec::Vec<Option<OpenHandle>>,
}

impl HandleTable {
    pub fn new() -> Self {
        let mut entries = alloc::vec::Vec::with_capacity(MAX_HANDLES);
        for _ in 0..MAX_HANDLES {
            entries.push(None);
        }
        Self { entries }
    }

    /// Insert `node` into the table and return the allocated handle.
    ///
    /// Scans from slot 0 and returns the first free slot.  Because slots 0–2
    /// are pre-populated at spawn time, regular opens naturally receive handle ≥ 3.
    /// Returns `EMFILE` when all slots are exhausted.
    pub fn open(
        &mut self,
        node: Arc<dyn VfsNode>,
        flags: OpenFlags,
        path: String,
    ) -> SysResult<u32> {
        for i in 0..MAX_HANDLES {
            if self.entries[i].is_none() {
                self.entries[i] = Some(OpenHandle {
                    node,
                    status_flags: Arc::new(Mutex::new(flags)),
                    handle_flags: 0,
                    path: Arc::new(path),
                    offset: Arc::new(Mutex::new(0)),
                });
                return Ok(i as u32);
            }
        }
        Err(Errno::EMFILE)
    }

    /// Insert `node` at a specific handle slot.
    ///
    /// Used at process creation to populate stdin (0), stdout (1), stderr (2).
    /// Returns `EBADF` if `fd` is out of range or already occupied.
    pub fn insert_at(
        &mut self,
        thing: u32,
        node: Arc<dyn VfsNode>,
        flags: OpenFlags,
        path: String,
    ) -> SysResult<()> {
        let idx = thing as usize;
        if idx >= MAX_HANDLES {
            return Err(Errno::EBADF);
        }
        if self.entries[idx].is_some() {
            return Err(Errno::EBADF);
        }
        self.entries[idx] = Some(OpenHandle {
            node,
            status_flags: Arc::new(Mutex::new(flags)),
            handle_flags: 0,
            path: Arc::new(path),
            offset: Arc::new(Mutex::new(0)),
        });
        Ok(())
    }

    /// Duplicate `old_handle` to the lowest available descriptor.
    ///
    /// The new descriptor shares the same `VfsNode` **and** file offset as
    /// `old_handle`, matching POSIX open-handle-description semantics.
    /// Returns the new handle, or `EBADF` if `old_handle` is not open.
    pub fn dup(&mut self, old_handle: u32) -> SysResult<u32> {
        let idx = old_handle as usize;
        if idx >= MAX_HANDLES {
            return Err(Errno::EBADF);
        }
        let mut entry = self.entries[idx].clone().ok_or(Errno::EBADF)?;
        entry.handle_flags &= !HANDLE_CLOEXEC;
        for i in 0..MAX_HANDLES {
            if self.entries[i].is_none() {
                self.entries[i] = Some(entry);
                return Ok(i as u32);
            }
        }
        Err(Errno::EMFILE)
    }

    /// Duplicate `old_handle` to `new_handle`.
    ///
    /// If `new_handle` is already open it is closed first.  If `old_handle == new_handle`
    /// this is a no-op that returns `new_handle`.  Both descriptors will share the
    /// same file offset after this call.  Returns `EBADF` if `old_handle` is not
    /// open or either handle is out of range.
    pub fn dup2(&mut self, old_handle: u32, new_handle: u32) -> SysResult<u32> {
        let old_idx = old_handle as usize;
        let new_idx = new_handle as usize;
        if old_idx >= MAX_HANDLES || new_idx >= MAX_HANDLES {
            return Err(Errno::EBADF);
        }
        if old_handle == new_handle {
            // Verify old_handle is open.
            self.entries[old_idx].as_ref().ok_or(Errno::EBADF)?;
            return Ok(new_handle);
        }
        let mut entry = self.entries[old_idx].clone().ok_or(Errno::EBADF)?;
        entry.handle_flags &= !HANDLE_CLOEXEC;
        // Close new_handle if open.
        if let Some(old_entry) = self.entries[new_idx].take() {
            old_entry.node.close();
        }
        self.entries[new_idx] = Some(entry);
        Ok(new_handle)
    }

    /// Return a reference to the open-handle entry for `fd`, or `EBADF`.
    pub fn get(&self, thing: u32) -> SysResult<&OpenHandle> {
        let idx = thing as usize;
        if idx >= MAX_HANDLES {
            return Err(Errno::EBADF);
        }
        self.entries[idx].as_ref().ok_or(Errno::EBADF)
    }

    /// Return a mutable reference to the open-handle entry for `fd`, or `EBADF`.
    pub fn get_mut(&mut self, thing: u32) -> SysResult<&mut OpenHandle> {
        let idx = thing as usize;
        if idx >= MAX_HANDLES {
            return Err(Errno::EBADF);
        }
        self.entries[idx].as_mut().ok_or(Errno::EBADF)
    }

    /// Read the thing flags (`FD_*`) for `fd`.
    pub fn get_handle_flags(&self, thing: u32) -> SysResult<u32> {
        Ok(self.get(thing)?.handle_flags)
    }

    /// Replace the thing flags (`FD_*`) for `fd`.
    pub fn set_handle_flags(&mut self, thing: u32, flags: u32) -> SysResult<()> {
        let entry = self.get_mut(thing)?;
        entry.handle_flags = flags & HANDLE_CLOEXEC;
        Ok(())
    }

    /// Remove the entry for handle `fd` from the table and return it.
    /// Returns `EBADF` if not open.
    pub fn take(&mut self, thing: u32) -> SysResult<OpenHandle> {
        let idx = thing as usize;
        if idx >= MAX_HANDLES {
            return Err(Errno::EBADF);
        }
        self.entries[idx].take().ok_or(Errno::EBADF)
    }

    /// Close handle `fd`.  Returns `EBADF` if not open.
    pub fn close(&mut self, thing: u32) -> SysResult<()> {
        let entry = self.take(thing)?;
        entry.node.close();
        Ok(())
    }

    /// Close all open handles (called on process exit).
    pub fn close_all(&mut self) {
        for slot in self.entries.iter_mut() {
            if let Some(entry) = slot.take() {
                entry.node.close();
            }
        }
    }

    /// Close all handles that have the `HANDLE_CLOEXEC` flag set.
    ///
    /// Called during `exec` to implement close-on-exec semantics.  File
    /// descriptors without `HANDLE_CLOEXEC` are preserved across the exec.
    pub fn close_on_exec(&mut self) {
        for (_, slot) in self.entries.iter_mut().enumerate() {
            let should_close =
                slot.as_ref().map(|e| e.handle_flags & HANDLE_CLOEXEC != 0).unwrap_or(false);
            if should_close {
                if let Some(entry) = slot.take() {
                    entry.node.close();
                }
            }
        }
    }
}

impl Drop for HandleTable {
    fn drop(&mut self) {
        self.close_all();
    }
}

impl Default for HandleTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use alloc::sync::Arc;

    use abi::errors::Errno;

    use super::*;
    use crate::vfs::{VfsNode, VfsStat};

    struct NullNode;
    impl VfsNode for NullNode {
        fn read(&self, _: u64, _: &mut [u8]) -> SysResult<usize> {
            Ok(0)
        }
        fn write(&self, _: u64, buf: &[u8]) -> SysResult<usize> {
            Ok(buf.len())
        }
        fn stat(&self) -> SysResult<VfsStat> {
            Ok(VfsStat { mode: VfsStat::S_IFCHR | 0o666, size: 0, ino: 1, ..Default::default() })
        }
    }

    fn null_node() -> Arc<dyn VfsNode> {
        Arc::new(NullNode)
    }

    #[test]
    fn test_open_allocates_from_0_when_empty() {
        let mut table = HandleTable::new();
        let thing = table.open(null_node(), OpenFlags::read_only(), "/null".into()).unwrap();
        assert_eq!(thing, 0, "first thing in empty table should be 0");
    }

    #[test]
    fn test_open_skips_occupied_slots() {
        let mut table = HandleTable::new();
        // Pre-populate slots 0-2 (simulate stdio setup).
        table.insert_at(0, null_node(), OpenFlags::read_only(), "/in".into()).unwrap();
        table.insert_at(1, null_node(), OpenFlags::write_only(), "/out".into()).unwrap();
        table.insert_at(2, null_node(), OpenFlags::write_only(), "/err".into()).unwrap();
        let thing = table.open(null_node(), OpenFlags::read_only(), "/null".into()).unwrap();
        assert_eq!(thing, 3, "first non-stdio VFS thing should be 3");
    }

    #[test]
    fn test_open_sequential_fds() {
        let mut table = HandleTable::new();
        // Pre-populate slots 0-2 (simulate stdio setup).
        table.insert_at(0, null_node(), OpenFlags::read_only(), "/in".into()).unwrap();
        table.insert_at(1, null_node(), OpenFlags::write_only(), "/out".into()).unwrap();
        table.insert_at(2, null_node(), OpenFlags::write_only(), "/err".into()).unwrap();
        let fd1 = table.open(null_node(), OpenFlags::read_only(), "/f1".into()).unwrap();
        let fd2 = table.open(null_node(), OpenFlags::read_only(), "/f2".into()).unwrap();
        assert_eq!(fd1, 3);
        assert_eq!(fd2, 4);
    }

    #[test]
    fn test_get_unknown_fd_returns_ebadf() {
        let table = HandleTable::new();
        assert!(matches!(table.get(10), Err(Errno::EBADF)));
    }

    #[test]
    fn test_close_frees_slot() {
        let mut table = HandleTable::new();
        let thing = table.open(null_node(), OpenFlags::read_only(), "/null".into()).unwrap();
        table.close(thing).unwrap();
        assert!(matches!(table.get(thing), Err(Errno::EBADF)));
    }

    #[test]
    fn test_close_reuses_slot() {
        let mut table = HandleTable::new();
        let fd1 = table.open(null_node(), OpenFlags::read_only(), "/f1".into()).unwrap();
        table.close(fd1).unwrap();
        let fd2 = table.open(null_node(), OpenFlags::read_only(), "/f2".into()).unwrap();
        // Slot 0 was freed, so it should be reused.
        assert_eq!(fd2, 0);
    }

    #[test]
    fn test_close_ebadf_for_not_open() {
        let mut table = HandleTable::new();
        assert!(matches!(table.close(99), Err(Errno::EBADF)));
    }

    #[test]
    fn test_insert_at_populates_specific_slot() {
        let mut table = HandleTable::new();
        table.insert_at(0, null_node(), OpenFlags::read_only(), "/in".into()).unwrap();
        table.insert_at(1, null_node(), OpenFlags::write_only(), "/out".into()).unwrap();
        table.insert_at(2, null_node(), OpenFlags::write_only(), "/err".into()).unwrap();
        assert!(table.get(0).is_ok());
        assert!(table.get(1).is_ok());
        assert!(table.get(2).is_ok());
    }

    #[test]
    fn test_insert_at_rejects_occupied_slot() {
        let mut table = HandleTable::new();
        table.insert_at(0, null_node(), OpenFlags::read_only(), "/null".into()).unwrap();
        assert!(matches!(
            table.insert_at(0, null_node(), OpenFlags::read_only(), "/null".into()),
            Err(Errno::EBADF)
        ));
    }

    #[test]
    fn test_insert_at_rejects_out_of_range() {
        let mut table = HandleTable::new();
        assert!(matches!(
            table.insert_at(
                MAX_HANDLES as u32,
                null_node(),
                OpenFlags::read_only(),
                "/null".into()
            ),
            Err(Errno::EBADF)
        ));
    }

    #[test]
    fn test_dup_clones_to_next_free() {
        let mut table = HandleTable::new();
        table.insert_at(0, null_node(), OpenFlags::read_only(), "/null".into()).unwrap();
        let new_handle = table.dup(0).unwrap();
        assert_eq!(new_handle, 1, "dup should use first free slot after 0");
        assert!(table.get(1).is_ok());
    }

    #[test]
    fn test_dup_ebadf_for_closed_fd() {
        let mut table = HandleTable::new();
        assert!(matches!(table.dup(5), Err(Errno::EBADF)));
    }

    #[test]
    fn test_dup2_creates_alias() {
        let mut table = HandleTable::new();
        table.insert_at(0, null_node(), OpenFlags::read_only(), "/null".into()).unwrap();
        let result = table.dup2(0, 5).unwrap();
        assert_eq!(result, 5);
        assert!(table.get(5).is_ok());
        // Original still open.
        assert!(table.get(0).is_ok());
    }

    #[test]
    fn test_dup2_closes_existing_target() {
        let mut table = HandleTable::new();
        table.insert_at(0, null_node(), OpenFlags::read_only(), "/in".into()).unwrap();
        table.insert_at(1, null_node(), OpenFlags::write_only(), "/out".into()).unwrap();
        // dup2(0, 1) should close slot 1 and replace it with a dup of slot 0.
        table.dup2(0, 1).unwrap();
        assert!(table.get(1).is_ok());
    }

    #[test]
    fn test_dup2_same_fd_is_noop() {
        let mut table = HandleTable::new();
        table.insert_at(3, null_node(), OpenFlags::read_only(), "/null".into()).unwrap();
        let result = table.dup2(3, 3).unwrap();
        assert_eq!(result, 3);
        assert!(table.get(3).is_ok());
    }

    #[test]
    fn test_dup2_ebadf_for_closed_old_thing() {
        let mut table = HandleTable::new();
        assert!(matches!(table.dup2(99, 5), Err(Errno::EBADF)));
    }

    #[test]
    fn test_dup_shares_offset() {
        let mut table = HandleTable::new();
        table.insert_at(0, null_node(), OpenFlags::read_only(), "/null".into()).unwrap();
        let new_handle = table.dup(0).unwrap();
        // Advance the original fd's offset.
        *table.get(0).unwrap().offset.lock() = 42;
        // Duplicated thing should see the same offset.
        let dup_offset = *table.get(new_handle).unwrap().offset.lock();
        assert_eq!(dup_offset, 42, "dup should share file offset");
    }

    #[test]
    fn test_dup2_shares_offset() {
        let mut table = HandleTable::new();
        table.insert_at(0, null_node(), OpenFlags::read_only(), "/null".into()).unwrap();
        table.dup2(0, 5).unwrap();
        // Advance via thing 5.
        *table.get(5).unwrap().offset.lock() = 100;
        // thing 0 should see the same value.
        let orig_offset = *table.get(0).unwrap().offset.lock();
        assert_eq!(orig_offset, 100, "dup2 should share file offset");
    }

    #[test]
    fn test_dup_shares_status_flags() {
        let mut table = HandleTable::new();
        table
            .insert_at(0, null_node(), OpenFlags(abi::syscall::vfs_flags::O_RDONLY), "/null".into())
            .unwrap();
        let new_handle = table.dup(0).unwrap();

        *table.get(0).unwrap().status_flags.lock() =
            OpenFlags(abi::syscall::vfs_flags::O_RDONLY | abi::syscall::vfs_flags::O_NONBLOCK);

        let dup_flags = *table.get(new_handle).unwrap().status_flags.lock();
        assert!(dup_flags.is_nonblock(), "dup should share status flags");
    }

    #[test]
    fn test_dup_clears_descriptor_flags() {
        let mut table = HandleTable::new();
        table.insert_at(0, null_node(), OpenFlags::read_only(), "/null".into()).unwrap();
        table.set_handle_flags(0, HANDLE_CLOEXEC).unwrap();

        let new_handle = table.dup(0).unwrap();

        assert_eq!(table.get_handle_flags(0).unwrap(), HANDLE_CLOEXEC);
        assert_eq!(
            table.get_handle_flags(new_handle).unwrap(),
            0,
            "dup should not inherit HANDLE_CLOEXEC"
        );
    }

    #[test]
    fn test_emfile_when_full() {
        // Open MAX_HANDLES files and ensure EMFILE on the next open.
        let mut table = HandleTable::new();
        let mut count = 0usize;
        loop {
            match table.open(null_node(), OpenFlags::read_only(), "/null".to_string()) {
                Ok(_) => count += 1,
                Err(Errno::EMFILE) => break,
                Err(e) => panic!("unexpected error {:?}", e),
            }
        }
        assert_eq!(count, MAX_HANDLES);
    }

    // ── close_on_exec tests ───────────────────────────────────────────────────

    /// FDs with HANDLE_CLOEXEC set should be closed; others should survive.
    #[test]
    fn test_close_on_exec_closes_flagged_fds() {
        let mut table = HandleTable::new();
        // thing 0: no flag → should survive exec
        table.insert_at(0, null_node(), OpenFlags::read_only(), "/in".into()).unwrap();
        // thing 1: HANDLE_CLOEXEC → should be closed on exec
        table.insert_at(1, null_node(), OpenFlags::write_only(), "/out".into()).unwrap();
        table.set_handle_flags(1, HANDLE_CLOEXEC).unwrap();
        // thing 3: HANDLE_CLOEXEC → should be closed on exec
        table.insert_at(3, null_node(), OpenFlags::read_only(), "/extra".into()).unwrap();
        table.set_handle_flags(3, HANDLE_CLOEXEC).unwrap();

        table.close_on_exec();

        assert!(table.get(0).is_ok(), "thing 0 (no HANDLE_CLOEXEC) should survive");
        assert!(
            matches!(table.get(1), Err(Errno::EBADF)),
            "thing 1 (HANDLE_CLOEXEC) should be closed"
        );
        assert!(
            matches!(table.get(3), Err(Errno::EBADF)),
            "thing 3 (HANDLE_CLOEXEC) should be closed"
        );
    }

    /// If no FDs are flagged, close_on_exec is a no-op.
    #[test]
    fn test_close_on_exec_preserves_unflagged_fds() {
        let mut table = HandleTable::new();
        table.insert_at(0, null_node(), OpenFlags::read_only(), "/in".into()).unwrap();
        table.insert_at(1, null_node(), OpenFlags::write_only(), "/out".into()).unwrap();

        table.close_on_exec();

        assert!(table.get(0).is_ok(), "thing 0 should survive");
        assert!(table.get(1).is_ok(), "thing 1 should survive");
    }

    /// An empty table is handled gracefully.
    #[test]
    fn test_close_on_exec_empty_table() {
        let mut table = HandleTable::new();
        // Should not panic.
        table.close_on_exec();
    }

    /// After close_on_exec the closed slot can be reused.
    #[test]
    fn test_close_on_exec_slot_reuse() {
        let mut table = HandleTable::new();
        table.insert_at(0, null_node(), OpenFlags::read_only(), "/f".into()).unwrap();
        table.set_handle_flags(0, HANDLE_CLOEXEC).unwrap();

        table.close_on_exec();

        // Slot 0 is now free; the next open() should reuse it.
        let new_handle = table.open(null_node(), OpenFlags::read_only(), "/new".into()).unwrap();
        assert_eq!(new_handle, 0, "freed cloexec slot should be reusable");
    }
}
