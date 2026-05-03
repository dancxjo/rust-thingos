use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

use abi::hid::{
    BristleEventHeader, EventType, Key, KeyEventPayload, PointerButtonPayload, PointerMovePayload,
    ScrollPayload,
};
use abi::{KindId, ui_event};
use stem::syscall::message::msg_send;
use stem::syscall::port_send_all;
use stem::syscall::vfs::{vfs_close, vfs_open};

use crate::damage::DamageTracker;
use crate::protocol::{
    EVT_KEYBOARD_ENTER, EVT_KEYBOARD_KEY, EVT_KEYBOARD_LEAVE, EVT_POINTER_BUTTON,
    EVT_POINTER_ENTER, EVT_POINTER_LEAVE, EVT_POINTER_MOTION, KIND_KEYBOARD_ENTER,
    KIND_KEYBOARD_KEY, KIND_KEYBOARD_LEAVE, KIND_POINTER_BUTTON, KIND_POINTER_ENTER,
    KIND_POINTER_LEAVE, KIND_POINTER_MOTION, KIND_UI_EVENT, KeyboardEnterEvent, KeyboardKeyEvent,
    KeyboardLeaveEvent, PointerButtonEvent, PointerEnterEvent, PointerLeaveEvent,
    PointerMotionEvent, msg_header, to_vec,
};
use crate::scene::{ChromeButton, CursorKind, HitTarget, ResizeEdge, Scene};
use crate::wayland::ipc;

const CURSOR_DAMAGE_W: u32 = 192;
const CURSOR_DAMAGE_H: u32 = 192;
const CURSOR_HOTSPOT_X: i32 = 96;
const CURSOR_HOTSPOT_Y: i32 = 96;
const MIN_RESIZE_W: u32 = 160;
const MIN_RESIZE_H: u32 = 96;
static POINTER_MOVE_LOGS: AtomicU32 = AtomicU32::new(0);
static CURSOR_SMOOTHING_LOGS: AtomicU32 = AtomicU32::new(0);
/// Counts every raw PointerMove event received (pre-coalesce).
static COALESCE_PRE: AtomicU32 = AtomicU32::new(0);
/// Limits how many times coalesce-flush stats are logged.
static COALESCE_FLUSH_LOGS: AtomicU32 = AtomicU32::new(0);
static BLOOM_HANDLE_EVENT_COUNT: AtomicU64 = AtomicU64::new(0);
/// Maximum number of times per-category debug stats are logged at startup.
const MAX_STARTUP_LOGS: u32 = 8;
const INPUT_TRACE_INITIAL: u64 = 24;
const INPUT_TRACE_INTERVAL: u64 = 128;

pub struct InputState {
    /// Latest logical pointer position from Bristle input. Clients and focus
    /// tracking see this immediately so input latency stays low.
    pointer_x: i32,
    pointer_y: i32,
    /// Position of the cursor plane from the last successful present. Raw input
    /// can advance `pointer_*` many times between frames; this advances once in
    /// the present path so cursor commits stay frame-paced.
    visible_x: i32,
    visible_y: i32,
    pending_cursor_motion: bool,
    deferred_cursor_target: Option<(i32, i32)>,
    deferred_cursor_latest_ts: Option<u64>,
    deferred_cursor_events: u32,
    /// Timestamp of the most recent coalesced PointerMove event awaiting
    /// delivery to clients.  Multiple raw motion samples between frames are
    /// collapsed here so focus lookup and client delivery happen only once per
    /// frame rather than per sample.  `None` means no motion since last flush.
    pending_motion_ts: Option<u64>,
    pointer_overlay_enabled: bool,
    primary_button_down: bool,
    pointer_grab: Option<PointerGrab>,
    cursor_kind: CursorKind,
    visible_cursor_kind: CursorKind,
    keyboard_modifiers: u8,
    output_w: i32,
    output_h: i32,
    /// Pending resize event to be sent to a Wayland client on the next frame boundary.
    /// Stores (surface_id, width, height).
    pending_resize: Option<(u32, u32, u32)>,
    /// Whether we have already sent a resize event to a client during the current frame.
    resize_sent_this_frame: bool,
    runbox: blossom::runbox::RunState,
    runbox_keyboard_modal: bool,
}

#[derive(Clone, Copy, Debug)]
struct PointerGrab {
    surface_id: u32,
    kind: PointerGrabKind,
}

#[derive(Clone, Copy, Debug)]
enum PointerGrabKind {
    Move { offset_x: i32, offset_y: i32 },
    Resize { edge: ResizeEdge, start_rect: abi::display_protocol::Rect, start_x: i32, start_y: i32 },
}

impl InputState {
    pub fn new(output_w: u32, output_h: u32) -> Self {
        let output_w = output_w as i32;
        let output_h = output_h as i32;
        let pointer_x = output_w / 2;
        let pointer_y = output_h / 2;
        Self {
            pointer_x,
            pointer_y,
            visible_x: pointer_x,
            visible_y: pointer_y,
            pending_cursor_motion: false,
            deferred_cursor_target: None,
            deferred_cursor_latest_ts: None,
            deferred_cursor_events: 0,
            pending_motion_ts: None,
            pointer_overlay_enabled: false,
            primary_button_down: false,
            pointer_grab: None,
            cursor_kind: CursorKind::Default,
            visible_cursor_kind: CursorKind::Default,
            keyboard_modifiers: 0,
            output_w,
            output_h,
            pending_resize: None,
            resize_sent_this_frame: false,
            runbox: blossom::runbox::RunState::new(),
            runbox_keyboard_modal: false,
        }
    }

    pub fn runbox(&self) -> &blossom::runbox::RunState {
        &self.runbox
    }

    pub fn is_resizing(&self) -> bool {
        let Some(grab) = &self.pointer_grab else {
            return false;
        };
        matches!(grab.kind, PointerGrabKind::Resize { .. })
    }

    pub fn update_dimensions(&mut self, output_w: u32, output_h: u32) {
        self.output_w = output_w as i32;
        self.output_h = output_h as i32;
        self.pointer_x = self.pointer_x.clamp(0, self.output_w.saturating_sub(1));
        self.pointer_y = self.pointer_y.clamp(0, self.output_h.saturating_sub(1));
        self.visible_x = self.visible_x.clamp(0, self.output_w.saturating_sub(1));
        self.visible_y = self.visible_y.clamp(0, self.output_h.saturating_sub(1));
        if let Some((x, y)) = self.deferred_cursor_target.as_mut() {
            *x = (*x).clamp(0, self.output_w.saturating_sub(1));
            *y = (*y).clamp(0, self.output_h.saturating_sub(1));
        }
        self.pending_cursor_motion |=
            self.pointer_x != self.visible_x || self.pointer_y != self.visible_y;
    }

    pub fn visible_pointer_position(&self) -> (i32, i32) {
        (self.visible_x, self.visible_y)
    }

    pub fn has_pending_cursor_motion(&self) -> bool {
        self.pending_cursor_motion
            || self.pointer_x != self.visible_x
            || self.pointer_y != self.visible_y
            || self.cursor_kind != self.visible_cursor_kind
    }

    pub fn pointer_overlay_enabled(&self) -> bool {
        self.pointer_overlay_enabled
    }

    pub fn primary_button_down(&self) -> bool {
        self.primary_button_down
    }

    pub fn visible_cursor_kind(&self) -> CursorKind {
        self.visible_cursor_kind
    }

