//! Image creation tasks - ISO and HDD.

use std::path::{Path, PathBuf};

use walkdir::WalkDir;
use xshell::{Shell, cmd};

use crate::common::{Result, image_name};
use crate::rustc_thingos::{
    THINGOS_CARGO_BINARY, THINGOS_RUSTC_BINARY, THINGOS_RUSTLIB_CACHE_DIR, stage_rustc_for_iso,
};

pub struct ProgramConfig {
    pub name: &'static str,
    pub is_init: bool,
    pub boot_module: bool,
    pub features: Vec<&'static str>,
}

pub struct SharedLibraryConfig {
    pub package: &'static str,
    pub file_name: &'static str,
}

/// Configuration for ISO builds.
#[derive(Default)]
pub struct IsoConfig<'a> {
    pub resolution: Option<&'a str>,
    pub iso_path: Option<&'a Path>,
}

pub fn default_programs() -> Vec<ProgramConfig> {
    vec![
        ProgramConfig {
            name: "sprout",
            is_init: true,
            boot_module: true,
            features: vec!["diagnostic-apps"],
        },
        ProgramConfig { name: "bristle", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "rtc_cmos", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "ps2_kbd", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "sh", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "ls", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "ln", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "ps", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "kill", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "cat", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "head", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "tail", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "top", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "wc", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "yes", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "which", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "cp", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "mv", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "rm", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "rmdir", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "mkdir", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "echo", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "grep", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "pwd", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "touch", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "setshell", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "dmesg", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "stat", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "file", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "dirname", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "basename", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "sleep", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "sort", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "env", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "uname", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "true", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "false", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "input_echo", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "ps2_mouse", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig {
            name: "display_bootfb",
            is_init: false,
            boot_module: true,
            features: vec![],
        },
        ProgramConfig {
            name: "display_virtio_gpu",
            is_init: false,
            boot_module: true,
            features: vec![],
        },
        ProgramConfig { name: "cambium", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "virtio_netd", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "rtl8168d", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "netd", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "fetchd", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "httpsd", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "find", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "iso_reader", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "ping", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "nslookup", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "ahci_disk", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "iso9660d", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "virtio_sound", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "hdaudio", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "pci_stubd", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "chime", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "vfs_hello", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "show_args", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig {
            name: "env_roundtrip",
            is_init: false,
            boot_module: true,
            features: vec![],
        },
        ProgramConfig { name: "cwd_test", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "date", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig {
            name: "wayland_hello",
            is_init: false,
            boot_module: true,
            features: vec![],
        },
        ProgramConfig { name: "terminal", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "tee", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "xargs", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "placed", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "bloom", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "clear", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "loglevel", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "poll_mux", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig {
            name: "ipc_service_demo",
            is_init: false,
            boot_module: true,
            features: vec![],
        },
        ProgramConfig {
            name: "ipc_pipe_demo",
            is_init: false,
            boot_module: true,
            features: vec![],
        },
        ProgramConfig {
            name: "ipc_provider_demo",
            is_init: false,
            boot_module: true,
            features: vec![],
        },
        ProgramConfig {
            name: "ipc_memfd_demo",
            is_init: false,
            boot_module: true,
            features: vec![],
        },
        ProgramConfig { name: "test_exec", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig {
            name: "test_vm_protect",
            is_init: false,
            boot_module: true,
            features: vec![],
        },
        ProgramConfig {
            name: "test_exec_env",
            is_init: false,
            boot_module: true,
            features: vec![],
        },
        ProgramConfig { name: "test_threads", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "test_futex", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "ld_so", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig {
            name: "test_dyn_loader",
            is_init: false,
            boot_module: true,
            features: vec![],
        },
        ProgramConfig { name: "test_dlopen", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "reboot", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "shutdown", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "lsv", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "rn", is_init: false, boot_module: true, features: vec![] },
    ]
}

pub fn default_shared_libraries() -> Vec<SharedLibraryConfig> {
    vec![SharedLibraryConfig { package: "pistil", file_name: "libpistil.so" }]
}

fn generate_limine_config(
    _sh: &Shell,
    programs: &[ProgramConfig],
    assets: &[PathBuf],
    resolution: Option<&str>,
    include_busybox: bool,
    include_default_shell: bool,
) -> String {
    let res = resolution.unwrap_or("1920x1080");
    let mut conf = String::new();
    conf.push_str("timeout: 3\nquiet: yes\nverbose: no\nserial: yes\n");
    conf.push_str("interface_branding: yes\n");
    conf.push_str("interface_help: no\n");
    conf.push_str("interface_background: 121212\n");
    conf.push_str("interface_foreground: cccccc\n");
    conf.push_str("interface_active_background: 333333\n");
    conf.push_str("interface_active_foreground: ffffff\n");
    conf.push_str("menu_hidden: yes\n\n");

    let mut common_modules = String::new();
    for prog in programs {
        if !prog.boot_module {
            continue;
        }
        let bin_path = if is_driver(prog.name) {
            format!("boot():/drivers/{}", prog.name)
        } else {
            format!("boot():/bin/{}", prog.name)
        };
        common_modules.push_str(&format!("    module_path: {}\n", bin_path));
        if !is_driver(prog.name) {
            for alias in userspace_aliases(prog.name) {
                common_modules.push_str(&format!("    module_path: boot():/bin/{alias}\n"));
            }
        }
        if prog.is_init {
            common_modules.push_str("    module_cmdline: init\n");
        }
    }

    for lib in default_shared_libraries() {
        common_modules.push_str(&format!("    module_path: boot():/lib/{}\n", lib.file_name));
    }

    for asset in assets {
        let path_str = asset.to_string_lossy();
        let clean_path = path_str.replace('\\', "/");

        let allowed = clean_path.ends_with("NotoSans-Regular.ttf")
            || clean_path.ends_with("future/default.svg")
            || clean_path.ends_with("wallpapers/flower.bmp")
            || clean_path.ends_with("wallpapers/clouds.bmp")
            || clean_path.ends_with("wallpapers/leather.bmp")
            || clean_path.ends_with("wallpapers/linen.bmp")
            || clean_path.ends_with("themes/genie_circles.wasm")
            || clean_path.ends_with("unifont.hex");

        if allowed && !clean_path.ends_with("unifont.hex") && !clean_path.ends_with("locale.conf") {
            let iso_path = clean_path.replace("assets/", "share/");
            common_modules.push_str(&format!("    module_path: boot():/{iso_path}\n"));
        }
    }

    common_modules.push_str("    module_path: boot():/share/fonts/unifont.hex\n");
    common_modules.push_str("    module_path: boot():/etc/locale.conf\n");
    common_modules.push_str("    module_path: boot():/etc/profile\n");
    common_modules.push_str("    module_path: boot():/etc/motd\n");
    if include_default_shell {
        common_modules.push_str("    module_path: boot():/etc/default/shell\n");
    }
    if include_busybox {
        common_modules.push_str("    module_path: boot():/bin/busybox\n");
        common_modules.push_str("    module_path: boot():/bin/ash\n");
    }

    struct LimineEntry<'a> {
        title: &'a str,
        kernel_cmdline: &'a str,
    }

    let entries = [
        LimineEntry { title: "ThingOS", kernel_cmdline: "loglevel=info" },
        LimineEntry {
            title: "ThingOS (BootFB Fallback)",
            kernel_cmdline: "loglevel=info display=bootfb",
        },
        LimineEntry { title: "ThingOS (Debug)", kernel_cmdline: "loglevel=debug" },
        LimineEntry { title: "ThingOS (Trace)", kernel_cmdline: "loglevel=5" },
    ];

    for (index, entry) in entries.iter().enumerate() {
        if index > 0 {
            conf.push('\n');
        }

        conf.push_str(&format!("/{}\n", entry.title));
        conf.push_str("    protocol: limine\n");
        conf.push_str(&format!("    resolution: {res}\n"));
        conf.push_str("    kernel_path: boot():/boot/kernel\n");
        conf.push_str(&format!("    kernel_cmdline: {}\n", entry.kernel_cmdline));
        conf.push_str(&common_modules);
    }

    conf
}

fn generate_motd() -> String {
    const WIDTH: usize = 60;
    let mut motd = String::new();
    let border = format!("  +{}+\n", "-".repeat(WIDTH + 2));

    motd.push_str(&border);
    motd.push_str(&format!(
        "  | {:<width$} |\n",
        "THING-OS  v0.1  (thingos ACT IV)                 2026-04-16",
        width = WIDTH
    ));
    motd.push_str(&format!("  | {:<width$} |\n", "", width = WIDTH));
    motd.push_str(&format!("  | {:<width$} |\n", "People, places, things", width = WIDTH));
    motd.push_str(&format!("  | {:<width$} |\n", "", width = WIDTH));
    motd.push_str(&format!("  | {:<width$} |\n", "", width = WIDTH));
    motd.push_str(&format!("  | {:<width$} |\n", "Try: ls /bin, ps, cat /version", width = WIDTH));
    motd.push_str(&border);
    motd
}

/// Build an ISO image for the target architecture with default settings.
pub fn build_iso(sh: &Shell, arch: &str, programs: &[ProgramConfig]) -> Result<PathBuf> {
    build_iso_with_config(sh, arch, programs, &IsoConfig::default())
}

/// Build an ISO image for the target architecture with custom configuration.
pub fn build_iso_with_config(
    sh: &Shell,
    arch: &str,
    programs: &[ProgramConfig],
    config: &IsoConfig<'_>,
) -> Result<PathBuf> {
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
    let timestamp = now.as_secs();
    let nanos = now.subsec_nanos();

    let iso_name = if let Some(explicit_path) = config.iso_path {
        explicit_path.to_string_lossy().to_string()
    } else {
        format!("thing-os-{arch}-{timestamp}-{nanos}.iso")
    };

    let iso_root_name = format!("iso_root_{nanos}");
    let iso_root = Path::new(&iso_root_name);

    println!("Building ISO {iso_name}...");

    if sh.path_exists(iso_root) {
        sh.remove_path(iso_root)?;
    }
    sh.create_dir(iso_root.join("boot"))?;
    sh.create_dir(iso_root.join("boot/limine"))?;
    sh.create_dir(iso_root.join("bin"))?;
    sh.create_dir(iso_root.join("drivers"))?;
    sh.create_dir(iso_root.join("etc"))?;
    sh.create_dir(iso_root.join("usr/lib"))?;
    sh.create_dir(iso_root.join("lib"))?;
    sh.create_dir(iso_root.join("EFI/BOOT"))?;

    cmd!(sh, "cp -r assets {iso_root_name}/share").run()?;

    let mut asset_files = Vec::new();
    for entry in WalkDir::new("assets") {
        let entry = entry?;
        if entry.file_type().is_file() {
            let path = entry.path();
            if path.to_string_lossy().contains("assets/icons")
                && !path.to_string_lossy().contains("thingos")
            {
                continue;
            }
            asset_files.push(path.to_path_buf());
        }
    }

    asset_files.sort_by(|a, b| {
        let a_str = a.to_string_lossy();
        let b_str = b.to_string_lossy();

        let a_priority = if a_str.ends_with(".cur") || a_str.ends_with(".ani") {
            0
        } else if a_str.ends_with(".bmp") {
            1
        } else if a_str.ends_with(".ttf") {
            2
        } else if a_str.ends_with(".svg") {
            4
        } else {
            3
        };

        let b_priority = if b_str.ends_with(".cur") || b_str.ends_with(".ani") {
            0
        } else if b_str.ends_with(".bmp") {
            1
        } else if b_str.ends_with(".ttf") {
            2
        } else if b_str.ends_with(".svg") {
            4
        } else {
            3
        };

        match a_priority.cmp(&b_priority) {
            std::cmp::Ordering::Equal => a_str.cmp(&b_str),
            other => other,
        }
    });

    let kernel_src = format!("thingos/bran/bin-{arch}/kernel");
    sh.copy_file(&kernel_src, iso_root.join("boot/kernel"))?;
    sh.copy_file("assets/fonts/unifont.hex", iso_root.join("share/fonts/unifont.hex"))?;

    sh.write_file(
        iso_root.join("etc/locale.conf"),
        "LOCALE=en_US\nTZ_OFFSET=-8\nOLLAMA_SERVER=http://10.0.2.2:11434\nOLLAMA_MODEL=tinyllama\n",
    )?;

    sh.write_file(
        iso_root.join("etc/profile"),
        "export PATH=/bin:/drivers\nalias ll='loglevel'\nalias halt='shutdown'\n",
    )?;

    sh.write_file(iso_root.join("etc/motd"), generate_motd())?;

    let mut include_default_shell = false;
    if let Ok(default_shell) = std::env::var("THINGOS_DEFAULT_SHELL") {
        let shell = default_shell.trim();
        if !shell.is_empty() {
            let default_dir = iso_root.join("etc/default");
            if !sh.path_exists(&default_dir) {
                sh.create_dir(&default_dir)?;
            }
            sh.write_file(iso_root.join("etc/default/shell"), format!("{shell}\n"))?;
            include_default_shell = true;
        }
    }

    let mut include_busybox = false;
    if let Ok(busybox_bin) = std::env::var("THINGOS_BUSYBOX_BIN") {
        let busybox_path = PathBuf::from(busybox_bin.trim());
        let meta = std::fs::metadata(&busybox_path).map_err(|_| {
            anyhow::anyhow!("THINGOS_BUSYBOX_BIN does not exist: {}", busybox_path.display())
        })?;
        if !meta.is_file() || meta.len() == 0 {
            return Err(anyhow::anyhow!(
                "THINGOS_BUSYBOX_BIN is not a valid non-empty file: {}",
                busybox_path.display()
            ));
        }
        sh.copy_file(&busybox_path, iso_root.join("bin/busybox"))?;
        sh.copy_file(&busybox_path, iso_root.join("bin/ash"))?;
        include_busybox = true;
    }

    println!("Building userspace programs...");

    let cwd = std::env::current_dir().unwrap();
    let target_json = if arch == "x86_64" {
        cwd.join("targets/x86_64-unknown-thingos.json")
    } else if arch == "riscv64" {
        cwd.join("targets/riscv64gc-unknown-thingos.json")
    } else {
        cwd.join(format!("targets/{arch}-unknown-thingos.json"))
    };
    let target = target_json.to_str().unwrap();

    for prog in programs {
        build_userspace_app_with_features(sh, prog.name, target, "release", &prog.features)?;
        let dest_subdir = if is_driver(prog.name) { "drivers" } else { "bin" };
        let dest_path = iso_root.join(format!("{}/{}", dest_subdir, prog.name));
        copy_userspace_binary(sh, prog.name, target, "release", dest_path.to_str().unwrap())?;
        if !is_driver(prog.name) {
            for alias in userspace_aliases(prog.name) {
                sh.copy_file(&dest_path, iso_root.join(format!("{}/{}", dest_subdir, alias)))?;
            }
        }
    }

    for lib in default_shared_libraries() {
        build_shared_library(sh, lib.package, target, "release")?;
        copy_shared_library(
            sh,
            lib.package,
            lib.file_name,
            target,
            "release",
            iso_root.join(format!("lib/{}", lib.file_name)).to_str().unwrap(),
        )?;
    }

    // Ensure /lib/ld.so exists for dynamic executables.
    let ld_so_src = iso_root.join("bin/ld_so");
    if ld_so_src.exists() {
        sh.copy_file(&ld_so_src, iso_root.join("lib/ld.so"))?;
    }

    // Stage libstd.so from bootstrap artifacts.
    // Note: rustc-thingos/bootstrap build puts artifacts under build/
    if arch == "x86_64" {
        let std_src = cwd.join(
            "build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-thingos/release/libstd.so",
        );
        if std_src.exists() {
            let std_dst = iso_root.join("lib/libstd.so");
            sh.create_dir(std_dst.parent().unwrap())?;
            sh.copy_file(&std_src, &std_dst)?;
        }
    }

    stage_rustc_for_iso(sh, iso_root)?;

    let limine_conf_content = generate_limine_config(
        sh,
        programs,
        &asset_files,
        config.resolution,
        include_busybox,
        include_default_shell,
    );
    println!("--- DEBUG: Generated limine.conf ---\n{}\n--- END DEBUG ---", limine_conf_content);
    sh.write_file(iso_root.join("boot/limine/limine.conf"), limine_conf_content)?;

    match arch {
        "x86_64" => {
            sh.copy_file(
                "vendor/limine/limine-bios.sys",
                iso_root.join("boot/limine/limine-bios.sys"),
            )?;
            sh.copy_file(
                "vendor/limine/limine-bios-cd.bin",
                iso_root.join("boot/limine/limine-bios-cd.bin"),
            )?;
            sh.copy_file(
                "vendor/limine/limine-uefi-cd.bin",
                iso_root.join("boot/limine/limine-uefi-cd.bin"),
            )?;
            sh.copy_file("vendor/limine/BOOTX64.EFI", iso_root.join("EFI/BOOT/BOOTX64.EFI"))?;
            sh.copy_file("vendor/limine/BOOTIA32.EFI", iso_root.join("EFI/BOOT/BOOTIA32.EFI"))?;

            cmd!(sh, "xorriso -as mkisofs -R -J -b boot/limine/limine-bios-cd.bin -no-emul-boot -boot-load-size 4 -boot-info-table --efi-boot boot/limine/limine-uefi-cd.bin -efi-boot-part --efi-boot-image --protective-msdos-label {iso_root_name} -o {iso_name}").run()?;
            cmd!(sh, "./vendor/limine/limine bios-install {iso_name}").run()?;

            sh.remove_path(&iso_root_name)?;
            println!("ISO created: {iso_name}");
            Ok(PathBuf::from(iso_name))
        }
        "aarch64" => {
            sh.copy_file(
                "vendor/limine/limine-uefi-cd.bin",
                iso_root.join("boot/limine/limine-uefi-cd.bin"),
            )?;
            sh.copy_file("vendor/limine/BOOTAA64.EFI", iso_root.join("EFI/BOOT/BOOTAA64.EFI"))?;

            cmd!(sh, "xorriso -as mkisofs -R -J --efi-boot boot/limine/limine-uefi-cd.bin -efi-boot-part --efi-boot-image --protective-msdos-label {iso_root_name} -o {iso_name}").run()?;

            sh.remove_path(&iso_root_name)?;
            println!("ISO created: {iso_name}");
            Ok(PathBuf::from(iso_name))
        }
        "riscv64" => {
            let efi_img = iso_root.join("boot/limine/limine-uefi-riscv64.bin");
            let efi_img_str = efi_img.to_str().unwrap();
            cmd!(sh, "dd if=/dev/zero of={efi_img_str} bs=1K count=2880 status=none").run()?;
            cmd!(sh, "mformat -i {efi_img_str} -f 2880 ::").run()?;
            cmd!(sh, "mmd -i {efi_img_str} ::/EFI ::/EFI/BOOT").run()?;
            cmd!(
                sh,
                "mcopy -i {efi_img_str} vendor/limine/BOOTRISCV64.EFI ::/EFI/BOOT/BOOTRISCV64.EFI"
            )
            .run()?;
            sh.write_file(iso_root.join("startup.nsh"), "\\EFI\\BOOT\\BOOTRISCV64.EFI\n")?;
            let startup_nsh = iso_root.join("startup.nsh");
            let startup_nsh_str = startup_nsh.to_str().unwrap();
            cmd!(sh, "mcopy -i {efi_img_str} {startup_nsh_str} ::").run()?;
            sh.copy_file(
                "vendor/limine/BOOTRISCV64.EFI",
                iso_root.join("EFI/BOOT/BOOTRISCV64.EFI"),
            )?;

            cmd!(sh, "xorriso -as mkisofs -R -J --efi-boot boot/limine/limine-uefi-riscv64.bin -efi-boot-part --efi-boot-image --protective-msdos-label {iso_root_name} -o {iso_name}").run()?;

            sh.remove_path(&iso_root_name)?;
            println!("ISO created: {iso_name}");
            Ok(PathBuf::from(iso_name))
        }
        "loongarch64" => {
            let efi_img = iso_root.join("boot/limine/limine-uefi-loongarch64.bin");
            let efi_img_str = efi_img.to_str().unwrap();
            cmd!(sh, "dd if=/dev/zero of={efi_img_str} bs=1K count=2880 status=none").run()?;
            cmd!(sh, "mformat -i {efi_img_str} -f 2880 ::").run()?;
            cmd!(sh, "mmd -i {efi_img_str} ::/EFI ::/EFI/BOOT").run()?;
            cmd!(sh, "mcopy -i {efi_img_str} vendor/limine/BOOTLOONGARCH64.EFI ::/EFI/BOOT/BOOTLOONGARCH64.EFI").run()?;
            sh.write_file(iso_root.join("startup.nsh"), "\\EFI\\BOOT\\BOOTLOONGARCH64.EFI\n")?;
            let startup_nsh = iso_root.join("startup.nsh");
            let startup_nsh_str = startup_nsh.to_str().unwrap();
            cmd!(sh, "mcopy -i {efi_img_str} {startup_nsh_str} ::").run()?;
            sh.copy_file(
                "vendor/limine/BOOTLOONGARCH64.EFI",
                iso_root.join("EFI/BOOT/BOOTLOONGARCH64.EFI"),
            )?;

            cmd!(sh, "xorriso -as mkisofs -R -J --efi-boot boot/limine/limine-uefi-loongarch64.bin -efi-boot-part --efi-boot-image --protective-msdos-label {iso_root_name} -o {iso_name}").run()?;

            sh.remove_path(&iso_root_name)?;
            println!("ISO created: {iso_name}");
            Ok(PathBuf::from(iso_name))
        }
        _ => Err(anyhow::anyhow!("Unsupported architecture: {arch}")),
    }
}

fn build_userspace_app_with_features(
    sh: &Shell,
    name: &str,
    target: &str,
    profile: &str,
    features: &[&str],
) -> Result<()> {
    println!("Building {name} ...");

    let extra_flags =
        if target.ends_with(".json") { vec!["-Z", "json-target-spec"] } else { vec![] };

    let cwd = std::env::current_dir().unwrap();
    let std_src = cwd.join("library");

    let stage1_rustc = cwd.join("target/rustc-thingos/rustc");
    let stage1_rustc_wrapper = cwd.join("target/rustc-thingos/rustc-wrapper");
    let skip_rustc_thingos = std::env::var("SKIP_RUSTC_THINGOS").as_deref() == Ok("1");
    let use_fork_rustc = target.ends_with(".json")
        && target.contains("thingos")
        && target.contains("x86_64-unknown-thingos")
        && stage1_rustc.exists()
        && stage1_rustc_wrapper.exists()
        && !skip_rustc_thingos;

    let build_std_crates =
        if use_fork_rustc { "core,alloc,std,panic_abort" } else { "core,alloc,panic_abort" };

    let rustflags = String::from("-Awarnings");
    let mut cmd_obj = cmd!(
        sh,
        "cargo -Z build-std={build_std_crates} -Z build-std-features=compiler-builtins-mem {extra_flags...} build --target {target} --profile {profile} -p {name}"
    )
    .env("RUSTFLAGS", &rustflags);

    if use_fork_rustc {
        let target_path = cwd.join("targets");
        cmd_obj = cmd_obj
            .env("RUSTC", &stage1_rustc_wrapper)
            .env("RUSTFLAGS", &rustflags)
            .env("__CARGO_TESTS_ONLY_SRC_ROOT", std_src.to_str().unwrap())
            .env("RUST_TARGET_PATH", target_path);
    }

    for f in features {
        cmd_obj = cmd_obj.arg("--features").arg(f);
    }

    cmd_obj.run()?;
    Ok(())
}

fn copy_userspace_binary(
    sh: &Shell,
    name: &str,
    target: &str,
    profile_dir: &str,
    dst: &str,
) -> Result<()> {
    let target_name =
        std::path::Path::new(target).file_stem().and_then(|s| s.to_str()).unwrap_or(target);

    let elf = format!("target/{target_name}/{profile_dir}/{name}");
    cmd!(sh, "cp {elf} {dst}").run()?;
    Ok(())
}

fn build_shared_library(sh: &Shell, package: &str, target: &str, profile: &str) -> Result<()> {
    build_userspace_app_with_features(sh, package, target, profile, &[])
}

fn copy_shared_library(
    sh: &Shell,
    package: &str,
    file_name: &str,
    target: &str,
    profile_dir: &str,
    dst: &str,
) -> Result<()> {
    let target_name =
        std::path::Path::new(target).file_stem().and_then(|s| s.to_str()).unwrap_or(target);
    let profile_root = Path::new("target").join(target_name).join(profile_dir);
    let mut candidates = Vec::new();

    if profile_root.exists() {
        for entry in WalkDir::new(&profile_root) {
            let entry = entry?;
            if !entry.file_type().is_file() {
                continue;
            }
            let name = entry.file_name().to_string_lossy();
            if name == file_name {
                candidates.push(entry.path().to_path_buf());
            }
        }
    }

    // Stable deterministic selection when multiple build directories contain
    // the same SONAME (e.g. release root + deps copies).
    candidates.sort();
    let src = candidates.first().ok_or_else(|| {
        anyhow::anyhow!("shared library artifact not found for package '{package}'")
    })?;

    cmd!(sh, "cp {src} {dst}").run()?;
    Ok(())
}

pub fn build_hdd(sh: &Shell, arch: &str, programs: &[ProgramConfig]) -> Result<PathBuf> {
    let name = image_name(arch);
    let hdd = format!("{name}.hdd");

    println!("Building HDD image for {arch}...");

    sh.remove_path(&hdd)?;
    cmd!(sh, "dd if=/dev/zero bs=1M count=0 seek=64 of={hdd}").run()?;
    cmd!(sh, "sgdisk {hdd} -n 1:2048 -t 1:ef00").run()?;

    if arch == "x86_64" {
        cmd!(sh, "./vendor/limine/limine bios-install {hdd}").run()?;
    }

    cmd!(sh, "mformat -i {hdd}@@1M").run()?;
    cmd!(sh, "mmd -i {hdd}@@1M ::/EFI ::/EFI/BOOT ::/boot ::/boot/limine ::/drivers ::/bin ::/lib")
        .run()?;
    cmd!(sh, "mcopy -i {hdd}@@1M -s assets ::").run()?;

    let mut asset_files = Vec::new();
    for entry in WalkDir::new("assets") {
        let entry = entry?;
        if entry.file_type().is_file() {
            let path = entry.path();
            if path.to_string_lossy().contains("assets/icons") {
                continue;
            }
            asset_files.push(path.to_path_buf());
        }
    }

    let kernel_src = format!("thingos/bran/bin-{arch}/kernel");
    cmd!(sh, "mcopy -i {hdd}@@1M {kernel_src} ::/boot").run()?;

    let cwd = std::env::current_dir().unwrap();
    let target_json = if arch == "x86_64" {
        cwd.join("targets/x86_64-unknown-thingos.json")
    } else if arch == "riscv64" {
        cwd.join("targets/riscv64gc-unknown-thingos.json")
    } else {
        cwd.join(format!("targets/{arch}-unknown-thingos.json"))
    };
    let target = target_json.to_str().unwrap();

    for lib in default_shared_libraries() {
        build_shared_library(sh, lib.package, target, "release")?;
        let staged_lib = std::env::temp_dir()
            .join(format!("thingos_{}_{}", arch, lib.file_name))
            .to_string_lossy()
            .to_string();
        let lib_dst = format!("::/lib/{}", lib.file_name);
        copy_shared_library(sh, lib.package, lib.file_name, target, "release", &staged_lib)?;
        cmd!(sh, "mcopy -i {hdd}@@1M {staged_lib} {lib_dst}").run()?;
        sh.remove_path(&staged_lib)?;
    }

    sh.write_file(
        "locale.conf",
        "LOCALE=en_US\nTZ_OFFSET=-8\nOLLAMA_SERVER=http://10.0.2.2:11434\nOLLAMA_MODEL=tinyllama\n",
    )?;
    cmd!(sh, "mcopy -i {hdd}@@1M locale.conf ::/boot/locale.conf").run()?;
    sh.remove_path("locale.conf")?;

    sh.write_file(
        "profile",
        "export PATH=/bin:/drivers\nalias ll='loglevel'\nalias halt='shutdown'\n",
    )?;
    sh.write_file("motd", generate_motd())?;

    // create /etc in the fat32 image if it doesn't exist
    cmd!(sh, "mmd -i {hdd}@@1M ::/etc").run().ok();
    cmd!(sh, "mcopy -i {hdd}@@1M profile ::/etc/profile").run()?;
    cmd!(sh, "mcopy -i {hdd}@@1M motd ::/etc/motd").run()?;

    sh.remove_path("profile")?;
    sh.remove_path("motd")?;

    let limine_conf_content =
        generate_limine_config(sh, programs, &asset_files, None, false, false);
    let limine_cfg = "limine.generated.conf";
    sh.write_file(limine_cfg, limine_conf_content)?;
    cmd!(sh, "mcopy -i {hdd}@@1M {limine_cfg} ::/boot/limine/limine.conf").run()?;
    sh.remove_path(limine_cfg)?;

    match arch {
        "x86_64" => {
            cmd!(sh, "mcopy -i {hdd}@@1M vendor/limine/limine-bios.sys ::/boot/limine").run()?;
            cmd!(sh, "mcopy -i {hdd}@@1M vendor/limine/BOOTX64.EFI ::/EFI/BOOT").run()?;
            cmd!(sh, "mcopy -i {hdd}@@1M vendor/limine/BOOTIA32.EFI ::/EFI/BOOT").run()?;

            if std::env::var("INCLUDE_RUST_TOOLCHAIN").as_deref() == Ok("1") {
                let thingos_rustc = Path::new(THINGOS_RUSTC_BINARY);
                let thingos_cargo = Path::new(THINGOS_CARGO_BINARY);
                let thingos_rustlib = Path::new(THINGOS_RUSTLIB_CACHE_DIR);

                if thingos_rustc.exists() {
                    cmd!(sh, "mcopy -i {hdd}@@1M {THINGOS_RUSTC_BINARY} ::/bin/rustc").run()?;
                }
                if thingos_cargo.exists() {
                    cmd!(sh, "mcopy -i {hdd}@@1M {THINGOS_CARGO_BINARY} ::/bin/cargo").run()?;
                }
                if thingos_rustlib.exists() {
                    cmd!(sh, "mmd -i {hdd}@@1M ::/lib/rustlib").run().ok();
                    let src = THINGOS_RUSTLIB_CACHE_DIR;
                    cmd!(sh, "bash -c \"mcopy -i {hdd}@@1M -s {src}/* ::/lib/rustlib\"").run()?;
                }
            }
        }
        "aarch64" => {
            cmd!(sh, "mcopy -i {hdd}@@1M vendor/limine/BOOTAA64.EFI ::/EFI/BOOT").run()?;
        }
        "riscv64" => {
            cmd!(sh, "mcopy -i {hdd}@@1M vendor/limine/BOOTRISCV64.EFI ::/EFI/BOOT").run()?;
        }
        "loongarch64" => {
            cmd!(sh, "mcopy -i {hdd}@@1M vendor/limine/BOOTLOONGARCH64.EFI ::/EFI/BOOT").run()?;
        }
        _ => return Err(anyhow::anyhow!("Unsupported architecture: {arch}")),
    }

    println!("HDD image created: {hdd}");
    Ok(PathBuf::from(hdd))
}

fn is_driver(name: &str) -> bool {
    let drivers = [
        "ahci_disk",
        "ata_disk",
        "chime",
        "display_bootfb",
        "display_fake",
        "display_virtio_gpu",
        "driver_wasm_host",
        "hdaudio",
        "hwrng",
        "pci_stubd",
        "proto_driver_wasm",
        "ps2_kbd",
        "ps2_mouse",
        "rtc_cmos",
        "rtl8168d",
        "virtio_gpu",
        "virtio_netd",
        "virtio_sound",
    ];
    drivers.contains(&name)
}

fn userspace_aliases(name: &str) -> &'static [&'static str] {
    match name {
        "lsv" => &["lvs"],
        "rn" => &["runa"],
        _ => &[],
    }
}
