//! Centralized ThingOS syscall numbers for std internals.
//!
//! This intentionally reuses the wire-only ABI numbers file directly so
//! syscall IDs stay in lockstep with kernel/stem/ABI.

mod abi_numbers {
    include!("../../../../thingos/abi/src/numbers.rs");
}

pub(crate) use abi_numbers::*;
