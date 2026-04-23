#![unstable(feature = "thingos_pal", issue = "none")]
//! ThingOS process implementation for `std::process`.
//!
//! # Spawn contract
//!
//! `SYS_SPAWN_PROCESS_EX` (0x1006) launches child processes.
//!
//! ## Request (SpawnProcessExReq, abi/src/types/system.rs)
//!
//!  - name_ptr/name_len : Executable path (UTF-8, not null-terminated)
//!  - argv_ptr/argv_len : count:u32, then (len:u32, bytes)xN
//!  - env_ptr/env_len   : count:u32, then (klen, key, vlen, val)xN
//!  - stdin/stdout/stderr_mode : 0=inherit, 1=null, 2=pipe
//!
//! ## Response (SpawnProcessExResp)
//!
//!  - child_pid   : use with SYS_WAITPID
//!  - stdin_pipe  : parent WRITE fd when mode==pipe (else 0)
//!  - stdout_pipe : parent READ fd when mode==pipe (else 0)
//!  - stderr_pipe : parent READ fd when mode==pipe (else 0)
//!
//! When `Command::current_dir()` is set, the cwd bytes are forwarded via the
//! `cwd_ptr`/`cwd_len` fields of `SpawnProcessExReq`.  The kernel sets the
//! child's working directory atomically at spawn time.  If cwd is not set,
//! the child inherits the parent's cwd.
//!
//! ## FD Remapping
//!
//! When passing an existing `ChildPipe` (from a previous child's stdout/stderr)
//! to a new process's stdin/stdout/stderr, the FD is passed via the `fd_remap`
//! table. The kernel applies `dup2` from the parent's FD to the child's
//! target FD during spawn.

use super::env::{CommandEnv, CommandEnvs};
pub use crate::ffi::OsString as EnvKey;
use crate::ffi::{OsStr, OsString};
use crate::sys::fs::{FileDesc, File};
use crate::num::NonZero;
use crate::path::Path;
use crate::process::StdioPipes;
use crate::sys::abi::{FdRemap, SpawnProcessExReq, SpawnProcessExResp};
use crate::sys::pal::raw_syscall6;
use crate::sys::thingos_syscall_numbers::{
    SYS_FS_CLOSE, SYS_FS_FCNTL, SYS_FS_POLL, SYS_SPAWN_PROCESS_EX, SYS_TASK_KILL, SYS_TASK_WAIT, SYS_WAITPID,
    fcntl_cmd, poll_flags, vfs_flags,
};
use crate::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};
use crate::sys::{AsInner, FromInner, IntoInner};
use crate::sys::pipe::Pipe;
use crate::{fmt, io};

/// waitpid WNOHANG: return immediately if no child has exited yet.
const WNOHANG: usize = 1;

/// stdio mode constants (abi/src/types/system.rs `stdio_mode`)
mod stdio_mode {
    pub const INHERIT: u32 = 0;
    pub const NULL: u32 = 1;
    pub const PIPE: u32 = 2;
}

#[inline]
fn cvt(ret: isize) -> crate::io::Result<usize> {
    if ret < 0 { Err(crate::io::Error::from_raw_os_error((-ret) as i32)) } else { Ok(ret as usize) }
}

const F_GETFL: u32 = fcntl_cmd::F_GETFL;
const F_SETFL: u32 = fcntl_cmd::F_SETFL;
const O_NONBLOCK: u32 = vfs_flags::O_NONBLOCK;

const POLLIN: u16 = poll_flags::POLLIN;
const POLLERR: u16 = poll_flags::POLLERR;
const POLLHUP: u16 = poll_flags::POLLHUP;

#[repr(C)]
struct PollFd {
    fd: i32,
    events: u16,
    revents: u16,
}

// ── ABI structs (must match abi/src/types/system.rs) ─────────────────────────

// ── Serialization helpers ─────────────────────────────────────────────────────

fn serialize_argv(args: &[OsString]) -> crate::vec::Vec<u8> {
    let mut blob = crate::vec::Vec::new();
    blob.extend_from_slice(&(args.len() as u32).to_le_bytes());
    for arg in args {
        let bytes = arg.as_encoded_bytes();
        blob.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        blob.extend_from_slice(bytes);
    }
    blob
}

