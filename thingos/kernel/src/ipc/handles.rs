//! Handle table: Per-process capability-gated port access
//!
//! Handles are indices into a per-task table that map to ports with
//! specific access modes (read or write).

use super::PortId;

/// A handle is an index into the process handle table
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IpcThing(pub u32);

/// Backward-compatible alias while code migrates from handle terminology.
pub type Handle = IpcThing;

/// Access mode for a handle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpcThingMode {
    Read,
    Write,
}

/// Entry in the handle table
#[derive(Debug, Clone, Copy)]
pub struct IpcThingEntry {
    pub port_id: PortId,
    pub mode: IpcThingMode,
}

/// Maximum handles per process (v0 limit)
pub const MAX_IPC_THINGS: usize = 1024;

/// Per-process handle table
#[derive(Debug)]
pub struct IpcThingTable {
    entries: [Option<IpcThingEntry>; MAX_IPC_THINGS],
}

impl IpcThingTable {
    /// Create a new empty handle table
    pub const fn new() -> Self {
        Self {
            entries: [None; MAX_IPC_THINGS],
        }
    }

    /// Allocate a new handle for the given port and mode.
    /// Handle 0 is reserved as "invalid" for userspace conventions.
    pub fn alloc(&mut self, port_id: PortId, mode: IpcThingMode) -> Option<IpcThing> {
        for (i, slot) in self.entries.iter_mut().enumerate().skip(1) {
            if slot.is_none() {
                *slot = Some(IpcThingEntry { port_id, mode });
                let h = IpcThing(i as u32);
                crate::kinfo!("ALLOC_HANDLE: handle={} port_id={:?} mode={:?}", h.0, port_id, mode);
                return Some(h);
            }
        }
        None // No free slots
    }

    /// Get the entry for a handle, validating mode
    pub fn get(&self, handle: IpcThing, required_mode: IpcThingMode) -> Option<&IpcThingEntry> {
        let idx = handle.0 as usize;
        if idx >= MAX_IPC_THINGS {
            return None;
        }
        self.entries[idx]
            .as_ref()
            .filter(|e| e.mode == required_mode)
    }

    /// Get the entry for a handle without mode validation
    pub fn get_any(&self, handle: IpcThing) -> Option<&IpcThingEntry> {
        let idx = handle.0 as usize;
        if idx >= MAX_IPC_THINGS {
            return None;
        }
        self.entries[idx].as_ref()
    }

    /// Close a handle, freeing the slot
    pub fn close(&mut self, handle: IpcThing) -> Option<IpcThingEntry> {
        let idx = handle.0 as usize;
        if idx >= MAX_IPC_THINGS {
            return None;
        }
        self.entries[idx].take()
    }
}

impl Default for IpcThingTable {
    fn default() -> Self {
        Self::new()
    }
}
