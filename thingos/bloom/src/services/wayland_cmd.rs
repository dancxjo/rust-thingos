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
use alloc::string::String;
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
    rx_buf: Vec<u8>,
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
            rx_buf: Vec::new(),
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
            ipc::WCMD_SET_CHROME => self.handle_set_chrome(data, world),
            ipc::WCMD_SET_TITLE => self.handle_set_title(data, world),
            ipc::WCMD_SET_SUBSURFACE => self.handle_set_subsurface(data, world),
            ipc::WCMD_SET_LAYER_SURFACE => self.handle_set_layer_surface(data, world),
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
        if bloom_id != 0 {
            let _ = world.scene.set_surface_chrome(
                self.wayland_client_id,
                bloom_id,
                crate::scene::SurfaceChrome {
                    titlebar_height: blossom::DEFAULT_TITLEBAR_HEIGHT,
                    frame_thickness: blossom::DEFAULT_FRAME_THICKNESS,
                },
            );
            stem::info!(
                "bloom: registered titlebar drag zone surface={} height={} frame={}",
                bloom_id,
                blossom::DEFAULT_TITLEBAR_HEIGHT,
                blossom::DEFAULT_FRAME_THICKNESS
            );
        }
        debug!("wayland-cmd: created surface bloom_id={}", bloom_id);
        let _ = port_send_all(reply_port, &bloom_id.to_ne_bytes());
        if bloom_id != 0 {
            world.sync_wayland_session_fs(alloc::format!("surface_created id={}\n", bloom_id));
        }
        true
    }

    fn handle_destroy_surface(&mut self, data: &[u8], world: &mut BloomWorld) -> bool {
        if data.len() < 8 {
            return false;
        }
        let bloom_surface_id = u32::from_ne_bytes(data[4..8].try_into().unwrap_or([0; 4]));
        if let Some(rect) = world.scene.surface_visual_rect(bloom_surface_id) {
            world.damage.mark_rect(rect);
        }
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
            world.remove_wayland_session_surface(
                bloom_surface_id,
                alloc::format!("surface_destroyed id={}\n", bloom_surface_id),
            );
        }
        true
    }

    fn handle_import_attach(&mut self, data: &[u8], world: &mut BloomWorld) -> bool {
        if data.len() < 48 {
            return false;
        }
        let bloom_surface_id = u32::from_ne_bytes(data[4..8].try_into().unwrap_or([0; 4]));
        let wl_buf_key = u32::from_ne_bytes(data[8..12].try_into().unwrap_or([0; 4]));
        let handle = u32::from_ne_bytes(data[12..16].try_into().unwrap_or([0; 4]));
        let width = u32::from_ne_bytes(data[16..20].try_into().unwrap_or([0; 4]));
        let height = u32::from_ne_bytes(data[20..24].try_into().unwrap_or([0; 4]));
        let stride = u32::from_ne_bytes(data[24..28].try_into().unwrap_or([0; 4]));
        let format = u32::from_ne_bytes(data[28..32].try_into().unwrap_or([0; 4]));
        let offset = u64::from_ne_bytes(data[32..40].try_into().unwrap_or([0; 8]));
        let modifier = u64::from_ne_bytes(data[40..48].try_into().unwrap_or([0; 8]));

        let pixel_fmt = match format {
            1 => PixelFormat::Bgra8888,
            2 => PixelFormat::Bgrx8888,
            3 => PixelFormat::Rgb565,
            _ => PixelFormat::Bgra8888,
        };

        let buffer_id = match world
            .display
            .import_buffer(handle, width, height, stride, pixel_fmt, offset, modifier)
        {
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

        let result = match world.scene.commit_surface(
            self.wayland_client_id,
            bloom_surface_id,
            world.primary.width,
            world.primary.height,
        ) {
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
        world.sync_wayland_session_fs(alloc::format!(
            "surface_committed id={} frame_serial={}\n",
            bloom_surface_id,
            result.frame_serial
        ));

        result.changed
    }

    fn handle_set_chrome(&mut self, data: &[u8], world: &mut BloomWorld) -> bool {
        if data.len() < 16 {
            return false;
        }
        let bloom_surface_id = u32::from_ne_bytes(data[4..8].try_into().unwrap_or([0; 4]));
        let titlebar_height = u32::from_ne_bytes(data[8..12].try_into().unwrap_or([0; 4]));
        let frame_thickness = u32::from_ne_bytes(data[12..16].try_into().unwrap_or([0; 4]));
        let old_visual = world.scene.surface_visual_rect(bloom_surface_id);
        if world.scene.set_surface_chrome(
            self.wayland_client_id,
            bloom_surface_id,
            crate::scene::SurfaceChrome { titlebar_height, frame_thickness },
        ) {
            debug!(
                "bloom: registered titlebar drag zone surface={} height={} frame={}",
                bloom_surface_id, titlebar_height, frame_thickness
            );
            if let Some(rect) = old_visual {
                world.damage.mark_rect(rect);
            }
            if let Some(rect) = world.scene.surface_visual_rect(bloom_surface_id) {
                world.damage.mark_rect(rect);
            }
            world.sync_wayland_session_fs(alloc::format!(
                "surface_chrome_changed id={}\n",
                bloom_surface_id
            ));
        }
        true
    }

    fn handle_set_title(&mut self, data: &[u8], world: &mut BloomWorld) -> bool {
        if data.len() < 8 + ipc::MAX_TITLE_BYTES {
            return false;
        }
        let title_len = (data[1] as usize).min(ipc::MAX_TITLE_BYTES);
        let bloom_surface_id = u32::from_ne_bytes(data[4..8].try_into().unwrap_or([0; 4]));
        let title = String::from_utf8_lossy(&data[8..8 + title_len]).into_owned();
        if world.scene.set_surface_title(self.wayland_client_id, bloom_surface_id, title) {
            if let Some(rect) = world.scene.surface_visual_rect(bloom_surface_id) {
                world.damage.mark_rect(rect);
            }
            world.sync_wayland_session_fs(alloc::format!(
                "surface_title_changed id={}\n",
                bloom_surface_id
            ));
            return true;
        }
        false
    }

    fn handle_set_subsurface(&mut self, data: &[u8], world: &mut BloomWorld) -> bool {
        if data.len() < 24 {
            return false;
        }
        let child = u32::from_ne_bytes(data[4..8].try_into().unwrap_or([0; 4]));
        let parent = u32::from_ne_bytes(data[8..12].try_into().unwrap_or([0; 4]));
        let x = i32::from_ne_bytes(data[12..16].try_into().unwrap_or([0; 4]));
        let y = i32::from_ne_bytes(data[16..20].try_into().unwrap_or([0; 4]));
        let z_above = i32::from_ne_bytes(data[20..24].try_into().unwrap_or([0; 4]));
        let parent_opt = if parent == 0 { None } else { Some(parent) };
        let old_visual = world.scene.surface_visual_rect(child);
        if world.scene.set_subsurface(child, parent_opt, x, y, z_above) {
            if let Some(rect) = old_visual {
                world.damage.mark_rect(rect);
            }
            if let Some(rect) = world.scene.surface_visual_rect(child) {
                world.damage.mark_rect(rect);
            }
            world.sync_wayland_session_fs(alloc::format!(
                "subsurface_state child={} parent={} x={} y={} z_above={}\n",
                child,
                parent,
                x,
                y,
                z_above
            ));
            return true;
        }
        false
    }

    /// Apply a `WCMD_SET_LAYER_SURFACE` from the Wayland thread.
    ///
    /// Computes the target rectangle and z-order via
    /// [`blossom::compute_layer_placement`] using the current primary output
    /// size, then pushes the result through the same scene mutators that
    /// xdg-shell surfaces use.  The chrome is also cleared so the surface
    /// renders without a titlebar/frame regardless of the default applied at
    /// `WCMD_CREATE_SURFACE` time.
    fn handle_set_layer_surface(&mut self, data: &[u8], world: &mut BloomWorld) -> bool {
        if data.len() < 44 {
            return false;
        }
        let active = data[1] != 0;
        let bloom_surface_id = u32::from_ne_bytes(data[4..8].try_into().unwrap_or([0; 4]));
        let layer = u32::from_ne_bytes(data[8..12].try_into().unwrap_or([0; 4]));
        let anchor = u32::from_ne_bytes(data[12..16].try_into().unwrap_or([0; 4]));
        let exclusive_zone = i32::from_ne_bytes(data[16..20].try_into().unwrap_or([0; 4]));
        let margin_top = i32::from_ne_bytes(data[20..24].try_into().unwrap_or([0; 4]));
        let margin_right = i32::from_ne_bytes(data[24..28].try_into().unwrap_or([0; 4]));
        let margin_bottom = i32::from_ne_bytes(data[28..32].try_into().unwrap_or([0; 4]));
        let margin_left = i32::from_ne_bytes(data[32..36].try_into().unwrap_or([0; 4]));
        let width = u32::from_ne_bytes(data[36..40].try_into().unwrap_or([0; 4]));
        let height = u32::from_ne_bytes(data[40..44].try_into().unwrap_or([0; 4]));

        if bloom_surface_id == 0 {
            return false;
        }

        if !active {
            // Surface is no longer a layer surface — clear its layer-shell
            // placement contribution.  We deliberately do not destroy the
            // scene surface (the underlying wl_surface remains live).
            debug!("wayland-cmd: layer-surface deactivated id={}", bloom_surface_id);
            return false;
        }

        let layer_enum = match blossom::LayerShellLayer::from_wire(layer) {
            Some(l) => l,
            None => {
                warn!(
                    "wayland-cmd: invalid layer-shell layer value={} (surface={})",
                    layer, bloom_surface_id
                );
                return false;
            }
        };
        let cfg = blossom::LayerSurfaceConfig {
            layer: layer_enum,
            anchor,
            size: (width, height),
            exclusive_zone,
            margin_top,
            margin_right,
            margin_bottom,
            margin_left,
        };
        let (out_w, out_h) = world.display.output_size();
        let p = blossom::compute_layer_placement(out_w, out_h, &cfg);

        // Layer surfaces have no compositor-drawn chrome.  Clear it so any
        // default titlebar/frame applied at create-surface time goes away.
        let _ = world.scene.set_surface_chrome(
            self.wayland_client_id,
            bloom_surface_id,
            crate::scene::SurfaceChrome { titlebar_height: 0, frame_thickness: 0 },
        );

        if p.x < 0 || p.y < 0 {
            warn!(
                "wayland-cmd: layer-surface id={} placement out of bounds x={} y={} (clamped to 0)",
                bloom_surface_id, p.x, p.y
            );
        }
        let rect = abi::display_protocol::Rect {
            x: p.x.max(0) as u32,
            y: p.y.max(0) as u32,
            w: p.width,
            h: p.height,
        };
        let old_visual = world.scene.surface_visual_rect(bloom_surface_id);
        world.scene.set_pending_dest_rect(self.wayland_client_id, bloom_surface_id, rect);
        world.scene.set_pending_z_order(self.wayland_client_id, bloom_surface_id, p.z_order);
        if let Some(r) = old_visual {
            world.damage.mark_rect(r);
        }
        world.damage.mark_rect(rect);
        debug!(
            "wayland-cmd: layer-surface id={} layer={:?} rect=({},{},{},{}) z={}",
            bloom_surface_id, layer_enum, rect.x, rect.y, rect.w, rect.h, p.z_order
        );
        world.sync_wayland_session_fs(alloc::format!(
            "layer_surface id={} layer={} x={} y={} w={} h={} z={}\n",
            bloom_surface_id,
            layer,
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            p.z_order
        ));
        true
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
                    self.rx_buf.extend_from_slice(&buf[..n]);
                }
            }
        }

        let mut consumed = 0usize;
        while consumed < self.rx_buf.len() {
            let Some(command_len) = wayland_command_len(&self.rx_buf[consumed..]) else {
                break;
            };
            if consumed + command_len > self.rx_buf.len() {
                break;
            }
            let command = self.rx_buf[consumed..consumed + command_len].to_vec();
            if self.handle_command(&command, world) {
                repaint = true;
            }
            consumed += command_len;
        }

        if consumed > 0 {
            self.rx_buf.drain(..consumed);
        }

        if repaint { LoopAction::RequestRepaint } else { LoopAction::None }
    }
}

fn wayland_command_len(data: &[u8]) -> Option<usize> {
    let msg_type = *data.first()?;
    let len = match msg_type {
        ipc::WCMD_CREATE_SURFACE => 8,
        ipc::WCMD_DESTROY_SURFACE => 8,
        ipc::WCMD_IMPORT_ATTACH => 48,
        ipc::WCMD_DAMAGE => 24,
        ipc::WCMD_COMMIT => 12,
        ipc::WCMD_SET_CHROME => 16,
        ipc::WCMD_SET_TITLE => 8 + ipc::MAX_TITLE_BYTES,
        ipc::WCMD_SET_SUBSURFACE => 24,
        ipc::WCMD_SET_LAYER_SURFACE => 44,
        _ => 1,
    };
    Some(len)
}
