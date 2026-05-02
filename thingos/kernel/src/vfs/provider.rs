//! Userland VFS provider port — kernel side.
//!
//! When a userland process calls `SYS_FS_MOUNT`, the kernel instantiates a
//! [`ProviderFs`] and registers it in the global mount table.  From that point
//! on every VFS operation whose path falls under the mount point is serialised
//! into a [`VfsRpcOp`] message and forwarded to the provider process via the
//! IPC port it supplied.
//!
//! # Concurrency model
//!
//! ## Kernel side — already fully concurrent
//!
//! The kernel RPC path is **already designed for concurrent callers**.  Each
//! call to [`ProviderRpc::rpc`] atomically claims a unique 16-bit `req_id`,
//! registers a per-request [`WaitQueue`], and blocks independently.  Responses
//! arriving on the shared response port are matched by `req_id` and routed to
//! the correct waiter; responses for other in-flight requests are buffered in
//! [`RpcState::responses`] and their waiters are woken.  Up to 65 535
//! concurrent kernel-side callers can be in flight simultaneously without any
//! serialisation beyond the brief lock windows around the response map.
//!
//! **No kernel change is required to support higher provider concurrency.**
//!
//! ## Userland side — current bottleneck
//!
//! Observed parallelism is currently limited by **userland providers that
//! process requests serially** (e.g. `httpsd`, `iso9660d`).  A single-threaded
//! provider reads one request, does its work, writes one response, and then
//! reads the next request.  While it is busy, other kernel callers wait even
//! though the kernel is perfectly capable of dispatching them in parallel.
//!
//! To unlock the full benefit of kernel-side multiplexing, userland providers
//! should adopt a parallel-dispatch model, for example:
//!
//! - Spawn a worker thread (or async task) per incoming request.
//! - Use a fixed-size thread pool and queue incoming request messages.
//! - Leverage async I/O to pipeline multiple responses.
//!
//! See `docs/kernel/provider-concurrency.md` for the full architecture
//! rationale and guidance for contributors.

use alloc::collections::BTreeMap;
use alloc::sync::{Arc, Weak};
use alloc::vec;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

use abi::device::DeviceKind;
use abi::display::ioctl::{
    DISPLAY_OP_ACCEL2D, DISPLAY_OP_COMMIT, DISPLAY_OP_GET_INFO, DISPLAY_OP_IMPORT_BUFFER,
    DISPLAY_OP_MOVE_CURSOR, DISPLAY_OP_RELEASE_BUFFER, DISPLAY_OP_SET_CURSOR,
};
use abi::display::types::BufferHandle;
use abi::errors::{Errno, SysResult};
use abi::vfs_rpc::{VFS_RPC_MAX_DATA, VfsRpcOp};
use spin::Mutex;

use super::{OpenFlags, VfsDriver, VfsNode, VfsStat};
use crate::sched::wait_queue::WaitQueue;
use crate::syscall::validate::{copyin, copyout};
use crate::vfs::memfd::MemFdNode;

const VFS_RPC_MAX_RESP: usize = VFS_RPC_MAX_DATA + 64;
const VFS_RPC_TIMEOUT_NS: u64 = 5_000_000_000;

fn rpc_timeout_wake_tick() -> u64 {
    let now_tick = crate::sched::TICK_COUNT.load(Ordering::Relaxed);
    let timeout_ticks = crate::time::duration_to_sleep_ticks(VFS_RPC_TIMEOUT_NS);
    now_tick.saturating_add(timeout_ticks)
}

// ── ProviderRpc ─────────────────────────────────────────────────────────────

/// Shared mutable state for concurrent VFS RPCs.
///
/// Multiple kernel threads can be in-flight simultaneously: each holds a
/// unique `req_id` and a slot in `waiters`.  The first thread to receive a
/// response message routes it to the correct waiter via `req_id`; responses
/// that belong to *other* waiters are parked in `responses` and their
/// [`WaitQueue`] is woken so they can collect their own reply on the next
/// iteration.
///
/// The `tainted` flag short-circuits all new RPCs after a provider error and
/// self-clears after [`PROVIDER_TAINT_COOLDOWN_NS`].
struct RpcState {
    tainted: bool,
    tainted_until_ns: u64,
    /// Next candidate request id. Request ids are allocated while holding this
    /// state lock so a late response cannot collide with a live or retained op.
    next_req_id: u16,
    /// Per-request wait queues, keyed by `req_id`.
    waiters: BTreeMap<u16, Arc<WaitQueue>>,
    /// Responses received for other waiters and not yet collected.
    responses: BTreeMap<u16, Vec<u8>>,
    /// In-flight or recently timed-out operation types for length calculation.
    ///
    /// Timed-out requests keep their opcode until the late response arrives.
    /// Without this, a successful late response for a variable-length op cannot
    /// be framed and can consume subsequent responses from the shared stream.
    ops: BTreeMap<u16, VfsRpcOp>,
    /// Byte-stream framing buffer for fragmented/coalesced responses.
    pending: Vec<u8>,
}

const PROVIDER_TAINT_COOLDOWN_NS: u64 = 2 * crate::time::NANOS_PER_SEC;

struct ProviderRpc {
    /// The provider's request port (kernel → provider).
    req: crate::ipc::Sender,
    /// The kernel's private response port (provider → kernel).
    resp: crate::ipc::Receiver,
    /// Write handle for the response port. Sent to the provider in every
    /// request header.
    resp_write_handle: u32,

    close_supported: AtomicBool,
    state: Mutex<RpcState>,
    /// Serialises access to the shared response port and ensures consistent framing.
    dispatch_lock: Mutex<()>,
}

impl ProviderRpc {
    fn new(
        req_port: Arc<crate::ipc::Port>,
        resp_port: Arc<crate::ipc::Port>,
        resp_write_handle: u32,
    ) -> Self {
        Self {
            req: crate::ipc::Sender::new(req_port),
            resp: crate::ipc::Receiver::new(resp_port),
            resp_write_handle,
            close_supported: AtomicBool::new(true),
            state: Mutex::new(RpcState {
                tainted: false,
                tainted_until_ns: 0,
                next_req_id: 1,
                waiters: BTreeMap::new(),
                responses: BTreeMap::new(),
                ops: BTreeMap::new(),
                pending: Vec::new(),
            }),
            dispatch_lock: Mutex::new(()),
        }
    }

    fn is_tainted(state: &mut RpcState) -> bool {
        if state.tainted && crate::time::monotonic_now_ns() >= state.tainted_until_ns {
            state.tainted = false;
            state.tainted_until_ns = 0;
            state.responses.clear();
            crate::kwarn!("VFS RPC: provider taint cooldown elapsed; untainting provider");
        }
        state.tainted
    }

    fn taint_with_cooldown(state: &mut RpcState) {
        state.tainted = true;
        state.tainted_until_ns = crate::time::monotonic_now_ns() + PROVIDER_TAINT_COOLDOWN_NS;
    }

    /// Return the PID of the provider process (the primary reader of the
    /// request port).  Returns 0 when the provider PID is not yet known.
    pub fn provider_pid(&self) -> u64 {
        self.req.port().primary_reader_pid()
    }

