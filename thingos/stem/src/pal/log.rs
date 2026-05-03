//! Logging platform abstraction.
//!
//! Provides a stable interface for logging that abstracts over the underlying
//! syscall mechanism. This allows the logging implementation to evolve
//! independently of consumers.

use core::fmt;

use crate::syscall::log_write;

/// Log levels matching ABI definitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Level {
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

/// Write formatted output to the log at the specified level.
///
/// This is the low-level PAL primitive. Most code should use the
/// `error!`, `warn!`, `info!`, `debug!`, `trace!` macros instead.
pub fn write(level: Level, args: fmt::Arguments) {
    use core::fmt::Write;
    let mut cons = BufConsole::new(level as usize);
    let _ = cons.write_fmt(args);
    cons.flush();
}

/// Write formatted output to the log with source provenance.
///
/// The provenance string typically contains module path information.
pub fn write_with_provenance(level: Level, provenance: &str, args: fmt::Arguments) {
    use core::fmt::Write;
    let mut cons = BufConsole::new(level as usize);
    let _ = cons.write_str(provenance);
    let _ = cons.write_str(": ");
    let _ = cons.write_fmt(args);
    cons.flush();
}

/// Buffered console writer that accumulates output before syscall.
struct BufConsole {
    buf: [u8; 2048],
    len: usize,
    level: usize,
    truncated: bool,
}

impl BufConsole {
    fn new(level: usize) -> Self {
        BufConsole { buf: [0u8; 2048], len: 0, level, truncated: false }
    }

    fn flush(&mut self) {
        if self.truncated && self.len < self.buf.len() {
            let suffix = b" [truncated]";
            let available = self.buf.len() - self.len;
            let n = core::cmp::min(available, suffix.len());
            self.buf[self.len..self.len + n].copy_from_slice(&suffix[..n]);
            self.len += n;
        }

        if self.len > 0 {
            if let Ok(s) = core::str::from_utf8(&self.buf[..self.len]) {
                let _ = log_write(s, self.level);
            }
            self.len = 0;
            self.truncated = false;
        }
    }
}

impl fmt::Write for BufConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.truncated {
            return Ok(());
        }

        let space = self.buf.len().saturating_sub(self.len);
        let bytes = s.as_bytes();
        if bytes.len() <= space {
            self.buf[self.len..self.len + bytes.len()].copy_from_slice(bytes);
            self.len += bytes.len();
        } else {
            let suffix_len = b" [truncated]".len();
            let target_len = self.buf.len().saturating_sub(suffix_len);
            if self.len > target_len {
                self.len = target_len;
            }
            let copy_len = core::cmp::min(bytes.len(), target_len.saturating_sub(self.len));
            self.buf[self.len..self.len + copy_len].copy_from_slice(&bytes[..copy_len]);
            self.len += copy_len;
            self.truncated = true;
        }
        Ok(())
    }
}
