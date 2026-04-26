use core::arch::asm;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use kernel::time::MonotonicClamp;

/// 16550 UART TX FIFO depth (standard).
const UART_FIFO_DEPTH: usize = 16;

/// COM1 base I/O port.
const COM1: u16 = 0x3F8;
/// COM2 base I/O port.
const COM2: u16 = 0x2F8;

/// Serial port implementation for x86_64 using I/O port 0x3F8 (COM1).
pub struct SerialPort {
    pub(crate) clamp: MonotonicClamp,
    freq_hz: AtomicU64,
    /// Whether TX interrupts are currently armed on COM1.
    tx_irq_armed: AtomicBool,
}

impl SerialPort {
    pub const fn new() -> Self {
        Self {
            clamp: MonotonicClamp::new(),
            freq_hz: AtomicU64::new(0),
            tx_irq_armed: AtomicBool::new(false),
        }
    }

    // -----------------------------------------------------------------------
    // Non-blocking / FIFO-aware helpers
    // -----------------------------------------------------------------------

    /// Returns `true` if the TX Holding Register is empty (THRE bit in LSR).
    /// This means the FIFO is fully drained and up to FIFO_DEPTH bytes can
    /// be written without blocking.
    #[inline]
    pub fn uart_tx_ready() -> bool {
        unsafe { (inb(COM1 + 5) & 0x20) != 0 }
    }

    /// Write up to `UART_FIFO_DEPTH` bytes into the TX FIFO without any
    /// busy-wait.  Returns the number of bytes actually written.
    ///
    /// Caller must ensure THRE is set (via `uart_tx_ready()`) before calling.
    /// After the burst, the FIFO will drain at wire speed; subsequent writes
    /// should wait for THRE again.
    pub fn write_fifo_burst(data: &[u8]) -> usize {
        let n = data.len().min(UART_FIFO_DEPTH);
        for &b in &data[..n] {
            unsafe { outb(COM1, b) };
        }
        n
    }

    /// Arm the TX Holding Register Empty (THRE) interrupt on COM1.
    ///
    /// When armed, the UART fires IRQ4 each time the TX FIFO drains below
    /// the trigger level, allowing us to refill it from the deferred ring
    /// without polling.
    pub fn arm_tx_interrupt(&self) {
        if self.tx_irq_armed.load(Ordering::Relaxed) {
            return;
        }
        unsafe {
            // IER: bit 0 = Received Data Available, bit 1 = THRE
            let ier = inb(COM1 + 1);
            outb(COM1 + 1, ier | 0x02);
        }
        self.tx_irq_armed.store(true, Ordering::Relaxed);
    }

    /// Disarm the THRE interrupt on COM1.
    ///
    /// Called when the ring buffer is empty so we don't generate spurious
    /// interrupts.
    pub fn disarm_tx_interrupt(&self) {
        if !self.tx_irq_armed.load(Ordering::Relaxed) {
            return;
        }
        unsafe {
            let ier = inb(COM1 + 1);
            outb(COM1 + 1, ier & !0x02);
        }
        self.tx_irq_armed.store(false, Ordering::Relaxed);
    }

    /// Returns `true` if the THRE interrupt is currently armed.
    #[inline]
    pub fn is_tx_irq_armed(&self) -> bool {
        self.tx_irq_armed.load(Ordering::Relaxed)
    }

    /// Synchronous single-byte write.  Spin-waits for TX ready.
    ///
    /// This is the **panic/sync fallback** path.  Normal logging should go
    /// through the deferred ring buffer instead.
    pub fn putchar(&self, c: u8) {
        unsafe {
            // Wait for COM1 transmit empty
            while (inb(COM1 + 5) & 0x20) == 0 {}
            outb(COM1, c);

            // Mirror to COM2 if it exists (check if Scratch Register sticks)
            outb(COM2 + 7, 0xAE);
            if inb(COM2 + 7) == 0xAE {
                while (inb(COM2 + 5) & 0x20) == 0 {}
                outb(COM2, c);
            }
        }
    }

    /// Read from COM1 or COM2. Returns `Some(byte)` if data ready on either.
    pub fn getchar(&self) -> Option<u8> {
        unsafe {
            // Check COM1 (Data Ready)
            if (inb(COM1 + 5) & 0x01) != 0 {
                return Some(inb(COM1));
            }

            // Check COM2 (Data Ready)
            if (inb(COM2 + 5) & 0x01) != 0 {
                return Some(inb(COM2));
            }

            None
        }
    }

    /// Calibrate TSC using the PIT (Programmable Interval Timer).
    /// This is a simplified calibration that runs once on the first call to mono_freq_hz.
    pub fn calibrate(&self) -> u64 {
        // Check if we already calibrated
        let cached = self.freq_hz.load(Ordering::Relaxed);
        if cached != 0 {
            return cached;
        }

        // basic calibration using PIT port 2 (speaker) or port 0 (system timer).
        let freq = unsafe { calibrate_tsc_pit() };
        self.freq_hz.store(freq, Ordering::Relaxed);
        freq
    }
}

#[inline]
unsafe fn outb(port: u16, val: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack, preserves_flags));
    }
}

#[inline]
unsafe fn inb(port: u16) -> u8 {
    let ret: u8;
    unsafe {
        asm!("in al, dx", out("al") ret, in("dx") port, options(nomem, nostack, preserves_flags));
    }
    ret
}

#[inline]
pub(crate) unsafe fn rdtsc() -> u64 {
    let low: u32;
    let high: u32;
    unsafe {
        asm!("rdtsc", out("eax") low, out("edx") high, options(nomem, nostack));
    }
    ((high as u64) << 32) | (low as u64)
}

/// Calibrate TSC against PIT.
/// Returns frequency in Hz, or 0 if failed.
unsafe fn calibrate_tsc_pit() -> u64 {
    // 1. Setup PIT Channel 2 (0x42) to one-shot mode (Mode 0)
    // We want to wait for a known duration.
    // The PIT runs at 1.193182 MHz.
    // Let's measure for ~10ms.
    // 10ms = 11932 ticks.
    
    // Control Word: Channel 2, Access Lo/Hi, Mode 0 (Interrupt on Terminal Count), Binary
    // 0b10_11_000_0 = 0xB0
    unsafe { outb(0x43, 0xB0) };
    
    // Reload value = 11932 (0x2E9C) for ~10ms
    let count = 11932u16;
    unsafe {
        outb(0x42, (count & 0xFF) as u8);
        outb(0x42, (count >> 8) as u8);
    }
    
    // Enable Channel 2 Gate (bit 0 of Port 0x61)
    let port61 = unsafe { inb(0x61) };
    unsafe { outb(0x61, port61 | 0x01) };
    
    let start_tsc = unsafe { rdtsc() };
    
    // Spin until bit 5 of 0x61 becomes 1.
    let mut timeout = 100_000_000;
    while (unsafe { inb(0x61) } & 0x20) == 0 {
        core::hint::spin_loop();
        timeout -= 1;
        if timeout == 0 {
            return 0; // Failed
        }
    }
    
    let end_tsc = unsafe { rdtsc() };
    
    // Disable Gate just in case
    unsafe { outb(0x61, port61 & !0x01) };
    
    let delta = end_tsc.saturating_sub(start_tsc);
    
    // freq = delta * 100
    delta.saturating_mul(100)
}
