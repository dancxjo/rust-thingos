//! Unix domain socket syscall handlers.
//!
//! Implements:
//! - [`sys_socket`]    — create a new unbound socket fd
//! - [`sys_bind`]      — bind the socket to a filesystem path
//! - [`sys_listen`]    — mark the socket as listening
//! - [`sys_accept`]    — block and accept one incoming connection
//! - [`sys_connect`]   — connect to a listening socket by path
//! - [`sys_shutdown`]  — shut down one or both directions of a connection
//! - [`sys_socketpair`]— create an anonymous connected socket pair

use alloc::sync::Arc;
use alloc::vec;

use abi::errors::{Errno, SysResult};
use abi::syscall::{socket_domain, socket_type};

use crate::syscall::validate::{copyin, copyout, validate_user_range};
use crate::vfs::OpenFlags;

// ---------------------------------------------------------------------------
// Helper: copy a path string from userspace
// ---------------------------------------------------------------------------

fn copy_path(path_ptr: usize, path_len: usize) -> SysResult<alloc::string::String> {
    if path_len == 0 || path_len > 4096 {
        return Err(Errno::EINVAL);
    }
    validate_user_range(path_ptr, path_len, false)?;
    let mut buf = vec![0u8; path_len];
    unsafe { copyin(&mut buf, path_ptr)? };
    alloc::string::String::from_utf8(buf).map_err(|_| Errno::EINVAL)
}

// ---------------------------------------------------------------------------
// sys_socket
// ---------------------------------------------------------------------------

/// Create a new Unix domain socket and return a file descriptor.
///
/// Only `AF_UNIX + SOCK_STREAM + protocol=0` is currently supported.
/// Returns `EAFNOSUPPORT` for unknown domains, `EPROTOTYPE` for unknown types.
pub fn sys_socket(domain: usize, type_: usize, _protocol: usize) -> SysResult<usize> {
    if domain as u32 != socket_domain::AF_UNIX {
        return Err(Errno::EAFNOSUPPORT);
    }
    if type_ as u32 != socket_type::SOCK_STREAM {
        return Err(Errno::EPROTOTYPE);
    }

    let node = crate::ipc::unix_socket::UnixSocketNode::new();
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let thing = pinfo_arc.lock().handle_table.open(
        node as Arc<dyn crate::vfs::VfsNode>,
        OpenFlags::read_write(),
        alloc::string::String::from("socket:[unix]"),
    )?;
    Ok(thing as usize)
}

// ---------------------------------------------------------------------------
// sys_bind
// ---------------------------------------------------------------------------

/// Bind a socket fd to a filesystem path.
///
/// Args: `(fd, path_ptr, path_len)`
pub fn sys_bind(thing: usize, path_ptr: usize, path_len: usize) -> SysResult<usize> {
    let path = copy_path(path_ptr, path_len)?;

    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let node = {
        let lock = pinfo_arc.lock();
        lock.handle_table.get(thing as u32)?.node.clone()
    };
    node.sock_bind(&path)?;
    Ok(0)
}

// ---------------------------------------------------------------------------
// sys_listen
// ---------------------------------------------------------------------------

/// Mark a socket as listening.
///
/// Args: `(fd, backlog)`
pub fn sys_listen(thing: usize, backlog: usize) -> SysResult<usize> {
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let node = {
        let lock = pinfo_arc.lock();
        lock.handle_table.get(thing as u32)?.node.clone()
    };
    node.sock_listen(backlog)?;
    Ok(0)
}

// ---------------------------------------------------------------------------
// sys_accept
// ---------------------------------------------------------------------------

/// Accept one incoming connection on a listening socket.
///
/// Args: `(fd)`
/// Returns the new file descriptor for the accepted connection.
pub fn sys_accept(thing: usize) -> SysResult<usize> {
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let node = {
        let lock = pinfo_arc.lock();
        lock.handle_table.get(thing as u32)?.node.clone()
    };
    let accepted = node.sock_accept()?;
    let new_handle = pinfo_arc.lock().handle_table.open(
        accepted,
        OpenFlags::read_write(),
        alloc::string::String::from("socket:[unix/accepted]"),
    )?;
    Ok(new_handle as usize)
}

// ---------------------------------------------------------------------------
// sys_connect
// ---------------------------------------------------------------------------

/// Connect a socket to a listening socket at the given path.
///
/// Args: `(fd, path_ptr, path_len)`
pub fn sys_connect(thing: usize, path_ptr: usize, path_len: usize) -> SysResult<usize> {
    let path = copy_path(path_ptr, path_len)?;

    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let node = {
        let lock = pinfo_arc.lock();
        lock.handle_table.get(thing as u32)?.node.clone()
    };
    node.sock_connect(&path)?;
    Ok(0)
}

