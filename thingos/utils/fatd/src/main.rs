//! fatd — FAT filesystem VFS provider
//!
//! Scans `/dev/block/` for USB partition block devices, probes each for a
//! FAT16 or FAT32 filesystem, and mounts the first found volume at
//! `/media/usb` as a read-only VFS provider.
#![no_std]
#![no_main]
extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use abi::errors::Errno;
use abi::vfs_rpc::VfsRpcOp;
use fat::{FatFs, FatType};
use ipc_helpers::provider::{ProviderLoop, ProviderRequest, ProviderResponse};
use stem::abi::module_manifest::{MANIFEST_MAGIC, ManifestHeader, ModuleKind, device_kind_bytes};
use stem::block::{BlockDevice, BlockError};
use stem::syscall::vfs::{vfs_close, vfs_mount, vfs_open, vfs_read, vfs_readdir, vfs_seek};
use stem::syscall::{PortHandle, port_create};
use stem::{info, warn};

const DEFAULT_MOUNT_POINT: &str = "/media/usb";

#[unsafe(link_section = ".thing_manifest")]
#[unsafe(no_mangle)]
#[used]
pub static MANIFEST: ManifestHeader = ManifestHeader {
    magic: MANIFEST_MAGIC,
    kind: ModuleKind::Service,
    device_kind: device_kind_bytes(b"svc.vfs.fat"),
    version: 1,
    _reserved: 0,
};

// ── Block device adapter ─────────────────────────────────────────────────────

/// Reads from a VFS path using seek+read.  One open/close per call keeps the
/// implementation simple; the kernel handles rapid open/close efficiently.
struct VfsBlockDevice {
    path: String,
}

impl BlockDevice for VfsBlockDevice {
    fn sector_size(&self) -> u64 {
        fat::FAT_SECTOR_SIZE
    }

    fn read_sectors(&self, lba: u64, count: u64, buf: &mut [u8]) -> Result<(), BlockError> {
        let fd = vfs_open(&self.path, abi::syscall::vfs_flags::O_RDONLY)
            .map_err(|_| BlockError::IoError)?;
        let offset = lba * fat::FAT_SECTOR_SIZE;
        vfs_seek(fd, offset as i64, 0).map_err(|_| {
            let _ = vfs_close(fd);
            BlockError::IoError
        })?;

        let need = (count * fat::FAT_SECTOR_SIZE) as usize;
        let mut total = 0usize;
        while total < need {
            match vfs_read(fd, &mut buf[total..need]) {
                Ok(0) => break,
                Ok(n) => total += n,
                Err(_) => {
                    let _ = vfs_close(fd);
                    return Err(BlockError::IoError);
                }
            }
        }
        let _ = vfs_close(fd);
        if total == need { Ok(()) } else { Err(BlockError::IoError) }
    }
}

// ── Handle encoding ──────────────────────────────────────────────────────────

/// Pack (cluster, size, is_directory) into a single u64 handle.
///
/// Layout:
/// - bit 63:     `is_directory` flag
/// - bits 32–62: cluster number (28-bit FAT cluster fits comfortably)
/// - bits  0–31: file size in bytes (0 for directories)
fn encode_handle(cluster: u32, size: u32, is_dir: bool) -> u64 {
    let flag = if is_dir { 1u64 << 63 } else { 0u64 };
    flag | ((cluster as u64) << 32) | (size as u64)
}

fn decode_handle(h: u64) -> (u32, u32, bool) {
    let is_dir = (h >> 63) != 0;
    let cluster = ((h >> 32) & 0x7FFF_FFFF) as u32;
    let size = (h & 0xFFFF_FFFF) as u32;
    (cluster, size, is_dir)
}

const S_IFDIR: u32 = 0o040000;
const S_IFREG: u32 = 0o100000;

// ── VFS RPC dispatch ──────────────────────────────────────────────────────────

fn dispatch_request(fs: &FatFs, dev: &VfsBlockDevice, req: &ProviderRequest) -> ProviderResponse {
    match req.op {
        VfsRpcOp::Lookup => handle_lookup(fs, dev, &req.payload),
        VfsRpcOp::Read => handle_read(fs, dev, &req.payload),
        VfsRpcOp::Readdir => handle_readdir(fs, dev, &req.payload),
        VfsRpcOp::Stat => handle_stat(fs, &req.payload),
        VfsRpcOp::Close | VfsRpcOp::SubscribeReady | VfsRpcOp::UnsubscribeReady => {
            ProviderResponse::ok_empty()
        }
        VfsRpcOp::Poll => ProviderResponse::ok_bytes(&1u32.to_le_bytes()),
        _ => ProviderResponse::err(Errno::ENOSYS),
    }
}

fn handle_lookup(fs: &FatFs, dev: &VfsBlockDevice, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 4 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let path_len = u32::from_le_bytes(payload[0..4].try_into().unwrap()) as usize;
    if 4 + path_len > payload.len() {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let path = core::str::from_utf8(&payload[4..4 + path_len]).unwrap_or("");

    if path.is_empty() || path == "/" {
        let root_cluster = root_cluster_for(fs);
        return ProviderResponse::ok_u64(encode_handle(root_cluster, 0, true));
    }

    match fs.lookup_path(dev, path) {
        Some(entry) => {
            let h = encode_handle(entry.first_cluster, entry.size, entry.is_directory);
            ProviderResponse::ok_u64(h)
        }
        None => ProviderResponse::err(Errno::ENOENT),
    }
}

fn handle_read(fs: &FatFs, dev: &VfsBlockDevice, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 20 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap());
    let offset = u64::from_le_bytes(payload[8..16].try_into().unwrap());
    let len = u32::from_le_bytes(payload[16..20].try_into().unwrap()) as usize;

    let (cluster, size, is_dir) = decode_handle(handle);
    if is_dir {
        return ProviderResponse::err(Errno::EISDIR);
    }
    if len == 0 {
        return ProviderResponse::ok_read(&[]);
    }

    let data = fs.read_file(dev, cluster, size, offset, len);
    ProviderResponse::ok_read(&data)
}

