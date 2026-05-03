#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
extern crate alloc;

mod cache;
mod cache_mount;
mod xattr;

use alloc::collections::{BTreeMap, VecDeque};
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
use stem::{debug, error, info, trace, warn};

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

// ── Per-handle worker thread infrastructure ────────────────────────────────

/// Message forwarded to a per-handle worker thread via [`WorkerChannel`].
enum WorkerMsg {
    Read { resp_port: u32, req_id: u16, offset: usize, max_len: usize },
    Close,
}

/// Lock-based FIFO channel for routing RPC messages to a handle's worker thread.
struct WorkerChannel {
    queue: spin::Mutex<VecDeque<WorkerMsg>>,
}

impl WorkerChannel {
    fn new() -> Self {
        Self { queue: spin::Mutex::new(VecDeque::new()) }
    }

    fn push(&self, msg: WorkerMsg) {
        self.queue.lock().push_back(msg);
    }

    fn pop(&self) -> Option<WorkerMsg> {
        self.queue.lock().pop_front()
    }
}

/// Lightweight per-handle state kept by the main loop after the handle has
/// been promoted to a worker thread.  The full `HttpsHandle` is owned by
/// the worker; the main loop only retains what it needs for `Stat`,
/// `AttrList`, `AttrGet`, and `Readlink`.
struct WorkerHandle {
    /// Preserved for cache-key lookups by non-streaming RPC ops.
    node: HttpsNode,
    channel: Arc<WorkerChannel>,
}

struct HttpsProvider {
    next_handle: u64,
    /// Handles waiting for their first `Read`; owned exclusively by the main loop.
    handles: BTreeMap<u64, HttpsHandle>,
    /// Handles with active worker threads; main loop retains node metadata + channel.
    workers: BTreeMap<u64, WorkerHandle>,
    shared: Arc<SharedState>,
    fixed_host: Option<String>,
    mount_point: String,
}

