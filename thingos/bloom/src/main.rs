#![no_std]
#![no_main]

extern crate alloc;

mod damage;
mod display;
mod input;
mod protocol;
mod render;
mod scene;

use abi::syscall::vfs_flags::{O_CREAT, O_RDWR, O_TRUNC};
use damage::DamageTracker;
use display::DisplayBackend;
use input::InputState;
use protocol::{
    AckEvent, ClientRequest, EVT_ACK, EVT_FRAME_DONE, FrameDoneEvent, MessageHeader, parse_request,
    to_vec,
};
use render::CompositorVisuals;
use scene::{Scene, SurfaceBuffer};
use stem::syscall::vfs::{
    vfs_close, vfs_handle_from_port, vfs_mkdir, vfs_open, vfs_read, vfs_watch_fd, vfs_watch_path,
    vfs_write,
};
use stem::syscall::{port_create, port_send_all};
use stem::{error, info, warn};

const SERVICE_PATH: &str = "/services/bloom";

#[stem::main]
fn main(arg: usize) -> ! {
    info!("bloom: compositor service starting");

    let mut display_opt = None;
    for _ in 0..50 {
        display_opt = DisplayBackend::connect("/dev/display/card0");
        if display_opt.is_some() {
            break;
        }
        stem::sleep_ms(100);
    }

    let mut display = if let Some(d) = display_opt {
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
    let mut primary = outputs[0];
    info!("bloom: output0 {}x{} @ {}mHz", primary.width, primary.height, primary.refresh_mhz);

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

    let _ = vfs_mkdir("/session");
    let _ = vfs_mkdir("/session/desktop");
    let wp_path = "/session/desktop/wallpaper";

    let mut visuals = CompositorVisuals::new();
    visuals.prepare_background(&display, wp_path);
    // If first attempt failed (e.g. no wallpaper file), try fallback
    if visuals.fallback_buffer_id().is_none() {
        visuals.prepare_background(&display, "/share/wallpapers/flower.bmp");
    }

    let mut scene = Scene::new();
    let mut damage = DamageTracker::new();
    damage.mark_full(primary.width, primary.height);

    let mut input = InputState::new(primary.width, primary.height);
    let bristle_evt_read = decode_bristle_arg(arg as u64);
    if bristle_evt_read != 0 {
        info!("bloom: listening for bristle events on port {}", bristle_evt_read);
    } else {
        warn!("bloom: no bristle event port provided");
    }

    let service_fd = vfs_handle_from_port(service_read).ok();
    let bristle_fd =
        if bristle_evt_read != 0 { vfs_handle_from_port(bristle_evt_read).ok() } else { None };

    let mut ws = stem::wait_set::WaitSet::new();
    let service_token = service_fd.and_then(|fd| ws.add_fd_readable(fd).ok());
    let bristle_token = bristle_fd.and_then(|fd| ws.add_fd_readable(fd).ok());

    let wp_watch_fd = vfs_watch_path(wp_path, abi::vfs_watch::mask::ALL_EVENTS, 0).ok();
    let wp_watch_token = wp_watch_fd.and_then(|fd| ws.add_fd_readable(fd).ok());

    let mut needs_redraw = true;
    let mut io_buf = [0u8; 512];

    loop {
        if needs_redraw && damage.is_dirty() {
            let composition = scene.collect_composition();
            let pending_damage = damage.take();
            let present =
                display.present(&composition, &pending_damage, visuals.fallback_buffer_id());
            if present.success {
                let ts = stem::monotonic_ns();
                for entry in composition {
                    if let Some(client_id) = scene.surface_client(entry.surface_id) {
                        if let Some(ch) = scene.client_event_port(client_id) {
                            let done = FrameDoneEvent {
                                header: msg_header(EVT_FRAME_DONE),
                                surface_id: entry.surface_id,
                                serial: ts,
                                timestamp_ns: ts,
                            };
                            let _ = port_send_all(ch, &to_vec(&done));
                        }
                    }
                }
                needs_redraw = false;
            } else {
                damage.restore(pending_damage);
                needs_redraw = true;
            }
        }

        let events = match ws.wait(None::<stem::time::Duration>) {
            Ok(evs) => evs,
            Err(_) => {
                stem::sleep_ms(10);
                continue;
            }
        };

        for ev in events {
            if !ev.is_readable() {
                continue;
            }

            if Some(ev.token()) == service_token {
                if let Some(fd) = service_fd {
                    if let Ok(n) = vfs_read(fd, &mut io_buf) {
                        if n > 0 {
                            if process_client_message(
                                &io_buf[..n],
                                &display,
                                &mut scene,
                                &mut damage,
                                &mut needs_redraw,
                            ) {
                                needs_redraw = true;
                            }
                        }
                    }
                }
            } else if Some(ev.token()) == bristle_token {
                if let Some(fd) = bristle_fd {
                    if let Ok(n) = vfs_read(fd, &mut io_buf) {
                        if n > 0 {
                            input.handle_bristle_event(&io_buf[..n], &mut scene, &mut damage);
                            needs_redraw = true;
                        }
                    }
                }
            } else if Some(ev.token()) == wp_watch_token {
                // Drain watch events
                if let Some(fd) = wp_watch_fd {
                    let mut dump = [0u8; 1024];
                    let _ = vfs_read(fd, &mut dump);
                }

                info!("bloom: reacting to wallpaper change");

                visuals.prepare_background(&display, wp_path);
                if visuals.fallback_buffer_id().is_none() {
                    visuals.prepare_background(&display, "/share/wallpapers/flower.bmp");
                }

                damage.mark_full(primary.width, primary.height);
                needs_redraw = true;
            }
        }
    }
}

fn process_client_message(
    raw: &[u8],
    display: &DisplayBackend,
    scene: &mut Scene,
    damage: &mut DamageTracker,
    needs_redraw: &mut bool,
) -> bool {
    let Some(req) = parse_request(raw) else {
        return false;
    };

    match req {
        ClientRequest::Connect(req) => {
            let client_id = if req.event_port == 0 {
                0
            } else {
                scene.register_client(req.event_port, None)
            };
            send_ack(req.reply_port, 0, client_id, 0);
            client_id != 0
        }
        ClientRequest::ConnectInbox(req) => {
            let client_id = if req.event_port == 0 {
                0
            } else {
                let input_pid = if req.input_pid == 0 { None } else { Some(req.input_pid) };
                scene.register_client(req.event_port, input_pid)
            };
            send_ack(req.reply_port, 0, client_id, 0);
            client_id != 0
        }
        ClientRequest::CreateSurface(req) => {
            let Some(surface_id) = scene.create_surface(req.client_id) else {
                send_ack(req.reply_port, 1, 0, 0);
                return false;
            };
            send_ack(req.reply_port, 0, surface_id, 0);
            true
        }
        ClientRequest::DestroySurface(req) => {
            let Some(release_ids) = scene.destroy_surface(req.client_id, req.surface_id) else {
                send_ack(req.reply_port, 1, 0, 0);
                return false;
            };
            for id in release_ids {
                display.release_buffer(id);
            }
            send_ack(req.reply_port, 0, req.surface_id, 0);
            damage.mark_dirty();
            *needs_redraw = true;
            true
        }
        ClientRequest::AttachBuffer(req) => {
            let Some(buffer_id) = display.import_buffer(
                req.handle_thing,
                req.width,
                req.height,
                req.stride,
                req.format,
                req.modifier,
            ) else {
                send_ack(req.reply_port, 2, 0, 0);
                return false;
            };

            let old_pending = scene.attach_pending_buffer(
                req.client_id,
                req.surface_id,
                SurfaceBuffer {
                    buffer_id,
                    width: req.width,
                    height: req.height,
                    stride: req.stride,
                },
            );
            match old_pending {
                Some(Some(old_id)) => {
                    display.release_buffer(old_id);
                    send_ack(req.reply_port, 0, buffer_id, 0);
                    true
                }
                Some(None) => {
                    send_ack(req.reply_port, 0, buffer_id, 0);
                    true
                }
                None => {
                    display.release_buffer(buffer_id);
                    send_ack(req.reply_port, 1, 0, 0);
                    false
                }
            }
        }
        ClientRequest::Damage(req) => {
            if scene.damage_pending(req.client_id, req.surface_id, req.rect) {
                damage.mark_rect(req.rect);
                send_ack(req.reply_port, 0, 0, 0);
                true
            } else {
                send_ack(req.reply_port, 1, 0, 0);
                false
            }
        }
        ClientRequest::SetInputRegion(req) => {
            if scene.set_pending_input_region(req.client_id, req.surface_id, req.rect) {
                send_ack(req.reply_port, 0, 0, 0);
                true
            } else {
                send_ack(req.reply_port, 1, 0, 0);
                false
            }
        }
        ClientRequest::SetOpaqueRegion(req) => {
            if scene.set_pending_opaque_region(req.client_id, req.surface_id, req.rect) {
                send_ack(req.reply_port, 0, 0, 0);
                true
            } else {
                send_ack(req.reply_port, 1, 0, 0);
                false
            }
        }
        ClientRequest::SetDestRect(req) => {
            if scene.set_pending_dest_rect(req.client_id, req.surface_id, req.rect) {
                send_ack(req.reply_port, 0, 0, 0);
                true
            } else {
                send_ack(req.reply_port, 1, 0, 0);
                false
            }
        }
        ClientRequest::SetZOrder(req) => {
            if scene.set_pending_z_order(req.client_id, req.surface_id, req.z_order) {
                send_ack(req.reply_port, 0, 0, 0);
                true
            } else {
                send_ack(req.reply_port, 1, 0, 0);
                false
            }
        }
        ClientRequest::Commit(req) => {
            let Some(result) = scene.commit_surface(req.client_id, req.surface_id) else {
                send_ack(req.reply_port, 1, 0, 0);
                return false;
            };
            for id in result.released_buffer_ids {
                display.release_buffer(id);
            }
            send_ack(req.reply_port, 0, req.surface_id, result.frame_serial);
            if result.changed {
                damage.mark_dirty();
                *needs_redraw = true;
            }
            result.changed
        }
    }
}

fn publish_service_handle(path: &str, handle: u32) {
    let _ = vfs_mkdir("/services");
    if let Ok(fd) = vfs_open(path, O_CREAT | O_TRUNC | O_RDWR) {
        let text = alloc::format!("{}\n", handle);
        let _ = vfs_write(fd, text.as_bytes());
        let _ = vfs_close(fd);
    }
}

fn send_ack(reply_port: u32, status: u32, value: u32, serial: u64) {
    if reply_port == 0 {
        return;
    }
    let ack = AckEvent { header: msg_header(EVT_ACK), status, value, serial };
    let _ = port_send_all(reply_port, &to_vec(&ack));
}

fn decode_bristle_arg(arg: u64) -> u32 {
    if arg == 0 {
        0
    } else if arg <= u32::MAX as u64 {
        arg as u32
    } else {
        (arg & 0xFFFF) as u32
    }
}

fn msg_header(msg_type: u16) -> MessageHeader {
    protocol::msg_header(msg_type)
}
