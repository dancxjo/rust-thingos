//! Platform-dependent file descriptor abstraction.

#![forbid(unsafe_op_in_unsafe_fn)]

cfg_select! {
    target_os = "thingos" => {
        // FileDesc is re-exported from sys::fs for ThingOS.
        pub use crate::sys::fs::FileDesc;
    }
    any(all(target_family = "unix", not(target_os = "thingos")), target_os = "wasi") => {
        mod unix;
        pub use unix::*;
    }
    target_os = "hermit" => {
        mod hermit;
        pub use hermit::*;
    }
    target_os = "motor" => {
        mod motor;
        pub use motor::*;
    }
    all(target_vendor = "fortanix", target_env = "sgx") => {
        mod sgx;
        pub use sgx::*;
    }
    _ => {}
}
