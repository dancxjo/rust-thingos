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
//! capability passing).  See `docs/concepts/ports_vs_pipes.md`.
//!
//! # Terminology
//!
//! The kernel-managed reference to an open object is a **handle**.
//! User-facing docs and APIs should use precise terminology: **port** for the
//! IPC object and **handle/fd** for the opened reference.
//!
//! # Ontology note
//!
//! In the ThingOS typed-world ontology a port or pipe is a **Thing** whose
//! **Kind** determines its message semantics.  An `IpcHandle` is the current
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

pub use handles::{
    IpcHandle, IpcHandleEntry, IpcHandleMode, IpcHandleTable, MAX_IPC_HANDLES,
};
pub use port::{Port, PortId, Receiver, Sender};

use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::Mutex;

/// Global port registry
static PORTS: Mutex<Vec<Option<Arc<Port>>>> = Mutex::new(Vec::new());

/// Global Handle Table (Single Process Model for v0)


/// Create a new port and return its ID
pub fn create_port(capacity: usize) -> PortId {
    let port = Arc::new(Port::new(capacity));
    let mut ports = PORTS.lock();
    crate::kdebug!("CREATE_PORT: capacity={} port={:p}", capacity, Arc::as_ptr(&port));

    // Find a free slot or append
    for (i, slot) in ports.iter_mut().enumerate() {
        if slot.is_none() {
            *slot = Some(port);
            let id = PortId(i as u32);
            crate::kdebug!("CREATE_PORT: slot={} id={:?}", i, id);
            return id;
        }
    }

    // No free slot, append
    let id = PortId(ports.len() as u32);
    ports.push(Some(port));
    crate::kdebug!("CREATE_PORT: appended id={:?}", id);
    id
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
