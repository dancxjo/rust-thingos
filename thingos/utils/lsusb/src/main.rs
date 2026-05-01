#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use stem::abi::syscall::vfs_flags::O_RDONLY;
use stem::syscall::{argv_get, exit, vfs_close, vfs_open, vfs_read, vfs_readdir, vfs_write};

const SYS_DEVICES: &str = "/sys/devices";

#[derive(Default)]
struct Flags {
    numeric: bool,
    verbose: bool,
    tree: bool,
    help: bool,
}

struct UsbEntry {
    bus: String,
    device: String,
    vendor: u16,
    product: u16,
    class: u8,
    subclass: u8,
    protocol: u8,
    speed: Option<String>,
    name: String,
    kind: String,
    sysfs: String,
    is_controller: bool,
}

fn print(fd: u32, s: &str) {
    let _ = vfs_write(fd, s.as_bytes());
}

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

    let raw_args = stem::utils::parse_argv(&buf);
    let mut args = Vec::new();
    for arg in raw_args {
        if let Ok(s) = core::str::from_utf8(arg) {
            args.push(String::from(s));
        }
    }
    args
}

fn parse_flags(args: &[String]) -> Result<Flags, String> {
    let mut flags = Flags::default();
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-h" | "--help" => flags.help = true,
            "-n" | "--numeric" => flags.numeric = true,
            "-v" | "--verbose" => flags.verbose = true,
            "-t" | "--tree" => flags.tree = true,
            _ if arg.starts_with('-') => {
                for ch in arg.chars().skip(1) {
                    match ch {
                        'h' => flags.help = true,
                        'n' => flags.numeric = true,
                        'v' => flags.verbose = true,
                        't' => flags.tree = true,
                        _ => return Err(alloc::format!("unknown option '{}'", arg)),
                    }
                }
            }
            _ => return Err(alloc::format!("unexpected argument '{}'", arg)),
        }
    }
    Ok(flags)
}

fn read_text(path: &str) -> Result<String, ()> {
    let fd = vfs_open(path, O_RDONLY).map_err(|_| ())?;
    let mut buf = alloc::vec![0u8; 256];
    let n = match vfs_read(fd, &mut buf) {
        Ok(n) => n,
        Err(_) => {
            let _ = vfs_close(fd);
            return Err(());
        }
    };
    let _ = vfs_close(fd);
    buf.truncate(n);
    Ok(String::from_utf8_lossy(&buf).trim().to_string())
}

fn read_hex_u32(path: &str) -> Result<u32, ()> {
    let text = read_text(path)?;
    let text = text.trim().trim_start_matches("0x");
    u32::from_str_radix(text, 16).map_err(|_| ())
}

fn read_dec_u32(path: &str) -> Result<u32, ()> {
    read_text(path)?.parse::<u32>().map_err(|_| ())
}

fn read_dir_entries(path: &str) -> Result<Vec<String>, ()> {
    let fd = vfs_open(path, O_RDONLY).map_err(|_| ())?;
    let mut entries = Vec::new();
    let mut buf = [0u8; 4096];

    loop {
        match vfs_readdir(fd, &mut buf) {
            Ok(0) => break,
            Ok(n) => {
                let mut offset = 0;
                while offset < n {
                    let mut end = offset;
                    while end < n && buf[end] != 0 {
                        end += 1;
                    }
                    if end > offset {
                        if let Ok(name) = core::str::from_utf8(&buf[offset..end]) {
                            entries.push(String::from(name));
                        }
                    }
                    offset = end + 1;
                }
            }
            Err(_) => {
                let _ = vfs_close(fd);
                return Err(());
            }
        }
    }

    let _ = vfs_close(fd);
    entries.sort();
    entries.dedup();
    Ok(entries)
}

fn load_entries() -> Result<Vec<UsbEntry>, ()> {
    let mut entries = Vec::new();
    for name in read_dir_entries(SYS_DEVICES)? {
        let base = alloc::format!("{}/{}", SYS_DEVICES, name);
        if name.starts_with("usb-") {
            if let Some(entry) = load_usb_device(&base) {
                entries.push(entry);
            }
        } else if name.starts_with("pci-") {
            if let Some(entry) = load_usb_controller(&name, &base) {
                entries.push(entry);
            }
        }
    }

    entries.sort_by(|a, b| {
        a.bus.cmp(&b.bus).then(a.device.cmp(&b.device)).then(a.sysfs.cmp(&b.sysfs))
    });
    Ok(entries)
}

