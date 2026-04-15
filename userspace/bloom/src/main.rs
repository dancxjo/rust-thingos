#![no_std]
#![no_main]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use abi::device::{DeviceCall, DeviceKind};
use abi::display::{
    BufferHandle, CommitFlags, CommitRequest, DisplayInfo, PlaneCommit, PlaneId,
    DISPLAY_OP_COMMIT, DISPLAY_OP_GET_INFO, DISPLAY_OP_IMPORT_BUFFER, DISPLAY_OP_RELEASE_BUFFER,
};
use abi::display_driver_protocol::{FB_INFO_PAYLOAD_SIZE, FbInfoPayload};
use abi::display_protocol::Rect;
use abi::pixel::PixelFormat;
use abi::syscall::vfs_flags::{O_RDONLY, O_WRONLY};
use petals::bmp::load_bmp;
use petals::Texture;
use stem::syscall::vfs::{vfs_close, vfs_device_call_raw, vfs_open, vfs_read, vfs_write};
use stem::{error, info};

#[stem::main]
fn main(_arg: usize) -> ! {
    stem::debug!("Bloom: VFS-native compositor starting...");

    // 1. Discover the boot framebuffer at /dev/fb0.
    let mut fb_info = None;
    for _ in 0..50 {
        if let Some(info) = read_bootfb_info() {
            fb_info = Some(info);
            break;
        }
        stem::sleep_ms(100);
    }
    let fb_info = fb_info.expect("No /dev/fb0 found after retry!");
    stem::info!(
        "Bloom: Using boot framebuffer {}x{} stride={} bpp={} format={}",
        fb_info.width,
        fb_info.height,
        fb_info.stride,
        fb_info.bpp,
        fb_info.format
    );

    if fb_info.bpp != 32 || fb_info.stride < fb_info.width * 4 {
        error!(
            "Bloom: Unsupported boot framebuffer layout: width={} height={} stride={} bpp={}",
            fb_info.width, fb_info.height, fb_info.stride, fb_info.bpp
        );
        loop {
            stem::yield_now();
        }
    }

    // 2. Load wallpaper
    let wallpaper_path = get_wallpaper_path();
    stem::debug!("Bloom: Loading wallpaper: {}", wallpaper_path);

    let mut wallpaper = load_bmp(&wallpaper_path).expect("Failed to load wallpaper BMP");
    stem::debug!("Bloom: Wallpaper loaded ({}x{})", wallpaper.width, wallpaper.height);

    // 3. Prefer the display service path. It can use the real display backend
    // (for example virtio-gpu) instead of forcing first paint through /dev/fb0.
    if present_via_display_card(&mut wallpaper) {
        info!("Bloom: First paint committed via /dev/display/card0");
    } else {
        // Fallback: compose a framebuffer-sized image and write it directly to /dev/fb0.
        let frame = compose_wallpaper_frame(&mut wallpaper, &fb_info);
        let frame_bytes = unsafe {
            core::slice::from_raw_parts(
                frame.as_ptr() as *const u8,
                frame.len() * core::mem::size_of::<u32>(),
            )
        };
        let fd = vfs_open("/dev/fb0", O_WRONLY).expect("Failed to open /dev/fb0 for writing");
        match vfs_write(fd, frame_bytes) {
            Ok(n) if n == frame_bytes.len() => {
                info!("Bloom: First paint committed successfully!");
            }
            Ok(n) => {
                error!(
                    "Bloom: Short write to /dev/fb0: wrote {} of {} bytes",
                    n,
                    frame_bytes.len()
                );
            }
            Err(e) => error!("Bloom: Failed to write /dev/fb0: {:?}", e),
        }
        let _ = vfs_close(fd);
    }

    stem::debug!("Bloom: Transitioning to event loop...");
    loop {
        stem::yield_now();
    }
}

fn read_bootfb_info() -> Option<FbInfoPayload> {
    let fd = vfs_open("/dev/fb0", O_RDONLY).ok()?;
    let mut payload = FbInfoPayload {
        device_handle: 0,
        width: 0,
        height: 0,
        stride: 0,
        bpp: 0,
        format: 0,
        _reserved: 0,
    };
    let slice = unsafe {
        core::slice::from_raw_parts_mut(&mut payload as *mut _ as *mut u8, FB_INFO_PAYLOAD_SIZE)
    };
    let n = vfs_read(fd, slice).ok()?;
    let _ = vfs_close(fd);
    if n < FB_INFO_PAYLOAD_SIZE || payload.width == 0 || payload.height == 0 || payload.stride == 0
    {
        return None;
    }
    Some(payload)
}

fn compose_wallpaper_frame(wallpaper: &mut petals::Texture, fb_info: &FbInfoPayload) -> Vec<u32> {
    let dst_stride_pixels = (fb_info.stride / 4) as usize;
    let dst_len = (fb_info.height as usize) * dst_stride_pixels;
    let mut frame = alloc::vec![0xFF000000u32; dst_len];
    compose_wallpaper_into(
        &mut frame,
        dst_stride_pixels,
        fb_info.width as usize,
        fb_info.height as usize,
        wallpaper,
    );
    frame
}

