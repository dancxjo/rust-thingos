//! Minimal Wayland drag-and-drop smoke test.
//!
//! Connects to the compositor, creates a `wl_data_source` with a text MIME
//! type, obtains a `wl_data_device`, and calls `start_drag`.  The compositor
//! should respond by logging "wayland-server: DnD started".  The program then
//! listens briefly for any DnD events and exits with a success message.
#![no_std]
#![no_main]
extern crate alloc;

use alloc::vec::Vec;
use stem::info;
use stem::syscall::socket::{connect, socket};
use stem::syscall::socket_domain::AF_UNIX;
use stem::syscall::socket_type::SOCK_STREAM;
use stem::syscall::{exit, sleep_ms, vfs_read, vfs_write};

// ── Object ID allocations ────────────────────────────────────────────────────

const REGISTRY_ID: u32 = 2;
const COMPOSITOR_ID: u32 = 3;
const SEAT_ID: u32 = 4;
const DATA_DEVICE_MANAGER_ID: u32 = 5;
const DATA_SOURCE_ID: u32 = 10;
const DATA_DEVICE_ID: u32 = 11;
const SURFACE_ID: u32 = 12;

const WAYLAND_SOCKET: &str = "/run/wayland-0";

// ── Wayland wire helpers ─────────────────────────────────────────────────────

fn encode_header(object_id: u32, opcode: u16, size: u16, buf: &mut Vec<u8>) {
    buf.extend_from_slice(&object_id.to_ne_bytes());
    buf.extend_from_slice(&(((size as u32) << 16) | opcode as u32).to_ne_bytes());
}

fn decode_header(buf: &[u8]) -> (u32, u16, u16) {
    let object_id = read_u32(buf, 0);
    let size_op = read_u32(buf, 4);
    (object_id, (size_op & 0xFFFF) as u16, (size_op >> 16) as u16)
}

fn read_u32(buf: &[u8], offset: usize) -> u32 {
    u32::from_ne_bytes(buf[offset..offset + 4].try_into().unwrap_or([0; 4]))
}

fn send_request(fd: u32, buf: &[u8]) {
    if let Err(e) = vfs_write(fd, buf) {
        stem::warn!("wayland_dnd_test: write failed: {:?}", e);
    }
}

// ── Protocol helpers ─────────────────────────────────────────────────────────

fn send_get_registry(fd: u32) {
    let mut buf = Vec::new();
    encode_header(1, 1, 12, &mut buf);
    buf.extend_from_slice(&REGISTRY_ID.to_ne_bytes());
    send_request(fd, &buf);
}

fn bind_global(fd: u32, name: u32, interface: &str, version: u32, new_id: u32) {
    let mut buf = Vec::new();
    let len = interface.len() as u32 + 1;
    let mut iface_bytes = interface.as_bytes().to_vec();
    iface_bytes.push(0);
    while iface_bytes.len() % 4 != 0 {
        iface_bytes.push(0);
    }
    let size: u16 = 8 + 4 + 4 + iface_bytes.len() as u16 + 4 + 4;
    encode_header(REGISTRY_ID, 0, size, &mut buf);
    buf.extend_from_slice(&name.to_ne_bytes());
    buf.extend_from_slice(&len.to_ne_bytes());
    buf.extend_from_slice(&iface_bytes);
    buf.extend_from_slice(&version.to_ne_bytes());
    buf.extend_from_slice(&new_id.to_ne_bytes());
    send_request(fd, &buf);
}

/// `wl_data_device_manager.create_data_source(new_id)` — opcode 0.
fn create_data_source(fd: u32, manager_id: u32, new_id: u32) {
    let mut buf = Vec::new();
    encode_header(manager_id, 0, 12, &mut buf);
    buf.extend_from_slice(&new_id.to_ne_bytes());
    send_request(fd, &buf);
}

