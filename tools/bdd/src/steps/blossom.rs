use cucumber::{given, then, when};

use super::basic::turn_on_machine;
use super::helpers::{StepError, capture_failure_diagnostics, color_close};
use crate::input::{
    WindowInfo, drag_pointer, find_window as find_shared_window, qmp_mouse_rel,
    read_wayland_windows as read_shared_wayland_windows, send_qmp_sequence,
};
use crate::world::ThingOsWorld;

// ===== Blossom XDG-Shell Steps =====

fn is_brass_pixel(pixel: [u8; 3]) -> bool {
    color_close(pixel, [0xF2, 0xC9, 0x4C], 20)
}

fn is_paper_pixel(pixel: [u8; 3]) -> bool {
    color_close(pixel, [0x0B, 0x0A, 0x10], 12)
        || color_close(pixel, [0x12, 0x0E, 0x18], 16)
        || color_close(pixel, [0x22, 0x1A, 0x30], 18)
        || color_close(pixel, [0xFF, 0xF9, 0xEC], 16)
        || color_close(pixel, [0xFD, 0xF1, 0xD2], 16)
        || (pixel[0] >= 245 && pixel[1] >= 232 && pixel[2] >= 200)
}

fn is_paper_text_pixel(pixel: [u8; 3]) -> bool {
    color_close(pixel, [0xB8, 0xA8, 0xFF], 28)
        || color_close(pixel, [0xE6, 0xE1, 0xFF], 24)
        || color_close(pixel, [0x3F, 0x3A, 0x2F], 30)
        || color_close(pixel, [0x3B, 0x2A, 0x0A], 30)
}

fn is_chrome_border_pixel(pixel: [u8; 3]) -> bool {
    color_close(pixel, [0x0F, 0x0C, 0x18], 16)
        || color_close(pixel, [0x1A, 0x14, 0x24], 18)
        || color_close(pixel, [0x23, 0x1A, 0x33], 18)
        || color_close(pixel, [0x2A, 0x1F, 0x3A], 18)
        || color_close(pixel, [0x7C, 0x5C, 0xFF], 24)
        || color_close(pixel, [0xB8, 0xA8, 0xFF], 24)
}

fn is_window_chrome_pixel(pixel: [u8; 3]) -> bool {
    is_brass_pixel(pixel) || is_chrome_border_pixel(pixel)
}

#[derive(Clone, Copy)]
struct TestRect {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
}

async fn read_wayland_windows(world: &mut ThingOsWorld) -> Result<Vec<WindowInfo>, StepError> {
    let _ = world.wait_for_serial(" > ", 60.0).await;
    let mut last = Vec::new();
    for _ in 0..8 {
        let windows = read_shared_wayland_windows(world)
            .await
            .map_err(|e| StepError(format!("Could not read Wayland windows: {}", e)))?;
        last = windows.clone();
        if windows.iter().any(|w| w.title.contains("Thing-OS Wayland Lab"))
            && windows.iter().any(|w| w.title.contains("Clock"))
        {
            return Ok(windows);
        }
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }
    Err(StepError(format!(
        "Could not read Wayland hello and Clock windows from /session/wayland/windows/index: {:?}",
        last
    )))
}

fn find_window<'a>(windows: &'a [WindowInfo], title: &str) -> Result<&'a WindowInfo, StepError> {
    find_shared_window(windows, title)
        .ok_or_else(|| StepError(format!("Window titled '{}' was not present", title)))
}

fn intersect_test_rect(a: TestRect, b: TestRect) -> Option<TestRect> {
    let x0 = a.x.max(b.x);
    let y0 = a.y.max(b.y);
    let x1 = a.x.saturating_add(a.w as i32).min(b.x.saturating_add(b.w as i32));
    let y1 = a.y.saturating_add(a.h as i32).min(b.y.saturating_add(b.h as i32));
    if x1 <= x0 || y1 <= y0 {
        None
    } else {
        Some(TestRect { x: x0, y: y0, w: (x1 - x0) as u32, h: (y1 - y0) as u32 })
    }
}

/// Background: `Given the bloom compositor is running with blossom support`
#[given("the bloom compositor is running with blossom support")]
async fn bloom_compositor_running_with_blossom(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // Boot if not already running.
    if world.qemu.is_none() {
        turn_on_machine(world).await?;
    }
    // Wait for bloom's wayland server to announce it is listening.
    let found = world.wait_for_serial("wayland-server: listening on /run/wayland-0", 90.0).await;
    if !found {
        let log = world.get_serial_log().await;
        if log.contains("bloom:") || log.contains("wayland-server") {
            eprintln!(
                "│  │  │      ⚠️  Bloom started but wayland-server not listening yet — checking"
            );
        } else {
            capture_failure_diagnostics(world, "bloom compositor").await;
            return Err(StepError("Bloom compositor did not start".to_string()));
        }
        return Err(StepError(
            "Bloom started but wayland-server not listening on /run/wayland-0".to_string(),
        ));
    }
    eprintln!("│  │  │      ✅ Bloom compositor running with blossom xdg-shell support");
    Ok(())
}

/// Background: `And a Wayland client has connected via /run/wayland-0`
#[given("a Wayland client has connected via /run/wayland-0")]
async fn wayland_client_connected(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    if log.contains("wayland_hello: connected to /run/wayland-0")
        || log.contains("wayland-server: new client")
    {
        eprintln!("│  │  │      ✅ Wayland client connected via /run/wayland-0");
        return Ok(());
    }

    // Sprout autolaunches wayland_hello after bloom publishes /run/wayland-0.
    // This keeps the BDD coverage on the boot-time pipeline instead of using
    // the serial shell as an out-of-band launcher.
    let _ = world.wait_for_serial("SPROUT: Spawned wayland_hello", 60.0).await;
    let found = world.wait_for_serial("wayland_hello: connected to /run/wayland-0", 30.0).await;
    if !found {
        let alt = world.wait_for_serial("wayland-server: new client", 10.0).await;
        if !alt {
            capture_failure_diagnostics(world, "wayland client connect").await;
            return Err(StepError(
                "Autolaunched Wayland client (wayland_hello) failed to connect to /run/wayland-0"
                    .to_string(),
            ));
        }
    }
    eprintln!("│  │  │      ✅ Wayland client connected via /run/wayland-0");
    Ok(())
}

/// Background: `And the client has bound wl_compositor and xdg_wm_base`
#[given("the client has bound wl_compositor and xdg_wm_base")]
async fn client_bound_compositor_and_wm_base(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // wayland_hello binds wl_compositor, wl_shm, and xdg_wm_base immediately
    // after connecting. Verify the client is alive.
    let log = world.get_serial_log().await;
    if log.contains("wayland_hello: connected to /run/wayland-0")
        || log.contains("wayland-server: new client")
    {
        eprintln!("│  │  │      ✅ Client has bound wl_compositor and xdg_wm_base");
        Ok(())
    } else {
        Err(StepError(
            "No evidence that a Wayland client has bound wl_compositor/xdg_wm_base".to_string(),
        ))
    }
}

/// `Given the client has created a wl_surface`
#[given("the client has created a wl_surface")]
async fn client_created_wl_surface(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // wayland_hello creates two surfaces (toplevel + popup).  Wait for
    // bloom to have processed at least one get_xdg_surface call, which
    // implies that a wl_surface was created first.
    let found = world.wait_for_serial("wayland-server: xdg_surface obj=", 10.0).await;
    if found {
        eprintln!("│  │  │      ✅ wl_surface created (xdg_surface creation observed)");
        return Ok(());
    }
    // Accept that the surface was created even if we can't observe it directly.
    let log = world.get_serial_log().await;
    if log.contains("wayland_hello: connected") {
        eprintln!("│  │  │      ⚠️  Client connected but xdg_surface log not yet seen");
        return Ok(());
    }
    Err(StepError("No evidence that client created a wl_surface".to_string()))
}

/// `When the client calls xdg_wm_base.get_xdg_surface for that wl_surface`
#[when("the client calls xdg_wm_base.get_xdg_surface for that wl_surface")]
async fn client_calls_get_xdg_surface(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // This action is performed by wayland_hello; we just observe the log.
    let found = world.wait_for_serial("wayland-server: xdg_surface obj=", 10.0).await;
    if found {
        eprintln!("│  │  │      ✅ get_xdg_surface call observed in compositor log");
        return Ok(());
    }
    Err(StepError("get_xdg_surface was not observed in compositor log within timeout".to_string()))
}

