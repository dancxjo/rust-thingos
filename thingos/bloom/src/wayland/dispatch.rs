//! Wayland message dispatcher.
//!
//! Each `dispatch_*` function handles messages arriving on a particular
//! interface.  The dispatcher receives a decoded [`WireMsg`] and mutates
//! the client state, emitting port commands to the main bloom thread as
//! needed.
//!
//! # Global table (advertised in `wl_registry`)
//!
//! | name | interface        | version |
//! |------|------------------|---------|
//! | 1    | wl_compositor    | 4       |
//! | 2    | wl_shm           | 1       |
//! | 3    | xdg_wm_base      | 1       |

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use blossom::{BlossomCommand, BlossomError};

use crate::wayland::client::{ObjectEntry, WaylandClient};
use crate::wayland::ipc;
use crate::wayland::wire::{read_i32, read_string, read_u32, WireMsg};

// ── Global registry constants ────────────────────────────────────────────────

pub const GLOBAL_WL_COMPOSITOR: u32 = 1;
pub const GLOBAL_WL_SHM: u32 = 2;
pub const GLOBAL_XDG_WM_BASE: u32 = 3;

// ── Top-level dispatcher ─────────────────────────────────────────────────────

/// Dispatch a single decoded Wayland message.
///
/// Returns a (possibly empty) list of raw IPC command bytes that must be sent
/// to the main bloom thread via the command port.
pub fn dispatch(
    msg: &WireMsg,
    client: &mut WaylandClient,
    blossom: &mut blossom::Blossom,
    cmd_write: u32,
    next_surface_key: &mut u32,
) -> Vec<Vec<u8>> {
    let obj_id = msg.object_id;

    // Look up the object type.  Clone/copy just the variant discriminant info
    // we need without holding the borrow.
    let obj_kind = match client.objects.get(&obj_id) {
        Some(e) => classify(e),
        None => {
            // Unknown object — send protocol error.
            client.send_protocol_error(obj_id, 0, "unknown object");
            return vec![];
        }
    };

    match obj_kind {
        ObjKind::Display => dispatch_display(msg, client, next_surface_key, cmd_write),
        ObjKind::Registry => dispatch_registry(msg, client),
        ObjKind::Compositor => dispatch_compositor(msg, client, next_surface_key, cmd_write),
        ObjKind::Shm => dispatch_shm(msg, client),
        ObjKind::ShmPool => dispatch_shm_pool(msg, client, obj_id),
        ObjKind::Buffer => dispatch_buffer(msg, client, obj_id),
        ObjKind::Surface => dispatch_surface(msg, client, obj_id, blossom, cmd_write),
        ObjKind::Callback => vec![],
        ObjKind::XdgWmBase => dispatch_xdg_wm_base(msg, client, obj_id, blossom),
        ObjKind::XdgPositioner => vec![],
        ObjKind::XdgSurface => dispatch_xdg_surface(msg, client, obj_id, blossom, cmd_write),
        ObjKind::XdgToplevel => dispatch_xdg_toplevel(msg, client, obj_id, blossom),
        ObjKind::Destroyed | ObjKind::Unknown => vec![],
    }
}

// ── Object kind classifier ────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq)]
enum ObjKind {
    Display,
    Registry,
    Compositor,
    Shm,
    ShmPool,
    Buffer,
    Surface,
    Callback,
    XdgWmBase,
    XdgPositioner,
    XdgSurface,
    XdgToplevel,
    Destroyed,
    Unknown,
}

fn classify(e: &ObjectEntry) -> ObjKind {
    match e {
        ObjectEntry::Display => ObjKind::Display,
        ObjectEntry::Registry => ObjKind::Registry,
        ObjectEntry::Compositor => ObjKind::Compositor,
        ObjectEntry::Shm => ObjKind::Shm,
        ObjectEntry::ShmPool { .. } => ObjKind::ShmPool,
        ObjectEntry::Buffer { .. } => ObjKind::Buffer,
        ObjectEntry::Surface { .. } => ObjKind::Surface,
        ObjectEntry::Callback => ObjKind::Callback,
        ObjectEntry::XdgWmBase => ObjKind::XdgWmBase,
        ObjectEntry::XdgPositioner => ObjKind::XdgPositioner,
        ObjectEntry::XdgSurface { .. } => ObjKind::XdgSurface,
        ObjectEntry::XdgToplevel { .. } => ObjKind::XdgToplevel,
        ObjectEntry::Destroyed => ObjKind::Destroyed,
    }
}

