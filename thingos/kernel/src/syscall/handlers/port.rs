//! Port IPC syscalls

use abi::errors::{Errno, SysResult};

use super::{copyin, copyout};
use crate::syscall::validate::validate_user_range;

// Lines 7-9 are duplicates of 3-5

pub fn sys_port_create(capacity: usize) -> SysResult<usize> {
    let capacity = capacity.min(65536).max(64);
    let port_id = crate::ipc::create_port(capacity);
    let port = crate::ipc::get_port(port_id).ok_or(Errno::ENOMEM)?;

    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let pid = {
        let lock = pinfo_arc.lock();
        lock.pid
    };
    port.set_primary_reader_pid(pid as u64);

    let mut table = pinfo_arc.lock();
    let write_handle = table
        .ipc_table
        .alloc(port.clone(), crate::ipc::IpcHandleMode::Write)
        .ok_or(Errno::ENOMEM)?;
    let read_handle =
        table.ipc_table.alloc(port, crate::ipc::IpcHandleMode::Read).ok_or(Errno::ENOMEM)?;

    let packed = ((write_handle.0 as usize) << 16) | (read_handle.0 as usize);
    Ok(packed)
}

pub fn sys_port_send(handle: usize, ptr: usize, len: usize) -> SysResult<usize> {
    let len = len.min(65536);
    if len == 0 {
        return Ok(0);
    }

    validate_user_range(ptr, len, false)?;

    let handle = crate::ipc::IpcHandle(handle as u32);
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let entry = {
        let lock = pinfo_arc.lock();
        lock.ipc_table.get(handle, crate::ipc::IpcHandleMode::Write).cloned().ok_or(Errno::EBADF)?
    };

    let port = entry.port.clone();
    if !port.has_readers() {
        crate::ipc::diag::PORT_PEER_DEATHS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        return Err(Errno::EPIPE);
    }

    let mut buf = alloc::vec![0u8; len];
    unsafe {
        copyin(&mut buf[..len], ptr)?;
    }

    let written = port.send(&buf[..len]);
    if written > 0 {
        crate::ipc::diag::PORT_SENDS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        crate::ipc::diag::PORT_BYTES_SENT
            .fetch_add(written as u64, core::sync::atomic::Ordering::Relaxed);
    }
    Ok(written)
}

pub fn sys_port_send_all(handle: usize, ptr: usize, len: usize) -> SysResult<usize> {
    let len = len.min(65536);
    if len == 0 {
        return Ok(0);
    }

    validate_user_range(ptr, len, false)?;
    let handle = crate::ipc::IpcHandle(handle as u32);
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let entry = {
        let lock = pinfo_arc.lock();
        lock.ipc_table.get(handle, crate::ipc::IpcHandleMode::Write).cloned().ok_or(Errno::EBADF)?
    };

    let port = entry.port.clone();
    if !port.has_readers() {
        crate::ipc::diag::PORT_PEER_DEATHS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        return Err(Errno::EPIPE);
    }

    let mut buf = alloc::vec![0u8; len];
    unsafe {
        copyin(&mut buf[..len], ptr)?;
    }

    if port.send_all(&buf[..len]) {
        let port_id = crate::ipc::find_port_id(&port).unwrap_or(crate::ipc::PortId(0));
        crate::ktrace!("sys_port_send_all: wrote {} bytes to port {:?}", len, port_id);
        crate::ipc::diag::PORT_SENDS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        crate::ipc::diag::PORT_BYTES_SENT
            .fetch_add(len as u64, core::sync::atomic::Ordering::Relaxed);
        Ok(len)
    } else {
        let port_id = crate::ipc::find_port_id(&port).unwrap_or(crate::ipc::PortId(0));
        crate::ktrace!("sys_port_send_all: port {:?} FULL, returning EAGAIN", port_id);
        crate::ipc::diag::PORT_FULL_EVENTS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        Err(Errno::EAGAIN)
    }
}

fn sys_port_recv_impl(handle: usize, ptr: usize, len: usize, blocking: bool) -> SysResult<usize> {
    let len = len.min(65536);
    if len == 0 {
        return Ok(0);
    }

    validate_user_range(ptr, len, true)?;

    let handle = crate::ipc::IpcHandle(handle as u32);
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let entry = {
        let table = pinfo_arc.lock();
        table.ipc_table.get(handle, crate::ipc::IpcHandleMode::Read).cloned().ok_or(Errno::EBADF)?
    };

    let port = entry.port.clone();
    let mut buf = alloc::vec![0u8; len];
    let tid = unsafe { crate::sched::current_tid_current() };

    loop {
        let read = port.try_recv(&mut buf[..len]);
        if read > 0 {
            unsafe {
                copyout(ptr, &buf[..read])?;
            }
            let port_id = crate::ipc::find_port_id(&port).unwrap_or(crate::ipc::PortId(0));
            crate::ktrace!("sys_port_recv: read {} bytes from port {:?}", read, port_id);
            crate::ipc::diag::PORT_RECVS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
            crate::ipc::diag::PORT_BYTES_RECV
                .fetch_add(read as u64, core::sync::atomic::Ordering::Relaxed);
            return Ok(read);
        }

        if !port.has_writers() {
            crate::ipc::diag::PORT_PEER_DEATHS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
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
            let port_id = crate::ipc::find_port_id(&port).unwrap_or(crate::ipc::PortId(0));
            crate::ktrace!(
                "sys_port_recv: read {} bytes from port {:?} after wait registration",
                read,
                port_id
            );
            crate::ipc::diag::PORT_RECVS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
            crate::ipc::diag::PORT_BYTES_RECV
                .fetch_add(read as u64, core::sync::atomic::Ordering::Relaxed);
            return Ok(read);
        }

        if !port.has_writers() {
            port.remove_waiter_read(tid);
            crate::ipc::diag::PORT_PEER_DEATHS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
            return Err(Errno::EPIPE);
        }

        unsafe {
            crate::sched::block_current_erased();
        }

        if crate::sched::take_pending_interrupt_current() {
            port.remove_waiter_read(tid);
            return Err(Errno::EINTR);
        }
    }
}

pub fn sys_port_recv(handle: usize, ptr: usize, len: usize) -> SysResult<usize> {
    sys_port_recv_impl(handle, ptr, len, true)
}

pub fn sys_port_try_recv(handle: usize, ptr: usize, len: usize) -> SysResult<usize> {
    sys_port_recv_impl(handle, ptr, len, false)
}

pub fn sys_port_close(handle: usize) -> SysResult<usize> {
    let handle = crate::ipc::IpcHandle(handle as u32);
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let entry = {
        let mut table = pinfo_arc.lock();
        table.ipc_table.close(handle).ok_or(Errno::EBADF)?
    };

    if entry.port.has_readers() == false && entry.port.has_writers() == false {
        let port_id = crate::ipc::find_port_id(&entry.port).unwrap();
        crate::ipc::close_port(port_id);
    }
    Ok(0)
}

pub fn sys_port_info(handle: usize) -> SysResult<usize> {
    let handle = crate::ipc::IpcHandle(handle as u32);
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
    let lock = pinfo_arc.lock();
    let entry = lock
        .ipc_table
        .get(handle, crate::ipc::IpcHandleMode::Read)
        .or_else(|| lock.ipc_table.get(handle, crate::ipc::IpcHandleMode::Write))
        .cloned()
        .ok_or(Errno::EBADF)?;

    let port = entry.port.clone();

    let len = port.len();
    let cap = port.capacity();

    // Return packed: top 32 bits capacity, bottom 32 bits length
    Ok((cap << 32) | (len & 0xFFFFFFFF))
}
