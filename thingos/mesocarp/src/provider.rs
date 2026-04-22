//! VFS provider for the `/hosts` tree.
//!
//! Layout served at the mount point:
//!
//! ```text
//! /hosts/                           ← directory (listed by readdir)
//! ├── <peer>.local                  ← regular file; attrs carry metadata
//! ...
//! ```
//!
//! Each host entry is a regular file (mode 0o100444). The file's contents
//! are a single line like `192.168.2.23\n` for a quick `cat`. The same
//! address is also exposed as the `net.ip.ipv4` provider attribute, and
//! `net.hostname` carries the name. `ls /hosts` readdir enumerates the
//! currently-cached hosts.

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
#[cfg(test)]
use alloc::string::ToString;
use alloc::vec::Vec;

use abi::attrs::{AttrListEntryHeader, AttrType};
use abi::errors::Errno;
use ipc_helpers::provider::ProviderResponse;

use crate::mdns;

/// Stable handle for the root directory.
pub const HANDLE_ROOT: u64 = 1;
/// Dynamic host handles start here so they never collide with [`HANDLE_ROOT`].
const HANDLE_HOST_BASE: u64 = 0x0000_0001_0000_0000;

const S_IFDIR: u32 = 0o040_000;
const S_IFREG: u32 = 0o100_000;

const POLLIN: u32 = 0x0001;

/// A single host we know about.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostEntry {
    /// Normalized hostname, e.g. `"forebrain.local"`.
    pub name: String,
    /// IPv4 address of the host.
    pub ipv4: [u8; 4],
    /// Monotonic-time deadline (in milliseconds) after which the entry
    /// should be evicted. `None` means "never expire" (used for our own
    /// self entry).
    pub expires_at_ms: Option<u64>,
}

impl HostEntry {
    fn body(&self) -> Vec<u8> {
        let mut s =
            alloc::format!("{}.{}.{}.{}\n", self.ipv4[0], self.ipv4[1], self.ipv4[2], self.ipv4[3]);
        // Truncate to a max size just in case.
        s.truncate(64);
        s.into_bytes()
    }

    fn ipv4_string(&self) -> String {
        alloc::format!("{}.{}.{}.{}", self.ipv4[0], self.ipv4[1], self.ipv4[2], self.ipv4[3])
    }
}

/// VFS provider state.
pub struct HostsProvider {
    /// Name → entry. Name is the key for O(log n) lookups on [`lookup_name`].
    entries: BTreeMap<String, HostEntry>,
    /// Stable handle for each name so path lookups return a consistent u64
    /// across calls (required by Readdir/Stat/Close round-trips).
    handles: BTreeMap<String, u64>,
    next_handle: u64,
}

impl Default for HostsProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl HostsProvider {
    pub fn new() -> Self {
        Self { entries: BTreeMap::new(), handles: BTreeMap::new(), next_handle: HANDLE_HOST_BASE }
    }

    /// Insert or update a host. Returns `true` if this is a new entry.
    pub fn upsert(&mut self, entry: HostEntry) -> bool {
        let name = entry.name.clone();
        let is_new = !self.entries.contains_key(&name);
        if is_new {
            let handle = self.next_handle;
            self.next_handle = self.next_handle.wrapping_add(1);
            if self.next_handle < HANDLE_HOST_BASE {
                self.next_handle = HANDLE_HOST_BASE;
            }
            self.handles.insert(name.clone(), handle);
        }
        self.entries.insert(name, entry);
        is_new
    }

    /// Drop every host whose TTL has passed `now_ms`. `None` expiries are
    /// treated as permanent (e.g. our own pinned self-entry).
    ///
    /// Handles are removed alongside entries: if the peer reappears, it
    /// will get a fresh `u64` on the next upsert. Any stale kernel-side
    /// file descriptors pointing at the old handle will cleanly see
    /// `ENOENT`/`EBADF` on their next call, which is the desired
    /// disconnected-peer signal.
    pub fn expire(&mut self, now_ms: u64) {
        let dead: Vec<String> = self
            .entries
            .iter()
            .filter_map(|(name, e)| match e.expires_at_ms {
                Some(exp) if exp <= now_ms => Some(name.clone()),
                _ => None,
            })
            .collect();
        for name in dead {
            self.entries.remove(&name);
            self.handles.remove(&name);
        }
    }

