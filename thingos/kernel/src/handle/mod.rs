//! Handle module: canonical resource-reference boundary for open objects.
//!
//! New resource lookup and conversion code should go through
//! [`bridge`], which defines the first-class `Handle` abstraction and the
//! explicit fd-compatibility boundary while `thing_table` migration is in
//! progress.

pub mod bridge;
