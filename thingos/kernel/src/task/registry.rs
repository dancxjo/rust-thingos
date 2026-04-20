use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use spin::Mutex;

use super::Thread;
use crate::BootRuntime;

/// Registry of all live kernel threads for a given runtime.
pub struct ThreadRegistry<R: BootRuntime> {
    pub threads: Vec<Box<Thread<R>>>,
    thread_index_by_id: BTreeMap<u64, usize>,
}
/// Backward-compatible alias — prefer `ThreadRegistry` in new code.
pub type TaskRegistry<R> = ThreadRegistry<R>;

impl<R: BootRuntime> ThreadRegistry<R> {
    pub fn new() -> Self {
        Self { threads: Vec::with_capacity(1024), thread_index_by_id: BTreeMap::new() }
    }

    pub fn insert(&mut self, thread: Box<Thread<R>>) {
        let id = thread.id;
        if self.thread_index_by_id.contains_key(&id) {
            panic!("Thread ID {} already exists in registry", id);
        }
        let idx = self.threads.len();
        self.threads.push(thread);
        self.thread_index_by_id.insert(id, idx);
    }

    /// Return the current backing-vector slot for `id`.
    ///
    /// The returned index is valid only until the next `insert` or `remove`
    /// call, because `swap_remove` can relocate the last live thread and
    /// `push` can change vector bounds assumptions.
    #[inline]
    pub fn get_index(&self, id: u64) -> Option<usize> {
        self.thread_index_by_id.get(&id).copied()
    }

    pub fn get(&self, id: u64) -> Option<&Thread<R>> {
        let idx = self.get_index(id)?;
        let thread = self.threads.get(idx)?;
        debug_assert_eq!(thread.id, id);
        Some(&**thread)
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut Thread<R>> {
        let idx = self.get_index(id)?;
        let thread = self.threads.get_mut(idx)?;
        debug_assert_eq!(thread.id, id);
        Some(&mut **thread)
    }

    pub fn remove(&mut self, id: u64) -> Option<Box<Thread<R>>> {
        let idx = self.thread_index_by_id.remove(&id)?;
        let removed = self.threads.swap_remove(idx);
        debug_assert_eq!(removed.id, id);
        if idx < self.threads.len() {
            // If the removed thread was not the last element, `swap_remove`
            // moved the former tail thread into `idx`; refresh its map entry.
            let moved_id = self.threads[idx].id;
            self.thread_index_by_id.insert(moved_id, idx);
        }
        Some(removed)
    }
}

pub static REGISTRY: Mutex<Option<usize>> = Mutex::new(None);

pub fn init<R: BootRuntime>() {
    let registry = Box::new(ThreadRegistry::<R>::new());
    *REGISTRY.lock() = Some(Box::into_raw(registry) as usize);
}

pub struct RegistryGuard<R: BootRuntime> {
    guard: Option<spin::MutexGuard<'static, Option<usize>>>,
    irq_state: crate::IrqState,
    _marker: core::marker::PhantomData<R>,
}

impl<R: BootRuntime> Drop for RegistryGuard<R> {
    fn drop(&mut self) {
        // Drop the lock before restoring interrupts.
        self.guard.take();
        unsafe {
            crate::irq::irq_restore_erased(self.irq_state);
        }
    }
}

impl<R: BootRuntime> core::ops::Deref for RegistryGuard<R> {
    type Target = ThreadRegistry<R>;
    fn deref(&self) -> &Self::Target {
        let ptr = self.guard.as_ref().unwrap().expect("ThreadRegistry not initialized");
        unsafe { &*(ptr as *const ThreadRegistry<R>) }
    }
}

impl<R: BootRuntime> core::ops::DerefMut for RegistryGuard<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let ptr = self.guard.as_mut().unwrap().expect("ThreadRegistry not initialized");
        unsafe { &mut *(ptr as *mut ThreadRegistry<R>) }
    }
}

pub fn get_registry<R: BootRuntime>() -> RegistryGuard<R> {
    let irq_state = unsafe { crate::irq::irq_disable_erased() };
    RegistryGuard { guard: Some(REGISTRY.lock()), irq_state, _marker: core::marker::PhantomData }
}

pub struct ThreadRef<R: BootRuntime> {
    guard: RegistryGuard<R>,
    idx: usize,
}
/// Backward-compatible alias — prefer `ThreadRef` in new code.
pub type TaskRef<R> = ThreadRef<R>;

impl<R: BootRuntime> core::ops::Deref for ThreadRef<R> {
    type Target = Thread<R>;
    fn deref(&self) -> &Self::Target {
        &self.guard.threads[self.idx]
    }
}

pub struct ThreadMut<R: BootRuntime> {
    guard: RegistryGuard<R>,
    idx: usize,
}
/// Backward-compatible alias — prefer `ThreadMut` in new code.
pub type TaskMut<R> = ThreadMut<R>;

impl<R: BootRuntime> core::ops::Deref for ThreadMut<R> {
    type Target = Thread<R>;
    fn deref(&self) -> &Self::Target {
        &self.guard.threads[self.idx]
    }
}

impl<R: BootRuntime> core::ops::DerefMut for ThreadMut<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard.threads[self.idx]
    }
}

/// Look up a thread by its `ThreadId`.  Returns `None` if not found.
pub fn get_thread<R: BootRuntime>(id: u64) -> Option<ThreadRef<R>> {
    let guard = get_registry::<R>();
    let idx = guard.get_index(id)?;
    Some(ThreadRef { guard, idx })
}

/// Look up a thread mutably by its `ThreadId`.  Returns `None` if not found.
pub fn get_thread_mut<R: BootRuntime>(id: u64) -> Option<ThreadMut<R>> {
    let guard = get_registry::<R>();
    let idx = guard.get_index(id)?;
    Some(ThreadMut { guard, idx })
}

// ── Backward-compatible forwarding functions ──────────────────────────────────

/// Backward-compatible alias for `get_thread`.
#[inline]
pub fn get_task<R: BootRuntime>(id: u64) -> Option<ThreadRef<R>> {
    get_thread::<R>(id)
}

/// Backward-compatible alias for `get_thread_mut`.
#[inline]
pub fn get_task_mut<R: BootRuntime>(id: u64) -> Option<ThreadMut<R>> {
    get_thread_mut::<R>(id)
}
