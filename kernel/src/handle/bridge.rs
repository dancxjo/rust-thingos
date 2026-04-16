//! Bridge layer: canonical handle lookup/ownership over transitional tables.
//!
//! # Purpose
//!
//! This module is the explicit compatibility boundary between:
//! - canonical kernel `Handle` semantics, and
//! - transitional raw-fd / `thing_table` / `IpcThingTable` backing.
//!
//! New code should resolve resources through this bridge rather than manually
//! probing `thing_table` and global IPC handle tables in syscall handlers.

use abi::errors::{Errno, SysResult};
use alloc::sync::Arc;
use spin::Mutex;

/// Canonical kernel handle token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Handle(pub u32);

/// Canonical resource class behind a handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandleKind {
    File,
    Pipe,
    Channel,
}

/// Resolved handle target across compatibility tables.
#[derive(Clone)]
pub enum ResolvedHandle {
    FileLike {
        kind: HandleKind,
        node: Arc<dyn crate::vfs::VfsNode>,
    },
    Channel {
        port: Arc<crate::ipc::Port>,
        mode: crate::ipc::IpcThingMode,
    },
}

impl ResolvedHandle {
    pub fn kind(&self) -> HandleKind {
        match self {
            Self::FileLike { kind, .. } => *kind,
            Self::Channel { .. } => HandleKind::Channel,
        }
    }
}

fn classify_file_like(node: &Arc<dyn crate::vfs::VfsNode>) -> HandleKind {
    // Classification is best-effort only and must not fail lookup paths.
    // If metadata cannot be read, keep the conservative default `File`.
    match node.stat() {
        Ok(stat) if stat.is_fifo() => HandleKind::Pipe,
        _ => HandleKind::File,
    }
}

fn lookup_ipc_entry_with(
    handle: Handle,
    lookup: impl FnOnce(
        &crate::ipc::IpcThingTable,
        crate::ipc::IpcThing,
    ) -> Option<&crate::ipc::IpcThingEntry>,
) -> SysResult<crate::ipc::IpcThingEntry> {
    let ipc_handle = crate::ipc::IpcThing(handle.0);
    // Keep this lock scope tiny: copy the entry and drop immediately.
    // This path is intentionally short because it sits on syscall hot paths.
    let table = crate::ipc::GLOBAL_THING_TABLE.lock();
    lookup(&table, ipc_handle).copied().ok_or(Errno::EBADF)
}

fn lookup_ipc_entry_any(handle: Handle) -> SysResult<crate::ipc::IpcThingEntry> {
    lookup_ipc_entry_with(handle, |table, ipc_handle| table.get_any(ipc_handle))
}

fn lookup_ipc_entry_write(handle: Handle) -> SysResult<crate::ipc::IpcThingEntry> {
    lookup_ipc_entry_with(handle, |table, ipc_handle| {
        table.get(ipc_handle, crate::ipc::IpcThingMode::Write)
    })
}

/// Resolve a handle by checking the process-owned open table first, then the
/// IPC handle table compatibility path.
pub fn resolve_handle(
    pinfo_arc: &Arc<Mutex<crate::task::ProcessInfo>>,
    handle: Handle,
) -> SysResult<ResolvedHandle> {
    {
        let lock = pinfo_arc.lock();
        if let Ok(file) = lock.thing_table.get(handle.0) {
            let node = file.node.clone();
            return Ok(ResolvedHandle::FileLike {
                kind: classify_file_like(&node),
                node,
            });
        }
    }

    let entry = lookup_ipc_entry_any(handle)?;
    let port = crate::ipc::get_port(entry.port_id).ok_or(Errno::EBADF)?;
    Ok(ResolvedHandle::Channel {
        port,
        mode: entry.mode,
    })
}

/// Resolve a handle for stream I/O call paths that consume a `VfsNode`.
pub fn resolve_io_node_compat(
    pinfo_arc: &Arc<Mutex<crate::task::ProcessInfo>>,
    handle: Handle,
) -> SysResult<Arc<dyn crate::vfs::VfsNode>> {
    match resolve_handle(pinfo_arc, handle)? {
        ResolvedHandle::FileLike { node, .. } => Ok(node),
        ResolvedHandle::Channel { port, mode } => {
            Ok(Arc::new(crate::vfs::port_node::PortNode::new(port, mode)))
        }
    }
}

