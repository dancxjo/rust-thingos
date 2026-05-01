//! Virtio-GPU command structures
#![no_std]
use alloc::string::ToString;
use core::default::Default;
extern crate alloc;

// Command types
pub const VIRTIO_GPU_CMD_GET_DISPLAY_INFO: u32 = 0x0100;
pub const VIRTIO_GPU_CMD_RESOURCE_CREATE_2D: u32 = 0x0101;
pub const VIRTIO_GPU_CMD_RESOURCE_UNREF: u32 = 0x0102;
pub const VIRTIO_GPU_CMD_SET_SCANOUT: u32 = 0x0103;
pub const VIRTIO_GPU_CMD_RESOURCE_FLUSH: u32 = 0x0104;
pub const VIRTIO_GPU_CMD_TRANSFER_TO_HOST_2D: u32 = 0x0105;
pub const VIRTIO_GPU_CMD_RESOURCE_ATTACH_BACKING: u32 = 0x0106;
pub const VIRTIO_GPU_CMD_RESOURCE_DETACH_BACKING: u32 = 0x0107;

// Cursor queue command types (sent on cursorq, not controlq)
pub const VIRTIO_GPU_CMD_UPDATE_CURSOR: u32 = 0x0300;
pub const VIRTIO_GPU_CMD_MOVE_CURSOR: u32 = 0x0301;

// Response types
pub const VIRTIO_GPU_RESP_OK_NODATA: u32 = 0x1100;
pub const VIRTIO_GPU_RESP_OK_DISPLAY_INFO: u32 = 0x1101;
pub const VIRTIO_GPU_RESP_ERR_UNSPEC: u32 = 0x1200;
pub const VIRTIO_GPU_RESP_ERR_OUT_OF_MEMORY: u32 = 0x1201;
pub const VIRTIO_GPU_RESP_ERR_INVALID_SCANOUT_ID: u32 = 0x1202;
pub const VIRTIO_GPU_RESP_ERR_INVALID_RESOURCE_ID: u32 = 0x1203;
pub const VIRTIO_GPU_RESP_ERR_INVALID_CONTEXT_ID: u32 = 0x1204;
pub const VIRTIO_GPU_RESP_ERR_INVALID_PARAMETER: u32 = 0x1205;

// Formats
pub const VIRTIO_GPU_FORMAT_B8G8R8A8_UNORM: u32 = 1;
pub const VIRTIO_GPU_FORMAT_B8G8R8X8_UNORM: u32 = 2;
pub const VIRTIO_GPU_FORMAT_R8G8B8A8_UNORM: u32 = 67;
pub const VIRTIO_GPU_FORMAT_R8G8B8X8_UNORM: u32 = 68;

/// Control header for all virtio-gpu commands
#[repr(C, packed)]
pub struct VirtioGpuCtrlHdr {
    pub type_: u32,
    pub flags: u32,
    pub fence_id: u64,
    pub ctx_id: u32,
    pub padding: u32,
}

/// Display info for one scanout
#[repr(C, packed)]
pub struct VirtioGpuDisplayOne {
    pub r_x: u32,
    pub r_y: u32,
    pub r_width: u32,
    pub r_height: u32,
    pub enabled: u32,
    pub flags: u32,
}

/// Response to GET_DISPLAY_INFO
#[repr(C, packed)]
pub struct VirtioGpuRespDisplayInfo {
    pub hdr: VirtioGpuCtrlHdr,
    pub pmodes: [VirtioGpuDisplayOne; 16],
}

/// Create 2D resource request
#[repr(C, packed)]
pub struct VirtioGpuResourceCreate2d {
    pub hdr: VirtioGpuCtrlHdr,
    pub resource_id: u32,
    pub format: u32,
    pub width: u32,
    pub height: u32,
}

/// Destroy a resource.
#[repr(C, packed)]
pub struct VirtioGpuResourceUnref {
    pub hdr: VirtioGpuCtrlHdr,
    pub resource_id: u32,
    pub padding: u32,
}

/// Attach backing memory entry
#[repr(C, packed)]
pub struct VirtioGpuMemEntry {
    pub addr: u64,
    pub length: u32,
    pub padding: u32,
}

