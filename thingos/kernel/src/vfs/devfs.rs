//! devfs — kernel-native device filesystem mounted at `/dev`.
//!
//! Provides a minimal set of built-in device nodes plus a **runtime
//! registration** mechanism so that kernel subsystems and drivers can add
//! new device entries without modifying this file.
//!
//! # Built-in nodes
//!
//! | Path            | Kind     | Description                              |
//! |-----------------|----------|------------------------------------------|
//! | `/dev/console`  | char     | Writes go to the boot console; reads from the per-process console input queue |
//! | `/dev/null`     | char     | Discards writes; returns EOF on reads    |
//! | `/dev/zero`     | char     | Returns zero bytes; discards writes      |
//!
//! # Extensibility
//! Additional device nodes can be registered at runtime via the global
//! [`register`] function:
//!
//! ```ignore
//! devfs::register("ttyS0", Arc::new(my_uart_node));
//! ```
//!
//! Registered nodes are consulted **before** the built-in match, so they can
//! shadow built-in names when needed (last registration wins).  The global
//! registry is protected by a spin-lock.

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;
use alloc::{format, vec};

use abi::errors::{Errno, SysResult};
use spin::Mutex;

use super::{VfsDriver, VfsNode, VfsStat};

// ── Global device registry ────────────────────────────────────────────────────

/// Global table of dynamically registered `/dev` entries.
///
/// Keys are bare device names (no leading `/dev/`).  The table is consulted
/// *after* the built-in match, so built-in names (`null`, `zero`, `console`)
/// can still be overridden if needed.
static DEVICE_REGISTRY: Mutex<BTreeMap<String, Arc<dyn VfsNode>>> = Mutex::new(BTreeMap::new());
static BOOT_FB_INFO: Mutex<Option<(crate::FramebufferInfo, u64)>> = Mutex::new(None);
static KERNEL_CMDLINE: Mutex<Option<String>> = Mutex::new(None);

/// Register a device node under the name `name` in `/dev`.
///
/// The `name` must be the bare device name, e.g. `"ttyS0"` (not `/dev/ttyS0`).
/// If a node with the same name was previously registered, it is replaced.
///
/// # Example
/// ```ignore
/// use kernel::vfs::devfs;
/// devfs::register("ttyS0", Arc::new(UartNode::new()));
/// ```
pub fn register(name: &str, node: Arc<dyn VfsNode>) {
    DEVICE_REGISTRY.lock().insert(name.to_string(), node);
}

/// Remove a previously registered device node.
///
/// Returns `true` if a node was found and removed, `false` if the name was
/// not registered.
pub fn unregister(name: &str) -> bool {
    DEVICE_REGISTRY.lock().remove(name).is_some()
}

pub fn set_boot_fb(fb: crate::FramebufferInfo, resource_id: u64) {
    crate::kinfo!(
        "devfs: set_boot_fb width={} height={} pitch={} resource_id=0x{:x}",
        fb.width,
        fb.height,
        fb.pitch,
        resource_id
    );
    *BOOT_FB_INFO.lock() = Some((fb, resource_id));
}

pub fn set_cmdline(cmdline: String) {
    *KERNEL_CMDLINE.lock() = Some(cmdline);
}

// ── DevFs driver ─────────────────────────────────────────────────────────────

/// The device filesystem driver.  Mounted at `/dev` by `vfs::init`.
pub struct DevFs;

impl DevFs {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DevFs {
    fn default() -> Self {
        Self::new()
    }
}

impl VfsDriver for DevFs {
    fn lookup(&self, path: &str) -> SysResult<Arc<dyn VfsNode>> {
        if path == "fb0" || path.starts_with("fb") {
            crate::ktrace!("devfs: lookup entry path='{}' len={}", path, path.len());
        }
        // Empty path → the /dev directory node itself.
        if path.is_empty() {
            return Ok(Arc::new(DevDirNode));
        }

        // Check the dynamic registry first; registered nodes take precedence
        // over built-in names, allowing callers to override defaults.
        {
            let reg = DEVICE_REGISTRY.lock();
            if let Some(node) = reg.get(path) {
                if path == "fb0" || path.starts_with("fb") {
                    crate::kinfo!("devfs: dynamic registry hit path='{}'", path);
                }
                return Ok(node.clone());
            }
        }

        // Handle synthetic subdirectories
        match path {
            "display" => return Ok(Arc::new(DevSubDirNode::new("display/"))),
            "input" => return Ok(Arc::new(DevSubDirNode::new("input/"))),
            "audio" => return Ok(Arc::new(DevSubDirNode::new("audio/"))),
            "net" => return Ok(Arc::new(DevSubDirNode::new("net/"))),
            _ => {}
        }

        // Fall back to built-in nodes.
        match path {
            "console" => Ok(Arc::new(ConsoleNode)),
            "null" => Ok(Arc::new(NullNode)),
            "zero" => Ok(Arc::new(ZeroNode)),
            "fb0" => {
                if let Some((fb, resource_id)) = *BOOT_FB_INFO.lock() {
                    crate::kinfo!(
                        "devfs: lookup fb0 -> hit ({}x{} stride={})",
                        fb.width,
                        fb.height,
                        fb.pitch
                    );
                    Ok(Arc::new(FbNode::new(fb, resource_id)))
                } else {
                    crate::kwarn!("devfs: lookup fb0 -> missing boot fb state");
                    Err(Errno::ENOENT)
                }
            }
            "rtc" => Ok(Arc::new(RtcNode)),
            "random" => Ok(Arc::new(RandomNode)),
            "urandom" => Ok(Arc::new(UrandomNode)),
            "kmsg" => Ok(Arc::new(KmsgNode)),
            "cmdline" => Ok(Arc::new(CmdlineNode)),
            "tty0" => Ok(Arc::new(FbTerminalNode)),
            _ => Err(Errno::ENOENT),
        }
    }
}

// ── Synthetic subdirectory node ──────────────────────────────────────────────

struct DevSubDirNode {
    prefix: String,
}

impl DevSubDirNode {
    fn new(prefix: &str) -> Self {
        Self { prefix: prefix.to_string() }
    }
}

impl VfsNode for DevSubDirNode {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFDIR | 0o755,
            size: 0,
            ino: 101, // arbitrary
            nlink: 2,
            ..Default::default()
        })
    }
    fn readdir(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let mut names = Vec::new();
        {
            let reg = DEVICE_REGISTRY.lock();
            for name in reg.keys() {
                if name.starts_with(&self.prefix) {
                    let subname = &name[self.prefix.len()..];
                    if !subname.is_empty() && !subname.contains('/') {
                        names.push(subname.to_string());
                    }
                }
            }
        }
        super::write_readdir_entries(names.iter().map(|s| s.as_str()), offset, buf)
    }
}

