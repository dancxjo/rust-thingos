use alloc::vec::Vec;

use abi::device::{DeviceCall, DeviceKind};
use abi::display::{
    BufferHandle, CommitFlags, CommitRequest, DISPLAY_OP_COMMIT, DISPLAY_OP_GET_INFO,
    DISPLAY_OP_IMPORT_BUFFER, DISPLAY_OP_RELEASE_BUFFER, DisplayInfo, PlaneCommit, PlaneId,
};
use abi::display_protocol::Rect;
use abi::pixel::PixelFormat;
use stem::syscall::vfs::{vfs_close, vfs_device_call_raw, vfs_open};

use crate::render::{CursorPlane, OverlayPlane};
use crate::scene::CompositionEntry;

const MAX_COMMIT_PLANES: usize = 16;

#[repr(C)]
struct CommitPacket {
    req: CommitRequest,
    planes: [PlaneCommit; MAX_COMMIT_PLANES],
}

const fn empty_plane_commit() -> PlaneCommit {
    PlaneCommit {
        plane_id: PlaneId(0),
        buffer_id: abi::display::BufferId(0),
        dest_rect: Rect { x: 0, y: 0, w: 0, h: 0 },
        src_rect: Rect { x: 0, y: 0, w: 0, h: 0 },
        z_order: 0,
        alpha: 0,
        _reserved: [0; 7],
    }
}

#[derive(Clone, Copy, Debug)]
pub struct OutputInfo {
    pub output_id: u32,
    pub width: u32,
    pub height: u32,
    pub refresh_mhz: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct PresentResult {
    pub success: bool,
}

pub struct DisplayBackend {
    fd: u32,
    info: DisplayInfo,
}

impl Drop for DisplayBackend {
    fn drop(&mut self) {
        let _ = vfs_close(self.fd);
    }
}

impl DisplayBackend {
    pub fn connect(path: &str) -> Option<Self> {
        let fd = vfs_open(path, abi::syscall::vfs_flags::O_RDWR).ok()?;
        let mut backend = Self {
            fd,
            info: DisplayInfo {
                card_id: 0,
                preferred_mode: abi::display::DisplayMode { width: 0, height: 0, refresh_mhz: 0 },
                plane_count: 0,
                max_buffers: 0,
                supported_formats: 0,
                caps: abi::display::DisplayCaps::empty(),
            },
        };
        backend.refresh_info()?;
        Some(backend)
    }

    pub fn refresh_info(&mut self) -> Option<()> {
        self.info = get_display_info(self.fd)?;
        Some(())
    }

    pub fn enumerate_outputs(&self) -> Vec<OutputInfo> {
        alloc::vec![OutputInfo {
            output_id: 0,
            width: self.info.preferred_mode.width,
            height: self.info.preferred_mode.height,
            refresh_mhz: self.info.preferred_mode.refresh_mhz,
        }]
    }

    pub fn output_size(&self) -> (u32, u32) {
        (self.info.preferred_mode.width, self.info.preferred_mode.height)
    }

    pub fn import_buffer(
        &self,
        thing: u32,
        width: u32,
        height: u32,
        stride: u32,
        format: PixelFormat,
        offset: u64,
    ) -> Option<u32> {
        let bh = BufferHandle { handle: thing, width, height, stride, format, offset, modifier: 0 };
        let mut id = 0u32;
        match device_call(self.fd, DISPLAY_OP_IMPORT_BUFFER, &bh, Some(&mut id)) {
            Some(_) => {
                stem::info!("bloom: imported buffer {}x{} as ID={}", width, height, id);
                Some(id)
            }
            None => {
                stem::error!("bloom: failed to import buffer");
                None
            }
        }
    }

    pub fn release_buffer(&self, buffer_id: u32) {
        let _ = device_call::<u32, ()>(self.fd, DISPLAY_OP_RELEASE_BUFFER, &buffer_id, None);
    }

