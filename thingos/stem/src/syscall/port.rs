//! Port IPC syscall wrappers for userspace

use crate::syscall::arch::raw_syscall6;
use abi::errors::Errno;
use abi::syscall::*;

/// A handle referring to a port endpoint for IPC.
pub type PortHandle = u32;

/// Create a new port pair (returns packed read/write handles).
/// Result: (write_handle << 16) | read_handle
pub fn port_create(capacity: usize) -> Result<(PortHandle, PortHandle), Errno> {
    let ret = unsafe { raw_syscall6(SYS_PORT_CREATE, capacity, 0, 0, 0, 0, 0) };
    let val = abi::errors::errno(ret)?;
    let write_handle = ((val >> 16) & 0xFFFF) as PortHandle;
    let read_handle = (val & 0xFFFF) as PortHandle;
    Ok((write_handle, read_handle))
}

/// Create a new port pair and immediately expose both ends as fds.
///
/// This is the preferred FD-first entry point for new code.  The returned
/// `(write_fd, read_fd)` can be used directly with `vfs_write`, `vfs_read`,
/// `vfs_poll`, and `vfs_close` without ever touching the underlying handles.
pub fn port_create_fds(capacity: usize) -> Result<(u32, u32), Errno> {
    let (write_handle, read_handle) = port_create(capacity)?;
    let write_fd = super::vfs::vfs_handle_from_port(write_handle)?;
    let read_fd = super::vfs::vfs_handle_from_port(read_handle)?;
    Ok((write_fd, read_fd))
}

pub fn port_send(handle: PortHandle, data: &[u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_PORT_SEND,
            handle as usize,
            data.as_ptr() as usize,
            data.len(),
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret)
}

pub fn port_send_all(handle: PortHandle, data: &[u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_PORT_SEND_ALL,
            handle as usize,
            data.as_ptr() as usize,
            data.len(),
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret)
}

pub fn port_recv(handle: PortHandle, buf: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_PORT_RECV,
            handle as usize,
            buf.as_mut_ptr() as usize,
            buf.len(),
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret)
}

pub fn port_try_recv(handle: PortHandle, buf: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_PORT_TRY_RECV,
            handle as usize,
            buf.as_mut_ptr() as usize,
            buf.len(),
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret)
}

pub fn port_close(handle: PortHandle) -> Result<(), Errno> {
    let ret = unsafe { raw_syscall6(SYS_PORT_CLOSE, handle as usize, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|_| ())
}

pub fn port_len(handle: PortHandle) -> Result<usize, Errno> {
    let ret = unsafe { raw_syscall6(SYS_PORT_INFO, handle as usize, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|v| (v & 0xFFFFFFFF) as usize)
}

pub fn port_capacity(handle: PortHandle) -> Result<usize, Errno> {
    let ret = unsafe { raw_syscall6(SYS_PORT_INFO, handle as usize, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|v| (v >> 32) as usize)
}