impl HttpsProvider {
    fn new(shared: Arc<SharedState>, fixed_host: Option<String>, mount_point: String) -> Self {
        Self {
            next_handle: ROOT_HANDLE + 1,
            handles: BTreeMap::new(),
            workers: BTreeMap::new(),
            shared,
            fixed_host,
            mount_point,
        }
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
            if let Some(wh) = self.workers.remove(&handle) {
                // Signal the worker thread to shut down.
                wh.channel.push(WorkerMsg::Close);
            } else {
                self.handles.remove(&handle);
            }
        }
    }

    /// Promote an unstarted handle to a dedicated worker thread.
    ///
    /// The `HttpsHandle` is moved into the new thread; the main loop retains
    /// only a [`WorkerHandle`] (node metadata + channel).  Returns the
    /// channel so the caller can immediately enqueue the first message.
    fn promote_to_worker(&mut self, handle: u64) -> Result<Arc<WorkerChannel>, Errno> {
        if let Some(wh) = self.workers.get(&handle) {
            return Ok(wh.channel.clone());
        }

        let h = match self.handles.remove(&handle) {
            Some(h) => h,
            None => return Err(Errno::EBADF),
        };

        let node = h.node.clone();
        let channel = Arc::new(WorkerChannel::new());
        let shared = self.shared.clone();
        let ch_clone = channel.clone();
        let mount_point = self.mount_point.clone();

        match stem::thread::spawn_task_detached(move || {
            run_handle_worker(handle, h, ch_clone, shared, mount_point);
        }) {
            Ok(_) => {
                self.workers.insert(handle, WorkerHandle { node, channel: channel.clone() });
                Ok(channel)
            }
            Err(e) => {
                warn!(
                    "httpsd: failed to spawn worker for handle={}: {:?} \
                     (check system thread/task limits; this handle will not stream concurrently)",
                    handle, e
                );
                // The handle has been moved into the closure and cannot be recovered.
                // Return EIO so the caller can propagate an error to the client.
                Err(Errno::EIO)
            }
        }
    }

    fn resolve_path(&mut self, path: &str) -> Result<u64, Errno> {
        trace!("httpsd: resolve_path path='{}'", path);
        let clean = path.trim_matches('/');
        if clean.is_empty() {
            debug!("httpsd: lookup '{}' -> root", path);
            return Ok(ROOT_HANDLE);
        }

        // Strip the virtual /@index suffix.  `/@index` is an identity
        // operation: it resolves to "the document at this URL path itself",
        // which avoids the UNIX file-vs-directory ambiguity.
        let clean = if clean == "@index" {
            ""
        } else if let Some(prefix) = clean.strip_suffix("/@index") {
            prefix
        } else {
            clean
        };

        if clean.is_empty() {
            debug!("httpsd: lookup '{}' -> root (via @index)", path);
            return Ok(ROOT_HANDLE);
        }

        let (host_owned, rest_str): (String, String) = if let Some(fixed) = &self.fixed_host {
            (fixed.clone(), clean.to_string())
        } else {
            let mut parts = clean.split('/');
            let host = parts.next().ok_or(Errno::ENOENT)?;
            if !Self::is_valid_host_label(host) {
                debug!("httpsd: lookup '{}' rejected: invalid host '{}'", path, host);
                return Err(Errno::ENOENT);
            }
            let rest = parts.collect::<Vec<_>>().join("/");
            (host.to_string(), rest)
        };
        let host = &host_owned;

        // Keep lookup side-effect free: actual network I/O is deferred to read.
        let node = HttpsNode::new(host, &rest_str);
        debug!("httpsd: lookup '{}' -> staging {}", path, node.url());
        let handle = self.allocate_node(host, &rest_str, None);
        trace!("httpsd: lookup '{}' -> handle {}", path, handle);
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
        ensure_upstream_on(handle, state, &self.shared)
    }

    fn read_node(&mut self, handle: u64, offset: usize, max_len: usize) -> Result<Vec<u8>, Errno> {
        if handle == ROOT_HANDLE {
            return Err(Errno::EISDIR);
        }
        if max_len == 0 {
            return Ok(Vec::new());
        }
        let state = self.handles.get_mut(&handle).ok_or(Errno::EBADF)?;
        perform_read(handle, state, &self.shared, offset, max_len)
    }

    fn stat_node(&self, handle: u64) -> Result<(u32, u64, u64), Errno> {
        if handle == ROOT_HANDLE {
            return Ok((0o040_555, 0, ROOT_HANDLE));
        }
        // Resolve (is_redirect hint, cache key) from whichever map holds this handle.
        let (is_redirect_local, cache_key) = if let Some(state) = self.handles.get(&handle) {
            (state.is_redirect, state.node.cache_key())
        } else if let Some(wh) = self.workers.get(&handle) {
            (false, wh.node.cache_key())
        } else {
            return Err(Errno::EBADF);
        };
        // Consult the shared cache: once we've observed a redirect response
        // for this (host, path), subsequent stats report a symlink so the
        // kernel can follow the `Readlink` chain transparently.
        let is_redirect = is_redirect_local
            || self.shared.cache.lock().peek(&cache_key).map(|e| e.is_redirect()).unwrap_or(false);
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
        let key = if let Some(state) = self.handles.get(&handle) {
            state.node.cache_key()
        } else if let Some(wh) = self.workers.get(&handle) {
            wh.node.cache_key()
        } else {
            return Err(Errno::EBADF);
        };
        debug!("httpsd: ensure_cached_entry handle={} key={:?}", handle, key);
        if let Some(e) = self.shared.cache.lock().peek(&key) {
            return Ok(e.clone());
        }
        // If the handle has been promoted to a worker, the main loop can no
        // longer call ensure_upstream (the worker owns the Response).  The
        // shared cache will be populated after the worker completes its first
        // Read chunk — callers should retry the AttrList/AttrGet after
        // issuing at least one Read on this handle.
        if self.workers.contains_key(&handle) {
            error!(
                "httpsd: ensure_cached_entry handle={} worker not yet cached \
                 (retry after first Read completes)",
                handle
            );
            return Err(Errno::EIO);
        }
        self.ensure_upstream(handle)?;
        let entry = self.shared.cache.lock().peek(&key).cloned().ok_or_else(|| {
            error!(
                "httpsd: ensure_cached_entry handle={} FAILED: not in cache after ensure_upstream",
                handle
            );
            Errno::EIO
        })?;
        Ok(entry)
    }

    fn redirect_target_for(&self, handle: u64) -> Result<String, Errno> {
        if handle == ROOT_HANDLE {
            return Err(Errno::EINVAL);
        }
        let cache_key = if let Some(state) = self.handles.get(&handle) {
            state.node.cache_key()
        } else if let Some(wh) = self.workers.get(&handle) {
            wh.node.cache_key()
        } else {
            return Err(Errno::EBADF);
        };
        let cache = self.shared.cache.lock();
        // Readlink never triggers an upstream fetch — that would turn every
        // path-component resolution into a network round-trip.  It only
        // reports a target if we've already observed a 3xx response for
        // this URL.
        let entry = cache.peek(&cache_key).ok_or(Errno::EINVAL)?;
        let target = entry.redirect_target.as_ref().ok_or(Errno::EINVAL)?;
        // Translate the absolute `https://host/path` target into a VFS path
        // under the active mount so the kernel can follow the link in-tree.
        Ok(self.url_to_vfs_path(target))
    }
}

