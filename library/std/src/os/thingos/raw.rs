//! ThingOS-specific raw type definitions.

#![stable(feature = "os_thingos", since = "1.0.0")]

use crate::ffi::{c_int, c_ulong};

#[stable(feature = "os_thingos", since = "1.0.0")]
pub type uid_t = u32;
#[stable(feature = "os_thingos", since = "1.0.0")]
pub type gid_t = u32;
#[stable(feature = "os_thingos", since = "1.0.0")]
pub type pid_t = i32;

#[stable(feature = "os_thingos", since = "1.0.0")]
pub const STDIN_FILENO: c_int = 0;
#[stable(feature = "os_thingos", since = "1.0.0")]
pub const STDOUT_FILENO: c_int = 1;
#[stable(feature = "os_thingos", since = "1.0.0")]
pub const STDERR_FILENO: c_int = 2;

#[stable(feature = "os_thingos", since = "1.0.0")]
pub const TCSANOW: c_int = 0;
#[stable(feature = "os_thingos", since = "1.0.0")]
pub const TCSADRAIN: c_int = 1;
#[stable(feature = "os_thingos", since = "1.0.0")]
pub const TCSAFLUSH: c_int = 2;

#[stable(feature = "os_thingos", since = "1.0.0")]
pub const TIOCGWINSZ: c_ulong = 0x5413;

#[stable(feature = "os_thingos", since = "1.0.0")]
pub const NCCS: usize = 32;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[stable(feature = "os_thingos", since = "1.0.0")]
#[allow(non_camel_case_types)]
pub struct termios {
    pub c_iflag: u32,
    pub c_oflag: u32,
    pub c_cflag: u32,
    pub c_lflag: u32,
    pub c_line: u8,
    pub _pad: [u8; 3],
    pub c_cc: [u8; NCCS],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[stable(feature = "os_thingos", since = "1.0.0")]
#[allow(non_camel_case_types)]
pub struct winsize {
    pub ws_row: u16,
    pub ws_col: u16,
    pub ws_xpixel: u16,
    pub ws_ypixel: u16,
}
