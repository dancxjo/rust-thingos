use crate::task::TaskId;

/// Result of an enhanced spawn: child tid + pipe IDs for piped stdio.
#[derive(Debug, Clone)]
pub struct SpawnExResult {
    pub child_tid: TaskId,
    pub child_pid: u32,
    /// Parent's fd for stdin (parent writes to this fd). 0 if not piped.
    pub stdin_pipe: u64,
    /// Parent's fd for stdout (parent reads from this fd). 0 if not piped.
    pub stdout_pipe: u64,
    /// Parent's fd for stderr (parent reads from this fd). 0 if not piped.
    pub stderr_pipe: u64,
}
