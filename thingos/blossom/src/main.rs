#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use core::convert::TryInto;

use stem::application::{
    AppAction, Application, ApplicationContext, ServiceLooper, run_application,
};
use stem::info;
use stem::service_loop::ServiceEvent;
use stem::syscall::socket::{connect, sendmsg, socket};
use stem::syscall::socket_domain::AF_UNIX;
use stem::syscall::socket_type::SOCK_STREAM;
use stem::syscall::{memfd_create, sleep_ms, vfs_close, vfs_read, vfs_write, vm_map};
use stem::wait_set::WaitToken;

const REGISTRY_ID: u32 = 2;
const COMPOSITOR_ID: u32 = 3;
const SEAT_ID: u32 = 4;
const LAYER_SHELL_ID: u32 = 5;
const KEYBOARD_ID: u32 = 6;
const SHM_ID: u32 = 7;
const SHORTCUT_SURFACE_ID: u32 = 10;
const SHORTCUT_LAYER_ID: u32 = 11;
const SHORTCUT_POOL_ID: u32 = 12;
const SHORTCUT_BUFFER_ID: u32 = 13;

const GLOBAL_WL_COMPOSITOR: u32 = 1;
const GLOBAL_WL_SHM: u32 = 2;
const GLOBAL_WL_SEAT: u32 = 4;
const GLOBAL_ZWLR_LAYER_SHELL: u32 = 10;

const MOD_ALT: u32 = 1 << 3;
const MOD_META: u32 = 1 << 6;
const EVDEV_R: u32 = 19;
const EVDEV_F2: u32 = 60;

#[stem::main]
fn main(_arg: usize) -> ! {
    run_application::<BlossomApp>()
}

struct BlossomApp {
    fd: u32,
    wayland_token: WaitToken,
    state: ShellState,
}

struct ShellState {
    modifiers: u32,
    shortcut_configured: bool,
    shortcut_mapped: bool,
}

impl Application for BlossomApp {
    const NAME: &'static str = "blossom";

    fn init(
        ctx: &mut ApplicationContext,
        looper: &mut ServiceLooper,
    ) -> Result<Self, stem::errors::Errno> {
        let fd = connect_wayland();
        let wayland_token = looper.add_fd_readable(fd)?;
        send_get_registry(fd, REGISTRY_ID);
        read_initial_globals(fd);
        bind_global(fd, GLOBAL_WL_COMPOSITOR, "wl_compositor", 4, COMPOSITOR_ID);
        bind_global(fd, GLOBAL_WL_SHM, "wl_shm", 1, SHM_ID);
        bind_global(fd, GLOBAL_WL_SEAT, "wl_seat", 5, SEAT_ID);
        bind_global(fd, GLOBAL_ZWLR_LAYER_SHELL, "zwlr_layer_shell_v1", 4, LAYER_SHELL_ID);
        get_keyboard(fd, SEAT_ID, KEYBOARD_ID);
        register_shortcut_surface(fd);
        ctx.register_window("shortcut layer surface", move || close_shortcut_surface(fd));
        ctx.register_cleanup("wayland fd", move || {
            let _ = vfs_close(fd);
        });
        Ok(Self {
            fd,
            wayland_token,
            state: ShellState { modifiers: 0, shortcut_configured: false, shortcut_mapped: false },
        })
    }

    fn ready(&mut self, _ctx: &mut ApplicationContext) {
        info!("Blossom shell connected to Wayland");
    }

    fn handle_event(
        &mut self,
        _ctx: &mut ApplicationContext,
        event: ServiceEvent<'_>,
    ) -> AppAction {
        match event {
            ServiceEvent::Ready { token, event }
                if token == self.wayland_token && event.is_readable() =>
            {
                read_events(self.fd, &mut self.state);
                AppAction::Continue
            }
            ServiceEvent::Ready { token, event }
                if token == self.wayland_token && (event.is_hangup() || event.is_error()) =>
            {
                AppAction::Quit
            }
            ServiceEvent::Timeout | ServiceEvent::Message { .. } | ServiceEvent::Ready { .. } => {
                AppAction::Continue
            }
            ServiceEvent::InboxClosed => AppAction::Quit,
        }
    }
}

fn connect_wayland() -> u32 {
    loop {
        let fd = match socket(AF_UNIX, SOCK_STREAM, 0) {
            Ok(fd) => fd,
            Err(e) => {
                stem::warn!("blossom: socket(AF_UNIX) failed: {:?}", e);
                sleep_ms(250);
                continue;
            }
        };
        match connect(fd, "/run/wayland-0") {
            Ok(()) => return fd,
            Err(_) => {
                let _ = vfs_close(fd);
                sleep_ms(50);
            }
        }
    }
}

