//! VirtIO-NET VFS provider.
//!
//! Serves `/dev/net/virtio0/` with the following files:
//!
//! ```text
//! /dev/net/virtio0/
//! ├── ctl        ← write: "up", "down", "set-mtu <n>"
//! ├── status     ← read: human-readable device state
//! ├── mac        ← read: MAC address
//! ├── mtu        ← read/write: MTU value
//! ├── rx         ← read: stream of length-prefixed raw Ethernet frames
//! ├── tx         ← write: length-prefixed raw Ethernet frames
//! ├── features   ← read: virtio feature flags (hex text)
//! └── events     ← pollable: newline-delimited link events
//! ```
//!
//! Frame stream format on `rx` and `tx`:
//! ```text
//! [4 bytes: frame_length_le][frame_length bytes: raw Ethernet frame]
//! ```
//!
//! ## IPC substrate
//!
//! Request dispatch uses [`ipc_helpers::provider::ProviderLoop`] + [`ProviderResponse`],
//! the canonical VFS provider helper library.  The main event loop calls
//! [`ProviderLoop::try_next_request`] (non-blocking) so that hardware polling
//! can interleave with VFS RPC handling without either side starving the other.
extern crate alloc;

use alloc::collections::VecDeque;
use alloc::vec::Vec;

use abi::attrs::{
    ATTR_OP_GET, ATTR_OP_LIST, ATTR_OP_REMOVE, ATTR_OP_SET, AttrListEntryHeader, AttrNameHeader,
    AttrType, AttrValueHeader,
};
use abi::device::{DeviceCall, DeviceKind};
use abi::errors::Errno;
use abi::vfs_rpc::VfsRpcOp;
use ipc_helpers::provider::{ProviderRequest, ProviderResponse};
use stem::{debug, trace};

use crate::driver::VirtioNetDriver;

// ── Handle IDs ────────────────────────────────────────────────────────────────

/// Root directory handle.
pub const HANDLE_ROOT: u64 = 0;
const ATTR_DRIVER_NAME: &str = "virtio_netd";
const HANDLE_CTL: u64 = 1;
const HANDLE_STATUS: u64 = 2;
const HANDLE_MAC: u64 = 3;
const HANDLE_MTU: u64 = 4;
pub const HANDLE_RX: u64 = 5;
const HANDLE_TX: u64 = 6;
const HANDLE_FEATURES: u64 = 7;
pub const HANDLE_EVENTS: u64 = 8;

// ── Mode bits ─────────────────────────────────────────────────────────────────

const S_IFDIR: u32 = 0o040000;
const S_IFREG: u32 = 0o100000;

// ── Poll readiness bits ───────────────────────────────────────────────────────

const POLLIN: u32 = 0x0001;

// ── Shared driver state ───────────────────────────────────────────────────────

/// Mutable state shared between the hardware poll loop and the VFS handler.
pub struct NetVfsState {
    /// MAC address.
    pub mac: [u8; 6],
    /// Whether the link is considered up.
    pub link_up: bool,
    /// Current MTU (default 1500).
    pub mtu: u32,
    /// Negotiated virtio feature flags (read from device after init).
    pub features: u32,
    /// Buffered received Ethernet frames (raw frame bytes, no length prefix).
    pub rx_queue: VecDeque<Vec<u8>>,
    /// Buffered link-state events (newline-terminated strings).
    pub events_queue: VecDeque<Vec<u8>>,
}

impl NetVfsState {
    /// Create a new state instance.
    pub fn new(mac: [u8; 6], link_up: bool, features: u32) -> Self {
        Self {
            mac,
            link_up,
            mtu: 1500,
            features,
            rx_queue: VecDeque::new(),
            events_queue: VecDeque::new(),
        }
    }

    /// Push a received Ethernet frame onto the RX queue.
    pub fn push_rx_frame(&mut self, frame: Vec<u8>) {
        self.rx_queue.push_back(frame);
    }

    /// Push a newline-terminated link-state event string.
    pub fn push_event(&mut self, event: &str) {
        let mut ev = Vec::from(event.as_bytes());
        if ev.last() != Some(&b'\n') {
            ev.push(b'\n');
        }
        self.events_queue.push_back(ev);
    }
}

// ── VFS RPC dispatch ──────────────────────────────────────────────────────────

