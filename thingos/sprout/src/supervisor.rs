//! Supervisor service for Thing-OS (sprout)
//!
//! Orchestrates the system boot sequence:
//! 1. VFS Namespace management (via memfds and mounts).
//! 2. Sovereign registration handshake (receiving handles from drivers).
//! 3. Graphics stack bring-up (coordinating display + fonts + bloom).
//! 4. Monitoring device arrivals and spawning dependent services.
#![no_std]
use alloc::string::ToString;
use core::default::Default;
extern crate alloc;

// Modules are now declared in main.rs
use alloc::sync::Arc;
use alloc::vec::Vec;

use spin::Mutex;
use stem::service_loop::{ServiceEvent, ServiceLoop};
use stem::syscall::PortHandle;
use stem::time::Duration;
use stem::{info, warn};

use crate::ledger::DeviceLedger;
use crate::pipelines::{mount_hosts_cache, setup_serial_shell};
use crate::task::{ManagedTask, TaskKind};

const RUN_POLL_MUX_SELF_TEST: bool = false;
const NETD_PROVIDER_PATH: &str = "/dev/net/virtio0/rx";
const NETD_READY_PATH: &str = "/run/netd.ready";
const NETD_LIVENESS_PATH: &str = "/net/icmp/new";
const NETD_PROBE_INTERVAL_NS: u64 = 1_000_000_000;
const NETD_MAX_PROBE_FAILURES: u32 = 3;
const SERIAL_SHELL_HEADSTART_MS: u64 = 50;
/// Periodic supervisor cadence — preserves the previous 100 ms `sleep_ms`
/// rhythm that drove `process_registrations`, netd liveness probing, and the
/// health-vine restart loop.
const SUPERVISOR_TICK_MS: u64 = 100;
/// Inbox payload size for the Sprout supervisor `ServiceLoop`.  Sized to
/// match Cambium and to comfortably hold a `THINGOS_JOB_EXIT` notification
/// (10 bytes) plus future control messages.
const INBOX_MAX_PAYLOAD: usize = 256;

pub struct Config {
    pub force_bootfb: bool,
}

pub struct Supervisor {
    pub tasks: Arc<Mutex<Vec<ManagedTask>>>,
    pub ledger: Arc<Mutex<DeviceLedger>>,
    pub registry_ptr: usize,
    pub config: Config,
    netd_spawned: bool,
    netd_active_pid: Option<u64>,
    netd_verified: bool,
    netd_probe_failures: u32,
    netd_last_probe_ns: u64,
}

impl Supervisor {
    pub fn new(registry_ptr: usize) -> Self {
        let config = Self::parse_cmdline();
        Self {
            tasks: Arc::new(Mutex::new(Vec::new())),
            ledger: Arc::new(Mutex::new(DeviceLedger::new())),
            registry_ptr,
            config,
            netd_spawned: false,
            netd_active_pid: None,
            netd_verified: false,
            netd_probe_failures: 0,
            netd_last_probe_ns: 0,
        }
    }

    fn parse_cmdline() -> Config {
        let mut force_bootfb = false;
        let mut buf = [0u8; 1024];

        let mut cmdline_str: Option<alloc::string::String> = None;

        // Try standard argv_get first
        if let Ok(needed) = stem::syscall::argv_get(&mut buf) {
            let limit = needed.min(buf.len());
            if limit > 0 {
                if let Ok(cmdline) = core::str::from_utf8(&buf[..limit]) {
                    cmdline_str = Some(cmdline.to_string());
                }
            }
        }

        // Fallback to /dev/cmdline if needed
        if cmdline_str.is_none() {
            if let Ok(fd) =
                stem::syscall::vfs::vfs_open("/dev/cmdline", abi::syscall::vfs_flags::O_RDONLY)
            {
                if let Ok(n) = stem::syscall::vfs::vfs_read(fd, &mut buf) {
                    if let Ok(cmdline) = core::str::from_utf8(&buf[..n]) {
                        cmdline_str = Some(cmdline.to_string());
                    }
                }
                let _ = stem::syscall::vfs::vfs_close(fd);
            }
        }

        if let Some(cmdline) = cmdline_str {
            stem::debug!("SPROUT: Parsed command line: '{}'", cmdline);
            for part in cmdline.split(|c| c == ' ' || c == '\n' || c == '\r') {
                if part == "display=bootfb" {
                    force_bootfb = true;
                    stem::debug!("SPROUT: Detected 'display=bootfb' command line argument");
                }
            }
        }

        Config { force_bootfb }
    }

