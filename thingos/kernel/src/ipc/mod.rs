//! Kernel IPC: ports and pipes
//!
//! Provides two distinct IPC primitives:
//! - **Ports** (`port.rs`): capability-gated, message-oriented queues.
//!   Discrete messages, preserved message boundaries, capability (handle)
//!   transfer.  Use for commands, events, RPC, capability passing.
//! - **Pipes** (`pipe.rs`): anonymous byte streams.  No message boundaries,
//!   no capability transfer.  Use for stdio, shell pipelines, raw data streams.
//!
//! # Key Rule
//!
//! Do not use a port as a byte stream (streaming raw PCM, text output, etc.)
//! and do not use a pipe for structured message exchange (commands, replies,
//! capability passing).  See `docs/concepts/channels_vs_pipes.md`.
//!
//! # Terminology
//!
//! The kernel-managed reference to an open object is called a **thing** in
//! Thing-OS — what POSIX calls a "file descriptor" and Win32 calls a
//! "handle".  User-facing docs and APIs should use **port** and **thing**
//! terminology consistently.
//!
//! # Ontology note
//!
//! In the ThingOS typed-world ontology a port or pipe is a **Thing** whose
//! **Kind** determines its message semantics.  An `IpcThing` is the current
//! compatibility reference Form for a Thing — the planned replacement is a
//! typed `Handle` that carries Kind information.  **Message** (see
//! `thingos::message`) is the canonical typed envelope for port payloads.
//!
//! See `docs/architecture/ontology.md` §1.1 (Thing), §1.2 (Kind), and §1.3
//! (Form) for the full definitions.

pub mod diag;
mod handles;
pub mod msgqueue;
pub mod pipe;
pub mod unix_socket;
mod port;

pub use handles::{IpcThing, IpcThingEntry, IpcThingMode, IpcThingTable, MAX_IPC_THINGS};
pub use port::{Port, PortId, Receiver, Sender};

use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::Mutex;

/// Global port registry
static PORTS: Mutex<Vec<Option<Arc<Port>>>> = Mutex::new(Vec::new());

/// Global Handle Table (Single Process Model for v0)
pub static GLOBAL_THING_TABLE: Mutex<IpcThingTable> = Mutex::new(IpcThingTable::new());

/// Create a new port and return its ID
pub fn create_port(capacity: usize) -> PortId {
    let port = Arc::new(Port::new(capacity));
    let mut ports = PORTS.lock();

    // Find a free slot or append
    for (i, slot) in ports.iter_mut().enumerate() {
        if slot.is_none() {
            *slot = Some(port);
            return PortId(i as u32);
        }
    }

    // No free slot, append
    let id = ports.len() as u32;
    ports.push(Some(port));
    PortId(id)
}

/// Get a port by ID
pub fn get_port(id: PortId) -> Option<Arc<Port>> {
    let ports = PORTS.lock();
    ports.get(id.0 as usize).and_then(|opt| opt.clone())
}

/// Close a port (for cleanup)
pub fn close_port(id: PortId) {
    let mut ports = PORTS.lock();
    if let Some(slot) = ports.get_mut(id.0 as usize) {
        *slot = None;
    }
}

/// Find the ID of a port by its Arc pointer
pub fn find_port_id(port: &Arc<Port>) -> Option<PortId> {
    let ports = PORTS.lock();
    for (i, slot) in ports.iter().enumerate() {
        if let Some(p) = slot {
            if Arc::ptr_eq(p, port) {
                return Some(PortId(i as u32));
            }
        }
    }
    None
}

/// Get statistics about the port registry (for debugging)
pub fn port_count() -> usize {
    let ports = PORTS.lock();
    ports.iter().filter(|s| s.is_some()).count()
}
