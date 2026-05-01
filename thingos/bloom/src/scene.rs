use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use abi::display_protocol::Rect;

const CASCADE_START_X: u32 = 40;
const CASCADE_START_Y: u32 = 48;
const CASCADE_STEP_X: u32 = 36;
const CASCADE_STEP_Y: u32 = 32;
const CASCADE_SLOTS: u32 = 8;

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
    pub chrome: SurfaceChrome,
    pub title: Option<String>,
    pub frame_serial: u64,
    pub is_fullscreen: bool,
    pub is_shaded: bool,
    pub restored_rect: Option<Rect>,
    /// Subsurface relationship (if this surface has been assigned the
    /// `wl_subsurface` role).  `None` for top-level / standalone surfaces.
    pub subsurface: Option<SubsurfaceLink>,
    /// Ordered list of child surface IDs for which this surface is the parent.
    /// Order is bottom-to-top stacking.  Empty for surfaces with no children.
    pub subsurface_children: Vec<u32>,
}

#[derive(Clone, Copy, Debug)]
pub struct SubsurfaceLink {
    pub parent_id: u32,
    pub x: i32,
    pub y: i32,
    /// Stacking offset relative to the parent's z_order.
    pub z_above: i32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SurfaceChrome {
    pub titlebar_height: u32,
    pub frame_thickness: u32,
}

impl SurfaceChrome {
    pub fn is_empty(self) -> bool {
        self.titlebar_height == 0 && self.frame_thickness == 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HitTarget {
    Client { surface_id: u32 },
    TitleBar { surface_id: u32 },
    ChromeButton { surface_id: u32, button: ChromeButton },
    Frame { surface_id: u32, edge: ResizeEdge },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChromeButton {
    Minimize,
    Shade,
    Maximize,
    Fullscreen,
    Close,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResizeEdge {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CursorKind {
    #[default]
    Default,
    Move,
    ResizeNorth,
    ResizeSouth,
    ResizeEast,
    ResizeWest,
    ResizeNorthEast,
    ResizeNorthWest,
    ResizeSouthEast,
    ResizeSouthWest,
}

impl CursorKind {
    pub fn for_resize_edge(edge: ResizeEdge) -> Self {
        match edge {
            ResizeEdge::North => Self::ResizeNorth,
            ResizeEdge::South => Self::ResizeSouth,
            ResizeEdge::East => Self::ResizeEast,
            ResizeEdge::West => Self::ResizeWest,
            ResizeEdge::NorthEast => Self::ResizeNorthEast,
            ResizeEdge::NorthWest => Self::ResizeNorthWest,
            ResizeEdge::SouthEast => Self::ResizeSouthEast,
            ResizeEdge::SouthWest => Self::ResizeSouthWest,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CompositionEntry {
    pub surface_id: u32,
    pub buffer_id: u32,
    pub src_rect: Rect,
    pub dest_rect: Rect,
    pub z_order: i32,
    pub alpha: u8,
    pub chrome: SurfaceChrome,
    pub active: bool,
    pub title: Option<String>,
    pub is_fullscreen: bool,
    pub is_shaded: bool,
    /// The committed opaque region in surface-local coordinates, if any.
    /// A rect that covers the full `src_rect` means the surface is fully opaque.
    pub opaque_region: Option<Rect>,
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
                chrome: SurfaceChrome::default(),
                title: None,
                frame_serial: 0,
                is_fullscreen: false,
                is_shaded: false,
                restored_rect: None,
                subsurface: None,
                subsurface_children: Vec::new(),
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
        // Detach from any parent's children list and orphan any of our own
        // subsurface children so we don't leave dangling parent references.
        if let Some(link) = removed.subsurface {
            if let Some(parent) = self.surfaces.get_mut(&link.parent_id) {
                parent.subsurface_children.retain(|&id| id != surface_id);
            }
        }
        for child_id in &removed.subsurface_children {
            if let Some(child) = self.surfaces.get_mut(child_id) {
                child.subsurface = None;
            }
        }
        if self.pointer_focus == Some(surface_id) {
            self.pointer_focus = None;
        }
        if self.keyboard_focus == Some(surface_id) {
            self.keyboard_focus = None;
        }
        Some(release)
    }

    /// Establish or update a subsurface relationship.
    ///
    /// `parent_id == None` (or `Some(0)`) detaches the child from its previous
    /// parent.  When attached, the child's `dest_rect` is recomputed to be
    /// `parent.dest_rect + (x, y)` and its `z_order` is set to
    /// `parent.z_order + z_above`.
    pub fn set_subsurface(
        &mut self,
        child_id: u32,
        parent_id: Option<u32>,
        x: i32,
        y: i32,
        z_above: i32,
    ) -> bool {
        if !self.surfaces.contains_key(&child_id) {
            return false;
        }

        // First, detach from the previous parent if any.
        let prev_parent =
            self.surfaces.get(&child_id).and_then(|s| s.subsurface.map(|l| l.parent_id));
        if let Some(prev) = prev_parent {
            if Some(prev) != parent_id {
                if let Some(p) = self.surfaces.get_mut(&prev) {
                    p.subsurface_children.retain(|&id| id != child_id);
                }
            }
        }

        let parent_id = parent_id.filter(|&p| p != 0);
        let Some(parent_id) = parent_id else {
            // Detach.
            if let Some(child) = self.surfaces.get_mut(&child_id) {
                child.subsurface = None;
            }
            return true;
        };
        if !self.surfaces.contains_key(&parent_id) || parent_id == child_id {
            return false;
        }

        // Add to the parent's children list (if not already present).
        if let Some(parent) = self.surfaces.get_mut(&parent_id) {
            if !parent.subsurface_children.contains(&child_id) {
                parent.subsurface_children.push(child_id);
            }
        }

        // Update child link and recompute absolute dest_rect / z_order.
        let parent_rect =
            self.surfaces.get(&parent_id).map(|p| p.current.dest_rect).unwrap_or_default();
        let parent_z = self.surfaces.get(&parent_id).map(|p| p.current.z_order).unwrap_or(0);

        if let Some(child) = self.surfaces.get_mut(&child_id) {
            child.subsurface = Some(SubsurfaceLink { parent_id, x, y, z_above });
            let new_x = parent_rect.x as i32 + x;
            let new_y = parent_rect.y as i32 + y;
            let w = child.current.dest_rect.w;
            let h = child.current.dest_rect.h;
            child.current.dest_rect = Rect { x: new_x.max(0) as u32, y: new_y.max(0) as u32, w, h };
            child.current.z_order = parent_z.saturating_add(z_above);
            // Subsurfaces are not focusable as standalone toplevels.
            child.focus_eligible = false;
            // Subsurfaces have no compositor-drawn chrome.
            child.chrome = SurfaceChrome::default();
        }
        true
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

    pub fn set_surface_chrome(
        &mut self,
        client_id: u32,
        surface_id: u32,
        chrome: SurfaceChrome,
    ) -> bool {
        let Some(surface) = self.surfaces.get_mut(&surface_id) else {
            return false;
        };
        if surface.client_id != client_id {
            return false;
        }
        surface.chrome = chrome;
        true
    }

    pub fn set_surface_title(&mut self, client_id: u32, surface_id: u32, title: String) -> bool {
        let Some(surface) = self.surfaces.get_mut(&surface_id) else {
            return false;
        };
        if surface.client_id != client_id {
            return false;
        }
        surface.title = if title.is_empty() { None } else { Some(title) };
        true
    }

    pub fn move_surface_absolute(
        &mut self,
        surface_id: u32,
        x: i32,
        y: i32,
    ) -> Option<SurfaceMove> {
        let surface = self.surfaces.get_mut(&surface_id)?;
        let old_rect = surface.current.dest_rect;
        let new_rect = Rect { x: x.max(0) as u32, y: y.max(0) as u32, ..old_rect };
        if old_rect.x == new_rect.x && old_rect.y == new_rect.y {
            return Some(SurfaceMove { old_rect, new_rect, changed: false });
        }
        surface.current.dest_rect = new_rect;
        Some(SurfaceMove { old_rect, new_rect, changed: true })
    }

    pub fn resize_surface_absolute(
        &mut self,
        surface_id: u32,
        rect: Rect,
    ) -> Option<SurfaceResize> {
        let surface = self.surfaces.get_mut(&surface_id)?;
        let old_rect = surface.current.dest_rect;
        if old_rect == rect {
            return Some(SurfaceResize { old_rect, new_rect: rect, changed: false });
        }
        surface.current.dest_rect = rect;
        Some(SurfaceResize { old_rect, new_rect: rect, changed: true })
    }

    pub fn toggle_surface_shaded(&mut self, surface_id: u32) -> Option<SurfaceToggle> {
        let surface = self.surfaces.get_mut(&surface_id)?;
        let old_rect = surface.current.dest_rect;
        let mut new_rect = old_rect;

        if surface.is_shaded {
            if let Some(restored) = surface.restored_rect.take() {
                new_rect = restored;
            }
            surface.is_shaded = false;
        } else {
            surface.restored_rect = Some(old_rect);
            surface.is_shaded = true;
            new_rect.h = surface.chrome.titlebar_height;
        }

        let changed = old_rect != new_rect;
        if changed {
            surface.current.dest_rect = new_rect;
        }

        Some(SurfaceToggle { old_rect, new_rect, changed, active: surface.is_shaded })
    }

    pub fn toggle_surface_fullscreen(
        &mut self,
        surface_id: u32,
        fullscreen_rect: Rect,
    ) -> Option<SurfaceToggle> {
        let surface = self.surfaces.get_mut(&surface_id)?;
        let old_rect = surface.current.dest_rect;
        let new_rect;

        if surface.is_fullscreen {
            new_rect = surface.restored_rect.take().unwrap_or(old_rect);
            surface.is_fullscreen = false;
        } else {
            surface.restored_rect = Some(old_rect);
            surface.is_fullscreen = true;
            new_rect = fullscreen_rect;
        }

        let changed = old_rect != new_rect;
        if changed {
            surface.current.dest_rect = new_rect;
        }

        Some(SurfaceToggle { old_rect, new_rect, changed, active: surface.is_fullscreen })
    }

    pub fn commit_surface(
        &mut self,
        client_id: u32,
        surface_id: u32,
        screen_w: u32,
        screen_h: u32,
    ) -> Option<CommitResult> {
        let existing_rects: Vec<Rect> = self
            .surfaces
            .values()
            .filter(|surface| {
                surface.id != surface_id
                    && surface.subsurface.is_none()
                    && surface.visible
                    && surface.mapped
                    && surface.current.buffer.is_some()
            })
            .map(|s| s.current.dest_rect)
            .collect();

        let next_z = self
            .surfaces
            .values()
            .map(|surface| surface.current.z_order)
            .max()
            .unwrap_or(0)
            .saturating_add(1);

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
                surface.current.dest_rect = if surface.subsurface.is_some() {
                    Rect {
                        x: surface.current.dest_rect.x,
                        y: surface.current.dest_rect.y,
                        w: pending_buf.width,
                        h: pending_buf.height,
                    }
                } else {
                    airy_window_rect(
                        screen_w,
                        screen_h,
                        &existing_rects,
                        pending_buf.width,
                        pending_buf.height,
                    )
                };
                if surface.current.z_order == 0 && surface.pending.z_order.is_none() {
                    surface.current.z_order = next_z;
                }
            }
            surface.current.buffer = Some(pending_buf);
            surface.mapped = true;
            changed = true;
            visual_full_damage = true;
        }

        if let Some(dest) = pending_dest {
            if old_mapped {
                damage_rects.push(surface_visual_rect(old_dest, surface.chrome));
            }
            surface.current.dest_rect = dest;
            damage_rects.push(surface_visual_rect(dest, surface.chrome));
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
            damage_rects.push(surface_visual_rect(surface.current.dest_rect, surface.chrome));
        }

        surface.frame_serial = surface.frame_serial.saturating_add(1);
        let frame_serial = surface.frame_serial;
        let parent_dest = surface.current.dest_rect;
        let parent_z = surface.current.z_order;
        let dest_changed_for_children = changed;

        // Propagate parent-relative offsets to subsurface children whenever
        // the parent's geometry has been touched.
        if dest_changed_for_children {
            self.propagate_subsurface_layout(surface_id, parent_dest, parent_z);
        }

        Some(CommitResult {
            changed,
            released_buffer_ids: released,
            frame_serial,
            damage_rects,
            needs_full_repaint: visual_full_damage && old_mapped,
        })
    }

    /// Recompute absolute `dest_rect` and `z_order` for every subsurface child
    /// of the given parent using the parent's current geometry.  Recurses so
    /// nested subsurface trees stay consistent.
    fn propagate_subsurface_layout(&mut self, parent_id: u32, parent_dest: Rect, parent_z: i32) {
        let children: Vec<u32> = match self.surfaces.get(&parent_id) {
            Some(p) => p.subsurface_children.clone(),
            None => return,
        };
        for child_id in children {
            let (link, w, h) = match self.surfaces.get(&child_id) {
                Some(child) => match child.subsurface {
                    Some(link) => (link, child.current.dest_rect.w, child.current.dest_rect.h),
                    None => continue,
                },
                None => continue,
            };
            let new_x = parent_dest.x as i32 + link.x;
            let new_y = parent_dest.y as i32 + link.y;
            let new_z = parent_z.saturating_add(link.z_above);
            let new_rect = Rect { x: new_x.max(0) as u32, y: new_y.max(0) as u32, w, h };
            if let Some(child) = self.surfaces.get_mut(&child_id) {
                child.current.dest_rect = new_rect;
                child.current.z_order = new_z;
            }
            // Recurse for grandchildren.
            self.propagate_subsurface_layout(child_id, new_rect, new_z);
        }
    }

    pub fn collect_composition(&self) -> Vec<CompositionEntry> {
        let mut list = Vec::new();
        let active_surface = self.keyboard_focus.or_else(|| {
            self.surfaces
                .values()
                .filter(|surface| surface.visible && surface.mapped && surface.focus_eligible)
                .max_by_key(|surface| surface.current.z_order)
                .map(|surface| surface.id)
        });
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
                chrome: surface.chrome,
                active: active_surface == Some(surface.id),
                title: surface.title.clone(),
                is_fullscreen: surface.is_fullscreen,
                is_shaded: surface.is_shaded,
                opaque_region: surface.current.opaque_region,
            });
        }
        list.sort_by_key(|entry| entry.z_order);
        list
    }

    pub fn top_surface_at(&self, x: i32, y: i32) -> Option<u32> {
        self.hit_test(x, y).map(|target| match target {
            HitTarget::Client { surface_id }
            | HitTarget::TitleBar { surface_id }
            | HitTarget::ChromeButton { surface_id, .. }
            | HitTarget::Frame { surface_id, .. } => surface_id,
        })
    }

    pub fn top_client_surface_at(&self, x: i32, y: i32) -> Option<u32> {
        match self.hit_test(x, y) {
            Some(HitTarget::Client { surface_id }) => Some(surface_id),
            _ => None,
        }
    }

    pub fn hit_test(&self, x: i32, y: i32) -> Option<HitTarget> {
        let mut best: Option<(u32, i32)> = None;
        for surface in self.surfaces.values() {
            if !surface.visible || !surface.mapped || !surface.focus_eligible {
                continue;
            }
            let rect = if surface.chrome.is_empty() {
                surface.current.input_region.unwrap_or(surface.current.dest_rect)
            } else {
                surface.current.dest_rect
            };
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
        let (surface_id, _) = best?;
        let surface = self.surfaces.get(&surface_id)?;
        let rect = surface.current.dest_rect;
        if surface.is_fullscreen {
            return Some(HitTarget::Client { surface_id });
        }
        if let Some(button) = chrome_button_at(rect, surface.chrome, x, y) {
            return Some(HitTarget::ChromeButton { surface_id, button });
        }
        if let Some(edge) = resize_edge_at(rect, surface.chrome.frame_thickness, x, y) {
            return Some(HitTarget::Frame { surface_id, edge });
        }
        let titlebar_bottom = rect.y.saturating_add(surface.chrome.titlebar_height);
        if surface.chrome.titlebar_height > 0
            && x >= rect.x as i32
            && x < rect.x.saturating_add(rect.w) as i32
            && y >= rect.y as i32
            && y < titlebar_bottom as i32
        {
            Some(HitTarget::TitleBar { surface_id })
        } else {
            Some(HitTarget::Client { surface_id })
        }
    }

    pub fn surface_client(&self, surface_id: u32) -> Option<u32> {
        self.surfaces.get(&surface_id).map(|s| s.client_id)
    }

    pub fn surface_rect(&self, surface_id: u32) -> Option<Rect> {
        self.surfaces.get(&surface_id).map(|s| s.current.dest_rect)
    }

    pub fn surface_visual_rect(&self, surface_id: u32) -> Option<Rect> {
        let surface = self.surfaces.get(&surface_id)?;
        Some(surface_visual_rect(surface.current.dest_rect, surface.chrome))
    }

    pub fn visual_rect_for_surface_rect(&self, surface_id: u32, rect: Rect) -> Option<Rect> {
        let surface = self.surfaces.get(&surface_id)?;
        Some(surface_visual_rect(rect, surface.chrome))
    }

    pub fn set_surface_visible(&mut self, surface_id: u32, visible: bool) -> Option<Rect> {
        let surface = self.surfaces.get_mut(&surface_id)?;
        if surface.visible == visible {
            return Some(surface.current.dest_rect);
        }
        surface.visible = visible;
        if !visible {
            if self.pointer_focus == Some(surface_id) {
                self.pointer_focus = None;
            }
            if self.keyboard_focus == Some(surface_id) {
                self.keyboard_focus = None;
            }
        }
        Some(surface.current.dest_rect)
    }

    pub fn surface_snapshots(&self) -> Vec<SurfaceSnapshot> {
        self.surfaces
            .values()
            .map(|surface| SurfaceSnapshot {
                id: surface.id,
                client_id: surface.client_id,
                rect: surface.current.dest_rect,
                z_order: surface.current.z_order,
                mapped: surface.mapped,
                visible: surface.visible,
                focus_eligible: surface.focus_eligible,
                buffer_id: surface.current.buffer.map(|buffer| buffer.buffer_id),
                title: surface.title.clone(),
                frame_serial: surface.frame_serial,
                is_window: surface.visible && surface.mapped && surface.current.buffer.is_some(),
            })
            .collect()
    }

    pub fn raise_to_top(&mut self, surface_id: u32) -> bool {
        let max_z = self.surfaces.values().map(|s| s.current.z_order).max().unwrap_or(0);
        let surfaces_len = self.surfaces.len();

        if let Some(surface) = self.surfaces.get_mut(&surface_id) {
            if surface.current.z_order < max_z || (surface.current.z_order == 0 && surfaces_len > 1) {
                let next_z = max_z.saturating_add(1);
                surface.current.z_order = next_z;
                let dest = surface.current.dest_rect;
                self.propagate_subsurface_layout(surface_id, dest, next_z);
                return true;
            }
        }
        false
    }

    pub fn cycle_focus(&mut self, forward: bool) -> (Option<u32>, Option<u32>) {
        let mut eligible: Vec<u32> = self
            .surfaces
            .values()
            .filter(|s| s.visible && s.mapped && s.focus_eligible)
            .map(|s| s.id)
            .collect();

        if eligible.is_empty() {
            return (self.keyboard_focus, None);
        }

        // Sort by Z-order descending (top-most first)
        eligible
            .sort_by_key(|id| core::cmp::Reverse(self.surfaces.get(id).unwrap().current.z_order));

        let current_idx =
            self.keyboard_focus.and_then(|id| eligible.iter().position(|&sid| sid == id));

        let new_idx = match current_idx {
            Some(idx) => {
                if forward {
                    (idx + 1) % eligible.len()
                } else {
                    (idx + eligible.len() - 1) % eligible.len()
                }
            }
            None => 0,
        };

        let old_focus = self.keyboard_focus;
        let new_focus = Some(eligible[new_idx]);
        self.keyboard_focus = new_focus;

        // Reorder Z-stack to enable cycling through all windows.
        // Forward (Alt+Tab): Sink the old focus to the bottom and raise the new one.
        // Backward (Alt+Shift+Tab): Just raise the new focus to the top.
        if let Some(id) = new_focus {
            let mut max_z = self.surfaces.values().map(|s| s.current.z_order).max().unwrap_or(0);
            let mut min_z = self.surfaces.values().map(|s| s.current.z_order).min().unwrap_or(0);

            if forward {
                if let Some(old_id) = old_focus {
                    if let Some(old_s) = self.surfaces.get_mut(&old_id) {
                        min_z = min_z.saturating_sub(1);
                        old_s.current.z_order = min_z;
                        let dest = old_s.current.dest_rect;
                        self.propagate_subsurface_layout(old_id, dest, min_z);
                    }
                }
            }

            if let Some(new_s) = self.surfaces.get_mut(&id) {
                max_z = max_z.max(new_s.current.z_order).saturating_add(1);
                new_s.current.z_order = max_z;
                let dest = new_s.current.dest_rect;
                self.propagate_subsurface_layout(id, dest, max_z);
            }
        }

        (old_focus, new_focus)
    }
}

fn airy_window_rect(
    screen_w: u32,
    screen_h: u32,
    existing_windows: &[Rect],
    width: u32,
    height: u32,
) -> Rect {
    let padding_x = 40;
    let padding_y = 60; // Leave space for a top bar / menu

    // If no windows, start at top-left
    if existing_windows.is_empty() {
        return Rect { x: padding_x, y: padding_y, w: width, h: height };
    }

    // Candidates for "urinal etiquette" placement:
    // Prefer corners and edges, maximizing distance from neighbors.
    let mut candidates = Vec::new();

    // 1. The four corners
    candidates.push((padding_x, padding_y));
    candidates.push((screen_w.saturating_sub(width).saturating_sub(padding_x), padding_y));
    candidates.push((padding_x, screen_h.saturating_sub(height).saturating_sub(padding_x)));
    candidates.push((
        screen_w.saturating_sub(width).saturating_sub(padding_x),
        screen_h.saturating_sub(height).saturating_sub(padding_x),
    ));

    // 2. The center
    candidates.push((
        screen_w.saturating_sub(width) / 2,
        (screen_h.saturating_sub(height).saturating_add(padding_y)) / 2,
    ));

    // 3. Edge midpoints
    candidates.push((screen_w.saturating_sub(width) / 2, padding_y));
    candidates.push((
        screen_w.saturating_sub(width) / 2,
        screen_h.saturating_sub(height).saturating_sub(padding_x),
    ));
    candidates.push((padding_x, screen_h.saturating_sub(height) / 2));
    candidates.push((
        screen_w.saturating_sub(width).saturating_sub(padding_x),
        screen_h.saturating_sub(height) / 2,
    ));

    let mut best_pos = candidates[0];
    let mut max_min_dist_sq = -1i64;

    for (cx, cy) in candidates {
        let cand = Rect { x: cx, y: cy, w: width, h: height };

        // Check for overlap with existing windows
        let mut overlaps = false;
        let mut min_dist_sq = i64::MAX;

        for w in existing_windows {
            if rect_overlaps(cand, *w) {
                overlaps = true;
                break;
            }

            let d_sq = rect_center_dist_sq(cand, *w);
            if d_sq < min_dist_sq {
                min_dist_sq = d_sq;
            }
        }

        if overlaps {
            continue;
        }

        if min_dist_sq > max_min_dist_sq {
            max_min_dist_sq = min_dist_sq;
            best_pos = (cx, cy);
        }
    }

    if max_min_dist_sq >= 0 {
        Rect { x: best_pos.0, y: best_pos.1, w: width, h: height }
    } else {
        // Fallback to a simple offset cascade if all ideal spots overlap
        let slot = (existing_windows.len() as u32) % CASCADE_SLOTS;
        Rect {
            x: CASCADE_START_X.saturating_add(slot.saturating_mul(CASCADE_STEP_X)),
            y: CASCADE_START_Y.saturating_add(slot.saturating_mul(CASCADE_STEP_Y)),
            w: width,
            h: height,
        }
    }
}

fn rect_overlaps(a: Rect, b: Rect) -> bool {
    a.x < b.x.saturating_add(b.w)
        && a.x.saturating_add(a.w) > b.x
        && a.y < b.y.saturating_add(b.h)
        && a.y.saturating_add(a.h) > b.y
}

fn rect_center_dist_sq(a: Rect, b: Rect) -> i64 {
    let ax = a.x as i64 + (a.w as i64 / 2);
    let ay = a.y as i64 + (a.h as i64 / 2);
    let bx = b.x as i64 + (b.w as i64 / 2);
    let by = b.y as i64 + (b.h as i64 / 2);
    let dx = ax - bx;
    let dy = ay - by;
    dx * dx + dy * dy
}

fn cascaded_window_rect(existing_standalone_windows: u32, width: u32, height: u32) -> Rect {
    let slot = existing_standalone_windows % CASCADE_SLOTS;
    Rect {
        x: CASCADE_START_X.saturating_add(slot.saturating_mul(CASCADE_STEP_X)),
        y: CASCADE_START_Y.saturating_add(slot.saturating_mul(CASCADE_STEP_Y)),
        w: width,
        h: height,
    }
}

pub struct CommitResult {
    pub changed: bool,
    pub released_buffer_ids: Vec<u32>,
    pub frame_serial: u64,
    pub damage_rects: Vec<Rect>,
    pub needs_full_repaint: bool,
}

#[derive(Clone, Debug)]
pub struct SurfaceSnapshot {
    pub id: u32,
    pub client_id: u32,
    pub rect: Rect,
    pub z_order: i32,
    pub mapped: bool,
    pub visible: bool,
    pub focus_eligible: bool,
    pub buffer_id: Option<u32>,
    pub title: Option<String>,
    pub frame_serial: u64,
    pub is_window: bool,
}

pub struct SurfaceMove {
    pub old_rect: Rect,
    pub new_rect: Rect,
    pub changed: bool,
}

pub struct SurfaceResize {
    pub old_rect: Rect,
    pub new_rect: Rect,
    pub changed: bool,
}

pub struct SurfaceToggle {
    pub old_rect: Rect,
    pub new_rect: Rect,
    pub changed: bool,
    pub active: bool,
}

pub fn surface_visual_rect(rect: Rect, chrome: SurfaceChrome) -> Rect {
    if chrome.is_empty() {
        return rect;
    }
    Rect {
        x: rect.x,
        y: rect.y,
        w: rect.w.saturating_add(2),
        h: rect.h.saturating_add(2),
    }
}

fn resize_edge_at(rect: Rect, thickness: u32, x: i32, y: i32) -> Option<ResizeEdge> {
    if thickness == 0 || rect.w == 0 || rect.h == 0 {
        return None;
    }
    let x0 = rect.x as i32;
    let y0 = rect.y as i32;
    let x1 = rect.x.saturating_add(rect.w) as i32;
    let y1 = rect.y.saturating_add(rect.h) as i32;
    if x < x0 || x >= x1 || y < y0 || y >= y1 {
        return None;
    }

    let t = thickness.min(rect.w / 2).min(rect.h / 2) as i32;
    if t <= 0 {
        return None;
    }
    let north = y < y0.saturating_add(t);
    let south = y >= y1.saturating_sub(t);
    let west = x < x0.saturating_add(t);
    let east = x >= x1.saturating_sub(t);

    match (north, south, west, east) {
        (true, _, true, _) => Some(ResizeEdge::NorthWest),
        (true, _, _, true) => Some(ResizeEdge::NorthEast),
        (_, true, true, _) => Some(ResizeEdge::SouthWest),
        (_, true, _, true) => Some(ResizeEdge::SouthEast),
        (true, _, _, _) => Some(ResizeEdge::North),
        (_, true, _, _) => Some(ResizeEdge::South),
        (_, _, true, _) => Some(ResizeEdge::West),
        (_, _, _, true) => Some(ResizeEdge::East),
        _ => None,
    }
}

pub fn chrome_button_rects(rect: Rect, chrome: SurfaceChrome) -> Option<[(ChromeButton, Rect); 3]> {
    if chrome.titlebar_height == 0 || rect.w == 0 || rect.h == 0 {
        return None;
    }
    // If the window is shaded, we still want the buttons.
    // If the window is fullscreen, we don't draw chrome.

    let frame = chrome.frame_thickness.min(rect.w / 2).min(rect.h / 2);
    let titlebar_height = chrome.titlebar_height.min(rect.h);
    let button_height = 24u32.min(titlebar_height);
    let button_width = 28u32;
    let spacing = 4u32;
    let right_inset = frame.saturating_add(6);
    if button_height < 8 || button_width < 12 {
        return None;
    }

    let right = rect.x.saturating_add(rect.w).saturating_sub(right_inset);
    let y = rect.y.saturating_add((titlebar_height.saturating_sub(button_height)) / 2);
    let total_w = button_width.saturating_mul(3).saturating_add(spacing.saturating_mul(2));
    if total_w.saturating_add(right_inset) > rect.w {
        return None;
    }

    let close_x = right.saturating_sub(button_width);
    let max_x = close_x.saturating_sub(spacing).saturating_sub(button_width);
    let min_x = max_x.saturating_sub(spacing).saturating_sub(button_width);

    Some([
        (ChromeButton::Shade, Rect { x: min_x, y, w: button_width, h: button_height }),
        (ChromeButton::Fullscreen, Rect { x: max_x, y, w: button_width, h: button_height }),
        (ChromeButton::Close, Rect { x: close_x, y, w: button_width, h: button_height }),
    ])
}

fn chrome_button_at(rect: Rect, chrome: SurfaceChrome, x: i32, y: i32) -> Option<ChromeButton> {
    let rects = chrome_button_rects(rect, chrome)?;
    for (button, bounds) in rects {
        let x0 = bounds.x as i32;
        let y0 = bounds.y as i32;
        let x1 = bounds.x.saturating_add(bounds.w) as i32;
        let y1 = bounds.y.saturating_add(bounds.h) as i32;
        if x >= x0 && x < x1 && y >= y0 && y < y1 {
            return Some(button);
        }
    }
    None
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
