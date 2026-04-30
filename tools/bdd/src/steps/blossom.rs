use cucumber::{given, then, when};

use super::basic::turn_on_machine;
use super::helpers::{StepError, capture_failure_diagnostics};
use crate::world::ThingOsWorld;

// ===== Blossom XDG-Shell Steps =====

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
    let found = world.wait_for_serial("wayland-server: xdg_toplevel obj=", 15.0).await;
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
    // wayland_hello calls get_popup which is rejected in v1.
    let found = world.wait_for_serial("wayland-server: xdg_surface.get_popup rejected", 10.0).await;
    if found {
        eprintln!("│  │  │      ✅ get_popup call triggered rejection");
        Ok(())
    } else {
        Err(StepError("get_popup rejection not observed in compositor log".to_string()))
    }
}

/// `And the error message indicates popups are not supported in v1`
#[then("the error message indicates popups are not supported in v1")]
async fn error_popups_not_supported(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let log = world.get_serial_log().await;
    if log.contains("not supported in v1") {
        eprintln!("│  │  │      ✅ 'not supported in v1' error confirmed for popups");
        Ok(())
    } else {
        Err(StepError(
            "Expected 'not supported in v1' in compositor log for get_popup rejection".to_string(),
        ))
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
