//! VirtIO sound driver — VFS-first PCM audio.
//!
//! This driver exposes the sound card as a small VFS tree at
//! `/dev/audio/card0/`:
//!
//! ```text
//! /dev/audio/card0/
//!     ctl     — control node (AUDIO_GET_INFO, future mixer controls)
//!     out0    — PCM playback stream (write PCM here, poll POLLOUT for space)
//! ```
//!
//! Apps open `out0`, configure it with an `AUDIO_SET_PARAMS` device call,
//! send `AUDIO_START`, then stream PCM via `write()`.  `poll(POLLOUT)` signals
//! when the driver's ring buffer has room.
//!
//! The driver runs a single-threaded event loop that services VFS RPC requests
//! from the kernel (via the provider channel) and feeds the virtio TX queue
//! from the internal ring buffer.
//!
//! ## IPC substrate
//!
//! VFS RPC handling uses [`ipc_helpers::provider::ProviderLoop::try_next_request`]
//! (non-blocking) so that hardware DMA feeding can interleave with RPC
//! dispatch without deadlock.  Response building uses [`ProviderResponse`]
//! helpers rather than hand-assembled byte vectors.
#![no_std]
#![no_main]
use alloc::string::{String, ToString};
extern crate alloc;

mod spec;

use alloc::vec::Vec;
use core::mem::size_of;

use abi::device::DeviceKind;
use abi::driver_interface::{
    BusKind, DRIVER_DESCRIPTOR_ABI_VERSION, DRIVER_FLAG_PCI, DRIVER_INTERFACE_ABI_VERSION,
    DeviceInfo, DriverClass, DriverDescriptor, DriverEntryCtx, DriverInterfaceV1,
    DriverStartContext, ProbeResult, Status,
};
use abi::errors::Errno;
use abi::sound::{
    AUDIO_DRAIN, AUDIO_GET_INFO, AUDIO_GET_MAPPED_RING_INFO, AUDIO_GET_MAPPED_RING_STATS,
    AUDIO_GET_PARAMS, AUDIO_GET_STATUS, AUDIO_MAPPED_RING_VERSION, AUDIO_SET_PARAMS, AUDIO_START,
    AUDIO_STOP, AudioMappedRingHeader, AudioMappedRingInfo, AudioMappedRingSetup,
    AudioMappedRingStats, AudioParams, AudioSampleFormat, AudioState, AudioStatus, AudioStreamInfo,
    format_bit,
};
use abi::vfs_rpc::{VFS_RPC_MAX_REQ, VfsRpcOp};
use ipc_helpers::provider::{ProviderLoop, ProviderResponse};
use spec::*;
use stem::syscall::channel::channel_create;
use stem::syscall::vfs::vfs_mount;
use stem::{error, info, warn};
use virtio::device::VirtioDevice;

const AUDIO_RING_SOCKET_PATH: &str = "/run/audio-card0.sock";
const AUDIO_RING_SUGGESTED_BYTES: usize = 128 * 1024;

#[unsafe(no_mangle)]
#[used]
pub static THING_DRIVER_V1: DriverInterfaceV1 = DriverInterfaceV1 {
    abi_version: DRIVER_INTERFACE_ABI_VERSION,
    flags: DRIVER_FLAG_PCI,
    vendor_id: 0x1af4,
    device_id: 0,
    class_code: 0x040100,
    class_mask: 0xffff00,
    entry_symbol: [0; 32],
};

const THINGOS_DRIVER_NAME: &[u8] = b"virtio_sound";

#[cfg(target_arch = "x86_64")]
unsafe extern "C" {
    fn thingos_driver_start_safe(ctx: *const DriverStartContext) -> Status;
}

#[unsafe(no_mangle)]
#[used]
pub static THINGOS_DRIVER: DriverDescriptor = DriverDescriptor {
    abi_version: DRIVER_DESCRIPTOR_ABI_VERSION,
    driver_name_ptr: THINGOS_DRIVER_NAME.as_ptr(),
    driver_name_len: THINGOS_DRIVER_NAME.len(),
    driver_class: DriverClass::Audio,
    flags: 0,
    probe: thingos_driver_probe,
    #[cfg(target_arch = "x86_64")]
    start: thingos_driver_start_safe,
    #[cfg(not(target_arch = "x86_64"))]
    start: thingos_driver_start_rust,
};

unsafe extern "C" fn thingos_driver_probe(dev: *const DeviceInfo, out: *mut ProbeResult) -> Status {
    if dev.is_null() || out.is_null() {
        return Status::InvalidArgument;
    }
    let dev = &*dev;
    let out = &mut *out;
    let is_match = dev.bus == BusKind::Pci as u32
        && dev.vendor_id as u16 == 0x1af4
        && (dev.class_code & 0xffff00) == 0x040100;
    out.matched = if is_match { 1 } else { 0 };
    out.score = if is_match { 1000 } else { 0 };
    out.claimed_class = DriverClass::Audio;
    out.flags = 0;
    if is_match { Status::Ok } else { Status::NoMatch }
}

#[cfg(target_arch = "x86_64")]
core::arch::global_asm!(
    r#"
    .section .text
    .global thingos_driver_start_safe
    thingos_driver_start_safe:
        // RSP = 16n (kernel spawn)
        sub rsp, 8
        push rdi
        // Call std initialization (TLS, etc)
        call thingos_runtime_setup
        // Restore RDI and realign for the next call.
        pop rdi
        add rsp, 8
        // CALL will push 8 bytes, so inside Rust entry RSP = 16n + 8.
        call thingos_driver_start_rust
        ret
"#
);

#[unsafe(no_mangle)]
unsafe extern "C" fn thingos_driver_start_rust(boot_fd: usize) -> Status {
    main(boot_fd)
}

// ── VirtIO queue indices ──────────────────────────────────────────────────────

const QUEUE_SIZE: u16 = 64;
const EVENT_QUEUE_SLOTS: usize = 8;
const INVALID_EVENT_SLOT: u8 = 0xFF;
const INVALID_DESC: u16 = u16::MAX;

// ── VFS provider handle constants ─────────────────────────────────────────────

/// Handle for the root directory of the card.
const HANDLE_ROOT: u64 = 0;
/// Handle for `ctl`.
const HANDLE_CTL: u64 = 1;
/// Handle for `out0` (playback stream).
const HANDLE_OUT0: u64 = 2;

// ── File-type bits ────────────────────────────────────────────────────────────

const S_IFDIR: u32 = 0o040_000;
const S_IFREG: u32 = 0o100_000;

// ── Poll bits ─────────────────────────────────────────────────────────────────

const POLLOUT: u16 = abi::syscall::poll_flags::POLLOUT;
const POLLIN: u16 = abi::syscall::poll_flags::POLLIN;
const POLLHUP: u16 = abi::syscall::poll_flags::POLLHUP;
const POLLERR: u16 = abi::syscall::poll_flags::POLLERR;

// ── Internal ring buffer ──────────────────────────────────────────────────────

/// Fixed-capacity ring buffer used to decouple app writes from DMA submission.
struct RingBuf {
    data: Vec<u8>,
    head: usize, // next read position
    tail: usize, // next write position
    len: usize,
    cap: usize,
}

impl RingBuf {
    fn new(capacity: usize) -> Self {
        let cap = capacity.max(1);
        let mut data = Vec::with_capacity(cap);
        data.resize(cap, 0u8);
        Self { data, head: 0, tail: 0, len: 0, cap }
    }

    fn free_space(&self) -> usize {
        self.cap - self.len
    }
    fn available(&self) -> usize {
        self.len
    }