    /// Number of currently-cached hosts.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    fn name_of_handle(&self, handle: u64) -> Option<&str> {
        self.handles
            .iter()
            .find_map(|(name, h)| if *h == handle { Some(name.as_str()) } else { None })
    }

    // ── RPC dispatch ─────────────────────────────────────────────────────

    pub fn handle_lookup(&mut self, path: &str) -> ProviderResponse {
        // Accept both absolute-from-root ("/forebrain.local") and
        // mount-relative ("forebrain.local") paths. The kernel strips the
        // mount prefix before forwarding so we normally see the latter.
        let trimmed = path.trim_matches('/');
        if trimmed.is_empty() {
            return ProviderResponse::ok_u64(HANDLE_ROOT);
        }
        // Only direct children are supported; nested paths don't exist.
        if trimmed.contains('/') {
            return ProviderResponse::err(Errno::ENOENT);
        }
        let key = mdns::normalize_name(trimmed);
        match self.handles.get(&key) {
            Some(h) if self.entries.contains_key(&key) => ProviderResponse::ok_u64(*h),
            _ => ProviderResponse::err(Errno::ENOENT),
        }
    }

    pub fn handle_stat(&self, handle: u64) -> ProviderResponse {
        if handle == HANDLE_ROOT {
            return ProviderResponse::ok_stat(S_IFDIR | 0o555, 0, HANDLE_ROOT);
        }
        let Some(name) = self.name_of_handle(handle) else {
            return ProviderResponse::err(Errno::EBADF);
        };
        let Some(entry) = self.entries.get(name) else {
            return ProviderResponse::err(Errno::ENOENT);
        };
        let size = entry.body().len() as u64;
        ProviderResponse::ok_stat(S_IFREG | 0o444, size, handle)
    }

    pub fn handle_readdir(&self, handle: u64, offset: u64) -> ProviderResponse {
        if handle != HANDLE_ROOT {
            return ProviderResponse::err(Errno::ENOTDIR);
        }
        let mut out: Vec<u8> = Vec::new();
        // Directory-entry wire format matches `iso9660d` so the stock `ls`
        // utility can parse our output:
        //   [ino: u64][file_type: u8][name_len: u8][name bytes]
        // Host names are short, and our caches stay tiny (typically <100
        // entries), so we don't paginate: every call returns the full list
        // starting at `offset`.
        let start_idx = offset as usize;
        for (idx, (name, entry)) in self.entries.iter().enumerate() {
            if idx < start_idx {
                continue;
            }
            let handle = self.handles.get(name).copied().unwrap_or(HANDLE_ROOT);
            let display = entry.name.as_bytes();
            let name_len = display.len().min(255) as u8;
            if name_len == 0 {
                continue;
            }
            out.extend_from_slice(&handle.to_le_bytes());
            out.push(8); // DT_REG — each host is a regular-file entry
            out.push(name_len);
            out.extend_from_slice(&display[..name_len as usize]);
        }
        ProviderResponse::ok_read(&out)
    }

    pub fn handle_read(&self, handle: u64, offset: u64, max_len: u32) -> ProviderResponse {
        if handle == HANDLE_ROOT {
            return ProviderResponse::err(Errno::EISDIR);
        }
        let Some(name) = self.name_of_handle(handle) else {
            return ProviderResponse::err(Errno::EBADF);
        };
        let Some(entry) = self.entries.get(name) else {
            return ProviderResponse::err(Errno::ENOENT);
        };
        let body = entry.body();
        let off = offset as usize;
        if off >= body.len() {
            return ProviderResponse::ok_read(&[]);
        }
        let end = body.len().min(off.saturating_add(max_len as usize));
        ProviderResponse::ok_read(&body[off..end])
    }

    pub fn handle_poll(&self, handle: u64) -> ProviderResponse {
        // The tree is always "ready" in the poll sense: directory listings
        // are synchronous and file reads are satisfied from the cache.
        let _ = handle;
        ProviderResponse::ok_poll(POLLIN)
    }

    pub fn handle_attr_get(&self, handle: u64, name: &str) -> ProviderResponse {
        if handle == HANDLE_ROOT {
            return ProviderResponse::err(Errno::ENOTSUP);
        }
        let Some(key) = self.name_of_handle(handle) else {
            return ProviderResponse::err(Errno::EBADF);
        };
        let Some(entry) = self.entries.get(key) else {
            return ProviderResponse::err(Errno::ENOENT);
        };
        match name {
            "net.ip.ipv4" => {
                let s = entry.ipv4_string();
                ProviderResponse::ok_attr_get(AttrType::Utf8 as u8, s.as_bytes())
            }
            "net.hostname" => {
                ProviderResponse::ok_attr_get(AttrType::Utf8 as u8, entry.name.as_bytes())
            }
            _ => ProviderResponse::err(Errno::ENOENT),
        }
    }

    pub fn handle_attr_list(&self, handle: u64) -> ProviderResponse {
        if handle == HANDLE_ROOT {
            return ProviderResponse::err(Errno::ENOTSUP);
        }
        let Some(key) = self.name_of_handle(handle) else {
            return ProviderResponse::err(Errno::EBADF);
        };
        let Some(entry) = self.entries.get(key) else {
            return ProviderResponse::err(Errno::ENOENT);
        };
        let ipv4 = entry.ipv4_string();
        let entries: [(&str, AttrType, u32); 2] = [
            ("net.ip.ipv4", AttrType::Utf8, ipv4.len() as u32),
            ("net.hostname", AttrType::Utf8, entry.name.len() as u32),
        ];
        let mut out: Vec<u8> = Vec::new();
        for (name, ty, value_len) in entries {
            let hdr = AttrListEntryHeader {
                name_len: name.len() as u16,
                value_type: ty as u8,
                flags: 0,
                value_len,
            };
            out.extend_from_slice(&hdr.name_len.to_le_bytes());
            out.push(hdr.value_type);
            out.push(hdr.flags);
            out.extend_from_slice(&hdr.value_len.to_le_bytes());
            out.extend_from_slice(name.as_bytes());
        }
        ProviderResponse::ok_bytes(&out)
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;

    fn entry(name: &str, ip: [u8; 4]) -> HostEntry {
        HostEntry { name: name.to_string(), ipv4: ip, expires_at_ms: None }
    }

    #[test]
    fn root_lookup_returns_root_handle() {
        let mut p = HostsProvider::new();
        assert_eq!(
            p.handle_lookup("").payload,
            std::vec![] as std::vec::Vec<u8>,
            "trivial payload"
        );
        // ok_u64 payload is 8 bytes little-endian.
        let r = p.handle_lookup("/");
        assert_eq!(r.status, 0);
        assert_eq!(u64::from_le_bytes(r.payload[..8].try_into().unwrap()), HANDLE_ROOT);
    }

    #[test]
    fn known_host_is_looked_up_case_insensitively() {
        let mut p = HostsProvider::new();
        p.upsert(entry("forebrain.local", [192, 168, 2, 23]));
        let r = p.handle_lookup("ForeBrain.Local");
        assert_eq!(r.status, 0);
        let h = u64::from_le_bytes(r.payload[..8].try_into().unwrap());
        // Stat on that handle should report a regular file with non-zero size.
        let s = p.handle_stat(h);
        assert_eq!(s.status, 0);
        let mode = u32::from_le_bytes(s.payload[..4].try_into().unwrap());
        assert_eq!(mode & 0o170_000, S_IFREG);
    }

    #[test]
    fn unknown_host_returns_enoent() {
        let mut p = HostsProvider::new();
        let r = p.handle_lookup("nobody.local");
        assert_eq!(r.status, Errno::ENOENT as u8);
    }

    #[test]
    fn nested_paths_return_enoent() {
        let mut p = HostsProvider::new();
        p.upsert(entry("a.local", [1, 2, 3, 4]));
        let r = p.handle_lookup("a.local/services");
        assert_eq!(r.status, Errno::ENOENT as u8);
    }

    #[test]
    fn attr_get_returns_ipv4_string() {
        let mut p = HostsProvider::new();
        p.upsert(entry("forebrain.local", [192, 168, 2, 23]));
        let lr = p.handle_lookup("forebrain.local");
        let h = u64::from_le_bytes(lr.payload[..8].try_into().unwrap());
        let r = p.handle_attr_get(h, "net.ip.ipv4");
        assert_eq!(r.status, 0);
        // Payload: [val_type][value bytes].
        assert_eq!(r.payload[0], AttrType::Utf8 as u8);
        assert_eq!(&r.payload[1..], b"192.168.2.23");
    }

    #[test]
    fn attr_get_unknown_name_returns_enoent() {
        let mut p = HostsProvider::new();
        p.upsert(entry("forebrain.local", [1, 2, 3, 4]));
        let lr = p.handle_lookup("forebrain.local");
        let h = u64::from_le_bytes(lr.payload[..8].try_into().unwrap());
        let r = p.handle_attr_get(h, "no.such.attr");
        assert_eq!(r.status, Errno::ENOENT as u8);
    }

    #[test]
    fn readdir_enumerates_cached_hosts() {
        let mut p = HostsProvider::new();
        p.upsert(entry("alpha.local", [1, 1, 1, 1]));
        p.upsert(entry("beta.local", [2, 2, 2, 2]));
        let r = p.handle_readdir(HANDLE_ROOT, 0);
        assert_eq!(r.status, 0);
        // Skip the 4-byte length prefix that ok_read adds.
        let body = &r.payload[4..];
        let as_text = std::string::String::from_utf8_lossy(body);
        assert!(as_text.contains("alpha.local"), "got: {as_text}");
        assert!(as_text.contains("beta.local"), "got: {as_text}");
    }

    #[test]
    fn expire_drops_hosts_past_deadline() {
        let mut p = HostsProvider::new();
        p.upsert(HostEntry {
            name: "x.local".into(),
            ipv4: [1, 2, 3, 4],
            expires_at_ms: Some(100),
        });
        p.upsert(HostEntry {
            name: "y.local".into(),
            ipv4: [5, 6, 7, 8],
            expires_at_ms: None, // pinned (e.g. self)
        });
        p.expire(500);
        assert_eq!(p.len(), 1);
        assert!(p.entries.contains_key("y.local"));
        // Handle for the expired host is dropped so we don't leak
        // mappings over a long uptime with many transient peers.
        assert!(!p.handles.contains_key("x.local"));
    }

    #[test]
    fn read_returns_dotted_decimal_body() {
        let mut p = HostsProvider::new();
        p.upsert(entry("selfo.local", [10, 0, 0, 42]));
        let lr = p.handle_lookup("selfo.local");
        let h = u64::from_le_bytes(lr.payload[..8].try_into().unwrap());
        let r = p.handle_read(h, 0, 64);
        assert_eq!(r.status, 0);
        // Skip the ok_read u32 length prefix.
        assert_eq!(&r.payload[4..], b"10.0.0.42\n");
    }
}
