//! ThingOS-specific definitions.

#![stable(feature = "os_thingos", since = "1.0.0")]
#![doc(cfg(target_os = "thingos"))]

pub mod fd;
pub mod fs;
pub mod net;
pub mod process;
pub mod raw;

/// Returns the raw scheduler-provided process argument captured at process entry.
///
/// Most ThingOS programs should use a normal `fn main()` and ignore this. Seed
/// hosts such as drivers can use it to recover their boot/module context while
/// still using the standard Rust entry point.
#[stable(feature = "os_thingos", since = "1.0.0")]
pub fn boot_arg() -> usize {
    crate::sys::pal::boot_arg()
}

/// A prelude for conveniently writing platform-specific code.
#[stable(feature = "os_thingos", since = "1.0.0")]
pub mod prelude {
    #[doc(no_inline)]
    #[stable(feature = "os_thingos", since = "1.0.0")]
    pub use super::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};
}
