//! BDD World - holds test state during scenario execution.

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::{Arc, OnceLock};

use cucumber::World;
use regex::Regex;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::net::{TcpStream, UnixStream};
use tokio::process::{Child, Command};
use tokio::sync::Mutex;

/// Default timeout for step helpers that poll for asynchronous behavior.
pub const DEFAULT_STEP_TIMEOUT_SECS: f64 = 5.0;

/// The test world shared across all steps in a scenario.
#[derive(Debug, Default, World)]
pub struct ThingOsWorld {
    /// Target architecture for this test run
    pub arch: String,
    /// QEMU child process
    #[world(skip)]
    pub qemu: Option<Child>,
    /// Accumulated serial output
    pub serial_log: Arc<Mutex<String>>,
    /// Path to QMP socket for QEMU control
    /// Path to QMP connection for step logic (separate from reporter)
    #[world(skip)]
    pub qmp_socket: Option<PathBuf>,
    /// QMP connection for step logic (separate from reporter)
    #[world(skip)]
    pub qmp_control: Option<QmpEndpoint>,
    /// VNC display number (for screenshot capture)
    #[world(skip)]
    pub vnc_display: Option<u16>,
    /// Path to the ISO file created for this scenario
    #[world(skip)]
    pub iso_path: Option<PathBuf>,
    /// Work directory for storing sockets
    #[world(skip)]
    pub work_dir: PathBuf,
    /// Sender for serial input (persistent connection)
    #[world(skip)]
    pub serial_tx: Option<tokio::sync::mpsc::UnboundedSender<Vec<u8>>>,
    /// Byte offset in serial_log captured before the last command was sent.
    #[world(skip)]
    pub serial_checkpoint: usize,
    /// The last command text typed via the serial console (used to filter echoed
    /// input from command-output assertions).
    #[world(skip)]
    pub last_typed_command: Option<String>,
    /// Scenario-wide default timeout derived from feature/scenario tags.
    #[world(skip)]
    pub scenario_timeout_secs: Option<f64>,
    /// Force the BootFB fallback boot entry for this scenario.
    #[world(skip)]
    pub force_bootfb: bool,
}

#[derive(Clone, Debug)]
pub enum QmpEndpoint {
    Unix(PathBuf),
    Tcp(std::net::SocketAddr),
}

#[derive(Clone, Copy, Debug, Default)]
struct BootOptions {
    qemu_xhci: bool,
    /// Attach a freshly-created FAT disk image via the xHCI USB controller.
    usb_fat_image: bool,
}

pub(crate) trait QmpStream: AsyncRead + AsyncWrite {}

impl<T: AsyncRead + AsyncWrite + ?Sized> QmpStream for T {}

impl ThingOsWorld {
    pub fn default_step_timeout_secs() -> f64 {
        std::env::var("BDD_STEP_TIMEOUT_SECS")
            .ok()
            .and_then(|value| value.parse::<f64>().ok())
            .filter(|value| *value > 0.0)
            .unwrap_or(DEFAULT_STEP_TIMEOUT_SECS)
    }

    pub fn effective_step_timeout_secs(&self) -> f64 {
        self.scenario_timeout_secs.unwrap_or_else(Self::default_step_timeout_secs)
    }

    pub fn parse_timeout_tag(tag: impl AsRef<str>) -> Option<f64> {
        let tag = tag.as_ref().trim_start_matches('@');
        let value = tag
            .strip_prefix("timeout.")
            .or_else(|| tag.strip_prefix("timeout="))
            .or_else(|| tag.strip_prefix("timeout-"))?;
        let value = value.strip_suffix('s').unwrap_or(value);
        value.parse::<f64>().ok().filter(|parsed| *parsed > 0.0)
    }

    fn bdd_cache_dir() -> PathBuf {
        PathBuf::from("target").join("bdd").join("cache")
    }

    fn env_flag(name: &str) -> bool {
        matches!(
            std::env::var(name).ok().as_deref().map(str::to_ascii_lowercase).as_deref(),
            Some("1") | Some("true") | Some("yes") | Some("on")
        )
    }

    fn cached_iso_path(
        arch: &str,
        resolution: &str,
        loglevel: &str,
        force_bootfb: bool,
    ) -> PathBuf {
        let safe_resolution: String =
            resolution.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect();
        let safe_loglevel: String =
            loglevel.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect();
        let display = if force_bootfb { "bootfb" } else { "auto" };
        PathBuf::from("target").join("bdd").join("images").join(format!(
            "thing-os-bdd-{}-{}-{}-{}.iso",
            arch, safe_resolution, safe_loglevel, display
        ))
    }

    fn bdd_audiodev_arg() -> Option<String> {
        let backend = std::env::var("BDD_AUDIODEV").unwrap_or_else(|_| "none".to_string());
        let backend = backend.trim();
        if backend.is_empty()
            || backend.eq_ignore_ascii_case("off")
            || backend.eq_ignore_ascii_case("false")
            || backend.eq_ignore_ascii_case("0")
        {
            None
        } else if backend.split(',').any(|part| part.trim_start().starts_with("id=")) {
            Some(backend.to_string())
        } else {
            Some(format!("{backend},id=audio0"))
        }
    }

    fn fetch_cache_marker_path(arch: &str) -> PathBuf {
        Self::bdd_cache_dir().join(format!("fetch-{}.stamp", arch))
    }

