#![no_std]
#![no_main]

extern crate alloc;

#[stem::main]
fn main(arg0: usize) -> ! {
    pistil::typography::run_service(arg0)
}
