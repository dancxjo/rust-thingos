#![no_std]
#![no_main]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use abi::errors::Errno;
use abi::seed::{
    HOST_PROGRAM, HOST_VFS_PROVIDER, INTERFACE_PROGRAM_V1, INTERFACE_VFS_PROVIDER_MOUNT_V1,
    INTERFACE_VFS_PROVIDER_UNMOUNT_V1, SEED_ABI_VERSION, Seed, SeedInterface,
};
use abi::vfs_rpc::VfsRpcOp;
use http::{HttpClient, Response};
use ipc_helpers::provider::{ProviderLoop, ProviderResponse};
use stem::syscall::argv_get;
use stem::syscall::vfs::{vfs_mount, vfs_umount};
use stem::{debug, info, warn};

const MOUNT_POINT: &str = "/https";
const ROOT_HANDLE: u64 = 1;
const BODY_WINDOW_CAP: usize = 128 * 1024; // Align with multiple of 16KB chunks
const HTTPS_PORT_CAPACITY_BYTES: usize = 32_768;
const SEED_NAME: &[u8] = b"httpsd";
const HOOK_MOUNT_V1: &[u8] = b"thingos_vfs_mount_v1";
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
        if self.path.is_empty() {
            alloc::format!("https://{}", self.host)
        } else {
            alloc::format!("https://{}/{}", self.host, self.path)
        }
    }
}

struct HttpsProvider {
    next_handle: u64,
    handles: BTreeMap<u64, HttpsHandle>,
}

struct HttpsHandle {
    node: HttpsNode,
    response: Option<Response>,
    body: Vec<u8>,
    body_start_offset: usize,
    eof: bool,
}

