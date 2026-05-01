use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::input::{
    WindowInfo, drag_pointer, find_window, move_pointer_to, parse_wayland_windows, qmp_left_button,
    qmp_mouse_rel, send_combo, send_key_tap, send_qmp_sequence, type_serial_command,
};
use crate::world::{DESKTOP_READY_SIGNALS, ThingOsWorld, missing_required_signals};

pub type HunterResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

// ---------------------------------------------------------------------------
// Boot-phase state machine
// ---------------------------------------------------------------------------

/// Serial-log substrings that confirm the Thing-OS kernel is executing.
/// Any one match is sufficient.
const KERNEL_MARKERS: &[&str] = &[
    "thing-os kernel",
    "Initializing SIMD",
    "Frame allocator initialized",
    "Initializing tasking",
    "Scheduler initialized",
    "Entering scheduler loop",
];

/// Serial-log substrings that confirm at least one userspace process is running.
const USERSPACE_MARKERS: &[&str] = &[
    "SPROUT:",
    "CAMBIUM:",
    "sprout: service",
    "BOOT: heartbeat",
    "BOOT: ready",
];

/// Serial-log substrings that confirm the graphical desktop is up.
const DESKTOP_MARKERS: &[&str] = &[
    "bloom: service loop started",
    "bloom: output0",
    "bloom: cursor ready",
    "First frame rendered",
    "wayland-server: listening on /run/wayland-0",
    "bloom: registered bristle pointer sink",
];

/// The ordered set of guest execution phases the hunter tracks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BootPhase {
    /// QEMU process started; no serial output analysed yet.
    QemuStarted,
    /// A kernel-identifying line appeared in the serial log.
    KernelSeen,
    /// At least one userspace process is running.
    UserspaceSeen,
    /// The graphical desktop (Bloom/Wayland) is up.
    DesktopSeen,
    /// Serial output went silent after the kernel was confirmed running.
    StalledAfterKernel,
}

impl std::fmt::Display for BootPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BootPhase::QemuStarted => write!(f, "qemu_started"),
            BootPhase::KernelSeen => write!(f, "kernel_seen"),
            BootPhase::UserspaceSeen => write!(f, "userspace_seen"),
            BootPhase::DesktopSeen => write!(f, "desktop_seen"),
            BootPhase::StalledAfterKernel => write!(f, "stalled_after_kernel"),
        }
    }
}

/// Lightweight tracker that infers the boot phase from the accumulated serial log.
#[derive(Debug, Default)]
struct BootPhaseTracker {
    pub phase: BootPhase,
    pub kernel_seen: bool,
    pub userspace_seen: bool,
    pub desktop_seen: bool,
}

impl Default for BootPhase {
    fn default() -> Self {
        BootPhase::QemuStarted
    }
}

impl BootPhaseTracker {
    fn new() -> Self {
        Self { phase: BootPhase::QemuStarted, kernel_seen: false, userspace_seen: false, desktop_seen: false }
    }

    /// Update internal state from the current serial log and return `true` if
    /// the phase advanced.
    fn update(&mut self, log: &str) -> bool {
        let lower = log.to_lowercase();
        let old_phase = self.phase;

        if !self.kernel_seen {
            if KERNEL_MARKERS.iter().any(|m| lower.contains(&m.to_lowercase())) {
                self.kernel_seen = true;
            }
        }
        if self.kernel_seen && !self.userspace_seen {
            if USERSPACE_MARKERS.iter().any(|m| lower.contains(&m.to_lowercase())) {
                self.userspace_seen = true;
            }
        }
        if self.kernel_seen && !self.desktop_seen {
            if DESKTOP_MARKERS.iter().any(|m| lower.contains(&m.to_lowercase())) {
                self.desktop_seen = true;
            }
        }

        self.phase = if self.desktop_seen {
            BootPhase::DesktopSeen
        } else if self.userspace_seen {
            BootPhase::UserspaceSeen
        } else if self.kernel_seen {
            BootPhase::KernelSeen
        } else {
            BootPhase::QemuStarted
        };

        self.phase != old_phase
    }

