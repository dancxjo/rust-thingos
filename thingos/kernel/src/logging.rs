//! Unified Logging System v1.0
//!
//! Human-readable format: [TIME] [LEVEL] [SOURCE] Message
//! With optional span correlation for multi-line output.

use core::fmt::{self, Write};
use core::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, AtomicUsize, Ordering};

// Re-export for macros
pub use abi::logging::Level;

use crate::BootRuntimeBase;
pub type LogLevel = Level;

static LOGGER_READY: AtomicBool = AtomicBool::new(false);
static mut LOGGER_RUNTIME: Option<&'static dyn BootRuntimeBase> = None;

const MAX_LOG_BUFFER_SIZE: usize = 128 * 1024; // 128 KB

struct LogSlot {
    seq: AtomicUsize,
    byte: AtomicU8,
}

impl LogSlot {
    const fn new() -> Self {
        Self { seq: AtomicUsize::new(0), byte: AtomicU8::new(0) }
    }
}

struct LogRing {
    slots: [LogSlot; MAX_LOG_BUFFER_SIZE],
    write: AtomicUsize,
}

impl LogRing {
    const fn new() -> Self {
        Self { slots: [const { LogSlot::new() }; MAX_LOG_BUFFER_SIZE], write: AtomicUsize::new(0) }
    }

    fn push_slice(&self, data: &[u8]) {
        if data.is_empty() {
            return;
        }
        let start = self.write.fetch_add(data.len(), Ordering::AcqRel);
        let keep_from = data.len().saturating_sub(MAX_LOG_BUFFER_SIZE);
        for (offset, &b) in data.iter().enumerate().skip(keep_from) {
            let seq = start + offset;
            let slot = &self.slots[seq % MAX_LOG_BUFFER_SIZE];
            slot.byte.store(b, Ordering::Relaxed);
            slot.seq.store(seq.wrapping_add(1), Ordering::Release);
        }
    }

    fn len(&self) -> usize {
        self.write.load(Ordering::Acquire).min(MAX_LOG_BUFFER_SIZE)
    }

    fn copy_to(&self, buf: &mut [u8]) -> usize {
        let write = self.write.load(Ordering::Acquire);
        let n = write.min(MAX_LOG_BUFFER_SIZE).min(buf.len());
        let start = write.saturating_sub(n);
        let mut copied = 0;
        for seq in start..write {
            if copied >= n {
                break;
            }
            let slot = &self.slots[seq % MAX_LOG_BUFFER_SIZE];
            if slot.seq.load(Ordering::Acquire) != seq.wrapping_add(1) {
                break;
            }
            let byte = slot.byte.load(Ordering::Relaxed);
            if slot.seq.load(Ordering::Acquire) != seq.wrapping_add(1) {
                break;
            }
            buf[copied] = byte;
            copied += 1;
        }
        copied
    }

    #[cfg(test)]
    fn clear(&self) {
        self.write.store(0, Ordering::Release);
    }
}

static LOG_RING: LogRing = LogRing::new();

static MUTE_SERIAL: AtomicBool = AtomicBool::new(false);

/// Fatal-path marker set by `force_unlock()`.  Normal logging is lock-free, so
/// this is diagnostic state rather than a lock recovery mechanism.
static PANIC_ACTIVE: AtomicBool = AtomicBool::new(false);

/// Minimum log level to output (1=Error, 2=Warn, 3=Info, 4=Debug, 5=Trace, 0=Off)
/// Default is 3 (info) — matches the xtask build default and keeps verbose
/// debug/trace messages suppressed unless explicitly enabled via the kernel
/// command-line (`loglevel=debug`) or `SYS_SET_PARAM`.
static MIN_LOG_LEVEL: AtomicU8 = AtomicU8::new(3);

/// Set the minimum log level for output (0=Off, 1=Error+, 2=Warn+, etc.)
pub fn set_log_level(level: u8) {
    MIN_LOG_LEVEL.store(level, Ordering::Relaxed);
}

/// Get the current minimum log level
pub fn get_log_level() -> u8 {
    MIN_LOG_LEVEL.load(Ordering::Relaxed)
}

