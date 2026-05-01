//! Per-client Wayland object registry and pending surface state.

use alloc::collections::{BTreeMap, VecDeque};
use alloc::string::String;
use alloc::vec::Vec;

use blossom::Blossom;

#[derive(Debug, Clone, Copy)]
pub struct DmabufPlane {
    pub fd: u32,
    pub plane_idx: u32,
    pub offset: u32,
    pub stride: u32,
    pub modifier: u64,
}

// ── Object entries ────────────────────────────────────────────────────────────

/// The type of a live Wayland object in a client's object table.
#[derive(Debug, Clone)]
pub enum ObjectEntry {
    /// Object 1 — the fixed wl_display.
    Display,
    /// wl_registry bound by the client.
    Registry,
    /// wl_compositor global.
    Compositor,
    /// wl_subcompositor global.
    Subcompositor,
    /// wl_shm global.
    Shm,
    /// zwp_linux_dmabuf_v1 global.
    Dmabuf,
    /// zwp_linux_buffer_params_v1.
    DmabufParams { used: bool, planes: Vec<DmabufPlane> },
    /// wl_shm_pool.
    ShmPool { handle: u32, size: u32 },
    /// wl_buffer — created from wl_shm or zwp_linux_dmabuf_v1.
    Buffer {
        handle: u32,
        offset: u32,
        width: u32,
        height: u32,
        stride: u32,
        format: u32,
        modifier: u64,
    },
    /// wl_surface.
    Surface {
        /// Bloom scene surface ID (assigned by the main thread).
        bloom_surface_id: u32,
        /// xdg_surface object ID, if assigned.
        xdg_surface_obj: Option<u32>,
        /// zwlr_layer_surface_v1 object ID, if assigned.  A `wl_surface` may
        /// carry at most one role; the dispatcher rejects requests that
        /// would assign a second role.
        layer_surface_obj: Option<u32>,
        /// Pending buffer object ID (set by wl_surface.attach).
        pending_buffer: Option<u32>,
        /// Pending damage rect (x, y as i32; w, h as u32 matching Wayland wire).
        pending_damage: Option<(i32, i32, u32, u32)>,
        /// Pending frame callback object ID.
        pending_frame_cb: Option<u32>,
        /// Pending wp_presentation_feedback object IDs.
        ///
        /// Per the `wp_presentation` protocol, each `wp_presentation.feedback`
        /// request creates a feedback object that is associated with the
        /// surface's *next* content update (i.e. the next `wl_surface.commit`).
        /// Multiple feedback objects may be requested between commits and they
        /// must all be settled (`presented` or `discarded`) for the same
        /// committed frame.
        pending_presentation_feedback: Vec<u32>,
        /// wl_subsurface object ID, if this surface has been assigned the
        /// subsurface role.
        subsurface_obj: Option<u32>,
        /// Ordered list of child wl_surface object IDs that have been made
        /// subsurfaces of this surface.  Order is bottom-to-top stacking
        /// (manipulated by `wl_subsurface.place_above` / `place_below`).
        subsurface_children: Vec<u32>,
        /// Pending opaque region object ID (set by wl_surface.set_opaque_region).
        /// `None` means no region is pending (i.e. the entire surface is not
        /// marked opaque or the opaque region is being cleared with a null id).
        pending_opaque_region: Option<u32>,
        /// Pending input region object ID (set by wl_surface.set_input_region).
        /// `None` means no region is pending (entire surface accepts input,
        /// matching the Wayland default).
        pending_input_region: Option<u32>,
    },
    /// wl_callback — frame done callback.
    Callback,
    /// xdg_wm_base global.
    XdgWmBase,
    /// xdg_positioner (no-op in v1).
    XdgPositioner,
    /// xdg_surface.
    XdgSurface { bloom_surface_id: u32 },
    /// xdg_toplevel.
    XdgToplevel { xdg_surface_obj: u32 },
    /// xdg_popup.
    XdgPopup { xdg_surface_obj: u32 },
    /// zwlr_layer_shell_v1 global.
    LayerShell,
    /// zwlr_layer_surface_v1 — per-surface layer-shell role state.
    ///
    /// State is owned by the dispatcher; this variant just identifies the
    /// object and links it back to its underlying `wl_surface`.
    LayerSurface {
        /// Object ID of the wl_surface that has the layer-surface role.
        wl_surface_obj: u32,
        /// Bloom scene surface ID (mirrors the wl_surface's value for fast
        /// lookup without a second hash step).
        bloom_surface_id: u32,
        /// Accumulated layer-surface state (size, anchors, margins, layer …).
        state: blossom::LayerSurfaceState,
    },
    /// wl_subsurface — relates a wl_surface to its parent wl_surface.
    ///
    /// Per the Wayland spec, `sync` defaults to `true` (synchronized mode):
    /// `pending_position` and `pending_place` are buffered and atomically
    /// applied on the parent surface's `wl_surface.commit`.  In desync mode
    /// (`set_desync`) those changes apply immediately.
    Subsurface {
        /// Child wl_surface object ID.
        child_wl_surface: u32,
        /// Parent wl_surface object ID.
        parent_wl_surface: u32,
        /// Current offset relative to the parent in surface-local pixels.
        x: i32,
        y: i32,
        /// Synchronized mode: pending state is applied at parent commit time.
        /// Wayland spec default is `true`.
        sync: bool,
        /// Pending offset (set via `wl_subsurface.set_position`).
        pending_position: Option<(i32, i32)>,
        /// Pending stacking change (`(sibling_wl_surface_or_zero, place_above)`).
        /// `sibling==0` means relative to the parent itself.
        pending_place: Option<(u32, bool)>,
    },
    /// wl_seat global.
    Seat,
    /// wl_pointer created from wl_seat.
    Pointer,
    /// wl_keyboard created from wl_seat.
    Keyboard,
    /// wl_output global — represents a physical or virtual display.
    Output,
    /// wl_data_device_manager global.
    DataDeviceManager,
    /// wl_data_source — client-created clipboard/DnD data source.
    DataSource { mime_types: Vec<String> },
    /// wl_data_device — per-seat data device for clipboard and DnD.
    DataDevice { seat_obj: u32 },
    /// wl_data_offer — server-created offer advertising data types to a receiver.
    DataOffer,
    /// wp_presentation global — Wayland Presentation Time protocol manager.
    Presentation,
    /// wp_presentation_feedback — one-shot feedback object created by
    /// `wp_presentation.feedback(surface)`.  After it has been settled with
    /// either `presented` or `discarded` (both destructor events), the object
    /// is destroyed by the server.
    PresentationFeedback,
    /// wl_region — an accumulation of rectangles used by wl_surface for
    /// opaque and input region configuration.
    ///
    /// The region is a simple rect list.  Rectangles added via `wl_region.add`
    /// are appended; rectangles removed via `wl_region.subtract` are subtracted
    /// by appending a negative-weight entry that is stored as `(x, y, w, h,
    /// false)`.  The second bool field is `true` for add, `false` for subtract.
    Region {
        /// Accumulated operations: `(x, y, w, h, is_add)`.
        rects: Vec<(i32, i32, i32, i32, bool)>,
    },
    /// Object has been destroyed (tombstone).
    Destroyed,
}