    fn try_collect_responses(&self) {
        let mut buf = [0u8; 4096];
        let Some(_dispatch_guard) = self.dispatch_lock.try_lock() else {
            return;
        };

        let n = self.resp.try_recv(&mut buf);
        if n == 0 {
            return;
        }

        let mut state = self.state.lock();
        state.pending.extend_from_slice(&buf[..n]);

        // Parse all complete messages from pending buffer.
        while state.pending.len() >= 3 {
            let resp_req_id = u16::from_le_bytes([state.pending[0], state.pending[1]]);
            let status = state.pending[2];

            let payload_len = if status == 0 {
                if let Some(&req_op) = state.ops.get(&resp_req_id) {
                    if let Some(len) = get_resp_payload_len(req_op, &state.pending[3..]) {
                        len
                    } else {
                        break;
                    }
                } else {
                    crate::kwarn!(
                        "VFS RPC: dropping unframeable OK response for unknown req_id={}",
                        resp_req_id
                    );
                    state.pending.clear();
                    break;
                }
            } else {
                0
            };

            let frame_len = 3 + payload_len;
            if state.pending.len() < frame_len {
                break;
            }

            let msg_payload = state.pending[3..frame_len].to_vec();
            if let Some(wq) = state.waiters.get(&resp_req_id).cloned() {
                let mut entry = vec![status];
                entry.extend_from_slice(&msg_payload);
                state.responses.insert(resp_req_id, entry);
                wq.wake_one();
            } else {
                // This is a late reply for a request that timed out or was
                // interrupted. Now that its frame has been safely consumed, the
                // retained opcode can be discarded.
                state.ops.remove(&resp_req_id);
            }
            state.pending.drain(..frame_len);
        }
    }

    fn has_buffered_response(&self, req_id: u16) -> bool {
        self.state.lock().responses.contains_key(&req_id)
    }

    /// Perform a multiplexed, concurrent round-trip RPC with the provider.
    pub fn rpc(&self, op: VfsRpcOp, payload: &[u8]) -> SysResult<Vec<u8>> {
        let tid = unsafe { crate::sched::current_tid_current() };
        crate::kdebug!("VFS_RPC: request op={:?} len={} tid={}", op, payload.len(), tid);
        let (req_id, wq) = {
            let mut state = self.state.lock();
            let mut req_id = state.next_req_id;
            let mut found = false;
            for _ in 0..u16::MAX {
                if req_id == 0 {
                    req_id = 1;
                }
                if !state.ops.contains_key(&req_id)
                    && !state.waiters.contains_key(&req_id)
                    && !state.responses.contains_key(&req_id)
                {
                    found = true;
                    break;
                }
                req_id = req_id.wrapping_add(1);
            }
            if !found {
                return Err(Errno::EAGAIN);
            }

            state.next_req_id = req_id.wrapping_add(1);
            if state.next_req_id == 0 {
                state.next_req_id = 1;
            }

            let wq = Arc::new(WaitQueue::new());
            wq.push_back(tid);
            state.waiters.insert(req_id, wq.clone());
            state.ops.insert(req_id, op);
            (req_id, wq)
        };

        // Record VFS RPC entry in the progress ring for freeze diagnostics.
        // A RAII guard records the matching exit on all return paths.
        let rpc_entry_ns = crate::trace::now_or_zero();
        let rpc_cpu = crate::runtime_base().current_cpu_index();
        crate::trace::progress_ring::push(
            crate::trace::progress_ring::ProgressTag::VfsRpcEntry,
            rpc_cpu,
            op as u64,
            rpc_entry_ns,
        );
        let _rpc_exit_guard = crate::trace::progress_ring::VfsRpcExitGuard::new(op as u64, rpc_cpu);

        let mut msg = Vec::with_capacity(7 + payload.len());
        msg.extend_from_slice(&self.resp_write_handle.to_le_bytes());
        msg.push(op as u8);
        msg.extend_from_slice(&req_id.to_le_bytes());
        msg.extend_from_slice(payload);

        // crate::kinfo!("VFS_RPC: sending request op={:?} id={} to port={:p}", op, req_id, Arc::as_ptr(self.req.port()));
        if !self.req.send_all(&msg) {
            crate::ipc::diag::VFS_RPC_ERRORS.fetch_add(1, Ordering::Relaxed);
            let mut state = self.state.lock();
            state.waiters.remove(&req_id);
            state.ops.remove(&req_id);
            return Err(Errno::EIO);
        }

        let deadline_ns = crate::time::monotonic_now_ns().saturating_add(VFS_RPC_TIMEOUT_NS);
        let mut timeout_armed = false;

        loop {
            // 1. Check if our response is already buffered
            {
                let mut state = self.state.lock();
                if let Some(resp) = state.responses.remove(&req_id) {
                    state.waiters.remove(&req_id);
                    state.ops.remove(&req_id);

                    let status = resp[0];
                    if status != 0 {
                        crate::kdebug!(
                            "VFS_RPC: response op={:?} id={} -> ERR({})",
                            op,
                            req_id,
                            status
                        );
                        return Err(errno_from_u8(status));
                    }
                    crate::kdebug!(
                        "VFS_RPC: response op={:?} id={} -> OK({})",
                        op,
                        req_id,
                        resp.len() - 1
                    );
                    return Ok(resp[1..].to_vec());
                }

                if !self.resp.has_writers() && state.pending.is_empty() {
                    state.waiters.remove(&req_id);
                    state.ops.remove(&req_id);
                    return Err(Errno::EPIPE);
                }
            }

            // 2. Try to receive and parse all pending responses.
            self.try_collect_responses();

            let now_ns = crate::time::monotonic_now_ns();
            if now_ns >= deadline_ns {
                // Upgrade to warn for VFS RPC timeouts — these stall the calling thread.
                crate::kwarn!("VFS RPC: TIMEOUT tid={} req_id={} op={:?}", tid, req_id, op);
                let mut state = self.state.lock();
                state.waiters.remove(&req_id);
                return Err(Errno::ETIMEDOUT);
            }

            self.resp.add_waiter(tid);
            wq.push_back(tid);
            self.try_collect_responses();
            if !self.has_buffered_response(req_id) {
                crate::sched::register_timeout_wake_current(tid, rpc_timeout_wake_tick());
                timeout_armed = true;
                unsafe {
                    crate::sched::block_current_erased();
                }
            }
            self.resp.remove_waiter(tid);
            if timeout_armed {
                crate::sched::unregister_timeout_wake_current(tid);
                timeout_armed = false;
            }

            if crate::sched::take_pending_interrupt_current() {
                let mut state = self.state.lock();
                state.waiters.remove(&req_id);
                return Err(Errno::EINTR);
            }
        }
    }
}

// ── ProviderFs ───────────────────────────────────────────────────────────────

/// A [`VfsDriver`] that forwards all operations to a userland provider via IPC.
pub struct ProviderFs {
    /// Shared RPC state (ports + serialization lock).
    rpc: Arc<ProviderRpc>,
    /// Map of provider-assigned handles to kernel-side wait queues.
    /// Protected by its own lock to avoid deadlocks with the RPC path.
    waiters: Mutex<BTreeMap<u64, Arc<WaitQueue>>>,
}

static PROVIDER_MAP: Mutex<BTreeMap<u32, Weak<ProviderFs>>> = Mutex::new(BTreeMap::new());

impl ProviderFs {
    pub fn new(
        req_port: Arc<crate::ipc::Port>,
        resp_port: Arc<crate::ipc::Port>,
        resp_write_handle: u32,
        req_port_id: u32,
    ) -> Arc<Self> {
        let this = Arc::new(Self {
            rpc: Arc::new(ProviderRpc::new(req_port, resp_port, resp_write_handle)),
            waiters: Mutex::new(BTreeMap::new()),
        });
        PROVIDER_MAP.lock().insert(req_port_id, Arc::downgrade(&this));
        this
    }

