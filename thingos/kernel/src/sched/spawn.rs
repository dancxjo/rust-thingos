//! Task and thread spawning functions.

mod boot;
mod path;
mod process_info;
mod stdio;
mod task;
mod types;

pub use boot::{boot_spawn_process, boot_spawn_process_ex};
pub use path::spawn_process_from_path;
pub use stdio::StdioSpec;
pub use task::{
    spawn, spawn_user_task_full, spawn_user_thread, spawn_user_thread_ex, spawn_with_priority,
    user_thread_trampoline,
};
pub use types::SpawnExResult;

#[cfg(test)]
mod tests;