    pub fn flush_visible_pointer(&mut self, damage: &mut DamageTracker) {
        if !self.has_pending_cursor_motion() {
            return;
        }

        let had_pending_cursor_motion = self.pending_cursor_motion;
        let old_x = self.visible_x;
        let old_y = self.visible_y;
        let old_kind = self.visible_cursor_kind;
        self.visible_x = self.pointer_x;
        self.visible_y = self.pointer_y;
        self.visible_cursor_kind = self.cursor_kind;
        self.pending_cursor_motion = false;
        mark_cursor_damage(damage, old_x, old_y, self.visible_x, self.visible_y);
        if old_kind != self.visible_cursor_kind {
            mark_cursor_rect(damage, self.visible_x, self.visible_y);
        }

        if had_pending_cursor_motion
            && CURSOR_SMOOTHING_LOGS.fetch_add(1, Ordering::Relaxed) < MAX_STARTUP_LOGS
        {
            stem::trace!(
                "Cursor smoothing visible={},{} target={},{}",
                self.visible_x,
                self.visible_y,
                self.pointer_x,
                self.pointer_y
            );
        }
    }

    /// Deliver the coalesced pointer motion event (if any) to the focused
    /// client and update pointer focus.
    ///
    /// Multiple raw `PointerMove` samples accumulate into `pending_motion_ts`
    /// between frames.  This method is called **once per frame boundary**
    /// (from `BloomWorld::try_present`) to flush the latest position to
    /// clients in a single event, reducing redundant focus lookups and
    /// client deliveries.
    ///
    /// It is also called immediately before button press/release events so
    /// that ordering is preserved: clients always see the latest motion
    /// position before a click.
    pub fn flush_pointer_motion(&mut self, scene: &mut Scene, wayland_evt_write: Option<u32>) {
        let Some(ts) = self.pending_motion_ts.take() else {
            return;
        };

        let flush_count = COALESCE_FLUSH_LOGS.fetch_add(1, Ordering::Relaxed);
        if flush_count < MAX_STARTUP_LOGS {
            let pre = COALESCE_PRE.load(Ordering::Relaxed);
            stem::trace!(
                "Motion coalesce pre={} post={} pos={},{}",
                pre,
                flush_count + 1,
                self.pointer_x,
                self.pointer_y
            );
        }

        if self.pointer_grab.is_some() {
            return;
        }

        self.update_pointer_focus(scene, wayland_evt_write);
        if let Some(surface_id) = scene.pointer_focus {
            if let Some(client_id) = scene.surface_client(surface_id) {
                let (local_x, local_y) =
                    surface_local_position(scene, surface_id, self.pointer_x, self.pointer_y);
                let ev = PointerMotionEvent {
                    header: msg_header(EVT_POINTER_MOTION),
                    surface_id,
                    x: local_x,
                    y: local_y,
                    timestamp_ns: ts,
                };
                send_client_event(scene, client_id, KIND_POINTER_MOTION, &to_vec(&ev));
                send_wayland_pointer_motion(wayland_evt_write, surface_id, local_x, local_y, ts);
            }
        }
    }

    pub fn flush_pointer_grab(
        &mut self,
        scene: &mut Scene,
        damage: &mut DamageTracker,
        wayland_evt_write: Option<u32>,
    ) {
        let Some(PointerGrab { kind: PointerGrabKind::Move { .. }, .. }) = self.pointer_grab else {
            return;
        };
        self.update_pointer_grab(scene, damage, wayland_evt_write);
    }

    pub fn flush_resizes(&mut self, wayland_evt_write: Option<u32>) {
        self.resize_sent_this_frame = false;
        let Some((surface_id, width, height)) = self.pending_resize.take() else {
            return;
        };
        send_wayland_configure(wayland_evt_write, surface_id, width, height, true);
    }

    pub fn replay_deferred_cursor_motion(
        &mut self,
        scene: &mut Scene,
        damage: &mut DamageTracker,
        wayland_evt_write: Option<u32>,
    ) -> bool {
        let Some((target_x, target_y)) = self.deferred_cursor_target.take() else {
            return false;
        };
        let timestamp_ns = self.deferred_cursor_latest_ts.take();
        let event_count = core::mem::take(&mut self.deferred_cursor_events);
        let old_x = self.pointer_x;
        let old_y = self.pointer_y;
        if target_x == old_x && target_y == old_y {
            stem::debug!(
                "Discarded {} deferred cursor motion events at cursor boundary",
                event_count
            );
            return false;
        }

        self.pointer_x = target_x;
        self.pointer_y = target_y;
        self.pending_cursor_motion = true;
        if let Some(ts) = timestamp_ns {
            self.pending_motion_ts = Some(ts);
        }
        COALESCE_PRE.fetch_add(event_count, Ordering::Relaxed);
        stem::debug!(
            "Replayed {} deferred cursor motion events pos={},{}",
            event_count,
            self.pointer_x,
            self.pointer_y
        );

        if self.update_pointer_grab(scene, damage, wayland_evt_write) {
            return true;
        }
        self.update_cursor_kind(scene, damage);
        true
    }