/// `Then a new xdg_surface object is registered successfully`
#[then("a new xdg_surface object is registered successfully")]
async fn xdg_surface_registered(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    if log.contains("wayland-server: xdg_surface obj=") && log.contains("created for surface=") {
        eprintln!("│  │  │      ✅ xdg_surface registered successfully");
        Ok(())
    } else {
        Err(StepError(
            "No 'wayland-server: xdg_surface obj=... created for surface=...' in log".to_string(),
        ))
    }
}

/// `And the wl_surface now has the xdg role`
#[then("the wl_surface now has the xdg role")]
async fn wl_surface_has_xdg_role(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    // Role is set when xdg_surface is created; absence of role conflict error confirms it.
    if log.contains("wayland-server: xdg_surface obj=")
        && !log.contains("wayland-server: xdg_surface error: role conflict")
    {
        eprintln!("│  │  │      ✅ wl_surface has xdg role (no role conflict error)");
        Ok(())
    } else if log.contains("wayland-server: xdg_surface error: role conflict") {
        Err(StepError("Role conflict error found — wl_surface already had an xdg role".to_string()))
    } else {
        Err(StepError("No evidence that wl_surface has the xdg role".to_string()))
    }
}

/// `And the client has already called get_xdg_surface for that surface`
#[given("the client has already called get_xdg_surface for that surface")]
async fn client_already_called_get_xdg_surface(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // Verify at least one xdg_surface creation happened.
    let found = world.wait_for_serial("wayland-server: xdg_surface obj=", 10.0).await;
    if found {
        eprintln!("│  │  │      ✅ First get_xdg_surface already called");
        Ok(())
    } else {
        Err(StepError("No xdg_surface creation observed — precondition not met".to_string()))
    }
}

/// `When the client calls get_xdg_surface again for the same wl_surface`
#[when("the client calls get_xdg_surface again for the same wl_surface")]
async fn client_calls_get_xdg_surface_again(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // This error path is not exercised by wayland_hello; the test will fail
    // (intentionally) unless a dedicated error-path client is used.
    // We check if the error already appeared in the log (would require a
    // separate client that explicitly triggers the error).
    let log = world.get_serial_log().await;
    if log.contains("wayland-server: xdg_surface error: role conflict") {
        eprintln!("│  │  │      ✅ Second get_xdg_surface call triggered role conflict");
        Ok(())
    } else {
        Err(StepError(
            "The role-conflict error path is not exercised: no client called get_xdg_surface \
             twice for the same wl_surface. A dedicated error-path test client is required."
                .to_string(),
        ))
    }
}

/// `Then the compositor sends a wl_display.error`
#[then("the compositor sends a wl_display.error")]
async fn compositor_sends_wl_display_error(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // Check for any protocol error from the compositor.
    let log = world.get_serial_log().await;
    if log.contains("wayland-server: xdg_surface error:")
        || log.contains("wayland-server: get_popup rejected")
    {
        eprintln!("│  │  │      ✅ Compositor sent a protocol error");
        Ok(())
    } else {
        Err(StepError(
            "No wl_display.error observed in compositor log — error path not triggered".to_string(),
        ))
    }
}

/// `And the error code indicates role conflict`
#[then("the error code indicates role conflict")]
async fn error_code_role_conflict(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    if log.contains("wayland-server: xdg_surface error: role conflict") {
        eprintln!("│  │  │      ✅ Role conflict error confirmed");
        Ok(())
    } else {
        Err(StepError(
            "Expected 'wayland-server: xdg_surface error: role conflict' in log".to_string(),
        ))
    }
}

/// `Given the client has an xdg_surface`
#[given("the client has an xdg_surface")]
async fn client_has_xdg_surface(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let found = world.wait_for_serial("wayland-server: xdg_surface obj=", 15.0).await;
    if found {
        eprintln!("│  │  │      ✅ xdg_surface is present");
        Ok(())
    } else {
        Err(StepError("No xdg_surface creation observed — precondition not met".to_string()))
    }
}

/// `When the client calls xdg_surface.get_toplevel`
#[when("the client calls xdg_surface.get_toplevel")]
async fn client_calls_get_toplevel(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let found = world.wait_for_serial("wayland-server: xdg_toplevel obj=", 10.0).await;
    if found {
        eprintln!("│  │  │      ✅ get_toplevel call observed");
        Ok(())
    } else {
        Err(StepError("xdg_toplevel assignment not observed in compositor log".to_string()))
    }
}

/// `Then the compositor emits xdg_toplevel.configure with width=0 height=0 and empty states`
#[then("the compositor emits xdg_toplevel.configure with width=0 height=0 and empty states")]
async fn compositor_emits_toplevel_configure(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // get_toplevel triggers SendXdgToplevelConfigure (logged at debug level)
    // and then SendXdgSurfaceConfigure.  The configure serial line is reliable.
    let found = world.wait_for_serial("wayland-server: xdg_surface.configure serial=", 10.0).await;
    if found {
        eprintln!("│  │  │      ✅ xdg_toplevel.configure + xdg_surface.configure emitted");
        Ok(())
    } else {
        Err(StepError("No configure sequence observed after get_toplevel".to_string()))
    }
}

/// `And the compositor then emits xdg_surface.configure with a fresh serial`
#[then("the compositor then emits xdg_surface.configure with a fresh serial")]
async fn compositor_emits_surface_configure(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    if log.contains("wayland-server: xdg_surface.configure serial=") {
        eprintln!("│  │  │      ✅ xdg_surface.configure serial observed");
        Ok(())
    } else {
        Err(StepError("No 'wayland-server: xdg_surface.configure serial=' in log".to_string()))
    }
}

/// `And the serial from xdg_surface.configure is greater than zero`
#[then("the serial from xdg_surface.configure is greater than zero")]
async fn configure_serial_greater_than_zero(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    let re = regex::Regex::new(r"wayland-server: xdg_surface\.configure serial=(\d+)").unwrap();
    if let Some(caps) = re.captures(&log) {
        let serial: u32 = caps[1].parse().unwrap_or(0);
        if serial > 0 {
            eprintln!("│  │  │      ✅ Configure serial={} > 0", serial);
            Ok(())
        } else {
            Err(StepError(format!("Configure serial is {} (expected > 0)", serial)))
        }
    } else {
        Err(StepError("No configure serial found in log to verify".to_string()))
    }
}

/// `Given the client has an xdg_surface that already has a toplevel role`
#[given("the client has an xdg_surface that already has a toplevel role")]
async fn client_has_xdg_surface_with_toplevel_role(
    world: &mut ThingOsWorld,
) -> Result<(), StepError> {
    let found = world.wait_for_serial("wayland-server: xdg_toplevel obj=", 15.0).await;
    if found {
        eprintln!("│  │  │      ✅ xdg_surface with toplevel role exists");
        Ok(())
    } else {
        Err(StepError("No xdg_toplevel assignment observed — precondition not met".to_string()))
    }
}

/// `When the client calls xdg_surface.get_toplevel again`
#[when("the client calls xdg_surface.get_toplevel again")]
async fn client_calls_get_toplevel_again(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // Check if the "already has a role" error has appeared.
    let log = world.get_serial_log().await;
    if log.contains("wayland-server: xdg_surface error:") && log.contains("already has a role") {
        eprintln!("│  │  │      ✅ Double get_toplevel triggered role error");
        Ok(())
    } else {
        Err(StepError(
            "The double-toplevel error path is not exercised: no client called get_toplevel \
             twice on the same xdg_surface. A dedicated error-path client is required."
                .to_string(),
        ))
    }
}

/// `And the error code indicates the surface already has a role`
#[then("the error code indicates the surface already has a role")]
async fn error_code_already_has_role(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    if log.contains("already has a role") {
        eprintln!("│  │  │      ✅ 'already has a role' error confirmed");
        Ok(())
    } else {
        Err(StepError("Expected 'already has a role' error in compositor log".to_string()))
    }
}

/// `Given the client has an xdg_toplevel that has received configure events`
#[given("the client has an xdg_toplevel that has received configure events")]
async fn client_has_toplevel_with_configure(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let found = world.wait_for_serial("wayland-server: xdg_surface.configure serial=", 15.0).await;
    if found {
        eprintln!("│  │  │      ✅ xdg_toplevel has received configure events");
        Ok(())
    } else {
        Err(StepError(
            "No configure event observed — xdg_toplevel precondition not met".to_string(),
        ))
    }
}

