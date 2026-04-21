#![no_std]
#![no_main]
extern crate alloc;

mod cache;
mod cache_mount;
mod xattr;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;

use abi::errors::Errno;
use abi::seed::{
    HOST_PROGRAM, HOST_VFS_PROVIDER, INTERFACE_PROGRAM_V1, INTERFACE_VFS_PROVIDER_MOUNT_V1,
    INTERFACE_VFS_PROVIDER_UNMOUNT_V1, SEED_ABI_VERSION, Seed, SeedInterface,
};
use abi::vfs_rpc::VfsRpcOp;
use http::{HttpClient, Response};
use ipc_helpers::provider::{ProviderLoop, ProviderResponse};
use spin::Mutex;
use stem::syscall::argv_get;
use stem::syscall::vfs::{vfs_mount, vfs_umount};
use stem::{debug, info, trace, warn};

use crate::cache::{
    CacheDirectives, CacheEntry, CacheKey, HttpsCache, canonical_url, resolve_redirect,
};
use crate::cache_mount::CachePathKind;

const MOUNT_POINT: &str = "/https";
const CACHE_MOUNT_POINT: &str = "/run/httpsd/cache";
const ROOT_HANDLE: u64 = 1;
const BODY_WINDOW_CAP: usize = 128 * 1024; // Align with multiple of 16KB chunks
/// Maximum body bytes we retain in the shared cache per entry.  Smaller than
/// the per-handle window because the shared cache is meant for quick
/// re-reads, not full body storage.
const SHARED_BODY_CAP_PER_ENTRY: usize = 64 * 1024;
const HTTPS_PORT_CAPACITY_BYTES: usize = 32_768;
const SEED_NAME: &[u8] = b"httpsd";
const HOOK_MOUNT_V1: &[u8] = b"_start";
const HOOK_UNMOUNT_V1: &[u8] = b"thingos_vfs_unmount_v1";

#[no_mangle]
#[used]
pub static THINGOS_SEED: Seed = Seed {
    abi_version: SEED_ABI_VERSION,
    interface_count: 3,
    hosting_modes: HOST_PROGRAM | HOST_VFS_PROVIDER,
    capabilities: 0,
    name_ptr: SEED_NAME.as_ptr(),
    name_len: SEED_NAME.len(),
    interfaces: [
        SeedInterface {
            interface_id: INTERFACE_PROGRAM_V1,
            interface_version: 1,
            flags: 0,
            reserved: 0,
            entry_symbol_ptr: core::ptr::null(),
            entry_symbol_len: 0,
        },
        SeedInterface {
            interface_id: INTERFACE_VFS_PROVIDER_MOUNT_V1,
            interface_version: 1,
            flags: 0,
            reserved: 0,
            entry_symbol_ptr: HOOK_MOUNT_V1.as_ptr(),
            entry_symbol_len: HOOK_MOUNT_V1.len(),
        },
        SeedInterface {
            interface_id: INTERFACE_VFS_PROVIDER_UNMOUNT_V1,
            interface_version: 1,
            flags: 0,
            reserved: 0,
            entry_symbol_ptr: HOOK_UNMOUNT_V1.as_ptr(),
            entry_symbol_len: HOOK_UNMOUNT_V1.len(),
        },
        SeedInterface::zero(),
    ],
};

// ── /https provider state ──────────────────────────────────────────────────

#[derive(Clone, Debug, Eq, PartialEq)]
struct HttpsNode {
    host: String,
    path: String,
}

impl HttpsNode {
    fn new(host: &str, path: &str) -> Self {
        Self { host: host.to_string(), path: path.to_string() }
    }

    fn url(&self) -> String {
        canonical_url(&self.host, &self.path)
    }

    fn cache_key(&self) -> CacheKey {
        CacheKey::new(&self.host, &self.path)
    }
}

struct HttpsHandle {
    node: HttpsNode,
    response: Option<Response>,
    body: Vec<u8>,
    body_start_offset: usize,
    eof: bool,
    /// When true this handle represents a 3xx redirect that should appear as
    /// a symlink.  Set after the first upstream response head is received.
    is_redirect: bool,
    /// True once headers for this handle have been materialised into the
    /// shared cache.
    headers_cached: bool,
}

impl HttpsHandle {
    fn new(node: HttpsNode, response: Option<Response>) -> Self {
        Self {
            node,
            response,
            body: Vec::new(),
            body_start_offset: 0,
            eof: false,
            is_redirect: false,
            headers_cached: false,
        }
    }

    fn push_chunk(&mut self, chunk: &[u8]) -> Result<(), Errno> {
        self.body.extend_from_slice(chunk);
        if self.body.len() > BODY_WINDOW_CAP {
            let trim = self.body.len() - BODY_WINDOW_CAP;
            self.body.drain(..trim);
            self.body_start_offset =
                self.body_start_offset.checked_add(trim).ok_or(Errno::EOVERFLOW)?;
        }
        Ok(())
    }
}

/// Mutable state shared between both mounts.
struct SharedState {
    cache: Mutex<HttpsCache>,
}

impl SharedState {
    fn new() -> Self {
        Self { cache: Mutex::new(HttpsCache::new()) }
    }
}

