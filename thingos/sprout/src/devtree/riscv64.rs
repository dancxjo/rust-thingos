#![no_std]
use alloc::string::ToString;
use core::default::Default;
extern crate alloc;
use stem::info;

use super::DevTreeCtx;

#[allow(dead_code)]
pub fn enumerate(_ctx: &DevTreeCtx) -> Result<(), ()> {
    info!("SPROUT: Enumerating RISC-V 64 platform (stub)...");
    Ok(())
}
