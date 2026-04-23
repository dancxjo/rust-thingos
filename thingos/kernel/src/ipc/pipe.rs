//! Anonymous pipe IPC — kernel-resident bounded byte stream.
//!
//! A **pipe** is a one-way, anonymous byte stream.  It is **not** a port:
//! pipes have no message boundaries and no capability-passing support.  Use a
//! pipe when you need a raw byte stream (stdio, shell pipelines, producer/consumer
//! data flows).  Use a port (`kernel/src/ipc/port.rs`) when you need discrete
//! messages, metadata, or capability (handle) transfer.
//!
//! See `docs/concepts/ports_vs_pipes.md` for the full comparison.
//!
//! Each pipe has a ring buffer, reader/writer ref counts, and wait queues.
//! Blocking uses the scheduler's `block_current_erased()` / `wake_task_erased()`.

use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

use spin::Mutex;

use crate::sched::wait_queue::WaitQueue;

// ---------------------------------------------------------------------------
// Ring buffer
// ---------------------------------------------------------------------------

struct RingBuf {
    data: Vec<u8>,
    head: usize, // next read position
    tail: usize, // next write position
    len: usize,  // bytes currently in buffer
    cap: usize,
}

impl RingBuf {
    fn new(capacity: usize) -> Self {
        let cap = capacity.max(1);
        Self {
            data: {
                let mut v = Vec::with_capacity(cap);
                v.resize(cap, 0);
                v
            },
            head: 0,
            tail: 0,
            len: 0,
            cap,
        }
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn is_full(&self) -> bool {
        self.len == self.cap
    }

    fn free_space(&self) -> usize {
        self.cap - self.len
    }

    fn available(&self) -> usize {
        self.len
    }

    /// Dequeue up to `dst.len()` bytes. Returns number of bytes read.
    fn dequeue(&mut self, dst: &mut [u8]) -> usize {
        let n = dst.len().min(self.len);
        if n == 0 {
            return 0;
        }
        // Split into at most two contiguous slices to avoid per-byte modulo.
        let first = (self.cap - self.head).min(n);
        dst[..first].copy_from_slice(&self.data[self.head..self.head + first]);
        let second = n - first;
        if second > 0 {
            dst[first..n].copy_from_slice(&self.data[..second]);
        }
        self.head = (self.head + n) % self.cap;
        self.len -= n;
        n
    }

    /// Enqueue up to `src.len()` bytes. Returns number of bytes written.
    fn enqueue(&mut self, src: &[u8]) -> usize {
        let n = src.len().min(self.free_space());
        if n == 0 {
            return 0;
        }
        // Split into at most two contiguous slices to avoid per-byte modulo.
        let first = (self.cap - self.tail).min(n);
        self.data[self.tail..self.tail + first].copy_from_slice(&src[..first]);
        let second = n - first;
        if second > 0 {
            self.data[..second].copy_from_slice(&src[first..n]);
        }
        self.tail = (self.tail + n) % self.cap;
        self.len += n;
        n
    }
}

// ---------------------------------------------------------------------------
// Pipe inner state
// ---------------------------------------------------------------------------

/// The volatile data protected by a mutex: buffer plus reference counts.
///
/// Wait queues live in the outer [`PipePair`] so that wakeups can be issued
/// *after* releasing this mutex, eliminating the scheduler-lock chain that
/// would otherwise form when holding `PipeData`'s lock during
/// `WaitQueue::wake_one()`.
struct PipeData {
    buf: RingBuf,
    readers: u32,
    writers: u32,
    nonblock: bool,
}

/// Shared pipe state for one pipe instance.
///
/// `Arc<PipePair>` is held by both [`PipeReadNode`] and [`PipeWriteNode`].
/// The wait queues are intentionally outside the `Mutex<PipeData>` so that:
///
/// * Wakeups (which eventually acquire the scheduler lock) happen after the
///   data lock is released, preventing lock convoys.
/// * Wait-queue registration can be performed *before* re-checking the buffer
///   under the data lock (see the "pre-register" pattern in the read/write
///   implementations), which prevents missed-wake races without requiring a
///   nested lock.
pub struct PipePair {
    inner: Mutex<PipeData>,
    read_waitq: WaitQueue,
    write_waitq: WaitQueue,
}

// ---------------------------------------------------------------------------
// Global registry
// ---------------------------------------------------------------------------

static NEXT_PIPE_ID: AtomicU64 = AtomicU64::new(1);
static PIPES: Mutex<BTreeMap<u64, Arc<PipePair>>> = Mutex::new(BTreeMap::new());

/// Default pipe capacity in bytes.
const DEFAULT_PIPE_CAPACITY: usize = 4096;

// ---------------------------------------------------------------------------
// Public API for syscall handlers
// ---------------------------------------------------------------------------

/// Create a new anonymous pipe. Returns the internal pipe ID used to back
/// read/write VFS endpoints.
pub fn create(capacity: u32, flags: u32) -> u64 {
    let cap = if capacity == 0 { DEFAULT_PIPE_CAPACITY } else { capacity as usize };
    let nonblock = (flags & abi::syscall::pipe_flags::NONBLOCK) != 0;

    let pair = Arc::new(PipePair {
        inner: Mutex::new(PipeData { buf: RingBuf::new(cap), readers: 0, writers: 0, nonblock }),
        read_waitq: WaitQueue::new(),
        write_waitq: WaitQueue::new(),
    });

    let id = NEXT_PIPE_ID.fetch_add(1, Ordering::Relaxed);
    PIPES.lock().insert(id, pair);
    id
}

/// Read from a pipe. Blocks (or returns EAGAIN) when empty and writers exist.
/// Returns Ok(0) on EOF (all writers closed).
pub fn read(pipe_id: u64, dst: &mut [u8]) -> Result<usize, abi::errors::Errno> {
    let pair = get_pipe(pipe_id)?;

    loop {
        if crate::sched::take_pending_interrupt_current() {
            return Err(abi::errors::Errno::EINTR);
        }

        let tid = unsafe { crate::sched::current_tid_current() };

        // Pre-register as a read waiter *before* inspecting the buffer.  If a
        // concurrent writer enqueues data and calls `write_waitq.wake_one()`
        // between our "buffer is empty" check and `block_current_erased`, the
        // scheduler's `wake_pending` flag ensures we return immediately rather
        // than sleeping forever.
        pair.read_waitq.push_back(tid as u64);

        let outcome = {
            let mut data = pair.inner.lock();

            if !data.buf.is_empty() {
                let n = data.buf.dequeue(dst);
                pair.read_waitq.remove(tid as u64);
                crate::ipc::diag::PIPE_READS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
                crate::ipc::diag::PIPE_BYTES_READ
                    .fetch_add(n as u64, core::sync::atomic::Ordering::Relaxed);
                Some(Ok(n))
            } else if data.writers == 0 {
                pair.read_waitq.remove(tid as u64);
                Some(Ok(0)) // EOF
            } else if data.nonblock {
                pair.read_waitq.remove(tid as u64);
                Some(Err(abi::errors::Errno::EAGAIN))
            } else {
                None // stay registered; will block
            }
        }; // data lock released

        match outcome {
            Some(Ok(n)) if n > 0 => {
                // Wake a blocked writer now that there is buffer space.
                pair.write_waitq.wake_one(); // outside data lock ✓
                return Ok(n);
            }
            Some(result) => return result,
            None => {
                unsafe { crate::task::block_current_erased() };
                pair.read_waitq.remove(tid as u64);
                if crate::sched::take_pending_interrupt_current() {
                    return Err(abi::errors::Errno::EINTR);
                }
                // Woken up — retry loop.
            }
        }
    }
}

/// Write to a pipe. Blocks (or returns EAGAIN) when full and readers exist.
/// Returns EPIPE if no readers remain.
pub fn write(pipe_id: u64, src: &[u8]) -> Result<usize, abi::errors::Errno> {
    if src.is_empty() {
        return Ok(0);
    }

    let pair = get_pipe(pipe_id)?;

    loop {
        if crate::sched::take_pending_interrupt_current() {
            return Err(abi::errors::Errno::EINTR);
        }

        let tid = unsafe { crate::sched::current_tid_current() };

        // Pre-register as a write waiter before inspecting the buffer (see
        // the read path above for the rationale).
        pair.write_waitq.push_back(tid as u64);

        let outcome = {
            let mut data = pair.inner.lock();

            if data.readers == 0 {
                pair.write_waitq.remove(tid as u64);
                crate::ipc::diag::PIPE_BROKEN_PIPE
                    .fetch_add(1, core::sync::atomic::Ordering::Relaxed);
                Some(Err(abi::errors::Errno::EPIPE))
            } else if !data.buf.is_full() {
                let n = data.buf.enqueue(src);
                pair.write_waitq.remove(tid as u64);
                crate::ipc::diag::PIPE_WRITES.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
                crate::ipc::diag::PIPE_BYTES_WRITTEN
                    .fetch_add(n as u64, core::sync::atomic::Ordering::Relaxed);
                Some(Ok(n))
            } else if data.nonblock {
                pair.write_waitq.remove(tid as u64);
                Some(Err(abi::errors::Errno::EAGAIN))
            } else {
                None // stay registered; will block
            }
        }; // data lock released

        match outcome {
            Some(Ok(n)) => {
                // Wake a blocked reader now that data is available.
                pair.read_waitq.wake_one(); // outside data lock ✓
                return Ok(n);
            }
            Some(result) => return result,
            None => {
                unsafe { crate::task::block_current_erased() };
                pair.write_waitq.remove(tid as u64);
                if crate::sched::take_pending_interrupt_current() {
                    return Err(abi::errors::Errno::EINTR);
                }
            }
        }
    }
}

/// Close the read end of a pipe.
pub fn close_read(pipe_id: u64) -> Result<(), abi::errors::Errno> {
    let pair = get_pipe(pipe_id)?;
    let (was_last_reader, should_remove) = {
        let mut data = pair.inner.lock();
        if data.readers == 0 {
            return Err(abi::errors::Errno::EBADF);
        }
        data.readers -= 1;
        let last = data.readers == 0;
        (last, last && data.writers == 0)
    }; // data lock released
    if was_last_reader {
        pair.write_waitq.wake_all(); // outside data lock ✓
    }
    if should_remove {
        PIPES.lock().remove(&pipe_id);
    }
    Ok(())
}

/// Close the write end of a pipe.
pub fn close_write(pipe_id: u64) -> Result<(), abi::errors::Errno> {
    let pair = get_pipe(pipe_id)?;
    let (was_last_writer, should_remove) = {
        let mut data = pair.inner.lock();
        if data.writers == 0 {
            return Err(abi::errors::Errno::EBADF);
        }
        data.writers -= 1;
        let last = data.writers == 0;
        (last, data.readers == 0 && last)
    }; // data lock released
    if was_last_writer {
        pair.read_waitq.wake_all(); // outside data lock ✓
    }
    if should_remove {
        PIPES.lock().remove(&pipe_id);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn get_pipe(id: u64) -> Result<Arc<PipePair>, abi::errors::Errno> {
    PIPES.lock().get(&id).cloned().ok_or(abi::errors::Errno::EBADF)
}

// ---------------------------------------------------------------------------
// VfsNode wrappers for pipe file descriptors
// ---------------------------------------------------------------------------

/// The read end of an anonymous pipe, exposed as a [`crate::vfs::VfsNode`].
///
/// Created by [`create_fd_pair`] and inserted into the process fd table as
/// stdin (fd 0) or as the read end of a pipe passed to `pipe()`.
pub struct PipeReadNode {
    inner: Arc<PipePair>,
    // Pipe ID kept for the global registry lookup (allows `close` to work).
    pipe_id: u64,
}

/// The write end of an anonymous pipe, exposed as a [`crate::vfs::VfsNode`].
pub struct PipeWriteNode {
    inner: Arc<PipePair>,
    pipe_id: u64,
}

impl crate::vfs::VfsNode for PipeReadNode {
    fn read(&self, _offset: u64, buf: &mut [u8]) -> abi::errors::SysResult<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        let pair = &self.inner;
        loop {
            if crate::sched::take_pending_interrupt_current() {
                return Err(abi::errors::Errno::EINTR);
            }
            let tid = unsafe { crate::sched::current_tid_current() };

            // Pre-register as a read waiter before inspecting the buffer so
            // that a concurrent writer cannot fire a wake between the "buffer
            // is empty" check and `block_current_erased` (missed-wake).
            pair.read_waitq.push_back(tid as u64);

            let outcome = {
                let mut data = pair.inner.lock();
                if !data.buf.is_empty() {
                    let n = data.buf.dequeue(buf);
                    pair.read_waitq.remove(tid as u64);
                    Some(Ok(n))
                } else if data.writers == 0 {
                    pair.read_waitq.remove(tid as u64);
                    Some(Ok(0)) // EOF
                } else if data.nonblock {
                    pair.read_waitq.remove(tid as u64);
                    Some(Err(abi::errors::Errno::EAGAIN))
                } else {
                    None // stay registered; will block
                }
            }; // data lock released

            match outcome {
                Some(Ok(n)) if n > 0 => {
                    pair.write_waitq.wake_one(); // outside data lock ✓
                    return Ok(n);
                }
                Some(result) => return result,
                None => {
                    unsafe { crate::task::block_current_erased() };
                    pair.read_waitq.remove(tid as u64);
                    if crate::sched::take_pending_interrupt_current() {
                        return Err(abi::errors::Errno::EINTR);
                    }
                }
            }
        }
    }

    fn write(&self, _offset: u64, _buf: &[u8]) -> abi::errors::SysResult<usize> {
        Err(abi::errors::Errno::EBADF)
    }

    fn stat(&self) -> abi::errors::SysResult<crate::vfs::VfsStat> {
        Ok(crate::vfs::VfsStat {
            mode: crate::vfs::VfsStat::S_IFIFO | 0o400,
            size: 0,
            ino: 0,
            ..Default::default()
        })
    }

    fn open(&self) {
        let mut data = self.inner.inner.lock();
        data.readers += 1;
        crate::kinfo!("PIPE_READ_OPEN: id={} readers={}", self.pipe_id, data.readers);
    }

    fn close(&self) {
        let pair = &self.inner;
        let (was_last_reader, readers, writers, should_remove) = {
            let mut data = pair.inner.lock();
            if data.readers > 0 {
                data.readers -= 1;
            }
            let last = data.readers == 0;
            (last, data.readers, data.writers, last && data.writers == 0)
        }; // data lock released
        crate::kinfo!("PIPE_READ_CLOSE: id={} readers={} writers={}", self.pipe_id, readers, writers);
        if was_last_reader {
            pair.write_waitq.wake_all(); // outside data lock ✓
        }
        if should_remove {
            PIPES.lock().remove(&self.pipe_id);
        }
    }

    fn poll(&self) -> u16 {
        use abi::syscall::poll_flags::{POLLHUP, POLLIN};
        let data = self.inner.inner.lock();
        let mut revents = 0;
        if !data.buf.is_empty() || data.writers == 0 {
            revents |= POLLIN;
        }
        if data.writers == 0 {
            revents |= POLLHUP;
        }
        revents
    }

    fn add_waiter(&self, tid: u64) {
        self.inner.read_waitq.push_back(tid);
    }

    fn remove_waiter(&self, tid: u64) {
        self.inner.read_waitq.remove(tid);
    }
}

impl crate::vfs::VfsNode for PipeWriteNode {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> abi::errors::SysResult<usize> {
        Err(abi::errors::Errno::EBADF)
    }

    fn write(&self, _offset: u64, buf: &[u8]) -> abi::errors::SysResult<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        let pair = &self.inner;
        loop {
            if crate::sched::take_pending_interrupt_current() {
                return Err(abi::errors::Errno::EINTR);
            }
            let tid = unsafe { crate::sched::current_tid_current() };

            // Pre-register as a write waiter before inspecting the buffer.
            pair.write_waitq.push_back(tid as u64);

            let outcome = {
                let mut data = pair.inner.lock();
                if data.readers == 0 {
                    pair.write_waitq.remove(tid as u64);
                    Some(Err(abi::errors::Errno::EPIPE))
                } else if !data.buf.is_full() {
                    let n = data.buf.enqueue(buf);
                    pair.write_waitq.remove(tid as u64);
                    Some(Ok(n))
                } else if data.nonblock {
                    pair.write_waitq.remove(tid as u64);
                    Some(Err(abi::errors::Errno::EAGAIN))
                } else {
                    None // stay registered; will block
                }
            }; // data lock released

            match outcome {
                Some(Ok(n)) => {
                    pair.read_waitq.wake_one(); // outside data lock ✓
                    return Ok(n);
                }
                Some(result) => return result,
                None => {
                    unsafe { crate::task::block_current_erased() };
                    pair.write_waitq.remove(tid as u64);
                    if crate::sched::take_pending_interrupt_current() {
                        return Err(abi::errors::Errno::EINTR);
                    }
                }
            }
        }
    }

    fn stat(&self) -> abi::errors::SysResult<crate::vfs::VfsStat> {
        Ok(crate::vfs::VfsStat {
            mode: crate::vfs::VfsStat::S_IFIFO | 0o200,
            size: 0,
            ino: 0,
            ..Default::default()
        })
    }

    fn open(&self) {
        let mut data = self.inner.inner.lock();
        data.writers += 1;
        crate::kinfo!("PIPE_WRITE_OPEN: id={} writers={}", self.pipe_id, data.writers);
    }

    fn close(&self) {
        let pair = &self.inner;
        let (was_last_writer, readers, writers, should_remove) = {
            let mut data = pair.inner.lock();
            if data.writers > 0 {
                data.writers -= 1;
            }
            let last = data.writers == 0;
            (last, data.readers, data.writers, data.readers == 0 && last)
        }; // data lock released
        crate::kinfo!("PIPE_WRITE_CLOSE: id={} readers={} writers={}", self.pipe_id, readers, writers);
        if was_last_writer {
            pair.read_waitq.wake_all(); // outside data lock ✓
        }
        if should_remove {
            PIPES.lock().remove(&self.pipe_id);
        }
    }

    fn poll(&self) -> u16 {
        use abi::syscall::poll_flags::{POLLERR, POLLHUP, POLLOUT};
        let data = self.inner.inner.lock();
        let mut revents = 0;
        if data.readers == 0 {
            revents |= POLLHUP | POLLERR;
        } else if !data.buf.is_full() {
            revents |= POLLOUT;
        }
        revents
    }

    fn add_waiter(&self, tid: u64) {
        self.inner.write_waitq.push_back(tid);
    }

    fn remove_waiter(&self, tid: u64) {
        self.inner.write_waitq.remove(tid);
    }
}

/// Wrap an existing pipe (by `pipe_id`) as a read-end `VfsNode`.
///
/// The pipe's existing `readers` count (set to 1 by [`create`]) represents
/// this node's reference.  When the node is closed via `VfsNode::close`,
/// `readers` is decremented, matching POSIX behaviour.
///
/// Returns `None` if `pipe_id` is not found.
pub fn read_node_for_id(pipe_id: u64) -> Option<alloc::sync::Arc<dyn crate::vfs::VfsNode>> {
    let pair = get_pipe(pipe_id).ok()?;
    Some(Arc::new(PipeReadNode { inner: pair, pipe_id }))
}

/// Wrap an existing pipe (by `pipe_id`) as a write-end `VfsNode`.
///
/// See [`read_node_for_id`] for reference-count semantics.
///
/// Returns `None` if `pipe_id` is not found.
pub fn write_node_for_id(pipe_id: u64) -> Option<alloc::sync::Arc<dyn crate::vfs::VfsNode>> {
    let pair = get_pipe(pipe_id).ok()?;
    Some(Arc::new(PipeWriteNode { inner: pair, pipe_id }))
}

/// Create an anonymous pipe and return a `(pipe_id, read_node, write_node)` triple.
///
/// The pipe ID remains an internal kernel identifier. The read/write nodes can
/// be inserted into a child process fd table via [`crate::vfs::handle_table::HandleTable::insert_at`].
pub fn create_fd_pair_with_id(
    capacity: u32,
    nonblock: bool,
) -> (u64, alloc::sync::Arc<dyn crate::vfs::VfsNode>, alloc::sync::Arc<dyn crate::vfs::VfsNode>) {
    let cap = if capacity == 0 { DEFAULT_PIPE_CAPACITY } else { capacity as usize };
    let pair = Arc::new(PipePair {
        inner: Mutex::new(PipeData { buf: RingBuf::new(cap), readers: 0, writers: 0, nonblock }),
        read_waitq: WaitQueue::new(),
        write_waitq: WaitQueue::new(),
    });
    let id = NEXT_PIPE_ID.fetch_add(1, Ordering::Relaxed);
    PIPES.lock().insert(id, pair.clone());
    let read_node: alloc::sync::Arc<dyn crate::vfs::VfsNode> =
        Arc::new(PipeReadNode { inner: pair.clone(), pipe_id: id });
    let write_node: alloc::sync::Arc<dyn crate::vfs::VfsNode> =
        Arc::new(PipeWriteNode { inner: pair, pipe_id: id });
    (id, read_node, write_node)
}

/// Create an anonymous pipe and return a `(read_node, write_node)` pair.
///
/// Both nodes are `Arc<dyn VfsNode>` and can be inserted directly into a
/// process fd table via `HandleTable::insert_at`.
pub fn create_fd_pair(
    capacity: u32,
    nonblock: bool,
) -> (alloc::sync::Arc<dyn crate::vfs::VfsNode>, alloc::sync::Arc<dyn crate::vfs::VfsNode>) {
    let (_id, r, w) = create_fd_pair_with_id(capacity, nonblock);
    (r, w)
}

#[cfg(test)]
mod tests {
    use core::sync::atomic::{AtomicBool, Ordering};

