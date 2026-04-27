//! Unified Logging System v1.0
//!
//! Human-readable format: [TIME] [LEVEL] [SOURCE] Message
//! With optional span correlation for multi-line output.

use core::fmt::{self, Write};
use core::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, Ordering};

// Re-export for macros
pub use abi::logging::Level;
use spin::Mutex;

use crate::BootRuntimeBase;
pub type LogLevel = Level;

static GLOBAL_LOGGER: Mutex<Option<Logger>> = Mutex::new(None);

const MAX_LOG_BUFFER_SIZE: usize = 128 * 1024; // 128 KB
static mut LOG_DATA: [u8; MAX_LOG_BUFFER_SIZE] = [0; MAX_LOG_BUFFER_SIZE];

struct LogBufferState {
    head: usize,
    len: usize,
}
static LOG_STATE: Mutex<LogBufferState> = Mutex::new(LogBufferState { head: 0, len: 0 });

static IN_GRAPH_LOG: AtomicBool = AtomicBool::new(false);
static MUTE_SERIAL: AtomicBool = AtomicBool::new(false);

/// Set by `force_unlock()` so any in-flight `write_str` on another CPU
/// notices it should stop immediately, preventing garbled output after a
/// panic handler yanks the logger lock away.
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
/// The cycle includes `Off` so repeated F11 presses can quiet a noisy system
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

        let rt = if crate::is_runtime_initialized() { Some(crate::runtime_base()) } else { None };
        let irq_state = if cfg!(test) { None } else { rt.map(|r| r.irq_disable()) };

        let mut lock = GLOBAL_LOGGER.lock();
        if let Some(writer) = lock.as_mut() {
            if !MUTE_SERIAL.load(Ordering::Relaxed) {
                let ts = writer.runtime.mono_ticks();
                let _ = writeln!(writer, "[{}] [INFO-] [logging] BEGIN {}", ts, name);
            }
        }
        drop(lock);

        if let (Some(r), Some(s)) = (rt, irq_state) {
            r.irq_restore(s);
        }

        Self { span_id, name }
    }
}

impl Drop for LogTransaction {
    fn drop(&mut self) {
        // Emit END marker (always, like contract)
        let _seq = GLOBAL_SEQ.fetch_add(1, Ordering::Relaxed);

        let rt = if crate::is_runtime_initialized() { Some(crate::runtime_base()) } else { None };
        let irq_state = if cfg!(test) { None } else { rt.map(|r| r.irq_disable()) };

        let mut lock = GLOBAL_LOGGER.lock();
        if let Some(writer) = lock.as_mut() {
            if !MUTE_SERIAL.load(Ordering::Relaxed) {
                let ts = writer.runtime.mono_ticks();
                let _ = writeln!(writer, "[{}] [INFO-] [logging] END {}", ts, self.name);
            }
        }
        drop(lock);

        if let (Some(r), Some(s)) = (rt, irq_state) {
            r.irq_restore(s);
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
        // If a panic handler has seized the logger, abandon this write
        // immediately so the panic message comes through cleanly.
        if PANIC_ACTIVE.load(Ordering::Relaxed) {
            return Err(fmt::Error);
        }
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
    *GLOBAL_LOGGER.lock() = Some(Logger::new(runtime));
    runtime.irq_restore(irq);
}

pub unsafe fn force_unlock() {
    // Signal any in-flight write_str on other CPUs to bail out before we
    // yank the lock from under them.  The Relaxed store is sufficient
    // because the subsequent force_unlock provides the necessary fence.
    PANIC_ACTIVE.store(true, Ordering::Relaxed);

    // SAFETY: Only called from panic handler when logger lock may be poisoned
    unsafe {
        GLOBAL_LOGGER.force_unlock();
        LOG_STATE.force_unlock();
    }
}

pub fn copy_log_buffer(buf: &mut [u8]) -> usize {
    let state = LOG_STATE.lock();
    let n = state.len.min(buf.len());

    let mut read_idx = if state.len < MAX_LOG_BUFFER_SIZE { 0 } else { state.head };

    for i in 0..n {
        unsafe {
            buf[i] = LOG_DATA[read_idx];
        }
        read_idx = (read_idx + 1) % MAX_LOG_BUFFER_SIZE;
    }

    n
}

pub fn get_log_buffer_len() -> usize {
    LOG_STATE.lock().len
}

struct LogBufferWriter;

impl Write for LogBufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let rt = if crate::is_runtime_initialized() { Some(crate::runtime_base()) } else { None };
        let irq_state = if cfg!(test) { None } else { rt.map(|r| r.irq_disable()) };

        let mut state = LOG_STATE.lock();
        for &b in s.as_bytes() {
            unsafe {
                LOG_DATA[state.head] = b;
            }
            state.head = (state.head + 1) % MAX_LOG_BUFFER_SIZE;
            if state.len < MAX_LOG_BUFFER_SIZE {
                state.len += 1;
            }
        }
        drop(state);

        if let (Some(r), Some(s)) = (rt, irq_state) {
            r.irq_restore(s);
        }
        Ok(())
    }
}