/// Attach backing request
#[repr(C, packed)]
pub struct VirtioGpuResourceAttachBacking {
    pub hdr: VirtioGpuCtrlHdr,
    pub resource_id: u32,
    pub nr_entries: u32,
    // Followed by VirtioGpuMemEntry array
}

/// Set scanout request
#[repr(C, packed)]
pub struct VirtioGpuSetScanout {
    pub hdr: VirtioGpuCtrlHdr,
    pub r_x: u32,
    pub r_y: u32,
    pub r_width: u32,
    pub r_height: u32,
    pub scanout_id: u32,
    pub resource_id: u32,
}

/// Transfer to host request
#[repr(C, packed)]
pub struct VirtioGpuTransferToHost2d {
    pub hdr: VirtioGpuCtrlHdr,
    pub r_x: u32,
    pub r_y: u32,
    pub r_width: u32,
    pub r_height: u32,
    pub offset: u64,
    pub resource_id: u32,
    pub padding: u32,
}

/// Flush resource request
#[repr(C, packed)]
pub struct VirtioGpuResourceFlush {
    pub hdr: VirtioGpuCtrlHdr,
    pub r_x: u32,
    pub r_y: u32,
    pub r_width: u32,
    pub r_height: u32,
    pub resource_id: u32,
    pub padding: u32,
}

// ============================================================================
// 3D/Virgl Commands (0x0200 range)
// ============================================================================

// 3D command types
pub const VIRTIO_GPU_CMD_CTX_CREATE: u32 = 0x0200;
pub const VIRTIO_GPU_CMD_CTX_DESTROY: u32 = 0x0201;
pub const VIRTIO_GPU_CMD_CTX_ATTACH_RESOURCE: u32 = 0x0202;
pub const VIRTIO_GPU_CMD_CTX_DETACH_RESOURCE: u32 = 0x0203;
pub const VIRTIO_GPU_CMD_RESOURCE_CREATE_3D: u32 = 0x0204;
pub const VIRTIO_GPU_CMD_TRANSFER_TO_HOST_3D: u32 = 0x0205;
pub const VIRTIO_GPU_CMD_TRANSFER_FROM_HOST_3D: u32 = 0x0206;
pub const VIRTIO_GPU_CMD_SUBMIT_3D: u32 = 0x0207;

// 3D resource targets (gallium pipe_texture_target)
pub const PIPE_BUFFER: u32 = 0;
pub const PIPE_TEXTURE_1D: u32 = 1;
pub const PIPE_TEXTURE_2D: u32 = 2;
pub const PIPE_TEXTURE_3D: u32 = 3;
pub const PIPE_TEXTURE_CUBE: u32 = 4;
pub const PIPE_TEXTURE_RECT: u32 = 5;
pub const PIPE_TEXTURE_1D_ARRAY: u32 = 6;
pub const PIPE_TEXTURE_2D_ARRAY: u32 = 7;

// 3D resource bind flags (gallium pipe_bind)
pub const PIPE_BIND_DEPTH_STENCIL: u32 = 1 << 0;
pub const PIPE_BIND_RENDER_TARGET: u32 = 1 << 1;
pub const PIPE_BIND_SAMPLER_VIEW: u32 = 1 << 3;
pub const PIPE_BIND_VERTEX_BUFFER: u32 = 1 << 4;
pub const PIPE_BIND_INDEX_BUFFER: u32 = 1 << 5;
pub const PIPE_BIND_CONSTANT_BUFFER: u32 = 1 << 6;
pub const PIPE_BIND_SCANOUT: u32 = 1 << 14;

/// Create 3D context request
#[repr(C, packed)]
pub struct VirtioGpuCtxCreate {
    pub hdr: VirtioGpuCtrlHdr,
    pub nlen: u32,
    pub context_init: u32, // capset id to use for context init
    pub debug_name: [u8; 64],
}

/// Destroy 3D context request
#[repr(C, packed)]
pub struct VirtioGpuCtxDestroy {
    pub hdr: VirtioGpuCtrlHdr,
}

