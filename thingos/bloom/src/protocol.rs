#![allow(dead_code)]

use alloc::vec::Vec;

use abi::KindId;
use abi::display_protocol::Rect;
use abi::pixel::PixelFormat;

pub const BLOOM_PROTOCOL_MAGIC: u32 = 0x424C_4F4F; // "BLOO"
pub const BLOOM_PROTOCOL_VERSION: u16 = 1;

pub const MSG_CONNECT: u16 = 1;
pub const MSG_CREATE_SURFACE: u16 = 2;
pub const MSG_DESTROY_SURFACE: u16 = 3;
pub const MSG_ATTACH_BUFFER: u16 = 4;
pub const MSG_DAMAGE: u16 = 5;
pub const MSG_SET_INPUT_REGION: u16 = 6;
pub const MSG_SET_OPAQUE_REGION: u16 = 7;
pub const MSG_SET_DEST_RECT: u16 = 8;
pub const MSG_SET_Z_ORDER: u16 = 9;
pub const MSG_COMMIT: u16 = 10;
pub const MSG_CONNECT_INBOX: u16 = 11;

pub const EVT_ACK: u16 = 0x8001;
pub const EVT_FRAME_DONE: u16 = 0x8002;
pub const EVT_POINTER_ENTER: u16 = 0x8101;
pub const EVT_POINTER_LEAVE: u16 = 0x8102;
pub const EVT_POINTER_MOTION: u16 = 0x8103;
pub const EVT_POINTER_BUTTON: u16 = 0x8104;
pub const EVT_KEYBOARD_ENTER: u16 = 0x8201;
pub const EVT_KEYBOARD_LEAVE: u16 = 0x8202;
pub const EVT_KEYBOARD_KEY: u16 = 0x8203;

pub const KIND_POINTER_ENTER: KindId =
    KindId([b'B', b'L', b'O', b'O', b'M', b'I', b'N', b'P', b'U', b'T', 0, 0, 0, 0, 0, 1]);
pub const KIND_POINTER_LEAVE: KindId =
    KindId([b'B', b'L', b'O', b'O', b'M', b'I', b'N', b'P', b'U', b'T', 0, 0, 0, 0, 0, 2]);
pub const KIND_POINTER_MOTION: KindId =
    KindId([b'B', b'L', b'O', b'O', b'M', b'I', b'N', b'P', b'U', b'T', 0, 0, 0, 0, 0, 3]);
pub const KIND_POINTER_BUTTON: KindId =
    KindId([b'B', b'L', b'O', b'O', b'M', b'I', b'N', b'P', b'U', b'T', 0, 0, 0, 0, 0, 4]);
pub const KIND_KEYBOARD_ENTER: KindId =
    KindId([b'B', b'L', b'O', b'O', b'M', b'I', b'N', b'P', b'U', b'T', 0, 0, 0, 0, 0, 5]);
pub const KIND_KEYBOARD_LEAVE: KindId =
    KindId([b'B', b'L', b'O', b'O', b'M', b'I', b'N', b'P', b'U', b'T', 0, 0, 0, 0, 0, 6]);