fn load_usb_device(base: &str) -> Option<UsbEntry> {
    let vendor =
        read_hex_any(&[alloc::format!("{}/idVendor", base), alloc::format!("{}/vendor", base)], 0)
            as u16;
    let product = read_hex_any(
        &[
            alloc::format!("{}/idProduct", base),
            alloc::format!("{}/product_id", base),
            alloc::format!("{}/device", base),
        ],
        0,
    ) as u16;
    let class = read_hex_any(
        &[alloc::format!("{}/bDeviceClass", base), alloc::format!("{}/class", base)],
        0,
    ) as u8;
    let subclass = read_hex_any(
        &[alloc::format!("{}/bDeviceSubClass", base), alloc::format!("{}/subclass", base)],
        0,
    ) as u8;
    let protocol = read_hex_any(
        &[alloc::format!("{}/bDeviceProtocol", base), alloc::format!("{}/protocol", base)],
        0,
    ) as u8;

    let bus = read_dec_u32(&alloc::format!("{}/busnum", base))
        .map(|n| alloc::format!("{:03}", n))
        .unwrap_or_else(|_| "000".to_string());
    let device = read_dec_u32(&alloc::format!("{}/devnum", base))
        .map(|n| alloc::format!("{:03}", n))
        .unwrap_or_else(|_| "000".to_string());
    let speed = read_text(&alloc::format!("{}/speed", base)).ok();
    let manufacturer = read_text(&alloc::format!("{}/manufacturer", base)).ok();
    let product_name = read_text(&alloc::format!("{}/product", base)).ok();
    let name = match (manufacturer, product_name) {
        (Some(m), Some(p)) if !m.is_empty() && !p.is_empty() => alloc::format!("{} {}", m, p),
        (_, Some(p)) if !p.is_empty() => p,
        _ => id_name(vendor, product),
    };
    let kind = read_text(&alloc::format!("{}/kind", base)).unwrap_or_else(|_| "usb_device".into());

    Some(UsbEntry {
        bus,
        device,
        vendor,
        product,
        class,
        subclass,
        protocol,
        speed,
        name,
        kind,
        sysfs: String::from(base),
        is_controller: false,
    })
}

fn load_usb_controller(name: &str, base: &str) -> Option<UsbEntry> {
    let class_full = read_hex_u32(&alloc::format!("{}/class", base)).ok()?;
    let class = ((class_full >> 16) & 0xff) as u8;
    let subclass = ((class_full >> 8) & 0xff) as u8;
    let protocol = (class_full & 0xff) as u8;
    if class != 0x0c || subclass != 0x03 {
        return None;
    }

    let vendor = read_hex_u32(&alloc::format!("{}/vendor", base)).unwrap_or(0) as u16;
    let product = read_hex_u32(&alloc::format!("{}/device", base)).unwrap_or(0) as u16;
    let (_, bdf) = name.strip_prefix("pci-").and_then(|s| s.split_once(':')).unwrap_or(("", name));
    let controller_name = controller_name(protocol);
    let pci_name = pci_id_name(vendor, product);

    Some(UsbEntry {
        bus: "pci".to_string(),
        device: bdf.to_string(),
        vendor,
        product,
        class,
        subclass,
        protocol,
        speed: None,
        name: alloc::format!("{} {}", pci_name, controller_name),
        kind: read_text(&alloc::format!("{}/kind", base)).unwrap_or_else(|_| "pci_usb".into()),
        sysfs: String::from(base),
        is_controller: true,
    })
}

fn read_hex_any(paths: &[String], default: u32) -> u32 {
    for path in paths {
        if let Ok(value) = read_hex_u32(path) {
            return value;
        }
    }
    default
}

fn controller_name(protocol: u8) -> &'static str {
    match protocol {
        0x00 => "UHCI host controller",
        0x10 => "OHCI host controller",
        0x20 => "EHCI host controller",
        0x30 => "xHCI host controller",
        0x80 => "USB device controller",
        _ => "USB host controller",
    }
}

fn class_name(class: u8, subclass: u8, protocol: u8) -> &'static str {
    match (class, subclass, protocol) {
        (0x00, _, _) => "per-interface",
        (0x01, 0x01, _) => "audio-control",
        (0x01, 0x02, _) => "audio-streaming",
        (0x01, _, _) => "audio",
        (0x02, _, _) => "communications",
        (0x03, 0x01, 0x01) => "keyboard",
        (0x03, 0x01, 0x02) => "mouse",
        (0x03, _, _) => "hid",
        (0x05, _, _) => "physical",
        (0x06, _, _) => "still-image",
        (0x07, _, _) => "printer",
        (0x08, 0x06, 0x50) => "mass-storage scsi-bot",
        (0x08, _, _) => "mass-storage",
        (0x09, _, _) => "hub",
        (0x0a, _, _) => "cdc-data",
        (0x0b, _, _) => "smart-card",
        (0x0d, _, _) => "content-security",
        (0x0e, _, _) => "video",
        (0x0f, _, _) => "personal-healthcare",
        (0xdc, _, _) => "diagnostic",
        (0xe0, _, _) => "wireless",
        (0xef, _, _) => "miscellaneous",
        (0xfe, _, _) => "application-specific",
        (0xff, _, _) => "vendor-specific",
        _ => "unknown",
    }
}