// ── /dev directory node ───────────────────────────────────────────────────────

/// Directory node for `/dev` itself.
struct DevDirNode;

impl VfsNode for DevDirNode {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EISDIR)
    }
    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFDIR | 0o755,
            size: 0,
            ino: 100,
            nlink: 2,
            ..Default::default()
        })
    }
    fn readdir(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let mut names = alloc::vec!["console".to_string(), "null".to_string(), "zero".to_string()];
        if BOOT_FB_INFO.lock().is_some() {
            names.push("fb0".to_string());
        }
        names.push("display".to_string());
        names.push("input".to_string());
        names.push("audio".to_string());
        names.push("net".to_string());
        names.push("rtc".to_string());
        names.push("random".to_string());
        names.push("urandom".to_string());
        names.push("kmsg".to_string());
        names.push("tty0".to_string());
        {
            let reg = DEVICE_REGISTRY.lock();
            for name in reg.keys() {
                if !matches!(
                    name.as_str(),
                    "console" | "null" | "zero" | "fb0" | "rtc" | "random" | "urandom"
                ) {
                    names.push(name.clone());
                }
            }
        }
        super::write_readdir_entries(names.iter().map(|s: &String| s.as_str()), offset, buf)
    }
}

use spin::Once;
/// Global tty line discipline for `/dev/console`.
static CONSOLE_LD: Once<Arc<crate::vfs::tty::LineDiscipline>> = Once::new();

fn get_console_ld() -> Arc<crate::vfs::tty::LineDiscipline> {
    CONSOLE_LD
        .call_once(|| {
            Arc::new(crate::vfs::tty::LineDiscipline::with_presence(
                crate::presence::get_console_presence_state(),
            ))
        })
        .clone()
}

/// Global tty line discipline for `/dev/tty0` (framebuffer terminal).
static FB_TTY_LD: Once<Arc<crate::vfs::tty::LineDiscipline>> = Once::new();

fn get_fb_tty_ld() -> Arc<crate::vfs::tty::LineDiscipline> {
    FB_TTY_LD.call_once(|| Arc::new(crate::vfs::tty::LineDiscipline::new())).clone()
}

struct SerialHardware;
impl crate::vfs::tty::TtyHardware for SerialHardware {
    fn read_byte(&self) -> Option<u8> {
        crate::runtime_base().getchar()
    }
    fn write_byte(&self, byte: u8) {
        crate::runtime_base().serial_putchar(byte)
    }
    fn write_buf(&self, buf: &[u8]) {
        crate::logging::write_serial_bytes_locked(buf);
    }
    fn winsize(&self) -> abi::termios::Winsize {
        derive_winsize_from_bootfb()
    }
}

struct FbHardware;
impl crate::vfs::tty::TtyHardware for FbHardware {
    fn read_byte(&self) -> Option<u8> {
        crate::irq::ps2::take_input_char()
    }
    fn write_byte(&self, byte: u8) {
        crate::runtime_base().fb_putchar(byte);
    }
    fn write_buf(&self, buf: &[u8]) {
        crate::logging::write_fb_bytes_locked(buf);
    }
    fn winsize(&self) -> abi::termios::Winsize {
        derive_winsize_from_bootfb()
    }
}

fn derive_winsize_from_bootfb() -> abi::termios::Winsize {
    if let Some((fb, _)) = *BOOT_FB_INFO.lock() {
        if fb.width > 0 && fb.height > 0 {
            const CELL_WIDTH_PX: u32 = 8;
            const CELL_HEIGHT_PX: u32 = 16;
            let clamp_u16 = |value: u32| value.min(u16::MAX as u32) as u16;
            let clamp_tty_cells = |value: u32| value.max(1).min(u16::MAX as u32) as u16;
            return abi::termios::Winsize {
                ws_row: clamp_tty_cells(fb.height / CELL_HEIGHT_PX),
                ws_col: clamp_tty_cells(fb.width / CELL_WIDTH_PX),
                ws_xpixel: clamp_u16(fb.width),
                ws_ypixel: clamp_u16(fb.height),
            };
        }
    }
    abi::termios::Winsize::default()
}

pub(crate) fn console_foreground_pgid() -> Option<u32> {
    get_console_ld().presence.lock().foreground_pgid
}

/// Character device node for `/dev/console`.
///
/// - **write**: each byte is forwarded to the kernel's boot console via
///   [`crate::runtime_base()`].
/// - **read**: reads from the boot console, honouring the current termios
///   settings (canonical vs. raw mode, echo, ISIG, etc.).  Blocks (yields)
///   until data is available, and returns `EINTR` if a pending interrupt is
///   detected or if Ctrl-C is received while `ISIG` is set.
/// - **device_call**: supports termios settings, process-group ioctls, and
///   `TIOCGWINSZ` geometry queries.
pub struct ConsoleNode;

impl ConsoleNode {
    pub fn handle_runtime_input_byte<R: crate::BootRuntimeBase + ?Sized>(_rt: &R, c: u8) -> bool {
        get_console_ld().drain_input(&SerialHardware)
    }

    pub fn poll_input() {
        get_console_ld().drain_input(&SerialHardware);
    }

    pub fn get_termios() -> abi::termios::Termios {
        *get_console_ld().termios.lock()
    }

    pub fn set_termios(t: abi::termios::Termios) {
        *get_console_ld().termios.lock() = t;
    }
}

impl VfsNode for ConsoleNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let tty = crate::vfs::tty::TtyNode { hw: Arc::new(SerialHardware), ld: get_console_ld() };
        tty.read(offset, buf)
    }

    fn write(&self, offset: u64, buf: &[u8]) -> SysResult<usize> {
        let tty = crate::vfs::tty::TtyNode { hw: Arc::new(SerialHardware), ld: get_console_ld() };
        tty.write(offset, buf)
    }

    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFCHR | 0o666,
            size: 0,
            ino: 1,
            nlink: 1,
            rdev: VfsStat::makedev(5, 1),
            ..Default::default()
        })
    }

    fn is_tty(&self) -> bool {
        true
    }

    fn device_call(&self, call: &abi::device::DeviceCall) -> SysResult<usize> {
        let tty = crate::vfs::tty::TtyNode { hw: Arc::new(SerialHardware), ld: get_console_ld() };
        tty.device_call(call)
    }
}

