//! ThingOS PAL — OS-level syscall bindings.
//!
//! All syscall numbers come from the shared ABI definitions (abi/src/numbers.rs).
//! Unsupported operations return `io::Error::UNSUPPORTED_PLATFORM` explicitly.

use super::raw_syscall6;
use crate::sys::thingos_syscall_numbers::{
    SYS_EXIT, SYS_FS_CHDIR, SYS_FS_GETCWD, SYS_FS_READLINK, SYS_GETPID,
};
use crate::ffi::{OsStr, OsString};
use crate::path::{self, PathBuf};
use crate::{fmt, io};

/// Converts a negative syscall return value to an io::Error.
#[inline]
fn syscall_err(ret: isize) -> io::Error {
    io::Error::from_raw_os_error((-ret) as i32)
}

#[inline]
fn path_as_bytes(path: &path::Path) -> &[u8] {
    path.as_os_str().as_encoded_bytes()
}

pub fn getcwd() -> io::Result<PathBuf> {
    // First call: buf_ptr=0 returns the number of bytes needed.
    let needed = unsafe { raw_syscall6(SYS_FS_GETCWD, 0, 0, 0, 0, 0, 0) };
    if needed < 0 {
        return Err(syscall_err(needed));
    }
    let needed = needed as usize;
    if needed == 0 {
        return Ok(PathBuf::from("/"));
    }
    let mut buf = crate::vec![0u8; needed];
    let ret = unsafe {
        raw_syscall6(SYS_FS_GETCWD, buf.as_mut_ptr() as usize, buf.len(), 0, 0, 0, 0)
    };
    if ret < 0 {
        return Err(syscall_err(ret));
    }
    let n = ret as usize;
    buf.truncate(n);
    // SAFETY: bytes originate from the kernel path API and are in platform path encoding.
    Ok(PathBuf::from(unsafe { OsString::from_encoded_bytes_unchecked(buf) }))
}

pub fn chdir(p: &path::Path) -> io::Result<()> {
    let path = path_as_bytes(p);
    let ret = unsafe {
        raw_syscall6(SYS_FS_CHDIR, path.as_ptr() as usize, path.len(), 0, 0, 0, 0)
    };
    if ret < 0 { Err(syscall_err(ret)) } else { Ok(()) }
}

#[cfg(test)]
mod tests {
    use super::path_as_bytes;
    use crate::ffi::OsStr;
    use crate::path::Path;

    #[test]
    fn path_as_bytes_accepts_non_utf8_paths() {
        let bytes = [b'/', b't', 0xff, b's', b't'];
        let os = unsafe { OsStr::from_encoded_bytes_unchecked(&bytes) };
        assert_eq!(path_as_bytes(Path::new(os)), bytes.as_slice());
    }
}

pub struct SplitPaths<'a> {
    iter: crate::slice::Split<'a, u8, fn(&u8) -> bool>,
}

pub fn split_paths(unparsed: &OsStr) -> SplitPaths<'_> {
    fn is_colon(b: &u8) -> bool {
        *b == b':'
    }
    SplitPaths { iter: unparsed.as_encoded_bytes().split(is_colon as fn(&u8) -> bool) }
}

impl<'a> Iterator for SplitPaths<'a> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> {
        self.iter.next().map(|b| {
            // SAFETY: bytes came from an OsStr via as_encoded_bytes, so they are valid.
            let s = unsafe { OsStr::from_encoded_bytes_unchecked(b) };
            PathBuf::from(s)
        })
    }
}

#[derive(Debug)]
pub struct JoinPathsError;

pub fn join_paths<I, T>(paths: I) -> Result<OsString, JoinPathsError>
where
    I: Iterator<Item = T>,
    T: AsRef<OsStr>,
{
    let mut joined = crate::vec::Vec::new();
    for (i, path) in paths.enumerate() {
        let bytes = path.as_ref().as_encoded_bytes();
        if bytes.contains(&b':') {
            return Err(JoinPathsError);
        }
        if i > 0 {
            joined.push(b':');
        }
        joined.extend_from_slice(bytes);
    }
    // SAFETY: bytes were assembled from OsStr encoded bytes, which are always valid.
    Ok(unsafe { OsString::from_encoded_bytes_unchecked(joined) })
}

impl fmt::Display for JoinPathsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "path segment contains separator `:`".fmt(f)
    }
}

impl crate::error::Error for JoinPathsError {}

pub fn exit(code: i32) -> ! {
    unsafe { raw_syscall6(SYS_EXIT, code as usize, 0, 0, 0, 0, 0) };
    loop {}
}

pub fn getpid() -> u32 {
    let ret = unsafe { raw_syscall6(SYS_GETPID, 0, 0, 0, 0, 0, 0) };
    if ret < 0 { 0 } else { ret as u32 }
}

pub fn home_dir() -> Option<PathBuf> {
    // ThingOS doesn't have a traditional system user DB.  Return None;
    // callers fall back to querying the HOME env var.
    None
}

pub fn temp_dir() -> PathBuf {
    PathBuf::from("/tmp")
}

/// Returns the path of the current process's executable by reading the
/// `/proc/self/exe` symlink exposed by the ThingOS kernel.
pub fn current_exe() -> io::Result<PathBuf> {
    const PATH: &[u8] = b"/proc/self/exe";
    let mut buf = crate::vec![0u8; 4096];
    let ret = unsafe {
        raw_syscall6(
            SYS_FS_READLINK,
            PATH.as_ptr() as usize,
            PATH.len(),
            buf.as_mut_ptr() as usize,
            buf.len(),
            0,
            0,
        )
    };
    if ret < 0 {
        return Err(syscall_err(ret));
    }
    let n = ret as usize;
    buf.truncate(n);
    // SAFETY: bytes originate from the kernel path API and are in platform path encoding.
    Ok(PathBuf::from(unsafe { OsString::from_encoded_bytes_unchecked(buf) }))
}
