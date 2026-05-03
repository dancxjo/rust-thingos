//! chime — generates a PCM tone and streams it to the sound driver.
//!
//! Opens `/dev/audio/card0/out0`, configures the stream with `AUDIO_SET_PARAMS`,
//! starts playback with `AUDIO_START`, and streams PCM frames via `vfs_write`.
//! Uses `poll(POLLOUT)` for backpressure.
#![no_std]
#![no_main]
use alloc::vec::Vec;
use core::default::Default;
extern crate alloc;

mod chime;
mod tone;

use abi::device::DeviceKind;
use abi::sound::{
    AUDIO_DRAIN, AUDIO_GET_INFO, AUDIO_GET_MAPPED_RING_INFO, AUDIO_GET_STATUS,
    AUDIO_MAPPED_RING_VERSION, AUDIO_SET_PARAMS, AUDIO_START, AudioMappedRingHeader,
    AudioMappedRingInfo, AudioMappedRingSetup, AudioParams, AudioSampleFormat, AudioState,
    AudioStatus, AudioStreamInfo,
};
use stem::{debug, info};

struct MappedProducer {
    control_fd: u32,
    ring_fd: u32,
    map_addr: usize,
    map_len: usize,
    capacity: usize,
}

fn mapped_enqueue(prod: &mut MappedProducer, src: &[u8]) -> usize {
    let hdr = prod.map_addr as *mut AudioMappedRingHeader;
    let w = unsafe { core::ptr::read_volatile(core::ptr::addr_of!((*hdr).write_index)) } as usize;
    let r = unsafe { core::ptr::read_volatile(core::ptr::addr_of!((*hdr).read_index)) } as usize;
    core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);

    let cap = prod.capacity;
    if cap == 0 {
        return 0;
    }

    // Keep one byte empty so full/empty states are unambiguous.
    let used = if w >= r { w - r } else { cap - (r - w) };
    let free = cap.saturating_sub(used + 1);
    let n = free.min(src.len());
    if n == 0 {
        return 0;
    }

    let base = (prod.map_addr + core::mem::size_of::<AudioMappedRingHeader>()) as *mut u8;
    let first = n.min(cap - w);
    unsafe {
        core::ptr::copy_nonoverlapping(src.as_ptr(), base.add(w), first);
    }
    if first < n {
        unsafe {
            core::ptr::copy_nonoverlapping(src.as_ptr().add(first), base, n - first);
        }
    }

    let new_w = (w + n) % cap;
    core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
    unsafe {
        core::ptr::write_volatile(core::ptr::addr_of_mut!((*hdr).write_index), new_w as u32);
    }
    n
}