// ── wl_display ────────────────────────────────────────────────────────────────

/// wl_display opcodes
const WL_DISPLAY_SYNC: u16 = 0;
const WL_DISPLAY_GET_REGISTRY: u16 = 1;

fn dispatch_display(
    msg: &WireMsg,
    client: &mut WaylandClient,
    _next_surface_key: &mut u32,
    _cmd_write: u32,
) -> Vec<Vec<u8>> {
    use crate::wayland::wire::{encode, encode_string};
    match msg.opcode {
        WL_DISPLAY_SYNC => {
            // sync(new_id) → create wl_callback, send done immediately.
            let new_id = match read_u32(&msg.data, 0) {
                Some(id) => id,
                None => return vec![],
            };
            client.insert(new_id, ObjectEntry::Callback);
            // wl_callback.done(serial=0)
            let payload = 0u32.to_ne_bytes();
            client.send(new_id, 0, &payload);
            client.destroy(new_id);
        }
        WL_DISPLAY_GET_REGISTRY => {
            let new_id = match read_u32(&msg.data, 0) {
                Some(id) => id,
                None => return vec![],
            };
            client.insert(new_id, ObjectEntry::Registry);
            // Send wl_registry.global for each advertised global.
            // wl_registry.global opcode = 0: (name: uint, interface: string, version: uint)
            for &(name, iface, version) in &[
                (GLOBAL_WL_COMPOSITOR, "wl_compositor", 4u32),
                (GLOBAL_WL_SHM, "wl_shm", 1u32),
                (GLOBAL_XDG_WM_BASE, "xdg_wm_base", 1u32),
            ] {
                let mut p = Vec::new();
                p.extend_from_slice(&name.to_ne_bytes());
                p.extend_from_slice(&encode_string(iface));
                p.extend_from_slice(&version.to_ne_bytes());
                client.send(new_id, 0, &p);
            }
            // No wl_registry.global_done in the Wayland protocol;
            // clients learn all globals when the initial global list is done
            // (they stop waiting after bind).
        }
        _ => {}
    }
    vec![]
}

// ── wl_registry ───────────────────────────────────────────────────────────────

/// wl_registry.bind opcode = 0
const WL_REGISTRY_BIND: u16 = 0;

fn dispatch_registry(msg: &WireMsg, client: &mut WaylandClient) -> Vec<Vec<u8>> {
    if msg.opcode != WL_REGISTRY_BIND {
        return vec![];
    }
    // bind(name: uint, interface: string, version: uint, new_id: uint)
    let name = match read_u32(&msg.data, 0) {
        Some(n) => n,
        None => return vec![],
    };
    let (_, consumed) = match read_string(&msg.data, 4) {
        Some(s) => s,
        None => return vec![],
    };
    let after_str = 4 + consumed;
    let _version = read_u32(&msg.data, after_str);
    let new_id = match read_u32(&msg.data, after_str + 4) {
        Some(id) => id,
        None => return vec![],
    };

    match name {
        GLOBAL_WL_COMPOSITOR => {
            client.insert(new_id, ObjectEntry::Compositor);
        }
        GLOBAL_WL_SHM => {
            client.insert(new_id, ObjectEntry::Shm);
            // Advertise ARGB8888 format (0) and XRGB8888 format (1).
            // wl_shm.format opcode = 0: (format: uint)
            client.send(new_id, 0, &0u32.to_ne_bytes()); // ARGB8888
            client.send(new_id, 0, &1u32.to_ne_bytes()); // XRGB8888
        }
        GLOBAL_XDG_WM_BASE => {
            client.insert(new_id, ObjectEntry::XdgWmBase);
        }
        _ => {
            client.send_protocol_error(new_id, 0, "unknown global");
        }
    }
    vec![]
}

