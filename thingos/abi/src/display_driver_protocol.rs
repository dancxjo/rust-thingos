#![allow(dead_code)]

// Wire format is explicitly little-endian for all fields.

use core::mem::size_of;

use crate::KindId;

pub const DRIVER_MAGIC: u32 = 0x4452_5650; // "DRVP"
pub const DRIVER_VERSION: u16 = 0;

pub const MSG_REGISTER: u16 = 1;
pub const MSG_BIND: u16 = 2;
pub const MSG_PRESENT: u16 = 3;
pub const MSG_ACK: u16 = 4;
pub const MSG_ERR: u16 = 5;
pub const MSG_HELLO: u16 = 6;
pub const MSG_WELCOME: u16 = 7;
pub const MSG_CAPS: u16 = 8;
pub const MSG_OFFER_FRAMEBUFFER: u16 = 9;
pub const MSG_ACCEPT_FRAMEBUFFER: u16 = 10;
pub const MSG_ACQUIRE: u16 = 11;
pub const MSG_ACQUIRED: u16 = 12;
pub const MSG_SUBMIT_3D: u16 = 13; // Virgl 3D command submission
pub const MSG_CREATE_TEXTURE_3D: u16 = 14; // Create GPU texture
pub const MSG_UPLOAD_TEXTURE_3D: u16 = 15; // Upload pixel data to texture
pub const MSG_DESTROY_TEXTURE_3D: u16 = 16; // Destroy GPU texture
pub const MSG_TEXTURE_CREATED: u16 = 17; // Response with texture resource ID

pub const PROTO_MAJOR: u16 = 1;
pub const PROTO_MINOR: u16 = 0;

pub const CAP_DIRTY_RECTS: u32 = 1 << 0;
pub const CAP_FULLFRAME: u32 = 1 << 1;
pub const CAP_MULTI_DISPLAY: u32 = 1 << 2;
pub const CAP_FENCE: u32 = 1 << 3;
pub const CAP_3D: u32 = 1 << 4; // Virgl 3D support (driver has submit_3d capability)

pub const PRESENT_FLAG_FULLFRAME: u32 = 1 << 0;

pub const DRIVER_KIND_BOOTFB: u32 = 1;
pub const DRIVER_KIND_VIRTIO_GPU: u32 = 2;
pub const DRIVER_KIND_RAMFB: u32 = 3;

/// Payload serialized and served by the `/dev/fb0` VFS node.
#[repr(C, align(8))]
#[derive(Clone, Copy, Debug)]
pub struct FbInfoPayload {
    pub device_handle: u64, // Device handle to pass to sys_device_claim
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub bpp: u32,
    pub format: u32,
    pub _reserved: u32,
}

