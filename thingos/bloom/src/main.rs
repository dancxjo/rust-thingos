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
mod wayland;
mod world;

use abi::syscall::vfs_flags::{O_CREAT, O_RDWR, O_TRUNC};
use damage::DamageTracker;
use display::DisplayBackend;
use frame_clock::FrameClock;
use input::InputState;
use loop_types::BloomLoop;
use render::CompositorVisuals;
use scene::Scene;
use services::input_service::InputService;
use services::wallpaper::{WallpaperService, ensure_wallpaper_config};
use services::wayland::WaylandService;
use services::wayland_cmd::WaylandCommandService;
use stem::syscall::port_create;
use stem::syscall::vfs::{
    vfs_close, vfs_handle_from_port, vfs_mkdir, vfs_open, vfs_watch_path, vfs_write,
};
use stem::{error, info, warn};
use wayland::WaylandThreadArgs;
use world::BloomWorld;

const SERVICE_PATH: &str = "/services/bloom";
const WP_PATH: &str = "/session/desktop/wallpaper";

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("bloom: ENTERING MAIN");
    info!("bloom: compositor service starting");

    // ── Connect to the display ────────────────────────────────────────────────
    let mut display_opt = None;
    for i in 0..50 {
        stem::debug!("bloom: connect try {}...", i);
        display_opt = DisplayBackend::connect("/dev/display/card0");
        if display_opt.is_some() {
            stem::info!("bloom: connected to /dev/display/card0 on try {}", i);
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
    if display.supports_vblank() {
        info!("bloom: display driver supports VBLANK (vsync)");
    } else {
        info!("bloom: display driver does not support VBLANK");
    }

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
    visuals.prepare_solid_background(&display, 0xFFCCCCFF);
    let initial_wallpaper = ensure_wallpaper_config(WP_PATH);
    stem::info!("bloom: initial wallpaper configured {}", initial_wallpaper);
    visuals.prepare_cursor(&display);

    // ── Initial scene / damage / input state ─────────────────────────────────
    let scene = Scene::new();
    let mut damage = DamageTracker::new();
    damage.mark_full(primary.width, primary.height);
    let input = InputState::new(primary.width, primary.height);

    // ── Bristle event port ────────────────────────────────────────────────────
    // Bloom keeps a read FD ready for bristle HID events and registers the
    // sink from the service loop after startup so first paint is independent
    // from Bristle readiness.
    let bristle_pair = port_create(65536).ok();

    // ── Wallpaper watch FD ────────────────────────────────────────────────────
    let wp_watch_fd = match vfs_watch_path(WP_PATH, abi::vfs_watch::mask::ALL_EVENTS, 0) {
        Ok(fd) => {
            info!("bloom: watching wallpaper config {}", WP_PATH);
            Some(fd)
        }
        Err(e) => {
            warn!("bloom: failed to watch wallpaper config {}: {:?}", WP_PATH, e);
            None
        }
    };

    // ── Assemble BloomWorld ───────────────────────────────────────────────────
    let mut world = BloomWorld::new(scene, damage, input, visuals, display, primary);

    // ── Register the Wayland compositor as a scene client ────────────────────
    // All Wayland surfaces are owned by this single bloom scene client.
    let wayland_client_id = world.scene.register_client(0, None);

    // ── Build and populate the BloomLoop ─────────────────────────────────────
    let frame_clock = FrameClock::new(primary.refresh_mhz);
    let mut bloom_loop = BloomLoop::new(frame_clock);

    // Service port → WaylandService (native bloom protocol)
    match vfs_handle_from_port(service_read) {
        Ok(fd) => {
            bloom_loop.add_service(alloc::boxed::Box::new(WaylandService::new(fd)));
        }
        Err(_) => {
            error!("bloom: failed to bridge service port to FD");
        }
    }

    // Bristle FD → InputService
    if let Some((bristle_write, bristle_read)) = bristle_pair {
        match vfs_handle_from_port(bristle_read) {
            Ok(fd) => {
                bloom_loop
                    .add_service(alloc::boxed::Box::new(InputService::new(fd, bristle_write)));
            }
            Err(e) => warn!("bloom: failed to bridge bristle input port to FD: {:?}", e),
        }
    }

    // Wallpaper watch → WallpaperService
    bloom_loop.add_service(alloc::boxed::Box::new(WallpaperService::new(wp_watch_fd, WP_PATH)));

    // ── Spawn Wayland server thread + wire IPC ports ──────────────────────────
    // cmd port: Wayland → Main (surface operations)
    // evt port: Main → Wayland (buffer releases, frame dones)
    match (port_create(65536), port_create(65536)) {
        (Ok((cmd_write, cmd_read)), Ok((evt_write, evt_read))) => {
            match (vfs_handle_from_port(cmd_read), vfs_handle_from_port(evt_read)) {
                (Ok(cmd_read_fd), Ok(evt_read_fd)) => {
                    // Let world send frame-done events to the Wayland thread.
                    world.wayland_evt_write = Some(evt_write);

                    // Spawn the Wayland server (runs its own ServiceLoop).
                    let args = alloc::boxed::Box::new(WaylandThreadArgs { cmd_write, evt_read_fd });
                    let arg_ptr = alloc::boxed::Box::into_raw(args) as usize;
                    match stem::thread::spawn_with_arg(wayland::wayland_thread_entry, arg_ptr) {
                        Ok(_) => {
                            info!("bloom: Wayland server thread spawned");
                            bloom_loop.add_service(alloc::boxed::Box::new(
                                WaylandCommandService::new(
                                    cmd_read_fd,
                                    evt_write,
                                    wayland_client_id,
                                ),
                            ));
                        }
                        Err(e) => {
                            warn!("bloom: failed to spawn Wayland server thread: {:?}", e);
                            // Recover the allocation to avoid leaking it.
                            let _ = unsafe {
                                alloc::boxed::Box::from_raw(arg_ptr as *mut WaylandThreadArgs)
                            };
                        }
                    }
                }
                _ => warn!("bloom: failed to bridge Wayland IPC ports to FDs"),
            }
        }
        _ => warn!("bloom: failed to create Wayland IPC port pair"),
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
