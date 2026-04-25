//! `ServiceProviderLoop` — inbox-aware wrapper for VFS providers.
//!
//! Combines a [`ProviderLoop`] (port-based VFS RPC dispatch) with a
//! [`stem::service_loop::ServiceLoop`] (inbox control plane) so a provider
//! can participate in the inbox-backed actor model while retaining the
//! existing port-based transport required by the kernel.
//!
//! # Design
//!
//! ```text
//! inbox ──────────────────────► ServiceProviderEvent::Message  (control priority)
//! provider port FD ───────────► ServiceProviderEvent::ProviderRequest
//! optional extra FDs/IRQs ────► ServiceProviderEvent::Ready
//! ```
//!
//! `ServiceProviderLoop` registers the provider's port read handle as a
//! secondary source inside the `ServiceLoop`'s `WaitSet`.  `next_event()`
//! preserves the inbox-first dispatch order of `ServiceLoop`: control-plane
//! inbox messages are delivered before provider RPC, even when both are ready
//! at the same wake.
//!
//! # Usage
//!
//! ```ignore
//! use ipc_helpers::provider::{ProviderLoop, ProviderResponse};
//! use ipc_helpers::service_provider::{ServiceProviderEvent, ServiceProviderLoop};
//! use abi::vfs_rpc::VfsRpcOp;
//! use abi::errors::Errno;
//! use stem::syscall::{port_create, vfs_mount};
//!
//! // 1. Create the port pair and mount the provider.
//! let (write_h, read_h) = port_create(65536).unwrap();
//! vfs_mount(write_h, "/run/myprovider").unwrap();
//!
//! // 2. Wrap the provider in a ServiceProviderLoop.
//! let mut svc =
//!     ServiceProviderLoop::new(ProviderLoop::new(read_h), 4096).unwrap();
//!
//! // 3. Optionally register extra readiness sources.
//! // let timer_token = svc.add_fd_readable(timer_fd).unwrap();
//!
//! // 4. Event loop — inbox messages get control priority.
//! loop {
//!     match svc.next_event(None).unwrap() {
//!         ServiceProviderEvent::ProviderRequest(req) => {
//!             let resp = match req.op {
//!                 VfsRpcOp::Lookup => ProviderResponse::ok_u64(1),
//!                 VfsRpcOp::Read   => ProviderResponse::ok_bytes(b"Hello!\n"),
//!                 VfsRpcOp::Close  => ProviderResponse::ok_empty(),
//!                 _                => ProviderResponse::err(Errno::ENOSYS),
//!             };
//!             svc.send_response(&req, resp).unwrap();
//!         }
//!         ServiceProviderEvent::Message { kind, payload } => {
//!             // Handle lifecycle / config messages.
//!             let _ = (kind, payload);
//!         }
//!         ServiceProviderEvent::InboxClosed => break,
//!         ServiceProviderEvent::Timeout => {}
//!         ServiceProviderEvent::Ready { .. } => {}
//!     }
//! }
//! ```
//!
//! # Non-goals
//!
//! - Does **not** replace the port-based [`ProviderLoop`] transport.
//! - Does **not** add new syscalls.
//! - Does **not** introduce an async runtime.

use abi::wire::KindId;
use alloc::string::String;
use alloc::vec::Vec;
use core::ops::ControlFlow;

use crate::provider::{ProviderLoop, ProviderRequest, ProviderResponse};
use stem::errors::Errno;
use stem::service_loop::{ServiceEvent, ServiceLoop};
use stem::syscall::vfs::{vfs_close, vfs_umount};
use stem::time::Duration;
use stem::wait_set::{WaitEvent, WaitToken};

// ─── ServiceProviderEvent ────────────────────────────────────────────────────