/// Helper to check if graph logging is safe/ready
fn can_log_to_graph(_level: Level) -> bool {
    false
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
/// allocation.  Used by `_log_event` to assemble the complete message before
/// acquiring any lock, keeping the `GLOBAL_LOGGER` critical section as short
/// as possible.
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
    // Fast path: bail out before any lock or formatting work.
    if !should_log(meta.level) {
        return;
    }
    let _seq = GLOBAL_SEQ.fetch_add(1, Ordering::Relaxed);

    // Collect timing/CPU info before acquiring any lock.
    let (ts, cpu) = if crate::is_runtime_initialized() {
        let rt = crate::runtime_base();
        (rt.mono_ticks(), rt.current_cpu_index())
    } else {
        (0, 0)
    };

    // 1. Serial Output — pre-format the entire line into a stack buffer so
    //    that the GLOBAL_LOGGER lock is held only for a single contiguous
    //    `putbuf` call.  This reduces lock hold time and contention between
    //    CPUs that log concurrently on the spawn / syscall hot path.
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

        let rt = if crate::is_runtime_initialized() { Some(crate::runtime_base()) } else { None };
        let irq_state = if cfg!(test) { None } else { rt.map(|r| r.irq_disable()) };

        let mut lock = GLOBAL_LOGGER.lock();
        if let Some(writer) = lock.as_mut() {
            write_crlf_translated(line, |chunk| writer.runtime.putbuf(chunk));
        }
        drop(lock);

        if let (Some(r), Some(s)) = (rt, irq_state) {
            r.irq_restore(s);
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
        let rt = if crate::is_runtime_initialized() { Some(crate::runtime_base()) } else { None };
        let irq_state = if cfg!(test) { None } else { rt.map(|r| r.irq_disable()) };

        let mut lock = GLOBAL_LOGGER.lock();
        if let Some(writer) = lock.as_mut() {
            let _ = writer.write_fmt(args);
        }
        drop(lock);

        if let (Some(r), Some(s)) = (rt, irq_state) {
            r.irq_restore(s);
        }
    }
}

/// Log a raw string synchronously (bypassing deferred buffers).
/// USE EXTREMELY SPARINGLY. Intended for fatal panic paths or low-level bring-up
/// debugging where deferred logging is unreliable or likely to deadlock.
#[doc(hidden)]
pub fn _log_raw_sync(args: fmt::Arguments) {
    if !MUTE_SERIAL.load(Ordering::Relaxed) {
        let rt = if crate::is_runtime_initialized() { Some(crate::runtime_base()) } else { None };
        if let Some(r) = rt {
            let mut writer = SyncLogger { runtime: r };
            let _ = writer.write_fmt(args);
        }
    }
}

/// Write a raw byte buffer to the serial console under the `GLOBAL_LOGGER`
/// lock.  Used by the TTY write path so that userspace console output does
/// not interleave character-by-character with kernel log messages.
pub fn write_bytes_locked(buf: &[u8]) {
    if MUTE_SERIAL.load(Ordering::Relaxed) {
        return;
    }
    let rt = if crate::is_runtime_initialized() { Some(crate::runtime_base()) } else { None };
    let irq_state = if cfg!(test) { None } else { rt.map(|r| r.irq_disable()) };

    let mut lock = GLOBAL_LOGGER.lock();
    if let Some(writer) = lock.as_mut() {
        write_crlf_translated(buf, |chunk| writer.runtime.putbuf(chunk));
    }
    drop(lock);

    if let (Some(r), Some(s)) = (rt, irq_state) {
        r.irq_restore(s);
    }
}