// ── wl_compositor ─────────────────────────────────────────────────────────────

const WL_COMPOSITOR_CREATE_SURFACE: u16 = 0;
const WL_COMPOSITOR_CREATE_REGION: u16 = 1;

fn dispatch_compositor(
    msg: &WireMsg,
    client: &mut WaylandClient,
    next_surface_key: &mut u32,
    cmd_write: u32,
) -> Vec<Vec<u8>> {
    match msg.opcode {
        WL_COMPOSITOR_CREATE_SURFACE => {
            let new_id = match read_u32(&msg.data, 0) {
                Some(id) => id,
                None => return vec![],
            };
            // Allocate a bloom_surface_id synchronously via port.
            let bloom_surface_id = alloc_bloom_surface(cmd_write);
            *next_surface_key += 1;
            client.insert(
                new_id,
                ObjectEntry::Surface {
                    bloom_surface_id,
                    xdg_surface_obj: None,
                    pending_buffer: None,
                    pending_damage: None,
                    pending_frame_cb: None,
                },
            );
        }
        WL_COMPOSITOR_CREATE_REGION => {
            // Regions are not used by the renderer; just register as Unknown.
            if let Some(new_id) = read_u32(&msg.data, 0) {
                client.insert(new_id, ObjectEntry::Destroyed);
            }
        }
        _ => {}
    }
    vec![]
}

/// Synchronously allocate a bloom scene surface via the command port.
///
/// Sends [`ipc::WCMD_CREATE_SURFACE`] with a one-shot reply port, blocks until
/// the main thread replies with the `bloom_surface_id`.
fn alloc_bloom_surface(cmd_write: u32) -> u32 {
    use stem::syscall::{port_create, port_recv, port_send_all};
    let (reply_write, reply_read) = match port_create(16) {
        Ok(p) => p,
        Err(_) => return 0,
    };
    let msg = ipc::encode_create_surface(reply_write);
    let _ = port_send_all(cmd_write, &msg);
    let mut buf = [0u8; 4];
    let _ = port_recv(reply_read, &mut buf);
    // Close the reply port handles (best-effort).
    use stem::syscall::vfs::vfs_close;
    if let Ok(fd) = stem::syscall::vfs::vfs_handle_from_port(reply_write) {
        let _ = vfs_close(fd);
    }
    if let Ok(fd) = stem::syscall::vfs::vfs_handle_from_port(reply_read) {
        let _ = vfs_close(fd);
    }
    u32::from_ne_bytes(buf)
}

// ── wl_shm ────────────────────────────────────────────────────────────────────

const WL_SHM_CREATE_POOL: u16 = 0;

fn dispatch_shm(msg: &WireMsg, client: &mut WaylandClient) -> Vec<Vec<u8>> {
    if msg.opcode != WL_SHM_CREATE_POOL {
        return vec![];
    }
    // create_pool(new_id: new_id<wl_shm_pool>, fd: fd, size: int)
    // In ThingOS the shared-memory "fd" is passed inline as a u32 ThingId.
    let new_id = match read_u32(&msg.data, 0) {
        Some(id) => id,
        None => return vec![],
    };
    let handle = read_u32(&msg.data, 4).unwrap_or(0);
    let size = read_u32(&msg.data, 8).unwrap_or(0);
    client.insert(new_id, ObjectEntry::ShmPool { handle, size });
    vec![]
}

// ── wl_shm_pool ───────────────────────────────────────────────────────────────

const WL_SHM_POOL_CREATE_BUFFER: u16 = 0;
const WL_SHM_POOL_DESTROY: u16 = 1;
const WL_SHM_POOL_RESIZE: u16 = 2;

