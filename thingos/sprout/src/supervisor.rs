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

use abi::display_driver_protocol;
use abi::supervisor_protocol::{
    self, MSG_BIND_ASSIGNED, MSG_BIND_FAILED, MSG_BIND_READY, MSG_SERVICE_EXITING,
    MSG_SERVICE_READY, classes,
};
use spin::Mutex;
use stem::syscall::{ChannelHandle, channel_create, channel_send_all, vfs_mount};
use stem::{debug, error, info, warn};

use crate::ledger::DeviceLedger;
use crate::pipelines::{
    DisplayHandles, setup_audio_stack, setup_display_pipeline, setup_graphics_stack,
    setup_input_broker, setup_serial_shell, setup_ui_services,
};
use crate::task::{ManagedTask, TaskKind};

const RUN_POLL_MUX_SELF_TEST: bool = false;

pub struct Config {
    pub force_bootfb: bool,
}

pub struct Supervisor {
    pub tasks: Arc<Mutex<Vec<ManagedTask>>>,
    pub ledger: Arc<Mutex<DeviceLedger>>,
    pub registry_ptr: usize,
    pub config: Config,
}

impl Supervisor {
    pub fn new(registry_ptr: usize) -> Self {
        let config = Self::parse_cmdline();
        Self {
            tasks: Arc::new(Mutex::new(Vec::new())),
            ledger: Arc::new(Mutex::new(DeviceLedger::new())),
            registry_ptr,
            config,
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
            info!("SPROUT: Parsed command line: '{}'", cmdline);
            for part in cmdline.split(|c| c == ' ' || c == '\n' || c == '\r') {
                if part == "display=bootfb" {
                    force_bootfb = true;
                    info!("SPROUT: Detected 'display=bootfb' command line argument");
                }
            }
        }
        
        info!("SPROUT: FORCING display=bootfb for diagnostic test!");
        force_bootfb = true;

        Config { force_bootfb }
    }

    pub fn run_forever(&mut self) -> ! {
        stem::debug!("SPROUT: Supervisor session started (Sovereign mode)");

        // Stage 1: Discover boot modules
        self.discover();

        // Stage 2: Create Sovereign Registrar channel
        let (supervisor_write, supervisor_read) =
            channel_create(4096).expect("Failed to create supervisor registrar channel");

        // Stage 3: Launch Serial Shell EARLY on its own processor
        info!("SPROUT: Launching early serial shell...");
        let tasks_cloned = self.tasks.clone();
        let _ = stem::thread::spawn_task(move || {
            setup_serial_shell(tasks_cloned);
        });

        // Stage 4: Launch cambium
        info!("SPROUT: Launching cambium...");
        self.spawn_cambium();

        // Stage 8: Optional readiness model verification test.
        // Keep this opt-in so early-boot diagnosis is not perturbed by extra
        // child process lifecycle traffic.
        if RUN_POLL_MUX_SELF_TEST {
            info!("SPROUT: Spawning poll_mux verification test...");
            let _ = stem::syscall::spawn_process("/bin/poll_mux", 0);
        } else {
            info!("SPROUT: poll_mux verification test disabled");
        }

        // Stage 5: Graphics Stack Bring-up
        info!("SPROUT: Bringing up graphics stack...");
        let display_handles = setup_display_pipeline(
            self.tasks.clone(),
            supervisor_write,
            0xB001,
            self.config.force_bootfb
        );

        // Stage 6: Wait for Display Driver to register its VFS provider
        self.wait_for_display();

        // Stage 7: High-level UI Services
        info!("SPROUT: Launching UI services...");
        let input_handles = setup_input_broker(self.tasks.clone());
        setup_graphics_stack(self.tasks.clone(), display_handles, input_handles);
        setup_ui_services(self.tasks.clone());

        loop {
            stem::trace!(
                "SPROUT: --- Supervisor Loop Cycle Start (tasks={}) ---",
                self.tasks.lock().len()
            );
            self.process_registrations();
            self.monitor();
            stem::trace!("SPROUT: --- Supervisor Loop Cycle End ---");
            stem::sleep_ms(100);
        }
    }

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

