use std::fs;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask crate must live directly under repository root")
        .to_path_buf()
}

fn read_repo_file(relative_path: &str) -> String {
    let path = repo_root().join(relative_path);
    fs::read_to_string(&path).unwrap_or_else(|err| {
        panic!(
            "failed to read guardrail target file '{}': {err}",
            path.display()
        )
    })
}

fn assert_no_forbidden_tokens(relative_path: &str, forbidden_tokens: &[&str]) {
    let contents = read_repo_file(relative_path);
    for token in forbidden_tokens {
        assert!(
            !contents.contains(token),
            "guardrail violated in {}: found forbidden token '{}'",
            relative_path,
            token
        );
    }
}

fn thread_sched_fields_block(state_rs: &str) -> &str {
    let start = state_rs
        .find("pub struct ThreadSchedFields {")
        .expect("ThreadSchedFields struct must exist");
    let after_start = &state_rs[start..];
    let end = after_start
        .find("\n}\n")
        .expect("ThreadSchedFields struct terminator must exist");
    &after_start[..end]
}

#[test]
fn scheduler_hot_paths_do_not_read_semantic_process_fields_directly() {
    let hot_scheduler_files = [
        "kernel/src/sched/blocking.rs",
        "kernel/src/sched/sleep.rs",
        "kernel/src/sched/wait_queue.rs",
        "kernel/src/sched/state.rs",
        "kernel/src/sched/vm.rs",
    ];
    let forbidden = [
        "process_info",
        "unix_compat",
        ".pid",
        ".ppid",
        ".pgid",
        ".sid",
        "session_leader",
        ".argv",
        ".cwd",
        ".namespace",
        ".exec_path",
        ".exit_code",
    ];

    for file in hot_scheduler_files {
        assert_no_forbidden_tokens(file, &forbidden);
    }
}

#[test]
fn scheduler_hot_paths_do_not_depend_on_compatibility_structs() {
    let hot_scheduler_files = [
        "kernel/src/sched/blocking.rs",
        "kernel/src/sched/sleep.rs",
        "kernel/src/sched/wait_queue.rs",
        "kernel/src/sched/state.rs",
        "kernel/src/sched/vm.rs",
    ];
    let forbidden = [
        "ProcessInfo",
        "ProcessUnixCompat",
        "thingos::job::",
        "thingos::group::",
        "thingos::authority::",
        "thingos::place::",
    ];

    for file in hot_scheduler_files {
        assert_no_forbidden_tokens(file, &forbidden);
    }
}

#[test]
fn thread_sched_fields_do_not_duplicate_lifecycle_or_identity_ownership() {
    let state_rs = read_repo_file("kernel/src/sched/state.rs");
    let fields = thread_sched_fields_block(&state_rs);
    let forbidden_field_decls = [
        "pub pid:",
        "pub ppid:",
        "pub pgid:",
        "pub sid:",
        "pub session_leader:",
        "pub argv:",
        "pub cwd:",
        "pub namespace:",
        "pub exec_path:",
        "pub exit_code:",
        "pub thread_ids:",
    ];

    for field_decl in forbidden_field_decls {
        assert!(
            !fields.contains(field_decl),
            "guardrail violated in ThreadSchedFields: duplicated semantic field '{}'",
            field_decl
        );
    }
}
