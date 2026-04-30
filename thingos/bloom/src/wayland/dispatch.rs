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
//! | 4    | wl_seat          | 5       |
//! | 5    | wl_output        | 2       |
//! | 6    | wl_subcompositor | 1       |
//! | 7    | wl_data_device_manager   | 3       |

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use blossom::{BlossomCommand, BlossomError};
use stem::{debug as blossom_debug, warn as blossom_warn};

use crate::wayland::client::{ObjectEntry, WaylandClient};
use crate::wayland::ipc;
use crate::wayland::wire::{WireMsg, read_i32, read_string, read_u32};

// ── Global registry constants ────────────────────────────────────────────────

pub const GLOBAL_WL_COMPOSITOR: u32 = 1;
pub const GLOBAL_WL_SHM: u32 = 2;
pub const GLOBAL_XDG_WM_BASE: u32 = 3;
pub const GLOBAL_WL_SEAT: u32 = 4;
pub const GLOBAL_WL_OUTPUT: u32 = 5;
pub const GLOBAL_WL_SUBCOMPOSITOR: u32 = 6;
pub const GLOBAL_WL_DATA_DEVICE_MANAGER: u32 = 7;

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
    output: &crate::display::OutputInfo,
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
        ObjKind::Registry => dispatch_registry(msg, client, output),
        ObjKind::Compositor => dispatch_compositor(msg, client, next_surface_key, cmd_write),
        ObjKind::Shm => dispatch_shm(msg, client),
        ObjKind::ShmPool => dispatch_shm_pool(msg, client, obj_id),
        ObjKind::Buffer => dispatch_buffer(msg, client, obj_id),
        ObjKind::Surface => dispatch_surface(msg, client, obj_id, blossom, cmd_write),
        ObjKind::Callback => vec![],
        ObjKind::XdgWmBase => dispatch_xdg_wm_base(msg, client, obj_id, blossom),
        ObjKind::XdgPositioner => vec![],
        ObjKind::XdgSurface => dispatch_xdg_surface(msg, client, obj_id, blossom, cmd_write),
        ObjKind::XdgToplevel => dispatch_xdg_toplevel(msg, client, obj_id, blossom, cmd_write),
        ObjKind::XdgPopup => dispatch_xdg_popup(msg, client, obj_id, blossom),
        ObjKind::Seat => dispatch_seat(msg, client, obj_id),
        ObjKind::Pointer => dispatch_pointer(msg, client, obj_id),
        ObjKind::Keyboard => dispatch_keyboard(msg, client, obj_id),
        ObjKind::Output => dispatch_output(msg, client, obj_id),
        ObjKind::Subcompositor => dispatch_subcompositor(msg, client, obj_id),
        ObjKind::Subsurface => dispatch_subsurface(msg, client, obj_id, cmd_write),
        ObjKind::DataDeviceManager => dispatch_data_device_manager(msg, client),
        ObjKind::DataSource => dispatch_data_source(msg, client, obj_id),
        ObjKind::DataDevice => dispatch_data_device(msg, client, obj_id),
        ObjKind::DataOffer => dispatch_data_offer(msg, client, obj_id),
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
    XdgPopup,
    Seat,
    Pointer,
    Keyboard,
    Output,
    Subcompositor,
    Subsurface,
    DataDeviceManager,
    DataSource,
    DataDevice,
    DataOffer,
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
        ObjectEntry::XdgPopup { .. } => ObjKind::XdgPopup,
        ObjectEntry::Seat => ObjKind::Seat,
        ObjectEntry::Pointer => ObjKind::Pointer,
        ObjectEntry::Keyboard => ObjKind::Keyboard,
        ObjectEntry::Output => ObjKind::Output,
        ObjectEntry::Subcompositor => ObjKind::Subcompositor,
        ObjectEntry::Subsurface { .. } => ObjKind::Subsurface,
        ObjectEntry::DataDeviceManager => ObjKind::DataDeviceManager,
        ObjectEntry::DataSource { .. } => ObjKind::DataSource,
        ObjectEntry::DataDevice { .. } => ObjKind::DataDevice,
        ObjectEntry::DataOffer => ObjKind::DataOffer,
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
    use crate::wayland::wire::encode_string;
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
                (GLOBAL_WL_SEAT, "wl_seat", 5u32),
                (GLOBAL_WL_OUTPUT, "wl_output", 2u32),
                (GLOBAL_WL_SUBCOMPOSITOR, "wl_subcompositor", 1u32),
                (GLOBAL_WL_DATA_DEVICE_MANAGER, "wl_data_device_manager", 3u32),
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

fn dispatch_registry(msg: &WireMsg, client: &mut WaylandClient, output: &crate::display::OutputInfo) -> Vec<Vec<u8>> {
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
        GLOBAL_WL_SEAT => {
            client.insert(new_id, ObjectEntry::Seat);
            send_seat_capabilities(client, new_id);
        }
        GLOBAL_WL_OUTPUT => {
            client.insert(new_id, ObjectEntry::Output);
            send_output_events(client, new_id, output);
        }
        GLOBAL_WL_SUBCOMPOSITOR => {
            client.insert(new_id, ObjectEntry::Subcompositor);
        GLOBAL_WL_DATA_DEVICE_MANAGER => {
            client.insert(new_id, ObjectEntry::DataDeviceManager);
        }
        _ => {
            client.send_protocol_error(new_id, 0, "unknown global");
        }
    }
    vec![]
}

// ── wl_seat / wl_pointer / wl_keyboard ──────────────────────────────────────

