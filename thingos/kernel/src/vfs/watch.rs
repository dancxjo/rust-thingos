//! Kernel-native VFS watch system.
//!
//! Implements Act I of the watch migration: replacing Root-service watches
//! with a first-class kernel mechanism integrated into the VFS.
//!
//! # Thread safety
//!
//! [`EventQueue`] is safe for concurrent use.  The event ring is protected by
//! an internal `Mutex` and the waiter list is a [`WaitQueue`].  Events are
//! pushed under the ring mutex, which is released before
//! [`WaitQueue::wake_all`] is called so that woken readers can immediately
//! acquire the ring mutex without contending with the producer.
//!
//! See `docs/kernel/threading-readiness.md` for how this supports expanded
//! userspace multithreading.

use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::sync::Arc;
use core::sync::atomic::{AtomicU64, Ordering};

use abi::errors::{Errno, SysResult};
use abi::vfs_watch::{self, WatchEvent};
use spin::Mutex;

use super::{VfsNode, VfsStat};
use crate::sched::wait_queue::WaitQueue;

/// A ring buffer of VFS events.
pub struct EventQueue {
    /// Queue of (header, optional_name) pairs.
    events: Mutex<VecDeque<(WatchEvent, Option<String>)>>,
    max_size: usize,
    /// Tasks waiting for events.
    ///
    /// Using [`WaitQueue`] instead of a plain `Mutex<Vec<u64>>` gives O(log n)
    /// insert/remove and avoids cloning the full waiter list on every event.
    waiters: WaitQueue,
}

impl EventQueue {
    pub fn new(max_size: usize) -> Self {
        Self { events: Mutex::new(VecDeque::new()), max_size, waiters: WaitQueue::new() }
    }

    pub fn push(&self, mut event: WatchEvent, name: Option<&str>) {
        {
            let mut lock = self.events.lock();
            if lock.len() >= self.max_size {
                // Check if last event was an overflow to avoid flooding.
                // In that case the queue state does not change, so there is
                // nothing new to wake waiters for.
                if let Some((last, _)) = lock.back() {
                    if last.mask & vfs_watch::mask::OVERFLOW != 0 {
                        return;
                    }
                }
                // Queue one overflow marker and then wake readers after all
                // queue locks have been released.
                event.mask = vfs_watch::mask::OVERFLOW;
                lock.push_back((event, None));
            } else {
                lock.push_back((event, name.map(|s| s.into())));
            }
        }

        // Wake waiters after dropping the queue lock so a woken task can read
        // immediately without inverting the queue/registry lock order.
        self.waiters.wake_all();
    }

    pub fn pop(&self) -> Option<(WatchEvent, Option<String>)> {
        self.events.lock().pop_front()
    }

    /// Re-insert an event at the **front** of the queue.
    ///
    /// Used by [`Watch::read`] to preserve an event when the caller's buffer
    /// is too small to hold it.  Unlike [`push`], this bypasses the overflow
    /// check because the event was already validated and queued once.
    pub fn push_front(&self, event: WatchEvent, name: Option<String>) {
        self.events.lock().push_front((event, name));
    }

    pub fn is_empty(&self) -> bool {
        self.events.lock().is_empty()
    }

    pub fn add_waiter(&self, tid: u64) {
        self.waiters.push_back(tid);
    }

    pub fn remove_waiter(&self, tid: u64) {
        self.waiters.remove(tid);
    }
}

/// A kernel watch object.
///
/// Implements [`VfsNode`] so it can be used as a file descriptor.
pub struct Watch {
    pub queue: Arc<EventQueue>,
    pub mask: u32,
    pub flags: u32,
    /// Stable registration ID allocated at construction time and used as the
    /// [`WatchRecord::subject_id`] when this watch is registered.  Also used
    /// by [`VfsNode::close`] to unregister the watch from the global registry
    /// and prevent the registry from growing without bound.
    registration_id: u64,
}

impl Watch {
    pub fn new(mask: u32, flags: u32) -> Self {
        let registration_id = NEXT_WATCH_ID.fetch_add(1, Ordering::Relaxed);
        Self { queue: Arc::new(EventQueue::new(1024)), mask, flags, registration_id }
    }
}