struct HttpsProvider {
    next_handle: u64,
    handles: BTreeMap<u64, HttpsHandle>,
    shared: Arc<SharedState>,
}

impl HttpsProvider {
    fn new(shared: Arc<SharedState>) -> Self {
        Self { next_handle: ROOT_HANDLE + 1, handles: BTreeMap::new(), shared }
    }

    fn allocate_node(&mut self, host: &str, path: &str, response: Option<Response>) -> u64 {
        let handle = self.next_handle;
        self.next_handle = self.next_handle.wrapping_add(1);
        if self.next_handle == ROOT_HANDLE {
            self.next_handle = ROOT_HANDLE + 1;
        }
        self.handles.insert(handle, HttpsHandle::new(HttpsNode::new(host, path), response));
        handle
    }

    fn close_node(&mut self, handle: u64) {
        if handle != ROOT_HANDLE {
            self.handles.remove(&handle);
        }
    }

    fn resolve_path(&mut self, path: &str) -> Result<u64, Errno> {
        let clean = path.trim_matches('/');
        if clean.is_empty() {
            debug!("httpsd: lookup '{}' -> root", path);
            return Ok(ROOT_HANDLE);
        }

        let mut parts = clean.split('/');
        let host = parts.next().ok_or(Errno::ENOENT)?;
        if !Self::is_valid_host_label(host) {
            debug!("httpsd: lookup '{}' rejected: invalid host '{}'", path, host);
            return Err(Errno::ENOENT);
        }

        let rest = parts.collect::<Vec<_>>().join("/");
        let node = HttpsNode::new(host, &rest);
        // Keep lookup side-effect free: actual network I/O is deferred to read.
        debug!("httpsd: lookup '{}' -> staging {}", path, node.url());
        let handle = self.allocate_node(host, &rest, None);
        debug!("httpsd: lookup '{}' -> handle {}", path, handle);
        Ok(handle)
    }