    pub fn run_forever(&mut self) -> ! {
        stem::debug!("SPROUT: Supervisor session started (MINIMAL MODE)");

        // Stage 1: Launch Serial Shell
        stem::info!("SPROUT: Launching serial shell...");
        setup_serial_shell(self.tasks.clone());
        stem::info!(
            "SPROUT: Serial shell launched; yielding {}ms so the prompt can take the foreground",
            SERIAL_SHELL_HEADSTART_MS
        );
        stem::sleep_ms(SERIAL_SHELL_HEADSTART_MS);
        stem::debug!("SPROUT: Continuing supervisor startup");

        // Stage 2: Start cambium for driver discovery.
        stem::debug!("SPROUT: Spawning cambium for driver discovery...");
        self.spawn_cambium();

        // Stage 3: Mount iso9660d (ISO9660 VFS provider).
        stem::debug!("SPROUT: Spawning iso9660d...");
        self.spawn_iso9660d();

        // Stage 4: Mount local hostname cache before netd. Mesocarp can serve
        // self entries from VFS state and attach its UDP socket later.
        mount_hosts_cache();

        // Stage 5: Start netd only after the network driver publishes its VFS tree.
        stem::debug!("SPROUT: Deferring netd until {} is ready...", NETD_PROVIDER_PATH);
        info!("SPROUT: Waiting for {} before spawning netd...", NETD_PROVIDER_PATH);

        stem::debug!("SPROUT: Running registration + health supervision loop");

        // Drive supervision through the canonical Layer 3 `ServiceLoop`
        // (inbox-backed actor) instead of a hand-rolled `sleep_ms(100)` loop.
        // The 100ms `Timeout` event preserves the previous periodic cadence;
        // future device-watch / typed control messages can be added as
        // secondary readiness sources without growing another local poll
        // loop.
        //
        // If the inbox cannot be opened (e.g. procfs unavailable during very
        // early boot bring-up), fall back to the legacy `sleep_ms(100)` loop
        // so existing behavior is preserved end-to-end.
        match ServiceLoop::new(INBOX_MAX_PAYLOAD) {
            Ok(svc) => self.run_service_loop(svc),
            Err(err) => {
                warn!(
                    "SPROUT: failed to construct ServiceLoop ({:?}); falling back to legacy supervisor loop",
                    err
                );
                self.run_legacy_supervisor_loop();
            }
        }
    }

    /// Inbox-backed Layer 3 service loop.  Wakes on:
    ///
    /// - the periodic 100ms `Timeout` (drives `tick_supervisor`);
    /// - typed inbox `Message`s (currently logged and ignored — Sprout has
    ///   no Layer 3 control-plane messages defined yet);
    /// - secondary `Ready` events (none registered yet — placeholder for
    ///   the upcoming devices-watch / `KindId::DRIVER_READY` migration);
    /// - `InboxClosed`, which falls back to the legacy `sleep_ms` loop so
    ///   supervision continues even if the inbox is revoked.
    fn run_service_loop(&mut self, mut svc: ServiceLoop) -> ! {
        let timeout = Some(Duration::from_millis(SUPERVISOR_TICK_MS));
        loop {
            match svc.next_event(timeout) {
                Ok(ServiceEvent::Message { kind, payload }) => {
                    // No Sprout-level inbox protocol is defined yet.  Log
                    // and drop unknown messages rather than failing the
                    // loop.  We deliberately do *not* run a tick here so
                    // that a hypothetical burst of inbox messages cannot
                    // accelerate the supervision cadence beyond the 100ms
                    // `Timeout` rhythm — this matches Cambium's
                    // `messages_drained` / `reconcile_due` separation.
                    stem::debug!(
                        "SPROUT: ServiceLoop inbox message kind={:?} ({} bytes) — ignored",
                        kind,
                        payload.len()
                    );
                }
                Ok(ServiceEvent::Ready { token, event }) => {
                    // No secondary readiness sources are registered yet, but
                    // surface unexpected events at debug level so future
                    // additions are observable, then run a normal tick.
                    stem::debug!(
                        "SPROUT: ServiceLoop unexpected secondary ready (token={:?}, flags=0x{:x})",
                        token,
                        event.flags()
                    );
                    self.tick_supervisor();
                }
                Ok(ServiceEvent::Timeout) => {
                    self.tick_supervisor();
                }
                Ok(ServiceEvent::InboxClosed) => {
                    warn!("SPROUT: inbox closed; falling back to legacy supervisor loop");
                    self.run_legacy_supervisor_loop();
                }
                Err(err) => {
                    warn!("SPROUT: ServiceLoop next_event error: {:?}; running tick", err);
                    self.tick_supervisor();
                }
            }
        }
    }

