//! Bridge layer: canonical handle lookup/ownership over transitional tables.
//!
//! # Purpose
//!
//! This module is the explicit compatibility boundary between:
//! - canonical kernel `Handle` semantics, and
//! - transitional raw-fd / `thing_table` / `IpcHandleTable` backing.
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
    Port,
}

/// Resolved handle target across compatibility tables.
#[derive(Clone)]
pub enum ResolvedHandle {
    FileLike {
        kind: HandleKind,
        node: Arc<dyn crate::vfs::VfsNode>,
    },
    Port {
        port: Arc<crate::ipc::Port>,
        mode: crate::ipc::IpcHandleMode,
    },
}

impl ResolvedHandle {
    pub fn kind(&self) -> HandleKind {
        match self {
            Self::FileLike { kind, .. } => *kind,
            Self::Port { .. } => HandleKind::Port,
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

/// Shared IPC-table lookup helper that keeps lock hold time minimal by copying
/// the matched entry out of the table before returning.
fn lookup_ipc_entry_with(
    ipc_table: &crate::ipc::IpcHandleTable,
    handle: Handle,
    lookup: impl FnOnce(
        &crate::ipc::IpcHandleTable,
        crate::ipc::IpcHandle,
    ) -> Option<&crate::ipc::IpcHandleEntry>,
) -> SysResult<crate::ipc::IpcHandleEntry> {
    let ipc_handle = crate::ipc::IpcHandle(handle.0);
    // Note: the caller (resolve_handle) already holds the Process lock which
    // protects the ipc_table.
    lookup(ipc_table, ipc_handle).cloned().ok_or(Errno::EBADF)
}

fn lookup_ipc_entry_any(
    ipc_table: &crate::ipc::IpcHandleTable,
    handle: Handle,
) -> SysResult<crate::ipc::IpcHandleEntry> {
    lookup_ipc_entry_with(ipc_table, handle, |table, ipc_handle| table.get_any(ipc_handle))
}

fn lookup_ipc_entry_write(
    ipc_table: &crate::ipc::IpcHandleTable,
    handle: Handle,
) -> SysResult<crate::ipc::IpcHandleEntry> {
    lookup_ipc_entry_with(ipc_table, handle, |table, ipc_handle| {
        table.get(ipc_handle, crate::ipc::IpcHandleMode::Write)
    })
}

/// Resolve a handle by checking the process-owned open table first, then the
/// IPC handle table compatibility path.
pub fn resolve_handle(
    pinfo_arc: &Arc<Mutex<crate::task::ProcessInfo>>,
    handle: Handle,
) -> SysResult<ResolvedHandle> {
    let lock = pinfo_arc.lock();
    if let Ok(file) = lock.thing_table.get(handle.0) {
        let node = file.node.clone();
        return Ok(ResolvedHandle::FileLike {
            kind: classify_file_like(&node),
            node,
        });
    }

    let entry = lookup_ipc_entry_any(&lock.ipc_table, handle)?;
    Ok(ResolvedHandle::Port {
        port: entry.port.clone(),
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
        ResolvedHandle::Port { port, mode } => {
            Ok(Arc::new(crate::vfs::port_node::PortNode::new(port, mode)))
        }
    }
}

/// Explicit fd compatibility boundary: project an IPC port handle into a
/// process `thing_table` slot so fd-based APIs can consume it.
pub fn install_fd_compat_for_port_handle(
    pinfo_arc: &Arc<Mutex<crate::task::ProcessInfo>>,
    handle: Handle,
) -> SysResult<u32> {
    let mut lock = pinfo_arc.lock();
    let entry = lookup_ipc_entry_any(&lock.ipc_table, handle)?;
    let node = Arc::new(crate::vfs::port_node::PortNode::new(entry.port.clone(), entry.mode));

    let fd = lock.thing_table.open(
        node.clone(),
        match entry.mode {
            crate::ipc::IpcHandleMode::Read => crate::vfs::OpenFlags::read_only(),
            crate::ipc::IpcHandleMode::Write => crate::vfs::OpenFlags::write_only(),
        },
        alloc::format!("handle:{}", handle.0),
    )?;
    crate::kinfo!("BRIDGE: handle={} -> fd={} node={:p} port={:p}", handle.0, fd, Arc::as_ptr(&node), Arc::as_ptr(&entry.port));
    Ok(fd)
}

/// Resolve a write-capable port endpoint from a compat token (fd or handle).
///
/// Used where migration still accepts either token shape at syscall boundaries.
pub fn resolve_write_port_compat(
    pinfo_arc: &Arc<Mutex<crate::task::ProcessInfo>>,
    handle: Handle,
) -> SysResult<Arc<crate::ipc::Port>> {
    let lock = pinfo_arc.lock();
    if let Ok(file) = lock.thing_table.get(handle.0) {
        if let Some(port) = file.node.as_port() {
            return Ok(port);
        }
    }

    let entry = lookup_ipc_entry_write(&lock.ipc_table, handle)?;
    Ok(entry.port.clone())
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

    fn make_test_process_with_thing_table(
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
            ipc_table: IpcHandleTable::new(),
            namespace: crate::vfs::NamespaceRef::global(),
            cwd: alloc::string::String::from("/"),
            root: alloc::string::String::from("/"),
            exec_path: alloc::string::String::new(),
            authority: crate::task::ProcessAuthority::root(),
            space: crate::task::ProcessAddressSpace::empty(),
        }))
    }

    #[test]
    fn test_resolve_handle_prefers_process_table() {
        let port_id = crate::ipc::create_port(8);
        let port = crate::ipc::get_port(port_id).unwrap();

        let node: Arc<dyn VfsNode> = Arc::new(NullNode);
        let pinfo = make_test_process_with_thing_table(&[(0, node)]); // Use a placeholder FD
        let handle = {
            let mut lock = pinfo.lock();
            lock.ipc_table
                .alloc(port, crate::ipc::IpcHandleMode::Read)
                .expect("allocate handle")
        };

        // Re-create pinfo with the handle in thing_table to test preference
        let node: Arc<dyn VfsNode> = Arc::new(NullNode);
        let pinfo = make_test_process_with_thing_table(&[(handle.0, node)]);
        // The handle must also be in the ipc_table of THIS pinfo for the fallback to work if preference fails
        // But here we want to test PREFERENCE, so we put it in both.
        {
            let port_id2 = crate::ipc::create_port(8);
            let port2 = crate::ipc::get_port(port_id2).unwrap();
            pinfo.lock().ipc_table.insert_at(handle.0 as usize, crate::ipc::IpcHandleEntry::new(port2, crate::ipc::IpcHandleMode::Read)).unwrap();
        }
        let resolved = resolve_handle(&pinfo, Handle(handle.0)).expect("resolve");
        assert_eq!(resolved.kind(), HandleKind::File);

        pinfo.lock().ipc_table.close(handle);
        crate::ipc::close_port(port_id);
    }

    #[test]
    fn test_install_fd_compat_for_port_handle_creates_port_backed_fd() {
        let port_id = crate::ipc::create_port(8);
        let port = crate::ipc::get_port(port_id).unwrap();
        let pinfo = make_test_process_with_thing_table(&[]);
        let handle = {
            let mut lock = pinfo.lock();
            lock.ipc_table
                .alloc(port, crate::ipc::IpcHandleMode::Read)
                .expect("allocate handle")
        };

        let fd = install_fd_compat_for_port_handle(&pinfo, Handle(handle.0)).expect("fd");

        let lock = pinfo.lock();
        let open = lock.thing_table.get(fd).expect("fd entry");
        assert!(open.node.as_port().is_some(), "fd must wrap a port");

        drop(lock);
        pinfo.lock().ipc_table.close(handle);
        crate::ipc::close_port(port_id);
    }
}