    /// Classify a freeze/stall event.  Returns a string tag used in log output.
    fn classify_freeze(&self) -> &'static str {
        if !self.kernel_seen {
            "pre_kernel_hunter_false_positive"
        } else {
            "os_freeze"
        }
    }

    /// Mark the current run as stalled.  If the kernel was previously observed
    /// the phase advances to `StalledAfterKernel`; otherwise the phase stays at
    /// `QemuStarted` (the stall is pre-kernel and classified as a false positive).
    fn mark_stalled(&mut self) {
        if self.kernel_seen {
            self.phase = BootPhase::StalledAfterKernel;
        }
    }
}

/// A snapshot of hunt state written to the log at every freeze/timeout event.
struct FreezeSummary<'a> {
    last_serial_timestamp: Instant,
    silence_duration: Duration,
    tracker: &'a BootPhaseTracker,
    classification: &'a str,
}

impl<'a> FreezeSummary<'a> {
    fn write(&self, file: &mut File) -> std::io::Result<()> {
        writeln!(file)?;
        writeln!(file, "[freeze_summary]")?;
        writeln!(file, "  last_serial_ago_secs: {:.1}", self.silence_duration.as_secs_f64())?;
        writeln!(file, "  kernel_marker_observed: {}", self.tracker.kernel_seen)?;
        writeln!(file, "  userspace_observed: {}", self.tracker.userspace_seen)?;
        writeln!(file, "  desktop_observed: {}", self.tracker.desktop_seen)?;
        writeln!(file, "  guest_phase_at_capture: {}", self.tracker.phase)?;
        writeln!(file, "  classification: {}", self.classification)?;
        writeln!(
            file,
            "  last_serial_wall_elapsed_secs: {:.1}",
            self.last_serial_timestamp.elapsed().as_secs_f64()
        )?;
        file.flush()
    }
}

#[derive(Clone, Debug)]
pub struct FreezeHunterConfig {
    pub arch: String,
    pub timeout: Duration,
    pub desktop_ready_timeout: Duration,
    pub loglevel: String,
    pub action_interval: Duration,
    pub sessions: Option<u64>,
    pub seed: Option<u64>,
    pub log_dir: PathBuf,
}

impl Default for FreezeHunterConfig {
    fn default() -> Self {
        Self {
            arch: "x86_64".to_string(),
            timeout: Duration::from_secs(15),
            desktop_ready_timeout: Duration::from_secs(180),
            loglevel: "5".to_string(),
            action_interval: Duration::from_millis(900),
            sessions: None,
            seed: None,
            log_dir: PathBuf::from("freeze_logs"),
        }
    }
}