fn try_setup_mapped_ring(out_fd: u32) -> Option<MappedProducer> {
    let mut info_buf = AudioMappedRingInfo::default();
    let info_call = abi::device::DeviceCall {
        kind: DeviceKind::Audio,
        op: AUDIO_GET_MAPPED_RING_INFO,
        in_ptr: 0,
        in_len: 0,
        out_ptr: &mut info_buf as *mut AudioMappedRingInfo as u64,
        out_len: core::mem::size_of::<AudioMappedRingInfo>() as u32,
    };
    if stem::syscall::vfs::vfs_device_call_raw(out_fd, &info_call).is_err() {
        return None;
    }
    if info_buf.supported == 0 || info_buf.socket_path_len == 0 {
        return None;
    }

    let path_len = (info_buf.socket_path_len as usize).min(info_buf.socket_path.len());
    let path = core::str::from_utf8(&info_buf.socket_path[..path_len]).ok()?;

    let control_fd = stem::syscall::socket::socket(
        abi::syscall::socket_domain::AF_UNIX,
        abi::syscall::socket_type::SOCK_STREAM,
        0,
    )
    .ok()?;
    if stem::syscall::socket::connect(control_fd, path).is_err() {
        let _ = stem::syscall::vfs::vfs_close(control_fd);
        return None;
    }

    let ring_bytes = (info_buf.suggested_ring_bytes as usize)
        .max(core::mem::size_of::<AudioMappedRingHeader>() + 64 * 1024);
    let ring_fd = stem::syscall::memfd_create("audio-ring", ring_bytes).ok()?;
    let req = abi::vm::VmMapReq {
        addr_hint: 0,
        len: ring_bytes,
        prot: abi::vm::VmProt::READ | abi::vm::VmProt::WRITE | abi::vm::VmProt::USER,
        flags: abi::vm::VmMapFlags::empty(),
        backing: abi::vm::VmBacking::File { thing: ring_fd, offset: 0 },
    };
    let map = match stem::syscall::vm_map(&req) {
        Ok(m) => m,
        Err(_) => {
            let _ = stem::syscall::vfs::vfs_close(ring_fd);
            let _ = stem::syscall::vfs::vfs_close(control_fd);
            return None;
        }
    };

    let cap = map.len.saturating_sub(core::mem::size_of::<AudioMappedRingHeader>());
    let hdr = map.addr as *mut AudioMappedRingHeader;
    unsafe {
        core::ptr::write_volatile(core::ptr::addr_of_mut!((*hdr).write_index), 0);
        core::ptr::write_volatile(core::ptr::addr_of_mut!((*hdr).read_index), 0);
        core::ptr::write_volatile(core::ptr::addr_of_mut!((*hdr).capacity_bytes), cap as u32);
        core::ptr::write_volatile(core::ptr::addr_of_mut!((*hdr).flags), 0);
    }

    let setup = AudioMappedRingSetup {
        version: AUDIO_MAPPED_RING_VERSION,
        ring_bytes: map.len as u32,
        _reserved: [0; 2],
    };
    let setup_bytes = unsafe {
        core::slice::from_raw_parts(
            &setup as *const AudioMappedRingSetup as *const u8,
            core::mem::size_of::<AudioMappedRingSetup>(),
        )
    };
    if stem::syscall::socket::sendmsg(control_fd, setup_bytes, &[ring_fd]).is_err() {
        let _ = stem::syscall::vm_unmap(map.addr, map.len);
        let _ = stem::syscall::vfs::vfs_close(ring_fd);
        let _ = stem::syscall::vfs::vfs_close(control_fd);
        return None;
    }

    Some(MappedProducer {
        control_fd,
        ring_fd,
        map_addr: map.addr,
        map_len: map.len,
        capacity: cap,
    })
}

fn get_audio_status(out_fd: u32) -> Option<AudioStatus> {
    let mut status = AudioStatus::default();
    let call = abi::device::DeviceCall {
        kind: DeviceKind::Audio,
        op: AUDIO_GET_STATUS,
        in_ptr: 0,
        in_len: 0,
        out_ptr: &mut status as *mut AudioStatus as u64,
        out_len: core::mem::size_of::<AudioStatus>() as u32,
    };
    stem::syscall::vfs::vfs_device_call_raw(out_fd, &call).ok()?;
    Some(status)
}

fn request_audio_drain(out_fd: u32) {
    let call = abi::device::DeviceCall {
        kind: DeviceKind::Audio,
        op: AUDIO_DRAIN,
        in_ptr: 0,
        in_len: 0,
        out_ptr: 0,
        out_len: 0,
    };
    let _ = stem::syscall::vfs::vfs_device_call_raw(out_fd, &call);
}

fn wait_for_playback_completion(out_fd: u32, sample_bytes: usize, params: &AudioParams) {
    let fmt = AudioSampleFormat::from_u32(params.sample_format).unwrap_or(AudioSampleFormat::S16LE);
    let bytes_per_frame =
        (fmt.bytes_per_sample() as usize).saturating_mul(params.ports as usize).max(1);
    let queued_frames = sample_bytes / bytes_per_frame;
    let playback_ms = if params.rate > 0 {
        ((queued_frames as u64) * 1000).div_ceil(params.rate as u64)
    } else {
        0
    };
    let deadline_ms = playback_ms.saturating_add(500).clamp(250, 10_000);
    let start_ns = stem::time::monotonic_ns();

    loop {
        if let Some(status) = get_audio_status(out_fd) {
            if status.state == AudioState::Stopped as u32 {
                break;
            }
        }

        let elapsed_ms = stem::time::monotonic_ns().saturating_sub(start_ns) / 1_000_000;
        if elapsed_ms >= deadline_ms {
            break;
        }
        stem::time::sleep_ms(10);
    }
}

