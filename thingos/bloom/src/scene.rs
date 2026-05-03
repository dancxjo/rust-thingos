use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use abi::display_protocol::Rect;
use blossom::Rect as BlossomRect;
pub use blossom::wm::{ChromeButton, CursorKind, HitTarget, ResizeEdge};

pub const DRAMATIC_CLOSE_DURATION_NS: u64 = 360_000_000;

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
pub struct SurfaceMove {
    pub old_rect: Rect,
    pub new_rect: Rect,
    pub changed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceResize {
    pub old_rect: Rect,
    pub new_rect: Rect,
    pub changed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceToggle {
    pub old_rect: Rect,
    pub new_rect: Rect,
    pub changed: bool,
    pub active: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DramaticClose {
    pub started_ns: u64,
    pub duration_ns: u64,
}

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
    /// When `true`, the input region is explicitly cleared on the next commit
    /// (i.e. the client sent `wl_surface.set_input_region(null)`).
    /// A cleared input region means the entire surface receives input (default).
    /// Takes precedence over `input_region` if both happen to be set.
    pub clear_input_region: bool,
    pub opaque_region: Option<Rect>,
    /// When `true`, the opaque region is explicitly cleared on the next commit
    /// (i.e. the client sent `wl_surface.set_opaque_region(null)`).
    /// Takes precedence over `opaque_region` if both happen to be set.
    pub clear_opaque_region: bool,
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
    pub keyboard_interactivity: blossom::LayerKeyboardInteractivity,
    pub chrome: SurfaceChrome,
    pub title: Option<String>,
    pub frame_serial: u64,
    pub is_fullscreen: bool,
    pub is_shaded: bool,
    pub dramatic_close: Option<DramaticClose>,
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

impl CompositionEntry {
    /// Returns `true` when this entry represents a fully-opaque plane.
    ///
    /// A plane is fully opaque when **all** of the following hold:
    ///
    /// * `alpha == 255` — per-plane opacity is fully opaque.
    /// * No compositor-drawn chrome — chrome (title bars, frames, rounded corners)
    ///   makes the visual boundary partially transparent.
    /// * The surface has an explicitly committed opaque region that covers the
    ///   entire source buffer area (`x = 0, y = 0, w ≥ src_rect.w,
    ///   h ≥ src_rect.h`).
    pub fn is_opaque(&self) -> bool {
        if self.alpha < 255 {
            return false;
        }
        // Surfaces with compositor-drawn chrome (title bars, frames) have
        // rounded corners and shadows that leave partial transparency at the
        // visual boundary.
        if !self.chrome.is_empty() {
            return false;
        }
        let Some(opaque) = self.opaque_region else {
            return false;
        };
        opaque.x == 0 && opaque.y == 0 && opaque.w >= self.src_rect.w && opaque.h >= self.src_rect.h
    }
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
                keyboard_interactivity: blossom::LayerKeyboardInteractivity::None,
                chrome: SurfaceChrome::default(),
                title: None,
                frame_serial: 0,
                is_fullscreen: false,
                is_shaded: false,
                dramatic_close: None,
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
        // Reset the clear flag so that setting a new rect never leaves the two
        // fields in a contradictory state (clear=true, rect=Some).  This also
        // handles the case where a client calls set_input_region(null) and then
        // set_input_region(region) before the next commit — the latter wins.
        surface.pending.clear_input_region = false;
        surface.pending.input_region = Some(rect);
        true
    }

    /// Clear the pending input region.  On the next commit `current.input_region`
    /// will be set to `None`, restoring the default where the entire surface
    /// receives pointer input (Wayland `null` region semantics).
    pub fn clear_pending_input_region(&mut self, client_id: u32, surface_id: u32) -> bool {
        let Some(surface) = self.surfaces.get_mut(&surface_id) else {
            return false;
        };
        if surface.client_id != client_id {
            return false;
        }
        surface.pending.input_region = None;
        surface.pending.clear_input_region = true;
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
        // Reset the clear flag first so that setting a new rect never leaves
        // the two fields in a contradictory state (clear=true, rect=Some).
        surface.pending.clear_opaque_region = false;
        surface.pending.opaque_region = Some(rect);
        true
    }

    /// Clear the pending opaque region.  On the next commit `current.opaque_region`
    /// will be set to `None`, signalling to the compositor that the surface has
    /// no declared opaque area (the Wayland `null` region semantics).
    pub fn clear_pending_opaque_region(&mut self, client_id: u32, surface_id: u32) -> bool {
        let Some(surface) = self.surfaces.get_mut(&surface_id) else {
            return false;
        };
        if surface.client_id != client_id {
            return false;
        }
        surface.pending.opaque_region = None;
        surface.pending.clear_opaque_region = true;
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
                    bloom_airy_window_rect(
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
                damage_rects.push(bloom_surface_visual_rect(old_dest, surface.chrome));
            }
            surface.current.dest_rect = dest;
            damage_rects.push(bloom_surface_visual_rect(dest, surface.chrome));
            changed = true;
        }
        if let Some(z) = surface.pending.z_order.take() {
            surface.current.z_order = z;
            changed = true;
            visual_full_damage = true;
        }
        if surface.pending.clear_input_region {
            debug_assert!(
                surface.pending.input_region.is_none(),
                "clear_input_region and input_region should not both be set"
            );
            surface.current.input_region = None;
            surface.pending.clear_input_region = false;
            changed = true;
        } else if let Some(region) = surface.pending.input_region.take() {
            surface.current.input_region = Some(region);
            changed = true;
        }
        if surface.pending.clear_opaque_region {
            surface.current.opaque_region = None;
            surface.pending.clear_opaque_region = false;
            changed = true;
        } else if let Some(region) = surface.pending.opaque_region.take() {
            surface.current.opaque_region = Some(region);
            changed = true;
        }
        if !surface.pending.damage.is_empty() {
            if let Some(buf) = surface.current.buffer {
                let dest = surface.current.dest_rect;
                for rect in surface.pending.damage.drain(..) {
                    if let Some(rect) =
                        bloom_translate_surface_damage(rect, buf.width, buf.height, dest)
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
            damage_rects.push(bloom_surface_visual_rect(surface.current.dest_rect, surface.chrome));
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
        self.collect_composition_at(0)
    }

    pub fn collect_composition_at(&self, now_ns: u64) -> Vec<CompositionEntry> {
        let mut list = Vec::new();
        let active_surface = self.keyboard_focus.or_else(|| {
            self.surfaces
                .values()
                .filter(|surface| {
                    surface.visible
                        && surface.mapped
                        && surface.focus_eligible
                        && surface.dramatic_close.is_none()
                })
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
            let (dest_rect, alpha) = if let Some(close) = surface.dramatic_close {
                dramatic_close_geometry(surface.current.dest_rect, close, now_ns)
            } else {
                (surface.current.dest_rect, 255)
            };
            list.push(CompositionEntry {
                surface_id: surface.id,
                buffer_id: buf.buffer_id,
                src_rect: Rect { x: 0, y: 0, w: buf.width, h: buf.height },
                dest_rect,
                z_order: surface.current.z_order,
                alpha,
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
            if !surface.visible
                || !surface.mapped
                || !surface.focus_eligible
                || surface.dramatic_close.is_some()
            {
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
        if let Some(button) = bloom_chrome_button_at(rect, surface.chrome, x, y) {
            return Some(HitTarget::ChromeButton { surface_id, button });
        }
        if let Some(edge) = bloom_resize_edge_at(rect, surface.chrome.frame_thickness, x, y) {
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
        Some(bloom_surface_visual_rect(surface.current.dest_rect, surface.chrome))
    }

    pub fn visual_rect_for_surface_rect(&self, surface_id: u32, rect: Rect) -> Option<Rect> {
        let surface = self.surfaces.get(&surface_id)?;
        Some(bloom_surface_visual_rect(rect, surface.chrome))
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

    pub fn set_surface_keyboard_interactivity(
        &mut self,
        client_id: u32,
        surface_id: u32,
        mode: blossom::LayerKeyboardInteractivity,
    ) -> bool {
        let Some(surface) = self.surfaces.get_mut(&surface_id) else {
            return false;
        };
        if surface.client_id != client_id {
            return false;
        }
        surface.keyboard_interactivity = mode;
        match mode {
            blossom::LayerKeyboardInteractivity::Exclusive => {
                self.keyboard_focus = Some(surface_id);
            }
            blossom::LayerKeyboardInteractivity::None
                if self.keyboard_focus == Some(surface_id) =>
            {
                self.keyboard_focus = None;
            }
            blossom::LayerKeyboardInteractivity::None
            | blossom::LayerKeyboardInteractivity::OnDemand => {}
        }
        true
    }

    pub fn global_shortcut_surface(&self) -> Option<u32> {
        self.surfaces
            .values()
            .filter(|surface| {
                surface.visible
                    && surface.mapped
                    && surface.dramatic_close.is_none()
                    && !matches!(
                        surface.keyboard_interactivity,
                        blossom::LayerKeyboardInteractivity::None
                    )
            })
            .max_by_key(|surface| surface.current.z_order)
            .map(|surface| surface.id)
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
                is_window: surface.visible
                    && surface.mapped
                    && surface.current.buffer.is_some()
                    && surface.dramatic_close.is_none(),
            })
            .collect()
    }

    pub fn start_dramatic_close(&mut self, surface_id: u32, now_ns: u64) -> Option<Rect> {
        let surface = self.surfaces.get_mut(&surface_id)?;
        if !surface.visible || !surface.mapped || surface.current.buffer.is_none() {
            return None;
        }
        if surface.dramatic_close.is_none() {
            surface.dramatic_close =
                Some(DramaticClose { started_ns: now_ns, duration_ns: DRAMATIC_CLOSE_DURATION_NS });
        }
        surface.focus_eligible = false;
        if self.pointer_focus == Some(surface_id) {
            self.pointer_focus = None;
        }
        if self.keyboard_focus == Some(surface_id) {
            self.keyboard_focus = None;
        }
        Some(bloom_surface_visual_rect(surface.current.dest_rect, surface.chrome))
    }

    pub fn has_dramatic_closes(&self) -> bool {
        self.surfaces.values().any(|surface| surface.dramatic_close.is_some())
    }

    pub fn dramatic_close_visual_rects(&self, now_ns: u64) -> Vec<Rect> {
        let mut rects = Vec::new();
        for surface in self.surfaces.values() {
            let Some(close) = surface.dramatic_close else {
                continue;
            };
            let (rect, _) = dramatic_close_geometry(surface.current.dest_rect, close, now_ns);
            rects.push(bloom_surface_visual_rect(rect, surface.chrome));
        }
        rects
    }

    pub fn take_finished_dramatic_closes(&mut self, now_ns: u64) -> Vec<ClosedSurface> {
        let ids: Vec<u32> = self
            .surfaces
            .values()
            .filter(|surface| {
                surface.dramatic_close.map(|close| close_finished(close, now_ns)).unwrap_or(false)
            })
            .map(|surface| surface.id)
            .collect();

        let mut closed = Vec::new();
        for surface_id in ids {
            let Some(visual_rect) = self.surface_visual_rect(surface_id) else {
                continue;
            };
            let Some(client_id) = self.surfaces.get(&surface_id).map(|surface| surface.client_id)
            else {
                continue;
            };
            if let Some(released_buffer_ids) = self.destroy_surface(client_id, surface_id) {
                closed.push(ClosedSurface { surface_id, released_buffer_ids, visual_rect });
            }
        }
        closed
    }

    pub fn raise_to_top(&mut self, surface_id: u32) -> bool {
        let max_z = self.surfaces.values().map(|s| s.current.z_order).max().unwrap_or(0);
        let surfaces_len = self.surfaces.len();

        if let Some(surface) = self.surfaces.get_mut(&surface_id) {
            if surface.current.z_order < max_z || (surface.current.z_order == 0 && surfaces_len > 1)
            {
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

pub struct CommitResult {
    pub changed: bool,
    pub released_buffer_ids: Vec<u32>,
    pub frame_serial: u64,
    pub damage_rects: Vec<Rect>,
    pub needs_full_repaint: bool,
}

pub struct ClosedSurface {
    pub surface_id: u32,
    pub released_buffer_ids: Vec<u32>,
    pub visual_rect: Rect,
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

pub fn to_blossom_rect(r: Rect) -> BlossomRect {
    BlossomRect { x: r.x as i32, y: r.y as i32, w: r.w as i32, h: r.h as i32 }
}

pub fn from_blossom_rect(r: BlossomRect) -> Rect {
    Rect { x: r.x.max(0) as u32, y: r.y.max(0) as u32, w: r.w.max(0) as u32, h: r.h.max(0) as u32 }
}

fn to_blossom_chrome(chrome: SurfaceChrome) -> blossom::wm::SurfaceChrome {
    blossom::wm::SurfaceChrome {
        titlebar_height: chrome.titlebar_height.min(i32::MAX as u32) as i32,
        frame_thickness: chrome.frame_thickness.min(i32::MAX as u32) as i32,
    }
}

fn bloom_airy_window_rect(
    screen_w: u32,
    screen_h: u32,
    existing_rects: &[Rect],
    width: u32,
    height: u32,
) -> Rect {
    let mut blossom_rects = Vec::new();
    for rect in existing_rects {
        blossom_rects.push(to_blossom_rect(*rect));
    }
    from_blossom_rect(blossom::wm::airy_window_rect(
        screen_w.min(i32::MAX as u32) as i32,
        screen_h.min(i32::MAX as u32) as i32,
        &blossom_rects,
        width.min(i32::MAX as u32) as i32,
        height.min(i32::MAX as u32) as i32,
    ))
}

fn bloom_surface_visual_rect(rect: Rect, chrome: SurfaceChrome) -> Rect {
    from_blossom_rect(blossom::wm::surface_visual_rect(
        to_blossom_rect(rect),
        to_blossom_chrome(chrome),
    ))
}

fn dramatic_close_geometry(rect: Rect, close: DramaticClose, now_ns: u64) -> (Rect, u8) {
    let elapsed = now_ns.saturating_sub(close.started_ns);
    let duration = close.duration_ns.max(1);
    let progress = elapsed.saturating_mul(1024).saturating_div(duration).min(1024) as u32;

    let pop_px = if progress < 220 {
        progress.saturating_mul(12) / 220
    } else if progress < 420 {
        (420 - progress).saturating_mul(12) / 200
    } else {
        0
    };
    let collapse = progress.saturating_sub(260);
    let collapse_range = 1024u32.saturating_sub(260);
    let inset_x = rect.w.saturating_mul(collapse).saturating_div(collapse_range).saturating_div(5);
    let inset_y = rect.h.saturating_mul(collapse).saturating_div(collapse_range).saturating_div(5);
    let drop = progress.saturating_mul(progress).saturating_mul(36).saturating_div(1024 * 1024);

    let x = rect.x.saturating_sub(pop_px).saturating_add(inset_x);
    let y = rect.y.saturating_sub(pop_px).saturating_add(inset_y).saturating_add(drop);
    let w =
        rect.w.saturating_add(pop_px.saturating_mul(2)).saturating_sub(inset_x.saturating_mul(2));
    let h =
        rect.h.saturating_add(pop_px.saturating_mul(2)).saturating_sub(inset_y.saturating_mul(2));

    let fade_start = 220u32;
    let alpha = if progress <= fade_start {
        255
    } else {
        let fade = progress - fade_start;
        let fade_range = 1024 - fade_start;
        255u32.saturating_sub(fade.saturating_mul(255).saturating_div(fade_range)) as u8
    };

    (Rect { x, y, w: w.max(1), h: h.max(1) }, alpha)
}

fn close_finished(close: DramaticClose, now_ns: u64) -> bool {
    now_ns.saturating_sub(close.started_ns) >= close.duration_ns
}

fn bloom_translate_surface_damage(local: Rect, src_w: u32, src_h: u32, dest: Rect) -> Option<Rect> {
    blossom::wm::translate_surface_damage(
        to_blossom_rect(local),
        src_w.min(i32::MAX as u32) as i32,
        src_h.min(i32::MAX as u32) as i32,
        to_blossom_rect(dest),
    )
    .map(from_blossom_rect)
}

fn bloom_chrome_button_at(
    rect: Rect,
    chrome: SurfaceChrome,
    x: i32,
    y: i32,
) -> Option<ChromeButton> {
    blossom::wm::chrome_button_at(to_blossom_rect(rect), to_blossom_chrome(chrome), x, y)
        .map(from_blossom_chrome_button)
}

fn bloom_resize_edge_at(rect: Rect, frame_thickness: u32, x: i32, y: i32) -> Option<ResizeEdge> {
    blossom::wm::resize_edge_at(
        to_blossom_rect(rect),
        frame_thickness.min(i32::MAX as u32) as i32,
        x,
        y,
    )
    .map(from_blossom_resize_edge)
}

fn from_blossom_chrome_button(button: blossom::wm::ChromeButton) -> ChromeButton {
    match button {
        blossom::wm::ChromeButton::Minimize => ChromeButton::Minimize,
        blossom::wm::ChromeButton::Shade => ChromeButton::Shade,
        blossom::wm::ChromeButton::Maximize => ChromeButton::Maximize,
        blossom::wm::ChromeButton::Fullscreen => ChromeButton::Fullscreen,
        blossom::wm::ChromeButton::Close => ChromeButton::Close,
    }
}

fn from_blossom_resize_edge(edge: blossom::wm::ResizeEdge) -> ResizeEdge {
    match edge {
        blossom::wm::ResizeEdge::North => ResizeEdge::North,
        blossom::wm::ResizeEdge::South => ResizeEdge::South,
        blossom::wm::ResizeEdge::East => ResizeEdge::East,
        blossom::wm::ResizeEdge::West => ResizeEdge::West,
        blossom::wm::ResizeEdge::NorthEast => ResizeEdge::NorthEast,
        blossom::wm::ResizeEdge::NorthWest => ResizeEdge::NorthWest,
        blossom::wm::ResizeEdge::SouthEast => ResizeEdge::SouthEast,
        blossom::wm::ResizeEdge::SouthWest => ResizeEdge::SouthWest,
    }
}