fn serialize_env(env: &crate::collections::BTreeMap<EnvKey, OsString>) -> crate::vec::Vec<u8> {
    let mut blob = crate::vec::Vec::new();
    blob.extend_from_slice(&(env.len() as u32).to_le_bytes());
    for (k, v) in env {
        let kb = k.as_encoded_bytes();
        let vb = v.as_encoded_bytes();
        blob.extend_from_slice(&(kb.len() as u32).to_le_bytes());
        blob.extend_from_slice(kb);
        blob.extend_from_slice(&(vb.len() as u32).to_le_bytes());
        blob.extend_from_slice(vb);
    }
    blob
}

// ── Stdio ─────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum Stdio {
    Inherit,
    Null,
    MakePipe,
    InheritPipe(ChildPipe),
    InheritFile(File),
    Fd(FileDesc),
    ParentStdout,
    ParentStderr,
}

impl Stdio {
    fn mode(&self) -> u32 {
        match self {
            Stdio::Inherit => stdio_mode::INHERIT,
            Stdio::Null => stdio_mode::NULL,
            Stdio::MakePipe => stdio_mode::PIPE,
            Stdio::InheritFile(_) | Stdio::InheritPipe(_) | Stdio::Fd(_) | Stdio::ParentStdout | Stdio::ParentStderr => stdio_mode::INHERIT,
        }
    }
}

#[derive(Debug)]
pub struct ChildPipe(Pipe);
impl ChildPipe {
    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
    pub fn read_buf(&self, buf: io::BorrowedCursor<'_>) -> io::Result<()> {
        self.0.read_buf(buf)
    }
    pub fn read_vectored(&self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        self.0.read_vectored(bufs)
    }
    pub fn is_read_vectored(&self) -> bool {
        self.0.is_read_vectored()
    }
    pub fn read_to_end(&self, buf: &mut crate::vec::Vec<u8>) -> io::Result<usize> {
        self.0.read_to_end(buf)
    }
    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }
    pub fn write_vectored(&self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        self.0.write_vectored(bufs)
    }
    pub fn is_write_vectored(&self) -> bool {
        self.0.is_write_vectored()
    }
}

impl AsRawFd for ChildPipe {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

impl FromRawFd for ChildPipe {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        ChildPipe(unsafe { Pipe::from_raw_fd(fd) })
    }
}

impl IntoRawFd for ChildPipe {
    fn into_raw_fd(self) -> RawFd {
        self.0.into_raw_fd()
    }
}

impl AsFd for ChildPipe {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl FromInner<Pipe> for ChildPipe {
    fn from_inner(pipe: Pipe) -> Self {
        ChildPipe(pipe)
    }
}



impl FromInner<OwnedFd> for ChildPipe {
    fn from_inner(owned_fd: OwnedFd) -> Self {
        ChildPipe(Pipe::from_inner(owned_fd))
    }
}

impl FromInner<u32> for ChildPipe {
    fn from_inner(fd: u32) -> Self {
        ChildPipe(Pipe::from_inner(fd))
    }
}

impl crate::sys::IntoInner<crate::os::fd::OwnedFd> for ChildPipe {
    fn into_inner(self) -> crate::os::fd::OwnedFd {
        self.0.into_inner()
    }
}
impl From<ChildPipe> for Stdio {
    fn from(pipe: ChildPipe) -> Stdio {
        Stdio::InheritPipe(pipe)
    }
}

impl From<io::Stdout> for Stdio {
    fn from(_: io::Stdout) -> Stdio {
        Stdio::ParentStdout
    }
}

impl From<io::Stderr> for Stdio {
    fn from(_: io::Stderr) -> Stdio {
        Stdio::ParentStderr
    }
}

impl From<File> for Stdio {
    fn from(file: File) -> Stdio {
        Stdio::InheritFile(file)
    }
}

// ── Command ───────────────────────────────────────────────────────────────────

pub struct Command {
    program: OsString,
    args: crate::vec::Vec<OsString>,
    env: CommandEnv,
    cwd: Option<OsString>,
    stdin: Option<Stdio>,
    stdout: Option<Stdio>,
    stderr: Option<Stdio>,
}

impl Command {
    pub fn new(program: &OsStr) -> Command {
        Command {
            program: program.to_owned(),
            args: crate::vec![program.to_owned()],
            env: Default::default(),
            cwd: None,
            stdin: None,
            stdout: None,
            stderr: None,
        }
    }

