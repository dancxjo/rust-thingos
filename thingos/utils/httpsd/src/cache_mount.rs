//! `/run/httpsd/cache` filesystem layout.
//!
//! This module renders a [`crate::cache::HttpsCache`] as a small tree of
//! virtual files so operators can `cat` and `ls` the cache directly:
//!
//! - `/run/httpsd/cache/index` — newline-delimited summary of cached URLs.
//! - `/run/httpsd/cache/<host>/<path>/status` — HTTP status line.
//! - `/run/httpsd/cache/<host>/<path>/headers` — raw header block as received.
//! - `/run/httpsd/cache/<host>/<path>/meta` — key=value metadata dump.
//! - `/run/httpsd/cache/<host>/<path>/body` — cached body window.
//!
//! The layout is compositional: helpers here only know how to render
//! entries into byte strings; the VFS provider in [`crate::main`] turns
//! request paths into the right helper call.

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::cache::{CacheEntry, HttpsCache};

/// Virtual file names rendered under each `<host>/<path>/` directory.
pub const LEAF_STATUS: &str = "status";
pub const LEAF_HEADERS: &str = "headers";
pub const LEAF_META: &str = "meta";
pub const LEAF_BODY: &str = "body";

/// Render the root `index` file: one line per cached URL with a few stats.
pub fn render_index(cache: &HttpsCache) -> Vec<u8> {
    let mut out = String::new();
    for e in cache.iter_mru() {
        let status = if e.head.status == 0 { 0 } else { e.head.status };
        let line = alloc::format!(
            "{}\t{}\tbody={}{}\thits={}\n",
            status,
            e.url,
            e.body.len(),
            if e.body_truncated { "+" } else { "" },
            e.hits,
        );
        out.push_str(&line);
    }
    out.into_bytes()
}

/// Status-line leaf (`"<code> <reason>\n"`).
pub fn render_status(entry: &CacheEntry) -> Vec<u8> {
    let line = if entry.head.reason.is_empty() {
        alloc::format!("{}\n", entry.head.status)
    } else {
        alloc::format!("{} {}\n", entry.head.status, entry.head.reason)
    };
    line.into_bytes()
}

/// Raw header block exactly as received (includes the trailing `\r\n\r\n`).
pub fn render_headers(entry: &CacheEntry) -> Vec<u8> {
    entry.head.raw.clone()
}

/// Key=value metadata dump.  Deterministic ordering so diffs are useful.
pub fn render_meta(entry: &CacheEntry) -> Vec<u8> {
    let mut out = String::new();
    push_kv(&mut out, "url", &entry.url);
    push_kv(&mut out, "host", &entry.host);
    push_kv(&mut out, "path", &entry.path);
    push_kv(&mut out, "status", &entry.head.status.to_string());
    if !entry.head.reason.is_empty() {
        push_kv(&mut out, "reason", &entry.head.reason);
    }
    push_kv(&mut out, "fetched_at_ns", &entry.fetched_at_ns.to_string());
    if let Some(n) = entry.expires_at_ns {
        push_kv(&mut out, "expires_at_ns", &n.to_string());
    }
    push_kv(&mut out, "body_bytes_cached", &entry.body.len().to_string());
    if entry.body_truncated {
        push_kv(&mut out, "body_truncated", "true");
    }
    push_kv(&mut out, "hits", &entry.hits.to_string());
    if let Some(v) = entry.directives.etag.as_deref() {
        push_kv(&mut out, "etag", v);
    }
    if let Some(v) = entry.directives.last_modified.as_deref() {
        push_kv(&mut out, "last_modified", v);
    }
    if let Some(v) = entry.directives.cache_control.as_deref() {
        push_kv(&mut out, "cache_control", v);
    }
    if let Some(n) = entry.directives.max_age {
        push_kv(&mut out, "max_age", &n.to_string());
    }
    if entry.directives.no_store {
        push_kv(&mut out, "no_store", "true");
    }
    if entry.directives.no_cache {
        push_kv(&mut out, "no_cache", "true");
    }
    if let Some(v) = entry.directives.expires.as_deref() {
        push_kv(&mut out, "expires", v);
    }
    if let Some(v) = entry.directives.vary.as_deref() {
        push_kv(&mut out, "vary", v);
    }
    if let Some(v) = entry.redirect_target.as_deref() {
        push_kv(&mut out, "redirect_target", v);
    }
    out.into_bytes()
}

/// Body leaf contents.
pub fn render_body(entry: &CacheEntry) -> Vec<u8> {
    entry.body.clone()
}

fn push_kv(out: &mut String, key: &str, value: &str) {
    out.push_str(key);
    out.push('=');
    out.push_str(value);
    out.push('\n');
}

/// Identify which kind of node a cache-mount path resolves to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CachePathKind {
    /// The mount root (`/`), a directory listing cached hosts.
    Root,
    /// The top-level `index` file.
    Index,
    /// A host directory (`/<host>`).
    HostDir { host: String },
    /// A resource directory (`/<host>/<path>`).
    EntryDir { host: String, path: String },
    /// A leaf inside an entry directory.
    EntryLeaf { host: String, path: String, leaf: CacheLeaf },
}