pub const FB_INFO_PAYLOAD_SIZE: usize = 32;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DriverHeader {
    pub magic: u32,
    pub version: u16,
    pub msg_type: u16,
    pub payload_len: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RegisterPayload {
    pub driver_kind: u32,
    pub caps: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct HelloPayload {
    pub proto_major: u16,
    pub proto_minor: u16,
    pub want_caps: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WelcomePayload {
    pub proto_major: u16,
    pub proto_minor: u16,
    pub have_caps: u32,
    pub max_rects: u16,
    pub reserved: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BindPayload {
    pub fb_fd: u32,
    pub _pad: u32,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PresentHeader {
    pub rect_count: u32,
    pub _pad: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ErrResp {
    pub code: u32,
}

/// Driver offers a pre-allocated framebuffer bytespace for zero-copy rendering.
/// Sent by driver to compositor after MSG_WELCOME.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct OfferFramebufferPayload {
    pub handle: u32,
    pub _pad: u32,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
}

/// Compositor response to OfferFramebufferPayload.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AcceptFramebufferPayload {
    /// 0 = rejected, 1 = accepted
    pub accepted: u32,
    pub _pad: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AcquiredPayload {
    pub handle: u32,
    pub _pad1: u32,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
    pub buffer_age: u32,
    pub _pad2: u32,
}

/// Header for 3D command submission (MSG_SUBMIT_3D).
/// The actual virgl command buffer follows this header.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Submit3dHeader {
    /// Virgl context ID
    pub ctx_id: u32,
    /// Length of command buffer in bytes (follows this header)
    pub cmd_len: u32,
}

/// Header for creating a GPU texture (MSG_CREATE_TEXTURE_3D).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CreateTexture3dHeader {
    /// Client-specified texture ID (for tracking)
    pub client_id: u64,
    /// Texture width in pixels
    pub width: u32,
    /// Texture height in pixels
    pub height: u32,
    /// Pixel format (e.g. BGRA8888 = 2)
    pub format: u32,
    /// Padding for alignment
    pub _pad: u32,
}

/// Header for uploading texture data (MSG_UPLOAD_TEXTURE_3D).
/// The pixel data follows this header.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct UploadTexture3dHeader {
    /// Texture resource ID (from driver)
    pub resource_id: u32,
    /// Width of upload region
    pub width: u32,
    /// Height of upload region
    pub height: u32,
    /// Stride of pixel data in bytes
    pub stride: u32,
    /// X offset in texture
    pub x: u32,
    /// Y offset in texture
    pub y: u32,
    /// Length of pixel data in bytes (follows header)
    pub data_len: u32,
    /// Padding for alignment
    pub _pad: u32,
}

/// Response for texture creation (MSG_TEXTURE_CREATED).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TextureCreatedResponse {
    /// Client-specified texture ID (echoed back)
    pub client_id: u64,
    /// Driver-allocated resource ID
    pub resource_id: u32,
    /// Status (0 = success)
    pub status: u32,
}

pub const HEADER_SIZE: usize = size_of::<DriverHeader>();
pub const REGISTER_PAYLOAD_WIRE_SIZE: usize = 8;
pub const HELLO_PAYLOAD_WIRE_SIZE: usize = 8;
pub const WELCOME_PAYLOAD_WIRE_SIZE: usize = 12;
pub const BIND_PAYLOAD_WIRE_SIZE: usize = 24;
pub const RECT_WIRE_SIZE: usize = 16;
pub const PRESENT_HEADER_WIRE_SIZE: usize = 8;
pub const ERR_RESP_WIRE_SIZE: usize = 4;
pub const OFFER_FRAMEBUFFER_PAYLOAD_WIRE_SIZE: usize = 24; // 8 + 4 + 4 + 4 + 4
pub const ACCEPT_FRAMEBUFFER_PAYLOAD_WIRE_SIZE: usize = 8; // 4 + 4
pub const ACQUIRED_PAYLOAD_WIRE_SIZE: usize = 32; // 8 + 4 + 4 + 4 + 4 + 4 + 4
pub const SUBMIT_3D_HEADER_WIRE_SIZE: usize = 8; // 4 + 4 (ctx_id + cmd_len)
pub const CREATE_TEXTURE_3D_HEADER_WIRE_SIZE: usize = 24; // 8 + 4 + 4 + 4 + 4
pub const UPLOAD_TEXTURE_3D_HEADER_WIRE_SIZE: usize = 32; // 4 + 4 + 4 + 4 + 4 + 4 + 4 + 4
pub const TEXTURE_CREATED_RESPONSE_WIRE_SIZE: usize = 16; // 8 + 4 + 4

pub fn encode_message(buf: &mut [u8], msg_type: u16, payload: &[u8]) -> Option<usize> {
    let total = HEADER_SIZE + payload.len();
    if buf.len() < total {
        return None;
    }

    let header = DriverHeader {
        magic: DRIVER_MAGIC,
        version: DRIVER_VERSION,
        msg_type,
        payload_len: payload.len() as u32,
    };

    buf[0..4].copy_from_slice(&header.magic.to_le_bytes());
    buf[4..6].copy_from_slice(&header.version.to_le_bytes());
    buf[6..8].copy_from_slice(&header.msg_type.to_le_bytes());
    buf[8..12].copy_from_slice(&header.payload_len.to_le_bytes());

    if !payload.is_empty() {
        buf[HEADER_SIZE..total].copy_from_slice(payload);
    }

    Some(total)
}

pub fn parse_message(buf: &[u8]) -> Option<(DriverHeader, &[u8])> {
    if buf.len() < HEADER_SIZE {
        return None;
    }

    let magic = u32::from_le_bytes(buf[0..4].try_into().ok()?);
    let version = u16::from_le_bytes(buf[4..6].try_into().ok()?);
    let msg_type = u16::from_le_bytes(buf[6..8].try_into().ok()?);
    let payload_len = u32::from_le_bytes(buf[8..12].try_into().ok()?);

    if magic != DRIVER_MAGIC || version != DRIVER_VERSION {
        return None;
    }

    let total = HEADER_SIZE + payload_len as usize;
    if buf.len() < total {
        return None;
    }

    let header = DriverHeader { magic, version, msg_type, payload_len };

    Some((header, &buf[HEADER_SIZE..total]))
}

pub fn message_total_len(buf: &[u8]) -> Option<usize> {
    if buf.len() < HEADER_SIZE {
        return None;
    }

    let magic = u32::from_le_bytes(buf[0..4].try_into().ok()?);
    let version = u16::from_le_bytes(buf[4..6].try_into().ok()?);
    let payload_len = u32::from_le_bytes(buf[8..12].try_into().ok()?);

    if magic != DRIVER_MAGIC || version != DRIVER_VERSION {
        return None;
    }

    Some(HEADER_SIZE + payload_len as usize)
}

pub fn encode_register_payload_le(payload: &RegisterPayload, out: &mut [u8]) -> Option<usize> {
    if out.len() < REGISTER_PAYLOAD_WIRE_SIZE {
        return None;
    }
    out[0..4].copy_from_slice(&payload.driver_kind.to_le_bytes());
    out[4..8].copy_from_slice(&payload.caps.to_le_bytes());
    Some(REGISTER_PAYLOAD_WIRE_SIZE)
}

pub fn decode_register_payload_le(buf: &[u8]) -> Option<RegisterPayload> {
    if buf.len() < REGISTER_PAYLOAD_WIRE_SIZE {
        return None;
    }
    Some(RegisterPayload {
        driver_kind: u32::from_le_bytes(buf[0..4].try_into().ok()?),
        caps: u32::from_le_bytes(buf[4..8].try_into().ok()?),
    })
}

pub fn encode_hello_payload_le(payload: &HelloPayload, out: &mut [u8]) -> Option<usize> {
    if out.len() < HELLO_PAYLOAD_WIRE_SIZE {
        return None;
    }
    out[0..2].copy_from_slice(&payload.proto_major.to_le_bytes());
    out[2..4].copy_from_slice(&payload.proto_minor.to_le_bytes());
    out[4..8].copy_from_slice(&payload.want_caps.to_le_bytes());
    Some(HELLO_PAYLOAD_WIRE_SIZE)
}

pub fn decode_hello_payload_le(buf: &[u8]) -> Option<HelloPayload> {
    if buf.len() < HELLO_PAYLOAD_WIRE_SIZE {
        return None;
    }
    Some(HelloPayload {
        proto_major: u16::from_le_bytes(buf[0..2].try_into().ok()?),
        proto_minor: u16::from_le_bytes(buf[2..4].try_into().ok()?),
        want_caps: u32::from_le_bytes(buf[4..8].try_into().ok()?),
    })
}

pub fn encode_welcome_payload_le(payload: &WelcomePayload, out: &mut [u8]) -> Option<usize> {
    if out.len() < WELCOME_PAYLOAD_WIRE_SIZE {
        return None;
    }
    out[0..2].copy_from_slice(&payload.proto_major.to_le_bytes());
    out[2..4].copy_from_slice(&payload.proto_minor.to_le_bytes());
    out[4..8].copy_from_slice(&payload.have_caps.to_le_bytes());
    out[8..10].copy_from_slice(&payload.max_rects.to_le_bytes());
    out[10..12].copy_from_slice(&payload.reserved.to_le_bytes());
    Some(WELCOME_PAYLOAD_WIRE_SIZE)
}

pub fn decode_welcome_payload_le(buf: &[u8]) -> Option<WelcomePayload> {
    if buf.len() < WELCOME_PAYLOAD_WIRE_SIZE {
        return None;
    }
    Some(WelcomePayload {
        proto_major: u16::from_le_bytes(buf[0..2].try_into().ok()?),
        proto_minor: u16::from_le_bytes(buf[2..4].try_into().ok()?),
        have_caps: u32::from_le_bytes(buf[4..8].try_into().ok()?),
        max_rects: u16::from_le_bytes(buf[8..10].try_into().ok()?),
        reserved: u16::from_le_bytes(buf[10..12].try_into().ok()?),
    })
}

pub fn encode_bind_payload_le(payload: &BindPayload, out: &mut [u8]) -> Option<usize> {
    if out.len() < BIND_PAYLOAD_WIRE_SIZE {
        return None;
    }
    out[0..4].copy_from_slice(&payload.fb_fd.to_le_bytes());
    out[4..8].copy_from_slice(&payload._pad.to_le_bytes());
    out[8..12].copy_from_slice(&payload.width.to_le_bytes());
    out[12..16].copy_from_slice(&payload.height.to_le_bytes());
    out[16..20].copy_from_slice(&payload.stride.to_le_bytes());
    out[20..24].copy_from_slice(&payload.format.to_le_bytes());
    Some(BIND_PAYLOAD_WIRE_SIZE)
}

pub fn decode_bind_payload_le(buf: &[u8]) -> Option<BindPayload> {
    if buf.len() < BIND_PAYLOAD_WIRE_SIZE {
        return None;
    }
    Some(BindPayload {
        fb_fd: u32::from_le_bytes(buf[0..4].try_into().ok()?),
        _pad: u32::from_le_bytes(buf[4..8].try_into().ok()?),
        width: u32::from_le_bytes(buf[8..12].try_into().ok()?),
        height: u32::from_le_bytes(buf[12..16].try_into().ok()?),
        stride: u32::from_le_bytes(buf[16..20].try_into().ok()?),
        format: u32::from_le_bytes(buf[20..24].try_into().ok()?),
    })
}

pub fn encode_rect_le(rect: &Rect, out: &mut [u8]) -> Option<usize> {
    if out.len() < RECT_WIRE_SIZE {
        return None;
    }
    out[0..4].copy_from_slice(&rect.x.to_le_bytes());
    out[4..8].copy_from_slice(&rect.y.to_le_bytes());
    out[8..12].copy_from_slice(&rect.w.to_le_bytes());
    out[12..16].copy_from_slice(&rect.h.to_le_bytes());
    Some(RECT_WIRE_SIZE)
}

pub fn decode_rect_le(buf: &[u8]) -> Option<Rect> {
    if buf.len() < RECT_WIRE_SIZE {
        return None;
    }
    Some(Rect {
        x: u32::from_le_bytes(buf[0..4].try_into().ok()?),
        y: u32::from_le_bytes(buf[4..8].try_into().ok()?),
        w: u32::from_le_bytes(buf[8..12].try_into().ok()?),
        h: u32::from_le_bytes(buf[12..16].try_into().ok()?),
    })
}

pub fn encode_present_header_le(rect_count: u32, out: &mut [u8]) -> Option<usize> {
    encode_present_header_with_flags_le(rect_count, 0, out)
}

pub fn encode_present_header_with_flags_le(
    rect_count: u32,
    flags: u32,
    out: &mut [u8],
) -> Option<usize> {
    if out.len() < PRESENT_HEADER_WIRE_SIZE {
        return None;
    }
    out[0..4].copy_from_slice(&rect_count.to_le_bytes());
    out[4..8].copy_from_slice(&flags.to_le_bytes());
    Some(PRESENT_HEADER_WIRE_SIZE)
}

pub fn decode_present_header_le(buf: &[u8]) -> Option<PresentHeader> {
    if buf.len() < PRESENT_HEADER_WIRE_SIZE {
        return None;
    }
    Some(PresentHeader {
        rect_count: u32::from_le_bytes(buf[0..4].try_into().ok()?),
        _pad: u32::from_le_bytes(buf[4..8].try_into().ok()?),
    })
}

pub fn encode_present_payload_le<I>(rect_count: u32, rects: I, out: &mut [u8]) -> Option<usize>
where
    I: IntoIterator<Item = Rect>,
{
    encode_present_payload_with_flags_le(rect_count, 0, rects, out)
}

pub fn encode_present_payload_with_flags_le<I>(
    rect_count: u32,
    flags: u32,
    rects: I,
    out: &mut [u8],
) -> Option<usize>
where
    I: IntoIterator<Item = Rect>,
{
    let required = PRESENT_HEADER_WIRE_SIZE + rect_count as usize * RECT_WIRE_SIZE;
    if out.len() < required {
        return None;
    }

    encode_present_header_with_flags_le(rect_count, flags, out)?;

    let mut written = 0usize;
    let mut offset = PRESENT_HEADER_WIRE_SIZE;
    for rect in rects {
        if written >= rect_count as usize {
            break;
        }
        encode_rect_le(&rect, &mut out[offset..offset + RECT_WIRE_SIZE])?;
        offset += RECT_WIRE_SIZE;
        written += 1;
    }

    if written != rect_count as usize {
        return None;
    }

    Some(required)
}

pub fn encode_err_resp_le(payload: &ErrResp, out: &mut [u8]) -> Option<usize> {
    if out.len() < ERR_RESP_WIRE_SIZE {
        return None;
    }
    out[0..4].copy_from_slice(&payload.code.to_le_bytes());
    Some(ERR_RESP_WIRE_SIZE)
}

pub fn decode_err_resp_le(buf: &[u8]) -> Option<ErrResp> {
    if buf.len() < ERR_RESP_WIRE_SIZE {
        return None;
    }
    Some(ErrResp { code: u32::from_le_bytes(buf[0..4].try_into().ok()?) })
}

pub fn encode_offer_framebuffer_payload_le(
    payload: &OfferFramebufferPayload,
    out: &mut [u8],
) -> Option<usize> {
    if out.len() < OFFER_FRAMEBUFFER_PAYLOAD_WIRE_SIZE {
        return None;
    }
    out[0..4].copy_from_slice(&payload.handle.to_le_bytes());
    out[4..8].copy_from_slice(&payload._pad.to_le_bytes());
    out[8..12].copy_from_slice(&payload.width.to_le_bytes());
    out[12..16].copy_from_slice(&payload.height.to_le_bytes());
    out[16..20].copy_from_slice(&payload.stride.to_le_bytes());
    out[20..24].copy_from_slice(&payload.format.to_le_bytes());
    Some(OFFER_FRAMEBUFFER_PAYLOAD_WIRE_SIZE)
}

pub fn decode_offer_framebuffer_payload_le(buf: &[u8]) -> Option<OfferFramebufferPayload> {
    if buf.len() < OFFER_FRAMEBUFFER_PAYLOAD_WIRE_SIZE {
        return None;
    }
    Some(OfferFramebufferPayload {
        handle: u32::from_le_bytes(buf[0..4].try_into().ok()?),
        _pad: u32::from_le_bytes(buf[4..8].try_into().ok()?),
        width: u32::from_le_bytes(buf[8..12].try_into().ok()?),
        height: u32::from_le_bytes(buf[12..16].try_into().ok()?),
        stride: u32::from_le_bytes(buf[16..20].try_into().ok()?),
        format: u32::from_le_bytes(buf[20..24].try_into().ok()?),
    })
}

pub fn encode_accept_framebuffer_payload_le(
    payload: &AcceptFramebufferPayload,
    out: &mut [u8],
) -> Option<usize> {
    if out.len() < ACCEPT_FRAMEBUFFER_PAYLOAD_WIRE_SIZE {
        return None;
    }
    out[0..4].copy_from_slice(&payload.accepted.to_le_bytes());
    out[4..8].copy_from_slice(&payload._pad.to_le_bytes());
    Some(ACCEPT_FRAMEBUFFER_PAYLOAD_WIRE_SIZE)
}

pub fn decode_accept_framebuffer_payload_le(buf: &[u8]) -> Option<AcceptFramebufferPayload> {
    if buf.len() < ACCEPT_FRAMEBUFFER_PAYLOAD_WIRE_SIZE {
        return None;
    }
    Some(AcceptFramebufferPayload {
        accepted: u32::from_le_bytes(buf[0..4].try_into().ok()?),
        _pad: u32::from_le_bytes(buf[4..8].try_into().ok()?),
    })
}

pub fn encode_acquired_payload_le(payload: &AcquiredPayload, out: &mut [u8]) -> Option<usize> {
    if out.len() < ACQUIRED_PAYLOAD_WIRE_SIZE {
        return None;
    }
    out[0..4].copy_from_slice(&payload.handle.to_le_bytes());
    out[4..8].copy_from_slice(&payload._pad1.to_le_bytes());
    out[8..12].copy_from_slice(&payload.width.to_le_bytes());
    out[12..16].copy_from_slice(&payload.height.to_le_bytes());
    out[16..20].copy_from_slice(&payload.stride.to_le_bytes());
    out[20..24].copy_from_slice(&payload.format.to_le_bytes());
    out[24..28].copy_from_slice(&payload.buffer_age.to_le_bytes());
    out[28..32].copy_from_slice(&payload._pad2.to_le_bytes());
    Some(ACQUIRED_PAYLOAD_WIRE_SIZE)
}

pub fn decode_acquired_payload_le(buf: &[u8]) -> Option<AcquiredPayload> {
    if buf.len() < ACQUIRED_PAYLOAD_WIRE_SIZE {
        return None;
    }
    Some(AcquiredPayload {
        handle: u32::from_le_bytes(buf[0..4].try_into().ok()?),
        _pad1: u32::from_le_bytes(buf[4..8].try_into().ok()?),
        width: u32::from_le_bytes(buf[8..12].try_into().ok()?),
        height: u32::from_le_bytes(buf[12..16].try_into().ok()?),
        stride: u32::from_le_bytes(buf[16..20].try_into().ok()?),
        format: u32::from_le_bytes(buf[20..24].try_into().ok()?),
        buffer_age: u32::from_le_bytes(buf[24..28].try_into().ok()?),
        _pad2: u32::from_le_bytes(buf[28..32].try_into().ok()?),
    })
}

/// Encode Submit3dHeader for virgl 3D command submission.
/// The virgl command buffer should be appended after this header.
pub fn encode_submit_3d_header_le(header: &Submit3dHeader, out: &mut [u8]) -> Option<usize> {
    if out.len() < SUBMIT_3D_HEADER_WIRE_SIZE {
        return None;
    }
    out[0..4].copy_from_slice(&header.ctx_id.to_le_bytes());
    out[4..8].copy_from_slice(&header.cmd_len.to_le_bytes());
    Some(SUBMIT_3D_HEADER_WIRE_SIZE)
}

pub fn decode_submit_3d_header_le(buf: &[u8]) -> Option<Submit3dHeader> {
    if buf.len() < SUBMIT_3D_HEADER_WIRE_SIZE {
        return None;
    }
    Some(Submit3dHeader {
        ctx_id: u32::from_le_bytes(buf[0..4].try_into().ok()?),
        cmd_len: u32::from_le_bytes(buf[4..8].try_into().ok()?),
    })
}

/// Encode CreateTexture3dHeader for texture creation.
pub fn encode_create_texture_3d_header_le(
    header: &CreateTexture3dHeader,
    out: &mut [u8],
) -> Option<usize> {
    if out.len() < CREATE_TEXTURE_3D_HEADER_WIRE_SIZE {
        return None;
    }
    out[0..8].copy_from_slice(&header.client_id.to_le_bytes());
    out[8..12].copy_from_slice(&header.width.to_le_bytes());
    out[12..16].copy_from_slice(&header.height.to_le_bytes());
    out[16..20].copy_from_slice(&header.format.to_le_bytes());
    out[20..24].copy_from_slice(&0u32.to_le_bytes());
    Some(CREATE_TEXTURE_3D_HEADER_WIRE_SIZE)
}

pub fn decode_create_texture_3d_header_le(buf: &[u8]) -> Option<CreateTexture3dHeader> {
    if buf.len() < CREATE_TEXTURE_3D_HEADER_WIRE_SIZE {
        return None;
    }
    Some(CreateTexture3dHeader {
        client_id: u64::from_le_bytes(buf[0..8].try_into().ok()?),
        width: u32::from_le_bytes(buf[8..12].try_into().ok()?),
        height: u32::from_le_bytes(buf[12..16].try_into().ok()?),
        format: u32::from_le_bytes(buf[16..20].try_into().ok()?),
        _pad: 0,
    })
}

/// Encode UploadTexture3dHeader for texture data upload.
pub fn encode_upload_texture_3d_header_le(
    header: &UploadTexture3dHeader,
    out: &mut [u8],
) -> Option<usize> {
    if out.len() < UPLOAD_TEXTURE_3D_HEADER_WIRE_SIZE {
        return None;
    }
    out[0..4].copy_from_slice(&header.resource_id.to_le_bytes());
    out[4..8].copy_from_slice(&header.width.to_le_bytes());
    out[8..12].copy_from_slice(&header.height.to_le_bytes());
    out[12..16].copy_from_slice(&header.stride.to_le_bytes());
    out[16..20].copy_from_slice(&header.x.to_le_bytes());
    out[20..24].copy_from_slice(&header.y.to_le_bytes());
    out[24..28].copy_from_slice(&header.data_len.to_le_bytes());
    out[28..32].copy_from_slice(&0u32.to_le_bytes());
    Some(UPLOAD_TEXTURE_3D_HEADER_WIRE_SIZE)
}

pub fn decode_upload_texture_3d_header_le(buf: &[u8]) -> Option<UploadTexture3dHeader> {
    if buf.len() < UPLOAD_TEXTURE_3D_HEADER_WIRE_SIZE {
        return None;
    }
    Some(UploadTexture3dHeader {
        resource_id: u32::from_le_bytes(buf[0..4].try_into().ok()?),
        width: u32::from_le_bytes(buf[4..8].try_into().ok()?),
        height: u32::from_le_bytes(buf[8..12].try_into().ok()?),
        stride: u32::from_le_bytes(buf[12..16].try_into().ok()?),
        x: u32::from_le_bytes(buf[16..20].try_into().ok()?),
        y: u32::from_le_bytes(buf[20..24].try_into().ok()?),
        data_len: u32::from_le_bytes(buf[24..28].try_into().ok()?),
        _pad: 0,
    })
}

/// Encode TextureCreatedResponse.
pub fn encode_texture_created_response_le(
    response: &TextureCreatedResponse,
    out: &mut [u8],
) -> Option<usize> {
    if out.len() < TEXTURE_CREATED_RESPONSE_WIRE_SIZE {
        return None;
    }
    out[0..8].copy_from_slice(&response.client_id.to_le_bytes());
    out[8..12].copy_from_slice(&response.resource_id.to_le_bytes());
    out[12..16].copy_from_slice(&response.status.to_le_bytes());
    Some(TEXTURE_CREATED_RESPONSE_WIRE_SIZE)
}

pub fn decode_texture_created_response_le(buf: &[u8]) -> Option<TextureCreatedResponse> {
    if buf.len() < TEXTURE_CREATED_RESPONSE_WIRE_SIZE {
        return None;
    }
    Some(TextureCreatedResponse {
        client_id: u64::from_le_bytes(buf[0..8].try_into().ok()?),
        resource_id: u32::from_le_bytes(buf[8..12].try_into().ok()?),
        status: u32::from_le_bytes(buf[12..16].try_into().ok()?),
    })
}

// ── Inbox control-plane ───────────────────────────────────────────────────────
//
// KindIds and payload types for the inbox-backed display driver control plane.
// Requests carry a `reply_pid` field so the driver can send typed replies back
// to the caller's inbox without a pre-established port connection.
//
// KindId values are randomly generated UUID v4 bytes.

/// Client → Driver: initiate protocol handshake.
///
/// UUID: `5a321b8f-91e7-4c5d-bc6f-a893740e2b17`
pub const KIND_DRV_DISPLAY_HELLO: KindId = KindId([
    0x5a, 0x32, 0x1b, 0x8f, 0x91, 0xe7, 0x4c, 0x5d,
    0xbc, 0x6f, 0xa8, 0x93, 0x74, 0x0e, 0x2b, 0x17,
]);

/// Driver → Client: reply to HELLO with negotiated capabilities.
///
/// UUID: `f12e7a4c-63d5-48a9-b781-3e5f920dc468`
pub const KIND_DRV_DISPLAY_WELCOME: KindId = KindId([
    0xf1, 0x2e, 0x7a, 0x4c, 0x63, 0xd5, 0x48, 0xa9,
    0xb7, 0x81, 0x3e, 0x5f, 0x92, 0x0d, 0xc4, 0x68,
]);

/// Client → Driver: bind a framebuffer FD.
///
/// UUID: `8c473f5e-219a-4b8d-e672-c38415f79b52`
pub const KIND_DRV_DISPLAY_BIND: KindId = KindId([
    0x8c, 0x47, 0x3f, 0x5e, 0x21, 0x9a, 0x4b, 0x8d,
    0xe6, 0x72, 0xc3, 0x84, 0x15, 0xf7, 0x9b, 0x52,
]);

/// Client → Driver: present a frame.
///
/// UUID: `9b4e276f-c183-4da2-e579-316c84f50ba7`
pub const KIND_DRV_DISPLAY_PRESENT: KindId = KindId([
    0x9b, 0x4e, 0x27, 0x6f, 0xc1, 0x83, 0x4d, 0xa2,
    0xe5, 0x79, 0x31, 0x6c, 0x84, 0xf5, 0x0b, 0xa7,
]);

/// Driver → Client: acknowledge a request.
///
/// UUID: `3e91b27c-584a-4fd6-a15e-8723c96b148d`
pub const KIND_DRV_DISPLAY_ACK: KindId = KindId([
    0x3e, 0x91, 0xb2, 0x7c, 0x58, 0x4a, 0x4f, 0xd6,
    0xa1, 0x5e, 0x87, 0x23, 0xc9, 0x6b, 0x14, 0x8d,
]);

/// Driver → Client: error response.
///
/// UUID: `726c8d39-e4b5-4a17-932f-c685471eda90`
pub const KIND_DRV_DISPLAY_ERR: KindId = KindId([
    0x72, 0x6c, 0x8d, 0x39, 0xe4, 0xb5, 0x4a, 0x17,
    0x93, 0x2f, 0xc6, 0x85, 0x47, 0x1e, 0xda, 0x90,
]);

// ── Inbox request payload types ───────────────────────────────────────────────

/// Payload for [`KIND_DRV_DISPLAY_HELLO`] (client → driver).
///
/// `reply_pid` carries the sender's PID so the driver can send the
/// [`KIND_DRV_DISPLAY_WELCOME`] reply to the caller's inbox.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct HelloReqInbox {
    pub reply_pid: u32,
    pub proto_major: u16,
    pub proto_minor: u16,
    pub want_caps: u32,
    pub _pad: u32,
}