// ── Per-client state ──────────────────────────────────────────────────────────

/// State for a single connected Wayland client.
pub struct WaylandClient {
    /// The connected socket FD.
    pub fd: u32,
    /// xdg-shell state machine for this client.
    ///
    /// Wayland object IDs are scoped to a single client.  Keeping Blossom
    /// state per client prevents clients that reuse object IDs such as
    /// wl_surface=10 / xdg_surface=11 from colliding with each other.
    pub blossom: Blossom,
    /// Map: Wayland object ID → entry.
    pub objects: BTreeMap<u32, ObjectEntry>,
    /// Accumulation buffer for partial socket reads.
    pub recv_buf: Vec<u8>,
    /// File descriptors received through socket ancillary data, consumed by
    /// Wayland requests with `fd` arguments in wire order.
    pub pending_fds: VecDeque<u32>,
    /// Monotonic buffer key counter (for IPC wl_buf_key).
    pub next_buf_key: u32,
    /// Map: wl_buf_key → wl_buffer object ID (for release events).
    pub buf_key_to_obj: BTreeMap<u32, u32>,
    /// Map: bloom_surface_id → frame callback object ID
    /// (populated on wl_surface.frame, cleared after sending done).
    pub frame_cbs: BTreeMap<u32, Vec<u32>>,
    /// Map: bloom_surface_id → wp_presentation_feedback object IDs awaiting
    /// either `presented` or `discarded`.  Populated on `wl_surface.commit`
    /// from the surface's `pending_presentation_feedback` list, cleared after
    /// either event is sent.
    pub presentation_feedbacks: BTreeMap<u32, Vec<u32>>,
    /// Monotonically-increasing presentation sequence number sent in the
    /// `presented` event of `wp_presentation_feedback`.  Best-effort software
    /// counter — not tied to real vblank/sequence reporting yet.
    pub presentation_seq: u64,
    /// Last modifier mask sent to this client's wl_keyboard.
    pub keyboard_modifiers: u8,
    /// The wl_data_device object ID bound by this client, if any.
    pub data_device_obj: Option<u32>,
    /// Set by `wl_data_device.set_selection` during message processing.
    /// Contains `(source_obj, mime_types)` to be broadcast to other clients.
    /// Cleared by the server after broadcasting.
    pub pending_clipboard_set: Option<(u32, Vec<String>)>,
    /// Set by `wl_data_offer.receive` during message processing.
    /// Contains `(offer_obj, mime_type, write_fd)` to be forwarded to the
    /// clipboard or DnD source owner.  The `offer_obj` lets the server decide
    /// whether to route to the clipboard source or the drag-and-drop source.
    /// Cleared by the server after forwarding.
    pub pending_offer_receive: Option<(u32, String, u32)>,
    /// Set by `wl_data_device.start_drag` during message processing.
    /// Contains `(source_obj, origin_wl_surface, icon_wl_surface, serial)`.
    /// Cleared by the server after initiating the drag.
    pub pending_start_drag: Option<(u32, u32, u32, u32)>,
    /// Set by `wl_data_offer.finish` to signal that the drop was accepted.
    /// Cleared by the server after forwarding `dnd_finished` to the source.
    pub pending_dnd_finish: bool,
    /// Set by `wl_data_offer.accept` to signal the accepted MIME type.
    /// `Some(mime)` where `mime` is empty means null/reject.
    /// Cleared by the server after forwarding `target` to the source.
    pub pending_dnd_accept: Option<String>,
}

