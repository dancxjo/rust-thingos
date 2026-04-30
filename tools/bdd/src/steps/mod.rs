//! Step definitions for BDD tests.
//!
//! Steps execute test logic. Artifact capture is handled by the reporter
//! which receives step events from cucumber and has access to the world.

mod basic;
mod blossom;
mod boot;
mod creative;
mod desktop;
mod driver_inbox;
mod e2e;
mod first_run;
pub mod helpers;
mod service_loop;
mod shutdown;

pub use helpers::StepError;