/// Dispatch one decoded VFS RPC request and return the appropriate response.
///
/// The caller is responsible for sending the returned [`ProviderResponse`]
/// back to the kernel via [`ipc_helpers::provider::ProviderLoop::send_response`].
pub fn handle_vfs_rpc(
    state: &mut NetVfsState,
    driver: &mut VirtioNetDriver,
    req: &ProviderRequest,
) -> ProviderResponse {
    let op = req.op;
    let payload = &req.payload;

    trace!("VIRTIO_NETD: rpc op={:?} payload_len={}", op, payload.len(),);

    match op {
        VfsRpcOp::Lookup => handle_lookup(payload),
        VfsRpcOp::Read => handle_read(state, payload),
        VfsRpcOp::Write => handle_write(state, driver, payload),
        VfsRpcOp::Readdir => handle_readdir(payload),
        VfsRpcOp::Stat => handle_stat(payload),
        VfsRpcOp::Close => ProviderResponse::ok_empty(),
        VfsRpcOp::Poll => handle_poll(state, payload),
        VfsRpcOp::DeviceCall => handle_device_call(state, payload),
        VfsRpcOp::Rename => ProviderResponse::err(Errno::ENOSYS),
        VfsRpcOp::SubscribeReady => ProviderResponse::ok_empty(),
        VfsRpcOp::UnsubscribeReady => ProviderResponse::ok_empty(),
    }
}

