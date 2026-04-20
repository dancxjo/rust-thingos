use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        let mut i = 0;
        while i < n {
            *dest.add(i) = *src.add(i);
            i += 1;
        }
        dest
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        if src < dest as *const u8 {
            let mut i = n;
            while i > 0 {
                i -= 1;
                *dest.add(i) = *src.add(i);
            }
        } else {
            let mut i = 0;
            while i < n {
                *dest.add(i) = *src.add(i);
                i += 1;
            }
        }
        dest
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(dest: *mut u8, c: i32, n: usize) -> *mut u8 {
    unsafe {
        let mut i = 0;
        while i < n {
            *dest.add(i) = c as u8;
            i += 1;
        }
        dest
    }
}

/// Maximum number of physical memory-map ranges cached from the Limine response.
const MAX_RANGES: usize = 64;

// Module-level statics so that atomic guards can be used for proper
// acquire/release ordering across CPUs.
static mut RANGES: [kernel::PhysRange; MAX_RANGES] =
    [kernel::PhysRange { start: 0, end: 0, kind: kernel::PhysRangeKind::Other }; MAX_RANGES];

/// Number of valid entries written into `RANGES`.
static RANGES_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Set to `true` (with `Release`) once `RANGES` and `RANGES_COUNT` are fully
/// written.  Readers load this with `Acquire` to guarantee they see the
/// completed writes.
static RANGES_INIT: AtomicBool = AtomicBool::new(false);

#[cfg(target_arch = "x86_64")]
fn early_serial_write(msg: &[u8]) {
    unsafe {
        let port = 0x3f8u16;
        for &b in msg {
            let mut spins = 0u32;
            loop {
                let lsr: u8;
                core::arch::asm!(
                    "in al, dx",
                    out("al") lsr,
                    in("dx") port + 5,
                    options(nostack, preserves_flags)
                );
                if (lsr & 0x20) != 0 || spins >= 100_000 {
                    break;
                }
                spins = spins.saturating_add(1);
                core::hint::spin_loop();
            }
            core::arch::asm!(
                "out dx, al",
                in("dx") port,
                in("al") b,
                options(nostack, preserves_flags)
            );
        }
    }
}

#[cfg(not(target_arch = "x86_64"))]
fn early_serial_write(_msg: &[u8]) {}

pub fn memory_map() -> &'static [kernel::PhysRange] {
    use crate::requests::MEMORY_MAP_REQUEST;
    use kernel::{PhysRange, PhysRangeKind};

    early_serial_write(b"[bran:mem] memory_map enter\r\n");

    // Fast path: already initialized.  The Acquire load synchronises with the
    // Release store below, so all writes to RANGES/RANGES_COUNT are visible.
    if RANGES_INIT.load(Ordering::Acquire) {
        early_serial_write(b"[bran:mem] cache hit\r\n");
        let count = RANGES_COUNT.load(Ordering::Relaxed);
        return unsafe { &RANGES[..count] };
    }

    early_serial_write(b"[bran:mem] before MEMORY_MAP_REQUEST response\r\n");
    if let Some(resp) = MEMORY_MAP_REQUEST.get_response() {
        early_serial_write(b"[bran:mem] got MEMORY_MAP_REQUEST response\r\n");
        let entries = resp.entries();
        let total = entries.len();
        early_serial_write(b"[bran:mem] entries read\r\n");

        if total > MAX_RANGES {
            early_serial_write(b"[bran:mem] truncating memory map to MAX_RANGES\r\n");
        }

        let count = total.min(MAX_RANGES);
        unsafe {
            early_serial_write(b"[bran:mem] begin cache fill\r\n");
            for (i, entry) in entries.into_iter().enumerate().take(count) {
                RANGES[i] = PhysRange {
                    start: entry.base,
                    end: entry.base + entry.length,
                    kind: match entry.entry_type {
                        limine::memory_map::EntryType::USABLE => PhysRangeKind::Usable,
                        limine::memory_map::EntryType::RESERVED => PhysRangeKind::Reserved,
                        limine::memory_map::EntryType::ACPI_RECLAIMABLE => PhysRangeKind::Acpi,
                        limine::memory_map::EntryType::BOOTLOADER_RECLAIMABLE => {
                            PhysRangeKind::Reserved
                        }
                        limine::memory_map::EntryType::FRAMEBUFFER => PhysRangeKind::Framebuffer,
                        _ => PhysRangeKind::Other,
                    },
                };
            }
            early_serial_write(b"[bran:mem] cache fill done\r\n");
        }

        // Publish count first (Relaxed is fine here — the Release on
        // RANGES_INIT below provides the ordering guarantee).
        RANGES_COUNT.store(count, Ordering::Relaxed);
        // Release: ensures all writes to RANGES and RANGES_COUNT happen-before
        // any Acquire load of RANGES_INIT in another CPU.
        RANGES_INIT.store(true, Ordering::Release);
        early_serial_write(b"[bran:mem] publish done\r\n");

        unsafe { &RANGES[..count] }
    } else {
        early_serial_write(b"[bran:mem] no MEMORY_MAP_REQUEST response\r\n");
        &[]
    }
}