    pub fn notify(&self, handle: u64, _revents: u16) {
        let waiters = self.waiters.lock();
        if let Some(wq) = waiters.get(&handle) {
            wq.wake_all();
        }
    }

    fn get_wait_queue(&self, handle: u64) -> Arc<WaitQueue> {
        let mut waiters = self.waiters.lock();
        waiters.entry(handle).or_insert_with(|| Arc::new(WaitQueue::new())).clone()
    }
}

impl VfsDriver for ProviderFs {
    fn lookup(&self, path: &str) -> SysResult<Arc<dyn VfsNode>> {
        let path_bytes = path.as_bytes();
        let path_len = path_bytes.len() as u32;

        let mut payload = vec![0u8; 4 + path_bytes.len()];
        payload[..4].copy_from_slice(&path_len.to_le_bytes());
        payload[4..].copy_from_slice(path_bytes);

        let resp = self.rpc.rpc(VfsRpcOp::Lookup, &payload)?;
        parse_response_handle(&resp).map(|handle| {
            let wq = self.get_wait_queue(handle);
            Arc::new(ProviderNode { handle, rpc: self.rpc.clone(), wait_queue: wq })
                as Arc<dyn VfsNode>
        })
    }

    fn rename(&self, old_path: &str, new_path: &str) -> SysResult<()> {
        let old_bytes = old_path.as_bytes();
        let new_bytes = new_path.as_bytes();
        let mut payload = vec![0u8; 4 + old_bytes.len() + 4 + new_bytes.len()];
        payload[0..4].copy_from_slice(&(old_bytes.len() as u32).to_le_bytes());
        payload[4..4 + old_bytes.len()].copy_from_slice(old_bytes);
        let off = 4 + old_bytes.len();
        payload[off..off + 4].copy_from_slice(&(new_bytes.len() as u32).to_le_bytes());
        payload[off + 4..].copy_from_slice(new_bytes);

        let _resp = self.rpc.rpc(VfsRpcOp::Rename, &payload)?;
        Ok(())
    }
}

impl Drop for ProviderFs {
    fn drop(&mut self) {
        // All Sender/Receiver handles held by this ProviderFs (via port
        // and rpc fields) will be dropped, automatically releasing the
        // kernel's reference counts on the IPC ports.
    }
}

fn parse_response_handle(resp: &[u8]) -> SysResult<u64> {
    if resp.len() < 8 {
        return Err(Errno::EIO);
    }
    Ok(u64::from_le_bytes([resp[0], resp[1], resp[2], resp[3], resp[4], resp[5], resp[6], resp[7]]))
}

/// Map a raw display op code to a human-readable name for log messages.
fn display_op_name(op: u32) -> &'static str {
    match op {
        DISPLAY_OP_GET_INFO => "GET_INFO",
        DISPLAY_OP_IMPORT_BUFFER => "IMPORT_BUFFER",
        DISPLAY_OP_RELEASE_BUFFER => "RELEASE_BUFFER",
        DISPLAY_OP_COMMIT => "COMMIT",
        DISPLAY_OP_SET_CURSOR => "SET_CURSOR",
        DISPLAY_OP_MOVE_CURSOR => "MOVE_CURSOR",
        DISPLAY_OP_ACCEL2D => "ACCEL2D",
        _ => "UNKNOWN",
    }
}

// ── ProviderNode ─────────────────────────────────────────────────────────────

pub struct ProviderNode {
    handle: u64,
    rpc: Arc<ProviderRpc>,
    wait_queue: Arc<WaitQueue>,
}

fn append_readdir_stream_entry(
    name: &[u8],
    offset: u64,
    virtual_pos: &mut u64,
    buf: &mut [u8],
    written: &mut usize,
) -> bool {
    let entry_stream_len = u64::try_from(name.len()).unwrap_or(u64::MAX).saturating_add(1);
    if *virtual_pos + entry_stream_len > offset {
        let start_in_entry = if offset > *virtual_pos {
            usize::try_from(offset - *virtual_pos).unwrap_or(usize::MAX)
        } else {
            0
        };
        if start_in_entry < name.len() {
            let chunk = &name[start_in_entry..];
            let copy_n = chunk.len().min(buf.len() - *written);
            buf[*written..*written + copy_n].copy_from_slice(&chunk[..copy_n]);
            *written += copy_n;
            if copy_n == chunk.len() && *written < buf.len() {
                buf[*written] = 0;
                *written += 1;
            }
        } else if start_in_entry == name.len() && *written < buf.len() {
            buf[*written] = 0;
            *written += 1;
        }
    }

    *virtual_pos = virtual_pos.saturating_add(entry_stream_len);
    *written == buf.len()
}

fn get_resp_payload_len(op: VfsRpcOp, pending: &[u8]) -> Option<usize> {
    match op {
        VfsRpcOp::Lookup => Some(8), // handle: u64
        VfsRpcOp::Read | VfsRpcOp::Readdir => {
            if pending.len() < 4 {
                return None;
            }
            let n = u32::from_le_bytes([pending[0], pending[1], pending[2], pending[3]]) as usize;
            Some(4 + n)
        }
        VfsRpcOp::Write => Some(4), // bytes_written: u32
        VfsRpcOp::Stat => Some(20), // mode: u32, size: u64, ino: u64
        VfsRpcOp::Close
        | VfsRpcOp::SubscribeReady
        | VfsRpcOp::UnsubscribeReady
        | VfsRpcOp::Rename
        | VfsRpcOp::AttrRemove => Some(0),
        VfsRpcOp::Poll => Some(4), // revents: u32
        VfsRpcOp::DeviceCall | VfsRpcOp::AttrGet | VfsRpcOp::AttrList => {
            if pending.len() < 8 {
                return None;
            }
            let n = u32::from_le_bytes([pending[4], pending[5], pending[6], pending[7]]) as usize;
            Some(8 + n)
        }
        VfsRpcOp::AttrSet => Some(4),
        VfsRpcOp::Readlink => {
            // Readlink is raw bytes with no length prefix.
            // This is a protocol weakness. For now we take everything in pending.
            Some(pending.len())
        }
        VfsRpcOp::ReadIntoFd => Some(4), // bytes_written: u32
    }
}

unsafe impl Send for ProviderNode {}
unsafe impl Sync for ProviderNode {}

