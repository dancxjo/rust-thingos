use alloc::sync::Arc;
use core::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

use abi::errors::SysResult;
use abi::syscall::{PollHandle, poll_flags};
use spin::Mutex;

use super::*;
use crate::sched::blocking::BLOCK_CURRENT_HOOK;
use crate::sched::hooks::CURRENT_TID_HOOK;
use crate::vfs::handle_table::HandleTable;
use crate::vfs::{OpenFlags, VfsNode, VfsStat};

// ── Test nodes ────────────────────────────────────────────────────────────

/// A node that is always readable and writable (POLLIN | POLLOUT).
struct AlwaysReadyNode;
impl VfsNode for AlwaysReadyNode {
    fn read(&self, _: u64, _: &mut [u8]) -> SysResult<usize> {
        Ok(0)
    }
    fn write(&self, _: u64, buf: &[u8]) -> SysResult<usize> {
        Ok(buf.len())
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat { mode: VfsStat::S_IFCHR | 0o666, ..Default::default() })
    }
    // poll() not overridden → returns POLLIN | POLLOUT (default)
}

/// A node whose poll() reports no readiness at all.
struct NeverReadyNode;
impl VfsNode for NeverReadyNode {
    fn read(&self, _: u64, _: &mut [u8]) -> SysResult<usize> {
        Ok(0)
    }
    fn write(&self, _: u64, buf: &[u8]) -> SysResult<usize> {
        Ok(buf.len())
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat { mode: VfsStat::S_IFCHR | 0o666, ..Default::default() })
    }
    fn poll(&self) -> u16 {
        0 // never ready
    }
}

// ── Serialization guard ───────────────────────────────────────────────────
//
// The kernel hooks (`PROCESS_INFO_HOOK`, `CURRENT_TID_HOOK`) are global
// mutable statics.  Tests run concurrently, so we serialize them with a
// spin-mutex guard.  The critical sections are short (non-blocking poll)
// so spinning is acceptable here.

static TEST_POLL_GUARD: spin::Mutex<()> = spin::Mutex::new(());

// Holds the current test's ProcessInfo while inside the critical section.
static TEST_PROCESS_INFO: spin::Mutex<Option<Arc<Mutex<crate::task::ProcessInfo>>>> =
    spin::Mutex::new(None);
static REGISTER_TIMEOUT_CALLS: AtomicUsize = AtomicUsize::new(0);
static UNREGISTER_TIMEOUT_CALLS: AtomicUsize = AtomicUsize::new(0);
static LAST_REGISTERED_TID: AtomicU64 = AtomicU64::new(0);
static LAST_REGISTERED_DEADLINE: AtomicU64 = AtomicU64::new(0);
static INTERRUPT_PENDING: AtomicBool = AtomicBool::new(false);

fn process_info_hook() -> Option<Arc<Mutex<crate::task::ProcessInfo>>> {
    TEST_PROCESS_INFO.lock().clone()
}

fn test_current_tid() -> u64 {
    42
}

fn test_take_interrupt() -> bool {
    INTERRUPT_PENDING.swap(false, Ordering::SeqCst)
}

fn test_register_timeout(tid: u64, wake_tick: u64) {
    REGISTER_TIMEOUT_CALLS.fetch_add(1, Ordering::SeqCst);
    LAST_REGISTERED_TID.store(tid, Ordering::SeqCst);
    LAST_REGISTERED_DEADLINE.store(wake_tick, Ordering::SeqCst);
}

fn test_unregister_timeout(_tid: u64) {
    UNREGISTER_TIMEOUT_CALLS.fetch_add(1, Ordering::SeqCst);
}

fn advance_past_timeout_on_block() {
    let deadline = LAST_REGISTERED_DEADLINE.load(Ordering::SeqCst);
    crate::sched::TICK_COUNT.store(deadline.saturating_add(1), Ordering::SeqCst);
}

// ── Helpers ───────────────────────────────────────────────────────────────

