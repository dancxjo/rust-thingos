//! Signal-to-inbox bridge: co-deliver Unix signals as typed inbox messages.
//!
//! # Design
//!
//! This module is the canonical **signal → Message pivot**.  It converts a
//! Unix signal delivery request into a typed [`Message`] with
//! [`KindId::THINGOS_SIGNAL`] and enqueues it into the target process inbox.
//! The inbox delivery is the **canonical asynchronous notification path** for
//! the new model; the legacy [`ProcessSignals::post`] call that follows
//! preserves POSIX compatibility for existing applications during the
//! migration period.
//!
//! # Payload encoding
//!
//! The `thingos.signal` message payload is a compact flat encoding:
//!
//! ```text
//! offset  size  field
//! ──────  ────  ─────────────────────────────────────────────────────────
//!    0      1   signum        — POSIX signal number (1–63)
//!    1      8   sender_tid    — LE u64, sender TID (0 = kernel/anonymous)
//!    9      1   has_fault     — 1 if fault context follows, 0 otherwise
//!   10      8   fault_addr    — LE u64, faulting address (fault only)
//!   18      8   rip           — LE u64, instruction pointer (fault only)
//!   26      8   rsp           — LE u64, stack pointer    (fault only)
//! ```
//!
//! When `has_fault == 0`, the payload is 10 bytes.
//! When `has_fault == 1`, the payload is 34 bytes.
//!
//! # Why SIGKILL and SIGSTOP bypass inbox delivery
//!
//! SIGKILL and SIGSTOP are uncatchable — by POSIX definition no application
//! may intercept or ignore them.  Delivering them to the inbox would allow
//! a process to observe (and potentially loop on) a request it must not
//! override.  These signals continue to take effect through the existing
//! kernel fast-path.
//!
//! # Compatibility invariant
//!
//! This bridge **does not replace** the legacy `ProcessSignals::post` call
//! during the migration period.  Both deliveries happen:
//!
//! 1. `post_signal_to_inbox` — enqueues typed Message to the process inbox
//!    (canonical new model).
//! 2. `ProcessSignals::post` — records signal in the pending bitmask
//!    (POSIX compatibility projection).
//!
//! Once applications are fully migrated to inbox-based event loops, step 2
//! can be removed and `ProcessUnixCompat.signals` can be deprecated.

use thingos::message::{KindId, Message};

// ── Public types ──────────────────────────────────────────────────────────────

/// Optional fault context for crash-class signal messages.
///
/// Populated for signals that originate from hardware faults (SIGSEGV, SIGBUS,
/// SIGILL, SIGFPE) so that applications can log rich crash diagnostics in
/// their main event loop without async-signal-safe constraints.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SignalFaultContext {
    /// Address that triggered the fault (e.g. invalid memory access address).
    pub fault_addr: u64,
    /// Instruction pointer at the time of the fault.
    pub rip: u64,
    /// Stack pointer at the time of the fault.
    pub rsp: u64,
}

// ── Payload construction ──────────────────────────────────────────────────────

/// Build the flat `thingos.signal` payload bytes from the given fields.
///
/// See module-level documentation for the byte layout.
pub fn build_signal_payload(
    signum: u8,
    sender_tid: u64,
    fault: Option<SignalFaultContext>,
) -> alloc::vec::Vec<u8> {
    let mut payload = alloc::vec::Vec::with_capacity(if fault.is_some() { 34 } else { 10 });
    payload.push(signum);
    payload.extend_from_slice(&sender_tid.to_le_bytes());
    match fault {
        Some(f) => {
            payload.push(1u8);
            payload.extend_from_slice(&f.fault_addr.to_le_bytes());
            payload.extend_from_slice(&f.rip.to_le_bytes());
            payload.extend_from_slice(&f.rsp.to_le_bytes());
        }
        None => {
            payload.push(0u8);
        }
    }
    payload
}

