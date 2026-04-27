use crate::BootRuntime;

/// Stdio specification for a single stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StdioSpec {
    /// Inherit parent's handle (child gets a dup of the parent's fd).
    Inherit,
    /// Attach to null sink/source.
    Null,
    /// Create a pipe; returns the pipe_id for the parent's end.
    Pipe,
    /// Clone the specified parent fd into this stdio slot.
    Fd(u32),
    /// Open the specified VFS path for this stdio slot.
    Path(alloc::string::String),
}

/// Populate the handle_table slots 0, 1, 2 in `handle_table` based on the given specs.
///
/// Returns the pipe IDs allocated for piped stdin/stdout/stderr (0 when not piped).
/// The parent uses these pipe IDs with the legacy `SYS_PIPE_*` syscalls; the child
/// reads/writes through the handle_table nodes which share the same underlying pipe.
pub(super) fn setup_stdio_fds<R: BootRuntime>(
    handle_table: &mut crate::vfs::handle_table::HandleTable,
    stdin_spec: StdioSpec,
    stdout_spec: StdioSpec,
    stderr_spec: StdioSpec,
) -> (u64, u64, u64) {
    use alloc::sync::Arc;

    use crate::vfs::{OpenFlags, VfsNode};

    crate::kdebug!(
        "SETUP_STDIO: stdin={:?} stdout={:?} stderr={:?}",
        stdin_spec,
        stdout_spec,
        stderr_spec
    );

    let console: Arc<dyn VfsNode> = Arc::new(crate::vfs::devfs::ConsoleNode);
    let null: Arc<dyn VfsNode> = Arc::new(crate::vfs::devfs::NullNode);

    // Helper: inherit parent's fd by cloning the node Arc and preserving the
    // original path. Watch delivery uses handle paths to compute mount IDs.
    let inherited_node = |fd: u32| -> Option<(Arc<dyn VfsNode>, OpenFlags, alloc::string::String)> {
        let tid = crate::runtime::<R>().current_tid();
        crate::task::registry::get_task::<R>(tid)
            .and_then(|task| task.process_info.clone())
            .and_then(|pi| {
                let lock = pi.lock();
                lock.handle_table.get(fd).ok().map(|f| {
                    f.node.on_dup();
                    (f.node.clone(), *f.status_flags.lock(), f.path.as_ref().clone())
                })
            })
    };

    let mut stdin_pipe: u64 = 0;
    let mut stdout_pipe: u64 = 0;
    let mut stderr_pipe: u64 = 0;

    // fd 0 — stdin
    match stdin_spec {
        StdioSpec::Inherit => {
            if let Some((node, flags, path)) = inherited_node(0) {
                let _ = handle_table.insert_at(0, node, flags, path);
            } else {
                let _ = handle_table.insert_at(
                    0,
                    console.clone(),
                    OpenFlags::read_only(),
                    "/dev/console".into(),
                );
            }
        }
        StdioSpec::Null => {
            let _ =
                handle_table.insert_at(0, null.clone(), OpenFlags::read_only(), "/dev/null".into());
        }
        StdioSpec::Pipe => {
            // Create the raw pipe (readers=1, writers=1).  The child's fd 0 is
            // the read end; the parent retains the write end via stdin_pipe.
            let id = crate::ipc::pipe::create(4096, 0);
            if let Some(read_node) = crate::ipc::pipe::read_node_for_id(id) {
                let _ = handle_table.insert_at(
                    0,
                    read_node,
                    OpenFlags::read_only(),
                    alloc::format!("pipe:{}", id),
                );
            }
            stdin_pipe = id;
        }
        StdioSpec::Fd(fd) => {
            if let Some((node, flags, path)) = inherited_node(fd) {
                let _ = handle_table.insert_at(0, node, flags, path);
            } else {
                crate::kwarn!(
                    "SPAWN: stdin explicit fd {} missing in parent; falling back to /dev/console",
                    fd
                );
                let _ = handle_table.insert_at(
                    0,
                    console.clone(),
                    OpenFlags::read_only(),
                    "/dev/console".into(),
                );
            }
        }
        StdioSpec::Path(ref path) => {
            // NOTE: We're in kernel context, so we use the global namespace.
            if let Ok(node) = crate::vfs::mount::lookup(path) {
                let _ = handle_table.insert_at(0, node, OpenFlags::read_write(), path.clone());
            } else {
                let _ = handle_table.insert_at(
                    0,
                    null.clone(),
                    OpenFlags::read_only(),
                    "/dev/null".into(),
                );
            }
        }
    }

    // fd 1 — stdout
    match stdout_spec {
        StdioSpec::Inherit => {
            if let Some((node, flags, path)) = inherited_node(1) {
                let _ = handle_table.insert_at(1, node, flags, path);
            } else {
                let _ = handle_table.insert_at(
                    1,
                    console.clone(),
                    OpenFlags::write_only(),
                    "/dev/console".into(),
                );
            }
        }
        StdioSpec::Null => {
            let _ = handle_table.insert_at(
                1,
                null.clone(),
                OpenFlags::write_only(),
                "/dev/null".into(),
            );
        }
        StdioSpec::Pipe => {
            let id = crate::ipc::pipe::create(4096, 0);
            if let Some(write_node) = crate::ipc::pipe::write_node_for_id(id) {
                let _ = handle_table.insert_at(
                    1,
                    write_node,
                    OpenFlags::write_only(),
                    alloc::format!("pipe:{}", id),
                );
            }
            stdout_pipe = id;
        }
        StdioSpec::Fd(fd) => {
            if let Some((node, flags, path)) = inherited_node(fd) {
                let _ = handle_table.insert_at(1, node, flags, path);
            } else {
                crate::kwarn!(
                    "SPAWN: stdout explicit fd {} missing in parent; falling back to /dev/console",
                    fd
                );
                let _ = handle_table.insert_at(
                    1,
                    console.clone(),
                    OpenFlags::write_only(),
                    "/dev/console".into(),
                );
            }
        }
        StdioSpec::Path(ref path) => {
            if let Ok(node) = crate::vfs::mount::lookup(path) {
                let _ = handle_table.insert_at(1, node, OpenFlags::read_write(), path.clone());
            } else {
                let _ = handle_table.insert_at(
                    1,
                    null.clone(),
                    OpenFlags::write_only(),
                    "/dev/null".into(),
                );
            }
        }
    }

    // fd 2 — stderr
    match stderr_spec {
        StdioSpec::Inherit => {
            if let Some((node, flags, path)) = inherited_node(2) {
                let _ = handle_table.insert_at(2, node, flags, path);
            } else {
                let _ = handle_table.insert_at(
                    2,
                    console,
                    OpenFlags::write_only(),
                    "/dev/console".into(),
                );
            }
        }
        StdioSpec::Null => {
            let _ = handle_table.insert_at(2, null, OpenFlags::write_only(), "/dev/null".into());
        }
        StdioSpec::Pipe => {
            let id = crate::ipc::pipe::create(4096, 0);
            if let Some(write_node) = crate::ipc::pipe::write_node_for_id(id) {
                let _ = handle_table.insert_at(
                    2,
                    write_node,
                    OpenFlags::write_only(),
                    alloc::format!("pipe:{}", id),
                );
            }
            stderr_pipe = id;
        }
        StdioSpec::Fd(fd) => {
            if let Some((node, flags, path)) = inherited_node(fd) {
                let _ = handle_table.insert_at(2, node, flags, path);
            } else {
                crate::kwarn!(
                    "SPAWN: stderr explicit fd {} missing in parent; falling back to /dev/console",
                    fd
                );
                let _ = handle_table.insert_at(
                    2,
                    console.clone(),
                    OpenFlags::write_only(),
                    "/dev/console".into(),
                );
            }
        }
        StdioSpec::Path(ref path) => {
            if let Ok(node) = crate::vfs::mount::lookup(path) {
                let _ = handle_table.insert_at(2, node, OpenFlags::read_write(), path.clone());
            } else {
                let _ = handle_table.insert_at(
                    2,
                    null.clone(),
                    OpenFlags::write_only(),
                    "/dev/null".into(),
                );
            }
        }
    }

    (stdin_pipe, stdout_pipe, stderr_pipe)
}