fn handle_readdir(fs: &FatFs, dev: &VfsBlockDevice, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 20 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap());
    let offset = u64::from_le_bytes(payload[8..16].try_into().unwrap());
    let max_bytes = u32::from_le_bytes(payload[16..20].try_into().unwrap()) as usize;

    let (cluster, _, is_dir) = decode_handle(handle);
    if !is_dir {
        return ProviderResponse::err(Errno::ENOTDIR);
    }

    let entries = fs.list_dir(dev, cluster);
    let mut out = Vec::new();
    for entry in entries.iter().skip(offset as usize) {
        let name = entry.name.as_bytes();
        let name_len = name.len().min(255) as u8;
        let file_type: u8 = if entry.is_directory { 4 } else { 8 };
        let ino = encode_handle(entry.first_cluster, entry.size, entry.is_directory);
        if out.len() + 10 + name_len as usize > max_bytes {
            break;
        }
        out.extend_from_slice(&ino.to_le_bytes());
        out.push(file_type);
        out.push(name_len);
        out.extend_from_slice(&name[..name_len as usize]);
    }
    ProviderResponse::ok_read(&out)
}

fn handle_stat(_fs: &FatFs, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 8 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap());
    let (_, size, is_dir) = decode_handle(handle);
    let mode = if is_dir { S_IFDIR | 0o555 } else { S_IFREG | 0o444 };
    ProviderResponse::ok_stat(mode, size as u64, handle)
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn root_cluster_for(fs: &FatFs) -> u32 {
    match fs.fat_type {
        FatType::Fat16 => 0,
        FatType::Fat32 => fs.root_first_cluster,
    }
}

// ── Device discovery ──────────────────────────────────────────────────────────

/// Return `true` if `name` matches the pattern `usb<digits>p<digits>` —
/// the naming scheme used by the xhci driver for USB storage partitions.
///
/// Examples of accepted names: `usb0p1`, `usb1p2`.
/// Examples of rejected names: `usb0`, `usb_port`, `sdap1`.
fn is_usb_partition_name(name: &str) -> bool {
    // Must start with "usb"
    let rest = match name.strip_prefix("usb") {
        Some(r) => r,
        None => return false,
    };
    // One or more decimal digits for the device number
    let after_digits = rest.trim_start_matches(|c: char| c.is_ascii_digit());
    if after_digits.len() == rest.len() {
        return false; // no digits after "usb"
    }
    // Then 'p'
    let rest = match after_digits.strip_prefix('p') {
        Some(r) => r,
        None => return false,
    };
    // One or more decimal digits for the partition number
    !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit())
}

/// Read null-separated names from `/dev/block/` and return those that look
/// like USB partition devices (e.g. `usb0p1`, `usb1p2`).
fn find_usb_partitions() -> Vec<String> {
    let mut result = Vec::new();
    let dir_fd = match vfs_open("/dev/block", abi::syscall::vfs_flags::O_RDONLY) {
        Ok(fd) => fd,
        Err(_) => return result,
    };
    let mut buf = [0u8; 4096];
    let n = vfs_readdir(dir_fd, &mut buf).unwrap_or(0);
    let _ = vfs_close(dir_fd);

    let mut offset = 0;
    while offset < n {
        let mut end = offset;
        while end < n && buf[end] != 0 {
            end += 1;
        }
        if end > offset {
            if let Ok(name) = core::str::from_utf8(&buf[offset..end]) {
                // Accept names of the form "usb<digits>p<digits>" only.
                // This prevents false matches on names like "usb_port".
                if is_usb_partition_name(name) {
                    result.push(format!("/dev/block/{}", name));
                }
            }
        }
        offset = end + 1;
    }
    result
}

fn try_scan_and_mount() -> Option<(FatFs, VfsBlockDevice, PortHandle)> {
    let partitions = find_usb_partitions();
    for path in &partitions {
        let dev = VfsBlockDevice { path: path.clone() };
        if let Some(fs) = FatFs::probe(&dev) {
            info!("fatd: found {:?} filesystem on {}", fs.fat_type, path);
            let (w, r) = port_create(65536).ok()?;
            let _ = stem::syscall::vfs::vfs_mkdir("/media");
            if vfs_mount(w, DEFAULT_MOUNT_POINT).is_ok() {
                info!("fatd: mounted {} at {}", path, DEFAULT_MOUNT_POINT);
                return Some((fs, dev, r));
            }
        }
    }
    None
}

// ── Entry point ──────────────────────────────────────────────────────────────

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("fatd: starting FAT VFS provider");

    let (fs, dev, req_read) = loop {
        if let Some(res) = try_scan_and_mount() {
            break res;
        }
        stem::time::sleep_ms(500);
    };

    let mut lp = ProviderLoop::new(req_read);
    loop {
        match lp.next_request() {
            Ok(req) => {
                let resp = dispatch_request(&fs, &dev, &req);
                let _ = lp.send_response(&req, resp);
            }
            Err(e) => {
                warn!("fatd: provider loop error: {:?}", e);
                break;
            }
        }
    }

    loop {
        stem::yield_now();
    }
}