pub struct FbTerminalNode;

impl VfsNode for FbTerminalNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let tty = crate::vfs::tty::TtyNode { hw: Arc::new(FbHardware), ld: get_fb_tty_ld() };
        tty.read(offset, buf)
    }

    fn write(&self, offset: u64, buf: &[u8]) -> SysResult<usize> {
        let tty = crate::vfs::tty::TtyNode { hw: Arc::new(FbHardware), ld: get_fb_tty_ld() };
        tty.write(offset, buf)
    }

    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFCHR | 0o666,
            size: 0,
            ino: 10,
            nlink: 1,
            rdev: VfsStat::makedev(4, 0),
            ..Default::default()
        })
    }

    fn is_tty(&self) -> bool {
        true
    }

    fn device_call(&self, call: &abi::device::DeviceCall) -> SysResult<usize> {
        let tty = crate::vfs::tty::TtyNode { hw: Arc::new(FbHardware), ld: get_fb_tty_ld() };
        tty.device_call(call)
    }
}

// ── /dev/null ────────────────────────────────────────────────────────────────

/// Character device node for `/dev/null`.
/// Reads return EOF immediately; writes silently succeed.
pub struct NullNode;

impl VfsNode for NullNode {
    fn read(&self, _offset: u64, _buf: &mut [u8]) -> SysResult<usize> {
        Ok(0) // EOF
    }

    fn write(&self, _offset: u64, buf: &[u8]) -> SysResult<usize> {
        Ok(buf.len()) // silently discard
    }

    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFCHR | 0o666,
            size: 0,
            ino: 2,
            nlink: 1,
            rdev: VfsStat::makedev(1, 3),
            ..Default::default()
        })
    }
}

// ── CmdlineNode ─────────────────────────────────────────────────────────────

struct CmdlineNode;

impl VfsNode for CmdlineNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        let cmdline = KERNEL_CMDLINE.lock();
        if let Some(cmdline) = cmdline.as_ref() {
            crate::ktrace!("devfs: cmdline read offset={} len={}", offset, buf.len());
            let bytes = cmdline.as_bytes();
            if offset >= bytes.len() as u64 {
                return Ok(0);
            }
            let n = core::cmp::min(buf.len(), bytes.len() - offset as usize);
            buf[..n].copy_from_slice(&bytes[offset as usize..offset as usize + n]);
            Ok(n)
        } else {
            Ok(0)
        }
    }

    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EPERM)
    }

    fn stat(&self) -> SysResult<VfsStat> {
        let size = KERNEL_CMDLINE.lock().as_ref().map(|s| s.len()).unwrap_or(0);
        Ok(VfsStat {
            mode: VfsStat::S_IFCHR | 0o444,
            size: size as u64,
            ino: 102, // arbitrary
            nlink: 1,
            uid: 0,
            gid: 0,
            rdev: VfsStat::makedev(1, 10), // arbitrary major/minor
            ..Default::default()
        })
    }
}

// ── /dev/zero ────────────────────────────────────────────────────────────────

/// Character device node for `/dev/zero`.
/// Reads fill the buffer with zero bytes; writes succeed silently.
pub struct ZeroNode;

impl VfsNode for ZeroNode {
    fn read(&self, _offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        buf.fill(0);
        Ok(buf.len())
    }

    fn write(&self, _offset: u64, buf: &[u8]) -> SysResult<usize> {
        Ok(buf.len())
    }

    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFCHR | 0o666,
            size: 0,
            ino: 3,
            nlink: 1,
            rdev: VfsStat::makedev(1, 5),
            ..Default::default()
        })
    }
}

// ── /dev/fb0 ─────────────────────────────────────────────────────────────────

#[repr(C)]
pub struct FbNode {
    fb: crate::FramebufferInfo,
    resource_id: u64,
    shadow: Mutex<FbShadow>,
}

struct FbShadow {
    bytes: Vec<u8>,
}

impl FbNode {
    pub fn new(fb: crate::FramebufferInfo, resource_id: u64) -> Self {
        Self { fb, resource_id, shadow: Mutex::new(FbShadow::new(fb)) }
    }
}

impl FbShadow {
    fn new(fb: crate::FramebufferInfo) -> Self {
        let len = fb.byte_len as usize;
        let mut bytes = vec![0u8; len];
        if len > 0 {
            unsafe {
                core::ptr::copy_nonoverlapping(fb.addr as *const u8, bytes.as_mut_ptr(), len);
            }
        }
        Self { bytes }
    }
}