impl VfsNode for Watch {
    fn read(&self, _offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let (event, name) = match self.queue.pop() {
            Some(e) => e,
            None => {
                if self.flags & vfs_watch::flags::NONBLOCK != 0 {
                    return Err(Errno::EAGAIN);
                }
                return Err(Errno::EAGAIN); // Should block in a real impl, but wait_many handles it
            }
        };

        let mut event = event;
        let name_bytes = name.as_ref().map(|s| s.as_bytes()).unwrap_or(&[]);
        event.name_len = name_bytes.len() as u16;

        let header_size = core::mem::size_of::<WatchEvent>();
        let total_size = header_size + name_bytes.len();

        if buf.len() < total_size {
            // Re-queue the event so it is not lost, then return EINVAL.
            // Inotify semantics: the event is preserved; the caller must
            // provide a buffer large enough to hold at least one event.
            self.queue.push_front(event, name);
            return Err(Errno::EINVAL);
        }

        // Copy header
        unsafe {
            let ptr = &event as *const _ as *const u8;
            buf[..header_size].copy_from_slice(core::slice::from_raw_parts(ptr, header_size));
        }

        // Copy name
        if !name_bytes.is_empty() {
            buf[header_size..total_size].copy_from_slice(name_bytes);
        }

        Ok(total_size)
    }

    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EBADF)
    }

    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat { mode: VfsStat::S_IFCHR | 0o666, size: 0, ino: 0, ..Default::default() })
    }

    fn poll(&self) -> u16 {
        use abi::syscall::poll_flags::*;
        if self.queue.is_empty() { 0 } else { POLLIN }
    }

    fn add_waiter(&self, tid: u64) {
        self.queue.add_waiter(tid);
    }

    fn remove_waiter(&self, tid: u64) {
        self.queue.remove_waiter(tid);
    }

    fn close(&self) {
        // Remove this watch from the global registry so that:
        // 1. The registry cannot grow without bound (leak fix).
        // 2. Subsequent emit_event calls do not fan out to closed watches.
        unregister_watch(self.registration_id);
    }
}

// ── Registry ────────────────────────────────────────────────────────────────

/// Monotonically-increasing counter used to assign stable per-watch
/// subject IDs at construction time (see [`Watch::new`]).
static NEXT_WATCH_ID: AtomicU64 = AtomicU64::new(1);

struct WatchRecord {
    watch: Arc<Watch>,
    target_ino: u64,
    mount_id: u64,
    /// Stable identifier for this registration, emitted as `WatchEvent::subject_id`.
    ///
    /// Unlike raw inode numbers, this ID is unique per [`register_watch`] call
    /// and is never reused, making it safe for long-lived watchers to use as a
    /// stable correlation key.
    subject_id: u64,
}

static REGISTRY: Mutex<alloc::vec::Vec<WatchRecord>> = Mutex::new(alloc::vec::Vec::new());

pub fn register_watch(node: &Arc<dyn VfsNode>, watch: Arc<Watch>, mount_id: u64) -> SysResult<()> {
    let stat = node.stat()?;
    // Use the stable ID pre-allocated in Watch::new so that VfsNode::close
    // can remove by the same ID without needing to look up the Arc pointer.
    let subject_id = watch.registration_id;
    let mut lock = REGISTRY.lock();
    lock.push(WatchRecord { watch, target_ino: stat.ino, mount_id, subject_id });
    Ok(())
}

/// Remove the watch with the given `registration_id` from the global registry.
///
/// Called by [`Watch::close`] to prevent the registry from growing without bound
/// and to stop delivering events to closed file descriptors.
pub fn unregister_watch(registration_id: u64) {
    REGISTRY.lock().retain(|r| r.subject_id != registration_id);
}

pub fn emit_event(node: &dyn VfsNode, mask: u32, name: Option<&str>, cookie: u32, mount_id: u64) {
    let stat = match node.stat() {
        Ok(s) => s,
        Err(_) => return,
    };

    let pending: alloc::vec::Vec<(Arc<Watch>, WatchEvent)> = {
        let lock = REGISTRY.lock();
        let mut pending = alloc::vec::Vec::new();
        for record in lock.iter() {
            if record.target_ino == stat.ino
                && record.mount_id == mount_id
                && record.watch.mask & mask != 0
            {
                let mut event = WatchEvent::default();
                event.mask = mask;
                event.cookie = cookie;
                event.subject_id = record.subject_id;
                if stat.is_dir() {
                    event.flags |= vfs_watch::event_flags::IS_DIR;
                }
                pending.push((record.watch.clone(), event));
            }
        }
        pending
    };

    for (watch, event) in pending {
        watch.queue.push(event, name);
    }
}

#[cfg(test)]
mod tests {
    use abi::vfs_watch::WatchEvent;

    use super::*;

    fn make_event(mask: u32) -> WatchEvent {
        let mut e = WatchEvent::default();
        e.mask = mask;
        e
    }

    /// A too-small buffer must NOT drop the pending event.
    #[test]
    fn small_buffer_preserves_event() {
        let watch = Watch::new(0xffff_ffff, 0);

        watch.queue.push(make_event(1), None);

        // Buffer is smaller than a WatchEvent header — must fail with EINVAL.
        let mut tiny = [0u8; 1];
        let result = watch.read(0, &mut tiny);
        assert_eq!(result, Err(Errno::EINVAL));

        // Event must still be in the queue.
        assert!(!watch.queue.is_empty(), "event was dropped after EINVAL");
    }