    pub fn arg(&mut self, arg: &OsStr) {
        self.args.push(arg.to_owned());
    }

    pub fn env_mut(&mut self) -> &mut CommandEnv {
        &mut self.env
    }

    pub fn cwd(&mut self, dir: &OsStr) {
        self.cwd = Some(dir.to_owned());
    }

    pub fn stdin(&mut self, stdin: Stdio) {
        self.stdin = Some(stdin);
    }
    pub fn stdout(&mut self, stdout: Stdio) {
        self.stdout = Some(stdout);
    }
    pub fn stderr(&mut self, stderr: Stdio) {
        self.stderr = Some(stderr);
    }

    // ── Unix-specific extension stubs ─────────────────────────────────────────

    pub fn uid(&mut self, _id: u32) {
        // Ignored on ThingOS
    }

    pub fn gid(&mut self, _id: u32) {
        // Ignored on ThingOS
    }

    pub fn groups(&mut self, _groups: &[u32]) {
        // Ignored on ThingOS
    }

    pub fn pre_exec(&mut self, _f: Box<dyn FnMut() -> crate::io::Result<()> + Send + Sync + 'static>) {
        // Ignored on ThingOS
    }

    pub fn exec(&mut self, _default: Stdio) -> crate::io::Error {
        crate::io::const_error!(crate::io::ErrorKind::Unsupported, "exec not supported on ThingOS")
    }

    pub fn set_arg_0<S>(&mut self, _arg: S)
    where
        S: AsRef<OsStr>,
    {
        // Ignored
    }

    pub fn chroot<P: AsRef<Path>>(&mut self, _dir: P) {
        // Ignored
    }

    pub fn setsid(&mut self, _setsid: bool) {
        // Ignored
    }

    pub fn process_group(&mut self, _pgroup: i32) {
        // Ignored on ThingOS
    }

    pub fn pgroup(&mut self, _pgroup: i32) {
        // Ignored on ThingOS
    }


    pub fn get_envs(&self) -> CommandEnvs<'_> {
        self.env.iter()
    }

    pub fn get_cwd(&self) -> Option<&OsStr> {
        self.cwd.as_deref()
    }

    pub fn get_uid(&self) -> Option<u32> {
        None
    }

    pub fn get_gid(&self) -> Option<u32> {
        None
    }

    pub fn get_groups(&self) -> Option<&[u32]> {
        None
    }


    pub fn get_program(&self) -> &OsStr {
        &self.program
    }

