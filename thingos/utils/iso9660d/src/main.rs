//! ISO9660 VFS Provider (iso9660d)
//!
//! Discovers block devices in the system graph, probes them for ISO9660
//! filesystems, and mounts the first one found as a userland VFS provider at
//! `/media/cdrom` using the thingos Act V VFS provider mechanism.
//!
//! ## How it works
//!
//! 1. **Discovery** — scans the graph for DEV_STORAGE_BLOCK_DEVICE nodes.
//! 2. **Probing** — reads each block device and looks for a valid ISO9660 PVD.
//! 3. **Mounting** — calls `SYS_FS_MOUNT(provider_port, "/media/cdrom")` so the
//!    kernel routes VFS operations here.
//! 4. **Service loop** — uses [`ipc_helpers::provider::ProviderLoop`] to read
//!    requests, dispatch them to the [`IsoFs`] library, and send typed
//!    responses back to the kernel.
//!
//! ## VFS RPC protocol
//!
//! See [`abi::vfs_rpc`] for the full wire format.  This service implements:
//! - `Lookup` — resolve a path to a provider handle (encoded as `(lba << 32) | size`)
//! - `Read`   — read file bytes from a handle
//! - `Readdir`— list directory entries for a handle
//! - `Stat`   — return mode/size/ino for a handle
//! - `Close`  — no-op (handles are stateless in this implementation)
//! - `Write`  — returns `EROFS` (ISO9660 is read-only)
//! - `Poll`   — returns `POLLIN` always
#![no_std]
#![no_main]
use alloc::string::{String, ToString};
use core::default::Default;
extern crate alloc;

use alloc::vec::Vec;

use abi::block_device_protocol::{
    BlockDeviceError, BlockDeviceRequest, BlockDeviceResponse, ReadRequest, ReadResponse,
};
use abi::errors::Errno;
use abi::seed::{
    HOST_PROGRAM, HOST_VFS_PROVIDER, INTERFACE_PROGRAM_V1, INTERFACE_VFS_PROVIDER_MOUNT_V1,
    INTERFACE_VFS_PROVIDER_UNMOUNT_V1, SEED_ABI_VERSION, Seed, SeedInterface,
};
use abi::vfs_rpc::VfsRpcOp;
use ipc_helpers::provider::{ProviderLoop, ProviderRequest, ProviderResponse};
use iso9660::{ISO_SECTOR_SIZE, IsoFs};
use stem::abi::module_manifest::{MANIFEST_MAGIC, ManifestHeader, ModuleKind};
use stem::block::{BlockDevice, BlockError};
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read, vfs_readdir, vfs_umount};
use stem::syscall::{PortHandle, argv_get, port_create, vfs_mount};
use stem::{info, warn};

const DEFAULT_MOUNT_POINT: &str = "/media/cdrom";
const SEED_NAME: &[u8] = b"iso9660d";
const HOOK_MOUNT_V1: &[u8] = b"_start";
const HOOK_UNMOUNT_V1: &[u8] = b"thingos_vfs_unmount_v1";

#[unsafe(no_mangle)]
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

// ── Block device adapter ─────────────────────────────────────────────────────

/// [`BlockDevice`] implementation that talks to a virtio/ATA driver via ports.
struct PortBlockDevice {
    port: PortHandle,
    resp_w: PortHandle,
    resp_r: PortHandle,
}

impl PortBlockDevice {
    fn new(port: PortHandle) -> Option<Self> {
        let (resp_w, resp_r) = port_create(256 * 1024).ok()?;
        Some(Self { port, resp_w, resp_r })
    }

    /// Returns the underlying storage-driver request port so that worker
    /// threads can create their own independent `PortBlockDevice` instances
    /// (each with their own response port pair) pointing at the same driver.
    fn storage_port(&self) -> PortHandle {
        self.port
    }
}

