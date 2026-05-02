extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};

use abi::syscall::vfs_flags::{O_RDONLY, O_RDWR};
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read};
use stem::{info, warn};

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

pub fn spawn_shell() -> Option<u64> {
    let shell_path = select_shell();
    info!("SPROUT: launching shell '{}'", shell_path);

    let open_console = || vfs_open("/dev/console", O_RDWR);
    let stdin_fd = match open_console() {
        Ok(fd) => fd,
        Err(err) => {
            warn!("SPROUT: failed to open /dev/console for shell stdin: {:?}", err);
            return None;
        }
    };
    let stdout_fd = match open_console() {
        Ok(fd) => fd,
        Err(err) => {
            warn!("SPROUT: failed to open /dev/console for shell stdout: {:?}", err);
            let _ = vfs_close(stdin_fd);
            return None;
        }
    };
    let stderr_fd = match open_console() {
        Ok(fd) => fd,
        Err(err) => {
            warn!("SPROUT: failed to open /dev/console for shell stderr: {:?}", err);
            let _ = vfs_close(stdin_fd);
            let _ = vfs_close(stdout_fd);
            return None;
        }
    };

    let mut env = BTreeMap::new();
    env.insert(b"SHELL".to_vec(), shell_path.as_bytes().to_vec());

    let result = stem::syscall::spawn_process_ex(
        &shell_path,
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
            info!("SPROUT: spawned shell '{}' (PID={})", shell_path, resp.child_tid);
            Some(resp.child_tid)
        }
        Err(err) => {
            warn!("SPROUT: failed to spawn shell '{}': {:?}", shell_path, err);
            None
        }
    }
}

pub fn spawn_bristle() -> Option<u64> {
    let path = "/bin/bristle";
    let argv: [&[u8]; 1] = [path.as_bytes()];
    let env = BTreeMap::new();
    let inherit = abi::types::stdio_mode::INHERIT;
    let null = abi::types::stdio_mode::NULL;

    match stem::syscall::spawn_process_ex(path, &argv, &env, null, inherit, inherit, 0, &[]) {
        Ok(resp) => {
            let pid = resp.child_tid;
            info!("SPROUT: spawned bristle (PID={})", pid);
            let _ = stem::thread::set_priority(pid, 3);
            Some(pid)
        }
        Err(err) => {
            warn!("SPROUT: failed to spawn bristle: {:?}", err);
            None
        }
    }
}
