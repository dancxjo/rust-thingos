//! Resource cache for display backend imports.
//!
//! Avoids repeated [`DisplayBackend::import_buffer`] calls for client buffers
//! and compositor-owned assets that have not changed since the last frame.
//!
//! # Cached resources
//!
//! - **Client buffers** — keyed by `(handle, format, width, height, stride,
//!   modifier)`.  The entry is evicted when the buffer is released so that a
//!   re-use of the same handle (with new pixel content) always triggers a fresh
//!   import.
//! - **Wallpaper** — the path of the currently loaded background texture is
//!   stored so callers can skip `prepare_background` when the path has not
//!   changed.
//! - **Shadow atlas** — a reserved slot for the pistil shadow-atlas GPU buffer.
//!   Currently unused; populated by future integration work.
//!
//! # Reserved slots (future)
//!
//! - Window chrome overlay buffer
//! - Rounded-mask buffer
//! - Glyph-atlas buffer
//!
//! # Debug counters
//!
//! [`CacheCounters`] records cumulative hits, misses, and invalidations.
//! Call [`ResourceCache::take_counters`] to drain and reset them.

use alloc::collections::BTreeMap;
use alloc::string::String;

use abi::pixel::PixelFormat;

use crate::display::DisplayBackend;

// ── public types ─────────────────────────────────────────────────────────────

/// Cumulative cache-activity counters.
///
/// Counts are *cumulative* from the last [`ResourceCache::take_counters`] call.
#[derive(Clone, Copy, Debug, Default)]
pub struct CacheCounters {
    /// Imports satisfied by returning a previously cached buffer_id.
    pub hits: u64,
    /// Imports that required a new [`DisplayBackend::import_buffer`] call.
    pub misses: u64,
    /// Cache entries removed because their buffer was released or replaced.
    pub invalidations: u64,
}

/// Resource cache for compositor-owned and client-imported display buffers.
///
/// See module documentation for the caching strategy and current slot layout.
pub struct ResourceCache {
    /// Client buffer imports: `(handle, format, w, h, stride, modifier)` → buffer_id.
    client_buffers: BTreeMap<ClientBufferKey, u32>,

    /// Path of the wallpaper that is currently loaded as the background buffer.
    ///
    /// `None` means no wallpaper has been loaded yet this session.
    wallpaper_path: Option<String>,

    /// Reserved: imported buffer_id of the pistil shadow-atlas texture.
    ///
    /// Set via [`ResourceCache::set_shadow_buffer_id`] when pistil exports the
    /// atlas.  Invalidated via [`ResourceCache::invalidate_shadow`] when the
    /// display backend is reset.
    shadow_buffer_id: Option<u32>,

    // Reserved slots (not yet used):
    //   chrome_buffer_id:       Option<u32>,
    //   rounded_mask_buffer_id: Option<u32>,
    //   glyph_atlas_buffer_id:  Option<u32>,
    /// Cumulative activity counters since the last `take_counters` call.
    pub counters: CacheCounters,
}

// ── private key ──────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ClientBufferKey {
    /// 8-byte field first to avoid padding before the u32 fields.
    modifier: u64,
    handle: u32,
    width: u32,
    height: u32,
    stride: u32,
    /// `PixelFormat` is `#[repr(u8)]` and derives `Ord`, so it is safe to use
    /// as a BTreeMap key.
    format: PixelFormat,
}

// ── implementation ───────────────────────────────────────────────────────────

impl ResourceCache {
    pub fn new() -> Self {
        Self {
            client_buffers: BTreeMap::new(),
            wallpaper_path: None,
            shadow_buffer_id: None,
            counters: CacheCounters::default(),
        }
    }

    // ── client buffer cache ───────────────────────────────────────────────────

    /// Import a client buffer, reusing the cached entry when the source has not
    /// changed.
    ///
    /// A cache entry is keyed on `(handle, format, width, height, stride,
    /// modifier)`.  If a matching entry exists its `buffer_id` is returned
    /// directly (hit).  Otherwise [`DisplayBackend::import_buffer`] is called
    /// and the result is cached (miss).
    ///
    /// Returns `None` if the backend rejected the import.
    pub fn import_client_buffer(
        &mut self,
        display: &DisplayBackend,
        handle: u32,
        width: u32,
        height: u32,
        stride: u32,
        format: PixelFormat,
        offset: u64,
        modifier: u64,
    ) -> Option<u32> {
        let key = ClientBufferKey { modifier, handle, width, height, stride, format };

        if let Some(&buffer_id) = self.client_buffers.get(&key) {
            self.counters.hits += 1;
            stem::trace!(
                "bloom: cache hit handle={} {}x{} buffer_id={}",
                handle,
                width,
                height,
                buffer_id
            );
            return Some(buffer_id);
        }

        let buffer_id =
            display.import_buffer(handle, width, height, stride, format, offset, modifier)?;
        self.counters.misses += 1;
        stem::trace!(
            "bloom: cache miss handle={} {}x{} → buffer_id={}",
            handle,
            width,
            height,
            buffer_id
        );
        self.client_buffers.insert(key, buffer_id);
        Some(buffer_id)
    }