impl BlockDevice for PortBlockDevice {
    fn read_sectors(&self, lba: u64, count: u64, buf: &mut [u8]) -> Result<(), BlockError> {
        let mut req = [0u8; 4 + 1 + core::mem::size_of::<ReadRequest>()];
        req[0..4].copy_from_slice(&(self.resp_w as u32).to_le_bytes());
        req[4] = BlockDeviceRequest::Read as u8;
        let read_req = ReadRequest { lba, sector_count: count as u32 };
        let req_bytes = unsafe {
            core::slice::from_raw_parts(
                &read_req as *const ReadRequest as *const u8,
                core::mem::size_of::<ReadRequest>(),
            )
        };
        req[5..].copy_from_slice(req_bytes);
        stem::syscall::port_send(self.port, &req).map_err(|_| BlockError::IoError)?;

        let expected =
            core::mem::size_of::<ReadResponse>() + (count as usize * ISO_SECTOR_SIZE as usize) + 1;
        let mut resp_buf = alloc::vec![0u8; expected];
        let n = stem::syscall::port_recv(self.resp_r, &mut resp_buf)
            .map_err(|_| BlockError::IoError)?;
        if n < core::mem::size_of::<ReadResponse>() + 1 {
            return Err(BlockError::IoError);
        }
        let resp_type = resp_buf[0];
        if resp_type != BlockDeviceResponse::Ok as u8 {
            if resp_type == BlockDeviceResponse::Error as u8 && n >= 5 {
                return Err(match resp_buf[1] {
                    x if x == BlockDeviceError::InvalidParam as u8 => BlockError::InvalidParam,
                    x if x == BlockDeviceError::IoError as u8 => BlockError::IoError,
                    x if x == BlockDeviceError::NotReady as u8 => BlockError::NotReady,
                    x if x == BlockDeviceError::OutOfRange as u8 => BlockError::OutOfRange,
                    _ => BlockError::NotSupported,
                });
            }
            return Err(BlockError::IoError);
        }
        let header_bytes = &resp_buf[1..1 + core::mem::size_of::<ReadResponse>()];
        let data_len = u32::from_le_bytes([
            header_bytes[0],
            header_bytes[1],
            header_bytes[2],
            header_bytes[3],
        ]) as usize;
        let data_start = 1 + core::mem::size_of::<ReadResponse>();
        let data_end = data_start + data_len;
        if data_end > n || data_len > buf.len() {
            return Err(BlockError::InvalidParam);
        }
        buf[..data_len].copy_from_slice(&resp_buf[data_start..data_end]);
        Ok(())
    }

    fn sector_size(&self) -> u64 {
        ISO_SECTOR_SIZE
    }

    fn sector_count(&self) -> Option<u64> {
        None
    }
}

// ── Handle encoding ───────────────────────────────────────────────────────────

/// Encode `(lba, size)` into a 64-bit provider handle.
///
/// The high 32 bits hold the LBA, the low 32 bits hold the extent size in
/// bytes.  This encoding is identical to the one used by the old Tree Provider
/// implementation and lets us reconstruct extent info from the handle without
/// any additional state.
fn encode_handle(lba: u32, size: u32) -> u64 {
    ((lba as u64) << 32) | (size as u64)
}

fn decode_handle(h: u64) -> (u32, u32) {
    ((h >> 32) as u32, (h & 0xFFFF_FFFF) as u32)
}

// ── File type bits ────────────────────────────────────────────────────────────

const S_IFDIR: u32 = 0o040000;
const S_IFREG: u32 = 0o100000;

// ── VFS RPC dispatch ─────────────────────────────────────────────────────────

