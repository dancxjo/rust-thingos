//! Desktop App Steps — `check_window_bg_color` and `check_text_pixels` are in `super::helpers`.

use cucumber::{then, when};

use super::helpers::{StepError, capture_failure_diagnostics};
use crate::world::{DESKTOP_READY_SIGNALS, ThingOsWorld};

async fn wait_for_shared_desktop_readiness(
    world: &mut ThingOsWorld,
    timeout_secs: f64,
) -> Result<(), StepError> {
    match world.wait_for_desktop_ready(timeout_secs).await {
        Ok(()) => Ok(()),
        Err(missing) => {
            capture_failure_diagnostics(world, "desktop readiness").await;
            eprintln!("\n=== Desktop Readiness Signals ===");
            for group in DESKTOP_READY_SIGNALS {
                eprintln!("  {}", group.join(" AND "));
            }
            eprintln!("\n=== Missing Desktop Signals ===");
            for signal in &missing {
                eprintln!("  {}", signal);
            }
            eprintln!("=== End Desktop Readiness ===\n");

            Err(StepError(format!(
                "Desktop did not become ready for stress input within {:.1}s; missing: {:?}",
                timeout_secs, missing
            )))
        }
    }
}

#[then(regex = r#"^the desktop should be ready for stress input within ([0-9.]+)s$"#)]
async fn desktop_should_be_ready_for_stress_input(
    world: &mut ThingOsWorld,
    timeout: String,
) -> Result<(), StepError> {
    let timeout_secs = timeout.parse::<f64>().unwrap_or(180.0);
    wait_for_shared_desktop_readiness(world, timeout_secs).await
}

#[when(regex = r#"^I wait for the desktop to be ready for stress input within ([0-9.]+)s$"#)]
async fn wait_for_desktop_ready_for_stress_input(
    world: &mut ThingOsWorld,
    timeout: String,
) -> Result<(), StepError> {
    let timeout_secs = timeout.parse::<f64>().unwrap_or(180.0);
    wait_for_shared_desktop_readiness(world, timeout_secs).await
}
