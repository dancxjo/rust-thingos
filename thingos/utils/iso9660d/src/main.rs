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
use stem::syscall::vfs::{vfs_close, vfs_mount, vfs_open, vfs_read, vfs_readdir, vfs_seek};
use stem::syscall::{PortHandle, argv_get, port_create};
use stem::{info, warn};

const DEFAULT_MOUNT_POINT: &str = "/media/cdrom";

struct MountConfig {
    device: Option<String>,
    mount_point: String,
}

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
    fn sector_size(&self) -> u64 {
        ISO_SECTOR_SIZE
    }

    fn read_sectors(&self, lba: u64, _count: u64, buf: &mut [u8]) -> Result<(), BlockError> {
        let fd = vfs_open(&self.path, abi::syscall::vfs_flags::O_RDONLY)
            .map_err(|_| BlockError::IoError)?;
        let offset = lba * ISO_SECTOR_SIZE;
        vfs_seek(fd, offset as i64, 0).map_err(|_| BlockError::IoError)?;

        let mut total = 0;
        while total < buf.len() {
            match vfs_read(fd, &mut buf[total..]) {
                Ok(0) => break,
                Ok(n) => total += n,
                Err(_) => {
                    let _ = vfs_close(fd);
                    return Err(BlockError::IoError);
                }
            }
        }
        let _ = vfs_close(fd);
        if total == buf.len() { Ok(()) } else { Err(BlockError::IoError) }
    }
}

// ── Handle encoding ─────────────────────────────────────────────────────────

const HANDLE_DIR_FLAG: u32 = 1 << 31;

fn encode_handle(lba: u32, size: u32, is_dir: bool) -> u64 {
    let encoded_size = if is_dir { size | HANDLE_DIR_FLAG } else { size };
    ((lba as u64) << 32) | (encoded_size as u64)
}

fn decode_handle(h: u64) -> (u32, u32, bool) {
    let encoded_size = h as u32;
    let is_dir = (encoded_size & HANDLE_DIR_FLAG) != 0;
    let size = if is_dir { encoded_size & !HANDLE_DIR_FLAG } else { encoded_size };
    ((h >> 32) as u32, size, is_dir)
}

const S_IFDIR: u32 = 0o040000;
const S_IFREG: u32 = 0o100000;
const MAX_READ_CHUNK: usize = 16 * 1024;

// ── VFS RPC dispatch ────────────────────────────────────────────────────────

fn dispatch_request(fs: &IsoFs, dev: &VfsBlockDevice, req: &ProviderRequest) -> ProviderResponse {
    match req.op {
        VfsRpcOp::Lookup => handle_lookup(fs, dev, &req.payload),
        VfsRpcOp::Read => handle_read(dev, &req.payload),
        VfsRpcOp::ReadIntoFd => ProviderResponse::err(Errno::ENOSYS),
        VfsRpcOp::Readdir => handle_readdir(fs, dev, &req.payload),
        VfsRpcOp::Stat => handle_stat(&req.payload),
        VfsRpcOp::Close | VfsRpcOp::SubscribeReady | VfsRpcOp::UnsubscribeReady => {
            ProviderResponse::ok_empty()
        }
        VfsRpcOp::Poll => ProviderResponse::ok_bytes(&1u32.to_le_bytes()),
        _ => ProviderResponse::err(Errno::ENOSYS),
    }
}

fn handle_lookup(fs: &IsoFs, dev: &VfsBlockDevice, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 4 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let path_len = u32::from_le_bytes(payload[0..4].try_into().unwrap()) as usize;
    let path = core::str::from_utf8(&payload[4..4 + path_len]).unwrap_or("");

    let (lba, size, is_dir) = if path.is_empty() || path == "/" {
        (fs.pvd.root_dir_extent, fs.pvd.root_dir_size, true)
    } else {
        match fs.lookup_path(dev, path) {
            Some(entry) => (entry.extent_lba, entry.size, entry.is_directory),
            None => return ProviderResponse::err(Errno::ENOENT),
        }
    };
    ProviderResponse::ok_u64(encode_handle(lba, size, is_dir))
}

fn handle_read(dev: &VfsBlockDevice, payload: &[u8]) -> ProviderResponse {
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap());
    let offset = u64::from_le_bytes(payload[8..16].try_into().unwrap());
    let len = u32::from_le_bytes(payload[16..20].try_into().unwrap()) as usize;
    let (lba, size, is_dir) = decode_handle(handle);

    if is_dir {
        return ProviderResponse::err(Errno::EISDIR);
    }

    if offset >= size as u64 {
        return ProviderResponse::ok_read(&[]);
    }
    let iso_file = iso9660::IsoFile { extent_lba: lba, size };
    match iso_file.read_range(dev, offset, len.min(MAX_READ_CHUNK)) {
        Ok(data) => ProviderResponse::ok_read(&data),
        Err(_) => ProviderResponse::err(Errno::EIO),
    }
}