impl HttpsHandle {
    fn new(node: HttpsNode, response: Option<Response>) -> Self {
        Self { node, response, body: Vec::new(), body_start_offset: 0, eof: false }
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

impl HttpsProvider {
    fn new() -> Self {
        Self { next_handle: ROOT_HANDLE + 1, handles: BTreeMap::new() }
    }

    fn allocate_node(&mut self, host: &str, path: &str, response: Option<Response>) -> u64 {
        let handle = self.next_handle;
        self.next_handle = self.next_handle.saturating_add(1);
        self.handles.insert(handle, HttpsHandle::new(HttpsNode::new(host, path), response));
        handle
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

    fn read_node(&mut self, handle: u64, offset: usize, max_len: usize) -> Result<Vec<u8>, Errno> {
        if handle == ROOT_HANDLE {
            return Err(Errno::EISDIR);
        }
        if max_len == 0 {
            return Ok(Vec::new());
        }
        let Some(state) = self.handles.get_mut(&handle) else {
            return Err(Errno::EBADF);
        };

        debug!(
            "httpsd: read handle={} url={} offset={} len={} cached={} start={} eof={}",
            handle,
            state.node.url(),
            offset,
            max_len,
            state.body.len(),
            state.body_start_offset,
            state.eof
        );

        if state.response.is_none() && !state.eof {
            debug!("httpsd: opening upstream stream for handle={} {}", handle, state.node.url());
            state.response = Some(HttpClient::get(&state.node.url()).map_err(|err| {
                warn!(
                    "httpsd: upstream open failed for handle={} {}: {}",
                    handle,
                    state.node.url(),
                    err
                );
                Errno::EIO
            })?);
        }

        let needed_end = offset.saturating_add(max_len);
        let mut body_end =
            state.body_start_offset.checked_add(state.body.len()).ok_or(Errno::EOVERFLOW)?;

        // Offset reads are constrained to the retained body window.
        // If the caller seeks behind `body_start_offset`, those bytes have been evicted.
        if offset < state.body_start_offset {
            debug!(
                "httpsd: read handle={} offset={} before retained window start={} (cap={})",
                handle, offset, state.body_start_offset, BODY_WINDOW_CAP
            );
            return Err(Errno::EINVAL);
        }

        // Streaming reads only fetch until there is data at `offset` (or EOF),
        // instead of caching the entire upstream response.
        while body_end <= offset && !state.eof {
            let Some(response) = state.response.as_mut() else {
                state.eof = true;
                break;
            };

            let chunk = response.read_chunk().map_err(|err| {
                warn!(
                    "httpsd: upstream read failed for handle={} {}: {}",
                    handle,
                    state.node.url(),
                    err
                );
                Errno::EIO
            })?;
            if chunk.is_empty() {
                debug!("httpsd: upstream EOF for handle={} cached={}", handle, state.body.len());
                state.response = None;
                state.eof = true;
                break;
            }
            debug!("httpsd: upstream chunk handle={} bytes={}", handle, chunk.len());
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
        Ok(out)
    }

    fn stat_node(&self, handle: u64) -> Result<(u32, u64, u64), Errno> {
        if handle == ROOT_HANDLE {
            return Ok((0o040_555, 0, ROOT_HANDLE));
        }
        if self.handles.contains_key(&handle) {
            return Ok((0o100_444, 0, handle));
        }
        Err(Errno::EBADF)
    }
}

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
pub extern "C" fn thingos_vfs_mount_v1(_arg: usize) -> ! {
    let mount_point = mount_point_from_args();
    run_provider(&mount_point)
}

#[no_mangle]
pub extern "C" fn thingos_vfs_unmount_v1(_arg: usize) -> i32 {
    let mount_point = mount_point_from_args();
    match vfs_umount(&mount_point) {
        Ok(()) => 0,
        Err(_) => 1,
    }
}

fn run_provider(mount_point: &str) -> ! {
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

    let mut provider = HttpsProvider::new();
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
        let _ = lp.send_response(req.resp_port, resp);
    }

    stem::syscall::exit(0);
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

fn dispatch(provider: &mut HttpsProvider, op: VfsRpcOp, payload: &[u8]) -> ProviderResponse {
    debug!("httpsd: rpc {:?} payload_len={}", op, payload.len());
    match op {
        VfsRpcOp::Lookup => dispatch_lookup(provider, payload),
        VfsRpcOp::Read => dispatch_read(provider, payload),
        VfsRpcOp::Stat => dispatch_stat(provider, payload),
        VfsRpcOp::Readdir => dispatch_readdir(payload),
        VfsRpcOp::Close | VfsRpcOp::SubscribeReady | VfsRpcOp::UnsubscribeReady => {
            ProviderResponse::ok_empty()
        }
        VfsRpcOp::Poll => ProviderResponse::ok_poll(abi::syscall::poll_flags::POLLIN as u32),
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

fn dispatch_readdir(payload: &[u8]) -> ProviderResponse {
    if payload.len() < 20 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let _handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
    ProviderResponse::ok_read(&[])
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use abi::errors::Errno;

    use super::HttpsProvider;
    extern crate std;

    #[test]
    fn host_validation_requires_domain_shape() {
        assert!(HttpsProvider::is_valid_host_label("en.wikipedia.org"));
        assert!(!HttpsProvider::is_valid_host_label("localhost"));
        assert!(!HttpsProvider::is_valid_host_label("bad host"));
    }

    #[test]
    fn node_url_builds_https_paths() {
        let node = super::HttpsNode::new("en.wikipedia.org", "wiki/Dormouse");
        assert_eq!(node.url(), "https://en.wikipedia.org/wiki/Dormouse");
    }

    #[test]
    fn read_node_rejects_offsets_before_retained_window() {
        let mut provider = HttpsProvider::new();
        let handle = provider.allocate_node("example.com", "", None);
        let state = provider.handles.get_mut(&handle).expect("allocated handle");
        state.body.extend_from_slice(b"abcdef");
        state.body_start_offset = 3;
        state.eof = true;

        assert_eq!(provider.read_node(handle, 2, 4), Err(Errno::EINVAL));
    }

    #[test]
    fn read_node_maps_offsets_within_retained_window() {
        let mut provider = HttpsProvider::new();
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
        let mut handle = super::HttpsHandle::new(super::HttpsNode::new("example.com", ""), None);
        let chunk = vec![7u8; super::BODY_WINDOW_CAP + 5];
        handle.push_chunk(&chunk).expect("chunk push");
        assert_eq!(handle.body.len(), super::BODY_WINDOW_CAP);
        assert_eq!(handle.body_start_offset, 5);
        assert!(handle.body.iter().all(|b| *b == 7));
    }
}