    pub fn present(
        &self,
        composition_list: &[CompositionEntry],
        _damage: &[Rect],
        fallback_buffer: Option<u32>,
        pointer_overlay: Option<OverlayPlane>,
        cursor: Option<CursorPlane>,
    ) -> PresentResult {
        let mut planes = [empty_plane_commit(); MAX_COMMIT_PLANES];
        let mut plane_count = 0usize;

        // 1. Background plane
        if let Some(id) = fallback_buffer {
            let (w, h) = self.output_size();
            planes[plane_count] = PlaneCommit {
                plane_id: PlaneId(0),
                buffer_id: abi::display::BufferId(id),
                dest_rect: Rect { x: 0, y: 0, w, h },
                src_rect: Rect { x: 0, y: 0, w, h },
                z_order: 0,
                alpha: 255,
                _reserved: [0; 7],
            };
            plane_count += 1;
        }

        // 2. Surface planes
        for entry in composition_list {
            if plane_count >= MAX_COMMIT_PLANES {
                stem::warn!("bloom: dropping display plane beyond fixed commit capacity");
                break;
            }
            planes[plane_count] = PlaneCommit {
                plane_id: PlaneId(plane_count as u32),
                buffer_id: abi::display::BufferId(entry.buffer_id),
                dest_rect: entry.dest_rect,
                src_rect: entry.src_rect,
                z_order: entry.z_order,
                alpha: entry.alpha,
                _reserved: [0; 7],
            };
            plane_count += 1;
        }

        // 3. Diagnostic overlay. This is intentionally above the wallpaper and
        // client surfaces so pointer coordinates stay visible while debugging.
        if let Some(overlay) = pointer_overlay {
            if plane_count < MAX_COMMIT_PLANES {
                planes[plane_count] = PlaneCommit {
                    plane_id: PlaneId(plane_count as u32),
                    buffer_id: abi::display::BufferId(overlay.buffer_id),
                    dest_rect: Rect {
                        x: overlay.x.max(0) as u32,
                        y: overlay.y.max(0) as u32,
                        w: overlay.width,
                        h: overlay.height,
                    },
                    src_rect: Rect { x: 0, y: 0, w: overlay.width, h: overlay.height },
                    z_order: i32::MAX - 1,
                    alpha: 255,
                    _reserved: [0; 7],
                };
                plane_count += 1;
            }
        }

        // 4. Cursor plane, composed last. Software display drivers alpha-blend
        // this plane when hardware cursor planes are not available.
        if let Some(cursor) = cursor {
            if plane_count < MAX_COMMIT_PLANES {
                let (out_w, out_h) = self.output_size();
                let dst_x = cursor.x.max(0) as u32;
                let dst_y = cursor.y.max(0) as u32;
                let src_x = if cursor.x < 0 { cursor.x.saturating_neg() as u32 } else { 0 };
                let src_y = if cursor.y < 0 { cursor.y.saturating_neg() as u32 } else { 0 };
                let visible_w = cursor.width.saturating_sub(src_x).min(out_w.saturating_sub(dst_x));
                let visible_h =
                    cursor.height.saturating_sub(src_y).min(out_h.saturating_sub(dst_y));
                if visible_w > 0 && visible_h > 0 {
                    planes[plane_count] = PlaneCommit {
                        plane_id: PlaneId(plane_count as u32),
                        buffer_id: abi::display::BufferId(cursor.buffer_id),
                        dest_rect: Rect { x: dst_x, y: dst_y, w: visible_w, h: visible_h },
                        src_rect: Rect { x: src_x, y: src_y, w: visible_w, h: visible_h },
                        z_order: i32::MAX,
                        alpha: 255,
                        _reserved: [0; 7],
                    };
                    plane_count += 1;
                }
            }
        }

        if plane_count == 0 {
            return PresentResult { success: false };
        }

        planes[..plane_count].sort_unstable_by_key(|plane| plane.z_order);

        PresentResult { success: self.commit_display_planes(&planes[..plane_count]) }
    }

    pub fn commit_display_planes(&self, planes: &[PlaneCommit]) -> bool {
        if planes.len() > MAX_COMMIT_PLANES {
            stem::warn!("bloom: commit has too many planes ({})", planes.len());
            return false;
        }

        let mut packet = CommitPacket {
            req: CommitRequest {
                commit_count: planes.len() as u32,
                flags: CommitFlags::VSYNC,
                commits_ptr: 0,
            },
            planes: [empty_plane_commit(); MAX_COMMIT_PLANES],
        };
        packet.planes[..planes.len()].copy_from_slice(planes);

        let in_len = core::mem::size_of::<CommitRequest>()
            + planes.len() * core::mem::size_of::<PlaneCommit>();
        let call = abi::device::DeviceCall {
            kind: abi::device::DeviceKind::Display,
            op: DISPLAY_OP_COMMIT,
            in_ptr: &packet as *const CommitPacket as u64,
            in_len: in_len as u32,
            out_ptr: 0,
            out_len: 0,
        };

        match vfs_device_call_raw(self.fd, &call) {
            Ok(_) => true,
            Err(e) => {
                stem::error!("bloom: DISPLAY_OP_COMMIT failed: {:?}", e);
                false
            }
        }
    }
}

fn get_display_info(fd: u32) -> Option<DisplayInfo> {
    let mut info = DisplayInfo {
        card_id: 0,
        preferred_mode: abi::display::DisplayMode { width: 0, height: 0, refresh_mhz: 0 },
        plane_count: 0,
        max_buffers: 0,
        supported_formats: 0,
        caps: abi::display::DisplayCaps::empty(),
    };
    device_call::<(), DisplayInfo>(fd, DISPLAY_OP_GET_INFO, &(), Some(&mut info)).map(|_| info)
}

fn device_call<I, O>(fd: u32, op: u32, input: &I, output: Option<&mut O>) -> Option<u32> {
    let (out_ptr, out_len) = if let Some(out) = output {
        (out as *mut O as u64, core::mem::size_of::<O>() as u32)
    } else {
        (0, 0)
    };

    let call = DeviceCall {
        kind: DeviceKind::Display,
        op,
        in_ptr: input as *const I as u64,
        in_len: core::mem::size_of::<I>() as u32,
        out_ptr,
        out_len,
    };
    vfs_device_call_raw(fd, &call).ok().map(|v| v as u32)
}
