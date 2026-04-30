//! Per-client Wayland object registry and pending surface state.

use alloc::collections::{BTreeMap, VecDeque};
use alloc::string::String;
use alloc::vec::Vec;

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
    /// wl_shm global.
    Shm,
    /// wl_shm_pool.
    ShmPool { handle: u32, size: u32 },
    /// wl_buffer — created from a shm_pool.
    Buffer { handle: u32, offset: u32, width: u32, height: u32, stride: u32, format: u32 },
    /// wl_surface.
    Surface {
        /// Bloom scene surface ID (assigned by the main thread).
        bloom_surface_id: u32,
        /// xdg_surface object ID, if assigned.
        xdg_surface_obj: Option<u32>,
        /// Pending buffer object ID (set by wl_surface.attach).
        pending_buffer: Option<u32>,
        /// Pending damage rect (x, y as i32; w, h as u32 matching Wayland wire).
        pending_damage: Option<(i32, i32, u32, u32)>,
        /// Pending frame callback object ID.
        pending_frame_cb: Option<u32>,
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
    /// wl_seat global.
    Seat,
    /// wl_pointer created from wl_seat.
    Pointer,
    /// wl_keyboard created from wl_seat.
    Keyboard,
    /// wl_output global — represents a physical or virtual display.
    Output,
    /// Object has been destroyed (tombstone).
    Destroyed,
}

// ── Per-client state ──────────────────────────────────────────────────────────

/// State for a single connected Wayland client.
pub struct WaylandClient {
    /// The connected socket FD.
    pub fd: u32,
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
    /// Last modifier mask sent to this client's wl_keyboard.
    pub keyboard_modifiers: u8,
}

impl WaylandClient {
    pub fn new(fd: u32, bloom_surface_id_seed: u32) -> Self {
        // Wayland object 1 is always wl_display.
        let mut objects = BTreeMap::new();
        objects.insert(1, ObjectEntry::Display);
        Self {
            fd,
            objects,
            recv_buf: Vec::new(),
            pending_fds: VecDeque::new(),
            next_buf_key: bloom_surface_id_seed * 1000,
            buf_key_to_obj: BTreeMap::new(),
            frame_cbs: BTreeMap::new(),
            keyboard_modifiers: 0,
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
