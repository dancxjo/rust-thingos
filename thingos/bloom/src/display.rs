use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

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
const MAX_DAMAGE_RECTS: usize = 32;
const BACKGROUND_Z_ORDER: i32 = i32::MIN;
const MAX_COMMIT_PAYLOAD_BYTES: usize = core::mem::size_of::<CommitRequest>()
    + MAX_COMMIT_PLANES * core::mem::size_of::<PlaneCommit>()
    + MAX_DAMAGE_RECTS * core::mem::size_of::<Rect>();
static BOUNDED_DAMAGE_LOGS: AtomicU32 = AtomicU32::new(0);

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

    /// Returns `true` when the connected display driver supports blocking vsync
    /// (`DisplayCaps::VBLANK`).
    pub fn supports_vblank(&self) -> bool {
        self.info.caps.contains(abi::display::DisplayCaps::VBLANK)
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
        damage: &[Rect],
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
                z_order: BACKGROUND_Z_ORDER,
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

        PresentResult { success: self.commit_display_planes(&planes[..plane_count], damage) }
    }

    pub fn commit_display_planes(&self, planes: &[PlaneCommit], damage: &[Rect]) -> bool {
        if planes.len() > MAX_COMMIT_PLANES {
            stem::warn!("bloom: commit has too many planes ({})", planes.len());
            return false;
        }

        let mut damage_count = damage.len().min(MAX_DAMAGE_RECTS);
        let mut damage_rects = [Rect { x: 0, y: 0, w: 0, h: 0 }; MAX_DAMAGE_RECTS];
        if damage_count == 0 || damage.len() > MAX_DAMAGE_RECTS {
            let (w, h) = self.output_size();
            damage_rects[0] = Rect { x: 0, y: 0, w, h };
            damage_count = 1;
        } else {
            let (w, h) = self.output_size();
            let mut out_count = 0usize;
            for rect in damage.iter().take(MAX_DAMAGE_RECTS) {
                if let Some(clipped) = clip_rect_to_output(*rect, w, h) {
                    if !damage_rects[..out_count].contains(&clipped) {
                        damage_rects[out_count] = clipped;
                        out_count += 1;
                    }
                }
            }
            if out_count == 0 {
                damage_rects[0] = Rect { x: 0, y: 0, w, h };
                damage_count = 1;
            } else {
                damage_count = out_count;
            }
        }
        if damage_count > 0
            && !is_full_output_damage(
                damage_rects[0],
                self.info.preferred_mode.width,
                self.info.preferred_mode.height,
            )
            && BOUNDED_DAMAGE_LOGS.fetch_add(1, Ordering::Relaxed) < 4
        {
            stem::info!("bloom: committing bounded damage rects={}", damage_count);
        }

        let req = CommitRequest {
            commit_count: planes.len() as u32,
            flags: CommitFlags::VSYNC,
            commits_ptr: 0,
            damage_count: damage_count as u32,
            _reserved: 0,
            damage_ptr: 0,
        };

        let mut payload = [0u8; MAX_COMMIT_PAYLOAD_BYTES];
        let mut payload_len = 0usize;
        push_plain_bytes(&mut payload, &mut payload_len, &req);
        push_plain_slice(&mut payload, &mut payload_len, planes);
        push_plain_slice(&mut payload, &mut payload_len, &damage_rects[..damage_count]);
        let call = abi::device::DeviceCall {
            kind: abi::device::DeviceKind::Display,
            op: DISPLAY_OP_COMMIT,
            in_ptr: payload.as_ptr() as u64,
            in_len: payload_len as u32,
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

fn push_plain_bytes<T>(out: &mut [u8], len: &mut usize, value: &T) {
    let bytes = unsafe {
        core::slice::from_raw_parts(value as *const T as *const u8, core::mem::size_of::<T>())
    };
    let end = len.saturating_add(bytes.len());
    if end <= out.len() {
        out[*len..end].copy_from_slice(bytes);
        *len = end;
    }
}

fn push_plain_slice<T>(out: &mut [u8], len: &mut usize, values: &[T]) {
    if values.is_empty() {
        return;
    }
    let bytes = unsafe {
        core::slice::from_raw_parts(values.as_ptr() as *const u8, core::mem::size_of_val(values))
    };
    let end = len.saturating_add(bytes.len());
    if end <= out.len() {
        out[*len..end].copy_from_slice(bytes);
        *len = end;
    }
}

fn is_full_output_damage(rect: Rect, width: u32, height: u32) -> bool {
    rect.x == 0 && rect.y == 0 && rect.w >= width && rect.h >= height
}

fn clip_rect_to_output(rect: Rect, width: u32, height: u32) -> Option<Rect> {
    let x = rect.x.min(width);
    let y = rect.y.min(height);
    let w = rect.w.min(width.saturating_sub(x));
    let h = rect.h.min(height.saturating_sub(y));
    if w == 0 || h == 0 { None } else { Some(Rect { x, y, w, h }) }
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