fn make_process_info_with_nodes(
    nodes: &[(u32, Arc<dyn VfsNode>)],
) -> Arc<Mutex<crate::task::ProcessInfo>> {
    let mut handle_table = HandleTable::new();
    for (fd, node) in nodes {
        handle_table
            .insert_at(*fd, node.clone(), OpenFlags::read_write(), "/test".into())
            .expect("insert_at");
    }
    Arc::new(Mutex::new(crate::task::ProcessInfo {
        pid: 1,
        job: crate::task::ProcessLifecycle::new(0, 1),
        unix_compat: crate::task::ProcessUnixCompat::isolated(1, false),
        handle_table,
        ipc_table: crate::ipc::IpcHandleTable::new(),
        namespace: crate::vfs::NamespaceRef::global(),
        cwd: alloc::string::String::from("/"),
        root: alloc::string::String::from("/"),
        exec_path: alloc::string::String::new(),
        authority: crate::task::ProcessAuthority::root(),
        space: crate::task::ProcessAddressSpace::empty(),
        service_loop: None,
    }))
}

/// Run `sys_fs_poll` with `timeout_ms = 0` (non-blocking) over `fds`.
///
/// Acquires `TEST_POLL_GUARD` to serialize concurrent tests that share
/// the global `PROCESS_INFO_HOOK` and `CURRENT_TID_HOOK`.
fn poll_nonblocking(
    pinfo: Arc<Mutex<crate::task::ProcessInfo>>,
    fds: &mut [PollHandle],
) -> SysResult<usize> {
    let _guard = TEST_POLL_GUARD.lock();

    // Install hooks while holding the guard so no other test can clobber
    // them between setup and the actual sys_fs_poll call.
    unsafe {
        CURRENT_TID_HOOK = Some(test_current_tid);
        crate::sched::hooks::PROCESS_INFO_HOOK = Some(process_info_hook);
    }
    TEST_PROCESS_INFO.lock().replace(pinfo);

    // Use the slice's heap address as "user" memory.
    // validate_user_range only rejects NULL and kernel-high addresses.
    let ptr = fds.as_mut_ptr() as usize;
    let nfds = fds.len();
    let res = sys_fs_poll(ptr, nfds, 0 /* non-blocking */);

    // Clean up before releasing the guard.
    unsafe {
        crate::sched::hooks::PROCESS_INFO_HOOK = None;
        CURRENT_TID_HOOK = None;
    }
    TEST_PROCESS_INFO.lock().take();

    res
    // _guard released here
}

fn reset_timeout_hooks() {
    REGISTER_TIMEOUT_CALLS.store(0, Ordering::SeqCst);
    UNREGISTER_TIMEOUT_CALLS.store(0, Ordering::SeqCst);
    LAST_REGISTERED_TID.store(0, Ordering::SeqCst);
    LAST_REGISTERED_DEADLINE.store(0, Ordering::SeqCst);
    INTERRUPT_PENDING.store(false, Ordering::SeqCst);
}

// ── Tests ─────────────────────────────────────────────────────────────────

/// Passing nfds=0 must succeed immediately without touching any state.
#[test]
fn poll_zero_nfds_returns_ok_zero() {
    let res = sys_fs_poll(0, 0, 0);
    assert_eq!(res, Ok(0));
}

/// A single fd backed by an always-ready node → returns 1 with POLLIN set.
#[test]
fn poll_single_ready_fd_returns_one() {
    let node: Arc<dyn VfsNode> = Arc::new(AlwaysReadyNode);
    let pinfo = make_process_info_with_nodes(&[(3, node)]);

    let mut fds = [PollHandle { handle: 3, events: poll_flags::POLLIN, revents: 0 }];

    let n = poll_nonblocking(pinfo, &mut fds).expect("poll should succeed");
    assert_eq!(n, 1, "one fd should be ready");
    assert_ne!(fds[0].revents & poll_flags::POLLIN, 0, "POLLIN should be set");
}