fn handle_device_call(state: &mut NetVfsState, payload: &[u8]) -> ProviderResponse {
    let dc_size = core::mem::size_of::<DeviceCall>();
    if payload.len() < 8 + dc_size {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes(payload[..8].try_into().unwrap_or([0; 8]));
    if handle == HANDLE_ROOT {
        return ProviderResponse::err(Errno::EISDIR);
    }
    let call: DeviceCall =
        unsafe { core::ptr::read_unaligned(payload[8..8 + dc_size].as_ptr() as *const _) };
    if call.kind != DeviceKind::Attr {
        return ProviderResponse::err(Errno::ENOSYS);
    }
    let in_data = &payload[8 + dc_size..];
    match call.op {
        ATTR_OP_GET => attr_get(state, in_data),
        ATTR_OP_SET => attr_set(state, in_data),
        ATTR_OP_REMOVE => ProviderResponse::err(Errno::ENOTSUP),
        ATTR_OP_LIST => attr_list(),
        _ => ProviderResponse::err(Errno::ENOSYS),
    }
}

fn attr_get(state: &NetVfsState, payload: &[u8]) -> ProviderResponse {
    if payload.len() < core::mem::size_of::<AttrNameHeader>() {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let name_len = u16::from_le_bytes([payload[0], payload[1]]) as usize;
    let off = core::mem::size_of::<AttrNameHeader>();
    if payload.len() < off + name_len {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let name = match core::str::from_utf8(&payload[off..off + name_len]) {
        Ok(v) => v,
        Err(_) => return ProviderResponse::err(Errno::EINVAL),
    };
    let mut value_buf = [0u8; 8];
    let (ty, bytes): (AttrType, &[u8]) = match name {
        "driver.name" => (AttrType::Utf8, ATTR_DRIVER_NAME.as_bytes()),
        "net.mtu" => {
            value_buf.copy_from_slice(&(state.mtu as u64).to_le_bytes());
            (AttrType::U64, &value_buf)
        }
        "net.link_up" => {
            value_buf[0] = if state.link_up { 1 } else { 0 };
            (AttrType::Bool, &value_buf[..1])
        }
        _ => return ProviderResponse::err(Errno::ENOENT),
    };
    let mut out = Vec::with_capacity(core::mem::size_of::<AttrValueHeader>() + bytes.len());
    out.push(ty as u8);
    out.push(0);
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(bytes);
    ProviderResponse::ok_device_call(bytes.len() as u32, &out)
}

fn attr_set(state: &mut NetVfsState, payload: &[u8]) -> ProviderResponse {
    if payload.len() < core::mem::size_of::<abi::attrs::AttrSetHeader>() {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let name_len = u16::from_le_bytes([payload[0], payload[1]]) as usize;
    let value_type = payload[2];
    let value_len = u32::from_le_bytes([payload[4], payload[5], payload[6], payload[7]]) as usize;
    let off = core::mem::size_of::<abi::attrs::AttrSetHeader>();
    if payload.len() < off + name_len + value_len {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let name = match core::str::from_utf8(&payload[off..off + name_len]) {
        Ok(v) => v,
        Err(_) => return ProviderResponse::err(Errno::EINVAL),
    };
    let value = &payload[off + name_len..off + name_len + value_len];
    match name {
        "net.mtu" => {
            if value_type != AttrType::U64 as u8 || value.len() != 8 {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let mtu = u64::from_le_bytes(value.try_into().unwrap()) as u32;
            state.mtu = mtu;
            ProviderResponse::ok_device_call(value.len() as u32, &[])
        }
        "net.link_up" => {
            if value_type != AttrType::Bool as u8 || value.len() != 1 {
                return ProviderResponse::err(Errno::EINVAL);
            }
            state.link_up = value[0] != 0;
            ProviderResponse::ok_device_call(1, &[])
        }
        _ => ProviderResponse::err(Errno::ENOENT),
    }
}

fn attr_list() -> ProviderResponse {
    let mut out = Vec::new();
    for (name, ty, value_len) in [
        ("driver.name", AttrType::Utf8, ATTR_DRIVER_NAME.len() as u32),
        ("net.mtu", AttrType::U64, 8u32),
        ("net.link_up", AttrType::Bool, 1u32),
    ] {
        let hdr = AttrListEntryHeader {
            name_len: name.len() as u16,
            value_type: ty as u8,
            flags: 0,
            value_len,
        };
        out.extend_from_slice(&hdr.name_len.to_le_bytes());
        out.push(hdr.value_type);
        out.push(0);
        out.extend_from_slice(&hdr.value_len.to_le_bytes());
        out.extend_from_slice(name.as_bytes());
    }
    ProviderResponse::ok_device_call(3, &out)
}

// ── Lookup ────────────────────────────────────────────────────────────────────

fn handle_lookup(payload: &[u8]) -> ProviderResponse {
    if payload.len() < 4 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let path_len = u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]) as usize;
    if payload.len() < 4 + path_len {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let path = match core::str::from_utf8(&payload[4..4 + path_len]) {
        Ok(s) => s,
        Err(_) => return ProviderResponse::err(Errno::EINVAL),
    };
    let path = path.trim_matches('/');
    stem::debug!("VIRTIO_NETD: lookup '{}'", path);

    let handle: u64 = match path {
        "" => HANDLE_ROOT,
        "ctl" => HANDLE_CTL,
        "status" => HANDLE_STATUS,
        "mac" => HANDLE_MAC,
        "mtu" => HANDLE_MTU,
        "rx" => HANDLE_RX,
        "tx" => HANDLE_TX,
        "features" => HANDLE_FEATURES,
        "events" => HANDLE_EVENTS,
        _ => {
            stem::debug!("VIRTIO_NETD: lookup '{}' -> ENOENT", path);
            return ProviderResponse::err(Errno::ENOENT);
        }
    };

    stem::debug!("VIRTIO_NETD: lookup '{}' -> handle={}", path, handle);
    ProviderResponse::ok_u64(handle)
}

// ── Stat ──────────────────────────────────────────────────────────────────────

fn handle_stat(payload: &[u8]) -> ProviderResponse {
    if payload.len() < 8 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes([
        payload[0], payload[1], payload[2], payload[3], payload[4], payload[5], payload[6],
        payload[7],
    ]);

    let (mode, size): (u32, u64) = match handle {
        HANDLE_ROOT => (S_IFDIR | 0o755, 0),
        HANDLE_CTL | HANDLE_TX => (S_IFREG | 0o200, 0), // write-only
        HANDLE_RX | HANDLE_EVENTS => (S_IFREG | 0o400, 0), // read-only stream
        HANDLE_STATUS | HANDLE_MAC | HANDLE_FEATURES => (S_IFREG | 0o444, 0),
        HANDLE_MTU => (S_IFREG | 0o644, 0),
        _ => return ProviderResponse::err(Errno::ENOENT),
    };

    ProviderResponse::ok_stat(mode, size, handle)
}

// ── Readdir ───────────────────────────────────────────────────────────────────

struct DirEntry {
    name: &'static str,
    handle: u64,
}

const DIR_ENTRIES: &[DirEntry] = &[
    DirEntry { name: "ctl", handle: HANDLE_CTL },
    DirEntry { name: "status", handle: HANDLE_STATUS },
    DirEntry { name: "mac", handle: HANDLE_MAC },
    DirEntry { name: "mtu", handle: HANDLE_MTU },
    DirEntry { name: "rx", handle: HANDLE_RX },
    DirEntry { name: "tx", handle: HANDLE_TX },
    DirEntry { name: "features", handle: HANDLE_FEATURES },
    DirEntry { name: "events", handle: HANDLE_EVENTS },
];

fn handle_readdir(payload: &[u8]) -> ProviderResponse {
    if payload.len() < 20 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes([
        payload[0], payload[1], payload[2], payload[3], payload[4], payload[5], payload[6],
        payload[7],
    ]);
    let offset = u64::from_le_bytes([
        payload[8],
        payload[9],
        payload[10],
        payload[11],
        payload[12],
        payload[13],
        payload[14],
        payload[15],
    ]) as usize;
    let max_bytes =
        u32::from_le_bytes([payload[16], payload[17], payload[18], payload[19]]) as usize;

    if handle != HANDLE_ROOT {
        return ProviderResponse::err(Errno::EINVAL);
    }

    let mut out: Vec<u8> = Vec::new();
    for entry in DIR_ENTRIES.iter().skip(offset) {
        let name_bytes = entry.name.as_bytes();
        let name_len = name_bytes.len().min(255) as u8;
        // DT_REG = 8
        let file_type: u8 = 8;
        let entry_size = 10 + name_len as usize;
        if out.len() + entry_size > max_bytes {
            break;
        }
        // DirentWire: [ino: u64][file_type: u8][name_len: u8][name bytes]
        out.extend_from_slice(&entry.handle.to_le_bytes());
        out.push(file_type);
        out.push(name_len);
        out.extend_from_slice(&name_bytes[..name_len as usize]);
    }

    ProviderResponse::ok_read(&out)
}

// ── Read ──────────────────────────────────────────────────────────────────────

fn handle_read(state: &mut NetVfsState, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 20 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes([
        payload[0], payload[1], payload[2], payload[3], payload[4], payload[5], payload[6],
        payload[7],
    ]);
    let offset = u64::from_le_bytes([
        payload[8],
        payload[9],
        payload[10],
        payload[11],
        payload[12],
        payload[13],
        payload[14],
        payload[15],
    ]) as usize;
    let len = u32::from_le_bytes([payload[16], payload[17], payload[18], payload[19]]) as usize;

    match handle {
        HANDLE_STATUS => {
            debug!("VIRTIO_NETD: read status");
            let text = alloc::format!(
                "state: {}\nlink: {}\nmac: {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}\nmtu: {}\n",
                if state.link_up { "up" } else { "down" },
                if state.link_up { "up" } else { "down" },
                state.mac[0],
                state.mac[1],
                state.mac[2],
                state.mac[3],
                state.mac[4],
                state.mac[5],
                state.mtu,
            );
            ProviderResponse::ok_read(text_slice(text.as_bytes(), offset, len))
        }
        HANDLE_MAC => {
            debug!("VIRTIO_NETD: read mac");
            let text = alloc::format!(
                "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}\n",
                state.mac[0],
                state.mac[1],
                state.mac[2],
                state.mac[3],
                state.mac[4],
                state.mac[5],
            );
            ProviderResponse::ok_read(text_slice(text.as_bytes(), offset, len))
        }
        HANDLE_MTU => {
            debug!("VIRTIO_NETD: read mtu");
            let text = alloc::format!("{}\n", state.mtu);
            ProviderResponse::ok_read(text_slice(text.as_bytes(), offset, len))
        }
        HANDLE_FEATURES => {
            debug!("VIRTIO_NETD: read features");
            let text = alloc::format!("0x{:08x}\n", state.features);
            ProviderResponse::ok_read(text_slice(text.as_bytes(), offset, len))
        }
        HANDLE_RX => {
            debug!("VIRTIO_NETD: read rx queued={}", state.rx_queue.len());
            // Return one length-prefixed frame, or empty if none available.
            if let Some(frame) = state.rx_queue.pop_front() {
                let frame_len = frame.len() as u32;
                let mut out = Vec::with_capacity(4 + frame.len());
                out.extend_from_slice(&frame_len.to_le_bytes());
                out.extend_from_slice(&frame);
                ProviderResponse::ok_read(&out)
            } else {
                ProviderResponse::ok_read(&[])
            }
        }
        HANDLE_EVENTS => {
            debug!("VIRTIO_NETD: read events queued={}", state.events_queue.len());
            // Return one newline-terminated event, or empty if none queued.
            if let Some(event) = state.events_queue.pop_front() {
                ProviderResponse::ok_read(&event)
            } else {
                ProviderResponse::ok_read(&[])
            }
        }
        HANDLE_CTL | HANDLE_TX => ProviderResponse::err(Errno::EINVAL), // write-only
        _ => ProviderResponse::err(Errno::ENOENT),
    }
}

/// Return a subslice of `text` starting at `offset`, capped at `max_len` bytes.
fn text_slice(text: &[u8], offset: usize, max_len: usize) -> &[u8] {
    let start = offset.min(text.len());
    let slice = &text[start..];
    &slice[..slice.len().min(max_len)]
}

// ── Write ─────────────────────────────────────────────────────────────────────

fn handle_write(
    state: &mut NetVfsState,
    driver: &mut VirtioNetDriver,
    payload: &[u8],
) -> ProviderResponse {
    if payload.len() < 20 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes([
        payload[0], payload[1], payload[2], payload[3], payload[4], payload[5], payload[6],
        payload[7],
    ]);
    // bytes 8..16: offset (ignored for these files)
    let data_len =
        u32::from_le_bytes([payload[16], payload[17], payload[18], payload[19]]) as usize;
    if payload.len() < 20 + data_len {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let data = &payload[20..20 + data_len];

    match handle {
        HANDLE_CTL => {
            debug!("VIRTIO_NETD: write ctl len={}", data_len);
            let cmd = core::str::from_utf8(data).unwrap_or("").trim();
            if cmd == "up" {
                state.link_up = true;
                state.push_event("link-up");
            } else if cmd == "down" {
                state.link_up = false;
                state.push_event("link-down");
            } else if let Some(rest) = cmd.strip_prefix("set-mtu ") {
                if let Ok(mtu) = rest.trim().parse::<u32>() {
                    state.mtu = mtu;
                } else {
                    return ProviderResponse::err(Errno::EINVAL);
                }
            }
            ProviderResponse::ok_written(data_len as u32)
        }
        HANDLE_MTU => {
            debug!("VIRTIO_NETD: write mtu len={}", data_len);
            let text = core::str::from_utf8(data).unwrap_or("").trim();
            if let Ok(mtu) = text.parse::<u32>() {
                state.mtu = mtu;
                ProviderResponse::ok_written(data_len as u32)
            } else {
                ProviderResponse::err(Errno::EINVAL)
            }
        }
        HANDLE_TX => {
            debug!("VIRTIO_NETD: write tx len={}", data_len);
            // Expect length-prefixed frame: [4 bytes: len][len bytes: frame data]
            if data.len() < 4 {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let frame_len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
            if data.len() < 4 + frame_len {
                return ProviderResponse::err(Errno::EINVAL);
            }
            let frame = &data[4..4 + frame_len];
            match driver.tx(frame) {
                Ok(()) => ProviderResponse::ok_written(data_len as u32),
                Err(_) => ProviderResponse::err(Errno::EIO),
            }
        }
        HANDLE_STATUS | HANDLE_MAC | HANDLE_FEATURES | HANDLE_RX | HANDLE_EVENTS => {
            ProviderResponse::err(Errno::EROFS)
        }
        _ => ProviderResponse::err(Errno::ENOENT),
    }
}

// ── Poll ──────────────────────────────────────────────────────────────────────

fn handle_poll(state: &NetVfsState, payload: &[u8]) -> ProviderResponse {
    if payload.len() < 12 {
        return ProviderResponse::err(Errno::EINVAL);
    }
    let handle = u64::from_le_bytes([
        payload[0], payload[1], payload[2], payload[3], payload[4], payload[5], payload[6],
        payload[7],
    ]);
    // bytes 8..12: requested events (ignored — we check readiness unconditionally)

    let revents: u32 = match handle {
        HANDLE_RX => {
            if !state.rx_queue.is_empty() {
                POLLIN
            } else {
                0
            }
        }
        HANDLE_EVENTS => {
            if !state.events_queue.is_empty() {
                POLLIN
            } else {
                0
            }
        }
        // These text files are always readable.
        HANDLE_STATUS | HANDLE_MAC | HANDLE_MTU | HANDLE_FEATURES => POLLIN,
        // Write-only files are never readable.
        HANDLE_CTL | HANDLE_TX => 0,
        _ => 0,
    };

    ProviderResponse::ok_poll(revents)
}