    pub fn get_args(&self) -> CommandArgs<'_> {
        let mut iter = self.args.iter();
        iter.next();
        CommandArgs { iter }
    }

    pub fn get_env_clear(&self) -> bool {
        self.env.does_clear()
    }

    pub fn get_current_dir(&self) -> Option<&Path> {
        self.cwd.as_ref().map(|cs| Path::new(cs))
    }

    pub fn spawn(
        &mut self,
        default: Stdio,
        _needs_stdin: bool,
    ) -> crate::io::Result<(Process, StdioPipes)> {
        let mut fd_remaps = crate::vec::Vec::new();

        let mut resolve_mode = |stdio: Option<&Stdio>, dst_fd: u32| -> u32 {
            let s = stdio.unwrap_or(&default);
            match s {
                Stdio::InheritPipe(p) => {
                    fd_remaps.push(FdRemap { src_fd: p.as_raw_fd() as u32 as u64, dst_fd: dst_fd as u64 });
                    stdio_mode::INHERIT
                }
                Stdio::InheritFile(f) => {
                    fd_remaps.push(FdRemap { src_fd: f.as_raw_fd() as u32 as u64, dst_fd: dst_fd as u64 });
                    stdio_mode::INHERIT
                }
                Stdio::Fd(f) => {
                    fd_remaps.push(FdRemap { src_fd: f.as_raw_fd() as u32 as u64, dst_fd: dst_fd as u64 });
                    stdio_mode::INHERIT
                }
                Stdio::ParentStdout => {
                    fd_remaps.push(FdRemap { src_fd: 1, dst_fd: dst_fd as u64 });
                    stdio_mode::INHERIT
                }
                Stdio::ParentStderr => {
                    fd_remaps.push(FdRemap { src_fd: 2, dst_fd: dst_fd as u64 });
                    stdio_mode::INHERIT
                }
                _ => s.mode(),
            }
        };

        let stdin_mode = resolve_mode(self.stdin.as_ref(), 0);
        let stdout_mode = resolve_mode(self.stdout.as_ref(), 1);
        let stderr_mode = resolve_mode(self.stderr.as_ref(), 2);

        let argv_blob = serialize_argv(&self.args);
        // Resolve the full child environment (current env + any overrides).
        let full_env = self.env.capture();
        let env_blob = serialize_env(&full_env);
        let name_bytes = self.program.as_encoded_bytes();

        // Encode optional cwd override.
        let cwd_bytes = self.cwd.as_ref().map(|c| c.as_encoded_bytes());

        let req = SpawnProcessExReq {
            name_ptr: name_bytes.as_ptr() as u64,
            name_len: name_bytes.len() as u32,
            argv_ptr: argv_blob.as_ptr() as u64,
            argv_len: argv_blob.len() as u32,
            env_ptr: if env_blob.is_empty() { 0 } else { env_blob.as_ptr() as u64 },
            env_len: env_blob.len() as u32,
            stdin_mode,
            stdout_mode,
            stderr_mode,
            cwd_ptr: cwd_bytes.map_or(0, |b| b.as_ptr() as u64),
            cwd_len: cwd_bytes.map_or(0, |b| b.len() as u32),
            fd_remap_ptr: if fd_remaps.is_empty() { 0 } else { fd_remaps.as_ptr() as u64 },
            fd_remap_len: fd_remaps.len() as u32,
            ..Default::default()
        };

        let mut resp = SpawnProcessExResp::default();
        let ret = unsafe {
            raw_syscall6(
                SYS_SPAWN_PROCESS_EX,
                &req as *const SpawnProcessExReq as usize,
                &mut resp as *mut SpawnProcessExResp as usize,
                0,
                0,
                0,
                0,
            )
        };
        cvt(ret)?;

        // SAFETY: kernel guarantees these fds are valid pipe ends.
        let stdin_pipe = if stdin_mode == stdio_mode::PIPE && resp.stdin_pipe != 0 {
            Some(ChildPipe::from_inner(resp.stdin_pipe as u32))
        } else {
            None
        };
        let stdout_pipe = if stdout_mode == stdio_mode::PIPE && resp.stdout_pipe != 0 {
            Some(ChildPipe::from_inner(resp.stdout_pipe as u32))
        } else {
            None
        };
        let stderr_pipe = if stderr_mode == stdio_mode::PIPE && resp.stderr_pipe != 0 {
            Some(ChildPipe::from_inner(resp.stderr_pipe as u32))
        } else {
            None
        };

        let process = Process { pid: resp.child_pid, tid: resp.child_tid, status: None };
        let pipes = StdioPipes {
            stdin: stdin_pipe,
            stdout: stdout_pipe,
            stderr: stderr_pipe,
        };
        Ok((process, pipes))
    }
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref cwd) = self.cwd {
            write!(f, "cd {cwd:?} && ")?;
        }
        write!(f, "{:?}", self.args[0])?;
        for arg in &self.args[1..] {
            write!(f, " {:?}", arg)?;
        }
        Ok(())
    }
}

// ── CommandArgs ───────────────────────────────────────────────────────────────

pub struct CommandArgs<'a> {
    iter: crate::slice::Iter<'a, OsString>,
}

impl<'a> Iterator for CommandArgs<'a> {
    type Item = &'a OsStr;
    fn next(&mut self) -> Option<&'a OsStr> {
        self.iter.next().map(|os| &**os)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for CommandArgs<'a> {
    fn len(&self) -> usize {
        self.iter.len()
    }
    fn is_empty(&self) -> bool {
        self.iter.is_empty()
    }
}

impl<'a> fmt::Debug for CommandArgs<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter.clone()).finish()
    }
}

// ── ExitStatus ────────────────────────────────────────────────────────────────

/// Child process exit status.  ThingOS stores the raw exit(code) as an i32.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct ExitStatus(i32);

