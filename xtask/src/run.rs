//! QEMU run tasks.

use std::net::TcpListener;
use std::path::Path;

use xshell::Shell;

use crate::common::{Result, image_name};

fn user_netdev_arg() -> String {
    if let Ok(v) = std::env::var("THINGOS_HOSTFWD") {
        let v = v.trim();
        if v.is_empty() || v.eq_ignore_ascii_case("off") || v.eq_ignore_ascii_case("none") {
            return "user,id=n0".to_string();
        }
        if let Ok(port) = v.parse::<u16>() {
            return format!("user,id=n0,hostfwd=tcp::{port}-:80");
        }
        return format!("user,id=n0,hostfwd={v}");
    }

    let mut netdev = String::from("user,id=n0");

    if TcpListener::bind(("127.0.0.1", 8888)).is_ok() {
        netdev.push_str(",hostfwd=tcp::8888-:80");
    } else {
        eprintln!("xtask: host port 8888 is busy; skipping HTTP forward");
    }

    if TcpListener::bind(("127.0.0.1", 2323)).is_ok() {
        netdev.push_str(",hostfwd=tcp::2323-:2323");
    } else {
        eprintln!("xtask: host port 2323 is busy; skipping telnet forward");
    }

    netdev
}

fn x86_qemu_trace_enabled() -> bool {
    if let Ok(v) = std::env::var("THINGOS_QEMU_TRACE") {
        let v = v.trim();
        if v.is_empty() {
            return false;
        }
        !matches!(v.to_ascii_lowercase().as_str(), "0" | "off" | "false" | "no")
    } else {
        false
    }
}

fn has_user_audio_args(qemu_flags: &str) -> bool {
    let f = qemu_flags;
    f.contains("-audiodev")
        || f.contains("virtio-sound")
        || f.contains("intel-hda")
        || f.contains("ich9-intel-hda")
        || f.contains("hda-output")
        || f.contains("hda-duplex")
        || f.contains("ac97")
}

fn has_user_display_args(qemu_flags: &str) -> bool {
    let f = qemu_flags;
    f.contains("-display") || f.contains("-nographic") || f.contains("-vnc")
}

fn push_x86_interactive_display_args<'a>(
    args: &mut Vec<&'a str>,
    interactive: bool,
    qemu_flags: &str,
) {
    if !interactive {
        return;
    }

    args.extend_from_slice(&["-device", "virtio-vga", "-M", "q35,vmport=off,i8042=on"]);

    if !has_user_display_args(qemu_flags) {
        // Bloom draws/manages the guest cursor. Forcing QEMU's host cursor
        // visible makes interactive runs show two differently scaled cursors.
        args.extend_from_slice(&["-display", "default,show-cursor=off"]);
    }
}

fn default_audiodev_arg() -> Option<String> {
    let backend = std::env::var("THINGOS_AUDIODEV").unwrap_or_else(|_| "pa".to_string());
    let backend = backend.trim();
    if backend.is_empty()
        || backend.eq_ignore_ascii_case("off")
        || backend.eq_ignore_ascii_case("none")
    {
        None
    } else if backend.split(',').any(|part| part.trim_start().starts_with("id=")) {
        Some(backend.to_string())
    } else {
        Some(format!("{backend},id=audio0"))
    }
}

fn push_stdio_serial_args(final_args: &mut Vec<String>, interactive: bool, monitor: bool) {
    if !interactive {
        final_args.push("-nographic".to_string());
        return;
    }

    if monitor {
        final_args.extend([
            "-chardev".to_string(),
            "stdio,mux=on,id=char0,signal=off".to_string(),
            "-mon".to_string(),
            "chardev=char0,mode=readline".to_string(),
            "-serial".to_string(),
            "chardev:char0".to_string(),
        ]);
    } else {
        final_args.extend([
            "-chardev".to_string(),
            "stdio,id=char0,signal=off".to_string(),
            "-serial".to_string(),
            "chardev:char0".to_string(),
        ]);
    }
}

