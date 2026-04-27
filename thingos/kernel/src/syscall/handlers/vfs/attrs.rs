use super::*;

pub fn sys_fs_device_call(fd: usize, call_ptr: usize) -> SysResult<usize> {
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct RawDeviceCall {
        kind: u32,
        op: u32,
        in_ptr: u64,
        in_len: u32,
        out_ptr: u64,
        out_len: u32,
    }

    fn decode_device_call(raw: RawDeviceCall) -> SysResult<abi::device::DeviceCall> {
        let kind = match raw.kind {
            1 => abi::device::DeviceKind::RtcCmos,
            2 => abi::device::DeviceKind::Keyboard,
            3 => abi::device::DeviceKind::Mouse,
            4 => abi::device::DeviceKind::Framebuffer,
            5 => abi::device::DeviceKind::Pci,
            6 => abi::device::DeviceKind::Display,
            7 => abi::device::DeviceKind::Terminal,
            8 => abi::device::DeviceKind::Audio,
            _ => return Err(Errno::EINVAL),
        };

        Ok(abi::device::DeviceCall {
            kind,
            op: raw.op,
            in_ptr: raw.in_ptr,
            in_len: raw.in_len,
            out_ptr: raw.out_ptr,
            out_len: raw.out_len,
        })
    }

    let size = core::mem::size_of::<RawDeviceCall>();
    validate_user_range(call_ptr, size, true)?;
    let mut raw = RawDeviceCall {
        kind: abi::device::DeviceKind::Terminal as u32,
        op: 0,
        in_ptr: 0,
        in_len: 0,
        out_ptr: 0,
        out_len: 0,
    };
    let slice = unsafe { core::slice::from_raw_parts_mut(&mut raw as *mut _ as *mut u8, size) };
    unsafe {
        copyin(slice, call_ptr)?;
    }
    let call = decode_device_call(raw)?;

    let node = {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let lock = pinfo_arc.lock();
        let file = lock.handle_table.get(fd as u32)?;
        file.node.clone()
    };

    node.device_call(&call)
}

pub fn sys_fs_attr_get(
    fd: usize,
    name_ptr: usize,
    name_len: usize,
    buf_ptr: usize,
    buf_len: usize,
    type_ptr: usize,
) -> SysResult<usize> {
    validate_user_range(name_ptr, name_len, false)?;
    let mut name_buf = vec![0u8; name_len];
    unsafe { copyin(&mut name_buf, name_ptr)? };
    let name = core::str::from_utf8(&name_buf).map_err(|_| Errno::EINVAL)?;

    let node = {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let lock = pinfo_arc.lock();
        lock.handle_table.get(fd as u32)?.node.clone()
    };

    let (val_type, val) = node.attr_get(name)?;

    if type_ptr != 0 {
        validate_user_range(type_ptr, 1, true)?;
        unsafe { copyout(type_ptr, &[val_type])? };
    }

    let copy_n = val.len().min(buf_len);
    if buf_ptr != 0 && copy_n > 0 {
        validate_user_range(buf_ptr, copy_n, true)?;
        unsafe { copyout(buf_ptr, &val[..copy_n])? };
    }
    Ok(val.len())
}

pub fn sys_fs_attr_set(
    fd: usize,
    name_ptr: usize,
    name_len: usize,
    val_ptr: usize,
    val_len: usize,
    type_and_flags: usize,
) -> SysResult<usize> {
    validate_user_range(name_ptr, name_len, false)?;
    validate_user_range(val_ptr, val_len, false)?;

    let mut name_buf = vec![0u8; name_len];
    let mut val_buf = vec![0u8; val_len];
    unsafe {
        copyin(&mut name_buf, name_ptr)?;
        copyin(&mut val_buf, val_ptr)?;
    }
    let name = core::str::from_utf8(&name_buf).map_err(|_| Errno::EINVAL)?;

    let node = {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let lock = pinfo_arc.lock();
        lock.handle_table.get(fd as u32)?.node.clone()
    };

    let val_type = (type_and_flags & 0xFF) as u8;
    let flags = ((type_and_flags >> 8) & 0xFF) as u8;
    node.attr_set(name, &val_buf, val_type, flags)?;
    Ok(0)
}

pub fn sys_fs_attr_remove(fd: usize, name_ptr: usize, name_len: usize) -> SysResult<usize> {
    validate_user_range(name_ptr, name_len, false)?;
    let mut name_buf = vec![0u8; name_len];
    unsafe { copyin(&mut name_buf, name_ptr)? };
    let name = core::str::from_utf8(&name_buf).map_err(|_| Errno::EINVAL)?;

    let node = {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let lock = pinfo_arc.lock();
        lock.handle_table.get(fd as u32)?.node.clone()
    };

    node.attr_remove(name)?;
    Ok(0)
}

pub fn sys_fs_attr_list(fd: usize, buf_ptr: usize, buf_len: usize) -> SysResult<usize> {
    crate::ktrace!("SYS_FS_ATTR_LIST: fd={} buf_ptr={:x} buf_len={}", fd, buf_ptr, buf_len);
    let node = {
        let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
        let lock = pinfo_arc.lock();
        lock.handle_table.get(fd as u32)?.node.clone()
    };

    if buf_ptr == 0 || buf_len == 0 {
        // Just query total size? For now we just return EFAULT or similar if buf is null
        // but maybe we want to allow querying size.
        // POSIX listxattr returns size if size is 0.
    }

    let mut kbuf = vec![0u8; buf_len];
    let res = node.attr_list(&mut kbuf);
    match res {
        Ok(n) => {
            crate::ktrace!("SYS_FS_ATTR_LIST: node.attr_list returned {}", n);
            if n > 0 {
                validate_user_range(buf_ptr, n, true)?;
                unsafe { copyout(buf_ptr, &kbuf[..n])? };
            }
            Ok(n)
        }
        Err(e) => {
            crate::ktrace!("SYS_FS_ATTR_LIST: node.attr_list failed: {:?}", e);
            Err(e)
        }
    }
}
