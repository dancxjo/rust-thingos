//! PS/2 input stress step definitions.
//!
//! Provides the "I stress PS/2 input with N high-frequency events" step that
//! floods the PS/2 keyboard and mouse interrupt paths with synthetic input so
//! diagnostics logs can be validated end-to-end.

use cucumber::when;

use super::helpers::StepError;
use crate::input::{qmp_key, qmp_mouse_rel, send_qmp_sequence};
use crate::world::ThingOsWorld;

/// Keys cycled during the stress test (non-modifier, safe for repeated taps).
const STRESS_KEYS: &[&str] = &["a", "s", "d", "f", "j", "k", "l"];

/// Stress PS/2 input by injecting `count` synthetic key-press and mouse-move
/// events in rapid succession via QEMU QMP.
///
/// Events are split evenly between keyboard key taps and relative mouse moves
/// so both IRQ1 (keyboard) and IRQ12 (mouse) paths are exercised.  Each event
/// pair (key-down + key-up, or mouse rel move) counts as one event unit toward
/// `count`.
#[when(regex = r#"^I stress PS/2 input with (\d+) high-frequency events$"#)]
async fn stress_ps2_input(
    world: &mut ThingOsWorld,
    count: u64,
) -> Result<(), StepError> {
    let settle = std::time::Duration::from_millis(15);
    let mut commands = Vec::with_capacity(count as usize * 3);

    for i in 0..count {
        if i % 2 == 0 {
            // Keyboard: tap a key (down + up = one logical event)
            let key = STRESS_KEYS[(i as usize) % STRESS_KEYS.len()];
            commands.push((qmp_key(key, true), settle));
            commands.push((qmp_key(key, false), settle));
        } else {
            // Mouse: small relative move (one event)
            let dx = if i % 4 == 1 { 2i32 } else { -2i32 };
            let dy = if i % 8 < 4 { 1i32 } else { -1i32 };
            commands.push((qmp_mouse_rel(dx, dy), settle));
        }
    }

    send_qmp_sequence(world, &commands, "ps2 stress input")
        .await
        .map_err(|e| StepError(format!("PS/2 stress input failed: {}", e)))
}