/// Convert an absolute `https://host/path` URL into a VFS path under the
/// mount.  HTTP URLs are coerced to HTTPS because `httpsd` only
/// serves HTTPS.
impl HttpsProvider {
    fn url_to_vfs_path(&self, url: &str) -> String {
        let rest =
            url.strip_prefix("https://").or_else(|| url.strip_prefix("http://")).unwrap_or(url);
        if rest.is_empty() {
            return self.mount_point.clone();
        }
        let (host, path) = match rest.find('/') {
            Some(idx) => (&rest[..idx], &rest[idx + 1..]),
            None => (rest, ""),
        };

        if let Some(fixed) = &self.fixed_host {
            if host == fixed {
                if path.is_empty() {
                    return self.mount_point.clone();
                } else {
                    return alloc::format!("{}/{}", self.mount_point, path);
                }
            }
        }

        // Fallback for cross-domain redirects or discovery-mode paths.
        // Use the active mount point instead of hardcoding `/https` so custom
        // mount targets (for example `/web`) produce in-tree redirect targets.
        if path.is_empty() {
            alloc::format!("{}/{}", self.mount_point, host)
        } else {
            alloc::format!("{}/{}/{}", self.mount_point, host, path)
        }
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
            CachePathKind::EntryDir { host, path }
            | CachePathKind::EntryLeaf { host, path, .. } => {
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
            CachePathKind::Root
            | CachePathKind::HostDir { .. }
            | CachePathKind::EntryDir { .. } => Ok((0o040_555, 0, handle)),
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
            CachePathKind::Root
            | CachePathKind::HostDir { .. }
            | CachePathKind::EntryDir { .. } => {
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
            CachePathKind::EntryDir { host, path }
            | CachePathKind::EntryLeaf { host, path, .. } => (host, path),
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
            CachePathKind::EntryDir { host, path }
            | CachePathKind::EntryLeaf { host, path, .. } => (host, path),
            _ => return Err(Errno::ENOENT),
        };
        let cache = self.shared.cache.lock();
        let entry = cache.peek(&CacheKey::new(&host, &path)).ok_or(Errno::ENOENT)?;
        let attr = xattr::entry_attr(entry, name).ok_or(Errno::ENOENT)?;
        Ok((attr.ty as u8, attr.value))
    }
}

// ── Standalone streaming helpers used by both the main loop and worker threads ──

/// Open (or verify) the upstream HTTP/S connection for `state`.
///
/// Idempotent: returns immediately if the stream is already open, the handle
/// has already reached EOF, or the headers have already been cached from a
/// previous fetch.
fn ensure_upstream_on(
    handle_id: u64,
    state: &mut HttpsHandle,
    shared: &SharedState,
) -> Result<(), Errno> {
    if state.response.is_some() || state.eof || state.headers_cached {
        return Ok(());
    }

    // Check shared cache first to avoid redundant network I/O for xattr/stat calls.
    let key = state.node.cache_key();
    if let Some(entry) = shared.cache.lock().peek(&key) {
        debug!("httpsd: ensure_upstream handle={} FOUND {} in cache", handle_id, state.node.url());
        state.is_redirect = entry.is_redirect();
        state.headers_cached = true;
        if state.is_redirect {
            state.eof = true;
        }
        return Ok(());
    }

    let url = state.node.url();
    debug!("httpsd: ensure_upstream handle={} MISS {} - opening network stream", handle_id, url);
    let response = HttpClient::get(&url).map_err(|err| {
        error!("httpsd: upstream open failed for handle={} {}: {}", handle_id, url, err);
        Errno::EIO
    })?;
    state.response = Some(response);
    populate_cache_headers_on(state, shared);
    Ok(())
}

/// Snapshot the response head of a newly-opened upstream response into the
/// shared cache.  Idempotent.
fn populate_cache_headers_on(state: &mut HttpsHandle, shared: &SharedState) {
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
    shared.cache.lock().insert(entry);
}

/// Mirror newly-read body bytes into the shared cache (capped at
/// `SHARED_BODY_CAP_PER_ENTRY`).
fn update_cache_body_on(key: &CacheKey, appended: &[u8], shared: &SharedState) {
    if appended.is_empty() {
        return;
    }
    let mut cache = shared.cache.lock();
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

/// Core streaming read: opens the upstream connection if necessary, reads
/// chunks until data at `offset` is available (or EOF), and returns the
/// requested slice.
///
/// Used by both the main-loop synchronous path (`read_node`) and by worker
/// threads (`run_handle_worker`).
fn perform_read(
    handle_id: u64,
    state: &mut HttpsHandle,
    shared: &SharedState,
    offset: usize,
    max_len: usize,
) -> Result<Vec<u8>, Errno> {
    if max_len == 0 {
        return Ok(Vec::new());
    }

    ensure_upstream_on(handle_id, state, shared)?;

    let key = state.node.cache_key();

    trace!(
        "httpsd: read handle={} url={} offset={} len={} cached={} start={} eof={}",
        handle_id,
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
            handle_id, offset, state.body_start_offset, BODY_WINDOW_CAP
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
            "httpsd: read handle={} calling read_chunk (offset={} end={} eof={})",
            handle_id, offset, body_end, state.eof
        );
        let chunk = response.read_chunk().map_err(|err| {
            warn!(
                "httpsd: upstream read failed for handle={} {}: {}",
                handle_id,
                state.node.url(),
                err
            );
            Errno::EIO
        })?;
        trace!("httpsd: read_chunk handle={} returned {} bytes", handle_id, chunk.len());
        if chunk.is_empty() {
            debug!("httpsd: upstream EOF for handle={} cached={}", handle_id, state.body.len());
            state.response = None;
            state.eof = true;
            break;
        }
        trace!("httpsd: upstream chunk handle={} bytes={}", handle_id, chunk.len());
        chunks_to_mirror.push(chunk.clone());
        state.push_chunk(&chunk)?;
        body_end = state.body_start_offset.checked_add(state.body.len()).ok_or(Errno::EOVERFLOW)?;
    }

    // Re-check after fetch because the retained window may have advanced while reading chunks.
    if offset < state.body_start_offset {
        debug!(
            "httpsd: read handle={} offset={} evicted while streaming (start={})",
            handle_id, offset, state.body_start_offset
        );
        return Err(Errno::EINVAL);
    }

    // Mirror chunks to shared cache.
    for c in &chunks_to_mirror {
        update_cache_body_on(&key, c, shared);
    }

    if offset >= body_end {
        debug!("httpsd: read handle={} -> EOF at offset {}", handle_id, offset);
        return Ok(Vec::new());
    }
    let start = offset - state.body_start_offset;
    let end_limit = needed_end.checked_sub(state.body_start_offset).ok_or(Errno::EINVAL)?;
    let end = state.body.len().min(end_limit);
    let out = state.body[start..end].to_vec();
    debug!(
        "httpsd: read handle={} -> returned {} bytes (cached={} start={} eof={})",
        handle_id,
        out.len(),
        state.body.len(),
        state.body_start_offset,
        state.eof
    );
    Ok(out)
}

// ── Worker thread ───────────────────────────────────────────────────────────

/// Send a formatted VFS RPC response directly to the kernel's per-request
/// response port.  Used by worker threads that need to reply independently
/// of the main RPC loop.
fn send_worker_response(resp_port: u32, req_id: u16, resp: ProviderResponse) {
    use abi::vfs_rpc::VFS_RPC_MAX_RESP;
    let total = 3 + resp.payload.len();
    let mut buf = alloc::vec![0u8; total.min(VFS_RPC_MAX_RESP)];
    buf[0..2].copy_from_slice(&req_id.to_le_bytes());
    buf[2] = resp.status;
    let payload_len = resp.payload.len().min(buf.len() - 3);
    buf[3..3 + payload_len].copy_from_slice(&resp.payload[..payload_len]);
    loop {
        match stem::syscall::port::port_send_all(resp_port, &buf[..3 + payload_len]) {
            Ok(_) => return,
            Err(Errno::EAGAIN) => {
                stem::syscall::yield_now();
                continue;
            }
            Err(e) => {
                warn!("httpsd: worker failed to send response: {:?}", e);
                return;
            }
        }
    }
}

/// Worker thread entry point.  Owns `state` (the `HttpsHandle`) exclusively,
/// processes `Read` messages by calling `perform_read`, and exits cleanly on
/// `Close`.
///
/// **Idle spin-wait**: the worker calls `yield_now` while its channel is empty.
/// This is acceptable because:
///  (a) workers spend most of their time blocked on network I/O, so the
///      idle window between consecutive reads from the same client is brief;
///  (b) Thing-OS does not yet expose condition-variable or semaphore
///      primitives that would allow a true sleep.
/// If many paused clients accumulate workers, CPU usage could rise.  A future
/// improvement would replace the spin loop with a blocking port-receive on a
/// per-worker notification port.
fn run_handle_worker(
    handle_id: u64,
    mut state: HttpsHandle,
    channel: Arc<WorkerChannel>,
    shared: Arc<SharedState>,
    _mount_point: String,
) {
    debug!("httpsd: worker started for handle={} url={}", handle_id, state.node.url());
    loop {
        let msg = loop {
            if let Some(m) = channel.pop() {
                break m;
            }
            stem::syscall::yield_now();
        };

        match msg {
            WorkerMsg::Read { resp_port, req_id, offset, max_len } => {
                let resp = match perform_read(handle_id, &mut state, &shared, offset, max_len) {
                    Ok(data) => ProviderResponse::ok_read(&data),
                    Err(e) => ProviderResponse::err(e),
                };
                send_worker_response(resp_port, req_id, resp);
            }
            WorkerMsg::Close => {
                debug!("httpsd: worker exiting for handle={}", handle_id);
                break;
            }
        }
    }
}

// ── Entry points ───────────────────────────────────────────────────────────

#[cfg(not(test))]
#[stem::main]
fn main(_arg: usize) -> ! {
    let (fixed_host, mount_point) = parse_args();
    run_provider(fixed_host, &mount_point)
}

#[cfg(not(test))]
#[used]
static KEEP_THINGOS_VFS_MOUNT_V1: extern "C" fn(usize) -> ! = thingos_vfs_mount_v1;

#[cfg(not(test))]
#[used]
static KEEP_THINGOS_VFS_UNMOUNT_V1: extern "C" fn(usize) -> i32 = thingos_vfs_unmount_v1;

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn thingos_vfs_mount_v1(arg: usize) -> ! {
    unsafe { stem::rt::entry_impl(arg) }
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn thingos_vfs_unmount_v1(_arg: usize) -> i32 {
    let (_fixed_host, mount_point) = parse_args();
    match vfs_umount(&mount_point) {
        Ok(()) => {}
        Err(_) => return 1,
    }
    // Best-effort: the secondary cache mount may not be present if setup failed.
    let _ = vfs_umount(CACHE_MOUNT_POINT);
    0
}

fn run_provider(fixed_host: Option<String>, mount_point: &str) -> ! {
    let shared = Arc::new(SharedState::new());

    // Spawn the cache-mount loop first; a failure to set up the secondary
    // mount should be logged but must not prevent the primary mount
    // from running (operators can still use xattrs).
    match spawn_cache_mount(shared.clone()) {
        Ok(()) => debug!("httpsd: cache mount started at {}", CACHE_MOUNT_POINT),
        Err(e) => warn!("httpsd: failed to start cache mount at {}: {:?}", CACHE_MOUNT_POINT, e),
    }

    run_https_mount(fixed_host, mount_point, shared)
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
            send_response(&lp, &req, resp);
        }
    })?;
    Ok(())
}

