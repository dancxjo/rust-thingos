#![no_std]
#![no_main]
extern crate alloc;

mod pipelines;
mod supervisor;

#[stem::main]
fn main(arg0: usize) -> ! {
    stem::info!("SPROUT: starting minimal session supervisor (arg0={})", arg0);
    let mut supervisor = supervisor::Supervisor::new();
    supervisor.run_forever()
}
