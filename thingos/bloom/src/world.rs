//! `BloomWorld` — the single source of truth for all compositor state.
//!
//! Grouping the mutable state here lets every [`crate::loop_types::BloomService`]
//! receive a single `&mut BloomWorld` parameter instead of a long list of
//! individual references.  The convenience methods on `BloomWorld` handle the
//! cases where multiple fields must be borrowed simultaneously.

use alloc::vec::Vec;

use stem::syscall::port_send_all;

use crate::damage::DamageTracker;
use crate::display::{DisplayBackend, OutputInfo};
use crate::input::InputState;
use crate::protocol::{
    AckEvent, ClientRequest, EVT_ACK, EVT_FRAME_DONE, FrameDoneEvent, msg_header, parse_request,
    to_vec,
};
use crate::render::CompositorVisuals;
use crate::scene::{CommitResult, CompositionEntry, Scene, SurfaceBuffer};
use crate::session_fs;

/// All mutable compositor state owned by the main loop.
pub struct BloomWorld {
    pub scene: Scene,
    pub damage: DamageTracker,
    pub input: InputState,
    pub visuals: CompositorVisuals,
    pub display: DisplayBackend,
    pub primary: OutputInfo,
    pub vsync_enabled: bool,
    cursor_present_logged: bool,
    /// Port write handle to the Wayland server thread's event port, if running.
    pub wayland_evt_write: Option<u32>,
    /// The ID of the surface that was active during the last frame.
    last_active_id: Option<u32>,
}

impl BloomWorld {
    pub fn new(
        scene: Scene,
        damage: DamageTracker,
        input: InputState,
        visuals: CompositorVisuals,
        display: DisplayBackend,
        primary: OutputInfo,
    ) -> Self {
        let vsync_enabled = display.supports_vblank();
        Self {
            scene,
            damage,
            input,
            visuals,
            display,
            primary,
            vsync_enabled,
            cursor_present_logged: false,
            wayland_evt_write: None,
            last_active_id: None,
        }
    }

