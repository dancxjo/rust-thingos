//! Platform Abstraction Layer (PAL)
//!
//! This module defines the explicit low-level contract between no_std
//! Thing-OS code and the underlying platform. Normal non-kernel userspace
//! may use Thing-OS `std` directly when that is more convenient.
//!
//! # Design Principles
//!
//! - **Explicit over implicit**: Platform capabilities are explicitly surfaced
//! - **Minimal and stable**: Only essential platform primitives are exposed
//! - **Replaceable**: Implementations can be swapped without breaking consumers
//! - **No std inside PAL**: This layer stays `no_std`
//!
//! # Platform Surface
//!
//! The PAL provides:
//! - `log`: Logging primitives
//! - `clock`: Time and monotonic clock access
//! - `abort`: Panic and abort behavior
//! - `alloc`: Memory allocator hooks
//! - `net`: Network device access

pub mod abort;
#[cfg(feature = "global-alloc")]
pub mod alloc;
pub mod clock;
pub mod log;
pub mod net;

/// Shared syscall numbers included directly to avoid dependency on the full
/// `abi` crate (and its transitive `serde` dependency).
mod numbers {
    include!("../../../abi/src/numbers.rs");
}
pub use numbers::*;