/// Attach resource to context request
#[repr(C, packed)]
pub struct VirtioGpuCtxResource {
    pub hdr: VirtioGpuCtrlHdr,
    pub resource_id: u32,
    pub padding: u32,
}

/// Create 3D resource request
#[repr(C, packed)]
pub struct VirtioGpuResourceCreate3d {
    pub hdr: VirtioGpuCtrlHdr,
    pub resource_id: u32,
    pub target: u32, // PIPE_TEXTURE_*
    pub format: u32, // virgl format
    pub bind: u32,   // PIPE_BIND_* flags
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub array_size: u32,
    pub last_level: u32,
    pub nr_samples: u32,
    pub flags: u32,
    pub padding: u32,
}

/// Transfer to host 3D request
#[repr(C, packed)]
pub struct VirtioGpuTransferToHost3d {
    pub hdr: VirtioGpuCtrlHdr,
    pub box_x: u32,
    pub box_y: u32,
    pub box_z: u32,
    pub box_w: u32,
    pub box_h: u32,
    pub box_d: u32,
    pub offset: u64,
    pub resource_id: u32,
    pub level: u32,
    pub stride: u32,
    pub layer_stride: u32,
}

/// Submit 3D command buffer request
#[repr(C, packed)]
pub struct VirtioGpuCmdSubmit3d {
    pub hdr: VirtioGpuCtrlHdr,
    pub size: u32,
    pub padding: u32,
    // Followed by `size` bytes of virgl command stream
}

// ============================================================================
// Virgl Command Encoding for GPU Alpha Blending
// ============================================================================

/// Virgl command opcode: BLIT — copy a region from one resource to another.
///
/// When the `VIRGL_BLIT_S0_ALPHA_BLEND` flag is set in the `s0` field the
/// virglrenderer performs a blitter-based render pass with alpha blending
/// enabled on the destination surface, compositing the source over the
/// destination using the standard Porter-Duff "over" operator.
///
/// Command header format (from virglrenderer `virgl_protocol.h`):
/// ```text
/// bits  0-7:  command opcode
/// bits  8-15: object type (0 for non-object commands)
/// bits 16-31: payload length in 32-bit DWORDs (excluding this header word)
/// ```
/// Virgl context command opcode for `VIRGL_CCMD_BLIT`.
///
/// The protocol defines this as an enum value, not a sparse hex command ID:
/// `NOP = 0`, `CREATE_OBJECT = 1`, ..., `SET_SCISSOR_STATE = 15`,
/// `BLIT = 16`.
pub const VIRGL_CCMD_BLIT: u32 = 16;

/// Number of additional 32-bit DWORDs in the BLIT payload (after the header).
pub const VIRGL_CMD_BLIT_SIZE: u32 = 21;

/// BLIT s0 field: write mask covering all four RGBA channels (bits 0-3).
pub const VIRGL_BLIT_S0_MASK_RGBA: u32 = 0x0f;

/// BLIT s0 field: enable Porter-Duff "over" alpha blending (bit 12).
///
/// When set, virglrenderer instructs the blitter to enable alpha blending on
/// the destination surface (`GL_SRC_ALPHA / GL_ONE_MINUS_SRC_ALPHA`).  The
/// result is:
/// `out = src_color * src_alpha + dst_color * (1 – src_alpha)`.
///
/// The `global_alpha` pre-multiplication must be baked into the source
/// texture's alpha channel by the caller before issuing this command so
/// that the GPU blend produces the same result as the CPU fallback.
///
/// Bit position 12 follows the virglrenderer `VIRGL_CMD_BLIT_S0_ALPHA_BLEND`
/// constant from `virgl_protocol.h`.
pub const VIRGL_BLIT_S0_ALPHA_BLEND: u32 = 1 << 12;

/// Build the first DWord of a virgl command buffer entry.
///
/// `cmd` is the VIRGL_CCMD_* opcode (8-bit value placed in bits 0-7).
/// `obj` is the object type (8-bit, placed in bits 8-15; 0 for non-object
/// commands like BLIT).
/// `len` is the number of 32-bit DWORDs that follow this header word.
#[inline]
pub const fn virgl_cmd0(cmd: u32, obj: u32, len: u32) -> u32 {
    (cmd & 0xff) | ((obj & 0xff) << 8) | ((len & 0xffff) << 16)
}