    /// Process one raw Wayland client message.
    ///
    /// Returns `true` when the scene changed and a redraw should be requested.
    pub fn handle_wayland_message(&mut self, raw: &[u8]) -> bool {
        let Some(req) = parse_request(raw) else {
            return false;
        };

        match req {
            ClientRequest::Connect(req) => {
                let client_id = if req.event_port == 0 {
                    0
                } else {
                    self.scene.register_client(req.event_port, None)
                };
                send_ack(req.reply_port, 0, client_id, 0);
                client_id != 0
            }
            ClientRequest::ConnectInbox(req) => {
                let client_id = if req.event_port == 0 {
                    0
                } else {
                    let input_pid = if req.input_pid == 0 { None } else { Some(req.input_pid) };
                    self.scene.register_client(req.event_port, input_pid)
                };
                send_ack(req.reply_port, 0, client_id, 0);
                client_id != 0
            }
            ClientRequest::CreateSurface(req) => {
                let Some(surface_id) = self.scene.create_surface(req.client_id) else {
                    send_ack(req.reply_port, 1, 0, 0);
                    return false;
                };
                send_ack(req.reply_port, 0, surface_id, 0);
                self.sync_wayland_session_fs(alloc::format!("surface_created id={}\n", surface_id));
                true
            }
            ClientRequest::DestroySurface(req) => {
                let client_id = req.client_id;
                let surface_id = req.surface_id;
                let Some(release_ids) = self.scene.destroy_surface(client_id, surface_id) else {
                    send_ack(req.reply_port, 1, 0, 0);
                    return false;
                };
                for id in release_ids {
                    self.display.release_buffer(id);
                }
                send_ack(req.reply_port, 0, surface_id, 0);
                self.damage.mark_dirty();
                self.remove_wayland_session_surface(
                    surface_id,
                    alloc::format!("surface_destroyed id={}\n", surface_id),
                );
                true
            }
            ClientRequest::AttachBuffer(req) => {
                let Some(buffer_id) = self.display.import_buffer(
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

                let old_pending = self.scene.attach_pending_buffer(
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
                        self.display.release_buffer(old_id);
                        send_ack(req.reply_port, 0, buffer_id, 0);
                        true
                    }
                    Some(None) => {
                        send_ack(req.reply_port, 0, buffer_id, 0);
                        true
                    }
                    None => {
                        self.display.release_buffer(buffer_id);
                        send_ack(req.reply_port, 1, 0, 0);
                        false
                    }
                }
            }
            ClientRequest::Damage(req) => {
                if self.scene.damage_pending(req.client_id, req.surface_id, req.rect) {
                    send_ack(req.reply_port, 0, 0, 0);
                    false
                } else {
                    send_ack(req.reply_port, 1, 0, 0);
                    false
                }
            }
            ClientRequest::SetInputRegion(req) => {
                if self.scene.set_pending_input_region(req.client_id, req.surface_id, req.rect) {
                    send_ack(req.reply_port, 0, 0, 0);
                    true
                } else {
                    send_ack(req.reply_port, 1, 0, 0);
                    false
                }
            }
            ClientRequest::SetOpaqueRegion(req) => {
                if self.scene.set_pending_opaque_region(req.client_id, req.surface_id, req.rect) {
                    send_ack(req.reply_port, 0, 0, 0);
                    true
                } else {
                    send_ack(req.reply_port, 1, 0, 0);
                    false
                }
            }
            ClientRequest::SetDestRect(req) => {
                if self.scene.set_pending_dest_rect(req.client_id, req.surface_id, req.rect) {
                    send_ack(req.reply_port, 0, 0, 0);
                    true
                } else {
                    send_ack(req.reply_port, 1, 0, 0);
                    false
                }
            }
            ClientRequest::SetZOrder(req) => {
                if self.scene.set_pending_z_order(req.client_id, req.surface_id, req.z_order) {
                    send_ack(req.reply_port, 0, 0, 0);
                    true
                } else {
                    send_ack(req.reply_port, 1, 0, 0);
                    false
                }
            }
            ClientRequest::Commit(req) => {
                let client_id = req.client_id;
                let surface_id = req.surface_id;
                let Some(result) = self.scene.commit_surface(client_id, surface_id) else {
                    send_ack(req.reply_port, 1, 0, 0);
                    return false;
                };
                for id in &result.released_buffer_ids {
                    self.display.release_buffer(*id);
                }
                send_ack(req.reply_port, 0, surface_id, result.frame_serial);
                self.apply_commit_damage(&result);
                self.sync_wayland_session_fs(alloc::format!(
                    "surface_committed id={} frame_serial={}\n",
                    surface_id,
                    result.frame_serial
                ));
                result.changed
            }
        }
    }

    pub fn apply_commit_damage(&mut self, result: &CommitResult) {
        if !result.changed {
            return;
        }
        if result.needs_full_repaint || result.damage_rects.is_empty() {
            self.damage.mark_full(self.primary.width, self.primary.height);
            return;
        }
        for rect in &result.damage_rects {
            self.damage.mark_rect(*rect);
        }
    }

    pub fn sync_wayland_session_fs(&self, event: alloc::string::String) {
        session_fs::sync_scene(&self.scene, &event);
    }

    pub fn remove_wayland_session_surface(&self, surface_id: u32, event: alloc::string::String) {
        session_fs::remove_surface(surface_id, &event);
        session_fs::sync_scene(&self.scene, &event);
    }

    pub fn apply_surface_visual_damage(
        &mut self,
        surface_id: u32,
        rect: abi::display_protocol::Rect,
    ) {
        let visual = self.scene.visual_rect_for_surface_rect(surface_id, rect).unwrap_or(rect);
        self.damage.mark_rect(visual);
    }

    /// Process one raw bristle HID event.
    pub fn handle_bristle_event(&mut self, data: &[u8]) -> bool {
        self.input.handle_bristle_event(
            data,
            &mut self.scene,
            &mut self.damage,
            self.wayland_evt_write,
        )
    }