    pub fn handle_bristle_event(
        &mut self,
        bytes: &[u8],
        scene: &mut Scene,
        damage: &mut DamageTracker,
        wayland_evt_write: Option<u32>,
        defer_cursor_motion: bool,
    ) -> bool {
        let mut immediate_repaint = false;
        if bytes.len() < abi::hid::WaylandIpcHeader::SIZE {
            return false;
        }
        let mut hdr_bytes = [0u8; abi::hid::WaylandIpcHeader::SIZE];
        hdr_bytes.copy_from_slice(&bytes[..abi::hid::WaylandIpcHeader::SIZE]);
        let header = abi::hid::WaylandIpcHeader::from_bytes(&hdr_bytes);

        let event_type = header.opcode();
        let timestamp_ns = stem::monotonic_ns();
        let event_no = BLOOM_HANDLE_EVENT_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
        let _trace = BloomInputTrace::new(event_no, event_type);
        let payload = &bytes[abi::hid::WaylandIpcHeader::SIZE..];

        match EventType::from_raw(event_type) {
            Ok(EventType::PointerMove) if payload.len() >= abi::hid::WaylandPointerMotion::SIZE => {
                let mut p = [0u8; abi::hid::WaylandPointerMotion::SIZE];
                p.copy_from_slice(&payload[..abi::hid::WaylandPointerMotion::SIZE]);
                let move_ev = abi::hid::WaylandPointerMotion::from_bytes(&p);
                let dx = (move_ev.x - self.pointer_x) as i16;
                let dy = (move_ev.y - self.pointer_y) as i16;
                if defer_cursor_motion {
                    self.defer_cursor_motion(dx, dy, timestamp_ns);
                    return false;
                }
                let old_x = self.pointer_x;
                let old_y = self.pointer_y;
                self.pointer_x = move_ev.x;
                self.pointer_y = move_ev.y;
                if self.pointer_x == old_x && self.pointer_y == old_y {
                    return false;
                }
                self.pending_cursor_motion = true;
                if matches!(
                    self.pointer_grab,
                    Some(PointerGrab { kind: PointerGrabKind::Move { .. }, .. })
                ) {
                    self.pending_motion_ts = Some(timestamp_ns);
                    return true;
                }
                if self.update_pointer_grab(scene, damage, wayland_evt_write) {
                    return true;
                }
                self.update_cursor_kind(scene, damage);
                immediate_repaint = true;
                // Coalesce: keep only the latest timestamp; focus lookup and
                // client delivery are deferred to flush_pointer_motion() which
                // is called once per frame boundary.
                self.pending_motion_ts = Some(timestamp_ns);
                COALESCE_PRE.fetch_add(1, Ordering::Relaxed);
                if POINTER_MOVE_LOGS.fetch_add(1, Ordering::Relaxed) < MAX_STARTUP_LOGS {
                    stem::trace!(
                        "Pointer moved dx={} dy={} pos={},{}",
                        dx,
                        dy,
                        self.pointer_x,
                        self.pointer_y
                    );
                }
                // Focus update and client PointerMotionEvent delivery are
                // deferred — do NOT call update_pointer_focus here.
            }
            Ok(EventType::PointerButtonDown)
                if payload.len() >= abi::hid::WaylandPointerButton::SIZE =>
            {
                let mut p = [0u8; abi::hid::WaylandPointerButton::SIZE];
                p.copy_from_slice(&payload[..abi::hid::WaylandPointerButton::SIZE]);
                let btn = abi::hid::WaylandPointerButton::from_bytes(&p);
                stem::debug!(
                    "PointerButtonDown at {},{} button={}",
                    self.pointer_x,
                    self.pointer_y,
                    btn.button
                );
                if btn.button == 0 {
                    self.primary_button_down = true;
                }
                // Flush any pending coalesced motion so clients see the latest
                // position before the button event (preserves ordering).
                // flush_pointer_motion calls update_pointer_focus internally
                // when motion was pending; only call it directly when there
                // was no pending motion so focus is still resolved correctly.
                let had_pending = self.pending_motion_ts.is_some();
                self.flush_pointer_motion(scene, wayland_evt_write);
                if !had_pending {
                    self.update_pointer_focus(scene, wayland_evt_write);
                }
                let old_focus = scene.keyboard_focus;
                if btn.button == 0 {
                    if self.handle_chrome_button(scene, damage, wayland_evt_write, old_focus) {
                        immediate_repaint = true;
                        mark_cursor_damage(
                            damage,
                            self.visible_x,
                            self.visible_y,
                            self.pointer_x,
                            self.pointer_y,
                        );
                        return true;
                    }
                    if let Some(surface_id) =
                        self.start_chrome_grab(scene, damage, wayland_evt_write)
                    {
                        let new_focus = Some(surface_id);
                        if scene.raise_to_top(surface_id) {
                            immediate_repaint = true;
                            if old_focus == new_focus {
                                if let Some(rect) = scene.surface_rect(surface_id) {
                                    mark_surface_visual_damage(scene, damage, surface_id, rect);
                                }
                            }
                        }
                        scene.keyboard_focus = new_focus;
                        mark_focus_damage(scene, damage, old_focus, new_focus);
                        self.send_keyboard_focus_events(
                            scene,
                            old_focus,
                            scene.keyboard_focus,
                            wayland_evt_write,
                        );
                        mark_cursor_damage(
                            damage,
                            self.visible_x,
                            self.visible_y,
                            self.pointer_x,
                            self.pointer_y,
                        );
                        return true;
                    }
                }
                let new_focus = scene.pointer_focus;
                if let Some(id) = new_focus {
                    if scene.raise_to_top(id) {
                        immediate_repaint = true;
                        if old_focus == new_focus {
                            if let Some(rect) = scene.surface_rect(id) {
                                mark_surface_visual_damage(scene, damage, id, rect);
                            }
                        }
                    }
                }
                scene.keyboard_focus = new_focus;
                mark_focus_damage(scene, damage, old_focus, new_focus);
                self.send_keyboard_focus_events(
                    scene,
                    old_focus,
                    scene.keyboard_focus,
                    wayland_evt_write,
                );
                if let Some(surface_id) = scene.pointer_focus {
                    if let Some(client_id) = scene.surface_client(surface_id) {
                        let ev = PointerButtonEvent {
                            header: msg_header(EVT_POINTER_BUTTON),
                            surface_id,
                            button: btn.button,
                            pressed: pressed_flag(true),
                            _pad: [0; 2],
                            timestamp_ns,
                        };
                        send_client_event(scene, client_id, KIND_POINTER_BUTTON, &to_vec(&ev));
                        send_wayland_pointer_button(
                            wayland_evt_write,
                            surface_id,
                            btn.button,
                            true,
                            timestamp_ns,
                        );
                    }
                } else if btn.button == 0 {
                    // Click on empty space: no focused surface. Notify the
                    // Wayland server with bloom_surface_id=0 so it can
                    // dismiss any open popup surfaces.
                    send_wayland_pointer_button(
                        wayland_evt_write,
                        0,
                        btn.button,
                        true,
                        timestamp_ns,
                    );
                }
                mark_cursor_damage(
                    damage,
                    self.visible_x,
                    self.visible_y,
                    self.pointer_x,
                    self.pointer_y,
                );
            }
            Ok(EventType::PointerButtonUp)
                if payload.len() >= abi::hid::WaylandPointerButton::SIZE =>
            {
                let mut p = [0u8; abi::hid::WaylandPointerButton::SIZE];
                p.copy_from_slice(&payload[..abi::hid::WaylandPointerButton::SIZE]);
                let btn = abi::hid::WaylandPointerButton::from_bytes(&p);
                if btn.button == 0 {
                    self.primary_button_down = false;
                }
                // Flush any pending coalesced motion before the button-up event.
                // flush_pointer_motion calls update_pointer_focus internally
                // when motion was pending; only call it directly when there
                // was no pending motion so focus is still resolved correctly.
                let had_pending = self.pending_motion_ts.is_some();
                self.flush_pointer_motion(scene, wayland_evt_write);
                if !had_pending {
                    self.update_pointer_focus(scene, wayland_evt_write);
                }
                if btn.button == 0 && self.end_pointer_grab(wayland_evt_write, scene) {
                    immediate_repaint = true;
                    self.update_cursor_kind(scene, damage);
                    mark_cursor_damage(
                        damage,
                        self.visible_x,
                        self.visible_y,
                        self.pointer_x,
                        self.pointer_y,
                    );
                    return true;
                }
                if let Some(surface_id) = scene.pointer_focus {
                    if let Some(client_id) = scene.surface_client(surface_id) {
                        let ev = PointerButtonEvent {
                            header: msg_header(EVT_POINTER_BUTTON),
                            surface_id,
                            button: btn.button,
                            pressed: pressed_flag(false),
                            _pad: [0; 2],
                            timestamp_ns,
                        };
                        send_client_event(scene, client_id, KIND_POINTER_BUTTON, &to_vec(&ev));
                        send_wayland_pointer_button(
                            wayland_evt_write,
                            surface_id,
                            btn.button,
                            false,
                            timestamp_ns,
                        );
                    }
                }
                mark_cursor_damage(
                    damage,
                    self.visible_x,
                    self.visible_y,
                    self.pointer_x,
                    self.pointer_y,
                );
            }
            Ok(EventType::Scroll) if payload.len() >= abi::hid::WaylandPointerAxis::SIZE => {
                let mut p = [0u8; abi::hid::WaylandPointerAxis::SIZE];
                p.copy_from_slice(&payload[..abi::hid::WaylandPointerAxis::SIZE]);
                let axis = abi::hid::WaylandPointerAxis::from_bytes(&p);
                if let Some(surface_id) = scene.pointer_focus {
                    if scene.surface_client(surface_id).is_some() {
                        let dx = if axis.axis == 1 { axis.value as i16 } else { 0 };
                        let dy = if axis.axis == 0 { axis.value as i16 } else { 0 };
                        send_wayland_pointer_scroll(
                            wayland_evt_write,
                            surface_id,
                            dx,
                            dy,
                            timestamp_ns,
                        );
                    }
                }
            }
            Ok(EventType::KeyDown) if payload.len() >= KeyEventPayload::SIZE => {
                let mut p = [0u8; KeyEventPayload::SIZE];
                p.copy_from_slice(&payload[..KeyEventPayload::SIZE]);
                let key = KeyEventPayload::from_bytes(&p);
                let raw_key = key.key;
                stem::trace!(
                    "KeyDown received: {:?} (raw={:#06x}, mods={:?}, repeat={})",
                    key.key(),
                    raw_key,
                    key.mods(),
                    key.is_repeat()
                );
                self.keyboard_modifiers = key.mods;
                if is_pointer_overlay_toggle(key) {
                    if !key.is_repeat() {
                        self.pointer_overlay_enabled = !self.pointer_overlay_enabled;
                        damage.mark_full(self.output_w as u32, self.output_h as u32);
                        stem::info!(
                            "Pointer debug overlay {}",
                            if self.pointer_overlay_enabled { "enabled" } else { "disabled" }
                        );
                    }
                    return true;
                }
                let wm_action =
                    blossom::input::handle_hotkey(key.key(), key.mods(), key.is_repeat());
                if matches!(
                    wm_action,
                    blossom::input::WmAction::ToggleRunBox
                        | blossom::input::WmAction::ToggleLauncher
                ) && self.forward_shell_shortcut(scene, wayland_evt_write, key, timestamp_ns)
                {
                    return true;
                }
                if matches!(wm_action, blossom::input::WmAction::ToggleRunBox) {
                    let visible = self.runbox.toggle();
                    self.runbox_keyboard_modal = true;
                    damage.mark_full(self.output_w as u32, self.output_h as u32);
                    if visible {
                        stem::info!("Run dialog. Input field focused");
                    } else {
                        stem::info!("Run dialog closed");
                    }
                    return true;
                }
                if self.runbox.visible {
                    match self.runbox.handle_key(key.key(), key.mods()) {
                        blossom::runbox::RunBoxEvent::Submit(
                            blossom::runbox::RunAction::Execute(command),
                        ) => {
                            damage.mark_full(self.output_w as u32, self.output_h as u32);
                            self.runbox_keyboard_modal = true;
                            let launcher = BloomLauncher;
                            match blossom::runbox::Launcher::run(&launcher, &command) {
                                blossom::runbox::LaunchResult::Started { pid } => {
                                    stem::info!("Launched {} as pid {}", command, pid);
                                }
                                blossom::runbox::LaunchResult::NotFound => {
                                    stem::warn!(
                                        "Could not launch '{}': command not found",
                                        command
                                    );
                                }
                                blossom::runbox::LaunchResult::Failed => {
                                    stem::warn!("Could not launch '{}'", command);
                                }
                            }
                        }
                        blossom::runbox::RunBoxEvent::Closed => {
                            self.runbox_keyboard_modal = true;
                            damage.mark_full(self.output_w as u32, self.output_h as u32);
                            stem::info!("Run dialog closed");
                        }
                        blossom::runbox::RunBoxEvent::Updated
                        | blossom::runbox::RunBoxEvent::Consumed => {
                            damage.mark_full(self.output_w as u32, self.output_h as u32);
                        }
                    }
                    return true;
                }
                match wm_action {
                    blossom::input::WmAction::CycleFocus { forward } => {
                        let (old_focus, new_focus) = scene.cycle_focus(forward);
                        stem::debug!(
                            "Focus cycled from {:?} to {:?} (forward={})",
                            old_focus,
                            new_focus,
                            forward
                        );
                        mark_focus_damage(scene, damage, old_focus, new_focus);
                        self.send_keyboard_focus_events(
                            scene,
                            old_focus,
                            new_focus,
                            wayland_evt_write,
                        );
                        return true;
                    }
                    blossom::input::WmAction::ToggleFullscreen => {
                        stem::info!("BLOOM_FULLSCREEN_TOGGLE_TRIGGERED");
                        if let Some(surface_id) = scene.keyboard_focus {
                            self.toggle_fullscreen(scene, damage, wayland_evt_write, surface_id);
                        }
                        return true;
                    }
                    blossom::input::WmAction::CloseSurface => {
                        if let Some(surface_id) = scene.keyboard_focus {
                            self.close_surface(scene, damage, wayland_evt_write, surface_id);
                        }
                        return true;
                    }
                    blossom::input::WmAction::ToggleRunBox => {}
                    blossom::input::WmAction::ToggleLauncher => {
                        stem::warn!("No shell surface registered for launcher shortcut");
                        return true;
                    }
                    blossom::input::WmAction::None => {}
                }
                if let Some(surface_id) = scene.keyboard_focus {
                    if let Some(client_id) = scene.surface_client(surface_id) {
                        let ev = KeyboardKeyEvent {
                            header: msg_header(EVT_KEYBOARD_KEY),
                            surface_id,
                            key: key.key,
                            pressed: pressed_flag(true),
                            modifiers: key.mods,
                            repeat: if key.is_repeat() { 1 } else { 0 },
                            _pad: [0; 3],
                            timestamp_ns,
                        };
                        send_client_event(scene, client_id, KIND_KEYBOARD_KEY, &to_vec(&ev));
                        send_wayland_keyboard_key(
                            wayland_evt_write,
                            surface_id,
                            key.key,
                            true,
                            key.mods,
                            key.is_repeat(),
                            timestamp_ns,
                        );
                    }
                }
            }
            Ok(EventType::KeyUp) if payload.len() >= KeyEventPayload::SIZE => {
                let mut p = [0u8; KeyEventPayload::SIZE];
                p.copy_from_slice(&payload[..KeyEventPayload::SIZE]);
                let key = KeyEventPayload::from_bytes(&p);
                self.keyboard_modifiers = key.mods;
                if is_pointer_overlay_toggle(key) {
                    return false;
                }
                if self.runbox.visible || self.runbox_keyboard_modal {
                    if key.mods == 0 {
                        self.runbox_keyboard_modal = self.runbox.visible;
                    }
                    return false;
                }
                if let Some(surface_id) = scene.keyboard_focus {
                    if let Some(client_id) = scene.surface_client(surface_id) {
                        let ev = KeyboardKeyEvent {
                            header: msg_header(EVT_KEYBOARD_KEY),
                            surface_id,
                            key: key.key,
                            pressed: pressed_flag(false),
                            modifiers: key.mods,
                            repeat: if key.is_repeat() { 1 } else { 0 },
                            _pad: [0; 3],
                            timestamp_ns,
                        };
                        send_client_event(scene, client_id, KIND_KEYBOARD_KEY, &to_vec(&ev));
                        send_wayland_keyboard_key(
                            wayland_evt_write,
                            surface_id,
                            key.key,
                            false,
                            key.mods,
                            key.is_repeat(),
                            timestamp_ns,
                        );
                    }
                }
            }
            _ => {}
        }
        immediate_repaint
    }