impl WaylandClient {
    pub fn new(fd: u32, bloom_surface_id_seed: u32) -> Self {
        // Wayland object 1 is always wl_display.
        let mut objects = BTreeMap::new();
        objects.insert(1, ObjectEntry::Display);
        Self {
            fd,
            blossom: Blossom::new(),
            objects,
            recv_buf: Vec::new(),
            pending_fds: VecDeque::new(),
            next_buf_key: bloom_surface_id_seed * 1000,
            buf_key_to_obj: BTreeMap::new(),
            frame_cbs: BTreeMap::new(),
            presentation_feedbacks: BTreeMap::new(),
            presentation_seq: 0,
            keyboard_modifiers: 0,
            data_device_obj: None,
            pending_clipboard_set: None,
            pending_offer_receive: None,
            pending_start_drag: None,
            pending_dnd_finish: false,
            pending_dnd_accept: None,
        }
    }

    /// Allocate a fresh wl_buf_key for the next buffer import.
    pub fn alloc_buf_key(&mut self) -> u32 {
        let k = self.next_buf_key;
        self.next_buf_key += 1;
        k
    }

    /// Look up an object, returning a reference.
    pub fn get(&self, id: u32) -> Option<&ObjectEntry> {
        self.objects.get(&id)
    }

    /// Look up an object, returning a mutable reference.
    pub fn get_mut(&mut self, id: u32) -> Option<&mut ObjectEntry> {
        self.objects.get_mut(&id)
    }

    /// Insert a new object.
    pub fn insert(&mut self, id: u32, entry: ObjectEntry) {
        self.objects.insert(id, entry);
    }

    /// Mark an object as destroyed.
    pub fn destroy(&mut self, id: u32) {
        if let Some(e) = self.objects.get_mut(&id) {
            *e = ObjectEntry::Destroyed;
        }
    }

    /// Encode and send a Wayland event to this client.
    ///
    /// Silently ignores write errors (the dispatcher will detect the dead
    /// socket on the next read).
    pub fn send(&self, object_id: u32, opcode: u16, payload: &[u8]) {
        let msg = crate::wayland::wire::encode(object_id, opcode, payload);
        let _ = stem::syscall::vfs::vfs_write(self.fd, &msg);
    }

