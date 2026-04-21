//! Extended-attribute view of [`crate::cache::CacheEntry`].
//!
//! Every cached HTTPS response is surfaced as a set of read-only xattrs on
//! the file at `/https/<host>/<path>` (and at the corresponding node under
//! `/run/httpsd/cache`).  All attribute names use the Linux-style `user.*`
//! namespace with a shared `user.http.` prefix so they are easy to discover
//! via `attr_list`.
//!
//! Names are deterministic and stable — tools can grep for `user.http.etag`
//! or iterate the full header set via `user.http.header.*`.  Every header
//! line is also exposed individually as `user.http.header.<lowercased-name>`
//! so arbitrary response fields (including non-standard ones like
//! `x-frame-options`) can be queried without a whitelist.

extern crate alloc;

use abi::attrs::{AttrListEntryHeader, AttrType};
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::cache::CacheEntry;

/// Shared prefix for every attribute we publish.
pub const XATTR_PREFIX: &str = "user.http.";
/// Prefix for per-header attributes.
pub const XATTR_HEADER_PREFIX: &str = "user.http.header.";

/// One attribute name + typed value, ready to be shipped over the AttrGet
/// or AttrList wire format.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attr {
    pub name: String,
    pub ty: AttrType,
    pub value: Vec<u8>,
}

impl Attr {
    fn utf8(name: &str, value: &str) -> Self {
        Self { name: name.to_string(), ty: AttrType::Utf8, value: value.as_bytes().to_vec() }
    }

    fn u64(name: &str, value: u64) -> Self {
        Self { name: name.to_string(), ty: AttrType::U64, value: value.to_le_bytes().to_vec() }
    }
}

/// Build the full ordered attribute set for a cache entry.
///
/// Order is deterministic: core metadata first (status, url, timestamps),
/// then the convenience aliases for well-known headers, then one entry per
/// raw header so the whole header set is enumerable.
pub fn entry_attrs(entry: &CacheEntry) -> Vec<Attr> {
    let mut out: Vec<Attr> = Vec::new();

    // ── Core metadata ─────────────────────────────────────────────────────
    if entry.head.status != 0 {
        let status_line = if entry.head.reason.is_empty() {
            alloc::format!("{}", entry.head.status)
        } else {
            alloc::format!("{} {}", entry.head.status, entry.head.reason)
        };
        out.push(Attr::utf8("user.http.status", &status_line));
        out.push(Attr::u64("user.http.status_code", entry.head.status as u64));
    }
    out.push(Attr::utf8("user.http.url", &entry.url));
    out.push(Attr::u64("user.http.fetched_at_ns", entry.fetched_at_ns));
    if let Some(expires) = entry.expires_at_ns {
        out.push(Attr::u64("user.http.expires_at_ns", expires));
    }
    out.push(Attr::u64("user.http.hits", entry.hits));
    out.push(Attr::u64("user.http.body_bytes_cached", entry.body.len() as u64));
    if entry.body_truncated {
        out.push(Attr { name: "user.http.body_truncated".into(), ty: AttrType::Bool, value: alloc::vec![1] });
    }

    // ── Convenience aliases for well-known headers ───────────────────────
    if let Some(v) = entry.directives.etag.as_deref() {
        out.push(Attr::utf8("user.http.etag", v));
    }
    if let Some(v) = entry.directives.last_modified.as_deref() {
        out.push(Attr::utf8("user.http.last_modified", v));
    }
    if let Some(v) = entry.head.header("Content-Type") {
        out.push(Attr::utf8("user.http.content_type", v));
    }
    if let Some(v) = entry.head.header("Content-Length") {
        if let Ok(n) = v.trim().parse::<u64>() {
            out.push(Attr::u64("user.http.content_length", n));
        } else {
            out.push(Attr::utf8("user.http.content_length", v));
        }
    }
    if let Some(v) = entry.directives.cache_control.as_deref() {
        out.push(Attr::utf8("user.http.cache_control", v));
    }
    if let Some(v) = entry.directives.expires.as_deref() {
        out.push(Attr::utf8("user.http.expires", v));
    }
    if let Some(v) = entry.directives.vary.as_deref() {
        out.push(Attr::utf8("user.http.vary", v));
    }
    if let Some(m) = entry.directives.max_age {
        out.push(Attr::u64("user.http.max_age", m));
    }
    if entry.directives.no_store {
        out.push(Attr { name: "user.http.no_store".into(), ty: AttrType::Bool, value: alloc::vec![1] });
    }
    if entry.directives.no_cache {
        out.push(Attr { name: "user.http.no_cache".into(), ty: AttrType::Bool, value: alloc::vec![1] });
    }
    if let Some(v) = entry.redirect_target.as_deref() {
        out.push(Attr::utf8("user.http.location", v));
    } else if let Some(v) = entry.head.header("Location") {
        out.push(Attr::utf8("user.http.location", v));
    }

    // ── Every header exposed individually ─────────────────────────────────
    // Multi-valued headers are joined with `\n` so the attribute remains
    // atomic while still losslessly carrying every received value.
    let mut seen: Vec<String> = Vec::new();
    for (name, _) in &entry.head.headers {
        let lower = lowercase_ascii(name);
        if seen.iter().any(|s| s == &lower) {
            continue;
        }
        seen.push(lower.clone());
        let joined: Vec<String> = entry
            .head
            .headers_named(name)
            .map(|v| v.to_string())
            .collect();
        let value = joined.join("\n");
        let attr_name = alloc::format!("{}{}", XATTR_HEADER_PREFIX, lower);
        out.push(Attr::utf8(&attr_name, &value));
    }

    out
}