    fn forward_shell_shortcut(
        &self,
        scene: &Scene,
        wayland_evt_write: Option<u32>,
        key: KeyEventPayload,
        timestamp_ns: u64,
    ) -> bool {
        let Some(surface_id) = scene.global_shortcut_surface() else {
            return false;
        };
        let Some(client_id) = scene.surface_client(surface_id) else {
            return false;
        };
        let ev = KeyboardKeyEvent {
            header: msg_header(EVT_KEYBOARD_KEY),
            surface_id,
            key: key.key,
            pressed: pressed_flag(true),
            modifiers: key.mods,
            repeat: if key.is_repeat() { 1 } else { 0 },
            _pad: [0; 3],
            timestamp_ns,
        };
        send_client_event(scene, client_id, KIND_KEYBOARD_KEY, &to_vec(&ev));
        send_wayland_keyboard_key(
            wayland_evt_write,
            surface_id,
            key.key,
            true,
            key.mods,
            key.is_repeat(),
            timestamp_ns,
        );
        stem::debug!("Forwarded shell shortcut to surface {}", surface_id);
        true
    }

    fn update_pointer_focus(&mut self, scene: &mut Scene, wayland_evt_write: Option<u32>) {
        let new_focus = scene.top_client_surface_at(self.pointer_x, self.pointer_y);
        if scene.pointer_focus == new_focus {
            return;
        }

        if let Some(old_surface) = scene.pointer_focus {
            if let Some(client_id) = scene.surface_client(old_surface) {
                let ev = PointerLeaveEvent {
                    header: msg_header(EVT_POINTER_LEAVE),
                    surface_id: old_surface,
                };
                send_client_event(scene, client_id, KIND_POINTER_LEAVE, &to_vec(&ev));
                send_wayland_pointer_leave(wayland_evt_write, old_surface);
            }
        }

        scene.pointer_focus = new_focus;
        if let Some(surface_id) = new_focus {
            if let Some(client_id) = scene.surface_client(surface_id) {
                let (local_x, local_y) =
                    surface_local_position(scene, surface_id, self.pointer_x, self.pointer_y);
                let ev = PointerEnterEvent {
                    header: msg_header(EVT_POINTER_ENTER),
                    surface_id,
                    x: local_x,
                    y: local_y,
                };
                send_client_event(scene, client_id, KIND_POINTER_ENTER, &to_vec(&ev));
                send_wayland_pointer_enter(wayland_evt_write, surface_id, local_x, local_y);
            }
        }
    }