fn dispatch_shm_pool(
    msg: &WireMsg,
    client: &mut WaylandClient,
    obj_id: u32,
) -> Vec<Vec<u8>> {
    let handle = match client.objects.get(&obj_id) {
        Some(ObjectEntry::ShmPool { handle, .. }) => *handle,
        _ => return vec![],
    };
    match msg.opcode {
        WL_SHM_POOL_CREATE_BUFFER => {
            // create_buffer(new_id, offset, width, height, stride, format)
            let new_id = match read_u32(&msg.data, 0) {
                Some(id) => id,
                None => return vec![],
            };
            let offset = read_u32(&msg.data, 4).unwrap_or(0);
            let width = read_u32(&msg.data, 8).unwrap_or(0);
            let height = read_u32(&msg.data, 12).unwrap_or(0);
            let stride = read_u32(&msg.data, 16).unwrap_or(0);
            let format = read_u32(&msg.data, 20).unwrap_or(0);
            client.insert(
                new_id,
                ObjectEntry::Buffer { handle, offset, width, height, stride, format },
            );
        }
        WL_SHM_POOL_DESTROY => {
            client.destroy(obj_id);
        }
        WL_SHM_POOL_RESIZE => {
            if let Some(ObjectEntry::ShmPool { size, .. }) = client.objects.get_mut(&obj_id) {
                *size = read_u32(&msg.data, 0).unwrap_or(*size);
            }
        }
        _ => {}
    }
    vec![]
}

// ── wl_buffer ─────────────────────────────────────────────────────────────────

const WL_BUFFER_DESTROY: u16 = 0;

fn dispatch_buffer(
    msg: &WireMsg,
    client: &mut WaylandClient,
    obj_id: u32,
) -> Vec<Vec<u8>> {
    if msg.opcode == WL_BUFFER_DESTROY {
        client.destroy(obj_id);
    }
    vec![]
}

// ── wl_surface ────────────────────────────────────────────────────────────────

const WL_SURFACE_DESTROY: u16 = 0;
const WL_SURFACE_ATTACH: u16 = 1;
const WL_SURFACE_DAMAGE: u16 = 2;
const WL_SURFACE_FRAME: u16 = 3;
const WL_SURFACE_SET_OPAQUE_REGION: u16 = 4;
const WL_SURFACE_SET_INPUT_REGION: u16 = 5;
const WL_SURFACE_COMMIT: u16 = 6;
const WL_SURFACE_SET_BUFFER_TRANSFORM: u16 = 7;
const WL_SURFACE_SET_BUFFER_SCALE: u16 = 8;
const WL_SURFACE_DAMAGE_BUFFER: u16 = 9;

fn dispatch_surface(
    msg: &WireMsg,
    client: &mut WaylandClient,
    obj_id: u32,
    blossom: &mut blossom::Blossom,
    cmd_write: u32,
) -> Vec<Vec<u8>> {
    let mut ipc_cmds: Vec<Vec<u8>> = vec![];

    match msg.opcode {
        WL_SURFACE_DESTROY => {
            let bloom_id = client.bloom_surface_id(obj_id).unwrap_or(0);
            client.destroy(obj_id);
            if bloom_id != 0 {
                ipc_cmds.push(ipc::encode_destroy_surface(bloom_id).to_vec());
            }
        }

        WL_SURFACE_ATTACH => {
            // attach(buffer: object<wl_buffer>, x: int, y: int)
            let buf_id = read_u32(&msg.data, 0).unwrap_or(0);
            if let Some(ObjectEntry::Surface { pending_buffer, .. }) =
                client.objects.get_mut(&obj_id)
            {
                *pending_buffer = if buf_id == 0 { None } else { Some(buf_id) };
            }
        }

        WL_SURFACE_DAMAGE | WL_SURFACE_DAMAGE_BUFFER => {
            let x = read_i32(&msg.data, 0).unwrap_or(0) as u32;
            let y = read_i32(&msg.data, 4).unwrap_or(0) as u32;
            let w = read_i32(&msg.data, 8).unwrap_or(0) as u32;
            let h = read_i32(&msg.data, 12).unwrap_or(0) as u32;
            if let Some(ObjectEntry::Surface { pending_damage, .. }) =
                client.objects.get_mut(&obj_id)
            {
                *pending_damage = Some((x, y, w, h));
            }
        }

        WL_SURFACE_FRAME => {
            let cb_id = read_u32(&msg.data, 0).unwrap_or(0);
            if cb_id != 0 {
                client.insert(cb_id, ObjectEntry::Callback);
                if let Some(ObjectEntry::Surface { pending_frame_cb, .. }) =
                    client.objects.get_mut(&obj_id)
                {
                    *pending_frame_cb = Some(cb_id);
                }
            }
        }

        WL_SURFACE_SET_OPAQUE_REGION
        | WL_SURFACE_SET_INPUT_REGION
        | WL_SURFACE_SET_BUFFER_TRANSFORM
        | WL_SURFACE_SET_BUFFER_SCALE => {
            // Accepted as no-ops.
        }

        WL_SURFACE_COMMIT => {
            ipc_cmds.extend(handle_surface_commit(obj_id, client, blossom, cmd_write));
        }

        _ => {}
    }

    ipc_cmds
}