/// A single dispatched event yielded by
/// [`ServiceProviderLoop::next_event`].
#[derive(Debug)]
pub enum ServiceProviderEvent<'a> {
    /// A VFS RPC request arrived from the kernel via the provider port.
    ///
    /// Pass the request — and the response returned by your dispatch logic
    /// — back to [`ServiceProviderLoop::send_response`].
    ProviderRequest(ProviderRequest),

    /// A typed control message was dequeued from the calling task's inbox.
    ///
    /// `payload` borrows the loop's internal scratch buffer.  Copy out
    /// before the next loop iteration if the data needs to outlive it.
    Message {
        /// 16-byte schema identifier for the payload.
        kind: KindId,
        /// Message payload bytes (borrowed from the loop's scratch buffer).
        payload: &'a [u8],
    },

    /// The inbox was closed (process is terminating or the inbox FD was
    /// revoked).  Treat this as a clean shutdown signal.
    InboxClosed,

    /// A registered secondary readiness source fired.
    ///
    /// Use the `token` (returned when you called `add_fd_readable` /
    /// `add_irq` etc.) to dispatch.
    Ready {
        /// The token assigned when the source was added.
        token: WaitToken,
        /// The full underlying [`WaitEvent`] for callers that need
        /// readability / writability / hangup / IRQ / exit detail.
        event: WaitEvent,
    },

    /// The supplied timeout elapsed before any source fired.
    ///
    /// Only returned when [`ServiceProviderLoop::next_event`] is called
    /// with a finite `timeout`.
    Timeout,
}

// ─── ServiceProviderLoop ────────────────────────────────────────────────────

/// Inbox-aware VFS provider loop.
///
/// Wraps a [`ProviderLoop`] and a [`ServiceLoop`] to provide a unified
/// event loop that prioritises inbox control messages over VFS RPC requests.
///
/// See the [module-level documentation](self) for the design contract and a
/// usage example.
pub struct ServiceProviderLoop {
    svc: ServiceLoop,
    provider: ProviderLoop,
    /// The `WaitToken` assigned to the provider port inside the
    /// `ServiceLoop`'s `WaitSet`.
    provider_token: WaitToken,
    /// VFS mount paths registered for cleanup during graceful shutdown.
    mount_paths: Vec<String>,
    /// Idempotency guard: set to `true` once `shutdown_sequence` has run.
    shutdown_done: bool,
}

impl ServiceProviderLoop {
    /// Construct a new `ServiceProviderLoop`.
    ///
    /// - `provider` — a fully constructed [`ProviderLoop`] bound to the
    ///   VFS provider port.
    /// - `max_payload` — size of the internal scratch buffer for inbox
    ///   messages (passed verbatim to [`ServiceLoop::new`]).
    ///
    /// # Errors
    ///
    /// Propagates any [`Errno`] from [`ServiceLoop::new`] (e.g. `ENOENT` if
    /// procfs is not mounted) or from registering the provider port handle
    /// (e.g. `ENOSPC` if the `WaitSet` is full, which is not possible here
    /// because there are only two sources at construction time).
    pub fn new(provider: ProviderLoop, max_payload: usize) -> Result<Self, Errno> {
        let mut svc = ServiceLoop::new(max_payload)?;
        // `add_port_readable` accepts a `u64` to match the wide `WaitSpec::object`
        // field used internally by the kernel.  The widening cast from the `u32`
        // port handle is always safe (zero-extends, no data loss).
        #[allow(deprecated)]
        let provider_token = svc.add_port_readable(provider.port_handle() as u64)?;
        Ok(Self { svc, provider, provider_token, mount_paths: Vec::new(), shutdown_done: false })
    }

    /// The [`WaitToken`] assigned to the provider port inside the
    /// underlying `ServiceLoop`.
    ///
    /// Useful when you need to distinguish a `ServiceProviderEvent::Ready`
    /// event fired by the provider token itself (rare) from tokens added
    /// via `add_fd_readable` etc.
    #[inline]
    pub fn provider_token(&self) -> WaitToken {
        self.provider_token
    }

