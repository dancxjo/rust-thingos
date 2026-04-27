//! ThingOS-specific file descriptor ownership and borrowing.

#![stable(feature = "os_thingos", since = "1.0.0")]

use crate::marker::PhantomData;
use crate::mem::ManuallyDrop;
use crate::sys::thingos_syscall_numbers::{SYS_FS_CLOSE, SYS_FS_DUP};
use crate::sys::{AsInner, FromInner, IntoInner, raw_syscall6};
use crate::{fmt, fs, io};

#[stable(feature = "os_thingos", since = "1.0.0")]
pub type RawFd = u32;

#[stable(feature = "os_thingos", since = "1.0.0")]
pub trait AsRawFd {
    #[stable(feature = "os_thingos", since = "1.0.0")]
    fn as_raw_fd(&self) -> RawFd;
}

#[stable(feature = "os_thingos", since = "1.0.0")]
pub trait FromRawFd {
    #[stable(feature = "os_thingos", since = "1.0.0")]
    unsafe fn from_raw_fd(fd: RawFd) -> Self;
}

#[stable(feature = "os_thingos", since = "1.0.0")]
pub trait IntoRawFd {
    #[stable(feature = "os_thingos", since = "1.0.0")]
    fn into_raw_fd(self) -> RawFd;
}

#[derive(Copy, Clone)]
#[repr(transparent)]
#[stable(feature = "os_thingos", since = "1.0.0")]
pub struct BorrowedFd<'fd> {
    fd: RawFd,
    _phantom: PhantomData<&'fd OwnedFd>,
}

#[repr(transparent)]
#[stable(feature = "os_thingos", since = "1.0.0")]
pub struct OwnedFd {
    fd: RawFd,
}

impl BorrowedFd<'_> {
    #[inline]
    #[track_caller]
    #[stable(feature = "os_thingos", since = "1.0.0")]
    #[rustc_const_stable(feature = "os_thingos", since = "1.0.0")]
    pub const unsafe fn borrow_raw(fd: RawFd) -> Self {
        assert!(fd != RawFd::MAX, "fd != RawFd::MAX");
        Self { fd, _phantom: PhantomData }
    }

    #[stable(feature = "os_thingos", since = "1.0.0")]
    pub fn try_clone_to_owned(&self) -> io::Result<OwnedFd> {
        let ret = unsafe { raw_syscall6(SYS_FS_DUP, self.as_raw_fd() as usize, 0, 0, 0, 0, 0) };
        if ret < 0 {
            Err(io::Error::from_raw_os_error((-ret) as i32))
        } else {
            Ok(unsafe { OwnedFd::from_raw_fd(ret as RawFd) })
        }
    }
}

impl OwnedFd {
    #[stable(feature = "os_thingos", since = "1.0.0")]
    pub fn try_clone(&self) -> io::Result<Self> {
        self.as_fd().try_clone_to_owned()
    }
}

#[stable(feature = "os_thingos", since = "1.0.0")]
impl AsRawFd for BorrowedFd<'_> {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}

#[stable(feature = "os_thingos", since = "1.0.0")]
impl AsRawFd for OwnedFd {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}

#[stable(feature = "os_thingos", since = "1.0.0")]
impl IntoRawFd for OwnedFd {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        ManuallyDrop::new(self).fd
    }
}

#[stable(feature = "os_thingos", since = "1.0.0")]
impl FromRawFd for OwnedFd {
    #[inline]
    #[track_caller]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        assert!(fd != RawFd::MAX, "fd != RawFd::MAX");
        Self { fd }
    }
}

#[stable(feature = "os_thingos", since = "1.0.0")]
impl Drop for OwnedFd {
    #[inline]
    fn drop(&mut self) {
        let _ = unsafe { raw_syscall6(SYS_FS_CLOSE, self.fd as usize, 0, 0, 0, 0, 0) };
    }
}

#[stable(feature = "os_thingos", since = "1.0.0")]
impl fmt::Debug for BorrowedFd<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BorrowedFd").field("fd", &self.fd).finish()
    }
}

