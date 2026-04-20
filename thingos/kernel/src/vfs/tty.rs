//! Generic TTY line discipline and base implementation.

use alloc::collections::VecDeque;
use alloc::sync::Arc;
use spin::Mutex;

use abi::errors::{Errno, SysResult};
use abi::termios::{Termios, Winsize, DEFAULT_TERMIOS, ICANON, ECHO, ECHOE, ISIG, VINTR, VQUIT, VSUSP, VMIN, ICRNL, TOSTOP};

use super::{VfsNode, VfsStat};

/// Trait for the hardware-side of a TTY.
pub trait TtyHardware: Send + Sync {
    /// Read one raw byte from the hardware. Non-blocking.
    fn read_byte(&self) -> Option<u8>;
    /// Write one raw byte to the hardware.
    fn write_byte(&self, byte: u8);
    /// Get the window size of this TTY.
    fn winsize(&self) -> Winsize;
}

/// Line discipline state for a TTY.
pub struct LineDiscipline {
    /// Input buffer (after processing).
    pub buf: Mutex<VecDeque<u8>>,
    /// Current terminal settings.
    pub termios: Mutex<Termios>,
    /// Controlling session and foreground group.
    pub presence: Arc<Mutex<crate::presence::ConsolePresenceState>>,
}

impl LineDiscipline {
    pub fn new() -> Self {
        Self {
            buf: Mutex::new(VecDeque::with_capacity(1024)),
            termios: Mutex::new(DEFAULT_TERMIOS),
            presence: Arc::new(Mutex::new(crate::presence::ConsolePresenceState::default())),
        }
    }

    pub fn with_presence(presence: Arc<Mutex<crate::presence::ConsolePresenceState>>) -> Self {
        Self {
            buf: Mutex::new(VecDeque::with_capacity(1024)),
            termios: Mutex::new(DEFAULT_TERMIOS),
            presence,
        }
    }

    /// Process raw input bytes from the hardware.
    pub fn drain_input(&self, hw: &dyn TtyHardware) -> bool {
        let termios = *self.termios.lock();
        let canonical = termios.c_lflag & ICANON != 0;
        let do_echo = termios.c_lflag & ECHO != 0;
        let do_echo_erase = termios.c_lflag & ECHOE != 0;
        let icrnl = termios.c_iflag & ICRNL != 0;
        let isig = termios.c_lflag & ISIG != 0;

        let mut interrupted = false;

        while let Some(c) = hw.read_byte() {
            if isig {
                let vintr = termios.c_cc[VINTR];
                let vquit = termios.c_cc[VQUIT];
                let vsusp = termios.c_cc[VSUSP];

                let (sig, caret) = if c == vintr {
                    (abi::signal::SIGINT, b'C')
                } else if c == vquit {
                    (abi::signal::SIGQUIT, b'\\')
                } else if c == vsusp {
                    (abi::signal::SIGTSTP, b'Z')
                } else {
                    (0, 0)
                };

                if sig != 0 {
                    if do_echo {
                        hw.write_byte(b'^');
                        hw.write_byte(caret);
                        hw.write_byte(b'\r');
                        hw.write_byte(b'\n');
                    }
                    self.buf.lock().clear();
                    if let Some(pgid) = self.presence.lock().foreground_pgid {
                        crate::signal::send_signal_to_group(pgid, sig);
                    }
                    interrupted = true;
                    continue;
                }
            }

            match c {
                b'\r' | b'\n' => {
                    let mapped = if icrnl { b'\n' } else { c };
                    if do_echo {
                        hw.write_byte(b'\r');
                        hw.write_byte(b'\n');
                    }
                    self.buf.lock().push_back(mapped);
                }
                0x08 | 0x7f => {
                    if canonical {
                        let mut cb = self.buf.lock();
                        let last = cb.back().copied();
                        if last.is_some() && last != Some(b'\n') {
                            cb.pop_back();
                            if do_echo && do_echo_erase {
                                hw.write_byte(0x08);
                                hw.write_byte(b' ');
                                hw.write_byte(0x08);
                            }
                        }
                    } else {
                        self.buf.lock().push_back(c);
                    }
                }
                0x04 => {
                    if canonical {
                        self.buf.lock().push_back(0x04);
                    } else {
                        self.buf.lock().push_back(c);
                    }
                }
                0x20..=0x7e => {
                    if do_echo {
                        hw.write_byte(c);
                    }
                    self.buf.lock().push_back(c);
                }
                _ => {
                    if !canonical {
                        self.buf.lock().push_back(c);
                    }
                }
            }
        }

        interrupted
    }
}