impl VfsNode for FbNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        use abi::display_driver_protocol::{FB_INFO_PAYLOAD_SIZE, FbInfoPayload};

        let payload = FbInfoPayload {
            device_handle: self.resource_id,
            width: self.fb.width,
            height: self.fb.height,
            stride: self.fb.pitch,
            bpp: self.fb.bpp as u32,
            format: self.fb.format as u32,
            _reserved: 0,
        };

        let slice = unsafe {
            core::slice::from_raw_parts(&payload as *const _ as *const u8, FB_INFO_PAYLOAD_SIZE)
        };

        let off = offset as usize;
        if off >= slice.len() {
            crate::kwarn!("FbNode::read: EOF (offset={} >= slice.len={})", off, slice.len());
            return Ok(0);
        }

        let avail = &slice[off..];
        let n = avail.len().min(buf.len());
        crate::kinfo!(
            "FbNode::read: off={} n={} buf_len={} total={}",
            off,
            n,
            buf.len(),
            slice.len()
        );
        buf[..n].copy_from_slice(&avail[..n]);
        Ok(n)
    }

    fn write(&self, offset: u64, buf: &[u8]) -> SysResult<usize> {
        use abi::display_driver_protocol::FB_INFO_PAYLOAD_SIZE;

        let raw_off = offset as usize;
        // /dev/fb0 read exposes a small metadata payload at the start of the
        // file, while write historically treated offset 0 as framebuffer byte 0.
        // Accept both conventions so a caller can read metadata then write on
        // the same fd without an explicit seek.
        let off = if raw_off >= FB_INFO_PAYLOAD_SIZE
            && ((raw_off - FB_INFO_PAYLOAD_SIZE) as u64) < self.fb.byte_len
        {
            raw_off - FB_INFO_PAYLOAD_SIZE
        } else {
            raw_off
        };

        if off as u64 >= self.fb.byte_len {
            return Ok(0);
        }

        let n = buf.len().min((self.fb.byte_len.saturating_sub(off as u64)) as usize);
        if n == 0 {
            return Ok(0);
        }

        let mut shadow = self.shadow.lock();
        shadow.bytes[off..off + n].copy_from_slice(&buf[..n]);

        let expected_frame_bytes = (self.fb.height as usize) * (self.fb.pitch as usize);

        // Full-frame writes are staged first, then published in one pass so
        // the boot framebuffer does not expose the wallpaper as it streams in.
        if off == 0 && (n as u64 == self.fb.byte_len || n == expected_frame_bytes) {
            let row_bytes = self.fb.pitch as usize;
            let bytes_per_pixel = ((self.fb.bpp as usize) + 7) / 8;
            let payload_bytes =
                (self.fb.width as usize).saturating_mul(bytes_per_pixel).min(row_bytes);

            // Fast blitting: cast pointers to u64/u32 to force wide MMIO transactions.
            // Generic [u8] copies often fall back to 1-byte writes on uncacheable memory,
            // bypassing PCIe Write Combining and causing multi-second screen freezes.
            if bytes_per_pixel == 4
                && row_bytes % 8 == 0
                && (self.fb.addr as usize) % 8 == 0
                && n % 8 == 0
            {
                let dst_u64 =
                    unsafe { core::slice::from_raw_parts_mut(self.fb.addr as *mut u64, n / 8) };
                let src_u64 = unsafe {
                    core::slice::from_raw_parts(shadow.bytes.as_ptr() as *const u64, n / 8)
                };

                if row_bytes == payload_bytes {
                    dst_u64.copy_from_slice(src_u64);
                } else {
                    let row_u64 = row_bytes / 8;
                    let payload_u64 = payload_bytes / 8;
                    for y in 0..(self.fb.height as usize) {
                        let start = y * row_u64;
                        if start + payload_u64 > n / 8 {
                            break;
                        }
                        dst_u64[start..start + payload_u64]
                            .copy_from_slice(&src_u64[start..start + payload_u64]);
                    }
                }
                return Ok(n);
            } else if bytes_per_pixel == 4
                && row_bytes % 4 == 0
                && (self.fb.addr as usize) % 4 == 0
                && n % 4 == 0
            {
                let dst_u32 =
                    unsafe { core::slice::from_raw_parts_mut(self.fb.addr as *mut u32, n / 4) };
                let src_u32 = unsafe {
                    core::slice::from_raw_parts(shadow.bytes.as_ptr() as *const u32, n / 4)
                };

                if row_bytes == payload_bytes {
                    dst_u32.copy_from_slice(src_u32);
                } else {
                    let row_u32 = row_bytes / 4;
                    let payload_u32 = payload_bytes / 4;
                    for y in 0..(self.fb.height as usize) {
                        let start = y * row_u32;
                        if start + payload_u32 > n / 4 {
                            break;
                        }
                        dst_u32[start..start + payload_u32]
                            .copy_from_slice(&src_u32[start..start + payload_u32]);
                    }
                }
                return Ok(n);
            }

            // Fallback for non-32bpp or unaligned framebuffers
            let dst_slice = unsafe { core::slice::from_raw_parts_mut(self.fb.addr as *mut u8, n) };

            if row_bytes == payload_bytes {
                dst_slice.copy_from_slice(&shadow.bytes[..n]);
            } else {
                for y in 0..(self.fb.height as usize) {
                    let start = y * row_bytes;
                    if start + payload_bytes > n {
                        break;
                    }
                    dst_slice[start..start + payload_bytes]
                        .copy_from_slice(&shadow.bytes[start..start + payload_bytes]);
                }
            }
            return Ok(n);
        }

        let dst_slice =
            unsafe { core::slice::from_raw_parts_mut((self.fb.addr as usize + off) as *mut u8, n) };
        dst_slice.copy_from_slice(&shadow.bytes[off..off + n]);
        Ok(n)
    }

    fn stat(&self) -> SysResult<VfsStat> {
        use abi::display_driver_protocol::FB_INFO_PAYLOAD_SIZE;
        Ok(VfsStat {
            mode: VfsStat::S_IFCHR | 0o666,
            size: FB_INFO_PAYLOAD_SIZE as u64,
            ino: 4,
            nlink: 1,
            rdev: VfsStat::makedev(29, 0),
            ..Default::default()
        })
    }

    fn phys_region(&self) -> SysResult<(u64, usize)> {
        Ok((self.fb.addr, self.fb.byte_len as usize))
    }
}

// ── /dev/rtc ─────────────────────────────────────────────────────────────────

pub struct RtcNode;

impl VfsNode for RtcNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        // Return unix seconds as a 64-bit value or text.
        // For compatibility with sprout: it expects properties, but since it's now path-based,
        // we can return a simple string or binary. Let's return the string first.
        let mono_ns = crate::runtime_base().mono_ticks() as u128 * 1_000_000_000
            / crate::runtime_base().mono_freq_hz() as u128;
        let sys_ns = if crate::time::is_anchored() {
            crate::time::get_system_time_ns(mono_ns as u64)
        } else {
            0
        };
        let text = format!("{}\n", sys_ns / 1_000_000_000);
        let slice = text.as_bytes();

        let off = offset as usize;
        if off >= slice.len() {
            return Ok(0);
        }
        let avail = &slice[off..];
        let n = avail.len().min(buf.len());
        buf[..n].copy_from_slice(&avail[..n]);
        Ok(n)
    }

    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        Err(Errno::EROFS)
    }

    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFCHR | 0o444,
            size: 0,
            ino: 5,
            nlink: 1,
            rdev: VfsStat::makedev(254, 0),
            ..Default::default()
        })
    }
}

// ── /dev/random ──────────────────────────────────────────────────────────────

/// Character device node for `/dev/random`.
///
/// Reads fill the buffer with cryptographically random bytes from the kernel
/// entropy pool.  Returns `EAGAIN` if the pool has not yet been seeded by a
/// hardware entropy source — callers that need blocking behaviour should
/// retry after a short sleep or use `/dev/urandom`.
/// Writes are ignored (Linux-compatible).
pub struct RandomNode;

impl VfsNode for RandomNode {
    fn read(&self, _offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        crate::entropy::fill(buf)?;
        Ok(buf.len())
    }

