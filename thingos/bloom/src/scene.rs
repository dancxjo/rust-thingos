use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use abi::display_protocol::Rect;

#[derive(Clone, Copy, Debug)]
pub struct Client {
    pub id: u32,
    pub event_port: u32,
    pub input_pid: Option<u32>,
}

#[derive(Clone, Copy, Debug)]
pub struct SurfaceBuffer {
    pub buffer_id: u32,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
}

#[derive(Clone, Debug, Default)]
pub struct SurfacePending {
    pub buffer: Option<SurfaceBuffer>,
    pub damage: Vec<Rect>,
    pub dest_rect: Option<Rect>,
    pub input_region: Option<Rect>,
    pub opaque_region: Option<Rect>,
    pub z_order: Option<i32>,
}

#[derive(Clone, Debug, Default)]
pub struct SurfaceCurrent {
    pub buffer: Option<SurfaceBuffer>,
    pub damage: Vec<Rect>,
    pub dest_rect: Rect,
    pub input_region: Option<Rect>,
    pub opaque_region: Option<Rect>,
    pub z_order: i32,
}

#[derive(Clone, Debug)]
pub struct Surface {
    pub id: u32,
    pub client_id: u32,
    pub current: SurfaceCurrent,
    pub pending: SurfacePending,
    pub mapped: bool,
    pub visible: bool,
    pub focus_eligible: bool,
    pub frame_serial: u64,
}

#[derive(Clone, Copy, Debug)]
pub struct CompositionEntry {
    pub surface_id: u32,
    pub buffer_id: u32,
    pub src_rect: Rect,
    pub dest_rect: Rect,
    pub z_order: i32,
    pub alpha: u8,
}

