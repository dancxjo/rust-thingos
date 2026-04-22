//! Header / response cache for [`httpsd`].
//!
//! The cache sits alongside the per-handle streaming body state and lives for
//! the lifetime of the `httpsd` process.  Each entry captures the response
//! status line, the raw header block, a small set of derived cache
//! directives, and (optionally) a body window.  The cache is intentionally
//! small and bounded — it is a transparent, best-effort mirror of what the
//! upstream server sent, not a persistent HTTP cache.
//!
//! Entries are keyed by `(host, path)`.  Inserting past the configured caps
//! evicts the least-recently-used entry of the matching kind
//! (header-only vs. body-carrying), so a single large body cannot starve
//! out everyone else's headers.

extern crate alloc;

use alloc::collections::VecDeque;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use http::ResponseHead;

/// Default cap on the combined raw header bytes retained across all entries.
pub const DEFAULT_HEADER_BYTE_CAP: usize = 1 * 1024 * 1024;
/// Default cap on the combined cached body bytes retained across all entries.
pub const DEFAULT_BODY_BYTE_CAP: usize = 4 * 1024 * 1024;

/// Cache-Control / freshness-related directives extracted from headers.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CacheDirectives {
    /// Raw `Cache-Control` header value, preserved verbatim.
    pub cache_control: Option<String>,
    /// `max-age` seconds parsed from `Cache-Control` when present.
    pub max_age: Option<u64>,
    /// True when the response carries `Cache-Control: no-store`.
    pub no_store: bool,
    /// True when the response carries `Cache-Control: no-cache`.
    pub no_cache: bool,
    /// `ETag` header value (weak tags keep their `W/` prefix).
    pub etag: Option<String>,
    /// `Last-Modified` header value.
    pub last_modified: Option<String>,
    /// `Expires` header value.
    pub expires: Option<String>,
    /// `Vary` header value.
    pub vary: Option<String>,
}

impl CacheDirectives {
    /// Extract freshness directives from a parsed response head.
    pub fn from_head(head: &ResponseHead) -> Self {
        let cache_control = head.header("Cache-Control").map(|s| s.to_string());
        let (max_age, no_store, no_cache) = parse_cache_control(cache_control.as_deref());
        Self {
            cache_control,
            max_age,
            no_store,
            no_cache,
            etag: head.header("ETag").map(|s| s.to_string()),
            last_modified: head.header("Last-Modified").map(|s| s.to_string()),
            expires: head.header("Expires").map(|s| s.to_string()),
            vary: head.header("Vary").map(|s| s.to_string()),
        }
    }
}

fn parse_cache_control(raw: Option<&str>) -> (Option<u64>, bool, bool) {
    let Some(raw) = raw else {
        return (None, false, false);
    };
    let mut max_age = None;
    let mut no_store = false;
    let mut no_cache = false;
    for directive in raw.split(',') {
        let directive = directive.trim();
        if directive.eq_ignore_ascii_case("no-store") {
            no_store = true;
        } else if directive.eq_ignore_ascii_case("no-cache") {
            no_cache = true;
        } else if let Some(value) = directive
            .strip_prefix("max-age=")
            .or_else(|| directive.strip_prefix("Max-Age="))
            .or_else(|| directive.strip_prefix("MAX-AGE="))
        {
            if let Ok(n) = value.trim().parse::<u64>() {
                max_age = Some(n);
            }
        }
    }
    (max_age, no_store, no_cache)
}

