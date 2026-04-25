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
use core::sync::atomic::{AtomicU8, Ordering};

use abi::display_driver_protocol;
use abi::supervisor_protocol::{self, classes};
use spin::Mutex;
use stem::kinds::{DriverReadyV1, KIND_ID_THINGOS_DRIVER_READY};
use stem::service_loop::{ServiceEvent, ServiceLoop};
use stem::time::Duration;
use stem::{debug, info, trace, warn};

use crate::ledger::DeviceLedger;
use crate::pipelines::{
    mount_hosts_cache, setup_display_pipeline, setup_input_broker, setup_serial_shell,
};
use crate::task::{ManagedTask, TaskKind};

const RUN_POLL_MUX_SELF_TEST: bool = false;
const NETD_PROVIDER_PATH: &str = "/dev/net/virtio0/rx";
const NETD_READY_PATH: &str = "/run/netd.ready";
const NETD_LIVENESS_PATH: &str = "/net/icmp/new";
const NETD_PROBE_INTERVAL_NS: u64 = 1_000_000_000;
const NETD_MAX_PROBE_FAILURES: u32 = 3;
const SERIAL_SHELL_HEADSTART_MS: u64 = 50;
/// Periodic supervisor cadence — preserves the previous 100 ms `sleep_ms`
/// rhythm that drove netd liveness probing and the health-vine restart loop.
const SUPERVISOR_TICK_MS: u64 = 100;
/// Inbox payload size for the Sprout supervisor `ServiceLoop`.  Sized to
/// match Cambium and to comfortably hold a `THINGOS_JOB_EXIT` notification
/// (10 bytes) plus future control messages.
const INBOX_MAX_PAYLOAD: usize = 256;
const PROBE_IDLE: u8 = 0;
const PROBE_IN_FLIGHT: u8 = 1;
const PROBE_SUCCESS: u8 = 2;
const PROBE_FAILURE: u8 = 3;

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
    /// Whether bristle has been spawned (guarded so we only launch once).
    bristle_spawned: bool,
    /// Whether bloom has been spawned (guarded so we only launch once).
    bloom_spawned: bool,
    /// Whether `/dev/display/card0` has been mounted by the display driver.
    display_card_ready: bool,
    /// Whether the display driver has been spawned.
    display_spawned: bool,
    /// Read end of the supervisor port for sovereign registration.
    supervisor_port_read: Option<u32>,
    /// Write end of the supervisor port for sovereign registration.
    supervisor_port_write: Option<u32>,
    /// Wait token for the supervisor port.
    supervisor_token: Option<stem::wait_set::WaitToken>,
    /// Atomic status of the background netd liveness probe.
    netd_probe_status: Arc<AtomicU8>,
}

