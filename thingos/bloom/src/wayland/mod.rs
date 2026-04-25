//! Wayland server — runs in a dedicated thread using `ServiceLoop`.
//!
//! # Architecture
//!
//! The Wayland server is a separate task (spawned via
//! `stem::thread::spawn_with_arg`).  It:
//!
//! 1. Creates a `ServiceLoop` backed by its own process inbox.
//! 2. Binds a `AF_UNIX SOCK_STREAM` socket at `/run/wayland-0`.
//! 3. Registers the accept socket as a secondary `ServiceLoop` source.
//! 4. On each new connection (`accept`), adds the client socket as another
//!    secondary source.
//! 5. Dispatches Wayland wire messages to [`dispatch`].
//! 6. Sends IPC commands to the main bloom thread via a port and receives
//!    frame-done / buffer-release events back.
//!
//! # Communication with the main thread
//!
//! Wayland → Main: `cmd_write` port handle (see [`ipc`]).
//! Main → Wayland: `evt_read_fd` VFS fd (readable when events are available).

pub mod client;
pub mod dispatch;
pub mod ipc;
pub mod wire;

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use blossom::Blossom;

use stem::service_loop::{ServiceEvent, ServiceLoop};
use stem::syscall::socket::{accept, bind, listen, socket};
use stem::syscall::socket_domain::AF_UNIX;
use stem::syscall::socket_type::SOCK_STREAM;
use stem::syscall::vfs::{vfs_close, vfs_mkdir, vfs_read, vfs_unlink};
use stem::wait_set::WaitToken;
use stem::{info, warn};

use self::client::WaylandClient;

pub const WAYLAND_SOCKET_PATH: &str = "/run/wayland-0";
const MAX_BACKLOG: usize = 8;

/// Initialization parameters passed from the main thread to the Wayland
/// server thread.
pub struct WaylandThreadArgs {
    /// Port write handle used to send IPC commands to the main bloom thread.
    pub cmd_write: u32,
    /// VFS fd for receiving IPC events from the main bloom thread.
    pub evt_read_fd: u32,
}

/// Thread entry point.  Called by `stem::thread::spawn_with_arg`.
///
/// # Safety
///
/// `arg` must be a valid pointer to a `Box<WaylandThreadArgs>` that was
/// created by the calling thread and will not be accessed after the spawn.
pub extern "C" fn wayland_thread_entry(arg: usize) -> ! {
    let args = unsafe { *Box::from_raw(arg as *mut WaylandThreadArgs) };
    WaylandServer::run(args)
}

// ── WaylandServer ────────────────────────────────────────────────────────────

struct WaylandServer {
    /// ServiceLoop driving the event loop.
    svc: ServiceLoop,
    /// Port write handle for sending commands to the main bloom thread.
    cmd_write: u32,
    /// Token for the event FD (main → wayland).
    evt_tok: WaitToken,
    /// The accept socket FD.
    accept_fd: u32,
    /// Token for the accept socket.
    accept_tok: WaitToken,
    /// Map: ServiceLoop token → WaylandClient.
    clients: BTreeMap<WaitToken, WaylandClient>,
    /// xdg-shell state machine (shared across all clients in this thread).
    blossom: Blossom,
    /// Counter for synthetic surface keys (used for alloc_bloom_surface).
    next_surface_key: u32,
    /// VFS fd for receiving events from the main thread.
    evt_read_fd: u32,
}