/// One cached response.
#[derive(Debug, Clone)]
pub struct CacheEntry {
    /// Canonical URL (always `https://`).
    pub url: String,
    /// Host component (duplicated from the key for readability).
    pub host: String,
    /// Normalized path (may be empty for the site root).
    pub path: String,
    /// Parsed response head.
    pub head: ResponseHead,
    /// Derived cache directives for quick lookup.
    pub directives: CacheDirectives,
    /// Resolved absolute redirect target (for 3xx with `Location`).
    ///
    /// This is the absolute URL the kernel should follow, **not** the raw
    /// `Location` value — relative locations are resolved against the
    /// request URL at insert time.
    pub redirect_target: Option<String>,
    /// Monotonic nanoseconds at which this entry was last refreshed.
    pub fetched_at_ns: u64,
    /// Monotonic nanoseconds after which the entry is considered stale,
    /// derived from `max-age` at insert time (`None` if unknown).
    pub expires_at_ns: Option<u64>,
    /// Optional cached body bytes.  May be truncated compared to the upstream
    /// response; see `body_truncated`.
    pub body: Vec<u8>,
    /// True when `body` is known to be shorter than the upstream response.
    pub body_truncated: bool,
    /// Number of times the entry has been observed since being cached.
    pub hits: u64,
}

impl CacheEntry {
    /// True when the response is a 3xx redirect that should be surfaced as a
    /// symlink instead of a regular file.
    pub fn is_redirect(&self) -> bool {
        self.redirect_target.is_some()
    }
}

/// Key identifying a cache entry.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CacheKey {
    pub host: String,
    pub path: String,
}

impl CacheKey {
    pub fn new(host: &str, path: &str) -> Self {
        Self { host: host.to_string(), path: path.to_string() }
    }
}

/// Bounded header/body cache for `httpsd`.
///
/// Eviction is strict LRU; a `get` that finds an entry moves it to the
/// most-recently-used position.  Two independent byte budgets (header and
/// body) ensure a single large body cannot push out unrelated headers.
#[derive(Debug)]
pub struct HttpsCache {
    entries: Vec<CacheEntry>,
    /// Most-recent-first ordering of indices into `entries`.
    lru: VecDeque<usize>,
    header_bytes: usize,
    body_bytes: usize,
    pub header_cap: usize,
    pub body_cap: usize,
}

impl Default for HttpsCache {
    fn default() -> Self {
        Self::with_caps(DEFAULT_HEADER_BYTE_CAP, DEFAULT_BODY_BYTE_CAP)
    }
}

