//! Bridge layer for typed immutable spawn context.
//!
//! `SpawnRecord` is the canonical spawn-time metadata object used by kernel
//! read paths that need argv/auxv.
//!
//! Policy/identity note:
//! - `argv` + `auxv` are execution-image metadata and are projected through this
//!   typed immutable record.
//! - `env` remains quarantined in Unix compatibility state because environment
//!   propagation has policy-sensitive semantics that must be coordinated with
//!   Authority (`who may set/propagate`) and Place (`world context`).

use alloc::vec::Vec;

/// Immutable typed spawn-time metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpawnRecord {
    argv: Vec<Vec<u8>>,
    auxv: Vec<(u64, u64)>,
}

impl SpawnRecord {
    /// Construct an immutable spawn record from typed parts.
    pub fn new(argv: Vec<Vec<u8>>, auxv: Vec<(u64, u64)>) -> Self {
        Self { argv, auxv }
    }

    /// Empty spawn metadata record.
    pub fn empty() -> Self {
        Self { argv: Vec::new(), auxv: Vec::new() }
    }

    /// Borrow spawn argument vector.
    pub fn argv(&self) -> &[Vec<u8>] {
        self.argv.as_slice()
    }

    /// Borrow ELF auxiliary vector.
    pub fn auxv(&self) -> &[(u64, u64)] {
        self.auxv.as_slice()
    }
}

/// Borrow typed spawn metadata from process backing.
pub fn spawn_record_from_process(process: &crate::task::Process) -> &SpawnRecord {
    process.unix_compat.spawn_record()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_record_holds_argv_and_auxv() {
        let record = SpawnRecord::new(
            alloc::vec![b"init".to_vec()],
            alloc::vec![(abi::auxv::AT_PAGESZ, 4096)],
        );
        assert_eq!(record.argv(), [b"init".to_vec()]);
        assert_eq!(record.auxv(), &[(abi::auxv::AT_PAGESZ, 4096)]);
    }

    #[test]
    fn empty_spawn_record_has_no_entries() {
        let record = SpawnRecord::empty();
        assert!(record.argv().is_empty());
        assert!(record.auxv().is_empty());
    }
}