    fn fetch_prerequisites(arch: &str) -> Vec<PathBuf> {
        vec![
            PathBuf::from("assets/fonts/unifont.hex"),
            PathBuf::from("vendor/limine/limine"),
            PathBuf::from(format!("vendor/ovmf/ovmf-code-{}.fd", arch)),
            PathBuf::from(format!("vendor/ovmf/ovmf-vars-{}.fd", arch)),
        ]
    }

    fn fetch_prerequisites_ready(arch: &str) -> bool {
        Self::fetch_prerequisites(arch).iter().all(|p| Path::new(p).exists())
    }

    async fn ensure_fetch_prerequisites(
        &self,
        arch: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        static GLOBAL_FETCH_LOCK: OnceLock<tokio::sync::Mutex<()>> = OnceLock::new();
        let lock = GLOBAL_FETCH_LOCK.get_or_init(|| tokio::sync::Mutex::new(()));
        let _guard = lock.lock().await;

        let force_fetch = Self::env_flag("BDD_FORCE_FETCH");
        let stamp_path = Self::fetch_cache_marker_path(arch);
        let prerequisites_ready = Self::fetch_prerequisites_ready(arch);

        if !force_fetch && prerequisites_ready {
            if stamp_path.exists() {
                eprintln!("[bdd] Reusing cached fetch assets for arch {arch}");
                return Ok(());
            }
            if let Some(parent) = stamp_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&stamp_path, b"ready\n")?;
            eprintln!("[bdd] Fetch prerequisites already available for arch {arch}");
            return Ok(());
        }

        eprintln!("[bdd] Running `xtask fetch` for BDD prerequisites (arch={arch})...");
        let output = std::process::Command::new("cargo")
            .args(["run", "-p", "xtask", "--", "fetch"])
            .output()?;