/// Non-blocking poll over a node with poll() == 0 must return 0 immediately.
#[test]
fn poll_nonblocking_returns_zero_when_no_fd_ready() {
    let node: Arc<dyn VfsNode> = Arc::new(NeverReadyNode);
    let pinfo = make_process_info_with_nodes(&[(3, node)]);

    let mut fds = [PollHandle { handle: 3, events: poll_flags::POLLIN, revents: 0 }];

    let n = poll_nonblocking(pinfo, &mut fds).expect("poll should succeed");
    assert_eq!(n, 0, "no fds ready in non-blocking mode");
    assert_eq!(fds[0].revents, 0, "revents must remain 0");
}

/// Multiple ready fds: the return value is the count of fds with revents != 0.
#[test]
fn poll_multiple_ready_fds_returns_correct_count() {
    let ready: Arc<dyn VfsNode> = Arc::new(AlwaysReadyNode);
    let not_ready: Arc<dyn VfsNode> = Arc::new(NeverReadyNode);
    let pinfo = make_process_info_with_nodes(&[
        (3, ready.clone()),
        (4, not_ready.clone()),
        (5, ready.clone()),
    ]);

    let mut fds = [
        PollHandle { handle: 3, events: poll_flags::POLLIN | poll_flags::POLLOUT, revents: 0 },
        PollHandle { handle: 4, events: poll_flags::POLLIN, revents: 0 },
        PollHandle { handle: 5, events: poll_flags::POLLOUT, revents: 0 },
    ];

    let n = poll_nonblocking(pinfo, &mut fds).expect("poll should succeed");
    assert_eq!(n, 2, "two ready fds (indices 0 and 2)");
    assert_ne!(fds[0].revents, 0, "fd 3 should be ready");
    assert_eq!(fds[1].revents, 0, "fd 4 should not be ready");
    assert_ne!(fds[2].revents, 0, "fd 5 should be ready");
}

/// A fd number that doesn't exist in the fd table must yield POLLNVAL.
#[test]
fn poll_invalid_fd_reports_pollnval() {
    let pinfo = make_process_info_with_nodes(&[]);

    let mut fds = [PollHandle {
        handle: 99, // no such fd
        events: poll_flags::POLLIN,
        revents: 0,
    }];

    let n = poll_nonblocking(pinfo, &mut fds).expect("poll should succeed");
    assert_eq!(n, 1, "POLLNVAL counts as a ready entry");
    assert_ne!(fds[0].revents & poll_flags::POLLNVAL, 0, "POLLNVAL should be set for missing fd");
}

/// A negative fd must be silently skipped (revents stays 0).
#[test]
fn poll_negative_fd_is_skipped() {
    let pinfo = make_process_info_with_nodes(&[]);

    let mut fds = [PollHandle { handle: -1, events: poll_flags::POLLIN, revents: 0 }];

    let n = poll_nonblocking(pinfo, &mut fds).expect("poll should succeed");
    assert_eq!(n, 0, "negative fd is silently skipped");
    assert_eq!(fds[0].revents, 0, "revents must stay 0 for negative fd");
}

/// Mixed poll: pipe (ready), port (not ready), VFS file (ready).
/// Exercises the multi-fd path with heterogeneous node types.
#[test]
fn poll_mixed_pipe_port_vfsfile() {
    let (pipe_r, pipe_w) = crate::ipc::pipe::create_fd_pair(0, false);
    // Write data so the read end is POLLIN-ready.
    pipe_w.write(0, b"hello").expect("write to pipe");

    let always: Arc<dyn VfsNode> = Arc::new(AlwaysReadyNode);
    let never: Arc<dyn VfsNode> = Arc::new(NeverReadyNode);

    let pinfo = make_process_info_with_nodes(&[
        (3, pipe_r.clone()),
        (4, never.clone()),
        (5, always.clone()),
    ]);

    let mut fds = [
        PollHandle { handle: 3, events: poll_flags::POLLIN, revents: 0 },
        PollHandle { handle: 4, events: poll_flags::POLLIN, revents: 0 },
        PollHandle { handle: 5, events: poll_flags::POLLIN | poll_flags::POLLOUT, revents: 0 },
    ];

    let n = poll_nonblocking(pinfo, &mut fds).expect("poll should succeed");
    assert_eq!(n, 2, "pipe-read and always-ready should fire; never-ready should not");
    assert_ne!(fds[0].revents & poll_flags::POLLIN, 0, "pipe read end has data → POLLIN");
    assert_eq!(fds[1].revents, 0, "NeverReadyNode → no events");
    assert_ne!(
        fds[2].revents & (poll_flags::POLLIN | poll_flags::POLLOUT),
        0,
        "AlwaysReadyNode → ready"
    );
}