/// Cycle the minimum log level for low-level hotkey handling.
///
/// The cycle includes `Off` so repeated F1 presses can quiet a noisy system
/// without requiring a userspace shell or privileged syscall.
pub fn cycle_log_level() -> u8 {
    let mut current = MIN_LOG_LEVEL.load(Ordering::Relaxed);
    loop {
        let next = match current {
            0 => 1,
            1 => 2,
            2 => 3,
            3 => 4,
            4 => 5,
            _ => 0,
        };
        match MIN_LOG_LEVEL.compare_exchange_weak(
            current,
            next,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(_) => return next,
            Err(actual) => current = actual,
        }
    }
}

pub fn log_level_name(level: u8) -> &'static str {
    match level {
        0 => "OFF",
        1 => "ERROR",
        2 => "WARN",
        3 => "INFO",
        4 => "DEBUG",
        5 => "TRACE",
        _ => "UNKNOWN",
    }
}

/// Global sequence counter for log ordering
static GLOBAL_SEQ: AtomicU64 = AtomicU64::new(1);

/// Current active span (0 = none)
static CURRENT_SPAN: AtomicU64 = AtomicU64::new(0);

/// Global span counter for generating unique span IDs
static SPAN_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Generate a new unique span ID
pub fn new_span() -> u64 {
    SPAN_COUNTER.fetch_add(1, Ordering::Relaxed)
}

/// Set the current active span for this thread/context
pub fn set_current_span(span: u64) {
    CURRENT_SPAN.store(span, Ordering::Relaxed);
}

/// Get the current active span
pub fn current_span() -> u64 {
    CURRENT_SPAN.load(Ordering::Relaxed)
}

/// Clear the current span
pub fn clear_span() {
    CURRENT_SPAN.store(0, Ordering::Relaxed);
}

#[inline]
fn logger_runtime() -> Option<&'static dyn BootRuntimeBase> {
    if LOGGER_READY.load(Ordering::Acquire) { unsafe { LOGGER_RUNTIME } } else { None }
}

/// RAII guard for log transactions (multi-line atomic output)
pub struct LogTransaction {
    #[allow(dead_code)]
    span_id: u64,
    name: &'static str,
}

impl LogTransaction {
    /// Begin a log transaction - acquires exclusive write access
    pub fn begin(name: &'static str) -> Self {
        let span_id = new_span();
        set_current_span(span_id);

        // Emit BEGIN marker (always, like contract)
        let _seq = GLOBAL_SEQ.fetch_add(1, Ordering::Relaxed);

        if !MUTE_SERIAL.load(Ordering::Relaxed) {
            if let Some(runtime) = logger_runtime() {
                let ts = runtime.mono_ticks();
                let mut writer = Logger::new(runtime);
                let _ = writeln!(writer, "[{}] [INFO-] [logging] BEGIN {}", ts, name);
            }
        }

        Self { span_id, name }
    }
}

impl Drop for LogTransaction {
    fn drop(&mut self) {
        // Emit END marker (always, like contract)
        let _seq = GLOBAL_SEQ.fetch_add(1, Ordering::Relaxed);

        if !MUTE_SERIAL.load(Ordering::Relaxed) {
            if let Some(runtime) = logger_runtime() {
                let ts = runtime.mono_ticks();
                let mut writer = Logger::new(runtime);
                let _ = writeln!(writer, "[{}] [INFO-] [logging] END {}", ts, self.name);
            }
        }
        clear_span();
    }
}

#[derive(Clone, Copy)]
pub struct LogMetadata {
    pub level: Level,
    pub file: &'static str,
    pub line: u32,
    pub module: &'static str,
}

pub struct Logger {
    runtime: &'static dyn BootRuntimeBase,
}

impl Logger {
    pub const fn new(runtime: &'static dyn BootRuntimeBase) -> Self {
        Self { runtime }
    }

    #[inline]
    pub fn mono_ticks(&self) -> u64 {
        self.runtime.mono_ticks()
    }
}

