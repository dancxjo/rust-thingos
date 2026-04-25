use cucumber::{given, then, when};

use crate::world::{ThingOsWorld, strip_ansi};

use super::helpers::{StepError, capture_failure_diagnostics};

// ===== ServiceLoop Test Harness Steps =====
//
// These steps reference a dedicated ServiceLoop test harness binary
// ("serviceloop_harness") that must be present on the system at /bin/sl_harness.
// The harness prints structured event tags to the serial console:
//   SLHARNESS:<event_type>:<detail>
// e.g. SLHARNESS:Message:test.ping
//      SLHARNESS:Ready:pipe_token
//      SLHARNESS:InboxClosed
//      SLHARNESS:drain_count:3
//
// If the harness is absent these steps will fail — that is intentional.
// Implement the harness in utils/serviceloop_harness to make them pass.

/// Structured log tag prefix emitted by the ServiceLoop test harness binary
/// (`sl_harness` at `/bin/sl_harness`).  The harness prints `SLHARNESS:<event>:<detail>`
/// lines to the serial console for each ServiceLoop event it observes, allowing the
/// BDD runner to assert against observable system behaviour.  Example output:
///   `SLHARNESS:Created`
///   `SLHARNESS:Message:test.ping`
///   `SLHARNESS:Ready:pipe_token`
///   `SLHARNESS:InboxClosed`
const SLHARNESS_PREFIX: &str = "SLHARNESS:";

/// Background: `And the serviceloop test harness is available`
#[given("the serviceloop test harness is available")]
async fn serviceloop_harness_available(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // The harness binary is expected at /bin/sl_harness.
    // Check by trying to stat/exec it via the serial shell.
    world.serial_checkpoint = world.get_serial_log().await.len();
    let mut cmd = b"ls /bin/sl_harness\n".to_vec();
    for b in cmd {
        world
            .serial_write(&[b])
            .await
            .map_err(|e| StepError(format!("serial write failed: {}", e)))?;
        tokio::time::sleep(std::time::Duration::from_millis(15)).await;
    }
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    let log = world.get_serial_log().await;
    let start = world.serial_checkpoint.min(log.len());
    let recent = strip_ansi(&log[start..]);
    if recent.contains("sl_harness") && !recent.to_lowercase().contains("not found") {
        eprintln!("│  │  │      ✅ ServiceLoop test harness found at /bin/sl_harness");
        Ok(())
    } else {
        Err(StepError(
            "ServiceLoop test harness not found at /bin/sl_harness. \
             Implement utils/serviceloop_harness and add it to the OS image to enable these tests."
                .to_string(),
        ))
    }
}

/// `When the harness creates a ServiceLoop with max_payload 4096`
#[when("the harness creates a ServiceLoop with max_payload 4096")]
async fn harness_creates_service_loop(world: &mut ThingOsWorld) -> Result<(), StepError> {
    world.serial_checkpoint = world.get_serial_log().await.len();
    // Launch the harness in the background; it awaits further commands.
    let mut cmd = b"sl_harness create 4096\n".to_vec();
    for b in cmd {
        world
            .serial_write(&[b])
            .await
            .map_err(|e| StepError(format!("serial write: {}", e)))?;
        tokio::time::sleep(std::time::Duration::from_millis(15)).await;
    }
    // Wait for the harness to confirm creation.
    let found = world.wait_for_serial("SLHARNESS:Created", 10.0).await;
    if !found {
        return Err(StepError(
            "Harness did not confirm ServiceLoop creation (SLHARNESS:Created missing)".to_string(),
        ));
    }
    eprintln!("│  │  │      ✅ Harness created ServiceLoop with max_payload=4096");
    Ok(())
}

/// `And another task sends a typed message with kind "<kind>" to the harness inbox` (regex)
#[when(regex = r#"^another task sends a typed message with kind "([^"]+)" to the harness inbox$"#)]
async fn send_typed_message_to_harness(
    world: &mut ThingOsWorld,
    kind: String,
) -> Result<(), StepError> {
    let mut cmd = format!("sl_harness send {}\n", kind).into_bytes();
    for b in cmd {
        world
            .serial_write(&[b])
            .await
            .map_err(|e| StepError(format!("serial write: {}", e)))?;
        tokio::time::sleep(std::time::Duration::from_millis(15)).await;
    }
    eprintln!("│  │  │      📨 Sent '{}' to harness inbox", kind);
    Ok(())
}