#[test]
fn poll_blocking_returns_eintr_when_interrupted() {
    let _guard = TEST_POLL_GUARD.lock();
    let node: Arc<dyn VfsNode> = Arc::new(NeverReadyNode);
    let pinfo = make_process_info_with_nodes(&[(3, node)]);
    let mut fds = [PollHandle { handle: 3, events: poll_flags::POLLIN, revents: 0 }];
    let ptr = fds.as_mut_ptr() as usize;

    reset_timeout_hooks();
    INTERRUPT_PENDING.store(true, Ordering::SeqCst);
    unsafe {
        CURRENT_TID_HOOK = Some(test_current_tid);
        crate::sched::hooks::PROCESS_INFO_HOOK = Some(process_info_hook);
        crate::sched::hooks::TAKE_PENDING_INTERRUPT_HOOK = Some(test_take_interrupt);
    }
    TEST_PROCESS_INFO.lock().replace(pinfo);

    let res = sys_fs_poll(ptr, fds.len(), usize::MAX);

    unsafe {
        crate::sched::hooks::TAKE_PENDING_INTERRUPT_HOOK = None;
        crate::sched::hooks::PROCESS_INFO_HOOK = None;
        CURRENT_TID_HOOK = None;
    }
    TEST_PROCESS_INFO.lock().take();
    INTERRUPT_PENDING.store(false, Ordering::SeqCst);

    assert_eq!(res, Err(Errno::EINTR));
    assert_eq!(fds[0].revents, 0);
}

#[test]
fn mode_denies_access_without_permission_bits() {
    assert!(!mode_allows_requested_access(0o000, true, false));
    assert!(!mode_allows_requested_access(0o000, false, true));
    assert!(!mode_allows_requested_access(0o000, true, true));
}

#[test]
fn mode_allows_access_with_matching_bits() {
    assert!(mode_allows_requested_access(0o444, true, false));
    assert!(mode_allows_requested_access(0o222, false, true));
    assert!(mode_allows_requested_access(0o666, true, true));
}

// ── sys_fs_lstat input validation ─────────────────────────────────────────

/// Zero-length path must return EINVAL immediately.
#[test]
fn lstat_zero_path_len_returns_einval() {
    let result = sys_fs_lstat(0x1000, 0, 0x2000);
    assert_eq!(result, Err(Errno::EINVAL));
}

/// Path length exceeding 4096 bytes must return EINVAL immediately.
#[test]
fn lstat_path_too_long_returns_einval() {
    let result = sys_fs_lstat(0x1000, 4097, 0x2000);
    assert_eq!(result, Err(Errno::EINVAL));
}

// ── sys_fs_seek – signed offset semantics ─────────────────────────────────

/// A file-like node with a fixed size for use in seek tests.
struct SizedFileNode(u64);

impl VfsNode for SizedFileNode {
    fn read(&self, _: u64, _: &mut [u8]) -> SysResult<usize> {
        Ok(0)
    }
    fn write(&self, _: u64, buf: &[u8]) -> SysResult<usize> {
        Ok(buf.len())
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat { mode: VfsStat::S_IFREG | 0o644, size: self.0, ..Default::default() })
    }
}

