#[cfg(not(test))]
use core::alloc::{GlobalAlloc, Layout};
#[cfg(not(test))]
use core::sync::atomic::{AtomicU64, Ordering};

#[cfg(not(test))]
use linked_list_allocator::LockedHeap;

use crate::BootRuntime;
#[cfg(not(test))]
use crate::memory::kheap::kernel_heap;

pub static TRACE_ALLOC: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);

/// Track the largest allocation seen
#[cfg(not(test))]
static LARGEST_ALLOC: AtomicU64 = AtomicU64::new(0);

/// Count of allocations over 1MB
#[cfg(not(test))]
static LARGE_ALLOC_COUNT: AtomicU64 = AtomicU64::new(0);

/// The inner heap allocator
#[cfg(not(test))]
static INNER_ALLOCATOR: LockedHeap = LockedHeap::empty();

#[cfg(not(test))]
const BOOTSTRAP_HEAP_PAGES: usize = 4096; // 16 MiB
#[cfg(not(test))]
const GROWTH_HEAP_PAGES: usize = 1024; // 4 MiB

#[cfg(not(test))]
static HEAP_TOP: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);
#[cfg(not(test))]
static mut HEAP_EXPAND_HOOK: Option<fn(usize) -> Result<(), ()>> = None;

/// Wrapper allocator that logs large allocations
#[cfg(not(test))]
struct TracingAllocator;

#[cfg(not(test))]
unsafe impl GlobalAlloc for TracingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            let orig_size = layout.size();
            let orig_align = layout.align();

            // Workaround for linked_list_allocator bug with small leftover holes:
            // By aligning the size to 32 bytes and ensuring a minimum of 32 bytes,
            // we guarantee that any leftover block is at least 32 bytes long
            // (which is > maximum alignment padding + Hole size), preventing it from
            // creating < 24 byte holes that corrupt the free list.
            //
            // Layout::from_size_align also requires that size is a multiple of align.
            let align = orig_align.max(32);
            let mask = align - 1;
            let size = (orig_size + mask) & !mask;

            let safe_layout = Layout::from_size_align_unchecked(size, align);

            let irq = crate::irq::irq_disable_erased();
            let mut ptr = unsafe { INNER_ALLOCATOR.alloc(safe_layout) };
            crate::irq::irq_restore_erased(irq);

            if ptr.is_null() {
                let needed_pages = (size + 4095) / 4096;
                let grow_pages = core::cmp::max(GROWTH_HEAP_PAGES, needed_pages);
                let expanded = unsafe {
                    if let Some(hook) = HEAP_EXPAND_HOOK { hook(grow_pages).is_ok() } else { false }
                };
                if expanded {
                    let irq = crate::irq::irq_disable_erased();
                    ptr = unsafe { INNER_ALLOCATOR.alloc(safe_layout) };
                    crate::irq::irq_restore_erased(irq);
                }
            }

            ptr
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            let orig_size = layout.size();
            let orig_align = layout.align();

            let align = orig_align.max(32);
            let mask = align - 1;
            let size = (orig_size + mask) & !mask;

            let safe_layout = Layout::from_size_align_unchecked(size, align);

            let irq = crate::irq::irq_disable_erased();
            unsafe { INNER_ALLOCATOR.dealloc(ptr, safe_layout) }
            crate::irq::irq_restore_erased(irq);
        }
    }
}

#[cfg(not(test))]
#[global_allocator]
static ALLOCATOR: TracingAllocator = TracingAllocator;

#[cfg(not(test))]
pub fn init<R: BootRuntime>(_rt: &R) {
    unsafe {
        HEAP_EXPAND_HOOK = Some(expand_heap_impl::<R>);
    }
    let mut heap = kernel_heap().lock();
    // Keep early boot fast: bootstrap with a smaller heap and grow on demand.
    let (base, size) = heap
        .reserve_region::<R>(BOOTSTRAP_HEAP_PAGES)
        .expect("Failed to reserve kernel heap region");

    unsafe {
        INNER_ALLOCATOR.lock().init(base as *mut u8, size);
    }
    HEAP_TOP.store(base + size as u64, Ordering::Relaxed);
    crate::ktrace!(
        "Bootstrap heap initialized: base=0x{:x} size={} pages={}",
        base,
        size,
        BOOTSTRAP_HEAP_PAGES
    );
}

#[cfg(not(test))]
fn expand_heap_impl<R: BootRuntime>(pages: usize) -> Result<(), ()> {
    let mut heap = kernel_heap().lock();
    let expected_base = HEAP_TOP.load(Ordering::Relaxed);
    let (base, size) = heap.reserve_region::<R>(pages)?;

    if base != expected_base {
        // linked_list_allocator::Heap::extend requires strictly contiguous memory.
        return Err(());
    }

    unsafe {
        INNER_ALLOCATOR.lock().extend(size);
    }
    HEAP_TOP.store(base + size as u64, Ordering::Relaxed);
    Ok(())
}

/// Get diagnostics about large allocations
#[cfg(not(test))]
pub fn alloc_stats() -> (u64, u64) {
    (LARGEST_ALLOC.load(Ordering::Relaxed), LARGE_ALLOC_COUNT.load(Ordering::Relaxed))
}

#[cfg(test)]
pub fn init<R: BootRuntime>(_rt: &R) {
    // In tests, we use the system allocator (std), so no manual init needed.
}

#[cfg(test)]
pub fn alloc_stats() -> (u64, u64) {
    (0, 0)
}