const WL_SEAT_GET_POINTER: u16 = 0;
const WL_SEAT_GET_KEYBOARD: u16 = 1;
const WL_SEAT_RELEASE: u16 = 3;
const WL_POINTER_RELEASE: u16 = 1;
const WL_KEYBOARD_RELEASE: u16 = 1;

const WL_SEAT_CAPABILITY_POINTER: u32 = 1;
const WL_SEAT_CAPABILITY_KEYBOARD: u32 = 2;

fn send_seat_capabilities(client: &WaylandClient, seat_obj: u32) {
    let caps = WL_SEAT_CAPABILITY_POINTER | WL_SEAT_CAPABILITY_KEYBOARD;
    client.send(seat_obj, 0, &caps.to_ne_bytes());
    client.send(seat_obj, 1, &crate::wayland::wire::encode_string("seat0"));
}

fn dispatch_seat(msg: &WireMsg, client: &mut WaylandClient, obj_id: u32) -> Vec<Vec<u8>> {
    match msg.opcode {
        WL_SEAT_GET_POINTER => {
            if let Some(new_id) = read_u32(&msg.data, 0) {
                client.insert(new_id, ObjectEntry::Pointer);
            }
        }
        WL_SEAT_GET_KEYBOARD => {
            if let Some(new_id) = read_u32(&msg.data, 0) {
                client.insert(new_id, ObjectEntry::Keyboard);
                send_keyboard_keymap(client, new_id);
                send_keyboard_repeat_info(client, new_id);
            }
        }
        WL_SEAT_RELEASE => {
            client.destroy(obj_id);
        }
        _ => {}
    }
    vec![]
}

fn dispatch_pointer(msg: &WireMsg, client: &mut WaylandClient, obj_id: u32) -> Vec<Vec<u8>> {
    if msg.opcode == WL_POINTER_RELEASE {
        client.destroy(obj_id);
    }
    vec![]
}

fn dispatch_keyboard(msg: &WireMsg, client: &mut WaylandClient, obj_id: u32) -> Vec<Vec<u8>> {
    if msg.opcode == WL_KEYBOARD_RELEASE {
        client.destroy(obj_id);
    }
    vec![]
}

fn send_keyboard_keymap(client: &WaylandClient, keyboard_obj: u32) {
    // wl_keyboard.keymap(format=no_keymap, fd, size). With no_keymap the fd is
    // ignored by clients; send no ancillary handle and a zero size.
    let mut payload = Vec::new();
    payload.extend_from_slice(&0u32.to_ne_bytes());
    payload.extend_from_slice(&0u32.to_ne_bytes());
    client.send_with_fds(keyboard_obj, 0, &payload, &[]);
}

fn send_keyboard_repeat_info(client: &WaylandClient, keyboard_obj: u32) {
    let mut payload = Vec::new();
    payload.extend_from_slice(&25i32.to_ne_bytes());
    payload.extend_from_slice(&600i32.to_ne_bytes());
    client.send(keyboard_obj, 5, &payload);
}

// ── wl_output ────────────────────────────────────────────────────────────────

/// `wl_output.release` request opcode (version 3+).
const WL_OUTPUT_RELEASE: u16 = 0;

fn dispatch_output(msg: &WireMsg, client: &mut WaylandClient, obj_id: u32) -> Vec<Vec<u8>> {
    // wl_output.release is a version-3 destructor; accept it gracefully even
    // though we only advertise version 2 (some clients send it anyway).
    if msg.opcode == WL_OUTPUT_RELEASE {
        client.destroy(obj_id);
    }
    vec![]
}

/// Send the initial burst of `wl_output` events to a newly-bound output object.
///
/// Sends (in order): `geometry`, `mode`, `scale` (v2), `done` (v2).
fn send_output_events(client: &WaylandClient, output_obj: u32, info: &crate::display::OutputInfo) {
    use crate::wayland::wire::encode_string;

    // wl_output.geometry (opcode 0)
    // x, y, physical_width_mm, physical_height_mm, subpixel, make, model, transform
    let mut geom = Vec::new();
    geom.extend_from_slice(&0i32.to_ne_bytes()); // x = 0
    geom.extend_from_slice(&0i32.to_ne_bytes()); // y = 0
    geom.extend_from_slice(&0i32.to_ne_bytes()); // physical_width_mm = 0 (unknown)
    geom.extend_from_slice(&0i32.to_ne_bytes()); // physical_height_mm = 0 (unknown)
    geom.extend_from_slice(&0i32.to_ne_bytes()); // subpixel = WL_OUTPUT_SUBPIXEL_UNKNOWN
    geom.extend_from_slice(&encode_string("ThingOS"));
    geom.extend_from_slice(&encode_string("Virtual"));
    geom.extend_from_slice(&0i32.to_ne_bytes()); // transform = WL_OUTPUT_TRANSFORM_NORMAL
    client.send(output_obj, 0, &geom);

    // wl_output.mode (opcode 1)
    // flags, width, height, refresh_mHz
    let mut mode = Vec::new();
    mode.extend_from_slice(&3u32.to_ne_bytes()); // WL_OUTPUT_MODE_CURRENT(1) | WL_OUTPUT_MODE_PREFERRED(2)
    mode.extend_from_slice(&(info.width as i32).to_ne_bytes());
    mode.extend_from_slice(&(info.height as i32).to_ne_bytes());
    mode.extend_from_slice(&(info.refresh_mhz as i32).to_ne_bytes());
    client.send(output_obj, 1, &mode);

    // wl_output.scale (opcode 3, version 2+)
    // factor: int = 1 (no HiDPI scaling)
    client.send(output_obj, 3, &1i32.to_ne_bytes());

    // wl_output.done (opcode 2, version 2+)
    // Signals that all output properties have been sent.
    client.send(output_obj, 2, &[]);
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
                    subsurface_obj: None,
                    subsurface_children: alloc::vec::Vec::new(),
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
    // The current Thing-OS socket path transfers `fd` through sendmsg/recvmsg
    // ancillary handles.  Older demos sent the handle inline; keep accepting
    // that layout so stale clients fail gracefully instead of desynchronizing.
    let new_id = match read_u32(&msg.data, 0) {
        Some(id) => id,
        None => return vec![],
    };
    let (handle, size) = if let Some(fd) = client.pending_fds.pop_front() {
        (fd, read_u32(&msg.data, 4).unwrap_or(0))
    } else {
        (read_u32(&msg.data, 4).unwrap_or(0), read_u32(&msg.data, 8).unwrap_or(0))
    };
    client.insert(new_id, ObjectEntry::ShmPool { handle, size });
    vec![]
}