/// Explicit fd compatibility boundary: project an IPC channel handle into a
/// process `thing_table` slot so fd-based APIs can consume it.
pub fn install_fd_compat_for_channel_handle(
    pinfo_arc: &Arc<Mutex<crate::task::ProcessInfo>>,
    handle: Handle,
) -> SysResult<u32> {
    let entry = lookup_ipc_entry_any(handle)?;
    let port = crate::ipc::get_port(entry.port_id).ok_or(Errno::EBADF)?;
    let node = Arc::new(crate::vfs::port_node::PortNode::new(port, entry.mode));

    let mut lock = pinfo_arc.lock();
    lock.thing_table.open(
        node,
        match entry.mode {
            crate::ipc::IpcThingMode::Read => crate::vfs::OpenFlags::read_only(),
            crate::ipc::IpcThingMode::Write => crate::vfs::OpenFlags::write_only(),
        },
        alloc::format!("handle:{}", handle.0),
    )
}

/// Resolve a write-capable channel endpoint from a compat token (fd or handle).
///
/// Used where migration still accepts either token shape at syscall boundaries.
pub fn resolve_write_port_compat(
    pinfo_arc: &Arc<Mutex<crate::task::ProcessInfo>>,
    handle: Handle,
) -> SysResult<Arc<crate::ipc::Port>> {
    {
        let lock = pinfo_arc.lock();
        if let Ok(file) = lock.thing_table.get(handle.0) {
            if let Some(port) = file.node.as_port() {
                return Ok(port);
            }
        }
    }

    let entry = lookup_ipc_entry_write(handle)?;
    crate::ipc::get_port(entry.port_id).ok_or(Errno::EBADF)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vfs::thing_table::ThingTable;
    use crate::vfs::{OpenFlags, VfsNode, VfsStat};

    struct NullNode;

    impl VfsNode for NullNode {
        fn read(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
            Ok(0)
        }

        fn write(&self, _offset: u64, buf: &[u8]) -> SysResult<usize> {
            Ok(buf.len())
        }

        fn stat(&self) -> SysResult<VfsStat> {
            Ok(VfsStat::default())
        }
    }

    fn make_test_process_info(
        thing_table_entries: &[(u32, Arc<dyn VfsNode>)],
    ) -> Arc<Mutex<crate::task::ProcessInfo>> {
        let mut thing_table = ThingTable::new();
        for (fd, node) in thing_table_entries {
            thing_table
                .insert_at(*fd, node.clone(), OpenFlags::read_write(), "/test".into())
                .expect("insert_at");
        }
        Arc::new(Mutex::new(crate::task::ProcessInfo {
            pid: 1,
            job: crate::task::ProcessLifecycle::new(0, 1),
            unix_compat: crate::task::ProcessUnixCompat::isolated(1, false),
            thing_table,
            namespace: crate::vfs::NamespaceRef::global(),
            cwd: alloc::string::String::from("/"),
            root: alloc::string::String::from("/"),
            exec_path: alloc::string::String::new(),
            authority: crate::task::ProcessAuthority::root(),
            space: crate::task::ProcessAddressSpace::empty(),
        }))
    }

    #[test]
    fn test_resolve_handle_prefers_process_table_over_global_ipc_table() {
        let port_id = crate::ipc::create_port(8);
        let handle = {
            let mut table = crate::ipc::GLOBAL_THING_TABLE.lock();
            table
                .alloc(port_id, crate::ipc::IpcThingMode::Read)
                .expect("allocate handle")
        };

        let node: Arc<dyn VfsNode> = Arc::new(NullNode);
        let pinfo = make_test_process_info(&[(handle.0, node)]);
        let resolved = resolve_handle(&pinfo, Handle(handle.0)).expect("resolve");
        assert_eq!(resolved.kind(), HandleKind::File);

        {
            let mut table = crate::ipc::GLOBAL_THING_TABLE.lock();
            let _ = table.close(handle);
        }
        crate::ipc::close_port(port_id);
    }

    #[test]
    fn test_install_fd_compat_for_channel_handle_creates_port_backed_fd() {
        let port_id = crate::ipc::create_port(8);
        let handle = {
            let mut table = crate::ipc::GLOBAL_THING_TABLE.lock();
            table
                .alloc(port_id, crate::ipc::IpcThingMode::Read)
                .expect("allocate handle")
        };

        let pinfo = make_test_process_info(&[]);
        let fd = install_fd_compat_for_channel_handle(&pinfo, Handle(handle.0)).expect("fd");

        let lock = pinfo.lock();
        let open = lock.thing_table.get(fd).expect("fd entry");
        assert!(open.node.as_port().is_some(), "fd must wrap a channel port");

        drop(lock);
        {
            let mut table = crate::ipc::GLOBAL_THING_TABLE.lock();
            let _ = table.close(handle);
        }
        crate::ipc::close_port(port_id);
    }
}