/// Handle `wl_surface.commit`: validate xdg-shell state, then issue IPC commands.
fn handle_surface_commit(
    wl_surface_obj: u32,
    client: &mut WaylandClient,
    blossom: &mut blossom::Blossom,
    cmd_write: u32,
) -> Vec<Vec<u8>> {
    let mut out: Vec<Vec<u8>> = vec![];

    // Extract surface state.
    let (bloom_surface_id, xdg_obj, pending_buffer, pending_damage, pending_frame_cb) = {
        match client.objects.get(&wl_surface_obj) {
            Some(ObjectEntry::Surface {
                bloom_surface_id,
                xdg_surface_obj,
                pending_buffer,
                pending_damage,
                pending_frame_cb,
            }) => (
                *bloom_surface_id,
                *xdg_surface_obj,
                *pending_buffer,
                *pending_damage,
                *pending_frame_cb,
            ),
            _ => return out,
        }
    };

    // xdg-shell lifecycle validation.
    if let Some(xdg_id) = xdg_obj {
        let has_buffer = pending_buffer.is_some();
        match blossom.on_surface_commit(xdg_id, has_buffer) {
            Err(BlossomError::CommitBeforeAckConfigure { .. }) => {
                client.send_protocol_error(
                    xdg_id,
                    1, // xdg_surface error: unconfigured_buffer
                    "buffer committed before ack_configure",
                );
                return out;
            }
            Err(_) => return out,
            Ok(blossom_cmds) => {
                // Handle any blossom commands (e.g. SendXdgSurfaceConfigure re-sent).
                send_blossom_commands(client, &blossom_cmds, cmd_write);
            }
        }
    }

    // Import and attach pending buffer.
    if let Some(buf_obj) = pending_buffer {
        if let Some(ObjectEntry::Buffer { handle, width, height, stride, format, .. }) =
            client.objects.get(&buf_obj).cloned()
        {
            let key = client.alloc_buf_key();
            client.buf_key_to_obj.insert(key, buf_obj);
            out.push(
                ipc::encode_import_attach(
                    bloom_surface_id, key, handle, width, height, stride, format,
                )
                .to_vec(),
            );
        }
    }

    // Damage.
    if let Some((x, y, w, h)) = pending_damage {
        out.push(ipc::encode_damage(bloom_surface_id, x, y, w, h).to_vec());
    }

    // Register frame callback with IPC if present.
    let has_cb = pending_frame_cb.is_some();
    let cb_key = if let Some(cb_id) = pending_frame_cb {
        // Map bloom_surface_id → cb_id so we can fire it on WEVT_FRAME_DONE.
        client.add_frame_cb(bloom_surface_id, cb_id);
        cb_id
    } else {
        0
    };

    // Commit.
    out.push(ipc::encode_commit(bloom_surface_id, has_cb, cb_key).to_vec());

    // Clear pending state.
    if let Some(ObjectEntry::Surface {
        pending_buffer,
        pending_damage,
        pending_frame_cb,
        ..
    }) = client.objects.get_mut(&wl_surface_obj)
    {
        *pending_buffer = None;
        *pending_damage = None;
        *pending_frame_cb = None;
    }

    out
}