/// `And the client has NOT yet sent ack_configure`
#[given("the client has NOT yet sent ack_configure")]
async fn client_has_not_sent_ack_configure(_world: &mut ThingOsWorld) {
    // State assertion: wayland_hello DOES send ack_configure, so this scenario
    // cannot be driven by wayland_hello. Note the precondition and continue.
    eprintln!(
        "│  │  │      ℹ️  Precondition 'not yet ack_configured' cannot be verified via \
         wayland_hello (which always acks). A dedicated error-path client is needed."
    );
}

/// `When the client attaches a shm buffer and calls wl_surface.commit`
#[when("the client attaches a shm buffer and calls wl_surface.commit")]
async fn client_commits_without_ack(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // Check if the unconfigured_buffer error has been triggered.
    let log = world.get_serial_log().await;
    if log.contains("wayland-server: xdg_surface error: buffer committed before ack_configure") {
        eprintln!("│  │  │      ✅ Commit-before-ack error observed");
        return Ok(());
    }
    // wayland_hello always acks before committing, so this error won't appear.
    Err(StepError(
        "The commit-before-ack-configure path is not exercised by wayland_hello. \
         A dedicated error-path client is needed to trigger this error."
            .to_string(),
    ))
}

/// `Then the compositor sends an xdg_surface error for unconfigured_buffer`
#[then("the compositor sends an xdg_surface error for unconfigured_buffer")]
async fn compositor_sends_unconfigured_buffer_error(
    world: &mut ThingOsWorld,
) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    if log.contains("wayland-server: xdg_surface error: buffer committed before ack_configure") {
        eprintln!("│  │  │      ✅ unconfigured_buffer error confirmed");
        Ok(())
    } else {
        Err(StepError("Expected 'unconfigured_buffer' error from compositor".to_string()))
    }
}

/// `Given the client has an xdg_toplevel`
#[given("the client has an xdg_toplevel")]
async fn client_has_xdg_toplevel(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let found = world.wait_for_serial("wayland-server: xdg_toplevel obj=", 60.0).await;
    if found {
        eprintln!("│  │  │      ✅ xdg_toplevel present");
        Ok(())
    } else {
        Err(StepError("No xdg_toplevel observed in compositor log".to_string()))
    }
}

/// `When the client sends xdg_surface.ack_configure with serial 99999`
#[when("the client sends xdg_surface.ack_configure with serial 99999")]
async fn client_sends_invalid_ack_configure(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    if log.contains("wayland-server: xdg_surface error: invalid serial") {
        eprintln!("│  │  │      ✅ Invalid serial error observed");
        Ok(())
    } else {
        Err(StepError(
            "The invalid-serial ack_configure path is not exercised by wayland_hello. \
             A dedicated error-path client is needed."
                .to_string(),
        ))
    }
}

/// `And the error indicates an invalid configure serial`
#[then("the error indicates an invalid configure serial")]
async fn error_indicates_invalid_serial(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    if log.contains("wayland-server: xdg_surface error: invalid serial") {
        eprintln!("│  │  │      ✅ Invalid configure serial error confirmed");
        Ok(())
    } else {
        Err(StepError("Expected 'invalid serial' error in compositor log".to_string()))
    }
}

/// `Given the client has received an xdg_surface.configure with a known serial`
#[given("the client has received an xdg_surface.configure with a known serial")]
async fn client_has_received_configure(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let found = world.wait_for_serial("wayland-server: xdg_surface.configure serial=", 15.0).await;
    if found {
        eprintln!("│  │  │      ✅ Client has received xdg_surface.configure with a serial");
        Ok(())
    } else {
        Err(StepError(
            "No xdg_surface.configure serial observed — precondition not met".to_string(),
        ))
    }
}

/// `When the client sends xdg_surface.ack_configure with that serial`
#[when("the client sends xdg_surface.ack_configure with that serial")]
async fn client_sends_valid_ack_configure(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // wayland_hello sends ack_configure with the received serial.
    let found =
        world.wait_for_serial("wayland-server: xdg_surface.ack_configure serial=", 10.0).await;
    if found {
        eprintln!("│  │  │      ✅ ack_configure sent with valid serial");
        Ok(())
    } else {
        Err(StepError("No ack_configure accepted message in compositor log".to_string()))
    }
}

/// `Then no error is sent` / `Then no protocol error is sent`
#[then("no error is sent")]
async fn no_error_is_sent(world: &mut ThingOsWorld) -> Result<(), StepError> {
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    let log = world.get_serial_log().await;
    if log.contains("wayland-server: xdg_surface error:") {
        Err(StepError(format!(
            "Unexpected protocol error found in compositor log: {}",
            log.lines()
                .filter(|l| l.contains("wayland-server: xdg_surface error:"))
                .collect::<Vec<_>>()
                .join("; ")
        )))
    } else {
        eprintln!("│  │  │      ✅ No protocol errors in compositor log");
        Ok(())
    }
}

#[then("no protocol error is sent")]
async fn no_protocol_error_is_sent(world: &mut ThingOsWorld) -> Result<(), StepError> {
    no_error_is_sent(world).await
}

/// `And the surface is now permitted to commit buffers`
#[then("the surface is now permitted to commit buffers")]
async fn surface_permitted_to_commit(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // ack_configure accepted + no unconfigured_buffer error = permitted to commit.
    let log = world.get_serial_log().await;
    if log.contains("wayland-server: xdg_surface.ack_configure serial=")
        && log.contains("accepted")
        && !log.contains("unconfigured_buffer")
    {
        eprintln!("│  │  │      ✅ Surface is configured and permitted to commit");
        Ok(())
    } else {
        Err(StepError(
            "Surface configure/ack handshake not complete — buffer commit not permitted"
                .to_string(),
        ))
    }
}

/// `Given the client has ack_configured its xdg_surface`
#[given("the client has ack_configured its xdg_surface")]
async fn client_has_ack_configured(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let found =
        world.wait_for_serial("wayland-server: xdg_surface.ack_configure serial=", 15.0).await;
    if found {
        eprintln!("│  │  │      ✅ ack_configure accepted — surface is configured");
        Ok(())
    } else {
        Err(StepError("No ack_configure observed — precondition not met".to_string()))
    }
}

/// `And the client has attached a valid wl_shm buffer`
#[given("the client has attached a valid wl_shm buffer")]
async fn client_has_attached_shm_buffer(_world: &mut ThingOsWorld) {
    // wayland_hello attaches shm buffers; this is a precondition assertion.
    eprintln!("│  │  │      ℹ️  wl_shm buffer attached by wayland_hello");
}

/// `When the client calls wl_surface.commit`
#[when("the client calls wl_surface.commit")]
async fn client_calls_wl_surface_commit(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // wayland_hello commits after ack. Check for mapping readiness.
    let found = world.wait_for_serial("wayland-server: surface ", 10.0).await;
    if found {
        eprintln!("│  │  │      ✅ wl_surface.commit processed by compositor");
        Ok(())
    } else {
        Err(StepError("Surface commit not observed in compositor log".to_string()))
    }
}

/// `And the compositor marks the surface as eligible for mapping`
#[then("the compositor marks the surface as eligible for mapping")]
async fn compositor_marks_surface_for_mapping(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let found = world.wait_for_serial("wayland-server: surface ", 10.0).await;
    let log = world.get_serial_log().await;
    if found && log.contains("ready for mapping") {
        eprintln!("│  │  │      ✅ Surface marked ready for mapping");
        Ok(())
    } else {
        Err(StepError(
            "Expected 'wayland-server: surface N ready for mapping' in compositor log".to_string(),
        ))
    }
}