#[inline]
fn write_crlf_translated(buf: &[u8], mut emit: impl FnMut(&[u8])) {
    let mut scratch = [0u8; 256];
    let mut used = 0usize;

    for &b in buf {
        if b == b'\n' {
            if used == scratch.len() {
                emit(&scratch[..used]);
                used = 0;
            }
            scratch[used] = b'\r';
            used += 1;
        }

        if used == scratch.len() {
            emit(&scratch[..used]);
            used = 0;
        }
        scratch[used] = b;
        used += 1;
    }

    if used != 0 {
        emit(&scratch[..used]);
    }
}

// Safety: BootRuntimeBase is effectively a singleton VTable provided by BRAN.
unsafe impl Sync for Logger {}
unsafe impl Send for Logger {}

impl fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write_crlf_translated(s.as_bytes(), |chunk| self.runtime.putbuf(chunk));
        Ok(())
    }
}

pub struct SyncLogger {
    runtime: &'static dyn BootRuntimeBase,
}

impl fmt::Write for SyncLogger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write_crlf_translated(s.as_bytes(), |chunk| self.runtime.serial_putbuf_sync(chunk));
        Ok(())
    }
}

pub unsafe fn init(runtime: &'static dyn BootRuntimeBase) {
    let irq = runtime.irq_disable();
    unsafe {
        LOGGER_RUNTIME = Some(runtime);
    }
    LOGGER_READY.store(true, Ordering::Release);
    runtime.irq_restore(irq);
}

pub unsafe fn force_unlock() {
    // Preserve the fatal-path marker for callers that use it as a diagnostic
    // transition, but normal logging no longer has global locks to seize.
    PANIC_ACTIVE.store(true, Ordering::Relaxed);
}

pub fn copy_log_buffer(buf: &mut [u8]) -> usize {
    LOG_RING.copy_to(buf)
}

pub fn get_log_buffer_len() -> usize {
    LOG_RING.len()
}

struct LogBufferWriter;

impl Write for LogBufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        LOG_RING.push_slice(s.as_bytes());
        Ok(())
    }
}

fn level_to_colored_str(level: Level) -> &'static str {
    match level {
        Level::Error => "\x1b[31;1mERROR\x1b[0m",
        Level::Warn => "\x1b[33mWARN \x1b[0m",
        Level::Info => "\x1b[32mINFO \x1b[0m",
        Level::Debug => "\x1b[34mDEBUG\x1b[0m",
        Level::Trace => "\x1b[35mTRACE\x1b[0m",
    }
}

/// Check if this level should be logged (considering MIN_LOG_LEVEL)
#[inline]
fn should_log(level: Level) -> bool {
    let min = MIN_LOG_LEVEL.load(Ordering::Relaxed);
    // If min is 0, all log output is disabled.
    // Otherwise, check if level <= min (Error=1 is most severe, Trace=5 is least)
    min > 0 && (level as u8) <= min
}

/// Maximum size of a pre-formatted log line (truncated if exceeded).
const LOG_LINE_BUF_SIZE: usize = 1024;

/// Fixed-capacity stack buffer for pre-formatting a log line without heap
/// allocation.  Used by `_log_event` to assemble one contiguous reservation
/// for the deferred output rings.
struct FixedBuf {
    buf: [u8; LOG_LINE_BUF_SIZE],
    pos: usize,
}

impl FixedBuf {
    #[inline]
    fn new() -> Self {
        Self { buf: [0; LOG_LINE_BUF_SIZE], pos: 0 }
    }

    #[inline]
    fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.pos]
    }
}

impl fmt::Write for FixedBuf {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let src = s.as_bytes();
        let available = self.buf.len() - self.pos;
        if src.len() > available {
            // Buffer full: copy what fits and signal truncation so callers
            // know the output is incomplete.
            self.buf[self.pos..].copy_from_slice(&src[..available]);
            self.pos = self.buf.len();
            return Err(fmt::Error);
        }
        self.buf[self.pos..self.pos + src.len()].copy_from_slice(src);
        self.pos += src.len();
        Ok(())
    }
}

