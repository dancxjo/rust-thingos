use core::arch::asm;

// Legacy SBI Console Putchar Extension ID
const SBI_EID_CONSOLE_PUTCHAR: usize = 1;
// Legacy SBI Console Getchar Extension ID
const SBI_EID_CONSOLE_GETCHAR: usize = 2;

pub struct SerialPort;

impl SerialPort {
    pub const fn new() -> Self {
        Self
    }

    pub fn putchar(&self, c: u8) {
        putchar(c);
    }

    pub fn putbuf(&self, buf: &[u8]) {
        putbuf(buf);
    }

    /// Non-blocking read via SBI Legacy Console Getchar.
    /// Returns `Some(byte)` if a character is available, `None` otherwise.
    pub fn getchar(&self) -> Option<u8> {
        getchar()
    }
}

pub fn early_serial_write(buf: &[u8]) {
    putbuf(buf);
}

pub fn putbuf(buf: &[u8]) {
    for &b in buf {
        putchar(b);
    }
}

pub fn putchar(c: u8) {
    unsafe {
        // Legacy SBI: EID=1, a0=char
        asm!(
            "ecall",
            in("a7") SBI_EID_CONSOLE_PUTCHAR,
            in("a0") c as usize,
            options(nostack, preserves_flags)
        );
    }
}

/// Non-blocking read via SBI Legacy Console Getchar.
/// Returns `Some(byte)` if a character is available, `None` otherwise.
pub fn getchar() -> Option<u8> {
    let ret: isize;
    unsafe {
        asm!(
            "ecall",
            in("a7") SBI_EID_CONSOLE_GETCHAR,
            lateout("a0") ret,
            options(nostack, preserves_flags)
        );
    }
    if ret >= 0 { Some(ret as u8) } else { None }
}