fn register_shortcut_surface(fd: u32) {
    create_surface(fd, COMPOSITOR_ID, SHORTCUT_SURFACE_ID);
    get_layer_surface(
        fd,
        LAYER_SHELL_ID,
        SHORTCUT_LAYER_ID,
        SHORTCUT_SURFACE_ID,
        3,
        "thingos.blossom.shortcuts",
    );
    set_layer_size(fd, SHORTCUT_LAYER_ID, 1, 1);
    set_layer_anchor(
        fd,
        SHORTCUT_LAYER_ID,
        blossom::layer_anchor::TOP | blossom::layer_anchor::LEFT,
    );
    set_layer_keyboard_interactivity(fd, SHORTCUT_LAYER_ID, 2);
    commit_surface(fd, SHORTCUT_SURFACE_ID);
}

fn close_shortcut_surface(fd: u32) {
    destroy_object(fd, SHORTCUT_LAYER_ID);
    destroy_object(fd, SHORTCUT_SURFACE_ID);
}

fn read_events(fd: u32, state: &mut ShellState) {
    let mut buf = [0u8; 4096];
    let len = match vfs_read(fd, &mut buf) {
        Ok(n) if n > 0 => n,
        _ => return,
    };
    let mut offset = 0usize;
    while offset + 8 <= len {
        let (object_id, opcode, size) = decode_header(&buf[offset..len]);
        if size < 8 || offset + size as usize > len {
            break;
        }
        let payload = &buf[offset + 8..offset + size as usize];
        match (object_id, opcode) {
            (SHORTCUT_LAYER_ID, 0) if payload.len() >= 12 => {
                let serial = read_u32(payload, 0);
                ack_layer_configure(fd, SHORTCUT_LAYER_ID, serial);
                state.shortcut_configured = true;
                if !state.shortcut_mapped {
                    map_shortcut_surface(fd);
                    state.shortcut_mapped = true;
                    info!("Blossom shortcut surface ready");
                }
            }
            (KEYBOARD_ID, 4) if payload.len() >= 20 => {
                state.modifiers = read_u32(payload, 4);
            }
            (KEYBOARD_ID, 3) if payload.len() >= 16 => {
                let key = read_u32(payload, 8);
                let pressed = read_u32(payload, 12) != 0;
                if pressed && key == EVDEV_R && (state.modifiers & MOD_META) != 0 {
                    info!("Blossom received runbox shortcut");
                }
                if pressed && key == EVDEV_F2 && (state.modifiers & MOD_ALT) != 0 {
                    info!("Blossom received runbox shortcut");
                }
            }
            _ => {}
        }
        offset += size as usize;
    }
}

fn map_shortcut_surface(fd: u32) {
    let memfd = memfd_create("blossom.shortcut", 4).expect("create shortcut memfd");
    use abi::vm::{VmBacking, VmMapReq, VmProt};
    let req = VmMapReq {
        addr_hint: 0,
        len: 4,
        prot: VmProt::READ | VmProt::WRITE | VmProt::USER,
        flags: abi::vm::VmMapFlags::empty(),
        backing: VmBacking::File { thing: memfd, offset: 0 },
    };
    let resp = vm_map(&req).expect("map shortcut memfd");
    unsafe {
        *(resp.addr as *mut u32) = 0;
    }
    create_pool(fd, SHM_ID, SHORTCUT_POOL_ID, memfd, 4);
    create_buffer(fd, SHORTCUT_POOL_ID, SHORTCUT_BUFFER_ID, 1, 1, 4);
    attach_buffer(fd, SHORTCUT_SURFACE_ID, SHORTCUT_BUFFER_ID);
    damage_surface(fd, SHORTCUT_SURFACE_ID, 0, 0, 1, 1);
    commit_surface(fd, SHORTCUT_SURFACE_ID);
}

fn send_get_registry(fd: u32, new_id: u32) {
    let mut buf = Vec::new();
    encode_header(1, 1, 12, &mut buf);
    buf.extend_from_slice(&new_id.to_ne_bytes());
    send_request(fd, &buf);
}

fn bind_global(fd: u32, name: u32, interface: &str, version: u32, new_id: u32) {
    let mut buf = Vec::new();
    let len = interface.len() as u32 + 1;
    let mut bytes = interface.as_bytes().to_vec();
    bytes.push(0);
    while bytes.len() % 4 != 0 {
        bytes.push(0);
    }
    let size = 8 + 4 + 4 + bytes.len() as u16 + 4 + 4;
    encode_header(REGISTRY_ID, 0, size, &mut buf);
    buf.extend_from_slice(&name.to_ne_bytes());
    buf.extend_from_slice(&len.to_ne_bytes());
    buf.extend_from_slice(&bytes);
    buf.extend_from_slice(&version.to_ne_bytes());
    buf.extend_from_slice(&new_id.to_ne_bytes());
    send_request(fd, &buf);
}

fn create_surface(fd: u32, compositor_id: u32, new_id: u32) {
    let mut buf = Vec::new();
    encode_header(compositor_id, 0, 12, &mut buf);
    buf.extend_from_slice(&new_id.to_ne_bytes());
    send_request(fd, &buf);
}

fn get_keyboard(fd: u32, seat_id: u32, new_id: u32) {
    let mut buf = Vec::new();
    encode_header(seat_id, 1, 12, &mut buf);
    buf.extend_from_slice(&new_id.to_ne_bytes());
    send_request(fd, &buf);
}