// ── wl_shm_pool ───────────────────────────────────────────────────────────────

const WL_SHM_POOL_CREATE_BUFFER: u16 = 0;
const WL_SHM_POOL_DESTROY: u16 = 1;
const WL_SHM_POOL_RESIZE: u16 = 2;

fn dispatch_shm_pool(msg: &WireMsg, client: &mut WaylandClient, obj_id: u32) -> Vec<Vec<u8>> {
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

fn dispatch_buffer(msg: &WireMsg, client: &mut WaylandClient, obj_id: u32) -> Vec<Vec<u8>> {
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
            let x = read_i32(&msg.data, 0).unwrap_or(0);
            let y = read_i32(&msg.data, 4).unwrap_or(0);
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
                blossom_debug!(
                    "wayland-server: wl_surface obj={} registered frame callback cb={}",
                    obj_id,
                    cb_id
                );
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
                ..
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
                blossom_warn!(
                    "wayland-server: xdg_surface error: buffer committed before ack_configure (obj={})",
                    xdg_id
                );
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
                    bloom_surface_id,
                    key,
                    handle,
                    width,
                    height,
                    stride,
                    format,
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
    if let Some(ObjectEntry::Surface { pending_buffer, pending_damage, pending_frame_cb, .. }) =
        client.objects.get_mut(&wl_surface_obj)
    {
        *pending_buffer = None;
        *pending_damage = None;
        *pending_frame_cb = None;
    }

    // Atomically apply pending state of any synchronized subsurface children.
    // (`wl_subsurface` spec: in synchronized mode the subsurface state is
    // cached and applied as part of the parent's commit.)
    flush_synchronized_subsurfaces(wl_surface_obj, client, &mut out);

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
                    blossom_debug!(
                        "wayland-server: xdg_surface obj={} created for surface={}",
                        new_id,
                        bloom_surface_id
                    );
                    client.insert(new_id, ObjectEntry::XdgSurface { bloom_surface_id });
                    // Link the wl_surface back to this xdg_surface.
                    if let Some(ObjectEntry::Surface { xdg_surface_obj, .. }) =
                        client.objects.get_mut(&wl_surface_obj)
                    {
                        *xdg_surface_obj = Some(new_id);
                    }
                }
                Err(BlossomError::XdgSurfaceAlreadyExists { .. }) => {
                    blossom_warn!(
                        "wayland-server: xdg_surface error: role conflict for surface={}",
                        bloom_surface_id
                    );
                    client.send_protocol_error(obj_id, 0, "wl_surface already has xdg_surface");
                }
                Err(_) => {}
            }
        }
        XDG_WM_BASE_PONG => {
            let serial = read_u32(&msg.data, 0).unwrap_or(0);
            blossom_debug!("wayland-server: xdg_wm_base.pong serial={} received", serial);
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
    let _bloom_surface_id = match client.objects.get(&obj_id) {
        Some(ObjectEntry::XdgSurface { bloom_surface_id }) => *bloom_surface_id,
        _ => return vec![],
    };
    let client_id = client.fd;

    match msg.opcode {
        XDG_SURFACE_DESTROY => {
            blossom_debug!("wayland-server: xdg_surface obj={} destroyed", obj_id);
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
                    stem::info!(
                        "wayland-server: xdg_toplevel obj={} assigned to xdg_surface={}",
                        new_id,
                        obj_id
                    );
                    client.insert(new_id, ObjectEntry::XdgToplevel { xdg_surface_obj: obj_id });
                    send_blossom_commands(client, &cmds, cmd_write);
                }
                Err(BlossomError::XdgSurfaceAlreadyHasRole { .. }) => {
                    blossom_warn!(
                        "wayland-server: xdg_surface error: surface={} already has a role",
                        obj_id
                    );
                    client.send_protocol_error(obj_id, 4, "xdg_surface already has a role");
                }
                Err(_) => {}
            }
        }
        XDG_SURFACE_GET_POPUP => {
            let new_id = match read_u32(&msg.data, 0) {
                Some(id) => id,
                None => return vec![],
            };
            let parent = read_u32(&msg.data, 4).filter(|id| *id != 0);
            let positioner = match read_u32(&msg.data, 8) {
                Some(id) => id,
                None => return vec![],
            };
            match blossom.get_popup(client_id, obj_id, new_id, parent, positioner) {
                Ok(cmds) => {
                    stem::info!(
                        "wayland-server: xdg_popup obj={} assigned to xdg_surface={}",
                        new_id,
                        obj_id
                    );
                    client.insert(new_id, ObjectEntry::XdgPopup { xdg_surface_obj: obj_id });
                    send_blossom_commands(client, &cmds, cmd_write);
                }
                Err(BlossomError::XdgSurfaceAlreadyHasRole { .. }) => {
                    blossom_warn!(
                        "wayland-server: xdg_surface error: surface={} already has a role",
                        obj_id
                    );
                    client.send_protocol_error(obj_id, 4, "xdg_surface already has a role");
                }
                Err(_) => {}
            }
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
                Ok(_) => {
                    blossom_debug!(
                        "wayland-server: xdg_surface.ack_configure serial={} accepted",
                        serial
                    );
                }
                Err(BlossomError::UnknownSerial { .. }) => {
                    blossom_warn!(
                        "wayland-server: xdg_surface error: invalid serial {} in ack_configure",
                        serial
                    );
                    client.send_protocol_error(obj_id, 3, "invalid serial in ack_configure");
                }
                Err(_) => {}
            }
        }
        _ => {}
    }
    vec![]
}