pub fn run(
    sh: &Shell,
    arch: &str,
    qemu_flags: &str,
    iso_path: &Path,
    interactive: bool,
    monitor: bool,
) -> Result<()> {
    let name = image_name(arch);
    let iso = iso_path.to_str().unwrap();
    let ovmf_code = format!("vendor/ovmf/ovmf-code-{arch}.fd");
    let ovmf_vars = format!("vendor/ovmf/ovmf-vars-{arch}.fd");

    println!("Running {name} in QEMU...");

    let qemu_args: Vec<&str> = qemu_flags.split_whitespace().collect();
    let mut final_args = Vec::new();
    push_stdio_serial_args(&mut final_args, interactive, monitor);

    let netdev = user_netdev_arg();
    match arch {
        "x86_64" => {
            let pflash0 = format!("if=pflash,unit=0,format=raw,file={ovmf_code},readonly=on");
            let pflash1 = format!("if=pflash,unit=1,format=raw,file={ovmf_vars}");
            let usb_boot_drive = format!("file={iso},if=none,id=usbstick,format=raw,readonly=on");
            let netdev_arg =
                "virtio-net-pci,netdev=n0,mac=52:54:00:12:34:56,disable-legacy=on".to_string();
            let netdev_val = netdev.to_string();

            let mut args = if x86_qemu_trace_enabled() {
                vec![
                    "-M",
                    "q35",
                    "-drive",
                    &pflash0,
                    "-drive",
                    &pflash1,
                    "-drive",
                    &usb_boot_drive,
                    "-device",
                    "qemu-xhci,id=xhci",
                    "-device",
                    "usb-storage,bus=xhci.0,drive=usbstick,bootindex=1",
                    "-no-reboot",
                    "-d",
                    "int,cpu_reset",
                    "-D",
                    "qemu.log",
                    "-device",
                    &netdev_arg,
                    "-netdev",
                    &netdev_val,
                ]
            } else {
                vec![
                    "-M",
                    "q35",
                    "-drive",
                    &pflash0,
                    "-drive",
                    &pflash1,
                    "-drive",
                    &usb_boot_drive,
                    "-device",
                    "qemu-xhci,id=xhci",
                    "-device",
                    "usb-storage,bus=xhci.0,drive=usbstick,bootindex=1",
                    "-no-reboot",
                    "-device",
                    &netdev_arg,
                    "-netdev",
                    &netdev_val,
                ]
            };

            push_x86_interactive_display_args(&mut args, interactive, qemu_flags);

            let default_audio =
                if !has_user_audio_args(qemu_flags) { default_audiodev_arg() } else { None };

            if !has_user_audio_args(qemu_flags) {
                if let Some(audiodev) = default_audio.as_deref() {
                    args.extend_from_slice(&[
                        "-audiodev",
                        audiodev,
                        "-device",
                        "virtio-sound-pci,audiodev=audio0",
                    ]);
                }
            }

            args.extend(final_args.iter().map(String::as_str));
            run_qemu(sh, "qemu-system-x86_64", &args, &qemu_args)?;
        }
        "aarch64" => {
            let pflash0 = format!("if=pflash,unit=0,format=raw,file={ovmf_code},readonly=on");
            let pflash1 = format!("if=pflash,unit=1,format=raw,file={ovmf_vars}");
            let mut args = vec![
                "-M",
                "virt",
                "-cpu",
                "cortex-a72",
                "-semihosting",
                "-drive",
                &pflash0,
                "-drive",
                &pflash1,
                "-cdrom",
                iso,
            ];
            if interactive {
                args.extend_from_slice(&[
                    "-device",
                    "ramfb",
                    "-device",
                    "qemu-xhci",
                    "-device",
                    "usb-kbd",
                    "-device",
                    "usb-mouse",
                ]);
            }
            args.extend(final_args.iter().map(String::as_str));
            run_qemu(sh, "qemu-system-aarch64", &args, &qemu_args)?;
        }
        "riscv64" => {
            let pflash0 =
                format!("node-name=pflash0,driver=file,read-only=on,filename={ovmf_code}");
            let pflash1 = format!("node-name=pflash1,driver=file,filename={ovmf_vars}");
            let drive0 = format!("file={iso},format=raw,if=none,id=drive0,readonly=on");
            let mut args = vec![
                "-blockdev",
                &pflash0,
                "-blockdev",
                &pflash1,
                "-M",
                "virt,pflash0=pflash0,pflash1=pflash1",
                "-cpu",
                "rv64",
                "-m",
                "2G",
                "-drive",
                &drive0,
                "-device",
                "virtio-blk-device,drive=drive0",
            ];
            if interactive {
                args.extend_from_slice(&[
                    "-device",
                    "ramfb",
                    "-device",
                    "qemu-xhci",
                    "-device",
                    "usb-kbd",
                    "-device",
                    "usb-mouse",
                ]);
            }
            args.extend(final_args.iter().map(String::as_str));
            run_qemu(sh, "qemu-system-riscv64", &args, &qemu_args)?;
        }
        "loongarch64" => {
            let pflash0 = format!("if=pflash,unit=0,format=raw,file={ovmf_code},readonly=on");
            let pflash1 = format!("if=pflash,unit=1,format=raw,file={ovmf_vars}");
            let mut args = vec![
                "-M", "virt", "-cpu", "la464", "-drive", &pflash0, "-drive", &pflash1, "-cdrom",
                iso,
            ];
            if interactive {
                args.extend_from_slice(&[
                    "-device",
                    "ramfb",
                    "-device",
                    "qemu-xhci",
                    "-device",
                    "usb-kbd",
                    "-device",
                    "usb-mouse",
                ]);
            }
            args.extend(final_args.iter().map(String::as_str));
            run_qemu(sh, "qemu-system-loongarch64", &args, &qemu_args)?;
        }
        _ => return Err(anyhow::anyhow!("Unsupported architecture: {arch}")),
    }

    Ok(())
}

