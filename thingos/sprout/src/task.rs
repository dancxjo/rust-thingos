#![no_std]
use alloc::string::ToString;
use core::default::Default;
extern crate alloc;
use alloc::string::String;

#[derive(Debug, PartialEq, Default)]
pub enum TaskKind {
    #[default]
    App,
    Driver(String),  // Device Kind
    Service(String), // Service Kind
}

#[derive(Default)]
pub struct ManagedTask {
    pub name: String,
    pub kind: TaskKind,
    #[allow(dead_code)]
    pub module_path: String,
    pub pid: Option<u64>,
    pub restarts: u32,
    /// Original argument passed to spawn_process, preserved for restarts
    pub spawn_arg: usize,
    /// Unique token for sovereign registration handshake
    pub bind_instance_id: u64,
    /// Set to `true` once a `DRIVER_READY` inbox message is received for this
    /// task's PID.  Services that do not send `DRIVER_READY` remain `false`
    /// but are still supervised normally.
    pub ready: bool,
}

impl ManagedTask {
    pub fn new(name: String, kind: TaskKind) -> Self {
        Self { name, kind, ..Default::default() }
    }
}
