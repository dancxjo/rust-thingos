use alloc::vec;
use core::sync::atomic::{AtomicPtr, Ordering};

use abi::errors::{Errno, SysResult};
use abi::trace::TraceEvent;

use crate::syscall::validate::{copyout, validate_user_range};
use crate::trace::irq_ring;

pub fn sys_trace_read(ptr: usize, len: usize) -> SysResult<usize> {
    if len > 8192 {
        // Sanity cap
        return Err(Errno::EINVAL);
    }

    // Validate output buffer
    let size_struct = core::mem::size_of::<TraceEvent>();
    let total_bytes = len.checked_mul(size_struct).ok_or(Errno::EINVAL)?;
    validate_user_range(ptr, total_bytes, true)?;

    let mut temp = vec![TraceEvent::Empty; len];

    let count = {
        let ring = irq_ring::IRQ_RING.lock();
        ring.read_all(&mut temp)
    };

    // now copyout
    let bytes = count * size_struct;
    unsafe {
        let src = core::slice::from_raw_parts(temp.as_ptr() as *const u8, bytes);
        copyout(ptr, src)?;
    }

    Ok(count)
}

pub fn sys_trace_mark_input(
    source: usize,
    bytes: usize,
    events: usize,
    drops: usize,
) -> SysResult<usize> {
    irq_ring::push(TraceEvent::InputSummary {
        source: source.min(u8::MAX as usize) as u8,
        events: events.min(u32::MAX as usize) as u32,
        bytes: bytes.min(u32::MAX as usize) as u32,
        drops: drops.min(u32::MAX as usize) as u32,
        timestamp: crate::trace::now_or_zero(),
    });
    Ok(0)
}

/// Registered boot console disable function (set by bran at init)
static CONSOLE_DISABLE_FN: AtomicPtr<()> = AtomicPtr::new(core::ptr::null_mut());

/// Register a console disable callback (called by bran at boot)
pub fn register_console_disable(f: fn()) {
    CONSOLE_DISABLE_FN.store(f as *mut (), Ordering::Release);
}

/// Disable the boot console (compositor takes over framebuffer)
pub fn sys_console_disable() -> SysResult<usize> {
    let ptr = CONSOLE_DISABLE_FN.load(Ordering::Acquire);
    if !ptr.is_null() {
        let f: fn() = unsafe { core::mem::transmute(ptr) };
        f();
        crate::kinfo!("Boot framebuffer console disabled");
    }
    Ok(0)
}