    /// Enqueue up to `src.len()` bytes. Returns bytes written.
    fn enqueue(&mut self, src: &[u8]) -> usize {
        let n = src.len().min(self.free_space());
        for i in 0..n {
            self.data[self.tail] = src[i];
            self.tail = (self.tail + 1) % self.cap;
        }
        self.len += n;
        n
    }

    /// Dequeue up to `dst.len()` bytes. Returns bytes read.
    fn dequeue(&mut self, dst: &mut [u8]) -> usize {
        let n = dst.len().min(self.len);
        for i in 0..n {
            dst[i] = self.data[self.head];
            self.head = (self.head + 1) % self.cap;
        }
        self.len -= n;
        n
    }
}

// ── Audio card state ──────────────────────────────────────────────────────────

/// Mutable state shared between the VFS provider loop and hardware feed loop.
struct AudioCard {
    ring: RingBuf,
    mapped: Option<MappedRing>,
    mapped_attach_count: u32,
    mapped_detach_count: u32,
    mapped_underrun_events: u32,
    mapped_bytes_consumed: u64,
    mapped_was_empty: bool,
    params: AudioParams,
    state: u32, // AudioState as u32
    hw_frame: u64,
    app_frame: u64,
    xruns: u32,
    hw_config_dirty: bool,
    hw_started: bool,
    hw_faulted: bool,
    /// True when at least one SubscribeReady request has been received for out0
    /// and no matching UnsubscribeReady has been received.
    out0_subscribed: bool,
}

struct MappedRing {
    control_fd: u32,
    ring_fd: u32,
    map_addr: usize,
    map_len: usize,
}

fn mapped_header_mut(mapped: &mut MappedRing) -> &mut AudioMappedRingHeader {
    unsafe { &mut *(mapped.map_addr as *mut AudioMappedRingHeader) }
}

fn mapped_header_ptr(mapped: &MappedRing) -> *mut AudioMappedRingHeader {
    mapped.map_addr as *mut AudioMappedRingHeader
}

fn mapped_data_ptr(mapped: &MappedRing) -> *mut u8 {
    (mapped.map_addr + core::mem::size_of::<AudioMappedRingHeader>()) as *mut u8
}

fn mapped_dequeue(mapped: &mut MappedRing, dst: &mut [u8]) -> usize {
    let hdr = mapped_header_ptr(mapped);
    let cap =
        unsafe { core::ptr::read_volatile(core::ptr::addr_of!((*hdr).capacity_bytes)) } as usize;
    if cap == 0 {
        return 0;
    }

    let w = unsafe { core::ptr::read_volatile(core::ptr::addr_of!((*hdr).write_index)) } as usize;
    core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);

    let mut r =
        unsafe { core::ptr::read_volatile(core::ptr::addr_of!((*hdr).read_index)) } as usize;
    let avail = if w >= r { w - r } else { cap - (r - w) };
    let n = avail.min(dst.len());
    if n == 0 {
        return 0;
    }

    let base = mapped_data_ptr(mapped);
    let first = n.min(cap - r);
    unsafe {
        core::ptr::copy_nonoverlapping(base.add(r), dst.as_mut_ptr(), first);
    }
    if first < n {
        unsafe {
            core::ptr::copy_nonoverlapping(base, dst.as_mut_ptr().add(first), n - first);
        }
    }

    r = (r + n) % cap;
    core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
    unsafe {
        core::ptr::write_volatile(core::ptr::addr_of_mut!((*hdr).read_index), r as u32);
    }
    n
}

fn mapped_fill_and_capacity(mapped: &MappedRing) -> (usize, usize) {
    let hdr = mapped_header_ptr(mapped);
    let cap =
        unsafe { core::ptr::read_volatile(core::ptr::addr_of!((*hdr).capacity_bytes)) } as usize;
    if cap == 0 {
        return (0, 0);
    }
    let w = unsafe { core::ptr::read_volatile(core::ptr::addr_of!((*hdr).write_index)) } as usize;
    let r = unsafe { core::ptr::read_volatile(core::ptr::addr_of!((*hdr).read_index)) } as usize;
    let fill = if w >= r { w - r } else { cap - (r - w) };
    (fill, cap)
}

fn teardown_mapped_ring(card: &mut AudioCard) {
    if let Some(mapped) = card.mapped.take() {
        let _ = stem::syscall::vm_unmap(mapped.map_addr, mapped.map_len);
        let _ = stem::syscall::vfs_close(mapped.ring_fd);
        let _ = stem::syscall::vfs_close(mapped.control_fd);
        card.mapped_detach_count = card.mapped_detach_count.saturating_add(1);
        card.mapped_was_empty = false;
    }
}

fn mapped_ring_session_dead(mapped: &MappedRing) -> bool {
    let mut pollfds = [abi::syscall::PollThing {
        thing: mapped.control_fd as i32,
        events: POLLHUP | POLLERR,
        revents: 0,
    }];
    match stem::syscall::vfs::vfs_poll(&mut pollfds, 0) {
        Ok(_) => (pollfds[0].revents & (POLLHUP | POLLERR)) != 0,
        Err(_) => true,
    }
}

fn setup_mapped_ring_listener() -> Option<u32> {
    use abi::syscall::fcntl_cmd::F_SETFL;
    use abi::syscall::socket_domain::AF_UNIX;
    use abi::syscall::socket_type::SOCK_STREAM;
    use abi::syscall::vfs_flags::O_NONBLOCK;

    let _ = stem::syscall::vfs::vfs_unlink(AUDIO_RING_SOCKET_PATH);
    let fd = stem::syscall::socket::socket(AF_UNIX, SOCK_STREAM, 0).ok()?;
    if stem::syscall::socket::bind(fd, AUDIO_RING_SOCKET_PATH).is_err() {
        let _ = stem::syscall::vfs_close(fd);
        return None;
    }
    if stem::syscall::socket::listen(fd, 1).is_err() {
        let _ = stem::syscall::vfs_close(fd);
        return None;
    }
    let _ = stem::syscall::vfs::vfs_fcntl(fd, F_SETFL, O_NONBLOCK);
    Some(fd)
}

impl AudioCard {
    fn new() -> Self {
        // Default: S16LE, 44100 Hz, stereo, 1024-frame periods, 4096-frame buffer.
        Self {
            ring: RingBuf::new(64 * 1024),
            mapped: None,
            mapped_attach_count: 0,
            mapped_detach_count: 0,
            mapped_underrun_events: 0,
            mapped_bytes_consumed: 0,
            mapped_was_empty: false,
            params: AudioParams {
                sample_format: AudioSampleFormat::S16LE as u32,
                rate: 44100,
                channels: 2,
                period_frames: 1024,
                buffer_frames: 4096,
                _reserved: [0; 3],
            },
            state: AudioState::Stopped as u32,
            hw_frame: 0,
            app_frame: 0,
            xruns: 0,
            hw_config_dirty: true,
            hw_started: false,
            hw_faulted: false,
            out0_subscribed: false,
        }
    }

    /// Bytes per interleaved frame given the current params.
    fn bytes_per_frame(&self) -> usize {
        let fmt = AudioSampleFormat::from_u32(self.params.sample_format)
            .unwrap_or(AudioSampleFormat::S16LE);
        (fmt.bytes_per_sample() * self.params.channels) as usize
    }

    fn stream_info(&self) -> AudioStreamInfo {
        AudioStreamInfo {
            supported_formats: format_bit(AudioSampleFormat::S16LE)
                | format_bit(AudioSampleFormat::U8),
            min_rate: 8000,
            max_rate: 96000,
            max_channels: 2,
            min_buffer_frames: 256,
            max_buffer_frames: 65536,
            min_period_frames: 64,
            current_params: self.params,
            _reserved: [0; 4],
        }
    }