impl ProviderNode {
    /// Attempt to load all `buf.len()` bytes via the memfd bulk-transfer path.
    ///
    /// 1. Allocates a `MemFdNode` of `buf.len()` bytes.
    /// 2. Injects a writable fd for it into the provider's handle table.
    /// 3. Issues a single `ReadIntoFd` RPC — the provider writes the entire
    ///    file into the memfd with one `vfs_write` syscall.
    /// 4. Removes the injected fd and copies out of the memfd.
    ///
    /// Returns `Err(Errno::ENOSYS)` when the provider PID is unknown so the
    /// caller can fall back to the legacy looped-read path.
    fn read_all_via_memfd(&self, buf: &mut [u8]) -> SysResult<usize> {
        let size = buf.len();

        // We need the provider's PID to inject the fd.
        let provider_pid = self.rpc.provider_pid();
        if provider_pid == 0 {
            return Err(Errno::ENOSYS);
        }

        // Locate the provider's process info.
        let provider_pinfo =
            crate::sched::process_info_for_pid_current(provider_pid as u32).ok_or(Errno::ESRCH)?;

        // Allocate a MemFdNode sized for the whole file.
        let memfd = Arc::new(MemFdNode::new("elf-bulk-read", size)?);
        let memfd_node: Arc<dyn VfsNode> = memfd.clone();

        // Inject the memfd as a writable fd into the provider's handle table.
        let dest_fd = {
            let mut lock = provider_pinfo.lock();
            lock.handle_table.open(
                memfd_node,
                OpenFlags::write_only(),
                alloc::string::String::from("elf-bulk-read"),
            )?
        };

        // Build the ReadIntoFd payload:
        // [handle: u64][offset: u64][len: u32][dest_fd: u32]
        let mut payload = [0u8; 24];
        payload[0..8].copy_from_slice(&self.handle.to_le_bytes());
        payload[8..16].copy_from_slice(&0u64.to_le_bytes()); // offset = 0
        payload[16..20].copy_from_slice(&(size as u32).to_le_bytes()); // len
        payload[20..24].copy_from_slice(&dest_fd.to_le_bytes()); // dest_fd

        let rpc_result = self.rpc.rpc(VfsRpcOp::ReadIntoFd, &payload);

        // Always remove the injected fd once the RPC has resolved.
        {
            let mut lock = provider_pinfo.lock();
            let _ = lock.handle_table.close(dest_fd);
        }

        let resp = rpc_result?;
        if resp.len() < 4 {
            return Err(Errno::EIO);
        }
        let bytes_written = u32::from_le_bytes([resp[0], resp[1], resp[2], resp[3]]) as usize;

        // Copy from the memfd physical pages into the caller's buffer.
        // This is a plain memcpy inside the kernel — no IPC ring-buffer involved.
        let n = memfd.read(0, &mut buf[..bytes_written.min(size)])?;
        Ok(n)
    }
}

impl VfsNode for ProviderNode {
    fn close(&self) {
        // Best-effort close: notify the provider but ignore errors so a
        // stalled provider cannot wedge the calling task.
        if !self.rpc.close_supported.load(Ordering::Relaxed) {
            return;
        }
        let mut payload = [0u8; 8];
        payload[..8].copy_from_slice(&self.handle.to_le_bytes());
        if matches!(self.rpc.rpc(VfsRpcOp::Close, &payload), Err(Errno::ENOSYS)) {
            self.rpc.close_supported.store(false, Ordering::Relaxed);
        }
    }