    /// A correctly-sized buffer delivers the event and removes it from the queue.
    #[test]
    fn correct_size_buffer_delivers_event() {
        let watch = Watch::new(0xffff_ffff, 0);

        watch.queue.push(make_event(42), None);

        let mut buf = vec![0u8; core::mem::size_of::<WatchEvent>()];
        let n = watch.read(0, &mut buf).expect("read should succeed");
        assert_eq!(n, core::mem::size_of::<WatchEvent>());
        assert!(watch.queue.is_empty(), "queue should be empty after successful read");
    }

    /// Buffer too small for name portion must also preserve the event.
    #[test]
    fn small_buffer_with_name_preserves_event() {
        let watch = Watch::new(0xffff_ffff, 0);

        watch.queue.push(make_event(3), Some("hello.txt"));

        // Just big enough for the header but not the name.
        let mut buf = vec![0u8; core::mem::size_of::<WatchEvent>()];
        let result = watch.read(0, &mut buf);
        assert_eq!(result, Err(Errno::EINVAL));
        assert!(!watch.queue.is_empty(), "event with name was dropped after EINVAL");
    }

    /// The re-queued event is returned on the next read with a large enough buffer.
    #[test]
    fn requeued_event_is_delivered_on_retry() {
        let watch = Watch::new(0xffff_ffff, 0);
        let name = "foo.txt";

        watch.queue.push(make_event(7), Some(name));

        // First attempt: too small.
        let mut tiny = [0u8; 2];
        assert_eq!(watch.read(0, &mut tiny), Err(Errno::EINVAL));

        // Second attempt: large enough buffer.
        let total = core::mem::size_of::<WatchEvent>() + name.len();
        let mut buf = vec![0u8; total];
        let n = watch.read(0, &mut buf).expect("retry read should succeed");
        assert_eq!(n, total);
        assert!(watch.queue.is_empty());
    }

    // ── Watch close unregisters from registry ────────────────────────────────

    /// Opening and immediately closing a watch must not leave a stale entry in
    /// the registry.  Without the `VfsNode::close` → `unregister_watch` fix the
    /// registry would grow by one entry per watch lifetime.
    #[test]
    fn close_removes_watch_from_registry() {
        let mount_id: u64 = 0xdead_0020;
        let ino: u64 = 100;
        let node = Arc::new(FixedInoNode(ino)) as Arc<dyn VfsNode>;

        let watch = Arc::new(Watch::new(0xffff_ffff, 0));
        let reg_id = watch.registration_id;

        register_watch(&node, watch.clone(), mount_id).unwrap();

        // Registry must contain our entry.
        assert!(
            REGISTRY.lock().iter().any(|r| r.subject_id == reg_id),
            "watch should be in registry after register_watch"
        );

        // Simulate fd close.
        watch.close();

        // Registry must no longer contain the entry.
        assert!(
            !REGISTRY.lock().iter().any(|r| r.subject_id == reg_id),
            "watch must be removed from registry after close"
        );
    }

    /// Repeated open/close cycles must not grow the registry.
    #[test]
    fn repeated_open_close_does_not_grow_registry() {
        let mount_id: u64 = 0xdead_0021;
        let ino: u64 = 101;
        let node = Arc::new(FixedInoNode(ino)) as Arc<dyn VfsNode>;

        // Record baseline size to be resilient against other tests leaving entries.
        let baseline = REGISTRY.lock().len();

        for _ in 0..10 {
            let w = Arc::new(Watch::new(0xffff_ffff, 0));
            register_watch(&node, w.clone(), mount_id).unwrap();
            w.close();
        }

        let after = REGISTRY.lock().len();
        assert_eq!(after, baseline, "registry must not grow after close");
    }

    struct FixedInoNode(u64);

    impl VfsNode for FixedInoNode {
        fn read(&self, _: u64, _: &mut [u8]) -> SysResult<usize> {
            Ok(0)
        }
        fn write(&self, _: u64, _: &[u8]) -> SysResult<usize> {
            Ok(0)
        }
        fn stat(&self) -> SysResult<VfsStat> {
            Ok(VfsStat { ino: self.0, mode: VfsStat::S_IFREG | 0o644, ..Default::default() })
        }
    }

