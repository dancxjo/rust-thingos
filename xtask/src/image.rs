//! Image creation tasks - ISO and HDD.

use std::io::Write;
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

#[derive(Clone, Copy)]
struct WallpaperSpec {
    file_name: &'static str,
    variant: u8,
}

const DEFAULT_WALLPAPERS: [WallpaperSpec; 5] = [
    WallpaperSpec { file_name: "flower.bmp", variant: 0 },
    WallpaperSpec { file_name: "flower.png", variant: 0 },
    WallpaperSpec { file_name: "clouds.bmp", variant: 1 },
    WallpaperSpec { file_name: "leather.bmp", variant: 2 },
    WallpaperSpec { file_name: "linen.bmp", variant: 3 },
];

const ISO_ROOT_DIRS: &[&str] = &[
    "boot",
    "boot/limine",
    "bin",
    "applications",
    "lib",
    "etc",
    "etc/roots",
    "dev",
    "proc",
    "sys",
    "run",
    "tmp",
    "media",
    "media/cdrom",
    "drivers",
    "session",
    "public",
    "services",
    "version",
    "EFI",
    "EFI/BOOT",
];

const PUBLIC_ASSET_DIRS: &[(&str, &str)] = &[
    ("assets/cursors", "cursors"),
    ("assets/fonts", "fonts"),
    ("assets/icons/lucide", "icons/lucide"),
    ("assets/themes", "themes"),
    ("assets/wallpapers", "wallpapers"),
];

/// Configuration for ISO builds.
#[derive(Default)]
pub struct IsoConfig<'a> {
    pub resolution: Option<&'a str>,
    pub iso_path: Option<&'a Path>,
    pub loglevel: Option<&'a str>,
    pub bootfb_default: bool,
}

fn ensure_default_wallpapers(root: &Path) -> Result<()> {
    let dir = root.join("wallpapers");
    std::fs::create_dir_all(&dir)?;

    for spec in DEFAULT_WALLPAPERS {
        let path = dir.join(spec.file_name);
        if !path.exists() {
            write_generated_wallpaper(&path, spec.variant)?;
        }
    }

    Ok(())
}

fn create_iso_root_layout(sh: &Shell, iso_root: &Path) -> Result<()> {
    sh.create_dir(iso_root)?;
    for dir in ISO_ROOT_DIRS {
        sh.create_dir(iso_root.join(dir))?;
    }
    Ok(())
}