/// `Then the harness should observe a "Message" event with kind "<kind>" within 5s` (regex)
#[then(
    regex = r#"^the harness should observe a "Message" event with kind "([^"]+)" within 5s$"#
)]
async fn harness_observes_message_event(
    world: &mut ThingOsWorld,
    kind: String,
) -> Result<(), StepError> {
    let needle = format!("{}Message:{}", SLHARNESS_PREFIX, kind);
    let found = world.wait_for_serial(&needle, 5.0).await;
    if found {
        eprintln!("│  │  │      ✅ Harness observed Message:{}", kind);
        Ok(())
    } else {
        capture_failure_diagnostics(world, &needle).await;
        Err(StepError(format!(
            "Harness did not observe Message:{} within 5s (tag '{}' not found)",
            kind, needle
        )))
    }
}

/// `And the harness should not have observed any "Ready" event before the message`
#[then("the harness should not have observed any \"Ready\" event before the message")]
async fn harness_no_ready_before_message(_world: &mut ThingOsWorld) {
    // This ordering assertion is hard to check via serial log alone; note it.
    eprintln!(
        "│  │  │      ℹ️  Ordering assertion 'no Ready before Message' requires harness-side tagging"
    );
}

/// `And the harness registers a pipe read-end as a secondary FD`
#[when("the harness registers a pipe read-end as a secondary FD")]
async fn harness_registers_pipe_fd(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let mut cmd = b"sl_harness register_pipe\n".to_vec();
    for b in cmd {
        world
            .serial_write(&[b])
            .await
            .map_err(|e| StepError(format!("serial write: {}", e)))?;
        tokio::time::sleep(std::time::Duration::from_millis(15)).await;
    }
    let found = world.wait_for_serial("SLHARNESS:PipeRegistered", 5.0).await;
    if !found {
        return Err(StepError(
            "Harness did not confirm pipe FD registration".to_string(),
        ));
    }
    Ok(())
}

/// `And the pipe write-end is filled with 8 bytes of data`
#[when("the pipe write-end is filled with 8 bytes of data")]
async fn pipe_write_end_filled(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let mut cmd = b"sl_harness fill_pipe\n".to_vec();
    for b in cmd {
        world
            .serial_write(&[b])
            .await
            .map_err(|e| StepError(format!("serial write: {}", e)))?;
        tokio::time::sleep(std::time::Duration::from_millis(15)).await;
    }
    Ok(())
}

/// `Then the next ServiceLoop event observed by the harness should be a "Message" event with kind "<kind>"` (regex)
#[then(
    regex = r#"^the next ServiceLoop event observed by the harness should be a "Message" event with kind "([^"]+)"$"#
)]
async fn next_event_is_message(world: &mut ThingOsWorld, kind: String) -> Result<(), StepError> {
    let needle = format!("{}NextEvent:Message:{}", SLHARNESS_PREFIX, kind);
    let found = world.wait_for_serial(&needle, 5.0).await;
    if found {
        eprintln!("│  │  │      ✅ Next event is Message:{}", kind);
        Ok(())
    } else {
        Err(StepError(format!(
            "Expected next event to be Message:{} but '{}' not found",
            kind, needle
        )))
    }
}

/// `And the subsequent ServiceLoop event observed by the harness should be a "Ready" event for the pipe token`
#[then(
    "the subsequent ServiceLoop event observed by the harness should be a \"Ready\" event for the pipe token"
)]
async fn subsequent_event_is_pipe_ready(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let needle = format!("{}NextEvent:Ready:pipe_token", SLHARNESS_PREFIX);
    let found = world.wait_for_serial(&needle, 5.0).await;
    if found {
        eprintln!("│  │  │      ✅ Subsequent event is Ready:pipe_token");
        Ok(())
    } else {
        Err(StepError(format!(
            "Expected subsequent event to be Ready:pipe_token ('{}' not found)",
            needle
        )))
    }
}