/// Encode a `VIRGL_CCMD_BLIT` command into a byte buffer (22 DWORDs = 88 bytes).
///
/// The produced command composites the source rectangle `(src_x, src_y,
/// src_w, src_h)` from `src_res_id` over the destination rectangle
/// `(dst_x, dst_y, dst_w, dst_h)` in `dst_res_id`.
///
/// When `alpha_blend` is `true` the virglrenderer enables GL alpha blending
/// (`GL_SRC_ALPHA / GL_ONE_MINUS_SRC_ALPHA`) so the result is
/// `out = src_color * src_alpha + dst_color * (1 – src_alpha)`.  The
/// `global_alpha` pre-multiplication must be baked into the source texture's
/// alpha channel by the caller before issuing this command.
///
/// Both resources must be attached to the virgl context before submission
/// (via `VIRTIO_GPU_CMD_CTX_ATTACH_RESOURCE`).
#[allow(clippy::too_many_arguments)]
pub fn virgl_encode_blit(
    src_res_id: u32,
    dst_res_id: u32,
    src_x: u32,
    src_y: u32,
    src_w: u32,
    src_h: u32,
    dst_x: u32,
    dst_y: u32,
    dst_w: u32,
    dst_h: u32,
    src_format: u32,
    dst_format: u32,
    alpha_blend: bool,
) -> [u8; 88] {
    // Header: cmd_opcode | (obj_type=0 << 8) | (payload_dwords << 16)
    let header: u32 = virgl_cmd0(VIRGL_CCMD_BLIT, 0, VIRGL_CMD_BLIT_SIZE);
    let mut s0 = VIRGL_BLIT_S0_MASK_RGBA;
    if alpha_blend {
        s0 |= VIRGL_BLIT_S0_ALPHA_BLEND;
    }
    let dwords: [u32; 22] = [
        header, s0, 0, // scissor minx|miny (scissor disabled)
        0, // scissor maxx|maxy
        // dst (9 DWORDs)
        dst_res_id, 0, // dst_level
        dst_format, dst_x, dst_y, 0, // dst_z
        dst_w, dst_h, 1, // dst_depth
        // src (9 DWORDs)
        src_res_id, 0, // src_level
        src_format, src_x, src_y, 0, // src_z
        src_w, src_h, 1, // src_depth
    ];

    let mut bytes = [0u8; 88];
    for (i, &dw) in dwords.iter().enumerate() {
        let off = i * 4;
        bytes[off] = (dw & 0xff) as u8;
        bytes[off + 1] = ((dw >> 8) & 0xff) as u8;
        bytes[off + 2] = ((dw >> 16) & 0xff) as u8;
        bytes[off + 3] = (dw >> 24) as u8;
    }
    bytes
}

// ============================================================================
// Tests for virgl command encoding
// ============================================================================

#[cfg(test)]
mod virgl_blit_tests {
    use super::*;

    #[test]
    fn virgl_cmd0_encodes_correct_header() {
        // cmd in bits 0-7, obj in bits 8-15, len in bits 16-31
        let hdr = virgl_cmd0(VIRGL_CCMD_BLIT, 0, 21);
        assert_eq!(hdr & 0xff, VIRGL_CCMD_BLIT, "opcode must be in low byte");
        assert_eq!((hdr >> 8) & 0xff, 0, "object type must be 0");
        assert_eq!((hdr >> 16) & 0xffff, 21, "length must be 21");
    }

    #[test]
    fn virgl_encode_blit_header_byte_layout() {
        let cmd = virgl_encode_blit(1, 2, 0, 0, 16, 16, 8, 8, 16, 16, 1, 2, false);
        // DWord 0 (bytes 0-3) is the virgl command header.
        // Expected: virgl_cmd0(VIRGL_CCMD_BLIT, 0, 21) in little-endian = [0x10, 0x00, 0x15, 0x00]
        assert_eq!(cmd[0], 0x10, "opcode byte 0");
        assert_eq!(cmd[1], 0x00, "opcode byte 1");
        assert_eq!(cmd[2], 0x15, "length low byte (21 = 0x15)");
        assert_eq!(cmd[3], 0x00, "length high byte");
    }

