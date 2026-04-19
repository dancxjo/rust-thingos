//! Handle table: Per-process capability-gated port access
//!
//! Handles are indices into a per-task table that map to ports with
//! specific access modes (read or write).

use super::PortId;

/// A handle is an index into the process handle table
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IpcHandle(pub u32);

/// Backward-compatible alias.
pub type IpcThing = IpcHandle;
/// Backward-compatible alias.
pub type Handle = IpcHandle;

/// Access mode for a handle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpcHandleMode {
    Read,
    Write,
}
/// Backward-compatible alias.
pub type IpcThingMode = IpcHandleMode;

use alloc::sync::Arc;

/// Entry in the handle table
#[derive(Debug)]
pub struct IpcHandleEntry {
    pub port: Arc<super::Port>,
    pub mode: IpcHandleMode,
}
/// Backward-compatible alias.
pub type IpcThingEntry = IpcHandleEntry;

impl Clone for IpcHandleEntry {
    fn clone(&self) -> Self {
        match self.mode {
            IpcHandleMode::Read => self.port.open_reader(),
            IpcHandleMode::Write => self.port.open_writer(),
        }
        Self { port: Arc::clone(&self.port), mode: self.mode }
    }
}

impl IpcHandleEntry {
    pub fn new(port: Arc<super::Port>, mode: IpcHandleMode) -> Self {
        match mode {
            IpcHandleMode::Read => port.open_reader(),
            IpcHandleMode::Write => port.open_writer(),
        }
        Self { port, mode }
    }
}

impl Drop for IpcHandleEntry {
    fn drop(&mut self) {
        match self.mode {
            IpcHandleMode::Read => {
                self.port.close_reader();
            }
            IpcHandleMode::Write => {
                self.port.close_writer();
            }
        }
    }
}

/// Maximum handles per process (v0 limit)
pub const MAX_IPC_HANDLES: usize = 1024;

/// Per-process handle table
#[derive(Debug, Clone)]
pub struct IpcHandleTable {
    entries: alloc::vec::Vec<Option<IpcHandleEntry>>,
}
/// Backward-compatible alias.
pub type IpcThingTable = IpcHandleTable;

impl IpcHandleTable {
    pub fn new() -> Self {
        let mut entries = alloc::vec::Vec::with_capacity(MAX_IPC_HANDLES);
        for _ in 0..MAX_IPC_HANDLES {
            entries.push(None);
        }
        Self { entries }
    }

    /// Allocate a new handle for the given port and mode.
    /// Handle 0 is reserved as "invalid" for userspace conventions.
    pub fn alloc(&mut self, port: Arc<super::Port>, mode: IpcHandleMode) -> Option<IpcHandle> {
        for (i, slot) in self.entries.iter_mut().enumerate().skip(1) {
            if slot.is_none() {
                let port_id = super::find_port_id(&port).unwrap();
                *slot = Some(IpcHandleEntry::new(port, mode));
                let h = IpcHandle(i as u32);
                crate::kdebug!(
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
    pub fn get(&self, handle: IpcHandle, required_mode: IpcHandleMode) -> Option<&IpcHandleEntry> {
        let idx = handle.0 as usize;
        if idx >= MAX_IPC_HANDLES {
            return None;
        }
        self.entries[idx]
            .as_ref()
            .filter(|e| e.mode == required_mode)
    }

    /// Get the entry for a handle without mode validation
    pub fn get_any(&self, handle: IpcHandle) -> Option<&IpcHandleEntry> {
        let idx = handle.0 as usize;
        if idx >= MAX_IPC_HANDLES {
            return None;
        }
        self.entries[idx].as_ref()
    }

    /// Close a handle, freeing the slot
    pub fn close(&mut self, handle: IpcHandle) -> Option<IpcHandleEntry> {
        let idx = handle.0 as usize;
        if idx >= MAX_IPC_HANDLES {
            return None;
        }
        self.entries[idx].take()
    }

    /// Insert an entry at a specific index. Used for handle translation.
    pub fn insert_at(&mut self, idx: usize, entry: IpcHandleEntry) -> Result<(), ()> {
        if idx >= MAX_IPC_HANDLES {
            return Err(());
        }
        if self.entries[idx].is_some() {
            return Err(());
        }
        self.entries[idx] = Some(entry);
        Ok(())
    }
}


impl Default for IpcHandleTable {
    fn default() -> Self {
        Self::new()
    }
}
