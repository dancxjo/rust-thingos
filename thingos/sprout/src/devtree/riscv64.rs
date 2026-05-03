#![no_std]
use alloc::string::ToString;
use core::default::Default;
extern crate alloc;
use stem::debug;

use super::DevTreeCtx;

#[allow(dead_code)]
pub fn enumerate(_ctx: &DevTreeCtx) -> Result<(), ()> {
    debug!("SPROUT: RISC-V 64 platform enumeration is stubbed");
    Ok(())
}