impl WaylandServer {
    fn run(args: WaylandThreadArgs) -> ! {
        let _ = stem::thread::set_name(b"wayland-server");
        info!("wayland-server: starting");

        // ── ServiceLoop ───────────────────────────────────────────────────
        let mut svc = match ServiceLoop::new(256) {
            Ok(s) => s,
            Err(e) => {
                warn!("wayland-server: ServiceLoop::new failed: {:?}", e);
                loop {
                    stem::sleep_ms(1000);
                }
            }
        };

        // ── Register event FD (main → wayland) ───────────────────────────
        let evt_tok = match svc.add_fd_readable(args.evt_read_fd) {
            Ok(t) => t,
            Err(e) => {
                warn!("wayland-server: failed to register evt_read_fd: {:?}", e);
                loop {
                    stem::sleep_ms(1000);
                }
            }
        };

        // ── Bind Unix domain socket ───────────────────────────────────────
        let _ = vfs_mkdir("/run");
        // Remove any stale socket file.
        let _ = vfs_unlink(WAYLAND_SOCKET_PATH);

        let accept_fd = match socket(AF_UNIX, SOCK_STREAM, 0) {
            Ok(fd) => fd,
            Err(e) => {
                warn!("wayland-server: socket() failed: {:?}", e);
                loop {
                    stem::sleep_ms(1000);
                }
            }
        };

        if let Err(e) = bind(accept_fd, WAYLAND_SOCKET_PATH) {
            warn!("wayland-server: bind({}) failed: {:?}", WAYLAND_SOCKET_PATH, e);
            loop {
                stem::sleep_ms(1000);
            }
        }

        if let Err(e) = listen(accept_fd, MAX_BACKLOG) {
            warn!("wayland-server: listen() failed: {:?}", e);
            loop {
                stem::sleep_ms(1000);
            }
        }

        let accept_tok = match svc.add_fd_readable(accept_fd) {
            Ok(t) => t,
            Err(e) => {
                warn!("wayland-server: failed to register accept_fd: {:?}", e);
                loop {
                    stem::sleep_ms(1000);
                }
            }
        };

        info!("wayland-server: listening on {}", WAYLAND_SOCKET_PATH);

        let mut server = WaylandServer {
            svc,
            cmd_write: args.cmd_write,
            evt_tok,
            accept_fd,
            accept_tok,
            clients: BTreeMap::new(),
            blossom: Blossom::new(),
            next_surface_key: 1,
            evt_read_fd: args.evt_read_fd,
        };

        server.event_loop()
    }

    fn event_loop(&mut self) -> ! {
        let mut evt_buf = [0u8; 128];

        loop {
            let event = match self.svc.next_event(None::<stem::time::Duration>) {
                Ok(ev) => ev,
                Err(e) => {
                    warn!("wayland-server: next_event error: {:?}", e);
                    stem::sleep_ms(10);
                    continue;
                }
            };

            match event {
                ServiceEvent::Message { .. } => {
                    // Control-plane inbox messages are not used by the Wayland server.
                }

                ServiceEvent::Ready { token, event: ev } if ev.is_readable() => {
                    if token == self.accept_tok {
                        self.handle_accept();
                    } else if token == self.evt_tok {
                        // Events from main thread (buffer releases, frame dones).
                        loop {
                            match vfs_read(self.evt_read_fd, &mut evt_buf) {
                                Ok(0) | Err(_) => break,
                                Ok(n) => self.handle_main_event(&evt_buf[..n]),
                            }
                        }
                    } else {
                        // Client socket.
                        self.handle_client_readable(token);
                    }
                }

                ServiceEvent::Ready { token, event: ev } if ev.is_hangup() || ev.is_error() => {
                    // Client or accept socket closed.
                    if token != self.accept_tok && token != self.evt_tok {
                        self.remove_client(token);
                    }
                }

                ServiceEvent::InboxClosed => {
                    info!("wayland-server: inbox closed, shutting down");
                    loop {
                        stem::sleep_ms(1000);
                    }
                }

                ServiceEvent::Timeout => {}
                ServiceEvent::Ready { .. } => {}
            }
        }
    }

    // ── Accept ───────────────────────────────────────────────────────────

