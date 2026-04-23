//! ThingOS-specific extensions to primitives in the [`std::fs`] module.

#![stable(feature = "os_thingos", since = "1.0.0")]

use crate::fs;

use crate::sys::AsInner;

/// OS-specific extensions to [`fs::Metadata`].
#[stable(feature = "os_thingos", since = "1.0.0")]
pub trait MetadataExt {
    #[stable(feature = "os_thingos", since = "1.0.0")]
    fn st_dev(&self) -> u64;
    #[stable(feature = "os_thingos", since = "1.0.0")]
    fn st_ino(&self) -> u64;
    #[stable(feature = "os_thingos", since = "1.0.0")]
    fn st_mode(&self) -> u32;
    #[stable(feature = "os_thingos", since = "1.0.0")]
    fn st_nlink(&self) -> u64;
    #[stable(feature = "os_thingos", since = "1.0.0")]
    fn st_uid(&self) -> u32;
    #[stable(feature = "os_thingos", since = "1.0.0")]
    fn st_gid(&self) -> u32;
    #[stable(feature = "os_thingos", since = "1.0.0")]
    fn st_rdev(&self) -> u64;
    #[stable(feature = "os_thingos", since = "1.0.0")]
    fn st_size(&self) -> u64;
    #[stable(feature = "os_thingos", since = "1.0.0")]
    fn st_atime(&self) -> i64;
    #[stable(feature = "os_thingos", since = "1.0.0")]
    fn st_atime_nsec(&self) -> i64;
    #[stable(feature = "os_thingos", since = "1.0.0")]
    fn st_mtime(&self) -> i64;
    #[stable(feature = "os_thingos", since = "1.0.0")]
    fn st_mtime_nsec(&self) -> i64;
    #[stable(feature = "os_thingos", since = "1.0.0")]
    fn st_ctime(&self) -> i64;
    #[stable(feature = "os_thingos", since = "1.0.0")]
    fn st_ctime_nsec(&self) -> i64;
    #[stable(feature = "os_thingos", since = "1.0.0")]
    fn st_blksize(&self) -> u64;
    #[stable(feature = "os_thingos", since = "1.0.0")]
    fn st_blocks(&self) -> u64;
}

#[stable(feature = "os_thingos", since = "1.0.0")]
impl MetadataExt for fs::Metadata {
    fn st_dev(&self) -> u64 { self.as_inner().st_dev() }
    fn st_ino(&self) -> u64 { self.as_inner().st_ino() }
    fn st_mode(&self) -> u32 { self.as_inner().st_mode() }
    fn st_nlink(&self) -> u64 { self.as_inner().st_nlink() }
    fn st_uid(&self) -> u32 { self.as_inner().st_uid() }
    fn st_gid(&self) -> u32 { self.as_inner().st_gid() }
    fn st_rdev(&self) -> u64 { self.as_inner().st_rdev() }
    fn st_size(&self) -> u64 { self.as_inner().st_size() }
    fn st_atime(&self) -> i64 { self.as_inner().st_atime() }
    fn st_atime_nsec(&self) -> i64 { self.as_inner().st_atime_nsec() }
    fn st_mtime(&self) -> i64 { self.as_inner().st_mtime() }
    fn st_mtime_nsec(&self) -> i64 { self.as_inner().st_mtime_nsec() }
    fn st_ctime(&self) -> i64 { self.as_inner().st_ctime() }
    fn st_ctime_nsec(&self) -> i64 { self.as_inner().st_ctime_nsec() }
    fn st_blksize(&self) -> u64 { self.as_inner().st_blksize() }
    fn st_blocks(&self) -> u64 { self.as_inner().st_blocks() }
}