    // ── graceful shutdown ─────────────────────────────────────────────────

    /// Register a VFS mount path to be unmounted during graceful shutdown.
    ///
    /// Call this once per mounted path *before* entering the event loop.
    /// Paths are unmounted in registration order by [`shutdown_sequence`].
    ///
    /// [`shutdown_sequence`]: Self::shutdown_sequence
    pub fn register_mount_path(&mut self, path: &str) {
        self.mount_paths.push(String::from(path));
    }

    /// Execute the canonical graceful-shutdown sequence.
    ///
    /// Steps (in order):
    /// 1. **Stop accepting new work** — the caller must not call
    ///    `next_event` after this returns.
    /// 2. **Unmount VFS provider paths** — every path registered with
    ///    [`register_mount_path`] is unmounted via `vfs_umount`.  Failures
    ///    are logged as warnings and do not abort the sequence.
    /// 3. **Close the provider port handle** — best-effort `vfs_close`
    ///    on the port read handle so the kernel can reclaim resources.
    /// 4. **Log shutdown progression** — each step is logged at `info`
    ///    level so operators can observe the sequence in the kernel log.
    ///
    /// This method is **idempotent**: subsequent calls after the first are
    /// silent no-ops, so it is safe to call from error paths and drop
    /// implementations.
    ///
    /// [`register_mount_path`]: Self::register_mount_path
    pub fn shutdown_sequence(&mut self) {
        if self.shutdown_done {
            return;
        }
        self.shutdown_done = true;

        stem::info!("ServiceProviderLoop: initiating graceful shutdown");

        // Step 2: Unmount all registered VFS provider paths.
        for path in &self.mount_paths {
            stem::info!("ServiceProviderLoop: unmounting {}", path);
            match vfs_umount(path) {
                Ok(()) => stem::info!("ServiceProviderLoop: unmounted {}", path),
                Err(e) => stem::warn!(
                    "ServiceProviderLoop: vfs_umount({}) failed: {:?} (continuing)",
                    path,
                    e
                ),
            }
        }

        // Step 3: Close the provider port read handle.
        let _ = vfs_close(self.provider.port_handle());

        stem::info!("ServiceProviderLoop: shutdown complete");
    }