    /// Use the memfd bulk-transfer path when possible, falling back to the
    /// legacy looped-`read` approach if the provider does not support
    /// `ReadIntoFd` or if the provider PID is not known.
    fn read_all_into(&self, buf: &mut [u8]) -> SysResult<usize> {
        match self.read_all_via_memfd(buf) {
            Ok(n) => return Ok(n),
            // ENOSYS: provider PID unknown or provider does not support ReadIntoFd.
            // ESRCH:  provider process gone (shouldn't happen in practice).
            // Fall through to the legacy path for both.
            Err(Errno::ENOSYS) | Err(Errno::ESRCH) => {}
            Err(e) => return Err(e),
        }

        // Legacy fallback: loop over 64 KB `Read` RPCs.
        let size = buf.len();
        let mut pos = 0;
        while pos < size {
            let n = self.read(pos as u64, &mut buf[pos..])?;
            if n == 0 {
                break;
            }
            pos += n;
        }
        Ok(pos)
    }

    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let len = buf.len().min(abi::vfs_rpc::VFS_RPC_MAX_DATA) as u32;
        let mut payload = [0u8; 20];
        payload[..8].copy_from_slice(&self.handle.to_le_bytes());
        payload[8..16].copy_from_slice(&offset.to_le_bytes());
        payload[16..20].copy_from_slice(&len.to_le_bytes());
        let resp = self.rpc.rpc(VfsRpcOp::Read, &payload)?;
        parse_response_read(&resp, buf)
    }

    fn write(&self, offset: u64, data: &[u8]) -> SysResult<usize> {
        let data_len = data.len().min(abi::vfs_rpc::VFS_RPC_MAX_DATA) as u32;
        let mut payload = vec![0u8; 20 + data_len as usize];
        payload[..8].copy_from_slice(&self.handle.to_le_bytes());
        payload[8..16].copy_from_slice(&offset.to_le_bytes());
        payload[16..20].copy_from_slice(&data_len.to_le_bytes());
        payload[20..].copy_from_slice(&data[..data_len as usize]);
        let resp = self.rpc.rpc(VfsRpcOp::Write, &payload)?;
        parse_response_u32(&resp).map(|n| n as usize)
    }

    fn stat(&self) -> SysResult<VfsStat> {
        let payload = self.handle.to_le_bytes();
        let resp = self.rpc.rpc(VfsRpcOp::Stat, &payload)?;
        parse_response_stat(&resp)
    }

    fn readdir(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        if buf.is_empty() {
            return Ok(0);
        }

        let mut provider_index = 0u64;
        let mut virtual_pos = 0u64;
        let mut written = 0usize;
        // Keep each provider call large enough to avoid excessive tiny round-trips,
        // but still capped to the RPC protocol's maximum payload size.
        let request_len = u32::try_from(core::cmp::min(
            core::cmp::max(buf.len(), 512),
            abi::vfs_rpc::VFS_RPC_MAX_DATA,
        ))
        .unwrap_or(u32::MAX);

        loop {
            let mut payload = [0u8; 20];
            payload[..8].copy_from_slice(&self.handle.to_le_bytes());
            payload[8..16].copy_from_slice(&provider_index.to_le_bytes());
            payload[16..20].copy_from_slice(&request_len.to_le_bytes());

            let resp = self.rpc.rpc(VfsRpcOp::Readdir, &payload)?;
            if resp.len() < 4 {
                return Err(Errno::EIO);
            }

            let payload_len = u32::from_le_bytes([resp[0], resp[1], resp[2], resp[3]]) as usize;
            if resp.len() < 4 + payload_len {
                crate::kwarn!(
                    "VFS provider readdir: truncated response payload len={} actual={}",
                    payload_len,
                    resp.len().saturating_sub(4)
                );
                return Err(Errno::EIO);
            }
            let data = &resp[4..4 + payload_len];
            if data.is_empty() {
                return Ok(written);
            }

            let mut read_ptr = 0usize;
            let mut parsed_entries = 0u64;
            while read_ptr + 10 <= data.len() {
                let name_len = data[read_ptr + 9] as usize;
                let entry_len = 10 + name_len;
                if read_ptr + entry_len > data.len() {
                    return Err(Errno::EIO);
                }

                let name = &data[read_ptr + 10..read_ptr + entry_len];
                if append_readdir_stream_entry(name, offset, &mut virtual_pos, buf, &mut written) {
                    return Ok(written);
                }
                parsed_entries += 1;
                read_ptr += entry_len;
            }

            if parsed_entries == 0 {
                return Ok(written);
            }
            provider_index = provider_index.saturating_add(parsed_entries);
        }
    }

    fn attr_get(&self, name: &str) -> SysResult<(u8, alloc::vec::Vec<u8>)> {
        let name_bytes = name.as_bytes();
        let mut payload = vec![0u8; 8 + 2 + name_bytes.len()];
        payload[..8].copy_from_slice(&self.handle.to_le_bytes());
        payload[8..10].copy_from_slice(&(name_bytes.len() as u16).to_le_bytes());
        payload[10..].copy_from_slice(name_bytes);

        let resp = self.rpc.rpc(VfsRpcOp::AttrGet, &payload)?;
        if resp.is_empty() {
            return Err(Errno::EIO);
        }
        let val_type = resp[0];
        Ok((val_type, resp[1..].to_vec()))
    }

    fn attr_set(&self, name: &str, value: &[u8], value_type: u8, flags: u8) -> SysResult<()> {
        let name_bytes = name.as_bytes();
        let mut payload = vec![0u8; 8 + 8 + name_bytes.len() + value.len()];
        payload[..8].copy_from_slice(&self.handle.to_le_bytes());

        let header = abi::attrs::AttrSetHeader {
            name_len: name_bytes.len() as u16,
            value_type,
            flags,
            value_len: value.len() as u32,
        };
        unsafe {
            core::ptr::copy_nonoverlapping(
                &header as *const _ as *const u8,
                payload[8..16].as_mut_ptr(),
                8,
            );
        }
        payload[16..16 + name_bytes.len()].copy_from_slice(name_bytes);
        payload[16 + name_bytes.len()..].copy_from_slice(value);

        let _resp = self.rpc.rpc(VfsRpcOp::AttrSet, &payload)?;
        Ok(())
    }

    fn attr_remove(&self, name: &str) -> SysResult<()> {
        let name_bytes = name.as_bytes();
        let mut payload = vec![0u8; 8 + 2 + name_bytes.len()];
        payload[..8].copy_from_slice(&self.handle.to_le_bytes());
        payload[8..10].copy_from_slice(&(name_bytes.len() as u16).to_le_bytes());
        payload[10..].copy_from_slice(name_bytes);

        let _resp = self.rpc.rpc(VfsRpcOp::AttrRemove, &payload)?;
        Ok(())
    }

    fn attr_list(&self, buf: &mut [u8]) -> SysResult<usize> {
        let payload = self.handle.to_le_bytes();
        let resp = self.rpc.rpc(VfsRpcOp::AttrList, &payload)?;
        let data = &resp[..];
        let n = data.len().min(buf.len());
        buf[..n].copy_from_slice(&data[..n]);
        Ok(n)
    }

    fn device_call(&self, call: &abi::device::DeviceCall) -> SysResult<usize> {
        let in_len = call.in_len as usize;
        let out_len = call.out_len as usize;
        if in_len > VFS_RPC_MAX_DATA || out_len > VFS_RPC_MAX_DATA {
            return Err(Errno::EINVAL);
        }
        let mut payload = vec![0u8; 8 + core::mem::size_of::<abi::device::DeviceCall>() + in_len];
        payload[..8].copy_from_slice(&self.handle.to_le_bytes());
        unsafe {
            core::ptr::copy_nonoverlapping(
                call as *const _ as *const u8,
                payload[8..].as_mut_ptr(),
                core::mem::size_of::<abi::device::DeviceCall>(),
            );
        }
        if in_len > 0 {
            unsafe {
                copyin(
                    &mut payload[8 + core::mem::size_of::<abi::device::DeviceCall>()..],
                    call.in_ptr as usize,
                )?;
            }
        }

        // FD Translation for Display IMPORT_BUFFER
        if call.kind == DeviceKind::Display && call.op == DISPLAY_OP_IMPORT_BUFFER {
            if in_len >= core::mem::size_of::<BufferHandle>() {
                let bh_offset = 8 + core::mem::size_of::<abi::device::DeviceCall>();
                let mut bh: BufferHandle =
                    unsafe { core::ptr::read_unaligned(payload[bh_offset..].as_ptr() as *const _) };

                // 1. Resolve node in caller
                let node = {
                    let pinfo = crate::sched::process_info_current().ok_or(Errno::ENOENT)?;
                    let lock = pinfo.lock();
                    lock.handle_table.get(bh.handle)?.node.clone()
                };

                // 2. Find provider process and install node
                let provider_pid = self.rpc.req.port().primary_reader_pid();
                if provider_pid != 0 {
                    if let Some(provider_pinfo) =
                        crate::sched::process_info_for_pid_current(provider_pid as u32)
                    {
                        let mut lock = provider_pinfo.lock();
                        let new_handle = lock.handle_table.open(
                            node,
                            OpenFlags::read_write(),
                            alloc::format!("imported-buffer-{}", bh.handle),
                        )?;
                        bh.handle = new_handle;

                        // Update payload with the translated handle
                        unsafe {
                            core::ptr::write_unaligned(
                                payload[bh_offset..].as_mut_ptr() as *mut BufferHandle,
                                bh,
                            );
                        }
                    }
                }
            }
        }

        // For Display device calls, emit entry/exit tracing and per-RPC watchdog.
        let is_display = call.kind == DeviceKind::Display;
        let caller_tid =
            if is_display { unsafe { crate::sched::current_tid_current() } } else { 0 };
        let provider_pid = if is_display { self.rpc.provider_pid() } else { 0 };
        let enter_ns = if is_display { crate::time::monotonic_now_ns() } else { 0 };
        if is_display {
            crate::kdebug!(
                "VFS_RPC: display device_call.enter op={} caller_tid={} provider_pid={}",
                display_op_name(call.op),
                caller_tid,
                provider_pid,
            );
        }

        let resp = self.rpc.rpc(VfsRpcOp::DeviceCall, &payload)?;

        if is_display {
            let duration_ms = crate::time::monotonic_now_ns().saturating_sub(enter_ns) / 1_000_000;
            crate::kdebug!(
                "VFS_RPC: display device_call.exit op={} caller_tid={} provider_pid={} duration_ms={}",
                display_op_name(call.op),
                caller_tid,
                provider_pid,
                duration_ms,
            );
            // Per-RPC watchdog: upgrade to warn on high-latency or stalled calls.
            if duration_ms >= 1000 {
                crate::kwarn!(
                    "VFS_RPC: display device_call.stall op={} duration_ms={} caller_tid={} provider_pid={}",
                    display_op_name(call.op),
                    duration_ms,
                    caller_tid,
                    provider_pid,
                );
            } else if duration_ms >= 200 {
                crate::kwarn!(
                    "VFS_RPC: display device_call.slow op={} duration_ms={} caller_tid={} provider_pid={}",
                    display_op_name(call.op),
                    duration_ms,
                    caller_tid,
                    provider_pid,
                );
            }
        }

        if resp.len() < 8 {
            return Err(Errno::EIO);
        }
        let ret_val = u32::from_le_bytes([resp[0], resp[1], resp[2], resp[3]]);
        let actual_out_len = u32::from_le_bytes([resp[4], resp[5], resp[6], resp[7]]) as usize;
        if actual_out_len > 0 && out_len > 0 {
            let copy_n = actual_out_len.min(out_len).min(resp.len() - 8);
            unsafe {
                copyout(call.out_ptr as usize, &resp[8..8 + copy_n])?;
            }
        }
        Ok(ret_val as usize)
    }

    fn poll(&self) -> u16 {
        // North Star: VFS poll() must never block. Provider RPCs are synchronous
        // and may block for seconds, which deadlocks the kernel if poll() is
        // called while holding the ProcessInfo spinlock.
        //
        // Even though sys_wait_many now releases the lock, blocking in poll()
        // still makes wait_many essentially synchronous (stalling on one
        // provider). We return "always ready" and let the subsequent read/write
        // block safely.
        abi::syscall::poll_flags::POLLIN | abi::syscall::poll_flags::POLLOUT
    }

    fn add_waiter(&self, tid: u64) {
        self.wait_queue.push_back(tid);
    }

    fn remove_waiter(&self, tid: u64) {
        self.wait_queue.remove(tid);
    }

    fn readlink(&self) -> SysResult<alloc::string::String> {
        let payload = self.handle.to_le_bytes();
        let resp = self.rpc.rpc(VfsRpcOp::Readlink, &payload)?;
        parse_response_readlink(&resp)
    }
}

