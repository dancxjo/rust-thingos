#![no_std]
use alloc::string::ToString;
use core::default::Default;
extern crate alloc;
use alloc::sync::Arc;
use alloc::vec::Vec;

use abi::display_driver_protocol::{FB_INFO_PAYLOAD_SIZE, FbInfoPayload};
use abi::schema::{keys, kinds};
use abi::syscall::vfs_flags::O_RDONLY;
use spin::Mutex;
use stem::abi::driver_ctx::DriverCtx;
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read};
use stem::syscall::{PortHandle, port_create};
use stem::{debug, info, warn};

use crate::task::{ManagedTask, TaskKind};

fn file_exists(path: &str) -> bool {
    match vfs_open(path, O_RDONLY) {
        Ok(fd) => {
            let _ = vfs_close(fd);
            true
        }
        Err(_) => false,
    }
}

fn read_trimmed_text(path: &str) -> Option<alloc::string::String> {
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

pub fn select_serial_shell() -> alloc::string::String {
    // Prefer runtime override, then system default, then known built-in fallback.
    for cfg in ["/run/sprout/shell", "/etc/default/shell"] {
        if let Some(candidate) = read_trimmed_text(cfg) {
            if file_exists(&candidate) {
                return candidate;
            }
            warn!("SPROUT: Ignoring shell override '{}' from {} (missing binary)", candidate, cfg);
        }
    }

    for candidate in ["/bin/sh", "/bin/smallsh"] {
        if file_exists(candidate) {
            return candidate.to_string();
        }
    }

    "/bin/sh".to_string()
}

fn ensure_session_roots() {
    use stem::syscall::vfs::vfs_mkdir;
    let _ = vfs_mkdir("/session");
    let _ = vfs_mkdir("/session/seat0");
    let _ = vfs_mkdir("/session/seat0/presences");
    let _ = vfs_mkdir("/session/seat0/keyboard");
    let _ = vfs_mkdir("/session/seat0/pointer");
    let _ = vfs_mkdir("/session/display");
}

#[derive(Clone, Copy, Debug)]
pub struct DisplayHandles {
    pub drv_req_write: PortHandle,
    pub drv_resp_read: PortHandle,
    pub bs_id: u32,
    /// Which display backend was selected
    pub backend_name: &'static str,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
}

fn find_sys_device(class_prefix: &str) -> Option<alloc::string::String> {
    let fd = match vfs_open("/sys/devices", O_RDONLY) {
        Ok(fd) => fd,
        Err(_) => return None,
    };

    let mut buf = [0u8; 4096];
    let n = match stem::syscall::vfs::vfs_readdir(fd, &mut buf) {
        Ok(n) => n,
        Err(e) => {
            warn!("SPROUT: readdir(/sys/devices) failed: {:?}", e);
            let _ = vfs_close(fd);
            return None;
        }
    };
    let _ = vfs_close(fd);

    debug!("SPROUT: readdir found {} bytes", n);

    let mut offset = 0usize;
    while offset < n {
        if buf[offset] == 0 {
            offset += 1;
            continue;
        }

        let mut end = offset;
        while end < n && buf[end] != 0 {
            end += 1;
        }

        if let Ok(name) = core::str::from_utf8(&buf[offset..end]) {
            debug!("SPROUT:   Checking entry at {}: '{}'", offset, name);
            if name.starts_with("pci-") {
                let class_path = alloc::format!("/sys/devices/{}/class", name);
                if let Ok(class_fd) = vfs_open(&class_path, O_RDONLY) {
                    let mut class_buf = [0u8; 16];
                    if let Ok(cn) = vfs_read(class_fd, &mut class_buf) {
                        let class_str = core::str::from_utf8(&class_buf[..cn]).unwrap_or("");
                        debug!("SPROUT: Checked device {} class='{}'", name, class_str.trim());
                        if class_str.trim().starts_with(class_prefix) {
                            let _ = vfs_close(class_fd);
                            return Some(alloc::format!("/sys/devices/{}", name));
                        }
                    } else {
                        debug!("SPROUT: Failed to read {}", class_path);
                    }
                    let _ = vfs_close(class_fd);
                } else {
                    debug!("SPROUT: Failed to open {}", class_path);
                }
            }
        }
        offset = end.saturating_add(1);
    }
    None
}

fn find_sys_device_with_vendor(
    class_prefix: &str,
    vendor_prefix: &str,
) -> Option<alloc::string::String> {
    let fd = match vfs_open("/sys/devices", O_RDONLY) {
        Ok(fd) => fd,
        Err(_) => return None,
    };

    let mut buf = [0u8; 4096];
    let n = match stem::syscall::vfs::vfs_readdir(fd, &mut buf) {
        Ok(n) => n,
        Err(e) => {
            debug!("SPROUT: readdir(/sys/devices) failed: {:?}", e);
            let _ = vfs_close(fd);
            return None;
        }
    };
    let _ = vfs_close(fd);

    let mut offset = 0usize;
    while offset < n {
        if buf[offset] == 0 {
            offset += 1;
            continue;
        }

        let mut end = offset;
        while end < n && buf[end] != 0 {
            end += 1;
        }

        if let Ok(name) = core::str::from_utf8(&buf[offset..end]) {
            if name.starts_with("pci-") {
                let class_path = alloc::format!("/sys/devices/{}/class", name);
                let vendor_path = alloc::format!("/sys/devices/{}/vendor", name);

                let class_matches = if let Ok(class_fd) = vfs_open(&class_path, O_RDONLY) {
                    let mut class_buf = [0u8; 16];
                    let matched = if let Ok(cn) = vfs_read(class_fd, &mut class_buf) {
                        let class_str = core::str::from_utf8(&class_buf[..cn]).unwrap_or("");
                        class_str.trim().starts_with(class_prefix)
                    } else {
                        false
                    };
                    let _ = vfs_close(class_fd);
                    matched
                } else {
                    false
                };

                if !class_matches {
                    offset = end.saturating_add(1);
                    continue;
                }

                let vendor_matches = if let Ok(vendor_fd) = vfs_open(&vendor_path, O_RDONLY) {
                    let mut vendor_buf = [0u8; 16];
                    let matched = if let Ok(vn) = vfs_read(vendor_fd, &mut vendor_buf) {
                        let vendor_str = core::str::from_utf8(&vendor_buf[..vn]).unwrap_or("");
                        vendor_str.trim().starts_with(vendor_prefix)
                    } else {
                        false
                    };
                    let _ = vfs_close(vendor_fd);
                    matched
                } else {
                    false
                };

                if vendor_matches {
                    return Some(alloc::format!("/sys/devices/{}", name));
                }
            }
        }

        offset = end.saturating_add(1);
    }

    None
}

fn probe_bootfb_vfs() -> Option<(u32, u32, u32, u32)> {
    let fd = match vfs_open("/dev/fb0", O_RDONLY) {
        Ok(fd) => fd,
        Err(e) => {
            debug!("SPROUT: open(/dev/fb0) failed: {:?}", e);
            return None;
        }
    };
    let mut payload = FbInfoPayload {
        device_handle: 0,
        width: 0,
        height: 0,
        stride: 0,
        bpp: 32,
        format: 0,
        _reserved: 0,
    };
    let slice = unsafe {
        core::slice::from_raw_parts_mut(&mut payload as *mut _ as *mut u8, FB_INFO_PAYLOAD_SIZE)
    };
    let n = match vfs_read(fd, slice) {
        Ok(n) => n,
        Err(e) => {
            let _ = vfs_close(fd);
            debug!("SPROUT: read(/dev/fb0) failed: {:?}", e);
            return None;
        }
    };
    let _ = vfs_close(fd);
    if n < FB_INFO_PAYLOAD_SIZE || payload.width == 0 || payload.height == 0 || payload.stride == 0
    {
        debug!(
            "SPROUT: /dev/fb0 payload invalid: n={} width={} height={} stride={} format={}",
            n, payload.width, payload.height, payload.stride, payload.format
        );
        return None;
    }
    Some((payload.width, payload.height, payload.stride, payload.format))
}

// Storage, Network, and Audio are now handled by cambium

pub fn setup_display_pipeline(
    _shared_tasks: Arc<Mutex<Vec<ManagedTask>>>,
    _supervisor_port: stem::syscall::PortHandle,
    _bind_instance_id: u64,
    _force_bootfb: bool,
) -> Option<DisplayHandles> {
    info!("SPROUT: Display driver startup is disabled (network-only mode)");
    None
}

pub fn setup_terminal(
    shared_tasks: Arc<Mutex<Vec<ManagedTask>>>,
    display: Option<DisplayHandles>,
    _input: InputHandles,
) {
    debug!("SPROUT: Setting up Terminal...");
    ensure_session_roots();

    let Some(display) = display else {
        debug!("SPROUT: Cannot setup terminal without display!");
        return;
    };

    let boot_size = 4096;
    let boot_fd = stem::syscall::memfd_create("terminal.boot", boot_size).unwrap_or(0);

    if boot_fd != 0 {
        use abi::vm::{VmBacking, VmMapFlags, VmMapReq, VmProt};
        let req = VmMapReq {
            addr_hint: 0,
            len: boot_size,
            prot: VmProt::READ | VmProt::WRITE | VmProt::USER,
            flags: VmMapFlags::empty(),
            backing: VmBacking::File { thing: boot_fd, offset: 0 },
        };
        if let Ok(resp) = stem::syscall::vm_map(&req) {
            let ptr = resp.addr;
            let slice = unsafe { core::slice::from_raw_parts_mut(ptr as *mut u32, boot_size / 4) };
            slice[0] = 0xB100AA01; // Magic
            slice[1] = display.drv_req_write as u32;
            slice[2] = display.drv_resp_read as u32;
            slice[4] = display.bs_id;
            debug!(
                "SPROUT: Bootstrapping terminal via memfd {}: req_w={}, resp_r={}, bs_id={}",
                boot_fd, slice[1], slice[2], slice[4]
            );
        }
    }

    let term_arg = boot_fd as u32;

    match stem::syscall::spawn_process("/bin/terminal", term_arg as usize) {
        Ok(pid) => {
            debug!("SPROUT: Spawned terminal (PID={})", pid);
            let _ = stem::thread::set_priority(pid, 2);
            let mut tasks = shared_tasks.lock();
            tasks.push(ManagedTask {
                name: "terminal".to_string(),
                kind: TaskKind::App,
                module_path: "/bin/terminal".to_string(),
                pid: Some(pid),
                restarts: 0,
                spawn_arg: term_arg as usize,
                bind_instance_id: 0,
                drv_req_write: 0,
                drv_resp_read: 0,
                boot_req_read: 0,
                boot_resp_write: 0,
                resp_fd: None,
            });
        }
        Err(e) => {
            stem::error!("SPROUT: Failed to spawn terminal: {:?}", e);
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct InputHandles {
    pub bloom_evt_read: PortHandle,
    pub evt_input_echo_read: PortHandle,
}

pub fn setup_input_broker(_shared_tasks: Arc<Mutex<Vec<ManagedTask>>>) -> InputHandles {
    info!("SPROUT: Input driver startup is disabled (network-only mode)");
    InputHandles { bloom_evt_read: 0, evt_input_echo_read: 0 }
}

/// Launch the userspace network stack.
///
/// The NIC driver is owned by cambium; `netd` can start immediately and will
/// block until a `/dev/net/virtioN` provider appears.
fn spawn_netd(shared_tasks: Arc<Mutex<Vec<ManagedTask>>>) {
    {
        let tasks = shared_tasks.lock();
        if tasks.iter().any(|t| t.name == "netd" && t.pid.is_some()) {
            info!("SPROUT: netd already running; skipping duplicate spawn");
            return;
        }
    }

    info!("SPROUT: Launching netd...");
    match stem::syscall::spawn_process("/bin/netd", 0) {
        Ok(pid) => {
            info!("SPROUT: Spawned netd (PID={})", pid);
            let _ = stem::thread::set_priority(pid, 2);
            let mut tasks = shared_tasks.lock();
            tasks.push(ManagedTask {
                name: "netd".to_string(),
                kind: TaskKind::Service("svc.net".to_string()),
                module_path: "/bin/netd".to_string(),
                pid: Some(pid),
                restarts: 0,
                spawn_arg: 0,
                bind_instance_id: 0,
                drv_req_write: 0,
                drv_resp_read: 0,
                boot_req_read: 0,
                boot_resp_write: 0,
                resp_fd: None,
            });
        }
        Err(e) => {
            warn!("SPROUT: Failed to spawn netd: {:?}", e);
        }
    }
}

fn apply_fstab_mounts() {
    info!("SPROUT: Applying mounts from /etc/fstab...");
    let argv: [&[u8]; 2] = [b"/bin/mount", b"-a"];
    match stem::syscall::spawn_process_ex(
        "/bin/mount",
        &argv,
        &alloc::collections::BTreeMap::new(),
        abi::types::stdio_mode::INHERIT,
        abi::types::stdio_mode::INHERIT,
        abi::types::stdio_mode::INHERIT,
        0,
        &[],
    ) {
        Ok(resp) => info!("SPROUT: Spawned mount -a (PID={})", resp.child_tid),
        Err(e) => {
            warn!("SPROUT: Failed to spawn mount -a: {:?}", e);
        }
    }
}

pub fn setup_network_stack(shared_tasks: Arc<Mutex<Vec<ManagedTask>>>) {
    let netd_tasks = shared_tasks.clone();
    let _ = stem::thread::spawn_task(move || {
        // Let critical boot tasks and driver binding start first, then launch
        // the network service without blocking the rest of boot.
        stem::sleep_ms(500);
        spawn_netd(netd_tasks);
    });

    let _ = stem::thread::spawn_task(move || {
        stem::sleep_ms(500);
        info!("SPROUT: Waiting for /net/tcp/new before applying /etc/fstab mounts...");
        loop {
            if file_exists("/net/tcp/new") {
                apply_fstab_mounts();
                break;
            }
            stem::sleep_ms(100);
        }
    });
}

pub fn setup_network_apps(shared_tasks: Arc<Mutex<Vec<ManagedTask>>>) {
    debug!("SPROUT: Setting up network apps...");

    match stem::syscall::spawn_process("/bin/nectar", 0) {
        Ok(pid) => {
            debug!("SPROUT: Spawned nectar (PID={})", pid);
            let _ = stem::thread::set_priority(pid, 2);
            let mut tasks = shared_tasks.lock();
            tasks.push(ManagedTask {
                name: "nectar".to_string(),
                kind: TaskKind::Service("svc.nectar".to_string()),
                module_path: "/bin/nectar".to_string(),
                pid: Some(pid),
                restarts: 0,
                spawn_arg: 0,
                bind_instance_id: 0,
                drv_req_write: 0,
                drv_resp_read: 0,
                boot_req_read: 0,
                boot_resp_write: 0,
                resp_fd: None,
            });
        }
        Err(e) => {
            warn!("SPROUT: Failed to spawn nectar: {:?}", e);
        }
    }

    match stem::syscall::spawn_process("/bin/fetchd", 0) {
        Ok(pid) => {
            debug!("SPROUT: Spawned fetchd (PID={})", pid);
            let _ = stem::thread::set_priority(pid, 2);
            let mut tasks = shared_tasks.lock();
            tasks.push(ManagedTask {
                name: "fetchd".to_string(),
                kind: TaskKind::App,
                module_path: "/bin/fetchd".to_string(),
                pid: Some(pid),
                restarts: 0,
                spawn_arg: 0,
                bind_instance_id: 0,
                drv_req_write: 0,
                drv_resp_read: 0,
                boot_req_read: 0,
                boot_resp_write: 0,
                resp_fd: None,
            });
        }
        Err(e) => {
            warn!("SPROUT: Failed to spawn fetchd: {:?}", e);
        }
    }

    apply_fstab_mounts();
}

pub fn setup_taskman_service(_shared_tasks: Arc<Mutex<Vec<ManagedTask>>>) {
    // Taskman removed
}

pub fn setup_ui_services(shared_tasks: Arc<Mutex<Vec<ManagedTask>>>) {
    // Keep `placed` deterministic (window placement policy source), then fan out
    // independent UI services in parallel so startup is not serialized on one lane.
    // spawn_ui_service(shared_tasks, "/bin/placed", "svc.placed", 2);
}

pub fn setup_font_service(_shared_tasks: Arc<Mutex<Vec<ManagedTask>>>) {
    // Font handling is integrated into Bloom directly
}

fn spawn_ui_service(
    shared_tasks: Arc<Mutex<Vec<ManagedTask>>>,
    name: &str,
    service: &str,
    priority: usize,
) {
    {
        let tasks = shared_tasks.lock();
        if tasks.iter().any(|t| t.name == name && t.pid.is_some()) {
            return;
        }
    }

    match stem::syscall::spawn_process(name, 0) {
        Ok(pid) => {
            debug!("SPROUT: Spawned {} (PID={})", &name[1..], pid);
            let _ = stem::thread::set_priority(pid, priority);
            let mut tasks = shared_tasks.lock();
            tasks.push(ManagedTask {
                name: name.to_string(),
                kind: TaskKind::Service(service.to_string()),
                module_path: name.to_string(),
                pid: Some(pid),
                restarts: 0,
                spawn_arg: 0,
                bind_instance_id: 0,
                drv_req_write: 0,
                drv_resp_read: 0,
                boot_req_read: 0,
                boot_resp_write: 0,
                resp_fd: None,
            });
        }
        Err(e) => {
            warn!("SPROUT: Failed to spawn {}: {:?}", &name[1..], e);
        }
    }
}

/// Maximum time to wait for the audio VFS node to appear after spawning the driver.
const AUDIO_DEVICE_TIMEOUT_NS: u64 = 5_000_000_000; // 5 seconds

/// Poll interval while waiting for `/dev/audio/card0/out0` to appear.
const AUDIO_POLL_INTERVAL_MS: u64 = 100;

/// Probe for an audio device, spawn the right driver, wait for the VFS node
/// to appear, then launch the chime to play the start-up chime.
///
/// Detection order (first match wins):
///   1. VirtIO sound — PCI class 0x0401xx **and** vendor 0x1af4
///   2. Intel HDA    — PCI class 0x0403xx
pub fn setup_audio_stack(shared_tasks: Arc<Mutex<Vec<ManagedTask>>>) {
    let _ = shared_tasks;
    info!("SPROUT: Audio driver startup is disabled (network-only mode)");
}

pub fn setup_graphics_stack(
    shared_tasks: Arc<Mutex<Vec<ManagedTask>>>,
    _display: Option<DisplayHandles>,
    _input: InputHandles,
) {
    let _ = shared_tasks;
    info!("SPROUT: Bloom launch is temporarily disabled in setup_graphics_stack");
}

pub fn setup_serial_shell(shared_tasks: Arc<Mutex<Vec<ManagedTask>>>) {
    debug!("SPROUT: Setting up serial shell on /dev/console...");
    let shell_path = select_serial_shell();

    let console_fd =
        match stem::syscall::vfs::vfs_open("/dev/console", abi::syscall::vfs_flags::O_RDWR) {
            Ok(fd) => fd,
            Err(e) => {
                warn!("SPROUT: Failed to open /dev/console for shell: {:?}", e);
                return;
            }
        };

    match stem::syscall::spawn_process_ex(
        &shell_path,
        &[shell_path.as_bytes()],
        &alloc::collections::BTreeMap::new(),
        abi::types::stdio_mode::thing(console_fd), // stdin
        abi::types::stdio_mode::thing(console_fd), // stdout
        abi::types::stdio_mode::thing(console_fd), // stderr
        0,
        &[],
    ) {
        Ok(resp) => {
            debug!("SPROUT: Spawned serial shell '{}' (PID={})", shell_path, resp.child_tid);
            let _ = vfs_close(console_fd);
            let mut tasks = shared_tasks.lock();
            tasks.push(ManagedTask {
                name: "shell".to_string(),
                kind: TaskKind::App,
                module_path: shell_path,
                pid: Some(resp.child_tid),
                restarts: 0,
                spawn_arg: 0,
                bind_instance_id: 0,
                drv_req_write: 0,
                drv_resp_read: 0,
                boot_req_read: 0,
                boot_resp_write: 0,
                resp_fd: None,
            });
        }
        Err(e) => {
            warn!("SPROUT: Failed to spawn serial shell: {:?}", e);
            let _ = vfs_close(console_fd);
        }
    }
}