    /// Release a client buffer: evict its cache entry and call
    /// [`DisplayBackend::release_buffer`].
    ///
    /// If no cache entry exists for `buffer_id` the display release is still
    /// issued (the buffer may have been imported outside the cache).
    pub fn release_client_buffer(&mut self, display: &DisplayBackend, buffer_id: u32) {
        // Scan for the key whose value matches buffer_id (O(n) but the map is
        // small and this path is infrequent compared to frame rendering).
        let key = self.client_buffers.iter().find(|(_, &v)| v == buffer_id).map(|(k, _)| *k);
        if let Some(key) = key {
            self.client_buffers.remove(&key);
            self.counters.invalidations += 1;
            stem::trace!("bloom: cache invalidated buffer_id={}", buffer_id);
        }
        display.release_buffer(buffer_id);
    }

    // ── wallpaper path cache ──────────────────────────────────────────────────

    /// Returns the wallpaper path currently loaded in the cache, if any.
    ///
    /// Callers compare this against the requested path to decide whether
    /// `prepare_background` can be skipped.
    pub fn wallpaper_path(&self) -> Option<&str> {
        self.wallpaper_path.as_deref()
    }

    /// Record the path of the wallpaper that was just successfully loaded.
    ///
    /// Call this after importing the wallpaper texture so subsequent
    /// [`ResourceCache::wallpaper_path`] checks are correct.
    pub fn set_wallpaper_path(&mut self, path: &str) {
        self.wallpaper_path = Some(String::from(path));
    }

    /// Clear the cached wallpaper path without releasing any buffer.
    ///
    /// Use when the display backend reconnects so the next
    /// `prepare_background` call re-uploads the wallpaper unconditionally.
    pub fn clear_wallpaper_path(&mut self) {
        self.wallpaper_path = None;
    }

    // ── shadow atlas cache ────────────────────────────────────────────────────

    /// Returns the cached shadow-atlas buffer_id, if any.
    pub fn shadow_buffer_id(&self) -> Option<u32> {
        self.shadow_buffer_id
    }

    /// Store the imported shadow-atlas buffer_id.
    ///
    /// The previous entry (if any) is silently replaced without releasing the
    /// old buffer; call [`ResourceCache::invalidate_shadow`] before this if you
    /// want to release the old entry.
    pub fn set_shadow_buffer_id(&mut self, buffer_id: u32) {
        self.shadow_buffer_id = Some(buffer_id);
    }

    /// Invalidate the shadow-atlas cache entry and release the buffer from the
    /// display backend.
    ///
    /// No-op if no shadow atlas is currently cached.
    pub fn invalidate_shadow(&mut self, display: &DisplayBackend) {
        if let Some(id) = self.shadow_buffer_id.take() {
            display.release_buffer(id);
            self.counters.invalidations += 1;
            stem::debug!("bloom: cache shadow atlas invalidated buffer_id={}", id);
        }
    }

    // ── bulk invalidation ─────────────────────────────────────────────────────

    /// Invalidate all cached entries, releasing every imported buffer.
    ///
    /// Call when the display backend reconnects or is fully reset so stale
    /// buffer_ids are not presented.
    pub fn invalidate_all(&mut self, display: &DisplayBackend) {
        // Collect buffer_ids then clear, to avoid retaining any reference into
        // the map while iterating and releasing.
        let ids: alloc::vec::Vec<u32> = self.client_buffers.values().copied().collect();
        self.client_buffers.clear();
        for buffer_id in ids {
            display.release_buffer(buffer_id);
            self.counters.invalidations += 1;
        }
        self.invalidate_shadow(display);
        self.wallpaper_path = None;
        stem::debug!("bloom: resource cache fully invalidated");
    }

    // ── diagnostics ──────────────────────────────────────────────────────────

    /// Drain and return the accumulated counters, resetting them to zero.
    pub fn take_counters(&mut self) -> CacheCounters {
        core::mem::take(&mut self.counters)
    }

    /// Log a one-line summary of the current counter state at `debug` level.
    pub fn log_stats(&self) {
        stem::debug!(
            "bloom: cache stats hits={} misses={} invalidations={} entries={}",
            self.counters.hits,
            self.counters.misses,
            self.counters.invalidations,
            self.client_buffers.len(),
        );
    }
}