impl HttpsCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_caps(header_cap: usize, body_cap: usize) -> Self {
        Self {
            entries: Vec::new(),
            lru: VecDeque::new(),
            header_bytes: 0,
            body_bytes: 0,
            header_cap,
            body_cap,
        }
    }

    /// Total number of entries currently retained.
    pub fn len(&self) -> usize {
        self.entries.iter().filter(|e| !e.url.is_empty()).count()
    }

    /// True when no entries are currently retained.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterate over cached entries in most-recently-used order.
    pub fn iter_mru(&self) -> impl Iterator<Item = &CacheEntry> {
        self.lru.iter().filter_map(move |&idx| {
            let e = self.entries.get(idx)?;
            if e.url.is_empty() { None } else { Some(e) }
        })
    }

    fn find_index(&self, key: &CacheKey) -> Option<usize> {
        self.entries
            .iter()
            .position(|e| !e.url.is_empty() && e.host == key.host && e.path == key.path)
    }

    /// Look up a cache entry without updating LRU order.  Useful for
    /// read-only queries (e.g. serving xattrs) that should not influence
    /// eviction.
    pub fn peek(&self, key: &CacheKey) -> Option<&CacheEntry> {
        self.find_index(key).map(|idx| &self.entries[idx])
    }

    /// Look up a cache entry and mark it as most-recently-used.
    pub fn get(&mut self, key: &CacheKey) -> Option<&CacheEntry> {
        let idx = self.find_index(key)?;
        self.touch(idx);
        self.entries[idx].hits = self.entries[idx].hits.saturating_add(1);
        Some(&self.entries[idx])
    }

    fn touch(&mut self, idx: usize) {
        self.lru.retain(|&i| i != idx);
        self.lru.push_front(idx);
    }

    /// Insert (or replace) an entry.  Evicts LRU entries of the matching
    /// budget until the caps are respected.
    pub fn insert(&mut self, entry: CacheEntry) {
        let key = CacheKey::new(&entry.host, &entry.path);
        if let Some(idx) = self.find_index(&key) {
            let old = core::mem::replace(&mut self.entries[idx], entry);
            self.header_bytes = self.header_bytes.saturating_sub(old.head.raw.len());
            self.body_bytes = self.body_bytes.saturating_sub(old.body.len());
            self.header_bytes = self.header_bytes.saturating_add(self.entries[idx].head.raw.len());
            self.body_bytes = self.body_bytes.saturating_add(self.entries[idx].body.len());
            self.touch(idx);
        } else {
            let idx = self.slot_for_new();
            self.header_bytes = self.header_bytes.saturating_add(entry.head.raw.len());
            self.body_bytes = self.body_bytes.saturating_add(entry.body.len());
            self.entries[idx] = entry;
            self.lru.push_front(idx);
        }
        self.evict_to_fit();
    }

    fn slot_for_new(&mut self) -> usize {
        for (idx, e) in self.entries.iter().enumerate() {
            if e.url.is_empty() {
                return idx;
            }
        }
        self.entries.push(CacheEntry {
            url: String::new(),
            host: String::new(),
            path: String::new(),
            head: ResponseHead::default(),
            directives: CacheDirectives::default(),
            redirect_target: None,
            fetched_at_ns: 0,
            expires_at_ns: None,
            body: Vec::new(),
            body_truncated: false,
            hits: 0,
        });
        self.entries.len() - 1
    }

    fn evict_to_fit(&mut self) {
        // Trim bodies first (cheapest to drop), then whole entries if we are
        // still over budget.
        if self.body_bytes > self.body_cap {
            let mut victims: Vec<usize> = self.lru.iter().rev().copied().collect();
            while self.body_bytes > self.body_cap {
                let Some(idx) = victims.pop() else { break };
                if idx >= self.entries.len() {
                    continue;
                }
                let e = &mut self.entries[idx];
                if e.url.is_empty() || e.body.is_empty() {
                    continue;
                }
                self.body_bytes = self.body_bytes.saturating_sub(e.body.len());
                e.body.clear();
                e.body_truncated = true;
            }
        }
        while self.header_bytes > self.header_cap && self.lru.len() > 1 {
            let Some(idx) = self.lru.pop_back() else { break };
            if idx >= self.entries.len() {
                continue;
            }
            let e = &mut self.entries[idx];
            if e.url.is_empty() {
                continue;
            }
            self.header_bytes = self.header_bytes.saturating_sub(e.head.raw.len());
            self.body_bytes = self.body_bytes.saturating_sub(e.body.len());
            *e = CacheEntry {
                url: String::new(),
                host: String::new(),
                path: String::new(),
                head: ResponseHead::default(),
                directives: CacheDirectives::default(),
                redirect_target: None,
                fetched_at_ns: 0,
                expires_at_ns: None,
                body: Vec::new(),
                body_truncated: false,
                hits: 0,
            };
        }
    }

    /// Current retained header bytes (for diagnostics).
    pub fn header_bytes(&self) -> usize {
        self.header_bytes
    }

    /// Current retained body bytes (for diagnostics).
    pub fn body_bytes(&self) -> usize {
        self.body_bytes
    }
}

/// Build an absolute `https://host/path`-style URL from a host + path pair.
pub fn canonical_url(host: &str, path: &str) -> String {
    if path.is_empty() {
        alloc::format!("https://{}", host)
    } else if path.starts_with('/') {
        alloc::format!("https://{}{}", host, path)
    } else {
        alloc::format!("https://{}/{}", host, path)
    }
}