pub fn _log_event(
    meta: LogMetadata,
    event_str: &str,
    msg_fmt: fmt::Arguments,
    fields: &[(&'static str, u64)],
    _about: &[u64],
) {
    // Fast path: bail out before any formatting work.
    if !should_log(meta.level) {
        return;
    }
    let _seq = GLOBAL_SEQ.fetch_add(1, Ordering::Relaxed);

    // Collect timing/CPU info before formatting the line.
    let (ts, cpu) = if crate::is_runtime_initialized() {
        let rt = crate::runtime_base();
        (rt.mono_ticks(), rt.current_cpu_index())
    } else {
        (0, 0)
    };

    // 1. Serial Output — pre-format the line before handing it to the deferred
    //    output rings. Each emitted chunk uses an atomic reservation, keeping
    //    producers non-blocking on spawn / syscall hot paths.
    if !MUTE_SERIAL.load(Ordering::Relaxed) {
        // Format: [TIME] [LEVEL] [SOURCE] [CPUx] Message [fields]\n
        let mut linebuf = FixedBuf::new();
        let _ = write!(
            linebuf,
            "[{}] [{}] [{}] [CPU{}] ",
            ts,
            level_to_colored_str(meta.level),
            event_str,
            cpu
        );
        let _ = linebuf.write_fmt(msg_fmt);
        if !fields.is_empty() {
            for (k, v) in fields {
                let _ = write!(linebuf, " {}={}", k, v);
            }
        }
        let _ = linebuf.write_char('\n');
        let line = linebuf.as_bytes();

        if let Some(runtime) = logger_runtime() {
            write_crlf_translated(line, |chunk| runtime.putbuf(chunk));
        }
    }

    // 2. Log Buffer Output — plain (no ANSI colour codes) version stored in
    //    the in-memory ring buffer for later retrieval.
    {
        let mut writer = LogBufferWriter;
        let _ = write!(writer, "[{}] [{}] [{}] [CPU{}] ", ts, meta.level.as_str(), event_str, cpu);
        let _ = writer.write_fmt(msg_fmt);
        if !fields.is_empty() {
            for (k, v) in fields {
                let _ = write!(writer, " {}={}", k, v);
            }
        }
        let _ = writer.write_str("\n");
    }

    // 3. Graph Persistence (REMOVED)
}

// Backward compatibility shim for kinfo! etc
pub fn _log(meta: LogMetadata, args: fmt::Arguments) {
    _log_event(meta.clone(), meta.module, args, &[], &[]);
}

/// Contract-level logging now maps directly to INFO logging semantics.
pub fn _log_contract(source: &'static str, args: fmt::Arguments) {
    _log_event(
        LogMetadata { level: Level::Info, file: file!(), line: line!(), module: source },
        source,
        args,
        &[],
        &[],
    );
}

/// Log a raw string without any formatting (for kprint! compatibility)
pub fn _log_raw(args: fmt::Arguments) {
    if !MUTE_SERIAL.load(Ordering::Relaxed) {
        if let Some(runtime) = logger_runtime() {
            let mut writer = Logger::new(runtime);
            let _ = writer.write_fmt(args);
        }
    }
}

/// Log a raw string synchronously (bypassing deferred buffers).
/// USE EXTREMELY SPARINGLY. Intended for fatal panic paths or low-level bring-up
/// debugging where deferred logging is unreliable or likely to deadlock.
#[doc(hidden)]
pub fn _log_raw_sync(args: fmt::Arguments) {
    if !MUTE_SERIAL.load(Ordering::Relaxed) {
        if let Some(runtime) = logger_runtime() {
            let mut writer = SyncLogger { runtime };
            let _ = writer.write_fmt(args);
        }
    }
}

/// Write a raw byte buffer to the serial and framebuffer consoles.
///
/// The historical name is kept for callers, but this path now uses the
/// lock-free deferred output rings.
pub fn write_bytes_locked(buf: &[u8]) {
    if MUTE_SERIAL.load(Ordering::Relaxed) {
        return;
    }
    if let Some(runtime) = logger_runtime() {
        write_crlf_translated(buf, |chunk| runtime.putbuf(chunk));
    }
}

/// Write a raw byte buffer to the serial port only (not the FB console).
pub fn write_serial_bytes_locked(buf: &[u8]) {
    if MUTE_SERIAL.load(Ordering::Relaxed) {
        return;
    }
    if let Some(runtime) = logger_runtime() {
        write_crlf_translated(buf, |chunk| runtime.serial_putbuf(chunk));
    }
}

/// Write a raw byte buffer to the framebuffer console only (not the serial
/// port).
pub fn write_fb_bytes_locked(buf: &[u8]) {
    if let Some(runtime) = logger_runtime() {
        write_crlf_translated(buf, |chunk| runtime.fb_putbuf(chunk));
    }
}

/// Contract-level logging macro (mapped to INFO level).
#[macro_export]
macro_rules! contract {
    ($($arg:tt)*) => {
        $crate::logging::_log_contract(module_path!(), format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_event {
    // With fields and about
    ($lvl:expr, $event:expr, $msg:expr, { $($k:ident : $v:expr),* }, about=[$($about:expr),*]) => {
        $crate::logging::_log_event(
            $crate::logging::LogMetadata {
                level: $lvl,
                file: file!(),
                line: line!(),
                module: module_path!(),
            },
            $event,
            format_args!($msg),
            &[ $( (stringify!($k), $v) ),* ],
            &[ $($about),* ]
        )
    };
    // With format args, no extra fields
    ($lvl:expr, $event:expr, $($arg:tt)*) => {
        $crate::logging::_log_event(
            $crate::logging::LogMetadata {
                level: $lvl,
                file: file!(),
                line: line!(),
                module: module_path!(),
            },
            $event,
            format_args!($($arg)*),
            &[],
            &[]
        )
    };
}

#[macro_export]
macro_rules! kinfo {
    ($($arg:tt)*) => {
        $crate::logging::_log(
            $crate::logging::LogMetadata {
                level: $crate::logging::LogLevel::Info,
                file: file!(),
                line: line!(),
                module: module_path!(),
            },
            format_args!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! kerror {
    ($($arg:tt)*) => {
        $crate::logging::_log(
            $crate::logging::LogMetadata {
                level: $crate::logging::LogLevel::Error,
                file: file!(),
                line: line!(),
                module: module_path!(),
            },
            format_args!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! kwarn {
    ($($arg:tt)*) => {
        $crate::logging::_log(
            $crate::logging::LogMetadata {
                level: $crate::logging::LogLevel::Warn,
                file: file!(),
                line: line!(),
                module: module_path!(),
            },
            format_args!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! kdebug {
    ($($arg:tt)*) => {
        $crate::logging::_log(
            $crate::logging::LogMetadata {
                level: $crate::logging::LogLevel::Debug,
                file: file!(),
                line: line!(),
                module: module_path!(),
            },
            format_args!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! ktrace {
    ($($arg:tt)*) => {
        $crate::logging::_log(
            $crate::logging::LogMetadata {
                level: $crate::logging::LogLevel::Trace,
                file: file!(),
                line: line!(),
                module: module_path!(),
            },
            format_args!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {
        $crate::logging::_log_raw(format_args!($($arg)*))
    };
}

/// Synchronous print macro that bypasses deferred ring buffers.
/// USE EXTREMELY SPARINGLY. Intended for fatal panic paths or low-level bring-up debugging.
#[macro_export]
macro_rules! kprint_sync {
    ($($arg:tt)*) => {
        $crate::logging::_log_raw_sync(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! kprintln {
    () => ($crate::kprint!("\n"));
    ($($arg:tt)*) => ($crate::kprint!("{}\n", format_args!($($arg)*)));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_LOCK: spin::Mutex<()> = spin::Mutex::new(());

    fn reset_test_state(min_level: u8) {
        MUTE_SERIAL.store(true, Ordering::Relaxed);
        set_log_level(min_level);
        LOG_RING.clear();
    }

    #[test]
    fn contract_log_is_formatted_as_info() {
        let _guard = TEST_LOCK.lock();
        reset_test_state(3);

        _log_contract("contract.test", format_args!("hello"));

        let mut buf = [0u8; 256];
        let n = copy_log_buffer(&mut buf);
        let s = core::str::from_utf8(&buf[..n]).expect("utf8");
        assert!(s.contains("[INFO] [contract.test] [CPU0] hello"));
    }

    #[test]
    fn contract_log_respects_info_filtering() {
        let _guard = TEST_LOCK.lock();
        reset_test_state(2);

        _log_contract("contract.test", format_args!("hidden"));

        assert_eq!(get_log_buffer_len(), 0);
    }
}
