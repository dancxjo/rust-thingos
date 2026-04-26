//! Kernel-side read-only file content cache.
//!
//! Provides a simple, non-evicting in-memory cache of file contents keyed by
//! absolute VFS path.  The primary motivation is to avoid repeated IPC
//! round-trips to userland filesystem providers (e.g. `iso9660d`) when the
//! same executable is spawned more than once.
//!
//! # Design
//! - **Read-only / write-through**: entries are only populated on first read.
//!   There is no invalidation path because the initial target (ISO 9660) is
//!   read-only media.
//! - **Non-evicting**: the cache grows without bound until the kernel is
//!   rebooted.  A future extension can add LRU eviction or memory-pressure
//!   callbacks.
//! - **Path-keyed**: the cache key is the normalised absolute VFS path string.
//!   This is sufficient for the ISO 9660 use-case where paths are stable.
//! - **Arc-shared**: callers receive an `Arc<alloc::vec::Vec<u8>>` so multiple
//!   concurrent spawn requests for the same binary share a single allocation
//!   without copying.
//!
//! # Thread safety
//! The cache is protected by a `spin::Mutex`.  The lock is held only while
//! checking or inserting; it is never held across an IPC read.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

use spin::Mutex;

static PAGE_CACHE: Mutex<BTreeMap<String, Arc<Vec<u8>>>> = Mutex::new(BTreeMap::new());

/// Look up a cached file by its absolute VFS path.
///
/// Returns `Some(Arc<Vec<u8>>)` on a cache hit, `None` on a miss.
#[inline]
pub fn get(path: &str) -> Option<Arc<Vec<u8>>> {
    PAGE_CACHE.lock().get(path).cloned()
}

/// Store file contents in the cache under the given absolute VFS path.
///
/// If an entry for `path` already exists it is **not** replaced — the first
/// writer wins.  This is safe because cached content is always read-only.
#[inline]
pub fn put(path: &str, data: Arc<Vec<u8>>) {
    let mut cache = PAGE_CACHE.lock();
    cache.entry(String::from(path)).or_insert(data);
}

/// Return the number of entries currently held in the cache.
///
/// Primarily useful for diagnostics and unit tests.
#[cfg(any(test, feature = "diagnostic-apps"))]
pub fn len() -> usize {
    PAGE_CACHE.lock().len()
}

/// Return `true` when the cache holds no entries.
///
/// Primarily useful for diagnostics and unit tests.
#[cfg(any(test, feature = "diagnostic-apps"))]
pub fn is_empty() -> bool {
    PAGE_CACHE.lock().is_empty()
}

/// Remove all entries from the cache.
///
/// Only intended for use in unit tests; calling this at runtime would force
/// all subsequent spawns to re-read from the provider.
#[cfg(test)]
pub fn clear_for_test() {
    PAGE_CACHE.lock().clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_miss_returns_none() {
        clear_for_test();
        assert!(get("/bin/nonexistent").is_none());
    }

    #[test]
    fn cache_hit_returns_stored_data() {
        clear_for_test();
        let data = Arc::new(alloc::vec![1u8, 2, 3, 4]);
        put("/bin/test_bin", data.clone());
        let hit = get("/bin/test_bin").expect("expected cache hit");
        assert_eq!(&*hit, &[1u8, 2, 3, 4]);
    }

    #[test]
    fn put_is_idempotent_first_writer_wins() {
        clear_for_test();
        let first = Arc::new(alloc::vec![0xAAu8]);
        let second = Arc::new(alloc::vec![0xBBu8]);
        put("/bin/idem", first.clone());
        put("/bin/idem", second.clone());
        let hit = get("/bin/idem").expect("expected cache hit");
        // First writer must win.
        assert_eq!(hit[0], 0xAA);
    }

    #[test]
    fn distinct_paths_are_independent() {
        clear_for_test();
        put("/bin/a", Arc::new(alloc::vec![1u8]));
        put("/bin/b", Arc::new(alloc::vec![2u8]));
        assert_eq!(get("/bin/a").unwrap()[0], 1);
        assert_eq!(get("/bin/b").unwrap()[0], 2);
    }
}
