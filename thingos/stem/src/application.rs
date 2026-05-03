//! Application lifecycle helpers built on [`crate::service_loop`].
//!
//! `Application` is the user-facing shape for long-running ThingOS programs:
//! create a [`ServiceLooper`], register any windows or cleanup hooks in an
//! [`ApplicationContext`], then dispatch readiness and inbox events until the
//! application asks to quit.

use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::errors::Errno;
use crate::service_loop::{ServiceEvent, ServiceLoop};
use crate::time::Duration;
use crate::wait_set::WaitToken;

/// Action returned from an application event handler.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppAction {
    /// Keep the application running.
    Continue,
    /// Begin the normal quit path.
    Quit,
}

/// Result of asking an application whether it may quit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuitDecision {
    /// Continue with shutdown.
    Allow,
    /// Cancel this quit request and return to the event loop.
    Cancel,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WindowToken(usize);

struct WindowHook {
    name: &'static str,
    closed: bool,
    close: Box<dyn FnMut()>,
}

struct CleanupHook {
    name: &'static str,
    ran: bool,
    cleanup: Box<dyn FnMut()>,
}

/// Mutable application state owned by the lifecycle runner.
///
/// Apps use this to register windows and process cleanup.  On quit, windows
/// are closed in reverse registration order, then cleanup hooks are run in
/// reverse registration order.
pub struct ApplicationContext {
    name: &'static str,
    windows: Vec<WindowHook>,
    cleanup: Vec<CleanupHook>,
    quit_requested: bool,
}

impl ApplicationContext {
    pub fn new(name: &'static str) -> Self {
        Self { name, windows: Vec::new(), cleanup: Vec::new(), quit_requested: false }
    }

    /// Application name used by diagnostics.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Register a top-level or transient window-like protocol object.
    pub fn register_window<F>(&mut self, name: &'static str, close: F) -> WindowToken
    where
        F: FnMut() + 'static,
    {
        let token = WindowToken(self.windows.len());
        self.windows.push(WindowHook { name, closed: false, close: Box::new(close) });
        token
    }

    /// Close one registered window if it has not already been closed.
    pub fn close_window(&mut self, token: WindowToken) {
        if let Some(window) = self.windows.get_mut(token.0) {
            if !window.closed {
                window.closed = true;
                (window.close)();
            }
        }
    }

    /// Close all still-open windows in reverse creation order.
    pub fn close_all_windows(&mut self) {
        for window in self.windows.iter_mut().rev() {
            if !window.closed {
                crate::debug!("Closing window {}", window.name);
                window.closed = true;
                (window.close)();
            }
        }
    }

    /// Register process cleanup that should happen after windows close.
    pub fn register_cleanup<F>(&mut self, name: &'static str, cleanup: F)
    where
        F: FnMut() + 'static,
    {
        self.cleanup.push(CleanupHook { name, ran: false, cleanup: Box::new(cleanup) });
    }

    /// Run all cleanup hooks in reverse creation order.
    pub fn run_cleanup(&mut self) {
        for hook in self.cleanup.iter_mut().rev() {
            if !hook.ran {
                crate::debug!("Running cleanup {}", hook.name);
                hook.ran = true;
                (hook.cleanup)();
            }
        }
    }

    /// Ask the runner to enter the normal quit path after the current event.
    pub fn request_quit(&mut self) {
        self.quit_requested = true;
    }

    fn take_quit_requested(&mut self) -> bool {
        let requested = self.quit_requested;
        self.quit_requested = false;
        requested
    }
}

/// Thin application-owned wrapper around [`ServiceLoop`].
pub struct ServiceLooper {
    inner: ServiceLoop,
}

impl ServiceLooper {
    pub fn new(max_payload: usize) -> Result<Self, Errno> {
        Ok(Self { inner: ServiceLoop::new(max_payload)? })
    }

    pub fn set_name(&mut self, name: &str) {
        self.inner.set_name(name);
    }

    pub fn add_fd_readable(&mut self, fd: u32) -> Result<WaitToken, Errno> {
        self.inner.add_fd_readable(fd)
    }

    pub fn add_fd_writable(&mut self, fd: u32) -> Result<WaitToken, Errno> {
        self.inner.add_fd_writable(fd)
    }

    pub fn next_event(&mut self, timeout: Option<Duration>) -> Result<ServiceEvent<'_>, Errno> {
        self.inner.next_event(timeout)
    }

    pub fn service_loop(&self) -> &ServiceLoop {
        &self.inner
    }

    pub fn service_loop_mut(&mut self) -> &mut ServiceLoop {
        &mut self.inner
    }
}

/// Standard lifecycle trait for long-running ThingOS applications.
pub trait Application: Sized {
    /// Human-readable application name and default service-loop name.
    const NAME: &'static str;

