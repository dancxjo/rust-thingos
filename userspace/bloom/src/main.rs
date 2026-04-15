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

const USE_DISPLAY_CARD_FIRST_PAINT: bool = false;

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
    if USE_DISPLAY_CARD_FIRST_PAINT && present_via_display_card(&mut wallpaper, 50, 100) {
        info!("Bloom: First paint committed via /dev/display/card0");
    } else {
        if !USE_DISPLAY_CARD_FIRST_PAINT {
            info!("Bloom: display/card0 first paint disabled; using /dev/fb0 fallback");
        }
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
        device_thing: 0,
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

    // Fast path: exact-size blit can be copied as whole rows.
    if scaled_w == src_w && scaled_h == src_h {
        for y in 0..src_h {
            let dst_row = (offset_y + y) * dst_stride_pixels + offset_x;
            let src_row = y * src_stride_pixels;
            let dst_span = &mut frame[dst_row..dst_row + src_w];
            let src_span = &src[src_row..src_row + src_w];
            copy_row_u32_fast(dst_span, src_span);
        }
        return;
    }

    // Precompute source coordinate mappings once; this removes costly divides
    // from the nearest-neighbor scaling loops.
    let mut sx_lut = alloc::vec![0usize; scaled_w];
    for (dx, sx) in sx_lut.iter_mut().enumerate() {
        *sx = ((dx as u64 * src_w as u64) / scaled_w as u64) as usize;
    }
    let mut sy_lut = alloc::vec![0usize; scaled_h];
    for (dy, sy) in sy_lut.iter_mut().enumerate() {
        *sy = ((dy as u64 * src_h as u64) / scaled_h as u64) as usize;
    }

    for dy in 0..scaled_h {
        let sy = sy_lut[dy];
        let dst_row = (offset_y + dy) * dst_stride_pixels;
        let src_row = sy * src_stride_pixels;
        for dx in 0..scaled_w {
            frame[dst_row + offset_x + dx] = src[src_row + sx_lut[dx]];
        }
    }
}

#[inline(always)]
fn copy_row_u32_fast(dst: &mut [u32], src: &[u32]) {
    debug_assert_eq!(dst.len(), src.len());

    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    unsafe {
        use core::arch::x86_64::{__m128i, _mm_loadu_si128, _mm_storeu_si128};

        let len = dst.len();
        let mut i = 0usize;
        while i + 4 <= len {
            let s = _mm_loadu_si128(src.as_ptr().add(i) as *const __m128i);
            _mm_storeu_si128(dst.as_mut_ptr().add(i) as *mut __m128i, s);
            i += 4;
        }
        if i < len {
            dst[i..].copy_from_slice(&src[i..]);
        }
        return;
    }

    #[cfg(all(target_arch = "x86", target_feature = "sse2"))]
    unsafe {
        use core::arch::x86::{__m128i, _mm_loadu_si128, _mm_storeu_si128};

        let len = dst.len();
        let mut i = 0usize;
        while i + 4 <= len {
            let s = _mm_loadu_si128(src.as_ptr().add(i) as *const __m128i);
            _mm_storeu_si128(dst.as_mut_ptr().add(i) as *mut __m128i, s);
            i += 4;
        }
        if i < len {
            dst[i..].copy_from_slice(&src[i..]);
        }
        return;
    }

    #[allow(unreachable_code)]
    dst.copy_from_slice(src);
}

fn get_wallpaper_path() -> String {
    String::from("/share/wallpapers/flower.bmp")
}

fn present_via_display_card(
    wallpaper: &mut petals::Texture,
    retries: usize,
    retry_delay_ms: u64,
) -> bool {
    let mut last_err = None;
    let mut fd = None;
    for _ in 0..retries {
        match vfs_open("/dev/display/card0", abi::syscall::vfs_flags::O_RDWR) {
            Ok(open_fd) => {
                fd = Some(open_fd);
                break;
            }
            Err(e) => {
                last_err = Some(e);
                stem::sleep_ms(retry_delay_ms);
            }
        }
    }
    let fd = match fd {
        Some(fd) => fd,
        None => {
            stem::debug!(
                "Bloom: /dev/display/card0 unavailable after retries: {:?}",
                last_err
            );
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
        thing: frame.fd,
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
    let ok = commit_display_planes(fd, &[plane], CommitFlags::empty());
    // Keep the imported first-paint buffer alive. Releasing immediately allows
    // backends that present asynchronously to lose the source buffer too early.
    if ok {
        core::mem::forget(frame);
    } else {
        let _ = device_call::<u32, ()>(fd, DISPLAY_OP_RELEASE_BUFFER, &buffer_id, None);
    }
    let _ = vfs_close(fd);
    ok
}

fn commit_display_planes(fd: u32, planes: &[PlaneCommit], flags: CommitFlags) -> bool {
    let req = CommitRequest {
        commit_count: planes.len() as u32,
        flags,
        commits_ptr: 0,
    };

    let req_size = core::mem::size_of::<CommitRequest>();
    let planes_size = core::mem::size_of_val(planes);
    let mut payload = Vec::with_capacity(req_size + planes_size);

    let req_bytes = unsafe { core::slice::from_raw_parts(&req as *const _ as *const u8, req_size) };
    payload.extend_from_slice(req_bytes);

    if !planes.is_empty() {
        let planes_bytes = unsafe {
            core::slice::from_raw_parts(planes.as_ptr() as *const u8, planes_size)
        };
        payload.extend_from_slice(planes_bytes);
    }

    let call = DeviceCall {
        kind: DeviceKind::Display,
        op: DISPLAY_OP_COMMIT,
        in_ptr: payload.as_ptr() as u64,
        in_len: payload.len() as u32,
        out_ptr: 0,
        out_len: 0,
    };

    vfs_device_call_raw(fd, &call).is_ok()
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
