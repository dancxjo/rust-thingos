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
static mut RANGES: [kernel::PhysRange; MAX_RANGES] = [kernel::PhysRange {
    start: 0,
    end: 0,
    kind: kernel::PhysRangeKind::Other,
}; MAX_RANGES];

/// Number of valid entries written into `RANGES`.
static RANGES_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Set to `true` (with `Release`) once `RANGES` and `RANGES_COUNT` are fully
/// written.  Readers load this with `Acquire` to guarantee they see the
/// completed writes.
static RANGES_INIT: AtomicBool = AtomicBool::new(false);

pub fn memory_map() -> &'static [kernel::PhysRange] {
    use crate::requests::MEMORY_MAP_REQUEST;
    use kernel::{PhysRange, PhysRangeKind};

    // Fast path: already initialized.  The Acquire load synchronises with the
    // Release store below, so all writes to RANGES/RANGES_COUNT are visible.
    if RANGES_INIT.load(Ordering::Acquire) {
        let count = RANGES_COUNT.load(Ordering::Relaxed);
        return unsafe { &RANGES[..count] };
    }

    if let Some(resp) = MEMORY_MAP_REQUEST.get_response() {
        let entries = resp.entries();
        let total = entries.len();

        if total > MAX_RANGES {
            kernel::kwarn!(
                "memory_map: Limine reports {} entries; only first {} fit in cache (rest dropped)",
                total,
                MAX_RANGES
            );
        }

        let count = total.min(MAX_RANGES);
        unsafe {
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
        }

        // Publish count first (Relaxed is fine here — the Release on
        // RANGES_INIT below provides the ordering guarantee).
        RANGES_COUNT.store(count, Ordering::Relaxed);
        // Release: ensures all writes to RANGES and RANGES_COUNT happen-before
        // any Acquire load of RANGES_INIT in another CPU.
        RANGES_INIT.store(true, Ordering::Release);

        unsafe { &RANGES[..count] }
    } else {
        &[]
    }
}