    fn write(&self, _offset: u64, buf: &[u8]) -> SysResult<usize> {
        // Writes add entropy (Linux-compatible behaviour).
        crate::entropy::add_sample(buf);
        Ok(buf.len())
    }

    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFCHR | 0o666,
            size: 0,
            ino: 6,
            nlink: 1,
            rdev: VfsStat::makedev(1, 8),
            ..Default::default()
        })
    }

    fn poll(&self) -> u16 {
        // Always ready for read once the pool is seeded.
        if crate::entropy::is_seeded() {
            abi::syscall::poll_flags::POLLIN | abi::syscall::poll_flags::POLLOUT
        } else {
            abi::syscall::poll_flags::POLLOUT
        }
    }
}

// ── /dev/urandom ─────────────────────────────────────────────────────────────

/// Character device node for `/dev/urandom`.
///
/// Reads always return random bytes even before the entropy pool is fully
/// seeded (non-blocking, like Linux `/dev/urandom`).  When the pool is not
/// yet seeded the output is deterministic but still mixed from the initial
/// pool state — suitable for bootstrapping purposes.
/// Writes are ignored (Linux-compatible).
pub struct UrandomNode;

impl VfsNode for UrandomNode {
    fn read(&self, _offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        // Always generate output regardless of seeded state.
        crate::entropy::fill_or_weak(buf);
        Ok(buf.len())
    }

    fn write(&self, _offset: u64, buf: &[u8]) -> SysResult<usize> {
        // Writes add entropy (Linux-compatible behaviour).
        crate::entropy::add_sample(buf);
        Ok(buf.len())
    }

    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFCHR | 0o666,
            size: 0,
            ino: 7,
            nlink: 1,
            rdev: VfsStat::makedev(1, 9),
            ..Default::default()
        })
    }

    fn poll(&self) -> u16 {
        // Always ready for both read and write.
        abi::syscall::poll_flags::POLLIN | abi::syscall::poll_flags::POLLOUT
    }
}

// ── /dev/kmsg ────────────────────────────────────────────────────────────────

/// Character device node for `/dev/kmsg`.
///
/// Provides a read-only view of the kernel message buffer (ring buffer).
/// Currently handles a single snapshot of the buffer per read call for simplicity.
pub struct KmsgNode;

impl VfsNode for KmsgNode {
    fn read(&self, offset: u64, buf: &mut [u8]) -> SysResult<usize> {
        if buf.is_empty() {
            return Ok(0);
        }

        // We use a temporary buffer to avoid holding the log lock for too long
        // and because copy_log_buffer currently returns the whole buffer.
        // For dmesg, a full snapshot is usually what's wanted.
        let mut temp = vec![0u8; crate::logging::get_log_buffer_len()];
        let n = crate::logging::copy_log_buffer(&mut temp);

        let off = offset as usize;
        if off >= n {
            return Ok(0);
        }

        let avail = &temp[off..n];
        let count = avail.len().min(buf.len());
        buf[..count].copy_from_slice(&avail[..count]);
        Ok(count)
    }

    fn write(&self, _offset: u64, _buf: &[u8]) -> SysResult<usize> {
        // Linux allows writing to /dev/kmsg to inject logs, but we'll stick to
        // read-only for now.
        Err(Errno::EPERM)
    }