fn read_piped_stdin() -> Option<Vec<u8>> {
    use abi::syscall::poll_flags::POLLIN;
    use stem::syscall::vfs::{vfs_isatty, vfs_poll, vfs_read};

    // Interactive launches inherit /dev/console on fd 0. Treat that as
    // "no piped PCM" so we don't block waiting for EOF from a tty.
    if matches!(vfs_isatty(0), Ok(true)) {
        return None;
    }

    let mut pollfds = [abi::syscall::PollHandle { handle: 0, events: POLLIN, revents: 0 }];

    if vfs_poll(&mut pollfds, 0).is_err() || (pollfds[0].revents & POLLIN) == 0 {
        return None;
    }

    let mut out = Vec::new();
    let mut chunk = [0u8; 4096];
    loop {
        match vfs_read(0, &mut chunk) {
            Ok(0) => break,
            Ok(n) => out.extend_from_slice(&chunk[..n]),
            Err(_) => break,
        }
    }

    if out.is_empty() { None } else { Some(out) }
}

fn interrupted_while_waiting_for_audio() -> bool {
    use abi::errors::Errno;
    use abi::syscall::PollHandle;
    use abi::syscall::poll_flags::POLLIN;
    use stem::syscall::vfs::{vfs_isatty, vfs_poll, vfs_read};

    // Only interactive runs should watch for Ctrl-C from stdin.
    if !matches!(vfs_isatty(0), Ok(true)) {
        return false;
    }

    let mut pollfds = [PollHandle { handle: 0, events: POLLIN, revents: 0 }];
    match vfs_poll(&mut pollfds, 0) {
        Ok(0) => false,
        Ok(_) => {
            if (pollfds[0].revents & POLLIN) == 0 {
                return false;
            }
            let mut byte = [0u8; 1];
            match vfs_read(0, &mut byte) {
                Ok(1) => byte[0] == 0x03,
                // ISIG-enabled ttys can convert Ctrl-C into EINTR instead of a byte.
                Err(Errno::EINTR) => true,
                _ => false,
            }
        }
        Err(Errno::EINTR) => true,
        Err(_) => false,
    }
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let tone_freq: Option<f64> = None;
    let seconds: f64 = 1.0;

    info!("Waiting for audio output...");

    // Wait for the audio driver to mount its VFS tree.
    let mut open_attempts: u32 = 0;
    let out_fd = loop {
        use abi::syscall::vfs_flags::O_RDWR;
        use stem::syscall::vfs::vfs_open;
        match vfs_open("/dev/audio/card0/out0", O_RDWR) {
            Ok(fd) => break fd,
            Err(err) => {
                open_attempts = open_attempts.saturating_add(1);
                if open_attempts == 1 || open_attempts % 20 == 0 {
                    debug!("Audio output not ready yet: attempt={} err={:?}", open_attempts, err);
                }
                if interrupted_while_waiting_for_audio() {
                    info!("Audio startup was interrupted");
                    stem::syscall::exit(130);
                }
                stem::time::sleep_ms(100);
            }
        }
    };

    debug!("Audio output opened with fd={}", out_fd);

    // Query capabilities.
    let mut info_buf = AudioStreamInfo::default();
    {
        let call = abi::device::DeviceCall {
            kind: DeviceKind::Audio,
            op: AUDIO_GET_INFO,
            in_ptr: 0,
            in_len: 0,
            out_ptr: &mut info_buf as *mut AudioStreamInfo as u64,
            out_len: core::mem::size_of::<AudioStreamInfo>() as u32,
        };
        let _ = stem::syscall::vfs::vfs_device_call_raw(out_fd, &call);
    }

    // Request S16LE at 44100 Hz stereo.
    let desired = AudioParams {
        sample_format: AudioSampleFormat::S16LE as u32,
        rate: 44100,
        ports: 2,
        period_frames: 1024,
        buffer_frames: 4096,
        _reserved: [0; 3],
    };
    let mut accepted = AudioParams::default();
    {
        let call = abi::device::DeviceCall {
            kind: DeviceKind::Audio,
            op: AUDIO_SET_PARAMS,
            in_ptr: &desired as *const AudioParams as u64,
            in_len: core::mem::size_of::<AudioParams>() as u32,
            out_ptr: &mut accepted as *mut AudioParams as u64,
            out_len: core::mem::size_of::<AudioParams>() as u32,
        };
        let _ = stem::syscall::vfs::vfs_device_call_raw(out_fd, &call);
    }

    let sample_rate = accepted.rate;
    debug!(
        "Audio stream configured: rate={}Hz format={} channels={}",
        sample_rate, accepted.sample_format, accepted.ports
    );

    // Start playback.
    {
        let call = abi::device::DeviceCall {
            kind: DeviceKind::Audio,
            op: AUDIO_START,
            in_ptr: 0,
            in_len: 0,
            out_ptr: 0,
            out_len: 0,
        };
        let _ = stem::syscall::vfs::vfs_device_call_raw(out_fd, &call);
    }

    // Consume piped PCM when stdin is connected, otherwise generate samples.
    let samples: Vec<u8> = if let Some(stdin_pcm) = read_piped_stdin() {
        debug!("Streaming piped PCM from stdin: {} bytes", stdin_pcm.len());
        stdin_pcm
    } else if let Some(freq) = tone_freq {
        debug!("Generating {}Hz sine wave for {}s...", freq, seconds);
        let mut gen = tone::ToneGenerator::new(freq, sample_rate as f64);
        let count = (seconds * sample_rate as f64) as usize;
        let mut v_i16 = Vec::with_capacity(count * 2);
        unsafe { v_i16.set_len(count * 2) };
        gen.fill_buffer(&mut v_i16);
        let mut v_u8 = Vec::with_capacity(count * 4);
        for s in v_i16 {
            v_u8.extend_from_slice(&s.to_le_bytes());
        }
        v_u8
    } else {
        debug!("Generating classic chime...");
        chime::generate_chime(sample_rate)
    };

    info!("Playing startup chime");

    if let Some(mapped) = try_setup_mapped_ring(out_fd) {
        debug!("Using mapped audio ring for {} bytes", samples.len());
        let mut mapped = mapped;
        let mut offset = 0usize;
        while offset < samples.len() {
            let n = mapped_enqueue(&mut mapped, &samples[offset..]);
            if n == 0 {
                stem::time::sleep_ms(1);
            } else {
                offset += n;
            }
        }
        request_audio_drain(out_fd);
        wait_for_playback_completion(out_fd, samples.len(), &accepted);
        let _ = stem::syscall::vm_unmap(mapped.map_addr, mapped.map_len);
        let _ = stem::syscall::vfs::vfs_close(mapped.ring_fd);
        let _ = stem::syscall::vfs::vfs_close(mapped.control_fd);
        let _ = stem::syscall::vfs::vfs_close(out_fd);
        info!("Startup chime finished");
        stem::syscall::exit(0);
    }

    let chunk_size = 4096usize;
    let mut offset = 0;

    while offset < samples.len() {
        let to_write = (samples.len() - offset).min(chunk_size);
        let buf = &samples[offset..offset + to_write];

        let mut sent = 0;
        while sent < buf.len() {
            use stem::syscall::vfs::vfs_write;
            match vfs_write(out_fd, &buf[sent..]) {
                Ok(0) | Err(_) => {
                    // No space — wait for POLLOUT.
                    let mut pollfds = [abi::syscall::PollHandle {
                        handle: out_fd as i32,
                        events: abi::syscall::poll_flags::POLLOUT,
                        revents: 0,
                    }];
                    let _ = stem::syscall::vfs::vfs_poll(&mut pollfds, u64::MAX);
                }
                Ok(n) => {
                    sent += n;
                }
            }
        }
        offset += to_write;
    }

    request_audio_drain(out_fd);
    wait_for_playback_completion(out_fd, samples.len(), &accepted);
    let _ = stem::syscall::vfs::vfs_close(out_fd);
    info!("Startup chime finished");
    stem::syscall::exit(0);
}
