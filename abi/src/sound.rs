//! VFS-first audio ABI — device-call ops, stream parameters, and status types.
//!
//! # Architecture
//!
//! Audio devices are exposed through the VFS at `/dev/audio/card<N>/`:
//! ```text
//! /dev/audio/card0/
//!     ctl         — control node: card info and format enumeration
//!     out0        — PCM playback stream (write PCM here)
//!     in0         — PCM capture stream  (read PCM from here)
//! ```
//!
//! ## Stream lifecycle
//! 1. Open `/dev/audio/card0/out0` with `O_RDWR`.
//! 2. Call `AUDIO_SET_PARAMS` with the desired [`AudioParams`].
//! 3. Call `AUDIO_START`.
//! 4. `write()` PCM frames in a loop; use `poll(POLLOUT)` for backpressure.
//! 5. Call `AUDIO_DRAIN` (optional) before close to play remaining buffered frames.
//! 6. Call `AUDIO_STOP` when done.

// ── Sample format ──────────────────────────────────────────────────────────────

/// PCM sample format tag.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AudioSampleFormat {
    /// Unsigned 8-bit.
    U8 = 0,
    /// Signed 16-bit little-endian.
    #[default]
    S16LE = 1,
    /// Signed 16-bit big-endian.
    S16BE = 2,
    /// Signed 32-bit little-endian.
    S32LE = 3,
    /// 32-bit IEEE 754 float, little-endian.
    F32LE = 4,
}

impl AudioSampleFormat {
    /// Bytes per sample for one channel.
    pub const fn bytes_per_sample(self) -> u32 {
        match self {
            Self::U8 => 1,
            Self::S16LE | Self::S16BE => 2,
            Self::S32LE | Self::F32LE => 4,
        }
    }

    /// Try to convert a raw `u32` tag to an `AudioSampleFormat`.
    pub fn from_u32(v: u32) -> Option<Self> {
        match v {
            0 => Some(Self::U8),
            1 => Some(Self::S16LE),
            2 => Some(Self::S16BE),
            3 => Some(Self::S32LE),
            4 => Some(Self::F32LE),
            _ => None,
        }
    }
}

// ── Stream state ───────────────────────────────────────────────────────────────

/// Lifecycle state of a PCM stream.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AudioState {
    /// Stream is idle; no PCM is being transferred.
    #[default]
    Stopped = 0,
    /// Stream is running; PCM data flows to/from hardware.
    Running = 1,
    /// Playback only: draining remaining buffered frames before stopping.
    Draining = 2,
}

// ── Stream parameters ──────────────────────────────────────────────────────────

/// Parameters used to configure a PCM stream.
///
/// Pass to `AUDIO_SET_PARAMS` before calling `AUDIO_START`.
/// The driver fills `out_ptr` with the nearest accepted values.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct AudioParams {
    /// Sample format (cast of [`AudioSampleFormat`]).
    pub sample_format: u32,
    /// Sample rate in Hz (e.g. 44100, 48000).
    pub rate: u32,
    /// Channel count (1 = mono, 2 = stereo, …).
    pub channels: u32,
    /// Hardware period size in frames.
    ///
    /// One period is the minimum DMA transfer unit.
    /// Smaller periods → lower latency, higher CPU load.
    pub period_frames: u32,
    /// Ring-buffer size in frames (must be a multiple of `period_frames`).
    pub buffer_frames: u32,
    pub _reserved: [u32; 3],
}

// ── Stream capabilities ────────────────────────────────────────────────────────

/// Capability snapshot returned by `AUDIO_GET_INFO`.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct AudioStreamInfo {
    /// Bitmask of supported [`AudioSampleFormat`] values
    /// (bit N set ↔ format N is supported; see [`format_bit`]).
    pub supported_formats: u32,
    /// Minimum supported sample rate in Hz.
    pub min_rate: u32,
    /// Maximum supported sample rate in Hz.
    pub max_rate: u32,
    /// Maximum channel count.
    pub max_channels: u32,
    /// Minimum buffer size in frames.
    pub min_buffer_frames: u32,
    /// Maximum buffer size in frames.
    pub max_buffer_frames: u32,
    /// Minimum period size in frames.
    pub min_period_frames: u32,
    /// Active [`AudioParams`] (zeroed if not yet configured).
    pub current_params: AudioParams,
    pub _reserved: [u32; 4],
}

// ── Live status ────────────────────────────────────────────────────────────────

/// Live status snapshot returned by `AUDIO_GET_STATUS`.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct AudioStatus {
    /// Current stream state (cast of [`AudioState`]).
    pub state: u32,
    /// Hardware frame counter (monotonically increasing).
    pub hw_frame: u64,
    /// Application frame counter (total frames written or read so far).
    pub app_frame: u64,
    /// Available space for writes (playback) or bytes ready to read (capture),
    /// expressed in frames.
    pub avail_frames: u32,
    /// Xrun (underrun or overrun) event count since stream start.
    pub xruns: u32,
    pub _reserved: [u32; 4],
}

