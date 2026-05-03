use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

pub static SYSTEM_TIME_OFFSET: AtomicU64 = AtomicU64::new(0);
static IS_ANCHORED: AtomicBool = AtomicBool::new(false);
static MONOTONIC_CLAMP: MonotonicClamp = MonotonicClamp::new();
static LAST_SYSTEM_CLOCK_TICK_LOG_NS: AtomicU64 = AtomicU64::new(0);

pub const NANOS_PER_SEC: u64 = 1_000_000_000;
pub const SYSTEM_CLOCK_TICK_LOG_INTERVAL_NS: u64 = 3 * NANOS_PER_SEC;
pub const SCHED_TICK_HZ: u64 = 100;
pub const SCHED_TICK_NANOS: u64 = NANOS_PER_SEC / SCHED_TICK_HZ;

/// Returns true if the system clock has been anchored to a wall-clock time source.
pub fn is_anchored() -> bool {
    IS_ANCHORED.load(Ordering::Relaxed)
}

/// Returns system time in nanoseconds (monotonic + offset)
pub fn get_system_time_ns(mono_ns: u64) -> u64 {
    mono_ns.saturating_add(SYSTEM_TIME_OFFSET.load(Ordering::Relaxed))
}

/// Set the system time offset directly (in nanoseconds)
pub fn set_system_time_offset(offset_ns: u64) {
    SYSTEM_TIME_OFFSET.store(offset_ns, Ordering::Relaxed);
}

/// Anchor system clock: given a Unix timestamp and the corresponding monotonic time,
/// compute and store the offset so that future time queries return correct wall-clock time.
pub fn anchor_system_clock(unix_secs: u64, mono_ns: u64) {
    let unix_ns = unix_secs.saturating_mul(NANOS_PER_SEC);
    let offset = unix_ns.saturating_sub(mono_ns);
    set_system_time_offset(offset);
    IS_ANCHORED.store(true, Ordering::Relaxed);
    crate::kdebug!(
        "System clock anchored: unix_secs={} mono_ns={} offset={}ns",
        unix_secs,
        mono_ns,
        offset
    );
}

/// Return the current wall-clock time as `(seconds, nanoseconds)`.
///
/// If the runtime has not yet been initialized (e.g. during early boot or
/// in unit tests), or if the clock has not been anchored to a real-time
/// source, returns `(0, 0)` — which corresponds to the Unix epoch.
/// Callers that need precise wall-clock time should check [`is_anchored`] first.
///
/// This is the canonical time source for VFS node timestamps.
pub fn now_timespec() -> (u64, u32) {
    let mono_ns = monotonic_now_ns();
    let sys_ns = if is_anchored() { get_system_time_ns(mono_ns) } else { 0 };
    let sec = sys_ns / NANOS_PER_SEC;
    let nsec = (sys_ns % NANOS_PER_SEC) as u32;
    (sec, nsec)
}

pub fn runtime_ticks_to_nanos(ticks: u64, freq_hz: u64) -> u64 {
    if freq_hz == 0 {
        return 0;
    }
    ((ticks as u128).saturating_mul(NANOS_PER_SEC as u128) / freq_hz as u128) as u64
}

pub fn monotonic_now_ns() -> u64 {
    if !crate::is_runtime_initialized() {
        return 0;
    }
    let rt = crate::runtime_base();
    let raw = runtime_ticks_to_nanos(rt.mono_ticks(), rt.mono_freq_hz());
    MONOTONIC_CLAMP.clamp(raw)
}

pub fn realtime_now_ns() -> Option<u64> {
    if !is_anchored() {
        return None;
    }
    Some(get_system_time_ns(monotonic_now_ns()))
}

/// Emit the kernel-owned wall-clock heartbeat at a fixed cadence.
///
/// This is driven by the scheduler timer, so the serial clock continues to tick
/// even if no userspace clock client is running or repainting.
pub fn maybe_log_system_clock_tick(mono_ns: u64) {
    if !is_anchored() {
        return;
    }

    let last = LAST_SYSTEM_CLOCK_TICK_LOG_NS.load(Ordering::Relaxed);
    if last != 0 && mono_ns.saturating_sub(last) < SYSTEM_CLOCK_TICK_LOG_INTERVAL_NS {
        return;
    }
    if LAST_SYSTEM_CLOCK_TICK_LOG_NS
        .compare_exchange(last, mono_ns, Ordering::Relaxed, Ordering::Relaxed)
        .is_err()
    {
        return;
    }

    let realtime_ns = get_system_time_ns(mono_ns);
    let secs = realtime_ns / NANOS_PER_SEC;
    let nanos = realtime_ns % NANOS_PER_SEC;
    let utc = unix_to_utc_datetime(secs);
    crate::kdebug!(
        "System clock sample: utc={:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:09} unix_secs={}.{:09}",
        utc.year,
        utc.month,
        utc.day,
        utc.hour,
        utc.minute,
        utc.second,
        nanos,
        secs,
        nanos
    );
}