    #[test]
    fn virgl_encode_blit_alpha_blend_flag() {
        // Without alpha_blend: s0 = VIRGL_BLIT_S0_MASK_RGBA = 0x0f
        let no_blend = virgl_encode_blit(1, 2, 0, 0, 4, 4, 0, 0, 4, 4, 1, 2, false);
        let s0_no = u32::from_le_bytes([no_blend[4], no_blend[5], no_blend[6], no_blend[7]]);
        assert_eq!(s0_no, 0x0f, "s0 without alpha_blend");

        // With alpha_blend: s0 = 0x0f | (1 << 12) = 0x100f
        let with_blend = virgl_encode_blit(1, 2, 0, 0, 4, 4, 0, 0, 4, 4, 1, 2, true);
        let s0_blend =
            u32::from_le_bytes([with_blend[4], with_blend[5], with_blend[6], with_blend[7]]);
        assert_eq!(s0_blend, 0x100f, "s0 with alpha_blend enabled");
        assert_ne!(s0_no, s0_blend, "alpha_blend flag must change s0");
    }

    #[test]
    fn virgl_encode_blit_total_size() {
        let cmd = virgl_encode_blit(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, false);
        assert_eq!(cmd.len(), 88, "BLIT command must be exactly 22 DWORDs = 88 bytes");
    }

    #[test]
    fn virgl_encode_blit_matches_protocol_field_order() {
        let cmd = virgl_encode_blit(10, 20, 1, 2, 3, 4, 5, 6, 7, 8, 30, 40, true);
        let mut words = [0u32; 22];
        for (i, word) in words.iter_mut().enumerate() {
            let off = i * 4;
            *word = u32::from_le_bytes([cmd[off], cmd[off + 1], cmd[off + 2], cmd[off + 3]]);
        }

        assert_eq!(words[1], 0x100f, "s0");
        assert_eq!(words[4], 20, "dst resource");
        assert_eq!(words[5], 0, "dst level");
        assert_eq!(words[6], 40, "dst format");
        assert_eq!(words[7], 5, "dst x");
        assert_eq!(words[8], 6, "dst y");
        assert_eq!(words[9], 0, "dst z");
        assert_eq!(words[10], 7, "dst w");
        assert_eq!(words[11], 8, "dst h");
        assert_eq!(words[12], 1, "dst depth");
        assert_eq!(words[13], 10, "src resource");
        assert_eq!(words[14], 0, "src level");
        assert_eq!(words[15], 30, "src format");
        assert_eq!(words[16], 1, "src x");
        assert_eq!(words[17], 2, "src y");
        assert_eq!(words[18], 0, "src z");
        assert_eq!(words[19], 3, "src w");
        assert_eq!(words[20], 4, "src h");
        assert_eq!(words[21], 1, "src depth");
    }
}

// ============================================================================
// Cursor Queue Commands (0x0300 range — sent on cursorq)
// ============================================================================

/// Screen position used in cursor queue commands.
#[repr(C, packed)]
pub struct VirtioGpuCursorPos {
    pub scanout_id: u32,
    pub x: u32,
    pub y: u32,
    pub padding: u32,
}

/// Update cursor image and/or position (`VIRTIO_GPU_CMD_UPDATE_CURSOR`).
///
/// - `resource_id`: ID of a 2D resource holding the 64×64 ARGB cursor image.
///   Pass 0 to hide the cursor.
/// - `hot_x`, `hot_y`: hotspot within the cursor image.
/// - `pos`: where the hotspot should appear on screen.
#[repr(C, packed)]
pub struct VirtioGpuUpdateCursor {
    pub hdr: VirtioGpuCtrlHdr,
    pub pos: VirtioGpuCursorPos,
    pub resource_id: u32,
    pub hot_x: u32,
    pub hot_y: u32,
    pub padding: u32,
}
