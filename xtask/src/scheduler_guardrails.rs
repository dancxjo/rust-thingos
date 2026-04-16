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
    let contents_lower = contents.to_ascii_lowercase();
    for token in forbidden_tokens {
        let token_lower = token.to_ascii_lowercase();
        assert!(
            !contents_lower.contains(&token_lower),
            "guardrail violated in {}: found forbidden token '{}'",
            relative_path,
            token
        );
    }
}

fn thread_sched_fields_block(state_rs: &str) -> &str {
    const STRUCT_DECL: &str = "pub struct ThreadSchedFields {";
    let start = state_rs
        .find(STRUCT_DECL)
        .expect("ThreadSchedFields struct must exist");
    let open_brace = start + STRUCT_DECL.len() - 1;
    assert_eq!(
        state_rs.as_bytes().get(open_brace).copied(),
        Some(b'{'),
        "ThreadSchedFields opening brace must exist at declaration"
    );
    let mut depth = 0usize;
    for (offset, ch) in state_rs[open_brace..].char_indices() {
        match ch {
            '{' => depth = depth.saturating_add(1),
            '}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    let close_brace = open_brace + offset;
                    return &state_rs[(open_brace + 1)..close_brace];
                }
            }
            _ => {}
        }
    }
    panic!("ThreadSchedFields closing brace must exist")
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

    // This is intentionally conservative string matching for fast CI guardrails.
    // If a new comment or literal trips a rule, tighten the path or token list.
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