    fn discover(&mut self) {
        // We no longer auto-spawn everything in /bin.
        // We only scan to keep the registry metadata if needed.
        stem::debug!("SPROUT: Discovery loop disabled in favor of cambium.");
    }

    fn spawn_cambium(&mut self) {
        let (write, read) = match stem::syscall::channel_create(4096) {
            Ok(h) => h,
            Err(_) => return,
        };
        // We could pass the registrar port to cambium if it needs to register things,
        // but for now cambium just spawns drivers.
        match stem::syscall::spawn_process("/bin/cambium", 0) {
            Ok(pid) => {
                info!("SPROUT: Spawned cambium (PID={})", pid);
                let mut tasks = self.tasks.lock();
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

    pub fn monitor(&mut self) {
        // Drain all pending child exits using a single non-blocking
        // waitpid(-1, WNOHANG) loop.  This replaces the old per-task
        // task_poll scan and eliminates the O(N) scheduler-lock pressure it
        // introduced: instead of one kernel call per supervised task, we make
        // at most one call per exited child plus one final call that returns 0
        // (no more exited children).
        loop {
            match stem::syscall::waitpid(-1, abi::types::waitpid_flags::WNOHANG) {
                Ok((0, _)) => break, // No more exited children right now.
                Ok((child_pid, wait_status)) => {
                    // Decode exit code: normal exits carry the code in bits [15:8];
                    // signal-terminated exits carry the raw wait_status (negative by
                    // convention so callers can distinguish them from clean exits).
                    let exit_code: i32 = if abi::signal::wifexited(wait_status) {
                        abi::signal::wexitstatus(wait_status) as i32
                    } else {
                        wait_status
                    };
                    let child_pid = child_pid as u64;
                    let mut tasks = self.tasks.lock();
                    if let Some(task) = tasks.iter_mut().find(|t| t.pid == Some(child_pid)) {
                        info!(
                            "SPROUT: Task '{}' (PID {}) died with code {}. Restarting...",
                            task.name, child_pid, exit_code
                        );
                        task.pid = None;
                        if let Some(fd) = task.resp_fd.take() {
                            let _ = stem::syscall::vfs::vfs_close(fd);
                        }
                        task.restarts += 1;
                    } else {
                        info!(
                            "SPROUT: Unknown child PID {} exited (code {})",
                            child_pid, exit_code
                        );
                    }
                }
                Err(abi::errors::Errno::ECHILD) => break, // No children remain; expected.
                Err(e) => {
                    warn!("SPROUT: waitpid(-1, WNOHANG) returned unexpected error: {:?}", e);
                    break;
                }
            }
        }

        // Init-style lifecycle: if all supervised children are gone, halt the system.
        let should_shutdown = {
            let tasks = self.tasks.lock();
            !tasks.is_empty() && tasks.iter().all(|t| t.pid.is_none())
        };
        if should_shutdown {
            info!("SPROUT: All supervised tasks have exited. Performing system shutdown...");
            stem::syscall::shutdown();
        }

        // Handle Spawning/Restarting for tasks without PIDs
        let mut tasks = self.tasks.lock();
        for task in tasks.iter_mut() {
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

                let handles_owned: Vec<u64> =
                    if task.boot_req_read != 0 && task.boot_resp_write != 0 {
                        alloc::vec![task.boot_req_read as u64, task.boot_resp_write as u64]
                    } else {
                        alloc::vec![]
                    };

                let arg_str = alloc::format!("{}", task.spawn_arg);
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
                    if let Ok(console_fd) = stem::syscall::vfs::vfs_open(
                        "/dev/console",
                        abi::syscall::vfs_flags::O_RDWR,
                    ) {
                        stdin_mode = stem::abi::types::stdio_mode::thing(console_fd);
                        stdout_mode = stem::abi::types::stdio_mode::thing(console_fd);
                        stderr_mode = stem::abi::types::stdio_mode::thing(console_fd);
                        console_fd_to_close = Some(console_fd);
                    }
                }

                let spawn_res = stem::syscall::spawn_process_ex(
                    spawn_path,
                    &[spawn_path.as_bytes(), arg_str.as_bytes()],
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

    fn process_registrations(&mut self) {
        // Collect tasks that need polling. We only poll tasks that have a response
        // channel and are currently alive.
        let tasks_to_poll: Vec<(u32, u32, alloc::string::String)> = {
            let mut tasks = self.tasks.lock();
            let mut poll_set = Vec::new();

            for t in tasks.iter_mut() {
                if t.drv_resp_read != 0 && t.pid.is_some() {
                    // Initialize the cached FD if we haven't already.
                    if t.resp_fd.is_none() {
                        if let Ok(fd) = stem::syscall::vfs::vfs_thing_from_channel(t.drv_resp_read) {
                            t.resp_fd = Some(fd);
                            stem::info!("SPROUT: Bridged resp_channel {} -> FD {} for task '{}'",
                                t.drv_resp_read, fd, t.name);
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
            .map(|(resp_fd, _, _)| abi::syscall::PollThing {
                thing: *resp_fd as i32,
                events: abi::syscall::poll_flags::POLLIN
                    | abi::syscall::poll_flags::POLLHUP
                    | abi::syscall::poll_flags::POLLERR,
                revents: 0,
            })
            .collect::<Vec<_>>();

        if !pollfds.is_empty() {
            let mut poll_trace = alloc::string::String::new();
            for t in &tasks_to_poll {
                use core::fmt::Write;
                let _ = write!(&mut poll_trace, " {}:{}", t.2, t.0);
            }
            stem::trace!("SPROUT: Polling FDs:{}", poll_trace);

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
        for ((resp_fd, drv_req_write, task_name), pollfd) in tasks_to_poll.into_iter().zip(pollfds.iter()) {
            let mut msg_data = [0u8; 1024];
            let mut msg_fds = [0u32; 1];
            
            loop {
                // info!("SPROUT: Calling recvmsg for {}...", task_name);
                match stem::syscall::socket::recvmsg(resp_fd, &mut msg_data, &mut msg_fds) {
                    Ok((n, n_fds)) => {
                        info!("SPROUT: Recvmsg SUCCESS from {}: n={}, nfds={}", task_name, n, n_fds);
                        if n == 0 && n_fds == 0 { break; }
                        
                        let bundled_fd = if n_fds > 0 { msg_fds[0] } else { 0 };
                        let parsed = abi::display_driver_protocol::parse_message(&msg_data[..n]);
                        if let Some((header, payload)) = parsed {
                            if header.msg_type == abi::supervisor_protocol::MSG_BIND_READY {
                                info!("SPROUT: BIND_READY from {} (bundle_fd={})", task_name, bundled_fd);
                                self.handle_bind_ready(&task_name, drv_req_write, payload, bundled_fd);
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

    fn handle_bind_ready(
        &mut self,
        task_name: &str,
        drv_req_write: ChannelHandle,
        payload: &[u8],
        bundled_fd: u32,
    ) {
        use abi::supervisor_protocol::{self, MSG_BIND_ASSIGNED, MSG_BIND_FAILED, classes};
        use stem::syscall::{channel_send_all, vfs_mount, vfs::vfs_close};

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
                    let _ = channel_send_all(req_write, &reply_buf[..total_len]);
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
                            let _ = channel_send_all(drv_req_write, &reply_buf[..total_len]);
                        }
                    }
                }
                Err(e) => {
                    warn!("SPROUT: Sovereign mount FAILED for {}: {:?}", task_name, e);
                    send_failed(
                        drv_req_write,
                        ready.bind_instance_id,
                        supervisor_protocol::errors::ERR_MOUNT_FAILED,
                        b"vfs_mount failed",
                    );
                }
            }
            // Close the bundled FD now that we're done with it (mount is established or failed).
            let _ = vfs_close(bundled_fd);
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
