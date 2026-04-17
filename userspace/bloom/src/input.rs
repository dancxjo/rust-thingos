use abi::hid::{
    BristleEventHeader, EventType, KeyEventPayload, PointerButtonPayload, PointerMovePayload,
};
use stem::syscall::channel_send_all;

use crate::damage::DamageTracker;
use crate::protocol::{
    KeyboardEnterEvent, KeyboardKeyEvent, KeyboardLeaveEvent, PointerButtonEvent, PointerEnterEvent,
    PointerLeaveEvent, PointerMotionEvent, EVT_KEYBOARD_ENTER, EVT_KEYBOARD_KEY, EVT_KEYBOARD_LEAVE,
    EVT_POINTER_BUTTON, EVT_POINTER_ENTER, EVT_POINTER_LEAVE, EVT_POINTER_MOTION, msg_header, to_vec,
};
use crate::scene::Scene;

pub struct InputState {
    pointer_x: i32,
    pointer_y: i32,
    output_w: i32,
    output_h: i32,
}

impl InputState {
    pub fn new(output_w: u32, output_h: u32) -> Self {
        Self {
            pointer_x: 0,
            pointer_y: 0,
            output_w: output_w as i32,
            output_h: output_h as i32,
        }
    }

    pub fn handle_bristle_event(&mut self, bytes: &[u8], scene: &mut Scene, damage: &mut DamageTracker) {
        if bytes.len() < BristleEventHeader::SIZE {
            return;
        }
        let mut hdr_bytes = [0u8; BristleEventHeader::SIZE];
        hdr_bytes.copy_from_slice(&bytes[..BristleEventHeader::SIZE]);
        let Ok(header) = BristleEventHeader::from_bytes(&hdr_bytes) else {
            return;
        };
        let payload = &bytes[BristleEventHeader::SIZE..];

        match EventType::from_raw(header.event_type) {
            Ok(EventType::PointerMove) if payload.len() >= PointerMovePayload::SIZE => {
                let mut p = [0u8; PointerMovePayload::SIZE];
                p.copy_from_slice(&payload[..PointerMovePayload::SIZE]);
                let move_ev = PointerMovePayload::from_bytes(&p);
                self.pointer_x = (self.pointer_x + move_ev.dx as i32).clamp(0, self.output_w.saturating_sub(1));
                self.pointer_y = (self.pointer_y + move_ev.dy as i32).clamp(0, self.output_h.saturating_sub(1));
                self.update_pointer_focus(scene);
                if let Some(surface_id) = scene.pointer_focus {
                    if let Some(ch) = scene
                        .surface_client(surface_id)
                        .and_then(|client| scene.client_event_channel(client))
                    {
                        let ev = PointerMotionEvent {
                            header: msg_header(EVT_POINTER_MOTION),
                            surface_id,
                            x: self.pointer_x,
                            y: self.pointer_y,
                            timestamp_ns: header.timestamp_ns,
                        };
                        let _ = channel_send_all(ch, &to_vec(&ev));
                    }
                }
                damage.mark_dirty();
            }
            Ok(EventType::PointerButtonDown) if payload.len() >= PointerButtonPayload::SIZE => {
                let mut p = [0u8; PointerButtonPayload::SIZE];
                p.copy_from_slice(&payload[..PointerButtonPayload::SIZE]);
                let btn = PointerButtonPayload::from_bytes(&p);
                self.update_pointer_focus(scene);
                let old_focus = scene.keyboard_focus;
                scene.keyboard_focus = scene.pointer_focus;
                self.send_keyboard_focus_events(scene, old_focus, scene.keyboard_focus);
                if let Some(surface_id) = scene.pointer_focus {
                    if let Some(ch) = scene
                        .surface_client(surface_id)
                        .and_then(|client| scene.client_event_channel(client))
                    {
                        let ev = PointerButtonEvent {
                            header: msg_header(EVT_POINTER_BUTTON),
                            surface_id,
                            button: btn.button,
                            pressed: pressed_flag(true),
                            _pad: [0; 2],
                            timestamp_ns: header.timestamp_ns,
                        };
                        let _ = channel_send_all(ch, &to_vec(&ev));
                    }
                }
                damage.mark_dirty();
            }
            Ok(EventType::PointerButtonUp) if payload.len() >= PointerButtonPayload::SIZE => {
                let mut p = [0u8; PointerButtonPayload::SIZE];
                p.copy_from_slice(&payload[..PointerButtonPayload::SIZE]);
                let btn = PointerButtonPayload::from_bytes(&p);
                self.update_pointer_focus(scene);
                if let Some(surface_id) = scene.pointer_focus {
                    if let Some(ch) = scene
                        .surface_client(surface_id)
                        .and_then(|client| scene.client_event_channel(client))
                    {
                        let ev = PointerButtonEvent {
                            header: msg_header(EVT_POINTER_BUTTON),
                            surface_id,
                            button: btn.button,
                            pressed: pressed_flag(false),
                            _pad: [0; 2],
                            timestamp_ns: header.timestamp_ns,
                        };
                        let _ = channel_send_all(ch, &to_vec(&ev));
                    }
                }
                damage.mark_dirty();
            }
            Ok(EventType::KeyDown) if payload.len() >= KeyEventPayload::SIZE => {
                let mut p = [0u8; KeyEventPayload::SIZE];
                p.copy_from_slice(&payload[..KeyEventPayload::SIZE]);
                let key = KeyEventPayload::from_bytes(&p);
                if let Some(surface_id) = scene.keyboard_focus {
                    if let Some(ch) = scene
                        .surface_client(surface_id)
                        .and_then(|client| scene.client_event_channel(client))
                    {
                        let ev = KeyboardKeyEvent {
                            header: msg_header(EVT_KEYBOARD_KEY),
                            surface_id,
                            key: key.key,
                            pressed: pressed_flag(true),
                            modifiers: key.mods,
                            repeat: if key.is_repeat() { 1 } else { 0 },
                            _pad: [0; 3],
                            timestamp_ns: header.timestamp_ns,
                        };
                        let _ = channel_send_all(ch, &to_vec(&ev));
                    }
                }
            }
            Ok(EventType::KeyUp) if payload.len() >= KeyEventPayload::SIZE => {
                let mut p = [0u8; KeyEventPayload::SIZE];
                p.copy_from_slice(&payload[..KeyEventPayload::SIZE]);
                let key = KeyEventPayload::from_bytes(&p);
                if let Some(surface_id) = scene.keyboard_focus {
                    if let Some(ch) = scene
                        .surface_client(surface_id)
                        .and_then(|client| scene.client_event_channel(client))
                    {
                        let ev = KeyboardKeyEvent {
                            header: msg_header(EVT_KEYBOARD_KEY),
                            surface_id,
                            key: key.key,
                            pressed: pressed_flag(false),
                            modifiers: key.mods,
                            repeat: if key.is_repeat() { 1 } else { 0 },
                            _pad: [0; 3],
                            timestamp_ns: header.timestamp_ns,
                        };
                        let _ = channel_send_all(ch, &to_vec(&ev));
                    }
                }
            }
            _ => {}
        }
    }