#[then("the Wayland hello client should be visible")]
async fn wayland_hello_client_visible(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let _ = world.wait_for_serial("First frame rendered", 60.0).await;

    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(30);
    let mut last_counts = (0u32, 0u32, 0u32);
    let mut attempt = 0u32;

    while start.elapsed() < timeout {
        attempt += 1;
        let screenshot_path = crate::artifacts::global()
            .lock()
            .await
            .screenshot_path(&format!("wayland_hello_visible_{}", attempt));
        let png_path = world
            .take_screenshot(&screenshot_path)
            .await
            .map_err(|e| StepError(format!("Failed to take screenshot: {}", e)))?;

        let img = image::open(&png_path)
            .map_err(|e| StepError(format!("Failed to open screenshot: {}", e)))?
            .to_rgb8();
        let (width, height) = img.dimensions();
        let max_x = width.min(520);
        let max_y = height.min(360);
        let mut title_pixels = 0u32;
        let mut body_pixels = 0u32;
        let mut text_pixels = 0u32;

        for y in 0..max_y {
            for x in 0..max_x {
                let [r, g, b] = img.get_pixel(x, y).0;
                let pixel = [r, g, b];
                if is_window_chrome_pixel(pixel) {
                    title_pixels += 1;
                } else if is_paper_pixel(pixel) {
                    body_pixels += 1;
                } else if is_paper_text_pixel(pixel) {
                    text_pixels += 1;
                }
            }
        }

        last_counts = (title_pixels, body_pixels, text_pixels);
        if title_pixels > 2_000 && body_pixels > 25_000 && text_pixels > 80 {
            eprintln!(
                "│  │  │      ✅ Wayland hello client visible (title={}, body={}, text={})",
                title_pixels, body_pixels, text_pixels
            );
            return Ok(());
        }

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }

    Err(StepError(format!(
        "Wayland hello client was not visible above the background (title={}, body={}, text={})",
        last_counts.0, last_counts.1, last_counts.2
    )))
}

#[then("the compositor should render themed window bodies")]
async fn compositor_should_render_themed_window_bodies(
    world: &mut ThingOsWorld,
) -> Result<(), StepError> {
    if world.wait_for_serial("bloom: flat window overlays ready", 30.0).await {
        eprintln!("│  │  │      ✅ Bloom rendered the theme-owned window body overlay");
        Ok(())
    } else {
        Err(StepError("Expected Bloom to allocate the theme-owned window body overlay".to_string()))
    }
}

#[then("active window chrome button glyphs should be centered inside their buttons")]
async fn active_window_chrome_button_glyphs_are_centered_inside_their_buttons(
    world: &mut ThingOsWorld,
) -> Result<(), StepError> {
    let _ = world.wait_for_serial("First frame rendered", 60.0).await;

    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(30);
    let mut last_counts = (0u32, 0u32, 0u32, 0u32, 0u32);
    let mut attempt = 0u32;

    while start.elapsed() < timeout {
        attempt += 1;
        let screenshot_path = crate::artifacts::global()
            .lock()
            .await
            .screenshot_path(&format!("active_chrome_gold_{}", attempt));
        let png_path = world
            .take_screenshot(&screenshot_path)
            .await
            .map_err(|e| StepError(format!("Failed to take screenshot: {}", e)))?;

        let img = image::open(&png_path)
            .map_err(|e| StepError(format!("Failed to open screenshot: {}", e)))?
            .to_rgb8();
        let (width, height) = img.dimensions();
        let max_x = width.min(520);
        let max_y = height.min(80);
        let mut gold_pixels = 0u32;
        let mut dark_text_pixels = 0u32;
        let mut icon_pixels_high_in_frame = 0u32;
        let mut icon_pixels_in_button_band = 0u32;
        let mut button_icon_pixels = 0u32;

        for y in 0..max_y {
            for x in 0..max_x {
                let pixel = img.get_pixel(x, y).0;
                if is_brass_pixel(pixel) {
                    gold_pixels += 1;
                } else if is_paper_text_pixel(pixel) {
                    dark_text_pixels += 1;
                }
                if x >= 340 && y < 40 && is_paper_text_pixel(pixel) {
                    button_icon_pixels += 1;
                    if y < 7 {
                        icon_pixels_high_in_frame += 1;
                    }
                    if (11..=38).contains(&y) {
                        icon_pixels_in_button_band += 1;
                    }
                }
            }
        }

        last_counts = (
            gold_pixels,
            dark_text_pixels,
            icon_pixels_high_in_frame,
            icon_pixels_in_button_band,
            button_icon_pixels,
        );
        if gold_pixels >= 4
            && dark_text_pixels > 40
            && icon_pixels_high_in_frame < 500
            && icon_pixels_in_button_band > 20
            && button_icon_pixels > 20
        {
            eprintln!(
                "│  │  │      ✅ Active chrome button glyphs are centered inside their buttons (gold={}, dark_text={}, icon_high_in_frame={}, icon_in_button_band={}, button_icons={})",
                gold_pixels,
                dark_text_pixels,
                icon_pixels_high_in_frame,
                icon_pixels_in_button_band,
                button_icon_pixels
            );
            return Ok(());
        }

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }

    Err(StepError(format!(
        "Active chrome button glyphs were not centered inside their buttons (gold={}, dark_text={}, icon_high_in_frame={}, icon_in_button_band={}, button_icons={})",
        last_counts.0, last_counts.1, last_counts.2, last_counts.3, last_counts.4
    )))
}

#[then("active window chrome should be rendered with flat thick borders")]
async fn active_window_chrome_should_be_rendered_with_flat_thick_borders(
    world: &mut ThingOsWorld,
) -> Result<(), StepError> {
    let _ = world.wait_for_serial("First frame rendered", 60.0).await;

    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(30);
    let mut last = (0u32, 0u32, 0u32, 0u32);
    let mut attempt = 0u32;

    while start.elapsed() < timeout {
        attempt += 1;
        let screenshot_path = crate::artifacts::global()
            .lock()
            .await
            .screenshot_path(&format!("active_chrome_rounded_{}", attempt));
        let png_path = world
            .take_screenshot(&screenshot_path)
            .await
            .map_err(|e| StepError(format!("Failed to take screenshot: {}", e)))?;

        let img = image::open(&png_path)
            .map_err(|e| StepError(format!("Failed to open screenshot: {}", e)))?
            .to_rgb8();
        let (width, height) = img.dimensions();
        let max_x = width.min(640);
        let max_y = height.min(160);

        let mut best_row = 0u32;
        let mut best_count = 0u32;
        for y in 0..max_y {
            let mut count = 0u32;
            for x in 0..max_x {
                if is_window_chrome_pixel(img.get_pixel(x, y).0) {
                    count += 1;
                }
            }
            if count > best_count {
                best_count = count;
                best_row = y;
            }
        }
        if best_count < 200 {
            last = (best_row, best_count, 0, 0);
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            continue;
        }

        let mut thick_rows = 0u32;
        for y in 0..max_y {
            let mut count = 0u32;
            for x in 0..max_x {
                if is_window_chrome_pixel(img.get_pixel(x, y).0) {
                    count += 1;
                }
            }
            if count >= 200 {
                thick_rows += 1;
            }
        }
        last = (best_row, best_count, thick_rows, 0);
        if best_count >= 300 && thick_rows >= 6 {
            eprintln!(
                "│  │  │      Active chrome has flat thick border geometry (row={}, row_pixels={}, thick_rows={})",
                best_row, best_count, thick_rows
            );
            return Ok(());
        }

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }

    Err(StepError(format!(
        "Active chrome did not have flat thick border geometry (best_row={}, row_pixels={}, thick_rows={}, unused={})",
        last.0, last.1, last.2, last.3
    )))
}

#[then("active window chrome should include facet frame focus accents")]
async fn active_window_chrome_should_include_facet_frame_focus_accents(
    world: &mut ThingOsWorld,
) -> Result<(), StepError> {
    let _ = world.wait_for_serial("First frame rendered", 60.0).await;

    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(30);
    let mut last = (0u32, 0u32, 0u32, 0u32);
    let mut attempt = 0u32;

    while start.elapsed() < timeout {
        attempt += 1;
        let screenshot_path = crate::artifacts::global()
            .lock()
            .await
            .screenshot_path(&format!("active_chrome_facet_frame_{}", attempt));
        let png_path = world
            .take_screenshot(&screenshot_path)
            .await
            .map_err(|e| StepError(format!("Failed to take screenshot: {}", e)))?;

        let img = image::open(&png_path)
            .map_err(|e| StepError(format!("Failed to open screenshot: {}", e)))?
            .to_rgb8();
        let (width, height) = img.dimensions();
        let max_x = width.min(640);
        let max_y = height.min(180);

        let mut facet_pixels = 0u32;
        let mut gold_pixels = 0u32;
        let mut edge_light_pixels = 0u32;
        let mut inner_line_pixels = 0u32;
        for y in 0..max_y {
            for x in 0..max_x {
                let pixel = img.get_pixel(x, y).0;
                if color_close(pixel, [0x23, 0x1A, 0x33], 18) {
                    facet_pixels += 1;
                }
                if is_brass_pixel(pixel) {
                    gold_pixels += 1;
                }
                if color_close(pixel, [0x7C, 0x5C, 0xFF], 24) {
                    edge_light_pixels += 1;
                }
                if color_close(pixel, [0x2A, 0x1F, 0x3A], 18) {
                    inner_line_pixels += 1;
                }
            }
        }

        last = (facet_pixels, gold_pixels, edge_light_pixels, inner_line_pixels);
        if facet_pixels >= 30
            && gold_pixels >= 4
            && edge_light_pixels >= 20
            && inner_line_pixels >= 120
        {
            eprintln!(
                "│  │  │      ✅ Active chrome facet frame accents visible (facet={}, gold={}, edge={}, inner={})",
                facet_pixels, gold_pixels, edge_light_pixels, inner_line_pixels
            );
            return Ok(());
        }

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }

    Err(StepError(format!(
        "Active chrome facet frame accents were not visible (facet={}, gold={}, edge={}, inner={})",
        last.0, last.1, last.2, last.3
    )))
}