/// Dispatch one decoded VFS RPC request and return the appropriate response.
fn dispatch_request(fs: &IsoFs, dev: &PortBlockDevice, req: &ProviderRequest) -> ProviderResponse {
    match req.op {
        VfsRpcOp::Lookup => handle_lookup(fs, dev, &req.payload),
        VfsRpcOp::Read => handle_read(dev, &req.payload),
        VfsRpcOp::Write => ProviderResponse::err(Errno::EROFS),
        VfsRpcOp::Readdir => handle_readdir(fs, dev, &req.payload),
        VfsRpcOp::Stat => handle_stat(fs, dev, &req.payload),
        VfsRpcOp::Close => ProviderResponse::ok_empty(),
        VfsRpcOp::Poll => {
            // Always report readable (POLLIN = 1).
            ProviderResponse::ok_bytes(&1u32.to_le_bytes())
        }
        VfsRpcOp::SubscribeReady | VfsRpcOp::UnsubscribeReady => ProviderResponse::ok_empty(),
        VfsRpcOp::AttrGet | VfsRpcOp::AttrSet | VfsRpcOp::AttrRemove | VfsRpcOp::AttrList => {
            ProviderResponse::err(Errno::ENOTSUP)
        }
        _ => ProviderResponse::err(Errno::ENOSYS),
    }
}

/// LOOKUP: resolve a path within the ISO and return a handle.
fn handle_lookup(fs: &IsoFs, dev: &PortBlockDevice, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 4 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let path_len = u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]) as usize;
    if payload.len() < 4 + path_len {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let path_bytes = &payload[4..4 + path_len];
    let path = match core::str::from_utf8(path_bytes) {
        Ok(p) => p,
        Err(_) => return ProviderResponse::err(Errno::EINVAL),
    };

    // Empty path means the mount-point root directory.
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

/// READ: read file data from a handle.
fn handle_read(dev: &PortBlockDevice, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 20 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap());
    let offset = u64::from_le_bytes(payload[8..16].try_into().unwrap());
    let len = u32::from_le_bytes(payload[16..20].try_into().unwrap()) as usize;

    let (lba, size) = decode_handle(handle);

    if offset >= size as u64 {
        // Past EOF — return 0 bytes (ok_read prepends the length).
        return ProviderResponse::ok_read(&[]);
    }

    let iso_file = iso9660::IsoFile { extent_lba: lba, size };
    let clamped_len = len.min((size as u64 - offset) as usize);

    match iso_file.read_range(dev, offset, clamped_len) {
        Ok(data) => ProviderResponse::ok_read(&data),
        Err(_) => ProviderResponse::err(Errno::EIO),
    }
}

/// READDIR: list directory entries.
///
/// The `offset` parameter is treated as an entry index (not a byte offset).
fn handle_readdir(fs: &IsoFs, dev: &PortBlockDevice, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 20 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap());
    let offset = u64::from_le_bytes(payload[8..16].try_into().unwrap());
    let max_bytes = u32::from_le_bytes(payload[16..20].try_into().unwrap()) as usize;

    let (lba, size) = decode_handle(handle);
    let entries = fs.list_dir(dev, lba, size);

    let start_idx = offset as usize;
    let mut out: Vec<u8> = Vec::new();

    for entry in entries.iter().skip(start_idx) {
        let name_bytes = entry.name.as_bytes();
        let name_len = name_bytes.len().min(255) as u8;
        let file_type: u8 = if entry.is_directory { 4 } else { 8 }; // DT_DIR=4, DT_REG=8
        let ino = encode_handle(entry.extent_lba, entry.size);

        // DirentWire: [ino: u64][file_type: u8][name_len: u8][name...]
        let entry_size = 10 + name_len as usize;
        if out.len() + entry_size > max_bytes {
            break;
        }
        out.extend_from_slice(&ino.to_le_bytes());
        out.push(file_type);
        out.push(name_len);
        out.extend_from_slice(&name_bytes[..name_len as usize]);
    }

    ProviderResponse::ok_read(&out)
}

/// STAT: return metadata for a handle.
fn handle_stat(fs: &IsoFs, dev: &PortBlockDevice, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 8 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes(payload[0..8].try_into().unwrap());
    let (lba, size) = decode_handle(handle);

    let is_dir = if lba == fs.pvd.root_dir_extent {
        true
    } else {
        let entries = fs.list_dir(dev, lba, size);
        !entries.is_empty()
    };

    let mode = if is_dir { S_IFDIR | 0o555 } else { S_IFREG | 0o444 };

    ProviderResponse::ok_stat(mode, size as u64, handle)
}