pub fn notify_by_port(port_id: u32, handle: u64, revents: u16) -> SysResult<()> {
    let weak = PROVIDER_MAP.lock().get(&port_id).cloned().ok_or(Errno::ENOENT)?;
    if let Some(fs) = weak.upgrade() {
        fs.notify(handle, revents);
        Ok(())
    } else {
        Err(Errno::ENOENT)
    }
}

fn parse_response_read(resp: &[u8], buf: &mut [u8]) -> SysResult<usize> {
    if resp.len() < 4 {
        return Err(Errno::EIO);
    }
    let n = u32::from_le_bytes([resp[0], resp[1], resp[2], resp[3]]) as usize;
    let data = &resp[4..];
    let copy_n = n.min(data.len()).min(buf.len());
    buf[..copy_n].copy_from_slice(&data[..copy_n]);
    Ok(copy_n)
}

fn parse_response_u32(resp: &[u8]) -> SysResult<u32> {
    if resp.len() < 4 {
        return Err(Errno::EIO);
    }
    Ok(u32::from_le_bytes([resp[0], resp[1], resp[2], resp[3]]))
}

fn parse_response_stat(resp: &[u8]) -> SysResult<VfsStat> {
    if resp.len() < 20 {
        return Err(Errno::EIO);
    }
    let mode = u32::from_le_bytes([resp[0], resp[1], resp[2], resp[3]]);
    let size = u64::from_le_bytes([
        resp[4], resp[5], resp[6], resp[7], resp[8], resp[9], resp[10], resp[11],
    ]);
    let ino = u64::from_le_bytes([
        resp[12], resp[13], resp[14], resp[15], resp[16], resp[17], resp[18], resp[19],
    ]);
    Ok(VfsStat { mode, size, ino, ..Default::default() })
}

/// Parse a `Readlink` response.
///
/// Wire layout on success: `[status=0][target_bytes...]`.  The target is
/// returned as a UTF-8 string — non-UTF-8 targets produce `EIO`.
fn parse_response_readlink(resp: &[u8]) -> SysResult<alloc::string::String> {
    match alloc::string::String::from_utf8(resp.to_vec()) {
        Ok(s) => Ok(s),
        Err(_) => Err(Errno::EIO),
    }
}