#[when("I drag the Clock window over the Wayland hello title bar")]
async fn drag_clock_window_over_wayland_hello_title_bar(
    world: &mut ThingOsWorld,
) -> Result<(), StepError> {
    if world.qmp_control.is_none() {
        return Err(StepError("No QMP connection for clock-window drag input".to_string()));
    }
    if !world.wait_for_serial("bloom: registered titlebar drag zone", 60.0).await {
        return Err(StepError("Bloom did not register compositor chrome".to_string()));
    }
    if !world.wait_for_serial("ps2_mouse: bristle pid=", 60.0).await {
        return Err(StepError("PS/2 mouse driver did not connect to Bristle".to_string()));
    }
    if !world.wait_for_serial("bloom: registered bristle pointer sink", 60.0).await {
        return Err(StepError("Bloom did not register its Bristle pointer sink".to_string()));
    }

    let windows = read_wayland_windows(world).await?;
    let hello = find_window(&windows, "Thing-OS Wayland Lab")?;
    let clock = find_window(&windows, "Clock")?;
    let target_x = hello.x.saturating_add(40);
    let target_y = hello.y.saturating_sub(20);
    let start_x = clock.x.saturating_add(24);
    let start_y = clock.y.saturating_add(14);
    let commands = drag_pointer(start_x, start_y, target_x, target_y, 1);
    send_qmp_sequence(world, &commands, "clock-window drag")
        .await
        .map_err(|e| StepError(e.to_string()))?;
    tokio::time::sleep(std::time::Duration::from_millis(1_500)).await;
    Ok(())
}

#[then("higher z-order window content should obscure lower window chrome")]
async fn higher_z_content_obscures_lower_window_chrome(
    world: &mut ThingOsWorld,
) -> Result<(), StepError> {
    let windows = read_wayland_windows(world).await?;
    let hello = find_window(&windows, "Thing-OS Wayland Lab")?;
    let clock = find_window(&windows, "Clock")?;
    let hello_title = TestRect {
        x: hello.x.saturating_add(8),
        y: hello.y.saturating_add(6),
        w: hello.w.saturating_sub(16),
        h: 18,
    };
    let clock_content_band = TestRect {
        x: clock.x.saturating_add(40),
        y: clock.y.saturating_add(32),
        w: clock.w.saturating_sub(80),
        h: 18,
    };
    let overlap = intersect_test_rect(hello_title, clock_content_band).ok_or_else(|| {
        StepError(format!(
            "Clock content did not overlap Wayland hello title bar (hello={}x{}+{},{} clock={}x{}+{},{})",
            hello.w, hello.h, hello.x, hello.y, clock.w, clock.h, clock.x, clock.y
        ))
    })?;

    let screenshot_path =
        crate::artifacts::global().lock().await.screenshot_path("chrome_z_order_overlap");
    let png_path = world
        .take_screenshot(&screenshot_path)
        .await
        .map_err(|e| StepError(format!("Failed to take screenshot: {}", e)))?;
    let img = image::open(&png_path)
        .map_err(|e| StepError(format!("Failed to open screenshot: {}", e)))?
        .to_rgb8();

    let mut chrome_pixels = 0u32;
    let mut body_pixels = 0u32;
    let mut total = 0u32;
    for y in overlap.y.max(0) as u32..overlap.y.max(0) as u32 + overlap.h {
        for x in overlap.x.max(0) as u32..overlap.x.max(0) as u32 + overlap.w {
            if x >= img.width() || y >= img.height() {
                continue;
            }
            let pixel = img.get_pixel(x, y).0;
            total += 1;
            if is_window_chrome_pixel(pixel) {
                chrome_pixels += 1;
            }
            if is_paper_pixel(pixel) {
                body_pixels += 1;
            }
        }
    }

    if total > 0 && chrome_pixels.saturating_mul(10) < total && body_pixels > total / 3 {
        eprintln!(
            "│  │  │      ✅ Higher z-order content hides lower chrome (chrome={} body={} total={})",
            chrome_pixels, body_pixels, total
        );
        return Ok(());
    }

    Err(StepError(format!(
        "Lower window chrome leaked through higher z-order content (chrome={} body={} total={})",
        chrome_pixels, body_pixels, total
    )))
}

#[when("I drag the Wayland hello title bar")]
async fn drag_wayland_hello_title_bar(world: &mut ThingOsWorld) -> Result<(), StepError> {
    if world.qmp_control.is_none() {
        return Err(StepError("No QMP connection for title-bar drag input".to_string()));
    }
    if !world.wait_for_serial("bloom: registered titlebar drag zone", 60.0).await {
        return Err(StepError("Bloom did not register a title-bar drag zone".to_string()));
    }
    if !world.wait_for_serial("ps2_mouse: bristle pid=", 60.0).await {
        return Err(StepError("PS/2 mouse driver did not connect to Bristle".to_string()));
    }
    if !world.wait_for_serial("bloom: registered bristle pointer sink", 60.0).await {
        return Err(StepError("Bloom did not register its Bristle pointer sink".to_string()));
    }

    let commands = [
        (
            r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "rel", "data": {"axis": "x", "value": -10000}}, {"type": "rel", "data": {"axis": "y", "value": -10000}}]}}"#,
            1_000,
        ),
        (
            r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "rel", "data": {"axis": "x", "value": 48}}, {"type": "rel", "data": {"axis": "y", "value": 20}}]}}"#,
            500,
        ),
        (
            r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "btn", "data": {"down": true, "button": "left"}}]}}"#,
            200,
        ),
        (
            r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "rel", "data": {"axis": "x", "value": 80}}, {"type": "rel", "data": {"axis": "y", "value": 48}}]}}"#,
            500,
        ),
        (
            r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "btn", "data": {"down": false, "button": "left"}}]}}"#,
            100,
        ),
    ];

    for (command, settle_ms) in commands {
        world
            .execute_qmp_control(command)
            .await
            .map_err(|e| StepError(format!("QMP title-bar drag failed: {}", e)))?;
        tokio::time::sleep(std::time::Duration::from_millis(settle_ms)).await;
    }
    Ok(())
}

#[then("the compositor should move the toplevel window")]
async fn compositor_moves_toplevel_window(world: &mut ThingOsWorld) -> Result<(), StepError> {
    if !world.wait_for_serial("bloom: window drag started", 30.0).await {
        return Err(StepError("Bloom did not start a title-bar drag".to_string()));
    }
    if !world.wait_for_serial("bloom: window drag moved", 30.0).await {
        return Err(StepError("Bloom did not move the dragged window".to_string()));
    }
    if !world.wait_for_serial("bloom: window drag ended", 30.0).await {
        return Err(StepError("Bloom did not end the title-bar drag".to_string()));
    }
    Ok(())
}