// ── Main ─────────────────────────────────────────────────────────────────────

const DISCOVERY_RETRY_MS: u64 = 500;
const DISCOVERY_LOG_EVERY_ATTEMPTS: u32 = 20;

/// Send a provider response directly to the kernel without going through a
/// `ProviderLoop`.  Used by worker threads that own the request metadata but
/// not the loop itself.
fn send_response_direct(resp_port: PortHandle, req_id: u16, response: ProviderResponse) {
    use abi::vfs_rpc::VFS_RPC_MAX_RESP;

    let total = 3 + response.payload.len();
    let mut buf = alloc::vec![0u8; total.min(VFS_RPC_MAX_RESP)];
    buf[0..2].copy_from_slice(&req_id.to_le_bytes());
    buf[2] = response.status;
    let payload_len = response.payload.len().min(buf.len() - 3);
    buf[3..3 + payload_len].copy_from_slice(&response.payload[..payload_len]);
    let _ = stem::syscall::port_send_all(resp_port, &buf[..3 + payload_len]);
}

/// Read file bytes from a block device using pre-decoded handle fields.
///
/// This mirrors [`handle_read`] but takes `(lba, size, offset, len)` directly
/// so the function can be called from a worker thread that received these
/// values as `Copy` captures rather than a borrowed payload slice.
fn handle_read_direct(
    dev: &PortBlockDevice,
    lba: u32,
    size: u32,
    offset: u64,
    len: usize,
) -> ProviderResponse {
    if offset >= size as u64 {
        return ProviderResponse::ok_read(&[]);
    }
    let iso_file = iso9660::IsoFile { extent_lba: lba, size };
    let clamped_len = len.min((size as u64 - offset) as usize);
    match iso_file.read_range(dev, offset, clamped_len) {
        Ok(data) => ProviderResponse::ok_read(&data),
        Err(_) => ProviderResponse::err(Errno::EIO),
    }
}

