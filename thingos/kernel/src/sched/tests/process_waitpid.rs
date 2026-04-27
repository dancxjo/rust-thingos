use core::sync::atomic::Ordering;

use super::super::*;
use super::support::*;
use crate::task::{Affinity, TaskId, TaskPriority, TaskState};
use crate::{BootRuntimeBase, BootTasking};

#[test]
fn test_list_processes_hides_exit_code_for_live_job() {
    let _g = init_test_env();

    // Stale/non-authoritative exit_code on a live task should not surface
    // through ProcessSnapshot.
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(
        make_process_task(1200, TaskState::Running, 1200, 1, Some(77)),
    ));

    let snapshots = list_processes::<MockRuntime>();
    assert_eq!(snapshots.len(), 1);
    assert_eq!(snapshots[0].pid, 1200);
    assert_eq!(snapshots[0].state, TaskState::Running);
    assert_eq!(snapshots[0].exit_code, None);
}

#[test]
fn test_list_processes_prefers_job_leader_exit_code_for_leader() {
    let _g = init_test_env();

    let mut leader = make_process_task(1210, TaskState::Dead, 1210, 1, Some(7));
    let pinfo = leader.process_info.as_ref().unwrap();
    pinfo.lock().job.leader_exit_code = Some(33);

    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(leader));

    let snapshots = list_processes::<MockRuntime>();
    assert_eq!(snapshots.len(), 1);
    assert_eq!(snapshots[0].pid, 1210);
    assert_eq!(snapshots[0].tid, 1210);
    assert_eq!(snapshots[0].state, TaskState::Dead);
    assert_eq!(snapshots[0].exit_code, Some(33));
}

#[test]
fn test_list_processes_uses_thread_exit_code_for_non_leader_tasks() {
    let _g = init_test_env();

    let pinfo = alloc::sync::Arc::new(spin::Mutex::new(crate::task::ProcessInfo {
        pid: 1220,
        job: crate::job::Job {
            ppid: 1,
            thread_ids: alloc::vec![1220, 1221],
            exec_in_progress: false,
            children_done: alloc::collections::VecDeque::new(),
            exit_observer_inbox: None,
            leader_exit_code: Some(44),
            leader_exit_waiters: crate::sched::WaitQueue::new(),
        },
        unix_compat: crate::task::ProcessUnixCompat::isolated(1220, false),
        handle_table: crate::vfs::handle_table::HandleTable::new(),
        ipc_table: crate::ipc::IpcHandleTable::new(),
        namespace: crate::vfs::NamespaceRef::global(),
        cwd: alloc::string::String::from("/"),
        root: alloc::string::String::from("/"),
        exec_path: alloc::string::String::new(),
        authority: crate::task::ProcessAuthority::root(),
        space: crate::task::ProcessAddressSpace::empty(),
        service_loop: None,
    }));

    let mut sibling = make_task(1221, TaskState::Dead, TaskPriority::Normal);
    sibling.exit_code = Some(9);
    sibling.process_info = Some(alloc::sync::Arc::clone(&pinfo));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(sibling));

    let snapshots = list_processes::<MockRuntime>();
    assert_eq!(snapshots.len(), 1);
    assert_eq!(snapshots[0].tid, 1221);
    assert_eq!(snapshots[0].exit_code, Some(9));
}

#[test]
fn test_list_processes_projects_place_and_authority_from_process_aggregator() {
    let _g = init_test_env();

    let mut leader = make_process_task(1230, TaskState::Running, 1230, 1, None);
    let pinfo = leader.process_info.as_ref().unwrap();
    {
        let mut pi = pinfo.lock();
        pi.cwd = alloc::string::String::from("/work");
        pi.root = alloc::string::String::from("/srv/chroot");
        pi.exec_path = alloc::string::String::from("/bin/demo");
        pi.authority.uid = 1000;
        pi.authority.gid = 1001;
        pi.authority.capability_mask = 0x24;
        pi.namespace = crate::vfs::NamespaceRef::isolated();
    }
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(leader));

    let snapshots = list_processes::<MockRuntime>();
    assert_eq!(snapshots.len(), 1);
    let snap = &snapshots[0];
    assert_eq!(snap.pid, 1230);
    assert_eq!(snap.cwd, "/work");
    assert_eq!(snap.root_path, "/srv/chroot");
    assert_eq!(snap.exec_path, "/bin/demo");
    assert_eq!(snap.uid, 1000);
    assert_eq!(snap.gid, 1001);
    assert_eq!(snap.capability_mask, 0x24);
    assert_ne!(snap.namespace_label, "global");
}

#[test]
fn test_waitpid_returns_exit_code_for_dead_specific_child() {
    let _g = init_test_env();

    // Child task: pid=1001, ppid=1000, exit_code=42.
    // Tests call waitpid_for_pid directly so no parent task is needed.
    let child = make_process_task(9101, TaskState::Dead, 1001, 1000, Some(42));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(child));

    let (child_pid, code) =
        waitpid_for_pid::<MockRuntime>(1000, 1001, 0).expect("waitpid specific");
    assert_eq!(child_pid, 1001, "returned child pid");
    assert_eq!(code, 42, "returned exit code");
}

#[test]
fn test_waitpid_returns_exit_code_for_any_dead_child() {
    let _g = init_test_env();

    // Child: pid=2001, ppid=2000, exit_code=7
    let child = make_process_task(9201, TaskState::Dead, 2001, 2000, Some(7));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(child));

    // pid == -1: wait for any child of process 2000
    let (child_pid, code) = waitpid_for_pid::<MockRuntime>(2000, -1, 0).expect("waitpid any");
    assert_eq!(child_pid, 2001);
    assert_eq!(code, 7);
}

