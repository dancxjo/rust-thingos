//! Unix domain stream sockets — first-class VFS-backed IPC.
//!
//! # Design
//!
//! A Unix domain socket is a VFS file descriptor that participates in the
//! standard `poll()` readiness infrastructure.  Sockets are bound to
//! filesystem paths under `/run` (or elsewhere) via `bind()`, which:
//!
//!  1. Records the socket in the global [`SOCKET_REGISTRY`] keyed by path.
//!  2. Inserts a lightweight marker node into the VFS at that path.
//!
//! A client calls `connect(fd, "/run/foo.sock")` to create a peer connection.
//! A server calls `listen()` then `accept()` to hand out new fds for each
//! incoming connection.
//!
//! Connected socket pairs share a [`SocketPeer`]: two ring buffers
//! (one per direction) plus four wait queues.  A `UnixSocketNode` holds an
//! `Arc<Mutex<SocketPeer>>` along with a side tag (`SideA` / `SideB`).
//!
//! # Supported socket types
//! Currently only `AF_UNIX + SOCK_STREAM` is implemented.  `SOCK_DGRAM` will
//! be added later and can share this module.
//!
//! # State machine (per socket fd)
//! ```text
//!  Unbound ──bind──► Bound ──listen──► Listening
//!                                          │
//!                                      accept()
//!                                          │
//!                                      Connected ◄──connect()
//! ```

#![allow(clippy::mutex_atomic)]

use alloc::collections::{BTreeMap, VecDeque};
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

use spin::Mutex;

use crate::sched::wait_queue::WaitQueue;
use crate::vfs::{VfsNode, VfsStat};

// ---------------------------------------------------------------------------
// Ring buffer (reused from pipe module pattern)
// ---------------------------------------------------------------------------

struct RingBuf {
    data: Vec<u8>,
    head: usize,
    tail: usize,
    len: usize,
    cap: usize,
}

impl RingBuf {
    fn new(capacity: usize) -> Self {
        let cap = capacity.max(1);
        let mut data = Vec::with_capacity(cap);
        data.resize(cap, 0u8);
        Self { data, head: 0, tail: 0, len: 0, cap }
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }
    fn is_full(&self) -> bool {
        self.len == self.cap
    }
    fn available(&self) -> usize {
        self.len
    }
    fn free_space(&self) -> usize {
        self.cap - self.len
    }

