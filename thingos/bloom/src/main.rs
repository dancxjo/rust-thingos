#![no_std]
#![no_main]

extern crate alloc;

mod accel2d_batch;
mod cache;
mod compositor;
mod damage;
mod display;
mod frame_clock;
mod input;
mod loop_types;
mod protocol;
mod render;
mod scene;
mod services;
mod session_fs;
mod theme;
mod wayland;
mod world;

use alloc::string::String;

use abi::syscall::vfs_flags::{O_CREAT, O_RDONLY, O_RDWR, O_TRUNC};
use damage::DamageTracker;
use display::DisplayBackend;
use frame_clock::FrameClock;
use input::InputState;
use loop_types::BloomLoop;
use render::CompositorVisuals;
use scene::Scene;
use services::busy_spinner::BusySpinnerService;
use services::input_service::InputService;
use services::resources::ResourceRetryService;
use services::theme_service::{
    DEFAULT_THEME_CONFIG_PATH, WALLPAPER_CONFIG_PATH, ThemeService, ensure_theme_config,
    write_themed_wallpaper,
};
use services::wayland::WaylandService;
use services::wayland_cmd::WaylandCommandService;
use stem::syscall::port_create;
use stem::syscall::vfs::{
    vfs_close, vfs_handle_from_port, vfs_mkdir, vfs_open, vfs_read, vfs_watch_path, vfs_write,
};
use stem::{error, info, warn};
use wayland::WaylandThreadArgs;
use world::BloomWorld;