    use abi::syscall::poll_flags;

    use super::*;
    use crate::sched::blocking::BLOCK_CURRENT_HOOK;
    use crate::sched::hooks::{CURRENT_TID_HOOK, TAKE_PENDING_INTERRUPT_HOOK};
    use crate::vfs::VfsNode;

    static TEST_INTERRUPT_PENDING: AtomicBool = AtomicBool::new(false);

    fn test_current_tid() -> u64 {
        99
    }

    fn test_take_interrupt() -> bool {
        TEST_INTERRUPT_PENDING.swap(false, Ordering::SeqCst)
    }

    fn interrupt_on_block() {
        TEST_INTERRUPT_PENDING.store(true, Ordering::SeqCst);
    }

    fn make_pair() -> (Arc<dyn VfsNode>, Arc<dyn VfsNode>) {
        create_fd_pair(0, false)
    }

    // ── Read-end poll semantics ────────────────────────────────────────────

    #[test]
    fn read_end_not_ready_when_empty_with_writer() {
        let (r, _w) = make_pair();
        // Empty buffer, writer still alive → not readable.
        assert_eq!(r.poll() & poll_flags::POLLIN, 0);
    }

    #[test]
    fn read_end_ready_after_write() {
        let (r, w) = make_pair();
        w.write(0, b"hi").expect("write");
        assert_ne!(r.poll() & poll_flags::POLLIN, 0, "should be POLLIN after write");
    }