    fn dequeue(&mut self, dst: &mut [u8]) -> usize {
        let n = dst.len().min(self.len);
        if n == 0 {
            return 0;
        }
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

    fn enqueue(&mut self, src: &[u8]) -> usize {
        let n = src.len().min(self.free_space());
        if n == 0 {
            return 0;
        }
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
// Shared peer state for connected socket pair
// ---------------------------------------------------------------------------

/// Default socket buffer capacity in bytes.
const DEFAULT_SOCK_CAPACITY: usize = 4096;

/// A datagram-like message carrying data bytes and zero or more capability
/// handles (VFS nodes).  Used by the `sendmsg`/`recvmsg` path.
struct SockMsg {
    data: Vec<u8>,
    fds: Vec<Arc<dyn VfsNode>>,
}

/// The volatile, mutex-protected part of a connected socket pair.
///
/// Wait queues live in the outer [`SocketPeer`] so that wakeups can be issued
/// *after* releasing this mutex, eliminating scheduler-lock chains.
struct SocketPeerData {
    a_to_b: RingBuf,
    b_to_a: RingBuf,
    msgs_a_to_b: VecDeque<SockMsg>,
    msgs_b_to_a: VecDeque<SockMsg>,
    /// Is A still alive (not shut down for writing / not closed)?
    a_alive: bool,
    /// Is B still alive?
    b_alive: bool,
}

/// State shared between the two ends of a connected socket pair.
///
/// `Arc<SocketPeer>` is stored in both sides' [`SocketState::Connected`]
/// variants.  The wait queues are *outside* the `Mutex<SocketPeerData>` so
/// that:
///
/// * Wakeups happen after the data lock is released (no lock convoy).
/// * Wait-queue registration can happen *before* the data lock is taken (see
///   the "pre-register" pattern in the read/write implementations), eliminating
///   missed-wake races without nested locking.
pub struct SocketPeer {
    data: Mutex<SocketPeerData>,
    /// A waiting to read (from b_to_a).
    a_read_waitq: WaitQueue,
    /// B waiting to read (from a_to_b).
    b_read_waitq: WaitQueue,
    /// A waiting to write (to a_to_b, which may be full).
    a_write_waitq: WaitQueue,
    /// B waiting to write (to b_to_a, which may be full).
    b_write_waitq: WaitQueue,
}

impl SocketPeer {
    fn new() -> Self {
        Self {
            data: Mutex::new(SocketPeerData {
                a_to_b: RingBuf::new(DEFAULT_SOCK_CAPACITY),
                b_to_a: RingBuf::new(DEFAULT_SOCK_CAPACITY),
                msgs_a_to_b: VecDeque::new(),
                msgs_b_to_a: VecDeque::new(),
                a_alive: true,
                b_alive: true,
            }),
            a_read_waitq: WaitQueue::new(),
            b_read_waitq: WaitQueue::new(),
            a_write_waitq: WaitQueue::new(),
            b_write_waitq: WaitQueue::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Listening socket entry (server side)
// ---------------------------------------------------------------------------

/// Server-side listening state kept in the socket registry.
pub struct ListeningSocket {
    backlog: usize,
    /// Accepted-but-not-yet-accept()ed connections: each is an
    /// `Arc<UnixSocketNode>` in the Connected/SideB state.
    accept_queue: Mutex<VecDeque<Arc<dyn VfsNode>>>,
    /// Woken when a new connection is enqueued.
    accept_waitq: WaitQueue,
}

impl ListeningSocket {
    fn new(backlog: usize) -> Self {
        Self { backlog, accept_queue: Mutex::new(VecDeque::new()), accept_waitq: WaitQueue::new() }
    }

    pub fn queue_len(&self) -> usize {
        self.accept_queue.lock().len()
    }
}

// ---------------------------------------------------------------------------
// Global socket registry — path → ListeningSocket
// ---------------------------------------------------------------------------

static SOCKET_REGISTRY: Mutex<BTreeMap<String, Arc<ListeningSocket>>> = Mutex::new(BTreeMap::new());

/// Register a path in the global socket registry, overwriting any stale entry.
fn registry_insert(path: &str, listener: Arc<ListeningSocket>) {
    SOCKET_REGISTRY.lock().insert(path.to_string(), listener);
}

/// Remove a path from the global socket registry.
fn registry_remove(path: &str) {
    SOCKET_REGISTRY.lock().remove(path);
}

/// Look up a listener by path; returns `None` if no such listener exists.
fn registry_get(path: &str) -> Option<Arc<ListeningSocket>> {
    SOCKET_REGISTRY.lock().get(path).cloned()
}

// ---------------------------------------------------------------------------
// UnixSocketNode — the VfsNode implementation
// ---------------------------------------------------------------------------

/// Which side of a connected [`SocketPeer`] this node represents.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Side {
    A,
    B,
}

/// Internal state of a [`UnixSocketNode`].
enum SocketState {
    /// Freshly created via `socket()`.
    Unbound,
    /// `bind()` called; socket is waiting for `listen()`.
    Bound { path: String },
    /// `listen()` called; server is ready to accept.
    Listening { path: String, listener: Arc<ListeningSocket> },
    /// `connect()` or `accept()` succeeded.
    Connected { side: Side, peer: Arc<SocketPeer>, shutdown_rd: bool, shutdown_wr: bool },
    /// Socket has been fully closed.
    Closed,
}

static NEXT_SOCKET_INO: AtomicU64 = AtomicU64::new(1);

/// A Unix domain stream socket exposed as a [`VfsNode`].
pub struct UnixSocketNode {
    ino: u64,
    state: Mutex<SocketState>,
}

impl UnixSocketNode {
    /// Create a new unbound socket.
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            ino: NEXT_SOCKET_INO.fetch_add(1, Ordering::Relaxed),
            state: Mutex::new(SocketState::Unbound),
        })
    }

    /// Create a pre-connected pair for `socketpair()`.
    ///
    /// Returns `(side_a, side_b)` — both are in the Connected state and can
    /// be inserted directly into the calling process's fd table.
    pub fn new_pair() -> (Arc<Self>, Arc<Self>) {
        let peer = Arc::new(SocketPeer::new());
        let a = Arc::new(Self {
            ino: NEXT_SOCKET_INO.fetch_add(1, Ordering::Relaxed),
            state: Mutex::new(SocketState::Connected {
                side: Side::A,
                peer: peer.clone(),
                shutdown_rd: false,
                shutdown_wr: false,
            }),
        });
        let b = Arc::new(Self {
            ino: NEXT_SOCKET_INO.fetch_add(1, Ordering::Relaxed),
            state: Mutex::new(SocketState::Connected {
                side: Side::B,
                peer,
                shutdown_rd: false,
                shutdown_wr: false,
            }),
        });
        (a, b)
    }

    /// Bind this socket to `path`.  Returns `EADDRINUSE` if already bound or
    /// if the path is already registered.
    pub fn bind(&self, path: &str) -> abi::errors::SysResult<()> {
        let mut state = self.state.lock();
        match &*state {
            SocketState::Unbound => {}
            _ => return Err(abi::errors::Errno::EINVAL),
        }
        if SOCKET_REGISTRY.lock().contains_key(path) {
            return Err(abi::errors::Errno::EADDRINUSE);
        }
        *state = SocketState::Bound { path: path.to_string() };
        Ok(())
    }

    /// Mark the socket as listening.  Must be called after `bind()`.
    pub fn listen(&self, backlog: usize) -> abi::errors::SysResult<()> {
        let mut state = self.state.lock();
        let path = match &*state {
            SocketState::Bound { path } => path.clone(),
            _ => return Err(abi::errors::Errno::EINVAL),
        };
        let backlog = backlog.max(1).min(128);
        let listener = Arc::new(ListeningSocket::new(backlog));
        registry_insert(&path, listener.clone());
        *state = SocketState::Listening { path, listener };
        Ok(())
    }

    /// Accept one incoming connection.  Blocks until a connection arrives.
    ///
    /// Returns a new `Arc<dyn VfsNode>` for the server side of the connection.
    pub fn accept(&self) -> abi::errors::SysResult<Arc<dyn VfsNode>> {
        loop {
            if crate::sched::take_pending_interrupt_current() {
                return Err(abi::errors::Errno::EINTR);
            }
            let tid = unsafe { crate::sched::current_tid_current() };
            {
                let state = self.state.lock();
                let listener = match &*state {
                    SocketState::Listening { listener, .. } => listener.clone(),
                    _ => return Err(abi::errors::Errno::EINVAL),
                };
                drop(state);

                let mut queue = listener.accept_queue.lock();
                if let Some(server_node) = queue.pop_front() {
                    return Ok(server_node);
                }
                // Register as waiter before dropping queue lock.
                listener.accept_waitq.push_back(tid as u64);
            }
            unsafe { crate::task::block_current_erased() };
            // After wakeup, loop and re-check.
            if crate::sched::take_pending_interrupt_current() {
                return Err(abi::errors::Errno::EINTR);
            }
        }
    }

    /// Connect to a listening socket at `path`.
    pub fn connect(&self, path: &str) -> abi::errors::SysResult<()> {
        let listener = registry_get(path).ok_or(abi::errors::Errno::ECONNREFUSED)?;

        let peer = Arc::new(SocketPeer::new());

        // Server-side socket (SideB): pre-connected, placed into accept queue.
        let server_node = Arc::new(UnixSocketNode {
            ino: NEXT_SOCKET_INO.fetch_add(1, Ordering::Relaxed),
            state: Mutex::new(SocketState::Connected {
                side: Side::B,
                peer: peer.clone(),
                shutdown_rd: false,
                shutdown_wr: false,
            }),
        });

        {
            let mut queue = listener.accept_queue.lock();
            if queue.len() >= listener.backlog {
                return Err(abi::errors::Errno::ECONNREFUSED);
            }
            queue.push_back(server_node as Arc<dyn VfsNode>);
        }
        // Wake the accept() side.
        listener.accept_waitq.wake_one();

        // Transition self to Connected (SideA).
        let mut state = self.state.lock();
        match &*state {
            SocketState::Unbound => {}
            _ => return Err(abi::errors::Errno::EINVAL),
        }
        *state =
            SocketState::Connected { side: Side::A, peer, shutdown_rd: false, shutdown_wr: false };
        Ok(())
    }

    /// Perform a shutdown in direction `how` (0=RD, 1=WR, 2=RDWR).
    pub fn shutdown(&self, how: u32) -> abi::errors::SysResult<()> {
        // Extract what we need while holding the state lock, then release it
        // before performing wakeups so we do not wake tasks while holding the
        // state lock (which would create a scheduler-lock chain).
        let (peer, side, shut_rd, shut_wr) = {
            let mut state = self.state.lock();
            match &mut *state {
                SocketState::Connected { side, peer, shutdown_rd, shutdown_wr } => {
                    let shut_rd = how == 0 || how == 2;
                    let shut_wr = how == 1 || how == 2;
                    if shut_rd {
                        *shutdown_rd = true;
                    }
                    if shut_wr {
                        *shutdown_wr = true;
                    }
                    (peer.clone(), *side, shut_rd, shut_wr)
                }
                SocketState::Closed => return Err(abi::errors::Errno::EBADF),
                _ => return Err(abi::errors::Errno::ENOTCONN),
            }
        }; // state lock released

        // Wake peer's blocked tasks outside the state lock.
        if shut_wr {
            // Peer's readers will see EOF.
            match side {
                Side::A => peer.b_read_waitq.wake_all(),
                Side::B => peer.a_read_waitq.wake_all(),
            }
        }
        if shut_rd {
            // Peer's writers will get EPIPE / ECONNRESET.
            match side {
                Side::A => peer.b_write_waitq.wake_all(),
                Side::B => peer.a_write_waitq.wake_all(),
            }
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// VfsNode implementation
// ---------------------------------------------------------------------------

impl VfsNode for UnixSocketNode {
    fn read(&self, _offset: u64, buf: &mut [u8]) -> abi::errors::SysResult<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        loop {
            if crate::sched::take_pending_interrupt_current() {
                return Err(abi::errors::Errno::EINTR);
            }
            let tid = unsafe { crate::sched::current_tid_current() };

            // Extract the peer Arc and side from the state lock, then release it
            // so wakeups do not need to hold the state lock.
            let (peer, side, shutdown_rd) = {
                let state = self.state.lock();
                match &*state {
                    SocketState::Connected { side, peer, shutdown_rd, .. } => {
                        (peer.clone(), *side, *shutdown_rd)
                    }
                    SocketState::Closed => return Err(abi::errors::Errno::EBADF),
                    _ => return Err(abi::errors::Errno::ENOTCONN),
                }
            }; // state lock released

            if shutdown_rd {
                return Ok(0);
            }

            // Pre-register as a read waiter before inspecting the buffer so
            // that a concurrent writer/closer cannot fire a wake between the
            // "no data" check and `block_current_erased` (missed-wake fix).
            let read_waitq = match side {
                Side::A => &peer.a_read_waitq,
                Side::B => &peer.b_read_waitq,
            };
            read_waitq.push_back(tid as u64);

            let outcome = {
                let mut d = peer.data.lock();
                let other_alive = match side {
                    Side::A => d.b_alive,
                    Side::B => d.a_alive,
                };
                let has_data = match side {
                    Side::A => !d.b_to_a.is_empty(),
                    Side::B => !d.a_to_b.is_empty(),
                };

                if has_data {
                    let n = match side {
                        Side::A => d.b_to_a.dequeue(buf),
                        Side::B => d.a_to_b.dequeue(buf),
                    };
                    read_waitq.remove(tid as u64);
                    Some(Ok(n))
                } else if !other_alive {
                    read_waitq.remove(tid as u64);
                    Some(Ok(0)) // EOF
                } else {
                    None // stay registered; will block
                }
            }; // data lock released

            match outcome {
                Some(Ok(n)) if n > 0 => {
                    // Wake a blocked writer on the peer side — outside data lock ✓
                    match side {
                        Side::A => peer.b_write_waitq.wake_one(),
                        Side::B => peer.a_write_waitq.wake_one(),
                    }
                    return Ok(n);
                }
                Some(result) => return result,
                None => {
                    unsafe { crate::task::block_current_erased() };
                    read_waitq.remove(tid as u64);
                    if crate::sched::take_pending_interrupt_current() {
                        return Err(abi::errors::Errno::EINTR);
                    }
                }
            }
        }
    }

    fn write(&self, _offset: u64, buf: &[u8]) -> abi::errors::SysResult<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        loop {
            if crate::sched::take_pending_interrupt_current() {
                return Err(abi::errors::Errno::EINTR);
            }
            let tid = unsafe { crate::sched::current_tid_current() };

            let (peer, side, shutdown_wr) = {
                let state = self.state.lock();
                match &*state {
                    SocketState::Connected { side, peer, shutdown_wr, .. } => {
                        (peer.clone(), *side, *shutdown_wr)
                    }
                    SocketState::Closed => return Err(abi::errors::Errno::EBADF),
                    _ => return Err(abi::errors::Errno::ENOTCONN),
                }
            }; // state lock released

            if shutdown_wr {
                return Err(abi::errors::Errno::EPIPE);
            }

            // Pre-register as a write waiter before inspecting the buffer.
            let write_waitq = match side {
                Side::A => &peer.a_write_waitq,
                Side::B => &peer.b_write_waitq,
            };
            write_waitq.push_back(tid as u64);

            let outcome = {
                let mut d = peer.data.lock();
                let other_alive = match side {
                    Side::A => d.b_alive,
                    Side::B => d.a_alive,
                };
                let tx_full = match side {
                    Side::A => d.a_to_b.is_full(),
                    Side::B => d.b_to_a.is_full(),
                };

                if !other_alive {
                    write_waitq.remove(tid as u64);
                    Some(Err(abi::errors::Errno::EPIPE))
                } else if !tx_full {
                    let n = match side {
                        Side::A => d.a_to_b.enqueue(buf),
                        Side::B => d.b_to_a.enqueue(buf),
                    };
                    write_waitq.remove(tid as u64);
                    Some(Ok(n))
                } else {
                    None // stay registered; will block
                }
            }; // data lock released

            match outcome {
                Some(Ok(n)) => {
                    // Wake a blocked reader on the peer side — outside data lock ✓
                    match side {
                        Side::A => peer.b_read_waitq.wake_one(),
                        Side::B => peer.a_read_waitq.wake_one(),
                    }
                    return Ok(n);
                }
                Some(result) => return result,
                None => {
                    unsafe { crate::task::block_current_erased() };
                    write_waitq.remove(tid as u64);
                    if crate::sched::take_pending_interrupt_current() {
                        return Err(abi::errors::Errno::EINTR);
                    }
                }
            }
        }
    }

    fn stat(&self) -> abi::errors::SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFSOCK | 0o600,
            ino: self.ino,
            nlink: 1,
            ..Default::default()
        })
    }