    /// Encode and send a Wayland event with ancillary file descriptors.
    pub fn send_with_fds(&self, object_id: u32, opcode: u16, payload: &[u8], fds: &[u32]) {
        let msg = crate::wayland::wire::encode(object_id, opcode, payload);
        let _ = stem::syscall::socket::sendmsg(self.fd, &msg, fds);
    }

    /// Send a `wl_display.error(object_id, code, message)` protocol error.
    pub fn send_protocol_error(&self, object_id: u32, code: u32, msg: &str) {
        use crate::wayland::wire::encode_string;
        let mut payload = alloc::vec::Vec::new();
        payload.extend_from_slice(&object_id.to_ne_bytes());
        payload.extend_from_slice(&code.to_ne_bytes());
        payload.extend_from_slice(&encode_string(msg));
        // wl_display opcode 0 = error
        self.send(1, 0, &payload);
    }

    /// Find the wl_surface object ID whose xdg_surface_obj matches `xdg_id`.
    pub fn surface_for_xdg(&self, xdg_id: u32) -> Option<u32> {
        for (obj_id, entry) in &self.objects {
            if let ObjectEntry::Surface { xdg_surface_obj: Some(x), .. } = entry {
                if *x == xdg_id {
                    return Some(*obj_id);
                }
            }
        }
        None
    }

    /// Find the bloom_surface_id for a given wl_surface object ID.
    pub fn bloom_surface_id(&self, wl_surface_obj: u32) -> Option<u32> {
        if let Some(ObjectEntry::Surface { bloom_surface_id, .. }) =
            self.objects.get(&wl_surface_obj)
        {
            Some(*bloom_surface_id)
        } else {
            None
        }
    }

    /// Find the wl_surface object ID for a Bloom surface.
    pub fn wl_surface_for_bloom_surface(&self, bloom_id: u32) -> Option<u32> {
        for (obj_id, entry) in &self.objects {
            if let ObjectEntry::Surface { bloom_surface_id, .. } = entry {
                if *bloom_surface_id == bloom_id {
                    return Some(*obj_id);
                }
            }
        }
        None
    }

    /// Return the `bloom_surface_id` values for all live `xdg_popup` objects
    /// owned by this client.
    ///
    /// Used by the compositor to check whether a pointer click lands on a
    /// popup surface so it can emit `xdg_popup.popup_done` when appropriate.
    pub fn popup_bloom_surface_ids(&self) -> Vec<u32> {
        let mut result = Vec::new();
        for entry in self.objects.values() {
            if let ObjectEntry::XdgPopup { xdg_surface_obj } = entry {
                if let Some(ObjectEntry::XdgSurface { bloom_surface_id }) =
                    self.objects.get(xdg_surface_obj)
                {
                    result.push(*bloom_surface_id);
                }
            }
        }
        result
    }

    pub fn pointer_object(&self) -> Option<u32> {
        self.objects.iter().find_map(|(id, entry)| match entry {
            ObjectEntry::Pointer => Some(*id),
            _ => None,
        })
    }

    pub fn keyboard_object(&self) -> Option<u32> {
        self.objects.iter().find_map(|(id, entry)| match entry {
            ObjectEntry::Keyboard => Some(*id),
            _ => None,
        })
    }

    /// Return a `wl_output` object ID that this client has bound, if any.
    /// Used by `wp_presentation_feedback.sync_output`.
    pub fn output_object(&self) -> Option<u32> {
        self.objects.iter().find_map(|(id, entry)| match entry {
            ObjectEntry::Output => Some(*id),
            _ => None,
        })
    }

    /// Return the xdg_surface object ID associated with a wl_surface.
    pub fn xdg_surface_for_surface(&self, wl_surface_obj: u32) -> Option<u32> {
        if let Some(ObjectEntry::Surface { xdg_surface_obj: Some(x), .. }) =
            self.objects.get(&wl_surface_obj)
        {
            Some(*x)
        } else {
            None
        }
    }