impl ExitStatus {
    pub fn exit_ok(&self) -> Result<(), ExitStatusError> {
        if self.0 == 0 {
            Ok(())
        } else {
            Err(ExitStatusError(NonZero::new(self.0).expect("non-zero exit code")))
        }
    }
    pub fn code(&self) -> Option<i32> {
        Some(self.0)
    }

    pub fn signal(&self) -> Option<i32> { None }
    pub fn core_dumped(&self) -> bool { false }
    pub fn stopped_signal(&self) -> Option<i32> { None }
    pub fn continued(&self) -> bool { false }
    pub fn into_raw(&self) -> i32 { self.0 }
}

impl Default for ExitStatus {
    fn default() -> Self {
        ExitStatus(0)
    }
}

impl fmt::Display for ExitStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "exit status: {}", self.0)
    }
}

// ── ExitStatusError ───────────────────────────────────────────────────────────

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct ExitStatusError(NonZero<i32>);

impl Into<ExitStatus> for ExitStatusError {
    fn into(self) -> ExitStatus {
        ExitStatus(self.0.get())
    }
}

impl ExitStatusError {
    pub fn code(self) -> Option<NonZero<i32>> {
        Some(self.0)
    }

    pub fn into_raw(self) -> i32 {
        self.0.get()
    }
}

// ── ExitCode ──────────────────────────────────────────────────────────────────

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct ExitCode(u8);

impl ExitCode {
    pub const SUCCESS: ExitCode = ExitCode(0);
    pub const FAILURE: ExitCode = ExitCode(1);
    pub fn as_i32(&self) -> i32 {
        self.0 as i32
    }
}

impl From<u8> for ExitCode {
    fn from(code: u8) -> Self {
        Self(code)
    }
}

// ── Process (child handle) ────────────────────────────────────────────────────

pub struct Process {
    pid: u32,
    /// Main thread TID of the child, used for kill().
    tid: u64,
    /// Cached exit status after the first successful wait().
    status: Option<ExitStatus>,
}

impl Process {
    pub fn id(&self) -> u32 {
        self.tid as u32
    }

    pub fn into_id(self) -> u32 {
        self.tid as u32
    }

    /// Request termination of the child process via SYS_TASK_KILL (uses TID).
    pub fn kill(&mut self) -> crate::io::Result<()> {
        let ret = unsafe { raw_syscall6(SYS_TASK_KILL, self.tid as usize, 0, 0, 0, 0, 0) };
        cvt(ret).map(|_| ())
    }

    pub fn send_signal(&self, _signal: i32) -> crate::io::Result<()> {
        // Ignored on ThingOS
        Ok(())
    }

    pub fn join(self) {
        unsafe {
            raw_syscall6(SYS_TASK_WAIT, self.tid as usize, 0, 0, 0, 0, 0);
        }
    }

    /// Block until child exits; returns cached status on second call.
    pub fn wait(&mut self) -> crate::io::Result<ExitStatus> {
        if let Some(status) = self.status {
            return Ok(status);
        }
        let mut code: i32 = 0;
        let ret = unsafe {
            raw_syscall6(SYS_WAITPID, self.pid as usize, &mut code as *mut i32 as usize, 0, 0, 0, 0)
        };
        cvt(ret)?;
        let status = ExitStatus(code);
        self.status = Some(status);
        Ok(status)
    }

    /// Non-blocking check; returns Ok(None) if child is still running.
    pub fn try_wait(&mut self) -> crate::io::Result<Option<ExitStatus>> {
        if let Some(status) = self.status {
            return Ok(Some(status));
        }
        let mut code: i32 = 0;
        let ret = unsafe {
            raw_syscall6(
                SYS_WAITPID,
                self.pid as usize,
                &mut code as *mut i32 as usize,
                WNOHANG,
                0,
                0,
                0,
            )
        };
        let child_pid = cvt(ret)?;
        if child_pid == 0 {
            Ok(None)
        } else {
            let status = ExitStatus(code);
            self.status = Some(status);
            Ok(Some(status))
        }
    }
}

// ── ChildPipe + read_output + output ─────────────────────────────────────────

/// A pipe end held by the parent for child stdio I/O.

