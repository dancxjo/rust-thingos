//! Compile-time and runtime checks for kernel boundary contract types.
//!
//! These tests ensure kernel-facing bridge functions keep returning canonical
//! ThingOS boundary types and that KindId constants remain aligned with
//! kindc-generated schema output.
//!
//! # Guardrail coverage
//!
//! | Surface                              | Test                                    |
//! |--------------------------------------|-----------------------------------------|
//! | task bridge signatures               | [`bridge_signatures_return_canonical_types`] |
//! | job bridge signatures                | [`bridge_signatures_return_canonical_types`] |
//! | group bridge signatures              | [`bridge_signatures_return_canonical_types`] |
//! | authority bridge signatures          | [`bridge_signatures_return_canonical_types`] |
//! | place bridge signatures              | [`bridge_signatures_return_canonical_types`] |
//! | message bridge signatures            | [`bridge_signatures_return_canonical_types`] |
//! | KIND_ID_THINGOS_MESSAGE              | [`kind_ids_match_kindc_generated_constants`] |
//! | KIND_ID_THINGOS_JOB_EXIT             | [`kind_ids_match_kindc_generated_constants`] |
//! | KIND_ID_THINGOS_AUTHORITY            | [`kind_ids_match_kindc_generated_constants`] |
//! | KIND_ID_THINGOS_TASK / TASK_STATE    | [`kind_ids_match_kindc_generated_constants`] |
//! | KIND_ID_THINGOS_JOB / JOB_STATE / JOB_WAIT_RESULT | [`kind_ids_match_kindc_generated_constants`] |
//! | KIND_ID_THINGOS_GROUP                | [`kind_ids_match_kindc_generated_constants`] |
//! | KIND_ID_THINGOS_PLACE                | [`kind_ids_match_kindc_generated_constants`] |

use thingos::message::KindId;

#[test]
fn bridge_signatures_return_canonical_types() {
    let _: fn(crate::task::ThreadState) -> thingos::task::Task =
        crate::task::bridge::task_from_thread_state;

    let _: fn(&crate::sched::hooks::ProcessSnapshot) -> thingos::task::Task =
        crate::task::bridge::task_from_snapshot;

    let _: fn(&[crate::sched::state::ThreadState]) -> thingos::job::Job =
        crate::job::bridge::job_from_thread_states;

    let _: fn(&crate::sched::hooks::ProcessSnapshot) -> thingos::job::JobExit =
        crate::job::bridge::job_exit_from_snapshot;

    let _: fn(Option<i32>) -> thingos::job::JobWaitResult =
        crate::job::bridge::job_wait_result_from_poll;

    let _: fn(&crate::sched::hooks::ProcessSnapshot) -> thingos::group::Group =
        crate::group::bridge::group_from_snapshot;

    let _: fn(&crate::sched::hooks::ProcessSnapshot) -> thingos::authority::Authority =
        crate::authority::bridge::authority_from_snapshot;

    let _: fn(&crate::sched::hooks::ProcessSnapshot) -> thingos::place::Place =
        crate::place::bridge::place_from_snapshot;

    let _: fn(KindId, alloc::vec::Vec<u8>) -> thingos::message::Message =
        crate::message::bridge::message_from_parts;
}

#[test]
fn kind_ids_match_kindc_generated_constants() {
    // ── message ───────────────────────────────────────────────────────────────
    assert_eq!(KindId::THINGOS_MESSAGE.0, thingos::kinds::KIND_ID_THINGOS_MESSAGE);
    assert_eq!(KindId::THINGOS_JOB_EXIT.0, thingos::kinds::KIND_ID_THINGOS_JOB_EXIT);

    // ── authority ─────────────────────────────────────────────────────────────
    assert_eq!(
        thingos::authority::KIND_ID_THINGOS_AUTHORITY,
        thingos::kinds::KIND_ID_THINGOS_AUTHORITY,
    );

    // ── task ──────────────────────────────────────────────────────────────────
    assert_eq!(
        thingos::task::KIND_ID_THINGOS_TASK,
        thingos::kinds::KIND_ID_THINGOS_TASK,
    );
    assert_eq!(
        thingos::task::KIND_ID_THINGOS_TASK_STATE,
        thingos::kinds::KIND_ID_THINGOS_TASK_STATE,
    );

    // ── job ───────────────────────────────────────────────────────────────────
    assert_eq!(
        thingos::job::KIND_ID_THINGOS_JOB,
        thingos::kinds::KIND_ID_THINGOS_JOB,
    );
    assert_eq!(
        thingos::job::KIND_ID_THINGOS_JOB_STATE,
        thingos::kinds::KIND_ID_THINGOS_JOB_STATE,
    );
    assert_eq!(
        thingos::job::KIND_ID_THINGOS_JOB_EXIT,
        thingos::kinds::KIND_ID_THINGOS_JOB_EXIT,
    );
    assert_eq!(
        thingos::job::KIND_ID_THINGOS_JOB_WAIT_RESULT,
        thingos::kinds::KIND_ID_THINGOS_JOB_WAIT_RESULT,
    );

    // ── group ─────────────────────────────────────────────────────────────────
    assert_eq!(
        thingos::group::KIND_ID_THINGOS_GROUP,
        thingos::kinds::KIND_ID_THINGOS_GROUP,
    );

    // ── place ─────────────────────────────────────────────────────────────────
    assert_eq!(
        thingos::place::KIND_ID_THINGOS_PLACE,
        thingos::kinds::KIND_ID_THINGOS_PLACE,
    );
}