fn get_layer_surface(
    fd: u32,
    layer_shell_id: u32,
    new_id: u32,
    surface_id: u32,
    layer: u32,
    namespace: &str,
) {
    let mut buf = Vec::new();
    let mut bytes = namespace.as_bytes().to_vec();
    bytes.push(0);
    let len = bytes.len() as u32;
    while bytes.len() % 4 != 0 {
        bytes.push(0);
    }
    let size = 8 + 4 + 4 + 4 + 4 + 4 + bytes.len() as u16;
    encode_header(layer_shell_id, 0, size, &mut buf);
    buf.extend_from_slice(&new_id.to_ne_bytes());
    buf.extend_from_slice(&surface_id.to_ne_bytes());
    buf.extend_from_slice(&0u32.to_ne_bytes());
    buf.extend_from_slice(&layer.to_ne_bytes());
    buf.extend_from_slice(&len.to_ne_bytes());
    buf.extend_from_slice(&bytes);
    send_request(fd, &buf);
}

fn set_layer_size(fd: u32, layer_surface_id: u32, width: u32, height: u32) {
    let mut buf = Vec::new();
    encode_header(layer_surface_id, 0, 16, &mut buf);
    buf.extend_from_slice(&width.to_ne_bytes());
    buf.extend_from_slice(&height.to_ne_bytes());
    send_request(fd, &buf);
}

fn set_layer_anchor(fd: u32, layer_surface_id: u32, anchor: u32) {
    let mut buf = Vec::new();
    encode_header(layer_surface_id, 1, 12, &mut buf);
    buf.extend_from_slice(&anchor.to_ne_bytes());
    send_request(fd, &buf);
}

fn set_layer_keyboard_interactivity(fd: u32, layer_surface_id: u32, mode: u32) {
    let mut buf = Vec::new();
    encode_header(layer_surface_id, 4, 12, &mut buf);
    buf.extend_from_slice(&mode.to_ne_bytes());
    send_request(fd, &buf);
}

fn ack_layer_configure(fd: u32, layer_surface_id: u32, serial: u32) {
    let mut buf = Vec::new();
    encode_header(layer_surface_id, 6, 12, &mut buf);
    buf.extend_from_slice(&serial.to_ne_bytes());
    send_request(fd, &buf);
}

fn create_pool(fd: u32, shm_id: u32, pool_id: u32, memfd: u32, size: u32) {
    let mut buf = Vec::new();
    encode_header(shm_id, 0, 16, &mut buf);
    buf.extend_from_slice(&pool_id.to_ne_bytes());
    buf.extend_from_slice(&size.to_ne_bytes());
    if let Err(e) = sendmsg(fd, &buf, &[memfd]) {
        stem::warn!("blossom: sendmsg failed: {:?}", e);
    }
}

fn create_buffer(fd: u32, pool_id: u32, buffer_id: u32, width: u32, height: u32, stride: u32) {
    let mut buf = Vec::new();
    encode_header(pool_id, 0, 32, &mut buf);
    buf.extend_from_slice(&buffer_id.to_ne_bytes());
    buf.extend_from_slice(&0u32.to_ne_bytes());
    buf.extend_from_slice(&width.to_ne_bytes());
    buf.extend_from_slice(&height.to_ne_bytes());
    buf.extend_from_slice(&stride.to_ne_bytes());
    buf.extend_from_slice(&0u32.to_ne_bytes());
    send_request(fd, &buf);
}

fn attach_buffer(fd: u32, surface_id: u32, buffer_id: u32) {
    let mut buf = Vec::new();
    encode_header(surface_id, 1, 20, &mut buf);
    buf.extend_from_slice(&buffer_id.to_ne_bytes());
    buf.extend_from_slice(&0u32.to_ne_bytes());
    buf.extend_from_slice(&0u32.to_ne_bytes());
    send_request(fd, &buf);
}

fn damage_surface(fd: u32, surface_id: u32, x: i32, y: i32, width: u32, height: u32) {
    let mut buf = Vec::new();
    encode_header(surface_id, 2, 24, &mut buf);
    buf.extend_from_slice(&x.to_ne_bytes());
    buf.extend_from_slice(&y.to_ne_bytes());
    buf.extend_from_slice(&(width as i32).to_ne_bytes());
    buf.extend_from_slice(&(height as i32).to_ne_bytes());
    send_request(fd, &buf);
}

fn commit_surface(fd: u32, surface_id: u32) {
    let mut buf = Vec::new();
    encode_header(surface_id, 6, 8, &mut buf);
    send_request(fd, &buf);
}

fn destroy_object(fd: u32, object_id: u32) {
    let mut buf = Vec::new();
    encode_header(object_id, 0, 8, &mut buf);
    send_request(fd, &buf);
}

fn read_initial_globals(fd: u32) {
    let mut buf = [0u8; 512];
    let _ = vfs_read(fd, &mut buf);
}

fn send_request(fd: u32, buf: &[u8]) {
    if let Err(e) = vfs_write(fd, buf) {
        stem::warn!("blossom: write failed: {:?}", e);
    }
}

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
    u32::from_ne_bytes(buf[offset..offset + 4].try_into().unwrap())
}
