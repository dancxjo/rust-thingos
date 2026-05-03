//! `BloomWorld` — the single source of truth for all compositor state.
//!
//! Grouping the mutable state here lets every [`crate::loop_types::BloomService`]
//! receive a single `&mut BloomWorld` parameter instead of a long list of
//! individual references.  The convenience methods on `BloomWorld` handle the
//! cases where multiple fields must be borrowed simultaneously.

use alloc::string::String;
use alloc::vec::Vec;

use stem::syscall::port_send_all;
use stem::time::monotonic_ns;

use crate::cache::ResourceCache;
use crate::compositor::cull_composition;
use crate::damage::DamageTracker;
use crate::display::{DisplayBackend, OutputInfo};
use crate::input::InputState;
use crate::protocol::{
    AckEvent, ClientRequest, EVT_ACK, EVT_FRAME_DONE, FrameDoneEvent, msg_header, parse_request,
    to_vec,
};
use crate::render::CompositorVisuals;
use crate::scene::{ClosedSurface, CommitResult, CompositionEntry, Scene, SurfaceBuffer};
use crate::session_fs;

const ENABLE_HARDWARE_CURSOR: bool = false;
const SESSION_FS_SYNC_INTERVAL_NS: u64 = 1_000_000_000;
const BUSY_SPINNER_STABLE_SCENE_FRAMES: u8 = 2;

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
    /// Client buffer import cache and wallpaper/shadow-atlas slots.
    pub cache: ResourceCache,
    /// The buffer ID of the cursor image last uploaded to the hardware cursor
    /// plane via `DISPLAY_OP_SET_CURSOR`.  `None` means the hardware cursor has
    /// not been initialised yet this session.
    hw_cursor_buffer: Option<u32>,
    hw_cursor_position: Option<(i32, i32, bool)>,
    /// Port write handle to the Wayland server thread's event port, if running.
    pub wayland_evt_write: Option<u32>,
    /// The ID of the surface that was active during the last frame.
    last_active_id: Option<u32>,
    /// Frame counter used to schedule periodic cache-stats logging.
    frame_count: u64,
    busy_spinner_handoff_armed: bool,
    busy_spinner_stable_frames: u8,
    busy_spinner_stable_logged: bool,
    pending_session_event: Option<String>,
    next_session_sync_ns: u64,
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
        let mut damage = damage;
        // Bind the tracker to the primary output so `take()` can clip and
        // coalesce damage rects (and fall back to full-output damage when the
        // rect count exceeds `damage::MAX_DAMAGE_RECTS`).
        damage.set_output_bounds(primary.width, primary.height);
        Self::send_resolution_to_bristle(primary.width, primary.height);

        Self {
            scene,
            damage,
            input,
            visuals,
            display,
            primary,
            vsync_enabled,
            cursor_present_logged: false,
            cache: ResourceCache::new(),
            hw_cursor_buffer: None,
            hw_cursor_position: None,
            wayland_evt_write: None,
            last_active_id: None,
            frame_count: 0,
            busy_spinner_handoff_armed: false,
            busy_spinner_stable_frames: 0,
            busy_spinner_stable_logged: false,
            pending_session_event: None,
            next_session_sync_ns: 0,
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
                if let Some(rect) = self.scene.surface_visual_rect(surface_id) {
                    self.damage.mark_rect(rect);
                }
                let Some(release_ids) = self.scene.destroy_surface(client_id, surface_id) else {
                    send_ack(req.reply_port, 1, 0, 0);
                    return false;
                };
                for id in release_ids {
                    self.cache.release_client_buffer(&self.display, id);
                }
                send_ack(req.reply_port, 0, surface_id, 0);
                self.remove_wayland_session_surface(
                    surface_id,
                    alloc::format!("surface_destroyed id={}\n", surface_id),
                );
                true
            }
            ClientRequest::AttachBuffer(req) => {
                // Use the resource cache: if the same handle with the same
                // dimensions and format was already imported and has not been
                // released since, reuse the existing buffer_id (cache hit).
                let Some(buffer_id) = self.cache.import_client_buffer(
                    &self.display,
                    req.handle_thing,
                    req.width,
                    req.height,
                    req.stride,
                    req.format,
                    0,
                    req.modifier,
                    req.generation,
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
                        // The pending slot previously held `old_id`.  When the
                        // resource cache returns the *same* buffer_id for the
                        // same handle (cache hit), `old_id == buffer_id` and
                        // there is nothing to release — the buffer is simply
                        // being re-attached without content change.  Only
                        // release when they differ (e.g. the client replaced
                        // one buffer with a different one before committing).
                        if old_id != buffer_id {
                            self.cache.release_client_buffer(&self.display, old_id);
                        }
                        send_ack(req.reply_port, 0, buffer_id, 0);
                        true
                    }
                    Some(None) => {
                        // No previously pending buffer; nothing to release.
                        send_ack(req.reply_port, 0, buffer_id, 0);
                        true
                    }
                    None => {
                        // Surface not found; release the just-imported buffer.
                        self.cache.release_client_buffer(&self.display, buffer_id);
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
                let Some(result) = self.scene.commit_surface(
                    client_id,
                    surface_id,
                    self.primary.width,
                    self.primary.height,
                ) else {
                    send_ack(req.reply_port, 1, 0, 0);
                    return false;
                };
                for id in &result.released_buffer_ids {
                    self.cache.release_client_buffer(&self.display, *id);
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

    pub fn check_resource_retries(&mut self) -> bool {
        let mut status = self.visuals.retry_deferred_resources(&self.display, None, true, true);
        if status.improved {
            self.damage.mark_full(self.primary.width, self.primary.height);
        }
        if status.improved && !self.visuals.cursor_is_fallback() {
            status.improved |= self.replay_deferred_cursor_motion();
        }
        status.improved
    }

    fn send_resolution_to_bristle(w: u32, h: u32) {
        if let Ok(fd) =
            stem::syscall::vfs::vfs_open("/run/bristle/control", abi::syscall::vfs_flags::O_RDONLY)
        {
            let mut buf = [0u8; 16];
            if let Ok(n) = stem::syscall::vfs::vfs_read(fd, &mut buf) {
                let _ = stem::syscall::vfs::vfs_close(fd);
                if let Ok(s) = core::str::from_utf8(&buf[..n]) {
                    if let Ok(port) = s.trim().parse::<u32>() {
                        let mut res_buf = [0u8; 8];
                        res_buf[0..4].copy_from_slice(&w.to_le_bytes());
                        res_buf[4..8].copy_from_slice(&h.to_le_bytes());
                        let _ = stem::syscall::port_send_all(port, &res_buf);
                    }
                }
            } else {
                let _ = stem::syscall::vfs::vfs_close(fd);
            }
        }
    }

    pub fn refresh_display_output(&mut self) -> bool {
        let old = self.primary;
        let Some(changed) = self.display.refresh_info_changed() else {
            return false;
        };
        if !changed {
            return false;
        }

        let next = self.display.primary_output_info();
        if next.width == 0 || next.height == 0 {
            return false;
        }

        self.primary = next;
        self.vsync_enabled = self.display.supports_vblank();
        self.damage.set_output_bounds(next.width, next.height);
        self.input.update_dimensions(next.width, next.height);
        Self::send_resolution_to_bristle(next.width, next.height);
        self.visuals.reconfigure_for_output(&self.display);
        self.hw_cursor_position = None;
        self.cursor_present_logged = false;
        self.damage.mark_full(next.width, next.height);
        if let Some(port) = self.wayland_evt_write {
            let msg =
                crate::wayland::ipc::encode_output_info(next.width, next.height, next.refresh_mhz);
            let _ = port_send_all(port, &msg);
        }
        stem::info!(
            "Display resized from {}x{} to {}x{} @ {}mHz.",
            old.width,
            old.height,
            next.width,
            next.height,
            next.refresh_mhz
        );
        true
    }

    pub fn sync_wayland_session_fs(&mut self, event: alloc::string::String) {
        self.pending_session_event = Some(event);
        if self.next_session_sync_ns == 0 {
            self.next_session_sync_ns = monotonic_ns().saturating_add(SESSION_FS_SYNC_INTERVAL_NS);
        }
    }

    pub fn remove_wayland_session_surface(
        &mut self,
        surface_id: u32,
        event: alloc::string::String,
    ) {
        session_fs::remove_surface(surface_id, &event);
        self.sync_wayland_session_fs(event);
    }

    pub fn next_session_fs_sync_delay_ns(&self) -> Option<u64> {
        self.pending_session_event.as_ref()?;
        Some(self.next_session_sync_ns.saturating_sub(monotonic_ns()))
    }

    pub fn publish_wayland_session_fs_if_due(&mut self) {
        if self.pending_session_event.is_none() || monotonic_ns() < self.next_session_sync_ns {
            return;
        }
        let event = self.pending_session_event.take().unwrap_or_default();
        session_fs::sync_scene_summary(&self.scene, &event);
        self.next_session_sync_ns = 0;
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
        let defer_cursor_motion = self.visuals.cursor_is_fallback();
        self.input.handle_bristle_event(
            data,
            &mut self.scene,
            &mut self.damage,
            self.wayland_evt_write,
            defer_cursor_motion,
        )
    }

    pub fn replay_deferred_cursor_motion(&mut self) -> bool {
        self.input.replay_deferred_cursor_motion(
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
        self.input.flush_pointer_grab(&mut self.scene, &mut self.damage, self.wayland_evt_write);
        let now_ns = monotonic_ns();
        self.mark_dramatic_close_damage(now_ns);
        let composition = self.scene.collect_composition_at(now_ns);
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

        let (pointer_x, pointer_y) = self.input.visible_pointer_position();
        let cursor_kind = self.input.visible_cursor_kind();
        let cursor = self.visuals.cursor_plane(&self.display, cursor_kind, pointer_x, pointer_y);
        let use_hw_cursor = ENABLE_HARDWARE_CURSOR && self.display.supports_hw_cursor();

        // ── Hardware cursor fast path ──────────────────────────────────────
        // When the display driver supports hardware cursor planes and there is
        // no window/content damage (only cursor position changed), skip full
        // scene recomposition and issue a lightweight cursor position update
        // instead. Frames with content damage reassert the cursor after the
        // framebuffer commit so the scanout update cannot cover it.
        if use_hw_cursor {
            // (Re-)upload the cursor image whenever the buffer changes.
            if let Some(c) = cursor {
                if self.hw_cursor_buffer != Some(c.buffer_id) {
                    if self.display.set_cursor_image(
                        c.buffer_id,
                        c.width,
                        c.height,
                        c.hotspot_x,
                        c.hotspot_y,
                        true,
                    ) {
                        stem::debug!(
                            "bloom: hw cursor image set buffer={} size={}x{} hotspot={},{}",
                            c.buffer_id,
                            c.width,
                            c.height,
                            c.hotspot_x,
                            c.hotspot_y,
                        );
                        self.hw_cursor_buffer = Some(c.buffer_id);
                        self.hw_cursor_position = None;
                    }
                }
            }

            let cursor_visible = cursor.is_some();
            let wanted_cursor_position = (pointer_x, pointer_y, cursor_visible);
            let mut hw_cursor_positioned = self.hw_cursor_position == Some(wanted_cursor_position);
            if self.hw_cursor_buffer.is_some() && !hw_cursor_positioned {
                hw_cursor_positioned =
                    self.display.move_cursor(pointer_x, pointer_y, cursor_visible);
                if hw_cursor_positioned {
                    self.hw_cursor_position = Some(wanted_cursor_position);
                    stem::trace!("bloom: hw cursor move {},{}", pointer_x, pointer_y);
                } else {
                    stem::warn!("bloom: hw cursor move failed");
                }
            }

            // Cursor-only update: no content damage, only cursor motion.
            if self.damage.has_only_cursor_damage() {
                // Drain cursor damage — we only care about the side-effect of
                // resetting the tracker's dirty flag; the individual rects are
                // not needed because the hardware cursor is positioned directly
                // via DISPLAY_OP_MOVE_CURSOR without a framebuffer upload.
                // Reset dirty flag; cursor positioning is direct via hardware.
                let _ = self.damage.take(); // Side-effect only: clears dirty state.
                if hw_cursor_positioned {
                    stem::trace!(
                        "bloom: hw cursor move {},{} (no recompose)",
                        pointer_x,
                        pointer_y
                    );
                    // Return an empty composition so the caller sends no frame
                    // callbacks (no surface was recomposed this frame).
                    return Some(alloc::vec![]);
                }
                // Move failed — fall through to the full compose path and
                // restore dirty so we retry on the next frame.
                self.damage.mark_full(self.primary.width, self.primary.height);
            }
        }

        let pending_damage = self.damage.take();
        let pointer_overlay = if self.input.pointer_overlay_enabled() {
            self.visuals.pointer_overlay_plane(&self.display, pointer_x, pointer_y)
        } else {
            None
        };

        // When hardware cursor is active the cursor plane is handled
        // independently; pass `None` to the software compositor so it is not
        // also blended as a plane.
        let compositor_cursor =
            if use_hw_cursor && self.hw_cursor_buffer.is_some() { None } else { cursor };

        let (body_overlay, chrome_overlay) = self.visuals.chrome_overlay_plane(
            &self.display,
            &composition,
            pointer_x,
            pointer_y,
            self.input.primary_button_down(),
        );

        let mut flags = if self.vsync_enabled {
            abi::display::CommitFlags::VSYNC
        } else {
            abi::display::CommitFlags::empty()
        };
        if self.input.is_resizing() {
            flags.remove(abi::display::CommitFlags::VSYNC);
        }

        // Apply opaque-region culling: remove planes fully hidden by opaque
        // planes above them, and log aggregate debug counters.
        let (culled_composition, cull_counters) =
            cull_composition(&composition, self.primary.width, self.primary.height);
        if cull_counters.hidden_regions_skipped > 0 || cull_counters.fullscreen_direct_present > 0 {
            stem::trace!(
                "bloom: cull opaque={} blend={} skipped={} fullscreen={}",
                cull_counters.opaque_planes_copied,
                cull_counters.alpha_blend_planes,
                cull_counters.hidden_regions_skipped,
                cull_counters.fullscreen_direct_present,
            );
        }

        let result = self.display.present(
            &culled_composition,
            &pending_damage,
            self.visuals.fallback_buffer_id(),
            &body_overlay,
            &chrome_overlay,
            pointer_overlay,
            compositor_cursor,
            flags,
            self.visuals.corner_radius(),
        );
        if result.success {
            self.frame_count = self.frame_count.wrapping_add(1);
            // Log cache stats once every 300 frames (~5 s at 60 fps).
            if self.frame_count % 300 == 0 {
                self.cache.log_stats();
            }
            if !self.cursor_present_logged {
                if self.display.supports_hw_cursor() && self.hw_cursor_buffer.is_some() {
                    stem::debug!("bloom: hw cursor active, software cursor plane omitted",);
                } else if let Some(c) = cursor {
                    stem::debug!(
                        "bloom: presented cursor buffer={} at {},{} size={}x{}",
                        c.buffer_id,
                        c.x,
                        c.y,
                        c.width,
                        c.height
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

    pub fn start_dramatic_surface_close(&mut self, surface_id: u32) -> bool {
        let now_ns = monotonic_ns();
        let Some(rect) = self.scene.start_dramatic_close(surface_id, now_ns) else {
            return false;
        };
        self.damage.mark_rect(rect);
        stem::info!("bloom: dramatic close started surface={}", surface_id);
        self.sync_wayland_session_fs(alloc::format!(
            "surface_dramatic_close_started id={}\n",
            surface_id
        ));
        true
    }

    pub fn has_dramatic_surface_closes(&self) -> bool {
        self.scene.has_dramatic_closes()
    }

    pub fn tick_dramatic_surface_closes(&mut self) -> (bool, Vec<ClosedSurface>) {
        let now_ns = monotonic_ns();
        self.mark_dramatic_close_damage(now_ns);
        let closed = self.scene.take_finished_dramatic_closes(now_ns);
        for surface in &closed {
            self.damage.mark_rect(surface.visual_rect);
            self.remove_wayland_session_surface(
                surface.surface_id,
                alloc::format!("surface_destroyed id={}\n", surface.surface_id),
            );
            stem::info!("bloom: dramatic close finished surface={}", surface.surface_id);
        }
        (self.scene.has_dramatic_closes(), closed)
    }

    pub fn tick_busy_spinner(&mut self, frame: u32) -> bool {
        self.tick_busy_spinner_with_opacity(frame, 255)
    }

    pub fn tick_busy_spinner_with_opacity(&mut self, frame: u32, opacity: u8) -> bool {
        let Some((width, height, hotspot_x, hotspot_y)) =
            self.visuals.draw_busy_spinner_frame(frame, opacity)
        else {
            return false;
        };
        let (pointer_x, pointer_y) = self.input.visible_pointer_position();
        self.damage.mark_cursor_rect(abi::display_protocol::Rect {
            x: pointer_x.saturating_sub(hotspot_x as i32).max(0) as u32,
            y: pointer_y.saturating_sub(hotspot_y as i32).max(0) as u32,
            w: width,
            h: height,
        });
        self.hw_cursor_buffer = None;
        true
    }

    pub fn arm_busy_spinner_handoff(&mut self) {
        if self.busy_spinner_handoff_armed {
            return;
        }
        self.busy_spinner_handoff_armed = true;
        self.busy_spinner_stable_frames = 0;
        self.busy_spinner_stable_logged = false;
        stem::info!("bloom: busy spinner waiting for stable scene");
    }

    pub fn note_presented_frame(&mut self) {
        if !self.busy_spinner_handoff_armed
            || !self.visuals.cursor_is_fallback()
            || !self.visuals.pending_cursor_ready()
        {
            return;
        }
        if self.busy_spinner_stable_frames < BUSY_SPINNER_STABLE_SCENE_FRAMES {
            self.busy_spinner_stable_frames = self.busy_spinner_stable_frames.saturating_add(1);
        }
        if self.busy_spinner_stable_frames >= BUSY_SPINNER_STABLE_SCENE_FRAMES
            && !self.busy_spinner_stable_logged
        {
            self.busy_spinner_stable_logged = true;
            stem::info!(
                "bloom: busy spinner scene stable after {} frame(s)",
                self.busy_spinner_stable_frames
            );
        }
    }

    pub fn busy_spinner_can_fade(&self) -> bool {
        self.busy_spinner_handoff_armed
            && self.busy_spinner_stable_frames >= BUSY_SPINNER_STABLE_SCENE_FRAMES
            && self.visuals.cursor_is_fallback()
            && self.visuals.pending_cursor_ready()
    }

    pub fn finish_busy_spinner_handoff(&mut self) -> bool {
        if !self.visuals.activate_pending_cursor(&self.display) {
            return false;
        }
        self.busy_spinner_handoff_armed = false;
        self.busy_spinner_stable_frames = 0;
        self.busy_spinner_stable_logged = false;
        self.cursor_present_logged = false;
        self.hw_cursor_buffer = None;
        let (pointer_x, pointer_y) = self.input.visible_pointer_position();
        self.damage.mark_cursor_rect(abi::display_protocol::Rect {
            x: pointer_x.saturating_sub(48).max(0) as u32,
            y: pointer_y.saturating_sub(48).max(0) as u32,
            w: 96,
            h: 96,
        });
        let _ = self.replay_deferred_cursor_motion();
        true
    }

    fn mark_dramatic_close_damage(&mut self, now_ns: u64) {
        if self.scene.dramatic_close_visual_rects(now_ns).is_empty() {
            return;
        }
        self.damage.mark_full(self.primary.width, self.primary.height);
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