/// Resolve a `Location` header against a base `(host, path)` pair and
/// return an absolute URL.
///
/// - Absolute URLs (`http://…`, `https://…`) pass through unchanged, except
///   that `http://` is rewritten to `https://` because `httpsd` only serves
///   HTTPS.
/// - Protocol-relative (`//host/path`) becomes `https://host/path`.
/// - Root-relative (`/path`) is appended to `https://<base_host>`.
/// - Path-relative (`path`) is resolved against the directory of the base
///   path.
pub fn resolve_redirect(base_host: &str, base_path: &str, location: &str) -> String {
    let loc = location.trim();
    if let Some(rest) = loc.strip_prefix("https://") {
        return alloc::format!("https://{}", rest);
    }
    if let Some(rest) = loc.strip_prefix("http://") {
        return alloc::format!("https://{}", rest);
    }
    if let Some(rest) = loc.strip_prefix("//") {
        return alloc::format!("https://{}", rest);
    }
    if loc.starts_with('/') {
        return alloc::format!("https://{}{}", base_host, loc);
    }
    // Path-relative: drop the last segment of base_path and append.
    let parent = match base_path.rfind('/') {
        Some(0) => "/",
        Some(idx) => &base_path[..idx + 1],
        None => "/",
    };
    if parent.starts_with('/') {
        alloc::format!("https://{}{}{}", base_host, parent, loc)
    } else {
        alloc::format!("https://{}/{}{}", base_host, parent, loc)
    }
}