    fn is_valid_host_label(host: &str) -> bool {
        host.contains('.')
            && host.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'.' || b == b'-')
    }

    /// Make sure the handle has an upstream response object.  Opening the
    /// upstream stream is deferred from `Lookup` to the first `Read`,
    /// `AttrList` or `AttrGet` so that directory listings don't trigger
    /// network traffic.
    fn ensure_upstream(&mut self, handle: u64) -> Result<(), Errno> {
        let state = self.handles.get_mut(&handle).ok_or(Errno::EBADF)?;
        if state.response.is_some() || state.eof {
            return Ok(());
        }
        let url = state.node.url();
        info!("httpsd: opening upstream stream for handle={} {}", handle, url);
        let response = HttpClient::get(&url).map_err(|err| {
            warn!("httpsd: upstream open failed for handle={} {}: {}", handle, url, err);
            Errno::EIO
        })?;
        state.response = Some(response);
        self.populate_cache_headers(handle);
        Ok(())
    }

    /// Snapshot the response head of a newly-opened upstream response into
    /// the shared cache.  Idempotent per handle.
    fn populate_cache_headers(&mut self, handle: u64) {
        let state = match self.handles.get_mut(&handle) {
            Some(s) => s,
            None => return,
        };
        if state.headers_cached {
            return;
        }
        let Some(response) = state.response.as_ref() else {
            return;
        };
        if response.status() == 0 {
            // Malformed upstream response: don't cache junk.
            return;
        }
        let head = response.head().clone();
        let directives = CacheDirectives::from_head(&head);
        let fetched_at_ns = stem::syscall::monotonic_ns();
        let expires_at_ns = directives.max_age.map(|s| fetched_at_ns.saturating_add(s * 1_000_000_000));
        let is_redirect = head.is_redirect();
        let redirect_target = if is_redirect {
            head.header("Location").map(|loc| resolve_redirect(&state.node.host, &state.node.path, loc))
        } else {
            None
        };
        state.is_redirect = is_redirect;
        state.headers_cached = true;

        let entry = CacheEntry {
            url: state.node.url(),
            host: state.node.host.clone(),
            path: state.node.path.clone(),
            head,
            directives,
            redirect_target,
            fetched_at_ns,
            expires_at_ns,
            body: Vec::new(),
            body_truncated: false,
            hits: 0,
        };
        self.shared.cache.lock().insert(entry);
    }

    /// After `read_chunk` appends new body bytes, mirror them into the
    /// shared cache (capped at `SHARED_BODY_CAP_PER_ENTRY`).
    fn update_cache_body(&self, key: &CacheKey, appended: &[u8]) {
        if appended.is_empty() {
            return;
        }
        let mut cache = self.shared.cache.lock();
        // We use peek (not get) here so the byte accounting we perform below
        // can read the existing bytes, and then reinsert.  The full public
        // API only exposes `insert` for mutation, so we need a clone-replace
        // cycle.
        let Some(existing) = cache.peek(key).cloned() else {
            return;
        };
        let mut new_body = existing.body;
        new_body.extend_from_slice(appended);
        let mut body_truncated = existing.body_truncated;
        if new_body.len() > SHARED_BODY_CAP_PER_ENTRY {
            let drop = new_body.len() - SHARED_BODY_CAP_PER_ENTRY;
            new_body.drain(..drop);
            body_truncated = true;
        }
        let updated = CacheEntry { body: new_body, body_truncated, ..existing };
        cache.insert(updated);
    }

    fn read_node(&mut self, handle: u64, offset: usize, max_len: usize) -> Result<Vec<u8>, Errno> {
        if handle == ROOT_HANDLE {
            return Err(Errno::EISDIR);
        }
        if max_len == 0 {
            return Ok(Vec::new());
        }

        // Snapshot key for cache body updates before the &mut borrows below.
        let key = {
            let state = self.handles.get(&handle).ok_or(Errno::EBADF)?;
            state.node.cache_key()
        };

        // Opening the upstream stream is a `&mut self` op, so do it before we
        // take the per-handle borrow for the streaming read loop.  The call
        // is idempotent: it's a cheap check when the stream is already open.
        self.ensure_upstream(handle)?;

        let Some(state) = self.handles.get_mut(&handle) else {
            return Err(Errno::EBADF);
        };

        info!(
            "httpsd: read handle={} url={} offset={} len={} cached={} start={} eof={}",
            handle,
            state.node.url(),
            offset,
            max_len,
            state.body.len(),
            state.body_start_offset,
            state.eof
        );

        // Redirects are zero-length files on /https (the kernel follows the
        // symlink via Readlink); we still allow reads to produce EOF so
        // tools that `cat` a raw redirect node just see nothing.
        if state.is_redirect {
            return Ok(Vec::new());
        }

        let needed_end = offset.saturating_add(max_len);
        let mut body_end =
            state.body_start_offset.checked_add(state.body.len()).ok_or(Errno::EOVERFLOW)?;

        // Offset reads are constrained to the retained body window.
        if offset < state.body_start_offset {
            debug!(
                "httpsd: read handle={} offset={} before retained window start={} (cap={})",
                handle, offset, state.body_start_offset, BODY_WINDOW_CAP
            );
            return Err(Errno::EINVAL);
        }

        // Streaming reads only fetch until there is data at `offset` (or EOF).
        let mut chunks_to_mirror: Vec<Vec<u8>> = Vec::new();
        while body_end <= offset && !state.eof {
            let Some(response) = state.response.as_mut() else {
                state.eof = true;
                break;
            };

            trace!(
                "httpsd: read_node handle={} calling read_chunk (offset={} end={} eof={})",
                handle, offset, body_end, state.eof
            );
            let chunk = response.read_chunk().map_err(|err| {
                warn!(
                    "httpsd: upstream read failed for handle={} {}: {}",
                    handle,
                    state.node.url(),
                    err
                );
                Errno::EIO
            })?;
            trace!("httpsd: read_chunk handle={} returned {} bytes", handle, chunk.len());
            if chunk.is_empty() {
                info!("httpsd: upstream EOF for handle={} cached={}", handle, state.body.len());
                state.response = None;
                state.eof = true;
                break;
            }
            debug!("httpsd: upstream chunk handle={} bytes={}", handle, chunk.len());
            chunks_to_mirror.push(chunk.clone());
            state.push_chunk(&chunk)?;
            body_end =
                state.body_start_offset.checked_add(state.body.len()).ok_or(Errno::EOVERFLOW)?;
        }

        // Re-check after fetch because the retained window may have advanced while reading chunks.
        if offset < state.body_start_offset {
            debug!(
                "httpsd: read handle={} offset={} evicted while streaming (start={})",
                handle, offset, state.body_start_offset
            );
            return Err(Errno::EINVAL);
        }

        if offset >= body_end {
            // Mirror any chunks we just read into the shared cache before returning.
            for c in &chunks_to_mirror {
                self.update_cache_body(&key, c);
            }
            debug!("httpsd: read handle={} -> EOF at offset {}", handle, offset);
            return Ok(Vec::new());
        }
        let start = offset - state.body_start_offset;
        let end_limit = needed_end.checked_sub(state.body_start_offset).ok_or(Errno::EINVAL)?;
        let end = state.body.len().min(end_limit);
        let out = state.body[start..end].to_vec();
        debug!(
            "httpsd: read handle={} -> returned {} bytes (cached={} start={} eof={})",
            handle,
            out.len(),
            state.body.len(),
            state.body_start_offset,
            state.eof
        );
        for c in &chunks_to_mirror {
            self.update_cache_body(&key, c);
        }
        Ok(out)
    }

    fn stat_node(&self, handle: u64) -> Result<(u32, u64, u64), Errno> {
        if handle == ROOT_HANDLE {
            return Ok((0o040_555, 0, ROOT_HANDLE));
        }
        let state = self.handles.get(&handle).ok_or(Errno::EBADF)?;
        // Consult the shared cache: once we've observed a redirect response
        // for this (host, path), subsequent stats report a symlink so the
        // kernel can follow the `Readlink` chain transparently.
        let is_redirect = state.is_redirect
            || self
                .shared
                .cache
                .lock()
                .peek(&state.node.cache_key())
                .map(|e| e.is_redirect())
                .unwrap_or(false);
        if is_redirect {
            // S_IFLNK (0o120000) with rwx for everyone — the kernel follows
            // the link without checking permissions but tooling like
            // `ls -l` does.
            return Ok((0o120_777, 0, handle));
        }
        Ok((0o100_444, 0, handle))
    }

    /// Ensure the cache has an entry for `handle` so AttrList/AttrGet can
    /// answer without a prior `Read`.
    fn ensure_cached_entry(&mut self, handle: u64) -> Result<CacheEntry, Errno> {
        if handle == ROOT_HANDLE {
            return Err(Errno::EISDIR);
        }
        let key = {
            let state = self.handles.get(&handle).ok_or(Errno::EBADF)?;
            state.node.cache_key()
        };
        if let Some(e) = self.shared.cache.lock().peek(&key) {
            return Ok(e.clone());
        }
        self.ensure_upstream(handle)?;
        self.shared.cache.lock().peek(&key).cloned().ok_or(Errno::EIO)
    }

    fn redirect_target_for(&self, handle: u64) -> Result<String, Errno> {
        if handle == ROOT_HANDLE {
            return Err(Errno::EINVAL);
        }
        let state = self.handles.get(&handle).ok_or(Errno::EBADF)?;
        let key = state.node.cache_key();
        let cache = self.shared.cache.lock();
        // Readlink never triggers an upstream fetch — that would turn every
        // path-component resolution into a network round-trip.  It only
        // reports a target if we've already observed a 3xx response for
        // this URL.
        let entry = cache.peek(&key).ok_or(Errno::EINVAL)?;
        let target = entry.redirect_target.as_ref().ok_or(Errno::EINVAL)?;
        // Translate the absolute `https://host/path` target into a VFS path
        // under our `/https` mount so the kernel can follow the link in-tree.
        Ok(url_to_vfs_path(target))
    }
}