// ---------------------------------------------------------------------------
// sys_shutdown
// ---------------------------------------------------------------------------

/// Shut down part or all of a socket connection.
///
/// Args: `(fd, how)` — how: 0=SHUT_RD, 1=SHUT_WR, 2=SHUT_RDWR
pub fn sys_shutdown(thing: usize, how: usize) -> SysResult<usize> {
    if how > 2 {
        return Err(Errno::EINVAL);
    }
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let node = {
        let lock = pinfo_arc.lock();
        lock.handle_table.get(thing as u32)?.node.clone()
    };
    node.sock_shutdown(how as u32)?;
    Ok(0)
}

// ---------------------------------------------------------------------------
// sys_socketpair
// ---------------------------------------------------------------------------

/// Create a connected pair of sockets (anonymous, no filesystem binding).
///
/// Args: `(domain, type, protocol, fds_ptr)`
/// `fds_ptr` must point to a `[u32; 2]` writable user buffer.
/// On success writes `[fd_a, fd_b]` and returns 0.
pub fn sys_socketpair(
    domain: usize,
    type_: usize,
    _protocol: usize,
    fds_ptr: usize,
) -> SysResult<usize> {
    if domain as u32 != socket_domain::AF_UNIX {
        return Err(Errno::EAFNOSUPPORT);
    }
    if type_ as u32 != socket_type::SOCK_STREAM {
        return Err(Errno::EPROTOTYPE);
    }
    validate_user_range(fds_ptr, 8, true)?;

    let (a, b) = crate::ipc::unix_socket::UnixSocketNode::new_pair();

    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let (fd_a, fd_b) = {
        let mut lock = pinfo_arc.lock();
        let fa = lock.handle_table.open(
            a as Arc<dyn crate::vfs::VfsNode>,
            OpenFlags::read_write(),
            alloc::string::String::from("socketpair:[unix/a]"),
        )?;
        match lock.handle_table.open(
            b as Arc<dyn crate::vfs::VfsNode>,
            OpenFlags::read_write(),
            alloc::string::String::from("socketpair:[unix/b]"),
        ) {
            Ok(fb) => (fa, fb),
            Err(e) => {
                let _ = lock.handle_table.close(fa);
                return Err(e);
            }
        }
    };

    // Write [fd_a, fd_b] to userspace as two consecutive u32 values (8 bytes,
    // little-endian to keep ABI stable across host endianness).
    let mut fds_bytes = [0u8; 8];
    fds_bytes[..4].copy_from_slice(&fd_a.to_le_bytes());
    fds_bytes[4..].copy_from_slice(&fd_b.to_le_bytes());
    unsafe { copyout(fds_ptr, &fds_bytes)? };
    Ok(0)
}

// ---------------------------------------------------------------------------
// sys_sendmsg
// ---------------------------------------------------------------------------

/// Send data + zero or more FDs atomically over a socket or port FD.
///
/// Syscall args:
///   0: fd (socket or PortNode FD — must be writable)
///   1: data pointer (may be null when data_len == 0)
///   2: data length (capped at 4096)
///   3: fds pointer — array of `u32` fd/handle numbers (may be null when count == 0)
///   4: fds count (capped at 64)
///   5: (reserved, must be 0)
///
/// The fd/handle numbers in the `fds` array are resolved from the caller's
/// FD table first, then from the global IPC handle table.  Transfer
/// semantics are **duplicate**: the caller retains its own fd/handle.
pub fn sys_sendmsg(
    thing: usize,
    data_ptr: usize,
    data_len: usize,
    fds_ptr: usize,
    fds_count: usize,
) -> SysResult<usize> {
    const MAX_MSG_DATA: usize = 4096;
    const MAX_MSG_FDS: usize = 64;

    if fds_count > MAX_MSG_FDS {
        return Err(Errno::EINVAL);
    }
    let data_len = data_len.min(MAX_MSG_DATA);

    if data_len > 0 {
        validate_user_range(data_ptr, data_len, false)?;
    }
    if fds_count > 0 {
        validate_user_range(fds_ptr, fds_count * 4, false)?;
    }

    // Read payload bytes.
    let mut data_buf = alloc::vec![0u8; data_len];
    if data_len > 0 {
        unsafe { copyin(&mut data_buf, data_ptr)? };
    }

    // Read fd/handle numbers and resolve each to a VfsNode.
    let mut fd_nums = alloc::vec![0u32; fds_count];
    if fds_count > 0 {
        unsafe {
            copyin(
                core::slice::from_raw_parts_mut(fd_nums.as_mut_ptr() as *mut u8, fds_count * 4),
                fds_ptr,
            )?;
        }
    }
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let mut caps = alloc::vec::Vec::with_capacity(fds_count);
    for &raw in &fd_nums {
        let node = resolve_fd_or_handle(&pinfo_arc, raw)?;
        caps.push(node);
    }

    // Look up the destination FD and call sock_sendmsg.
    let node = {
        let lock = pinfo_arc.lock();
        lock.handle_table.get(thing as u32)?.node.clone()
    };

    let caps_len = caps.len();
    crate::ktrace!(
        "SENDMSG: thing={} data_len={} caps_len={} node={:p}",
        thing,
        data_len,
        caps_len,
        Arc::as_ptr(&node)
    );

    node.sock_sendmsg(&data_buf, caps)?;
    Ok(0)
}