fn errno_from_u8(v: u8) -> Errno {
    match v {
        1 => Errno::EPERM,
        2 => Errno::ENOENT,
        5 => Errno::EIO,
        9 => Errno::EBADF,
        11 => Errno::EAGAIN,
        12 => Errno::ENOMEM,
        13 => Errno::EACCES,
        17 => Errno::EEXIST,
        20 => Errno::ENOTDIR,
        21 => Errno::EISDIR,
        22 => Errno::EINVAL,
        28 => Errno::ENOSPC,
        32 => Errno::EPIPE,
        38 => Errno::ENOSYS,
        _ => Errno::EIO,
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use alloc::sync::Arc;
    use alloc::vec;

    use abi::vfs_rpc::VfsRpcReqHeader;

    use super::*;

    fn make_port(cap: usize) -> Arc<crate::ipc::Port> {
        Arc::new(crate::ipc::Port::new(cap))
    }

    fn encode_dirent_entry(ino: u64, file_type: u8, name: &str, out: &mut vec::Vec<u8>) {
        out.extend_from_slice(&ino.to_le_bytes());
        out.push(file_type);
        out.push(name.len().min(255) as u8);
        out.extend_from_slice(&name.as_bytes()[..name.len().min(255)]);
    }

    fn ok_readdir_payload(entries: &[(u64, u8, &str)]) -> vec::Vec<u8> {
        let mut data = vec::Vec::new();
        for (ino, ftype, name) in entries {
            encode_dirent_entry(*ino, *ftype, name, &mut data);
        }
        let mut resp = vec::Vec::with_capacity(5 + data.len());
        resp.push(0);
        resp.extend_from_slice(&(data.len() as u32).to_le_bytes());
        resp.extend_from_slice(&data);
        resp
    }

    fn ok_read_frame(req_id: u16, data: &[u8]) -> vec::Vec<u8> {
        let mut resp = vec::Vec::with_capacity(7 + data.len());
        resp.extend_from_slice(&req_id.to_le_bytes());
        resp.push(0);
        resp.extend_from_slice(&(data.len() as u32).to_le_bytes());
        resp.extend_from_slice(data);
        resp
    }

    fn ok_stat_frame(req_id: u16, mode: u32, size: u64, ino: u64) -> vec::Vec<u8> {
        let mut resp = vec::Vec::with_capacity(23);
        resp.extend_from_slice(&req_id.to_le_bytes());
        resp.push(0);
        resp.extend_from_slice(&mode.to_le_bytes());
        resp.extend_from_slice(&size.to_le_bytes());
        resp.extend_from_slice(&ino.to_le_bytes());
        resp
    }

    // ── errno_from_u8 ────────────────────────────────────────────────────────

    #[test]
    fn errno_from_u8_known_values() {
        assert!(matches!(errno_from_u8(1), Errno::EPERM));
        assert!(matches!(errno_from_u8(2), Errno::ENOENT));
        assert!(matches!(errno_from_u8(5), Errno::EIO));
        assert!(matches!(errno_from_u8(9), Errno::EBADF));
        assert!(matches!(errno_from_u8(11), Errno::EAGAIN));
        assert!(matches!(errno_from_u8(12), Errno::ENOMEM));
        assert!(matches!(errno_from_u8(13), Errno::EACCES));
        assert!(matches!(errno_from_u8(17), Errno::EEXIST));
        assert!(matches!(errno_from_u8(20), Errno::ENOTDIR));
        assert!(matches!(errno_from_u8(21), Errno::EISDIR));
        assert!(matches!(errno_from_u8(22), Errno::EINVAL));
        assert!(matches!(errno_from_u8(28), Errno::ENOSPC));
        assert!(matches!(errno_from_u8(32), Errno::EPIPE));
        assert!(matches!(errno_from_u8(38), Errno::ENOSYS));
    }

    #[test]
    fn errno_from_u8_unknown_falls_back_to_eio() {
        assert!(matches!(errno_from_u8(200), Errno::EIO));
        assert!(matches!(errno_from_u8(255), Errno::EIO));
        assert!(matches!(errno_from_u8(0), Errno::EIO)); // 0 is "OK", not an errno
    }

    // ── parse_response_handle ────────────────────────────────────────────────

    #[test]
    fn parse_response_handle_ok() {
        let mut resp = vec![0u8; 8];
        resp[0..8].copy_from_slice(&42u64.to_le_bytes());
        assert_eq!(parse_response_handle(&resp).unwrap(), 42u64);
    }

    #[test]
    fn parse_response_handle_too_short() {
        let resp = vec![0u8, 0u8, 1u8];
        assert!(matches!(parse_response_handle(&resp), Err(Errno::EIO)));
    }

    #[test]
    fn parse_response_handle_empty() {
        assert!(matches!(parse_response_handle(&[]), Err(Errno::EIO)));
    }

    // ── parse_response_u32 ───────────────────────────────────────────────────

    #[test]
    fn parse_response_u32_ok() {
        let mut resp = vec![0u8; 4];
        resp[0..4].copy_from_slice(&1024u32.to_le_bytes());
        assert_eq!(parse_response_u32(&resp).unwrap(), 1024u32);
    }

    #[test]
    fn parse_response_u32_empty() {
        assert!(matches!(parse_response_u32(&[]), Err(Errno::EIO)));
    }

    #[test]
    fn parse_response_u32_too_short() {
        let resp = vec![1u8]; // only 1 byte payload
        assert!(matches!(parse_response_u32(&resp), Err(Errno::EIO)));
    }

    // ── parse_response_stat ──────────────────────────────────────────────────

    #[test]
    fn parse_response_stat_ok() {
        let mut resp = vec![0u8; 20]; // 4 (mode) + 8 (size) + 8 (ino)
        resp[0..4].copy_from_slice(&0o100644u32.to_le_bytes());
        resp[4..12].copy_from_slice(&4096u64.to_le_bytes());
        resp[12..20].copy_from_slice(&7u64.to_le_bytes());
        let stat = parse_response_stat(&resp).unwrap();
        assert_eq!(stat.mode, 0o100644);
        assert_eq!(stat.size, 4096);
        assert_eq!(stat.ino, 7);
    }

    #[test]
    fn parse_response_stat_too_short() {
        let resp = vec![0u8; 9]; // only 9 payload bytes (need 20)
        assert!(matches!(parse_response_stat(&resp), Err(Errno::EIO)));
    }

    // ── parse_response_read ──────────────────────────────────────────────────

    #[test]
    fn parse_response_read_ok() {
        let data = b"hello";
        let mut resp = vec![0u8; 4 + data.len()];
        resp[0..4].copy_from_slice(&(data.len() as u32).to_le_bytes());
        resp[4..].copy_from_slice(data);
        let mut buf = [0u8; 16];
        let n = parse_response_read(&resp, &mut buf).unwrap();
        assert_eq!(n, 5);
        assert_eq!(&buf[..5], b"hello");
    }

    #[test]
    fn parse_response_read_buf_smaller_than_data() {
        // Provider reports 10 bytes, but caller only has a 4-byte buf
        let data = b"0123456789";
        let mut resp = vec![0u8; 4 + data.len()];
        resp[0..4].copy_from_slice(&(data.len() as u32).to_le_bytes());
        resp[4..].copy_from_slice(data);
        let mut buf = [0u8; 4];
        let n = parse_response_read(&resp, &mut buf).unwrap();
        assert_eq!(n, 4);
        assert_eq!(&buf, b"0123");
    }

    // ── parse_response_readlink ──────────────────────────────────────────────

    #[test]
    fn parse_response_readlink_ok() {
        let target = "/https/b.example/new";
        let mut resp = vec![0u8; target.len()];
        resp[0..].copy_from_slice(target.as_bytes());
        let got = parse_response_readlink(&resp).expect("readlink");
        assert_eq!(got, target);
    }

    #[test]
    fn parse_response_readlink_empty_payload_is_ok_empty_string() {
        // A provider may serve an empty target (unusual, but valid).
        let resp = vec![];
        assert_eq!(parse_response_readlink(&resp).unwrap(), "");
    }

    #[test]
    fn parse_response_readlink_non_utf8_maps_to_eio() {
        let resp = vec![0xFF, 0xFE, 0xFD];
        assert!(matches!(parse_response_readlink(&resp), Err(Errno::EIO)));
    }

    // ── Dead provider: request ring full → EIO ───────────────────────────────

    /// When the kernel cannot write to the provider's request port (ring full),
    /// `rpc()` must return `Err(EIO)` immediately — this is the
    /// "provider back-pressure / dead" path.
    #[test]
    fn rpc_returns_eio_when_request_ring_full() {
        let req_port = make_port(16); // tiny ring so it fills quickly
        let resp_port = make_port(256);

        // Flood the ring buffer so the next send() will return 0.
        let fill = vec![0xABu8; req_port.capacity()];
        req_port.send(&fill);

        let ch = ProviderRpc::new(req_port, resp_port, 99);

        // A Stat request payload is 8 bytes (handle: u64); combined with the
        // 7-byte header the message is 15 bytes and won't fit the full ring.
        let result = ch.rpc(VfsRpcOp::Stat, &[0u8; 8]);
        assert!(
            matches!(result, Err(Errno::EIO)),
            "expected EIO when request ring is full, got {:?}",
            result
        );
    }

    #[test]
    fn rpc_request_write_is_all_or_nothing() {
        let req_port = make_port(16);
        let resp_port = make_port(256);

        let fill = vec![0xABu8; 12];
        assert!(req_port.send_all(&fill));

        let ch = ProviderRpc::new(req_port.clone(), resp_port, 99);
        let result = ch.rpc(VfsRpcOp::Stat, &[0u8; 8]);
        assert!(matches!(result, Err(Errno::EIO)));

        let mut drain = [0u8; 32];
        let n = req_port.try_recv(&mut drain);
        assert_eq!(n, fill.len(), "failed RPC must not leave a partial request frame");
        assert_eq!(&drain[..n], fill.as_slice());
    }

    #[test]
    fn rpc_timeout_wake_uses_scheduler_ticks() {
        use core::sync::atomic::Ordering;

        crate::sched::TICK_COUNT.store(1234, Ordering::Relaxed);
        let wake_tick = rpc_timeout_wake_tick();
        let expected = 1234 + crate::time::duration_to_sleep_ticks(VFS_RPC_TIMEOUT_NS);

        assert_eq!(wake_tick, expected);
        assert!(
            wake_tick < 10_000,
            "RPC timeout wake should be in scheduler ticks, not an absolute nanosecond deadline"
        );
    }

    // ── Dead provider: response writer gone → EPIPE ──────────────────────────

    /// When the provider dies after the kernel sends a request but before it
    /// sends a response, the response port's writer count drops to zero.
    /// `recv_response()` must detect this and return `Err(EPIPE)` without
    /// blocking forever.
    #[test]
    fn rpc_returns_epipe_when_response_writer_gone() {
        let req_port = make_port(4096);
        let resp_port = make_port(256);

        // Simulate provider death: drop the write end of the response port.
        // (In production the write handle is in the provider's handle table;
        // when the process exits the handle table drops all handles.)
        resp_port.close_writer();

        let ch = ProviderRpc::new(req_port, resp_port, 99);

        // Send succeeds (data lands in the ring), but response never arrives.
        let payload = b"\x05\x00\x00\x00hello"; // Lookup "hello"
        let result = ch.rpc(VfsRpcOp::Lookup, payload);
        assert!(
            matches!(result, Err(Errno::EPIPE)),
            "expected EPIPE when response writer is gone, got {:?}",
            result
        );
    }

    // ── Dead provider: diagnostics counter ───────────────────────────────────

    /// Every dead-provider event must increment `VFS_RPC_DEAD_PROVIDER`.
    #[test]
    fn dead_provider_increments_counter() {
        use core::sync::atomic::Ordering;

        let before = crate::ipc::diag::VFS_RPC_DEAD_PROVIDER.load(Ordering::Relaxed);

        let req_port = make_port(4096);
        let resp_port = make_port(256);
        resp_port.close_writer();

        let ch = ProviderRpc::new(req_port, resp_port, 0);
        // Ignore the result; we only care about the counter.
        let _ = ch.rpc(VfsRpcOp::Stat, &[0u8; 8]);

        let after = crate::ipc::diag::VFS_RPC_DEAD_PROVIDER.load(Ordering::Relaxed);
        assert!(after > before, "VFS_RPC_DEAD_PROVIDER should have been incremented");
    }

    /// Every dead-provider event must also increment the generic
    /// `VFS_RPC_ERRORS` counter.
    #[test]
    fn dead_provider_increments_error_counter() {
        use core::sync::atomic::Ordering;

        let before = crate::ipc::diag::VFS_RPC_ERRORS.load(Ordering::Relaxed);

        let req_port = make_port(4096);
        let resp_port = make_port(256);
        resp_port.close_writer();

        let ch = ProviderRpc::new(req_port, resp_port, 0);
        let _ = ch.rpc(VfsRpcOp::Stat, &[0u8; 8]);

        let after = crate::ipc::diag::VFS_RPC_ERRORS.load(Ordering::Relaxed);
        assert!(after > before);
    }

    // ── Successful round-trip (response in ring before recv) ─────────────────

    /// When a response is already waiting in the ring before `rpc()` is called,
    /// the call must return successfully without ever blocking.
    #[test]
    fn rpc_ok_when_response_preloaded() {
        let req_port = make_port(4096);
        let resp_port = make_port(4096);

        // Pre-load a valid Stat response into the response ring.
        // Format: [req_id: u16][status=0][mode: u32 LE][size: u64 LE][ino: u64 LE]
        let mut preloaded = vec![0u8; 23];
        preloaded[0..2].copy_from_slice(&1u16.to_le_bytes()); // req_id: 1
        preloaded[2] = 0; // OK
        preloaded[3..7].copy_from_slice(&0o040755u32.to_le_bytes()); // mode: dir
        preloaded[7..15].copy_from_slice(&0u64.to_le_bytes()); // size: 0
        preloaded[15..23].copy_from_slice(&1u64.to_le_bytes()); // ino: 1
        resp_port.send(&preloaded);

        let ch = ProviderRpc::new(req_port, resp_port, 0);

        // The Stat RPC should complete without blocking.
        let raw = ch.rpc(VfsRpcOp::Stat, &[0u8; 8]).unwrap();
        let stat = parse_response_stat(&raw).unwrap();
        assert_eq!(stat.mode, 0o040755);
        assert_eq!(stat.ino, 1);
    }

    #[test]
    fn late_variable_length_response_does_not_consume_next_response() {
        let req_port = make_port(4096);
        let resp_port = make_port(4096);
        let ch = ProviderRpc::new(req_port, resp_port.clone(), 0);

        {
            let mut state = ch.state.lock();
            // Request 7 has already timed out: keep its opcode so the late
            // variable-length Read response can still be framed and dropped.
            state.ops.insert(7, VfsRpcOp::Read);

            // Request 8 is still waiting and its response is coalesced behind
            // the late response for request 7 in the same response-port read.
            state.ops.insert(8, VfsRpcOp::Stat);
            state.waiters.insert(8, Arc::new(WaitQueue::new()));
        }

        let mut coalesced = ok_read_frame(7, b"late");
        coalesced.extend_from_slice(&ok_stat_frame(8, 0o100644, 123, 99));
        resp_port.send(&coalesced);

        ch.try_collect_responses();

        let mut state = ch.state.lock();
        assert!(
            !state.responses.contains_key(&7),
            "late response for timed-out request must be dropped"
        );
        assert!(
            !state.ops.contains_key(&7),
            "opcode retained for a timed-out request must be released after its late reply"
        );

        let resp = state.responses.remove(&8).expect("waiting response should remain framed");
        drop(state);

        assert_eq!(resp[0], 0);
        let stat = parse_response_stat(&resp[1..]).unwrap();
        assert_eq!(stat.mode, 0o100644);
        assert_eq!(stat.size, 123);
        assert_eq!(stat.ino, 99);
    }

    #[test]
    fn provider_node_add_waiter_does_not_send_subscribe_rpc() {
        let req_port = make_port(4096);
        let resp_port = make_port(4096);
        let node = ProviderNode {
            handle: 7,
            rpc: Arc::new(ProviderRpc::new(req_port.clone(), resp_port, 0)),
            wait_queue: Arc::new(WaitQueue::new()),
        };

        node.add_waiter(42);

        let mut buf = [0u8; 64];
        assert_eq!(req_port.try_recv(&mut buf), 0);
    }

    #[test]
    fn provider_node_remove_waiter_does_not_send_unsubscribe_rpc() {
        let req_port = make_port(4096);
        let resp_port = make_port(4096);
        let node = ProviderNode {
            handle: 9,
            rpc: Arc::new(ProviderRpc::new(req_port.clone(), resp_port, 0)),
            wait_queue: Arc::new(WaitQueue::new()),
        };

        node.add_waiter(100);
        node.remove_waiter(100);

        let mut buf = [0u8; 64];
        assert_eq!(req_port.try_recv(&mut buf), 0);
    }

    #[test]
    fn provider_readdir_translates_from_exact_stream_offset() {
        let req_port = make_port(4096);
        let resp_port = make_port(4096);
        resp_port.send(&ok_readdir_payload(&[(1, 8, "a"), (2, 8, "b")]));

        let node = ProviderNode {
            handle: 7,
            rpc: Arc::new(ProviderRpc::new(req_port.clone(), resp_port, 0)),
            wait_queue: Arc::new(WaitQueue::new()),
        };

        let mut out = [0u8; 2];
        let n = node.readdir(2, &mut out).unwrap();
        assert_eq!(n, 2);
        assert_eq!(&out, b"b\0");

        let mut req_msg = [0u8; 64];
        let msg_n = req_port.try_recv(&mut req_msg);
        assert!(msg_n >= core::mem::size_of::<VfsRpcReqHeader>() + 20);
        let payload_off = core::mem::size_of::<VfsRpcReqHeader>();
        let sent_index =
            u64::from_le_bytes(req_msg[payload_off + 8..payload_off + 16].try_into().unwrap());
        assert_eq!(sent_index, 0);
    }

    #[test]
    fn provider_readdir_paginates_by_provider_entry_index() {
        let req_port = make_port(4096);
        let resp_port = make_port(4096);
        resp_port.send(&ok_readdir_payload(&[(1, 8, "a"), (2, 8, "b")]));
        resp_port.send(&ok_readdir_payload(&[(3, 8, "c")]));

        let node = ProviderNode {
            handle: 7,
            rpc: Arc::new(ProviderRpc::new(req_port.clone(), resp_port, 0)),
            wait_queue: Arc::new(WaitQueue::new()),
        };

        let mut out = [0u8; 2];
        let n = node.readdir(4, &mut out).unwrap();
        assert_eq!(n, 2);
        assert_eq!(&out, b"c\0");

        let mut first = [0u8; 64];
        let mut second = [0u8; 64];
        let n1 = req_port.try_recv(&mut first);
        let n2 = req_port.try_recv(&mut second);
        assert!(n1 >= core::mem::size_of::<VfsRpcReqHeader>() + 20);
        assert!(n2 >= core::mem::size_of::<VfsRpcReqHeader>() + 20);
        let payload_off = core::mem::size_of::<VfsRpcReqHeader>();
        let first_index =
            u64::from_le_bytes(first[payload_off + 8..payload_off + 16].try_into().unwrap());
        let second_index =
            u64::from_le_bytes(second[payload_off + 8..payload_off + 16].try_into().unwrap());
        assert_eq!(first_index, 0);
        assert_eq!(second_index, 2);
    }
}