// ── Device-call operation codes ────────────────────────────────────────────────

/// `DeviceCall::op` — read [`AudioStreamInfo`] into `out_ptr`.
pub const AUDIO_GET_INFO: u32 = 0x8001;

/// `DeviceCall::op` — configure the stream.
///
/// `in_ptr` → [`AudioParams`] (desired).
/// `out_ptr` → [`AudioParams`] (nearest accepted values, filled by driver).
pub const AUDIO_SET_PARAMS: u32 = 0x8002;

/// `DeviceCall::op` — read the currently active [`AudioParams`] into `out_ptr`.
pub const AUDIO_GET_PARAMS: u32 = 0x8003;

/// `DeviceCall::op` — read [`AudioStatus`] into `out_ptr`.
pub const AUDIO_GET_STATUS: u32 = 0x8004;

/// `DeviceCall::op` — start PCM transfer (no payload).
pub const AUDIO_START: u32 = 0x8005;

/// `DeviceCall::op` — stop immediately, discarding buffered data (no payload).
pub const AUDIO_STOP: u32 = 0x8006;

/// `DeviceCall::op` — play remaining buffered frames then stop (playback only, no payload).
pub const AUDIO_DRAIN: u32 = 0x8007;

/// `DeviceCall::op` — query mapped-ring control endpoint information.
///
/// `out_ptr` -> [`AudioMappedRingInfo`].
pub const AUDIO_GET_MAPPED_RING_INFO: u32 = 0x8008;

/// `DeviceCall::op` — read mapped/legacy ring runtime counters.
///
/// `out_ptr` -> [`AudioMappedRingStats`].
pub const AUDIO_GET_MAPPED_RING_STATS: u32 = 0x8009;

/// Mapped-ring protocol version.
pub const AUDIO_MAPPED_RING_VERSION: u32 = 1;

/// Producer (app) -> driver setup message sent on the mapped-ring control socket.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct AudioMappedRingSetup {
    /// Must be [`AUDIO_MAPPED_RING_VERSION`].
    pub version: u32,
    /// Total bytes in the shared memory region.
    pub ring_bytes: u32,
    /// Reserved; must be zero.
    pub _reserved: [u32; 2],
}

/// Header stored at the start of the mapped shared memory region.
///
/// Layout in memory: `[AudioMappedRingHeader][PCM bytes...]`.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct AudioMappedRingHeader {
    /// Producer write index in bytes, modulo `capacity_bytes`.
    pub write_index: u32,
    /// Consumer read index in bytes, modulo `capacity_bytes`.
    pub read_index: u32,
    /// Data area size in bytes (not including this header).
    pub capacity_bytes: u32,
    /// Producer/consumer flags (reserved for future use).
    pub flags: u32,
    pub _reserved: [u32; 4],
}

/// Endpoint details returned by [`AUDIO_GET_MAPPED_RING_INFO`].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioMappedRingInfo {
    /// 0 => unsupported, non-zero => supported.
    pub supported: u32,
    /// Suggested total shared memory size (header + data bytes).
    pub suggested_ring_bytes: u32,
    /// Byte length of UTF-8 control socket path in `socket_path`.
    pub socket_path_len: u32,
    /// NUL-free UTF-8 path for AF_UNIX control socket.
    pub socket_path: [u8; 96],
    pub _reserved: [u32; 4],
}

impl Default for AudioMappedRingInfo {
    fn default() -> Self {
        Self {
            supported: 0,
            suggested_ring_bytes: 0,
            socket_path_len: 0,
            socket_path: [0; 96],
            _reserved: [0; 4],
        }
    }
}

/// Runtime counters and occupancy for audio ring diagnostics.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct AudioMappedRingStats {
    /// 1 when mapped-ring streaming is active; 0 for legacy write ring.
    pub mapped_active: u32,
    /// Current queued PCM bytes in the active ring.
    pub ring_fill_bytes: u32,
    /// Total PCM capacity bytes in the active ring.
    pub ring_capacity_bytes: u32,
    /// Number of successful mapped-ring attach handshakes since boot.
    pub mapped_attach_count: u32,
    /// Number of mapped-ring detach/teardown events since boot.
    pub mapped_detach_count: u32,
    /// Number of mapped-ring starvation edge events while running.
    pub mapped_underrun_events: u32,
    /// Total bytes consumed by hardware from mapped ring.
    pub mapped_bytes_consumed: u64,
    pub _reserved: [u32; 4],
}

// ── Capability helpers ─────────────────────────────────────────────────────────

/// Returns the `AudioStreamInfo::supported_formats` bitmask bit for `fmt`.
pub const fn format_bit(fmt: AudioSampleFormat) -> u32 {
    1 << (fmt as u32)
}