const SERVICE_PATH: &str = "/run/services/bloom";
const THEME_PATH: &str = DEFAULT_THEME_CONFIG_PATH;

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("Starting desktop compositor...");

    // ── Check for minimal boot mode ───────────────────────────────────────────
    let minimal_mode = read_minimal_boot_mode();
    stem::debug!("bloom: startup minimal_mode={}", minimal_mode);

    // ── Connect to the display ────────────────────────────────────────────────
    info!("Connecting to display service...");
    stem::debug!("bloom.phase=open_display");
    let mut display_opt = None;
    for i in 0..50 {
        stem::debug!("bloom: connect try {}...", i);
        display_opt = connect_display_card();
        if let Some((_, path)) = display_opt.as_ref() {
            stem::debug!("bloom: connected to {} on try {}", path, i);
            break;
        }
        stem::sleep_ms(100);
    }

    let (display, display_path) = if let Some(d) = display_opt {
        d
    } else {
        error!("bloom: failed to connect to any /dev/display/cardN after retries");
        loop {
            stem::sleep_ms(1000);
        }
    };
    stem::debug!("bloom.phase=get_info");
    let outputs = display.enumerate_outputs();
    if outputs.is_empty() {
        error!("bloom: no outputs enumerated");
        loop {
            stem::sleep_ms(1000);
        }
    }
    let primary = outputs[0];
    info!("Display ready at {}x{}", primary.width, primary.height);
    stem::debug!(
        "bloom: display details path={} refresh_mhz={} vblank={} dmabuf={} gpu_blit={} direct_scanout={} partial_flush={} resource_cache={} gpu_alpha={} gpu_scale={} rounded_clip={} fences={} accel2d_gpu={}",
        display_path,
        primary.refresh_mhz,
        display.supports_vblank(),
        primary.supports_dmabuf,
        display.supports_gpu_blit(),
        display.supports_direct_scanout(),
        display.supports_partial_flush(),
        display.supports_resource_cache(),
        display.supports_gpu_alpha_blend(),
        display.supports_gpu_scale(),
        display.supports_gpu_rounded_clip(),
        display.supports_fences(),
        display.supports_accel2d_gpu()
    );

    // ── Create and publish the service port ───────────────────────────────────
    stem::debug!("bloom: creating service port");
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
    session_fs::init();

    let mut visuals = CompositorVisuals::new();

    stem::debug!("bloom.phase=import_primary_buffer");
    let initial_theme = ensure_theme_config(THEME_PATH);
    let applied_theme = visuals.set_theme_by_name(&initial_theme);
    stem::debug!("bloom: initial theme configured {}", applied_theme);
    visuals.prepare_solid_background(&display, 0xFF0B0A10);

    // Write the theme's wallpaper path so blossom has it before it first paints.
    if !minimal_mode {
        let theme_obj = crate::theme::theme_by_name(&initial_theme);
        write_themed_wallpaper(WALLPAPER_CONFIG_PATH, theme_obj.wallpaper_path);
        stem::debug!("bloom: initial wallpaper path set to {}", theme_obj.wallpaper_path);
    }

    stem::debug!("bloom.phase=init_cursor");
    if !minimal_mode {
        visuals.prepare_busy_spinner(&display);
        stem::debug!("bloom: cursor init deferred until visual resources are ready");
    } else {
        stem::debug!("bloom: skipping cursor init (minimal mode)");
    }

    // ── Initial scene / damage / input state ─────────────────────────────────
    let scene = Scene::new();
    let mut damage = DamageTracker::new();
    damage.mark_full(primary.width, primary.height);
    stem::debug!("bloom.phase=first_damage width={} height={}", primary.width, primary.height);
    let input = InputState::new(primary.width, primary.height);

    // ── Bristle event port ────────────────────────────────────────────────────
    // Bloom keeps a read FD ready for bristle HID events and registers the
    // sink from the service loop after startup so first paint is independent
    // from Bristle readiness.
    let bristle_pair = port_create(65536).ok();

    // ── Theme watch FD ────────────────────────────────────────────────────────
    let theme_watch_fd = if !minimal_mode {
        match vfs_watch_path(THEME_PATH, abi::vfs_watch::mask::ALL_EVENTS, 0) {
            Ok(fd) => {
                stem::debug!("bloom: watching theme config {}", THEME_PATH);
                Some(fd)
            }
            Err(e) => {
                warn!("bloom: failed to watch theme config {}: {:?}", THEME_PATH, e);
                None
            }
        }
    } else {
        stem::debug!("bloom: skipping theme watch (minimal mode)");
        None
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

    // Wallpaper watch → blossom (wallpaper is now managed by blossom).
    // Deferred fonts / cursor → ResourceRetryService
    bloom_loop.add_service(alloc::boxed::Box::new(ResourceRetryService::new(None, !minimal_mode)));
    if !minimal_mode {
        bloom_loop.add_service(alloc::boxed::Box::new(BusySpinnerService::new()));
    }

    // Theme watch → ThemeService
    bloom_loop.add_service(alloc::boxed::Box::new(ThemeService::new(theme_watch_fd, THEME_PATH)));

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
                    let args = alloc::boxed::Box::new(WaylandThreadArgs {
                        cmd_write,
                        evt_read_fd,
                        output: primary,
                    });
                    let arg_ptr = alloc::boxed::Box::into_raw(args) as usize;
                    match stem::thread::spawn_with_arg(wayland::wayland_thread_entry, arg_ptr) {
                        Ok(_) => {
                            stem::debug!("Wayland server thread spawned");
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

fn connect_display_card() -> Option<(DisplayBackend, String)> {
    for card in 0..32u32 {
        let path = alloc::format!("/dev/display/card{}", card);
        if let Some(display) = DisplayBackend::connect(&path) {
            return Some((display, path));
        }
    }
    None
}

fn publish_service_handle(path: &str, handle: u32) {
    let _ = vfs_mkdir("/run/services");
    if let Ok(fd) = vfs_open(path, O_CREAT | O_TRUNC | O_RDWR) {
        let text = alloc::format!("{}\n", handle);
        let _ = vfs_write(fd, text.as_bytes());
        let _ = vfs_close(fd);
    }
}

/// Read `/dev/cmdline` and return `true` if the kernel command line contains
/// the exact token `bloom.minimal=1`.  This enables the no-wallpaper / no-cursor
/// boot mode which eliminates asset-loading from the first-frame path to help
/// isolate display-handoff stalls.
fn read_minimal_boot_mode() -> bool {
    let Ok(fd) = vfs_open("/dev/cmdline", O_RDONLY) else {
        return false;
    };
    let mut buf = [0u8; 512];
    let n = vfs_read(fd, &mut buf).unwrap_or_else(|e| {
        stem::warn!("bloom: failed to read /dev/cmdline: {:?}", e);
        0
    });
    let _ = vfs_close(fd);
    if n == 0 {
        return false;
    }
    let cmdline = match core::str::from_utf8(&buf[..n]) {
        Ok(s) => s,
        Err(_) => {
            stem::warn!("bloom: /dev/cmdline contains invalid UTF-8; ignoring bloom.minimal check");
            return false;
        }
    };
    // Match whole-token to avoid false positives like `bloom.minimal=10`.
    cmdline.split_whitespace().any(|token| token == "bloom.minimal=1")
}
