#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use abi::syscall::vfs_flags::O_RDONLY;
use stem::syscall::{argv_get, vfs_close, vfs_open, vfs_read, vfs_write};

const IFACE: &str = "eth0";

fn get_args() -> Vec<String> {
    let mut len = 0;
    if let Ok(l) = argv_get(&mut []) {
        len = l;
    }
    if len == 0 {
        return Vec::new();
    }

    let mut buf = alloc::vec![0u8; len];
    if argv_get(&mut buf).is_err() {
        return Vec::new();
    }

    let mut args = Vec::new();
    if buf.len() >= 4 {
        let count = u32::from_le_bytes(buf[0..4].try_into().unwrap()) as usize;
        let mut offset = 4;
        for _ in 0..count {
            if offset + 4 > buf.len() {
                break;
            }
            let str_len = u32::from_le_bytes(buf[offset..offset + 4].try_into().unwrap()) as usize;
            offset += 4;
            if offset + str_len > buf.len() {
                break;
            }
            if let Ok(s) = core::str::from_utf8(&buf[offset..offset + str_len]) {
                args.push(String::from(s));
            }
            offset += str_len;
        }
    }

    args
}

fn print(fd: u32, s: &str) {
    let _ = vfs_write(fd, s.as_bytes());
}

fn read_text(path: &str) -> Result<String, &'static str> {
    let fd = vfs_open(path, O_RDONLY).map_err(|_| "open failed")?;
    let mut buf = alloc::vec![0u8; 2048];
    let n = vfs_read(fd, &mut buf).map_err(|_| "read failed")?;
    let _ = vfs_close(fd);
    buf.truncate(n);
    Ok(String::from_utf8_lossy(&buf).trim().to_string())
}

fn status_value(status: &str, key: &str) -> Option<String> {
    for line in status.lines() {
        let line = line.trim();
        let Some((k, v)) = line.split_once(':') else {
            continue;
        };
        if k.trim() == key {
            return Some(v.trim().to_string());
        }
    }
    None
}

fn print_usage() {
    print(1, "usage: ip [addr|link|route|dns|stats]\n");
}

fn print_addr() -> Result<(), &'static str> {
    let path = alloc::format!("/net/interfaces/{}/addr", IFACE);
    let addr = read_text(&path)?;
    print(1, &alloc::format!("{}: {}\n", IFACE, addr));
    Ok(())
}

fn print_link() -> Result<(), &'static str> {
    let path = alloc::format!("/net/interfaces/{}/status", IFACE);
    let status = read_text(&path)?;
    let state = status_value(&status, "state").unwrap_or_else(|| "unknown".into());
    let link = status_value(&status, "link").unwrap_or_else(|| "unknown".into());
    let mac = status_value(&status, "mac").unwrap_or_else(|| "unknown".into());
    let mtu = status_value(&status, "mtu").unwrap_or_else(|| "unknown".into());

    print(1, &alloc::format!("{}: state={} link={} mtu={} mac={}\n", IFACE, state, link, mtu, mac));
    Ok(())
}

fn print_route() -> Result<(), &'static str> {
    let routes = read_text("/net/routes")?;
    print(1, "routes:\n");
    if routes.is_empty() {
        print(1, "  (none)\n");
    } else {
        for line in routes.lines() {
            print(1, &alloc::format!("  {}\n", line));
        }
    }
    Ok(())
}

fn print_dns() -> Result<(), &'static str> {
    let dns = read_text("/net/dns/server")?;
    print(1, &alloc::format!("dns: {}\n", dns));
    Ok(())
}

fn print_stats() -> Result<(), &'static str> {
    let path = alloc::format!("/net/interfaces/{}/stats", IFACE);
    let stats = read_text(&path)?;
    print(1, "stats:\n");
    if stats.is_empty() {
        print(1, "  (none)\n");
    } else {
        for line in stats.lines() {
            print(1, &alloc::format!("  {}\n", line));
        }
    }
    Ok(())
}

fn print_summary() -> Result<(), &'static str> {
    let status_path = alloc::format!("/net/interfaces/{}/status", IFACE);
    let addr_path = alloc::format!("/net/interfaces/{}/addr", IFACE);

    let status = read_text(&status_path)?;
    let addr = read_text(&addr_path)?;
    let routes = read_text("/net/routes").unwrap_or_else(|_| String::new());
    let dns = read_text("/net/dns/server").unwrap_or_else(|_| String::new());

    let state = status_value(&status, "state").unwrap_or_else(|| "unknown".into());
    let link = status_value(&status, "link").unwrap_or_else(|| "unknown".into());
    let mac = status_value(&status, "mac").unwrap_or_else(|| "unknown".into());
    let mtu = status_value(&status, "mtu").unwrap_or_else(|| "unknown".into());

    print(1, &alloc::format!("{}\n", IFACE));
    print(1, &alloc::format!("  addr: {}\n", addr));
    print(1, &alloc::format!("  state: {}\n", state));
    print(1, &alloc::format!("  link: {}\n", link));
    print(1, &alloc::format!("  mtu: {}\n", mtu));
    print(1, &alloc::format!("  mac: {}\n", mac));
    if !dns.is_empty() {
        print(1, &alloc::format!("  dns: {}\n", dns));
    }

    print(1, "  routes:\n");
    if routes.is_empty() {
        print(1, "    (none)\n");
    } else {
        for line in routes.lines() {
            print(1, &alloc::format!("    {}\n", line));
        }
    }

    Ok(())
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let args = get_args();

    let result = if args.len() <= 1 {
        print_summary()
    } else {
        match args[1].as_str() {
            "addr" => print_addr(),
            "link" => print_link(),
            "route" | "routes" => print_route(),
            "dns" => print_dns(),
            "stats" => print_stats(),
            "-h" | "--help" | "help" => {
                print_usage();
                Ok(())
            }
            _ => {
                print_usage();
                Err("unknown subcommand")
            }
        }
    };

    match result {
        Ok(()) => stem::syscall::exit(0),
        Err(e) => {
            print(2, &alloc::format!("ip: {}\n", e));
            stem::syscall::exit(1)
        }
    }
}
