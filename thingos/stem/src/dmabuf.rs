//! Small helpers for fd-backed buffers passed to display and media services.
//!
//! Thing-OS currently represents client-visible dmabuf-style buffers as
//! shareable memfd handles mapped into the caller.  This module keeps the
//! syscall plumbing in one place so applications can request a mapped buffer
//! without assembling `VmMapReq` by hand.

use abi::errors::Errno;
use abi::vm::{VmBacking, VmMapFlags, VmMapReq, VmProt};

use crate::syscall::{memfd_create, vfs_close, vm_map};

#[derive(Clone, Copy, Debug)]
pub struct DmaBuf {
    fd: u32,
    ptr: *mut u8,
    len: usize,
}

impl DmaBuf {
    pub fn create(name: &str, len: usize) -> Result<Self, Errno> {
        let fd = memfd_create(name, len)?;
        let req = VmMapReq {
            addr_hint: 0,
            len,
            prot: VmProt::READ | VmProt::WRITE | VmProt::USER,
            flags: VmMapFlags::empty(),
            backing: VmBacking::File { thing: fd, offset: 0 },
        };

        match vm_map(&req) {
            Ok(resp) => Ok(Self { fd, ptr: resp.addr as *mut u8, len: resp.len }),
            Err(e) => {
                let _ = vfs_close(fd);
                Err(e)
            }
        }
    }

    #[inline]
    pub fn fd(self) -> u32 {
        self.fd
    }

    #[inline]
    pub fn as_mut_ptr(self) -> *mut u8 {
        self.ptr
    }

    #[inline]
    pub fn len(self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        self.len == 0
    }
}
