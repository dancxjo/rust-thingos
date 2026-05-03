extern crate alloc;

use alloc::collections::BTreeMap;

use stem::kinds::{
    KIND_ID_THINGOS_SHUTDOWN_READY, KIND_ID_THINGOS_SHUTDOWN_REQUEST, ShutdownReadyV1,
    ShutdownRequestV1,
};
use stem::service_loop::{ServiceEvent, ServiceLoop};
use stem::syscall::message::{KindId, msg_send};
use stem::{debug, info, warn};

use crate::pipelines::{
    kernel_terminal_requested, safe_shell_requested, spawn_bloom, spawn_blossom, spawn_bristle,
    spawn_chime, spawn_kernel_terminal_shell, spawn_safe_shell, spawn_shell,
};

const SHELL_HEADSTART_MS: u64 = 50;
const INBOX_MAX_PAYLOAD: usize = 256;
const MOUNT_BIN: &str = "/bin/mount";

pub struct Supervisor;

impl Supervisor {
    pub fn new() -> Self {
        Self
    }

    pub fn run_forever(&mut self) -> ! {
        info!("Starting session supervisor...");

        if safe_shell_requested() {
            let shell_pid = spawn_safe_shell();
            self.wait_for_shell(shell_pid);
            stem::shutdown();
        }

        let kernel_terminal = kernel_terminal_requested();
        let shell_pid = if kernel_terminal { spawn_kernel_terminal_shell() } else { spawn_shell() };
        let _chime_pid = spawn_chime();
        stem::sleep_ms(SHELL_HEADSTART_MS);
        info!("Continuing supervisor startup");

        let cambium_pid = if kernel_terminal {
            info!("Kernel terminal requested; skipping desktop handoff");
            None
        } else {
            // Bristle must come up before Cambium can launch input drivers that
            // publish through `/run/bristle/*`.
            if spawn_bristle().is_some() {
                debug!("Bristle launched; disabling kernel framebuffer terminal");
                stem::syscall::console_disable();
            }
            self.spawn_cambium()
        };

        self.activate_boot_roots();
        if !kernel_terminal {
            // Now that the root filesystem is populated (display drivers, fonts, etc),
            // we can launch the bloom compositor and the Blossom shell that
            // owns window-management UI over Wayland.
            let _bloom_pid = spawn_bloom();
            let _blossom_pid = spawn_blossom();
        }

        self.wait_for_shell(shell_pid);

        self.request_cambium_shutdown_and_wait(cambium_pid);
        stem::shutdown();
    }

    fn wait_for_shell(&mut self, shell_pid: Option<u64>) {
        match shell_pid {
            Some(pid) => {
                stem::debug!("SPROUT: waiting for shell PID {} to exit", pid);
                match stem::syscall::waitpid(pid as i64, 0) {
                    Ok((reaped_pid, status)) => {
                        debug!("Shell PID {} exited with status {:#x}", reaped_pid, status);
                    }
                    Err(err) => {
                        warn!("SPROUT: waitpid for shell PID {} failed: {:?}", pid, err);
                    }
                }
            }
            None => warn!("SPROUT: shell did not spawn"),
        }
    }

    fn spawn_cambium(&mut self) -> Option<u64> {
        match stem::syscall::spawn_process("/services/cambium", 0) {
            Ok(pid) => {
                debug!("Spawned cambium with PID {}", pid);
                Some(pid)
            }
            Err(err) => {
                warn!("SPROUT: failed to spawn cambium: {:?}", err);
                None
            }
        }
    }

    fn activate_boot_roots(&mut self) {
        let argv: [&[u8]; 2] = [MOUNT_BIN.as_bytes(), b"--roots"];
        let env = BTreeMap::new();
        let null = abi::types::stdio_mode::NULL;
        let inherit = abi::types::stdio_mode::INHERIT;

        let pid = match stem::syscall::spawn_process_ex(
            MOUNT_BIN,
            &argv,
            &env,
            null,
            null,
            inherit,
            0,
            &[],
        ) {
            Ok(resp) => resp.child_tid,
            Err(err) => {
                warn!("SPROUT: failed to spawn /etc/roots activator: {:?}", err);
                return;
            }
        };

        match stem::syscall::waitpid(pid as i64, 0) {
            Ok((_pid, 0)) => info!("Root filesystem activation complete"),
            Ok((_pid, status)) => {
                warn!("SPROUT: /etc/roots activation exited status {:#x}", status)
            }
            Err(err) => warn!("SPROUT: waitpid for /etc/roots activation failed: {:?}", err),
        }
    }

    fn request_cambium_shutdown_and_wait(&mut self, cambium_pid: Option<u64>) {
        let Some(cambium_pid) = cambium_pid else {
            warn!("SPROUT: cambium is not running; halting without shutdown handshake");
            return;
        };

        let request = ShutdownRequestV1::new(stem::syscall::getpid(), 0);
        debug!("Sending shutdown request to cambium PID {}", cambium_pid);
        if let Err(err) = msg_send(
            cambium_pid as u32,
            KindId(KIND_ID_THINGOS_SHUTDOWN_REQUEST),
            request.as_bytes(),
        ) {
            warn!(
                "SPROUT: failed to send shutdown request to cambium PID {}: {:?}",
                cambium_pid, err
            );
            return;
        }

        let mut svc = match ServiceLoop::new(INBOX_MAX_PAYLOAD) {
            Ok(svc) => svc,
            Err(err) => {
                warn!("SPROUT: cannot wait for cambium shutdown-ready: {:?}", err);
                return;
            }
        };

        loop {
            match svc.next_event(None) {
                Ok(ServiceEvent::Message { kind, payload, .. })
                    if kind.0 == KIND_ID_THINGOS_SHUTDOWN_READY =>
                {
                    let Some(ready) = ShutdownReadyV1::from_bytes(payload) else {
                        warn!(
                            "SPROUT: ignoring malformed cambium shutdown-ready payload ({} bytes)",
                            payload.len()
                        );
                        continue;
                    };
                    if ready.pid as u64 != cambium_pid {
                        stem::debug!(
                            "SPROUT: ignoring shutdown-ready from unexpected PID {}",
                            ready.pid
                        );
                        continue;
                    }
                    info!(
                        "SPROUT: cambium ready for shutdown status={} active_children={}",
                        ready.status, ready.active_children
                    );
                    break;
                }
                Ok(ServiceEvent::Message { kind, .. }) => {
                    stem::debug!("SPROUT: ignored inbox message while shutting down: {:?}", kind);
                }
                Ok(ServiceEvent::Ready { .. }) | Ok(ServiceEvent::Timeout) => {}
                Ok(ServiceEvent::InboxClosed) => {
                    warn!("SPROUT: inbox closed while waiting for cambium shutdown-ready");
                    return;
                }
                Err(err) => {
                    warn!("SPROUT: error waiting for cambium shutdown-ready: {:?}", err);
                    return;
                }
            }
        }

        match stem::syscall::waitpid(cambium_pid as i64, 0) {
            Ok((pid, status)) => {
                debug!("Cambium PID {} exited with status {:#x}", pid, status)
            }
            Err(err) => warn!("SPROUT: waitpid for cambium PID {} failed: {:?}", cambium_pid, err),
        }
    }
}
