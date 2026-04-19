#![no_std]
#![no_main]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use abi::errors::Errno;
use abi::seed::{
    INTERFACE_VFS_PROVIDER_MOUNT_V1, INTERFACE_VFS_PROVIDER_UNMOUNT_V1, SEED_ABI_VERSION,
    SEED_SYMBOL, Seed,
};
use stem::syscall::{argv_get, exit, spawn_driver_ex, vfs_close, vfs_open, vfs_read, vfs_write};

const MOUNT_VERIFICATION_ATTEMPTS: usize = 50;
const MOUNT_VERIFICATION_DELAY_MS: u64 = 100;
const READ_FILE_CHUNK_SIZE: usize = 1024;

fn get_args() -> Vec<String> {
    let len = match argv_get(&mut []) {
        Ok(l) if l > 0 => l,
        _ => return Vec::new(),
    };
    let mut buf = alloc::vec![0u8; len];
    if argv_get(&mut buf).is_err() {
        return Vec::new();
    }

    stem::utils::parse_argv(&buf)
        .into_iter()
        .skip(1)
        .filter_map(|b| core::str::from_utf8(b).ok().map(String::from))
        .collect()
}

fn out(msg: &str) {
    let _ = vfs_write(1, msg.as_bytes());
}

fn err(msg: &str) {
    let _ = vfs_write(2, msg.as_bytes());
}

#[stem::main]
fn main(_arg: usize) -> ! {
    let args = get_args();
    if args.is_empty() {
        print_usage();
        exit(2);
    }

    if args.len() == 1 && args[0] == "-a" {
        let status = mount_all_from_fstab("/etc/fstab");
        exit(status);
    }

    if args.len() != 3 || args[0] != "-t" {
        print_usage();
        exit(2);
    }

    let fs_type = args[1].as_str();
    let target = args[2].as_str();
    match mount_one(fs_type, target) {
        Ok(()) => {
            exit(0);
        }
        Err(e) => {
            err(&alloc::format!("mount: failed to mount {} on {}: {:?}\n", fs_type, target, e));
            exit(1);
        }
    }
}

fn print_usage() {
    err("usage: mount -t <type> <target>\n       mount -a\n");
}

fn mount_all_from_fstab(path: &str) -> i32 {
    let Some(data) = read_file(path, 64 * 1024) else {
        err("mount: cannot read /etc/fstab\n");
        return 1;
    };
    let Ok(text) = core::str::from_utf8(&data) else {
        err("mount: /etc/fstab is not valid UTF-8\n");
        return 1;
    };

    let mut had_error = false;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = line.split_whitespace().collect();
        let (fs_type, target) = if fields.len() >= 3 {
            (fields[2], fields[1])
        } else if fields.len() >= 2 {
            (fields[0], fields[1])
        } else {
            continue;
        };

        if let Err(e) = mount_one(fs_type, target) {
            had_error = true;
            err(&alloc::format!(
                "mount: fstab entry failed for type={} target={}: {:?}\n",
                fs_type,
                target,
                e
            ));
        }
    }

    if had_error { 1 } else { 0 }
}

fn mount_one(fs_type: &str, target: &str) -> Result<(), Errno> {
    let provider_path = resolve_provider_binary(fs_type).ok_or(Errno::ENOENT)?;
    let bytes = read_file(&provider_path, 8 * 1024 * 1024).ok_or(Errno::EINVAL)?;
    let mount_sym = resolve_provider_mount_symbol(&bytes);

    let argv = [provider_path.as_bytes(), target.as_bytes()];
    let _resp =
        spawn_driver_ex(&provider_path, &argv, &BTreeMap::new(), 0, &[], mount_sym.as_deref())?;

    wait_for_mount(target, MOUNT_VERIFICATION_ATTEMPTS, MOUNT_VERIFICATION_DELAY_MS)?;
    out(&alloc::format!("mounted type={} target={}\n", fs_type, target));
    Ok(())
}