    fn stat(&self) -> SysResult<VfsStat> {
        Ok(VfsStat {
            mode: VfsStat::S_IFCHR | 0o444,
            size: crate::logging::get_log_buffer_len() as u64,
            ino: 8,
            nlink: 1,
            rdev: VfsStat::makedev(1, 11),
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use super::*;

    fn lookup(path: &str) -> SysResult<Arc<dyn VfsNode>> {
        DevFs::new().lookup(path)
    }

    #[test]
    fn test_null_read_returns_zero() {
        let node = lookup("null").unwrap();
        let mut buf = [0xFFu8; 8];
        let n = node.read(0, &mut buf).unwrap();
        assert_eq!(n, 0);
    }

    #[test]
    fn test_null_write_succeeds() {
        let node = lookup("null").unwrap();
        let n = node.write(0, b"hello").unwrap();
        assert_eq!(n, 5);
    }

    #[test]
    fn test_zero_read_fills_zeros() {
        let node = lookup("zero").unwrap();
        let mut buf = [0xFFu8; 4];
        let n = node.read(0, &mut buf).unwrap();
        assert_eq!(n, 4);
        assert_eq!(buf, [0u8; 4]);
    }

    #[test]
    fn test_zero_write_succeeds() {
        let node = lookup("zero").unwrap();
        let n = node.write(0, b"ignored").unwrap();
        assert_eq!(n, 7);
    }

    #[test]
    fn test_lookup_console_returns_node() {
        assert!(lookup("console").is_ok());
    }

    #[test]
    fn test_lookup_unknown_returns_enoent() {
        assert!(matches!(lookup("nonexistent"), Err(Errno::ENOENT)));
    }

    #[test]
    fn test_null_stat() {
        let node = lookup("null").unwrap();
        let stat = node.stat().unwrap();
        assert!(stat.is_chr());
    }

    #[test]
    fn test_zero_stat() {
        let node = lookup("zero").unwrap();
        let stat = node.stat().unwrap();
        assert!(stat.is_chr());
    }

    #[test]
    fn test_lookup_root_is_dir() {
        let node = lookup("").unwrap();
        let stat = node.stat().unwrap();
        assert!(stat.is_dir());
    }

    #[test]
    fn test_readdir_lists_builtin_nodes() {
        let node = lookup("").unwrap();
        let mut buf = [0u8; 64];
        let n = node.readdir(0, &mut buf).unwrap();
        assert!(n > 0);
        let s = core::str::from_utf8(&buf[..n]).unwrap();
        assert!(s.contains("console"));
        assert!(s.contains("null"));
        assert!(s.contains("zero"));
    }

    #[test]
    fn test_lookup_net_is_dir() {
        let node = lookup("net").unwrap();
        let stat = node.stat().unwrap();
        assert!(stat.is_dir());
    }

    #[test]
    fn test_readdir_lists_net_directory() {
        let node = lookup("").unwrap();
        let mut buf = [0u8; 128];
        let n = node.readdir(0, &mut buf).unwrap();
        assert!(n > 0);
        let s = core::str::from_utf8(&buf[..n]).unwrap();
        assert!(s.contains("net"));
    }

    #[test]
    fn test_register_and_lookup_dynamic_device() {
        struct TestDev;
        impl VfsNode for TestDev {
            fn read(&self, _: u64, _: &mut [u8]) -> SysResult<usize> {
                Ok(0)
            }
            fn write(&self, _: u64, buf: &[u8]) -> SysResult<usize> {
                Ok(buf.len())
            }
            fn stat(&self) -> SysResult<VfsStat> {
                Ok(VfsStat {
                    mode: VfsStat::S_IFCHR | 0o666,
                    size: 0,
                    ino: 999,
                    ..Default::default()
                })
            }
        }

        register("test_unique_dev_42", Arc::new(TestDev));
        let node = DevFs::new().lookup("test_unique_dev_42").unwrap();
        assert!(node.stat().unwrap().is_chr());
        // Clean up.
        unregister("test_unique_dev_42");
    }

    #[test]
    fn test_unregister_removes_device() {
        struct TestDev2;
        impl VfsNode for TestDev2 {
            fn read(&self, _: u64, _: &mut [u8]) -> SysResult<usize> {
                Ok(0)
            }
            fn write(&self, _: u64, buf: &[u8]) -> SysResult<usize> {
                Ok(buf.len())
            }
            fn stat(&self) -> SysResult<VfsStat> {
                Ok(VfsStat {
                    mode: VfsStat::S_IFCHR | 0o666,
                    size: 0,
                    ino: 998,
                    ..Default::default()
                })
            }
        }

        register("test_unique_dev_99", Arc::new(TestDev2));
        assert!(DevFs::new().lookup("test_unique_dev_99").is_ok());
        let removed = unregister("test_unique_dev_99");
        assert!(removed);
        assert!(matches!(DevFs::new().lookup("test_unique_dev_99"), Err(Errno::ENOENT)));
    }

    // ── /dev/urandom tests ───────────────────────────────────────────────────

    #[test]
    fn test_lookup_urandom_returns_node() {
        assert!(lookup("urandom").is_ok());
    }

    #[test]
    fn test_urandom_stat_is_chr() {
        let node = lookup("urandom").unwrap();
        let stat = node.stat().unwrap();
        assert!(stat.is_chr());
    }

    #[test]
    fn test_urandom_read_fills_buffer() {
        let node = lookup("urandom").unwrap();
        let mut buf = [0u8; 16];
        let n = node.read(0, &mut buf).unwrap();
        assert_eq!(n, 16);
    }

    #[test]
    fn test_urandom_read_empty_buffer_returns_zero() {
        let node = lookup("urandom").unwrap();
        let mut buf = [];
        let n = node.read(0, &mut buf).unwrap();
        assert_eq!(n, 0);
    }

    #[test]
    fn test_urandom_write_succeeds() {
        let node = lookup("urandom").unwrap();
        let n = node.write(0, b"some entropy").unwrap();
        assert_eq!(n, 12);
    }

    #[test]
    fn test_urandom_poll_always_readable() {
        let node = lookup("urandom").unwrap();
        let mask = node.poll();
        assert!(mask & abi::syscall::poll_flags::POLLIN != 0);
    }

    // ── /dev/random tests ────────────────────────────────────────────────────

    #[test]
    fn test_lookup_random_returns_node() {
        assert!(lookup("random").is_ok());
    }

    #[test]
    fn test_random_stat_is_chr() {
        let node = lookup("random").unwrap();
        let stat = node.stat().unwrap();
        assert!(stat.is_chr());
    }

    #[test]
    fn test_random_read_after_seeding() {
        // Seed the pool so /dev/random becomes readable.
        crate::entropy::add_sample(b"test_entropy_data_12345678");
        crate::entropy::mark_seeded();

        let node = lookup("random").unwrap();
        let mut buf = [0u8; 8];
        let n = node.read(0, &mut buf).unwrap();
        assert_eq!(n, 8);
    }

    #[test]
    fn test_random_write_succeeds() {
        let node = lookup("random").unwrap();
        let n = node.write(0, b"more entropy").unwrap();
        assert_eq!(n, 12);
    }

    #[test]
    fn test_readdir_lists_random_nodes() {
        let node = lookup("").unwrap();
        let mut buf = [0u8; 128];
        let n = node.readdir(0, &mut buf).unwrap();
        assert!(n > 0);
        let s = core::str::from_utf8(&buf[..n]).unwrap();
        assert!(s.contains("random"));
        assert!(s.contains("urandom"));
    }

    // ── ConsoleNode termios tests ─────────────────────────────────────────────

    /// Serialise console-state tests that touch global statics.
    static CONSOLE_TEST_GUARD: spin::Mutex<()> = spin::Mutex::new(());

    #[test]
    fn test_console_is_tty() {
        let node = lookup("console").unwrap();
        assert!(node.is_tty());
    }

    #[test]
    fn test_console_default_termios_icanon() {
        let _g = CONSOLE_TEST_GUARD.lock();
        // Reset to a known state.
        ConsoleNode::set_termios(abi::termios::DEFAULT_TERMIOS);
        let t = ConsoleNode::get_termios();
        assert_ne!(t.c_lflag & abi::termios::ICANON, 0, "ICANON should be set");
        assert_ne!(t.c_lflag & abi::termios::ECHO, 0, "ECHO should be set");
        assert_ne!(t.c_lflag & abi::termios::ISIG, 0, "ISIG should be set");
        assert_ne!(t.c_iflag & abi::termios::ICRNL, 0, "ICRNL should be set");
    }

    #[test]
    fn test_console_set_raw_mode_clears_icanon() {
        let _g = CONSOLE_TEST_GUARD.lock();
        let mut raw = abi::termios::DEFAULT_TERMIOS;
        raw.c_lflag &= !(abi::termios::ICANON | abi::termios::ECHO | abi::termios::ISIG);
        raw.c_iflag &= !(abi::termios::ICRNL | abi::termios::IXON);
        ConsoleNode::set_termios(raw);

        let t = ConsoleNode::get_termios();
        assert_eq!(t.c_lflag & abi::termios::ICANON, 0, "ICANON should be clear");
        assert_eq!(t.c_lflag & abi::termios::ECHO, 0, "ECHO should be clear");
        assert_eq!(t.c_lflag & abi::termios::ISIG, 0, "ISIG should be clear");

        // Restore.
        ConsoleNode::set_termios(abi::termios::DEFAULT_TERMIOS);
    }

    #[test]
    fn test_console_read_returns_eintr_on_pending_interrupt() {
        use core::sync::atomic::{AtomicBool, Ordering};

        use crate::sched::hooks::TAKE_PENDING_INTERRUPT_HOOK;

        static INTERRUPT_PENDING: AtomicBool = AtomicBool::new(false);

        fn take_interrupt() -> bool {
            INTERRUPT_PENDING.swap(false, Ordering::SeqCst)
        }

        // Set the interrupt flag so the first loop iteration returns EINTR.
        INTERRUPT_PENDING.store(true, Ordering::SeqCst);
        unsafe { TAKE_PENDING_INTERRUPT_HOOK = Some(take_interrupt) };

        let node = ConsoleNode;
        let mut buf = [0u8; 4];
        let result = node.read(0, &mut buf);

        // Clear the hook to not affect other tests.
        unsafe { TAKE_PENDING_INTERRUPT_HOOK = None };

        assert_eq!(result, Err(abi::errors::Errno::EINTR));
    }

    #[test]
    fn test_console_device_call_unknown_kind_returns_enosys() {
        let node = ConsoleNode;
        let call = abi::device::DeviceCall {
            kind: abi::device::DeviceKind::RtcCmos,
            op: 1,
            in_ptr: 0,
            in_len: 0,
            out_ptr: 0,
            out_len: 0,
        };
        let result = node.device_call(&call);
        assert_eq!(result, Err(abi::errors::Errno::ENOSYS));
    }

    #[test]
    fn test_console_device_call_tcgets_null_ptr_returns_einval() {
        let node = ConsoleNode;
        let call = abi::device::DeviceCall {
            kind: abi::device::DeviceKind::Terminal,
            op: abi::termios::TERMINAL_OP_TCGETS,
            in_ptr: 0,
            in_len: 0,
            out_ptr: 0, // null → EINVAL
            out_len: 0,
        };
        let result = node.device_call(&call);
        assert_eq!(result, Err(abi::errors::Errno::EINVAL));
    }

    #[test]
    fn test_console_device_call_tcsets_null_ptr_returns_einval() {
        let node = ConsoleNode;
        let call = abi::device::DeviceCall {
            kind: abi::device::DeviceKind::Terminal,
            op: abi::termios::TERMINAL_OP_TCSETS,
            in_ptr: 0, // null → EINVAL
            in_len: 0,
            out_ptr: 0,
            out_len: 0,
        };
        let result = node.device_call(&call);
        assert_eq!(result, Err(abi::errors::Errno::EINVAL));
    }

    #[test]
    fn test_console_device_call_tcgets_writes_termios() {
        let _g = CONSOLE_TEST_GUARD.lock();
        ConsoleNode::set_termios(abi::termios::DEFAULT_TERMIOS);

        let node = ConsoleNode;
        let mut out = abi::termios::Termios::default();
        let size = core::mem::size_of::<abi::termios::Termios>();
        let call = abi::device::DeviceCall {
            kind: abi::device::DeviceKind::Terminal,
            op: abi::termios::TERMINAL_OP_TCGETS,
            in_ptr: 0,
            in_len: 0,
            out_ptr: &mut out as *mut _ as u64,
            out_len: size as u32,
        };
        let result = node.device_call(&call);
        assert_eq!(result, Ok(0));
        assert_eq!(out.c_lflag & abi::termios::ICANON, abi::termios::ICANON);
    }

    #[test]
    fn test_console_device_call_tcsets_updates_termios() {
        let _g = CONSOLE_TEST_GUARD.lock();
        ConsoleNode::set_termios(abi::termios::DEFAULT_TERMIOS);

        let node = ConsoleNode;
        let mut raw = abi::termios::DEFAULT_TERMIOS;
        raw.c_lflag &= !(abi::termios::ICANON | abi::termios::ECHO);

        let size = core::mem::size_of::<abi::termios::Termios>();
        let call = abi::device::DeviceCall {
            kind: abi::device::DeviceKind::Terminal,
            op: abi::termios::TERMINAL_OP_TCSETS,
            in_ptr: &raw as *const _ as u64,
            in_len: size as u32,
            out_ptr: 0,
            out_len: 0,
        };
        let result = node.device_call(&call);
        assert_eq!(result, Ok(0));

        let t = ConsoleNode::get_termios();
        assert_eq!(t.c_lflag & abi::termios::ICANON, 0);
        assert_eq!(t.c_lflag & abi::termios::ECHO, 0);

        // Restore.
        ConsoleNode::set_termios(abi::termios::DEFAULT_TERMIOS);
    }

    #[test]
    fn test_console_device_call_tiocgwinsz_returns_framebuffer_geometry() {
        let _g = CONSOLE_TEST_GUARD.lock();
        let prev_fb = *BOOT_FB_INFO.lock();
        set_boot_fb(
            crate::FramebufferInfo {
                addr: 0,
                byte_len: 1600 * 1200 * 4,
                width: 1600,
                height: 1200,
                pitch: 1600 * 4,
                bpp: 32,
                format: crate::PixelFormat::Bgra8888,
            },
            0,
        );

        let node = ConsoleNode;
        let mut out = abi::termios::Winsize::default();
        let call = abi::device::DeviceCall {
            kind: abi::device::DeviceKind::Terminal,
            op: abi::termios::TERMINAL_OP_TIOCGWINSZ,
            in_ptr: 0,
            in_len: 0,
            out_ptr: &mut out as *mut _ as u64,
            out_len: core::mem::size_of::<abi::termios::Winsize>() as u32,
        };
        let result = node.device_call(&call);
        assert_eq!(result, Ok(0));
        assert_eq!(out.ws_row, 75);
        assert_eq!(out.ws_col, 200);
        assert_eq!(out.ws_xpixel, 1600);
        assert_eq!(out.ws_ypixel, 1200);

        *BOOT_FB_INFO.lock() = prev_fb;
    }

    #[test]
    fn test_console_device_call_tiocgwinsz_without_framebuffer_returns_enosys() {
        let _g = CONSOLE_TEST_GUARD.lock();
        let prev_fb = *BOOT_FB_INFO.lock();
        *BOOT_FB_INFO.lock() = None;

        let node = ConsoleNode;
        let mut out = abi::termios::Winsize::default();
        let call = abi::device::DeviceCall {
            kind: abi::device::DeviceKind::Terminal,
            op: abi::termios::TERMINAL_OP_TIOCGWINSZ,
            in_ptr: 0,
            in_len: 0,
            out_ptr: &mut out as *mut _ as u64,
            out_len: core::mem::size_of::<abi::termios::Winsize>() as u32,
        };
        let result = node.device_call(&call);
        assert_eq!(result, Err(abi::errors::Errno::ENOSYS));

        *BOOT_FB_INFO.lock() = prev_fb;
    }

    #[test]
    fn test_console_device_call_tiocgwinsz_too_small_framebuffer_returns_enosys() {
        let _g = CONSOLE_TEST_GUARD.lock();
        let prev_fb = *BOOT_FB_INFO.lock();
        set_boot_fb(
            crate::FramebufferInfo {
                addr: 0,
                byte_len: 7 * 15 * 4,
                width: 7,
                height: 15,
                pitch: 7 * 4,
                bpp: 32,
                format: crate::PixelFormat::Bgra8888,
            },
            0,
        );

        let node = ConsoleNode;
        let mut out = abi::termios::Winsize::default();
        let call = abi::device::DeviceCall {
            kind: abi::device::DeviceKind::Terminal,
            op: abi::termios::TERMINAL_OP_TIOCGWINSZ,
            in_ptr: 0,
            in_len: 0,
            out_ptr: &mut out as *mut _ as u64,
            out_len: core::mem::size_of::<abi::termios::Winsize>() as u32,
        };
        let result = node.device_call(&call);
        assert_eq!(result, Err(abi::errors::Errno::ENOSYS));

        *BOOT_FB_INFO.lock() = prev_fb;
    }

    #[test]
    fn test_console_device_call_tcgetpgrp_requires_process_context() {
        let _g = CONSOLE_TEST_GUARD.lock();
        ConsoleNode::set_tty_owner_for_test(Some(1), Some(1));

        let node = ConsoleNode;
        let mut out_pgid = 0u32;
        let call = abi::device::DeviceCall {
            kind: abi::device::DeviceKind::Terminal,
            op: abi::termios::TERMINAL_OP_TCGETPGRP,
            in_ptr: 0,
            in_len: 0,
            out_ptr: &mut out_pgid as *mut u32 as u64,
            out_len: core::mem::size_of::<u32>() as u32,
        };
        let result = node.device_call(&call);
        assert_eq!(result, Err(abi::errors::Errno::ENOTTY));

        ConsoleNode::set_tty_owner_for_test(None, None);
    }

    #[test]
    fn test_console_device_call_tcsetpgrp_requires_process_context() {
        let _g = CONSOLE_TEST_GUARD.lock();
        ConsoleNode::set_tty_owner_for_test(Some(1), Some(1));

        let node = ConsoleNode;
        let new_pgid = 2u32;
        let call = abi::device::DeviceCall {
            kind: abi::device::DeviceKind::Terminal,
            op: abi::termios::TERMINAL_OP_TCSETPGRP,
            in_ptr: &new_pgid as *const u32 as u64,
            in_len: core::mem::size_of::<u32>() as u32,
            out_ptr: 0,
            out_len: 0,
        };
        let result = node.device_call(&call);
        assert_eq!(result, Err(abi::errors::Errno::ENOTTY));

        ConsoleNode::set_tty_owner_for_test(None, None);
    }

    // ── Ownership / rdev / nlink metadata tests ──────────────────────────────

    #[test]
    fn test_null_stat_has_rdev_and_nlink() {
        let st = NullNode.stat().unwrap();
        assert!(st.is_chr());
        assert_eq!(st.nlink, 1);
        assert_eq!(st.rdev, VfsStat::makedev(1, 3));
        assert_eq!(st.uid, 0);
        assert_eq!(st.gid, 0);
    }

    #[test]
    fn test_zero_stat_has_rdev_and_nlink() {
        let st = ZeroNode.stat().unwrap();
        assert!(st.is_chr());
        assert_eq!(st.nlink, 1);
        assert_eq!(st.rdev, VfsStat::makedev(1, 5));
    }

    #[test]
    fn test_console_stat_has_rdev_and_nlink() {
        let st = ConsoleNode.stat().unwrap();
        assert!(st.is_chr());
        assert_eq!(st.nlink, 1);
        assert_eq!(st.rdev, VfsStat::makedev(5, 1));
    }

    #[test]
    fn test_rtc_stat_has_rdev_and_nlink() {
        let st = RtcNode.stat().unwrap();
        assert!(st.is_chr());
        assert_eq!(st.nlink, 1);
        assert_eq!(st.rdev, VfsStat::makedev(254, 0));
    }

    #[test]
    fn test_random_stat_has_rdev_and_nlink() {
        let st = RandomNode.stat().unwrap();
        assert!(st.is_chr());
        assert_eq!(st.nlink, 1);
        assert_eq!(st.rdev, VfsStat::makedev(1, 8));
    }

    #[test]
    fn test_urandom_stat_has_rdev_and_nlink() {
        let st = UrandomNode.stat().unwrap();
        assert!(st.is_chr());
        assert_eq!(st.nlink, 1);
        assert_eq!(st.rdev, VfsStat::makedev(1, 9));
    }

    #[test]
    fn test_dev_dir_stat_has_nlink_two() {
        let st = DevDirNode.stat().unwrap();
        assert!(st.is_dir());
        assert!(st.nlink >= 2, "dev dir nlink should be >= 2, got {}", st.nlink);
    }

    #[test]
    fn test_fbnode_full_frame_write_updates_scanout_once_buffer_is_staged() {
        let mut backing = vec![0u8; 16];
        let fb = crate::FramebufferInfo {
            addr: backing.as_mut_ptr() as u64,
            byte_len: backing.len() as u64,
            width: 2,
            height: 2,
            pitch: 8,
            bpp: 32,
            format: crate::PixelFormat::Bgra8888,
        };
        let node = FbNode::new(fb, 0);
        let new_frame = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

        let written = node.write(0, &new_frame).unwrap();
        assert_eq!(written, new_frame.len());
        assert_eq!(&backing[..], &new_frame);
    }

    #[test]
    fn test_fbnode_partial_write_preserves_other_pixels_via_shadow_buffer() {
        let mut backing = vec![10u8; 16];
        let fb = crate::FramebufferInfo {
            addr: backing.as_mut_ptr() as u64,
            byte_len: backing.len() as u64,
            width: 2,
            height: 2,
            pitch: 8,
            bpp: 32,
            format: crate::PixelFormat::Bgra8888,
        };
        let node = FbNode::new(fb, 0);

        let written = node.write(4, &[1u8, 2, 3, 4]).unwrap();
        assert_eq!(written, 4);
        assert_eq!(&backing[..4], &[10u8; 4]);
        assert_eq!(&backing[4..8], &[1u8, 2, 3, 4]);
        assert_eq!(&backing[8..], &[10u8; 8]);
    }
}
