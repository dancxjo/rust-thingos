use super::ThreadId;

pub(crate) const SLEEP_WHEEL_SLOTS: usize = 256;

/// Timer-wheel entry for a sleeping thread.
#[derive(Debug, Clone, Copy)]
pub struct SleepEntry {
    pub tid: ThreadId,
    pub wake_tick: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SleepMembership {
    pub wake_tick: u64,
    pub bucket_index: usize,
}

#[inline]
pub(super) fn sleep_wheel_slot(wake_tick: u64) -> usize {
    (wake_tick as usize) % SLEEP_WHEEL_SLOTS
}