fn compose_wallpaper_into(
    frame: &mut [u32],
    dst_stride_pixels: usize,
    dst_w: usize,
    dst_h: usize,
    wallpaper: &mut petals::Texture,
) {
    let src_stride_pixels = (wallpaper.stride / 4) as usize;
    let src_w = wallpaper.width as usize;
    let src_h = wallpaper.height as usize;
    let src = wallpaper.as_slice_mut();

    if src_w == 0 || src_h == 0 || dst_w == 0 || dst_h == 0 {
        return;
    }

    let scale_x = (dst_w as u64 * 1_000_000) / src_w as u64;
    let scale_y = (dst_h as u64 * 1_000_000) / src_h as u64;
    let scale = scale_x.min(scale_y).max(1);

    let scaled_w = ((src_w as u64 * scale) / 1_000_000) as usize;
    let scaled_h = ((src_h as u64 * scale) / 1_000_000) as usize;
    let offset_x = (dst_w.saturating_sub(scaled_w)) / 2;
    let offset_y = (dst_h.saturating_sub(scaled_h)) / 2;

    for dy in 0..scaled_h {
        let sy = ((dy as u64 * src_h as u64) / scaled_h as u64) as usize;
        let dst_row = (offset_y + dy) * dst_stride_pixels;
        let src_row = sy * src_stride_pixels;
        for dx in 0..scaled_w {
            let sx = ((dx as u64 * src_w as u64) / scaled_w as u64) as usize;
            frame[dst_row + offset_x + dx] = src[src_row + sx];
        }
    }
}

fn get_wallpaper_path() -> String {
    String::from("/share/wallpapers/flower.bmp")
}

fn present_via_display_card(wallpaper: &mut petals::Texture) -> bool {
    let fd = match vfs_open("/dev/display/card0", abi::syscall::vfs_flags::O_RDWR) {
        Ok(fd) => fd,
        Err(e) => {
            stem::debug!("Bloom: /dev/display/card0 unavailable: {:?}", e);
            return false;
        }
    };

    let info = match get_display_info(fd) {
        Some(info) => info,
        None => {
            stem::warn!("Bloom: Failed to query /dev/display/card0 info; falling back to /dev/fb0");
            let _ = vfs_close(fd);
            return false;
        }
    };

    let width = info.preferred_mode.width;
    let height = info.preferred_mode.height;
    let mut frame = match Texture::new("bloom.first-paint", width, height, 4) {
        Some(texture) => texture,
        None => {
            stem::warn!("Bloom: Failed to allocate first-paint back buffer");
            let _ = vfs_close(fd);
            return false;
        }
    };

    {
        let pixels = frame.as_slice_mut();
        pixels.fill(0xFF000000);
    }
    let frame_stride_pixels = (frame.stride / 4) as usize;
    let frame_pixels = frame.as_slice_mut();
    compose_wallpaper_into(
        frame_pixels,
        frame_stride_pixels,
        width as usize,
        height as usize,
        wallpaper,
    );

    let handle = BufferHandle {
        fd: frame.fd,
        offset: 0,
        width,
        height,
        stride: frame.stride,
        format: PixelFormat::Bgra8888,
        modifier: 0,
    };
    let buffer_id = match import_display_buffer(fd, &handle) {
        Some(id) => id,
        None => {
            stem::warn!("Bloom: Failed to import first-paint buffer into display driver");
            let _ = vfs_close(fd);
            return false;
        }
    };

    let plane = PlaneCommit {
        plane_id: PlaneId(0),
        buffer_id: abi::display::BufferId(buffer_id),
        dest_rect: Rect {
            x: 0,
            y: 0,
            w: width,
            h: height,
        },
        src_rect: Rect {
            x: 0,
            y: 0,
            w: width,
            h: height,
        },
        z_order: 0,
        alpha: 255,
        _reserved: [0; 7],
    };
    let req = CommitRequest {
        commit_count: 1,
        flags: CommitFlags::empty(),
        commits_ptr: &plane as *const PlaneCommit as u64,
    };

    let ok = device_call::<CommitRequest, ()>(fd, DISPLAY_OP_COMMIT, &req, None).is_some();
    let _ = device_call::<u32, ()>(fd, DISPLAY_OP_RELEASE_BUFFER, &buffer_id, None);
    let _ = vfs_close(fd);
    ok
}

fn get_display_info(fd: u32) -> Option<DisplayInfo> {
    let mut info = DisplayInfo {
        card_id: 0,
        preferred_mode: abi::display::DisplayMode {
            width: 0,
            height: 0,
            refresh_mhz: 0,
        },
        plane_count: 0,
        max_buffers: 0,
        supported_formats: 0,
        caps: abi::display::DisplayCaps::empty(),
    };
    device_call::<(), DisplayInfo>(fd, DISPLAY_OP_GET_INFO, &(), Some(&mut info)).map(|_| info)
}

fn import_display_buffer(fd: u32, handle: &BufferHandle) -> Option<u32> {
    device_call::<BufferHandle, ()>(fd, DISPLAY_OP_IMPORT_BUFFER, handle, None)
}

fn device_call<I, O>(fd: u32, op: u32, input: &I, output: Option<&mut O>) -> Option<u32> {
    let (out_ptr, out_len) = if let Some(out) = output {
        (
            out as *mut O as u64,
            core::mem::size_of::<O>() as u32,
        )
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