    fn status(&self) -> AudioStatus {
        let bpf = self.bytes_per_frame();
        let avail_bytes = if let Some(mapped) = self.mapped.as_ref() {
            let (fill, cap) = mapped_fill_and_capacity(mapped);
            cap.saturating_sub(fill)
        } else {
            self.ring.free_space()
        };
        let avail = if bpf > 0 { (avail_bytes / bpf) as u32 } else { 0 };
        AudioStatus {
            state: self.state,
            hw_frame: self.hw_frame,
            app_frame: self.app_frame,
            avail_frames: avail,
            xruns: self.xruns,
            _reserved: [0; 4],
        }
    }
}

// ── Device discovery ──────────────────────────────────────────────────────────

fn find_virtio_sound_device() -> Option<String> {
    use abi::syscall::vfs_flags;
    use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read, vfs_readdir};

    let fd = vfs_open("/sys/devices", vfs_flags::O_RDONLY).ok()?;
    let mut buf = [0u8; 4096];
    let n = vfs_readdir(fd, &mut buf).unwrap_or(0);
    let _ = vfs_close(fd);

    let mut pos = 0;
    while pos < n {
        let entry_buf = &buf[pos..n];
        let name = core::str::from_utf8(entry_buf).unwrap_or("").split('\0').next().unwrap_or("");
        if name.is_empty() {
            break;
        }
        if name.starts_with("pci-") {
            let path = alloc::format!("/sys/devices/{}/class", name);
            if let Ok(id_fd) = vfs_open(&path, vfs_flags::O_RDONLY) {
                let mut id_buf = [0u8; 64];
                if let Ok(id_len) = vfs_read(id_fd, &mut id_buf) {
                    let id_str = core::str::from_utf8(&id_buf[..id_len]).unwrap_or("");
                    if id_str.trim().starts_with("0x0401") {
                        let _ = vfs_close(id_fd);
                        return Some(alloc::format!("/sys/devices/{}", name));
                    }
                }
                let _ = vfs_close(id_fd);
            }
        }
        pos += name.len() + 1;
    }
    None
}

// ── VFS RPC helpers ───────────────────────────────────────────────────────────

// ── VFS RPC readdir helper ────────────────────────────────────────────────────

/// Encode `names` as packed DirentWire entries into a response payload.
fn encode_readdir(names: &[(&str, u32, u64)], offset: u64) -> Vec<u8> {
    // names: (name, file_type, ino)
    let mut out = Vec::new();
    for (i, (name, ftype, ino)) in names.iter().enumerate() {
        if (i as u64) < offset {
            continue;
        }
        let name_bytes = name.as_bytes();
        let name_len = name_bytes.len().min(255) as u8;
        // DirentWire: [ino: u64][file_type: u8][name_len: u8][name bytes]
        out.extend_from_slice(&ino.to_le_bytes());
        out.push(*ftype as u8);
        out.push(name_len);
        out.extend_from_slice(&name_bytes[..name_len as usize]);
    }
    out
}

// ── VFS RPC dispatch ──────────────────────────────────────────────────────────

fn dispatch_rpc(
    op: VfsRpcOp,
    payload: &[u8],
    card: &mut AudioCard,
) -> (ProviderResponse, bool /* ring_changed */) {
    debug_log_rpc(op, payload);
    match op {
        VfsRpcOp::Lookup => {
            if payload.len() < 4 {
                return (ProviderResponse::err(Errno::EINVAL), false);
            }
            let path_len =
                u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]) as usize;
            let path = if payload.len() >= 4 + path_len {
                core::str::from_utf8(&payload[4..4 + path_len]).unwrap_or("")
            } else {
                ""
            };
            let handle = match path {
                "" => HANDLE_ROOT,
                "ctl" => HANDLE_CTL,
                "out0" => HANDLE_OUT0,
                _ => return (ProviderResponse::err(Errno::ENOENT), false),
            };
            (ProviderResponse::ok_u64(handle), false)
        }

        VfsRpcOp::Stat => {
            if payload.len() < 8 {
                return (ProviderResponse::err(Errno::EINVAL), false);
            }
            let handle = u64::from_le_bytes(payload[..8].try_into().unwrap_or([0; 8]));
            let resp = match handle {
                HANDLE_ROOT => ProviderResponse::ok_stat(S_IFDIR | 0o755, 0, 1),
                HANDLE_CTL => ProviderResponse::ok_stat(S_IFREG | 0o444, 0, 2),
                // Apps currently open out0 as O_RDWR before issuing device calls,
                // so advertise read/write permissions for compatibility.
                HANDLE_OUT0 => ProviderResponse::ok_stat(S_IFREG | 0o666, 0, 3),
                _ => ProviderResponse::err(Errno::ENOENT),
            };
            (resp, false)
        }

        VfsRpcOp::Readdir => {
            if payload.len() < 20 {
                return (ProviderResponse::err(Errno::EINVAL), false);
            }
            let handle = u64::from_le_bytes(payload[..8].try_into().unwrap_or([0; 8]));
            let offset = u64::from_le_bytes(payload[8..16].try_into().unwrap_or([0; 8]));
            if handle != HANDLE_ROOT {
                return (ProviderResponse::err(Errno::ENOTDIR), false);
            }
            let entries: &[(&str, u32, u64)] = &[("ctl", S_IFREG, 2), ("out0", S_IFREG, 3)];
            let dir_bytes = encode_readdir(entries, offset);
            (ProviderResponse::ok_read(&dir_bytes), false)
        }

        VfsRpcOp::Read => {
            // ctl and out0 are not readable in v1
            (ProviderResponse::ok_read(&[]), false)
        }

        VfsRpcOp::Write => {
            if payload.len() < 20 {
                return (ProviderResponse::err(Errno::EINVAL), false);
            }
            let handle = u64::from_le_bytes(payload[..8].try_into().unwrap_or([0; 8]));
            if handle != HANDLE_OUT0 {
                return (ProviderResponse::err(Errno::EROFS), false);
            }
            let data_len =
                u32::from_le_bytes(payload[16..20].try_into().unwrap_or([0; 4])) as usize;
            let data = if payload.len() >= 20 + data_len {
                &payload[20..20 + data_len]
            } else {
                &payload[20..]
            };
            let n = card.ring.enqueue(data);
            let bpf = card.bytes_per_frame().max(1);
            card.app_frame += (n / bpf) as u64;
            let changed = n > 0;
            (ProviderResponse::ok_written(n as u32), changed)
        }

        VfsRpcOp::Poll => {
            if payload.len() < 8 {
                return (ProviderResponse::err(Errno::EINVAL), false);
            }
            let handle = u64::from_le_bytes(payload[..8].try_into().unwrap_or([0; 8]));
            if handle != HANDLE_OUT0 {
                // ctl: always return POLLOUT
                return (ProviderResponse::ok_poll(POLLOUT as u32), false);
            }
            let revents: u32 = if card.ring.free_space() > 0 { POLLOUT as u32 } else { 0 };
            (ProviderResponse::ok_poll(revents), false)
        }

        VfsRpcOp::DeviceCall => dispatch_device_call(payload, card),

        VfsRpcOp::SubscribeReady => {
            if payload.len() >= 8 {
                let handle = u64::from_le_bytes(payload[..8].try_into().unwrap_or([0; 8]));
                if handle == HANDLE_OUT0 {
                    card.out0_subscribed = true;
                }
            }
            (ProviderResponse::ok_empty(), false)
        }

        VfsRpcOp::UnsubscribeReady => {
            if payload.len() >= 8 {
                let handle = u64::from_le_bytes(payload[..8].try_into().unwrap_or([0; 8]));
                if handle == HANDLE_OUT0 {
                    card.out0_subscribed = false;
                }
            }
            (ProviderResponse::ok_empty(), false)
        }

        VfsRpcOp::Close => (ProviderResponse::ok_empty(), false),

        VfsRpcOp::Rename => (ProviderResponse::err(Errno::EROFS), false),
    }
}