// ── xdg_popup ────────────────────────────────────────────────────────────────

const XDG_POPUP_DESTROY: u16 = 0;

fn dispatch_xdg_popup(
    msg: &WireMsg,
    client: &mut WaylandClient,
    obj_id: u32,
    blossom: &mut blossom::Blossom,
) -> Vec<Vec<u8>> {
    if msg.opcode == XDG_POPUP_DESTROY {
        blossom_debug!("wayland-server: xdg_popup obj={} destroyed", obj_id);
        let _ = blossom.destroy_popup(obj_id);
        client.destroy(obj_id);
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
    cmd_write: u32,
) -> Vec<Vec<u8>> {
    match msg.opcode {
        XDG_TOPLEVEL_DESTROY => {
            blossom_debug!("wayland-server: xdg_toplevel obj={} destroyed", obj_id);
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
                blossom_debug!("wayland-server: xdg_toplevel obj={} title=\"{}\"", obj_id, title);
                let _ = blossom.set_title(obj_id, title);
                if let Some(bloom_surface_id) = toplevel_bloom_surface(client, obj_id) {
                    let msg = ipc::encode_set_title(bloom_surface_id, &String::from_utf8_lossy(s));
                    let _ = stem::syscall::port_send_all(cmd_write, &msg);
                }
            }
        }
        XDG_TOPLEVEL_SET_APP_ID => {
            if let Some((s, _)) = read_string(&msg.data, 0) {
                let app_id = String::from_utf8_lossy(s).into_owned();
                blossom_debug!("wayland-server: xdg_toplevel obj={} app_id=\"{}\"", obj_id, app_id);
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

fn toplevel_bloom_surface(client: &WaylandClient, toplevel_obj: u32) -> Option<u32> {
    let xdg_surface_obj = match client.objects.get(&toplevel_obj)? {
        ObjectEntry::XdgToplevel { xdg_surface_obj } => *xdg_surface_obj,
        _ => return None,
    };
    match client.objects.get(&xdg_surface_obj)? {
        ObjectEntry::XdgSurface { bloom_surface_id } => Some(*bloom_surface_id),
        _ => None,
    }
}

// ── wl_subcompositor ─────────────────────────────────────────────────────────

const WL_SUBCOMPOSITOR_DESTROY: u16 = 0;
const WL_SUBCOMPOSITOR_GET_SUBSURFACE: u16 = 1;

/// Error code values from `wl_subcompositor.error`.
const WL_SUBCOMPOSITOR_ERROR_BAD_SURFACE: u32 = 0;

fn dispatch_subcompositor(
    msg: &WireMsg,
    client: &mut WaylandClient,
    obj_id: u32,
) -> Vec<Vec<u8>> {
    match msg.opcode {
        WL_SUBCOMPOSITOR_DESTROY => {
            client.destroy(obj_id);
        }
        WL_SUBCOMPOSITOR_GET_SUBSURFACE => {
            // get_subsurface(id: new_id<wl_subsurface>, surface: object<wl_surface>,
            //                parent:  object<wl_surface>)
            let new_id = match read_u32(&msg.data, 0) {
                Some(id) => id,
                None => return vec![],
            };
            let surface_obj = read_u32(&msg.data, 4).unwrap_or(0);
            let parent_obj = read_u32(&msg.data, 8).unwrap_or(0);

            // The surface must exist, must not already be a subsurface, and
            // must not be the parent of itself.
            if surface_obj == 0 || parent_obj == 0 || surface_obj == parent_obj {
                client.send_protocol_error(
                    obj_id,
                    WL_SUBCOMPOSITOR_ERROR_BAD_SURFACE,
                    "invalid surface for get_subsurface",
                );
                return vec![];
            }

            // Validate child is a wl_surface and not yet a subsurface and not
            // already an xdg_surface (Wayland spec: a wl_surface may carry at
            // most one role).
            let (child_is_surface, child_already_sub, child_already_xdg) =
                match client.objects.get(&surface_obj) {
                    Some(ObjectEntry::Surface { subsurface_obj, xdg_surface_obj, .. }) => {
                        (true, subsurface_obj.is_some(), xdg_surface_obj.is_some())
                    }
                    _ => (false, false, false),
                };
            let parent_is_surface =
                matches!(client.objects.get(&parent_obj), Some(ObjectEntry::Surface { .. }));
            if !child_is_surface
                || !parent_is_surface
                || child_already_sub
                || child_already_xdg
            {
                client.send_protocol_error(
                    obj_id,
                    WL_SUBCOMPOSITOR_ERROR_BAD_SURFACE,
                    "surface already has the subsurface role or is invalid",
                );
                return vec![];
            }

            // Register the subsurface object.  Spec defaults: position (0,0),
            // synchronized mode.
            client.insert(
                new_id,
                ObjectEntry::Subsurface {
                    child_wl_surface: surface_obj,
                    parent_wl_surface: parent_obj,
                    x: 0,
                    y: 0,
                    sync: true,
                    pending_position: None,
                    pending_place: None,
                },
            );
            // Link child surface back to its subsurface object.
            if let Some(ObjectEntry::Surface { subsurface_obj, .. }) =
                client.objects.get_mut(&surface_obj)
            {
                *subsurface_obj = Some(new_id);
            }
            // Append to the parent's children stack at the top.
            if let Some(ObjectEntry::Surface { subsurface_children, .. }) =
                client.objects.get_mut(&parent_obj)
            {
                subsurface_children.push(surface_obj);
            }
        }
        _ => {}
    }
    vec![]
}

// ── wl_subsurface ────────────────────────────────────────────────────────────

const WL_SUBSURFACE_DESTROY: u16 = 0;
const WL_SUBSURFACE_SET_POSITION: u16 = 1;
const WL_SUBSURFACE_PLACE_ABOVE: u16 = 2;
const WL_SUBSURFACE_PLACE_BELOW: u16 = 3;
const WL_SUBSURFACE_SET_SYNC: u16 = 4;
const WL_SUBSURFACE_SET_DESYNC: u16 = 5;

/// Error code values from `wl_subsurface.error`.
const WL_SUBSURFACE_ERROR_BAD_SURFACE: u32 = 0;

fn dispatch_subsurface(
    msg: &WireMsg,
    client: &mut WaylandClient,
    obj_id: u32,
    cmd_write: u32,
) -> Vec<Vec<u8>> {
    let mut ipc_cmds: Vec<Vec<u8>> = vec![];

    match msg.opcode {
        WL_SUBSURFACE_DESTROY => {
            destroy_subsurface(obj_id, client, &mut ipc_cmds, cmd_write);
        }
        WL_SUBSURFACE_SET_POSITION => {
            let x = read_i32(&msg.data, 0).unwrap_or(0);
            let y = read_i32(&msg.data, 4).unwrap_or(0);
            let sync = subsurface_is_sync(client, obj_id);
            if let Some(ObjectEntry::Subsurface { pending_position, .. }) =
                client.objects.get_mut(&obj_id)
            {
                *pending_position = Some((x, y));
            }
            if !sync {
                // Desync: apply immediately.
                if let Some((x, y)) = take_pending_position(client, obj_id) {
                    apply_subsurface_position(client, obj_id, x, y);
                    if let Some(cmd) = subsurface_state_command(client, obj_id) {
                        let _ = stem::syscall::port_send_all(cmd_write, &cmd);
                    }
                }
            }
        }
        WL_SUBSURFACE_PLACE_ABOVE | WL_SUBSURFACE_PLACE_BELOW => {
            let sibling = read_u32(&msg.data, 0).unwrap_or(0);
            let above = msg.opcode == WL_SUBSURFACE_PLACE_ABOVE;
            let sync = subsurface_is_sync(client, obj_id);
            if let Some(ObjectEntry::Subsurface { pending_place, .. }) =
                client.objects.get_mut(&obj_id)
            {
                *pending_place = Some((sibling, above));
            }
            if !sync {
                if let Some((sibling, above)) = take_pending_place(client, obj_id) {
                    if !apply_subsurface_place(client, obj_id, sibling, above) {
                        client.send_protocol_error(
                            obj_id,
                            WL_SUBSURFACE_ERROR_BAD_SURFACE,
                            "place_above/below sibling not a sibling subsurface",
                        );
                    } else if let Some(cmd) = subsurface_state_command(client, obj_id) {
                        let _ = stem::syscall::port_send_all(cmd_write, &cmd);
                    }
                }
            }
        }
        WL_SUBSURFACE_SET_SYNC => {
            if let Some(ObjectEntry::Subsurface { sync, .. }) = client.objects.get_mut(&obj_id) {
                *sync = true;
            }
        }
        WL_SUBSURFACE_SET_DESYNC => {
            if let Some(ObjectEntry::Subsurface { sync, .. }) = client.objects.get_mut(&obj_id) {
                *sync = false;
            }
            // Flush any pending state immediately.
            if let Some((x, y)) = take_pending_position(client, obj_id) {
                apply_subsurface_position(client, obj_id, x, y);
            }
            if let Some((sibling, above)) = take_pending_place(client, obj_id) {
                if !apply_subsurface_place(client, obj_id, sibling, above) {
                    client.send_protocol_error(
                        obj_id,
                        WL_SUBSURFACE_ERROR_BAD_SURFACE,
                        "place_above/below sibling not a sibling subsurface",
                    );
                }
            }
            if let Some(cmd) = subsurface_state_command(client, obj_id) {
                let _ = stem::syscall::port_send_all(cmd_write, &cmd);
            }
        }
        _ => {}
    }

    ipc_cmds
}

/// Tear down a subsurface, removing it from the parent's child list and
/// detaching the role from the child wl_surface.  An IPC notification is
/// emitted so the scene can drop the parent/offset relationship.
fn destroy_subsurface(
    obj_id: u32,
    client: &mut WaylandClient,
    _ipc_cmds: &mut Vec<Vec<u8>>,
    cmd_write: u32,
) {
    let (child_wl_surface, parent_wl_surface) = match client.objects.get(&obj_id) {
        Some(ObjectEntry::Subsurface { child_wl_surface, parent_wl_surface, .. }) => {
            (*child_wl_surface, *parent_wl_surface)
        }
        _ => {
            client.destroy(obj_id);
            return;
        }
    };
    let child_bloom = client.bloom_surface_id(child_wl_surface).unwrap_or(0);

    // Detach role from child.
    if let Some(ObjectEntry::Surface { subsurface_obj, .. }) =
        client.objects.get_mut(&child_wl_surface)
    {
        if *subsurface_obj == Some(obj_id) {
            *subsurface_obj = None;
        }
    }
    // Remove from parent's children stack.
    if let Some(ObjectEntry::Surface { subsurface_children, .. }) =
        client.objects.get_mut(&parent_wl_surface)
    {
        subsurface_children.retain(|&id| id != child_wl_surface);
    }
    client.destroy(obj_id);

    if child_bloom != 0 {
        let cmd = ipc::encode_set_subsurface(child_bloom, 0, 0, 0, 0);
        let _ = stem::syscall::port_send_all(cmd_write, &cmd);
    }
}

fn subsurface_is_sync(client: &WaylandClient, obj_id: u32) -> bool {
    match client.objects.get(&obj_id) {
        Some(ObjectEntry::Subsurface { sync, .. }) => *sync,
        _ => true,
    }
}

fn take_pending_position(client: &mut WaylandClient, obj_id: u32) -> Option<(i32, i32)> {
    match client.objects.get_mut(&obj_id) {
        Some(ObjectEntry::Subsurface { pending_position, .. }) => pending_position.take(),
        _ => None,
    }
}

fn take_pending_place(client: &mut WaylandClient, obj_id: u32) -> Option<(u32, bool)> {
    match client.objects.get_mut(&obj_id) {
        Some(ObjectEntry::Subsurface { pending_place, .. }) => pending_place.take(),
        _ => None,
    }
}

fn apply_subsurface_position(client: &mut WaylandClient, obj_id: u32, new_x: i32, new_y: i32) {
    if let Some(ObjectEntry::Subsurface { x, y, .. }) = client.objects.get_mut(&obj_id) {
        *x = new_x;
        *y = new_y;
    }
}

/// Reorder the parent's `subsurface_children` list per `place_above` / `place_below`.
///
/// Returns `false` when the requested sibling is not actually a sibling of the
/// child being moved (Wayland spec calls for a `bad_surface` protocol error).
/// `sibling == 0` is interpreted as "relative to the parent itself" (place at
/// top or bottom of the children stack).
fn apply_subsurface_place(
    client: &mut WaylandClient,
    obj_id: u32,
    sibling: u32,
    above: bool,
) -> bool {
    let (child, parent) = match client.objects.get(&obj_id) {
        Some(ObjectEntry::Subsurface { child_wl_surface, parent_wl_surface, .. }) => {
            (*child_wl_surface, *parent_wl_surface)
        }
        _ => return false,
    };
    let Some(ObjectEntry::Surface { subsurface_children, .. }) =
        client.objects.get_mut(&parent)
    else {
        return false;
    };

    let Some(child_idx) = subsurface_children.iter().position(|&c| c == child) else {
        return false;
    };
    subsurface_children.remove(child_idx);

    if sibling == 0 || sibling == parent {
        // Relative to parent: above => top, below => bottom.
        if above {
            subsurface_children.push(child);
        } else {
            subsurface_children.insert(0, child);
        }
        return true;
    }

    let Some(sibling_idx) = subsurface_children.iter().position(|&c| c == sibling) else {
        // Sibling not part of this parent's children: protocol error.  Restore
        // child to the top to keep state consistent.
        subsurface_children.push(child);
        return false;
    };
    let insert_at = if above { sibling_idx + 1 } else { sibling_idx };
    subsurface_children.insert(insert_at, child);
    true
}

/// Build a `WCMD_SET_SUBSURFACE` for the given wl_subsurface object reflecting
/// its current parent/position/stacking state.
fn subsurface_state_command(client: &WaylandClient, obj_id: u32) -> Option<[u8; 24]> {
    let (child_wl_surface, parent_wl_surface, x, y) = match client.objects.get(&obj_id)? {
        ObjectEntry::Subsurface { child_wl_surface, parent_wl_surface, x, y, .. } => {
            (*child_wl_surface, *parent_wl_surface, *x, *y)
        }
        _ => return None,
    };
    let child_bloom = client.bloom_surface_id(child_wl_surface)?;
    let parent_bloom = client.bloom_surface_id(parent_wl_surface)?;
    let z_above = subsurface_z_above(client, parent_wl_surface, child_wl_surface);
    Some(ipc::encode_set_subsurface(child_bloom, parent_bloom, x, y, z_above))
}

/// Return the stacking offset of `child_wl_surface` relative to its parent.
///
/// Children are numbered 1..=N from bottom-to-top, so the first child is
/// `+1` (just above the parent) and the topmost child is `+N`.  Returns `0`
/// when the child is not currently in the parent's stack (caller filters).
fn subsurface_z_above(
    client: &WaylandClient,
    parent_wl_surface: u32,
    child_wl_surface: u32,
) -> i32 {
    let Some(ObjectEntry::Surface { subsurface_children, .. }) =
        client.objects.get(&parent_wl_surface)
    else {
        return 0;
    };
    match subsurface_children.iter().position(|&c| c == child_wl_surface) {
        Some(idx) => (idx as i32) + 1,
        None => 0,
    }
}

/// On parent `wl_surface.commit`, atomically apply pending state of every
/// synchronized subsurface child and emit IPC updates so the scene matches.
fn flush_synchronized_subsurfaces(
    parent_wl_surface: u32,
    client: &mut WaylandClient,
    out: &mut Vec<Vec<u8>>,
) {
    // Snapshot current children list (cloned to release the borrow).
    let children: Vec<u32> = match client.objects.get(&parent_wl_surface) {
        Some(ObjectEntry::Surface { subsurface_children, .. }) => subsurface_children.clone(),
        _ => return,
    };
    // Track which subsurface objects had their stacking applied so we can
    // re-emit z_above for siblings whose ordinal might have shifted.
    let mut any_place = false;
    for child_wl_surface in &children {
        let sub_obj = match client.objects.get(child_wl_surface) {
            Some(ObjectEntry::Surface { subsurface_obj: Some(s), .. }) => *s,
            _ => continue,
        };
        let sync = subsurface_is_sync(client, sub_obj);
        if !sync {
            continue;
        }
        if let Some((x, y)) = take_pending_position(client, sub_obj) {
            apply_subsurface_position(client, sub_obj, x, y);
        }
        if let Some((sibling, above)) = take_pending_place(client, sub_obj) {
            let _ = apply_subsurface_place(client, sub_obj, sibling, above);
            any_place = true;
        }
    }
    // Re-emit current state for every synchronized child after the parent
    // commit so the scene picks up parent-relative offsets / stacking changes.
    let final_children: Vec<u32> = match client.objects.get(&parent_wl_surface) {
        Some(ObjectEntry::Surface { subsurface_children, .. }) => subsurface_children.clone(),
        _ => return,
    };
    for child_wl_surface in &final_children {
        let sub_obj = match client.objects.get(child_wl_surface) {
            Some(ObjectEntry::Surface { subsurface_obj: Some(s), .. }) => *s,
            _ => continue,
        };
        if !subsurface_is_sync(client, sub_obj) && !any_place {
            // Desync subsurfaces only need state push when stacking shifted.
            continue;
        }
        if let Some(cmd) = subsurface_state_command(client, sub_obj) {
            out.push(cmd.to_vec());
        }
    }
}

// ── Blossom command translation ───────────────────────────────────────────────

/// Execute blossom commands by sending Wayland events back to the client.
///
/// Commands that produce outgoing IPC to the main thread are written to
/// `cmd_write` directly.
pub fn send_blossom_commands(client: &mut WaylandClient, cmds: &[BlossomCommand], cmd_write: u32) {
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
                blossom_debug!(
                    "wayland-server: xdg_surface.configure serial={} sent to obj={}",
                    serial,
                    xdg_surface
                );
                client.send(*xdg_surface, 0, &serial.to_ne_bytes());
            }
            BlossomCommand::SendXdgPopupConfigure { xdg_popup, x, y, width, height, .. } => {
                // xdg_popup.configure(x: int, y: int, width: int, height: int)
                // opcode 0
                blossom_debug!(
                    "wayland-server: xdg_popup.configure obj={} x={} y={} width={} height={}",
                    xdg_popup,
                    x,
                    y,
                    width,
                    height
                );
                let mut payload = alloc::vec::Vec::new();
                payload.extend_from_slice(&x.to_ne_bytes());
                payload.extend_from_slice(&y.to_ne_bytes());
                payload.extend_from_slice(&width.to_ne_bytes());
                payload.extend_from_slice(&height.to_ne_bytes());
                client.send(*xdg_popup, 0, &payload);
            }
            BlossomCommand::MarkSurfaceReadyForMapping { surface } => {
                blossom_debug!("wayland-server: surface {} ready for mapping", surface);
                // No outgoing Wayland event needed; the compositor will map
                // the surface based on the commit IPC command.
            }
            BlossomCommand::SetToplevelChrome { surface, titlebar_height, frame_thickness } => {
                blossom_debug!(
                    "wayland-server: surface {} titlebar height {} frame {}",
                    surface,
                    titlebar_height,
                    frame_thickness
                );
                let msg = ipc::encode_set_chrome(*surface, *titlebar_height, *frame_thickness);
                let _ = stem::syscall::port_send_all(cmd_write, &msg);
            }
            BlossomCommand::CloseToplevel { toplevel } => {
                // xdg_toplevel.close()
                // opcode 1 (no payload)
                client.send(*toplevel, 1, &[]);
            }
            BlossomCommand::SendPing { wm_base, serial, .. } => {
                // xdg_wm_base.ping(serial: uint)
                // opcode 0
                blossom_debug!("wayland-server: xdg_wm_base.ping serial={} sent", serial);
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

// ── wl_data_device_manager ────────────────────────────────────────────────────

/// wl_data_device_manager request opcodes.
const WL_DATA_DEVICE_MANAGER_CREATE_DATA_SOURCE: u16 = 0;
const WL_DATA_DEVICE_MANAGER_GET_DATA_DEVICE: u16 = 1;

fn dispatch_data_device_manager(msg: &WireMsg, client: &mut WaylandClient) -> Vec<Vec<u8>> {
    match msg.opcode {
        WL_DATA_DEVICE_MANAGER_CREATE_DATA_SOURCE => {
            if let Some(new_id) = read_u32(&msg.data, 0) {
                client.insert(new_id, ObjectEntry::DataSource { mime_types: alloc::vec::Vec::new() });
                blossom_debug!("wayland-server: data_source obj={} created", new_id);
            }
        }
        WL_DATA_DEVICE_MANAGER_GET_DATA_DEVICE => {
            // get_data_device(new_id: new_id<wl_data_device>, seat: object<wl_seat>)
            let new_id = match read_u32(&msg.data, 0) {
                Some(id) => id,
                None => return vec![],
            };
            let seat_obj = read_u32(&msg.data, 4).unwrap_or(0);
            client.insert(new_id, ObjectEntry::DataDevice { seat_obj });
            client.data_device_obj = Some(new_id);
            blossom_debug!("wayland-server: data_device obj={} created for seat={}", new_id, seat_obj);
        }
        _ => {}
    }
    vec![]
}

// ── wl_data_source ────────────────────────────────────────────────────────────

/// wl_data_source request opcodes.
const WL_DATA_SOURCE_OFFER: u16 = 0;
const WL_DATA_SOURCE_DESTROY: u16 = 1;
const WL_DATA_SOURCE_SET_ACTIONS: u16 = 2;

fn dispatch_data_source(msg: &WireMsg, client: &mut WaylandClient, obj_id: u32) -> Vec<Vec<u8>> {
    match msg.opcode {
        WL_DATA_SOURCE_OFFER => {
            // offer(mime_type: string)
            if let Some((mime_bytes, _)) = read_string(&msg.data, 0) {
                let mime = String::from_utf8_lossy(mime_bytes).into_owned();
                if let Some(ObjectEntry::DataSource { mime_types }) =
                    client.objects.get_mut(&obj_id)
                {
                    mime_types.push(mime);
                }
            }
        }
        WL_DATA_SOURCE_DESTROY => {
            blossom_debug!("wayland-server: data_source obj={} destroyed", obj_id);
            client.destroy(obj_id);
        }
        WL_DATA_SOURCE_SET_ACTIONS => {
            // DnD actions — accepted as no-op for clipboard-only support.
        }
        _ => {}
    }
    vec![]
}

// ── wl_data_device ────────────────────────────────────────────────────────────

/// wl_data_device request opcodes.
const WL_DATA_DEVICE_START_DRAG: u16 = 0;
const WL_DATA_DEVICE_SET_SELECTION: u16 = 1;
const WL_DATA_DEVICE_RELEASE: u16 = 2;

fn dispatch_data_device(msg: &WireMsg, client: &mut WaylandClient, obj_id: u32) -> Vec<Vec<u8>> {
    match msg.opcode {
        WL_DATA_DEVICE_SET_SELECTION => {
            // set_selection(source: object<wl_data_source>|null, serial: uint)
            let source_obj = read_u32(&msg.data, 0).unwrap_or(0);
            let mime_types = if source_obj != 0 {
                match client.objects.get(&source_obj) {
                    Some(ObjectEntry::DataSource { mime_types }) => mime_types.clone(),
                    _ => alloc::vec::Vec::new(),
                }
            } else {
                alloc::vec::Vec::new()
            };
            blossom_debug!(
                "wayland-server: data_device.set_selection source={} mimes={}",
                source_obj,
                mime_types.len()
            );
            // Signal to the server to broadcast the new selection.
            client.pending_clipboard_set = Some((source_obj, mime_types));
        }
        WL_DATA_DEVICE_RELEASE => {
            blossom_debug!("wayland-server: data_device obj={} released", obj_id);
            client.destroy(obj_id);
            client.data_device_obj = None;
        }
        WL_DATA_DEVICE_START_DRAG => {
            // Drag-and-drop — not implemented; accepted as no-op.
            blossom_debug!("wayland-server: data_device.start_drag ignored (DnD not implemented)");
        }
        _ => {}
    }
    vec![]
}

// ── wl_data_offer ─────────────────────────────────────────────────────────────

/// wl_data_offer request opcodes.
const WL_DATA_OFFER_ACCEPT: u16 = 0;
const WL_DATA_OFFER_RECEIVE: u16 = 1;
const WL_DATA_OFFER_DESTROY: u16 = 2;
const WL_DATA_OFFER_FINISH: u16 = 3;
const WL_DATA_OFFER_SET_ACTIONS: u16 = 4;

fn dispatch_data_offer(msg: &WireMsg, client: &mut WaylandClient, obj_id: u32) -> Vec<Vec<u8>> {
    match msg.opcode {
        WL_DATA_OFFER_RECEIVE => {
            // receive(mime_type: string, fd: fd)
            // The write end of a pipe is sent as ancillary data by the client.
            let mime = match read_string(&msg.data, 0) {
                Some((bytes, _)) => String::from_utf8_lossy(bytes).into_owned(),
                None => return vec![],
            };
            let write_fd = match client.pending_fds.pop_front() {
                Some(fd) => fd,
                None => {
                    blossom_warn!("wayland-server: data_offer.receive: no fd received");
                    return vec![];
                }
            };
            blossom_debug!(
                "wayland-server: data_offer.receive obj={} mime=\"{}\" fd={}",
                obj_id,
                mime,
                write_fd
            );
            // Signal to the server to forward the fd to the clipboard source.
            client.pending_offer_receive = Some((mime, write_fd));
        }
        WL_DATA_OFFER_DESTROY => {
            blossom_debug!("wayland-server: data_offer obj={} destroyed", obj_id);
            client.destroy(obj_id);
        }
        // DnD-only requests: accepted as no-ops.
        WL_DATA_OFFER_ACCEPT | WL_DATA_OFFER_FINISH | WL_DATA_OFFER_SET_ACTIONS => {}
        _ => {}
    }
    vec![]
}
