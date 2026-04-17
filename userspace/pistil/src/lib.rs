#![no_std]

extern crate alloc;

use abi::seed::{SEED_ABI_VERSION, Seed, SeedInterface};

pub mod typography;
pub mod compositor;

const SEED_NAME: &[u8] = b"pistil";

#[unsafe(no_mangle)]
#[used]
pub static THINGOS_SEED: Seed = Seed {
    // Shared libraries are planted as inspectable Seeds but are not germinated
    // directly as Program/Lifecycle/Driver shoots.
    abi_version: SEED_ABI_VERSION,
    interface_count: 0,
    hosting_modes: 0,
    capabilities: 0,
    name_ptr: SEED_NAME.as_ptr(),
    name_len: SEED_NAME.len(),
    interfaces: [
        SeedInterface::zero(),
        SeedInterface::zero(),
        SeedInterface::zero(),
        SeedInterface::zero(),
    ],
};