        if !output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!(
                "Failed to fetch BDD prerequisites for arch {}.\nstdout:\n{}\nstderr:\n{}",
                arch, stdout, stderr
            )
            .into());
        }

        if !Self::fetch_prerequisites_ready(arch) {
            return Err(format!(
                "Fetch completed but BDD prerequisites are still missing for arch {}: {:?}",
                arch,
                Self::fetch_prerequisites(arch)
            )
            .into());
        }

        if let Some(parent) = stamp_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&stamp_path, b"ready\n")?;
        eprintln!("[bdd] Cached fetch prerequisites for arch {arch}");
        Ok(())
    }

    /// Boot the OS in QEMU for the given architecture.
    /// Reuses a cached ISO for the architecture/resolution when available.
    pub async fn boot(&mut self, arch: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.boot_with_options(arch, BootOptions::default()).await
    }

    /// Boot the OS with an explicit qemu-xhci PCI controller on x86_64.
    pub async fn boot_with_qemu_xhci(
        &mut self,
        arch: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.boot_with_options(arch, BootOptions { qemu_xhci: true, usb_fat_image: false }).await
    }

    /// Boot the OS with a USB FAT disk image attached via the xHCI controller.
    ///
    /// Creates a small MBR-partitioned FAT16 disk image containing a single
    /// `hello.txt` file in the work directory, then passes it to QEMU as a
    /// USB mass-storage device.
    pub async fn boot_with_usb_fat_image(
        &mut self,
        arch: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.boot_with_options(arch, BootOptions { qemu_xhci: true, usb_fat_image: true }).await
    }

    async fn boot_with_options(
        &mut self,
        arch: &str,
        options: BootOptions,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.arch = arch.to_string();
        {
            let mut log = self.serial_log.lock().await;
            log.clear();
        }
        crate::artifacts::clear_latest_serial().await;

        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        let pid = std::process::id();

        self.work_dir = std::env::temp_dir().join(format!("thingos-bdd-{}-{}", pid, nanos));
        std::fs::create_dir_all(&self.work_dir)?;
        self.ensure_fetch_prerequisites(arch).await?;

        // Get resolution from environment (default 1920x1080 for BDD tests)
        let resolution =
            std::env::var("BDD_RESOLUTION").unwrap_or_else(|_| "1920x1080".to_string());

        let loglevel = std::env::var("BDD_LOGLEVEL").unwrap_or_else(|_| "3".to_string());

        let iso_path = Self::cached_iso_path(arch, &resolution, &loglevel, self.force_bootfb);
        let force_rebuild = Self::env_flag("BDD_FORCE_REBUILD_IMAGE");

        if let Some(parent) = iso_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        if iso_path.exists() && !force_rebuild {
            eprintln!("[bdd] Reusing cached ISO: {}", iso_path.display());
        } else {
            if force_rebuild && iso_path.exists() {
                eprintln!("[bdd] Forcing ISO rebuild: {}", iso_path.display());
            } else {
                eprintln!(
                    "[bdd] Building ISO {} with resolution {}{}...",
                    iso_path.display(),
                    resolution,
                    if self.force_bootfb { " (bootfb default)" } else { "" }
                );
            }

            let iso_output = iso_path.to_string_lossy().to_string();
            // Use `cargo run -p xtask -- ...` instead of `cargo xtask ...` so CI does not
            // depend on Cargo alias resolution from .cargo/config.toml.
            let mut build_cmd = std::process::Command::new("cargo");
            build_cmd.args([
                "run",
                "-p",
                "xtask",
                "--",
                "iso",
                "--env",
                arch,
                "--resolution",
                &resolution,
                "--output",
                &iso_output,
                "--loglevel",
                &loglevel,
            ]);
            if self.force_bootfb {
                build_cmd.arg("--bootfb-default");
            }
            let build_output = build_cmd.env("RUSTFLAGS", "-Awarnings").output()?;

            if !build_output.status.success() {
                let stdout = String::from_utf8_lossy(&build_output.stdout);
                let stderr = String::from_utf8_lossy(&build_output.stderr);
                return Err(format!(
                    "Failed to build ISO: {}\nstdout:\n{}\nstderr:\n{}",
                    iso_path.display(),
                    stdout,
                    stderr
                )
                .into());
            }

            if !iso_path.exists() {
                return Err(format!(
                    "Failed to build ISO: {} (xtask succeeded but output file was not created)",
                    iso_path.display()
                )
                .into());
            }
        }

        self.iso_path = Some(iso_path.clone());
        eprintln!("[bdd] ISO ready: {}", iso_path.display());

        // Optionally build a USB FAT disk image before starting QEMU.
        let usb_fat_path = if options.usb_fat_image {
            let fat_img = self.work_dir.join("usb.img");
            Self::create_usb_fat_image(&fat_img)?;
            eprintln!("[bdd] USB FAT image ready: {}", fat_img.display());
            Some(fat_img)
        } else {
            None
        };

        let ovmf_code = format!("vendor/ovmf/ovmf-code-{}.fd", arch);
        let ovmf_vars = format!("vendor/ovmf/ovmf-vars-{}.fd", arch);

        // Use localhost TCP for QMP. Unix sockets are denied in some restricted
        // environments, but several BDD scenarios need QMP for input injection
        // and screenshots, so leaving QMP disabled makes them false-pass.
        let qmp_addr = {
            let listener = std::net::TcpListener::bind(("127.0.0.1", 0))?;
            let addr = listener.local_addr()?;
            drop(listener);
            addr
        };
        self.qmp_socket = None;
        self.qmp_control = Some(QmpEndpoint::Tcp(qmp_addr));

        // Use a random VNC display to avoid conflicts with potential zombies
        let vnc_nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        let vnc_display = ((vnc_nanos % 5000) + 1000) as u16;
        self.vnc_display = Some(vnc_display);

        let qemu_bin = match arch {
            "x86_64" => "qemu-system-x86_64",
            "aarch64" => "qemu-system-aarch64",
            "riscv64" => "qemu-system-riscv64",
            "loongarch64" => "qemu-system-loongarch64",
            _ => return Err(format!("Unsupported architecture: {}", arch).into()),
        };

        // Build QEMU command with serial output to stdio and QMP control
        let mut cmd = Command::new(qemu_bin);

        // Handle machine type and pflash - riscv64 requires special blockdev syntax
        match arch {
            "x86_64" => {
                cmd.args(["-M", "q35,usb=off,vmport=off,i8042=on"]);
                cmd.args(["-device", "virtio-vga"]);
                if options.qemu_xhci {
                    cmd.args(["-device", "qemu-xhci,id=xhci"]);
                    if let Some(ref fat_path) = usb_fat_path {
                        cmd.args([
                            "-blockdev",
                            &format!(
                                "driver=raw,node-name=usbdisk,file.driver=file,file.filename={}",
                                fat_path.to_string_lossy()
                            ),
                        ]);
                    } else {
                        cmd.args([
                            "-blockdev",
                            "driver=null-co,node-name=usbdisk,size=1073741824",
                        ]);
                    }
                    cmd.args(["-device", "usb-storage,bus=xhci.0,drive=usbdisk"]);
                }
                cmd.args([
                    "-drive",
                    &format!("if=pflash,unit=0,format=raw,file={},readonly=on", ovmf_code),
                ]);
                cmd.args(["-drive", &format!("if=pflash,unit=1,format=raw,file={}", ovmf_vars)]);
                cmd.args(["-cdrom", &iso_path.to_string_lossy()]);

                cmd.args(["-device", "virtio-net-pci,netdev=n0"]);
                cmd.args(["-netdev", "user,id=n0"]);
                if let Some(audiodev) = Self::bdd_audiodev_arg() {
                    cmd.args(["-audiodev", &audiodev]);
                    cmd.args(["-device", "virtio-sound-pci,audiodev=audio0"]);
                }
            }
            "aarch64" => {
                cmd.args(["-M", "virt"]);
                cmd.args(["-cpu", "cortex-a72"]);
                cmd.args(["-device", "ramfb"]);
                cmd.args(["-device", "qemu-xhci"]);
                cmd.args(["-device", "usb-kbd"]);
                cmd.args(["-device", "usb-mouse"]);
                cmd.args([
                    "-drive",
                    &format!("if=pflash,unit=0,format=raw,file={},readonly=on", ovmf_code),
                ]);
                cmd.args(["-drive", &format!("if=pflash,unit=1,format=raw,file={}", ovmf_vars)]);
                cmd.args(["-cdrom", &iso_path.to_string_lossy()]);
                cmd.args(["-semihosting"]);
            }
            "riscv64" => {
                // riscv64 virt requires blockdev syntax with machine-level pflash assignment
                // Also uses virtio-blk instead of -cdrom since riscv64 virt doesn't expose cdrom to UEFI properly
                cmd.args([
                    "-blockdev",
                    &format!("node-name=pflash0,driver=file,read-only=on,filename={}", ovmf_code),
                ]);
                cmd.args([
                    "-blockdev",
                    &format!("node-name=pflash1,driver=file,filename={}", ovmf_vars),
                ]);
                cmd.args(["-M", "virt,pflash0=pflash0,pflash1=pflash1"]);
                cmd.args(["-cpu", "rv64"]);
                cmd.args(["-device", "ramfb"]);
                cmd.args(["-device", "qemu-xhci"]);
                cmd.args(["-device", "usb-kbd"]);
                cmd.args(["-device", "usb-mouse"]);
                cmd.args([
                    "-drive",
                    &format!(
                        "file={},format=raw,if=none,id=drive0,readonly=on",
                        iso_path.display()
                    ),
                ]);
                cmd.args(["-device", "virtio-blk-device,drive=drive0"]);
            }
            "loongarch64" => {
                cmd.args(["-M", "virt"]);
                cmd.args(["-cpu", "la464"]);
                cmd.args(["-device", "ramfb"]);
                cmd.args(["-device", "qemu-xhci"]);
                cmd.args(["-device", "usb-kbd"]);
                cmd.args(["-device", "usb-mouse"]);
                cmd.args([
                    "-drive",
                    &format!("if=pflash,unit=0,format=raw,file={},readonly=on", ovmf_code),
                ]);
                cmd.args(["-drive", &format!("if=pflash,unit=1,format=raw,file={}", ovmf_vars)]);
                cmd.args(["-cdrom", &iso_path.to_string_lossy()]);
            }
            _ => {}
        }

        cmd.args([
            "-m",
            "2G",
            "-smp",
            "4",
            // Disable default display, use VNC instead
            "-display",
            "none",
            "-no-shutdown",
            // Drive the serial console over stdio instead of a UNIX socket.
            // Some environments deny QEMU's socket creation, which breaks all BDD scenarios
            // before the guest even starts.
            "-serial",
            "stdio",
        ]);
        cmd.args(["-qmp", &format!("tcp:{},server=on,wait=off", qmp_addr)]);

        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped()); // Capture stderr too to see QEMU errors
        cmd.stdin(Stdio::piped());

        let mut child = cmd.spawn()?;

        let mut child_stdout = child.stdout.take().ok_or("Failed to capture QEMU stdout")?;
        let mut child_stdin = child.stdin.take().ok_or("Failed to capture QEMU stdin")?;
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Vec<u8>>();
        self.serial_tx = Some(tx);

        // Spawn a task to read serial output from QEMU stdio and sync it to the global cache.
        let serial_log = self.serial_log.clone();
        tokio::spawn(async move {
            use tokio::io::{AsyncReadExt, AsyncWriteExt};
            let mut buf = vec![0u8; 4096];
            let mut last_update = std::time::Instant::now();
            loop {
                tokio::select! {
                    // Handle incoming data from QEMU
                    result = child_stdout.read(&mut buf) => {
                        match result {
                            Ok(n) if n > 0 => {
                                let text = String::from_utf8_lossy(&buf[..n]);
                                let mut log = serial_log.lock().await;
                                log.push_str(&text);

                                // Sync to global cache for reporter access (throttled)
                                if last_update.elapsed().as_millis() > 50 {
                                    crate::artifacts::set_latest_serial(&log).await;
                                    last_update = std::time::Instant::now();
                                }
                            }
                            _ => break, // EOF or error
                        }
                    }
                    // Handle outgoing data to QEMU
                    Some(data) = rx.recv() => {
                        if let Err(e) = child_stdin.write_all(&data).await {
                            eprintln!("│  │  │      ⚠️ Failed to write to serial stream: {}", e);
                        }
                        let _ = child_stdin.flush().await;
                    }
                }
            }
        });

        // Also read stderr. On some architectures QEMU routes serial-style output there when
        // running with `-serial stdio`, so include it in the collected serial log as well.
        if let Some(stderr) = child.stderr.take() {
            let serial_log = self.serial_log.clone();
            tokio::spawn(async move {
                let reader = BufReader::new(stderr);
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    {
                        let mut log = serial_log.lock().await;
                        log.push_str(&line);
                        log.push('\n');
                        crate::artifacts::set_latest_serial(&log).await;
                    }
                    eprintln!("[qemu-stderr] {}", line);
                }
            });
        }

        self.qemu = Some(child);

        crate::artifacts::set_qmp_stream(None).await;

        Ok(())
    }

    /// Take a screenshot using the world's private QMP connection.
    pub async fn take_screenshot(
        &mut self,
        output_path: &std::path::Path,
    ) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
        // Ensure output directory exists
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Get absolute path for QEMU
        let ppm_path = output_path.with_extension("ppm");
        let ppm_abs = std::fs::canonicalize(output_path.parent().ok_or("Invalid output path")?)?
            .join(ppm_path.file_name().ok_or("Invalid PPM path")?);

        let cmd = format!(
            r#"{{"execute": "screendump", "arguments": {{"filename": "{}"}}}}"#,
            ppm_abs.display()
        );

        let endpoint = self.qmp_control.as_ref().ok_or("No world QMP connection")?;
        let mut stream = Self::connect_qmp(endpoint).await.map_err(|e| e.to_string())?;

        let resp = crate::artifacts::qmp::execute_on_stream(&mut stream, &cmd).await?;
        if resp.contains("error") {
            return Err(format!("QMP error: {}", resp).into());
        }

        // Give QEMU a moment to flush
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        if !ppm_path.exists() {
            return Err("Screenshot file not created".into());
        }

        // Convert PPM to PNG
        let png_path = output_path.with_extension("png");
        let img = image::open(&ppm_path)?;
        img.save(&png_path)?;
        let _ = std::fs::remove_file(&ppm_path);

        Ok(png_path)
    }

    pub async fn execute_qmp_control(
        &self,
        cmd: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let endpoint = self.qmp_control.as_ref().ok_or("No world QMP connection")?;
        let mut stream = Self::connect_qmp(endpoint).await?;
        crate::artifacts::qmp::execute_on_stream(&mut stream, cmd).await
    }

    /// Connect to a QMP socket and perform handshake.
    pub(crate) async fn connect_qmp(
        endpoint: &QmpEndpoint,
    ) -> Result<Box<dyn QmpStream + Unpin + Send>, Box<dyn std::error::Error + Send + Sync>> {
        let mut stream: Box<dyn QmpStream + Unpin + Send> = match endpoint {
            QmpEndpoint::Unix(socket_path) => Box::new(UnixStream::connect(socket_path).await?),
            QmpEndpoint::Tcp(addr) => Box::new(TcpStream::connect(addr).await?),
        };

        // Read greeting
        let mut buf = vec![0u8; 4096];
        let _ = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            tokio::io::AsyncReadExt::read(&mut stream, &mut buf),
        )
        .await??;

        // Send qmp_capabilities to enter command mode
        let caps_cmd = r#"{"execute": "qmp_capabilities"}"#;
        stream.write_all(caps_cmd.as_bytes()).await?;
        stream.write_all(b"\n").await?;

        // Read capability response
        let _ = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            tokio::io::AsyncReadExt::read(&mut stream, &mut buf),
        )
        .await??;

        Ok(stream)
    }

    /// Wait for a string to appear in the serial log.
    pub async fn wait_for_serial(&self, needle: &str, timeout_secs: f64) -> bool {
        let needle = strip_ansi(needle);
        let start = std::time::Instant::now();
        let timeout = std::time::Duration::from_secs_f64(timeout_secs);
        let mut last_print = std::time::Instant::now();

        loop {
            {
                let log = self.serial_log.lock().await;
                if last_print.elapsed() > std::time::Duration::from_secs(5) {
                    eprintln!(
                        "│  │  │      debug: waiting for {}. current log len: {}",
                        needle,
                        log.len()
                    );
                    let tail = if log.len() > DEBUG_TAIL_LENGTH {
                        &log[log.len() - DEBUG_TAIL_LENGTH..]
                    } else {
                        &log[..]
                    };
                    eprintln!("│  │  │      debug: tail: {:?}", tail);
                    last_print = std::time::Instant::now();
                }

                if strip_ansi(&log).to_lowercase().contains(&needle.to_lowercase()) {
                    eprintln!("│  │  │      debug: found {} in serial", needle);
                    return true;
                }
            }

            if start.elapsed() > timeout {
                eprintln!("│  │  │      debug: timed out waiting for {}", needle);
                return false;
            }

            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    }

    /// Wait for a regex pattern to appear in the serial log.
    ///
    /// Polls every 100 ms up to `timeout_secs`.  Returns `true` if the
    /// pattern is matched before the deadline, `false` otherwise.
    pub async fn wait_for_regex_pattern(&self, pattern: &str, timeout_secs: f64) -> bool {
        let re = match Regex::new(pattern) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("│  │  │      warn: invalid regex '{}': {}", pattern, e);
                return false;
            }
        };
        let start = std::time::Instant::now();
        let timeout = std::time::Duration::from_secs_f64(timeout_secs);
        let mut last_print = std::time::Instant::now();

        loop {
            {
                let log = self.serial_log.lock().await;
                let clean = strip_ansi(&log);
                if last_print.elapsed() > std::time::Duration::from_secs(5) {
                    eprintln!(
                        "│  │  │      debug: waiting for pattern /{}/ log len: {}",
                        pattern,
                        log.len()
                    );
                    let tail = if clean.len() > DEBUG_TAIL_LENGTH {
                        &clean[clean.len() - DEBUG_TAIL_LENGTH..]
                    } else {
                        &clean[..]
                    };
                    eprintln!("│  │  │      debug: tail: {:?}", tail);
                    last_print = std::time::Instant::now();
                }

                if re.is_match(&clean) {
                    eprintln!("│  │  │      debug: found pattern /{}/", pattern);
                    return true;
                }
            }

            if start.elapsed() > timeout {
                eprintln!("│  │  │      debug: timed out waiting for pattern /{}/", pattern);
                return false;
            }

            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    }

    /// Get the current serial log contents.
    pub async fn get_serial_log(&self) -> String {
        self.serial_log.lock().await.clone()
    }

    /// Write bytes to the serial console.
    pub async fn serial_write(
        &self,
        data: &[u8],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(ref tx) = self.serial_tx {
            tx.send(data.to_vec()).map_err(|_| "Serial channel closed")?;
            Ok(())
        } else {
            Err("Serial connection not established".into())
        }
    }

    /// Kill the QEMU process if running.
    pub async fn shutdown(&mut self) {
        // Wait a bit to ensure any pending screenshots/logs are captured
        // The user specifically requested to keep QEMU open long enough.
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        if let Some(ref mut child) = self.qemu {
            let _ = child.kill().await;
        }

        // Clean up global QMP stream before removing socket file (although kill closes it)
        crate::artifacts::set_qmp_stream(None).await;

        // Clean up QMP socket
        if let Some(ref socket_path) = self.qmp_socket {
            let _ = std::fs::remove_file(socket_path);
        }
    }
}