    /// Run the event loop until the inbox is closed (or the handler
    /// requests a break), then call [`shutdown_sequence`] and return.
    ///
    /// This is the preferred entry point for provider daemons that want
    /// the full graceful-shutdown contract built in:
    ///
    /// ```text
    /// InboxClosed  ──►  shutdown_sequence()  ──►  return
    /// handler Break ──►  shutdown_sequence()  ──►  return
    /// next_event Err ──►  shutdown_sequence()  ──►  return
    /// ```
    ///
    /// Register mount paths before calling this method:
    ///
    /// ```ignore
    /// svc.register_mount_path("/dev/net/virtio0");
    /// svc.run_until_shutdown(|ev| {
    ///     // handle ProviderRequest / Message / Ready events
    ///     ControlFlow::Continue(())
    /// }, None);
    /// ```
    ///
    /// [`shutdown_sequence`]: Self::shutdown_sequence
    pub fn run_until_shutdown<F>(&mut self, mut handler: F, timeout: Option<Duration>)
    where
        F: FnMut(ServiceProviderEvent<'_>) -> ControlFlow<()>,
    {
        loop {
            match self.next_event(timeout) {
                Ok(ServiceProviderEvent::InboxClosed) => {
                    stem::info!(
                        "ServiceProviderLoop: inbox closed — initiating graceful shutdown"
                    );
                    self.shutdown_sequence();
                    return;
                }
                Ok(event) => {
                    if let ControlFlow::Break(()) = handler(event) {
                        stem::info!(
                            "ServiceProviderLoop: handler requested shutdown"
                        );
                        self.shutdown_sequence();
                        return;
                    }
                }
                Err(e) => {
                    stem::warn!(
                        "ServiceProviderLoop: next_event error {:?} — initiating graceful shutdown",
                        e
                    );
                    self.shutdown_sequence();
                    return;
                }
            }
        }
    }

    // ── secondary registration (thin pass-throughs to ServiceLoop) ───────

    /// Register an additional VFS file descriptor for read readiness.
    ///
    /// Returns a [`WaitToken`] that will appear in
    /// [`ServiceProviderEvent::Ready`] when the FD becomes readable.
    pub fn add_fd_readable(&mut self, fd: u32) -> Result<WaitToken, Errno> {
        self.svc.add_fd_readable(fd)
    }

    /// Register an IRQ subscription handle (from `device_irq_subscribe`).
    pub fn add_irq(&mut self, irq_handle: u64) -> Result<WaitToken, Errno> {
        self.svc.add_irq(irq_handle)
    }

    /// Register a child task ID for exit notification.
    pub fn add_task_exit(&mut self, tid: u64) -> Result<WaitToken, Errno> {
        self.svc.add_task_exit(tid)
    }

    /// Remove a previously registered secondary source.
    ///
    /// Returns `true` if the token was found and removed.  Attempting to
    /// remove the inbox token or the provider token returns `false`.
    pub fn remove(&mut self, token: WaitToken) -> bool {
        if token == self.provider_token {
            return false;
        }
        self.svc.remove(token)
    }

    // ── core dispatch ────────────────────────────────────────────────────

    /// Block until at least one source becomes ready, then dispatch one
    /// event.
    ///
    /// Dispatch priority (when multiple sources fire on the same wake):
    /// 1. Inbox closure → [`ServiceProviderEvent::InboxClosed`]
    /// 2. Inbox message → [`ServiceProviderEvent::Message`]
    /// 3. Provider port ready → [`ServiceProviderEvent::ProviderRequest`]
    ///    (calls [`ProviderLoop::try_next_request`] internally)
    /// 4. Other secondary sources → [`ServiceProviderEvent::Ready`]
    ///
    /// `timeout = None` blocks indefinitely.
    pub fn next_event(
        &mut self,
        timeout: Option<Duration>,
    ) -> Result<ServiceProviderEvent<'_>, Errno> {
        let provider_token = self.provider_token;
        loop {
            // We use a raw pointer to bypass the borrow checker's conservative
            // lifetime pinning in loops. This is safe because we either return
            // the borrow immediately or ensure it is dropped before the next
            // iteration (by not holding any references to it).
            let svc = &mut self.svc as *mut ServiceLoop;
            let svc_ev = unsafe { (*svc).next_event(timeout)? };

            match svc_ev {
                ServiceEvent::InboxClosed => return Ok(ServiceProviderEvent::InboxClosed),
                ServiceEvent::Timeout => return Ok(ServiceProviderEvent::Timeout),
                ServiceEvent::Message { kind, payload } => {
                    return Ok(ServiceProviderEvent::Message { kind, payload });
                }
                ServiceEvent::Ready { token, event } => {
                    if token == provider_token {
                        // The provider port has data; try to decode one request.
                        match self.provider.try_next_request() {
                            Ok(Some(req)) => return Ok(ServiceProviderEvent::ProviderRequest(req)),
                            Ok(None) => {
                                // Spurious wake — continue loop.
                            }
                            Err(Errno::EPIPE) => return Ok(ServiceProviderEvent::InboxClosed),
                            Err(e) => return Err(e),
                        }
                    } else {
                        return Ok(ServiceProviderEvent::Ready { token, event });
                    }
                }
            }
        }
    }



    /// Send `response` back to the kernel for the given `req`.
    ///
    /// Thin delegation to [`ProviderLoop::send_response`].
    #[inline]
    pub fn send_response(
        &self,
        req: &ProviderRequest,
        response: ProviderResponse,
    ) -> Result<(), Errno> {
        self.provider.send_response(req, response)
    }
}