fn resolve_provider_mount_symbol(bytes: &[u8]) -> Option<String> {
    if let Some(seed_vaddr) = resolve_elf64_symbol_from_bytes(bytes, SEED_SYMBOL) {
        if let Some(seed) = read_seed_descriptor(bytes, seed_vaddr) {
            if seed.abi_version == SEED_ABI_VERSION {
                if let Some(mount_iface) = seed.interface(INTERFACE_VFS_PROVIDER_MOUNT_V1, 1) {
                    if let Some(unmount_iface) =
                        seed.interface(INTERFACE_VFS_PROVIDER_UNMOUNT_V1, 1)
                    {
                        if !mount_iface.entry_symbol_ptr.is_null()
                            && mount_iface.entry_symbol_len > 0
                            && !unmount_iface.entry_symbol_ptr.is_null()
                            && unmount_iface.entry_symbol_len > 0
                        {
                            if let Some(mount_sym) = read_vaddr_bytes(
                                bytes,
                                mount_iface.entry_symbol_ptr as u64,
                                mount_iface.entry_symbol_len,
                            )
                            .and_then(|s| core::str::from_utf8(s).ok().map(String::from))
                            {
                                return Some(mount_sym);
                            }
                        }
                    }
                }
            }
        }
    }

    if resolve_elf64_symbol_from_bytes(bytes, "_start").is_some() {
        return Some(String::from("_start"));
    }
    if resolve_elf64_symbol_from_bytes(bytes, "thingos_vfs_mount_v1").is_some() {
        return Some(String::from("thingos_vfs_mount_v1"));
    }
    if resolve_elf64_symbol_from_bytes(bytes, "entry_impl").is_some() {
        return Some(String::from("entry_impl"));
    }

    None
}

fn wait_for_mount(target: &str, attempts: usize, delay_ms: u64) -> Result<(), Errno> {
    for _ in 0..attempts {
        if let Ok(fd) = vfs_open(target, abi::syscall::vfs_flags::O_RDONLY) {
            let _ = vfs_close(fd);
            return Ok(());
        }
        stem::time::sleep_ms(delay_ms);
    }
    Err(Errno::ETIMEDOUT)
}

fn resolve_provider_binary(fs_type: &str) -> Option<String> {
    if fs_type.starts_with('/') {
        if path_exists(fs_type) {
            return Some(fs_type.to_string());
        }
        return None;
    }

    let direct = alloc::format!("/bin/{}", fs_type);
    if path_exists(&direct) {
        return Some(direct);
    }
    let daemon = alloc::format!("/bin/{}d", fs_type);
    if path_exists(&daemon) {
        return Some(daemon);
    }
    let driver = alloc::format!("/drivers/{}", fs_type);
    if path_exists(&driver) {
        return Some(driver);
    }
    None
}

fn path_exists(path: &str) -> bool {
    match vfs_open(path, abi::syscall::vfs_flags::O_RDONLY) {
        Ok(fd) => {
            let _ = vfs_close(fd);
            true
        }
        Err(_) => false,
    }
}

fn read_file(path: &str, max_bytes: usize) -> Option<Vec<u8>> {
    let fd = vfs_open(path, abi::syscall::vfs_flags::O_RDONLY).ok()?;
    let mut out = Vec::new();
    let mut buf = [0u8; READ_FILE_CHUNK_SIZE];
    loop {
        let n = vfs_read(fd, &mut buf).ok()?;
        if n == 0 {
            break;
        }
        let next_len = match out.len().checked_add(n) {
            Some(v) => v,
            None => {
                let _ = vfs_close(fd);
                return None;
            }
        };
        if next_len > max_bytes {
            let _ = vfs_close(fd);
            return None;
        }
        out.extend_from_slice(&buf[..n]);
    }
    let _ = vfs_close(fd);
    Some(out)
}

fn read_u16(bytes: &[u8], off: usize) -> Option<u16> {
    let end = off.checked_add(2)?;
    let b = bytes.get(off..end)?;
    Some(u16::from_le_bytes([b[0], b[1]]))
}

fn read_u32(bytes: &[u8], off: usize) -> Option<u32> {
    let end = off.checked_add(4)?;
    let b = bytes.get(off..end)?;
    Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

fn read_u64(bytes: &[u8], off: usize) -> Option<u64> {
    let end = off.checked_add(8)?;
    let b = bytes.get(off..end)?;
    Some(u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]))
}

