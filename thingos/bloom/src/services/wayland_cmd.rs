//! `WaylandCommandService` — handles IPC commands from the Wayland server thread.
//!
//! The Wayland server thread sends fixed-size port messages whenever a Wayland
//! client modifies surface state.  This service reads those commands and applies
//! them to [`BloomWorld`] (scene + display) while running on the main
//! composition thread.
//!
//! It also maintains the buffer-key mapping needed to generate `wl_buffer.release`
//! events back to Wayland clients after surface commits.

use alloc::collections::BTreeMap;
use alloc::vec;
use alloc::vec::Vec;

use abi::pixel::PixelFormat;
use stem::syscall::port_send_all;
use stem::syscall::vfs::vfs_read;
use stem::{debug, warn};

use crate::loop_types::{BloomService, Interest, LoopAction, LoopEvent};
use crate::scene::SurfaceBuffer;
use crate::wayland::ipc;
use crate::world::BloomWorld;

/// Service that reads Wayland IPC commands from the compositor side.
pub struct WaylandCommandService {
    /// VFS fd for reading commands from the Wayland server thread.
    cmd_read_fd: u32,
    /// Port write handle for sending events back to the Wayland server thread.
    evt_write: u32,
    /// Scene client ID registered for all Wayland surfaces.
    wayland_client_id: u32,
    /// wl_buf_key → bloom buffer_id (reverse lookup for releases).
    buf_key_to_bloom: BTreeMap<u32, u32>,
    /// bloom buffer_id → wl_buf_key.
    bloom_to_buf_key: BTreeMap<u32, u32>,
    interests: Vec<Interest>,
}

impl WaylandCommandService {
    pub fn new(cmd_read_fd: u32, evt_write: u32, wayland_client_id: u32) -> Self {
        let interests = vec![Interest::FdReadable(cmd_read_fd)];
        Self {
            cmd_read_fd,
            evt_write,
            wayland_client_id,
            buf_key_to_bloom: BTreeMap::new(),
            bloom_to_buf_key: BTreeMap::new(),
            interests,
        }
    }

    fn handle_command(&mut self, data: &[u8], world: &mut BloomWorld) -> bool {
        if data.is_empty() {
            return false;
        }
        match data[0] {
            ipc::WCMD_CREATE_SURFACE => self.handle_create_surface(data, world),
            ipc::WCMD_DESTROY_SURFACE => self.handle_destroy_surface(data, world),
            ipc::WCMD_IMPORT_ATTACH => self.handle_import_attach(data, world),
            ipc::WCMD_DAMAGE => self.handle_damage(data, world),
            ipc::WCMD_COMMIT => self.handle_commit(data, world),
            other => {
                warn!("wayland-cmd: unknown command type {}", other);
                false
            }
        }
    }

    fn handle_create_surface(&mut self, data: &[u8], world: &mut BloomWorld) -> bool {
        if data.len() < 8 {
            return false;
        }
        let reply_port = u32::from_ne_bytes(data[4..8].try_into().unwrap_or([0; 4]));
        let bloom_id = world.scene.create_surface(self.wayland_client_id).unwrap_or(0);
        debug!("wayland-cmd: created surface bloom_id={}", bloom_id);
        let _ = port_send_all(reply_port, &bloom_id.to_ne_bytes());
        true
    }

    fn handle_destroy_surface(&mut self, data: &[u8], world: &mut BloomWorld) -> bool {
        if data.len() < 8 {
            return false;
        }
        let bloom_surface_id = u32::from_ne_bytes(data[4..8].try_into().unwrap_or([0; 4]));
        if let Some(released) =
            world.scene.destroy_surface(self.wayland_client_id, bloom_surface_id)
        {
            for buf_id in released {
                world.display.release_buffer(buf_id);
                if let Some(key) = self.bloom_to_buf_key.remove(&buf_id) {
                    self.buf_key_to_bloom.remove(&key);
                    let msg = ipc::encode_buffer_release(key);
                    let _ = port_send_all(self.evt_write, &msg);
                }
            }
        }
        true
    }

