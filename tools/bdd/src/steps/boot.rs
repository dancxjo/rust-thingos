use cucumber::{then, when};

use crate::world::{LIVENESS_SIGNALS, REQUIRED_BOOT_SIGNALS, ThingOsWorld, diag_enabled};

use super::helpers::{StepError, capture_failure_diagnostics};

// ===== Consolidated Boot Feature Steps =====


/// Default timeout for boot ready state (seconds).
const BOOT_READY_TIMEOUT_SECS: f64 = 120.0;

#[when("I wait for the system to reach ready state")]
pub(super) async fn wait_for_ready_state(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // Use longer timeout in diagnostics mode
    let timeout =
        if diag_enabled() { BOOT_READY_TIMEOUT_SECS + 15.0 } else { BOOT_READY_TIMEOUT_SECS };

    // Wait for scheduler loop entry as the primary "ready" signal
    let found = world.wait_for_serial("Entering scheduler loop", timeout).await;

    if !found {
        capture_failure_diagnostics(world, "system ready state").await;
        let log = world.get_serial_log().await;
        eprintln!("\n=== Serial Log (last 200 lines) ===");
        for line in log.lines().rev().take(200).collect::<Vec<_>>().into_iter().rev() {
            eprintln!("{}", line);
        }
        eprintln!("=== End Serial Log ===\n");
        return Err(StepError("System did not reach ready state within timeout".to_string()));
    }
    Ok(())
}

#[then("the boot log should contain all required signals")]
async fn check_required_signals(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let result = world.wait_for_all_signals(REQUIRED_BOOT_SIGNALS, 5.0).await;

    if let Err(missing) = result {
        capture_failure_diagnostics(world, "required boot signals").await;
        let log = world.get_serial_log().await;

        eprintln!("\n=== Missing Boot Signals ===");
        for sig in &missing {
            eprintln!("  ❌ {}", sig);
        }
        eprintln!("\n=== Serial Log (last 200 lines) ===");
        for line in log.lines().rev().take(200).collect::<Vec<_>>().into_iter().rev() {
            eprintln!("{}", line);
        }
        eprintln!("=== End Serial Log ===\n");

        return Err(StepError(format!("Boot log missing required signals: {:?}", missing)));
    }

    // In diagnostics mode, print what we found
    if diag_enabled() {
        eprintln!("\n=== Boot Signals Verified ===");
        for alts in REQUIRED_BOOT_SIGNALS {
            eprintln!("  ✅ {}", alts.join(" OR "));
        }
    }
    Ok(())
}

#[then("the system should show liveness")]
async fn check_liveness(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // Give the scheduler a moment to show thread execution
    let found = world.wait_for_liveness(10.0).await;

    if !found {
        capture_failure_diagnostics(world, "liveness signal").await;
        let log = world.get_serial_log().await;

        eprintln!("\n=== Liveness Check Failed ===");
        eprintln!("Expected at least one of: {:?}", LIVENESS_SIGNALS);
        eprintln!("\n=== Serial Log (last 100 lines) ===");
        for line in log.lines().rev().take(100).collect::<Vec<_>>().into_iter().rev() {
            eprintln!("{}", line);
        }
        eprintln!("=== End Serial Log ===\n");

        return Err(StepError(
            "System did not show liveness (no Thread ticks or heartbeat)".to_string(),
        ));
    }

    if diag_enabled() {
        eprintln!("  ✅ Liveness detected");
    }
    Ok(())
}

// ===== Torture Garden Steps =====

#[then(regex = r#"^the log does not contain "(.+)"$"#)]
async fn log_does_not_contain(world: &mut ThingOsWorld, pattern: String) -> Result<(), StepError> {
    // Give a brief window for any late output
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let log = world.get_serial_log().await;
    if log.contains(&pattern) {
        eprintln!("\n=== Unexpected pattern found in log ===");
        eprintln!("Pattern: {}", pattern);
        eprintln!("\n=== Serial Log (context) ===");
        for line in log.lines().filter(|l| l.contains(&pattern)) {
            eprintln!(">>> {}", line);
        }
        eprintln!("=== End Context ===\n");
        return Err(StepError(format!("Log unexpectedly contains '{}'", pattern)));
    }
    Ok(())
}

#[then(regex = r#"^the log should not contain "(.+)"$"#)]
async fn log_should_not_contain(
    world: &mut ThingOsWorld,
    pattern: String,
) -> Result<(), StepError> {
    log_does_not_contain(world, pattern).await
}