pub struct TtyNode {
    pub hw: Arc<dyn TtyHardware>,
    pub ld: Arc<LineDiscipline>,
}

#[derive(Clone, Copy)]
struct TtyCaller {
    sid: u32,
    pgid: u32,
    session_leader: bool,
}

impl TtyNode {
    pub fn new(hw: Arc<dyn TtyHardware>) -> Self {
        Self {
            hw,
            ld: Arc::new(LineDiscipline::new()),
        }
    }

    fn get_caller(&self) -> Option<TtyCaller> {
        let pinfo = crate::sched::process_info_current()?;
        let p = pinfo.lock();
        Some(TtyCaller {
            sid: p.unix_compat.sid,
            pgid: p.unix_compat.pgid,
            session_leader: p.unix_compat.session_leader,
        })
    }

    fn maybe_acquire_controlling_tty(&self, caller: Option<TtyCaller>) {
        if let Some(c) = caller {
            if !c.session_leader { return; }
            let mut presence = self.ld.presence.lock();
            if presence.controlling_sid.is_none() {
                presence.controlling_sid = Some(c.sid);
                presence.foreground_pgid = Some(c.pgid);
            }
        }
    }

    fn is_background_caller(&self, caller: TtyCaller) -> bool {
        let presence_guard = self.ld.presence.lock();
        match (presence_guard.controlling_sid, presence_guard.foreground_pgid) {
            (Some(sid), Some(fg_pgid)) => caller.sid == sid && caller.pgid != fg_pgid,
            _ => false,
        }
    }

    fn enforce_job_control_before_read(&self) -> SysResult<()> {
        let caller = match self.get_caller() {
            Some(c) => c,
            None => return Ok(()),
        };
        self.maybe_acquire_controlling_tty(Some(caller));
        if self.is_background_caller(caller) {
            crate::signal::send_signal_to_group(caller.pgid, abi::signal::SIGTTIN);
            return Err(Errno::EINTR);
        }
        Ok(())
    }

    fn enforce_job_control_before_write(&self) -> SysResult<()> {
        let caller = match self.get_caller() {
            Some(c) => c,
            None => return Ok(()),
        };
        self.maybe_acquire_controlling_tty(Some(caller));
        let tostop = (self.ld.termios.lock().c_lflag & TOSTOP) != 0;
        if tostop && self.is_background_caller(caller) {
            crate::signal::send_signal_to_group(caller.pgid, abi::signal::SIGTTOU);
            return Err(Errno::EINTR);
        }
        Ok(())
    }
}

impl VfsNode for TtyNode {
    fn read(&self, _offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        self.enforce_job_control_before_read()?;

        loop {
            if crate::sched::take_pending_interrupt_current() {
                return Err(Errno::EINTR);
            }

            if self.ld.drain_input(&*self.hw) {
                return Err(Errno::EINTR);
            }

            let termios = *self.ld.termios.lock();
            let canonical = termios.c_lflag & ICANON != 0;
            let vmin = termios.c_cc[VMIN] as usize;
            let vmin_eff = vmin.max(1);

            let mut cb = self.ld.buf.lock();
            let ready = if canonical {
                cb.iter().any(|&b| b == b'\n' || b == 0x04) || cb.len() >= buf.len()
            } else {
                cb.len() >= vmin_eff || cb.len() >= buf.len()
            };

            if ready {
                let mut read_bytes = 0;
                while read_bytes < buf.len() {
                    if let Some(b) = cb.pop_front() {
                        if canonical && b == 0x04 {
                            // EOF
                            break;
                        }
                        buf[read_bytes] = b;
                        read_bytes += 1;
                        if canonical && b == b'\n' {
                            break;
                        }
                        if !canonical && read_bytes >= vmin_eff {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                return Ok(read_bytes);
            }
            drop(cb);

            unsafe { crate::sched::yield_now_current() };
        }
    }

    fn write(&self, _offset: u64, buf: &[u8]) -> SysResult<usize> {
        self.enforce_job_control_before_write()?;
        for &b in buf {
            if b == b'\n' {
                self.hw.write_byte(b'\r');
            }
            self.hw.write_byte(b);
        }
        Ok(buf.len())
    }

    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFCHR | 0o666,
            size: 0,
            ino: 1, // This should probably be unique per TTY
            nlink: 1,
            rdev: VfsStat::makedev(5, 0),
            ..Default::default()
        })
    }