    fn handle_accept(&mut self) {
        match accept(self.accept_fd) {
            Ok(client_fd) => {
                let tok = match self.svc.add_fd_readable(client_fd) {
                    Ok(t) => t,
                    Err(e) => {
                        warn!("wayland-server: failed to register client fd: {:?}", e);
                        let _ = vfs_close(client_fd);
                        return;
                    }
                };
                let client = WaylandClient::new(client_fd, self.next_surface_key);
                self.next_surface_key += 100;
                self.clients.insert(tok, client);
                info!("wayland-server: new client fd={}", client_fd);
            }
            Err(e) => {
                warn!("wayland-server: accept() failed: {:?}", e);
            }
        }
    }

    // ── Client read ──────────────────────────────────────────────────────

    fn handle_client_readable(&mut self, token: WaitToken) {
        // Phase 1: read bytes while client is still in the map.
        let client_fd = match self.clients.get(&token) {
            Some(c) => c.fd,
            None => return,
        };

        let mut tmp = [0u8; 4096];
        let mut new_bytes: Vec<u8> = Vec::new();
        let mut dead = false;
        loop {
            match vfs_read(client_fd, &mut tmp) {
                Ok(0) => {
                    dead = true;
                    break;
                }
                Ok(n) => new_bytes.extend_from_slice(&tmp[..n]),
                Err(abi::errors::Errno::EAGAIN) => break,
                Err(_) => {
                    dead = true;
                    break;
                }
            }
        }

        if dead {
            self.remove_client(token);
            return;
        }

        // Phase 2: take client out of the map to allow disjoint field access.
        let mut client = match self.clients.remove(&token) {
            Some(c) => c,
            None => return,
        };
        client.recv_buf.extend_from_slice(&new_bytes);

        // Dispatch all complete messages.
        let mut consumed = 0usize;
        loop {
            match wire::decode_one(&client.recv_buf[consumed..]) {
                None => break,
                Some((msg, sz)) => {
                    consumed += sz;
                    let cmds = dispatch::dispatch(
                        &msg,
                        &mut client,
                        &mut self.blossom,
                        self.cmd_write,
                        &mut self.next_surface_key,
                    );
                    for cmd in cmds {
                        let _ = stem::syscall::port_send_all(self.cmd_write, &cmd);
                    }
                }
            }
        }

        // Remove consumed bytes.
        client.recv_buf.drain(..consumed);

        // Reinsert client.
        self.clients.insert(token, client);
    }

    // ── Main thread events ───────────────────────────────────────────────

    fn handle_main_event(&mut self, data: &[u8]) {
        if data.is_empty() {
            return;
        }
        match data[0] {
            ipc::WEVT_BUFFER_RELEASE => {
                if data.len() < 8 {
                    return;
                }
                let wl_buf_key =
                    u32::from_ne_bytes(data[4..8].try_into().unwrap_or([0; 4]));
                // Find which client owns this key and send wl_buffer.release.
                for client in self.clients.values_mut() {
                    if let Some(&buf_obj) = client.buf_key_to_obj.get(&wl_buf_key) {
                        // wl_buffer.release opcode = 0 (no payload)
                        client.send(buf_obj, 0, &[]);
                        client.buf_key_to_obj.remove(&wl_buf_key);
                        break;
                    }
                }
            }
            ipc::WEVT_FRAME_DONE => {
                if data.len() < 12 {
                    return;
                }
                let bloom_surface_id =
                    u32::from_ne_bytes(data[4..8].try_into().unwrap_or([0; 4]));
                let timestamp_ms =
                    u32::from_ne_bytes(data[8..12].try_into().unwrap_or([0; 4]));
                for client in self.clients.values_mut() {
                    // fire_frame_cbs returns the IDs that were fired; we only
                    // need the side-effect (sending wl_callback.done), so the
                    // list is intentionally dropped.
                    drop(client.fire_frame_cbs(bloom_surface_id, timestamp_ms));
                }
            }
            _ => {}
        }
    }

    // ── Remove client ────────────────────────────────────────────────────

    fn remove_client(&mut self, token: WaitToken) {
        if let Some(client) = self.clients.remove(&token) {
            info!("wayland-server: client disconnected fd={}", client.fd);
            let _ = vfs_close(client.fd);
            self.svc.remove(token);
        }
    }
}