    fn handle_import_attach(&mut self, data: &[u8], world: &mut BloomWorld) -> bool {
        if data.len() < 32 {
            return false;
        }
        let bloom_surface_id = u32::from_ne_bytes(data[4..8].try_into().unwrap_or([0; 4]));
        let wl_buf_key = u32::from_ne_bytes(data[8..12].try_into().unwrap_or([0; 4]));
        let handle = u32::from_ne_bytes(data[12..16].try_into().unwrap_or([0; 4]));
        let width = u32::from_ne_bytes(data[16..20].try_into().unwrap_or([0; 4]));
        let height = u32::from_ne_bytes(data[20..24].try_into().unwrap_or([0; 4]));
        let stride = u32::from_ne_bytes(data[24..28].try_into().unwrap_or([0; 4]));
        let format = u32::from_ne_bytes(data[28..32].try_into().unwrap_or([0; 4]));

        let pixel_fmt = match format {
            0 => PixelFormat::Bgra8888, // Wayland ARGB8888
            1 => PixelFormat::Bgrx8888, // Wayland XRGB8888
            _ => PixelFormat::Bgra8888,
        };

        let buffer_id =
            match world.display.import_buffer(handle, width, height, stride, pixel_fmt, 0) {
                Some(id) => id,
                None => {
                    warn!("wayland-cmd: import_buffer failed for surface {}", bloom_surface_id);
                    return false;
                }
            };

        // Track key → bloom_id mapping for later release.
        self.buf_key_to_bloom.insert(wl_buf_key, buffer_id);
        self.bloom_to_buf_key.insert(buffer_id, wl_buf_key);

        world.scene.attach_pending_buffer(
            self.wayland_client_id,
            bloom_surface_id,
            SurfaceBuffer { buffer_id, width, height, stride },
        );
        false
    }

    fn handle_damage(&mut self, data: &[u8], world: &mut BloomWorld) -> bool {
        if data.len() < 24 {
            return false;
        }
        let bloom_surface_id = u32::from_ne_bytes(data[4..8].try_into().unwrap_or([0; 4]));
        let x = i32::from_ne_bytes(data[8..12].try_into().unwrap_or([0; 4]));
        let y = i32::from_ne_bytes(data[12..16].try_into().unwrap_or([0; 4]));
        let w = u32::from_ne_bytes(data[16..20].try_into().unwrap_or([0; 4]));
        let h = u32::from_ne_bytes(data[20..24].try_into().unwrap_or([0; 4]));
        // Clamp x/y to zero: Wayland permits negative damage coordinates to
        // indicate off-screen area, but the scene damage tracker uses u32.
        let x = x.max(0) as u32;
        let y = y.max(0) as u32;
        world.scene.damage_pending(
            self.wayland_client_id,
            bloom_surface_id,
            abi::display_protocol::Rect { x, y, w, h },
        );
        false
    }

    fn handle_commit(&mut self, data: &[u8], world: &mut BloomWorld) -> bool {
        if data.len() < 12 {
            return false;
        }
        let bloom_surface_id = u32::from_ne_bytes(data[4..8].try_into().unwrap_or([0; 4]));

        let result = match world.scene.commit_surface(self.wayland_client_id, bloom_surface_id) {
            Some(r) => r,
            None => {
                warn!("wayland-cmd: commit_surface({}) failed", bloom_surface_id);
                return false;
            }
        };

        // Release replaced buffers.
        for released_id in &result.released_buffer_ids {
            world.display.release_buffer(*released_id);
            if let Some(key) = self.bloom_to_buf_key.remove(released_id) {
                self.buf_key_to_bloom.remove(&key);
                let msg = ipc::encode_buffer_release(key);
                let _ = port_send_all(self.evt_write, &msg);
            }
        }

        world.apply_commit_damage(&result);

        result.changed
    }
}

impl BloomService for WaylandCommandService {
    fn name(&self) -> &'static str {
        "wayland-cmd"
    }

    fn interests(&self) -> &[Interest] {
        &self.interests
    }

    fn dispatch(&mut self, _event: LoopEvent, world: &mut BloomWorld) -> LoopAction {
        let mut buf = [0u8; 512];
        let mut repaint = false;

        loop {
            match vfs_read(self.cmd_read_fd, &mut buf) {
                Ok(0) | Err(_) => break,
                Ok(n) => {
                    if self.handle_command(&buf[..n], world) {
                        repaint = true;
                    }
                }
            }
        }

        if repaint { LoopAction::RequestRepaint } else { LoopAction::None }
    }
}