pub const KIND_KEYBOARD_KEY: KindId =
    KindId([b'B', b'L', b'O', b'O', b'M', b'I', b'N', b'P', b'U', b'T', 0, 0, 0, 0, 0, 7]);

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct MessageHeader {
    pub magic: u32,
    pub version: u16,
    pub msg_type: u16,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct ConnectRequest {
    pub header: MessageHeader,
    pub reply_port: u32,
    pub event_port: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
/// Extended connect request that enables inbox-based input delivery.
///
/// `input_pid` is the target process ID whose inbox will receive typed pointer
/// and keyboard events from Bloom.
pub struct ConnectInboxRequest {
    pub header: MessageHeader,
    pub reply_port: u32,
    pub event_port: u32,
    pub input_pid: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct CreateSurfaceRequest {
    pub header: MessageHeader,
    pub reply_port: u32,
    pub client_id: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct DestroySurfaceRequest {
    pub header: MessageHeader,
    pub reply_port: u32,
    pub client_id: u32,
    pub surface_id: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct AttachBufferRequest {
    pub header: MessageHeader,
    pub reply_port: u32,
    pub client_id: u32,
    pub surface_id: u32,
    pub handle_thing: u32,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: PixelFormat,
    pub modifier: u64,
    /// Monotonic generation counter for this logical buffer.
    ///
    /// Must be incremented by the client each time new pixel content is written
    /// to the backing storage before re-submitting the same handle.  Bloom
    /// forwards this to the resource cache so stale imports are evicted.
    pub generation: u64,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct RectRequest {
    pub header: MessageHeader,
    pub reply_port: u32,
    pub client_id: u32,
    pub surface_id: u32,
    pub rect: Rect,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct SetZOrderRequest {
    pub header: MessageHeader,
    pub reply_port: u32,
    pub client_id: u32,
    pub surface_id: u32,
    pub z_order: i32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct CommitRequest {
    pub header: MessageHeader,
    pub reply_port: u32,
    pub client_id: u32,
    pub surface_id: u32,
}

#[derive(Clone, Copy, Debug)]
pub enum ClientRequest {
    Connect(ConnectRequest),
    ConnectInbox(ConnectInboxRequest),
    CreateSurface(CreateSurfaceRequest),
    DestroySurface(DestroySurfaceRequest),
    AttachBuffer(AttachBufferRequest),
    Damage(RectRequest),
    SetInputRegion(RectRequest),
    SetOpaqueRegion(RectRequest),
    SetDestRect(RectRequest),
    SetZOrder(SetZOrderRequest),
    Commit(CommitRequest),
}

pub fn parse_request(raw: &[u8]) -> Option<ClientRequest> {
    if raw.len() < core::mem::size_of::<MessageHeader>() {
        return None;
    }

    let header = read_packed::<MessageHeader>(raw)?;
    if header.magic != BLOOM_PROTOCOL_MAGIC || header.version != BLOOM_PROTOCOL_VERSION {
        return None;
    }

    match header.msg_type {
        MSG_CONNECT if raw.len() >= core::mem::size_of::<ConnectRequest>() => {
            read_packed::<ConnectRequest>(raw).map(ClientRequest::Connect)
        }
        MSG_CONNECT_INBOX if raw.len() >= core::mem::size_of::<ConnectInboxRequest>() => {
            read_packed::<ConnectInboxRequest>(raw).map(ClientRequest::ConnectInbox)
        }
        MSG_CREATE_SURFACE if raw.len() >= core::mem::size_of::<CreateSurfaceRequest>() => {
            read_packed::<CreateSurfaceRequest>(raw).map(ClientRequest::CreateSurface)
        }
        MSG_DESTROY_SURFACE if raw.len() >= core::mem::size_of::<DestroySurfaceRequest>() => {
            read_packed::<DestroySurfaceRequest>(raw).map(ClientRequest::DestroySurface)
        }
        MSG_ATTACH_BUFFER if raw.len() >= core::mem::size_of::<AttachBufferRequest>() => {
            read_packed::<AttachBufferRequest>(raw).map(ClientRequest::AttachBuffer)
        }
        MSG_DAMAGE if raw.len() >= core::mem::size_of::<RectRequest>() => {
            read_packed::<RectRequest>(raw).map(ClientRequest::Damage)
        }
        MSG_SET_INPUT_REGION if raw.len() >= core::mem::size_of::<RectRequest>() => {
            read_packed::<RectRequest>(raw).map(ClientRequest::SetInputRegion)
        }
        MSG_SET_OPAQUE_REGION if raw.len() >= core::mem::size_of::<RectRequest>() => {
            read_packed::<RectRequest>(raw).map(ClientRequest::SetOpaqueRegion)
        }
        MSG_SET_DEST_RECT if raw.len() >= core::mem::size_of::<RectRequest>() => {
            read_packed::<RectRequest>(raw).map(ClientRequest::SetDestRect)
        }
        MSG_SET_Z_ORDER if raw.len() >= core::mem::size_of::<SetZOrderRequest>() => {
            read_packed::<SetZOrderRequest>(raw).map(ClientRequest::SetZOrder)
        }
        MSG_COMMIT if raw.len() >= core::mem::size_of::<CommitRequest>() => {
            read_packed::<CommitRequest>(raw).map(ClientRequest::Commit)
        }
        _ => None,
    }
}

fn read_packed<T: Copy>(raw: &[u8]) -> Option<T> {
    if raw.len() < core::mem::size_of::<T>() {
        return None;
    }
    Some(unsafe { core::ptr::read_unaligned(raw.as_ptr() as *const T) })
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct AckEvent {
    pub header: MessageHeader,
    pub status: u32,
    pub value: u32,
    pub serial: u64,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct FrameDoneEvent {
    pub header: MessageHeader,
    pub surface_id: u32,
    pub serial: u64,
    pub timestamp_ns: u64,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct PointerEnterEvent {
    pub header: MessageHeader,
    pub surface_id: u32,
    pub x: i32,
    pub y: i32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct PointerLeaveEvent {
    pub header: MessageHeader,
    pub surface_id: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct PointerMotionEvent {
    pub header: MessageHeader,
    pub surface_id: u32,
    pub x: i32,
    pub y: i32,
    pub timestamp_ns: u64,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct PointerButtonEvent {
    pub header: MessageHeader,
    pub surface_id: u32,
    pub button: u8,
    pub pressed: u8,
    pub _pad: [u8; 2],
    pub timestamp_ns: u64,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct KeyboardEnterEvent {
    pub header: MessageHeader,
    pub surface_id: u32,
    pub modifiers: u8,
    pub _pad: [u8; 3],
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct KeyboardLeaveEvent {
    pub header: MessageHeader,
    pub surface_id: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct KeyboardKeyEvent {
    pub header: MessageHeader,
    pub surface_id: u32,
    pub key: u16,
    pub pressed: u8,
    pub modifiers: u8,
    pub repeat: u8,
    pub _pad: [u8; 3],
    pub timestamp_ns: u64,
}

pub fn msg_header(msg_type: u16) -> MessageHeader {
    MessageHeader { magic: BLOOM_PROTOCOL_MAGIC, version: BLOOM_PROTOCOL_VERSION, msg_type }
}

pub fn as_bytes<T>(value: &T) -> &[u8] {
    unsafe {
        core::slice::from_raw_parts(value as *const T as *const u8, core::mem::size_of::<T>())
    }
}

pub fn to_vec<T>(value: &T) -> Vec<u8> {
    as_bytes(value).to_vec()
}