pub struct Scene {
    next_client_id: u32,
    next_surface_id: u32,
    clients: BTreeMap<u32, Client>,
    surfaces: BTreeMap<u32, Surface>,
    pub pointer_focus: Option<u32>,
    pub keyboard_focus: Option<u32>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            next_client_id: 1,
            next_surface_id: 1,
            clients: BTreeMap::new(),
            surfaces: BTreeMap::new(),
            pointer_focus: None,
            keyboard_focus: None,
        }
    }

    pub fn register_client(&mut self, event_port: u32, input_pid: Option<u32>) -> u32 {
        let id = self.next_client_id;
        self.next_client_id = self.next_client_id.saturating_add(1);
        self.clients.insert(id, Client { id, event_port, input_pid });
        id
    }

    pub fn update_client_port(&mut self, client_id: u32, event_port: u32) -> bool {
        if let Some(client) = self.clients.get_mut(&client_id) {
            client.event_port = event_port;
            true
        } else {
            false
        }
    }

    pub fn client_event_port(&self, client_id: u32) -> Option<u32> {
        self.clients.get(&client_id).map(|c| c.event_port)
    }

    pub fn client_inbox_pid(&self, client_id: u32) -> Option<u32> {
        self.clients.get(&client_id).and_then(|c| c.input_pid)
    }

    pub fn create_surface(&mut self, client_id: u32) -> Option<u32> {
        if !self.clients.contains_key(&client_id) {
            return None;
        }
        let id = self.next_surface_id;
        self.next_surface_id = self.next_surface_id.saturating_add(1);
        self.surfaces.insert(
            id,
            Surface {
                id,
                client_id,
                current: SurfaceCurrent::default(),
                pending: SurfacePending::default(),
                mapped: false,
                visible: true,
                focus_eligible: true,
                frame_serial: 0,
            },
        );
        Some(id)
    }

    pub fn destroy_surface(&mut self, client_id: u32, surface_id: u32) -> Option<Vec<u32>> {
        let surface = self.surfaces.get(&surface_id)?;
        if surface.client_id != client_id {
            return None;
        }
        let removed = self.surfaces.remove(&surface_id)?;
        let mut release = Vec::new();
        if let Some(buf) = removed.current.buffer {
            release.push(buf.buffer_id);
        }
        if let Some(buf) = removed.pending.buffer {
            release.push(buf.buffer_id);
        }
        if self.pointer_focus == Some(surface_id) {
            self.pointer_focus = None;
        }
        if self.keyboard_focus == Some(surface_id) {
            self.keyboard_focus = None;
        }
        Some(release)
    }

    pub fn attach_pending_buffer(
        &mut self,
        client_id: u32,
        surface_id: u32,
        buffer: SurfaceBuffer,
    ) -> Option<Option<u32>> {
        let surface = self.surfaces.get_mut(&surface_id)?;
        if surface.client_id != client_id {
            return None;
        }
        let old = surface.pending.buffer.replace(buffer).map(|b| b.buffer_id);
        Some(old)
    }

    pub fn damage_pending(&mut self, client_id: u32, surface_id: u32, rect: Rect) -> bool {
        let Some(surface) = self.surfaces.get_mut(&surface_id) else {
            return false;
        };
        if surface.client_id != client_id {
            return false;
        }
        surface.pending.damage.push(rect);
        true
    }

    pub fn set_pending_input_region(
        &mut self,
        client_id: u32,
        surface_id: u32,
        rect: Rect,
    ) -> bool {
        let Some(surface) = self.surfaces.get_mut(&surface_id) else {
            return false;
        };
        if surface.client_id != client_id {
            return false;
        }
        surface.pending.input_region = Some(rect);
        true
    }

    pub fn set_pending_opaque_region(
        &mut self,
        client_id: u32,
        surface_id: u32,
        rect: Rect,
    ) -> bool {
        let Some(surface) = self.surfaces.get_mut(&surface_id) else {
            return false;
        };
        if surface.client_id != client_id {
            return false;
        }
        surface.pending.opaque_region = Some(rect);
        true
    }

    pub fn set_pending_dest_rect(&mut self, client_id: u32, surface_id: u32, rect: Rect) -> bool {
        let Some(surface) = self.surfaces.get_mut(&surface_id) else {
            return false;
        };
        if surface.client_id != client_id {
            return false;
        }
        surface.pending.dest_rect = Some(rect);
        true
    }

    pub fn set_pending_z_order(&mut self, client_id: u32, surface_id: u32, z_order: i32) -> bool {
        let Some(surface) = self.surfaces.get_mut(&surface_id) else {
            return false;
        };
        if surface.client_id != client_id {
            return false;
        }
        surface.pending.z_order = Some(z_order);
        true
    }

    pub fn commit_surface(&mut self, client_id: u32, surface_id: u32) -> Option<CommitResult> {
        let surface = self.surfaces.get_mut(&surface_id)?;
        if surface.client_id != client_id {
            return None;
        }

        let mut released = Vec::new();
        let mut changed = false;
        let mut visual_full_damage = false;
        let mut damage_rects = Vec::new();
        let old_dest = surface.current.dest_rect;
        let old_mapped = surface.visible && surface.mapped && surface.current.buffer.is_some();
        let pending_dest = surface.pending.dest_rect.take();

        if let Some(pending_buf) = surface.pending.buffer.take() {
            let had_current = surface.current.buffer.is_some();
            if let Some(current_buf) = surface.current.buffer.take() {
                if current_buf.buffer_id != pending_buf.buffer_id {
                    released.push(current_buf.buffer_id);
                }
            }
            if pending_dest.is_none() && !had_current {
                surface.current.dest_rect =
                    Rect { x: 0, y: 0, w: pending_buf.width, h: pending_buf.height };
            }
            surface.current.buffer = Some(pending_buf);
            surface.mapped = true;
            changed = true;
            visual_full_damage = true;
        }

        if let Some(dest) = pending_dest {
            if old_mapped {
                damage_rects.push(old_dest);
            }
            surface.current.dest_rect = dest;
            damage_rects.push(dest);
            changed = true;
        }
        if let Some(z) = surface.pending.z_order.take() {
            surface.current.z_order = z;
            changed = true;
            visual_full_damage = true;
        }
        if let Some(region) = surface.pending.input_region.take() {
            surface.current.input_region = Some(region);
            changed = true;
        }
        if let Some(region) = surface.pending.opaque_region.take() {
            surface.current.opaque_region = Some(region);
            changed = true;
        }
        if !surface.pending.damage.is_empty() {
            if let Some(buf) = surface.current.buffer {
                let dest = surface.current.dest_rect;
                for rect in surface.pending.damage.drain(..) {
                    if let Some(rect) = translate_surface_damage(rect, buf.width, buf.height, dest)
                    {
                        damage_rects.push(rect);
                    }
                }
            } else {
                surface.pending.damage.clear();
            }
            changed = true;
        }
        if visual_full_damage {
            damage_rects.push(surface.current.dest_rect);
        }

        surface.frame_serial = surface.frame_serial.saturating_add(1);
        Some(CommitResult {
            changed,
            released_buffer_ids: released,
            frame_serial: surface.frame_serial,
            damage_rects,
            needs_full_repaint: visual_full_damage && old_mapped,
        })
    }

    pub fn collect_composition(&self) -> Vec<CompositionEntry> {
        let mut list = Vec::new();
        for surface in self.surfaces.values() {
            if !surface.visible || !surface.mapped {
                continue;
            }
            let Some(buf) = surface.current.buffer else {
                continue;
            };
            list.push(CompositionEntry {
                surface_id: surface.id,
                buffer_id: buf.buffer_id,
                src_rect: Rect { x: 0, y: 0, w: buf.width, h: buf.height },
                dest_rect: surface.current.dest_rect,
                z_order: surface.current.z_order,
                alpha: 255,
            });
        }
        list.sort_by_key(|entry| entry.z_order);
        list
    }

    pub fn top_surface_at(&self, x: i32, y: i32) -> Option<u32> {
        let mut best: Option<(u32, i32)> = None;
        for surface in self.surfaces.values() {
            if !surface.visible || !surface.mapped || !surface.focus_eligible {
                continue;
            }
            let rect = surface.current.input_region.unwrap_or(surface.current.dest_rect);
            let max_x = rect.x.saturating_add(rect.w) as i32;
            let max_y = rect.y.saturating_add(rect.h) as i32;
            let inside = x >= rect.x as i32 && y >= rect.y as i32 && x < max_x && y < max_y;
            if !inside {
                continue;
            }
            match best {
                Some((_, z)) if z > surface.current.z_order => {}
                _ => best = Some((surface.id, surface.current.z_order)),
            }
        }
        best.map(|(id, _)| id)
    }

    pub fn surface_client(&self, surface_id: u32) -> Option<u32> {
        self.surfaces.get(&surface_id).map(|s| s.client_id)
    }
}