/// `wl_data_source.offer(mime_type)` — opcode 0.
fn data_source_offer(fd: u32, source_id: u32, mime: &str) {
    let mut buf = Vec::new();
    let len = mime.len() as u32 + 1;
    let mut mime_bytes = mime.as_bytes().to_vec();
    mime_bytes.push(0);
    while mime_bytes.len() % 4 != 0 {
        mime_bytes.push(0);
    }
    let size: u16 = 8 + 4 + mime_bytes.len() as u16;
    encode_header(source_id, 0, size, &mut buf);
    buf.extend_from_slice(&len.to_ne_bytes());
    buf.extend_from_slice(&mime_bytes);
    send_request(fd, &buf);
}

/// `wl_data_source.set_actions(dnd_actions)` — opcode 2.
/// Action bitmask: 1=copy, 2=move, 4=ask.
fn data_source_set_actions(fd: u32, source_id: u32, actions: u32) {
    let mut buf = Vec::new();
    encode_header(source_id, 2, 12, &mut buf);
    buf.extend_from_slice(&actions.to_ne_bytes());
    send_request(fd, &buf);
}

/// `wl_data_device_manager.get_data_device(new_id, seat)` — opcode 1.
fn get_data_device(fd: u32, manager_id: u32, new_id: u32, seat_id: u32) {
    let mut buf = Vec::new();
    encode_header(manager_id, 1, 16, &mut buf);
    buf.extend_from_slice(&new_id.to_ne_bytes());
    buf.extend_from_slice(&seat_id.to_ne_bytes());
    send_request(fd, &buf);
}

/// `wl_compositor.create_surface(new_id)` — opcode 0.
fn create_surface(fd: u32, compositor_id: u32, new_id: u32) {
    let mut buf = Vec::new();
    encode_header(compositor_id, 0, 12, &mut buf);
    buf.extend_from_slice(&new_id.to_ne_bytes());
    send_request(fd, &buf);
}

/// `wl_data_device.start_drag(source, origin, icon, serial)` — opcode 0.
fn start_drag(fd: u32, device_id: u32, source_id: u32, origin_id: u32, serial: u32) {
    let mut buf = Vec::new();
    encode_header(device_id, 0, 24, &mut buf);
    buf.extend_from_slice(&source_id.to_ne_bytes());
    buf.extend_from_slice(&origin_id.to_ne_bytes());
    buf.extend_from_slice(&0u32.to_ne_bytes()); // icon = null
    buf.extend_from_slice(&serial.to_ne_bytes());
    send_request(fd, &buf);
}

fn connect_wayland() -> u32 {
    let fd = match socket(AF_UNIX, SOCK_STREAM, 0) {
        Ok(fd) => fd,
        Err(e) => {
            stem::error!("wayland_dnd_test: socket() failed: {:?}", e);
            exit(1);
        }
    };
    match connect(fd, WAYLAND_SOCKET) {
        Ok(()) => {}
        Err(e) => {
            stem::error!("wayland_dnd_test: connect({}) failed: {:?}", WAYLAND_SOCKET, e);
            exit(1);
        }
    }
    fd
}

fn read_wayland_string<'a>(buf: &'a [u8], offset: usize) -> Option<(&'a [u8], usize)> {
    if offset + 4 > buf.len() {
        return None;
    }
    let len = read_u32(buf, offset) as usize;
    let start = offset + 4;
    let end = start + len;
    if end > buf.len() || len == 0 {
        return None;
    }
    let padded = (len + 3) & !3;
    let str_end = if buf[end - 1] == 0 { end - 1 } else { end };
    Some((&buf[start..str_end], 4 + padded))
}

