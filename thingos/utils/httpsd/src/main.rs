#![no_std]
#![no_main]
extern crate alloc;

use abi::errors::Errno;
use abi::vfs_rpc::VfsRpcOp;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use http::HttpClient;
use ipc_helpers::provider::{ProviderLoop, ProviderResponse};
use stem::syscall::vfs::{vfs_close, vfs_mount, vfs_open, vfs_read, vfs_write};
use stem::{info, warn};

const MOUNT_POINT: &str = "/https";
const ROOT_HANDLE: u64 = 1;
const DNS_LOOKUP_PATH: &str = "/net/dns/lookup";
const DNS_MAX_ATTEMPTS: usize = 20;
const DNS_RETRY_MS: u64 = 25;
const DT_DIR: u8 = 4;

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
    handles: BTreeMap<u64, HttpsNode>,
    reverse: BTreeMap<String, u64>,
    host_cache: BTreeMap<String, bool>,
}

impl HttpsProvider {
    fn new() -> Self {
        Self {
            next_handle: ROOT_HANDLE + 1,
            handles: BTreeMap::new(),
            reverse: BTreeMap::new(),
            host_cache: BTreeMap::new(),
        }
    }

    fn key(host: &str, path: &str) -> String {
        alloc::format!("{}|{}", host, path)
    }

    fn lookup_or_insert_node(&mut self, host: &str, path: &str) -> u64 {
        let key = Self::key(host, path);
        if let Some(handle) = self.reverse.get(&key) {
            return *handle;
        }

        let handle = self.next_handle;
        self.next_handle = self.next_handle.saturating_add(1);
        self.handles.insert(handle, HttpsNode::new(host, path));
        self.reverse.insert(key, handle);
        handle
    }

    fn resolve_path(&mut self, path: &str) -> Result<u64, Errno> {
        let clean = path.trim_matches('/');
        if clean.is_empty() {
            return Ok(ROOT_HANDLE);
        }

        let mut parts = clean.split('/');
        let host = parts.next().ok_or(Errno::ENOENT)?;
        if !Self::is_valid_host_label(host) || !self.resolve_host(host) {
            return Err(Errno::ENOENT);
        }

        let rest = parts.collect::<Vec<_>>().join("/");
        Ok(self.lookup_or_insert_node(host, &rest))
    }

    fn resolve_host(&mut self, host: &str) -> bool {
        if let Some(&true) = self.host_cache.get(host) {
            return true;
        }
        let ok = resolve_host_via_netd(host);
        if ok {
            self.host_cache.insert(host.to_string(), true);
        }
        ok
    }

    fn is_valid_host_label(host: &str) -> bool {
        host.contains('.') && host.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'.' || b == b'-')
    }

    fn read_node(&self, handle: u64) -> Result<Vec<u8>, Errno> {
        if handle == ROOT_HANDLE {
            return Err(Errno::EISDIR);
        }
        let Some(node) = self.handles.get(&handle) else {
            return Err(Errno::EBADF);
        };
        let mut response = HttpClient::get(&node.url()).map_err(|_| Errno::EIO)?;
        let mut all = Vec::new();
        loop {
            let chunk = response.read_chunk().map_err(|_| Errno::EIO)?;
            if chunk.is_empty() {
                break;
            }
            all.extend_from_slice(&chunk);
        }
        Ok(all)
    }

    fn stat_node(&self, handle: u64) -> Result<(u32, u64, u64), Errno> {
        if handle == ROOT_HANDLE {
            return Ok((0o040_555, 0, ROOT_HANDLE));
        }
        if self.handles.contains_key(&handle) {
            return Ok((0o040_555, 0, handle));
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

fn dispatch_read(provider: &HttpsProvider, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 20 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
    let offset = u64::from_le_bytes(payload[8..16].try_into().unwrap_or([0; 8])) as usize;
    let max_len = u32::from_le_bytes(payload[16..20].try_into().unwrap_or([0; 4])) as usize;

    let data = match provider.read_node(handle) {
        Ok(d) => d,
        Err(e) => return ProviderResponse::err(e),
    };
    if offset >= data.len() {
        return ProviderResponse::ok_read(&[]);
    }
    let end = data.len().min(offset + max_len);
    ProviderResponse::ok_read(&data[offset..end])
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
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap_or([0; 8]));
    if handle != ROOT_HANDLE {
        return ProviderResponse::ok_read(&[]);
    }

    // Root is intentionally sparse: it only grows as hosts are traversed.
    // We still return "." so standard directory readers have a stable entry.
    let mut out = Vec::new();
    out.extend_from_slice(&ROOT_HANDLE.to_le_bytes());
    out.push(DT_DIR);
    out.push(1);
    out.push(b'.');
    ProviderResponse::ok_read(&out)
}

fn resolve_host_via_netd(host: &str) -> bool {
    use abi::syscall::vfs_flags::{O_RDONLY, O_WRONLY};

    let Ok(write_fd) = vfs_open(DNS_LOOKUP_PATH, O_WRONLY) else {
        return false;
    };
    if vfs_write(write_fd, host.as_bytes()).is_err() {
        let _ = vfs_close(write_fd);
        return false;
    }
    let _ = vfs_close(write_fd);

    let Ok(read_fd) = vfs_open(DNS_LOOKUP_PATH, O_RDONLY) else {
        return false;
    };
    let mut buf = [0u8; 64];
    for _ in 0..DNS_MAX_ATTEMPTS {
        match vfs_read(read_fd, &mut buf) {
            Ok(n) if n > 0 => {
                let _ = vfs_close(read_fd);
                return core::str::from_utf8(&buf[..n]).map(|s| s.trim().contains('.')).unwrap_or(false);
            }
            Err(abi::errors::Errno::EAGAIN) => stem::time::sleep_ms(DNS_RETRY_MS),
            _ => break, // Fail fast on EIO or other errors
        }
    }
    let _ = vfs_close(read_fd);
    false
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