#[stable(feature = "os_thingos", since = "1.0.0")]
impl fmt::Debug for OwnedFd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OwnedFd").field("fd", &self.fd).finish()
    }
}

#[stable(feature = "os_thingos", since = "1.0.0")]
pub trait AsFd {
    #[stable(feature = "os_thingos", since = "1.0.0")]
    fn as_fd(&self) -> BorrowedFd<'_>;
}

#[stable(feature = "os_thingos", since = "1.0.0")]
impl<T: AsFd + ?Sized> AsFd for &T {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        T::as_fd(self)
    }
}

#[stable(feature = "os_thingos", since = "1.0.0")]
impl<T: AsFd + ?Sized> AsFd for &mut T {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        T::as_fd(self)
    }
}

#[stable(feature = "os_thingos", since = "1.0.0")]
impl AsFd for BorrowedFd<'_> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        *self
    }
}

#[stable(feature = "os_thingos", since = "1.0.0")]
impl AsFd for OwnedFd {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw(self.as_raw_fd()) }
    }
}

#[stable(feature = "os_thingos", since = "1.0.0")]
impl AsRawFd for RawFd {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        *self
    }
}

#[stable(feature = "os_thingos", since = "1.0.0")]
impl IntoRawFd for RawFd {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self
    }
}

#[stable(feature = "os_thingos", since = "1.0.0")]
impl FromRawFd for RawFd {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> RawFd {
        fd
    }
}

#[stable(feature = "os_thingos", since = "1.0.0")]
impl AsRawFd for fs::File {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.as_inner().as_raw_fd()
    }
}

#[stable(feature = "os_thingos", since = "1.0.0")]
impl FromRawFd for fs::File {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> fs::File {
        fs::File::from_inner(unsafe { crate::sys::fs::File::from_raw_fd(fd) })
    }
}

#[stable(feature = "os_thingos", since = "1.0.0")]
impl IntoRawFd for fs::File {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.into_inner().into_raw_fd()
    }
}

#[stable(feature = "os_thingos", since = "1.0.0")]
impl AsFd for fs::File {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw(self.as_raw_fd()) }
    }
}

#[stable(feature = "os_thingos", since = "1.0.0")]
impl From<fs::File> for OwnedFd {
    #[inline]
    fn from(file: fs::File) -> OwnedFd {
        unsafe { OwnedFd::from_raw_fd(file.into_raw_fd()) }
    }
}

#[stable(feature = "os_thingos", since = "1.0.0")]
impl From<OwnedFd> for fs::File {
    #[inline]
    fn from(owned_fd: OwnedFd) -> Self {
        unsafe { fs::File::from_raw_fd(owned_fd.into_raw_fd()) }
    }
}

#[stable(feature = "os_thingos", since = "1.0.0")]
impl<T: AsRawFd + ?Sized> AsRawFd for crate::sync::Arc<T> {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        (**self).as_raw_fd()
    }
}

#[stable(feature = "os_thingos", since = "1.0.0")]
impl<T: AsRawFd + ?Sized> AsRawFd for crate::rc::Rc<T> {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        (**self).as_raw_fd()
    }
}

#[stable(feature = "os_thingos", since = "1.0.0")]
impl<T: AsRawFd + ?Sized> AsRawFd for Box<T> {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        (**self).as_raw_fd()
    }
}

#[stable(feature = "os_thingos", since = "1.0.0")]
impl<T: AsFd + ?Sized> AsFd for crate::sync::Arc<T> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        (**self).as_fd()
    }
}

#[stable(feature = "os_thingos", since = "1.0.0")]
impl<T: AsFd + ?Sized> AsFd for crate::rc::Rc<T> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        (**self).as_fd()
    }
}

#[stable(feature = "os_thingos", since = "1.0.0")]
impl<T: AsFd + ?Sized> AsFd for Box<T> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        (**self).as_fd()
    }
}
