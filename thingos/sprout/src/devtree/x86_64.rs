#![no_std]
use alloc::string::ToString;
use core::default::Default;
extern crate alloc;
use abi::schema::{confidence, keys, kinds, rels, source};
use stem::debug;
use stem::thing::sys as thingsys;

use super::DevTreeCtx;

#[allow(dead_code)]
pub fn enumerate(_ctx: &DevTreeCtx) -> Result<(), ()> {
    debug!("SPROUT: x86_64 platform enrichment disabled during VFS migration");
    Ok(())
}
