use std::time::{Duration, Instant};

use regex::Regex;

use crate::world::{ThingOsWorld, strip_ansi};

pub type InputResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Debug)]
pub struct WindowInfo {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
    pub z: i32,
    pub title: String,
}

impl WindowInfo {
    pub fn right(&self) -> i32 {
        self.x.saturating_add(self.w as i32)
    }

    pub fn bottom(&self) -> i32 {
        self.y.saturating_add(self.h as i32)
    }
}

pub fn qmp_mouse_rel(dx: i32, dy: i32) -> String {
    format!(
        r#"{{"execute": "input-send-event", "arguments": {{"events": [{{"type": "rel", "data": {{"axis": "x", "value": {}}}}}, {{"type": "rel", "data": {{"axis": "y", "value": {}}}}}]}}}}"#,
        dx, dy
    )
}

pub fn qmp_mouse_button(button: &str, down: bool) -> String {
    format!(
        r#"{{"execute": "input-send-event", "arguments": {{"events": [{{"type": "btn", "data": {{"down": {}, "button": "{}"}}}}]}}}}"#,
        if down { "true" } else { "false" },
        button
    )
}

pub fn qmp_left_button(down: bool) -> String {
    qmp_mouse_button("left", down)
}

pub fn qmp_key(key: &str, down: bool) -> String {
    format!(
        r#"{{"execute": "input-send-event", "arguments": {{"events": [{{"type": "key", "data": {{"down": {}, "key": {{"type": "qcode", "data": "{}"}}}}}}]}}}}"#,
        if down { "true" } else { "false" },
        key
    )
}

pub fn qmp_abs_pointer(x: i32, y: i32, width: u32, height: u32) -> String {
    let width = width.max(1) as f64;
    let height = height.max(1) as f64;
    let qx = (x.max(0) as f64 * 32768.0 / width).round() as u32;
    let qy = (y.max(0) as f64 * 32768.0 / height).round() as u32;
    format!(
        r#"{{"execute": "input-send-event", "arguments": {{"events": [{{"type": "abs", "data": {{"axis": "x", "value": {}}}}}, {{"type": "abs", "data": {{"axis": "y", "value": {}}}}}]}}}}"#,
        qx, qy
    )
}

pub async fn send_qmp_sequence(
    world: &ThingOsWorld,
    commands: &[(String, Duration)],
    context: &str,
) -> InputResult<()> {
    for (command, settle) in commands {
        world
            .execute_qmp_control(command)
            .await
            .map_err(|e| format!("QMP {context} failed: {e}"))?;
        tokio::time::sleep(*settle).await;
    }
    Ok(())
}

pub async fn send_key_tap(world: &ThingOsWorld, key: &str, settle: Duration) -> InputResult<()> {
    let commands = [(qmp_key(key, true), Duration::from_millis(30)), (qmp_key(key, false), settle)];
    send_qmp_sequence(world, &commands, "key tap").await
}

pub async fn send_combo(
    world: &ThingOsWorld,
    modifiers: &[&str],
    key: &str,
    settle: Duration,
) -> InputResult<()> {
    let mut commands = Vec::new();
    for modifier in modifiers {
        commands.push((qmp_key(modifier, true), Duration::from_millis(25)));
    }
    commands.push((qmp_key(key, true), Duration::from_millis(35)));
    commands.push((qmp_key(key, false), Duration::from_millis(25)));
    for modifier in modifiers.iter().rev() {
        commands.push((qmp_key(modifier, false), Duration::from_millis(25)));
    }
    commands.push((qmp_mouse_rel(1, 0), settle));
    send_qmp_sequence(world, &commands, "key combo").await
}

pub fn move_pointer_to(x: i32, y: i32) -> Vec<(String, Duration)> {
    vec![
        (qmp_mouse_rel(-10000, -10000), Duration::from_millis(80)),
        (qmp_mouse_rel(x.max(0), y.max(0)), Duration::from_millis(80)),
    ]
}

