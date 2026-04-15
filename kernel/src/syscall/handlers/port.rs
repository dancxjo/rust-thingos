//! Port IPC syscalls

use super::{copyin, copyout};
use crate::syscall::validate::validate_user_range;
use abi::errors::{Errno, SysResult};
use alloc::sync::Arc;

// Lines 7-9 are duplicates of 3-5

pub fn sys_channel_create(capacity: usize) -> SysResult<usize> {
    let capacity = capacity.min(65536).max(64);
    let port_id = crate::ipc::create_port(capacity);

    let mut table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
    let write_handle = table
        .alloc(port_id, crate::ipc::HandleMode::Write)
        .ok_or(Errno::ENOMEM)?;
    let read_handle = table
        .alloc(port_id, crate::ipc::HandleMode::Read)
        .ok_or(Errno::ENOMEM)?;

    let packed = ((write_handle.0 as usize) << 16) | (read_handle.0 as usize);
    Ok(packed)
}

pub fn sys_channel_send(handle: usize, ptr: usize, len: usize) -> SysResult<usize> {
    let len = len.min(4096);
    if len == 0 {
        return Ok(0);
    }

    validate_user_range(ptr, len, false)?;

    let handle = crate::ipc::Handle(handle as u32);
    let entry = {
        let table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
        table
            .get(handle, crate::ipc::HandleMode::Write)
            .copied()
            .ok_or(Errno::EBADF)?
    };

    let port = crate::ipc::get_port(entry.port_id).ok_or(Errno::EBADF)?;
    if !port.has_readers() {
        crate::ipc::diag::CHANNEL_PEER_DEATHS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        return Err(Errno::EPIPE);
    }

    let mut buf = [0u8; 4096];
    unsafe {
        copyin(&mut buf[..len], ptr)?;
    }

    let written = port.send(&buf[..len]);
    if written > 0 {
        crate::ipc::diag::CHANNEL_SENDS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        crate::ipc::diag::CHANNEL_BYTES_SENT
            .fetch_add(written as u64, core::sync::atomic::Ordering::Relaxed);
    }
    Ok(written)
}

pub fn sys_channel_send_all(handle: usize, ptr: usize, len: usize) -> SysResult<usize> {
    let len = len.min(4096);
    if len == 0 {
        return Ok(0);
    }

    validate_user_range(ptr, len, false)?;

    let handle = crate::ipc::Handle(handle as u32);
    let entry = {
        let table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
        table
            .get(handle, crate::ipc::HandleMode::Write)
            .copied()
            .ok_or(Errno::EBADF)?
    };

    let port = crate::ipc::get_port(entry.port_id).ok_or(Errno::EBADF)?;
    if !port.has_readers() {
        crate::ipc::diag::CHANNEL_PEER_DEATHS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        return Err(Errno::EPIPE);
    }

    let mut buf = [0u8; 4096];
    unsafe {
        copyin(&mut buf[..len], ptr)?;
    }

    if port.send_all(&buf[..len]) {
        crate::ktrace!(
            "sys_channel_send_all: wrote {} bytes to port {}",
            len,
            entry.port_id.0
        );
        crate::ipc::diag::CHANNEL_SENDS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        crate::ipc::diag::CHANNEL_BYTES_SENT
            .fetch_add(len as u64, core::sync::atomic::Ordering::Relaxed);
        Ok(len)
    } else {
        crate::ktrace!(
            "sys_channel_send_all: port {} FULL, returning EAGAIN",
            entry.port_id.0
        );
        crate::ipc::diag::CHANNEL_FULL_EVENTS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        Err(Errno::EAGAIN)
    }
}

