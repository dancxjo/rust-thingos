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
    BusKind, DeviceInfo, DriverClass, DriverDescriptor, DriverEntryCtx, DriverInterfaceV1,
    DriverStartContext, ProbeResult, Status, DRIVER_DESCRIPTOR_ABI_VERSION, DRIVER_FLAG_PCI,
    DRIVER_INTERFACE_ABI_VERSION,
};
use abi::errors::Errno;
use abi::sound::{
    AUDIO_DRAIN, AUDIO_GET_INFO, AUDIO_GET_PARAMS, AUDIO_GET_STATUS, AUDIO_SET_PARAMS, AUDIO_START,
    AUDIO_STOP, AudioParams, AudioSampleFormat, AudioState, AudioStatus, AudioStreamInfo,
    format_bit,
};
use abi::vfs_rpc::{VFS_RPC_MAX_REQ, VfsRpcOp};
use ipc_helpers::provider::{ProviderLoop, ProviderResponse};
use spec::*;
use stem::syscall::channel::channel_create;
use stem::syscall::vfs::vfs_mount;
use stem::{error, info, warn};
use virtio::device::VirtioDevice;

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
    params: AudioParams,
    state: u32, // AudioState as u32
    hw_frame: u64,
    app_frame: u64,
    xruns: u32,
    /// True when at least one SubscribeReady request has been received for out0
    /// and no matching UnsubscribeReady has been received.
    out0_subscribed: bool,
}