/// Parse a flat `thingos.signal` payload back into typed fields.
///
/// Returns `None` if the payload is malformed (too short or invalid has_fault byte).
///
/// # Format
///
/// See module-level documentation for the byte layout.
pub fn parse_signal_payload(
    payload: &[u8],
) -> Option<(u8, u64, Option<SignalFaultContext>)> {
    if payload.len() < 10 {
        return None;
    }
    let signum = payload[0];
    let sender_tid = u64::from_le_bytes(payload[1..9].try_into().ok()?);
    let has_fault = payload[9];
    let fault = match has_fault {
        0 => None,
        1 => {
            if payload.len() < 34 {
                return None;
            }
            Some(SignalFaultContext {
                fault_addr: u64::from_le_bytes(payload[10..18].try_into().ok()?),
                rip: u64::from_le_bytes(payload[18..26].try_into().ok()?),
                rsp: u64::from_le_bytes(payload[26..34].try_into().ok()?),
            })
        }
        _ => return None,
    };
    Some((signum, sender_tid, fault))
}

/// Construct the canonical `thingos.signal` [`Message`] envelope.
pub fn build_signal_message(
    signum: u8,
    sender_tid: u64,
    fault: Option<SignalFaultContext>,
) -> Message {
    let payload = build_signal_payload(signum, sender_tid, fault);
    Message::new(KindId::THINGOS_SIGNAL, payload)
}

// ── Inbox delivery ────────────────────────────────────────────────────────────

