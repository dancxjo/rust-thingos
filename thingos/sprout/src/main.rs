#![no_std]
#![no_main]
extern crate alloc;

mod pipelines;
mod supervisor;

#[stem::main]
fn main(arg0: usize) -> ! {
    stem::debug!("Starting minimal session supervisor with arg0={}", arg0);
    let mut supervisor = supervisor::Supervisor::new();
    supervisor.run_forever()
}
