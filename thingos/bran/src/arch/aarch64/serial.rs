use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use kernel::time::MonotonicClamp;

const PL011_UART0_PHYS: u64 = 0x0900_0000;
const UARTDR: usize = 0;
const UARTFR: usize = 6;
const UARTFR_RXFE: u32 = 1 << 4;
const UART_FIFO_DEPTH: usize = 16;
const SEMIHOSTING_SYS_WRITEC: u32 = 0x03;

static UART_BASE: AtomicU64 = AtomicU64::new(0);
static UART_AVAILABLE: AtomicBool = AtomicBool::new(false);

pub struct SerialPort {
    pub clamp: MonotonicClamp,
}

impl SerialPort {
    pub const fn new() -> Self {
        Self { clamp: MonotonicClamp::new() }
    }

    pub fn init(&self, hhdm_offset: u64) {
        init(hhdm_offset);
    }

    pub fn putchar(&self, c: u8) {
        putchar(c);
    }

    pub fn putbuf(&self, buf: &[u8]) {
        putbuf(buf);
    }

    pub fn getchar(&self) -> Option<u8> {
        getchar()
    }
}

pub fn init(hhdm_offset: u64) {
    if hhdm_offset != 0 {
        UART_BASE.store(PL011_UART0_PHYS + hhdm_offset, Ordering::Release);
        UART_AVAILABLE.store(true, Ordering::Release);
    }
}

pub fn early_serial_write(buf: &[u8]) {
    semihosting_write(buf);
}

pub fn semihosting_write(buf: &[u8]) {
    for &b in buf {
        semihosting_putchar(b);
    }
}

fn semihosting_putchar(c: u8) {
    let ch = c;
    unsafe {
        core::arch::asm!(
            "hlt #0xF000",
            in("w0") SEMIHOSTING_SYS_WRITEC,
            in("x1") &ch,
            options(nostack, preserves_flags)
        );
    }
}

pub fn putchar(c: u8) {
    semihosting_putchar(c);
}

pub fn putbuf(buf: &[u8]) {
    semihosting_write(buf);
}

pub fn getchar() -> Option<u8> {
    let uart = uart()?;
    unsafe {
        if (core::ptr::read_volatile(uart.add(UARTFR)) & UARTFR_RXFE) != 0 {
            return None;
        }
        Some((core::ptr::read_volatile(uart.add(UARTDR)) & 0xff) as u8)
    }
}

pub fn tx_ready() -> bool {
    true
}

pub fn write_fifo_burst(data: &[u8]) -> usize {
    let n = data.len().min(UART_FIFO_DEPTH);
    putbuf(&data[..n]);
    n
}

fn uart() -> Option<*mut u32> {
    if !UART_AVAILABLE.load(Ordering::Acquire) {
        return None;
    }
    let base = UART_BASE.load(Ordering::Acquire);
    if base == 0 { None } else { Some(base as *mut u32) }
}
