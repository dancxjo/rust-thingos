use alloc::{boxed::Box, collections::BTreeMap};
use core::ffi::{c_int, c_void};

use abi::errors::Errno;
use spin::Mutex;

use crate::{
    stack::{Stack, StackSpec},
    syscall,
};

#[allow(non_camel_case_types)]
pub type pthread_t = u64;

#[allow(non_camel_case_types)]
pub type pthread_attr_t = c_void;

type StartRoutine = extern "C" fn(*mut c_void) -> *mut c_void;

struct StartContext {
    start_routine: StartRoutine,
    arg: *mut c_void,
}

struct ThreadRecord {
    _stack: Stack,
    retval: *mut c_void,
    join_in_progress: bool,
}

// SAFETY: Access to thread records is synchronized by THREADS.
unsafe impl Send for ThreadRecord {}

static THREADS: Mutex<BTreeMap<pthread_t, ThreadRecord>> = Mutex::new(BTreeMap::new());

#[inline]
const fn errno_code(errno: Errno) -> c_int {
    errno as c_int
}

extern "C" fn pthread_start_trampoline(arg: usize) -> ! {
    let start = unsafe { Box::from_raw(arg as *mut StartContext) };
    let retval = (start.start_routine)(start.arg);
    pthread_exit(retval)
}

#[no_mangle]
pub unsafe extern "C" fn pthread_create(
    thread: *mut pthread_t,
    _attr: *const pthread_attr_t,
    start_routine: StartRoutine,
    arg: *mut c_void,
) -> c_int {
    if thread.is_null() {
        return errno_code(Errno::EINVAL);
    }

    let stack = match Stack::alloc_growing_stack(StackSpec::default()) {
        Ok(stack) => stack,
        Err(e) => return errno_code(e),
    };

    let start_ctx = Box::new(StartContext { start_routine, arg });
    let start_ctx_ptr = Box::into_raw(start_ctx) as usize;

    let tid = match syscall::spawn_thread(
        pthread_start_trampoline as *const () as usize,
        start_ctx_ptr,
        &stack,
    ) {
        Ok(tid) => tid,
        Err(e) => {
            drop(unsafe { Box::from_raw(start_ctx_ptr as *mut StartContext) });
            return errno_code(e);
        }
    };

    THREADS.lock().insert(
        tid,
        ThreadRecord {
            _stack: stack,
            retval: core::ptr::null_mut(),
            join_in_progress: false,
        },
    );

    unsafe {
        *thread = tid;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int {
    let self_tid = match syscall::get_tid() {
        Ok(tid) => tid,
        Err(e) => return errno_code(e),
    };

    if self_tid == thread {
        return errno_code(Errno::EINVAL);
    }

    {
        let mut threads = THREADS.lock();
        let Some(record) = threads.get_mut(&thread) else {
            return errno_code(Errno::ESRCH);
        };
        if record.join_in_progress {
            return errno_code(Errno::EINVAL);
        }
        record.join_in_progress = true;
    }

    if let Err(e) = syscall::task_wait(thread) {
        if let Some(record) = THREADS.lock().get_mut(&thread) {
            record.join_in_progress = false;
        }
        return errno_code(e);
    }

    let Some(record) = THREADS.lock().remove(&thread) else {
        return errno_code(Errno::ESRCH);
    };

    if !retval.is_null() {
        unsafe {
            *retval = record.retval;
        }
    }

    0
}

#[no_mangle]
pub extern "C" fn pthread_exit(retval: *mut c_void) -> ! {
    if let Ok(tid) = syscall::get_tid() {
        if let Some(record) = THREADS.lock().get_mut(&tid) {
            record.retval = retval;
        }
    }
    syscall::exit(0)
}

#[no_mangle]
pub extern "C" fn pthread_self() -> pthread_t {
    syscall::get_tid().unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn pthread_equal(t1: pthread_t, t2: pthread_t) -> c_int {
    (t1 == t2) as c_int
}
