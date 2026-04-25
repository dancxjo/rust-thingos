//! Step definitions for BDD tests.
//!
//! Steps execute test logic. Artifact capture is handled by the reporter
//! which receives step events from cucumber and has access to the world.

pub mod helpers;
mod basic;
mod first_run;
mod boot;
mod e2e;
mod desktop;
mod creative;
mod driver_inbox;
mod blossom;
mod service_loop;
mod shutdown;

pub use helpers::StepError;