    fn start_chrome_grab(
        &mut self,
        scene: &Scene,
        damage: &mut DamageTracker,
        wayland_evt_write: Option<u32>,
    ) -> Option<u32> {
        match scene.hit_test(self.pointer_x, self.pointer_y)? {
            HitTarget::TitleBar { surface_id } => {
                let rect = scene.surface_rect(surface_id)?;
                let offset_x = self.pointer_x.saturating_sub(rect.x as i32);
                let offset_y = self.pointer_y.saturating_sub(rect.y as i32);
                self.pointer_grab = Some(PointerGrab {
                    surface_id,
                    kind: PointerGrabKind::Move { offset_x, offset_y },
                });
                self.set_cursor_kind(CursorKind::Move, damage);
                stem::debug!(
                    "Window drag started surface={} pointer={},{} offset={},{}",
                    surface_id,
                    self.pointer_x,
                    self.pointer_y,
                    offset_x,
                    offset_y
                );
                Some(surface_id)
            }
            HitTarget::Frame { surface_id, edge } => {
                let rect = scene.surface_rect(surface_id)?;
                self.pointer_grab = Some(PointerGrab {
                    surface_id,
                    kind: PointerGrabKind::Resize {
                        edge,
                        start_rect: rect,
                        start_x: self.pointer_x,
                        start_y: self.pointer_y,
                    },
                });
                self.set_cursor_kind(CursorKind::for_resize_edge(edge), damage);
                send_wayland_configure(wayland_evt_write, surface_id, rect.w, rect.h, true);
                stem::debug!(
                    "Window resize started surface={} edge={:?} pointer={},{}",
                    surface_id,
                    edge,
                    self.pointer_x,
                    self.pointer_y
                );
                Some(surface_id)
            }
            HitTarget::Client { .. } | HitTarget::ChromeButton { .. } => None,
        }
    }

    fn handle_chrome_button(
        &mut self,
        scene: &mut Scene,
        damage: &mut DamageTracker,
        wayland_evt_write: Option<u32>,
        old_focus: Option<u32>,
    ) -> bool {
        let Some(HitTarget::ChromeButton { surface_id, button }) =
            scene.hit_test(self.pointer_x, self.pointer_y)
        else {
            return false;
        };

        match button {
            ChromeButton::Close => {
                self.close_surface(scene, damage, wayland_evt_write, surface_id);
            }
            ChromeButton::Minimize | ChromeButton::Shade => {
                self.toggle_shade(scene, damage, wayland_evt_write, surface_id);
            }
            ChromeButton::Maximize => {
                let target = abi::display_protocol::Rect {
                    x: 0,
                    y: 0,
                    w: self.output_w.max(MIN_RESIZE_W as i32) as u32,
                    h: self.output_h.max(MIN_RESIZE_H as i32) as u32,
                };
                if let Some(resized) = scene.resize_surface_absolute(surface_id, target) {
                    mark_surface_visual_damage(scene, damage, surface_id, resized.old_rect);
                    mark_surface_visual_damage(scene, damage, surface_id, resized.new_rect);
                }
                let new_focus = Some(surface_id);
                if scene.raise_to_top(surface_id) {
                    if old_focus == new_focus {
                        if let Some(rect) = scene.surface_rect(surface_id) {
                            mark_surface_visual_damage(scene, damage, surface_id, rect);
                        }
                    }
                }
                scene.keyboard_focus = new_focus;
                mark_focus_damage(scene, damage, old_focus, new_focus);
                self.send_keyboard_focus_events(
                    scene,
                    old_focus,
                    scene.keyboard_focus,
                    wayland_evt_write,
                );
                send_wayland_toplevel_action(
                    wayland_evt_write,
                    surface_id,
                    ipc::TOPLEVEL_ACTION_MAXIMIZE,
                    target.w as i32,
                    target.h as i32,
                );
                stem::debug!(
                    "Maximize button pressed surface={} size={}x{}",
                    surface_id,
                    target.w,
                    target.h
                );
            }
            ChromeButton::Fullscreen => {
                self.toggle_fullscreen(scene, damage, wayland_evt_write, surface_id);
            }
        }
        true
    }

    fn close_surface(
        &mut self,
        scene: &mut Scene,
        damage: &mut DamageTracker,
        wayland_evt_write: Option<u32>,
        surface_id: u32,
    ) {
        let old_focus = scene.keyboard_focus;
        let new_focus = Some(surface_id);
        scene.raise_to_top(surface_id);
        scene.keyboard_focus = new_focus;
        mark_focus_damage(scene, damage, old_focus, new_focus);
        self.send_keyboard_focus_events(scene, old_focus, scene.keyboard_focus, wayland_evt_write);
        send_close_requested_to_owner(scene, surface_id);
        send_wayland_toplevel_action(
            wayland_evt_write,
            surface_id,
            ipc::TOPLEVEL_ACTION_CLOSE,
            0,
            0,
        );
        stem::debug!("Close button pressed for surface={}", surface_id);
    }

    fn toggle_shade(
        &mut self,
        scene: &mut Scene,
        damage: &mut DamageTracker,
        wayland_evt_write: Option<u32>,
        surface_id: u32,
    ) {
        let Some(toggled) = scene.toggle_surface_shaded(surface_id) else { return };
        mark_surface_visual_damage(scene, damage, surface_id, toggled.old_rect);
        mark_surface_visual_damage(scene, damage, surface_id, toggled.new_rect);

        let new_focus = Some(surface_id);
        let old_focus = scene.keyboard_focus;
        scene.raise_to_top(surface_id);
        scene.keyboard_focus = new_focus;
        mark_focus_damage(scene, damage, old_focus, new_focus);
        self.send_keyboard_focus_events(scene, old_focus, scene.keyboard_focus, wayland_evt_write);
        stem::debug!("Shade toggled for surface={} shaded={}", surface_id, toggled.active);
    }