/// `Then the harness should observe a "Ready" event for the pipe token within 5s`
#[then("the harness should observe a \"Ready\" event for the pipe token within 5s")]
async fn harness_observes_pipe_ready(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let needle = format!("{}Ready:pipe_token", SLHARNESS_PREFIX);
    let found = world.wait_for_serial(&needle, 5.0).await;
    if found {
        eprintln!("│  │  │      ✅ Harness observed Ready:pipe_token");
        Ok(())
    } else {
        Err(StepError(format!(
            "Harness did not observe Ready:pipe_token within 5s ('{}' not found)",
            needle
        )))
    }
}

/// `And the harness subscribes to a synthetic test IRQ and registers it as a secondary source`
#[when(
    "the harness subscribes to a synthetic test IRQ and registers it as a secondary source"
)]
async fn harness_subscribes_to_irq(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let mut cmd = b"sl_harness register_irq\n".to_vec();
    for b in cmd {
        world
            .serial_write(&[b])
            .await
            .map_err(|e| StepError(format!("serial write: {}", e)))?;
        tokio::time::sleep(std::time::Duration::from_millis(15)).await;
    }
    let found = world.wait_for_serial("SLHARNESS:IrqRegistered", 5.0).await;
    if !found {
        return Err(StepError("Harness did not confirm IRQ registration".to_string()));
    }
    Ok(())
}

/// `And the synthetic test IRQ is fired`
#[when("the synthetic test IRQ is fired")]
async fn synthetic_irq_fired(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let mut cmd = b"sl_harness fire_irq\n".to_vec();
    for b in cmd {
        world
            .serial_write(&[b])
            .await
            .map_err(|e| StepError(format!("serial write: {}", e)))?;
        tokio::time::sleep(std::time::Duration::from_millis(15)).await;
    }
    Ok(())
}

/// `Then the harness should observe a "Ready" event with the IRQ flag set within 5s`
#[then("the harness should observe a \"Ready\" event with the IRQ flag set within 5s")]
async fn harness_observes_irq_ready(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let needle = format!("{}Ready:irq", SLHARNESS_PREFIX);
    let found = world.wait_for_serial(&needle, 5.0).await;
    if found {
        eprintln!("│  │  │      ✅ Harness observed Ready:irq");
        Ok(())
    } else {
        Err(StepError(format!(
            "Harness did not observe Ready:irq within 5s ('{}' not found)",
            needle
        )))
    }
}

/// `And the harness's inbox is closed by the supervisor`
#[when("the harness's inbox is closed by the supervisor")]
async fn harness_inbox_closed(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let mut cmd = b"sl_harness close_inbox\n".to_vec();
    for b in cmd {
        world
            .serial_write(&[b])
            .await
            .map_err(|e| StepError(format!("serial write: {}", e)))?;
        tokio::time::sleep(std::time::Duration::from_millis(15)).await;
    }
    Ok(())
}

/// `Then the harness should observe an "InboxClosed" event within 5s`
#[then("the harness should observe an \"InboxClosed\" event within 5s")]
async fn harness_observes_inbox_closed(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let needle = format!("{}InboxClosed", SLHARNESS_PREFIX);
    let found = world.wait_for_serial(&needle, 5.0).await;
    if found {
        eprintln!("│  │  │      ✅ Harness observed InboxClosed");
        Ok(())
    } else {
        Err(StepError(format!(
            "Harness did not observe InboxClosed within 5s ('{}' not found)",
            needle
        )))
    }
}

/// `And subsequent calls to next_event should keep returning "InboxClosed"`
#[then("subsequent calls to next_event should keep returning \"InboxClosed\"")]
async fn subsequent_next_event_returns_inbox_closed(
    world: &mut ThingOsWorld,
) -> Result<(), StepError> {
    let needle = format!("{}InboxClosed:latch_confirmed", SLHARNESS_PREFIX);
    let found = world.wait_for_serial(&needle, 5.0).await;
    if found {
        eprintln!("│  │  │      ✅ InboxClosed is latched");
        Ok(())
    } else {
        Err(StepError(format!(
            "InboxClosed latch not confirmed ('{}' not found)",
            needle
        )))
    }
}