    #[test]
    fn read_end_reports_pollhup_when_writer_closed() {
        let (r, w) = make_pair();
        // `VfsNode::close()` is the fd-close path that decrements the peer
        // refcount.  Dropping the Arc alone does not change the pipe state.
        w.close();
        // No data, no writer → POLLIN (EOF indicator) + POLLHUP
        assert_ne!(r.poll() & poll_flags::POLLIN, 0, "POLLIN set on EOF");
        assert_ne!(r.poll() & poll_flags::POLLHUP, 0, "POLLHUP set on writer closed");
    }

    #[test]
    fn read_end_reports_both_pollin_and_pollhup_with_data_and_closed_writer() {
        let (r, w) = make_pair();
        w.write(0, b"x").expect("write");
        // `VfsNode::close()` is the fd-close path that decrements the peer
        // refcount.  Dropping the Arc alone does not change the pipe state.
        w.close();
        // Data available AND writer closed → both POLLIN and POLLHUP.
        let flags = r.poll();
        assert_ne!(flags & poll_flags::POLLIN, 0, "POLLIN set when data present");
        assert_ne!(flags & poll_flags::POLLHUP, 0, "POLLHUP set when writer gone");
    }

    // ── Write-end poll semantics ───────────────────────────────────────────

    #[test]
    fn write_end_ready_when_buffer_has_space() {
        let (_r, w) = make_pair();
        assert_ne!(w.poll() & poll_flags::POLLOUT, 0, "POLLOUT when space available");
    }