    /// Watches on different mounts with the same inode number must be independent:
    /// an event emitted for mount A must not reach a watch registered for mount B.
    #[test]
    fn same_ino_different_mount_ids_are_independent() {
        // Isolate this test by operating on a private registry snapshot via the
        // public API, using distinct mount_ids that no other test uses.
        let mount_a: u64 = 0xdead_0001;
        let mount_b: u64 = 0xdead_0002;
        let shared_ino: u64 = 42;

        let node = Arc::new(FixedInoNode(shared_ino)) as Arc<dyn VfsNode>;

        let watch_a = Arc::new(Watch::new(0xffff_ffff, 0));
        let watch_b = Arc::new(Watch::new(0xffff_ffff, 0));

        register_watch(&node, watch_a.clone(), mount_a).unwrap();
        register_watch(&node, watch_b.clone(), mount_b).unwrap();

        // Emit an event only for mount_a.
        emit_event(&*node, abi::vfs_watch::mask::MODIFY, None, 0, mount_a);

        // watch_a must receive the event.
        assert!(!watch_a.queue.is_empty(), "watch_a should receive the event for mount_a");
        // watch_b must NOT receive the event.
        assert!(watch_b.queue.is_empty(), "watch_b must not receive events for a different mount");

        // Clean up registry entries added by this test.
        REGISTRY.lock().retain(|r| r.mount_id != mount_a && r.mount_id != mount_b);
    }

    /// A watch registered for a given (mount_id, ino) pair fires for that exact pair.
    #[test]
    fn same_mount_same_ino_fires() {
        let mount_id: u64 = 0xdead_0003;
        let shared_ino: u64 = 99;

        let node = Arc::new(FixedInoNode(shared_ino)) as Arc<dyn VfsNode>;
        let watch = Arc::new(Watch::new(0xffff_ffff, 0));

        register_watch(&node, watch.clone(), mount_id).unwrap();
        emit_event(&*node, abi::vfs_watch::mask::MODIFY, None, 0, mount_id);

        assert!(!watch.queue.is_empty(), "watch should fire for correct (mount_id, ino) pair");

        // Clean up.
        REGISTRY.lock().retain(|r| r.mount_id != mount_id);
    }

    /// `subject_id` must be a stable, per-registration ID — not a raw inode.
    ///
    /// Two separate registrations on the same node must produce different
    /// `subject_id` values.  Each watch must see its own stable ID in every
    /// event it receives.
    #[test]
    fn subject_id_is_stable_and_unique_per_registration() {
        let mount_id: u64 = 0xdead_0010;
        let ino: u64 = 77;

        let node = Arc::new(FixedInoNode(ino)) as Arc<dyn VfsNode>;

        let watch_x = Arc::new(Watch::new(0xffff_ffff, 0));
        let watch_y = Arc::new(Watch::new(0xffff_ffff, 0));

        register_watch(&node, watch_x.clone(), mount_id).unwrap();
        register_watch(&node, watch_y.clone(), mount_id).unwrap();

        // Emit two events so each watch gets one.
        emit_event(&*node, abi::vfs_watch::mask::MODIFY, None, 0, mount_id);
        emit_event(&*node, abi::vfs_watch::mask::MODIFY, None, 0, mount_id);

        // Read out the events.
        let buf_size = core::mem::size_of::<WatchEvent>();
        let mut buf_x = vec![0u8; buf_size];
        let mut buf_y = vec![0u8; buf_size];
        watch_x.read(0, &mut buf_x).expect("watch_x should have an event");
        watch_y.read(0, &mut buf_y).expect("watch_y should have an event");

        let event_x: WatchEvent = unsafe { core::ptr::read(buf_x.as_ptr() as *const _) };
        let event_y: WatchEvent = unsafe { core::ptr::read(buf_y.as_ptr() as *const _) };

        // The two registrations must have different subject_ids.
        assert_ne!(
            event_x.subject_id, event_y.subject_id,
            "different watch registrations must produce distinct subject_ids"
        );

        // Neither subject_id should equal the raw inode number.
        assert_ne!(event_x.subject_id, ino, "subject_id must not be a raw inode");
        assert_ne!(event_y.subject_id, ino, "subject_id must not be a raw inode");

        // A second event on watch_x must carry the same subject_id as the first.
        emit_event(&*node, abi::vfs_watch::mask::MODIFY, None, 0, mount_id);
        emit_event(&*node, abi::vfs_watch::mask::MODIFY, None, 0, mount_id);
        let mut buf_x2 = vec![0u8; buf_size];
        watch_x.read(0, &mut buf_x2).expect("watch_x second read");
        let event_x2: WatchEvent = unsafe { core::ptr::read(buf_x2.as_ptr() as *const _) };
        assert_eq!(
            event_x.subject_id, event_x2.subject_id,
            "subject_id must be stable across multiple events on the same registration"
        );

        // Clean up.
        REGISTRY.lock().retain(|r| r.mount_id != mount_id);
    }
}