    fn is_tty(&self) -> bool {
        true
    }

    fn device_call(&self, call: &abi::device::DeviceCall) -> SysResult<usize> {
        use abi::device::DeviceKind;
        use abi::termios::*;

        if call.kind != DeviceKind::Terminal {
            return Err(Errno::ENOSYS);
        }

        let termios_size = core::mem::size_of::<Termios>();
        let winsize_size = core::mem::size_of::<Winsize>();
        let pgid_size = core::mem::size_of::<u32>();

        match call.op {
            TERMINAL_OP_TCGETS => {
                if call.out_len < termios_size as u32 || call.out_ptr == 0 {
                    return Err(Errno::EINVAL);
                }
                let termios = *self.ld.termios.lock();
                unsafe {
                    crate::syscall::validate::copyout(call.out_ptr as usize, core::slice::from_raw_parts(&termios as *const _ as *const u8, termios_size))?;
                }
                Ok(0)
            }
            TERMINAL_OP_TCSETS | TERMINAL_OP_TCSETSW | TERMINAL_OP_TCSETSF => {
                if call.in_len < termios_size as u32 || call.in_ptr == 0 {
                    return Err(Errno::EINVAL);
                }
                let mut new_termios = Termios::default();
                unsafe {
                    crate::syscall::validate::copyin(core::slice::from_raw_parts_mut(&mut new_termios as *mut _ as *mut u8, termios_size), call.in_ptr as usize)?;
                }
                *self.ld.termios.lock() = new_termios;
                Ok(0)
            }
            TERMINAL_OP_TCGETPGRP => {
                if call.out_len < pgid_size as u32 || call.out_ptr == 0 {
                    return Err(Errno::EINVAL);
                }
                let fg_pgid = self.ld.presence.lock().foreground_pgid.ok_or(Errno::ENOTTY)?;
                unsafe {
                    crate::syscall::validate::copyout(call.out_ptr as usize, core::slice::from_raw_parts(&fg_pgid as *const u32 as *const u8, pgid_size))?;
                }
                Ok(0)
            }
            TERMINAL_OP_TCSETPGRP => {
                if call.in_len < pgid_size as u32 || call.in_ptr == 0 {
                    return Err(Errno::EINVAL);
                }
                let mut new_pgid = 0u32;
                unsafe {
                    crate::syscall::validate::copyin(core::slice::from_raw_parts_mut(&mut new_pgid as *mut u32 as *mut u8, pgid_size), call.in_ptr as usize)?;
                }
                self.ld.presence.lock().foreground_pgid = Some(new_pgid);
                Ok(0)
            }
            TERMINAL_OP_TIOCGWINSZ => {
                if call.out_len < winsize_size as u32 || call.out_ptr == 0 {
                    return Err(Errno::EINVAL);
                }
                let ws = self.hw.winsize();
                unsafe {
                    crate::syscall::validate::copyout(call.out_ptr as usize, core::slice::from_raw_parts(&ws as *const _ as *const u8, winsize_size))?;
                }
                Ok(0)
            }
            _ => Err(Errno::ENOSYS),
        }
    }
}