pub const HELLO_REQ_INBOX_WIRE_SIZE: usize = size_of::<HelloReqInbox>();

/// Payload for [`KIND_DRV_DISPLAY_BIND`] (client → driver).
///
/// `reply_pid` carries the sender's PID for the ACK/ERR reply.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BindReqInbox {
    pub reply_pid: u32,
    pub fb_fd: u32,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
}

pub const BIND_REQ_INBOX_WIRE_SIZE: usize = size_of::<BindReqInbox>();

/// Payload for [`KIND_DRV_DISPLAY_PRESENT`] (client → driver).
///
/// `reply_pid` carries the sender's PID for the ACK/ERR reply.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PresentReqInbox {
    pub reply_pid: u32,
    pub flags: u32,
    pub rect_count: u32,
    pub _pad: u32,
}

pub const PRESENT_REQ_INBOX_WIRE_SIZE: usize = size_of::<PresentReqInbox>();

// ── Inbox encode/decode helpers ───────────────────────────────────────────────

pub fn encode_hello_req_inbox_le(req: &HelloReqInbox, out: &mut [u8]) -> Option<usize> {
    if out.len() < HELLO_REQ_INBOX_WIRE_SIZE {
        return None;
    }
    out[0..4].copy_from_slice(&req.reply_pid.to_le_bytes());
    out[4..6].copy_from_slice(&req.proto_major.to_le_bytes());
    out[6..8].copy_from_slice(&req.proto_minor.to_le_bytes());
    out[8..12].copy_from_slice(&req.want_caps.to_le_bytes());
    out[12..16].copy_from_slice(&req._pad.to_le_bytes());
    Some(HELLO_REQ_INBOX_WIRE_SIZE)
}