    fn toggle_fullscreen(
        &mut self,
        scene: &mut Scene,
        damage: &mut DamageTracker,
        wayland_evt_write: Option<u32>,
        surface_id: u32,
    ) {
        let Some(toggled) = scene.toggle_surface_fullscreen(
            surface_id,
            abi::display_protocol::Rect {
                x: 0,
                y: 0,
                w: self.output_w as u32,
                h: self.output_h as u32,
            },
        ) else {
            return;
        };
        mark_surface_visual_damage(scene, damage, surface_id, toggled.old_rect);
        mark_surface_visual_damage(scene, damage, surface_id, toggled.new_rect);

        stem::info!("BLOOM_FULLSCREEN_TOGGLE_TRIGGERED (active={})", toggled.active);

        let new_focus = Some(surface_id);
        let old_focus = scene.keyboard_focus;
        scene.raise_to_top(surface_id);
        scene.keyboard_focus = new_focus;
        mark_focus_damage(scene, damage, old_focus, new_focus);
        self.send_keyboard_focus_events(scene, old_focus, scene.keyboard_focus, wayland_evt_write);

        let action = if toggled.active {
            ipc::TOPLEVEL_ACTION_FULLSCREEN
        } else {
            ipc::TOPLEVEL_ACTION_MAXIMIZE // Use maximize as fallback for "un-fullscreen"
        };

        send_wayland_toplevel_action(
            wayland_evt_write,
            surface_id,
            action,
            toggled.new_rect.w as i32,
            toggled.new_rect.h as i32,
        );
        stem::debug!("Fullscreen toggled surface={} fullscreen={}", surface_id, toggled.active);
    }

    fn update_pointer_grab(
        &mut self,
        scene: &mut Scene,
        damage: &mut DamageTracker,
        wayland_evt_write: Option<u32>,
    ) -> bool {
        let Some(grab) = self.pointer_grab else {
            return false;
        };

        match grab.kind {
            PointerGrabKind::Move { offset_x, offset_y } => {
                let target_x = self.pointer_x.saturating_sub(offset_x);
                let target_y = self.pointer_y.saturating_sub(offset_y);
                let Some(moved) = scene.move_surface_absolute(grab.surface_id, target_x, target_y)
                else {
                    self.pointer_grab = None;
                    return true;
                };
                if moved.changed {
                    mark_surface_visual_damage(scene, damage, grab.surface_id, moved.old_rect);
                    mark_surface_visual_damage(scene, damage, grab.surface_id, moved.new_rect);
                    let cursor_offset_x = self.pointer_x.saturating_sub(moved.new_rect.x as i32);
                    let cursor_offset_y = self.pointer_y.saturating_sub(moved.new_rect.y as i32);
                    stem::trace!(
                        "Window drag moved surface={} to {},{} pointer={},{} offset={},{}",
                        grab.surface_id,
                        moved.new_rect.x,
                        moved.new_rect.y,
                        self.pointer_x,
                        self.pointer_y,
                        cursor_offset_x,
                        cursor_offset_y
                    );
                }
                true
            }
            PointerGrabKind::Resize { edge, start_rect, start_x, start_y } => {
                let next_rect = resized_rect(
                    start_rect,
                    edge,
                    self.pointer_x.saturating_sub(start_x),
                    self.pointer_y.saturating_sub(start_y),
                    self.output_w.max(0) as u32,
                    self.output_h.max(0) as u32,
                );
                let Some(resized) = scene.resize_surface_absolute(grab.surface_id, next_rect)
                else {
                    self.pointer_grab = None;
                    return true;
                };
                if resized.changed {
                    mark_surface_visual_damage(scene, damage, grab.surface_id, resized.old_rect);
                    mark_surface_visual_damage(scene, damage, grab.surface_id, resized.new_rect);

                    let new_size = (grab.surface_id, resized.new_rect.w, resized.new_rect.h);
                    self.pending_resize = Some(new_size);

                    // Send configure on every motion during resize for ultra-realtime response.
                    send_wayland_configure(
                        wayland_evt_write,
                        new_size.0,
                        new_size.1 as u32,
                        new_size.2 as u32,
                        true,
                    );
                    self.resize_sent_this_frame = true;
                    stem::trace!(
                        "Window resize moved surface={} to {},{} {}x{} (realtime)",
                        grab.surface_id,
                        resized.new_rect.x,
                        resized.new_rect.y,
                        resized.new_rect.w,
                        resized.new_rect.h
                    );
                    return true;
                }
                false
            }
        }
    }

    fn end_pointer_grab(&mut self, wayland_evt_write: Option<u32>, scene: &Scene) -> bool {
        let Some(grab) = self.pointer_grab.take() else {
            return false;
        };
        self.pending_resize = None;
        self.resize_sent_this_frame = false;
        match grab.kind {
            PointerGrabKind::Move { .. } => {
                stem::debug!("Window drag ended for surface={}", grab.surface_id);
            }
            PointerGrabKind::Resize { .. } => {
                if let Some(rect) = scene.surface_rect(grab.surface_id) {
                    send_wayland_configure(
                        wayland_evt_write,
                        grab.surface_id,
                        rect.w,
                        rect.h,
                        false,
                    );
                }
                stem::debug!("Window resize ended for surface={}", grab.surface_id);
            }
        }
        true
    }

    fn update_cursor_kind(&mut self, scene: &Scene, damage: &mut DamageTracker) {
        let next = match self.pointer_grab {
            Some(PointerGrab { kind: PointerGrabKind::Move { .. }, .. }) => CursorKind::Move,
            Some(PointerGrab { kind: PointerGrabKind::Resize { edge, .. }, .. }) => {
                CursorKind::for_resize_edge(edge)
            }
            None => match scene.hit_test(self.pointer_x, self.pointer_y) {
                Some(HitTarget::TitleBar { .. }) => CursorKind::Move,
                Some(HitTarget::Frame { edge, .. }) => CursorKind::for_resize_edge(edge),
                Some(HitTarget::ChromeButton { .. }) => CursorKind::Default,
                _ => CursorKind::Default,
            },
        };
        self.set_cursor_kind(next, damage);
    }

    fn set_cursor_kind(&mut self, next: CursorKind, damage: &mut DamageTracker) {
        if self.cursor_kind == next {
            return;
        }
        self.cursor_kind = next;
        self.pending_cursor_motion = true;
        mark_cursor_rect(damage, self.visible_x, self.visible_y);
        stem::debug!("Cursor kind {:?}", next);
    }

    fn defer_cursor_motion(&mut self, dx: i16, dy: i16, timestamp_ns: u64) {
        let (old_x, old_y) =
            self.deferred_cursor_target.unwrap_or((self.pointer_x, self.pointer_y));
        let next_x = old_x.saturating_add(dx as i32).clamp(0, self.output_w.saturating_sub(1));
        let next_y = old_y.saturating_add(dy as i32).clamp(0, self.output_h.saturating_sub(1));
        self.deferred_cursor_target = Some((next_x, next_y));
        self.deferred_cursor_latest_ts = Some(timestamp_ns);
        self.deferred_cursor_events = self.deferred_cursor_events.saturating_add(1);
        if POINTER_MOVE_LOGS.fetch_add(1, Ordering::Relaxed) < MAX_STARTUP_LOGS {
            stem::trace!(
                "Deferred cursor motion dx={} dy={} queued={} target={},{}",
                dx,
                dy,
                self.deferred_cursor_events,
                next_x,
                next_y
            );
        }
    }

