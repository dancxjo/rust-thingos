//! Spawn module: typed, immutable spawn-context bridge.
//!
//! This module introduces `SpawnRecord` as the canonical typed representation
//! of spawn-time metadata (`argv` + `auxv`), replacing direct blob-style access.
//! `env` remains explicit Unix compatibility state in `ProcessUnixCompat` until
//! Authority/Place policy ownership is finalized.

pub mod bridge;