    /// Legacy `sleep_ms(100)`-driven supervisor loop, retained as a
    /// degraded fallback for the (essentially impossible) case where
    /// `ServiceLoop::new` fails or the inbox is closed mid-flight.
    fn run_legacy_supervisor_loop(&mut self) -> ! {
        loop {
            self.tick_supervisor();
            stem::sleep_ms(SUPERVISOR_TICK_MS);
        }
    }

    /// One iteration of the periodic supervisor work that previously ran
    /// inside the hand-rolled `loop { ...; sleep_ms(100); }` body.
    fn tick_supervisor(&mut self) {
        stem::trace!("SPROUT: Loop iteration: process_registrations starting");
        self.process_registrations();
        stem::trace!("SPROUT: Loop iteration: spawn_netd_if_ready");
        self.spawn_netd_if_ready();
        stem::trace!("SPROUT: Loop iteration: verify_netd_liveness");
        self.verify_netd_liveness();
        stem::trace!("SPROUT: Loop iteration: run_health_vine");
        run_health_vine(&self.tasks);
        stem::trace!("SPROUT: Loop iteration: tick complete");
    }

    #[allow(dead_code)]
    fn wait_for_display(&mut self) {
        stem::debug!("SPROUT: Waiting for display driver registration...");
        let start = stem::monotonic_ns();
        let timeout = 5_000_000_000; // 5 seconds
        let mut step = 0;

        loop {
            self.process_registrations();
            self.monitor();

            if step % 20 == 0 {
                stem::info!("SPROUT: Still waiting for display (step {})...", step);
            }
            if step % 100 == 0 {
                stem::info!(
                    "SPROUT: Health check: Loop still running, tasks={}",
                    self.tasks.lock().len()
                );
            }
            step += 1;

            if let Ok(fd) = stem::syscall::vfs::vfs_open(
                "/dev/display/card0",
                stem::abi::syscall::vfs_flags::O_RDONLY,
            ) {
                let _ = stem::syscall::vfs::vfs_close(fd);
                stem::info!("SPROUT: Display card0 detected. Proceeding.");
                break;
            }

            if stem::monotonic_ns() - start > timeout {
                warn!("SPROUT: Timeout waiting for display driver! UI may fail.");
                break;
            }

            stem::sleep_ms(100);
        }
    }

    #[allow(dead_code)]
    fn discover(&mut self) {
        // We no longer auto-spawn everything in /bin.
        // We only scan to keep the registry metadata if needed.
        stem::debug!("SPROUT: Discovery loop disabled in favor of cambium.");
    }

    fn spawn_cambium(&mut self) {
        spawn_cambium_task(self.tasks.clone());
    }

    fn spawn_netd_if_ready(&mut self) {
        if self.netd_spawned {
            return;
        }

        if path_exists(NETD_PROVIDER_PATH) {
            info!("SPROUT: {} is ready; spawning netd.", NETD_PROVIDER_PATH);
            spawn_netd_task(self.tasks.clone());
            self.netd_spawned = true;
        }
    }

    fn spawn_iso9660d(&mut self) {
        spawn_iso9660d_task(self.tasks.clone());
    }