// ===== Bag Matcher and Diagnostics Support =====

/// Check if runtime diagnostics mode is enabled.
pub fn diag_enabled() -> bool {
    std::env::var("THINGOS_DIAG").is_ok()
}

/// Maximum number of characters to show in debug tail output when polling.
const DEBUG_TAIL_LENGTH: usize = 200;

/// Required boot signals that must ALL appear (order irrelevant).
/// Each entry is a list of substrings that must all be present.
/// Keep these level-agnostic so INFO/DEBUG demotions don't break readiness checks.
pub const REQUIRED_BOOT_SIGNALS: &[&[&str]] = &[
    // Kernel start marker
    &["thing-os kernel"],
    // SIMD init (replaces legacy paging boundary signal)
    &["Initializing SIMD"],
    // Memory map / allocator
    &["Frame allocator initialized"],
    &["Initializing global allocator"],
    // Tasking bring-up
    &["Initializing tasking"],
    &["Scheduler initialized"],
    &["Entering scheduler loop"],
];

/// Liveness signals - at least one of these must appear.
pub const LIVENESS_SIGNALS: &[&str] =
    &["A: tick", "B: tick", "threads_demo", "BOOT: heartbeat", "BOOT: ready"];

impl ThingOsWorld {
    /// Wait until all required signals are found in the log (unordered).
    /// Returns Ok(()) if all found within timeout, Err with missing signals otherwise.
    pub async fn wait_for_all_signals(
        &self,
        required: &[&[&str]],
        timeout_secs: f64,
    ) -> Result<(), Vec<String>> {
        let start = std::time::Instant::now();
        let timeout = std::time::Duration::from_secs_f64(timeout_secs);

        loop {
            let log = self.serial_log.lock().await;
            let missing: Vec<String> = required
                .iter()
                .filter(|alts| {
                    !alts.iter().all(|sig| {
                        let clean_sig = strip_ansi(sig);
                        log.contains(&clean_sig)
                    })
                })
                .map(|alts| alts.join(" AND "))
                .collect();

            if missing.is_empty() {
                return Ok(());
            }

            drop(log);

            if start.elapsed() > timeout {
                return Err(missing);
            }

            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    }

    /// Check if any liveness signal is present.
    pub async fn has_liveness_signal(&self) -> bool {
        let log = self.serial_log.lock().await;
        LIVENESS_SIGNALS.iter().any(|sig| {
            let clean_sig = strip_ansi(sig);
            log.contains(&clean_sig)
        })
    }

    /// Wait for any liveness signal within timeout.
    pub async fn wait_for_liveness(&self, timeout_secs: f64) -> bool {
        let start = std::time::Instant::now();
        let timeout = std::time::Duration::from_secs_f64(timeout_secs);

        loop {
            if self.has_liveness_signal().await {
                return true;
            }

            if start.elapsed() > timeout {
                return false;
            }

            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    }

    /// Create a small MBR-partitioned FAT16 disk image at `path`.
    ///
    /// The image contains a single FAT partition and a `hello.txt` file with
    /// the text "hello from USB\n".  Uses Linux utilities (`mkdosfs`,
    /// `mtools`) when available; falls back to writing a pre-built raw
    /// image when they are not.
    fn create_usb_fat_image(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        // Image size: 16 MiB
        const IMG_SIZE: u64 = 16 * 1024 * 1024;
        // Partition starts at LBA 2048 (1 MiB alignment)
        const PART_START_LBA: u64 = 2048;
        const SECTOR_SIZE: u64 = 512;

        // Try using mkdosfs + mtools if available
        if std::process::Command::new("mkdosfs").arg("--help").output().is_ok() {
            if let Ok(()) =
                Self::create_usb_fat_image_with_tools(path, IMG_SIZE, PART_START_LBA)
            {
                return Ok(());
            }
        }
        // Fall back to a manually built minimal FAT16 image
        Self::create_usb_fat_image_minimal(path, IMG_SIZE, PART_START_LBA, SECTOR_SIZE)
    }

    /// Create a USB FAT disk image using standard Linux disk utilities.
    fn create_usb_fat_image_with_tools(
        path: &Path,
        img_size: u64,
        part_start_lba: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        const SECTOR_SIZE: u64 = 512;
        let part_start_bytes = part_start_lba * SECTOR_SIZE;
        let part_size_bytes = img_size - part_start_bytes;

        // Create sparse image file
        let out = std::process::Command::new("dd")
            .args([
                "if=/dev/zero",
                &format!("of={}", path.display()),
                "bs=512",
                "count=0",
                &format!("seek={}", img_size / SECTOR_SIZE),
            ])
            .output()?;
        if !out.status.success() {
            return Err(
                format!("dd failed: {}", String::from_utf8_lossy(&out.stderr)).into(),
            );
        }

        // Write MBR with one partition entry
        {
            use std::io::{Seek, Write};
            let mut f = std::fs::OpenOptions::new().read(true).write(true).open(path)?;
            let mut mbr = [0u8; 512];
            let p = &mut mbr[446..462];
            p[4] = 0x06; // FAT16 >= 32 MiB
            // CHS start/end: set to maximum (0xFE/0xFF) for LBA-only addressing
            p[1] = 0xFE; p[2] = 0xFF; p[3] = 0xFF;
            p[5] = 0xFE; p[6] = 0xFF; p[7] = 0xFF;
            let start = part_start_lba as u32;
            let size = (part_size_bytes / SECTOR_SIZE) as u32;
            p[8..12].copy_from_slice(&start.to_le_bytes());
            p[12..16].copy_from_slice(&size.to_le_bytes());
            mbr[510] = 0x55;
            mbr[511] = 0xAA;
            f.seek(std::io::SeekFrom::Start(0))?;
            f.write_all(&mbr)?;
        }

        // Format partition
        let offset_arg = format!("--offset={}", part_start_lba);
        let mkdosfs_out = std::process::Command::new("mkdosfs")
            .args(["-F", "16", "-n", "USBVOL", &offset_arg, &path.to_string_lossy()])
            .output()?;
        if !mkdosfs_out.status.success() {
            return Err(format!(
                "mkdosfs failed: {}",
                String::from_utf8_lossy(&mkdosfs_out.stderr)
            )
            .into());
        }

        // Write hello.txt via mcopy
        let drive_spec = format!("-i{}@@{}", path.to_string_lossy(), part_start_lba * 512);
        let mut child = std::process::Command::new("mcopy")
            .args([&drive_spec, "-", "::hello.txt"])
            .stdin(std::process::Stdio::piped())
            .spawn()?;
        {
            use std::io::Write;
            child.stdin.as_mut().unwrap().write_all(b"hello from USB\n")?;
        }
        let mcopy_out = child.wait_with_output()?;
        if !mcopy_out.status.success() {
            eprintln!(
                "[bdd] mcopy warning: {}",
                String::from_utf8_lossy(&mcopy_out.stderr)
            );
        }

        Ok(())
    }

    /// Build a minimal FAT16 disk image in pure Rust without external tools.
    fn create_usb_fat_image_minimal(
        path: &Path,
        img_size: u64,
        part_start_lba: u64,
        sector_size: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use std::io::Write;

        const SECTORS_PER_CLUSTER: u32 = 4;
        const RESERVED_SECTORS: u32 = 4;
        const NUM_FATS: u32 = 2;
        const ROOT_ENTRY_COUNT: u32 = 512;
        const MEDIA_BYTE: u8 = 0xF8;

        let total_lba = (img_size / sector_size) as u32;
        let part_sectors = total_lba - part_start_lba as u32;
        let root_dir_sectors =
            (ROOT_ENTRY_COUNT * 32 + sector_size as u32 - 1) / sector_size as u32;
        // Rough FAT size estimate
        let fat_size: u32 = {
            let data_sectors = part_sectors
                .saturating_sub(RESERVED_SECTORS + NUM_FATS * 8 + root_dir_sectors);
            let n_clusters = data_sectors / SECTORS_PER_CLUSTER;
            (n_clusters * 2 + sector_size as u32 - 1) / sector_size as u32
        };
        let data_start_lba =
            RESERVED_SECTORS + NUM_FATS * fat_size + root_dir_sectors;

        let mut img = vec![0u8; img_size as usize];

        // MBR
        {
            let p = &mut img[446..462];
            p[4] = 0x06; // FAT16 >= 32 MiB partition type
            // CHS start/end: set to maximum (0xFE/0xFF) for LBA-only addressing
            p[1] = 0xFE; p[2] = 0xFF; p[3] = 0xFF;
            p[5] = 0xFE; p[6] = 0xFF; p[7] = 0xFF;
            p[8..12].copy_from_slice(&(part_start_lba as u32).to_le_bytes());
            p[12..16].copy_from_slice(&part_sectors.to_le_bytes());
        }
        img[510] = 0x55;
        img[511] = 0xAA;

        // Boot sector
        let bs = part_start_lba as usize * sector_size as usize;
        {
            let s = &mut img[bs..bs + sector_size as usize];
            s[0] = 0xEB; s[1] = 0x58; s[2] = 0x90;
            s[3..11].copy_from_slice(b"MSWIN4.1");
            s[11..13].copy_from_slice(&(sector_size as u16).to_le_bytes());
            s[13] = SECTORS_PER_CLUSTER as u8;
            s[14..16].copy_from_slice(&(RESERVED_SECTORS as u16).to_le_bytes());
            s[16] = NUM_FATS as u8;
            s[17..19].copy_from_slice(&(ROOT_ENTRY_COUNT as u16).to_le_bytes());
            let ts16: u16 = if part_sectors < 0x10000 { part_sectors as u16 } else { 0 };
            s[19..21].copy_from_slice(&ts16.to_le_bytes());
            s[21] = MEDIA_BYTE;
            s[22..24].copy_from_slice(&(fat_size as u16).to_le_bytes());
            s[24..26].copy_from_slice(&63u16.to_le_bytes()); // sectors-per-track (legacy CHS)
            s[26..28].copy_from_slice(&255u16.to_le_bytes()); // number of heads (legacy CHS)
            s[28..32].copy_from_slice(&(part_start_lba as u32).to_le_bytes());
            if ts16 == 0 {
                s[32..36].copy_from_slice(&part_sectors.to_le_bytes());
            }
            s[36] = 0x80; // drive number: first hard disk
            s[38] = 0x29; // extended boot record signature
            // Volume serial number: fixed value (arbitrary, non-zero)
            s[39..43].copy_from_slice(&0xDEAD_BEEFu32.to_le_bytes());
            s[43..54].copy_from_slice(b"USBVOL     ");
            s[54..62].copy_from_slice(b"FAT16   ");
            s[510] = 0x55;
            s[511] = 0xAA;
        }

        // FAT1
        let fat1 = (part_start_lba as u32 + RESERVED_SECTORS) as usize * sector_size as usize;
        img[fat1] = MEDIA_BYTE; img[fat1 + 1] = 0xFF;
        img[fat1 + 2] = 0xFF; img[fat1 + 3] = 0xFF;
        // Cluster 2: end-of-chain
        img[fat1 + 4] = 0xFF; img[fat1 + 5] = 0xFF;

        // FAT2 mirror
        let fat2 = fat1 + fat_size as usize * sector_size as usize;
        img.copy_within(fat1..fat1 + fat_size as usize * sector_size as usize, fat2);

        // Root directory
        let root =
            (part_start_lba as u32 + RESERVED_SECTORS + NUM_FATS * fat_size) as usize
                * sector_size as usize;
        // Volume label entry
        img[root..root + 11].copy_from_slice(b"USBVOL     ");
        img[root + 11] = 0x08;
        // hello.txt entry at +32
        let e = root + 32;
        img[e..e + 8].copy_from_slice(b"HELLO   ");
        img[e + 8..e + 11].copy_from_slice(b"TXT");
        img[e + 11] = 0x20;
        let content = b"hello from USB\n";
        img[e + 26] = 2; // first cluster low
        img[e + 27] = 0;
        let fsz = content.len() as u32;
        img[e + 28..e + 32].copy_from_slice(&fsz.to_le_bytes());

        // File data at cluster 2
        let data = (part_start_lba as u32 + data_start_lba) as usize * sector_size as usize;
        img[data..data + content.len()].copy_from_slice(content);

        let mut f = std::fs::File::create(path)?;
        f.write_all(&img)?;
        Ok(())
    }
}

/// Strip ANSI escape sequences and carriage returns from a string.
pub fn strip_ansi(s: &str) -> String {
    use std::sync::OnceLock;
    static ANSI_RE: OnceLock<Regex> = OnceLock::new();
    let re = ANSI_RE.get_or_init(|| {
        Regex::new(r"[\u001b\u009b][\[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)*[0-9A-Za-z=><~]")
            .expect("Invalid ANSI regex")
    });
    re.replace_all(s, "").replace('\r', "")
}

#[cfg(test)]
mod tests {
    use super::ThingOsWorld;

    #[test]
    fn parses_supported_timeout_tags() {
        assert_eq!(ThingOsWorld::parse_timeout_tag("timeout.30s"), Some(30.0));
        assert_eq!(ThingOsWorld::parse_timeout_tag("timeout=45"), Some(45.0));
        assert_eq!(ThingOsWorld::parse_timeout_tag("@timeout-7.5s"), Some(7.5));
    }

    #[test]
    fn rejects_invalid_timeout_tags() {
        assert_eq!(ThingOsWorld::parse_timeout_tag("smoke"), None);
        assert_eq!(ThingOsWorld::parse_timeout_tag("timeout.0s"), None);
        assert_eq!(ThingOsWorld::parse_timeout_tag("timeout.bad"), None);
    }

    /// Smoke-test the minimal FAT image builder: write the image to a temp
    /// file, parse the MBR and the FAT16 boot sector, and verify the
    /// `hello.txt` directory entry is present.
    #[test]
    fn create_usb_fat_image_minimal_produces_valid_fat16() {
        let dir = std::env::temp_dir().join("fat_image_test");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("usb.img");

        ThingOsWorld::create_usb_fat_image_minimal(
            &path,
            16 * 1024 * 1024, // 16 MiB
            2048,              // partition LBA
            512,               // sector size
        )
        .expect("create_usb_fat_image_minimal should succeed");

        let img = std::fs::read(&path).expect("image file should exist");
        assert_eq!(img.len(), 16 * 1024 * 1024);

        // MBR signature
        assert_eq!(img[510], 0x55);
        assert_eq!(img[511], 0xAA);

        // Partition entry 0 should point to LBA 2048
        let start_lba =
            u32::from_le_bytes(img[446 + 8..446 + 12].try_into().unwrap());
        assert_eq!(start_lba, 2048);

        // Boot sector of the partition
        let bs = 2048 * 512;
        assert_eq!(img[bs + 510], 0x55);
        assert_eq!(img[bs + 511], 0xAA);

        // bytes_per_sector == 512
        let bps = u16::from_le_bytes([img[bs + 11], img[bs + 12]]);
        assert_eq!(bps, 512);

        // FAT type string
        assert_eq!(&img[bs + 54..bs + 62], b"FAT16   ");

        // Find "HELLO   TXT" in the root directory (somewhere after the FATs)
        let reserved = u16::from_le_bytes([img[bs + 14], img[bs + 15]]) as usize;
        let num_fats = img[bs + 16] as usize;
        let fat_size = u16::from_le_bytes([img[bs + 22], img[bs + 23]]) as usize;
        let root_off = (2048 + reserved + num_fats * fat_size) * 512;
        let found_hello = (0..512 / 32).any(|i| {
            let e = &img[root_off + i * 32..root_off + (i + 1) * 32];
            &e[0..8] == b"HELLO   " && &e[8..11] == b"TXT"
        });
        assert!(found_hello, "HELLO.TXT entry not found in root directory");

        // File content
        let root_entry_count =
            u16::from_le_bytes([img[bs + 17], img[bs + 18]]) as usize;
        let root_dir_sectors_count = root_entry_count * 32 / 512;
        let cluster2_sector =
            2048 + reserved + num_fats * fat_size + root_dir_sectors_count;
        let file_data_off = cluster2_sector * 512;
        assert_eq!(&img[file_data_off..file_data_off + 15], b"hello from USB\n");
    }
}
