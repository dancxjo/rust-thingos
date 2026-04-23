//! ThingOS anonymous pipe implementation.
//!
//! Uses `SYS_PIPE` (0x3015) to create a kernel pipe backed by `PipeReadNode`
//! and `PipeWriteNode` in the VFS.  The two returned fds behave like POSIX
//! pipe(2): reading blocks until data is available; the read end returns EOF
//! when the last write end is closed.
//!
//! I/O is done via `SYS_READ` (0x1400) and `SYS_WRITE` (0x1401), which work
//! uniformly across all VFS fds (files, pipes, devices).
//!
//! Cloning is done via `SYS_FS_DUP` (0x400C); the fd is closed on drop via
//! `SYS_FS_CLOSE` (0x4001).

use crate::fmt;
use crate::io::{BorrowedCursor, IoSlice, IoSliceMut};
use crate::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};
use crate::sys::pal::raw_syscall6;
use crate::sys::thingos_syscall_numbers::{SYS_FS_CLOSE, SYS_FS_DUP, SYS_PIPE, SYS_READ, SYS_WRITE};
use crate::sys::{AsInner, FromInner, IntoInner};

#[inline]
fn cvt(ret: isize) -> crate::io::Result<usize> {
    if ret < 0 { Err(crate::io::Error::from_raw_os_error((-ret) as i32)) } else { Ok(ret as usize) }
}

/// An anonymous pipe end (either read or write).
///
/// The fd is closed automatically when this value is dropped.
pub struct Pipe(i32);

impl Drop for Pipe {
    fn drop(&mut self) {
        // SAFETY: we own this fd; closing is always valid.
        unsafe { raw_syscall6(SYS_FS_CLOSE, self.0 as u32 as usize, 0, 0, 0, 0, 0) };
    }
}

/// Create an anonymous pipe.
///
/// Returns `(read_end, write_end)`.  The kernel writes two `u32` fd values
/// into a stack buffer via `SYS_PIPE`.
pub fn pipe() -> crate::io::Result<(Pipe, Pipe)> {
    let mut fds = [0u32; 2];
    let ret = unsafe {
        raw_syscall6(SYS_PIPE, fds.as_mut_ptr() as usize, 0, 0, 0, 0, 0)
    };
    cvt(ret)?;
    Ok((Pipe(fds[0] as i32), Pipe(fds[1] as i32)))
}

impl Pipe {
    pub fn try_clone(&self) -> crate::io::Result<Self> {
        let ret = unsafe { raw_syscall6(SYS_FS_DUP, self.0 as u32 as usize, 0, 0, 0, 0, 0) };
        Ok(Pipe(cvt(ret)? as u32 as i32))
    }

    pub fn read(&self, buf: &mut [u8]) -> crate::io::Result<usize> {
        let ret = unsafe {
            raw_syscall6(SYS_READ, self.0 as u32 as usize, buf.as_mut_ptr() as usize, buf.len(), 0, 0, 0)
        };
        cvt(ret)
    }

    pub fn read_buf(&self, mut cursor: BorrowedCursor<'_>) -> crate::io::Result<()> {
        // SAFETY: the syscall initialises the bytes it writes.
        let buf = unsafe { cursor.as_mut() };
        let ret = unsafe {
            raw_syscall6(SYS_READ, self.0 as u32 as usize, buf.as_mut_ptr() as usize, buf.len(), 0, 0, 0)
        };
        let n = cvt(ret)?;
        unsafe { cursor.advance_unchecked(n) };
        Ok(())
    }

    pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> crate::io::Result<usize> {
        match bufs.iter_mut().find(|b| !b.is_empty()) {
            Some(buf) => self.read(buf),
            None => Ok(0),
        }
    }

    #[inline]
    pub fn is_read_vectored(&self) -> bool {
        false
    }

    pub fn read_to_end(&self, buf: &mut crate::vec::Vec<u8>) -> crate::io::Result<usize> {
        let mut total = 0usize;
        let mut tmp = [0u8; 4096];
        loop {
            match self.read(&mut tmp) {
                Ok(0) => return Ok(total),
                Ok(n) => {
                    buf.extend_from_slice(&tmp[..n]);
                    total += n;
                }
                Err(e) => return Err(e),
            }
        }
    }

    pub fn write(&self, buf: &[u8]) -> crate::io::Result<usize> {
        let ret = unsafe {
            raw_syscall6(SYS_WRITE, self.0 as u32 as usize, buf.as_ptr() as usize, buf.len(), 0, 0, 0)
        };
        cvt(ret)
    }

    pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> crate::io::Result<usize> {
        let mut total = 0usize;
        for buf in bufs {
            if buf.is_empty() { continue; }
            total += self.write(buf)?;
        }
        Ok(total)
    }

    #[inline]
    pub fn is_write_vectored(&self) -> bool {
        false
    }

    #[allow(dead_code)]
    pub fn diverge(&self) -> ! {
        panic!("Pipe::diverge called on a live ThingOS pipe")
    }
}

impl fmt::Debug for Pipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pipe(fd={})", self.0)
    }
}

// ── Raw fd traits ────────────────────────────────────────────────────────────

impl AsRawFd for Pipe {
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

impl FromRawFd for Pipe {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Pipe(fd)
    }
}

impl IntoRawFd for Pipe {
    fn into_raw_fd(self) -> RawFd {
        let fd = self.0;
        crate::mem::forget(self);
        fd
    }
}

impl AsFd for Pipe {
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw(self.as_raw_fd()) }
    }
}

// ── Inner conversion traits ──────────────────────────────────────────────────

impl AsInner<i32> for Pipe {
    fn as_inner(&self) -> &i32 {
        &self.0
    }
}

impl IntoInner<RawFd> for Pipe {
    fn into_inner(self) -> RawFd {
        self.into_raw_fd()
    }
}

impl FromInner<RawFd> for Pipe {
    fn from_inner(fd: RawFd) -> Self {
        Pipe(fd)
    }
}

impl FromInner<u32> for Pipe {
    fn from_inner(fd: u32) -> Self {
        Pipe(fd as i32)
    }
}

impl IntoInner<u32> for Pipe {
    fn into_inner(self) -> u32 {
        self.into_raw_fd() as u32
    }
}

impl FromInner<OwnedFd> for Pipe {
    fn from_inner(owned_fd: OwnedFd) -> Self {
        Pipe(owned_fd.into_raw_fd())
    }
}

impl IntoInner<OwnedFd> for Pipe {
    fn into_inner(self) -> OwnedFd {
        unsafe { <OwnedFd as crate::os::fd::FromRawFd>::from_raw_fd(self.into_raw_fd()) }
    }
}