fn run_https_mount(fixed_host: Option<String>, mount_point: &str, shared: Arc<SharedState>) -> ! {
    let (req_write, req_read) = match stem::syscall::port::port_create(HTTPS_PORT_CAPACITY_BYTES) {
        Ok(pair) => pair,
        Err(e) => {
            warn!("httpsd: port_create failed: {:?}", e);
            stem::syscall::exit(1);
        }
    };

    match vfs_mount(req_write, mount_point) {
        Ok(()) => {
            debug!("httpsd: mounted at {}", mount_point);
            info!("HTTPSD_READY");
        }
        Err(e) => {
            warn!("httpsd: failed to mount {}: {:?}", mount_point, e);
            stem::syscall::exit(1);
        }
    }

    let mut provider = HttpsProvider::new(shared, fixed_host, mount_point.to_string());
    let mut lp = ProviderLoop::new(req_read);
    info!("httpsd: entering main RPC loop for {}", mount_point);
    loop {
        let req = match lp.next_request() {
            Ok(r) => r,
            Err(e) => {
                warn!("httpsd: provider loop ended: {:?}", e);
                break;
            }
        };
        // `dispatch` returns `None` when the Read has been forwarded to a
        // worker thread; the worker will send the response asynchronously.
        if let Some(resp) = dispatch(&mut provider, req.op, &req.payload, req.resp_port, req.req_id)
        {
            send_response(&lp, &req, resp);
        }
    }

    info!("httpsd: main RPC loop ended - exiting");
    stem::syscall::exit(0);
}