// ── xdg_wm_base ───────────────────────────────────────────────────────────────

const XDG_WM_BASE_DESTROY: u16 = 0;
const XDG_WM_BASE_CREATE_POSITIONER: u16 = 1;
const XDG_WM_BASE_GET_XDG_SURFACE: u16 = 2;
const XDG_WM_BASE_PONG: u16 = 3;

fn dispatch_xdg_wm_base(
    msg: &WireMsg,
    client: &mut WaylandClient,
    obj_id: u32,
    blossom: &mut blossom::Blossom,
) -> Vec<Vec<u8>> {
    match msg.opcode {
        XDG_WM_BASE_DESTROY => {
            client.destroy(obj_id);
        }
        XDG_WM_BASE_CREATE_POSITIONER => {
            if let Some(new_id) = read_u32(&msg.data, 0) {
                client.insert(new_id, ObjectEntry::XdgPositioner);
            }
            blossom.create_positioner();
        }
        XDG_WM_BASE_GET_XDG_SURFACE => {
            // get_xdg_surface(new_id, surface)
            let new_id = match read_u32(&msg.data, 0) {
                Some(id) => id,
                None => return vec![],
            };
            let wl_surface_obj = match read_u32(&msg.data, 4) {
                Some(id) => id,
                None => return vec![],
            };
            let bloom_surface_id = match client.bloom_surface_id(wl_surface_obj) {
                Some(id) => id,
                None => {
                    client.send_protocol_error(obj_id, 0, "invalid wl_surface");
                    return vec![];
                }
            };
            let client_id = client.fd;
            match blossom.get_xdg_surface(client_id, new_id, bloom_surface_id) {
                Ok(_) => {
                    client.insert(new_id, ObjectEntry::XdgSurface { bloom_surface_id });
                    // Link the wl_surface back to this xdg_surface.
                    if let Some(ObjectEntry::Surface { xdg_surface_obj, .. }) =
                        client.objects.get_mut(&wl_surface_obj)
                    {
                        *xdg_surface_obj = Some(new_id);
                    }
                }
                Err(BlossomError::XdgSurfaceAlreadyExists { .. }) => {
                    client.send_protocol_error(obj_id, 0, "wl_surface already has xdg_surface");
                }
                Err(_) => {}
            }
        }
        XDG_WM_BASE_PONG => {
            let serial = read_u32(&msg.data, 0).unwrap_or(0);
            blossom.pong(serial);
        }
        _ => {}
    }
    vec![]
}

// ── xdg_surface ───────────────────────────────────────────────────────────────

const XDG_SURFACE_DESTROY: u16 = 0;
const XDG_SURFACE_GET_TOPLEVEL: u16 = 1;
const XDG_SURFACE_GET_POPUP: u16 = 2;
const XDG_SURFACE_SET_WINDOW_GEOMETRY: u16 = 3;
const XDG_SURFACE_ACK_CONFIGURE: u16 = 4;