    /// Find the xdg objects for the toplevel associated with a Bloom surface.
    pub fn xdg_toplevel_for_bloom_surface(&self, bloom_id: u32) -> Option<(u32, u32)> {
        let mut xdg_surface = None;
        for (obj_id, entry) in &self.objects {
            if let ObjectEntry::XdgSurface { bloom_surface_id } = entry {
                if *bloom_surface_id == bloom_id {
                    xdg_surface = Some(*obj_id);
                    break;
                }
            }
        }
        let xdg_surface = xdg_surface?;
        for (obj_id, entry) in &self.objects {
            if let ObjectEntry::XdgToplevel { xdg_surface_obj } = entry {
                if *xdg_surface_obj == xdg_surface {
                    return Some((xdg_surface, *obj_id));
                }
            }
        }
        None
    }

    /// Find the `zwlr_layer_surface_v1` object ID for a Bloom surface, if any.
    pub fn layer_surface_for_bloom_surface(&self, bloom_id: u32) -> Option<u32> {
        for (obj_id, entry) in &self.objects {
            if let ObjectEntry::LayerSurface { bloom_surface_id, .. } = entry {
                if *bloom_surface_id == bloom_id {
                    return Some(*obj_id);
                }
            }
        }
        None
    }

    /// Register a frame callback for a bloom_surface_id.
    pub fn add_frame_cb(&mut self, bloom_surface_id: u32, cb_obj: u32) {
        self.frame_cbs.entry(bloom_surface_id).or_default().push(cb_obj);
    }

    /// Fire (and clear) all frame callbacks for a bloom_surface_id.
    /// Returns the callback object IDs that were fired.
    pub fn fire_frame_cbs(&mut self, bloom_surface_id: u32, timestamp_ms: u32) -> Vec<u32> {
        let cbs = self.frame_cbs.remove(&bloom_surface_id).unwrap_or_default();
        for &cb_id in &cbs {
            // wl_callback opcode 0 = done(callback_data: uint)
            let payload = timestamp_ms.to_ne_bytes();
            self.send(cb_id, 0, &payload);
            // wl_callback is a one-shot object; mark destroyed.
            self.destroy(cb_id);
        }
        cbs
    }

    /// Register a `wp_presentation_feedback` object as pending for the next
    /// commit of `wl_surface_obj`.  The object is moved from per-surface
    /// "pending" state to the per-`bloom_surface_id` in-flight map at commit
    /// time (see [`Self::flush_pending_presentation_feedback`]).
    pub fn add_pending_presentation_feedback(&mut self, wl_surface_obj: u32, fb_obj: u32) {
        if let Some(ObjectEntry::Surface { pending_presentation_feedback, .. }) =
            self.objects.get_mut(&wl_surface_obj)
        {
            pending_presentation_feedback.push(fb_obj);
        }
    }

    /// At commit time, move any pending `wp_presentation_feedback` objects
    /// for `wl_surface_obj` into the in-flight map keyed by `bloom_surface_id`.
    pub fn flush_pending_presentation_feedback(
        &mut self,
        wl_surface_obj: u32,
        bloom_surface_id: u32,
    ) -> usize {
        let drained: Vec<u32> = if let Some(ObjectEntry::Surface {
            pending_presentation_feedback,
            ..
        }) = self.objects.get_mut(&wl_surface_obj)
        {
            core::mem::take(pending_presentation_feedback)
        } else {
            Vec::new()
        };
        let n = drained.len();
        if !drained.is_empty() {
            self.presentation_feedbacks
                .entry(bloom_surface_id)
                .or_default()
                .extend(drained);
        }
        n
    }

    /// Allocate the next presentation sequence number and return both halves.
    pub fn next_presentation_seq(&mut self) -> (u32, u32) {
        let seq = self.presentation_seq;
        self.presentation_seq = self.presentation_seq.wrapping_add(1);
        ((seq >> 32) as u32, (seq & 0xffff_ffff_u64) as u32)
    }