    /// Scratch payload size for inbox messages.
    const MAX_PAYLOAD: usize = 4096;

    /// Construct the application and register its initial loop interests.
    fn init(ctx: &mut ApplicationContext, looper: &mut ServiceLooper) -> Result<Self, Errno>;

    /// Called after init succeeds and before the first event wait.
    fn ready(&mut self, _ctx: &mut ApplicationContext) {}

    /// Timeout for the next event wait.
    fn timeout(&self) -> Option<Duration> {
        None
    }

    /// Dispatch one service-loop event.
    fn handle_event(&mut self, ctx: &mut ApplicationContext, event: ServiceEvent<'_>) -> AppAction;

    /// Give the application a chance to cancel quit.
    fn can_quit(&mut self, _ctx: &mut ApplicationContext) -> QuitDecision {
        QuitDecision::Allow
    }

    /// Called once after quit is accepted and before windows are closed.
    fn quitting(&mut self, _ctx: &mut ApplicationContext) {}
}

/// Run an [`Application`] until it quits, then exit the process.
pub fn run_application<A: Application>() -> ! {
    let mut ctx = ApplicationContext::new(A::NAME);
    let mut looper = match ServiceLooper::new(A::MAX_PAYLOAD) {
        Ok(looper) => looper,
        Err(e) => {
            crate::error!("Failed to create application loop: {:?}", e);
            crate::syscall::exit(1);
        }
    };
    looper.set_name(A::NAME);

    let mut app = match A::init(&mut ctx, &mut looper) {
        Ok(app) => app,
        Err(e) => {
            crate::error!("Failed to initialize application: {:?}", e);
            ctx.close_all_windows();
            ctx.run_cleanup();
            crate::syscall::exit(1);
        }
    };
    app.ready(&mut ctx);

    loop {
        let action = match looper.next_event(app.timeout()) {
            Ok(ServiceEvent::InboxClosed) => AppAction::Quit,
            Ok(event) => app.handle_event(&mut ctx, event),
            Err(e) => {
                crate::warn!("Application loop wait failed: {:?}", e);
                AppAction::Quit
            }
        };

        if action == AppAction::Quit || ctx.take_quit_requested() {
            match app.can_quit(&mut ctx) {
                QuitDecision::Allow => {
                    app.quitting(&mut ctx);
                    ctx.close_all_windows();
                    ctx.run_cleanup();
                    crate::syscall::exit(0);
                }
                QuitDecision::Cancel => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::rc::Rc;
    use alloc::vec;
    use core::cell::RefCell;

    use super::*;

    #[test]
    fn windows_close_in_reverse_registration_order() {
        let order = Rc::new(RefCell::new(Vec::new()));
        let mut ctx = ApplicationContext::new("test");

        let a = order.clone();
        ctx.register_window("a", move || a.borrow_mut().push(1));
        let b = order.clone();
        ctx.register_window("b", move || b.borrow_mut().push(2));

        ctx.close_all_windows();
        assert_eq!(&*order.borrow(), &vec![2, 1]);
    }

    #[test]
    fn close_window_is_idempotent() {
        let count = Rc::new(RefCell::new(0));
        let mut ctx = ApplicationContext::new("test");
        let c = count.clone();
        let token = ctx.register_window("window", move || *c.borrow_mut() += 1);

        ctx.close_window(token);
        ctx.close_window(token);
        ctx.close_all_windows();

        assert_eq!(*count.borrow(), 1);
    }
}