/// Split an absolute `https://host/path` URL back into a `(host, path)`
/// pair usable as a cache key.  The returned path has no leading `/`.
///
/// Falls back to `(url, "")` for malformed input — callers typically treat
/// malformed URLs as opaque strings.
pub fn url_to_host_path(url: &str) -> (String, String) {
    let rest = url.strip_prefix("https://").or_else(|| url.strip_prefix("http://")).unwrap_or(url);
    match rest.find('/') {
        Some(idx) => {
            let host = rest[..idx].to_string();
            let path = rest[idx + 1..].to_string();
            (host, path)
        }
        None => (rest.to_string(), String::new()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;
    use alloc::vec;

    fn head_from(raw: &[u8]) -> ResponseHead {
        ResponseHead::parse(raw)
    }

    #[test]
    fn parse_cache_control_extracts_max_age_and_directives() {
        let (max, store, cache) = parse_cache_control(Some("public, max-age=3600, no-cache"));
        assert_eq!(max, Some(3600));
        assert!(!store);
        assert!(cache);
    }

    #[test]
    fn parse_cache_control_no_store() {
        let (max, store, cache) = parse_cache_control(Some("no-store"));
        assert_eq!(max, None);
        assert!(store);
        assert!(!cache);
    }

    #[test]
    fn directives_from_head_captures_etag_and_expires() {
        let raw = b"HTTP/1.1 200 OK\r\n\
                    Cache-Control: max-age=42\r\n\
                    ETag: W/\"abc\"\r\n\
                    Last-Modified: Wed, 21 Oct 2020 07:28:00 GMT\r\n\
                    Expires: Thu, 22 Oct 2020 07:28:00 GMT\r\n\
                    Vary: Accept-Encoding\r\n\r\n";
        let head = head_from(raw);
        let dir = CacheDirectives::from_head(&head);
        assert_eq!(dir.max_age, Some(42));
        assert_eq!(dir.etag.as_deref(), Some("W/\"abc\""));
        assert_eq!(dir.last_modified.as_deref(), Some("Wed, 21 Oct 2020 07:28:00 GMT"));
        assert_eq!(dir.expires.as_deref(), Some("Thu, 22 Oct 2020 07:28:00 GMT"));
        assert_eq!(dir.vary.as_deref(), Some("Accept-Encoding"));
    }

    #[test]
    fn resolve_redirect_handles_absolute_and_relative() {
        assert_eq!(
            resolve_redirect("a.example", "wiki/Foo", "https://b.example/bar"),
            "https://b.example/bar"
        );
        assert_eq!(
            resolve_redirect("a.example", "wiki/Foo", "http://b.example/bar"),
            "https://b.example/bar"
        );
        assert_eq!(
            resolve_redirect("a.example", "wiki/Foo", "//c.example/baz"),
            "https://c.example/baz"
        );
        assert_eq!(
            resolve_redirect("a.example", "wiki/Foo", "/Main_Page"),
            "https://a.example/Main_Page"
        );
        assert_eq!(
            resolve_redirect("a.example", "/wiki/Foo", "Main_Page"),
            "https://a.example/wiki/Main_Page"
        );
        assert_eq!(resolve_redirect("a.example", "", "/Main_Page"), "https://a.example/Main_Page");
    }

    #[test]
    fn url_to_host_path_round_trip() {
        assert_eq!(
            url_to_host_path("https://example.com/index.html"),
            ("example.com".to_string(), "index.html".to_string())
        );
        assert_eq!(
            url_to_host_path("https://example.com"),
            ("example.com".to_string(), String::new())
        );
    }

    #[test]
    fn canonical_url_handles_empty_and_leading_slash() {
        assert_eq!(canonical_url("example.com", ""), "https://example.com");
        assert_eq!(canonical_url("example.com", "foo/bar"), "https://example.com/foo/bar");
        assert_eq!(canonical_url("example.com", "/foo"), "https://example.com/foo");
    }

    fn make_entry(host: &str, path: &str, headers_body: &[u8], body: &[u8]) -> CacheEntry {
        let head = ResponseHead::parse(headers_body);
        let directives = CacheDirectives::from_head(&head);
        CacheEntry {
            url: canonical_url(host, path),
            host: host.to_string(),
            path: path.to_string(),
            head,
            directives,
            redirect_target: None,
            fetched_at_ns: 0,
            expires_at_ns: None,
            body: body.to_vec(),
            body_truncated: false,
            hits: 0,
        }
    }

    #[test]
    fn cache_insert_and_peek() {
        let mut c = HttpsCache::new();
        let e =
            make_entry("example.com", "foo", b"HTTP/1.1 200 OK\r\nETag: \"x\"\r\n\r\n", b"hello");
        c.insert(e);
        let found = c.peek(&CacheKey::new("example.com", "foo")).expect("cached");
        assert_eq!(found.head.status, 200);
        assert_eq!(found.directives.etag.as_deref(), Some("\"x\""));
        assert_eq!(found.body, b"hello");
    }

    #[test]
    fn cache_evicts_header_budget_lru() {
        let mut c = HttpsCache::with_caps(128, 1024);
        for i in 0..8 {
            let h = alloc::format!("h{}.example", i);
            let headers = alloc::format!("HTTP/1.1 200 OK\r\nX-Id: {}\r\n\r\n", i);
            c.insert(make_entry(&h, "", headers.as_bytes(), b""));
        }
        // The first inserts should have been evicted.
        assert!(c.peek(&CacheKey::new("h0.example", "")).is_none());
        // Most recent insert still present.
        assert!(c.peek(&CacheKey::new("h7.example", "")).is_some());
    }

    #[test]
    fn cache_body_cap_truncates_old_bodies_but_keeps_headers() {
        let mut c = HttpsCache::with_caps(1024 * 1024, 32);
        c.insert(make_entry("a.example", "", b"HTTP/1.1 200 OK\r\n\r\n", &vec![0u8; 20]));
        c.insert(make_entry("b.example", "", b"HTTP/1.1 200 OK\r\n\r\n", &vec![0u8; 20]));
        let a = c.peek(&CacheKey::new("a.example", "")).expect("a retained");
        // a's body should have been evicted/truncated; b's kept.
        assert!(a.body_truncated);
        assert!(a.body.is_empty());
        let b = c.peek(&CacheKey::new("b.example", "")).expect("b retained");
        assert_eq!(b.body.len(), 20);
    }

    #[test]
    fn cache_replacement_updates_byte_accounting() {
        let mut c = HttpsCache::with_caps(1024, 1024);
        c.insert(make_entry("x.example", "", b"HTTP/1.1 200 OK\r\n\r\n", b"old"));
        let before = c.body_bytes();
        c.insert(make_entry("x.example", "", b"HTTP/1.1 200 OK\r\n\r\n", b"newer-body"));
        assert_ne!(c.body_bytes(), before);
        let e = c.peek(&CacheKey::new("x.example", "")).unwrap();
        assert_eq!(e.body, b"newer-body");
    }
}