fn stage_public_assets(iso_root: &Path) -> Result<Vec<PathBuf>> {
    let public_root = iso_root.join("public");
    std::fs::create_dir_all(&public_root)?;

    for (src, dst) in PUBLIC_ASSET_DIRS {
        copy_dir_recursive(Path::new(src), &public_root.join(dst))?;
    }
    ensure_default_wallpapers(&public_root)?;

    collect_files_under(&public_root)
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else if file_type.is_file() {
            if let Some(parent) = dst_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

fn collect_files_under(root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in WalkDir::new(root) {
        let entry = entry?;
        if entry.file_type().is_file() {
            files.push(entry.path().to_path_buf());
        }
    }
    files.sort();
    Ok(files)
}

fn stage_default_wallpapers_hdd(sh: &Shell, hdd: &str, arch: &str) -> Result<()> {
    let staged = std::env::temp_dir().join(format!("thingos_default_wallpapers_{arch}"));
    if staged.exists() {
        std::fs::remove_dir_all(&staged)?;
    }
    ensure_default_wallpapers(&staged)?;

    cmd!(sh, "mmd -i {hdd}@@1M ::/public").run().ok();
    cmd!(sh, "mmd -i {hdd}@@1M ::/public/wallpapers").run().ok();
    for wallpaper in DEFAULT_WALLPAPERS {
        let src = staged.join("wallpapers").join(wallpaper.file_name);
        let dst = format!("::/public/wallpapers/{}", wallpaper.file_name);
        cmd!(sh, "mcopy -i {hdd}@@1M {src} {dst}").run()?;
    }

    std::fs::remove_dir_all(&staged)?;
    Ok(())
}

fn write_generated_wallpaper(path: &Path, variant: u8) -> Result<()> {
    if path.extension().is_some_and(|ext| ext == "png") {
        return write_generated_wallpaper_png(path, variant);
    }

    let width = 320u32;
    let height = 180u32;
    let row_bytes = ((width * 3 + 3) / 4) * 4;
    let image_size = row_bytes * height;
    let file_size = 54 + image_size;

    let mut file = std::fs::File::create(path)?;
    file.write_all(b"BM")?;
    file.write_all(&file_size.to_le_bytes())?;
    file.write_all(&[0u8; 4])?;
    file.write_all(&54u32.to_le_bytes())?;
    file.write_all(&40u32.to_le_bytes())?;
    file.write_all(&(width as i32).to_le_bytes())?;
    file.write_all(&(height as i32).to_le_bytes())?;
    file.write_all(&1u16.to_le_bytes())?;
    file.write_all(&24u16.to_le_bytes())?;
    file.write_all(&0u32.to_le_bytes())?;
    file.write_all(&image_size.to_le_bytes())?;
    file.write_all(&2835u32.to_le_bytes())?;
    file.write_all(&2835u32.to_le_bytes())?;
    file.write_all(&0u32.to_le_bytes())?;
    file.write_all(&0u32.to_le_bytes())?;

    let mut row = vec![0u8; row_bytes as usize];
    for file_y in 0..height {
        let y = height - 1 - file_y;
        row.fill(0);
        for x in 0..width {
            let (r, g, b) = wallpaper_pixel(variant, x, y, width, height);
            let off = (x * 3) as usize;
            row[off] = b;
            row[off + 1] = g;
            row[off + 2] = r;
        }
        file.write_all(&row)?;
    }

    Ok(())
}

fn write_generated_wallpaper_png(path: &Path, variant: u8) -> Result<()> {
    let width = 320u32;
    let height = 180u32;
    let mut pixmap = tiny_skia::Pixmap::new(width, height)
        .ok_or_else(|| anyhow::anyhow!("failed to allocate wallpaper pixmap"))?;

    let data = pixmap.data_mut();
    for y in 0..height {
        for x in 0..width {
            let (r, g, b) = wallpaper_pixel(variant, x, y, width, height);
            let off = ((y * width + x) * 4) as usize;
            data[off] = r;
            data[off + 1] = g;
            data[off + 2] = b;
            data[off + 3] = 255;
        }
    }

    let encoded = pixmap.encode_png()?;
    std::fs::write(path, encoded)?;
    Ok(())
}

fn wallpaper_pixel(variant: u8, x: u32, y: u32, width: u32, height: u32) -> (u8, u8, u8) {
    match variant {
        0 => {
            let mut r = 22 + (x * 36 / width) as u8 + (y * 18 / height) as u8;
            let mut g = 68 + (x * 54 / width) as u8;
            let mut b = 104 + (y * 44 / height) as u8;
            let cx = width as i32 / 2;
            let cy = height as i32 / 2;
            let xi = x as i32;
            let yi = y as i32;
            for (px, py) in [(0, -28), (27, -10), (18, 24), (-18, 24), (-27, -10)] {
                let dx = xi - (cx + px);
                let dy = yi - (cy + py);
                if dx * dx + dy * dy < 24 * 24 {
                    r = 236;
                    g = 116;
                    b = 174;
                }
            }
            let dx = xi - cx;
            let dy = yi - cy;
            if dx * dx + dy * dy < 17 * 17 {
                r = 255;
                g = 205;
                b = 83;
            }
            (r, g, b)
        }
        1 => {
            let band = ((x + y * 2) % 97) as u8;
            (60 + band / 4, 122 + band / 3, 182 + band / 2)
        }
        2 => {
            let grain = ((x * 17 + y * 31 + (x ^ y) * 7) % 55) as u8;
            (82 + grain, 53 + grain / 2, 38 + grain / 3)
        }
        _ => {
            let weave = (((x / 6) + (y / 4)) % 2) as u8 * 18;
            (188 + weave, 194 + weave, 176 + weave)
        }
    }
}

pub fn default_programs() -> Vec<ProgramConfig> {
    let mut programs = vec![
        ProgramConfig {
            name: "sprout",
            is_init: true,
            boot_module: true,
            features: vec!["diagnostic-apps"],
        },
        ProgramConfig { name: "bristle", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "rtc_cmos", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "hwrng", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "ps2_kbd", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "sh", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "ls", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "lsusb", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "lspci", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "ln", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "ps", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "kill", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "killall", is_init: true, boot_module: true, features: vec![] },
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
        ProgramConfig { name: "mount", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "echo", is_init: true, boot_module: true, features: vec![] },
        ProgramConfig { name: "printf", is_init: true, boot_module: true, features: vec![] },
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
        ProgramConfig { name: "mesocarp", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "fetchd", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "httpsd", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "find", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "ip", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "iso_reader", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "ping", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "nslookup", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "ahci_disk", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "ata_disk", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "iso9660d", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "fatd", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "virtio_sound", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "hdaudio", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "pci_stubd", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "xhci", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "chime", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "vfs_hello", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "show_args", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "clock", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "leaf", is_init: false, boot_module: true, features: vec![] },
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
            name: "vfs_test_provider",
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
        ProgramConfig { name: "attr_list", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "attr_get", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "attr_set", is_init: false, boot_module: true, features: vec![] },
        ProgramConfig { name: "attr_rm", is_init: false, boot_module: true, features: vec![] },
    ];

    for program in &mut programs {
        program.boot_module = is_bootstrap_boot_module(program.name);
    }

    programs
}

pub fn default_shared_libraries() -> Vec<SharedLibraryConfig> {
    vec![SharedLibraryConfig { package: "pistil", file_name: "libpistil.so" }]
}

fn generate_limine_config(
    _sh: &Shell,
    programs: &[ProgramConfig],
    assets: &[PathBuf],
    resolution: Option<&str>,
    loglevel: Option<&str>,
    bootfb_default: bool,
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

    let all_graphics_modules =
        limine_modules(programs, assets, include_busybox, include_default_shell, false, false);
    let bootfb_only_modules =
        limine_modules(programs, assets, include_busybox, include_default_shell, true, false);
    let safe_shell_modules =
        limine_modules(programs, assets, include_busybox, include_default_shell, true, true);

    struct LimineEntry {
        title: String,
        kernel_cmdline: String,
        bootfb_only: bool,
        safe_shell_only: bool,
    }

    let default_loglevel = loglevel.unwrap_or("info");
    let mut entries = vec![
        LimineEntry {
            title: "ThingOS".to_string(),
            kernel_cmdline: format!("loglevel={}", default_loglevel),
            bootfb_only: false,
            safe_shell_only: false,
        },
        LimineEntry {
            title: "ThingOS (BootFB Fallback)".to_string(),
            kernel_cmdline: format!("loglevel={} display=bootfb", default_loglevel),
            bootfb_only: true,
            safe_shell_only: false,
        },
        LimineEntry {
            title: "ThingOS (Safe Shell)".to_string(),
            kernel_cmdline: format!(
                "loglevel={} display=bootfb sprout.safe=sh sprout.active_ui=terminal",
                default_loglevel
            ),
            bootfb_only: true,
            safe_shell_only: true,
        },
        LimineEntry {
            title: "ThingOS (Debug)".to_string(),
            kernel_cmdline: "loglevel=4".to_string(),
            bootfb_only: false,
            safe_shell_only: false,
        },
        LimineEntry {
            title: "ThingOS (Trace)".to_string(),
            kernel_cmdline: "loglevel=5".to_string(),
            bootfb_only: false,
            safe_shell_only: false,
        },
    ];

    if let Some(l) = loglevel {
        if l != "info" && l != "4" && l != "5" {
            entries.insert(
                0,
                LimineEntry {
                    title: "ThingOS (Custom)".to_string(),
                    kernel_cmdline: format!("loglevel={l}"),
                    bootfb_only: false,
                    safe_shell_only: false,
                },
            );
        }
    }
    if bootfb_default {
        if let Some(index) = entries.iter().position(|entry| entry.bootfb_only) {
            let entry = entries.remove(index);
            entries.insert(0, entry);
        }
    }

    for (index, entry) in entries.iter().enumerate() {
        if index > 0 {
            conf.push('\n');
        }

        conf.push_str(&format!("/{}\n", entry.title));
        conf.push_str("    protocol: limine\n");
        conf.push_str(&format!("    resolution: {res}\n"));
        conf.push_str("    kernel_path: boot():/boot/kernel\n");
        conf.push_str(&format!("    kernel_cmdline: {}\n", entry.kernel_cmdline));
        if entry.safe_shell_only {
            conf.push_str(&safe_shell_modules);
        } else if entry.bootfb_only {
            conf.push_str(&bootfb_only_modules);
        } else {
            conf.push_str(&all_graphics_modules);
        }
    }

    conf
}

fn limine_modules(
    programs: &[ProgramConfig],
    _assets: &[PathBuf],
    include_busybox: bool,
    include_default_shell: bool,
    bootfb_only: bool,
    safe_shell_only: bool,
) -> String {
    let mut modules = String::new();
    for prog in programs {
        if !prog.boot_module {
            continue;
        }
        if safe_shell_only && !is_safe_shell_program(prog.name) {
            continue;
        }
        // The BootFB fallback must not expose alternate display drivers:
        // Cambium scans /drivers and can bind PCI display devices independently
        // of Sprout's display=bootfb pipeline selection.
        if bootfb_only && is_non_bootfb_graphics_driver(prog.name) {
            continue;
        }
        let bin_path = format!("boot():/{}/{}", executable_subdir(prog.name), prog.name);
        modules.push_str(&format!("    module_path: {}\n", bin_path));
        if !is_driver(prog.name) {
            for alias in userspace_aliases(prog.name) {
                modules.push_str(&format!(
                    "    module_path: boot():/{}/{alias}\n",
                    executable_subdir(prog.name)
                ));
            }
        }
        let is_init_module = if safe_shell_only { prog.name == "sprout" } else { prog.is_init };
        if is_init_module {
            modules.push_str("    module_cmdline: init\n");
        }
    }

    modules.push_str("    module_path: boot():/public/fonts/unifont.hex\n");
    modules.push_str("    module_path: boot():/etc/roots/root\n");
    if include_default_shell && !safe_shell_only {
        modules.push_str("    module_path: boot():/etc/default/shell\n");
    }
    if include_busybox && !safe_shell_only {
        modules.push_str("    module_path: boot():/bin/busybox\n");
        modules.push_str("    module_path: boot():/bin/ash\n");
    }

    modules
}

fn generate_motd() -> String {
    concat!(
        "\x1b[1;32m\n",
        "        .-.\n",
        "       /   \\        \x1b[1;36mTHING-OS\x1b[1;32m\n",
        "      |     |       \x1b[0;36m\"People, places, things.\"\x1b[1;32m\n",
        "       \\   /        \n",
        "        `-'        \n",
        "       /   \\        v0.1  \u{2022}  \n",
        "      |     |       2026-04-16\n",
        "       \\   /\n",
        "        `-'\n",
        "\x1b[0m\n",
        "\x1b[2m--------------------------------------------------------------\x1b[0m\n",
        "\x1b[1m sprout has taken root. the system is awake.\x1b[0m\n",
        "\n",
        "  try:\n",
        "    \x1b[36mls /bin\x1b[0m       browse available shoots\n",
        "    \x1b[36mps\x1b[0m            observe living processes\n",
        "    \x1b[36mcat /version/os\x1b[0m  inspect the genome\n",
        "\n",
        "\x1b[2m--------------------------------------------------------------\x1b[0m\n",
    )
    .to_string()
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
    create_iso_root_layout(sh, iso_root)?;

    let mut asset_files = stage_public_assets(iso_root)?;

    asset_files.sort_by(|a, b| {
        let a_str = a.to_string_lossy();
        let b_str = b.to_string_lossy();

        let a_priority = if a_str.ends_with(".cur") || a_str.ends_with(".ani") {
            0
        } else if a_str.ends_with(".bmp") || a_str.ends_with(".png") {
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
        } else if b_str.ends_with(".bmp") || b_str.ends_with(".png") {
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
    println!("ISO: Using kernel from {kernel_src}");
    cmd!(sh, "file {kernel_src}").run()?;
    sh.copy_file(&kernel_src, iso_root.join("boot/kernel"))?;
    sh.copy_file("assets/fonts/unifont.hex", iso_root.join("public/fonts/unifont.hex"))?;

    sh.write_file(
        iso_root.join("etc/locale.conf"),
        "LOCALE=en_US\nTZ_OFFSET=-8\nOLLAMA_SERVER=http://10.0.2.2:11434\nOLLAMA_MODEL=tinyllama\n",
    )?;

    sh.write_file(
        iso_root.join("etc/profile"),
        "export PATH=/bin:/applications:/drivers\nalias ll='loglevel'\nalias halt='shutdown'\n",
    )?;

    sh.write_file(iso_root.join("etc/motd"), generate_motd())?;
    sh.write_file(
        iso_root.join("etc/fstab"),
        "none /net net defaults 0 0\nnone /https https defaults 0 0\nnone /media/cdrom iso9660 defaults 0 0\n",
    )?;
    sh.write_file(
        iso_root.join("etc/roots/root"),
        "# Thing-OS early root overlay.\nsource=/media/cdrom\ntarget=/\nflags=before,cor\nwait_ms=10000\n",
    )?;
    sh.write_file(iso_root.join("etc/hostname"), "thingos\n")?;

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

    let cwd = std::env::current_dir().unwrap();
    let target_json = if arch == "x86_64" {
        cwd.join("targets/x86_64-unknown-thingos.json")
    } else if arch == "riscv64" {
        cwd.join("targets/riscv64gc-unknown-thingos.json")
    } else {
        cwd.join(format!("targets/{arch}-unknown-thingos.json"))
    };
    let target = target_json.to_str().unwrap();

    // Phase 1: build all userspace programs concurrently (one vine per batch).
    build_programs_parallel(programs, target, "release")?;

    // Phase 2: stage compiled binaries into the ISO root sequentially.
    // Ordering here is stable and avoids concurrent writes to iso_root.
    println!("Staging userspace programs...");
    for prog in programs {
        let dest_subdir = executable_subdir(prog.name);
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
    let ld_so_src = iso_root.join(format!("{}/ld_so", executable_subdir("ld_so")));
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
        config.loglevel,
        config.bootfb_default,
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
        && stage1_rustc.exists()
        && stage1_rustc_wrapper.exists()
        && !skip_rustc_thingos;

    let build_std_crates = "core,alloc,std,panic_abort";

    let rustflags = String::from("-Awarnings");
    let mut cmd_obj = cmd!(
        sh,
        "cargo -Z build-std={build_std_crates} -Z build-std-features=compiler-builtins-mem {extra_flags...} build --target {target} --profile {profile} -p {name}"
    )
    .env("RUSTFLAGS", &rustflags);

    if target.ends_with(".json") && target.contains("thingos") {
        cmd_obj = cmd_obj.env("__CARGO_TESTS_ONLY_SRC_ROOT", std_src.to_str().unwrap());
    }

    if use_fork_rustc {
        let target_path = cwd.join("targets");
        cmd_obj = cmd_obj
            .env("RUSTC", &stage1_rustc_wrapper)
            .env("RUSTFLAGS", &rustflags)
            .env("RUST_TARGET_PATH", target_path);
    }

    for f in features {
        cmd_obj = cmd_obj.arg("--features").arg(f);
    }

    cmd_obj.run()?;
    Ok(())
}

/// Build all programs concurrently using a pool of parallel vines.
///
/// Each vine creates its own `Shell` in the current working directory and
/// invokes `build_userspace_app_with_features` for its assigned program.
/// Programs are processed in batches whose size is bounded by the available
/// CPU parallelism (capped at 8 to avoid overwhelming `cargo`'s file locks).
/// After all vines in a batch finish the next batch starts, so errors are
/// surfaced promptly without letting a later batch race past them.
fn build_programs_parallel(programs: &[ProgramConfig], target: &str, profile: &str) -> Result<()> {
    let num_parallel =
        std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4).min(8).max(1);

    println!("Building {} userspace programs ({num_parallel} build vines)...", programs.len());

    for chunk in programs.chunks(num_parallel) {
        let handles: Vec<std::thread::JoinHandle<Result<()>>> = chunk
            .iter()
            .map(|prog| {
                let target = target.to_string();
                let profile = profile.to_string();
                let features: Vec<String> = prog.features.iter().map(|s| s.to_string()).collect();
                let name = prog.name;
                std::thread::spawn(move || -> Result<()> {
                    // Each vine creates its own Shell so none share mutable
                    // state; Shell::new() anchors to the process working dir.
                    let sh = Shell::new()?;
                    let feature_refs: Vec<&str> = features.iter().map(|s| s.as_str()).collect();
                    build_userspace_app_with_features(&sh, name, &target, &profile, &feature_refs)
                })
            })
            .collect();

        for handle in handles {
            handle.join().map_err(|_| anyhow::anyhow!("build vine panicked"))??;
        }
    }

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
    cmd!(
        sh,
        "mmd -i {hdd}@@1M ::/EFI ::/EFI/BOOT ::/boot ::/boot/limine ::/bin ::/applications ::/lib ::/etc ::/dev ::/proc ::/sys ::/run ::/tmp ::/media ::/media/cdrom ::/drivers ::/session ::/public ::/services ::/version"
    )
    .run()?;
    for (src, dst) in PUBLIC_ASSET_DIRS {
        let dst_parent = if let Some((parent, _)) = dst.rsplit_once('/') {
            format!("::/public/{parent}")
        } else {
            String::from("::/public")
        };
        cmd!(sh, "mmd -i {hdd}@@1M {dst_parent}").run().ok();
        cmd!(sh, "mcopy -i {hdd}@@1M -s {src} {dst_parent}").run()?;
    }
    stage_default_wallpapers_hdd(sh, &hdd, arch)?;

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
        "export PATH=/bin:/applications:/drivers\nalias ll='loglevel'\nalias halt='shutdown'\n",
    )?;
    sh.write_file("motd", generate_motd())?;

    // create /etc in the fat32 image if it doesn't exist
    cmd!(sh, "mmd -i {hdd}@@1M ::/etc").run().ok();
    cmd!(sh, "mcopy -i {hdd}@@1M profile ::/etc/profile").run()?;
    cmd!(sh, "mcopy -i {hdd}@@1M motd ::/etc/motd").run()?;

    sh.remove_path("profile")?;
    sh.remove_path("motd")?;

    let limine_conf_content =
        generate_limine_config(sh, programs, &asset_files, None, None, false, false, false);
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
        "xhci",
    ];
    drivers.contains(&name)
}

fn is_bin_program(name: &str) -> bool {
    matches!(
        name,
        "sh" | "ls"
            | "ln"
            | "cat"
            | "head"
            | "tail"
            | "wc"
            | "yes"
            | "cp"
            | "mv"
            | "rm"
            | "rmdir"
            | "mkdir"
            | "echo"
            | "printf"
            | "pwd"
            | "touch"
            | "stat"
            | "dirname"
            | "basename"
            | "sleep"
            | "sort"
            | "env"
            | "date"
            | "uname"
            | "true"
            | "false"
            | "tee"
    )
}

fn is_service_program(name: &str) -> bool {
    matches!(name, "bloom" | "bristle" | "cambium" | "fatd" | "iso9660d" | "mesocarp" | "netd")
}

fn executable_subdir(name: &str) -> &'static str {
    if is_driver(name) {
        "drivers"
    } else if is_service_program(name) {
        "services"
    } else if is_bin_program(name) {
        "bin"
    } else {
        "applications"
    }
}

fn is_bootstrap_boot_module(name: &str) -> bool {
    is_driver(name)
        || is_bin_program(name)
        || is_service_program(name)
        || matches!(
            name,
            "sprout" | "terminal" | "wayland_hello" | "clock" | "leaf" | "fetchd" | "httpsd"
        )
}

fn is_non_bootfb_graphics_driver(name: &str) -> bool {
    (name.starts_with("display_") && name != "display_bootfb") || name == "virtio_gpu"
}

fn is_safe_shell_program(name: &str) -> bool {
    is_bin_program(name) || matches!(name, "sprout" | "terminal" | "bristle")
}

fn userspace_aliases(name: &str) -> &'static [&'static str] {
    match name {
        // mesocarp is the mDNS daemon; `mount -t mdns` resolves to /services/mdns
        // (or /services/mdnsd). Aliases ensure both spellings land next to the
        // real binary in the image.
        "mesocarp" => &["mdns", "mdnsd"],
        _ => &[],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_program(name: &'static str) -> ProgramConfig {
        ProgramConfig { name, is_init: false, boot_module: true, features: vec![] }
    }

    fn limine_entry<'a>(conf: &'a str, title: &str) -> &'a str {
        let marker = format!("/{title}\n");
        let start = conf.find(&marker).expect("entry marker");
        let rest = &conf[start..];
        let next = rest[marker.len()..]
            .find("\n/")
            .map(|offset| marker.len() + offset + 1)
            .unwrap_or(rest.len());
        &rest[..next]
    }

    #[test]
    fn bootfb_fallback_entry_omits_other_graphics_drivers() {
        let sh = Shell::new().expect("shell");
        let programs = [
            test_program("display_bootfb"),
            test_program("display_virtio_gpu"),
            test_program("display_fake"),
            test_program("virtio_gpu"),
            test_program("bloom"),
        ];

        let conf = generate_limine_config(&sh, &programs, &[], None, None, false, false, false);
        let normal_entry = limine_entry(&conf, "ThingOS");
        let bootfb_entry = limine_entry(&conf, "ThingOS (BootFB Fallback)");

        assert!(normal_entry.contains("module_path: boot():/drivers/display_bootfb"));
        assert!(normal_entry.contains("module_path: boot():/drivers/display_virtio_gpu"));
        assert!(normal_entry.contains("module_path: boot():/drivers/display_fake"));
        assert!(normal_entry.contains("module_path: boot():/drivers/virtio_gpu"));

        assert!(bootfb_entry.contains("kernel_cmdline: loglevel=info display=bootfb"));
        assert!(bootfb_entry.contains("module_path: boot():/drivers/display_bootfb"));
        assert!(!bootfb_entry.contains("module_path: boot():/drivers/display_virtio_gpu"));
        assert!(!bootfb_entry.contains("module_path: boot():/drivers/display_fake"));
        assert!(!bootfb_entry.contains("module_path: boot():/drivers/virtio_gpu"));
        assert!(bootfb_entry.contains("module_path: boot():/services/bloom"));
    }

    #[test]
    fn safe_shell_entry_loads_only_sprout_sh_bristle_echo_and_terminal() {
        let sh = Shell::new().expect("shell");
        let mut sprout = test_program("sprout");
        sprout.is_init = true;
        let mut shell = test_program("sh");
        shell.is_init = true;
        let programs = [
            sprout,
            shell,
            test_program("echo"),
            test_program("cat"),
            test_program("cp"),
            test_program("ls"),
            test_program("mkdir"),
            test_program("stat"),
            test_program("terminal"),
            test_program("bloom"),
            test_program("bristle"),
            test_program("display_bootfb"),
            test_program("display_virtio_gpu"),
        ];

        let conf = generate_limine_config(&sh, &programs, &[], None, None, false, true, true);
        let safe_entry = limine_entry(&conf, "ThingOS (Safe Shell)");

        assert!(safe_entry.contains(
            "kernel_cmdline: loglevel=info display=bootfb sprout.safe=sh sprout.active_ui=terminal"
        ));
        assert!(safe_entry.contains("module_path: boot():/applications/sprout"));
        assert!(safe_entry.contains("module_path: boot():/bin/sh"));
        assert!(safe_entry.contains("module_path: boot():/bin/echo"));
        assert!(safe_entry.contains("module_path: boot():/bin/cat"));
        assert!(safe_entry.contains("module_path: boot():/bin/cp"));
        assert!(safe_entry.contains("module_path: boot():/bin/ls"));
        assert!(safe_entry.contains("module_path: boot():/bin/mkdir"));
        assert!(safe_entry.contains("module_path: boot():/bin/stat"));
        assert!(safe_entry.contains("module_path: boot():/applications/terminal"));
        assert!(safe_entry.contains("module_path: boot():/services/bristle"));
        assert!(!safe_entry.contains("module_path: boot():/services/bloom"));
        assert!(!safe_entry.contains("module_path: boot():/bin/busybox"));
        assert!(!safe_entry.contains("module_path: boot():/bin/ash"));
        assert!(!safe_entry.contains("module_path: boot():/etc/default/shell"));
        assert!(!safe_entry.contains("module_path: boot():/drivers/display_bootfb"));
        assert!(!safe_entry.contains("module_path: boot():/drivers/display_virtio_gpu"));
        assert_eq!(safe_entry.matches("module_cmdline: init").count(), 1);
    }

    #[test]
    fn limine_config_leaves_assets_on_the_filesystem() {
        let sh = Shell::new().expect("shell");
        let programs = [test_program("bloom")];
        let assets = [
            PathBuf::from("assets/cursors/future/default.svg"),
            PathBuf::from("assets/cursors/future/fleur.svg"),
            PathBuf::from("assets/cursors/future/top_side.svg"),
            PathBuf::from("assets/cursors/future/bottom_right_corner.svg"),
            PathBuf::from("assets/cursors/plain/Normal.cur"),
        ];

        let conf = generate_limine_config(&sh, &programs, &assets, None, None, false, false, false);

        assert!(!conf.contains("module_path: boot():/public/cursors/future/default.svg"));
        assert!(!conf.contains("module_path: boot():/public/cursors/future/fleur.svg"));
        assert!(!conf.contains("module_path: boot():/public/cursors/future/top_side.svg"));
        assert!(
            !conf.contains("module_path: boot():/public/cursors/future/bottom_right_corner.svg")
        );
        assert!(!conf.contains("module_path: boot():/public/cursors/plain/Normal.cur"));
    }

    #[test]
    fn iso_root_layout_is_root_shaped_without_provider_placeholders() {
        for dir in [
            "bin",
            "applications",
            "lib",
            "etc",
            "dev",
            "proc",
            "sys",
            "run",
            "tmp",
            "media",
            "drivers",
            "session",
            "public",
            "services",
            "version",
        ] {
            assert!(ISO_ROOT_DIRS.contains(&dir), "missing canonical ISO root dir {dir}");
        }
        assert!(ISO_ROOT_DIRS.contains(&"etc/roots"));
        assert!(ISO_ROOT_DIRS.contains(&"media/cdrom"));
        assert!(!ISO_ROOT_DIRS.contains(&"app"));
        assert!(!ISO_ROOT_DIRS.contains(&"drv"));
        assert!(!ISO_ROOT_DIRS.contains(&"pub"));
        assert!(!ISO_ROOT_DIRS.contains(&"ses"));
        assert!(!ISO_ROOT_DIRS.contains(&"srv"));
        assert!(!ISO_ROOT_DIRS.contains(&"ver"));
        assert!(!ISO_ROOT_DIRS.contains(&"share"));
        assert!(!ISO_ROOT_DIRS.contains(&"vol"));
        assert!(!ISO_ROOT_DIRS.contains(&"data"));
        assert!(!ISO_ROOT_DIRS.contains(&"net"));
        assert!(!ISO_ROOT_DIRS.contains(&"hosts"));
        assert!(!ISO_ROOT_DIRS.contains(&"https"));
        assert!(!ISO_ROOT_DIRS.contains(&"mnt"));
        assert!(!ISO_ROOT_DIRS.contains(&"usr/lib"));
    }

    #[test]
    fn iso_public_assets_are_explicit_runtime_subtrees() {
        assert!(PUBLIC_ASSET_DIRS.contains(&("assets/cursors", "cursors")));
        assert!(PUBLIC_ASSET_DIRS.contains(&("assets/fonts", "fonts")));
        assert!(PUBLIC_ASSET_DIRS.contains(&("assets/icons/lucide", "icons/lucide")));
        assert!(PUBLIC_ASSET_DIRS.contains(&("assets/themes", "themes")));
        assert!(PUBLIC_ASSET_DIRS.contains(&("assets/wallpapers", "wallpapers")));
        assert!(!PUBLIC_ASSET_DIRS.iter().any(|(_, dst)| dst.starts_with("assets")));
        assert!(!PUBLIC_ASSET_DIRS.iter().any(|(_, dst)| dst.starts_with("icons/chicago95")));
        assert!(!PUBLIC_ASSET_DIRS.iter().any(|(_, dst)| dst.starts_with("pci")));
    }

    #[test]
    fn executable_staging_keeps_bin_to_core_commands() {
        for name in ["sh", "ls", "cat", "cp", "mkdir", "printf", "stat"] {
            assert_eq!(executable_subdir(name), "bin", "{name} should stay in /bin");
        }
        for name in ["sprout", "grep", "find"] {
            assert_eq!(
                executable_subdir(name),
                "applications",
                "{name} should move to /applications"
            );
        }
        for name in ["bristle", "bloom", "cambium", "fatd", "iso9660d", "mesocarp", "netd"] {
            assert_eq!(executable_subdir(name), "services", "{name} should move to /services");
        }
        for name in ["ahci_disk", "display_bootfb", "virtio_netd", "xhci"] {
            assert_eq!(executable_subdir(name), "drivers", "{name} should move to /drivers");
        }
    }

    #[test]
    fn default_limine_config_keeps_bootstrap_supervised_services_and_unifont() {
        let sh = Shell::new().expect("shell");
        let programs = default_programs();

        let conf = generate_limine_config(&sh, &programs, &[], None, None, false, false, false);
        let normal_entry = limine_entry(&conf, "ThingOS");

        assert!(normal_entry.contains("module_path: boot():/applications/sprout"));
        assert!(normal_entry.contains("module_path: boot():/bin/sh"));
        assert!(normal_entry.contains("module_path: boot():/services/cambium"));
        assert!(normal_entry.contains("module_path: boot():/services/fatd"));
        assert!(normal_entry.contains("module_path: boot():/services/iso9660d"));
        assert!(normal_entry.contains("module_path: boot():/services/mesocarp"));
        assert!(normal_entry.contains("module_path: boot():/services/mdns"));
        assert!(normal_entry.contains("module_path: boot():/services/mdnsd"));
        assert!(normal_entry.contains("module_path: boot():/services/netd"));
        assert!(normal_entry.contains("module_path: boot():/bin/echo"));
        assert!(normal_entry.contains("module_path: boot():/bin/cat"));
        assert!(normal_entry.contains("module_path: boot():/bin/cp"));
        assert!(normal_entry.contains("module_path: boot():/bin/ls"));
        assert!(normal_entry.contains("module_path: boot():/bin/mkdir"));
        assert!(normal_entry.contains("module_path: boot():/bin/stat"));
        assert!(normal_entry.contains("module_path: boot():/services/bloom"));
        assert!(normal_entry.contains("module_path: boot():/services/bristle"));
        assert!(normal_entry.contains("module_path: boot():/applications/clock"));
        assert!(normal_entry.contains("module_path: boot():/drivers/ata_disk"));
        assert!(normal_entry.contains("module_path: boot():/drivers/ahci_disk"));
        assert!(normal_entry.contains("module_path: boot():/public/fonts/unifont.hex"));
        assert!(normal_entry.contains("module_path: boot():/etc/roots/root"));

        assert!(!normal_entry.contains("module_path: boot():/bin/ps"));
        assert!(!normal_entry.contains("module_path: boot():/bin/grep"));
        assert!(!normal_entry.contains("module_path: boot():/lib/libpistil.so"));
        assert!(!normal_entry.contains("module_path: boot():/etc/fstab"));
        assert!(!normal_entry.contains("module_path: boot():/public/wallpapers/flower.png"));
        assert!(!normal_entry.contains("module_path: boot():/public/wallpapers/flower.bmp"));
    }
}