    fn close(&self) {
        // Atomically transition state to Closed and extract peer info.
        // The actual wakeups happen *after* releasing both state and data
        // locks so we don't hold hot locks during scheduler operations.
        let action = {
            let mut state = self.state.lock();
            match core::mem::replace(&mut *state, SocketState::Closed) {
                SocketState::Listening { path, .. } => {
                    registry_remove(&path);
                    None
                }
                SocketState::Connected { side, peer, .. } => Some((peer, side)),
                _ => None,
            }
        }; // state lock released

        if let Some((peer, side)) = action {
            // Mark the closing side as dead under the data lock.
            {
                let mut d = peer.data.lock();
                match side {
                    Side::A => d.a_alive = false,
                    Side::B => d.b_alive = false,
                }
            } // data lock released

            // Wake the peer's blocked readers and writers outside all locks ✓
            match side {
                Side::A => {
                    peer.b_read_waitq.wake_all();
                    peer.b_write_waitq.wake_all();
                }
                Side::B => {
                    peer.a_read_waitq.wake_all();
                    peer.a_write_waitq.wake_all();
                }
            }
        }
    }

    fn poll(&self) -> u16 {
        use abi::syscall::poll_flags::{POLLERR, POLLHUP, POLLIN, POLLOUT};
        let state = self.state.lock();
        match &*state {
            SocketState::Connected { side, peer, shutdown_rd, shutdown_wr } => {
                let d = peer.data.lock();
                let mut events = 0u16;
                let (rx_buf, tx_buf, other_alive) = match side {
                    Side::A => (&d.b_to_a, &d.a_to_b, d.b_alive),
                    Side::B => (&d.a_to_b, &d.b_to_a, d.a_alive),
                };
                if !shutdown_rd && (!rx_buf.is_empty() || !other_alive) {
                    events |= POLLIN;
                }
                if !other_alive {
                    events |= POLLHUP;
                }
                if *shutdown_wr {
                    events |= POLLERR;
                } else if !tx_buf.is_full() && other_alive {
                    events |= POLLOUT;
                }
                events
            }
            SocketState::Listening { listener, .. } => {
                if listener.queue_len() > 0 {
                    POLLIN
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    fn add_waiter(&self, tid: u64) {
        let state = self.state.lock();
        match &*state {
            SocketState::Connected { side, peer, .. } => match side {
                Side::A => peer.a_read_waitq.push_back(tid),
                Side::B => peer.b_read_waitq.push_back(tid),
            },
            SocketState::Listening { listener, .. } => {
                listener.accept_waitq.push_back(tid);
            }
            _ => {}
        }
    }

    fn remove_waiter(&self, tid: u64) {
        let state = self.state.lock();
        match &*state {
            SocketState::Connected { side, peer, .. } => match side {
                Side::A => peer.a_read_waitq.remove(tid),
                Side::B => peer.b_read_waitq.remove(tid),
            },
            SocketState::Listening { listener, .. } => {
                listener.accept_waitq.remove(tid);
            }
            _ => {}
        }
    }

    // ── Socket-specific VfsNode methods ─────────────────────────────────────

    fn sock_bind(&self, path: &str) -> abi::errors::SysResult<()> {
        self.bind(path)
    }

    fn sock_listen(&self, backlog: usize) -> abi::errors::SysResult<()> {
        self.listen(backlog)
    }

    fn sock_accept(&self) -> abi::errors::SysResult<alloc::sync::Arc<dyn VfsNode>> {
        self.accept()
    }

    fn sock_connect(&self, path: &str) -> abi::errors::SysResult<()> {
        self.connect(path)
    }

    fn sock_shutdown(&self, how: u32) -> abi::errors::SysResult<()> {
        self.shutdown(how)
    }

    fn sock_sendmsg(
        &self,
        data: &[u8],
        fds: alloc::vec::Vec<Arc<dyn VfsNode>>,
    ) -> abi::errors::SysResult<()> {
        // Extract state snapshot then release lock before waking.
        let (peer, side, shut_wr) = {
            let state = self.state.lock();
            match &*state {
                SocketState::Connected { side, peer, shutdown_wr, .. } => {
                    (peer.clone(), *side, *shutdown_wr)
                }
                SocketState::Closed => return Err(abi::errors::Errno::EBADF),
                _ => return Err(abi::errors::Errno::ENOTCONN),
            }
        }; // state lock released

        if shut_wr {
            return Err(abi::errors::Errno::EPIPE);
        }

        let other_alive = {
            let mut d = peer.data.lock();
            let alive = match side {
                Side::A => d.b_alive,
                Side::B => d.a_alive,
            };
            if alive {
                let msg = SockMsg { data: data.to_vec(), fds };
                match side {
                    Side::A => d.msgs_a_to_b.push_back(msg),
                    Side::B => d.msgs_b_to_a.push_back(msg),
                }
            }
            alive
        }; // data lock released

        if !other_alive {
            return Err(abi::errors::Errno::EPIPE);
        }
        // Wake the peer's reader outside the data lock ✓
        match side {
            Side::A => peer.b_read_waitq.wake_one(),
            Side::B => peer.a_read_waitq.wake_one(),
        }
        Ok(())
    }

    fn sock_recvmsg(
        &self,
    ) -> abi::errors::SysResult<Option<(alloc::vec::Vec<u8>, alloc::vec::Vec<Arc<dyn VfsNode>>)>>
    {
        let (peer, side, shut_rd) = {
            let state = self.state.lock();
            match &*state {
                SocketState::Connected { side, peer, shutdown_rd, .. } => {
                    (peer.clone(), *side, *shutdown_rd)
                }
                SocketState::Closed => return Err(abi::errors::Errno::EBADF),
                _ => return Err(abi::errors::Errno::ENOTCONN),
            }
        }; // state lock released

        if shut_rd {
            return Ok(None);
        }
        let mut d = peer.data.lock();
        let msg = match side {
            Side::A => d.msgs_b_to_a.pop_front(),
            Side::B => d.msgs_a_to_b.pop_front(),
        };
        match msg {
            Some(m) => Ok(Some((m.data, m.fds))),
            None => Ok(None), // EAGAIN — no message queued
        }
    }
}

// ---------------------------------------------------------------------------
// VFS marker node — placed at the bound path so stat()/ls work correctly
// ---------------------------------------------------------------------------

/// A lightweight VFS node that appears at the socket's bind path.
///
/// This node is purely a filesystem marker (S_IFSOCK).  Actual connect/accept
/// traffic is routed via the [`SOCKET_REGISTRY`], not through this node.
pub struct SocketFileMarker {
    ino: u64,
}

impl SocketFileMarker {
    pub fn new() -> Arc<Self> {
        Arc::new(Self { ino: NEXT_SOCKET_INO.fetch_add(1, Ordering::Relaxed) })
    }
}

impl VfsNode for SocketFileMarker {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> abi::errors::SysResult<usize> {
        Err(abi::errors::Errno::EOPNOTSUPP)
    }

    fn write(&self, _offset: u64, _buf: &[u8]) -> abi::errors::SysResult<usize> {
        Err(abi::errors::Errno::EOPNOTSUPP)
    }

    fn stat(&self) -> abi::errors::SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFSOCK | 0o600,
            ino: self.ino,
            nlink: 1,
            ..Default::default()
        })
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use abi::syscall::poll_flags;

    use super::*;

    // Wire up no-op scheduling hooks for tests.
    fn setup_test_hooks() {
        fn tid() -> u64 {
            1
        }
        fn no_interrupt() -> bool {
            false
        }
        unsafe {
            crate::sched::hooks::CURRENT_TID_HOOK = Some(tid);
            crate::sched::hooks::TAKE_PENDING_INTERRUPT_HOOK = Some(no_interrupt);
        }
    }

    // ── socketpair ──────────────────────────────────────────────────────────

    #[test]
    fn socketpair_write_a_read_b() {
        setup_test_hooks();
        let (a, b) = UnixSocketNode::new_pair();
        a.write(0, b"hello").expect("write");
        let mut buf = [0u8; 8];
        let n = b.read(0, &mut buf).expect("read");
        assert_eq!(&buf[..n], b"hello");
    }

    #[test]
    fn socketpair_write_b_read_a() {
        setup_test_hooks();
        let (a, b) = UnixSocketNode::new_pair();
        b.write(0, b"world").expect("write");
        let mut buf = [0u8; 8];
        let n = a.read(0, &mut buf).expect("read");
        assert_eq!(&buf[..n], b"world");
    }

    #[test]
    fn socketpair_close_a_gives_eof_on_b() {
        setup_test_hooks();
        let (a, b) = UnixSocketNode::new_pair();
        a.close();
        let mut buf = [0u8; 8];
        let n = b.read(0, &mut buf).expect("read after peer close");
        assert_eq!(n, 0, "expected EOF");
    }

    #[test]
    fn socketpair_poll_write_then_read() {
        setup_test_hooks();
        let (a, b) = UnixSocketNode::new_pair();

        // Both writable, neither readable initially.
        assert_ne!(a.poll() & poll_flags::POLLOUT, 0, "A writable");
        assert_eq!(a.poll() & poll_flags::POLLIN, 0, "A not readable");

        a.write(0, b"x").expect("write");

        assert_ne!(b.poll() & poll_flags::POLLIN, 0, "B readable after A writes");
    }

    #[test]
    fn socketpair_poll_hangup_after_close() {
        setup_test_hooks();
        let (a, b) = UnixSocketNode::new_pair();
        a.close();
        let flags = b.poll();
        assert_ne!(flags & poll_flags::POLLHUP, 0, "POLLHUP after peer close");
    }

    // ── bind / connect / accept  ────────────────────────────────────────────

    #[test]
    fn bind_and_connect_exchange_data() {
        setup_test_hooks();

        let server = UnixSocketNode::new();
        server.bind("/run/test_sock_1").expect("bind");
        server.listen(4).expect("listen");

        // Client connects.
        let client = UnixSocketNode::new();
        client.connect("/run/test_sock_1").expect("connect");

        // Server accepts.
        let accepted = server.accept().expect("accept");

        // Client → server.
        client.write(0, b"ping").expect("write");
        let mut buf = [0u8; 8];
        let n = accepted.read(0, &mut buf).expect("read");
        assert_eq!(&buf[..n], b"ping");

        // Server → client.
        accepted.write(0, b"pong").expect("write back");
        let n = client.read(0, &mut buf).expect("read back");
        assert_eq!(&buf[..n], b"pong");

        // Clean up registry entry.
        server.close();
    }

    #[test]
    fn duplicate_bind_returns_eaddrinuse() {
        setup_test_hooks();
        let s1 = UnixSocketNode::new();
        s1.bind("/run/dup_bind_test").expect("first bind");
        s1.listen(1).expect("listen");

        let s2 = UnixSocketNode::new();
        let err = s2.bind("/run/dup_bind_test").expect_err("duplicate bind");
        assert_eq!(err, abi::errors::Errno::EADDRINUSE);

        s1.close();
    }

    #[test]
    fn connect_to_nonexistent_path_returns_econnrefused() {
        setup_test_hooks();
        let client = UnixSocketNode::new();
        let err = client.connect("/run/no_such_socket").expect_err("connect to missing");
        assert_eq!(err, abi::errors::Errno::ECONNREFUSED);
    }
}