/// Enqueue a typed `thingos.signal` message into the inbox of process `pid`.
///
/// # Skipped signals
///
/// SIGKILL and SIGSTOP are not delivered via inbox because they are
/// uncatchable (see module-level documentation for rationale).  This
/// function returns `Ok(())` immediately for those signals without posting.
///
/// # Errors
///
/// Returns an error string only for diagnostics; caller is not expected to
/// propagate inbox delivery failures as hard errors.  The legacy signal
/// pending-bit path remains authoritative for POSIX semantics.
pub fn deliver_signal_to_inbox(
    pid: u32,
    signum: u8,
    sender_tid: u64,
    fault: Option<SignalFaultContext>,
) -> Result<(), &'static str> {
    // Uncatchable signals bypass inbox delivery entirely.
    if signum == abi::signal::SIGKILL || signum == abi::signal::SIGSTOP {
        return Ok(());
    }

    let message = build_signal_message(signum, sender_tid, fault);

    let Some(pinfo) = crate::sched::process_info_for_pid_current(pid) else {
        return Err("process not found");
    };

    let inbox_id = pinfo.lock().unix_compat.message_inbox;

    let Some(inbox) = crate::inbox::get_inbox(inbox_id) else {
        return Err("inbox not found");
    };

    let envelope =
        crate::inbox::MessageEnvelope::with_sender(message, sender_tid);

    match inbox.send(envelope) {
        Ok(()) => {
            crate::kdebug!(
                "signal::inbox_bridge: delivered sig={} pid={} sender_tid={}",
                signum,
                pid,
                sender_tid
            );
            Ok(())
        }
        Err(crate::inbox::SendError::Full { .. }) => {
            crate::kdebug!(
                "signal::inbox_bridge: inbox full sig={} pid={}",
                signum,
                pid
            );
            Err("inbox full")
        }
        Err(crate::inbox::SendError::Closed) => {
            crate::kdebug!(
                "signal::inbox_bridge: inbox closed sig={} pid={}",
                signum,
                pid
            );
            Err("inbox closed")
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Payload round-trip tests ──────────────────────────────────────────────

    #[test]
    fn payload_round_trip_no_fault() {
        let signum = 15u8; // SIGTERM
        let sender_tid = 1234u64;
        let payload = build_signal_payload(signum, sender_tid, None);
        assert_eq!(payload.len(), 10);

        let (sig, tid, fault) = parse_signal_payload(&payload).unwrap();
        assert_eq!(sig, signum);
        assert_eq!(tid, sender_tid);
        assert!(fault.is_none());
    }

    #[test]
    fn payload_round_trip_with_fault() {
        let signum = 11u8; // SIGSEGV
        let sender_tid = 0u64; // kernel
        let fault_ctx = SignalFaultContext { fault_addr: 0xdeadbeef, rip: 0x400100, rsp: 0x7fff0000 };
        let payload = build_signal_payload(signum, sender_tid, Some(fault_ctx));
        assert_eq!(payload.len(), 34);

        let (sig, tid, fault) = parse_signal_payload(&payload).unwrap();
        assert_eq!(sig, signum);
        assert_eq!(tid, sender_tid);
        let fault = fault.unwrap();
        assert_eq!(fault.fault_addr, 0xdeadbeef);
        assert_eq!(fault.rip, 0x400100);
        assert_eq!(fault.rsp, 0x7fff0000);
    }

    #[test]
    fn payload_parse_rejects_too_short() {
        let short = alloc::vec![11u8, 0, 0]; // only 3 bytes, need at least 10
        assert!(parse_signal_payload(&short).is_none());
    }

    #[test]
    fn payload_parse_rejects_truncated_fault() {
        // has_fault=1 but only 20 bytes total (need 34)
        let mut payload = build_signal_payload(11, 0, None);
        payload[9] = 1u8; // set has_fault=1 manually
        assert!(parse_signal_payload(&payload).is_none());
    }

    #[test]
    fn payload_parse_rejects_invalid_has_fault_byte() {
        let signum = 2u8;
        let mut payload = build_signal_payload(signum, 99, None);
        payload[9] = 42u8; // invalid has_fault byte
        assert!(parse_signal_payload(&payload).is_none());
    }

    // ── Message construction tests ────────────────────────────────────────────

    #[test]
    fn signal_message_has_correct_kind_id() {
        let msg = build_signal_message(15, 0, None);
        assert_eq!(msg.kind, KindId::THINGOS_SIGNAL);
    }

    #[test]
    fn signal_message_payload_is_non_empty() {
        let msg = build_signal_message(9, 0, None);
        assert!(!msg.is_empty());
    }

    #[test]
    fn signal_message_with_fault_has_longer_payload() {
        let no_fault = build_signal_message(11, 0, None);
        let with_fault = build_signal_message(
            11,
            0,
            Some(SignalFaultContext { fault_addr: 1, rip: 2, rsp: 3 }),
        );
        assert!(with_fault.payload.len() > no_fault.payload.len());
        assert_eq!(no_fault.payload.len(), 10);
        assert_eq!(with_fault.payload.len(), 34);
    }

    // ── SIGKILL/SIGSTOP bypass tests ──────────────────────────────────────────

    /// SIGKILL and SIGSTOP must never be delivered to inbox — they are
    /// uncatchable and bypassing them would allow malicious/buggy receivers
    /// to observe (and loop on) a forced-exit request.
    #[test]
    fn sigkill_is_not_catchable_by_design_payload_can_be_built() {
        // The bridge does NOT prevent building a payload for SIGKILL; it
        // prevents *delivering* it to inbox.  Payload construction is a
        // separate low-level helper and may be called for diagnostics.
        let msg = build_signal_message(abi::signal::SIGKILL, 0, None);
        let (sig, _, _) = parse_signal_payload(&msg.payload).unwrap();
        assert_eq!(sig, abi::signal::SIGKILL);
    }

    #[test]
    fn sigstop_is_not_catchable_by_design_payload_can_be_built() {
        let msg = build_signal_message(abi::signal::SIGSTOP, 0, None);
        let (sig, _, _) = parse_signal_payload(&msg.payload).unwrap();
        assert_eq!(sig, abi::signal::SIGSTOP);
    }

    // ── sender_tid=0 round-trips ──────────────────────────────────────────────

    #[test]
    fn kernel_originated_signal_has_zero_sender_tid() {
        let payload = build_signal_payload(9, 0, None);
        let (_, tid, _) = parse_signal_payload(&payload).unwrap();
        assert_eq!(tid, 0);
    }

    // ── all standard signal numbers round-trip ────────────────────────────────

    #[test]
    fn all_signal_numbers_1_to_31_round_trip() {
        for signum in 1u8..=31 {
            let payload = build_signal_payload(signum, signum as u64 * 100, None);
            let (sig, tid, fault) = parse_signal_payload(&payload).unwrap();
            assert_eq!(sig, signum);
            assert_eq!(tid, signum as u64 * 100);
            assert!(fault.is_none());
        }
    }
}