fn debug_log_rpc(op: VfsRpcOp, payload: &[u8]) {
    match op {
        VfsRpcOp::Lookup => {
            if payload.len() >= 4 {
                let path_len =
                    u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]) as usize;
                if payload.len() >= 4 + path_len {
                    if let Ok(path) = core::str::from_utf8(&payload[4..4 + path_len]) {
                        info!("SND: rpc Lookup '{}'", path);
                        return;
                    }
                }
            }
            info!("SND: rpc Lookup <malformed>");
        }
        VfsRpcOp::Stat => {
            if payload.len() >= 8 {
                let handle = u64::from_le_bytes(payload[..8].try_into().unwrap_or([0; 8]));
                info!("SND: rpc Stat handle={}", handle);
            }
        }
        VfsRpcOp::DeviceCall => {
            if payload.len() >= 8 + size_of::<abi::device::DeviceCall>() {
                let handle = u64::from_le_bytes(payload[..8].try_into().unwrap_or([0; 8]));
                let dc: abi::device::DeviceCall = unsafe {
                    core::ptr::read_unaligned(
                        payload[8..].as_ptr() as *const abi::device::DeviceCall
                    )
                };
                info!("SND: rpc DeviceCall handle={} op={}", handle, dc.op);
            }
        }
        VfsRpcOp::Write => {
            if payload.len() >= 20 {
                let handle = u64::from_le_bytes(payload[..8].try_into().unwrap_or([0; 8]));
                let data_len =
                    u32::from_le_bytes(payload[16..20].try_into().unwrap_or([0; 4])) as usize;
                info!("SND: rpc Write handle={} len={}", handle, data_len);
            }
        }
        VfsRpcOp::Poll => {
            if payload.len() >= 8 {
                let handle = u64::from_le_bytes(payload[..8].try_into().unwrap_or([0; 8]));
                info!("SND: rpc Poll handle={}", handle);
            }
        }
        VfsRpcOp::SubscribeReady => {
            if payload.len() >= 8 {
                let handle = u64::from_le_bytes(payload[..8].try_into().unwrap_or([0; 8]));
                info!("SND: rpc SubscribeReady handle={}", handle);
            }
        }
        VfsRpcOp::UnsubscribeReady => {
            if payload.len() >= 8 {
                let handle = u64::from_le_bytes(payload[..8].try_into().unwrap_or([0; 8]));
                info!("SND: rpc UnsubscribeReady handle={}", handle);
            }
        }
        _ => {
            info!("SND: rpc {:?}", op as u8);
        }
    }
}

fn dispatch_device_call(payload: &[u8], card: &mut AudioCard) -> (ProviderResponse, bool) {
    let dc_size = size_of::<abi::device::DeviceCall>();
    if payload.len() < 8 + dc_size {
        return (ProviderResponse::err(Errno::EINVAL), false);
    }
    // payload: [handle: u64][DeviceCall][in_data...]
    let _handle = u64::from_le_bytes(payload[..8].try_into().unwrap_or([0; 8]));
    let dc: abi::device::DeviceCall = unsafe {
        core::ptr::read_unaligned(payload[8..].as_ptr() as *const abi::device::DeviceCall)
    };
    if dc.kind != DeviceKind::Audio {
        return (ProviderResponse::err(Errno::ENOSYS), false);
    }
    let in_data = &payload[8 + dc_size..];

    match dc.op {
        AUDIO_GET_INFO => {
            let info = card.stream_info();
            let bytes = unsafe {
                core::slice::from_raw_parts(
                    &info as *const AudioStreamInfo as *const u8,
                    size_of::<AudioStreamInfo>(),
                )
            };
            (ok_device_call(0, bytes), false)
        }

        AUDIO_SET_PARAMS => {
            if in_data.len() >= size_of::<AudioParams>() {
                let req: AudioParams =
                    unsafe { core::ptr::read_unaligned(in_data.as_ptr() as *const AudioParams) };
                // Accept the request as-is (clamp channels to 2 if unsupported).
                card.params = AudioParams {
                    sample_format: req.sample_format,
                    rate: req.rate.clamp(8000, 96000),
                    channels: req.channels.clamp(1, 2),
                    period_frames: req.period_frames.max(64),
                    buffer_frames: req.buffer_frames.max(256),
                    _reserved: [0; 3],
                };
                card.hw_config_dirty = true;
                card.hw_started = false;
                card.hw_faulted = false;
            }
            let bytes = unsafe {
                core::slice::from_raw_parts(
                    &card.params as *const AudioParams as *const u8,
                    size_of::<AudioParams>(),
                )
            };
            (ok_device_call(0, bytes), false)
        }

        AUDIO_GET_PARAMS => {
            let bytes = unsafe {
                core::slice::from_raw_parts(
                    &card.params as *const AudioParams as *const u8,
                    size_of::<AudioParams>(),
                )
            };
            (ok_device_call(0, bytes), false)
        }

        AUDIO_GET_STATUS => {
            let status = card.status();
            let bytes = unsafe {
                core::slice::from_raw_parts(
                    &status as *const AudioStatus as *const u8,
                    size_of::<AudioStatus>(),
                )
            };
            (ok_device_call(0, bytes), false)
        }

        AUDIO_GET_MAPPED_RING_INFO => {
            let mut info = AudioMappedRingInfo::default();
            let path = AUDIO_RING_SOCKET_PATH.as_bytes();
            let copy_n = path.len().min(info.socket_path.len());
            info.supported = 1;
            info.suggested_ring_bytes = AUDIO_RING_SUGGESTED_BYTES as u32;
            info.socket_path_len = copy_n as u32;
            info.socket_path[..copy_n].copy_from_slice(&path[..copy_n]);
            let bytes = unsafe {
                core::slice::from_raw_parts(
                    &info as *const AudioMappedRingInfo as *const u8,
                    size_of::<AudioMappedRingInfo>(),
                )
            };
            (ok_device_call(0, bytes), false)
        }

        AUDIO_GET_MAPPED_RING_STATS => {
            let (fill, cap, active) = if let Some(mapped) = card.mapped.as_ref() {
                let (f, c) = mapped_fill_and_capacity(mapped);
                (f, c, 1u32)
            } else {
                (card.ring.available(), card.ring.cap, 0u32)
            };
            let stats = AudioMappedRingStats {
                mapped_active: active,
                ring_fill_bytes: fill.min(u32::MAX as usize) as u32,
                ring_capacity_bytes: cap.min(u32::MAX as usize) as u32,
                mapped_attach_count: card.mapped_attach_count,
                mapped_detach_count: card.mapped_detach_count,
                mapped_underrun_events: card.mapped_underrun_events,
                mapped_bytes_consumed: card.mapped_bytes_consumed,
                _reserved: [0; 4],
            };
            let bytes = unsafe {
                core::slice::from_raw_parts(
                    &stats as *const AudioMappedRingStats as *const u8,
                    size_of::<AudioMappedRingStats>(),
                )
            };
            (ok_device_call(0, bytes), false)
        }

        AUDIO_START => {
            card.state = AudioState::Running as u32;
            card.hw_faulted = false;
            (ok_device_call(0, &[]), false)
        }

        AUDIO_STOP => {
            card.state = AudioState::Stopped as u32;
            card.ring = RingBuf::new(64 * 1024);
            teardown_mapped_ring(card);
            card.hw_started = false;
            card.hw_faulted = false;
            (ok_device_call(0, &[]), false)
        }

        AUDIO_DRAIN => {
            card.state = AudioState::Draining as u32;
            (ok_device_call(0, &[]), false)
        }

        _ => (ProviderResponse::err(Errno::ENOSYS), false),
    }
}