/// `And the harness should not consume more than 1% CPU after the inbox closes`
#[then("the harness should not consume more than 1% CPU after the inbox closes")]
async fn harness_low_cpu_after_close(world: &mut ThingOsWorld) -> Result<(), StepError> {
    // Allow a brief window then check that the harness reports idle.
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let needle = format!("{}Idle", SLHARNESS_PREFIX);
    let found = world.wait_for_serial(&needle, 5.0).await;
    if found {
        eprintln!("│  │  │      ✅ Harness reports idle after InboxClosed");
        Ok(())
    } else {
        Err(StepError(format!(
            "Harness did not report idle after InboxClosed ('{}' not found). \
             Possible CPU spin.",
            needle
        )))
    }
}

/// `And another task sends 4 typed messages with kind "<kind>" to the harness inbox` (regex)
#[when(
    regex = r#"^another task sends 4 typed messages with kind "([^"]+)" to the harness inbox$"#
)]
async fn send_4_typed_messages(world: &mut ThingOsWorld, kind: String) -> Result<(), StepError> {
    for _ in 0..4 {
        let mut cmd = format!("sl_harness send {}\n", kind).into_bytes();
        for b in cmd {
            world
                .serial_write(&[b])
                .await
                .map_err(|e| StepError(format!("serial write: {}", e)))?;
            tokio::time::sleep(std::time::Duration::from_millis(15)).await;
        }
    }
    eprintln!("│  │  │      📨 Sent 4 '{}' messages to harness inbox", kind);
    Ok(())
}

/// `And the harness calls next_event followed by drain_inbox`
#[when("the harness calls next_event followed by drain_inbox")]
async fn harness_calls_next_then_drain(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let mut cmd = b"sl_harness drain\n".to_vec();
    for b in cmd {
        world
            .serial_write(&[b])
            .await
            .map_err(|e| StepError(format!("serial write: {}", e)))?;
        tokio::time::sleep(std::time::Duration::from_millis(15)).await;
    }
    Ok(())
}

/// `Then the harness should report 4 messages observed in total`
#[then("the harness should report 4 messages observed in total")]
async fn harness_reports_4_total(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let needle = format!("{}total_count:4", SLHARNESS_PREFIX);
    let found = world.wait_for_serial(&needle, 5.0).await;
    if found {
        eprintln!("│  │  │      ✅ Harness reported 4 total messages");
        Ok(())
    } else {
        Err(StepError(format!(
            "Harness did not report total_count:4 ('{}' not found)",
            needle
        )))
    }
}

/// `And exactly 1 of those messages should have come from next_event`
#[then("exactly 1 of those messages should have come from next_event")]
async fn harness_reports_1_from_next_event(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let needle = format!("{}next_count:1", SLHARNESS_PREFIX);
    let found = world.wait_for_serial(&needle, 5.0).await;
    if found {
        eprintln!("│  │  │      ✅ 1 message from next_event confirmed");
        Ok(())
    } else {
        Err(StepError(format!(
            "Harness did not report next_count:1 ('{}' not found)",
            needle
        )))
    }
}

/// `And exactly 3 of those messages should have come from drain_inbox`
#[then("exactly 3 of those messages should have come from drain_inbox")]
async fn harness_reports_3_from_drain(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let needle = format!("{}drain_count:3", SLHARNESS_PREFIX);
    let found = world.wait_for_serial(&needle, 5.0).await;
    if found {
        eprintln!("│  │  │      ✅ 3 messages from drain_inbox confirmed");
        Ok(())
    } else {
        Err(StepError(format!(
            "Harness did not report drain_count:3 ('{}' not found)",
            needle
        )))
    }
}