/// List directory entries using pre-decoded handle fields.
///
/// This mirrors [`handle_readdir`] but takes decoded parameters directly so
/// the function can be called from a worker thread.  A fresh [`IsoFs`] is
/// constructed from the provided PVD clone, giving each worker thread its own
/// independent directory cache without any shared-state contention.
fn handle_readdir_direct(
    dev: &PortBlockDevice,
    pvd: iso9660::PrimaryVolumeDescriptor,
    lba: u32,
    size: u32,
    offset: u64,
    max_bytes: usize,
) -> ProviderResponse {
    let fs = IsoFs::with_pvd(pvd);
    let entries = fs.list_dir(dev, lba, size);
    let start_idx = offset as usize;
    let mut out: Vec<u8> = Vec::new();
    for entry in entries.iter().skip(start_idx) {
        let name_bytes = entry.name.as_bytes();
        let name_len = name_bytes.len().min(255) as u8;
        let file_type: u8 = if entry.is_directory { 4 } else { 8 };
        let ino = encode_handle(entry.extent_lba, entry.size);
        let entry_size = 10 + name_len as usize;
        if out.len() + entry_size > max_bytes {
            break;
        }
        out.extend_from_slice(&ino.to_le_bytes());
        out.push(file_type);
        out.push(name_len);
        out.extend_from_slice(&name_bytes[..name_len as usize]);
    }
    ProviderResponse::ok_read(&out)
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let mount_point = mount_point_from_args();
    stem::debug!("iso9660d: starting ISO9660 VFS provider");

    // Keep probing until storage services and an ISO9660-capable block device appear.
    let mut attempts = 0u32;
    let (fs, dev, req_read) = loop {
        if let Some(mounted) = try_mount_iso9660(&mount_point) {
            break mounted;
        }

        attempts = attempts.saturating_add(1);
        if attempts == 1 || attempts % DISCOVERY_LOG_EVERY_ATTEMPTS == 0 {
            info!("iso9660d: no ISO9660 filesystem found yet — retrying");
        }
        stem::time::sleep_ms(DISCOVERY_RETRY_MS);
    };

    // 5. Service loop using ProviderLoop — far less boilerplate than raw
    //    port_recv + manual header parsing.
    info!("iso9660d: entering VFS RPC service loop");
    let storage_port = dev.storage_port();
    let mut lp = ProviderLoop::new(req_read);
    loop {
        let req = match lp.next_request() {
            Ok(r) => r,
            Err(_) => break, // port closed — exit cleanly
        };

        // Dispatch Read requests to a background thread so multiple
        // concurrent block-device reads can proceed in parallel without
        // stalling Lookup / Stat on the main thread.
        if req.op == VfsRpcOp::Read {
            if req.payload.len() >= 20 {
                let handle = u64::from_le_bytes(req.payload[0..8].try_into().unwrap());
                let offset = u64::from_le_bytes(req.payload[8..16].try_into().unwrap());
                let len = u32::from_le_bytes(req.payload[16..20].try_into().unwrap()) as usize;
                let (lba, size) = decode_handle(handle);
                let resp_port = req.resp_port;
                let req_id = req.req_id;
                let port = storage_port;

                let spawn_result = stem::thread::spawn_task_detached(move || {
                    let dev = match PortBlockDevice::new(port) {
                        Some(d) => d,
                        None => {
                            send_response_direct(
                                resp_port,
                                req_id,
                                ProviderResponse::err(Errno::EIO),
                            );
                            return;
                        }
                    };
                    let resp = handle_read_direct(&dev, lba, size, offset, len);
                    send_response_direct(resp_port, req_id, resp);
                });

                if spawn_result.is_err() {
                    // Thread spawn failed — fall back to synchronous dispatch.
                    let resp = handle_read(&dev, &req.payload);
                    lp.send_response(&req, resp).ok();
                }
            } else {
                lp.send_response(&req, ProviderResponse::err(Errno::EINVAL)).ok();
            }
            continue;
        }

        // Dispatch Readdir requests to a background thread so concurrent
        // directory listings don't block each other or file reads.
        if req.op == VfsRpcOp::Readdir {
            if req.payload.len() >= 20 {
                let handle = u64::from_le_bytes(req.payload[0..8].try_into().unwrap());
                let offset = u64::from_le_bytes(req.payload[8..16].try_into().unwrap());
                let max_bytes =
                    u32::from_le_bytes(req.payload[16..20].try_into().unwrap()) as usize;
                let (lba, size) = decode_handle(handle);
                let resp_port = req.resp_port;
                let req_id = req.req_id;
                let port = storage_port;
                let pvd = fs.pvd.clone();

                let spawn_result = stem::thread::spawn_task_detached(move || {
                    let dev = match PortBlockDevice::new(port) {
                        Some(d) => d,
                        None => {
                            send_response_direct(
                                resp_port,
                                req_id,
                                ProviderResponse::err(Errno::EIO),
                            );
                            return;
                        }
                    };
                    let resp = handle_readdir_direct(&dev, pvd, lba, size, offset, max_bytes);
                    send_response_direct(resp_port, req_id, resp);
                });

                if spawn_result.is_err() {
                    // Thread spawn failed — fall back to synchronous dispatch.
                    let resp = handle_readdir(&fs, &dev, &req.payload);
                    lp.send_response(&req, resp).ok();
                }
            } else {
                lp.send_response(&req, ProviderResponse::err(Errno::EINVAL)).ok();
            }
            continue;
        }

        let resp = dispatch_request(&fs, &dev, &req);
        lp.send_response(&req, resp).ok();
    }

    info!("iso9660d: provider port closed — exiting");
    loop {
        stem::syscall::yield_now();
    }
}