    #[test]
    fn write_end_reports_pollhup_when_reader_closed() {
        let (r, w) = make_pair();
        // `VfsNode::close()` is the fd-close path that decrements the peer
        // refcount.  Dropping the Arc alone does not change the pipe state.
        r.close();
        // Reader gone → POLLHUP | POLLERR on the write end.
        let flags = w.poll();
        assert_ne!(flags & poll_flags::POLLHUP, 0, "POLLHUP when reader closed");
        assert_ne!(flags & poll_flags::POLLERR, 0, "POLLERR when reader closed");
    }

    #[test]
    fn write_end_not_pollout_when_buffer_full() {
        // Use a tiny capacity (1 byte) so we can fill it easily.
        let pair = Arc::new(PipePair {
            inner: Mutex::new(PipeData {
                buf: RingBuf::new(1),
                readers: 1,
                writers: 1,
                nonblock: true,
            }),
            read_waitq: WaitQueue::new(),
            write_waitq: WaitQueue::new(),
        });
        let _id = NEXT_PIPE_ID.fetch_add(1, Ordering::Relaxed);
        PIPES.lock().insert(_id, pair.clone());
        let write_node = Arc::new(PipeWriteNode { inner: pair, pipe_id: _id });

        // Fill the buffer.
        write_node.write(0, b"X").expect("first write");
        // Buffer now full → POLLOUT should not be set.
        assert_eq!(write_node.poll() & poll_flags::POLLOUT, 0, "POLLOUT clear when buffer full");
    }