#[test]
fn test_waitpid_echild_when_no_children_exist() {
    let _g = init_test_env();

    // Registry is empty; no children for pid=3000
    let err = waitpid_for_pid::<MockRuntime>(3000, -1, 0).unwrap_err();
    assert_eq!(err, abi::errors::Errno::ECHILD);
}

#[test]
fn test_waitpid_echild_when_specific_child_not_found() {
    let _g = init_test_env();

    // A child with a *different* ppid — should not be found for pid=4000
    let unrelated = make_process_task(9401, TaskState::Dead, 9999, 5000, Some(0));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(unrelated));

    // Looking for child pid=9999 under parent pid=4000 → ECHILD
    let err = waitpid_for_pid::<MockRuntime>(4000, 9999, 0).unwrap_err();
    assert_eq!(err, abi::errors::Errno::ECHILD);
}

#[test]
fn test_waitpid_wnohang_returns_zero_when_child_alive() {
    let _g = init_test_env();

    // Live child — not yet exited
    let child = make_process_task(9501, TaskState::Runnable, 5001, 5000, None);
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(child));

    let (child_pid, code) =
        waitpid_for_pid::<MockRuntime>(5000, -1, abi::types::waitpid_flags::WNOHANG)
            .expect("wnohang");
    assert_eq!(child_pid, 0, "no child exited yet");
    assert_eq!(code, 0);
}

#[test]
fn test_waitpid_multiple_children_returns_first_dead() {
    let _g = init_test_env();

    // Two children under parent pid=6000: first alive, second dead
    let child_alive = make_process_task(9601, TaskState::Runnable, 6001, 6000, None);
    let child_dead = make_process_task(9602, TaskState::Dead, 6002, 6000, Some(99));

    let mut reg = crate::task::registry::get_registry::<MockRuntime>();
    reg.insert(alloc::boxed::Box::new(child_alive));
    reg.insert(alloc::boxed::Box::new(child_dead));
    drop(reg);

    let (child_pid, code) = waitpid_for_pid::<MockRuntime>(6000, -1, 0).expect("waitpid multi");
    assert_eq!(child_pid, 6002);
    assert_eq!(code, 99);
}

/// After `waitpid` successfully returns a dead child's exit code the child's
/// registry entry must be removed (reaped).  Without reaping, dead process
/// records accumulate indefinitely ("zombie" leak).
#[test]
fn test_waitpid_reaps_dead_child() {
    let _g = init_test_env();

    let child = make_process_task(9701, TaskState::Dead, 7001, 7000, Some(55));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(child));

    // The child is still in the registry before the wait.
    assert!(
        crate::task::registry::get_task::<MockRuntime>(9701).is_some(),
        "child must be in registry before waitpid"
    );

    let (child_pid, code) = waitpid_for_pid::<MockRuntime>(7000, 7001, 0).expect("waitpid reap");
    assert_eq!(child_pid, 7001);
    assert_eq!(code, 55);

    // After a successful wait the child record must have been reaped.
    assert!(
        crate::task::registry::get_task::<MockRuntime>(9701).is_none(),
        "child record must be removed from registry after reaping"
    );
}

/// Once a child has been reaped, a second `waitpid` for the same child must
/// return `ECHILD` — the record no longer exists.
#[test]
fn test_waitpid_echild_after_reaping() {
    let _g = init_test_env();

    let child = make_process_task(9702, TaskState::Dead, 7002, 7003, Some(0));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(child));

    // First wait reaps the child.
    waitpid_for_pid::<MockRuntime>(7003, 7002, 0).expect("first waitpid");

    // Second wait must fail because the record was removed.
    let err = waitpid_for_pid::<MockRuntime>(7003, 7002, 0).unwrap_err();
    assert_eq!(err, abi::errors::Errno::ECHILD, "second waitpid after reaping must return ECHILD");
}

/// A dead child whose parent has NOT yet called `waitpid` must remain in
/// the registry (zombie semantics: exit status preserved until collected).
#[test]
fn test_dead_child_stays_in_registry_until_reaped() {
    let _g = init_test_env();

    let child = make_process_task(9703, TaskState::Dead, 7010, 7011, Some(3));
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(child));

    // No waitpid called yet — record must still be present.
    assert!(
        crate::task::registry::get_task::<MockRuntime>(9703).is_some(),
        "dead child must remain in registry until reaped"
    );

    // Verify state and exit code are accessible while zombie.
    let task = crate::task::registry::get_task::<MockRuntime>(9703).unwrap();
    assert_eq!(task.state, TaskState::Dead);
    assert_eq!(task.exit_code, Some(3));
}

/// `waitpid` with WNOHANG must not reap any child when no child has exited.
#[test]
fn test_waitpid_wnohang_does_not_reap_live_child() {
    let _g = init_test_env();

    let child = make_process_task(9704, TaskState::Runnable, 7020, 7021, None);
    crate::task::registry::get_registry::<MockRuntime>().insert(alloc::boxed::Box::new(child));

    let (returned_pid, _) =
        waitpid_for_pid::<MockRuntime>(7021, -1, abi::types::waitpid_flags::WNOHANG)
            .expect("wnohang on live child");
    assert_eq!(returned_pid, 0, "wnohang returns 0 when no child exited");

    // Live child must still be in registry.
    assert!(
        crate::task::registry::get_task::<MockRuntime>(9704).is_some(),
        "live child must remain in registry after WNOHANG poll"
    );
}