    fn send_keyboard_focus_events(
        &self,
        scene: &mut Scene,
        old_focus: Option<u32>,
        new_focus: Option<u32>,
        wayland_evt_write: Option<u32>,
    ) {
        if old_focus == new_focus {
            return;
        }

        if let Some(old_surface) = old_focus {
            if let Some(client_id) = scene.surface_client(old_surface) {
                let ev = KeyboardLeaveEvent {
                    header: msg_header(EVT_KEYBOARD_LEAVE),
                    surface_id: old_surface,
                };
                send_client_event(scene, client_id, KIND_KEYBOARD_LEAVE, &to_vec(&ev));
                send_wayland_keyboard_leave(wayland_evt_write, old_surface);
            }
        }

        if let Some(new_surface) = new_focus {
            if let Some(client_id) = scene.surface_client(new_surface) {
                let ev = KeyboardEnterEvent {
                    header: msg_header(EVT_KEYBOARD_ENTER),
                    surface_id: new_surface,
                    modifiers: self.keyboard_modifiers,
                    _pad: [0; 3],
                };
                send_client_event(scene, client_id, KIND_KEYBOARD_ENTER, &to_vec(&ev));
                send_wayland_keyboard_enter(
                    wayland_evt_write,
                    new_surface,
                    self.keyboard_modifiers,
                );
            }
        }
    }
}

fn mark_cursor_damage(damage: &mut DamageTracker, old_x: i32, old_y: i32, new_x: i32, new_y: i32) {
    mark_cursor_rect(damage, old_x, old_y);
    if old_x != new_x || old_y != new_y {
        mark_cursor_rect(damage, new_x, new_y);
    }
}

fn mark_focus_damage(
    scene: &Scene,
    damage: &mut DamageTracker,
    old_focus: Option<u32>,
    new_focus: Option<u32>,
) {
    if old_focus == new_focus {
        return;
    }
    if let Some(surface_id) = old_focus {
        if let Some(rect) = scene.surface_rect(surface_id) {
            mark_surface_visual_damage(scene, damage, surface_id, rect);
        }
    }
    if let Some(surface_id) = new_focus {
        if let Some(rect) = scene.surface_rect(surface_id) {
            mark_surface_visual_damage(scene, damage, surface_id, rect);
        }
    }
}

fn mark_surface_visual_damage(
    scene: &Scene,
    damage: &mut DamageTracker,
    surface_id: u32,
    rect: abi::display_protocol::Rect,
) -> abi::display_protocol::Rect {
    let visual = scene.visual_rect_for_surface_rect(surface_id, rect).unwrap_or(rect);
    damage.mark_rect(visual);
    visual
}

fn mark_cursor_rect(damage: &mut DamageTracker, x: i32, y: i32) {
    let x = x.saturating_sub(CURSOR_HOTSPOT_X).max(0) as u32;
    let y = y.saturating_sub(CURSOR_HOTSPOT_Y).max(0) as u32;
    // Route cursor damage through the dedicated cursor pool so that pointer
    // motion does not inflate window/app damage or contribute to the
    // full-output fallback threshold.
    damage.mark_cursor_rect(abi::display_protocol::Rect {
        x,
        y,
        w: CURSOR_DAMAGE_W,
        h: CURSOR_DAMAGE_H,
    });
}

fn resized_rect(
    start: abi::display_protocol::Rect,
    edge: ResizeEdge,
    dx: i32,
    dy: i32,
    output_w: u32,
    output_h: u32,
) -> abi::display_protocol::Rect {
    let mut x0 = start.x as i32;
    let mut y0 = start.y as i32;
    let mut x1 = start.x.saturating_add(start.w) as i32;
    let mut y1 = start.y.saturating_add(start.h) as i32;

    match edge {
        ResizeEdge::North => y0 = y0.saturating_add(dy),
        ResizeEdge::South => y1 = y1.saturating_add(dy),
        ResizeEdge::East => x1 = x1.saturating_add(dx),
        ResizeEdge::West => x0 = x0.saturating_add(dx),
        ResizeEdge::NorthEast => {
            y0 = y0.saturating_add(dy);
            x1 = x1.saturating_add(dx);
        }
        ResizeEdge::NorthWest => {
            y0 = y0.saturating_add(dy);
            x0 = x0.saturating_add(dx);
        }
        ResizeEdge::SouthEast => {
            y1 = y1.saturating_add(dy);
            x1 = x1.saturating_add(dx);
        }
        ResizeEdge::SouthWest => {
            y1 = y1.saturating_add(dy);
            x0 = x0.saturating_add(dx);
        }
    }

    x0 = x0.clamp(0, output_w.saturating_sub(MIN_RESIZE_W) as i32);
    y0 = y0.clamp(0, output_h.saturating_sub(MIN_RESIZE_H) as i32);
    x1 = x1.clamp(MIN_RESIZE_W as i32, output_w as i32);
    y1 = y1.clamp(MIN_RESIZE_H as i32, output_h as i32);

    if x1.saturating_sub(x0) < MIN_RESIZE_W as i32 {
        match edge {
            ResizeEdge::West | ResizeEdge::NorthWest | ResizeEdge::SouthWest => {
                x0 = x1.saturating_sub(MIN_RESIZE_W as i32).max(0);
            }
            _ => x1 = x0.saturating_add(MIN_RESIZE_W as i32).min(output_w as i32),
        }
    }
    if y1.saturating_sub(y0) < MIN_RESIZE_H as i32 {
        match edge {
            ResizeEdge::North | ResizeEdge::NorthEast | ResizeEdge::NorthWest => {
                y0 = y1.saturating_sub(MIN_RESIZE_H as i32).max(0);
            }
            _ => y1 = y0.saturating_add(MIN_RESIZE_H as i32).min(output_h as i32),
        }
    }

    abi::display_protocol::Rect {
        x: x0.max(0) as u32,
        y: y0.max(0) as u32,
        w: x1.saturating_sub(x0).max(MIN_RESIZE_W as i32) as u32,
        h: y1.saturating_sub(y0).max(MIN_RESIZE_H as i32) as u32,
    }
}

fn send_wayland_configure(
    wayland_evt_write: Option<u32>,
    surface_id: u32,
    width: u32,
    height: u32,
    resizing: bool,
) {
    let Some(evt_write) = wayland_evt_write else {
        return;
    };
    let msg = ipc::encode_configure_surface(surface_id, width as i32, height as i32, resizing);
    let _ = port_send_all(evt_write, &msg);
}

fn send_wayland_toplevel_action(
    wayland_evt_write: Option<u32>,
    surface_id: u32,
    action: u8,
    width: i32,
    height: i32,
) {
    let Some(evt_write) = wayland_evt_write else {
        return;
    };
    let msg = ipc::encode_toplevel_action(surface_id, action, width, height);
    let _ = port_send_all(evt_write, &msg);
}

/// Ask the Wayland server to emit `zwlr_layer_surface_v1.closed` for the given
/// surface and clean up the layer-surface object.
pub fn send_wayland_close_layer_surface(wayland_evt_write: Option<u32>, surface_id: u32) {
    let Some(evt_write) = wayland_evt_write else {
        return;
    };
    let msg = ipc::encode_close_layer_surface(surface_id);
    let _ = port_send_all(evt_write, &msg);
}

struct BloomInputTrace {
    event_no: u64,
    event_type: u16,
    start_ns: u64,
    enabled: bool,
}

impl BloomInputTrace {
    fn new(event_no: u64, event_type: u16) -> Self {
        let enabled = should_log_input(event_no);
        let start_ns = stem::monotonic_ns();
        if enabled {
            stem::trace!(
                "Handle_bristle_event entry event={} type={} start_ns={}",
                event_no,
                event_type_name(event_type),
                start_ns
            );
        }
        Self { event_no, event_type, start_ns, enabled }
    }
}

impl Drop for BloomInputTrace {
    fn drop(&mut self) {
        if self.enabled {
            let end_ns = stem::monotonic_ns();
            stem::trace!(
                "Handle_bristle_event exit event={} type={} elapsed_ns={}",
                self.event_no,
                event_type_name(self.event_type),
                end_ns.saturating_sub(self.start_ns)
            );
        }
    }
}