pub fn run_bios(
    sh: &Shell,
    qemu_flags: &str,
    iso_path: &Path,
    interactive: bool,
    monitor: bool,
) -> Result<()> {
    let iso = iso_path.to_str().unwrap();
    let qemu_args: Vec<&str> = qemu_flags.split_whitespace().collect();
    let netdev = user_netdev_arg();

    let mut final_args = Vec::new();
    push_stdio_serial_args(&mut final_args, interactive, monitor);

    println!("Running in QEMU BIOS mode...");
    let mut args = vec![
        "-M",
        "q35",
        "-cdrom",
        iso,
        "-boot",
        "d",
        "-device",
        "virtio-net-pci,netdev=n0,mac=52:54:00:12:34:56,disable-legacy=on",
        "-netdev",
        &netdev,
    ];

    let default_audio =
        if !has_user_audio_args(qemu_flags) { default_audiodev_arg() } else { None };

    if !has_user_audio_args(qemu_flags) {
        if let Some(audiodev) = default_audio.as_deref() {
            args.extend_from_slice(&[
                "-audiodev",
                audiodev,
                "-device",
                "virtio-sound-pci,audiodev=audio0",
            ]);
        }
    }

    push_x86_interactive_display_args(&mut args, interactive, qemu_flags);

    args.extend(final_args.iter().map(String::as_str));
    run_qemu(sh, "qemu-system-x86_64", &args, &qemu_args)?;
    Ok(())
}