fn resolve_elf64_symbol_from_bytes(bytes: &[u8], target: &str) -> Option<u64> {
    if bytes.len() < 64 {
        return None;
    }
    if &bytes[0..4] != b"\x7fELF" || bytes[4] != 2 || bytes[5] != 1 {
        return None;
    }

    let e_shoff = read_u64(bytes, 40)? as usize;
    let e_shentsize = read_u16(bytes, 58)? as usize;
    let e_shnum = read_u16(bytes, 60)? as usize;
    if e_shoff == 0 || e_shentsize < 64 || e_shnum == 0 {
        return None;
    }

    for i in 0..e_shnum {
        let sh_off = e_shoff.checked_add(i.checked_mul(e_shentsize)?)?;
        if sh_off.checked_add(e_shentsize).map_or(true, |v| v > bytes.len()) {
            break;
        }
        let sh_type = read_u32(bytes, sh_off.checked_add(4)?)?;
        if sh_type != 2 && sh_type != 11 {
            continue;
        }

        let sh_link = read_u32(bytes, sh_off.checked_add(40)?)? as usize;
        let strtab_sh_off = e_shoff.checked_add(sh_link.checked_mul(e_shentsize)?)?;
        if strtab_sh_off.checked_add(e_shentsize).map_or(true, |v| v > bytes.len()) {
            continue;
        }
        let strtab_off = read_u64(bytes, strtab_sh_off.checked_add(24)?)? as usize;
        let strtab_size = read_u64(bytes, strtab_sh_off.checked_add(32)?)? as usize;
        if strtab_off.checked_add(strtab_size).map_or(true, |v| v > bytes.len()) {
            continue;
        }

        let sym_off = read_u64(bytes, sh_off.checked_add(24)?)? as usize;
        let sym_size = read_u64(bytes, sh_off.checked_add(32)?)? as usize;
        const SYM_ENTRY: usize = 24;
        if sym_size == 0 || sym_off.checked_add(sym_size).map_or(true, |v| v > bytes.len()) {
            continue;
        }

        for s in 0..(sym_size / SYM_ENTRY) {
            let se = sym_off.checked_add(s.checked_mul(SYM_ENTRY)?)?;
            if se.checked_add(SYM_ENTRY).map_or(true, |v| v > bytes.len()) {
                break;
            }
            let st_name = read_u32(bytes, se)? as usize;
            let st_value = read_u64(bytes, se.checked_add(8)?)?;
            if st_value == 0 {
                continue;
            }
            let name_off = strtab_off.checked_add(st_name)?;
            if name_off >= bytes.len() {
                continue;
            }
            let name_end = bytes[name_off..]
                .iter()
                .position(|&b| b == 0)
                .and_then(|n| name_off.checked_add(n))
                .unwrap_or(bytes.len());
            if let Ok(name) = core::str::from_utf8(&bytes[name_off..name_end]) {
                if name == target {
                    return Some(st_value);
                }
            }
        }
    }
    None
}

fn vaddr_to_file_offset(bytes: &[u8], vaddr: u64) -> Option<usize> {
    if bytes.len() < 64 {
        return None;
    }
    let e_phoff = read_u64(bytes, 32)? as usize;
    let e_phentsize = read_u16(bytes, 54)? as usize;
    let e_phnum = read_u16(bytes, 56)? as usize;

    for i in 0..e_phnum {
        let off = e_phoff.checked_add(i.checked_mul(e_phentsize)?)?;
        if off.checked_add(e_phentsize).map_or(true, |v| v > bytes.len()) {
            break;
        }
        let p_type = read_u32(bytes, off)?;
        if p_type != 1 {
            continue;
        }
        let p_offset = read_u64(bytes, off.checked_add(8)?)?;
        let p_vaddr = read_u64(bytes, off.checked_add(16)?)?;
        let p_filesz = read_u64(bytes, off.checked_add(32)?)?;
        let end = p_vaddr.checked_add(p_filesz)?;
        if vaddr >= p_vaddr && vaddr < end {
            let delta = vaddr - p_vaddr;
            let file_off = p_offset.checked_add(delta)?;
            return Some(file_off as usize);
        }
    }
    None
}

fn read_seed_descriptor(bytes: &[u8], sym_vaddr: u64) -> Option<Seed> {
    let file_off = vaddr_to_file_offset(bytes, sym_vaddr)?;
    let size = core::mem::size_of::<Seed>();
    if file_off.checked_add(size).map_or(true, |v| v > bytes.len()) {
        return None;
    }
    let desc: Seed = unsafe {
        let mut tmp = core::mem::MaybeUninit::<Seed>::uninit();
        core::ptr::copy_nonoverlapping(
            bytes.as_ptr().add(file_off),
            tmp.as_mut_ptr() as *mut u8,
            size,
        );
        tmp.assume_init()
    };
    Some(desc)
}

fn read_vaddr_bytes<'a>(bytes: &'a [u8], vaddr: u64, len: usize) -> Option<&'a [u8]> {
    let file_off = vaddr_to_file_offset(bytes, vaddr)?;
    let end = file_off.checked_add(len)?;
    bytes.get(file_off..end)
}
