#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TraceEvent {
    Empty,
    TimerTick { timestamp: u64 },
    Irq { vector: u8, timestamp: u64 },
    ContextSwitch { from: u64, to: u64, timestamp: u64 },
    PreemptDisable { depth: u32, timestamp: u64 },
    /// Periodic scheduler/IRQ heartbeat recorded once per second.
    Heartbeat { cpu: u8, tid: u64, runq_len: u32, timestamp: u64 },
    /// Recorded immediately after a context switch completes.
    SchedSwitch { cpu: u8, from_tid: u64, to_tid: u64, timestamp: u64 },
    /// VFS RPC milestone: entry (`enter=true`) or exit (`enter=false`).
    VfsRpc { enter: bool, op_tag: u32, timestamp: u64 },
}