/// Run `sys_fs_seek` with the given process info, fd, offset, and whence.
///
/// Acquires `TEST_POLL_GUARD` so seek tests don't race with poll tests over
/// the shared `PROCESS_INFO_HOOK` / `CURRENT_TID_HOOK` globals.
fn seek_with_process_info(
    pinfo: Arc<Mutex<crate::task::ProcessInfo>>,
    fd: usize,
    offset: usize,
    whence: usize,
) -> SysResult<usize> {
    let _guard = TEST_POLL_GUARD.lock();
    unsafe {
        CURRENT_TID_HOOK = Some(test_current_tid);
        crate::sched::hooks::PROCESS_INFO_HOOK = Some(process_info_hook);
    }
    TEST_PROCESS_INFO.lock().replace(pinfo);

    let res = sys_fs_seek(fd, offset, whence);

    unsafe {
        crate::sched::hooks::PROCESS_INFO_HOOK = None;
        CURRENT_TID_HOOK = None;
    }
    TEST_PROCESS_INFO.lock().take();
    res
}

/// SEEK_SET to a positive offset must succeed and return that offset.
#[test]
fn seek_set_positive_returns_new_offset() {
    let node: Arc<dyn VfsNode> = Arc::new(SizedFileNode(64));
    let pinfo = make_process_info_with_nodes(&[(3, node)]);
    let result = seek_with_process_info(pinfo, 3, 16, 0 /* SEEK_SET */);
    assert_eq!(result, Ok(16));
}

/// SEEK_SET with a negative signed value must return EINVAL.
#[test]
fn seek_set_negative_returns_einval() {
    let node: Arc<dyn VfsNode> = Arc::new(SizedFileNode(64));
    let pinfo = make_process_info_with_nodes(&[(3, node)]);
    // Pass -1i64 as usize (two's complement).
    let neg1 = (-1i64) as usize;
    let result = seek_with_process_info(pinfo, 3, neg1, 0 /* SEEK_SET */);
    assert_eq!(result, Err(Errno::EINVAL));
}

/// SEEK_CUR with a positive offset advances the position.
#[test]
fn seek_cur_positive_advances_position() {
    let node: Arc<dyn VfsNode> = Arc::new(SizedFileNode(64));
    let pinfo = make_process_info_with_nodes(&[(3, node)]);
    // First seek to 10.
    seek_with_process_info(Arc::clone(&pinfo), 3, 10, 0 /* SEEK_SET */).unwrap();
    // Then advance by 5 → expected offset = 15.
    let result = seek_with_process_info(pinfo, 3, 5, 1 /* SEEK_CUR */);
    assert_eq!(result, Ok(15));
}

/// SEEK_CUR with a negative offset moves the position backward.
#[test]
fn seek_cur_negative_moves_backward() {
    let node: Arc<dyn VfsNode> = Arc::new(SizedFileNode(64));
    let pinfo = make_process_info_with_nodes(&[(3, node)]);
    // Seek to 20 first.
    seek_with_process_info(Arc::clone(&pinfo), 3, 20, 0 /* SEEK_SET */).unwrap();
    // Seek back by 4 → expected offset = 16.
    let neg4 = (-4i64) as usize;
    let result = seek_with_process_info(pinfo, 3, neg4, 1 /* SEEK_CUR */);
    assert_eq!(result, Ok(16));
}

/// SEEK_CUR with a negative offset that would go before the start must
/// return EINVAL.
#[test]
fn seek_cur_negative_before_start_returns_einval() {
    let node: Arc<dyn VfsNode> = Arc::new(SizedFileNode(64));
    let pinfo = make_process_info_with_nodes(&[(3, node)]);
    // Offset is 0 (default); seek back by 1 → would be -1.
    let neg1 = (-1i64) as usize;
    let result = seek_with_process_info(pinfo, 3, neg1, 1 /* SEEK_CUR */);
    assert_eq!(result, Err(Errno::EINVAL));
}