/// Drain stdout and stderr concurrently (multiplexed drain).
///
/// This prevents deadlocks where the child blocks writing to stderr
/// because the parent is blocked reading from stdout, or vice versa.
/// Semantics match upstream std's deadlock-safe dual-pipe draining.
pub fn read_output(
    out: ChildPipe,
    stdout: &mut crate::vec::Vec<u8>,
    err: ChildPipe,
    stderr: &mut crate::vec::Vec<u8>,
) -> crate::io::Result<()> {
    let out_fd = out.as_raw_fd() as i32;
    let err_fd = err.as_raw_fd() as i32;

    // Set non-blocking mode on both pipes so we can drain them concurrently
    // without one blocking the other's progress.
    for fd in &[out_fd, err_fd] {
        let fl = unsafe { raw_syscall6(SYS_FS_FCNTL, *fd as usize, F_GETFL as usize, 0, 0, 0, 0) };
        if fl < 0 {
            return Err(crate::io::Error::from_raw_os_error(-fl as i32));
        }
        let ret = unsafe {
            raw_syscall6(
                SYS_FS_FCNTL,
                *fd as usize,
                F_SETFL as usize,
                (fl as usize) | O_NONBLOCK as usize,
                0,
                0,
                0,
            )
        };
        if ret < 0 {
            return Err(crate::io::Error::from_raw_os_error(-ret as i32));
        }
    }

    let mut out_done = false;
    let mut err_done = false;
    let mut tmp = [0u8; 4096];

    while !out_done || !err_done {
        let mut pfds = [
            PollFd { fd: out_fd, events: if out_done { 0 } else { POLLIN }, revents: 0 },
            PollFd { fd: err_fd, events: if err_done { 0 } else { POLLIN }, revents: 0 },
        ];

        // Wait for data or hangup on either pipe.
        let ret = unsafe {
            raw_syscall6(SYS_FS_POLL, pfds.as_mut_ptr() as usize, 2, usize::MAX, 0, 0, 0)
        };
        if ret < 0 {
            let err = -ret as i32;
            if err == 4 {
                continue;
            } // EINTR
            return Err(crate::io::Error::from_raw_os_error(err));
        }

        // Drain stdout if there's activity.
        if !out_done && (pfds[0].revents & (POLLIN | POLLERR | POLLHUP) != 0) {
            loop {
                match out.read(&mut tmp) {
                    Ok(0) => {
                        out_done = true;
                        break;
                    }
                    Ok(n) => {
                        stdout.extend_from_slice(&tmp[..n]);
                    }
                    Err(e) if e.raw_os_error() == Some(11) => break, // EAGAIN
                    Err(e) => return Err(e),
                }
            }
        }

        // Drain stderr if there's activity.
        if !err_done && (pfds[1].revents & (POLLIN | POLLERR | POLLHUP) != 0) {
            loop {
                match err.read(&mut tmp) {
                    Ok(0) => {
                        err_done = true;
                        break;
                    }
                    Ok(n) => {
                        stderr.extend_from_slice(&tmp[..n]);
                    }
                    Err(e) if e.raw_os_error() == Some(11) => break, // EAGAIN
                    Err(e) => return Err(e),
                }
            }
        }
    }

    Ok(())
}

/// Implementation of Command::output() on ThingOS.
pub fn output(
    cmd: &mut Command,
) -> crate::io::Result<(ExitStatus, crate::vec::Vec<u8>, crate::vec::Vec<u8>)> {
    let (mut process, mut pipes) = cmd.spawn(Stdio::MakePipe, false)?;
    drop(pipes.stdin.take());
    let mut stdout = crate::vec::Vec::new();
    let mut stderr = crate::vec::Vec::new();
    if let (Some(out), Some(err)) = (pipes.stdout.take(), pipes.stderr.take()) {
        read_output(out, &mut stdout, err, &mut stderr)?;
    }
    let status = process.wait()?;
    Ok((status, stdout, stderr))
}

impl From<Pipe> for Stdio {
    fn from(pipe: Pipe) -> Stdio {
        Stdio::InheritPipe(ChildPipe(pipe))
    }
}


impl From<i32> for ExitStatus {
    fn from(code: i32) -> ExitStatus {
        ExitStatus(code)
    }
}
