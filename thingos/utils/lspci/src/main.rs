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
    show_domain: bool,
    help: bool,
}

struct PciDevice {
    slot: String,
    vendor: u16,
    device: u16,
    class: u32,
    kind: String,
    bars: [(u64, u64); 6],
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
            "-D" | "--domain" => flags.show_domain = true,
            _ if arg.starts_with('-') => {
                for ch in arg.chars().skip(1) {
                    match ch {
                        'n' => flags.numeric = true,
                        'v' => flags.verbose = true,
                        'D' => flags.show_domain = true,
                        'h' => flags.help = true,
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

fn read_bar(path: &str) -> (u64, u64) {
    let Ok(text) = read_text(path) else {
        return (0, 0);
    };
    let mut parts = text.split_whitespace();
    let addr = parts
        .next()
        .and_then(|s| u64::from_str_radix(s.trim_start_matches("0x"), 16).ok())
        .unwrap_or(0);
    let size = parts
        .next()
        .and_then(|s| u64::from_str_radix(s.trim_start_matches("0x"), 16).ok())
        .unwrap_or(0);
    (addr, size)
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

fn load_devices() -> Result<Vec<PciDevice>, ()> {
    let mut devices = Vec::new();
    for name in read_dir_entries(SYS_DEVICES)? {
        if !name.starts_with("pci-") {
            continue;
        }

        let base = alloc::format!("{}/{}", SYS_DEVICES, name);
        let vendor = read_hex_u32(&alloc::format!("{}/vendor", base)).unwrap_or(0) as u16;
        let device = read_hex_u32(&alloc::format!("{}/device", base)).unwrap_or(0) as u16;
        let class = read_hex_u32(&alloc::format!("{}/class", base)).unwrap_or(0);
        let kind =
            read_text(&alloc::format!("{}/kind", base)).unwrap_or_else(|_| "pci_device".into());

        let mut bars = [(0, 0); 6];
        for (index, bar) in bars.iter_mut().enumerate() {
            *bar = read_bar(&alloc::format!("{}/bar{}", base, index));
        }

        devices.push(PciDevice { slot: name, vendor, device, class, kind, bars });
    }
    Ok(devices)
}

fn short_slot(slot: &str, show_domain: bool) -> &str {
    let Some(rest) = slot.strip_prefix("pci-") else {
        return slot;
    };
    if show_domain { rest } else { rest.split_once(':').map(|(_, bdf)| bdf).unwrap_or(rest) }
}

fn class_name(class: u32) -> &'static str {
    let base = ((class >> 16) & 0xff) as u8;
    let sub = ((class >> 8) & 0xff) as u8;
    let prog = (class & 0xff) as u8;

    match (base, sub, prog) {
        (0x00, 0x00, _) => "Non-VGA unclassified device",
        (0x00, 0x01, _) => "VGA compatible unclassified device",
        (0x01, 0x00, _) => "SCSI storage controller",
        (0x01, 0x01, _) => "IDE interface",
        (0x01, 0x02, _) => "Floppy disk controller",
        (0x01, 0x03, _) => "IPI bus controller",
        (0x01, 0x04, _) => "RAID bus controller",
        (0x01, 0x05, _) => "ATA controller",
        (0x01, 0x06, _) => "SATA controller",
        (0x01, 0x07, _) => "Serial Attached SCSI controller",
        (0x01, 0x08, _) => "Non-Volatile memory controller",
        (0x01, _, _) => "Mass storage controller",
        (0x02, 0x00, _) => "Ethernet controller",
        (0x02, 0x01, _) => "Token ring network controller",
        (0x02, 0x02, _) => "FDDI network controller",
        (0x02, 0x03, _) => "ATM network controller",
        (0x02, 0x04, _) => "ISDN controller",
        (0x02, 0x07, _) => "Infiniband controller",
        (0x02, _, _) => "Network controller",
        (0x03, 0x00, _) => "VGA compatible controller",
        (0x03, 0x01, _) => "XGA compatible controller",
        (0x03, 0x02, _) => "3D controller",
        (0x03, _, _) => "Display controller",
        (0x04, 0x00, _) => "Multimedia video controller",
        (0x04, 0x01, _) => "Multimedia audio controller",
        (0x04, 0x02, _) => "Computer telephony device",
        (0x04, 0x03, _) => "Audio device",
        (0x04, _, _) => "Multimedia controller",
        (0x05, 0x00, _) => "RAM memory",
        (0x05, 0x01, _) => "FLASH memory",
        (0x05, _, _) => "Memory controller",
        (0x06, 0x00, _) => "Host bridge",
        (0x06, 0x01, _) => "ISA bridge",
        (0x06, 0x02, _) => "EISA bridge",
        (0x06, 0x03, _) => "MicroChannel bridge",
        (0x06, 0x04, _) => "PCI bridge",
        (0x06, 0x05, _) => "PCMCIA bridge",
        (0x06, 0x06, _) => "NuBus bridge",
        (0x06, 0x07, _) => "CardBus bridge",
        (0x06, 0x08, _) => "RACEway bridge",
        (0x06, 0x09, _) => "Semi-transparent PCI-to-PCI bridge",
        (0x06, 0x0a, _) => "InfiniBand to PCI host bridge",
        (0x06, _, _) => "Bridge",
        (0x07, 0x00, _) => "Serial controller",
        (0x07, 0x01, _) => "Parallel controller",
        (0x07, 0x02, _) => "Multiport serial controller",
        (0x07, 0x03, _) => "Modem",
        (0x07, _, _) => "Communication controller",
        (0x08, 0x00, _) => "PIC",
        (0x08, 0x01, _) => "DMA controller",
        (0x08, 0x02, _) => "Timer",
        (0x08, 0x03, _) => "RTC",
        (0x08, 0x04, _) => "PCI Hot-plug controller",
        (0x08, _, _) => "System peripheral",
        (0x09, 0x00, _) => "Keyboard controller",
        (0x09, 0x01, _) => "Digitizer pen",
        (0x09, 0x02, _) => "Mouse controller",
        (0x09, 0x03, _) => "Scanner controller",
        (0x09, 0x04, _) => "Gameport controller",
        (0x09, _, _) => "Input device controller",
        (0x0a, _, _) => "Docking station",
        (0x0b, 0x00, _) => "386 processor",
        (0x0b, 0x01, _) => "486 processor",
        (0x0b, 0x02, _) => "Pentium processor",
        (0x0b, 0x10, _) => "Alpha processor",
        (0x0b, 0x20, _) => "PowerPC processor",
        (0x0b, 0x30, _) => "MIPS processor",
        (0x0b, 0x40, _) => "Co-processor",
        (0x0b, _, _) => "Processor",
        (0x0c, 0x00, _) => "FireWire controller",
        (0x0c, 0x01, _) => "ACCESS bus controller",
        (0x0c, 0x02, _) => "SSA controller",
        (0x0c, 0x03, 0x00 | 0x10) => "USB controller",
        (0x0c, 0x03, 0x20) => "EHCI USB controller",
        (0x0c, 0x03, 0x30) => "XHCI USB controller",
        (0x0c, 0x03, _) => "USB controller",
        (0x0c, 0x04, _) => "Fibre Channel",
        (0x0c, 0x05, _) => "SMBus",
        (0x0c, 0x06, _) => "InfiniBand controller",
        (0x0c, 0x07, _) => "IPMI interface",
        (0x0c, 0x08, _) => "SERCOS interface",
        (0x0c, 0x09, _) => "CANBUS controller",
        (0x0c, _, _) => "Serial bus controller",
        (0x0d, _, _) => "Wireless controller",
        (0x0e, _, _) => "Intelligent controller",
        (0x0f, _, _) => "Satellite communications controller",
        (0x10, _, _) => "Encryption controller",
        (0x11, _, _) => "Signal processing controller",
        (0x12, _, _) => "Processing accelerators",
        (0x13, _, _) => "Non-Essential Instrumentation",
        (0x40, _, _) => "Co-processor",
        (0xff, _, _) => "Unassigned class",
        _ => "Unknown device class",
    }
}

fn id_name(vendor: u16, device: u16) -> String {
    let (vendor_name, device_name) = stem::pci::lookup_names(vendor, device);
    match (vendor_name, device_name) {
        (Some(v), Some(d)) => alloc::format!("{} {}", v, d),
        (Some(v), None) => alloc::format!("{} Device {:04x}", v, device),
        (None, _) => alloc::format!("Device {:04x}:{:04x}", vendor, device),
    }
}

fn print_device(dev: &PciDevice, flags: &Flags) {
    let slot = short_slot(&dev.slot, flags.show_domain);
    if flags.numeric {
        print(
            1,
            &alloc::format!("{} {:06x}: {:04x}:{:04x}\n", slot, dev.class, dev.vendor, dev.device),
        );
    } else {
        print(
            1,
            &alloc::format!(
                "{} {}: {} [{:04x}:{:04x}]\n",
                slot,
                class_name(dev.class),
                id_name(dev.vendor, dev.device),
                dev.vendor,
                dev.device
            ),
        );
    }

    if flags.verbose {
        print(1, &alloc::format!("\tSysfs: {}/{}\n", SYS_DEVICES, dev.slot));
        print(1, &alloc::format!("\tKind: {}\n", dev.kind));
        print(1, &alloc::format!("\tClass: 0x{:06x}\n", dev.class));
        print(1, &alloc::format!("\tVendor: 0x{:04x} Device: 0x{:04x}\n", dev.vendor, dev.device));
        for (index, (addr, size)) in dev.bars.iter().copied().enumerate() {
            if addr != 0 || size != 0 {
                print(
                    1,
                    &alloc::format!(
                        "\tBAR{}: resource at 0x{:x} [size=0x{:x}]\n",
                        index,
                        addr,
                        size
                    ),
                );
            }
        }
    }
}

fn print_usage() {
    print(1, "usage: lspci [-n] [-v] [-D]\n");
    print(1, "  -n, --numeric  show numeric class and vendor/device IDs only\n");
    print(1, "  -v, --verbose  include sysfs path, kind, IDs, and BAR resources\n");
    print(1, "  -D, --domain   include the PCI domain in the slot address\n");
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let args = get_args();
    let flags = match parse_flags(&args) {
        Ok(flags) => flags,
        Err(err) => {
            print(2, &alloc::format!("lspci: {}\n", err));
            print_usage();
            exit(2);
        }
    };

    if flags.help {
        print_usage();
        exit(0);
    }

    let devices = match load_devices() {
        Ok(devices) => devices,
        Err(_) => {
            print(2, "lspci: unable to read /sys/devices\n");
            exit(1);
        }
    };

    if devices.is_empty() {
        print(1, "lspci: no PCI devices found\n");
    } else {
        for dev in &devices {
            print_device(dev, &flags);
        }
    }

    exit(0)
}