impl Supervisor {
    pub fn new(registry_ptr: usize) -> Self {
        info!("SPROUT: Initializing Supervisor...");
        let config = Self::parse_cmdline();
        let (write, read) = stem::syscall::port_create(4096).unwrap_or((0, 0));
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
            bristle_spawned: false,
            bloom_spawned: false,
            display_card_ready: false,
            display_spawned: false,
            supervisor_port_read: if read != 0 { Some(read) } else { None },
            supervisor_port_write: if write != 0 { Some(write) } else { None },
            supervisor_token: None,
            netd_probe_status: Arc::new(AtomicU8::new(PROBE_IDLE)),
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
        info!("SPROUT: Supervisor session started (MINIMAL MODE)");

        // Ensure canonical device directories exist.
        let _ = stem::syscall::vfs::vfs_mkdir("/dev/display");

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
        stem::debug!("SPROUT: Waiting for {} before spawning netd...", NETD_PROVIDER_PATH);

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
    /// - typed inbox `Message`s — `DRIVER_READY` marks the corresponding
    ///   `ManagedTask` as ready; unknown kinds are logged and dropped;
    /// - secondary `Ready` events (none registered yet);
    /// - `InboxClosed`, which falls back to the legacy `sleep_ms` loop so
    ///   supervision continues even if the inbox is revoked.
    fn run_service_loop(&mut self, mut svc: ServiceLoop) -> ! {
        if let Some(read_handle) = self.supervisor_port_read {
            if let Ok(fd) = stem::syscall::vfs::vfs_handle_from_port(read_handle) {
                match svc.add_fd_readable(fd) {
                    Ok(tok) => self.supervisor_token = Some(tok),
                    Err(e) => {
                        warn!("SPROUT: failed to register supervisor port in ServiceLoop: {:?}", e)
                    }
                }
            }
        }

        let timeout = Some(Duration::from_millis(SUPERVISOR_TICK_MS));
        loop {
            stem::debug!("SPROUT: ServiceLoop waiting for next event...");
            match svc.next_event(timeout) {
                Ok(ServiceEvent::Message { kind, payload }) => {
                    if kind.0 == KIND_ID_THINGOS_DRIVER_READY {
                        self.handle_driver_ready(payload);
                    } else {
                        // Unknown message kind — log and drop.  We deliberately
                        // do *not* run a tick here so that a burst of inbox
                        // messages cannot accelerate the supervision cadence
                        // beyond the 100ms `Timeout` rhythm.
                        stem::debug!(
                            "SPROUT: ServiceLoop inbox message kind={:?} ({} bytes) — ignored",
                            kind,
                            payload.len()
                        );
                    }
                }
                Ok(ServiceEvent::Ready { token, event }) => {
                    if Some(token) == self.supervisor_token {
                        self.handle_supervisor_port_event();
                    } else {
                        // No secondary readiness sources are registered yet, but
                        // surface unexpected events at debug level so future
                        // additions are observable, then run a normal tick.
                        stem::debug!(
                            "SPROUT: ServiceLoop unexpected secondary ready (token={:?}, flags=0x{:x})",
                            token,
                            event.flags()
                        );
                    }
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

    /// Handle a `DRIVER_READY` inbox message by marking the corresponding
    /// `ManagedTask` as ready.
    fn handle_driver_ready(&mut self, payload: &[u8]) {
        let msg = match DriverReadyV1::from_bytes(payload) {
            Some(m) => m,
            None => {
                warn!(
                    "SPROUT: DRIVER_READY payload too short or wrong version ({} bytes)",
                    payload.len()
                );
                return;
            }
        };

        if msg.status != 0 {
            warn!("SPROUT: DRIVER_READY from PID {} reported error status {}", msg.pid, msg.status);
        }

        let mut tasks = self.tasks.lock();
        if let Some(task) = tasks.iter_mut().find(|t| t.pid == Some(msg.pid as u64)) {
            if !task.ready {
                task.ready = true;
                info!(
                    "SPROUT: DRIVER_READY received — task '{}' (PID {}) marked ready",
                    task.name, msg.pid
                );
            }
        } else {
            stem::debug!(
                "SPROUT: DRIVER_READY from PID {} — no matching ManagedTask (already exited?)",
                msg.pid
            );
        }
    }

    fn handle_supervisor_port_event(&mut self) {
        let read_handle = match self.supervisor_port_read {
            Some(h) => h,
            None => return,
        };

        let fd = match stem::syscall::vfs::vfs_handle_from_port(read_handle) {
            Ok(fd) => fd,
            Err(_) => return,
        };

        let mut data_buf = [0u8; 1024];
        let mut handles = [0u32; 4];
        match stem::syscall::socket::recvmsg(fd, &mut data_buf, &mut handles) {
            Ok((n, h_count)) => {
                let payload = &data_buf[..n];
                let handle = if h_count > 0 { Some(handles[0]) } else { None };
                if let Some((header, payload)) = display_driver_protocol::parse_message(payload) {
                    match header.msg_type {
                        supervisor_protocol::MSG_BIND_READY => {
                            if let Some(bind_payload) =
                                supervisor_protocol::decode_bind_ready_le(payload)
                            {
                                self.handle_bind_ready(bind_payload, handle);
                            }
                        }
                        _ => {
                            stem::debug!(
                                "SPROUT: supervisor port unknown msg_type=0x{:x}",
                                header.msg_type
                            );
                        }
                    }
                }
            }
            Err(e) if e == abi::errors::Errno::EAGAIN => {}
            Err(e) => {
                warn!("SPROUT: supervisor port recvmsg error: {:?}", e);
            }
        }
    }

    fn handle_bind_ready(
        &mut self,
        payload: supervisor_protocol::BindReadyPayload,
        attached_handle: Option<u32>,
    ) {
        debug!("SPROUT: BIND_READY from instance_id=0x{:x}", payload.bind_instance_id);

        if (payload.class_mask & classes::DISPLAY_CARD) != 0 {
            if let Some(handle) = attached_handle {
                debug!(
                    "SPROUT: Registering DISPLAY_CARD at /dev/display/card0 (handle={})",
                    handle
                );
                match stem::syscall::vfs::vfs_mount(handle, "/dev/display/card0") {
                    Ok(_) => {
                        info!("SPROUT: Mounted /dev/display/card0 successfully");
                        self.display_card_ready = true;
                    }
                    Err(e) => warn!("SPROUT: Failed to mount /dev/display/card0: {:?}", e),
                }

                let mut tasks = self.tasks.lock();
                if let Some(task) =
                    tasks.iter_mut().find(|t| t.bind_instance_id == payload.bind_instance_id)
                {
                    task.ready = true;
                    debug!("SPROUT: Handshake complete — task '{}' marked ready", task.name);

                    // Send MSG_BIND_ASSIGNED back to the driver.
                    if let Some(req_port) = task.req_write_port {
                        let assigned = supervisor_protocol::BindAssignedPayload {
                            bind_instance_id: payload.bind_instance_id,
                            status: 0,
                            unit_number: 0,
                            primary_path: {
                                let mut p = [0u8; 64];
                                let path = "/dev/display/card0".as_bytes();
                                p[..path.len()].copy_from_slice(path);
                                p
                            },
                        };
                        let mut assigned_bytes =
                            [0u8; supervisor_protocol::BIND_ASSIGNED_PAYLOAD_SIZE];
                        if let Some(len) = supervisor_protocol::encode_bind_assigned_le(
                            &assigned,
                            &mut assigned_bytes,
                        ) {
                            let mut msg_buf = [0u8; 256];
                            if let Some(total_len) = display_driver_protocol::encode_message(
                                &mut msg_buf,
                                supervisor_protocol::MSG_BIND_ASSIGNED,
                                &assigned_bytes[..len],
                            ) {
                                let res = stem::syscall::port_send(req_port, &msg_buf[..total_len]);
                                debug!(
                                    "SPROUT: Sent MSG_BIND_ASSIGNED to req_port {} (res={:?})",
                                    req_port, res
                                );
                            }
                        }
                    }
                }
            } else {
                warn!("SPROUT: BIND_READY for DISPLAY_CARD missing attached VFS handle");
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
        stem::trace!("SPROUT: Supervisor tick...");
        stem::trace!("SPROUT: Loop iteration: spawn_netd_if_ready");
        self.spawn_netd_if_ready();
        stem::trace!("SPROUT: Loop iteration: verify_netd_liveness");
        self.verify_netd_liveness();
        stem::trace!("SPROUT: Loop iteration: spawn_display_if_needed");
        self.spawn_display_if_needed();
        stem::trace!("SPROUT: Loop iteration: spawn_bristle_if_needed");
        self.spawn_bristle_if_needed();
        stem::trace!("SPROUT: Loop iteration: spawn_bloom_if_ready");
        self.spawn_bloom_if_ready();
        stem::trace!("SPROUT: Loop iteration: run_health_vine");
        run_health_vine(&self.tasks);
        stem::trace!("SPROUT: Loop iteration: check_serviceloop_watchdog");
        let pids = crate::watchdog::collect_monitored_pids(&self.tasks);
        crate::watchdog::check_daemons(&pids, crate::watchdog::DEFAULT_STUCK_DISPATCH_NS);
        stem::trace!("SPROUT: Loop iteration: tick complete");
    }

    fn spawn_display_if_needed(&mut self) {
        if self.display_spawned {
            return;
        }

        let supervisor_port_write = match self.supervisor_port_write {
            Some(w) => w,
            None => return,
        };

        let bind_instance_id = 0x1337_0001;

        if let Some(handles) = setup_display_pipeline(
            self.tasks.clone(),
            supervisor_port_write,
            bind_instance_id,
            self.config.force_bootfb,
        ) {
            info!("SPROUT: Display pipeline initialized (backend={})", handles.backend_name);
            self.display_spawned = true;
        }
    }

    #[allow(dead_code)]
    fn wait_for_display(&mut self) {
        stem::debug!("SPROUT: Waiting for display driver registration...");
        let start = stem::monotonic_ns();
        let timeout = 5_000_000_000; // 5 seconds
        let mut step = 0;

        loop {
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
            self.netd_probe_status.store(PROBE_IDLE, Ordering::SeqCst);
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

        // Check background probe results
        match self.netd_probe_status.load(Ordering::SeqCst) {
            PROBE_SUCCESS => {
                self.netd_verified = true;
                self.netd_probe_failures = 0;
                self.netd_probe_status.store(PROBE_IDLE, Ordering::SeqCst);
                info!(
                    "SPROUT: netd activation probe succeeded for PID {} ({} is responsive)",
                    netd_pid, NETD_LIVENESS_PATH
                );
                return;
            }
            PROBE_FAILURE => {
                self.netd_probe_failures = self.netd_probe_failures.saturating_add(1);
                self.netd_probe_status.store(PROBE_IDLE, Ordering::SeqCst);
                warn!(
                    "SPROUT: netd activation probe failed for PID {} (attempt {}/{}): VFS TIMEOUT/ERROR",
                    netd_pid, self.netd_probe_failures, NETD_MAX_PROBE_FAILURES
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
                return;
            }
            PROBE_IN_FLIGHT => return,
            PROBE_IDLE => {}
            _ => {}
        }

        if !path_exists(NETD_READY_PATH) {
            return;
        }

        let now_ns = stem::monotonic_ns();
        if now_ns.saturating_sub(self.netd_last_probe_ns) < NETD_PROBE_INTERVAL_NS {
            return;
        }
        self.netd_last_probe_ns = now_ns;

        // Launch background probe
        self.netd_probe_status.store(PROBE_IN_FLIGHT, Ordering::SeqCst);
        let status = self.netd_probe_status.clone();
        let _ = stem::thread::spawn_task_detached(move || {
            match stem::syscall::vfs::vfs_open(
                NETD_LIVENESS_PATH,
                abi::syscall::vfs_flags::O_RDONLY | abi::syscall::vfs_flags::O_NONBLOCK,
            ) {
                Ok(fd) => {
                    let _ = stem::syscall::vfs::vfs_close(fd);
                    status.store(PROBE_SUCCESS, Ordering::SeqCst);
                }
                Err(_) => {
                    status.store(PROBE_FAILURE, Ordering::SeqCst);
                }
            }
        });
    }

    #[allow(dead_code)]
    pub fn monitor(&mut self) {
        run_health_vine(&self.tasks);
    }

    /// Spawn bristle (HID broker) the first time we tick after boot.
    ///
    /// Bristle opens a ServiceLoop inbox and publishes device handles to VFS
    /// so PS/2 drivers and bloom can find it.
    fn spawn_bristle_if_needed(&mut self) {
        if self.bristle_spawned {
            return;
        }
        setup_input_broker(self.tasks.clone());
        self.bristle_spawned = true;
    }

    /// Spawn bloom once a display device is available.
    ///
    /// Bloom self-registers with bristle after startup via inbox message.
    fn spawn_bloom_if_ready(&mut self) {
        if self.bloom_spawned {
            return;
        }
        stem::info!("SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...");
        if !self.display_card_ready {
            stem::info!("SPROUT: display card not ready yet");
            return;
        }
        match stem::syscall::spawn_process("/bin/bloom", 0) {
            Ok(pid) => {
                info!("SPROUT: Spawned bloom (PID={})", pid);
                let _ = stem::thread::set_priority(pid, 2);
                let mut tasks = self.tasks.lock();
                tasks.push(ManagedTask {
                    name: "bloom".to_string(),
                    kind: TaskKind::Service("svc.bloom".to_string()),
                    module_path: "/bin/bloom".to_string(),
                    pid: Some(pid),
                    ..Default::default()
                });
            }
            Err(e) => {
                warn!("SPROUT: Failed to spawn bloom: {:?}", e);
            }
        }
        self.bloom_spawned = true;
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
    match stem::syscall::spawn_process("/bin/cambium", 0) {
        Ok(pid) => {
            stem::debug!("SPROUT: Spawned cambium (PID={})", pid);
            let mut tasks = tasks.lock();
            tasks.push(ManagedTask {
                name: "cambium".to_string(),
                kind: TaskKind::Service("svc.cambium".to_string()),
                module_path: "/bin/cambium".to_string(),
                pid: Some(pid),
                ..Default::default()
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
                    task.ready = false;
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
                    task.ready = false;
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

            let handles_owned: &[u64] = &[];

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