fn sys_channel_recv_impl(
    handle: usize,
    ptr: usize,
    len: usize,
    blocking: bool,
) -> SysResult<usize> {
    let len = len.min(4096);
    if len == 0 {
        return Ok(0);
    }

    validate_user_range(ptr, len, true)?;

    let handle = crate::ipc::Handle(handle as u32);
    let entry = {
        let table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
        table
            .get(handle, crate::ipc::HandleMode::Read)
            .copied()
            .ok_or(Errno::EBADF)?
    };

    let port = crate::ipc::get_port(entry.port_id).ok_or(Errno::EBADF)?;
    let mut buf = [0u8; 4096];
    let tid = unsafe { crate::sched::current_tid_current() };

    loop {
        let read = port.try_recv(&mut buf[..len]);
        if read > 0 {
            unsafe {
                copyout(ptr, &buf[..read])?;
            }
            crate::ktrace!(
                "sys_channel_recv: read {} bytes from port {}",
                read,
                entry.port_id.0
            );
            crate::ipc::diag::CHANNEL_RECVS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
            crate::ipc::diag::CHANNEL_BYTES_RECV
                .fetch_add(read as u64, core::sync::atomic::Ordering::Relaxed);
            return Ok(read);
        }

        if !port.has_writers() {
            crate::ipc::diag::CHANNEL_PEER_DEATHS
                .fetch_add(1, core::sync::atomic::Ordering::Relaxed);
            return Err(Errno::EPIPE);
        }

        if !blocking {
            return Err(Errno::EAGAIN);
        }

        port.add_waiter_read(tid);

        let read = port.try_recv(&mut buf[..len]);
        if read > 0 {
            port.remove_waiter_read(tid);
            unsafe {
                copyout(ptr, &buf[..read])?;
            }
            crate::ktrace!(
                "sys_channel_recv: read {} bytes from port {} after wait registration",
                read,
                entry.port_id.0
            );
            crate::ipc::diag::CHANNEL_RECVS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
            crate::ipc::diag::CHANNEL_BYTES_RECV
                .fetch_add(read as u64, core::sync::atomic::Ordering::Relaxed);
            return Ok(read);
        }

        if !port.has_writers() {
            port.remove_waiter_read(tid);
            crate::ipc::diag::CHANNEL_PEER_DEATHS
                .fetch_add(1, core::sync::atomic::Ordering::Relaxed);
            return Err(Errno::EPIPE);
        }

        unsafe {
            crate::sched::block_current_erased();
        }
    }
}

pub fn sys_channel_recv(handle: usize, ptr: usize, len: usize) -> SysResult<usize> {
    sys_channel_recv_impl(handle, ptr, len, true)
}

pub fn sys_channel_try_recv(handle: usize, ptr: usize, len: usize) -> SysResult<usize> {
    sys_channel_recv_impl(handle, ptr, len, false)
}

pub fn sys_channel_close(handle: usize) -> SysResult<usize> {
    let handle = crate::ipc::Handle(handle as u32);
    let mut table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
    if let Some(entry) = table.close(handle) {
        drop(table);

        let port = crate::ipc::get_port(entry.port_id).ok_or(Errno::EBADF)?;
        let destroy = match entry.mode {
            crate::ipc::HandleMode::Read => port.close_reader(),
            crate::ipc::HandleMode::Write => port.close_writer(),
        };
        if destroy {
            crate::ipc::close_port(entry.port_id);
        }
        Ok(0)
    } else {
        Err(Errno::EBADF)
    }
}

pub fn sys_channel_info(handle: usize) -> SysResult<usize> {
    let handle = crate::ipc::Handle(handle as u32);
    let table = crate::ipc::GLOBAL_HANDLE_TABLE.lock();
    let entry = table
        .get(handle, crate::ipc::HandleMode::Read)
        .or_else(|| table.get(handle, crate::ipc::HandleMode::Write))
        .ok_or(Errno::EBADF)?;

    let port = crate::ipc::get_port(entry.port_id).ok_or(Errno::EBADF)?;

    let len = port.len();
    let cap = port.capacity(); // Need to expose capacity

    // Return packed: top 32 bits capacity, bottom 32 bits length
    Ok((cap << 32) | (len & 0xFFFFFFFF))
}