#[then("the dragged window should keep a stable cursor offset")]
async fn dragged_window_keeps_stable_cursor_offset(
    world: &mut ThingOsWorld,
) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    let start_re = regex::Regex::new(
        r"bloom: window drag started surface=(\d+) pointer=(-?\d+),(-?\d+) offset=(-?\d+),(-?\d+)",
    )
    .unwrap();
    let move_re = regex::Regex::new(
        r"bloom: window drag moved surface=(\d+) to (-?\d+),(-?\d+) pointer=(-?\d+),(-?\d+) offset=(-?\d+),(-?\d+)",
    )
    .unwrap();

    let start = start_re
        .captures_iter(&log)
        .last()
        .ok_or_else(|| StepError("No drag-start offset log found".to_string()))?;
    let surface_id = start[1].to_string();
    let start_offset_x = start[4]
        .parse::<i32>()
        .map_err(|e| StepError(format!("Invalid drag-start x offset: {}", e)))?;
    let start_offset_y = start[5]
        .parse::<i32>()
        .map_err(|e| StepError(format!("Invalid drag-start y offset: {}", e)))?;

    let moved = move_re
        .captures_iter(&log)
        .filter(|caps| caps.get(1).map(|m| m.as_str()) == Some(surface_id.as_str()))
        .last()
        .ok_or_else(|| StepError("No drag-move offset log found".to_string()))?;
    let move_offset_x = moved[6]
        .parse::<i32>()
        .map_err(|e| StepError(format!("Invalid drag-move x offset: {}", e)))?;
    let move_offset_y = moved[7]
        .parse::<i32>()
        .map_err(|e| StepError(format!("Invalid drag-move y offset: {}", e)))?;

    if (move_offset_x, move_offset_y) != (start_offset_x, start_offset_y) {
        return Err(StepError(format!(
            "Dragged window cursor offset changed from {},{} to {},{}",
            start_offset_x, start_offset_y, move_offset_x, move_offset_y
        )));
    }

    Ok(())
}

#[when("I click inside the Wayland hello client and press A")]
async fn click_wayland_hello_client_and_press_a(world: &mut ThingOsWorld) -> Result<(), StepError> {
    if world.qmp_control.is_none() {
        return Err(StepError("No QMP connection for Wayland input".to_string()));
    }
    if !world.wait_for_serial("ps2_mouse: bristle pid=", 60.0).await {
        return Err(StepError("PS/2 mouse driver did not connect to Bristle".to_string()));
    }
    if !world.wait_for_serial("ps2_kbd: bristle pid=", 60.0).await {
        return Err(StepError("PS/2 keyboard driver did not connect to Bristle".to_string()));
    }
    if !world.wait_for_serial("bloom: registered bristle pointer sink", 60.0).await {
        return Err(StepError("Bloom did not register its Bristle pointer sink".to_string()));
    }

    let commands = [
        (
            r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "rel", "data": {"axis": "x", "value": -10000}}, {"type": "rel", "data": {"axis": "y", "value": -10000}}]}}"#,
            1_000,
        ),
        (
            r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "rel", "data": {"axis": "x", "value": 160}}, {"type": "rel", "data": {"axis": "y", "value": 96}}]}}"#,
            500,
        ),
        (
            r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "btn", "data": {"down": true, "button": "left"}}]}}"#,
            150,
        ),
        (
            r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "btn", "data": {"down": false, "button": "left"}}]}}"#,
            150,
        ),
        (
            r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "key", "data": {"down": true, "key": {"type": "qcode", "data": "a"}}}]}}"#,
            80,
        ),
        (
            r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "key", "data": {"down": false, "key": {"type": "qcode", "data": "a"}}}]}}"#,
            200,
        ),
    ];

    for (command, settle_ms) in commands {
        world
            .execute_qmp_control(command)
            .await
            .map_err(|e| StepError(format!("QMP Wayland input failed: {}", e)))?;
        tokio::time::sleep(std::time::Duration::from_millis(settle_ms)).await;
    }
    Ok(())
}

#[then("the Wayland hello client should receive pointer and keyboard input")]
async fn wayland_hello_receives_pointer_and_keyboard(
    world: &mut ThingOsWorld,
) -> Result<(), StepError> {
    if !world.wait_for_serial("wayland_hello: pointer enter", 30.0).await {
        return Err(StepError("Wayland client did not receive wl_pointer.enter".to_string()));
    }
    if !world.wait_for_serial("wayland_hello: pointer button", 30.0).await {
        return Err(StepError("Wayland client did not receive wl_pointer.button".to_string()));
    }
    if !world.wait_for_serial("wayland_hello: keyboard enter", 30.0).await {
        return Err(StepError("Wayland client did not receive wl_keyboard.enter".to_string()));
    }
    if !world.wait_for_serial("wayland_hello: keyboard key", 30.0).await {
        return Err(StepError("Wayland client did not receive wl_keyboard.key".to_string()));
    }
    Ok(())
}

#[when("I drag the Wayland hello frame")]
async fn drag_wayland_hello_frame(world: &mut ThingOsWorld) -> Result<(), StepError> {
    if world.qmp_control.is_none() {
        return Err(StepError("No QMP connection for frame resize input".to_string()));
    }
    if !world.wait_for_serial("bloom: registered titlebar drag zone", 60.0).await {
        return Err(StepError("Bloom did not register compositor chrome".to_string()));
    }
    if !world.wait_for_serial("ps2_mouse: bristle pid=", 60.0).await {
        return Err(StepError("PS/2 mouse driver did not connect to Bristle".to_string()));
    }
    if !world.wait_for_serial("bloom: registered bristle pointer sink", 60.0).await {
        return Err(StepError("Bloom did not register its Bristle pointer sink".to_string()));
    }

    let commands = [
        (
            r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "rel", "data": {"axis": "x", "value": -10000}}, {"type": "rel", "data": {"axis": "y", "value": -10000}}]}}"#,
            1_000,
        ),
        (
            r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "rel", "data": {"axis": "x", "value": 476}}, {"type": "rel", "data": {"axis": "y", "value": 316}}]}}"#,
            500,
        ),
        (
            r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "btn", "data": {"down": true, "button": "left"}}]}}"#,
            200,
        ),
        (
            r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "rel", "data": {"axis": "x", "value": 64}}, {"type": "rel", "data": {"axis": "y", "value": 48}}]}}"#,
            500,
        ),
        (
            r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "btn", "data": {"down": false, "button": "left"}}]}}"#,
            100,
        ),
    ];

    for (command, settle_ms) in commands {
        world
            .execute_qmp_control(command)
            .await
            .map_err(|e| StepError(format!("QMP frame resize failed: {}", e)))?;
        tokio::time::sleep(std::time::Duration::from_millis(settle_ms)).await;
    }
    Ok(())
}

#[then("the compositor should resize the toplevel window")]
async fn compositor_resizes_toplevel_window(world: &mut ThingOsWorld) -> Result<(), StepError> {
    if !world.wait_for_serial("bloom: window resize started", 30.0).await {
        return Err(StepError("Bloom did not start a frame resize".to_string()));
    }
    if !world.wait_for_serial("bloom: window resize moved", 30.0).await {
        return Err(StepError("Bloom did not resize the dragged window".to_string()));
    }
    if !world.wait_for_serial("wayland-server: configured surface=", 30.0).await {
        return Err(StepError("Wayland server did not send a resize configure".to_string()));
    }
    if !world.wait_for_serial("bloom: window resize ended", 30.0).await {
        return Err(StepError("Bloom did not end the frame resize".to_string()));
    }
    Ok(())
}

async fn click_wayland_hello_chrome_button(
    world: &mut ThingOsWorld,
    name: &str,
    x: i32,
) -> Result<(), StepError> {
    if world.qmp_control.is_none() {
        return Err(StepError(format!("No QMP connection for {} button input", name)));
    }
    if !world.wait_for_serial("bloom: registered titlebar drag zone", 60.0).await {
        return Err(StepError("Bloom did not register compositor chrome".to_string()));
    }
    if !world.wait_for_serial("ps2_mouse: bristle pid=", 60.0).await {
        return Err(StepError("PS/2 mouse driver did not connect to Bristle".to_string()));
    }
    if !world.wait_for_serial("bloom: registered bristle pointer sink", 60.0).await {
        return Err(StepError("Bloom did not register its Bristle pointer sink".to_string()));
    }

    let move_to_button = format!(
        r#"{{"execute": "input-send-event", "arguments": {{"events": [{{"type": "rel", "data": {{"axis": "x", "value": {}}}}}, {{"type": "rel", "data": {{"axis": "y", "value": 20}}}}]}}}}"#,
        x
    );
    let commands = [
        (
            r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "rel", "data": {"axis": "x", "value": -10000}}, {"type": "rel", "data": {"axis": "y", "value": -10000}}]}}"#.to_string(),
            1_000,
        ),
        (move_to_button, 500),
        (
            r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "btn", "data": {"down": true, "button": "left"}}]}}"#.to_string(),
            200,
        ),
        (
            r#"{"execute": "input-send-event", "arguments": {"events": [{"type": "btn", "data": {"down": false, "button": "left"}}]}}"#.to_string(),
            200,
        ),
    ];

    for (command, settle_ms) in commands {
        world
            .execute_qmp_control(&command)
            .await
            .map_err(|e| StepError(format!("QMP {} button click failed: {}", name, e)))?;
        tokio::time::sleep(std::time::Duration::from_millis(settle_ms)).await;
    }
    Ok(())
}