/// SEEK_END with offset 0 must return the file size.
#[test]
fn seek_end_zero_returns_file_size() {
    let node: Arc<dyn VfsNode> = Arc::new(SizedFileNode(64));
    let pinfo = make_process_info_with_nodes(&[(3, node)]);
    let result = seek_with_process_info(pinfo, 3, 0, 2 /* SEEK_END */);
    assert_eq!(result, Ok(64));
}

/// SEEK_END with a negative offset seeks from the end of the file.
#[test]
fn seek_end_negative_seeks_from_end() {
    let node: Arc<dyn VfsNode> = Arc::new(SizedFileNode(64));
    let pinfo = make_process_info_with_nodes(&[(3, node)]);
    // -4 from end of 64-byte file → offset 60.
    let neg4 = (-4i64) as usize;
    let result = seek_with_process_info(pinfo, 3, neg4, 2 /* SEEK_END */);
    assert_eq!(result, Ok(60));
}

/// SEEK_END with a positive offset seeks past the end (sparse / hole).
#[test]
fn seek_end_positive_seeks_past_end() {
    let node: Arc<dyn VfsNode> = Arc::new(SizedFileNode(64));
    let pinfo = make_process_info_with_nodes(&[(3, node)]);
    // +4 past the end of a 64-byte file → offset 68.
    let result = seek_with_process_info(pinfo, 3, 4, 2 /* SEEK_END */);
    assert_eq!(result, Ok(68));
}

/// SEEK_END with a negative offset that would precede the start must return
/// EINVAL.
#[test]
fn seek_end_negative_before_start_returns_einval() {
    let node: Arc<dyn VfsNode> = Arc::new(SizedFileNode(4));
    let pinfo = make_process_info_with_nodes(&[(3, node)]);
    // -8 from end of 4-byte file → would be -4.
    let neg8 = (-8i64) as usize;
    let result = seek_with_process_info(pinfo, 3, neg8, 2 /* SEEK_END */);
    assert_eq!(result, Err(Errno::EINVAL));
}

/// An unknown whence value must return EINVAL.
#[test]
fn seek_invalid_whence_returns_einval() {
    let node: Arc<dyn VfsNode> = Arc::new(SizedFileNode(64));
    let pinfo = make_process_info_with_nodes(&[(3, node)]);
    let result = seek_with_process_info(pinfo, 3, 0, 99 /* invalid */);
    assert_eq!(result, Err(Errno::EINVAL));
}

// ── sys_fs_getcwd – CWD reading ───────────────────────────────────────────

/// Helper: run `sys_fs_getcwd` with the given process info and a stack buffer.
fn getcwd_with_process_info(pinfo: Arc<Mutex<crate::task::ProcessInfo>>) -> SysResult<usize> {
    let _guard = TEST_POLL_GUARD.lock();
    unsafe {
        CURRENT_TID_HOOK = Some(test_current_tid);
        crate::sched::hooks::PROCESS_INFO_HOOK = Some(process_info_hook);
    }
    TEST_PROCESS_INFO.lock().replace(pinfo);

    // Pass buf_ptr=0, buf_len=0 so the handler only returns the length
    // without trying to copy to userspace (avoids validate_user_range).
    let res = sys_fs_getcwd(0, 0);

    unsafe {
        crate::sched::hooks::PROCESS_INFO_HOOK = None;
        CURRENT_TID_HOOK = None;
    }
    TEST_PROCESS_INFO.lock().take();
    res
}

/// A freshly spawned process starts with CWD = "/".
/// getcwd must return the byte length of "/" (1).
#[test]
fn getcwd_returns_root_for_default_process() {
    let pinfo = make_process_info_with_nodes(&[]);
    let len = getcwd_with_process_info(pinfo).expect("getcwd should succeed");
    assert_eq!(len, 1, "default CWD is '/' – length should be 1");
}