/// Convert an absolute `https://host/path` URL into a VFS path under the
/// `/https` mount.  HTTP URLs are coerced to HTTPS because `httpsd` only
/// serves HTTPS.
fn url_to_vfs_path(url: &str) -> String {
    let rest = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);
    if rest.is_empty() {
        return MOUNT_POINT.to_string();
    }
    let (host, path) = match rest.find('/') {
        Some(idx) => (&rest[..idx], &rest[idx + 1..]),
        None => (rest, ""),
    };
    if path.is_empty() {
        alloc::format!("{}/{}", MOUNT_POINT, host)
    } else {
        alloc::format!("{}/{}/{}", MOUNT_POINT, host, path)
    }
}

// ── /run/httpsd/cache provider state ───────────────────────────────────────

struct CacheFsHandle {
    kind: CachePathKind,
}

struct CacheFsProvider {
    next_handle: u64,
    handles: BTreeMap<u64, CacheFsHandle>,
    shared: Arc<SharedState>,
}

impl CacheFsProvider {
    fn new(shared: Arc<SharedState>) -> Self {
        Self { next_handle: ROOT_HANDLE + 1, handles: BTreeMap::new(), shared }
    }

    fn allocate(&mut self, kind: CachePathKind) -> u64 {
        let handle = self.next_handle;
        self.next_handle = self.next_handle.wrapping_add(1);
        if self.next_handle == ROOT_HANDLE {
            self.next_handle = ROOT_HANDLE + 1;
        }
        self.handles.insert(handle, CacheFsHandle { kind });
        handle
    }

    fn resolve(&mut self, path: &str) -> Result<u64, Errno> {
        let kind = cache_mount::resolve(path);
        // Validate existence for concrete entry paths; root / index /
        // host directories always resolve (empty listings are fine).
        match &kind {
            CachePathKind::EntryDir { host, path } | CachePathKind::EntryLeaf { host, path, .. } => {
                let cache = self.shared.cache.lock();
                if cache.peek(&CacheKey::new(host, path)).is_none() {
                    return Err(Errno::ENOENT);
                }
            }
            _ => {}
        }
        if matches!(kind, CachePathKind::Root) {
            return Ok(ROOT_HANDLE);
        }
        Ok(self.allocate(kind))
    }

    fn close(&mut self, handle: u64) {
        if handle != ROOT_HANDLE {
            self.handles.remove(&handle);
        }
    }

    fn kind_of(&self, handle: u64) -> Result<CachePathKind, Errno> {
        if handle == ROOT_HANDLE {
            return Ok(CachePathKind::Root);
        }
        self.handles.get(&handle).map(|h| h.kind.clone()).ok_or(Errno::EBADF)
    }

    fn stat(&self, handle: u64) -> Result<(u32, u64, u64), Errno> {
        match self.kind_of(handle)? {
            CachePathKind::Root | CachePathKind::HostDir { .. } | CachePathKind::EntryDir { .. } => {
                Ok((0o040_555, 0, handle))
            }
            CachePathKind::Index => {
                let bytes = cache_mount::render_index(&self.shared.cache.lock());
                Ok((0o100_444, bytes.len() as u64, handle))
            }
            CachePathKind::EntryLeaf { host, path, leaf } => {
                let cache = self.shared.cache.lock();
                let entry = cache.peek(&CacheKey::new(&host, &path)).ok_or(Errno::ENOENT)?;
                let bytes = leaf.render(entry);
                Ok((0o100_444, bytes.len() as u64, handle))
            }
        }
    }