#[when("I click the Wayland hello maximize button")]
async fn click_wayland_hello_maximize_button(world: &mut ThingOsWorld) -> Result<(), StepError> {
    click_wayland_hello_chrome_button(world, "maximize", 424).await
}

#[then("the compositor should send a maximized toplevel configure")]
async fn compositor_sends_maximized_configure(world: &mut ThingOsWorld) -> Result<(), StepError> {
    if !world.wait_for_serial("bloom: maximize button pressed", 30.0).await {
        return Err(StepError("Bloom did not handle the maximize chrome button".to_string()));
    }
    if !world.wait_for_serial("wayland-server: maximized surface=", 30.0).await {
        return Err(StepError("Wayland server did not send a maximized configure".to_string()));
    }
    Ok(())
}

#[when("I click the Wayland hello minimize button")]
async fn click_wayland_hello_minimize_button(world: &mut ThingOsWorld) -> Result<(), StepError> {
    click_wayland_hello_chrome_button(world, "minimize", 392).await
}

#[then("the compositor should minimize the toplevel window")]
async fn compositor_minimizes_toplevel_window(world: &mut ThingOsWorld) -> Result<(), StepError> {
    if !world.wait_for_serial("bloom: minimize button pressed", 30.0).await {
        return Err(StepError("Bloom did not handle the minimize chrome button".to_string()));
    }
    if !world.wait_for_serial("wayland-server: minimized surface=", 30.0).await {
        return Err(StepError("Wayland server did not observe the minimize action".to_string()));
    }
    Ok(())
}

#[when("I click the Wayland hello close button")]
async fn click_wayland_hello_close_button(world: &mut ThingOsWorld) -> Result<(), StepError> {
    click_wayland_hello_chrome_button(world, "close", 456).await
}

#[then("the compositor should send xdg_toplevel.close")]
async fn compositor_sends_toplevel_close(world: &mut ThingOsWorld) -> Result<(), StepError> {
    if !world.wait_for_serial("bloom: close button pressed", 30.0).await {
        return Err(StepError("Bloom did not handle the close chrome button".to_string()));
    }
    if !world.wait_for_serial("wayland-server: sent xdg_toplevel.close", 30.0).await {
        return Err(StepError("Wayland server did not send xdg_toplevel.close".to_string()));
    }
    if !world.wait_for_serial("wayland_hello: compositor requested close", 30.0).await {
        return Err(StepError("Wayland client did not receive xdg_toplevel.close".to_string()));
    }
    Ok(())
}

#[then("the compositor should send zwlr_layer_surface_v1.closed")]
async fn compositor_sends_layer_surface_closed(world: &mut ThingOsWorld) -> Result<(), StepError> {
    if !world.wait_for_serial("wayland-server: sent zwlr_layer_surface_v1.closed", 30.0).await {
        return Err(StepError(
            "Wayland server did not send zwlr_layer_surface_v1.closed".to_string(),
        ));
    }
    Ok(())
}

/// `When the client sends xdg_toplevel.set_title "..."` (regex)
#[when(regex = r#"^the client sends xdg_toplevel\.set_title "(.+)"$"#)]
async fn client_sends_set_title(world: &mut ThingOsWorld, title: String) -> Result<(), StepError> {
    let pattern = format!("wayland-server: xdg_toplevel obj=");
    // Wait briefly for the log to appear before checking.
    let _ = world.wait_for_serial(&pattern, 10.0).await;
    // wayland_hello sets title="Thing-OS XDG Demo".
    let log = world.get_serial_log().await;
    if log.contains("wayland-server: xdg_toplevel obj=") && log.contains("title=") {
        eprintln!("│  │  │      ✅ set_title observed in compositor log");
        Ok(())
    } else {
        Err(StepError(format!("set_title '{}' not observed in compositor log", title)))
    }
}

/// `And the client sends xdg_toplevel.set_app_id "..."` (regex)
#[when(regex = r#"^the client sends xdg_toplevel\.set_app_id "(.+)"$"#)]
async fn client_sends_set_app_id(
    world: &mut ThingOsWorld,
    app_id: String,
) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    if log.contains("wayland-server: xdg_toplevel obj=") && log.contains("app_id=") {
        eprintln!("│  │  │      ✅ set_app_id observed in compositor log");
        Ok(())
    } else {
        Err(StepError(format!("set_app_id '{}' not observed in compositor log", app_id)))
    }
}

/// `And the compositor records the title as "..."` (regex)
#[then(regex = r#"^the compositor records the title as "(.+)"$"#)]
async fn compositor_records_title(
    world: &mut ThingOsWorld,
    title: String,
) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    let pattern = format!("title=\"{}\"", title);
    if log.contains(&pattern) {
        eprintln!("│  │  │      ✅ Compositor recorded title \"{}\"", title);
        Ok(())
    } else {
        // wayland_hello uses "Thing-OS XDG Demo", not "My App", so this will fail.
        Err(StepError(format!(
            "Compositor did not record title \"{}\". Found: {:?}",
            title,
            log.lines().filter(|l| l.contains("title=")).collect::<Vec<_>>()
        )))
    }
}

/// `And the compositor records the app_id as "..."` (regex)
#[then(regex = r#"^the compositor records the app_id as "(.+)"$"#)]
async fn compositor_records_app_id(
    world: &mut ThingOsWorld,
    app_id: String,
) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    let pattern = format!("app_id=\"{}\"", app_id);
    if log.contains(&pattern) {
        eprintln!("│  │  │      ✅ Compositor recorded app_id \"{}\"", app_id);
        Ok(())
    } else {
        Err(StepError(format!(
            "Compositor did not record app_id \"{}\". Found: {:?}",
            app_id,
            log.lines().filter(|l| l.contains("app_id=")).collect::<Vec<_>>()
        )))
    }
}

/// `Given the compositor has sent an xdg_wm_base.ping with a serial`
#[given("the compositor has sent an xdg_wm_base.ping with a serial")]
async fn compositor_sent_ping(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // bloom periodically sends pings.  Wait for one to appear in the log.
    let found = world.wait_for_serial("wayland-server: xdg_wm_base.ping serial=", 30.0).await;
    if found {
        eprintln!("│  │  │      ✅ xdg_wm_base.ping sent by compositor");
        Ok(())
    } else {
        // Ping is sent by the compositor on a timer or when it decides to.
        // If wayland_hello is running and connected, a ping may be sent.
        Err(StepError("No xdg_wm_base.ping observed in compositor log within 30s".to_string()))
    }
}

/// `When the client responds with xdg_wm_base.pong using the same serial`
#[when("the client responds with xdg_wm_base.pong using the same serial")]
async fn client_responds_with_pong(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // wayland_hello sends pong in response to ping.
    let found = world.wait_for_serial("wayland-server: xdg_wm_base.pong serial=", 10.0).await;
    if found {
        eprintln!("│  │  │      ✅ xdg_wm_base.pong received by compositor");
        Ok(())
    } else {
        Err(StepError("No xdg_wm_base.pong observed in compositor log".to_string()))
    }
}

/// `When the client sends xdg_toplevel.destroy`
#[when("the client sends xdg_toplevel.destroy")]
async fn client_sends_toplevel_destroy(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // Wait briefly for any outstanding toplevel logs.
    let _ = world.wait_for_serial("wayland-server: xdg_toplevel obj=", 5.0).await;
    // wayland_hello doesn't explicitly destroy; the compositor gets client disconnected.
    // Check for toplevel destroy log (wayland_hello may stay running).
    let log = world.get_serial_log().await;
    if log.contains("wayland-server: xdg_toplevel obj=") && log.contains("destroyed") {
        eprintln!("│  │  │      ✅ xdg_toplevel destroyed");
        Ok(())
    } else {
        Err(StepError(
            "xdg_toplevel.destroy not observed — a dedicated teardown client is needed".to_string(),
        ))
    }
}

