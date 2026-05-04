extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};

use abi::errors::Errno;
use abi::syscall::vfs_flags::{O_RDONLY, O_RDWR};
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read};
use stem::{debug, info, warn};

const BLOOM_SPAWN_ATTEMPTS: usize = 3;
const BLOOM_SPAWN_RETRY_MS: u64 = 250;
const DEFAULT_PATH: &[u8] = b"/bin:/applications:/drivers";

fn file_exists(path: &str) -> bool {
    match vfs_open(path, O_RDONLY) {
        Ok(fd) => {
            let _ = vfs_close(fd);
            true
        }
        Err(_) => false,
    }
}

fn read_trimmed_text(path: &str) -> Option<String> {
    let fd = vfs_open(path, O_RDONLY).ok()?;
    let mut buf = [0u8; 256];
    let n = vfs_read(fd, &mut buf).ok()?;
    let _ = vfs_close(fd);
    if n == 0 {
        return None;
    }

    let s = core::str::from_utf8(&buf[..n]).ok()?.trim();
    if s.is_empty() { None } else { Some(s.to_string()) }
}

fn read_boot_cmdline() -> Option<String> {
    read_trimmed_text("/sys/boot/cmdline").or_else(|| read_trimmed_text("/dev/cmdline"))
}

pub fn select_shell() -> String {
    for cfg in ["/run/sprout/shell", "/etc/default/shell"] {
        if let Some(candidate) = read_trimmed_text(cfg) {
            if file_exists(&candidate) {
                return candidate;
            }
            warn!("SPROUT: ignoring shell override '{}' from {} (missing binary)", candidate, cfg);
        }
    }

    "/bin/sh".to_string()
}

fn spawn_shell_path(shell_path: &str, stdio_path: &str) -> Option<u64> {
    debug!("Launching shell '{}' on '{}'", shell_path, stdio_path);

    let open_stdio = || vfs_open(stdio_path, O_RDWR);
    let stdin_fd = match open_stdio() {
        Ok(fd) => fd,
        Err(err) => {
            warn!("Failed to open {} for shell stdin: {:?}", stdio_path, err);
            return None;
        }
    };
    let stdout_fd = match open_stdio() {
        Ok(fd) => fd,
        Err(err) => {
            warn!("Failed to open {} for shell stdout: {:?}", stdio_path, err);
            let _ = vfs_close(stdin_fd);
            return None;
        }
    };
    let stderr_fd = match open_stdio() {
        Ok(fd) => fd,
        Err(err) => {
            warn!("Failed to open {} for shell stderr: {:?}", stdio_path, err);
            let _ = vfs_close(stdin_fd);
            let _ = vfs_close(stdout_fd);
            return None;
        }
    };

    let mut env = BTreeMap::new();
    env.insert(b"SHELL".to_vec(), shell_path.as_bytes().to_vec());
    env.insert(b"PATH".to_vec(), DEFAULT_PATH.to_vec());

    let result = stem::syscall::spawn_process_ex(
        shell_path,
        &[shell_path.as_bytes()],
        &env,
        abi::types::stdio_mode::handle(stdin_fd),
        abi::types::stdio_mode::handle(stdout_fd),
        abi::types::stdio_mode::handle(stderr_fd),
        0,
        &[],
    );

    let _ = vfs_close(stdin_fd);
    let _ = vfs_close(stdout_fd);
    let _ = vfs_close(stderr_fd);

    match result {
        Ok(resp) => {
            debug!("Spawned shell '{}' with PID {}", shell_path, resp.child_tid);
            Some(resp.child_tid)
        }
        Err(err) => {
            warn!("SPROUT: failed to spawn shell '{}': {:?}", shell_path, err);
            None
        }
    }
}

pub fn spawn_shell() -> Option<u64> {
    let shell_path = select_shell();
    spawn_shell_path(&shell_path, "/dev/console")
}

pub fn spawn_kernel_terminal_shell() -> Option<u64> {
    let shell_path = select_shell();
    info!("Kernel terminal requested; launching shell on /dev/tty0");
    spawn_shell_path(&shell_path, "/dev/tty0")
}

pub fn spawn_safe_shell() -> Option<u64> {
    info!("Safe shell requested; launching /bin/sh on /dev/tty0");
    spawn_shell_path("/bin/sh", "/dev/tty0")
}

pub fn spawn_safe_serial_shell() -> Option<u64> {
    info!("Safe shell requested; launching /bin/sh on /dev/console");
    spawn_shell_path("/bin/sh", "/dev/console")
}

pub fn safe_shell_requested() -> bool {
    let Some(cmdline) = read_boot_cmdline() else {
        return false;
    };

    for token in cmdline.split_ascii_whitespace() {
        if token.eq_ignore_ascii_case("sprout.safe") {
            return true;
        }

        let Some(value) = token.strip_prefix("sprout.safe=") else {
            continue;
        };

        if value.eq_ignore_ascii_case("sh")
            || value.eq_ignore_ascii_case("shell")
            || value.eq_ignore_ascii_case("safe-shell")
        {
            return true;
        }

        if value == "0"
            || value.eq_ignore_ascii_case("false")
            || value.eq_ignore_ascii_case("no")
            || value.eq_ignore_ascii_case("off")
        {
            return false;
        }
    }

    false
}

pub fn kernel_terminal_requested() -> bool {
    let Some(cmdline) = read_boot_cmdline() else {
        return false;
    };

    for token in cmdline.split_ascii_whitespace() {
        if token.eq_ignore_ascii_case("kernel.terminal") {
            return true;
        }

        let Some(value) = token.strip_prefix("kernel.terminal=") else {
            continue;
        };
        if value == "1"
            || value.eq_ignore_ascii_case("true")
            || value.eq_ignore_ascii_case("yes")
            || value.eq_ignore_ascii_case("on")
        {
            return true;
        }
        if value == "0"
            || value.eq_ignore_ascii_case("false")
            || value.eq_ignore_ascii_case("no")
            || value.eq_ignore_ascii_case("off")
        {
            return false;
        }
    }

    false
}