fn dispatch_xdg_surface(
    msg: &WireMsg,
    client: &mut WaylandClient,
    obj_id: u32,
    blossom: &mut blossom::Blossom,
    cmd_write: u32,
) -> Vec<Vec<u8>> {
    let bloom_surface_id = match client.objects.get(&obj_id) {
        Some(ObjectEntry::XdgSurface { bloom_surface_id }) => *bloom_surface_id,
        _ => return vec![],
    };
    let client_id = client.fd;

    match msg.opcode {
        XDG_SURFACE_DESTROY => {
            if let Ok(cmds) = blossom.destroy_xdg_surface(obj_id) {
                send_blossom_commands(client, &cmds, cmd_write);
            }
            client.destroy(obj_id);
        }
        XDG_SURFACE_GET_TOPLEVEL => {
            let new_id = match read_u32(&msg.data, 0) {
                Some(id) => id,
                None => return vec![],
            };
            match blossom.get_toplevel(client_id, obj_id, new_id) {
                Ok(cmds) => {
                    client.insert(
                        new_id,
                        ObjectEntry::XdgToplevel { xdg_surface_obj: obj_id },
                    );
                    send_blossom_commands(client, &cmds, cmd_write);
                }
                Err(BlossomError::XdgSurfaceAlreadyHasRole { .. }) => {
                    client.send_protocol_error(obj_id, 4, "xdg_surface already has a role");
                }
                Err(_) => {}
            }
        }
        XDG_SURFACE_GET_POPUP => {
            // Unsupported in v1 — send protocol error.
            client.send_protocol_error(obj_id, 0, "get_popup not supported in v1");
        }
        XDG_SURFACE_SET_WINDOW_GEOMETRY => {
            let x = read_i32(&msg.data, 0).unwrap_or(0);
            let y = read_i32(&msg.data, 4).unwrap_or(0);
            let w = read_i32(&msg.data, 8).unwrap_or(0);
            let h = read_i32(&msg.data, 12).unwrap_or(0);
            let _ = blossom.set_window_geometry(obj_id, blossom::Rect { x, y, w, h });
        }
        XDG_SURFACE_ACK_CONFIGURE => {
            let serial = read_u32(&msg.data, 0).unwrap_or(0);
            match blossom.ack_configure(obj_id, serial) {
                Ok(_) => {}
                Err(BlossomError::UnknownSerial { .. }) => {
                    client.send_protocol_error(obj_id, 3, "invalid serial in ack_configure");
                }
                Err(_) => {}
            }
        }
        _ => {}
    }
    vec![]
}

// ── xdg_toplevel ─────────────────────────────────────────────────────────────

const XDG_TOPLEVEL_DESTROY: u16 = 0;
const XDG_TOPLEVEL_SET_PARENT: u16 = 1;
const XDG_TOPLEVEL_SET_TITLE: u16 = 2;
const XDG_TOPLEVEL_SET_APP_ID: u16 = 3;
const XDG_TOPLEVEL_SHOW_WINDOW_MENU: u16 = 4;
const XDG_TOPLEVEL_MOVE: u16 = 5;
const XDG_TOPLEVEL_RESIZE: u16 = 6;
const XDG_TOPLEVEL_SET_MAX_SIZE: u16 = 7;
const XDG_TOPLEVEL_SET_MIN_SIZE: u16 = 8;
const XDG_TOPLEVEL_SET_MAXIMIZED: u16 = 9;
const XDG_TOPLEVEL_UNSET_MAXIMIZED: u16 = 10;
const XDG_TOPLEVEL_SET_FULLSCREEN: u16 = 11;
const XDG_TOPLEVEL_UNSET_FULLSCREEN: u16 = 12;
const XDG_TOPLEVEL_SET_MINIMIZED: u16 = 13;