    fn verify_netd_liveness(&mut self) {
        let current_pid = {
            let tasks = self.tasks.lock();
            tasks.iter().find(|t| t.name == "netd").and_then(|t| t.pid)
        };

        if current_pid != self.netd_active_pid {
            self.netd_active_pid = current_pid;
            self.netd_verified = false;
            self.netd_probe_failures = 0;
            self.netd_last_probe_ns = 0;
            if let Some(pid) = current_pid {
                info!("SPROUT: Tracking netd activation (PID={})", pid);
            }
        }

        let Some(netd_pid) = self.netd_active_pid else {
            return;
        };

        if self.netd_verified {
            return;
        }

        if !path_exists(NETD_READY_PATH) {
            return;
        }

        let now_ns = stem::monotonic_ns();
        if now_ns.saturating_sub(self.netd_last_probe_ns) < NETD_PROBE_INTERVAL_NS {
            return;
        }
        self.netd_last_probe_ns = now_ns;

        match stem::syscall::vfs::vfs_open(
            NETD_LIVENESS_PATH,
            abi::syscall::vfs_flags::O_RDONLY | abi::syscall::vfs_flags::O_NONBLOCK,
        ) {
            Ok(fd) => {
                let _ = stem::syscall::vfs::vfs_close(fd);
                self.netd_verified = true;
                info!(
                    "SPROUT: netd activation probe succeeded for PID {} ({} is responsive)",
                    netd_pid,
                    NETD_LIVENESS_PATH
                );
            }
            Err(e) => {
                self.netd_probe_failures = self.netd_probe_failures.saturating_add(1);
                warn!(
                    "SPROUT: netd activation probe failed for PID {} (attempt {}/{}): {:?}",
                    netd_pid,
                    self.netd_probe_failures,
                    NETD_MAX_PROBE_FAILURES,
                    e
                );
                if self.netd_probe_failures >= NETD_MAX_PROBE_FAILURES {
                    warn!(
                        "SPROUT: netd PID {} is still unresponsive after readiness; restarting",
                        netd_pid
                    );
                    let _ = stem::syscall::signal::kill(netd_pid as i32, abi::signal::SIGTERM);
                    self.netd_probe_failures = 0;
                    self.netd_last_probe_ns = 0;
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn monitor(&mut self) {
        run_health_vine(&self.tasks);
    }

    #[allow(dead_code)]
    fn process_registrations(&mut self) {
        // Collect tasks that need polling. We only poll tasks that have a response
        // port and are currently alive.
        let tasks_to_poll: Vec<(u32, u32, alloc::string::String)> = {
            let mut tasks = self.tasks.lock();
            let mut poll_set = Vec::new();

            for t in tasks.iter_mut() {
                if t.drv_resp_read != 0 && t.pid.is_some() {
                    // Initialize the cached FD if we haven't already.
                    if t.resp_fd.is_none() {
                        if let Ok(fd) = stem::syscall::vfs::vfs_handle_from_port(t.drv_resp_read) {
                            t.resp_fd = Some(fd);
                            stem::debug!(
                                "SPROUT: Bridged resp_port {} -> FD {} for task '{}'",
                                t.drv_resp_read,
                                fd,
                                t.name
                            );
                        }
                    }

                    if let Some(fd) = t.resp_fd {
                        poll_set.push((fd, t.drv_req_write, t.name.clone()));
                    }
                }
            }
            poll_set
        };

        let mut pollfds = tasks_to_poll
            .iter()
            .map(|(resp_fd, _, _)| abi::syscall::PollHandle {
                handle: *resp_fd as i32,
                events: abi::syscall::poll_flags::POLLIN
                    | abi::syscall::poll_flags::POLLHUP
                    | abi::syscall::poll_flags::POLLERR,
                revents: 0,
            })
            .collect::<Vec<_>>();

        if !pollfds.is_empty() {
            match stem::syscall::vfs::vfs_poll(&mut pollfds, 100) {
                Ok(n) => {
                    if n > 0 {
                        stem::trace!("SPROUT: poll yielded {} ready events", n);
                    }
                }
                Err(e) => {
                    warn!("SPROUT: registration poll failed: {:?}", e);
                    return;
                }
            }
        }

        // DRAIN MESSAGES
        for ((resp_fd, drv_req_write, task_name), _pollfd) in
            tasks_to_poll.into_iter().zip(pollfds.iter())
        {
            let mut msg_data = [0u8; 1024];
            let mut msg_fds = [0u32; 1];

            loop {
                // info!("SPROUT: Calling recvmsg for {}...", task_name);
                match stem::syscall::socket::recvmsg(resp_fd, &mut msg_data, &mut msg_fds) {
                    Ok((n, n_fds)) => {
                        info!(
                            "SPROUT: Recvmsg SUCCESS from {}: n={}, nfds={}",
                            task_name, n, n_fds
                        );
                        if n == 0 && n_fds == 0 {
                            break;
                        }

                        let bundled_fd = if n_fds > 0 { msg_fds[0] } else { 0 };
                        let parsed = abi::display_driver_protocol::parse_message(&msg_data[..n]);
                        if let Some((header, payload)) = parsed {
                            if header.msg_type == abi::supervisor_protocol::MSG_BIND_READY {
                                info!(
                                    "SPROUT: BIND_READY from {} (bundle_fd={})",
                                    task_name, bundled_fd
                                );
                                self.handle_bind_ready(
                                    &task_name,
                                    drv_req_write,
                                    payload,
                                    bundled_fd,
                                );
                            }
                        }
                    }
                    Err(abi::errors::Errno::EAGAIN) => break,
                    Err(e) => {
                        warn!("SPROUT: recvmsg error from {}: {:?}", task_name, e);
                        break;
                    }
                }
            }
        }
    }

    #[allow(dead_code)]
    fn handle_bind_ready(
        &mut self,
        task_name: &str,
        drv_req_write: PortHandle,
        payload: &[u8],
        bundled_fd: u32,
    ) {
        use abi::supervisor_protocol::{self, MSG_BIND_ASSIGNED, MSG_BIND_FAILED, classes};
        use stem::syscall::{port_send_all, vfs_close, vfs_mount};

        // Helper: send MSG_BIND_FAILED back to the driver.
        let send_failed = |req_write: u32, id: u64, code: u32, msg: &[u8]| {
            let mut reason = [0u8; 64];
            let len = msg.len().min(64);
            reason[..len].copy_from_slice(&msg[..len]);
            let failed = supervisor_protocol::BindFailedPayload {
                bind_instance_id: id,
                error_code: code,
                _reserved: 0,
                reason,
            };
            let mut payload_bytes = [0u8; supervisor_protocol::BIND_FAILED_PAYLOAD_SIZE];
            let mut reply_buf = [0u8; 256];
            if let Some(p_len) =
                supervisor_protocol::encode_bind_failed_le(&failed, &mut payload_bytes)
            {
                if let Some(total_len) = abi::display_driver_protocol::encode_message(
                    &mut reply_buf,
                    MSG_BIND_FAILED,
                    &payload_bytes[..p_len],
                ) {
                    let _ = port_send_all(req_write, &reply_buf[..total_len]);
                }
            }
        };

        if let Some(ready) = supervisor_protocol::decode_bind_ready_le(payload) {
            let provider_port = bundled_fd;
            if provider_port == 0 {
                warn!("SPROUT: BIND_READY from {} carried no provider FD — rejecting", task_name);
                send_failed(
                    drv_req_write,
                    ready.bind_instance_id,
                    supervisor_protocol::errors::ERR_NO_PROVIDER_HANDLE,
                    b"no provider handle attached",
                );
                return;
            }

            let class_alloc = if ready.class_mask & classes::DISPLAY_CARD != 0 {
                Some(("display", "/dev/display/card"))
            } else if ready.class_mask & classes::INPUT_EVENT != 0 {
                Some(("input", "/dev/input/event"))
            } else if ready.class_mask & classes::BLOCK_DEVICE != 0 {
                Some(("block", "/dev/block/sd"))
            } else if ready.class_mask & classes::NETWORK_INTERFACE != 0 {
                Some(("net", "/dev/net/virtio"))
            } else if ready.class_mask & classes::SOUND_CARD != 0 {
                Some(("sound", "/dev/sound/card"))
            } else {
                None
            };

            let (class_name, root) = match class_alloc {
                Some(pair) => pair,
                None => {
                    warn!(
                        "SPROUT: BIND_READY from {} has unrecognised class_mask 0x{:x} — rejecting",
                        task_name, ready.class_mask
                    );
                    send_failed(
                        drv_req_write,
                        ready.bind_instance_id,
                        supervisor_protocol::errors::ERR_UNKNOWN_CLASS,
                        b"class_mask is zero or unrecognised",
                    );
                    let _ = vfs_close(provider_port);
                    return;
                }
            };

            let mut ledger = self.ledger.lock();
            let unit = ledger.get(class_name).cloned().unwrap_or(0);
            ledger.insert(class_name.to_string(), unit + 1);
            let path = alloc::format!("{root}{unit}");
            drop(ledger); // Release ledger lock before mounting

            match vfs_mount(provider_port, &path) {
                Ok(()) => {
                    info!("SPROUT: Sovereign mount success: {} -> {}", task_name, path);

                    let mut assigned = supervisor_protocol::BindAssignedPayload {
                        bind_instance_id: ready.bind_instance_id,
                        status: 0,
                        unit_number: unit,
                        primary_path: [0u8; 64],
                    };
                    let path_bytes = path.as_bytes();
                    let len = path_bytes.len().min(64);
                    assigned.primary_path[..len].copy_from_slice(&path_bytes[..len]);

                    let mut reply_buf = [0u8; 256];
                    let mut payload_bytes = [0u8; supervisor_protocol::BIND_ASSIGNED_PAYLOAD_SIZE];
                    if let Some(p_len) =
                        supervisor_protocol::encode_bind_assigned_le(&assigned, &mut payload_bytes)
                    {
                        if let Some(total_len) = abi::display_driver_protocol::encode_message(
                            &mut reply_buf,
                            MSG_BIND_ASSIGNED,
                            &payload_bytes[..p_len],
                        ) {
                            let _ = port_send_all(drv_req_write, &reply_buf[..total_len]);
                        }
                    }

                    // `vfs_mount` snapshots the provider port into a mounted
                    // ProviderFs, but it does not consume the caller's handle.
                    // Closing the bundled FD here tears down the only live
                    // writer the provider task still owns and makes the mount
                    // go stale under clients like bloom.
                }
                Err(e) => {
                    warn!("SPROUT: Sovereign mount FAILED for {}: {:?}", task_name, e);
                    send_failed(
                        drv_req_write,
                        ready.bind_instance_id,
                        supervisor_protocol::errors::ERR_MOUNT_FAILED,
                        b"vfs_mount failed",
                    );
                    let _ = vfs_close(bundled_fd);
                }
            }
        } else {
            warn!("SPROUT: Received malformed BIND_READY from {} — rejecting", task_name);
            send_failed(
                drv_req_write,
                0,
                supervisor_protocol::errors::ERR_INVALID_MESSAGE,
                b"BIND_READY payload is malformed",
            );
            if bundled_fd != 0 {
                let _ = vfs_close(bundled_fd);
            }
        }
    }
}

fn path_exists(path: &str) -> bool {
    match stem::syscall::vfs::vfs_open(path, abi::syscall::vfs_flags::O_RDONLY) {
        Ok(fd) => {
            let _ = stem::syscall::vfs::vfs_close(fd);
            true
        }
        Err(_) => false,
    }
}

fn spawn_cambium_task(tasks: Arc<Mutex<Vec<ManagedTask>>>) {
    let (write, read) = match stem::syscall::port_create(4096) {
        Ok(h) => h,
        Err(_) => return,
    };
    match stem::syscall::spawn_process("/bin/cambium", 0) {
        Ok(pid) => {
            stem::debug!("SPROUT: Spawned cambium (PID={})", pid);
            let mut tasks = tasks.lock();
            tasks.push(ManagedTask {
                name: "cambium".to_string(),
                kind: TaskKind::Service("svc.cambium".to_string()),
                module_path: "/bin/cambium".to_string(),
                pid: Some(pid),
                restarts: 0,
                spawn_arg: 0,
                bind_instance_id: 0,
                drv_req_write: write,
                drv_resp_read: read,
                boot_req_read: 0,
                boot_resp_write: 0,
                resp_fd: None,
            });
        }
        Err(e) => warn!("SPROUT: Failed to spawn cambium: {:?}", e),
    }
}

fn spawn_netd_task(tasks: Arc<Mutex<Vec<ManagedTask>>>) {
    // Spawn netd via VFS-loaded spawn_process_ex (not the legacy boot-modules
    // path) so the initial spawn uses the same code path the supervisor uses
    // when restarting netd.  Using two different paths produced two different
    // ProcessInfo / handle-table layouts and the boot-modules variant left
    // netd's request port unable to wake the main poll loop, so the first
    // ping after "Network ready" timed out.
    let path = "/bin/netd";
    let argv: &[&[u8]] = &[path.as_bytes()];
    let env = alloc::collections::BTreeMap::new();
    let inherit = stem::abi::types::stdio_mode::INHERIT;
    let spawn_res =
        stem::syscall::spawn_process_ex(path, argv, &env, inherit, inherit, inherit, 0, &[]);
    match spawn_res {
        Ok(resp) => {
            let pid = resp.child_tid;
            stem::debug!("SPROUT: Spawned netd (PID={})", pid);
            let mut tasks = tasks.lock();
            tasks.push(ManagedTask {
                name: "netd".to_string(),
                kind: TaskKind::Service("svc.netd".to_string()),
                module_path: path.to_string(),
                pid: Some(pid),
                ..Default::default()
            });
        }
        Err(e) => warn!("SPROUT: Failed to spawn netd: {:?}", e),
    }
}

fn spawn_iso9660d_task(tasks: Arc<Mutex<Vec<ManagedTask>>>) {
    match stem::syscall::spawn_process("/bin/iso9660d", 0) {
        Ok(pid) => {
            stem::debug!("SPROUT: Spawned iso9660d (PID={})", pid);
            let mut tasks = tasks.lock();
            tasks.push(ManagedTask {
                name: "iso9660d".to_string(),
                kind: TaskKind::Service("svc.iso9660d".to_string()),
                module_path: "/bin/iso9660d".to_string(),
                pid: Some(pid),
                ..Default::default()
            });
        }
        Err(e) => warn!("SPROUT: Failed to spawn iso9660d: {:?}", e),
    }
}

/// Health-monitoring vine body.
///
/// This runs on its own kernel task so it never blocks the registration loop.
/// It is the *only* caller of `waitpid(-1, WNOHANG)` which prevents races with
/// the main supervisor vine.
///
/// Logic:
/// 1. Drain all pending child exits via non-blocking waitpid.
/// 2. If every supervised task has exited, trigger a clean system shutdown.
/// 3. Restart any task whose PID slot was cleared by step 1.
fn run_health_vine(tasks: &Arc<Mutex<Vec<ManagedTask>>>) {
    // Step 1: poll each supervised child directly. This avoids broad
    // waitpid(-1) scans during early boot and keeps supervision scoped.
    let tracked_children: Vec<u64> = {
        let task_list = tasks.lock();
        task_list.iter().filter_map(|t| t.pid).collect()
    };

    for child_pid in tracked_children {
        let wait_res = stem::syscall::waitpid(child_pid as i64, abi::types::waitpid_flags::WNOHANG);
        match wait_res {
            Ok((0, _)) => {
                // Child still running.
            }
            Ok((reaped_pid, wait_status)) if reaped_pid > 0 => {
                // Decode exit code: normal exits carry the code in bits [15:8];
                // signal-terminated exits carry the raw wait_status (negative by
                // convention so callers can distinguish them from clean exits).
                let exit_code: i32 = if abi::signal::wifexited(wait_status) {
                    abi::signal::wexitstatus(wait_status) as i32
                } else {
                    wait_status
                };

                let mut task_list = tasks.lock();
                if let Some(task) = task_list.iter_mut().find(|t| t.pid == Some(reaped_pid as u64))
                {
                    info!(
                        "SPROUT: Task '{}' (PID {}) died with code {}. Restarting...",
                        task.name, reaped_pid, exit_code
                    );
                    task.pid = None;
                    if let Some(fd) = task.resp_fd.take() {
                        let _ = stem::syscall::vfs::vfs_close(fd);
                    }
                    task.restarts += 1;
                }
            }
            Ok(_) => {
                // Non-matching success tuple; ignore and continue supervision.
            }
            Err(abi::errors::Errno::ECHILD) => {
                // If the kernel says this PID is no longer our child, clear it so
                // restart logic can recover the managed task slot.
                let mut task_list = tasks.lock();
                if let Some(task) = task_list.iter_mut().find(|t| t.pid == Some(child_pid)) {
                    task.pid = None;
                    if let Some(fd) = task.resp_fd.take() {
                        let _ = stem::syscall::vfs::vfs_close(fd);
                    }
                    task.restarts += 1;
                }
            }
            Err(e) => {
                warn!("SPROUT: waitpid({}, WNOHANG) returned unexpected error: {:?}", child_pid, e);
            }
        }
    }

    // Step 2: init-style lifecycle — if all supervised children are gone, halt.
    let should_shutdown = {
        let task_list = tasks.lock();
        !task_list.is_empty() && task_list.iter().all(|t| t.pid.is_none())
    };
    if should_shutdown {
        info!("SPROUT: All supervised tasks have exited. Performing system shutdown...");
        stem::syscall::shutdown();
    }

    // Step 3: restart any task whose PID slot was cleared above.
    let mut task_list = tasks.lock();
    for task in task_list.iter_mut() {
        if task.pid.is_none() {
            if task.name == "shell" {
                let selected = crate::pipelines::select_serial_shell();
                if task.module_path != selected {
                    info!(
                        "SPROUT: Switching serial shell from '{}' to '{}'",
                        task.module_path, selected
                    );
                    task.module_path = selected;
                }
            }

            let handles_owned: Vec<u64> = if task.boot_req_read != 0 && task.boot_resp_write != 0 {
                alloc::vec![task.boot_req_read as u64, task.boot_resp_write as u64]
            } else {
                alloc::vec![]
            };

            // The spawn_arg integer is already passed through the syscall's
            // dedicated `boot_arg` field below; do not duplicate it as an
            // extra argv entry (that's what produced spurious "/bin/netd 0"
            // entries in `ps`).
            let spawn_path = if task.module_path.is_empty() {
                task.name.as_str()
            } else {
                task.module_path.as_str()
            };

            let mut stdin_mode = stem::abi::types::stdio_mode::INHERIT;
            let mut stdout_mode = stem::abi::types::stdio_mode::INHERIT;
            let mut stderr_mode = stem::abi::types::stdio_mode::INHERIT;
            let mut console_fd_to_close: Option<u32> = None;
            if task.name == "shell" {
                if let Ok(console_fd) =
                    stem::syscall::vfs::vfs_open("/dev/console", abi::syscall::vfs_flags::O_RDWR)
                {
                    stdin_mode = stem::abi::types::stdio_mode::handle(console_fd);
                    stdout_mode = stem::abi::types::stdio_mode::handle(console_fd);
                    stderr_mode = stem::abi::types::stdio_mode::handle(console_fd);
                    console_fd_to_close = Some(console_fd);
                }
            }

            let spawn_res = stem::syscall::spawn_process_ex(
                spawn_path,
                &[spawn_path.as_bytes()],
                &alloc::collections::BTreeMap::new(),
                stdin_mode,
                stdout_mode,
                stderr_mode,
                task.spawn_arg as u64,
                &handles_owned,
            );

            if let Some(fd) = console_fd_to_close {
                let _ = stem::syscall::vfs::vfs_close(fd);
            }

            if let Ok(resp) = spawn_res {
                task.pid = Some(resp.child_tid);
                if let TaskKind::Driver(_) = task.kind {
                    let _ = stem::thread::set_priority(resp.child_tid, 3);
                }
            }
        }
    }
}