// ---------------------------------------------------------------------------
// sys_recvmsg
// ---------------------------------------------------------------------------

/// Receive one message (data bytes + FDs) from a socket or port FD.
///
/// Syscall args:
///   0: fd (socket or PortNode FD — must be readable)
///   1: data buffer pointer
///   2: data buffer capacity
///   3: fds buffer pointer — array of `u32` (output FD numbers written here)
///   4: fds buffer capacity (max FDs to install)
///   5: out_lens pointer — points to `[usize; 2]` filled with
///        `[actual_data_len, actual_fds_count]`
///
/// Returns `EAGAIN` when no message is available.
pub fn sys_recvmsg(
    thing: usize,
    data_ptr: usize,
    data_cap: usize,
    fds_ptr: usize,
    fds_cap: usize,
    out_lens_ptr: usize,
) -> SysResult<usize> {
    validate_user_range(out_lens_ptr, core::mem::size_of::<usize>() * 2, true)?;
    if data_cap > 0 {
        validate_user_range(data_ptr, data_cap, true)?;
    }
    if fds_cap > 0 {
        validate_user_range(fds_ptr, fds_cap * 4, true)?;
    }

    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let node = {
        let lock = pinfo_arc.lock();
        lock.handle_table.get(thing as u32)?.node.clone()
    };

    let msg = node.sock_recvmsg()?.ok_or(Errno::EAGAIN)?;
    let (data, fds) = msg;
    crate::ktrace!(
        "RECVMSG: thing={} data_len={} caps_len={} node={:p}",
        thing,
        data.len(),
        fds.len(),
        Arc::as_ptr(&node)
    );

    // Copy data bytes to userspace (truncate if necessary).
    let copy_len = data.len().min(data_cap);
    if copy_len > 0 {
        unsafe { copyout(data_ptr, &data[..copy_len])? };
    }

    // Install attached capabilities as FDs in the receiving process.
    let install_count = fds.len().min(fds_cap);
    let mut out_fds = alloc::vec![0u32; install_count];
    {
        let mut pinfo = pinfo_arc.lock();
        for (i, cap) in fds.into_iter().take(install_count).enumerate() {
            cap.on_dup();
            let new_handle = pinfo.handle_table.open(
                cap,
                crate::vfs::OpenFlags::read_write(),
                "recvmsg".into(),
            )?;
            out_fds[i] = new_handle;
        }
    }

    // Write installed FD numbers to userspace.
    if install_count > 0 {
        unsafe {
            copyout(
                fds_ptr,
                core::slice::from_raw_parts(out_fds.as_ptr() as *const u8, install_count * 4),
            )?;
        }
    }

    // Write [actual_data_len, actual_fds_count] to out_lens_ptr.
    let out_lens: [usize; 2] = [copy_len, install_count];
    unsafe {
        copyout(
            out_lens_ptr,
            core::slice::from_raw_parts(
                out_lens.as_ptr() as *const u8,
                core::mem::size_of::<usize>() * 2,
            ),
        )?;
    }

    Ok(0)
}

// ---------------------------------------------------------------------------
// Private helper
// ---------------------------------------------------------------------------

/// Resolve a raw `u32` to a `VfsNode` by checking the calling process's FD
/// table first, then the global IPC handle table.
fn resolve_fd_or_handle(
    pinfo_arc: &alloc::sync::Arc<spin::Mutex<crate::task::ProcessInfo>>,
    raw: u32,
) -> SysResult<alloc::sync::Arc<dyn crate::vfs::VfsNode>> {
    crate::handle::bridge::resolve_io_node_compat(pinfo_arc, crate::handle::bridge::Handle(raw))
}