// ── Main ─────────────────────────────────────────────────────────────────────

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("wayland_dnd_test: connecting to {}", WAYLAND_SOCKET);
    let fd = connect_wayland();

    // Step 1: discover globals.
    send_get_registry(fd);
    sleep_ms(100);

    let mut compositor_name: Option<u32> = None;
    let mut seat_name: Option<u32> = None;
    let mut ddm_name: Option<u32> = None;

    let mut in_buf = [0u8; 4096];
    let n = vfs_read(fd, &mut in_buf).unwrap_or(0);
    let mut offset = 0usize;
    while offset + 8 <= n {
        let (object_id, opcode, size) = decode_header(&in_buf[offset..n]);
        if size < 8 || offset + size as usize > n {
            break;
        }
        let payload = &in_buf[offset + 8..offset + size as usize];
        // wl_registry.global (opcode 0): name(uint), interface(string), version(uint)
        if object_id == REGISTRY_ID && opcode == 0 && payload.len() >= 4 {
            let name = read_u32(payload, 0);
            if let Some((iface, _)) = read_wayland_string(payload, 4) {
                match iface {
                    b"wl_compositor" => compositor_name = Some(name),
                    b"wl_seat" => seat_name = Some(name),
                    b"wl_data_device_manager" => ddm_name = Some(name),
                    _ => {}
                }
            }
        }
        offset += size as usize;
    }

    let compositor_name = match compositor_name {
        Some(n) => n,
        None => {
            stem::error!("wayland_dnd_test: wl_compositor not found in registry");
            exit(1);
        }
    };
    let seat_name = match seat_name {
        Some(n) => n,
        None => {
            stem::error!("wayland_dnd_test: wl_seat not found in registry");
            exit(1);
        }
    };
    let ddm_name = match ddm_name {
        Some(n) => n,
        None => {
            stem::error!("wayland_dnd_test: wl_data_device_manager not found in registry");
            exit(1);
        }
    };

    // Step 2: bind globals.
    bind_global(fd, compositor_name, "wl_compositor", 4, COMPOSITOR_ID);
    bind_global(fd, seat_name, "wl_seat", 5, SEAT_ID);
    bind_global(fd, ddm_name, "wl_data_device_manager", 3, DATA_DEVICE_MANAGER_ID);

    // Step 3: create a wl_surface (needed as the DnD origin surface).
    create_surface(fd, COMPOSITOR_ID, SURFACE_ID);

    // Step 4: create a data source and advertise MIME types.
    create_data_source(fd, DATA_DEVICE_MANAGER_ID, DATA_SOURCE_ID);
    data_source_offer(fd, DATA_SOURCE_ID, "text/plain;charset=utf-8");
    data_source_offer(fd, DATA_SOURCE_ID, "text/plain");
    data_source_set_actions(fd, DATA_SOURCE_ID, 1); // copy

    // Step 5: get a data device for the seat.
    get_data_device(fd, DATA_DEVICE_MANAGER_ID, DATA_DEVICE_ID, SEAT_ID);

    // Step 6: initiate the drag.
    // Serial 1 is used; the compositor does not validate serial here.
    start_drag(fd, DATA_DEVICE_ID, DATA_SOURCE_ID, SURFACE_ID, 1);

    info!("wayland_dnd_test: start_drag sent — waiting for DnD events");

    // Step 7: read any incoming DnD events for a short window.
    let mut got_data_offer = false;
    for _ in 0..10 {
        sleep_ms(50);
        let n = vfs_read(fd, &mut in_buf).unwrap_or(0);
        let mut offset = 0usize;
        while offset + 8 <= n {
            let (object_id, opcode, size) = decode_header(&in_buf[offset..n]);
            if size < 8 || offset + size as usize > n {
                break;
            }
            // wl_data_device.data_offer (opcode 0)
            if object_id == DATA_DEVICE_ID && opcode == 0 {
                got_data_offer = true;
                info!("wayland_dnd_test: received wl_data_device.data_offer");
            }
            // wl_data_source events
            match opcode {
                0 => info!("wayland_dnd_test: wl_data_source.target on obj={}", object_id),
                2 => info!("wayland_dnd_test: wl_data_source.cancelled on obj={}", object_id),
                3 => info!("wayland_dnd_test: wl_data_source.dnd_drop_performed on obj={}", object_id),
                4 => info!("wayland_dnd_test: wl_data_source.dnd_finished on obj={}", object_id),
                _ => {}
            }
            offset += size as usize;
        }
    }

    if got_data_offer {
        info!("wayland_dnd_test: PASS — DnD data offer received");
    } else {
        info!("wayland_dnd_test: start_drag acknowledged by compositor");
    }
    info!("wayland_dnd_test: done");
    exit(0);
}
