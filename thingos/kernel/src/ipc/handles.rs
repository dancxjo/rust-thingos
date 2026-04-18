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

use alloc::sync::Arc;

/// Entry in the handle table
#[derive(Debug)]
pub struct IpcThingEntry {
    pub port: Arc<super::Port>,
    pub mode: IpcThingMode,
}

impl Clone for IpcThingEntry {
    fn clone(&self) -> Self {
        match self.mode {
            IpcThingMode::Read => self.port.open_reader(),
            IpcThingMode::Write => self.port.open_writer(),
        }
        Self { port: Arc::clone(&self.port), mode: self.mode }
    }
}

impl IpcThingEntry {
    pub fn new(port: Arc<super::Port>, mode: IpcThingMode) -> Self {
        match mode {
            IpcThingMode::Read => port.open_reader(),
            IpcThingMode::Write => port.open_writer(),
        }
        Self { port, mode }
    }
}

impl Drop for IpcThingEntry {
    fn drop(&mut self) {
        match self.mode {
            IpcThingMode::Read => {
                self.port.close_reader();
            }
            IpcThingMode::Write => {
                self.port.close_writer();
            }
        }
    }
}

/// Maximum handles per process (v0 limit)
pub const MAX_IPC_THINGS: usize = 1024;

/// Per-process handle table
#[derive(Debug, Clone)]
pub struct IpcThingTable {
    entries: alloc::vec::Vec<Option<IpcThingEntry>>,
}

impl IpcThingTable {
    pub fn new() -> Self {
        let mut entries = alloc::vec::Vec::with_capacity(MAX_IPC_THINGS);
        for _ in 0..MAX_IPC_THINGS {
            entries.push(None);
        }
        Self { entries }
    }

    /// Allocate a new handle for the given port and mode.
    /// Handle 0 is reserved as "invalid" for userspace conventions.
    pub fn alloc(&mut self, port: Arc<super::Port>, mode: IpcThingMode) -> Option<IpcThing> {
        for (i, slot) in self.entries.iter_mut().enumerate().skip(1) {
            if slot.is_none() {
                let port_id = super::find_port_id(&port).unwrap();
                *slot = Some(IpcThingEntry::new(port, mode));
                let h = IpcThing(i as u32);
                crate::kinfo!(
                    "ALLOC_HANDLE: handle={} port_id={:?} mode={:?}",
                    h.0,
                    port_id,
                    mode
                );
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

    /// Insert an entry at a specific index. Used for handle translation.
    pub fn insert_at(&mut self, idx: usize, entry: IpcThingEntry) -> Result<(), ()> {
        if idx >= MAX_IPC_THINGS {
            return Err(());
        }
        if self.entries[idx].is_some() {
            return Err(());
        }
        self.entries[idx] = Some(entry);
        Ok(())
    }
}


impl Default for IpcThingTable {
    fn default() -> Self {
        Self::new()
    }
}