    /// Send `FRAME_DONE` events to each client whose surface appeared in
    /// `composition`.
    pub fn send_frame_callbacks(&self, composition: &[CompositionEntry]) {
        let ts = stem::time::monotonic_ns();
        let timestamp_ms = (ts / 1_000_000) as u32;
        for entry in composition {
            if let Some(client_id) = self.scene.surface_client(entry.surface_id) {
                if let Some(ch) = self.scene.client_event_port(client_id) {
                    let done = FrameDoneEvent {
                        header: msg_header(EVT_FRAME_DONE),
                        surface_id: entry.surface_id,
                        serial: ts,
                        timestamp_ns: ts,
                    };
                    let _ = port_send_all(ch, &to_vec(&done));
                }
            }
            // Notify the Wayland server thread so it can fire wl_callback.done.
            if let Some(evt_write) = self.wayland_evt_write {
                let msg = crate::wayland::ipc::encode_frame_done(entry.surface_id, timestamp_ms);
                let _ = port_send_all(evt_write, &msg);
            }
        }
    }

    /// Collect the current scene into a sorted composition list and attempt to
    /// present it.  Returns the composition list on success so frame callbacks
    /// can be sent, or `None` on failure (damage is restored internally).
    pub fn try_present(&mut self) -> Option<Vec<CompositionEntry>> {
        let composition = self.scene.collect_composition();
        // Flush coalesced pointer motion: deliver the latest position to
        // clients once per frame rather than per raw sample.
        self.input.flush_pointer_motion(&mut self.scene, self.wayland_evt_write);
        self.input.flush_resizes(self.wayland_evt_write);
        self.input.flush_visible_pointer(&mut self.damage);

        // Damage check: if the active window changed since the last frame,
        // we must damage both the old and new active windows so their
        // chrome (titlebars, etc) can be updated.
        let active_id = composition.iter().find(|e| e.active).map(|e| e.surface_id);
        if active_id != self.last_active_id {
            if let Some(id) = self.last_active_id {
                if let Some(rect) = self.scene.surface_rect(id) {
                    self.apply_surface_visual_damage(id, rect);
                }
            }
            if let Some(id) = active_id {
                if let Some(rect) = self.scene.surface_rect(id) {
                    self.apply_surface_visual_damage(id, rect);
                }
            }
            self.last_active_id = active_id;
        }

        let pending_damage = self.damage.take();
        let (pointer_x, pointer_y) = self.input.visible_pointer_position();
        let pointer_overlay = if self.input.pointer_overlay_enabled() {
            self.visuals.pointer_overlay_plane(&self.display, pointer_x, pointer_y)
        } else {
            None
        };
        let cursor_kind = self.input.visible_cursor_kind();
        let chrome_overlay = self.visuals.chrome_overlay_plane(&self.display, &composition);
        let cursor = self.visuals.cursor_plane(&self.display, cursor_kind, pointer_x, pointer_y);

        let mut flags = if self.vsync_enabled {
            abi::display::CommitFlags::VSYNC
        } else {
            abi::display::CommitFlags::empty()
        };
        if self.input.is_resizing() {
            flags.remove(abi::display::CommitFlags::VSYNC);
        }

        let result = self.display.present(
            &composition,
            &pending_damage,
            self.visuals.fallback_buffer_id(),
            chrome_overlay,
            pointer_overlay,
            cursor,
            flags,
        );
        if result.success {
            if !self.cursor_present_logged {
                if let Some(cursor) = cursor {
                    stem::debug!(
                        "bloom: presented cursor buffer={} at {},{} size={}x{}",
                        cursor.buffer_id,
                        cursor.x,
                        cursor.y,
                        cursor.width,
                        cursor.height
                    );
                } else {
                    stem::warn!("bloom: presented without a cursor buffer");
                }
                self.cursor_present_logged = true;
            }
            Some(composition)
        } else {
            self.damage.restore(pending_damage);
            None
        }
    }
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn send_ack(reply_port: u32, status: u32, value: u32, serial: u64) {
    if reply_port == 0 {
        return;
    }
    let ack = AckEvent { header: msg_header(EVT_ACK), status, value, serial };
    let _ = port_send_all(reply_port, &to_vec(&ack));
}