pub async fn run(config: FreezeHunterConfig) -> HunterResult<()> {
    fs::create_dir_all(&config.log_dir)?;

    // BDD world boot reads BDD_LOGLEVEL, so set it before constructing sessions.
    // This binary is single-threaded at process start, before any guest work begins.
    unsafe {
        std::env::set_var("BDD_LOGLEVEL", &config.loglevel);
    }

    let seed = config.seed.unwrap_or_else(|| {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos() as u64
    });
    let mut rng = StdRng::seed_from_u64(seed);

    eprintln!("=== ThingOS Rust Freeze Hunter ===");
    eprintln!(
        "[hunter] arch={} timeout={:?} desktop_ready_timeout={:?} loglevel={}",
        config.arch, config.timeout, config.desktop_ready_timeout, config.loglevel
    );
    eprintln!("[hunter] action_interval={:?} seed={}", config.action_interval, seed);
    eprintln!("[hunter] logs={}", config.log_dir.display());

    let mut session_id = 0u64;
    loop {
        if config.sessions.is_some_and(|limit| session_id >= limit) {
            return Ok(());
        }

        if let Err(err) = run_session(session_id, &config, &mut rng).await {
            eprintln!("[hunter] session {session_id:04} ended with error: {err}");
        }
        session_id += 1;
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn run_session(
    session_id: u64,
    config: &FreezeHunterConfig,
    rng: &mut StdRng,
) -> HunterResult<()> {
    let timestamp = chrono::Utc::now().format("%Y%m%dT%H%M%SZ");
    let log_path = config.log_dir.join(format!("rust_run_{session_id:04}_{timestamp}.log"));
    eprintln!("[hunter] session {session_id:04} booting; log={}", log_path.display());
    let mut log_file = OpenOptions::new().create(true).append(true).open(&log_path)?;
    write_hunter_event(
        &mut log_file,
        format_args!(
            "session={session_id:04} arch={} timeout={:?} desktop_ready_timeout={:?} loglevel={}",
            config.arch, config.timeout, config.desktop_ready_timeout, config.loglevel
        ),
    )?;

    let mut world = ThingOsWorld::default();
    if let Err(err) = world.boot(&config.arch).await.map_err(|e| e.to_string()) {
        write_hunter_event(&mut log_file, format_args!("boot failed: {err}"))?;
        return Err(err.into());
    }

    let mut tracker = BootPhaseTracker::new();
    let mut last_len = 0usize;
    let mut last_output = Instant::now();
    if !wait_for_desktop_readiness(
        session_id,
        config,
        &mut world,
        &mut log_file,
        &mut last_len,
        &mut last_output,
        &mut tracker,
    )
    .await?
    {
        world.shutdown().await;
        return Ok(());
    }

    last_output = Instant::now();
    let mut next_action = Instant::now() + Duration::from_millis(500);

    loop {
        tokio::time::sleep(Duration::from_millis(100)).await;

        let log = world.get_serial_log().await;
        if tracker.update(&log) {
            write_hunter_event(
                &mut log_file,
                format_args!("boot phase advanced: {}", tracker.phase),
            )?;
        }
        if append_serial_delta(&mut log_file, &log, &mut last_len)? {
            last_output = Instant::now();
        }

        if last_output.elapsed() > config.timeout {
            let silence = last_output.elapsed();
            tracker.mark_stalled();
            let classification = tracker.classify_freeze();
            let message = format!(
                "[hunter] serial output stalled after {:.1}s in session {session_id:04}; phase={} classification={classification}",
                silence.as_secs_f64(),
                tracker.phase,
            );
            eprintln!("{message}");
            write_hunter_event(&mut log_file, format_args!("{message}"))?;
            let summary = FreezeSummary {
                last_serial_timestamp: last_output,
                silence_duration: silence,
                tracker: &tracker,
                classification,
            };
            summary.write(&mut log_file)?;
            capture_freeze_artifacts(&mut world, config, session_id, &tracker).await;
            world.shutdown().await;
            return Ok(());
        }

        // Only fire random actions once the kernel has been confirmed running.
        // This prevents accidental Limine/UEFI menu navigation that produces
        // false-positive freeze captures.
        if tracker.kernel_seen && Instant::now() >= next_action {
            write_hunter_event(&mut log_file, format_args!("running random action"))?;
            if let Err(err) = run_random_action(&mut world, rng).await {
                eprintln!("[hunter] action failed: {err}");
                write_hunter_event(&mut log_file, format_args!("action failed: {err}"))?;
            }
            let log = world.get_serial_log().await;
            append_serial_delta(&mut log_file, &log, &mut last_len)?;
            let jitter = rng.gen_range(0..=config.action_interval.as_millis().max(1) as u64);
            next_action = Instant::now() + config.action_interval + Duration::from_millis(jitter);
        }

        if world.qemu.as_mut().is_some_and(|child| child.try_wait().ok().flatten().is_some()) {
            let log = world.get_serial_log().await;
            append_serial_delta(&mut log_file, &log, &mut last_len)?;
            write_hunter_event(&mut log_file, format_args!("qemu exited"))?;
            world.shutdown().await;
            return Ok(());
        }
    }
}

async fn wait_for_desktop_readiness(
    session_id: u64,
    config: &FreezeHunterConfig,
    world: &mut ThingOsWorld,
    log_file: &mut File,
    last_len: &mut usize,
    last_output: &mut Instant,
    tracker: &mut BootPhaseTracker,
) -> HunterResult<bool> {
    write_hunter_event(
        log_file,
        format_args!(
            "waiting for desktop readiness before random actions: {}",
            DESKTOP_READY_SIGNALS
                .iter()
                .map(|group| group.join(" AND "))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    )?;

    let deadline = Instant::now() + config.desktop_ready_timeout;
    let mut last_status = Instant::now();

    loop {
        tokio::time::sleep(Duration::from_millis(100)).await;

        let log = world.get_serial_log().await;
        if tracker.update(&log) {
            write_hunter_event(
                log_file,
                format_args!("boot phase advanced: {}", tracker.phase),
            )?;
        }
        if append_serial_delta(log_file, &log, last_len)? {
            *last_output = Instant::now();
        }

        let missing = missing_required_signals(&log, DESKTOP_READY_SIGNALS);
        if missing.is_empty() {
            write_hunter_event(
                log_file,
                format_args!("desktop ready; starting random actions in session {session_id:04}"),
            )?;
            return Ok(true);
        }

        if last_status.elapsed() >= Duration::from_secs(5) {
            write_hunter_event(
                log_file,
                format_args!(
                    "desktop readiness pending; phase={} missing={missing:?}",
                    tracker.phase
                ),
            )?;
            last_status = Instant::now();
        }

        if last_output.elapsed() > config.timeout {
            let silence = last_output.elapsed();
            tracker.mark_stalled();
            let classification = tracker.classify_freeze();
            let message = format!(
                "[hunter] serial output stalled after {:.1}s in session {session_id:04}; phase={} classification={classification} missing={missing:?}",
                silence.as_secs_f64(),
                tracker.phase,
            );
            eprintln!("{message}");
            write_hunter_event(log_file, format_args!("{message}"))?;
            let summary = FreezeSummary {
                last_serial_timestamp: *last_output,
                silence_duration: silence,
                tracker,
                classification,
            };
            summary.write(log_file)?;
            capture_freeze_artifacts(world, config, session_id, tracker).await;
            return Ok(false);
        }

        if Instant::now() >= deadline {
            tracker.mark_stalled();
            let classification = tracker.classify_freeze();
            let message = format!(
                "[hunter] desktop readiness timed out after {:.1}s in session {session_id:04}; phase={} classification={classification} missing={missing:?}",
                config.desktop_ready_timeout.as_secs_f64(),
                tracker.phase,
            );
            eprintln!("{message}");
            write_hunter_event(log_file, format_args!("{message}"))?;
            let summary = FreezeSummary {
                last_serial_timestamp: *last_output,
                silence_duration: last_output.elapsed(),
                tracker,
                classification,
            };
            summary.write(log_file)?;
            capture_freeze_artifacts(world, config, session_id, tracker).await;
            return Ok(false);
        }

        if world.qemu.as_mut().is_some_and(|child| child.try_wait().ok().flatten().is_some()) {
            let log = world.get_serial_log().await;
            append_serial_delta(log_file, &log, last_len)?;
            write_hunter_event(
                log_file,
                format_args!(
                    "qemu exited while waiting for desktop readiness; phase={} missing={missing:?}",
                    tracker.phase,
                ),
            )?;
            return Ok(false);
        }
    }
}

fn append_serial_delta(file: &mut File, log: &str, cursor: &mut usize) -> std::io::Result<bool> {
    if *cursor > log.len() {
        *cursor = 0;
    }
    if *cursor == log.len() {
        return Ok(false);
    }

    file.write_all(&log.as_bytes()[*cursor..])?;
    file.flush()?;
    *cursor = log.len();
    Ok(true)
}

fn write_hunter_event(file: &mut File, args: std::fmt::Arguments<'_>) -> std::io::Result<()> {
    writeln!(file, "\n[hunter] {args}")?;
    file.flush()
}

async fn capture_freeze_artifacts(
    world: &mut ThingOsWorld,
    config: &FreezeHunterConfig,
    session_id: u64,
    tracker: &BootPhaseTracker,
) {
    let classification = tracker.classify_freeze();
    let screenshot_path = config
        .log_dir
        .join(format!("rust_run_{session_id:04}_freeze_{classification}"));
    eprintln!(
        "[hunter] classification={classification} phase={}",
        tracker.phase
    );
    match world.take_screenshot(&screenshot_path).await {
        Ok(path) => eprintln!("[hunter] screenshot={}", path.display()),
        Err(err) => eprintln!("[hunter] screenshot failed: {err}"),
    }
}

async fn run_random_action(world: &mut ThingOsWorld, rng: &mut StdRng) -> HunterResult<()> {
    match rng.gen_range(0..10) {
        0 | 1 => drag_window(world, rng).await,
        2 => resize_window(world, rng).await,
        3 => click_chrome_button(world, rng).await,
        4 => click_client_and_type(world, rng).await,
        5 => drag_clock_over_hello(world, rng).await,
        6 => launch_or_query(world, rng).await,
        7 => send_combo(world, &["alt"], "tab", Duration::from_millis(120)).await,
        8 => send_key_tap(world, random_key(rng), Duration::from_millis(80)).await,
        _ => random_pointer_noise(world, rng).await,
    }
}

async fn windows_or_fallback(world: &mut ThingOsWorld) -> Vec<WindowInfo> {
    let log = world.get_serial_log().await;
    let windows = parse_wayland_windows(&log);
    if !windows.is_empty() {
        return windows;
    }

    if log.contains(" > ") {
        if let Ok(output) =
            type_serial_command(world, "cat /session/wayland/windows/index", Duration::from_secs(1))
                .await
        {
            let windows = parse_wayland_windows(&output);
            if !windows.is_empty() {
                return windows;
            }
        }
    }

    vec![WindowInfo {
        x: 0,
        y: 0,
        w: 480,
        h: 320,
        z: 0,
        title: "fallback Wayland window".to_string(),
    }]
}

fn choose_window<'a>(windows: &'a [WindowInfo], rng: &mut StdRng) -> &'a WindowInfo {
    &windows[rng.gen_range(0..windows.len())]
}

async fn drag_window(world: &mut ThingOsWorld, rng: &mut StdRng) -> HunterResult<()> {
    let windows = windows_or_fallback(world).await;
    let window = choose_window(&windows, rng);
    let start_x = window.x + rng.gen_range(20..=window.w.min(140).max(21) as i32);
    let start_y = window.y + rng.gen_range(10..=24);
    let end_x = start_x + rng.gen_range(-260..=260);
    let end_y = start_y + rng.gen_range(-180..=180);
    eprintln!("[hunter] action=drag-window title={}", window.title);
    let commands = drag_pointer(start_x, start_y, end_x, end_y, rng.gen_range(3..=7));
    send_qmp_sequence(world, &commands, "hunter drag window").await
}

async fn resize_window(world: &mut ThingOsWorld, rng: &mut StdRng) -> HunterResult<()> {
    let windows = windows_or_fallback(world).await;
    let window =
        find_window(&windows, "Wayland Lab").unwrap_or_else(|| choose_window(&windows, rng));
    let start_x = window.right() - rng.gen_range(4..=18);
    let start_y = window.bottom() - rng.gen_range(4..=18);
    let end_x = start_x + rng.gen_range(-120..=220);
    let end_y = start_y + rng.gen_range(-90..=180);
    eprintln!("[hunter] action=resize-window title={}", window.title);
    let commands = drag_pointer(start_x, start_y, end_x, end_y, rng.gen_range(3..=6));
    send_qmp_sequence(world, &commands, "hunter resize window").await
}

async fn click_chrome_button(world: &mut ThingOsWorld, rng: &mut StdRng) -> HunterResult<()> {
    let windows = windows_or_fallback(world).await;
    let window =
        find_window(&windows, "Wayland Lab").unwrap_or_else(|| choose_window(&windows, rng));
    let (button, offset) = [("minimize", 86), ("maximize", 52), ("close", 20)][rng.gen_range(0..3)];
    let x = window.right() - offset;
    let y = window.y + 20;
    eprintln!("[hunter] action=chrome-button button={button} title={}", window.title);
    let mut commands = move_pointer_to(x, y);
    commands.push((qmp_left_button(true), Duration::from_millis(80)));
    commands.push((qmp_left_button(false), Duration::from_millis(160)));
    send_qmp_sequence(world, &commands, "hunter chrome button").await
}

async fn click_client_and_type(world: &mut ThingOsWorld, rng: &mut StdRng) -> HunterResult<()> {
    let windows = windows_or_fallback(world).await;
    let window =
        find_window(&windows, "Wayland Lab").unwrap_or_else(|| choose_window(&windows, rng));
    let x = window.x + (window.w as i32 / 3).max(80).min(window.w.saturating_sub(20) as i32);
    let y = window.y + (window.h as i32 / 3).max(70).min(window.h.saturating_sub(20) as i32);
    eprintln!("[hunter] action=click-and-type title={}", window.title);
    let mut commands = move_pointer_to(x, y);
    commands.push((qmp_left_button(true), Duration::from_millis(50)));
    commands.push((qmp_left_button(false), Duration::from_millis(80)));
    send_qmp_sequence(world, &commands, "hunter click client").await?;
    for _ in 0..rng.gen_range(1..=4) {
        send_key_tap(world, random_key(rng), Duration::from_millis(40)).await?;
    }
    Ok(())
}

async fn drag_clock_over_hello(world: &mut ThingOsWorld, rng: &mut StdRng) -> HunterResult<()> {
    let windows = windows_or_fallback(world).await;
    let Some(hello) = find_window(&windows, "Wayland Lab") else {
        return drag_window(world, rng).await;
    };
    let Some(clock) = find_window(&windows, "Clock") else {
        return drag_window(world, rng).await;
    };

    eprintln!("[hunter] action=drag-clock-over-hello");
    let commands = drag_pointer(clock.x + 24, clock.y + 14, hello.x + 40, hello.y - 20, 5);
    send_qmp_sequence(world, &commands, "hunter drag clock").await
}

async fn launch_or_query(world: &mut ThingOsWorld, rng: &mut StdRng) -> HunterResult<()> {
    const COMMANDS: &[&str] = &[
        "/bin/wayland_hello &",
        "/bin/clock &",
        "/bin/leaf &",
        "ps",
        "cat /session/wayland/windows/index",
        "cat /session/wayland/events/latest",
        "cat /session/wayland/components",
    ];
    let command = COMMANDS[rng.gen_range(0..COMMANDS.len())];
    eprintln!("[hunter] action=serial command={command:?}");
    let _ = type_serial_command(world, command, Duration::from_secs(2)).await;
    Ok(())
}

async fn random_pointer_noise(world: &ThingOsWorld, rng: &mut StdRng) -> HunterResult<()> {
    eprintln!("[hunter] action=random-pointer");
    let mut commands = Vec::new();
    for _ in 0..rng.gen_range(2..=8) {
        commands.push((
            qmp_mouse_rel(rng.gen_range(-120..=120), rng.gen_range(-90..=90)),
            Duration::from_millis(rng.gen_range(20..=90)),
        ));
    }
    send_qmp_sequence(world, &commands, "hunter random pointer").await
}

fn random_key(rng: &mut StdRng) -> &'static str {
    const KEYS: &[&str] = &[
        "up",
        "down",
        "left",
        "right",
        "pgup",
        "pgdn",
        "home",
        "end",
        "esc",
        "spc",
        "ret",
        "backspace",
        "a",
        "b",
        "c",
        "d",
        "e",
        "f",
        "tab",
    ];
    KEYS[rng.gen_range(0..KEYS.len())]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boot_phase_starts_at_qemu_started() {
        let tracker = BootPhaseTracker::new();
        assert_eq!(tracker.phase, BootPhase::QemuStarted);
        assert!(!tracker.kernel_seen);
        assert!(!tracker.userspace_seen);
        assert!(!tracker.desktop_seen);
    }

    #[test]
    fn kernel_marker_advances_phase() {
        let mut tracker = BootPhaseTracker::new();
        let changed = tracker.update("thing-os kernel v1.0");
        assert!(changed);
        assert_eq!(tracker.phase, BootPhase::KernelSeen);
        assert!(tracker.kernel_seen);
        assert!(!tracker.userspace_seen);
    }

    #[test]
    fn userspace_marker_advances_phase_after_kernel() {
        let mut tracker = BootPhaseTracker::new();
        tracker.update("thing-os kernel v1.0");
        let changed = tracker.update("thing-os kernel v1.0\nSPROUT: service starting");
        assert!(changed);
        assert_eq!(tracker.phase, BootPhase::UserspaceSeen);
        assert!(tracker.userspace_seen);
    }

    #[test]
    fn desktop_marker_advances_phase_after_kernel() {
        let mut tracker = BootPhaseTracker::new();
        tracker.update("thing-os kernel v1.0");
        let changed = tracker.update("thing-os kernel v1.0\nFirst frame rendered");
        assert!(changed);
        assert_eq!(tracker.phase, BootPhase::DesktopSeen);
        assert!(tracker.desktop_seen);
    }

    #[test]
    fn no_kernel_marker_stays_at_qemu_started() {
        let mut tracker = BootPhaseTracker::new();
        let changed = tracker.update("Limine bootloader menu");
        assert!(!changed);
        assert_eq!(tracker.phase, BootPhase::QemuStarted);
    }

    #[test]
    fn classify_freeze_without_kernel_is_false_positive() {
        let tracker = BootPhaseTracker::new();
        assert_eq!(tracker.classify_freeze(), "pre_kernel_hunter_false_positive");
    }

    #[test]
    fn classify_freeze_with_kernel_is_os_freeze() {
        let mut tracker = BootPhaseTracker::new();
        tracker.update("thing-os kernel v1.0");
        assert_eq!(tracker.classify_freeze(), "os_freeze");
    }

    #[test]
    fn update_is_idempotent_once_phase_reached() {
        let mut tracker = BootPhaseTracker::new();
        tracker.update("thing-os kernel v1.0");
        let changed = tracker.update("thing-os kernel v1.0");
        assert!(!changed, "phase should not re-advance on same log");
        assert_eq!(tracker.phase, BootPhase::KernelSeen);
    }

    #[test]
    fn mark_stalled_sets_phase_when_kernel_seen() {
        let mut tracker = BootPhaseTracker::new();
        tracker.update("thing-os kernel v1.0");
        tracker.mark_stalled();
        assert_eq!(tracker.phase, BootPhase::StalledAfterKernel);
    }

    #[test]
    fn mark_stalled_does_not_advance_phase_pre_kernel() {
        let mut tracker = BootPhaseTracker::new();
        tracker.mark_stalled();
        assert_eq!(tracker.phase, BootPhase::QemuStarted);
    }

    #[test]
    fn boot_phase_display() {
        assert_eq!(BootPhase::QemuStarted.to_string(), "qemu_started");
        assert_eq!(BootPhase::KernelSeen.to_string(), "kernel_seen");
        assert_eq!(BootPhase::UserspaceSeen.to_string(), "userspace_seen");
        assert_eq!(BootPhase::DesktopSeen.to_string(), "desktop_seen");
        assert_eq!(BootPhase::StalledAfterKernel.to_string(), "stalled_after_kernel");
    }
}