pub fn spawn_bristle() -> Option<u64> {
    let path = "/services/bristle";
    let argv: [&[u8]; 1] = [path.as_bytes()];
    let env = BTreeMap::new();
    let inherit = abi::types::stdio_mode::INHERIT;
    let null = abi::types::stdio_mode::NULL;

    match stem::syscall::spawn_process_ex(path, &argv, &env, null, inherit, inherit, 0, &[]) {
        Ok(resp) => {
            let pid = resp.child_tid;
            debug!("Spawned bristle with PID {}", pid);
            let _ = stem::thread::set_priority(pid, 3);
            Some(pid)
        }
        Err(err) => {
            warn!("SPROUT: failed to spawn bristle: {:?}", err);
            None
        }
    }
}

pub fn spawn_chime() -> Option<u64> {
    let path = "/bin/chime";
    let argv: [&[u8]; 1] = [path.as_bytes()];
    let env = BTreeMap::new();
    let null = abi::types::stdio_mode::NULL;

    match stem::syscall::spawn_process_ex(path, &argv, &env, null, null, null, 0, &[]) {
        Ok(resp) => {
            let pid = resp.child_tid;
            debug!("Spawned startup chime with PID {}", pid);
            match stem::thread::set_priority(pid, 3) {
                Ok(()) => debug!("Startup chime priority set to 3"),
                Err(err) => {
                    warn!("SPROUT: failed to set startup chime realtime priority: {:?}", err)
                }
            }
            Some(pid)
        }
        Err(err) => {
            warn!("SPROUT: failed to spawn startup chime: {:?}", err);
            None
        }
    }
}

pub fn spawn_bloom() -> Option<u64> {
    let path = "/services/bloom";
    let argv: [&[u8]; 1] = [path.as_bytes()];
    let env = BTreeMap::new();
    let inherit = abi::types::stdio_mode::INHERIT;
    let null = abi::types::stdio_mode::NULL;

    for attempt in 1..=BLOOM_SPAWN_ATTEMPTS {
        match stem::syscall::spawn_process_ex(path, &argv, &env, null, inherit, inherit, 0, &[]) {
            Ok(resp) => {
                let pid = resp.child_tid;
                debug!("Spawned bloom with PID {}", pid);
                let _ = stem::thread::set_priority(pid, 2);
                return Some(pid);
            }
            Err(err @ (Errno::ETIMEDOUT | Errno::EIO | Errno::EPIPE))
                if attempt < BLOOM_SPAWN_ATTEMPTS =>
            {
                warn!(
                    "SPROUT: failed to spawn bloom on attempt {}/{}: {:?}",
                    attempt, BLOOM_SPAWN_ATTEMPTS, err
                );
                stem::sleep_ms(BLOOM_SPAWN_RETRY_MS);
            }
            Err(err) => {
                warn!("SPROUT: failed to spawn bloom: {:?}", err);
                return None;
            }
        }
    }

    None
}

pub fn spawn_netd() -> Option<u64> {
    let path = "/services/netd";
    let argv: [&[u8]; 1] = [path.as_bytes()];
    let env = BTreeMap::new();
    let inherit = abi::types::stdio_mode::INHERIT;
    let null = abi::types::stdio_mode::NULL;

    match stem::syscall::spawn_process_ex(path, &argv, &env, null, inherit, inherit, 0, &[]) {
        Ok(resp) => {
            let pid = resp.child_tid;
            debug!("Spawned netd with PID {}", pid);
            Some(pid)
        }
        Err(err) => {
            warn!("SPROUT: failed to spawn netd: {:?}", err);
            None
        }
    }
}

pub fn spawn_blossom() -> Option<u64> {
    let path = "/services/blossom";
    let argv: [&[u8]; 1] = [path.as_bytes()];
    let env = BTreeMap::new();
    let inherit = abi::types::stdio_mode::INHERIT;
    let null = abi::types::stdio_mode::NULL;

    match stem::syscall::spawn_process_ex(path, &argv, &env, null, inherit, inherit, 0, &[]) {
        Ok(resp) => {
            let pid = resp.child_tid;
            debug!("Spawned blossom with PID {}", pid);
            Some(pid)
        }
        Err(err) => {
            warn!("SPROUT: failed to spawn blossom: {:?}", err);
            None
        }
    }
}

/// Spawn the ACPI namespace service (`/drivers/acpid`).
///
/// acpid parses firmware ACPI tables and mounts a device inventory at
/// `/services/acpi`.  It is spawned early, before cambium, so that driver
/// probing can consume the namespace on first boot.
///
/// Missing binary is non-fatal — non-ACPI platforms simply skip this step.
pub fn spawn_acpid() -> Option<u64> {
    let path = "/drivers/acpid";
    let argv: [&[u8]; 1] = [path.as_bytes()];
    let env = BTreeMap::new();
    let null = abi::types::stdio_mode::NULL;
    let inherit = abi::types::stdio_mode::INHERIT;

    match stem::syscall::spawn_process_ex(path, &argv, &env, null, inherit, inherit, 0, &[]) {
        Ok(resp) => {
            let pid = resp.child_tid;
            debug!("Spawned acpid with PID {}", pid);
            Some(pid)
        }
        Err(Errno::ENOENT) => {
            debug!("acpid binary not found; skipping ACPI service");
            None
        }
        Err(err) => {
            warn!("Failed to spawn acpid: {:?}", err);
            None
        }
    }
}
