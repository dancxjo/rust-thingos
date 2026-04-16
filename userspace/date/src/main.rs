#![no_std]
#![no_main]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::Write;
use stem::syscall::{argv_get, exit, vfs_open, vfs_read, vfs_close, vfs_write};
use stem::utils::parse_argv;

#[derive(Debug, Clone, Copy)]
struct DateTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    weekday: u8, // 0 = Sunday, 1 = Monday, ..., 6 = Saturday
}

const MONTH_NAMES: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", 
    "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"
];

const WEEKDAY_NAMES: [&str; 7] = [
    "Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"
];

const WEEKDAY_NAMES_LONG: [&str; 7] = [
    "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"
];

fn is_leap_year(year: u16) -> bool {
    (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0))
}

fn days_in_month(month: u8, year: u16) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap_year(year) { 29 } else { 28 },
        _ => 0,
    }
}

fn unix_to_datetime(seconds: u64) -> DateTime {
    let mut remaining = seconds;
    
    let sec = (remaining % 60) as u8;
    remaining /= 60;
    let min = (remaining % 60) as u8;
    remaining /= 60;
    let hour = (remaining % 24) as u8;
    remaining /= 24;
    
    // remaining is now total days since 1970-01-01
    // 1970-01-01 was a Thursday (4)
    let weekday = ((remaining + 4) % 7) as u8;
    
    let mut year = 1970u16;
    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining < days_in_year as u64 {
            break;
        }
        remaining -= days_in_year as u64;
        year += 1;
    }
    
    let mut month = 1u8;
    loop {
        let dim = days_in_month(month, year);
        if remaining < dim as u64 {
            break;
        }
        remaining -= dim as u64;
        month += 1;
    }
    
    let day = (remaining + 1) as u8;
    
    DateTime {
        year,
        month,
        day,
        hour,
        minute: min,
        second: sec,
        weekday,
    }
}

fn get_tz_offset() -> i32 {
    let mut buf = [0u8; 1024];
    match vfs_open("/etc/locale.conf", stem::syscall::vfs_flags::O_RDONLY) {
        Ok(fd) => {
            match vfs_read(fd, &mut buf) {
                Ok(n) => {
                    let content = &buf[..n];
                    if let Ok(s) = core::str::from_utf8(content) {
                        for line in s.lines() {
                            if line.starts_with("TZ_OFFSET=") {
                                if let Ok(offset) = line.trim_start_matches("TZ_OFFSET=").parse::<i32>() {
                                    vfs_close(fd).ok();
                                    return offset;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            vfs_close(fd).ok();
        }
        _ => {}
    }
    0 // Default to UTC
}

fn print(msg: &str) {
    let _ = vfs_write(1, msg.as_bytes());
}

fn format_date(dt: DateTime, format: &str) -> String {
    let mut result = String::new();
    let mut chars = format.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '%' {
            if let Some(&next) = chars.peek() {
                chars.next(); // Consume the format character
                match next {
                    'Y' => { let _ = write!(result, "{:04}", dt.year); },
                    'm' => { let _ = write!(result, "{:02}", dt.month); },
                    'd' => { let _ = write!(result, "{:02}", dt.day); },
                    'H' => { let _ = write!(result, "{:02}", dt.hour); },
                    'M' => { let _ = write!(result, "{:02}", dt.minute); },
                    'S' => { let _ = write!(result, "{:02}", dt.second); },
                    'A' => { let _ = write!(result, "{}", WEEKDAY_NAMES_LONG[dt.weekday as usize]); },
                    'a' => { let _ = write!(result, "{}", WEEKDAY_NAMES[dt.weekday as usize]); },
                    'b' | 'h' => { let _ = write!(result, "{}", MONTH_NAMES[dt.month as usize - 1]); },
                    '%' => { result.push('%'); },
                    'n' => { result.push('\n'); },
                    't' => { result.push('\t'); },
                    _ => {
                        result.push('%');
                        result.push(next);
                    }
                }
            } else {
                result.push('%');
            }
        } else {
            result.push(c);
        }
    }
    
    result
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let mut len = 0;
    if let Ok(l) = argv_get(&mut []) {
        len = l;
    }
    
    let args = if len > 0 {
        let mut buf = alloc::vec![0u8; len];
        if argv_get(&mut buf).is_ok() {
            parse_argv(&buf).into_iter().skip(1).filter_map(|b| core::str::from_utf8(b).ok().map(String::from)).collect::<Vec<_>>()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    let unix_secs = stem::time::now_unix_seconds();
    let tz_offset = get_tz_offset();
    
    // Apply timezone offset (in hours)
    let local_secs = if tz_offset >= 0 {
        unix_secs.saturating_add(tz_offset as u64 * 3600)
    } else {
        unix_secs.saturating_sub(tz_offset.abs() as u64 * 3600)
    };
    
    let dt = unix_to_datetime(local_secs);

    if args.len() == 1 && args[0].starts_with('+') {
        let format = &args[0][1..];
        print(&format_date(dt, format));
        print("\n");
    } else if args.is_empty() {
        // Default format: Thu Apr 16 01:28:22 UTC 2026
        // Actually showing TZ offset name might be hard, just use UTC or Local labels
        let tz_label = if tz_offset == 0 { "UTC" } else { "Local" };
        let out = alloc::format!(
            "{} {} {:02} {:02}:{:02}:{:02} {} {:04}\n",
            WEEKDAY_NAMES[dt.weekday as usize],
            MONTH_NAMES[dt.month as usize - 1],
            dt.day,
            dt.hour,
            dt.minute,
            dt.second,
            tz_label,
            dt.year
        );
        print(&out);
    } else {
        let _ = vfs_write(2, b"usage: date [+FORMAT]\n");
        exit(1);
    }

    exit(0)
}