fn pci_id_name(vendor: u16, device: u16) -> String {
    let (vendor_name, device_name) = stem::pci::lookup_names(vendor, device);
    match (vendor_name, device_name) {
        (Some(v), Some(d)) => alloc::format!("{} {}", v, d),
        (Some(v), None) => alloc::format!("{} Device {:04x}", v, device),
        (None, _) => alloc::format!("Device {:04x}:{:04x}", vendor, device),
    }
}

fn id_name(vendor: u16, product: u16) -> String {
    match usb_vendor_name(vendor) {
        Some(vendor_name) => alloc::format!("{} Device {:04x}", vendor_name, product),
        None => alloc::format!("Device {:04x}:{:04x}", vendor, product),
    }
}

fn usb_vendor_name(vendor: u16) -> Option<&'static str> {
    match vendor {
        0x03f0 => Some("HP"),
        0x046d => Some("Logitech"),
        0x05ac => Some("Apple"),
        0x05e3 => Some("Genesys Logic"),
        0x0781 => Some("SanDisk"),
        0x090c => Some("Silicon Motion"),
        0x0951 => Some("Kingston"),
        0x0bda => Some("Realtek"),
        0x13fe => Some("Phison"),
        0x154b => Some("PNY"),
        0x1d6b => Some("Linux Foundation"),
        0x1e3d => Some("Chipsbank"),
        0x8564 => Some("Transcend"),
        _ => None,
    }
}

fn print_entry(entry: &UsbEntry, flags: &Flags) {
    if flags.tree {
        let speed = entry.speed.as_deref().unwrap_or("-");
        print(
            1,
            &alloc::format!(
                "/:  Bus {} Dev {} Class={} Driver={} Speed={}\n",
                entry.bus,
                entry.device,
                class_name(entry.class, entry.subclass, entry.protocol),
                entry.kind,
                speed
            ),
        );
        return;
    }

    if flags.numeric {
        print(
            1,
            &alloc::format!(
                "Bus {} Device {}: ID {:04x}:{:04x} Class {:02x}:{:02x}:{:02x}\n",
                entry.bus,
                entry.device,
                entry.vendor,
                entry.product,
                entry.class,
                entry.subclass,
                entry.protocol
            ),
        );
    } else if entry.is_controller {
        print(
            1,
            &alloc::format!(
                "Bus {} Device {}: ID {:04x}:{:04x} {}\n",
                entry.bus,
                entry.device,
                entry.vendor,
                entry.product,
                entry.name
            ),
        );
    } else {
        print(
            1,
            &alloc::format!(
                "Bus {} Device {}: ID {:04x}:{:04x} {}\n",
                entry.bus,
                entry.device,
                entry.vendor,
                entry.product,
                entry.name
            ),
        );
    }

    if flags.verbose {
        print(1, &alloc::format!("\tSysfs: {}\n", entry.sysfs));
        print(1, &alloc::format!("\tKind: {}\n", entry.kind));
        print(
            1,
            &alloc::format!(
                "\tClass: {:02x} ({}) SubClass: {:02x} Protocol: {:02x}\n",
                entry.class,
                class_name(entry.class, entry.subclass, entry.protocol),
                entry.subclass,
                entry.protocol
            ),
        );
        if let Some(speed) = &entry.speed {
            print(1, &alloc::format!("\tSpeed: {}\n", speed));
        }
        if entry.is_controller {
            print(1, "\tRole: USB host controller\n");
        }
    }
}

fn print_usage() {
    print(1, "usage: lsusb [-n] [-v] [-t]\n");
    print(1, "  -n, --numeric  show numeric class and vendor/product IDs only\n");
    print(1, "  -v, --verbose  include sysfs path, kind, class, protocol, and speed\n");
    print(1, "  -t, --tree     show a compact topology-style view\n");
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let args = get_args();
    let flags = match parse_flags(&args) {
        Ok(flags) => flags,
        Err(err) => {
            print(2, &alloc::format!("lsusb: {}\n", err));
            print_usage();
            exit(2);
        }
    };

    if flags.help {
        print_usage();
        exit(0);
    }

    let entries = match load_entries() {
        Ok(entries) => entries,
        Err(_) => {
            print(2, "lsusb: unable to read /sys/devices\n");
            exit(1);
        }
    };

    if entries.is_empty() {
        print(1, "lsusb: no USB devices found\n");
    } else {
        for entry in &entries {
            print_entry(entry, &flags);
        }
    }

    exit(0)
}
