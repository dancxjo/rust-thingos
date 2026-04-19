#![no_std]
#![no_main]
extern crate alloc;

use abi::errors::Errno;
use abi::vfs_rpc::VfsRpcOp;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use http::{HttpClient, Response};
use ipc_helpers::provider::{ProviderLoop, ProviderResponse};
use stem::syscall::vfs::vfs_mount;
use stem::{info, warn};

const MOUNT_POINT: &str = "/https";
const ROOT_HANDLE: u64 = 1;

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
    eof: bool,
}

impl HttpsHandle {
    fn new(node: HttpsNode, response: Option<Response>) -> Self {
        Self { node, response, body: Vec::new(), eof: false }
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
            info!("httpsd: lookup '{}' -> root", path);
            return Ok(ROOT_HANDLE);
        }

        let mut parts = clean.split('/');
        let host = parts.next().ok_or(Errno::ENOENT)?;
        if !Self::is_valid_host_label(host) {
            info!("httpsd: lookup '{}' rejected: invalid host '{}'", path, host);
            return Err(Errno::ENOENT);
        }

        let rest = parts.collect::<Vec<_>>().join("/");
        let node = HttpsNode::new(host, &rest);
        info!("httpsd: lookup '{}' probing {}", path, node.url());
        let response = match HttpClient::get(&node.url()) {
            Ok(response) => response,
            Err(err) => {
                warn!("httpsd: lookup '{}' probe failed: {}", path, err);
                return Err(Errno::ENOENT);
            }
        };
        let handle = self.allocate_node(host, &rest, Some(response));
        info!("httpsd: lookup '{}' -> handle {}", path, handle);
        Ok(handle)
    }

    fn is_valid_host_label(host: &str) -> bool {
        host.contains('.') && host.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'.' || b == b'-')
    }

    fn read_node(&mut self, handle: u64, offset: usize, max_len: usize) -> Result<Vec<u8>, Errno> {
        if handle == ROOT_HANDLE {
            return Err(Errno::EISDIR);
        }
        let Some(state) = self.handles.get_mut(&handle) else {
            return Err(Errno::EBADF);
        };

        info!(
            "httpsd: read handle={} url={} offset={} len={} cached={} eof={}",
            handle,
            state.node.url(),
            offset,
            max_len,
            state.body.len(),
            state.eof
        );

        if state.response.is_none() && !state.eof {
            info!("httpsd: opening upstream stream for handle={} {}", handle, state.node.url());
            state.response = Some(HttpClient::get(&state.node.url()).map_err(|err| {
                warn!("httpsd: upstream open failed for handle={} {}: {}", handle, state.node.url(), err);
                Errno::EIO
            })?);
        }

        let needed_end = offset.saturating_add(max_len);
        while state.body.len() < needed_end && !state.eof {
            let Some(response) = state.response.as_mut() else {
                state.eof = true;
                break;
            };

            let chunk = response.read_chunk().map_err(|err| {
                warn!("httpsd: upstream read failed for handle={} {}: {}", handle, state.node.url(), err);
                Errno::EIO
            })?;
            if chunk.is_empty() {
                info!("httpsd: upstream EOF for handle={} cached={}", handle, state.body.len());
                state.response = None;
                state.eof = true;
                break;
            }
            info!("httpsd: upstream chunk handle={} bytes={}", handle, chunk.len());
            state.body.extend_from_slice(&chunk);
        }

        if offset >= state.body.len() {
            info!("httpsd: read handle={} -> EOF at offset {}", handle, offset);
            return Ok(Vec::new());
        }
        let end = state.body.len().min(needed_end);
        let out = state.body[offset..end].to_vec();
        info!(
            "httpsd: read handle={} -> returned {} bytes (cached={} eof={})",
            handle,
            out.len(),
            state.body.len(),
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
    let (req_write, req_read) = match stem::syscall::channel::port_create(64 * 1024) {
        Ok(pair) => pair,
        Err(e) => {
            warn!("httpsd: port_create failed: {:?}", e);
            stem::syscall::exit(1);
        }
    };

    match vfs_mount(req_write, MOUNT_POINT) {
        Ok(()) => info!("httpsd: mounted at {}", MOUNT_POINT),
        Err(e) => {
            warn!("httpsd: failed to mount {}: {:?}", MOUNT_POINT, e);
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

fn dispatch(provider: &mut HttpsProvider, op: VfsRpcOp, payload: &[u8]) -> ProviderResponse {
    info!("httpsd: rpc {:?} payload_len={}", op, payload.len());
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
}
