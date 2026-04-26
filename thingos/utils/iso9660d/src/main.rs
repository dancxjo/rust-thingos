//! ISO9660 VFS Provider (iso9660d)
//!
//! Discovers storage devices in `/dev/storage/`, probes them for ISO9660
//! filesystems, and mounts the first one found as a userland VFS provider at
//! `/media/cdrom`.
#![no_std]
#![no_main]
extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use abi::errors::Errno;
use abi::vfs_rpc::VfsRpcOp;
use ipc_helpers::provider::{ProviderLoop, ProviderRequest, ProviderResponse};
use iso9660::{ISO_SECTOR_SIZE, IsoFs};
use stem::abi::module_manifest::{MANIFEST_MAGIC, ManifestHeader, ModuleKind};
use stem::block::{BlockDevice, BlockError};
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read, vfs_readdir, vfs_seek, vfs_mount, vfs_write};
use stem::syscall::{PortHandle, port_create};
use stem::{info, warn};

const DEFAULT_MOUNT_POINT: &str = "/media/cdrom";

#[unsafe(link_section = ".thing_manifest")]
#[unsafe(no_mangle)]
#[used]
pub static MANIFEST: ManifestHeader = ManifestHeader {
    magic: MANIFEST_MAGIC,
    kind: ModuleKind::Service,
    device_kind: *b"svc.vfs.iso9660\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    version: 1,
    _reserved: 0,
};

// ── VFS Block Device Adapter ────────────────────────────────────────────────

/// A VFS block device that opens, seeks, reads, and closes the backing storage
/// file on every `read_sectors` call.
///
/// The fd is intentionally **not** cached here: each `read_sectors` call is
/// independent and iso9660d is single-threaded, so a cached fd would require
/// interior mutability with no practical benefit over the open/close-per-call
/// pattern (the kernel already handles rapid open/close efficiently at the
/// driver level).
struct VfsBlockDevice {
    path: String,
}

impl BlockDevice for VfsBlockDevice {
    fn sector_size(&self) -> u64 { ISO_SECTOR_SIZE }

    fn read_sectors(&self, lba: u64, count: u64, buf: &mut [u8]) -> Result<(), BlockError> {
        let fd = vfs_open(&self.path, abi::syscall::vfs_flags::O_RDONLY).map_err(|_| BlockError::IoError)?;
        let offset = lba * ISO_SECTOR_SIZE;
        vfs_seek(fd, offset as i64, 0).map_err(|_| BlockError::IoError)?;

        let mut total = 0;
        while total < buf.len() {
            match vfs_read(fd, &mut buf[total..]) {
                Ok(0) => break,
                Ok(n) => total += n,
                Err(_) => { vfs_close(fd); return Err(BlockError::IoError); }
            }
        }
        vfs_close(fd);
        if total == buf.len() { Ok(()) } else { Err(BlockError::IoError) }
    }
}

// ── Handle encoding ─────────────────────────────────────────────────────────

fn encode_handle(lba: u32, size: u32) -> u64 { ((lba as u64) << 32) | (size as u64) }
fn decode_handle(h: u64) -> (u32, u32) { ((h >> 32) as u32, (h & 0xFFFF_FFFF) as u32) }

const S_IFDIR: u32 = 0o040000;
const S_IFREG: u32 = 0o100000;

// ── VFS RPC dispatch ────────────────────────────────────────────────────────

fn dispatch_request(fs: &IsoFs, dev: &VfsBlockDevice, req: &ProviderRequest) -> ProviderResponse {
    match req.op {
        VfsRpcOp::Lookup => handle_lookup(fs, dev, &req.payload),
        VfsRpcOp::Read => handle_read(dev, &req.payload),
        VfsRpcOp::ReadIntoFd => handle_read_into_fd(dev, &req.payload),
        VfsRpcOp::Readdir => handle_readdir(fs, dev, &req.payload),
        VfsRpcOp::Stat => handle_stat(fs, dev, &req.payload),
        VfsRpcOp::Close | VfsRpcOp::SubscribeReady | VfsRpcOp::UnsubscribeReady => ProviderResponse::ok_empty(),
        VfsRpcOp::Poll => ProviderResponse::ok_bytes(&1u32.to_le_bytes()),
        _ => ProviderResponse::err(Errno::ENOSYS),
    }
}

fn handle_lookup(fs: &IsoFs, dev: &VfsBlockDevice, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 4 { return ProviderResponse::err(Errno::EINVAL); }
    let path_len = u32::from_le_bytes(payload[0..4].try_into().unwrap()) as usize;
    let path = core::str::from_utf8(&payload[4..4+path_len]).unwrap_or("");
    
    let (lba, size) = if path.is_empty() || path == "/" {
        (fs.pvd.root_dir_extent, fs.pvd.root_dir_size)
    } else {
        match fs.lookup_path(dev, path) {
            Some(entry) => (entry.extent_lba, entry.size),
            None => return ProviderResponse::err(Errno::ENOENT),
        }
    };
    ProviderResponse::ok_u64(encode_handle(lba, size))
}

