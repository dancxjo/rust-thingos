use alloc::vec::Vec;

use abi::device::{DeviceCall, DeviceKind};
use abi::display::{
    BufferHandle, CommitFlags, CommitRequest, DISPLAY_OP_COMMIT, DISPLAY_OP_GET_INFO,
    DISPLAY_OP_IMPORT_BUFFER, DISPLAY_OP_RELEASE_BUFFER, DisplayInfo, PlaneCommit, PlaneId,
};
use abi::display_protocol::Rect;
use abi::pixel::PixelFormat;
use stem::syscall::vfs::{vfs_close, vfs_device_call_raw, vfs_open};

use crate::scene::CompositionEntry;

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
        let bh = BufferHandle { handle: thing, width, height, stride, format, offset };
        let mut id = 0u32;
        match device_call(self.fd, DISPLAY_OP_IMPORT_BUFFER, &bh, Some(&mut id)) {
            Ok(_) => {
                stem::info!("bloom: imported buffer {}x{} as ID={}", width, height, id);
                Some(id)
            }
            Err(e) => {
                stem::error!("bloom: failed to import buffer: {:?}", e);
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
    ) -> PresentResult {
        let _ = damage;
        let selected = composition_list
            .iter()
            .max_by_key(|entry| entry.z_order)
            .map(|entry| {
                (entry.buffer_id, entry.src_rect, entry.dest_rect, entry.z_order, entry.alpha)
            })
            .or_else(|| {
                fallback_buffer.map(|id| {
                    let (w, h) = self.output_size();
                    (id, Rect { x: 0, y: 0, w, h }, Rect { x: 0, y: 0, w, h }, 0, 255)
                })
            });

        let Some((buffer_id, src_rect, dest_rect, z_order, alpha)) = selected else {
            return PresentResult { success: true };
        };

        let plane = PlaneCommit {
            plane_id: PlaneId(0),
            buffer_id: abi::display::BufferId(buffer_id),
            dest_rect,
            src_rect,
            z_order,
            alpha,
            _reserved: [0; 7],
        };
        PresentResult { success: self.commit_display_planes(&[plane]) }
    }

    pub fn commit_display_planes(&self, planes: &[PlaneCommit]) -> bool {
        let req = CommitRequest { commit_count: planes.len() as u32, flags: CommitFlags::VSYNC, commits_ptr: 0 };
        let header_size = core::mem::size_of::<CommitRequest>();
        let plane_size = core::mem::size_of::<PlaneCommit>();
        let mut buf = alloc::vec![0u8; header_size + planes.len() * plane_size];

        unsafe {
            core::ptr::copy_nonoverlapping(
                &req as *const _ as *const u8,
                buf.as_mut_ptr(),
                header_size,
            );
            core::ptr::copy_nonoverlapping(
                planes.as_ptr() as *const u8,
                buf.as_mut_ptr().add(header_size),
                planes.len() * plane_size,
            );
        }

        let call = abi::device::DeviceCall {
            kind: abi::device::DeviceKind::Display,
            op: DISPLAY_OP_COMMIT,
            in_ptr: buf.as_ptr() as u64,
            in_len: buf.len() as u32,
            out_ptr: 0,
            out_len: 0,
        };

        match unsafe { vfs_device_call_raw(self.fd, &call) } {
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