    fn read(&self, handle: u64, offset: usize, max_len: usize) -> Result<Vec<u8>, Errno> {
        if max_len == 0 {
            return Ok(Vec::new());
        }
        let data = match self.kind_of(handle)? {
            CachePathKind::Root | CachePathKind::HostDir { .. } | CachePathKind::EntryDir { .. } => {
                return Err(Errno::EISDIR);
            }
            CachePathKind::Index => cache_mount::render_index(&self.shared.cache.lock()),
            CachePathKind::EntryLeaf { host, path, leaf } => {
                let cache = self.shared.cache.lock();
                let entry = cache.peek(&CacheKey::new(&host, &path)).ok_or(Errno::ENOENT)?;
                leaf.render(entry)
            }
        };
        if offset >= data.len() {
            return Ok(Vec::new());
        }
        let end = data.len().min(offset.saturating_add(max_len));
        Ok(data[offset..end].to_vec())
    }

    fn attr_list(&self, handle: u64) -> Result<Vec<u8>, Errno> {
        let kind = self.kind_of(handle)?;
        let (host, path) = match kind {
            CachePathKind::EntryDir { host, path } | CachePathKind::EntryLeaf { host, path, .. } => {
                (host, path)
            }
            _ => return Ok(Vec::new()),
        };
        let cache = self.shared.cache.lock();
        let entry = cache.peek(&CacheKey::new(&host, &path)).ok_or(Errno::ENOENT)?;
        let attrs = xattr::entry_attrs(entry);
        Ok(xattr::serialize_attr_list(&attrs))
    }

    fn attr_get(&self, handle: u64, name: &str) -> Result<(u8, Vec<u8>), Errno> {
        let kind = self.kind_of(handle)?;
        let (host, path) = match kind {
            CachePathKind::EntryDir { host, path } | CachePathKind::EntryLeaf { host, path, .. } => {
                (host, path)
            }
            _ => return Err(Errno::ENOENT),
        };
        let cache = self.shared.cache.lock();
        let entry = cache.peek(&CacheKey::new(&host, &path)).ok_or(Errno::ENOENT)?;
        let attr = xattr::entry_attr(entry, name).ok_or(Errno::ENOENT)?;
        Ok((attr.ty as u8, attr.value))
    }
}

// ── Entry points ───────────────────────────────────────────────────────────

#[stem::main]
fn main(_arg: usize) -> ! {
    let mount_point = mount_point_from_args();
    run_provider(&mount_point)
}

#[used]
static KEEP_THINGOS_VFS_MOUNT_V1: extern "C" fn(usize) -> ! = thingos_vfs_mount_v1;

#[used]
static KEEP_THINGOS_VFS_UNMOUNT_V1: extern "C" fn(usize) -> i32 = thingos_vfs_unmount_v1;

#[no_mangle]
pub extern "C" fn thingos_vfs_mount_v1(arg: usize) -> ! {
    unsafe { stem::rt::entry_impl(arg) }
}

#[no_mangle]
pub extern "C" fn thingos_vfs_unmount_v1(_arg: usize) -> i32 {
    let mount_point = mount_point_from_args();
    match vfs_umount(&mount_point) {
        Ok(()) => {}
        Err(_) => return 1,
    }
    // Best-effort: the secondary cache mount may not be present if setup failed.
    let _ = vfs_umount(CACHE_MOUNT_POINT);
    0
}

fn run_provider(mount_point: &str) -> ! {
    let shared = Arc::new(SharedState::new());

    // Spawn the cache-mount loop first; a failure to set up the secondary
    // mount should be logged but must not prevent the primary /https mount
    // from running (operators can still use /https + xattrs).
    match spawn_cache_mount(shared.clone()) {
        Ok(()) => debug!("httpsd: cache mount started at {}", CACHE_MOUNT_POINT),
        Err(e) => warn!("httpsd: failed to start cache mount at {}: {:?}", CACHE_MOUNT_POINT, e),
    }

    run_https_mount(mount_point, shared)
}

fn spawn_cache_mount(shared: Arc<SharedState>) -> Result<(), Errno> {
    let (req_write, req_read) = stem::syscall::port::port_create(HTTPS_PORT_CAPACITY_BYTES)?;
    if let Err(e) = vfs_mount(req_write, CACHE_MOUNT_POINT) {
        warn!("httpsd: failed to mount cache at {}: {:?}", CACHE_MOUNT_POINT, e);
        return Err(e);
    }
    stem::thread::spawn_task_detached(move || {
        let mut provider = CacheFsProvider::new(shared);
        let mut lp = ProviderLoop::new(req_read);
        loop {
            let req = match lp.next_request() {
                Ok(r) => r,
                Err(e) => {
                    warn!("httpsd: cache loop ended: {:?}", e);
                    break;
                }
            };
            let resp = dispatch_cache(&mut provider, req.op, &req.payload);
            send_response(&lp, req.resp_port, resp);
        }
    })?;
    Ok(())
}