fn handle_read(dev: &VfsBlockDevice, payload: &[u8]) -> ProviderResponse {
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap());
    let offset = u64::from_le_bytes(payload[8..16].try_into().unwrap());
    let len = u32::from_le_bytes(payload[16..20].try_into().unwrap()) as usize;
    let (lba, size) = decode_handle(handle);

    if offset >= size as u64 { return ProviderResponse::ok_read(&[]); }
    let iso_file = iso9660::IsoFile { extent_lba: lba, size };
    match iso_file.read_range(dev, offset, len) {
        Ok(data) => ProviderResponse::ok_read(&data),
        Err(_) => ProviderResponse::err(Errno::EIO),
    }
}

/// Bulk-read handler: write the entire requested range directly into the
/// kernel-injected memfd (identified by `dest_fd`) using a single `vfs_write`
/// syscall.
///
/// Because `vfs_write` has no ring-buffer size limit, the full file content
/// travels in one shot, reducing the kernel↔provider IPC round-trips from
/// O(file_size / 64 KiB) to O(1).
fn handle_read_into_fd(dev: &VfsBlockDevice, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 24 { return ProviderResponse::err(Errno::EINVAL); }
    let handle  = u64::from_le_bytes(payload[0..8].try_into().unwrap());
    let offset  = u64::from_le_bytes(payload[8..16].try_into().unwrap());
    let len     = u32::from_le_bytes(payload[16..20].try_into().unwrap()) as usize;
    let dest_fd = u32::from_le_bytes(payload[20..24].try_into().unwrap());
    let (lba, size) = decode_handle(handle);

    if offset >= size as u64 {
        return ProviderResponse::ok_written(0);
    }

    let iso_file = iso9660::IsoFile { extent_lba: lba, size };
    let data = match iso_file.read_range(dev, offset, len) {
        Ok(d) => d,
        Err(_) => return ProviderResponse::err(Errno::EIO),
    };

    // Write the entire data into the kernel-allocated memfd in one syscall.
    // No IPC ring-buffer limit applies here.
    match vfs_write(dest_fd, &data) {
        Ok(n) => ProviderResponse::ok_written(n as u32),
        Err(_) => ProviderResponse::err(Errno::EIO),
    }
}

fn handle_readdir(fs: &IsoFs, dev: &VfsBlockDevice, payload: &[u8]) -> ProviderResponse {
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap());
    let offset = u64::from_le_bytes(payload[8..16].try_into().unwrap());
    let max_bytes = u32::from_le_bytes(payload[16..20].try_into().unwrap()) as usize;
    let (lba, size) = decode_handle(handle);
    let entries = fs.list_dir(dev, lba, size);

    let mut out = Vec::new();
    for entry in entries.iter().skip(offset as usize) {
        let name = entry.name.as_bytes();
        let name_len = name.len().min(255) as u8;
        let file_type: u8 = if entry.is_directory { 4 } else { 8 };
        let ino = encode_handle(entry.extent_lba, entry.size);
        if out.len() + 10 + name_len as usize > max_bytes { break; }
        out.extend_from_slice(&ino.to_le_bytes());
        out.push(file_type);
        out.push(name_len);
        out.extend_from_slice(&name[..name_len as usize]);
    }
    ProviderResponse::ok_read(&out)
}

fn handle_stat(fs: &IsoFs, dev: &VfsBlockDevice, payload: &[u8]) -> ProviderResponse {
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap());
    let (lba, size) = decode_handle(handle);
    let is_dir = lba == fs.pvd.root_dir_extent || !fs.list_dir(dev, lba, size).is_empty();
    let mode = if is_dir { S_IFDIR | 0o555 } else { S_IFREG | 0o444 };
    ProviderResponse::ok_stat(mode, size as u64, handle)
}

// ── Main ────────────────────────────────────────────────────────────────────

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("ISO9660D: Starting VFS provider");

    let (fs, dev, req_read) = loop {
        if let Some(res) = try_scan_and_mount() { break res; }
        stem::time::sleep_ms(500);
    };

    let mut lp = ProviderLoop::new(req_read);
    loop {
        if let Ok(req) = lp.next_request() {
            let resp = dispatch_request(&fs, &dev, &req);
            let _ = lp.send_response(&req, resp);
        } else { break; }
    }

    loop { stem::yield_now(); }
}

fn try_scan_and_mount() -> Option<(IsoFs, VfsBlockDevice, PortHandle)> {
    let dir_fd = vfs_open("/dev/storage", abi::syscall::vfs_flags::O_RDONLY).ok()?;
    let mut buf = [0u8; 4096];
    let n = vfs_readdir(dir_fd, &mut buf).unwrap_or(0);
    vfs_close(dir_fd);

    let mut offset = 0;
    while offset < n {
        let mut end = offset;
        while end < n && buf[end] != 0 { end += 1; }
        if end > offset {
            if let Ok(name) = core::str::from_utf8(&buf[offset..end]) {
                let path = format!("/dev/storage/{}", name);
                let dev = VfsBlockDevice { path: path.clone() };
                if let Some(fs) = IsoFs::probe(&dev) {
                    info!("ISO9660D: Found ISO on {}", path);
                    let (w, r) = port_create(65536).ok()?;
                    if vfs_mount(w, DEFAULT_MOUNT_POINT).is_ok() {
                        return Some((fs, dev, r));
                    }
                }
            }
        }
        offset = end + 1;
    }
    None
}