/// After the CWD is changed in the ProcessInfo struct, getcwd must report
/// the updated value.
#[test]
fn getcwd_reflects_updated_cwd() {
    let pinfo = make_process_info_with_nodes(&[]);
    pinfo.lock().cwd = alloc::string::String::from("/tmp");
    let len = getcwd_with_process_info(pinfo).expect("getcwd should succeed");
    assert_eq!(len, 4, "'/tmp' has 4 bytes");
}

// ── resolve_path – relative path resolution ───────────────────────────────

/// Helper: call `resolve_path` with the given process info's CWD set.
fn resolve_relative(cwd: &str, rel_path: &str) -> SysResult<alloc::string::String> {
    let _guard = TEST_POLL_GUARD.lock();
    unsafe {
        CURRENT_TID_HOOK = Some(test_current_tid);
        crate::sched::hooks::PROCESS_INFO_HOOK = Some(process_info_hook);
    }
    let pinfo = make_process_info_with_nodes(&[]);
    pinfo.lock().cwd = alloc::string::String::from(cwd);
    TEST_PROCESS_INFO.lock().replace(pinfo);

    let res = resolve_path(rel_path);

    unsafe {
        crate::sched::hooks::PROCESS_INFO_HOOK = None;
        CURRENT_TID_HOOK = None;
    }
    TEST_PROCESS_INFO.lock().take();
    res
}

fn run_with_process_info<R>(
    pinfo: Arc<Mutex<crate::task::ProcessInfo>>,
    f: impl FnOnce() -> R,
) -> R {
    struct HookCleanupGuard;
    impl Drop for HookCleanupGuard {
        fn drop(&mut self) {
            unsafe {
                crate::sched::hooks::PROCESS_INFO_HOOK = None;
                CURRENT_TID_HOOK = None;
            }
            TEST_PROCESS_INFO.lock().take();
        }
    }

    let _guard = TEST_POLL_GUARD.lock();
    let _cleanup = HookCleanupGuard;
    unsafe {
        CURRENT_TID_HOOK = Some(test_current_tid);
        crate::sched::hooks::PROCESS_INFO_HOOK = Some(process_info_hook);
    }
    TEST_PROCESS_INFO.lock().replace(pinfo);
    f()
}

/// A relative path is joined with the CWD and normalised.
#[test]
fn resolve_path_relative_joined_with_cwd() {
    let result = resolve_relative("/home/user", "docs/readme.txt").unwrap();
    assert_eq!(result, "/home/user/docs/readme.txt");
}

/// A relative path with ".." components is resolved correctly.
#[test]
fn resolve_path_relative_with_dotdot() {
    let result = resolve_relative("/home/user/projects", "../docs").unwrap();
    assert_eq!(result, "/home/user/docs");
}

/// A relative "." refers to the CWD itself.
#[test]
fn resolve_path_relative_dot_refers_to_cwd() {
    let result = resolve_relative("/tmp", ".").unwrap();
    assert_eq!(result, "/tmp");
}

/// An absolute path is left unchanged (CWD is irrelevant).
#[test]
fn resolve_path_absolute_ignores_cwd() {
    let result = resolve_relative("/some/cwd", "/etc/hosts").unwrap();
    assert_eq!(result, "/etc/hosts");
}

/// CWD with a trailing slash is handled without producing double slashes.
#[test]
fn resolve_path_cwd_with_trailing_slash() {
    let result = resolve_relative("/tmp/", "file.txt").unwrap();
    assert_eq!(result, "/tmp/file.txt");
}

#[test]
fn namespace_mount_privilege_denies_global_without_capability() {
    let pinfo = make_process_info_with_nodes(&[]);
    {
        let mut pi = pinfo.lock();
        pi.namespace = crate::vfs::NamespaceRef::global();
        pi.authority = crate::task::ProcessAuthority { uid: 1000, gid: 1000, capability_mask: 0 };
    }
    let res = run_with_process_info(pinfo, require_namespace_mount_privilege);
    assert_eq!(res, Err(Errno::EPERM));
}

