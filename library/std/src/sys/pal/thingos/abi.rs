//! Shared ThingOS PAL ABI mirror types.
//!
//! Keep this file synchronized with:
//! - `abi/src/types/system.rs`
//! - `abi/src/time.rs`

/// Mirrors `abi::time::TimeSpec`.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(crate) struct TimeSpec {
    pub(crate) secs: u64,
    pub(crate) nanos: u32,
    pub(crate) _reserved: u32,
}

/// Mirrors `abi::types::FdRemap`.
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub(crate) struct FdRemap {
    pub(crate) src_fd: u64,
    pub(crate) dst_fd: u64,
}

/// Mirrors `abi::types::SpawnProcessExReq`.
#[repr(C)]
#[derive(Default)]
pub(crate) struct SpawnProcessExReq {
    pub(crate) name_ptr: u64,
    pub(crate) name_len: u32,
    pub(crate) _pad0: u32,
    pub(crate) argv_ptr: u64,
    pub(crate) argv_len: u32,
    pub(crate) _pad1: u32,
    pub(crate) env_ptr: u64,
    pub(crate) env_len: u32,
    pub(crate) _pad2: u32,
    pub(crate) stdin_mode: u32,
    pub(crate) stdout_mode: u32,
    pub(crate) stderr_mode: u32,
    pub(crate) _reserved: u32,
    pub(crate) boot_arg: u64,
    pub(crate) handles_to_inherit: [u64; 8],
    pub(crate) num_inherited_handles: u32,
    pub(crate) _pad3: u32,
    pub(crate) cwd_ptr: u64,
    pub(crate) cwd_len: u32,
    pub(crate) _pad4: u32,
    pub(crate) fd_remap_ptr: u64,
    pub(crate) fd_remap_len: u32,
    pub(crate) _pad5: u32,
}

/// Mirrors `abi::types::SpawnProcessExResp`.
#[repr(C)]
#[derive(Default)]
pub(crate) struct SpawnProcessExResp {
    pub(crate) child_tid: u64,
    pub(crate) child_pid: u32,
    pub(crate) _pad: u32,
    pub(crate) stdin_pipe: u64,
    pub(crate) stdout_pipe: u64,
    pub(crate) stderr_pipe: u64,
}

// Layout guards (64-bit ThingOS targets).
const _: [(); 16] = [(); core::mem::size_of::<TimeSpec>()];
const _: [(); 16] = [(); core::mem::size_of::<FdRemap>()];
const _: [(); 176] = [(); core::mem::size_of::<SpawnProcessExReq>()];
const _: [(); 40] = [(); core::mem::size_of::<SpawnProcessExResp>()];