    #[test]
    fn read_returns_eintr_and_unregisters_waiter_when_interrupted() {
        let pair = Arc::new(PipePair {
            inner: Mutex::new(PipeData {
                buf: RingBuf::new(8),
                readers: 1,
                writers: 1,
                nonblock: false,
            }),
            read_waitq: WaitQueue::new(),
            write_waitq: WaitQueue::new(),
        });
        let read_node = PipeReadNode { inner: pair.clone(), pipe_id: 1 };
        unsafe {
            CURRENT_TID_HOOK = Some(test_current_tid);
            TAKE_PENDING_INTERRUPT_HOOK = Some(test_take_interrupt);
        }
        BLOCK_CURRENT_HOOK.store(interrupt_on_block as *mut (), Ordering::SeqCst);
        TEST_INTERRUPT_PENDING.store(false, Ordering::SeqCst);

        let mut buf = [0u8; 4];
        let err = read_node.read(0, &mut buf).unwrap_err();

        assert_eq!(err, abi::errors::Errno::EINTR);
        assert!(pair.read_waitq.is_empty());
    }

    #[test]
    fn write_returns_eintr_and_unregisters_waiter_when_interrupted() {
        let pair = Arc::new(PipePair {
            inner: Mutex::new(PipeData {
                buf: RingBuf::new(1),
                readers: 1,
                writers: 1,
                nonblock: false,
            }),
            read_waitq: WaitQueue::new(),
            write_waitq: WaitQueue::new(),
        });
        let write_node = PipeWriteNode { inner: pair.clone(), pipe_id: 2 };
        write_node.write(0, b"x").expect("fill pipe");

        unsafe {
            CURRENT_TID_HOOK = Some(test_current_tid);
            TAKE_PENDING_INTERRUPT_HOOK = Some(test_take_interrupt);
        }
        BLOCK_CURRENT_HOOK.store(interrupt_on_block as *mut (), Ordering::SeqCst);
        TEST_INTERRUPT_PENDING.store(false, Ordering::SeqCst);

        let err = write_node.write(0, b"y").unwrap_err();

        assert_eq!(err, abi::errors::Errno::EINTR);
        assert!(pair.write_waitq.is_empty());
    }
}