/// Write a raw byte buffer to the serial port only (not the FB console),
/// under the `GLOBAL_LOGGER` lock to prevent interleaving.
pub fn write_serial_bytes_locked(buf: &[u8]) {
    if MUTE_SERIAL.load(Ordering::Relaxed) {
        return;
    }
    let rt = if crate::is_runtime_initialized() { Some(crate::runtime_base()) } else { None };
    let irq_state = if cfg!(test) { None } else { rt.map(|r| r.irq_disable()) };

    let mut lock = GLOBAL_LOGGER.lock();
    if let Some(writer) = lock.as_mut() {
        write_crlf_translated(buf, |chunk| writer.runtime.serial_putbuf(chunk));
    }
    drop(lock);

    if let (Some(r), Some(s)) = (rt, irq_state) {
        r.irq_restore(s);
    }
}

/// Write a raw byte buffer to the framebuffer console only (not the serial
/// port), under the `GLOBAL_LOGGER` lock to prevent interleaving.
pub fn write_fb_bytes_locked(buf: &[u8]) {
    let rt = if crate::is_runtime_initialized() { Some(crate::runtime_base()) } else { None };
    let irq_state = if cfg!(test) { None } else { rt.map(|r| r.irq_disable()) };

    let mut lock = GLOBAL_LOGGER.lock();
    if let Some(writer) = lock.as_mut() {
        write_crlf_translated(buf, |chunk| writer.runtime.fb_putbuf(chunk));
    }
    drop(lock);

    if let (Some(r), Some(s)) = (rt, irq_state) {
        r.irq_restore(s);
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
macro_rules! ki18n_tr {
    ($key:expr, $fallback:expr) => {
        $fallback
    };
}

#[macro_export]
macro_rules! kinfo {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::logging::_log(
            $crate::logging::LogMetadata {
                level: $crate::logging::LogLevel::Info,
                file: file!(),
                line: line!(),
                module: module_path!(),
            },
            format_args!($crate::ki18n_tr!(concat!("kernel.log.", file!(), ".", line!()), $fmt) $(, $arg)*)
        )
    };
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
    ($fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::logging::_log(
            $crate::logging::LogMetadata {
                level: $crate::logging::LogLevel::Error,
                file: file!(),
                line: line!(),
                module: module_path!(),
            },
            format_args!($crate::ki18n_tr!(concat!("kernel.log.", file!(), ".", line!()), $fmt) $(, $arg)*)
        )
    };
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
    ($fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::logging::_log(
            $crate::logging::LogMetadata {
                level: $crate::logging::LogLevel::Warn,
                file: file!(),
                line: line!(),
                module: module_path!(),
            },
            format_args!($crate::ki18n_tr!(concat!("kernel.log.", file!(), ".", line!()), $fmt) $(, $arg)*)
        )
    };
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
    ($fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::logging::_log(
            $crate::logging::LogMetadata {
                level: $crate::logging::LogLevel::Debug,
                file: file!(),
                line: line!(),
                module: module_path!(),
            },
            format_args!($crate::ki18n_tr!(concat!("kernel.log.", file!(), ".", line!()), $fmt) $(, $arg)*)
        )
    };
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
    ($fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::logging::_log(
            $crate::logging::LogMetadata {
                level: $crate::logging::LogLevel::Trace,
                file: file!(),
                line: line!(),
                module: module_path!(),
            },
            format_args!($crate::ki18n_tr!(concat!("kernel.log.", file!(), ".", line!()), $fmt) $(, $arg)*)
        )
    };
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
    ($fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::logging::_log_raw(
            format_args!($crate::ki18n_tr!(concat!("kernel.print.", file!(), ".", line!()), $fmt) $(, $arg)*)
        )
    };
    ($($arg:tt)*) => {
        $crate::logging::_log_raw(format_args!($($arg)*))
    };
}

/// Synchronous print macro that bypasses deferred ring buffers.
/// USE EXTREMELY SPARINGLY. Intended for fatal panic paths or low-level bring-up debugging.
#[macro_export]
macro_rules! kprint_sync {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::logging::_log_raw_sync(
            format_args!($crate::ki18n_tr!(concat!("kernel.print.", file!(), ".", line!()), $fmt) $(, $arg)*)
        )
    };
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
        let mut state = LOG_STATE.lock();
        state.head = 0;
        state.len = 0;
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