#[test]
fn namespace_mount_privilege_allows_global_with_capability() {
    let pinfo = make_process_info_with_nodes(&[]);
    {
        let mut pi = pinfo.lock();
        pi.namespace = crate::vfs::NamespaceRef::global();
        pi.authority = crate::task::ProcessAuthority {
            uid: 1000,
            gid: 1000,
            capability_mask: crate::authority::bridge::CAP_MOUNT,
        };
    }
    let res = run_with_process_info(pinfo, require_namespace_mount_privilege);
    assert_eq!(res, Ok(()));
}

#[test]
fn namespace_mount_privilege_allows_isolated_without_capability() {
    let pinfo = make_process_info_with_nodes(&[]);
    {
        let mut pi = pinfo.lock();
        pi.namespace = crate::vfs::NamespaceRef::isolated();
        pi.authority = crate::task::ProcessAuthority { uid: 1000, gid: 1000, capability_mask: 0 };
    }
    let res = run_with_process_info(pinfo, require_namespace_mount_privilege);
    assert_eq!(res, Ok(()));
}

// ── sys_fs_chdir – input validation ──────────────────────────────────────

/// A zero-length path must return EINVAL immediately.
#[test]
fn chdir_zero_path_len_returns_einval() {
    let result = sys_fs_chdir(0x1000, 0);
    assert_eq!(result, Err(Errno::EINVAL));
}

/// A path longer than 4096 bytes must return EINVAL immediately.
#[test]
fn chdir_path_too_long_returns_einval() {
    let result = sys_fs_chdir(0x1000, 4097);
    assert_eq!(result, Err(Errno::EINVAL));
}

#[test]
fn poll_unregisters_timeout_after_blocking_timeout() {
    let _guard = TEST_POLL_GUARD.lock();
    let node: Arc<dyn VfsNode> = Arc::new(NeverReadyNode);
    let pinfo = make_process_info_with_nodes(&[(3, node)]);
    let mut fds = [PollHandle { handle: 3, events: poll_flags::POLLIN, revents: 0 }];

    reset_timeout_hooks();
    crate::sched::TICK_COUNT.store(100, Ordering::SeqCst);

    unsafe {
        CURRENT_TID_HOOK = Some(test_current_tid);
        crate::sched::hooks::PROCESS_INFO_HOOK = Some(process_info_hook);
        crate::sched::hooks::TAKE_PENDING_INTERRUPT_HOOK = Some(test_take_interrupt);
        crate::sched::hooks::REGISTER_TIMEOUT_WAKE_HOOK = Some(test_register_timeout);
        crate::sched::hooks::UNREGISTER_TIMEOUT_WAKE_HOOK = Some(test_unregister_timeout);
    }
    BLOCK_CURRENT_HOOK.store(advance_past_timeout_on_block as *mut (), Ordering::SeqCst);
    TEST_PROCESS_INFO.lock().replace(pinfo);

    let res = sys_fs_poll(fds.as_mut_ptr() as usize, fds.len(), 1);

    unsafe {
        crate::sched::hooks::UNREGISTER_TIMEOUT_WAKE_HOOK = None;
        crate::sched::hooks::REGISTER_TIMEOUT_WAKE_HOOK = None;
        crate::sched::hooks::TAKE_PENDING_INTERRUPT_HOOK = None;
        crate::sched::hooks::PROCESS_INFO_HOOK = None;
        CURRENT_TID_HOOK = None;
    }
    BLOCK_CURRENT_HOOK.store(core::ptr::null_mut(), Ordering::SeqCst);
    TEST_PROCESS_INFO.lock().take();

    assert_eq!(res, Ok(0), "poll should time out cleanly");
    assert_eq!(REGISTER_TIMEOUT_CALLS.load(Ordering::SeqCst), 1);
    assert_eq!(UNREGISTER_TIMEOUT_CALLS.load(Ordering::SeqCst), 1);
    assert_eq!(LAST_REGISTERED_TID.load(Ordering::SeqCst), 42);
}
