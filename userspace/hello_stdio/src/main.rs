//! Smoke test: println! and eprintln! go to stdout and stderr respectively.
//!
//! Acceptance criteria:
//!   - `println!("hi")` shows up on stdout (fd 1)
//!   - `eprintln!("oops")` shows up on stderr (fd 2)
//!   - stdin read_line works
#![no_std]
#![no_main]
use alloc::string::ToString;
use core::default::Default;
extern crate alloc;
use abi::motor::{
    MOTOR_DESCRIPTOR_ABI_VERSION, MOTOR_HOST_PROGRAM, MOTOR_INTERFACE_PROGRAM_V1, MotorDescriptor,
    MotorInterfaceDescriptor,
};

const THINGOS_MOTOR_NAME: &[u8] = b"hello_stdio.motor";

#[unsafe(no_mangle)]
#[used]
pub static THINGOS_MOTOR: MotorDescriptor = MotorDescriptor {
    abi_version: MOTOR_DESCRIPTOR_ABI_VERSION,
    interface_count: 1,
    hosting_modes: MOTOR_HOST_PROGRAM,
    capabilities: 0,
    motor_name_ptr: THINGOS_MOTOR_NAME.as_ptr(),
    motor_name_len: THINGOS_MOTOR_NAME.len(),
    interfaces: [
        MotorInterfaceDescriptor {
            interface_id: MOTOR_INTERFACE_PROGRAM_V1,
            interface_version: 1,
            flags: 0,
            reserved: 0,
            // Empty entry symbol means "use ELF default entry", which keeps
            // regular plain-main program flow intact.
            entry_symbol_ptr: core::ptr::null(),
            entry_symbol_len: 0,
        },
        MotorInterfaceDescriptor::default(),
        MotorInterfaceDescriptor::default(),
        MotorInterfaceDescriptor::default(),
    ],
};

fn main() {
    println!("[hello_stdio] stdout: hello from ThingOS!");
    eprintln!("[hello_stdio] stderr: this is stderr");

    // Read a line from stdin (non-blocking is fine for automated tests)
    use std::io::{BufRead, BufReader};
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    let n = reader.read_line(&mut line).unwrap_or(0);
    if n > 0 {
        println!("[hello_stdio] read from stdin: {:?}", line.trim_end());
    } else {
        println!("[hello_stdio] stdin: no input (EOF or empty)");
    }

    println!("[hello_stdio] PASS");
    std::process::exit(0);
}