/// `And the client sends xdg_surface.destroy`
#[when("the client sends xdg_surface.destroy")]
async fn client_sends_surface_destroy(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    if log.contains("wayland-server: xdg_surface obj=") && log.contains("destroyed") {
        eprintln!("│  │  │      ✅ xdg_surface destroyed");
        Ok(())
    } else {
        Err(StepError(
            "xdg_surface.destroy not observed — a dedicated teardown client is needed".to_string(),
        ))
    }
}

/// `And the compositor no longer tracks any state for those objects`
#[then("the compositor no longer tracks any state for those objects")]
async fn compositor_no_longer_tracks(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    if (log.contains("xdg_toplevel obj=") && log.contains("destroyed"))
        || log.contains("client disconnected")
    {
        eprintln!("│  │  │      ✅ Compositor cleaned up xdg objects");
        Ok(())
    } else {
        Err(StepError(
            "No evidence that compositor cleaned up xdg_toplevel/xdg_surface state".to_string(),
        ))
    }
}

/// `When the client calls xdg_surface.get_popup`
#[when("the client calls xdg_surface.get_popup")]
async fn client_calls_get_popup(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let found = world.wait_for_serial("wayland-server: xdg_popup obj=", 10.0).await;
    if found {
        eprintln!("│  │  │      ✅ get_popup call created xdg_popup");
        Ok(())
    } else {
        Err(StepError("xdg_popup creation not observed in compositor log".to_string()))
    }
}

/// `Then a new xdg_popup object is registered successfully`
#[then("a new xdg_popup object is registered successfully")]
async fn xdg_popup_registered(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    if log.contains("wayland-server: xdg_popup obj=")
        && log.contains("assigned to xdg_surface=")
        && !log.contains("wayland-server: xdg_surface.get_popup rejected")
    {
        eprintln!("│  │  │      ✅ xdg_popup registered successfully");
        Ok(())
    } else {
        Err(StepError("No successful xdg_popup registration observed".to_string()))
    }
}

/// `And the compositor emits xdg_popup.configure before xdg_surface.configure`
#[then("the compositor emits xdg_popup.configure before xdg_surface.configure")]
async fn xdg_popup_configure_precedes_surface_configure(
    world: &mut ThingOsWorld,
) -> Result<(), StepError> {
    let popup_found = world.wait_for_serial("wayland-server: xdg_popup.configure obj=", 10.0).await;
    let surface_found = world.wait_for_serial("sent to obj=21", 10.0).await;
    if !popup_found {
        return Err(StepError("No xdg_popup.configure observed".to_string()));
    }
    if !surface_found {
        return Err(StepError("No xdg_surface.configure observed after popup".to_string()));
    }

    let log = world.get_serial_log().await;
    let popup_pos = log.rfind("wayland-server: xdg_popup.configure obj=");
    let surface_pos = log.rfind("wayland-server: xdg_surface.configure serial=");
    match (popup_pos, surface_pos) {
        (Some(popup_pos), Some(surface_pos)) if popup_pos < surface_pos => {
            eprintln!("│  │  │      ✅ xdg_popup.configure precedes xdg_surface.configure");
            Ok(())
        }
        _ => {
            Err(StepError("Expected xdg_popup.configure before xdg_surface.configure".to_string()))
        }
    }
}

/// `Given the client has a fully configured xdg_toplevel`
#[given("the client has a fully configured xdg_toplevel")]
async fn client_has_fully_configured_toplevel(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // A fully configured toplevel has: xdg_surface created + get_toplevel + ack_configure.
    let ack =
        world.wait_for_serial("wayland-server: xdg_surface.ack_configure serial=", 15.0).await;
    if ack {
        eprintln!("│  │  │      ✅ Fully configured xdg_toplevel confirmed");
        Ok(())
    } else {
        Err(StepError("No ack_configure observed — xdg_toplevel not fully configured".to_string()))
    }
}

/// `And the client has called wl_surface.frame to register a callback`
#[given("the client has called wl_surface.frame to register a callback")]
async fn client_registered_frame_callback(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // Wait briefly for the frame callback log to appear.
    let _ = world.wait_for_serial("wayland-server: wl_surface obj=", 10.0).await;
    let log = world.get_serial_log().await;
    if log.contains("registered frame callback") {
        eprintln!("│  │  │      ✅ Frame callback registered");
        Ok(())
    } else {
        // wayland_hello registers frame callbacks but the log may not appear until commit.
        eprintln!("│  │  │      ⚠️  Frame callback registration log not yet seen");
        Ok(()) // Allow this to pass as a precondition — the Then step asserts the callback fires.
    }
}

/// `When the client commits the surface with a valid shm buffer`
#[when("the client commits the surface with a valid shm buffer")]
async fn client_commits_with_shm_buffer(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // wayland_hello attaches and commits shm buffers regularly.
    let found = world.wait_for_serial("wayland-server: surface ", 10.0).await;
    if found {
        eprintln!("│  │  │      ✅ Surface committed with shm buffer");
        Ok(())
    } else {
        Err(StepError("No surface commit observed in compositor log".to_string()))
    }
}

/// `And the compositor presents the next frame`
#[when("the compositor presents the next frame")]
async fn compositor_presents_frame(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // Wait for a frame callback done event.
    let found = world.wait_for_serial("wayland-server: frame callback done", 30.0).await;
    if found {
        eprintln!("│  │  │      ✅ Compositor presented a frame");
        Ok(())
    } else {
        Err(StepError(
            "No frame presentation observed (wayland-server: frame callback done)".to_string(),
        ))
    }
}

/// `Then the client receives wl_callback.done for that callback object`
#[then("the client receives wl_callback.done for that callback object")]
async fn client_receives_callback_done(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    if log.contains("wayland-server: frame callback done") {
        eprintln!("│  │  │      ✅ wl_callback.done received by client");
        Ok(())
    } else {
        Err(StepError(
            "Expected 'wayland-server: frame callback done' in compositor log".to_string(),
        ))
    }
}

// ── wp_presentation (Presentation Time) ─────────────────────────────────────
//
// These steps assert the `wp_presentation` global is advertised on the
// registry and that `wp_presentation_feedback.presented` is delivered to a
// client that requested feedback for a committed surface. The wayland_hello
// reference client binds `wp_presentation` and requests feedback alongside
// each `wl_surface.frame` callback, so the corresponding compositor and
// client logs can be observed on the serial console.

/// `When the client requests the wl_registry global list`
#[when("the client requests the wl_registry global list")]
async fn client_requests_registry(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // wayland_hello issues `wl_display.get_registry` immediately on connect,
    // so by the time it has connected the global list has been requested.
    let _ = world.wait_for_serial("wayland_hello: bound wp_presentation", 15.0).await;
    Ok(())
}

/// `Then wl_registry advertises wp_presentation version 1`
#[then("wl_registry advertises wp_presentation version 1")]
async fn registry_advertises_wp_presentation(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    if log.contains("wayland_hello: bound wp_presentation") {
        eprintln!("│  │  │      ✅ wp_presentation advertised by wl_registry");
        Ok(())
    } else {
        Err(StepError(
            "Expected client to bind wp_presentation (no 'wayland_hello: bound wp_presentation' log)"
                .to_string(),
        ))
    }
}

/// `And the client has requested wp_presentation.feedback for the surface`
#[given("the client has requested wp_presentation.feedback for the surface")]
async fn client_requested_presentation_feedback(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let _ =
        world.wait_for_serial("wayland-server: wp_presentation.feedback surface_obj=", 15.0).await;
    let log = world.get_serial_log().await;
    if log.contains("wayland-server: wp_presentation.feedback") {
        eprintln!("│  │  │      ✅ Presentation feedback registered");
        Ok(())
    } else {
        // Allow as precondition — the Then step asserts the presented event.
        eprintln!("│  │  │      ⚠️  Presentation feedback registration log not yet seen");
        Ok(())
    }
}

/// `Then the client receives wp_presentation_feedback.presented for that feedback object`
#[then("the client receives wp_presentation_feedback.presented for that feedback object")]
async fn client_receives_feedback_presented(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let found =
        world.wait_for_serial("wayland_hello: wp_presentation_feedback.presented", 30.0).await;
    if found {
        eprintln!("│  │  │      ✅ wp_presentation_feedback.presented received");
        Ok(())
    } else {
        Err(StepError(
            "Expected 'wayland_hello: wp_presentation_feedback.presented' in client log"
                .to_string(),
        ))
    }
}