pub fn run_hdd(
    sh: &Shell,
    arch: &str,
    qemu_flags: &str,
    hdd_path: &Path,
    interactive: bool,
    monitor: bool,
) -> Result<()> {
    let name = image_name(arch);
    let hdd = hdd_path.to_str().unwrap();
    let ovmf_code = format!("vendor/ovmf/ovmf-code-{arch}.fd");
    let ovmf_vars = format!("vendor/ovmf/ovmf-vars-{arch}.fd");

    println!("Running {name} HDD in QEMU...");

    let mut final_args = Vec::new();
    push_stdio_serial_args(&mut final_args, interactive, monitor);

    let qemu_args: Vec<&str> = qemu_flags.split_whitespace().collect();
    let netdev = user_netdev_arg();
    match arch {
        "x86_64" => {
            let pflash0 = format!("if=pflash,unit=0,format=raw,file={ovmf_code},readonly=on");
            let pflash1 = format!("if=pflash,unit=1,format=raw,file={ovmf_vars}");
            let netdev_val = netdev.to_string();
            let mut args = vec![
                "-M",
                "q35",
                "-drive",
                &pflash0,
                "-drive",
                &pflash1,
                "-hda",
                hdd,
                "-device",
                "virtio-net-pci,netdev=n0,mac=52:54:00:12:34:56,disable-legacy=on",
                "-netdev",
                &netdev_val,
            ];

            push_x86_interactive_display_args(&mut args, interactive, qemu_flags);

            let default_audio =
                if !has_user_audio_args(qemu_flags) { default_audiodev_arg() } else { None };

            if !has_user_audio_args(qemu_flags) {
                if let Some(audiodev) = default_audio.as_deref() {
                    args.extend_from_slice(&[
                        "-audiodev",
                        audiodev,
                        "-device",
                        "virtio-sound-pci,audiodev=audio0",
                    ]);
                }
            }

            args.extend(final_args.iter().map(String::as_str));
            run_qemu(sh, "qemu-system-x86_64", &args, &qemu_args)?;
        }
        "aarch64" => {
            let pflash0 = format!("if=pflash,unit=0,format=raw,file={ovmf_code},readonly=on");
            let pflash1 = format!("if=pflash,unit=1,format=raw,file={ovmf_vars}");
            let mut args = vec![
                "-M",
                "virt",
                "-cpu",
                "cortex-a72",
                "-drive",
                &pflash0,
                "-drive",
                &pflash1,
                "-hda",
                hdd,
            ];
            if interactive {
                args.extend_from_slice(&[
                    "-device",
                    "ramfb",
                    "-device",
                    "qemu-xhci",
                    "-device",
                    "usb-kbd",
                    "-device",
                    "usb-mouse",
                ]);
            }
            args.extend(final_args.iter().map(String::as_str));
            run_qemu(sh, "qemu-system-aarch64", &args, &qemu_args)?;
        }
        "riscv64" => {
            let pflash0 =
                format!("node-name=pflash0,driver=file,read-only=on,filename={ovmf_code}");
            let pflash1 = format!("node-name=pflash1,driver=file,filename={ovmf_vars}");
            let hda = format!("file={hdd},format=raw,if=none,id=drive0");
            let mut args = vec![
                "-blockdev",
                &pflash0,
                "-blockdev",
                &pflash1,
                "-M",
                "virt,pflash0=pflash0,pflash1=pflash1",
                "-cpu",
                "rv64",
                "-m",
                "2G",
                "-drive",
                &hda,
                "-device",
                "virtio-blk-device,drive=drive0",
            ];
            if interactive {
                args.extend_from_slice(&[
                    "-device",
                    "ramfb",
                    "-device",
                    "qemu-xhci",
                    "-device",
                    "usb-kbd",
                    "-device",
                    "usb-mouse",
                ]);
            }
            args.extend(final_args.iter().map(String::as_str));
            run_qemu(sh, "qemu-system-riscv64", &args, &qemu_args)?;
        }
        "loongarch64" => {
            let pflash0 = format!("if=pflash,unit=0,format=raw,file={ovmf_code},readonly=on");
            let pflash1 = format!("if=pflash,unit=1,format=raw,file={ovmf_vars}");
            let mut args = vec![
                "-M", "virt", "-cpu", "la464", "-drive", &pflash0, "-drive", &pflash1, "-hda", hdd,
            ];
            if interactive {
                args.extend_from_slice(&[
                    "-device",
                    "ramfb",
                    "-device",
                    "qemu-xhci",
                    "-device",
                    "usb-kbd",
                    "-device",
                    "usb-mouse",
                ]);
            }
            args.extend(final_args.iter().map(String::as_str));
            run_qemu(sh, "qemu-system-loongarch64", &args, &qemu_args)?;
        }
        _ => return Err(anyhow::anyhow!("Unsupported architecture: {arch}")),
    }

    Ok(())
}

fn run_qemu(_sh: &Shell, program: &str, base_args: &[&str], extra_args: &[&str]) -> Result<()> {
    let mut cmd = std::process::Command::new(program);
    cmd.stdout(std::process::Stdio::inherit());
    cmd.stderr(std::process::Stdio::inherit());
    cmd.stdin(std::process::Stdio::inherit());

    // VS Code Snap can inject GTK variables that point into /snap and crash
    // system QEMU with mixed libc symbols during GTK frontend startup.
    cmd.env_remove("GTK_PATH");
    cmd.env_remove("GTK_MODULES");

    for arg in base_args {
        cmd.arg(arg);
    }
    for arg in extra_args {
        cmd.arg(arg);
    }

    println!(
        "$ {} {}",
        program,
        cmd.get_args().map(|a| a.to_string_lossy()).collect::<Vec<_>>().join(" ")
    );

    let status = cmd.status().map_err(|e| anyhow::anyhow!("failed to run qemu: {e}"))?;
    if !status.success() {
        return Err(anyhow::anyhow!("qemu exited with non-zero code: {status}"));
    }
    Ok(())
}