fn should_log_input(count: u64) -> bool {
    count <= INPUT_TRACE_INITIAL || count % INPUT_TRACE_INTERVAL == 0
}

fn event_type_name(raw: u16) -> &'static str {
    match EventType::from_raw(raw) {
        Ok(EventType::KeyDown) => "KeyDown",
        Ok(EventType::KeyUp) => "KeyUp",
        Ok(EventType::PointerMove) => "PointerMove",
        Ok(EventType::PointerButtonDown) => "PointerButtonDown",
        Ok(EventType::PointerButtonUp) => "PointerButtonUp",
        Ok(EventType::Scroll) => "Scroll",
        Ok(EventType::DeviceAdded) => "DeviceAdded",
        Ok(EventType::DeviceRemoved) => "DeviceRemoved",
        _ => "Unknown",
    }
}

fn send_wayland_pointer_enter(wayland_evt_write: Option<u32>, surface_id: u32, x: i32, y: i32) {
    let Some(evt_write) = wayland_evt_write else {
        return;
    };
    let msg = ipc::encode_pointer_enter(surface_id, x, y);
    let _ = port_send_all(evt_write, &msg);
}

fn send_wayland_pointer_leave(wayland_evt_write: Option<u32>, surface_id: u32) {
    let Some(evt_write) = wayland_evt_write else {
        return;
    };
    let msg = ipc::encode_pointer_leave(surface_id);
    let _ = port_send_all(evt_write, &msg);
}

fn send_wayland_pointer_motion(
    wayland_evt_write: Option<u32>,
    surface_id: u32,
    x: i32,
    y: i32,
    timestamp_ns: u64,
) {
    let Some(evt_write) = wayland_evt_write else {
        return;
    };
    let msg = ipc::encode_pointer_motion(surface_id, x, y, timestamp_ns);
    let _ = port_send_all(evt_write, &msg);
}

fn send_wayland_pointer_button(
    wayland_evt_write: Option<u32>,
    surface_id: u32,
    button: u8,
    pressed: bool,
    timestamp_ns: u64,
) {
    let Some(evt_write) = wayland_evt_write else {
        return;
    };
    let msg = ipc::encode_pointer_button(surface_id, button, pressed, timestamp_ns);
    let _ = port_send_all(evt_write, &msg);
}

fn send_wayland_pointer_scroll(
    wayland_evt_write: Option<u32>,
    surface_id: u32,
    dx: i16,
    dy: i16,
    timestamp_ns: u64,
) {
    let Some(evt_write) = wayland_evt_write else {
        return;
    };
    let msg = ipc::encode_pointer_scroll(surface_id, dx, dy, timestamp_ns);
    let _ = port_send_all(evt_write, &msg);
}

fn send_wayland_keyboard_enter(wayland_evt_write: Option<u32>, surface_id: u32, modifiers: u8) {
    let Some(evt_write) = wayland_evt_write else {
        return;
    };
    let msg = ipc::encode_keyboard_enter(surface_id, modifiers);
    let _ = port_send_all(evt_write, &msg);
}

fn send_wayland_keyboard_leave(wayland_evt_write: Option<u32>, surface_id: u32) {
    let Some(evt_write) = wayland_evt_write else {
        return;
    };
    let msg = ipc::encode_keyboard_leave(surface_id);
    let _ = port_send_all(evt_write, &msg);
}

fn send_wayland_keyboard_key(
    wayland_evt_write: Option<u32>,
    surface_id: u32,
    key: u16,
    pressed: bool,
    modifiers: u8,
    repeat: bool,
    timestamp_ns: u64,
) {
    let Some(evt_write) = wayland_evt_write else {
        return;
    };
    let msg = ipc::encode_keyboard_key(surface_id, key, pressed, modifiers, repeat, timestamp_ns);
    let _ = port_send_all(evt_write, &msg);
}

fn surface_local_position(scene: &Scene, surface_id: u32, x: i32, y: i32) -> (i32, i32) {
    if let Some(rect) = scene.surface_rect(surface_id) {
        (x.saturating_sub(rect.x as i32), y.saturating_sub(rect.y as i32))
    } else {
        (x, y)
    }
}

#[inline]
fn pressed_flag(pressed: bool) -> u8 {
    if pressed { 1 } else { 0 }
}

fn is_pointer_overlay_toggle(key: KeyEventPayload) -> bool {
    key.key() == Key::F7 && key.mods().has_alt()
}

struct BloomLauncher;

impl blossom::runbox::Launcher for BloomLauncher {
    fn run(&self, command: &str) -> blossom::runbox::LaunchResult {
        let mut parts = command.split_ascii_whitespace();
        let Some(program) = parts.next() else {
            return blossom::runbox::LaunchResult::Failed;
        };
        let Some(path) = resolve_run_command(program) else {
            return blossom::runbox::LaunchResult::NotFound;
        };

        let mut argv = Vec::new();
        argv.push(path.as_bytes().to_vec());
        for arg in parts {
            argv.push(arg.as_bytes().to_vec());
        }
        let argv_slices: Vec<&[u8]> = argv.iter().map(|arg| arg.as_slice()).collect();
        let env = BTreeMap::new();

        match stem::syscall::spawn_process_ex(
            &path,
            &argv_slices,
            &env,
            abi::types::stdio_mode::INHERIT,
            abi::types::stdio_mode::INHERIT,
            abi::types::stdio_mode::INHERIT,
            0,
            &[],
        ) {
            Ok(resp) => blossom::runbox::LaunchResult::Started { pid: resp.child_pid as u64 },
            Err(_) => blossom::runbox::LaunchResult::Failed,
        }
    }
}

fn resolve_run_command(program: &str) -> Option<String> {
    if program.contains('/') {
        return file_exists(program).then(|| program.to_string());
    }
    for prefix in ["/bin", "/app", "/applications"] {
        let path = alloc::format!("{}/{}", prefix, program);
        if file_exists(&path) {
            return Some(path);
        }
    }
    None
}

fn file_exists(path: &str) -> bool {
    match vfs_open(path, abi::syscall::vfs_flags::O_RDONLY) {
        Ok(fd) => {
            let _ = vfs_close(fd);
            true
        }
        Err(_) => false,
    }
}

/// Deliver a serialized Bloom event to a client.
///
/// Prefers typed inbox delivery when the client registered `input_pid`
/// via `CONNECT_INBOX`; otherwise falls back to legacy event-port delivery.
/// Send failures are intentionally best-effort and drop the event to keep
/// compositor input handling non-blocking under backpressure.
fn send_client_event(scene: &Scene, client_id: u32, kind: KindId, payload: &[u8]) {
    if let Some(pid) = scene.client_inbox_pid(client_id) {
        let _ = msg_send(pid, kind, payload);
    } else if let Some(ch) = scene.client_event_port(client_id) {
        let _ = port_send_all(ch, payload);
    }
}

fn send_close_requested_to_owner(scene: &Scene, surface_id: u32) {
    let Some(client_id) = scene.surface_client(surface_id) else {
        return;
    };
    let Some(pid) = scene.client_inbox_pid(client_id) else {
        return;
    };

    let event = ui_event::UiEvent::CloseRequested { window: surface_id as u64 };
    let mut buf = [0u8; 32];
    let Some(len) = ui_event::encode(&event, &mut buf) else {
        return;
    };
    match msg_send(pid, KIND_UI_EVENT, &buf[..len]) {
        Ok(()) => {
            stem::debug!("Sent CloseRequested to app inbox pid={} surface={}", pid, surface_id)
        }
        Err(err) => stem::warn!(
            "Failed to send CloseRequested to app inbox pid={} surface={} err={:?}",
            pid,
            surface_id,
            err
        ),
    }
}