pub fn drag_pointer(
    from_x: i32,
    from_y: i32,
    to_x: i32,
    to_y: i32,
    steps: u32,
) -> Vec<(String, Duration)> {
    let steps = steps.max(1);
    let mut commands = move_pointer_to(from_x, from_y);
    commands.push((qmp_left_button(true), Duration::from_millis(80)));

    let mut last_x = from_x;
    let mut last_y = from_y;
    for step in 1..=steps {
        let next_x = from_x + ((to_x - from_x) as i64 * step as i64 / steps as i64) as i32;
        let next_y = from_y + ((to_y - from_y) as i64 * step as i64 / steps as i64) as i32;
        commands.push((qmp_mouse_rel(next_x - last_x, next_y - last_y), Duration::from_millis(45)));
        last_x = next_x;
        last_y = next_y;
    }

    commands.push((qmp_left_button(false), Duration::from_millis(120)));
    commands
}

pub async fn type_serial_command(
    world: &mut ThingOsWorld,
    command: &str,
    timeout: Duration,
) -> InputResult<String> {
    world.serial_checkpoint = world.get_serial_log().await.len();
    world.last_typed_command = Some(command.to_string());

    let mut data = command.as_bytes().to_vec();
    if !data.ends_with(b"\n") {
        data.push(b'\n');
    }
    for b in data {
        world.serial_write(&[b]).await?;
        tokio::time::sleep(Duration::from_millis(15)).await;
    }

    let start = Instant::now();
    loop {
        let log = world.get_serial_log().await;
        let start_offset = world.serial_checkpoint.min(log.len());
        let recent = &log[start_offset..];
        if recent.contains(" > ") {
            return Ok(recent.to_string());
        }
        if start.elapsed() >= timeout {
            return Err(format!("Timeout waiting for command '{command}' to complete").into());
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

pub fn parse_wayland_windows(text: &str) -> Vec<WindowInfo> {
    let re =
        Regex::new(r#"(?m)^\s*\d+\s+(\d+)x(\d+)\+(-?\d+),(-?\d+)\s+z=(-?\d+)\s+title="([^"]*)""#)
            .expect("valid Wayland window regex");
    re.captures_iter(&strip_ansi(text))
        .filter_map(|caps| {
            Some(WindowInfo {
                w: caps.get(1)?.as_str().parse().ok()?,
                h: caps.get(2)?.as_str().parse().ok()?,
                x: caps.get(3)?.as_str().parse().ok()?,
                y: caps.get(4)?.as_str().parse().ok()?,
                z: caps.get(5)?.as_str().parse().ok()?,
                title: caps.get(6)?.as_str().to_string(),
            })
        })
        .collect()
}

pub async fn read_wayland_windows(world: &mut ThingOsWorld) -> InputResult<Vec<WindowInfo>> {
    let output =
        type_serial_command(world, "cat /session/wayland/windows/index", Duration::from_secs(30))
            .await?;
    Ok(parse_wayland_windows(&output))
}

pub fn find_window<'a>(windows: &'a [WindowInfo], title: &str) -> Option<&'a WindowInfo> {
    windows.iter().find(|window| window.title.to_lowercase().contains(&title.to_lowercase()))
}

#[cfg(test)]
mod tests {
    use super::{parse_wayland_windows, qmp_key, qmp_mouse_rel};

    #[test]
    fn parses_wayland_window_index_lines() {
        let windows = parse_wayland_windows(
            r#"  7 480x320+48,20 z=3 title="Thing-OS Wayland Lab"
  9 220x90+1500,940 z=4 title="Clock""#,
        );
        assert_eq!(windows.len(), 2);
        assert_eq!(windows[0].title, "Thing-OS Wayland Lab");
        assert_eq!(windows[1].x, 1500);
    }

    #[test]
    fn builds_qmp_input_commands() {
        assert!(qmp_mouse_rel(8, 3).contains(r#""axis": "x""#));
        assert!(qmp_key("tab", true).contains(r#""data": "tab""#));
    }
}