    fn update_pointer_focus(&mut self, scene: &mut Scene) {
        let new_focus = scene.top_surface_at(self.pointer_x, self.pointer_y);
        if scene.pointer_focus == new_focus {
            return;
        }

        if let Some(old_surface) = scene.pointer_focus {
            if let Some(ch) = scene
                .surface_client(old_surface)
                .and_then(|client| scene.client_event_channel(client))
            {
                let ev = PointerLeaveEvent {
                    header: msg_header(EVT_POINTER_LEAVE),
                    surface_id: old_surface,
                };
                let _ = channel_send_all(ch, &to_vec(&ev));
            }
        }

        scene.pointer_focus = new_focus;
        if let Some(surface_id) = new_focus {
            if let Some(ch) = scene
                .surface_client(surface_id)
                .and_then(|client| scene.client_event_channel(client))
            {
                let ev = PointerEnterEvent {
                    header: msg_header(EVT_POINTER_ENTER),
                    surface_id,
                    x: self.pointer_x,
                    y: self.pointer_y,
                };
                let _ = channel_send_all(ch, &to_vec(&ev));
            }
        }
    }

    fn send_keyboard_focus_events(
        &self,
        scene: &mut Scene,
        old_focus: Option<u32>,
        new_focus: Option<u32>,
    ) {
        if old_focus == new_focus {
            return;
        }

        if let Some(old_surface) = old_focus {
            if let Some(ch) = scene
                .surface_client(old_surface)
                .and_then(|client| scene.client_event_channel(client))
            {
                let ev = KeyboardLeaveEvent {
                    header: msg_header(EVT_KEYBOARD_LEAVE),
                    surface_id: old_surface,
                };
                let _ = channel_send_all(ch, &to_vec(&ev));
            }
        }

        if let Some(new_surface) = new_focus {
            if let Some(ch) = scene
                .surface_client(new_surface)
                .and_then(|client| scene.client_event_channel(client))
            {
                let ev = KeyboardEnterEvent {
                    header: msg_header(EVT_KEYBOARD_ENTER),
                    surface_id: new_surface,
                    modifiers: 0,
                    _pad: [0; 3],
                };
                let _ = channel_send_all(ch, &to_vec(&ev));
            }
        }
    }
}

#[inline]
fn pressed_flag(pressed: bool) -> u8 {
    if pressed { 1 } else { 0 }
}