fn handle_readdir(fs: &IsoFs, dev: &VfsBlockDevice, payload: &[u8]) -> ProviderResponse {
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap());
    let offset = u64::from_le_bytes(payload[8..16].try_into().unwrap());
    let max_bytes = u32::from_le_bytes(payload[16..20].try_into().unwrap()) as usize;
    let (lba, size, is_dir) = decode_handle(handle);
    if !is_dir {
        return ProviderResponse::err(Errno::ENOTDIR);
    }
    let entries = fs.list_dir(dev, lba, size);

    let mut out = Vec::new();
    for entry in entries.iter().skip(offset as usize) {
        let name = entry.name.as_bytes();
        let name_len = name.len().min(255) as u8;
        let file_type: u8 = if entry.is_directory { 4 } else { 8 };
        let ino = encode_handle(entry.extent_lba, entry.size, entry.is_directory);
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

fn handle_stat(payload: &[u8]) -> ProviderResponse {
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap());
    let (_, size, is_dir) = decode_handle(handle);
    let mode = if is_dir { S_IFDIR | 0o555 } else { S_IFREG | 0o444 };
    ProviderResponse::ok_stat(mode, size as u64, handle)
}

// ── Main ────────────────────────────────────────────────────────────────────

#[stem::main]
fn main(_arg: usize) -> ! {
    let config = mount_config_from_args();
    if let Some(device) = config.device.as_ref() {
        info!("ISO9660D: Starting VFS provider for {} at {}", device, config.mount_point);
    } else {
        info!("ISO9660D: Starting VFS provider scan for {}", config.mount_point);
    }

    let (fs, dev, req_read) = loop {
        if let Some(res) = try_mount_config(&config) {
            break res;
        }
        stem::time::sleep_ms(500);
    };

    let mut lp = ProviderLoop::new(req_read);
    loop {
        if let Ok(req) = lp.next_request() {
            let resp = dispatch_request(&fs, &dev, &req);
            let _ = lp.send_response(&req, resp);
        } else {
            break;
        }
    }

    loop {
        stem::yield_now();
    }
}

fn mount_config_from_args() -> MountConfig {
    let args = get_args();
    if args.len() >= 2 {
        let device = device_arg(&args[0]);
        return MountConfig { device, mount_point: args[1].clone() };
    }
    if args.len() == 1 {
        return MountConfig { device: None, mount_point: args[0].clone() };
    }
    MountConfig { device: None, mount_point: DEFAULT_MOUNT_POINT.to_string() }
}

fn get_args() -> Vec<String> {
    let len = match argv_get(&mut []) {
        Ok(l) if l > 0 => l,
        _ => return Vec::new(),
    };
    let mut buf = alloc::vec![0u8; len];
    if argv_get(&mut buf).is_err() {
        return Vec::new();
    }

    stem::utils::parse_argv(&buf)
        .into_iter()
        .skip(1)
        .filter_map(|b| core::str::from_utf8(b).ok().map(String::from))
        .collect()
}

fn device_arg(arg: &str) -> Option<String> {
    if arg.is_empty() || arg == "none" { None } else { Some(arg.to_string()) }
}

fn try_mount_config(config: &MountConfig) -> Option<(IsoFs, VfsBlockDevice, PortHandle)> {
    if let Some(device) = config.device.as_ref() {
        let name = device.trim_start_matches("/dev/storage/");
        return try_mount_device(device, name, &config.mount_point);
    }

    try_scan_and_mount(&config.mount_point)
}

fn try_scan_and_mount(mount_point: &str) -> Option<(IsoFs, VfsBlockDevice, PortHandle)> {
    try_scan_dir_and_mount("/dev/storage", mount_point)
        .or_else(|| try_scan_dir_and_mount("/dev/block", mount_point))
        .or_else(|| try_probe_usb_block_paths(mount_point))
}

fn try_scan_dir_and_mount(
    dir_path: &str,
    mount_point: &str,
) -> Option<(IsoFs, VfsBlockDevice, PortHandle)> {
    let dir_fd = vfs_open(dir_path, abi::syscall::vfs_flags::O_RDONLY).ok()?;
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
                let path = format!("{}/{}", dir_path, name);
                if let Some(mounted) = try_mount_device(&path, name, mount_point) {
                    return Some(mounted);
                }
            }
        }
        offset = end + 1;
    }
    None
}

fn try_probe_usb_block_paths(mount_point: &str) -> Option<(IsoFs, VfsBlockDevice, PortHandle)> {
    for disk in 0..4 {
        let path = format!("/dev/block/usb{}", disk);
        if let Some(mounted) = try_mount_device(&path, &path, mount_point) {
            return Some(mounted);
        }

        for part in 1..8 {
            let path = format!("/dev/block/usb{}p{}", disk, part);
            if let Some(mounted) = try_mount_device(&path, &path, mount_point) {
                return Some(mounted);
            }
        }
    }
    None
}

fn try_mount_device(
    device_path: &str,
    device_name: &str,
    mount_point: &str,
) -> Option<(IsoFs, VfsBlockDevice, PortHandle)> {
    let fd = vfs_open(device_path, abi::syscall::vfs_flags::O_RDONLY).ok()?;
    let _ = vfs_close(fd);

    info!("iso9660d: probing device {}", device_name);
    let dev = VfsBlockDevice { path: device_path.to_string() };
    let fs = IsoFs::probe(&dev)?;
    info!("iso9660d: found ISO9660 on device {}", device_name);
    let (w, r) = port_create(65536).ok()?;
    match vfs_mount(w, mount_point) {
        Ok(()) => {
            info!("iso9660d: mounted at {}", mount_point);
            Some((fs, dev, r))
        }
        Err(e) => {
            warn!("iso9660d: failed to mount at {}: {:?}", mount_point, e);
            None
        }
    }
}