/// Build a DeviceCall OK response: `[ret_val: u32][out_len: u32][out_data]`.
fn ok_device_call(ret_val: u32, out_data: &[u8]) -> ProviderResponse {
    let mut payload = Vec::with_capacity(8 + out_data.len());
    payload.extend_from_slice(&ret_val.to_le_bytes());
    payload.extend_from_slice(&(out_data.len() as u32).to_le_bytes());
    payload.extend_from_slice(out_data);
    ProviderResponse::ok_bytes(&payload)
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn run_driver(mut boot_fd: usize, explicit_path: Option<&str>) -> ! {
    info!("SND: Starting VirtIO Sound Driver (boot_fd={})...", boot_fd);

    // Try to recover boot_fd from argv[1] if not passed directly.
    if boot_fd == 0 && explicit_path.is_none() {
        let mut buf = [0u8; 1024];
        if let Ok(needed) = stem::syscall::argv_get(&mut buf) {
            if needed >= 4 {
                let count = u32::from_le_bytes(buf[0..4].try_into().unwrap());
                if count >= 2 {
                    let mut offset = 4;
                    let arg0_len =
                        u32::from_le_bytes(buf[offset..offset + 4].try_into().unwrap()) as usize;
                    offset += 4 + arg0_len;
                    if offset + 4 <= buf.len() {
                        let arg1_len =
                            u32::from_le_bytes(buf[offset..offset + 4].try_into().unwrap())
                                as usize;
                        offset += 4;
                        if offset + arg1_len <= buf.len() {
                            if let Ok(s) = core::str::from_utf8(&buf[offset..offset + arg1_len]) {
                                if let Ok(val) = s.parse::<usize>() {
                                    boot_fd = val;
                                    info!("SND: Recovered boot_fd {} from argv[1]", boot_fd);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let path_str = if let Some(path) = explicit_path {
        path.to_string()
    } else if let Some(path) = resolve_device_path_from_boot_fd(boot_fd) {
        path
    } else if let Some(found) = find_virtio_sound_device() {
        info!("SND: Discovered device at {}", found);
        found
    } else {
        error!("SND: No virtio-sound device found");
        loop {
            stem::time::sleep_ms(1000);
        }
    };

    // Initialise hardware.
    let mut driver = match VirtioDevice::new(&path_str) {
        Ok(d) => d,
        Err(e) => {
            error!("SND: Failed to claim device at {}: {:?}", path_str, e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };

    if let Err(e) = driver.init(VIRTIO_SND_F_CTLS) {
        error!("SND: Failed to init device: {}", e);
        loop {
            stem::time::sleep_ms(1);
        }
    }

    for q in 0..4 {
        if let Err(e) = driver.setup_queue(q, QUEUE_SIZE) {
            error!("SND: Failed to setup queue {}: {}", q, e);
            loop {
                stem::time::sleep_ms(1);
            }
        }
    }

    driver.driver_ok();
    info!("SND: Device initialised");

    let dma_dev = driver.claim_thing();
    let control_dma = match setup_control_dma(dma_dev) {
        Some(v) => v,
        None => {
            error!("SND: Failed to allocate control DMA buffers");
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };

    let mut event_queue = match setup_event_queue(&mut driver, dma_dev) {
        Some(v) => v,
        None => {
            error!("SND: Failed to populate event queue");
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };

    let stream_id = match find_output_stream(&mut driver) {
        Some(id) => id,
        None => {
            error!("SND: No output stream found");
            loop {
                stem::time::sleep_ms(1);
            }
        }
    };
    info!("SND: Using stream {}", stream_id);

    // ── Mount VFS provider at /dev/audio/card0/ ───────────────────────────────
    // Mount early — before configure_stream/PCM_START/DMA alloc — so that
    // clients (e.g. chime) can open /dev/audio/card0/out0 without waiting for
    // potentially-blocking hardware initialisation steps.
    let (req_write, req_read) = match channel_create(VFS_RPC_MAX_REQ * 16) {
        Ok(p) => p,
        Err(e) => {
            error!("SND: channel_create failed: {:?}", e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };

    match vfs_mount(req_write, "/dev/audio/card0") {
        Ok(()) => info!("SND: Mounted at /dev/audio/card0"),
        Err(e) => {
            warn!("SND: vfs_mount failed: {:?} — continuing without VFS interface", e);
        }
    }

    // Pre-allocate a reusable TX DMA staging buffer. Reallocating per-chunk
    // eventually exhausts per-task DMA slots during sustained playback.
    let tx_dma_pages = 8;
    let tx_dma = match stem::syscall::device_alloc_dma(dma_dev, tx_dma_pages) {
        Ok(v) => v,
        Err(e) => {
            error!("SND: Failed to allocate TX DMA staging buffer: {:?}", e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };
    let tx_dma_phys = match stem::syscall::device_dma_phys(tx_dma) {
        Ok(p) => p,
        Err(e) => {
            error!("SND: Failed to resolve TX DMA physical address: {:?}", e);
            loop {
                stem::time::sleep_ms(1000);
            }
        }
    };
    let tx_dma_ptr = tx_dma as *mut u8;

    // ── Main event loop ───────────────────────────────────────────────────────
    let mut card = AudioCard::new();
    let mut provider_loop = ProviderLoop::new(req_read);
    let mapped_listener = setup_mapped_ring_listener();
    if mapped_listener.is_some() {
        info!("SND: mapped ring control socket at {}", AUDIO_RING_SOCKET_PATH);
    } else {
        warn!("SND: mapped ring socket unavailable; using write() path");
    }
    let mut mapped_pending_fd: Option<u32> = None;

    let mut tx_in_flight: [Option<u16>; 4] = [None; 4];
    let slot_size = 8192usize;

    loop {
        let mut progress = false;

        // 1. Process VFS requests first so optional mapped-ring housekeeping
        // cannot stall control-path RPCs such as lookup/stat/readdir on
        // /dev/audio/card0.
        loop {
            match provider_loop.try_next_request() {
                Ok(Some(req)) => {
                    progress = true;
                    let prev_free = card.ring.free_space();
                    let (resp, ring_changed) = dispatch_rpc(req.op, &req.payload, &mut card);
                    let _ = provider_loop.send_response(req.resp_port, resp);

                    if card.out0_subscribed && (ring_changed || card.ring.free_space() > prev_free)
                    {
                        let _ = stem::syscall::vfs::vfs_notify(
                            req_write,
                            HANDLE_OUT0,
                            abi::syscall::poll_flags::POLLOUT,
                        );
                    }
                }
                Ok(None) => break,
                Err(e) => {
                    warn!("SND: RPC loop error: {:?}", e);
                    break;
                }
            }
        }

        // 2. Accept new mapped ring connections.
        if let Some(listener_fd) = mapped_listener {
            if mapped_pending_fd.is_none() {
                // Guard accept() with poll so a blocking socket backend cannot
                // stall the entire provider/event loop.
                let mut pollfds = [abi::syscall::PollThing {
                    thing: listener_fd as i32,
                    events: POLLIN | POLLERR | POLLHUP,
                    revents: 0,
                }];
                match stem::syscall::vfs::vfs_poll(&mut pollfds, 0) {
                    Ok(n) if n > 0 => {
                        let revents = pollfds[0].revents;
                        if (revents & POLLIN) != 0 {
                            match stem::syscall::socket::accept(listener_fd) {
                                Ok(fd) => {
                                    use abi::syscall::fcntl_cmd::F_SETFL;
                                    use abi::syscall::vfs_flags::O_NONBLOCK;
                                    let _ = stem::syscall::vfs::vfs_fcntl(fd, F_SETFL, O_NONBLOCK);
                                    mapped_pending_fd = Some(fd);
                                }
                                Err(Errno::EAGAIN) => {}
                                Err(e) => {
                                    warn!("SND: mapped ring accept failed: {:?}", e);
                                }
                            }
                        } else if (revents & (POLLERR | POLLHUP)) != 0 {
                            warn!("SND: mapped ring listener poll revents=0x{:x}", revents);
                        }
                    }
                    Ok(_) => {}
                    Err(e) => warn!("SND: mapped ring listener poll failed: {:?}", e),
                }
            }

            if card.mapped.is_none() {
                if let Some(ctrl_fd) = mapped_pending_fd {
                    let mut setup_buf = [0u8; core::mem::size_of::<AudioMappedRingSetup>()];
                    let mut fds = [0u32; 1];
                    match stem::syscall::socket::recvmsg(ctrl_fd, &mut setup_buf, &mut fds) {
                        Ok((n, fdc)) if n >= setup_buf.len() && fdc > 0 => {
                            let setup: AudioMappedRingSetup = unsafe {
                                core::ptr::read_unaligned(
                                    setup_buf.as_ptr() as *const AudioMappedRingSetup
                                )
                            };
                            let ring_fd = fds[0];
                            if setup.version != AUDIO_MAPPED_RING_VERSION
                                || (setup.ring_bytes as usize)
                                    < core::mem::size_of::<AudioMappedRingHeader>() + 4096
                            {
                                let _ = stem::syscall::vfs_close(ring_fd);
                                let _ = stem::syscall::vfs_close(ctrl_fd);
                                mapped_pending_fd = None;
                            } else {
                                let req = abi::vm::VmMapReq {
                                    addr_hint: 0,
                                    len: setup.ring_bytes as usize,
                                    prot: abi::vm::VmProt::READ
                                        | abi::vm::VmProt::WRITE
                                        | abi::vm::VmProt::USER,
                                    flags: abi::vm::VmMapFlags::empty(),
                                    backing: abi::vm::VmBacking::File { thing: ring_fd, offset: 0 },
                                };
                                match stem::syscall::vm_map(&req) {
                                    Ok(resp) => {
                                        let mut mapped = MappedRing {
                                            control_fd: ctrl_fd,
                                            ring_fd,
                                            map_addr: resp.addr,
                                            map_len: resp.len,
                                        };
                                        let cap =
                                            mapped.map_len.saturating_sub(core::mem::size_of::<
                                                AudioMappedRingHeader,
                                            >(
                                            ));
                                        let hdr = mapped_header_mut(&mut mapped);
                                        unsafe {
                                            let current_cap = core::ptr::read_volatile(
                                                core::ptr::addr_of!((*hdr).capacity_bytes),
                                            );
                                            if current_cap == 0 {
                                                core::ptr::write_volatile(
                                                    &raw mut (*hdr).capacity_bytes,
                                                    cap as u32,
                                                );
                                            }
                                        }
                                        card.mapped = Some(mapped);
                                        card.mapped_attach_count =
                                            card.mapped_attach_count.saturating_add(1);
                                        card.mapped_was_empty = false;
                                        mapped_pending_fd = None;
                                        info!("SND: mapped ring attached ({} bytes)", resp.len);
                                        progress = true;
                                    }
                                    Err(e) => {
                                        warn!("SND: mapped ring vm_map failed: {:?}", e);
                                        let _ = stem::syscall::vfs_close(ring_fd);
                                        let _ = stem::syscall::vfs_close(ctrl_fd);
                                        mapped_pending_fd = None;
                                    }
                                }
                            }
                        }
                        Ok(_) => {}
                        Err(Errno::EAGAIN) => {}
                        Err(e) => {
                            warn!("SND: mapped ring recvmsg failed: {:?}", e);
                            let _ = stem::syscall::vfs_close(ctrl_fd);
                            mapped_pending_fd = None;
                        }
                    }
                }
            }
        }

        // 3. Detect xruns from the event queue.
        if process_event_queue(&mut driver, &mut event_queue) {
            card.xruns += 1;
            warn!("SND: PCM xrun");
            progress = true;
        }

        // 3a. Drop stale mapped-ring sessions once the producer exits.
        if card.mapped.as_ref().is_some_and(mapped_ring_session_dead) {
            teardown_mapped_ring(&mut card);
            progress = true;
        }

        // 4. Recycle completed TX descriptors.
        let prev_free_before_tx = card.ring.free_space();
        {
            let mut q = driver.queue_mut(VIRTIO_SND_VQ_TX).unwrap();
            while let Some((id, _len)) = q.poll_used() {
                for slot in tx_in_flight.iter_mut() {
                    if *slot == Some(id) {
                        *slot = None;
                        progress = true;
                        break;
                    }
                }
            }
        }

        // 4a. Lazily configure and start the hardware only once the provider is
        // already alive and a client has actually requested playback.
        if (card.state == AudioState::Running as u32 || card.state == AudioState::Draining as u32)
            && !card.hw_started
            && !card.hw_faulted
        {
            let configured = if card.hw_config_dirty {
                configure_stream(&mut driver, &control_dma, stream_id, &card.params)
            } else {
                true
            };

            if configured {
                card.hw_config_dirty = false;
                if send_pcm_command(&mut driver, &control_dma, VIRTIO_SND_R_PCM_START, stream_id) {
                    card.hw_started = true;
                    info!("SND: Hardware playback started");
                    progress = true;
                } else {
                    warn!("SND: Hardware playback start did not complete cleanly");
                    card.hw_faulted = true;
                    card.state = AudioState::Stopped as u32;
                }
            } else {
                warn!("SND: stream {} configuration did not complete cleanly", stream_id);
                card.hw_faulted = true;
                card.state = AudioState::Stopped as u32;
            }
        }

        // 5. Feed hardware from ring if running.
        if card.hw_started
            && (card.state == AudioState::Running as u32
                || card.state == AudioState::Draining as u32)
        {
            while let Some(slot_idx) = (0..tx_in_flight.len()).find(|&i| tx_in_flight[i].is_none())
            {
                let slot_ptr = unsafe { tx_dma_ptr.add(slot_idx * slot_size) };
                let slot_phys = tx_dma_phys + (slot_idx * slot_size) as u64;

                let hdr_sz = size_of::<VirtioSndPcmXfer>();
                unsafe {
                    *(slot_ptr as *mut VirtioSndPcmXfer) = VirtioSndPcmXfer { stream_id };
                }
                let data_slice =
                    unsafe { core::slice::from_raw_parts_mut(slot_ptr.add(hdr_sz), 4096) };

                let n = if let Some(mapped) = card.mapped.as_mut() {
                    mapped_dequeue(mapped, data_slice)
                } else {
                    card.ring.dequeue(data_slice)
                };

                if n > 0 {
                    if card.mapped.is_some() {
                        card.mapped_bytes_consumed =
                            card.mapped_bytes_consumed.saturating_add(n as u64);
                        card.mapped_was_empty = false;
                    }
                    let bpf = card.bytes_per_frame().max(1);
                    card.hw_frame += (n / bpf) as u64;

                    let added = {
                        let q = driver.queue_mut(VIRTIO_SND_VQ_TX).unwrap();
                        q.add_buffer_single(slot_phys, (hdr_sz + n) as u32, false)
                    };

                    if let Some(id) = added {
                        driver.notify_queue(VIRTIO_SND_VQ_TX);
                        tx_in_flight[slot_idx] = Some(id);
                        progress = true;
                    } else {
                        break; // Queue full
                    }
                } else {
                    if card.mapped.is_some() && card.state == AudioState::Running as u32 {
                        if !card.mapped_was_empty {
                            card.mapped_underrun_events =
                                card.mapped_underrun_events.saturating_add(1);
                            card.mapped_was_empty = true;
                        }
                    }
                    break; // No more data
                }
            }
        }

        // 6. Notify writers that ring space opened up.
        if card.mapped.is_none()
            && card.out0_subscribed
            && card.ring.free_space() > prev_free_before_tx
        {
            let _ = stem::syscall::vfs::vfs_notify(
                req_write,
                HANDLE_OUT0,
                abi::syscall::poll_flags::POLLOUT,
            );
        }

        // 7. Honour stop requests once in-flight DMA has drained.
        if card.state == AudioState::Stopped as u32
            && card.hw_started
            && tx_in_flight.iter().all(|s| s.is_none())
        {
            let _ = send_pcm_command(&mut driver, &control_dma, VIRTIO_SND_R_PCM_STOP, stream_id);
            card.hw_started = false;
            progress = true;
        }

        // 8. Stop if draining and ring is now empty and all HW buffers finished.
        if card.state == AudioState::Draining as u32
            && card
                .mapped
                .as_ref()
                .map(|mapped| mapped_fill_and_capacity(mapped).0 == 0)
                .unwrap_or(true)
            && card.ring.available() == 0
            && tx_in_flight.iter().all(|s| s.is_none())
        {
            card.state = AudioState::Stopped as u32;
            if card.hw_started {
                let _ =
                    send_pcm_command(&mut driver, &control_dma, VIRTIO_SND_R_PCM_STOP, stream_id);
                card.hw_started = false;
            }
            progress = true;
        }

        if !progress {
            stem::yield_now();
        }
    }
}

fn resolve_device_path_from_boot_fd(boot_fd: usize) -> Option<String> {
    if boot_fd == 0 {
        return None;
    }

    // New cambium path: boot arg contains a DriverEntryCtx payload.
    use abi::vm::{VmBacking, VmMapFlags, VmMapReq, VmProt};
    let req = VmMapReq {
        addr_hint: 0,
        len: 4096,
        prot: VmProt::READ | VmProt::USER,
        flags: VmMapFlags::empty(),
        backing: VmBacking::File { thing: boot_fd as u32, offset: 0 },
    };
    if let Ok(resp) = stem::syscall::vm_map(&req) {
        let ctx = unsafe { &*(resp.addr as *const DriverEntryCtx) };
        if ctx.version == 1 {
            let s = ctx.device_path_str();
            if !s.is_empty() {
                return Some(s.to_string());
            }
        }
    }

    // Legacy fallback: boot_fd holds a plain path string.
    let mut path_buf = [0u8; 128];
    use stem::syscall::vfs::vfs_read;
    let path_len = vfs_read(boot_fd as u32, &mut path_buf).unwrap_or(0);
    if path_len == 0 {
        return None;
    }
    let path = core::str::from_utf8(&path_buf[..path_len])
        .unwrap_or("")
        .trim_matches(char::from(0))
        .to_string();
    if path.is_empty() { None } else { Some(path) }
}

#[unsafe(no_mangle)]
pub extern "C" fn thing_driver_entry_v1(ctx_ptr: u64, ctx_len: u32) -> i32 {
    let path_opt = if ctx_ptr != 0 && (ctx_len as usize) >= size_of::<DriverEntryCtx>() {
        let ctx = unsafe { &*(ctx_ptr as *const DriverEntryCtx) };
        let s = ctx.device_path_str();
        if s.is_empty() { None } else { Some(s) }
    } else {
        None
    };

    run_driver(0, path_opt)
}

#[stem::main]
fn main(boot_fd: usize) -> ! {
    run_driver(boot_fd, None)
}

// ── Hardware helpers (unchanged from original) ────────────────────────────────

fn process_tx_queue(driver: &mut VirtioDevice) -> usize {
    let q = driver.queue_mut(VIRTIO_SND_VQ_TX).unwrap();
    let mut completed = 0usize;
    while q.poll_used().is_some() {
        completed += 1;
    }
    completed
}

#[derive(Clone, Copy, Default)]
struct DmaPage {
    virt: usize,
    phys: usize,
}

struct ControlDma {
    req: DmaPage,
    resp: DmaPage,
}

struct EventQueueState {
    pages: [DmaPage; EVENT_QUEUE_SLOTS],
    desc_to_slot: [u8; QUEUE_SIZE as usize],
}

fn alloc_dma_page(claim_thing: usize, tag: &str) -> Option<DmaPage> {
    let virt = match stem::syscall::device_alloc_dma(claim_thing, 1) {
        Ok(v) => v,
        Err(e) => {
            warn!("SND: DMA alloc failed for {}: {:?}", tag, e);
            return None;
        }
    };
    let phys = match stem::syscall::device_dma_phys(virt) {
        Ok(p) => p,
        Err(e) => {
            warn!("SND: DMA phys failed for {}: {:?}", tag, e);
            return None;
        }
    };
    Some(DmaPage { virt: virt as usize, phys: phys as usize })
}

fn setup_control_dma(claim_thing: usize) -> Option<ControlDma> {
    let req = alloc_dma_page(claim_thing, "control.req")?;
    let resp = alloc_dma_page(claim_thing, "control.resp")?;
    Some(ControlDma { req, resp })
}

fn setup_event_queue(driver: &mut VirtioDevice, claim_thing: usize) -> Option<EventQueueState> {
    let mut pages = [DmaPage::default(); EVENT_QUEUE_SLOTS];
    let mut desc_to_slot = [INVALID_EVENT_SLOT; QUEUE_SIZE as usize];
    let mut queued = 0usize;

    {
        let q = driver.queue_mut(VIRTIO_SND_VQ_EVENT).unwrap();
        for (slot, page) in pages.iter_mut().enumerate() {
            let dma = match alloc_dma_page(claim_thing, "event") {
                Some(v) => v,
                None => break,
            };
            let desc = match q.add_buffer_single(
                dma.phys as u64,
                size_of::<VirtioSndEvent>() as u32,
                true,
            ) {
                Some(d) => d,
                None => {
                    warn!("SND: event queue full while populating");
                    break;
                }
            };
            *page = dma;
            desc_to_slot[desc as usize] = slot as u8;
            queued += 1;
        }
    }

    if queued == 0 {
        return None;
    }

    driver.notify_queue(VIRTIO_SND_VQ_EVENT);
    Some(EventQueueState { pages, desc_to_slot })
}

fn process_event_queue(driver: &mut VirtioDevice, state: &mut EventQueueState) -> bool {
    let mut needs_notify = false;
    let mut xrun_seen = false;
    {
        let q = driver.queue_mut(VIRTIO_SND_VQ_EVENT).unwrap();
        while let Some((desc_id, _len)) = q.poll_used() {
            xrun_seen = true;

            if (desc_id as usize) >= state.desc_to_slot.len() {
                continue;
            }

            let slot = state.desc_to_slot[desc_id as usize];
            state.desc_to_slot[desc_id as usize] = INVALID_EVENT_SLOT;
            if slot == INVALID_EVENT_SLOT {
                continue;
            }

            let page = state.pages[slot as usize];
            let new_desc =
                q.add_buffer_single(page.phys as u64, size_of::<VirtioSndEvent>() as u32, true);
            if let Some(new_desc) = new_desc {
                state.desc_to_slot[new_desc as usize] = slot;
                needs_notify = true;
            } else {
                warn!("SND: event queue full while requeueing");
                break;
            }
        }
    }
    if needs_notify {
        driver.notify_queue(VIRTIO_SND_VQ_EVENT);
    }
    xrun_seen
}

fn pcm_cmd_name(cmd: u32) -> &'static str {
    match cmd {
        VIRTIO_SND_R_PCM_SET_PARAMS => "SET_PARAMS",
        VIRTIO_SND_R_PCM_PREPARE => "PREPARE",
        VIRTIO_SND_R_PCM_START => "START",
        VIRTIO_SND_R_PCM_STOP => "STOP",
        VIRTIO_SND_R_PCM_RELEASE => "RELEASE",
        _ => "UNKNOWN",
    }
}

fn send_pcm_command(
    driver: &mut VirtioDevice,
    control_dma: &ControlDma,
    cmd: u32,
    stream_id: u32,
) -> bool {
    info!("SND: control {} begin for stream {}", pcm_cmd_name(cmd), stream_id);
    unsafe {
        *(control_dma.req.virt as *mut VirtioSndPcmHdr) =
            VirtioSndPcmHdr { hdr: VirtioSndHdr { code: cmd }, stream_id };
        *(control_dma.resp.virt as *mut VirtioSndHdr) = VirtioSndHdr { code: 0 };
    }
    let bufs = [
        (control_dma.req.phys as u64, size_of::<VirtioSndPcmHdr>() as u32, false),
        (control_dma.resp.phys as u64, size_of::<VirtioSndHdr>() as u32, true),
    ];
    {
        let q = driver.queue_mut(VIRTIO_SND_VQ_CONTROL).unwrap();
        if q.add_buffer(&bufs).is_none() {
            warn!("SND: control {} queue full for stream {}", pcm_cmd_name(cmd), stream_id);
            return false;
        }
    }
    driver.notify_queue(VIRTIO_SND_VQ_CONTROL);
    let start_ns = stem::time::monotonic_ns();
    loop {
        let done = {
            let q = driver.queue_mut(VIRTIO_SND_VQ_CONTROL).unwrap();
            q.poll_used().is_some()
        };
        if done {
            let resp = unsafe { *(control_dma.resp.virt as *const VirtioSndHdr) };
            if resp.code != VIRTIO_SND_S_OK {
                warn!(
                    "SND: control {} for stream {} completed with status {:x}",
                    pcm_cmd_name(cmd),
                    stream_id,
                    resp.code
                );
                return false;
            }
            info!("SND: control {} complete for stream {}", pcm_cmd_name(cmd), stream_id);
            return true;
        }
        if stem::time::monotonic_ns().saturating_sub(start_ns) > 2_000_000_000 {
            let resp = unsafe { *(control_dma.resp.virt as *const VirtioSndHdr) };
            warn!(
                "SND: control {} for stream {} timed out (resp={:x})",
                pcm_cmd_name(cmd),
                stream_id,
                resp.code
            );
            return false;
        }
        stem::time::sleep_ms(1);
    }
}

fn map_audio_format(sample_format: u32) -> u8 {
    match AudioSampleFormat::from_u32(sample_format).unwrap_or(AudioSampleFormat::S16LE) {
        AudioSampleFormat::U8 => VIRTIO_SND_PCM_FMT_U8,
        AudioSampleFormat::S16LE => VIRTIO_SND_PCM_FMT_S16,
        _ => VIRTIO_SND_PCM_FMT_S16,
    }
}

fn map_audio_rate(rate: u32) -> u8 {
    match rate {
        5512 => VIRTIO_SND_PCM_RATE_5512,
        8000 => VIRTIO_SND_PCM_RATE_8000,
        11025 => VIRTIO_SND_PCM_RATE_11025,
        16000 => VIRTIO_SND_PCM_RATE_16000,
        22050 => VIRTIO_SND_PCM_RATE_22050,
        32000 => VIRTIO_SND_PCM_RATE_32000,
        44100 => VIRTIO_SND_PCM_RATE_44100,
        48000 => VIRTIO_SND_PCM_RATE_48000,
        64000 => VIRTIO_SND_PCM_RATE_64000,
        88200 => VIRTIO_SND_PCM_RATE_88200,
        96000 => VIRTIO_SND_PCM_RATE_96000,
        176400 => VIRTIO_SND_PCM_RATE_176400,
        192000 => VIRTIO_SND_PCM_RATE_192000,
        384000 => VIRTIO_SND_PCM_RATE_384000,
        _ => VIRTIO_SND_PCM_RATE_44100,
    }
}

fn configure_stream(
    driver: &mut VirtioDevice,
    control_dma: &ControlDma,
    stream_id: u32,
    params: &AudioParams,
) -> bool {
    info!("SND: control SET_PARAMS begin for stream {}", stream_id);
    let bytes_per_frame = AudioSampleFormat::from_u32(params.sample_format)
        .unwrap_or(AudioSampleFormat::S16LE)
        .bytes_per_sample() as u32
        * params.channels.max(1);
    let period_bytes = params.period_frames.max(64).saturating_mul(bytes_per_frame);
    let buffer_bytes =
        params.buffer_frames.max(params.period_frames.max(64)).saturating_mul(bytes_per_frame);
    unsafe {
        *(control_dma.req.virt as *mut VirtioSndPcmSetParams) = VirtioSndPcmSetParams {
            hdr: VirtioSndPcmHdr {
                hdr: VirtioSndHdr { code: VIRTIO_SND_R_PCM_SET_PARAMS },
                stream_id,
            },
            buffer_bytes,
            period_bytes,
            features: 0,
            channels: params.channels.clamp(1, 2) as u8,
            format: map_audio_format(params.sample_format),
            rate: map_audio_rate(params.rate),
            padding: 0,
        };
    }
    let bufs = [
        (control_dma.req.phys as u64, size_of::<VirtioSndPcmSetParams>() as u32, false),
        (control_dma.resp.phys as u64, size_of::<VirtioSndHdr>() as u32, true),
    ];
    {
        let q = driver.queue_mut(VIRTIO_SND_VQ_CONTROL).unwrap();
        if q.add_buffer(&bufs).is_none() {
            warn!("SND: control queue full for set_params");
            return false;
        }
    }
    driver.notify_queue(VIRTIO_SND_VQ_CONTROL);
    let start_ns = stem::time::monotonic_ns();
    loop {
        let done = {
            let q = driver.queue_mut(VIRTIO_SND_VQ_CONTROL).unwrap();
            q.poll_used().is_some()
        };
        if done {
            let resp = unsafe { *(control_dma.resp.virt as *const VirtioSndHdr) };
            if resp.code != VIRTIO_SND_S_OK {
                warn!(
                    "SND: set_params for stream {} completed with status {:x}",
                    stream_id, resp.code
                );
                return false;
            }
            info!("SND: control SET_PARAMS complete for stream {}", stream_id);
            break;
        }
        if stem::time::monotonic_ns().saturating_sub(start_ns) > 2_000_000_000 {
            let resp = unsafe { *(control_dma.resp.virt as *const VirtioSndHdr) };
            warn!("SND: set_params for stream {} timed out (resp={:x})", stream_id, resp.code);
            return false;
        }
        stem::time::sleep_ms(1);
    }
    send_pcm_command(driver, control_dma, VIRTIO_SND_R_PCM_PREPARE, stream_id)
}

fn find_output_stream(_driver: &mut VirtioDevice) -> Option<u32> {
    Some(0) // Stream 0 is the output in a standard virtio-snd device.
}