#[derive(Clone, Copy)]
struct DateTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

fn unix_to_utc_datetime(seconds: u64) -> DateTime {
    let mut remaining = seconds;

    let second = (remaining % 60) as u8;
    remaining /= 60;
    let minute = (remaining % 60) as u8;
    remaining /= 60;
    let hour = (remaining % 24) as u8;
    remaining /= 24;

    let mut year = 1970u16;
    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining < days_in_year as u64 {
            break;
        }
        remaining -= days_in_year as u64;
        year = year.saturating_add(1);
    }

    let mut month = 1u8;
    loop {
        let dim = days_in_month(year, month);
        if remaining < dim as u64 {
            break;
        }
        remaining -= dim as u64;
        month += 1;
    }

    DateTime { year, month, day: (remaining + 1) as u8, hour, minute, second }
}

fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_leap_year(year: u16) -> bool {
    (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0))
}

pub fn duration_to_sleep_ticks(duration_ns: u64) -> u64 {
    if duration_ns == 0 {
        0
    } else {
        duration_ns.saturating_add(SCHED_TICK_NANOS - 1) / SCHED_TICK_NANOS
    }
}

pub struct MonotonicClamp {
    last: AtomicU64,
}

impl MonotonicClamp {
    pub const fn new() -> Self {
        Self { last: AtomicU64::new(0) }
    }

    pub fn clamp(&self, raw: u64) -> u64 {
        let last = self.last.load(Ordering::Relaxed);
        if raw > last {
            if let Err(actual) =
                self.last.compare_exchange(last, raw, Ordering::Relaxed, Ordering::Relaxed)
            {
                if actual > raw { actual } else { raw }
            } else {
                raw
            }
        } else {
            last
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        MonotonicClamp, NANOS_PER_SEC, SCHED_TICK_NANOS, SYSTEM_CLOCK_TICK_LOG_INTERVAL_NS,
        duration_to_sleep_ticks, runtime_ticks_to_nanos, unix_to_utc_datetime,
    };

    #[test]
    fn runtime_ticks_to_nanos_handles_zero_frequency() {
        assert_eq!(runtime_ticks_to_nanos(123, 0), 0);
    }

    #[test]
    fn runtime_ticks_to_nanos_converts_fractional_seconds() {
        assert_eq!(runtime_ticks_to_nanos(50, 100), NANOS_PER_SEC / 2);
    }

    #[test]
    fn sleep_tick_rounding_is_ceiling() {
        assert_eq!(duration_to_sleep_ticks(0), 0);
        assert_eq!(duration_to_sleep_ticks(1), 1);
        assert_eq!(duration_to_sleep_ticks(SCHED_TICK_NANOS), 1);
        assert_eq!(duration_to_sleep_ticks(SCHED_TICK_NANOS + 1), 2);
    }

    #[test]
    fn system_clock_tick_log_interval_is_three_seconds() {
        assert_eq!(SYSTEM_CLOCK_TICK_LOG_INTERVAL_NS, 3 * NANOS_PER_SEC);
    }

    #[test]
    fn unix_datetime_conversion_formats_known_utc_date() {
        let dt = unix_to_utc_datetime(0);
        assert_eq!(
            (dt.year, dt.month, dt.day, dt.hour, dt.minute, dt.second),
            (1970, 1, 1, 0, 0, 0)
        );

        let dt = unix_to_utc_datetime(1_582_934_400);
        assert_eq!(
            (dt.year, dt.month, dt.day, dt.hour, dt.minute, dt.second),
            (2020, 2, 29, 0, 0, 0)
        );
    }

    #[test]
    fn monotonic_clamp_never_goes_backwards() {
        let clamp = MonotonicClamp::new();
        assert_eq!(clamp.clamp(100), 100);
        assert_eq!(clamp.clamp(90), 100);
        assert_eq!(clamp.clamp(110), 110);
    }
}
