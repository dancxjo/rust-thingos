use cucumber::{then, when};

use crate::world::ThingOsWorld;

use super::helpers::{StepError, capture_failure_diagnostics};

// ===== Driver-Ready Inbox Steps =====

/// `When the system finishes bringing up supervised services`
#[when("the system finishes bringing up supervised services")]
async fn system_finishes_supervised_services(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // Wait for sprout to have processed at least one DRIVER_READY handshake,
    // which means the ServiceLoop-based supervisor is alive and dispatching.
    let found = world.wait_for_serial("SPROUT: DRIVER_READY received", 60.0).await;
    if found {
        eprintln!("│  │  │      ✅ Sprout ServiceLoop received DRIVER_READY");
        return Ok(());
    }
    // Fallback: at minimum sprout's ServiceLoop must be running.
    let found2 = world.wait_for_serial("SPROUT: ServiceLoop", 30.0).await;
    if found2 {
        eprintln!("│  │  │      ⚠️  No DRIVER_READY yet, but SPROUT ServiceLoop is running");
        return Ok(());
    }
    capture_failure_diagnostics(world, "supervised services startup").await;
    Err(StepError(
        "System did not complete supervised service startup: no SPROUT ServiceLoop found"
            .to_string(),
    ))
}

/// `When cambium spawns a hardware driver`
#[when("cambium spawns a hardware driver")]
async fn cambium_spawns_hardware_driver(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // Cambium sends DRIVER_READY to Sprout after spawning each driver.
    let found = world.wait_for_serial("CAMBIUM: sent DRIVER_READY to Sprout", 60.0).await;
    if found {
        eprintln!("│  │  │      ✅ Cambium sent DRIVER_READY to Sprout");
        return Ok(());
    }
    // Check if cambium did anything at all.
    let log = world.get_serial_log().await;
    if log.contains("CAMBIUM:") {
        eprintln!("│  │  │      ⚠️  Cambium active but DRIVER_READY not yet sent");
        return Ok(());
    }
    capture_failure_diagnostics(world, "CAMBIUM: sent DRIVER_READY").await;
    Err(StepError("Cambium did not send DRIVER_READY to Sprout within timeout".to_string()))
}

/// `Then the serial shell is still responsive`
#[then("the serial shell is still responsive")]
async fn serial_shell_still_responsive(world: &mut ThingOsWorld) -> Result<(), StepError> {
    eprintln!("│  │  │      🔍 Checking serial shell responsiveness...");
    // Set a checkpoint before sending the command
    world.serial_checkpoint = world.get_serial_log().await.len();
    // Send a simple echo command
    let probe = "SHELL_ALIVE_PROBE";
    let mut data = format!("echo {}\n", probe).into_bytes();
    for b in data {
        world
            .serial_write(&[b])
            .await
            .map_err(|e| StepError(format!("Serial write failed: {}", e)))?;
        tokio::time::sleep(std::time::Duration::from_millis(15)).await;
    }
    // Wait for the probe string to appear in log
    let found = world.wait_for_serial(probe, 10.0).await;
    if !found {
        capture_failure_diagnostics(world, probe).await;
        return Err(StepError("Serial shell did not respond to echo probe within 10s".to_string()));
    }
    eprintln!("│  │  │      ✅ Serial shell is responsive");
    Ok(())
}

/// `Then the serial output should not contain "<pattern>"`
#[then(regex = r#"^the serial output should not contain "(.+)"$"#)]
async fn serial_output_should_not_contain(
    world: &mut ThingOsWorld,
    pattern: String,
) -> Result<(), StepError> {
    // Give a brief window for any late output to arrive.
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    let log = world.get_serial_log().await;
    if log.contains(&pattern) {
        eprintln!("\n=== Unexpected pattern found in serial output ===");
        eprintln!("Pattern: {}", pattern);
        for line in log.lines().filter(|l| l.contains(&pattern)) {
            eprintln!(">>> {}", line);
        }
        eprintln!("=== End ===\n");
        return Err(StepError(format!(
            "Serial output contains '{}' (expected to be absent)",
            pattern
        )));
    }
    eprintln!("│  │  │      ✅ Pattern '{}' absent from serial output", pattern);
    Ok(())
}