pub fn decode_hello_req_inbox_le(buf: &[u8]) -> Option<HelloReqInbox> {
    if buf.len() < HELLO_REQ_INBOX_WIRE_SIZE {
        return None;
    }
    Some(HelloReqInbox {
        reply_pid: u32::from_le_bytes(buf[0..4].try_into().ok()?),
        proto_major: u16::from_le_bytes(buf[4..6].try_into().ok()?),
        proto_minor: u16::from_le_bytes(buf[6..8].try_into().ok()?),
        want_caps: u32::from_le_bytes(buf[8..12].try_into().ok()?),
        _pad: u32::from_le_bytes(buf[12..16].try_into().ok()?),
    })
}

pub fn encode_bind_req_inbox_le(req: &BindReqInbox, out: &mut [u8]) -> Option<usize> {
    if out.len() < BIND_REQ_INBOX_WIRE_SIZE {
        return None;
    }
    out[0..4].copy_from_slice(&req.reply_pid.to_le_bytes());
    out[4..8].copy_from_slice(&req.fb_fd.to_le_bytes());
    out[8..12].copy_from_slice(&req.width.to_le_bytes());
    out[12..16].copy_from_slice(&req.height.to_le_bytes());
    out[16..20].copy_from_slice(&req.stride.to_le_bytes());
    out[20..24].copy_from_slice(&req.format.to_le_bytes());
    Some(BIND_REQ_INBOX_WIRE_SIZE)
}

