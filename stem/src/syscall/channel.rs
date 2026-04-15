//! Channel IPC syscall wrappers for userspace

use crate::syscall::arch::raw_syscall6;
use abi::errors::Errno;
use abi::syscall::*;

/// A handle to a channel endpoint for IPC.
pub type ChannelHandle = u32;

/// Create a new channel pair (returns packed read/write handles).
/// Result: (write_handle << 16) | read_handle
pub fn channel_create(capacity: usize) -> Result<(ChannelHandle, ChannelHandle), Errno> {
    let ret = unsafe { raw_syscall6(SYS_CHANNEL_CREATE, capacity, 0, 0, 0, 0, 0) };
    let val = abi::errors::errno(ret)?;
    let write_handle = ((val >> 16) & 0xFFFF) as ChannelHandle;
    let read_handle = (val & 0xFFFF) as ChannelHandle;
    Ok((write_handle, read_handle))
}

/// Create a new channel pair and immediately expose both ends as file descriptors.
///
/// This is the preferred FD-first entry point for new code.  The returned
/// `(write_fd, read_fd)` can be used directly with `vfs_write`, `vfs_read`,
/// `vfs_poll`, and `vfs_close` without ever touching the underlying handles.
pub fn channel_create_fds(capacity: usize) -> Result<(u32, u32), Errno> {
    let (write_handle, read_handle) = channel_create(capacity)?;
    let write_fd = super::vfs::vfs_fd_from_handle(write_handle)?;
    let read_fd = super::vfs::vfs_fd_from_handle(read_handle)?;
    Ok((write_fd, read_fd))
}

pub fn channel_send(handle: ChannelHandle, data: &[u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_CHANNEL_SEND,
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

pub fn channel_send_all(handle: ChannelHandle, data: &[u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_CHANNEL_SEND_ALL,
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

pub fn channel_recv(handle: ChannelHandle, buf: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_CHANNEL_RECV,
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

pub fn channel_try_recv(handle: ChannelHandle, buf: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_CHANNEL_TRY_RECV,
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

pub fn channel_close(handle: ChannelHandle) -> Result<(), Errno> {
    let ret = unsafe { raw_syscall6(SYS_CHANNEL_CLOSE, handle as usize, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|_| ())
}

pub fn channel_len(handle: ChannelHandle) -> Result<usize, Errno> {
    let ret = unsafe { raw_syscall6(SYS_CHANNEL_INFO, handle as usize, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|v| (v & 0xFFFFFFFF) as usize)
}

pub fn channel_capacity(handle: ChannelHandle) -> Result<usize, Errno> {
    let ret = unsafe { raw_syscall6(SYS_CHANNEL_INFO, handle as usize, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|v| (v >> 32) as usize)
}