impl AudioCard {
    fn new() -> Self {
        // Default: S16LE, 44100 Hz, stereo, 1024-frame periods, 4096-frame buffer.
        Self {
            ring: RingBuf::new(64 * 1024),
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
        let avail = if bpf > 0 { (self.ring.free_space() / bpf) as u32 } else { 0 };
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
                HANDLE_OUT0 => ProviderResponse::ok_stat(S_IFREG | 0o222, 0, 3),
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

        AUDIO_START => {
            card.state = AudioState::Running as u32;
            (ok_device_call(0, &[]), false)
        }

        AUDIO_STOP => {
            card.state = AudioState::Stopped as u32;
            card.ring = RingBuf::new(64 * 1024);
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

    populate_event_queue(&mut driver);

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

    configure_stream(&mut driver, stream_id);
    send_pcm_command(&mut driver, VIRTIO_SND_R_PCM_START, stream_id);
    info!("SND: Hardware playback started");

    // ── Mount VFS provider at /dev/audio/card0/ ───────────────────────────────
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

    // ── Main event loop ───────────────────────────────────────────────────────
    let dma_dev = driver.claim_thing();
    let mut card = AudioCard::new();
    let mut provider_loop = ProviderLoop::new(req_read);

    loop {
        // 1. Process any pending VFS RPC requests (non-blocking).
        let mut had_rpc = false;
        loop {
            match provider_loop.try_next_request() {
                Ok(Some(req)) => {
                    had_rpc = true;
                    let prev_free = card.ring.free_space();
                    let (resp, ring_changed) = dispatch_rpc(req.op, &req.payload, &mut card);
                    if let Err(e) = provider_loop.send_response(req.resp_port, resp) {
                        warn!("SND: send_response failed: {:?}", e);
                    }

                    // Notify waiting writers on ring change (e.g. Write enqueued data,
                    // freeing up space for the next writer if partially consumed by HW).
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
                    warn!("SND: provider channel error: {:?}", e);
                    break;
                }
            }
        }

        // 2. Detect xruns from the event queue.
        if process_event_queue(&mut driver) {
            card.xruns += 1;
            warn!("SND: PCM xrun");
        }

        // 3. Recycle completed TX descriptors.
        let prev_free_before_tx = card.ring.free_space();
        process_tx_queue(&mut driver);

        // 4. Feed hardware from ring if running.
        if card.state == AudioState::Running as u32 || card.state == AudioState::Draining as u32 {
            let chunk = 4096usize;
            if card.ring.available() >= chunk {
                let dma_addr = match stem::syscall::device_alloc_dma(dma_dev, 2) {
                    Ok(a) => a,
                    Err(_) => {
                        stem::syscall::yield_now();
                        continue;
                    }
                };
                let phys = match stem::syscall::device_dma_phys(dma_addr) {
                    Ok(p) => p,
                    Err(_) => {
                        stem::syscall::yield_now();
                        continue;
                    }
                };
                let ptr = dma_addr as *mut u8;
                let hdr_sz = size_of::<VirtioSndPcmXfer>();
                unsafe {
                    *(ptr as *mut VirtioSndPcmXfer) = VirtioSndPcmXfer { stream_id };
                }
                let data_slice = unsafe { core::slice::from_raw_parts_mut(ptr.add(hdr_sz), chunk) };
                let n = card.ring.dequeue(data_slice);
                if n > 0 {
                    let bpf = card.bytes_per_frame().max(1);
                    card.hw_frame += (n / bpf) as u64;
                    loop {
                        let added = {
                            let q = driver.queue_mut(VIRTIO_SND_VQ_TX).unwrap();
                            q.add_buffer_single(phys, (hdr_sz + n) as u32, false).is_some()
                        };
                        if added {
                            driver.notify_queue(VIRTIO_SND_VQ_TX);
                            stem::syscall::yield_now();
                            break;
                        } else {
                            process_tx_queue(&mut driver);
                            stem::syscall::yield_now();
                        }
                    }
                }

                // Notify writers that ring space opened up.
                if card.out0_subscribed && card.ring.free_space() > prev_free_before_tx {
                    let _ = stem::syscall::vfs::vfs_notify(
                        req_write,
                        HANDLE_OUT0,
                        abi::syscall::poll_flags::POLLOUT,
                    );
                }

                // Stop if draining and ring is now empty.
                if card.state == AudioState::Draining as u32 && card.ring.available() == 0 {
                    card.state = AudioState::Stopped as u32;
                    send_pcm_command(&mut driver, VIRTIO_SND_R_PCM_STOP, stream_id);
                }
            }
        }

        if !had_rpc {
            stem::time::sleep_ms(1);
        }
    }
}

fn resolve_device_path_from_boot_fd(boot_fd: usize) -> Option<String> {
    if boot_fd == 0 {
        return None;
    }

    // New devd path: boot arg contains a DriverEntryCtx payload.
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
    if path.is_empty() {
        None
    } else {
        Some(path)
    }
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

fn process_tx_queue(driver: &mut VirtioDevice) {
    let q = driver.queue_mut(VIRTIO_SND_VQ_TX).unwrap();
    while q.poll_used().is_some() {}
}

fn populate_event_queue(driver: &mut VirtioDevice) {
    let dma_dev = driver.claim_thing();
    {
        let q = driver.queue_mut(VIRTIO_SND_VQ_EVENT).unwrap();
        for _ in 0..8 {
            let size = size_of::<VirtioSndEvent>();
            let dma = stem::syscall::device_alloc_dma(dma_dev, 1).unwrap();
            let phys = stem::syscall::device_dma_phys(dma).unwrap();
            q.add_buffer_single(phys, size as u32, true);
        }
    }
    driver.notify_queue(VIRTIO_SND_VQ_EVENT);
}

fn process_event_queue(driver: &mut VirtioDevice) -> bool {
    let dma_dev = driver.claim_thing();
    let mut needs_notify = false;
    let mut xrun_seen = false;
    {
        let q = driver.queue_mut(VIRTIO_SND_VQ_EVENT).unwrap();
        while let Some((_desc_id, _len)) = q.poll_used() {
            xrun_seen = true;
            let size = size_of::<VirtioSndEvent>();
            let dma = stem::syscall::device_alloc_dma(dma_dev, 1).unwrap();
            let phys = stem::syscall::device_dma_phys(dma).unwrap();
            q.add_buffer_single(phys, size as u32, true);
            needs_notify = true;
        }
    }
    if needs_notify {
        driver.notify_queue(VIRTIO_SND_VQ_EVENT);
    }
    xrun_seen
}

fn send_pcm_command(driver: &mut VirtioDevice, cmd: u32, stream_id: u32) {
    let dma_req = stem::syscall::device_alloc_dma(driver.claim_thing(), 1).unwrap();
    let phys_req = stem::syscall::device_dma_phys(dma_req).unwrap();
    let dma_resp = stem::syscall::device_alloc_dma(driver.claim_thing(), 1).unwrap();
    let phys_resp = stem::syscall::device_dma_phys(dma_resp).unwrap();
    unsafe {
        *(dma_req as *mut VirtioSndPcmHdr) =
            VirtioSndPcmHdr { hdr: VirtioSndHdr { code: cmd }, stream_id };
        *(dma_resp as *mut VirtioSndHdr) = VirtioSndHdr { code: 0 };
    }
    let bufs = [
        (phys_req, size_of::<VirtioSndPcmHdr>() as u32, false),
        (phys_resp, size_of::<VirtioSndHdr>() as u32, true),
    ];
    {
        let q = driver.queue_mut(VIRTIO_SND_VQ_CONTROL).unwrap();
        q.add_buffer(&bufs);
    }
    driver.notify_queue(VIRTIO_SND_VQ_CONTROL);
    loop {
        let done = {
            let q = driver.queue_mut(VIRTIO_SND_VQ_CONTROL).unwrap();
            q.poll_used().is_some()
        };
        if done {
            break;
        }
        stem::time::sleep_ms(1);
    }
}

fn configure_stream(driver: &mut VirtioDevice, stream_id: u32) {
    let dma_req = stem::syscall::device_alloc_dma(driver.claim_thing(), 1).unwrap();
    let phys_req = stem::syscall::device_dma_phys(dma_req).unwrap();
    let dma_resp = stem::syscall::device_alloc_dma(driver.claim_thing(), 1).unwrap();
    let phys_resp = stem::syscall::device_dma_phys(dma_resp).unwrap();
    unsafe {
        *(dma_req as *mut VirtioSndPcmSetParams) = VirtioSndPcmSetParams {
            hdr: VirtioSndHdr { code: VIRTIO_SND_R_PCM_SET_PARAMS },
            buffer_bytes: 65536,
            period_bytes: 4096,
            features: 0,
            channels: 2,
            format: VIRTIO_SND_PCM_FMT_S16,
            rate: VIRTIO_SND_PCM_RATE_44100,
            padding: 0,
        };
    }
    let bufs = [
        (phys_req, size_of::<VirtioSndPcmSetParams>() as u32, false),
        (phys_resp, size_of::<VirtioSndHdr>() as u32, true),
    ];
    {
        let q = driver.queue_mut(VIRTIO_SND_VQ_CONTROL).unwrap();
        q.add_buffer(&bufs);
    }
    driver.notify_queue(VIRTIO_SND_VQ_CONTROL);
    loop {
        let done = {
            let q = driver.queue_mut(VIRTIO_SND_VQ_CONTROL).unwrap();
            q.poll_used().is_some()
        };
        if done {
            break;
        }
        stem::time::sleep_ms(1);
    }
    send_pcm_command(driver, VIRTIO_SND_R_PCM_PREPARE, stream_id);
}

fn find_output_stream(_driver: &mut VirtioDevice) -> Option<u32> {
    Some(0) // Stream 0 is the output in a standard virtio-snd device.
}