pub fn decode_bind_req_inbox_le(buf: &[u8]) -> Option<BindReqInbox> {
    if buf.len() < BIND_REQ_INBOX_WIRE_SIZE {
        return None;
    }
    Some(BindReqInbox {
        reply_pid: u32::from_le_bytes(buf[0..4].try_into().ok()?),
        fb_fd: u32::from_le_bytes(buf[4..8].try_into().ok()?),
        width: u32::from_le_bytes(buf[8..12].try_into().ok()?),
        height: u32::from_le_bytes(buf[12..16].try_into().ok()?),
        stride: u32::from_le_bytes(buf[16..20].try_into().ok()?),
        format: u32::from_le_bytes(buf[20..24].try_into().ok()?),
    })
}

pub fn encode_present_req_inbox_le(req: &PresentReqInbox, out: &mut [u8]) -> Option<usize> {
    if out.len() < PRESENT_REQ_INBOX_WIRE_SIZE {
        return None;
    }
    out[0..4].copy_from_slice(&req.reply_pid.to_le_bytes());
    out[4..8].copy_from_slice(&req.flags.to_le_bytes());
    out[8..12].copy_from_slice(&req.rect_count.to_le_bytes());
    out[12..16].copy_from_slice(&req._pad.to_le_bytes());
    Some(PRESENT_REQ_INBOX_WIRE_SIZE)
}

pub fn decode_present_req_inbox_le(buf: &[u8]) -> Option<PresentReqInbox> {
    if buf.len() < PRESENT_REQ_INBOX_WIRE_SIZE {
        return None;
    }
    Some(PresentReqInbox {
        reply_pid: u32::from_le_bytes(buf[0..4].try_into().ok()?),
        flags: u32::from_le_bytes(buf[4..8].try_into().ok()?),
        rect_count: u32::from_le_bytes(buf[8..12].try_into().ok()?),
        _pad: u32::from_le_bytes(buf[12..16].try_into().ok()?),
    })
}
