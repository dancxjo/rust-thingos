//! Channel IPC syscall wrappers for userspace

use crate::syscall::arch::raw_syscall6;
use abi::errors::Errno;
use abi::syscall::*;

/// A thing referring to a channel endpoint for IPC.
pub type ChannelThing = u32;

/// Create a new channel pair (returns packed read/write handles).
/// Result: (write_thing << 16) | read_thing
pub fn channel_create(capacity: usize) -> Result<(ChannelThing, ChannelThing), Errno> {
    let ret = unsafe { raw_syscall6(SYS_CHANNEL_CREATE, capacity, 0, 0, 0, 0, 0) };
    let val = abi::errors::errno(ret)?;
    let write_thing = ((val >> 16) & 0xFFFF) as ChannelThing;
    let read_thing = (val & 0xFFFF) as ChannelThing;
    Ok((write_thing, read_thing))
}

/// Create a new channel pair and immediately expose both ends as things.
///
/// This is the preferred FD-first entry point for new code.  The returned
/// `(write_thing_fd, read_thing_fd)` can be used directly with `vfs_write`, `vfs_read`,
/// `vfs_poll`, and `vfs_close` without ever touching the underlying handles.
pub fn channel_create_fds(capacity: usize) -> Result<(u32, u32), Errno> {
    let (write_thing, read_thing) = channel_create(capacity)?;
    let write_thing_fd = super::vfs::vfs_thing_from_channel(write_thing)?;
    let read_thing_fd = super::vfs::vfs_thing_from_channel(read_thing)?;
    Ok((write_thing_fd, read_thing_fd))
}

pub fn channel_send(thing: ChannelThing, data: &[u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_CHANNEL_SEND,
            thing as usize,
            data.as_ptr() as usize,
            data.len(),
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret)
}

pub fn channel_send_all(thing: ChannelThing, data: &[u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_CHANNEL_SEND_ALL,
            thing as usize,
            data.as_ptr() as usize,
            data.len(),
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret)
}

pub fn channel_recv(thing: ChannelThing, buf: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_CHANNEL_RECV,
            thing as usize,
            buf.as_mut_ptr() as usize,
            buf.len(),
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret)
}

pub fn channel_try_recv(thing: ChannelThing, buf: &mut [u8]) -> Result<usize, Errno> {
    let ret = unsafe {
        raw_syscall6(
            SYS_CHANNEL_TRY_RECV,
            thing as usize,
            buf.as_mut_ptr() as usize,
            buf.len(),
            0,
            0,
            0,
        )
    };
    abi::errors::errno(ret)
}

pub fn channel_close(thing: ChannelThing) -> Result<(), Errno> {
    let ret = unsafe { raw_syscall6(SYS_CHANNEL_CLOSE, thing as usize, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|_| ())
}

pub fn channel_len(thing: ChannelThing) -> Result<usize, Errno> {
    let ret = unsafe { raw_syscall6(SYS_CHANNEL_INFO, thing as usize, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|v| (v & 0xFFFFFFFF) as usize)
}

pub fn channel_capacity(thing: ChannelThing) -> Result<usize, Errno> {
    let ret = unsafe { raw_syscall6(SYS_CHANNEL_INFO, thing as usize, 0, 0, 0, 0, 0) };
    abi::errors::errno(ret).map(|v| (v >> 32) as usize)
}