fn send_response(
    lp: &ProviderLoop,
    req: &ipc_helpers::provider::ProviderRequest,
    resp: ProviderResponse,
) {
    loop {
        match lp.send_response(req, resp.clone()) {
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

fn parse_args() -> (Option<String>, String) {
    let len = match argv_get(&mut []) {
        Ok(l) if l > 0 => l,
        _ => return (None, MOUNT_POINT.to_string()),
    };
    let mut buf = alloc::vec![0u8; len];
    if argv_get(&mut buf).is_err() {
        return (None, MOUNT_POINT.to_string());
    }
    let args = stem::utils::parse_argv(&buf);
    let mut args_str = args.iter().filter_map(|b| core::str::from_utf8(b).ok());

    let _prog = args_str.next();
    let arg1 = args_str.next();
    let arg2 = args_str.next();

    match (arg1, arg2) {
        (Some(a1), Some(a2)) => {
            // mount -t https <device> <target>
            // argv[1] is device, argv[2] is target
            let device = if a1 == "none" { None } else { Some(a1.to_string()) };
            (device, a2.to_string())
        }
        (Some(a1), None) => {
            // mount -t https <target> (legacy or fstab without device)
            (None, a1.to_string())
        }
        _ => (None, MOUNT_POINT.to_string()),
    }
}

// ── RPC dispatch — /https mount ────────────────────────────────────────────

/// Dispatch an incoming VFS RPC.
///
/// Returns `Some(response)` for ops that can be answered immediately by the
/// main loop, and `None` for `Read` ops that have been forwarded to a
/// per-handle worker thread (the worker will send the response asynchronously
/// via `send_worker_response`).
fn dispatch(
    provider: &mut HttpsProvider,
    op: VfsRpcOp,
    payload: &[u8],
    resp_port: u32,
    req_id: u16,
) -> Option<ProviderResponse> {
    trace!("httpsd: RPC op={:?} payload_len={}", op, payload.len());
    match op {
        VfsRpcOp::Lookup => Some(dispatch_lookup(provider, payload)),
        VfsRpcOp::Read => dispatch_read(provider, payload, resp_port, req_id),
        VfsRpcOp::Stat => Some(dispatch_stat(provider, payload)),
        VfsRpcOp::Readdir => Some(dispatch_readdir(payload)),
        VfsRpcOp::Close => Some(dispatch_close(provider, payload)),
        VfsRpcOp::SubscribeReady | VfsRpcOp::UnsubscribeReady => Some(ProviderResponse::ok_empty()),
        VfsRpcOp::Poll => Some(ProviderResponse::ok_poll(abi::syscall::poll_flags::POLLIN as u32)),
        VfsRpcOp::AttrList => Some(dispatch_attr_list(provider, payload)),
        VfsRpcOp::AttrGet => Some(dispatch_attr_get(provider, payload)),
        VfsRpcOp::AttrSet | VfsRpcOp::AttrRemove => Some(ProviderResponse::err(Errno::EROFS)),
        VfsRpcOp::Readlink => Some(dispatch_readlink(provider, payload)),
        _ => Some(ProviderResponse::err(Errno::ENOSYS)),
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
    trace!("httpsd: dispatch_lookup path='{}'", path);
    match provider.resolve_path(path) {
        Ok(handle) => ProviderResponse::ok_u64(handle),
        Err(e) => ProviderResponse::err(e),
    }
}

/// Dispatch a `Read` RPC.
///
/// For non-root handles, the handle is promoted to a worker thread on the
/// first `Read`.  Subsequent reads on the same handle are forwarded to the
/// worker's channel and this function returns `None` — the worker will send
/// the response directly to the kernel.
fn dispatch_read(
    provider: &mut HttpsProvider,
    payload: &[u8],
    resp_port: u32,
    req_id: u16,
) -> Option<ProviderResponse> {
    if payload.len() < 20 {
        return Some(ProviderResponse::err(Errno::EINVAL));
    }
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
    let offset = u64::from_le_bytes(payload[8..16].try_into().unwrap_or([0; 8])) as usize;
    let max_len = u32::from_le_bytes(payload[16..20].try_into().unwrap_or([0; 4])) as usize;

    // Root handle: fast path — not a real streaming file.
    if handle == ROOT_HANDLE {
        return Some(ProviderResponse::err(Errno::EISDIR));
    }

    // If this handle already has a worker, forward the Read to it.
    if let Some(wh) = provider.workers.get(&handle) {
        wh.channel.push(WorkerMsg::Read { resp_port, req_id, offset, max_len });
        return None;
    }

    // First Read on this handle: promote it to a worker thread and forward.
    match provider.promote_to_worker(handle) {
        Ok(channel) => {
            channel.push(WorkerMsg::Read { resp_port, req_id, offset, max_len });
            None
        }
        Err(e) => {
            warn!(
                "httpsd: failed to promote handle={} to worker: {:?} \
                 — falling back to synchronous read (concurrent streaming unavailable for this handle)",
                handle, e
            );
            // Fallback: synchronous read (handle stays in the unstarted map).
            let data = match provider.read_node(handle, offset, max_len) {
                Ok(d) => d,
                Err(err) => return Some(ProviderResponse::err(err)),
            };
            Some(ProviderResponse::ok_read(&data))
        }
    }
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
    trace!("httpsd: dispatch_attr_list handle={}", handle);
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
    trace!("httpsd: dispatch_readlink handle={}", handle);
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
        HttpsProvider::new(Arc::new(SharedState::new()), None, MOUNT_POINT.to_string())
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
        let p = new_provider();
        assert_eq!(
            p.url_to_vfs_path("https://example.com/index.html"),
            "/https/example.com/index.html"
        );
        assert_eq!(p.url_to_vfs_path("https://example.com"), "/https/example.com");
        assert_eq!(p.url_to_vfs_path("http://example.com/legacy"), "/https/example.com/legacy");
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
        assert_eq!(provider.redirect_target_for(handle).unwrap(), "/https/b.example/new");
    }

    #[test]
    fn fixed_host_resolution_uses_direct_path() {
        let shared = Arc::new(SharedState::new());
        let mut p = HttpsProvider::new(shared, Some("example.com".into()), "/https/ex".into());
        let handle = p.resolve_path("/index.html").expect("resolve succeeds");
        let state = p.handles.get(&handle).unwrap();
        assert_eq!(state.node.host, "example.com");
        assert_eq!(state.node.path, "index.html");
        assert_eq!(state.node.url(), "https://example.com/index.html");
    }

    #[test]
    fn discovery_mount_resolves_host_qualified_index_path() {
        let mut p = new_provider();
        let handle =
            p.resolve_path("/www.example.com/@index").expect("host-qualified @index resolves");
        let state = p.handles.get(&handle).expect("resolved handle exists");
        assert_eq!(state.node.host, "www.example.com");
        assert_eq!(state.node.path, "");
        assert_eq!(state.node.url(), "https://www.example.com");
    }

    #[test]
    fn fixed_host_url_translation_uses_mount_point() {
        let shared = Arc::new(SharedState::new());
        let p = HttpsProvider::new(shared, Some("example.com".into()), "/https/ex".into());
        assert_eq!(p.url_to_vfs_path("https://example.com/foo"), "/https/ex/foo");
        assert_eq!(p.url_to_vfs_path("https://other.com/bar"), "/https/ex/other.com/bar");
    }

    #[test]
    fn discovery_mode_url_translation_uses_mount_point() {
        let shared = Arc::new(SharedState::new());
        let p = HttpsProvider::new(shared, None, "/web".into());
        assert_eq!(p.url_to_vfs_path("https://example.com/foo"), "/web/example.com/foo");
        assert_eq!(p.url_to_vfs_path("https://example.com"), "/web/example.com");
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

    // ── Worker infrastructure tests ─────────────────────────────────────────

    #[test]
    fn worker_channel_push_pop_ordering() {
        let ch = WorkerChannel::new();
        assert!(ch.pop().is_none());
        ch.push(WorkerMsg::Read { resp_port: 1, req_id: 10, offset: 0, max_len: 512 });
        ch.push(WorkerMsg::Close);
        let msg = ch.pop().unwrap();
        assert!(matches!(msg, WorkerMsg::Read { req_id: 10, .. }));
        let msg2 = ch.pop().unwrap();
        assert!(matches!(msg2, WorkerMsg::Close));
        assert!(ch.pop().is_none());
    }

    #[test]
    fn close_node_for_unstarted_handle_removes_from_handles() {
        let mut provider = new_provider();
        let handle = provider.allocate_node("example.com", "page", None);
        assert!(provider.handles.contains_key(&handle));
        provider.close_node(handle);
        assert!(!provider.handles.contains_key(&handle));
        assert!(!provider.workers.contains_key(&handle));
    }

    #[test]
    fn stat_node_rejects_unknown_handle() {
        let provider = new_provider();
        assert_eq!(provider.stat_node(9999), Err(Errno::EBADF));
    }

    #[test]
    fn stat_node_returns_dir_mode_for_root() {
        let provider = new_provider();
        let (mode, _, ino) = provider.stat_node(ROOT_HANDLE).unwrap();
        assert_eq!(mode & 0o170_000, 0o040_000, "expected S_IFDIR");
        assert_eq!(ino, ROOT_HANDLE);
    }

    #[test]
    fn read_node_rejects_root_handle() {
        let mut provider = new_provider();
        assert_eq!(provider.read_node(ROOT_HANDLE, 0, 64), Err(Errno::EISDIR));
    }

    #[test]
    fn perform_read_rejects_offset_before_window() {
        let shared = Arc::new(SharedState::new());
        let node = HttpsNode::new("example.com", "");
        let mut state = HttpsHandle::new(node, None);
        state.body.extend_from_slice(b"hello");
        state.body_start_offset = 3;
        state.eof = true;
        // offset=1 is before body_start_offset=3
        assert_eq!(perform_read(1, &mut state, &shared, 1, 4), Err(Errno::EINVAL));
    }

    #[test]
    fn perform_read_maps_window_offsets() {
        let shared = Arc::new(SharedState::new());
        let node = HttpsNode::new("example.com", "");
        let mut state = HttpsHandle::new(node, None);
        state.body.extend_from_slice(b"abcdef");
        state.body_start_offset = 3;
        state.eof = true;
        // offset=4, len=3 maps to body[1..4] = "bcd"
        assert_eq!(perform_read(1, &mut state, &shared, 4, 3).unwrap(), b"bcd");
        // offset=9 is past body_end (3+6=9) → EOF slice
        assert_eq!(perform_read(1, &mut state, &shared, 9, 8).unwrap(), b"");
    }

    #[test]
    fn ensure_cached_entry_returns_ebadf_for_unknown_handle() {
        let mut provider = new_provider();
        assert!(matches!(provider.ensure_cached_entry(9999), Err(Errno::EBADF)));
    }
}