pub struct CommitResult {
    pub changed: bool,
    pub released_buffer_ids: Vec<u32>,
    pub frame_serial: u64,
    pub damage_rects: Vec<Rect>,
    pub needs_full_repaint: bool,
}

fn translate_surface_damage(local: Rect, src_w: u32, src_h: u32, dest: Rect) -> Option<Rect> {
    if src_w == 0 || src_h == 0 || dest.w == 0 || dest.h == 0 || local.w == 0 || local.h == 0 {
        return None;
    }

    let x0 = local.x.min(src_w);
    let y0 = local.y.min(src_h);
    let x1 = local.x.saturating_add(local.w).min(src_w);
    let y1 = local.y.saturating_add(local.h).min(src_h);
    if x1 <= x0 || y1 <= y0 {
        return None;
    }

    let dx0 = dest.x.saturating_add(((x0 as u64 * dest.w as u64) / src_w as u64) as u32);
    let dy0 = dest.y.saturating_add(((y0 as u64 * dest.h as u64) / src_h as u64) as u32);
    let dx1 = dest
        .x
        .saturating_add((((x1 as u64 * dest.w as u64) + src_w as u64 - 1) / src_w as u64) as u32);
    let dy1 = dest
        .y
        .saturating_add((((y1 as u64 * dest.h as u64) + src_h as u64 - 1) / src_h as u64) as u32);

    if dx1 <= dx0 || dy1 <= dy0 {
        None
    } else {
        Some(Rect { x: dx0, y: dy0, w: dx1 - dx0, h: dy1 - dy0 })
    }
}