/// Look up a single attribute by name.
pub fn entry_attr(entry: &CacheEntry, name: &str) -> Option<Attr> {
    // Core + convenience aliases: generate the full list (cheap) and filter.
    // Per-header lookups go through a faster direct path so arbitrary
    // response headers resolve in one pass.
    if let Some(rest) = name.strip_prefix(XATTR_HEADER_PREFIX) {
        if rest.is_empty() {
            return None;
        }
        let mut any = false;
        let mut joined: Vec<String> = Vec::new();
        for (h, v) in &entry.head.headers {
            if lowercase_ascii(h) == rest {
                any = true;
                joined.push(v.clone());
            }
        }
        if !any {
            return None;
        }
        return Some(Attr::utf8(name, &joined.join("\n")));
    }
    entry_attrs(entry).into_iter().find(|a| a.name == name)
}

/// Serialize attribute names into the `AttrList` wire format.
///
/// Wire layout per entry: `[AttrListEntryHeader (8 bytes)][name bytes]`.
pub fn serialize_attr_list(attrs: &[Attr]) -> Vec<u8> {
    let mut out = Vec::new();
    for a in attrs {
        let hdr = AttrListEntryHeader {
            name_len: a.name.len() as u16,
            value_type: a.ty as u8,
            flags: 0,
            value_len: a.value.len() as u32,
        };
        // SAFETY: `AttrListEntryHeader` is `#[repr(C)]` and 8 bytes wide.
        let bytes = unsafe {
            core::slice::from_raw_parts(&hdr as *const _ as *const u8, core::mem::size_of_val(&hdr))
        };
        out.extend_from_slice(bytes);
        out.extend_from_slice(a.name.as_bytes());
    }
    out
}

fn lowercase_ascii(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        out.push(b.to_ascii_lowercase() as char);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::{CacheDirectives, CacheEntry};
    use http::ResponseHead;
    extern crate std;

    fn sample_entry() -> CacheEntry {
        let raw = b"HTTP/1.1 200 OK\r\n\
                    Content-Type: text/html; charset=UTF-8\r\n\
                    Content-Length: 123\r\n\
                    ETag: \"abc\"\r\n\
                    Cache-Control: max-age=60, no-cache\r\n\
                    X-Frame-Options: DENY\r\n\
                    Set-Cookie: a=1\r\n\
                    Set-Cookie: b=2\r\n\r\n";
        let head = ResponseHead::parse(raw);
        let directives = CacheDirectives::from_head(&head);
        CacheEntry {
            url: "https://example.com/page".into(),
            host: "example.com".into(),
            path: "page".into(),
            head,
            directives,
            redirect_target: None,
            fetched_at_ns: 42,
            expires_at_ns: Some(102),
            body: b"hi".to_vec(),
            body_truncated: false,
            hits: 3,
        }
    }

    #[test]
    fn attr_list_contains_core_metadata_and_well_known_aliases() {
        let attrs = entry_attrs(&sample_entry());
        let names: alloc::vec::Vec<&str> = attrs.iter().map(|a| a.name.as_str()).collect();
        for required in [
            "user.http.status",
            "user.http.status_code",
            "user.http.url",
            "user.http.fetched_at_ns",
            "user.http.etag",
            "user.http.content_type",
            "user.http.content_length",
            "user.http.cache_control",
            "user.http.max_age",
            "user.http.no_cache",
            "user.http.hits",
            "user.http.header.content-type",
            "user.http.header.x-frame-options",
            "user.http.header.set-cookie",
        ] {
            assert!(names.contains(&required), "missing attribute {}: {:?}", required, names);
        }
    }

    #[test]
    fn attr_get_etag_returns_utf8() {
        let e = sample_entry();
        let a = entry_attr(&e, "user.http.etag").expect("etag");
        assert_eq!(a.ty, AttrType::Utf8);
        assert_eq!(a.value, b"\"abc\"");
    }

    #[test]
    fn attr_get_status_code_returns_u64() {
        let e = sample_entry();
        let a = entry_attr(&e, "user.http.status_code").expect("status_code");
        assert_eq!(a.ty, AttrType::U64);
        assert_eq!(a.value, 200u64.to_le_bytes().to_vec());
    }

    #[test]
    fn attr_get_unknown_returns_none() {
        let e = sample_entry();
        assert!(entry_attr(&e, "user.http.nonexistent").is_none());
        assert!(entry_attr(&e, "user.http.header.not-sent").is_none());
    }

    #[test]
    fn per_header_attrs_join_multi_valued_headers_with_newline() {
        let e = sample_entry();
        let a = entry_attr(&e, "user.http.header.set-cookie").expect("set-cookie");
        assert_eq!(a.ty, AttrType::Utf8);
        assert_eq!(a.value, b"a=1\nb=2");
    }

    #[test]
    fn serialize_attr_list_produces_fixed_width_headers() {
        let attrs = alloc::vec![Attr::utf8("user.http.etag", "\"abc\"")];
        let bytes = serialize_attr_list(&attrs);
        // 8-byte header + 14-byte name.
        assert_eq!(bytes.len(), 8 + "user.http.etag".len());
        let name_len = u16::from_le_bytes([bytes[0], bytes[1]]);
        assert_eq!(name_len as usize, "user.http.etag".len());
        assert_eq!(bytes[2], AttrType::Utf8 as u8);
    }

    #[test]
    fn redirect_surfaces_location_attr() {
        let mut e = sample_entry();
        e.redirect_target = Some("https://example.com/other".into());
        let a = entry_attr(&e, "user.http.location").expect("location");
        assert_eq!(a.value, b"https://example.com/other");
    }
}
