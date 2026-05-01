use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::input::{
    WindowInfo, drag_pointer, find_window, move_pointer_to, parse_wayland_windows, qmp_left_button,
    qmp_mouse_rel, send_combo, send_key_tap, send_qmp_sequence, type_serial_command,
};
use crate::world::ThingOsWorld;

pub type HunterResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Debug)]
pub struct FreezeHunterConfig {
    pub arch: String,
    pub timeout: Duration,
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
        "[hunter] arch={} timeout={:?} loglevel={}",
        config.arch, config.timeout, config.loglevel
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

    let mut world = ThingOsWorld::default();
    world.boot(&config.arch).await.map_err(|e| e.to_string())?;

    let mut last_len = world.get_serial_log().await.len();
    let mut last_output = Instant::now();
    let mut next_action = Instant::now() + Duration::from_secs(2);

    loop {
        tokio::time::sleep(Duration::from_millis(100)).await;

        let log = world.get_serial_log().await;
        if log.len() != last_len {
            last_len = log.len();
            last_output = Instant::now();
        }

        if last_output.elapsed() > config.timeout {
            eprintln!(
                "[hunter] silence detected after {:.1}s in session {session_id:04}",
                last_output.elapsed().as_secs_f64()
            );
            fs::write(&log_path, &log)?;
            capture_freeze_artifacts(&mut world, config, session_id).await;
            world.shutdown().await;
            return Ok(());
        }

        if Instant::now() >= next_action {
            if let Err(err) = run_random_action(&mut world, rng).await {
                eprintln!("[hunter] action failed: {err}");
            }
            let jitter = rng.gen_range(0..=config.action_interval.as_millis().max(1) as u64);
            next_action = Instant::now() + config.action_interval + Duration::from_millis(jitter);
        }

        if world.qemu.as_mut().is_some_and(|child| child.try_wait().ok().flatten().is_some()) {
            let log = world.get_serial_log().await;
            fs::write(&log_path, &log)?;
            world.shutdown().await;
            return Ok(());
        }
    }
}

async fn capture_freeze_artifacts(
    world: &mut ThingOsWorld,
    config: &FreezeHunterConfig,
    session_id: u64,
) {
    let screenshot_path = config.log_dir.join(format!("rust_run_{session_id:04}_freeze"));
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