fn run_https_mount(mount_point: &str, shared: Arc<SharedState>) -> ! {
    let (req_write, req_read) = match stem::syscall::port::port_create(HTTPS_PORT_CAPACITY_BYTES) {
        Ok(pair) => pair,
        Err(e) => {
            warn!("httpsd: port_create failed: {:?}", e);
            stem::syscall::exit(1);
        }
    };

    match vfs_mount(req_write, mount_point) {
        Ok(()) => debug!("httpsd: mounted at {}", mount_point),
        Err(e) => {
            warn!("httpsd: failed to mount {}: {:?}", mount_point, e);
            stem::syscall::exit(1);
        }
    }

    let mut provider = HttpsProvider::new(shared);
    let mut lp = ProviderLoop::new(req_read);
    loop {
        let req = match lp.next_request() {
            Ok(r) => r,
            Err(e) => {
                warn!("httpsd: provider loop ended: {:?}", e);
                break;
            }
        };
        let resp = dispatch(&mut provider, req.op, &req.payload);
        send_response(&lp, req.resp_port, resp);
    }

    stem::syscall::exit(0);
}

fn send_response(lp: &ProviderLoop, resp_port: u32, resp: ProviderResponse) {
    loop {
        match lp.send_response(resp_port, resp.clone()) {
            Ok(_) => return,
            Err(Errno::EAGAIN) => {
                stem::syscall::yield_now();
                continue;
            }
            Err(e) => {
                warn!("httpsd: failed to send response: {:?}", e);
                return;
            }
        }
    }
}

fn mount_point_from_args() -> String {
    let len = match argv_get(&mut []) {
        Ok(l) if l > 0 => l,
        _ => return MOUNT_POINT.to_string(),
    };
    let mut buf = alloc::vec![0u8; len];
    if argv_get(&mut buf).is_err() {
        return MOUNT_POINT.to_string();
    }
    let args = stem::utils::parse_argv(&buf);
    if args.len() >= 2 {
        if let Ok(path) = core::str::from_utf8(args[1]) {
            if !path.is_empty() {
                return path.to_string();
            }
        }
    }
    MOUNT_POINT.to_string()
}

// ── RPC dispatch — /https mount ────────────────────────────────────────────

fn dispatch(provider: &mut HttpsProvider, op: VfsRpcOp, payload: &[u8]) -> ProviderResponse {
    debug!("httpsd: rpc {:?} payload_len={}", op, payload.len());
    match op {
        VfsRpcOp::Lookup => dispatch_lookup(provider, payload),
        VfsRpcOp::Read => dispatch_read(provider, payload),
        VfsRpcOp::Stat => dispatch_stat(provider, payload),
        VfsRpcOp::Readdir => dispatch_readdir(payload),
        VfsRpcOp::Close => dispatch_close(provider, payload),
        VfsRpcOp::SubscribeReady | VfsRpcOp::UnsubscribeReady => ProviderResponse::ok_empty(),
        VfsRpcOp::Poll => ProviderResponse::ok_poll(abi::syscall::poll_flags::POLLIN as u32),
        VfsRpcOp::AttrList => dispatch_attr_list(provider, payload),
        VfsRpcOp::AttrGet => dispatch_attr_get(provider, payload),
        VfsRpcOp::AttrSet | VfsRpcOp::AttrRemove => ProviderResponse::err(Errno::EROFS),
        VfsRpcOp::Readlink => dispatch_readlink(provider, payload),
        _ => ProviderResponse::err(Errno::ENOSYS),
    }
}

fn dispatch_lookup(provider: &mut HttpsProvider, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 4 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let path_len = u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]) as usize;
    if payload.len() < 4 + path_len {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let Ok(path) = core::str::from_utf8(&payload[4..4 + path_len]) else {
        return ProviderResponse::err(Errno::EINVAL);
    };
    match provider.resolve_path(path) {
        Ok(handle) => ProviderResponse::ok_u64(handle),
        Err(e) => ProviderResponse::err(e),
    }
}

fn dispatch_read(provider: &mut HttpsProvider, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 20 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
    let offset = u64::from_le_bytes(payload[8..16].try_into().unwrap_or([0; 8])) as usize;
    let max_len = u32::from_le_bytes(payload[16..20].try_into().unwrap_or([0; 4])) as usize;

    let data = match provider.read_node(handle, offset, max_len) {
        Ok(d) => d,
        Err(e) => return ProviderResponse::err(e),
    };
    ProviderResponse::ok_read(&data)
}

fn dispatch_stat(provider: &HttpsProvider, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 8 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
    match provider.stat_node(handle) {
        Ok((mode, size, ino)) => ProviderResponse::ok_stat(mode, size, ino),
        Err(e) => ProviderResponse::err(e),
    }
}

fn dispatch_close(provider: &mut HttpsProvider, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 8 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
    provider.close_node(handle);
    ProviderResponse::ok_empty()
}

fn dispatch_readdir(payload: &[u8]) -> ProviderResponse {
    if payload.len() < 20 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let _handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
    ProviderResponse::ok_read(&[])
}

fn dispatch_attr_list(provider: &mut HttpsProvider, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 8 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
    let entry = match provider.ensure_cached_entry(handle) {
        Ok(e) => e,
        Err(e) => return ProviderResponse::err(e),
    };
    let attrs = xattr::entry_attrs(&entry);
    ProviderResponse::ok_bytes(&xattr::serialize_attr_list(&attrs))
}