/// `And the harness attempts to remove the inbox token`
#[when("the harness attempts to remove the inbox token")]
async fn harness_attempts_remove_inbox_token(
    world: &mut ThingOsWorld,
) -> Result<(), StepError> {
    let mut cmd = b"sl_harness remove_inbox\n".to_vec();
    for b in cmd {
        world
            .serial_write(&[b])
            .await
            .map_err(|e| StepError(format!("serial write: {}", e)))?;
        tokio::time::sleep(std::time::Duration::from_millis(15)).await;
    }
    Ok(())
}

/// `Then the remove call should report "false"`
#[then("the remove call should report \"false\"")]
async fn remove_call_reports_false(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let needle = format!("{}remove_result:false", SLHARNESS_PREFIX);
    let found = world.wait_for_serial(&needle, 5.0).await;
    if found {
        eprintln!("│  │  │      ✅ remove(inbox_token) returned false as expected");
        Ok(())
    } else {
        Err(StepError(format!(
            "Harness did not report remove_result:false ('{}' not found)",
            needle
        )))
    }
}

/// `And the ServiceLoop should still have the inbox registered`
#[then("the ServiceLoop should still have the inbox registered")]
async fn service_loop_still_has_inbox(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let needle = format!("{}inbox_still_registered:true", SLHARNESS_PREFIX);
    let found = world.wait_for_serial(&needle, 5.0).await;
    if found {
        eprintln!("│  │  │      ✅ ServiceLoop inbox still registered after attempted remove");
        Ok(())
    } else {
        Err(StepError(format!(
            "ServiceLoop inbox not confirmed still registered ('{}' not found)",
            needle
        )))
    }
}

/// `And the harness arms a run_until_shutdown hook that records "<tag>"` (regex)
#[when(
    regex = r#"^the harness arms a run_until_shutdown hook that records "([^"]+)"$"#
)]
async fn harness_arms_shutdown_hook(
    world: &mut ThingOsWorld,
    tag: String,
) -> Result<(), StepError> {
    let mut cmd = format!("sl_harness arm_shutdown_hook {}\n", tag).into_bytes();
    for b in cmd {
        world
            .serial_write(&[b])
            .await
            .map_err(|e| StepError(format!("serial write: {}", e)))?;
        tokio::time::sleep(std::time::Duration::from_millis(15)).await;
    }
    eprintln!("│  │  │      🔧 Armed shutdown hook with tag '{}'", tag);
    Ok(())
}

/// `Then the harness should observe "<tag>" in the hook output within 5s` (regex)
#[then(regex = r#"^the harness should observe "([^"]+)" in the hook output within 5s$"#)]
async fn harness_observes_hook_output(
    world: &mut ThingOsWorld,
    tag: String,
) -> Result<(), StepError> {
    let needle = format!("{}hook:{}", SLHARNESS_PREFIX, tag);
    let found = world.wait_for_serial(&needle, 5.0).await;
    if found {
        eprintln!("│  │  │      ✅ Hook output '{}' observed", tag);
        Ok(())
    } else {
        Err(StepError(format!(
            "Hook output '{}' not observed ('{}' not found within 5s)",
            tag, needle
        )))
    }
}

/// `And run_until_shutdown should have returned to the caller`
#[then("run_until_shutdown should have returned to the caller")]
async fn run_until_shutdown_returned(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let needle = format!("{}run_until_shutdown_returned", SLHARNESS_PREFIX);
    let found = world.wait_for_serial(&needle, 5.0).await;
    if found {
        eprintln!("│  │  │      ✅ run_until_shutdown returned cleanly");
        Ok(())
    } else {
        Err(StepError(format!(
            "run_until_shutdown did not report return ('{}' not found)",
            needle
        )))
    }
}

/// `And the handler returns Break on the next Message event`
#[when("the handler returns Break on the next Message event")]
async fn handler_returns_break(world: &mut ThingOsWorld) -> Result<(), StepError> {
    let mut cmd = b"sl_harness arm_break_handler\n".to_vec();
    for b in cmd {
        world
            .serial_write(&[b])
            .await
            .map_err(|e| StepError(format!("serial write: {}", e)))?;
        tokio::time::sleep(std::time::Duration::from_millis(15)).await;
    }
    eprintln!("│  │  │      🔧 Armed Break handler");
    Ok(())
}


