#![no_std]
#![no_main]

extern crate alloc;

mod damage;
mod display;
mod frame_clock;
mod input;
mod loop_types;
mod protocol;
mod render;
mod scene;
mod services;
mod world;

use abi::syscall::vfs_flags::{O_CREAT, O_RDONLY, O_RDWR, O_TRUNC};
use damage::DamageTracker;
use display::DisplayBackend;
use frame_clock::FrameClock;
use input::InputState;
use loop_types::BloomLoop;
use render::CompositorVisuals;
use scene::Scene;
use services::input_service::InputService;
use services::wallpaper::WallpaperService;
use services::wayland::WaylandService;
use stem::syscall::vfs::{
    vfs_close, vfs_handle_from_port, vfs_mkdir, vfs_open, vfs_read, vfs_watch_path, vfs_write,
};
use stem::syscall::port_create;
use stem::{error, info, warn};
use world::BloomWorld;

const SERVICE_PATH: &str = "/services/bloom";
const WP_PATH: &str = "/session/desktop/wallpaper";

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("bloom: compositor service starting");

    // ── Connect to the display ────────────────────────────────────────────────
    let mut display_opt = None;
    for _ in 0..50 {
        display_opt = DisplayBackend::connect("/dev/display/card0");
        if display_opt.is_some() {
            break;
        }
        stem::sleep_ms(100);
    }

    let display = if let Some(d) = display_opt {
        d
    } else {
        error!("bloom: failed to connect to /dev/display/card0 after retries");
        loop {
            stem::sleep_ms(1000);
        }
    };
    let outputs = display.enumerate_outputs();
    if outputs.is_empty() {
        error!("bloom: no outputs enumerated");
        loop {
            stem::sleep_ms(1000);
        }
    }
    let primary = outputs[0];
    info!("bloom: output0 {}x{} @ {}mHz", primary.width, primary.height, primary.refresh_mhz);

    // ── Create and publish the service port ───────────────────────────────────
    info!("bloom: creating service port...");
    let (service_write, service_read) = match port_create(65536) {
        Ok(pair) => pair,
        Err(e) => {
            error!("bloom: failed to create service port: {:?}", e);
            loop {
                stem::sleep_ms(1000);
            }
        }
    };
    publish_service_handle(SERVICE_PATH, service_write);

    // ── Session directories + wallpaper ───────────────────────────────────────
    let _ = vfs_mkdir("/session");
    let _ = vfs_mkdir("/session/desktop");

    let mut visuals = CompositorVisuals::new();
    // Synchronous load at startup — no render loop running yet so blocking is fine.
    visuals.prepare_background(&display, WP_PATH);
    if visuals.fallback_buffer_id().is_none() {
        visuals.prepare_background(&display, "/share/wallpapers/flower.bmp");
    }

    // ── Initial scene / damage / input state ─────────────────────────────────
    let scene = Scene::new();
    let mut damage = DamageTracker::new();
    damage.mark_full(primary.width, primary.height);
    let input = InputState::new(primary.width, primary.height);

    // ── Bristle event port ────────────────────────────────────────────────────
    // bloom creates a port pair for bristle HID events and registers the write
    // end with bristle; the read end is watched in the service loop.
    // register_with_bristle is called only after the read end is successfully
    // bridged to a VFS FD, so the write handle we hand to bristle is always
    // paired with an FD that bloom will actually watch.
    let bristle_fd = match port_create(4096) {
        Ok((write_handle, read_handle)) => {
            match vfs_handle_from_port(read_handle) {
                Ok(fd) => {
                    register_with_bristle(write_handle);
                    Some(fd)
                }
                Err(e) => {
                    warn!("bloom: failed to bridge bristle port to FD: {:?}", e);
                    None
                }
            }
        }
        Err(e) => {
            warn!("bloom: failed to create bristle event port: {:?}", e);
            None
        }
    };

    // ── Wallpaper watch FD ────────────────────────────────────────────────────
    let wp_watch_fd = vfs_watch_path(WP_PATH, abi::vfs_watch::mask::ALL_EVENTS, 0).ok();

    // ── Assemble BloomWorld ───────────────────────────────────────────────────
    let mut world = BloomWorld::new(scene, damage, input, visuals, display, primary);

    // ── Build and populate the BloomLoop ─────────────────────────────────────
    let frame_clock = FrameClock::new(primary.refresh_mhz);
    let mut bloom_loop = BloomLoop::new(frame_clock);

    // Service port → WaylandService
    match vfs_handle_from_port(service_read) {
        Ok(fd) => {
            bloom_loop.add_service(alloc::boxed::Box::new(WaylandService::new(fd)));
        }
        Err(_) => {
            error!("bloom: failed to bridge service port to FD");
        }
    }

    // Bristle FD → InputService
    if let Some(fd) = bristle_fd {
        bloom_loop.add_service(alloc::boxed::Box::new(InputService::new(fd)));
    }

    // Wallpaper watch → WallpaperService
    if let Some(fd) = wp_watch_fd {
        bloom_loop.add_service(alloc::boxed::Box::new(WallpaperService::new(fd, WP_PATH)));
    }

    // ── Run forever ───────────────────────────────────────────────────────────
    bloom_loop.run(&mut world)
}

fn publish_service_handle(path: &str, handle: u32) {
    let _ = vfs_mkdir("/services");
    if let Ok(fd) = vfs_open(path, O_CREAT | O_TRUNC | O_RDWR) {
        let text = alloc::format!("{}\n", handle);
        let _ = vfs_write(fd, text.as_bytes());
        let _ = vfs_close(fd);
    }
}

/// Read bristle's PID from `/run/bristle/pid`.
fn read_bristle_pid() -> Option<u32> {
    let fd = vfs_open("/run/bristle/pid", O_RDONLY).ok()?;
    let mut buf = [0u8; 32];
    let n = vfs_read(fd, &mut buf).unwrap_or(0);
    let _ = vfs_close(fd);
    if n == 0 {
        return None;
    }
    let s = core::str::from_utf8(&buf[..n]).ok()?.trim();
    s.parse::<u32>().ok()
}

/// Register bloom as a bristle event sink.
///
/// Waits for bristle to be ready (its PID file to appear), then sends a
/// `RegisterSink` inbox message with `BRISTLE_SINK_TAG_BLOOM` and the bloom
/// port write handle so bristle can deliver normalized HID events.
fn register_with_bristle(evt_write_handle: u32) {
    if evt_write_handle == 0 {
        warn!("bloom: no event write handle — skipping bristle registration");
        return;
    }

    // Wait for bristle's PID file.
    if let Err(e) = stem::fs::wait_until_exists("/run/bristle/pid") {
        warn!("bloom: failed waiting for /run/bristle/pid: {:?}", e);
        return;
    }

    let bristle_pid = match read_bristle_pid() {
        Some(p) => p,
        None => {
            warn!("bloom: could not read bristle PID");
            return;
        }
    };

    use abi::hid::{BRISTLE_SINK_TAG_BLOOM, KIND_BRISTLE_REGISTER_SINK, encode_register_sink};
    use abi::wire::KindId;
    use stem::syscall::message::msg_send;

    let payload = encode_register_sink(BRISTLE_SINK_TAG_BLOOM, evt_write_handle);
    match msg_send(bristle_pid, KindId(KIND_BRISTLE_REGISTER_SINK), &payload) {
        Ok(()) => info!("bloom: registered with bristle (pid={})", bristle_pid),
        Err(e) => warn!("bloom: bristle registration failed: {:?}", e),
    }
}