fn dispatch_xdg_toplevel(
    msg: &WireMsg,
    client: &mut WaylandClient,
    obj_id: u32,
    blossom: &mut blossom::Blossom,
) -> Vec<Vec<u8>> {
    match msg.opcode {
        XDG_TOPLEVEL_DESTROY => {
            let _ = blossom.destroy_toplevel(obj_id);
            client.destroy(obj_id);
        }
        XDG_TOPLEVEL_SET_PARENT => {
            let parent = read_u32(&msg.data, 0);
            let _ = blossom.set_parent(obj_id, parent.filter(|&p| p != 0));
        }
        XDG_TOPLEVEL_SET_TITLE => {
            if let Some((s, _)) = read_string(&msg.data, 0) {
                let title = String::from_utf8_lossy(s).into_owned();
                let _ = blossom.set_title(obj_id, title);
            }
        }
        XDG_TOPLEVEL_SET_APP_ID => {
            if let Some((s, _)) = read_string(&msg.data, 0) {
                let app_id = String::from_utf8_lossy(s).into_owned();
                let _ = blossom.set_app_id(obj_id, app_id);
            }
        }
        XDG_TOPLEVEL_SHOW_WINDOW_MENU => {
            let _ = blossom.show_window_menu(obj_id);
        }
        XDG_TOPLEVEL_MOVE => {
            let _ = blossom.request_move(obj_id);
        }
        XDG_TOPLEVEL_RESIZE => {
            let _ = blossom.request_resize(obj_id);
        }
        XDG_TOPLEVEL_SET_MAX_SIZE => {
            let w = read_i32(&msg.data, 0).unwrap_or(0);
            let h = read_i32(&msg.data, 4).unwrap_or(0);
            let _ = blossom.set_max_size(obj_id, blossom::Size { w, h });
        }
        XDG_TOPLEVEL_SET_MIN_SIZE => {
            let w = read_i32(&msg.data, 0).unwrap_or(0);
            let h = read_i32(&msg.data, 4).unwrap_or(0);
            let _ = blossom.set_min_size(obj_id, blossom::Size { w, h });
        }
        XDG_TOPLEVEL_SET_MAXIMIZED => {
            let _ = blossom.set_maximized(obj_id);
        }
        XDG_TOPLEVEL_UNSET_MAXIMIZED => {
            let _ = blossom.unset_maximized(obj_id);
        }
        XDG_TOPLEVEL_SET_FULLSCREEN => {
            let _ = blossom.set_fullscreen(obj_id);
        }
        XDG_TOPLEVEL_UNSET_FULLSCREEN => {
            let _ = blossom.unset_fullscreen(obj_id);
        }
        XDG_TOPLEVEL_SET_MINIMIZED => {
            let _ = blossom.set_minimized(obj_id);
        }
        _ => {}
    }
    vec![]
}

// ── Blossom command translation ───────────────────────────────────────────────

/// Execute blossom commands by sending Wayland events back to the client.
///
/// Commands that produce outgoing IPC to the main thread are written to
/// `cmd_write` directly (currently none — all blossom commands produce
/// outgoing Wayland events).
pub fn send_blossom_commands(
    client: &mut WaylandClient,
    cmds: &[BlossomCommand],
    _cmd_write: u32,
) {
    use crate::wayland::wire::encode_array;

    for cmd in cmds {
        match cmd {
            BlossomCommand::SendXdgToplevelConfigure {
                xdg_toplevel,
                width,
                height,
                states,
                ..
            } => {
                // xdg_toplevel.configure(width: int, height: int, states: array)
                // opcode 0
                let state_bytes: Vec<u8> = states
                    .iter()
                    .flat_map(|s| {
                        let v: u32 = xdg_state_atom_value(s);
                        v.to_ne_bytes().to_vec()
                    })
                    .collect();
                let mut payload = alloc::vec::Vec::new();
                payload.extend_from_slice(&(*width as i32).to_ne_bytes());
                payload.extend_from_slice(&(*height as i32).to_ne_bytes());
                payload.extend_from_slice(&encode_array(&state_bytes));
                client.send(*xdg_toplevel, 0, &payload);
            }
            BlossomCommand::SendXdgSurfaceConfigure { xdg_surface, serial, .. } => {
                // xdg_surface.configure(serial: uint)
                // opcode 0
                client.send(*xdg_surface, 0, &serial.to_ne_bytes());
            }
            BlossomCommand::MarkSurfaceReadyForMapping { .. } => {
                // No outgoing Wayland event needed; the compositor will map
                // the surface based on the commit IPC command.
            }
            BlossomCommand::CloseToplevel { toplevel } => {
                // xdg_toplevel.close()
                // opcode 1 (no payload)
                client.send(*toplevel, 1, &[]);
            }
            BlossomCommand::SendPing { wm_base, serial, .. } => {
                // xdg_wm_base.ping(serial: uint)
                // opcode 0
                client.send(*wm_base, 0, &serial.to_ne_bytes());
            }
        }
    }
}

/// Wayland numeric values for xdg_toplevel state atoms.
fn xdg_state_atom_value(atom: &blossom::XdgToplevelStateAtom) -> u32 {
    use blossom::XdgToplevelStateAtom::*;
    match atom {
        Maximized => 1,
        Fullscreen => 2,
        Resizing => 3,
        Activated => 4,
        TiledLeft => 5,
        TiledRight => 6,
        TiledTop => 7,
        TiledBottom => 8,
    }
}