    /// Fire `wp_presentation_feedback.presented` for all in-flight feedbacks
    /// associated with `bloom_surface_id`.  Each feedback object is then
    /// destroyed (per protocol, `presented` is a destructor).
    ///
    /// `timestamp_ns` is the monotonic timestamp at which the frame became
    /// visible (best-effort software estimate until real vblank timing is
    /// available).  `refresh_ns` is the nominal output refresh interval in
    /// nanoseconds, or 0 if unknown.  `output_obj` is an optional client-side
    /// `wl_output` to advertise via `sync_output`.
    ///
    /// Returns the list of feedback object IDs that were fired.
    pub fn fire_presentation_feedbacks_presented(
        &mut self,
        bloom_surface_id: u32,
        timestamp_ns: u64,
        refresh_ns: u32,
        output_obj: Option<u32>,
    ) -> Vec<u32> {
        let fbs = self.presentation_feedbacks.remove(&bloom_surface_id).unwrap_or_default();
        if fbs.is_empty() {
            return fbs;
        }
        let tv_sec = timestamp_ns / 1_000_000_000;
        let tv_nsec = (timestamp_ns % 1_000_000_000) as u32;
        let tv_sec_hi = (tv_sec >> 32) as u32;
        let tv_sec_lo = (tv_sec & 0xffff_ffff_u64) as u32;
        // Software best-effort: no VSYNC / HW_CLOCK / HW_COMPLETION / ZERO_COPY
        // bits are claimed yet.  This keeps clients honest about the limited
        // timing precision until real vblank reporting is wired up.
        let flags: u32 = 0;
        for &fb in &fbs {
            let (seq_hi, seq_lo) = self.next_presentation_seq();
            // wp_presentation_feedback.sync_output(output) — opcode 0.
            // Only advertise if the client has bound the relevant wl_output.
            if let Some(out_obj) = output_obj {
                self.send(fb, 0, &out_obj.to_ne_bytes());
            }
            // wp_presentation_feedback.presented — opcode 1.
            // (tv_sec_hi:u32, tv_sec_lo:u32, tv_nsec:u32, refresh:u32,
            //  seq_hi:u32, seq_lo:u32, flags:u32)
            let mut payload = [0u8; 28];
            payload[0..4].copy_from_slice(&tv_sec_hi.to_ne_bytes());
            payload[4..8].copy_from_slice(&tv_sec_lo.to_ne_bytes());
            payload[8..12].copy_from_slice(&tv_nsec.to_ne_bytes());
            payload[12..16].copy_from_slice(&refresh_ns.to_ne_bytes());
            payload[16..20].copy_from_slice(&seq_hi.to_ne_bytes());
            payload[20..24].copy_from_slice(&seq_lo.to_ne_bytes());
            payload[24..28].copy_from_slice(&flags.to_ne_bytes());
            self.send(fb, 1, &payload);
            // `presented` is a destructor event — the object is gone.
            self.destroy(fb);
        }
        fbs
    }

    /// Fire `wp_presentation_feedback.discarded` for every in-flight feedback
    /// object on `bloom_surface_id` (used when the surface is destroyed or its
    /// content is otherwise superseded without being presented).
    pub fn fire_presentation_feedbacks_discarded(&mut self, bloom_surface_id: u32) -> Vec<u32> {
        let fbs = self.presentation_feedbacks.remove(&bloom_surface_id).unwrap_or_default();
        for &fb in &fbs {
            // wp_presentation_feedback.discarded — opcode 2 (no payload, destructor).
            self.send(fb, 2, &[]);
            self.destroy(fb);
        }
        fbs
    }

    /// Discard any `wp_presentation_feedback` objects that were registered
    /// against `wl_surface_obj` but were never committed (e.g. when a surface
    /// is destroyed without a final commit).
    pub fn discard_pending_presentation_feedback(&mut self, wl_surface_obj: u32) -> Vec<u32> {
        let drained: Vec<u32> = if let Some(ObjectEntry::Surface {
            pending_presentation_feedback,
            ..
        }) = self.objects.get_mut(&wl_surface_obj)
        {
            core::mem::take(pending_presentation_feedback)
        } else {
            Vec::new()
        };
        for &fb in &drained {
            self.send(fb, 2, &[]);
            self.destroy(fb);
        }
        drained
    }

    /// Return the pending buffer entry if there is one attached to `wl_surface_obj`.
    pub fn pending_buffer_entry(&self, wl_surface_obj: u32) -> Option<(u32, &ObjectEntry)> {
        let pb = match self.objects.get(&wl_surface_obj)? {
            ObjectEntry::Surface { pending_buffer: Some(pb), .. } => *pb,
            _ => return None,
        };
        Some((pb, self.objects.get(&pb)?))
    }

    /// Build a human-readable name for this client (for logs).
    pub fn describe(&self, app_id: Option<&str>) -> String {
        if let Some(s) = app_id {
            alloc::format!("fd={} app={}", self.fd, s)
        } else {
            alloc::format!("fd={}", self.fd)
        }
    }
}