fn dispatch_attr_get(provider: &mut HttpsProvider, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 10 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
    let name_len = u16::from_le_bytes([payload[8], payload[9]]) as usize;
    if payload.len() < 10 + name_len {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let name = match core::str::from_utf8(&payload[10..10 + name_len]) {
        Ok(s) => s,
        Err(_) => return ProviderResponse::err(Errno::EINVAL),
    };
    let entry = match provider.ensure_cached_entry(handle) {
        Ok(e) => e,
        Err(e) => return ProviderResponse::err(e),
    };
    match xattr::entry_attr(&entry, name) {
        Some(attr) => ProviderResponse::ok_attr_get(attr.ty as u8, &attr.value),
        None => ProviderResponse::err(Errno::ENOENT),
    }
}

fn dispatch_readlink(provider: &mut HttpsProvider, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 8 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
    // Readlink is a cheap cache-only lookup: we explicitly do NOT trigger
    // an upstream fetch here, because the kernel calls `readlink()` on
    // every path component during resolution and each fetch would be an
    // HTTPS round-trip.  The symlink view only activates once a previous
    // `Read`/`AttrGet` has populated the cache with the redirect metadata.
    match provider.redirect_target_for(handle) {
        Ok(target) => ProviderResponse::ok_readlink(&target),
        Err(e) => ProviderResponse::err(e),
    }
}

// ── RPC dispatch — /run/httpsd/cache mount ─────────────────────────────────

fn dispatch_cache(
    provider: &mut CacheFsProvider,
    op: VfsRpcOp,
    payload: &[u8],
) -> ProviderResponse {
    match op {
        VfsRpcOp::Lookup => {
            if payload.len() < 4 {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let path_len =
                u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]) as usize;
            if payload.len() < 4 + path_len {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let Ok(path) = core::str::from_utf8(&payload[4..4 + path_len]) else {
                return ProviderResponse::err(Errno::EINVAL);
            };
            match provider.resolve(path) {
                Ok(h) => ProviderResponse::ok_u64(h),
                Err(e) => ProviderResponse::err(e),
            }
        }
        VfsRpcOp::Stat => {
            if payload.len() < 8 {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
            match provider.stat(handle) {
                Ok((mode, size, ino)) => ProviderResponse::ok_stat(mode, size, ino),
                Err(e) => ProviderResponse::err(e),
            }
        }
        VfsRpcOp::Read => {
            if payload.len() < 20 {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
            let offset = u64::from_le_bytes(payload[8..16].try_into().unwrap_or([0; 8])) as usize;
            let max_len = u32::from_le_bytes(payload[16..20].try_into().unwrap_or([0; 4])) as usize;
            match provider.read(handle, offset, max_len) {
                Ok(d) => ProviderResponse::ok_read(&d),
                Err(e) => ProviderResponse::err(e),
            }
        }
        VfsRpcOp::Readdir => ProviderResponse::ok_read(&[]),
        VfsRpcOp::Close => {
            if payload.len() < 8 {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
            provider.close(handle);
            ProviderResponse::ok_empty()
        }
        VfsRpcOp::Poll => ProviderResponse::ok_poll(abi::syscall::poll_flags::POLLIN as u32),
        VfsRpcOp::AttrList => {
            if payload.len() < 8 {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
            match provider.attr_list(handle) {
                Ok(b) => ProviderResponse::ok_bytes(&b),
                Err(e) => ProviderResponse::err(e),
            }
        }
        VfsRpcOp::AttrGet => {
            if payload.len() < 10 {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
            let name_len = u16::from_le_bytes([payload[8], payload[9]]) as usize;
            if payload.len() < 10 + name_len {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let Ok(name) = core::str::from_utf8(&payload[10..10 + name_len]) else {
                return ProviderResponse::err(Errno::EINVAL);
            };
            match provider.attr_get(handle, name) {
                Ok((ty, bytes)) => ProviderResponse::ok_attr_get(ty, &bytes),
                Err(e) => ProviderResponse::err(e),
            }
        }
        VfsRpcOp::AttrSet | VfsRpcOp::AttrRemove => ProviderResponse::err(Errno::EROFS),
        VfsRpcOp::SubscribeReady | VfsRpcOp::UnsubscribeReady => ProviderResponse::ok_empty(),
        _ => ProviderResponse::err(Errno::ENOSYS),
    }
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use alloc::vec;

    use abi::errors::Errno;

    use super::*;
    extern crate std;

    fn new_provider() -> HttpsProvider {
        HttpsProvider::new(Arc::new(SharedState::new()))
    }

    #[test]
    fn host_validation_requires_domain_shape() {
        assert!(HttpsProvider::is_valid_host_label("en.wikipedia.org"));
        assert!(!HttpsProvider::is_valid_host_label("localhost"));
        assert!(!HttpsProvider::is_valid_host_label("bad host"));
    }

    #[test]
    fn node_url_builds_https_paths() {
        let node = HttpsNode::new("example.com", "index.html");
        assert_eq!(node.url(), "https://example.com/index.html");
    }

    #[test]
    fn read_node_rejects_offsets_before_retained_window() {
        let mut provider = new_provider();
        let handle = provider.allocate_node("example.com", "", None);
        let state = provider.handles.get_mut(&handle).expect("allocated handle");
        state.body.extend_from_slice(b"abcdef");
        state.body_start_offset = 3;
        state.eof = true;

        assert_eq!(provider.read_node(handle, 2, 4), Err(Errno::EINVAL));
    }

    #[test]
    fn read_node_maps_offsets_within_retained_window() {
        let mut provider = new_provider();
        let handle = provider.allocate_node("example.com", "", None);
        let state = provider.handles.get_mut(&handle).expect("allocated handle");
        state.body.extend_from_slice(b"abcdef");
        state.body_start_offset = 3;
        state.eof = true;

        assert_eq!(provider.read_node(handle, 4, 3).expect("windowed read"), b"bcd");
        assert_eq!(provider.read_node(handle, 9, 8).expect("eof read"), b"");
    }

    #[test]
    fn push_chunk_trims_to_bounded_window() {
        let mut handle = HttpsHandle::new(HttpsNode::new("example.com", ""), None);
        let chunk = vec![7u8; BODY_WINDOW_CAP + 5];
        handle.push_chunk(&chunk).expect("chunk push");
        assert_eq!(handle.body.len(), BODY_WINDOW_CAP);
        assert_eq!(handle.body_start_offset, 5);
        assert!(handle.body.iter().all(|b| *b == 7));
    }

    #[test]
    fn url_to_vfs_path_builds_under_https_mount() {
        assert_eq!(
            url_to_vfs_path("https://example.com/index.html"),
            "/https/example.com/index.html"
        );
        assert_eq!(url_to_vfs_path("https://example.com"), "/https/example.com");
        assert_eq!(
            url_to_vfs_path("http://example.com/legacy"),
            "/https/example.com/legacy"
        );
    }

    #[test]
    fn redirect_handle_reports_symlink_mode() {
        let mut provider = new_provider();
        let handle = provider.allocate_node("a.example", "old", None);
        // Simulate a cached redirect entry for this handle.
        {
            let head = http::ResponseHead::parse(
                b"HTTP/1.1 301 Moved Permanently\r\nLocation: https://b.example/new\r\n\r\n",
            );
            let directives = CacheDirectives::from_head(&head);
            let entry = CacheEntry {
                url: "https://a.example/old".into(),
                host: "a.example".into(),
                path: "old".into(),
                head,
                directives,
                redirect_target: Some("https://b.example/new".into()),
                fetched_at_ns: 0,
                expires_at_ns: None,
                body: Vec::new(),
                body_truncated: false,
                hits: 0,
            };
            provider.shared.cache.lock().insert(entry);
            provider.handles.get_mut(&handle).unwrap().is_redirect = true;
            provider.handles.get_mut(&handle).unwrap().headers_cached = true;
        }
        let (mode, _, _) = provider.stat_node(handle).unwrap();
        // S_IFLNK = 0o120000
        assert_eq!(mode & 0o170_000, 0o120_000);
        assert_eq!(
            provider.redirect_target_for(handle).unwrap(),
            "/https/b.example/new"
        );
    }

    #[test]
    fn cache_provider_resolves_index_and_entry_paths() {
        let shared = Arc::new(SharedState::new());
        // Seed one entry.
        let head = http::ResponseHead::parse(b"HTTP/1.1 200 OK\r\n\r\n");
        let directives = CacheDirectives::from_head(&head);
        shared.cache.lock().insert(CacheEntry {
            url: "https://example.com/x".into(),
            host: "example.com".into(),
            path: "x".into(),
            head,
            directives,
            redirect_target: None,
            fetched_at_ns: 0,
            expires_at_ns: None,
            body: b"hi".to_vec(),
            body_truncated: false,
            hits: 0,
        });
        let mut p = CacheFsProvider::new(shared);
        let idx_handle = p.resolve("/index").expect("index resolves");
        let (mode, size, _) = p.stat(idx_handle).unwrap();
        assert_eq!(mode & 0o170_000, 0o100_000);
        assert!(size > 0);
        let bytes = p.read(idx_handle, 0, 4096).unwrap();
        assert!(core::str::from_utf8(&bytes).unwrap().contains("example.com"));

        let headers_h = p.resolve("/example.com/x/headers").expect("headers resolves");
        let bytes = p.read(headers_h, 0, 4096).unwrap();
        assert!(bytes.starts_with(b"HTTP/1.1 200 OK"));

        let missing = p.resolve("/example.com/does-not-exist/headers");
        assert_eq!(missing, Err(Errno::ENOENT));
    }

    #[test]
    fn cache_provider_exposes_xattrs_on_entries() {
        let shared = Arc::new(SharedState::new());
        let head = http::ResponseHead::parse(
            b"HTTP/1.1 200 OK\r\nETag: \"z\"\r\nContent-Type: text/plain\r\n\r\n",
        );
        let directives = CacheDirectives::from_head(&head);
        shared.cache.lock().insert(CacheEntry {
            url: "https://example.com/x".into(),
            host: "example.com".into(),
            path: "x".into(),
            head,
            directives,
            redirect_target: None,
            fetched_at_ns: 0,
            expires_at_ns: None,
            body: Vec::new(),
            body_truncated: false,
            hits: 0,
        });
        let mut p = CacheFsProvider::new(shared);
        let h = p.resolve("/example.com/x/headers").unwrap();
        let (ty, value) = p.attr_get(h, "user.http.etag").unwrap();
        assert_eq!(ty, abi::attrs::AttrType::Utf8 as u8);
        assert_eq!(value, b"\"z\"");
    }
}