fn try_mount_iso9660(mount_point: &str) -> Option<(IsoFs, PortBlockDevice, PortHandle)> {
    let fd = vfs_open("/services/storage", abi::syscall::vfs_flags::O_RDONLY).ok()?;
    let mut mounted = None;

    let mut buf = [0u8; 4096];
    if let Ok(n) = vfs_readdir(fd, &mut buf) {
        let mut offset = 0;
        while offset < n {
            let mut end = offset;
            while end < n && buf[end] != 0 {
                end += 1;
            }
            if end > offset {
                if let Ok(name) = core::str::from_utf8(&buf[offset..end]) {
                    let path = alloc::format!("/services/storage/{}", name);
                    if let Ok(h_fd) = vfs_open(&path, abi::syscall::vfs_flags::O_RDONLY) {
                        let mut h_buf = [0u8; 32];
                        if let Ok(h_n) = vfs_read(h_fd, &mut h_buf) {
                            let h_str = core::str::from_utf8(&h_buf[..h_n]).unwrap_or("");
                            if let Ok(port_handle) = h_str.trim().parse::<u32>() {
                                let block_dev =
                                    match PortBlockDevice::new(port_handle as PortHandle) {
                                        Some(d) => d,
                                        None => {
                                            let _ = vfs_close(h_fd);
                                            offset = end + 1;
                                            continue;
                                        }
                                    };

                                if let Some(fs) = IsoFs::probe(&block_dev) {
                                    info!("iso9660d: found ISO9660 on device {}", name);

                                    let (req_write, req_read) = match port_create(
                                        abi::vfs_rpc::VFS_RPC_MAX_REQ * 8,
                                    ) {
                                        Ok(p) => p,
                                        Err(e) => {
                                            warn!(
                                                "iso9660d: failed to create provider port: {:?}",
                                                e
                                            );
                                            let _ = vfs_close(h_fd);
                                            offset = end + 1;
                                            continue;
                                        }
                                    };

                                    match vfs_mount(req_write, mount_point) {
                                        Ok(()) => {
                                            info!(
                                                "iso9660d: mounted at {} (req_read={})",
                                                mount_point, req_read
                                            );
                                            mounted = Some((fs, block_dev, req_read));
                                            let _ = vfs_close(h_fd);
                                            break;
                                        }
                                        Err(e) => {
                                            warn!("iso9660d: vfs_mount failed: {:?}", e);
                                        }
                                    }
                                }
                            }
                        }
                        let _ = vfs_close(h_fd);
                    }
                }
            }
            offset = end + 1;
        }
    }
    let _ = vfs_close(fd);
    mounted
}

fn mount_point_from_args() -> String {
    let len = match argv_get(&mut []) {
        Ok(l) if l > 0 => l,
        _ => return DEFAULT_MOUNT_POINT.to_string(),
    };
    let mut buf = alloc::vec![0u8; len];
    if argv_get(&mut buf).is_err() {
        return DEFAULT_MOUNT_POINT.to_string();
    }
    let args = stem::utils::parse_argv(&buf);
    if args.len() >= 2 {
        if let Ok(path) = core::str::from_utf8(args[1]) {
            if !path.is_empty() {
                return path.to_string();
            }
        }
    }
    DEFAULT_MOUNT_POINT.to_string()
}

#[used]
static KEEP_THINGOS_VFS_MOUNT_V1: extern "C" fn(usize) -> ! = thingos_vfs_mount_v1;

#[used]
static KEEP_THINGOS_VFS_UNMOUNT_V1: extern "C" fn(usize) -> i32 = thingos_vfs_unmount_v1;

#[unsafe(no_mangle)]
pub extern "C" fn thingos_vfs_mount_v1(arg: usize) -> ! {
    unsafe { stem::rt::entry_impl(arg) }
}

#[unsafe(no_mangle)]
pub extern "C" fn thingos_vfs_unmount_v1(_arg: usize) -> i32 {
    let mount_point = mount_point_from_args();
    match vfs_umount(&mount_point) {
        Ok(()) => 0,
        Err(_) => 1,
    }
}