/// One of the fixed leaves inside a cache entry directory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheLeaf {
    Status,
    Headers,
    Meta,
    Body,
}

impl CacheLeaf {
    pub fn as_filename(self) -> &'static str {
        match self {
            Self::Status => LEAF_STATUS,
            Self::Headers => LEAF_HEADERS,
            Self::Meta => LEAF_META,
            Self::Body => LEAF_BODY,
        }
    }

    fn from_name(s: &str) -> Option<Self> {
        match s {
            LEAF_STATUS => Some(Self::Status),
            LEAF_HEADERS => Some(Self::Headers),
            LEAF_META => Some(Self::Meta),
            LEAF_BODY => Some(Self::Body),
            _ => None,
        }
    }

    pub fn render(self, entry: &CacheEntry) -> Vec<u8> {
        match self {
            Self::Status => render_status(entry),
            Self::Headers => render_headers(entry),
            Self::Meta => render_meta(entry),
            Self::Body => render_body(entry),
        }
    }
}

/// Parse a path relative to the cache mount root into a [`CachePathKind`].
///
/// The lookup is structural only; callers must still verify that the
/// referenced `(host, path)` actually exists in the cache.
pub fn resolve(path: &str) -> CachePathKind {
    let clean = path.trim_matches('/');
    if clean.is_empty() {
        return CachePathKind::Root;
    }
    if clean == "index" {
        return CachePathKind::Index;
    }
    let parts: Vec<&str> = clean.split('/').collect();
    // `<host>`
    if parts.len() == 1 {
        return CachePathKind::HostDir { host: parts[0].to_string() };
    }
    // `<host>/<...>/<leaf?>`
    let host = parts[0].to_string();
    let last = *parts.last().unwrap();
    if let Some(leaf) = CacheLeaf::from_name(last) {
        let entry_path = parts[1..parts.len() - 1].join("/");
        return CachePathKind::EntryLeaf { host, path: entry_path, leaf };
    }
    let entry_path = parts[1..].join("/");
    CachePathKind::EntryDir { host, path: entry_path }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::{CacheDirectives, CacheEntry};
    use http::ResponseHead;
    extern crate std;

    fn sample() -> CacheEntry {
        let raw = b"HTTP/1.1 200 OK\r\nETag: \"e\"\r\nContent-Type: text/plain\r\n\r\n";
        let head = ResponseHead::parse(raw);
        let directives = CacheDirectives::from_head(&head);
        CacheEntry {
            url: "https://example.com/page".into(),
            host: "example.com".into(),
            path: "page".into(),
            head,
            directives,
            redirect_target: None,
            fetched_at_ns: 1,
            expires_at_ns: None,
            body: b"body-bytes".to_vec(),
            body_truncated: false,
            hits: 1,
        }
    }

    #[test]
    fn resolve_maps_top_level_paths() {
        assert_eq!(resolve(""), CachePathKind::Root);
        assert_eq!(resolve("/"), CachePathKind::Root);
        assert_eq!(resolve("index"), CachePathKind::Index);
        assert_eq!(
            resolve("/example.com"),
            CachePathKind::HostDir { host: "example.com".to_string() }
        );
    }

    #[test]
    fn resolve_maps_entry_dir_and_leaves() {
        assert_eq!(
            resolve("example.com/page"),
            CachePathKind::EntryDir { host: "example.com".into(), path: "page".into() }
        );
        assert_eq!(
            resolve("example.com/page/headers"),
            CachePathKind::EntryLeaf {
                host: "example.com".into(),
                path: "page".into(),
                leaf: CacheLeaf::Headers,
            }
        );
        assert_eq!(
            resolve("example.com/a/b/c/status"),
            CachePathKind::EntryLeaf {
                host: "example.com".into(),
                path: "a/b/c".into(),
                leaf: CacheLeaf::Status,
            }
        );
    }

    #[test]
    fn render_headers_preserves_raw_bytes() {
        let entry = sample();
        let rendered = render_headers(&entry);
        assert!(rendered.ends_with(b"\r\n\r\n"));
        let text = core::str::from_utf8(&rendered).unwrap();
        assert!(text.starts_with("HTTP/1.1 200 OK"));
        assert!(text.contains("ETag: \"e\""));
    }

    #[test]
    fn render_status_produces_single_line() {
        let rendered = render_status(&sample());
        assert_eq!(rendered, b"200 OK\n");
    }

    #[test]
    fn render_meta_contains_key_values() {
        let rendered = render_meta(&sample());
        let text = core::str::from_utf8(&rendered).unwrap();
        assert!(text.contains("url=https://example.com/page\n"));
        assert!(text.contains("status=200\n"));
        assert!(text.contains("etag=\"e\"\n"));
        assert!(text.contains("body_bytes_cached=10\n"));
    }

    #[test]
    fn render_index_lists_cached_entries() {
        let mut c = crate::cache::HttpsCache::new();
        c.insert(sample());
        let rendered = render_index(&c);
        let text = core::str::from_utf8(&rendered).unwrap();
        assert!(text.contains("https://example.com/page"));
        assert!(text.contains("200"));
    }
}
