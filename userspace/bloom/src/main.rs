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
    vfs_close, vfs_mkdir, vfs_open, vfs_read, vfs_thing_from_channel, vfs_write,
};
use stem::syscall::{channel_create, channel_send_all};
use stem::{error, info, warn};

const SERVICE_PATH: &str = "/services/bloom";

#[stem::main]
fn main(arg: usize) -> ! {
    info!("bloom: compositor service starting");

    let Some(display) = DisplayBackend::connect("/dev/display/card0") else {
        error!("bloom: failed to open /dev/display/card0");
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
    info!(
        "bloom: output0 {}x{} @ {}mHz",
        primary.width, primary.height, primary.refresh_mhz
    );

    let (service_write, service_read) = match channel_create(65536) {
        Ok(pair) => pair,
        Err(e) => {
            error!("bloom: failed to create service channel: {:?}", e);
            loop {
                stem::sleep_ms(1000);
            }
        }
    };
    publish_service_handle(SERVICE_PATH, service_write);

    let mut visuals = CompositorVisuals::new();
    visuals.prepare_background(&display, "/share/wallpapers/flower.bmp");

    let mut scene = Scene::new();
    let mut damage = DamageTracker::new();
    damage.mark_full(primary.width, primary.height);

    let mut input = InputState::new(primary.width, primary.height);
    let bristle_evt_read = decode_bristle_arg(arg as u64);
    if bristle_evt_read != 0 {
        info!("bloom: listening for bristle events on channel {}", bristle_evt_read);
    } else {
        warn!("bloom: no bristle event channel provided");
    }

    let service_fd = vfs_thing_from_channel(service_read).ok();
    let bristle_fd = if bristle_evt_read != 0 {
        vfs_thing_from_channel(bristle_evt_read).ok()
    } else {
        None
    };

    let mut ws = stem::wait_set::WaitSet::new();
    let service_token = service_fd.and_then(|fd| ws.add_fd_readable(fd).ok());
    let bristle_token = bristle_fd.and_then(|fd| ws.add_fd_readable(fd).ok());

    let mut needs_redraw = true;
    let mut io_buf = [0u8; 512];

    loop {
        if needs_redraw && damage.is_dirty() {
            let composition = scene.collect_composition();
            let pending_damage = damage.take();
            let present = display.present(&composition, &pending_damage, visuals.fallback_buffer_id());
            if present.success {
                let ts = stem::monotonic_ns();
                for entry in composition {
                    if let Some(client_id) = scene.surface_client(entry.surface_id) {
                        if let Some(ch) = scene.client_event_channel(client_id) {
                            let done = FrameDoneEvent {
                                header: msg_header(EVT_FRAME_DONE),
                                surface_id: entry.surface_id,
                                serial: ts,
                                timestamp_ns: ts,
                            };
                            let _ = channel_send_all(ch, &to_vec(&done));
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
            let client_id = if req.event_channel == 0 {
                0
            } else {
                scene.register_client(req.event_channel)
            };
            send_ack(req.reply_channel, 0, client_id, 0);
            client_id != 0
        }
        ClientRequest::CreateSurface(req) => {
            let Some(surface_id) = scene.create_surface(req.client_id) else {
                send_ack(req.reply_channel, 1, 0, 0);
                return false;
            };
            send_ack(req.reply_channel, 0, surface_id, 0);
            true
        }
        ClientRequest::DestroySurface(req) => {
            let Some(release_ids) = scene.destroy_surface(req.client_id, req.surface_id) else {
                send_ack(req.reply_channel, 1, 0, 0);
                return false;
            };
            for id in release_ids {
                display.release_buffer(id);
            }
            send_ack(req.reply_channel, 0, req.surface_id, 0);
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
                send_ack(req.reply_channel, 2, 0, 0);
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
                    send_ack(req.reply_channel, 0, buffer_id, 0);
                    true
                }
                Some(None) => {
                    send_ack(req.reply_channel, 0, buffer_id, 0);
                    true
                }
                None => {
                    display.release_buffer(buffer_id);
                    send_ack(req.reply_channel, 1, 0, 0);
                    false
                }
            }
        }
        ClientRequest::Damage(req) => {
            if scene.damage_pending(req.client_id, req.surface_id, req.rect) {
                damage.mark_rect(req.rect);
                send_ack(req.reply_channel, 0, 0, 0);
                true
            } else {
                send_ack(req.reply_channel, 1, 0, 0);
                false
            }
        }
        ClientRequest::SetInputRegion(req) => {
            if scene.set_pending_input_region(req.client_id, req.surface_id, req.rect) {
                send_ack(req.reply_channel, 0, 0, 0);
                true
            } else {
                send_ack(req.reply_channel, 1, 0, 0);
                false
            }
        }
        ClientRequest::SetOpaqueRegion(req) => {
            if scene.set_pending_opaque_region(req.client_id, req.surface_id, req.rect) {
                send_ack(req.reply_channel, 0, 0, 0);
                true
            } else {
                send_ack(req.reply_channel, 1, 0, 0);
                false
            }
        }
        ClientRequest::SetDestRect(req) => {
            if scene.set_pending_dest_rect(req.client_id, req.surface_id, req.rect) {
                send_ack(req.reply_channel, 0, 0, 0);
                true
            } else {
                send_ack(req.reply_channel, 1, 0, 0);
                false
            }
        }
        ClientRequest::SetZOrder(req) => {
            if scene.set_pending_z_order(req.client_id, req.surface_id, req.z_order) {
                send_ack(req.reply_channel, 0, 0, 0);
                true
            } else {
                send_ack(req.reply_channel, 1, 0, 0);
                false
            }
        }
        ClientRequest::Commit(req) => {
            let Some(result) = scene.commit_surface(req.client_id, req.surface_id) else {
                send_ack(req.reply_channel, 1, 0, 0);
                return false;
            };
            for id in result.released_buffer_ids {
                display.release_buffer(id);
            }
            send_ack(req.reply_channel, 0, req.surface_id, result.frame_serial);
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

fn send_ack(reply_channel: u32, status: u32, value: u32, serial: u64) {
    if reply_channel == 0 {
        return;
    }
    let ack = AckEvent {
        header: msg_header(EVT_ACK),
        status,
        value,
        serial,
    };
    let _ = channel_send_all(reply_channel, &to_vec(&ack));
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
