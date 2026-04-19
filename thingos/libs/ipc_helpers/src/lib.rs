//! Userspace helper library for writing VFS providers and port services.
//!
//! This library provides ergonomic wrappers around the raw Thing-OS IPC
//! syscalls, making it easy to write drivers and system services without
//! dealing with framing details manually.
//!
//! # Modules
//!
//! - [`port`] — port send/recv wrappers with RPC framing support
//! - [`inbox`]   — typed message send/recv over the process inbox IPC path
//! - [`provider`] — VFS provider server loop
//! - [`rpc`] — typed request/reply client and server helpers
//!
//! # Quick Start
//!
//! **Writing a simple service** (see [`port`]):
//!
//! ```ignore
//! use ipc_helpers::rpc::{RpcServer, RpcRequest};
//!
//! let (write_h, read_h) = stem::syscall::port::port_create(4096).unwrap();
//! // publish write_h to clients …
//!
//! let mut server = RpcServer::new(read_h);
//! loop {
//!     let req = server.next().unwrap();
//!     server.reply(req.request_id, b"pong").unwrap();
//! }
//! ```
//!
//! **Sending and receiving inbox messages** (see [`inbox`]):
//!
//! ```ignore
//! use ipc_helpers::inbox::{InboxReceiver, send_typed, MsgKindId};
//! use abi::KindId;
//!
//! // Receiver side — block until a message arrives
//! let rx = InboxReceiver::new(4096);
//! let msg = rx.recv_blocking();
//!
//! // Sender side — deliver a typed message by PID
//! send_typed(target_pid, KindId([0u8; 16]), b"hello").unwrap();
//! ```
//!
//! **Writing a VFS provider** (see [`provider`]):
//!
//! ```ignore
//! use ipc_helpers::provider::{ProviderLoop, ProviderResponse};
//! use abi::vfs_rpc::VfsRpcOp;
//! use abi::errors::Errno;
//!
//! let mut lp = ProviderLoop::new(vfs_read);
//! loop {
//!     let req = lp.next_request().unwrap();
//!     let resp = match req.op {
//!         VfsRpcOp::Lookup => ProviderResponse::ok_u64(1),
//!         VfsRpcOp::Read => ProviderResponse::ok_bytes(b"hello\n"),
//!         VfsRpcOp::Stat => ProviderResponse::ok_stat(0o100644, 6, 1),
//!         VfsRpcOp::Close => ProviderResponse::ok_empty(),
//!         _ => ProviderResponse::err(Errno::ENOSYS),
//!     };
//!     lp.send_response(req.resp_port, resp).unwrap();
//! }
//! ```

#![no_std]
extern crate alloc;

pub mod port;
pub mod inbox;
pub mod provider;
pub mod rpc;
