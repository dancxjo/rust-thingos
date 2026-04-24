#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod authority;
mod boot_progress;
pub mod device_registry;
pub mod entropy;
pub mod group;
pub mod handle;
pub mod inbox;
pub mod ipc;
pub mod irq;
pub mod job;
pub mod logging;
pub mod memory;
pub mod message;
pub mod once_cell;
pub mod place;
pub mod presence;
pub mod signal;
pub mod space;
pub mod spawn;

#[cfg(test)]
mod boundary_contract;
pub mod sched;
pub mod simd;
pub mod syscall;
pub mod task;
pub mod vfs;

pub mod time;
pub mod trace;
pub mod virtio;

use alloc::string::ToString;

use abi::errors::Errno;
use abi::vm::{VmBackingKind, VmMapFlags, VmProt, VmRegionInfo};

use crate::task::StartupArg;

const RAW_BOOT_TRACE_ENABLED: bool = false;

#[inline]
fn boot_trace<R: BootRuntime>(runtime: &R, msg: &[u8]) {
    if RAW_BOOT_TRACE_ENABLED {
        runtime.serial_putbuf(msg);
    } else {
        let _ = (runtime, msg);
    }
}

fn parse_loglevel_value(raw: &str) -> Option<u8> {
    let value = raw.trim_matches('"').trim();
    if let Ok(level) = value.parse::<u8>() {
        return match level {
            0..=5 => Some(level),
            _ => None,
        };
    }

    if value.eq_ignore_ascii_case("off") {
        Some(0)
    } else if value.eq_ignore_ascii_case("error") || value.eq_ignore_ascii_case("err") {
        Some(1)
    } else if value.eq_ignore_ascii_case("warn") || value.eq_ignore_ascii_case("warning") {
        Some(2)
    } else if value.eq_ignore_ascii_case("info") {
        Some(3)
    } else if value.eq_ignore_ascii_case("debug") {
        Some(4)
    } else if value.eq_ignore_ascii_case("trace") {
        Some(5)
    } else {
        None
    }
}

fn parse_cmdline_loglevel(cmdline: &str) -> Option<u8> {
    let mut parsed = None;
    let mut expect_value = false;

    for token in cmdline.split_ascii_whitespace() {
        if expect_value {
            if let Some(level) = parse_loglevel_value(token) {
                parsed = Some(level);
            }
            expect_value = false;
            continue;
        }

        if let Some(value) = token.strip_prefix("loglevel=") {
            if let Some(level) = parse_loglevel_value(value) {
                parsed = Some(level);
            }
            continue;
        }

        if token.eq_ignore_ascii_case("loglevel") {
            expect_value = true;
        }
    }

    parsed
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_handle_page_fault(rip: u64, addr: u64, err: u64) {
    // Decode x86_64 page fault error code bits
    let present = (err & 0x1) != 0;
    let write = (err & 0x2) != 0;
    let user = (err & 0x4) != 0;
    let instr_fetch = (err & 0x10) != 0;

    let stack_result = if user {
        unsafe { crate::sched::handle_user_stack_fault_current(addr) }
    } else {
        crate::sched::StackFaultResult::NotStack
    };
    if stack_result == crate::sched::StackFaultResult::Grew {
        return;
    }

    let tid = unsafe { crate::sched::current_tid_current() };
    let name_bytes = unsafe { crate::sched::current_task_name_current() };
    let name_len = name_bytes.iter().position(|&b| b == 0).unwrap_or(32);
    let task_name = core::str::from_utf8(&name_bytes[..name_len]).unwrap_or("unknown");
    let hw_tls_base = crate::runtime_base().get_user_tls_base_dyn();
    let task_tls_base = unsafe { crate::sched::current_user_fs_base_current() };

    // Structured page fault logging with decoded error bits
    crate::log_event!(
        crate::logging::LogLevel::Error,
        "kernel::trap",
        "user_page_fault tid={} task='{}' va=0x{:016x} rip=0x{:016x} err=0x{:04x} p={} u={} w={} i={} fs=0x{:016x} task_fs=0x{:016x}",
        tid,
        task_name,
        addr,
        rip,
        err,
        present as u8,
        user as u8,
        write as u8,
        instr_fetch as u8,
        hw_tls_base,
        task_tls_base
    );

    if stack_result == crate::sched::StackFaultResult::Overflow {
        crate::kerror!("STACK: overflow at va=0x{:x} (task {})", addr, task_name);
    }

    // Deliver SIGSEGV with rich fault context as a typed inbox message so that
    // the process can log crash diagnostics before the kernel terminates it.
    if user {
        if let Some(pinfo) = crate::sched::process_info_current() {
            let pid = pinfo.lock().pid;
            // rsp is not available in the page-fault handler context; pass 0.
            crate::signal::send_fault_signal_to_process(pid, abi::signal::SIGSEGV, addr, rip, 0);
        }
    }

    unsafe {
        crate::sched::exit_current(-1);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_handle_exception(
    rip: u64,
    error_code: u64,
    rsp: u64,
    _cs: u64,
    kind: u64,
) {
    let exception_name = match kind {
        0 => "Divide-by-zero",
        1 => "Debug",
        2 => "NMI",
        3 => "Breakpoint",
        4 => "Overflow",
        5 => "Bound Range",
        6 => "Invalid Opcode (UD2)",
        7 => "Device Not Available",
        8 => "Double Fault",
        10 => "Invalid TSS",
        11 => "Segment Not Present",
        12 => "Stack-Segment Fault",
        13 => "General Protection Fault",
        14 => "Page Fault",
        16 => "FPU Exception",
        17 => "Alignment Check",
        18 => "Machine Check",
        19 => "SIMD Exception",
        20 => "Virtualization",
        21 => "Control Protection",
        30 => "Security Exception",
        _ => "Unknown Exception",
    };

    let tid = unsafe { crate::sched::current_tid_current() };
    let name_bytes = unsafe { crate::sched::current_task_name_current() };
    let name_len = name_bytes.iter().position(|&b| b == 0).unwrap_or(32);
    let task_name = core::str::from_utf8(&name_bytes[..name_len]).unwrap_or("unknown");

    // Peeking at instruction bytes for Invalid Opcode
    let mut instr_bytes = [0u8; 8];
    let mut peek_ok = false;
    if kind == 6 {
        for i in 0..8 {
            if let Some(phys) = crate::memory::translate_user_page(rip + i as u64) {
                let off = crate::runtime_base().phys_to_virt_offset();
                let ptr = (phys + off) as *const u8;
                unsafe {
                    instr_bytes[i] = *ptr;
                }
                peek_ok = true;
            } else {
                break;
            }
        }
    }

    if peek_ok {
        crate::log_event!(
            crate::logging::LogLevel::Error,
            "kernel::trap",
            "{} tid={} task='{}' rip=0x{:016x} rsp=0x{:016x} err=0x{:04x} instr={:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            exception_name,
            tid,
            task_name,
            rip,
            rsp,
            error_code,
            instr_bytes[0],
            instr_bytes[1],
            instr_bytes[2],
            instr_bytes[3],
            instr_bytes[4],
            instr_bytes[5],
            instr_bytes[6],
            instr_bytes[7]
        );
    } else {
        crate::log_event!(
            crate::logging::LogLevel::Error,
            "kernel::trap",
            "{} tid={} task='{}' rip=0x{:016x} rsp=0x{:016x} err=0x{:04x} kind={}",
            exception_name,
            tid,
            task_name,
            rip,
            rsp,
            error_code,
            kind
        );
    }

    // Deliver a typed fault signal to the process inbox for crash-class exceptions
    // so applications can capture rich diagnostics in their event loop.
    //
    // Exception-to-POSIX-signal mapping (x86_64):
    //   0  Divide-by-zero         → SIGFPE
    //   4  Overflow                → SIGFPE
    //   5  Bound Range             → SIGSEGV
    //   6  Invalid Opcode (SIGILL) → SIGILL
    //  13  General Protection      → SIGSEGV
    //  16  FPU Exception           → SIGFPE
    //  17  Alignment Check         → SIGBUS
    //  19  SIMD Exception          → SIGFPE
    //
    // The fault_addr is the faulting memory address for memory-related exceptions
    // (GPF passes error_code which encodes the segment selector, not a vaddr; use
    // rip as the instruction address in that case).  For arithmetic/SIGFPE-class
    // exceptions there is no meaningful fault address; pass rip so the receiver
    // can locate the faulting instruction.
    let fault_sig = match kind {
        0 | 4 | 16 | 19 => Some((abi::signal::SIGFPE, rip)),
        5 => Some((abi::signal::SIGSEGV, rip)),
        6 => Some((abi::signal::SIGILL, rip)),
        13 => Some((abi::signal::SIGSEGV, rip)),
        17 => Some((abi::signal::SIGBUS, rip)),
        _ => None,
    };
    if let Some((sig, fault_addr)) = fault_sig {
        if let Some(pinfo) = crate::sched::process_info_current() {
            let pid = pinfo.lock().pid;
            crate::signal::send_fault_signal_to_process(pid, sig, fault_addr, rip, rsp);
        }
    }

    unsafe {
        crate::sched::exit_current(-1);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PhysRange {
    pub start: u64,
    pub end: u64,
    pub kind: PhysRangeKind,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysRangeKind {
    Usable,
    Reserved,
    Mmio,
    Firmware,
    KernelImage,
    BootModule,
    Framebuffer,
    Acpi,
    Other,
}

#[derive(Clone, Copy, Debug)]
pub struct BootModuleDesc {
    pub name: &'static str,
    pub cmdline: &'static str,
    pub bytes: &'static [u8],
    pub phys_start: u64,
    pub phys_end: u64,
    pub kind: BootModuleKind,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BootModuleKind {
    Unknown,
    Elf,
    Wasm,
    Data,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FramebufferInfo {
    pub addr: u64,
    pub byte_len: u64,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub bpp: u16,
    pub format: PixelFormat,
}

/// Pixel format for framebuffer surfaces.
///
/// Values match `abi::schema::pixel_format` constants for wire compatibility.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    /// Unknown or unsupported format.
    Unknown = 0,
    /// 32-bit BGRA: Memory [B, G, R, A] -> u32 0xAARRGGBB.
    Bgra8888 = 1,
    /// 32-bit BGRX: Memory [B, G, R, X] -> u32 0xXXRRGGBB (alpha ignored).
    Bgrx8888 = 2,
    /// 16-bit RGB565.
    Rgb565 = 3,
}

impl PixelFormat {
    /// Convert to wire-compatible u64 for graph properties.
    #[inline]
    pub const fn to_wire(self) -> u64 {
        self as u64
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct IrqState(pub usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CpuId(pub u32);

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct MapPerms {
    pub user: bool,
    pub read: bool,
    pub write: bool,
    pub exec: bool,
    pub kind: MapKind,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum MapKind {
    #[default]
    Normal,
    Device,
    Framebuffer,
}

pub struct UserTaskSpec<AS> {
    pub entry: u64,
    pub stack_top: u64,
    pub aspace: AS,
    pub arg: usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UserEntry {
    pub entry_pc: usize,
    pub user_sp: usize,
    pub arg0: usize,
}

pub trait FrameAllocatorHook {
    fn alloc_frame(&self) -> Option<u64>;
}

pub trait BootTasking {
    type Runtime: BootRuntime<Tasking = Self>;
    type Context: Copy + Default;
    type AddressSpace: Copy + Default + PartialEq;

    fn init(&self, hhdm_offset: u64);
    fn init_kernel_context(
        &self,
        entry: extern "C" fn(usize) -> !,
        stack_top: u64,
        arg: usize,
    ) -> Self::Context;
    fn init_user_context(
        &self,
        spec: UserTaskSpec<Self::AddressSpace>,
        kstack_top: u64,
    ) -> Self::Context;

    unsafe fn switch(&self, from: &mut Self::Context, to: &Self::Context, to_tid: u64);

    /// Like [`switch`] but also saves/restores the per-thread user TLS base.
    ///
    /// `from_user_fs_base` is a pointer into the outgoing task's `user_fs_base`
    /// field; on switch-out the arch layer reads the live hardware base and stores
    /// it there.  `to_user_fs_base` is the value loaded from the incoming task's
    /// field and written to hardware on switch-in.
    ///
    /// The default implementation simply delegates to [`switch`] — architectures
    /// without a dedicated user TLS register need no changes.
    unsafe fn switch_with_tls(
        &self,
        from: &mut Self::Context,
        to: &Self::Context,
        to_tid: u64,
        _from_user_fs_base: *mut u64,
        _to_user_fs_base: u64,
    ) {
        unsafe { self.switch(from, to, to_tid) }
    }

    /// Read the current thread's user TLS base from hardware (FS_BASE on x86_64).
    /// Returns 0 on architectures without a dedicated user TLS register.
    fn get_user_tls_base(&self) -> u64 {
        0
    }

    /// Write a new user TLS base to hardware immediately (FS_BASE on x86_64).
    /// No-op on architectures without a dedicated user TLS register.
    fn set_user_tls_base(&self, _base: u64) {}
    unsafe fn enter_user(&self, entry: UserEntry) -> !;

    fn make_user_address_space(&self) -> Self::AddressSpace;
    fn active_address_space(&self) -> Self::AddressSpace;
    fn activate_address_space(&self, aspace: Self::AddressSpace);

    fn map_page(
        &self,
        aspace: Self::AddressSpace,
        virt: u64,
        phys: u64,
        perms: MapPerms,
        allocator: &dyn FrameAllocatorHook,
    ) -> Result<(), ()>;

    fn unmap_page(&self, aspace: Self::AddressSpace, virt: u64) -> Result<Option<u64>, ()>;
    fn protect_page(
        &self,
        aspace: Self::AddressSpace,
        virt: u64,
        perms: MapPerms,
    ) -> Result<(), ()>;
    fn translate(&self, aspace: Self::AddressSpace, virt: u64) -> Option<u64>;
    fn tlb_flush_page(&self, virt: u64);

    /// Convert an address space handle to a raw `u64` token suitable for
    /// storage in the architecture-independent [`crate::task::Process`] struct.
    ///
    /// For single-register architectures (x86-64 CR3, RISC-V SATP, AArch64
    /// TTBR0) this returns the register value directly.  For dual-register
    /// architectures (LoongArch64 PGDL/PGDH) this returns the user-half
    /// register (PGDL); the kernel-half is shared across all processes and
    /// does not need per-process storage.
    ///
    /// The default implementation returns 0; architectures that require
    /// process-scoped address-space tracking must override this.
    fn aspace_to_raw(&self, _aspace: Self::AddressSpace) -> u64 {
        0
    }
}

pub trait BootRuntimeBase: 'static {
    fn putchar(&self, c: u8);
    /// Write multiple bytes to the combined console output.
    /// Default implementation forwards byte-by-byte to `putchar`.
    fn putbuf(&self, buf: &[u8]) {
        for &b in buf {
            self.putchar(b);
        }
    }
    /// Write one byte to the serial port synchronously, bypassing any deferred buffers.
    fn serial_putchar_sync(&self, c: u8) {
        self.serial_putchar(c); // Default fallback
    }
    /// Write multiple bytes to the serial port synchronously.
    fn serial_putbuf_sync(&self, buf: &[u8]) {
        for &b in buf {
            self.serial_putchar_sync(b);
        }
    }
    /// Write one byte to the serial port only (not the framebuffer console).
    fn serial_putchar(&self, c: u8) {
        self.putchar(c);
    }
    /// Write multiple bytes to serial only.
    /// Default implementation forwards byte-by-byte to `serial_putchar`.
    fn serial_putbuf(&self, buf: &[u8]) {
        for &b in buf {
            self.serial_putchar(b);
        }
    }
    /// Write one byte to the framebuffer console only (not the serial port).
    fn fb_putchar(&self, c: u8) {
        self.putchar(c);
    }
    /// Write multiple bytes to framebuffer only.
    /// Default implementation forwards byte-by-byte to `fb_putchar`.
    fn fb_putbuf(&self, buf: &[u8]) {
        for &b in buf {
            self.fb_putchar(b);
        }
    }
    /// Non-blocking serial read. Returns `Some(byte)` if data is available.
    fn getchar(&self) -> Option<u8> {
        None
    }
    /// Give the runtime a chance to immediately consume urgent console bytes
    /// such as VINTR/VQUIT/VSUSP before they are queued for later line-discipline
    /// processing.
    fn handle_console_input_byte(&self, byte: u8) -> bool {
        crate::vfs::devfs::ConsoleNode::handle_runtime_input_byte(self, byte)
    }
    fn mono_ticks(&self) -> u64;
    fn mono_freq_hz(&self) -> u64 {
        10_000_000
    }

    fn pci_cfg_read32(&self, _bus: u8, _dev: u8, _func: u8, _offset: u8) -> Result<u32, Errno> {
        Err(Errno::NotSupported)
    }
    fn pci_cfg_write32(
        &self,
        _bus: u8,
        _dev: u8,
        _func: u8,
        _offset: u8,
        _value: u32,
    ) -> Result<(), Errno> {
        Err(Errno::NotSupported)
    }

    fn lapic_id(&self) -> Result<u32, Errno> {
        Err(Errno::NotSupported)
    }
    fn lapic_base_phys(&self) -> Result<u64, Errno> {
        Err(Errno::NotSupported)
    }

    fn simd_init_cpu(&self) {}

    /// Wait for interrupt - low-power idle until next IRQ
    fn wait_for_interrupt(&self) {}

    /// Reboot the system. This should never return.
    fn reboot(&self) -> ! {
        loop {
            core::hint::spin_loop();
        }
    }

    /// Shutdown the system. This should never return.
    fn shutdown(&self) -> ! {
        loop {
            core::hint::spin_loop();
        }
    }

    fn irq_disable(&self) -> IrqState {
        IrqState(0)
    }
    fn irq_restore(&self, _state: IrqState) {}

    /// Send an Inter-Processor Interrupt (IPI) to a specific CPU.
    fn send_ipi(&self, _cpu_index: usize, _vector: u8) {}

    fn current_cpu_id(&self) -> CpuId {
        CpuId(0)
    }

    fn current_cpu_index(&self) -> usize {
        0
    }

    fn current_tid(&self) -> u64 {
        0
    }

    fn set_current_tid(&self, _tid: u64) {}

    /// Broadcast a TLB shootdown IPI to all other CPUs.
    fn tlb_shootdown_broadcast(&self) {}

    /// Per-CPU initialization for secondary CPUs.
    /// Initialize a secondary CPU after it has entered the kernel.
    fn init_secondary_cpu(&self, cpu_index: usize);

    /// Total CPUs discovered on this platform.
    fn cpu_total_count(&self) -> usize {
        1
    }

    /// Returns the next offline CPU id.
    fn next_offline_cpu(&self) -> Option<CpuId> {
        None
    }

    /// Request that one CPU be started.
    unsafe fn start_cpu(
        &self,
        _cpu: CpuId,
        _entry: extern "C" fn(usize) -> !,
        _arg: usize,
    ) -> Result<(), Errno> {
        Err(Errno::NotSupported)
    }

    /// Fill buffer with hardware entropy bytes.
    /// Returns the number of bytes actually filled (0 = no HW RNG available).
    fn fill_entropy(&self, _dst: &mut [u8]) -> usize {
        0
    }

    fn phys_to_virt_offset(&self) -> u64;

    /// Read the current thread's user TLS base from hardware (FS_BASE on x86_64).
    /// Returns 0 on architectures without a dedicated user TLS register.
    fn get_user_tls_base_dyn(&self) -> u64 {
        0
    }

    /// Write a new user TLS base to hardware immediately (FS_BASE on x86_64).
    /// No-op on architectures without a dedicated user TLS register.
    fn set_user_tls_base_dyn(&self, _base: u64) {}

    /// Returns `true` if the current CPU is executing the idle task.
    fn is_idle_task_current(&self) -> bool {
        false
    }

    /// Sets the idle task state for the current CPU.
    fn set_idle_task_current(&self, _idle: bool) {}

    /// Activates the onscreen terminal if supported by the runtime.
    fn activate_onscreen_terminal(&self) {}

    /// Flush deferred console output during idle time.
    ///
    /// Called from the scheduler idle loop when no tasks are runnable.
    /// Implementations should drain more of the console ring buffer than
    /// the timer ISR does, since idle time is free.
    fn idle_flush_console(&self) {}
}

pub trait BootRuntime: BootRuntimeBase + Sized + 'static {
    type Tasking: BootTasking<Runtime = Self>;
    fn tasking(&self) -> &Self::Tasking;

    fn halt(&self) -> !;

    fn threads_supported(&self) -> bool {
        false
    }
    // simd_init_cpu moved to BootRuntimeBase
    fn simd_state_layout(&self) -> (usize, usize) {
        (0, 1)
    }
    unsafe fn simd_save(&self, _dst: *mut u8) {}
    unsafe fn simd_restore(&self, _src: *const u8) {}

    /// Very early architecture initialization, called before any significant stack usage.
    /// Used for critical setup like switching stack modes on AArch64.
    /// Default implementation does nothing.
    unsafe fn early_init(&self) {}

    fn fence_full(&self) {}
    fn icache_invalidate(&self) {}

    // wait_for_interrupt moved to BootRuntimeBase
    // phys_to_virt_offset moved to BootRuntimeBase

    fn phys_memory_map(&self) -> &'static [PhysRange];
    fn modules(&self) -> &'static [BootModuleDesc];
    fn framebuffer(&self) -> Option<FramebufferInfo>;
    fn get_kernel_cmdline(&self) -> &'static str;

    fn page_size(&self) -> usize {
        4096
    }
    fn kernel_virt_base(&self) -> u64 {
        0xffffffff80000000
    }
    fn boot_cpu_id(&self) -> usize {
        0
    }
    fn cpu_ids(&self) -> &'static [CpuId] {
        const ONE: [CpuId; 1] = [CpuId(0)];
        &ONE
    }

    fn start_secondary_cpus(&self, _entry: extern "C" fn(usize) -> !) -> Result<(), Errno> {
        Err(Errno::NotSupported)
    }

    // irq_disable and irq_restore moved to BootRuntimeBase

    /// Setup periodic preemption timer (e.g. 100Hz heartbeat)
    fn setup_preemption_timer(&self, _hz: u32) {}

    fn acpi_rsdp(&self) -> Option<u64> {
        None
    }
    fn dtb_ptr(&self) -> Option<u64> {
        None
    }

    // IO Port primitives (x86-only, stubs for other archs)
    fn ioport_read_u8(&self, _port: u16) -> u8 {
        0
    }
    fn ioport_read_u16(&self, _port: u16) -> u16 {
        0
    }
    fn ioport_read_u32(&self, _port: u16) -> u32 {
        0
    }
    fn ioport_write_u8(&self, _port: u16, _value: u8) {}
    fn ioport_write_u16(&self, _port: u16, _value: u16) {}
    fn ioport_write_u32(&self, _port: u16, _value: u32) {}

    fn debug_active_aspace_root(&self) -> u64 {
        0
    }

    /// Map a physical range into a temporary virtual address for boot-time copies.
    /// This is used for reading firmware tables that might not be in the HHDM.
    /// Returns the virtual address of the start of the range.
    fn map_phys_temp(&self, _phys: u64, _size: usize) -> Result<u64, Errno> {
        Err(Errno::NotSupported)
    }

    /// Unmap a previously mapped temporary physical range.
    fn unmap_phys_temp(&self, _virt: u64, _size: usize) {}
}

// Per-CPU generic tracking
// In a full implementation, this should be a per-cpu structure or array.
// For now, we only trust this for the boot CPU or rely on atomic updates.
static CPU_ONLINE: once_cell::OnceCell<&'static core::sync::atomic::AtomicUsize> =
    once_cell::OnceCell::new();

static RUNTIME: once_cell::OnceCell<&'static dyn core::any::Any> = once_cell::OnceCell::new();
static RUNTIME_BASE: once_cell::OnceCell<&'static dyn BootRuntimeBase> = once_cell::OnceCell::new();
static mut RAW_RUNTIME_BASE: Option<&'static dyn BootRuntimeBase> = None;

/// Initialize the runtime. Panics if called more than once.
pub fn init_runtime<R: BootRuntime>(runtime: &'static R) {
    RUNTIME.set(runtime as &'static dyn core::any::Any);
    RUNTIME_BASE.set(runtime as &'static dyn BootRuntimeBase);
    unsafe {
        RAW_RUNTIME_BASE = Some(runtime as &'static dyn BootRuntimeBase);
    }
    boot_trace(runtime, b"[kernel:init_runtime] runtime refs set\r\n");
}

pub fn runtime<R: BootRuntime>() -> &'static R {
    let any_ref: &'static dyn core::any::Any = *RUNTIME.get();
    if let Some(rt) = any_ref.downcast_ref::<R>() {
        rt
    } else {
        panic!("Runtime type mismatch: expected {}", core::any::type_name::<R>());
    }
}

pub fn runtime_base() -> &'static dyn BootRuntimeBase {
    *RUNTIME_BASE.get()
}

/// Returns `true` if the runtime has been initialized.
///
/// Safe to call from any context (including early boot and unit tests).
/// Code that cannot tolerate a panic on `runtime_base()` should guard with this.
pub fn is_runtime_initialized() -> bool {
    RUNTIME_BASE.is_initialized()
}

// Global IO port accessor functions
// On x86, these use inline asm. On other archs, they are no-ops.
#[inline]
pub fn ioport_read_u8(_port: u16) -> u8 {
    #[cfg(target_arch = "x86_64")]
    {
        if _port == 0x60 {
            if let Some(byte) = crate::irq::ps2::take_scancode() {
                return byte;
            }
        }

        let val: u8;
        unsafe {
            core::arch::asm!("in al, dx", out("al") val, in("dx") _port, options(nostack, preserves_flags))
        };
        if _port == 0x64 { crate::irq::ps2::overlay_status(val) } else { val }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        0
    }
}

#[inline]
pub fn ioport_read_u16(_port: u16) -> u16 {
    #[cfg(target_arch = "x86_64")]
    {
        let val: u16;
        unsafe {
            core::arch::asm!("in ax, dx", out("ax") val, in("dx") _port, options(nostack, preserves_flags))
        };
        val
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        0
    }
}

#[inline]
pub fn ioport_read_u32(_port: u16) -> u32 {
    #[cfg(target_arch = "x86_64")]
    {
        let val: u32;
        unsafe {
            core::arch::asm!("in eax, dx", out("eax") val, in("dx") _port, options(nostack, preserves_flags))
        };
        val
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        0
    }
}

#[inline]
pub fn ioport_write_u8(_port: u16, _val: u8) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!("out dx, al", in("dx") _port, in("al") _val, options(nostack, preserves_flags))
    };
}

#[inline]
pub fn ioport_write_u16(_port: u16, _val: u16) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!("out dx, ax", in("dx") _port, in("ax") _val, options(nostack, preserves_flags))
    };
}

#[inline]
pub fn ioport_write_u32(_port: u16, _val: u32) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!("out dx, eax", in("dx") _port, in("eax") _val, options(nostack, preserves_flags))
    };
}

struct GlobalAllocHook;
impl FrameAllocatorHook for GlobalAllocHook {
    fn alloc_frame(&self) -> Option<u64> {
        crate::memory::alloc_frame()
    }
}

fn _irq_disable_wrapper<R: BootRuntime>() -> IrqState {
    runtime::<R>().irq_disable()
}
fn _irq_restore_wrapper<R: BootRuntime>(state: IrqState) {
    runtime::<R>().irq_restore(state);
}

#[inline]
/// Convert monotonic timer ticks to microseconds for a given timer frequency.
/// Returns 0 when `hz` is 0 to avoid divide-by-zero during early boot.
fn boot_timing_us_from_hz(ticks: u64, hz: u64) -> u64 {
    if hz == 0 { 0 } else { ((ticks as u128).saturating_mul(1_000_000) / hz as u128) as u64 }
}

#[inline]
/// Log elapsed timing for a scheduler-entry boot step.
/// Reports both per-step elapsed time and cumulative elapsed time since
/// `scheduler_entry_window_start`.
fn log_scheduler_entry_step<R: BootRuntime>(
    runtime: &R,
    boot_timing_hz: u64,
    scheduler_entry_window_start: u64,
    step: &str,
    step_start: u64,
) {
    let now = runtime.mono_ticks();
    // mono_ticks is a wrapping monotonic counter; wrapping_sub keeps elapsed
    // durations correct across counter rollover.
    let step_elapsed = now.wrapping_sub(step_start);
    let total_elapsed = now.wrapping_sub(scheduler_entry_window_start);
    let step_elapsed_us = boot_timing_us_from_hz(step_elapsed, boot_timing_hz);
    let total_elapsed_us = boot_timing_us_from_hz(total_elapsed, boot_timing_hz);
    crate::kdebug!(
        "[kernel:start] scheduler-entry step='{}' elapsed_ticks={} elapsed_us={} total_ticks={} total_us={}",
        step,
        step_elapsed,
        step_elapsed_us,
        total_elapsed,
        total_elapsed_us
    );
}

pub fn start<R: BootRuntime>(runtime: &'static R) -> ! {
    boot_trace(runtime, b"[kernel:start] enter\r\n");

    crate::irq::IRQ_DISABLE_HOOK
        .store(_irq_disable_wrapper::<R> as *mut (), core::sync::atomic::Ordering::SeqCst);
    crate::irq::IRQ_RESTORE_HOOK
        .store(_irq_restore_wrapper::<R> as *mut (), core::sync::atomic::Ordering::SeqCst);
    boot_trace(runtime, b"[kernel:start] irq hooks installed\r\n");

    init_runtime(runtime);
    boot_trace(runtime, b"[kernel:start] init_runtime ok\r\n");

    let early_fb = runtime.framebuffer();
    if let Some(fb) = early_fb {
        crate::boot_progress::init(fb);
        crate::boot_progress::push(crate::boot_progress::BootPhase::Framebuffer, "Framebuffer Initialized");
    }

    if let Some(level) = parse_cmdline_loglevel(runtime.get_kernel_cmdline()) {
        crate::logging::set_log_level(level);
    }

    unsafe { crate::logging::init(runtime) };
    boot_trace(runtime, b"[kernel:start] logging init ok\r\n");

    boot_trace(runtime, b"[kernel:start] framebuffer query begin\r\n");
    let fb_opt = runtime.framebuffer();
    boot_trace(runtime, b"[kernel:start] framebuffer query ok\r\n");
    if let Some(fb) = fb_opt {
        let _ = fb;
        boot_trace(runtime, b"[kernel:start] framebuffer detected\r\n");
        // paint_bootfb_probe(fb);
    }

    boot_trace(runtime, b"[kernel:start] pre-memory contract point\r\n");

    boot_trace(runtime, b"[kernel:start] memory::init\r\n");
    memory::init(runtime);
    boot_trace(runtime, b"[kernel:start] memory::init ok\r\n");
    crate::boot_progress::push(crate::boot_progress::BootPhase::Memory, "Memory Map OK");
    boot_trace(runtime, b"[kernel:start] after memory::init marker\r\n");
    boot_trace(runtime, b"[kernel:start] kinfo(global_alloc) begin\r\n");
    kinfo!("Initializing global allocator...");
    boot_trace(runtime, b"[kernel:start] kinfo(global_alloc) ok\r\n");
    boot_trace(runtime, b"[kernel:start] global_alloc::init\r\n");
    memory::global_alloc::init(runtime);
    boot_trace(runtime, b"[kernel:start] global_alloc::init ok\r\n");
    crate::boot_progress::push(crate::boot_progress::BootPhase::Allocator, "Global Allocator");
    boot_trace(runtime, b"[kernel:start] after global_alloc marker\r\n");
    let boot_timing_hz = runtime.mono_freq_hz();
    let boot_timing_us = |ticks: u64| -> u64 { boot_timing_us_from_hz(ticks, boot_timing_hz) };

    boot_trace(runtime, b"[kernel:start] framebuffer/devfs begin\r\n");
    boot_trace(runtime, b"[kernel:start] framebuffer/devfs query begin\r\n");
    if let Some(fb) = runtime.framebuffer() {
        boot_trace(runtime, b"[kernel:start] framebuffer/devfs have fb\r\n");
        let fb_resource_id = 0xFB00_0000;

        boot_trace(runtime, b"[kernel:start] framebuffer/devfs registry lock begin\r\n");
        {
            let mut reg = crate::device_registry::REGISTRY.lock();
            boot_trace(runtime, b"[kernel:start] framebuffer/devfs registry lock ok\r\n");
            let mut bars = [0; 6];
            let mut sizes = [0; 6];

            // CRITICAL: fb.addr from the runtime depends on the bootloader/arch,
            // but is typically a kernel virtual address (HHDM).
            // device_registry expects PHYSICAL addresses for BARs.
            let ph_offset = runtime.phys_to_virt_offset();
            let phys_addr = if fb.addr >= ph_offset { fb.addr - ph_offset } else { fb.addr };
            boot_trace(runtime, b"[kernel:start] framebuffer/devfs phys addr ok\r\n");

            bars[0] = phys_addr;
            sizes[0] = fb.byte_len as u64;
            boot_trace(runtime, b"[kernel:start] framebuffer/devfs registry register begin\r\n");
            reg.register(crate::device_registry::DeviceEntry::new_mmio(
                "display_fb",
                fb_resource_id,
                bars,
                sizes,
            ));
            boot_trace(runtime, b"[kernel:start] framebuffer/devfs registry register ok\r\n");
        }
        boot_trace(runtime, b"[kernel:start] framebuffer/devfs registry scope done\r\n");

        boot_trace(runtime, b"[kernel:start] framebuffer/devfs set_boot_fb begin\r\n");
        let set_boot_fb_start = runtime.mono_ticks();
        crate::vfs::devfs::set_boot_fb(fb, fb_resource_id);
        let set_boot_fb_elapsed = runtime.mono_ticks().wrapping_sub(set_boot_fb_start);
        crate::kdebug!(
            "[kernel:start] framebuffer/devfs set_boot_fb elapsed_ticks={} elapsed_us={}",
            set_boot_fb_elapsed,
            boot_timing_us(set_boot_fb_elapsed)
        );
        boot_trace(runtime, b"[kernel:start] framebuffer/devfs set_boot_fb ok\r\n");
        boot_trace(runtime, b"[kernel:start] framebuffer/devfs fb0 node new begin\r\n");
        let fb0_node_new_start = runtime.mono_ticks();
        let fb0_node = alloc::sync::Arc::new(crate::vfs::devfs::FbNode::new(fb, fb_resource_id));
        let fb0_node_new_elapsed = runtime.mono_ticks().wrapping_sub(fb0_node_new_start);
        crate::kdebug!(
            "[kernel:start] framebuffer/devfs FbNode::new elapsed_ticks={} elapsed_us={}",
            fb0_node_new_elapsed,
            boot_timing_us(fb0_node_new_elapsed)
        );
        boot_trace(runtime, b"[kernel:start] framebuffer/devfs fb0 node new ok\r\n");
        boot_trace(runtime, b"[kernel:start] framebuffer/devfs register fb0 begin\r\n");
        let register_fb0_start = runtime.mono_ticks();
        crate::vfs::devfs::register("fb0", fb0_node);
        let register_fb0_elapsed = runtime.mono_ticks().wrapping_sub(register_fb0_start);
        crate::kdebug!(
            "[kernel:start] framebuffer/devfs register fb0 elapsed_ticks={} elapsed_us={}",
            register_fb0_elapsed,
            boot_timing_us(register_fb0_elapsed)
        );
        boot_trace(runtime, b"[kernel:start] framebuffer/devfs register fb0 ok\r\n");
    } else {
        boot_trace(runtime, b"[kernel:start] framebuffer/devfs no fb\r\n");
    }
    boot_trace(runtime, b"[kernel:start] framebuffer/devfs ok\r\n");
    crate::boot_progress::push(crate::boot_progress::BootPhase::Devices, "Display Registry");

    boot_trace(runtime, b"[kernel:start] kinfo(simd) begin\r\n");
    kinfo!("Initializing SIMD...");
    boot_trace(runtime, b"[kernel:start] kinfo(simd) ok\r\n");
    runtime.simd_init_cpu();
    boot_trace(runtime, b"[kernel:start] simd init ok\r\n");
    crate::boot_progress::push(crate::boot_progress::BootPhase::Compute, "SIMD Ready");

    boot_trace(runtime, b"[kernel:start] kdebug(entropy) begin\r\n");
    kdebug!("Seeding entropy pool...");
    boot_trace(runtime, b"[kernel:start] kdebug(entropy) ok\r\n");
    let entropy_seed_start = runtime.mono_ticks();
    crate::entropy::seed_from_hardware();
    let entropy_seed_elapsed = runtime.mono_ticks().wrapping_sub(entropy_seed_start);
    crate::kdebug!(
        "[kernel:start] entropy::seed_from_hardware elapsed_ticks={} elapsed_us={}",
        entropy_seed_elapsed,
        boot_timing_us(entropy_seed_elapsed)
    );
    boot_trace(runtime, b"[kernel:start] entropy seeded\r\n");
    crate::boot_progress::push(crate::boot_progress::BootPhase::Compute, "SIMD Ready");

    boot_trace(runtime, b"[kernel:start] kinfo(task) begin\r\n");
    kinfo!("Initializing tasking...");
    boot_trace(runtime, b"[kernel:start] kinfo(task) ok\r\n");
    crate::task::init::<R>();
    boot_trace(runtime, b"[kernel:start] task init ok\r\n");
    crate::boot_progress::push(crate::boot_progress::BootPhase::Cpu, "Tasking Initialized");
    let scheduler_entry_window_start = runtime.mono_ticks();

    boot_trace(runtime, b"[kernel:start] kdebug(vfs) begin\r\n");
    kdebug!("Initializing VFS...");
    boot_trace(runtime, b"[kernel:start] kdebug(vfs) ok\r\n");
    let set_cmdline_start = runtime.mono_ticks();
    crate::vfs::devfs::set_cmdline(runtime.get_kernel_cmdline().to_string());
    log_scheduler_entry_step(
        runtime,
        boot_timing_hz,
        scheduler_entry_window_start,
        "set_cmdline",
        set_cmdline_start,
    );
    boot_trace(runtime, b"[kernel:start] vfs::set_cmdline ok\r\n");
    let vfs_init_start = runtime.mono_ticks();
    crate::vfs::init(runtime.modules());
    log_scheduler_entry_step(
        runtime,
        boot_timing_hz,
        scheduler_entry_window_start,
        "vfs_init",
        vfs_init_start,
    );
    boot_trace(runtime, b"[kernel:start] vfs init ok\r\n");
    crate::boot_progress::push(crate::boot_progress::BootPhase::Vfs, "VFS Root Ready");

    boot_trace(runtime, b"[kernel:start] kdebug(pci) begin\r\n");
    kdebug!("Scanning PCI bus...");
    boot_trace(runtime, b"[kernel:start] kdebug(pci) ok\r\n");
    let pci_scan_start = runtime.mono_ticks();
    scan_pci();
    log_scheduler_entry_step(
        runtime,
        boot_timing_hz,
        scheduler_entry_window_start,
        "scan_pci",
        pci_scan_start,
    );
    boot_trace(runtime, b"[kernel:start] pci scan ok\r\n");
    crate::boot_progress::push(crate::boot_progress::BootPhase::Pci, "PCI Bus Scanned");

    // Register legacy ISA devices
    boot_trace(runtime, b"[kernel:start] legacy device register begin\r\n");
    let register_legacy_devices_start = runtime.mono_ticks();
    {
        let mut reg = crate::device_registry::REGISTRY.lock();
        // RTC CMOS (0x70, 0x71)
        reg.register(crate::device_registry::DeviceEntry::new_legacy(
            "rtc_cmos",
            crate::device_registry::CMOS_IOPORT_RANGES,
            0x70, // Port base as unique-ish ID
        ));
        // PS/2 Controller (0x60, 0x64)
        reg.register(crate::device_registry::DeviceEntry::new_legacy(
            "ps2_controller",
            crate::device_registry::PS2_IOPORT_RANGES,
            0x60,
        ));
        // Legacy ISA IDE controller: primary (0x1F0) + secondary (0x170) channels.
        // Exposed as `isa-01f0` in sysfs with kind="dev.storage.ata" so cambium
        // can auto-discover and spawn the ata_disk userland driver.
        reg.register(crate::device_registry::DeviceEntry::new_legacy(
            "dev.storage.ata",
            crate::device_registry::ATA_LEGACY_IOPORT_RANGES,
            0x1F0,
        ));
    }
    log_scheduler_entry_step(
        runtime,
        boot_timing_hz,
        scheduler_entry_window_start,
        "register_legacy_devices",
        register_legacy_devices_start,
    );
    boot_trace(runtime, b"[kernel:start] legacy device register ok\r\n");
    crate::boot_progress::push(crate::boot_progress::BootPhase::Devices, "Legacy Devices");

    // CRITICAL: Calibrate the BSP preemption timer BEFORE starting secondary CPUs.
    // Secondary CPUs read timer_vector/timer_init_cnt in init_secondary_cpu().
    // If these aren't set yet, secondary CPUs get no LAPIC timer, meaning
    // wake_sleepers() (called only from on_tick → PreemptTick) never fires
    // on those CPUs, and any task that calls sleep_ms() is stuck forever.
    boot_trace(runtime, b"[kernel:start] kdebug(preemption timer) begin\r\n");
    kdebug!("System initialized. Setting up preemption timer (100Hz)...");
    boot_trace(runtime, b"[kernel:start] kdebug(preemption timer) ok\r\n");
    let setup_preemption_timer_start = runtime.mono_ticks();
    runtime.setup_preemption_timer(100);
    log_scheduler_entry_step(
        runtime,
        boot_timing_hz,
        scheduler_entry_window_start,
        "setup_preemption_timer",
        setup_preemption_timer_start,
    );
    boot_trace(runtime, b"[kernel:start] preemption timer ok\r\n");
    crate::boot_progress::push(crate::boot_progress::BootPhase::Cpu, "BSP Timer OK");

    // Bring up all secondary CPUs during early boot.
    boot_trace(runtime, b"[kernel:start] cpu_total_count begin\r\n");
    let cpu_total = runtime.cpu_total_count();
    boot_trace(runtime, b"[kernel:start] cpu_total_count ok\r\n");
    let smp_bringup_start = runtime.mono_ticks();
    if cpu_total > 1 {
        boot_trace(runtime, b"[kernel:start] smp start_secondary begin\r\n");
        crate::kdebug!(
            "Kernel: Detected {} CPUs. Starting {} secondaries...",
            cpu_total,
            cpu_total - 1
        );
        match runtime.start_secondary_cpus(kernel_secondary_entry::<R>) {
            Ok(()) => crate::kdebug!("Kernel: Secondary CPU bring-up complete."),
            Err(err) => crate::kerror!("Kernel: Secondary CPU bring-up failed: {:?}", err),
        }
        boot_trace(runtime, b"[kernel:start] smp start_secondary ok\r\n");
    } else {
        crate::kdebug!("Kernel: Detected {} CPU.", cpu_total);
    }
    log_scheduler_entry_step(
        runtime,
        boot_timing_hz,
        scheduler_entry_window_start,
        "start_secondary_cpus",
        smp_bringup_start,
    );
    boot_trace(runtime, b"[kernel:start] smp bring-up stage done\r\n");
    crate::boot_progress::push(crate::boot_progress::BootPhase::Cpu, "SMP Bring-up");

    // Store global boot info for syscalls
    crate::boot_info::set(crate::boot_info::BootSyscallInfo {
        memory_map: runtime.phys_memory_map(),
        modules: runtime.modules(),
        framebuffer: runtime.framebuffer(),
        hhdm_offset: runtime.phys_to_virt_offset(),
        acpi_rsdp: runtime.acpi_rsdp(),
        dtb_ptr: runtime.dtb_ptr(),
    });
    boot_trace(runtime, b"[kernel:start] boot_info set\r\n");
    crate::boot_progress::push(crate::boot_progress::BootPhase::Modules, "Boot Info OK");

    boot_trace(runtime, b"[kernel:start] modules enumerate begin\r\n");
    let modules = runtime.modules();
    boot_trace(runtime, b"[kernel:start] runtime.modules enumerate ok\r\n");
    kdebug!("Kernel: Enumerating {} boot modules...", modules.len());
    // Only iterate and format individual module entries when trace logging is
    // actually enabled; skipping this loop at debug level avoids ~108 function
    // calls and atomic reads that add measurable overhead during boot.
    if crate::logging::get_log_level() >= 5 {
        for (i, m) in modules.iter().enumerate() {
            crate::ktrace!(
                "  Module[{}]: name='{}' cmdline='{}' size={} bytes",
                i,
                m.name,
                m.cmdline,
                m.bytes.len()
            );
        }
    }
    boot_trace(runtime, b"[kernel:start] modules enumerate loop ok\r\n");
    crate::boot_progress::push(crate::boot_progress::BootPhase::Modules, "Modules Scanned");

    // Find unifont.hex and pass it to boot_progress
    if let Some(unifont_mod) = modules.iter().find(|m| m.name.contains("unifont.hex")) {
        crate::boot_progress::set_unifont_data(unifont_mod.bytes);
    }

    // Look for module with "init" in cmdline, otherwise fallback to "sprout" by name
    let init_module = modules
        .iter()
        .find(|m| m.cmdline.contains("init"))
        .or_else(|| modules.iter().find(|m| m.name.contains("sprout")));
    boot_trace(runtime, b"[kernel:start] init module selection ok\r\n");

    if let Some(mod_desc) = init_module {
        boot_trace(runtime, b"[kernel:start] init module found\r\n");
        kdebug!(
            "Found init module: {} (cmdline: '{}'), loading...",
            mod_desc.name,
            mod_desc.cmdline
        );

        let aspace = runtime.tasking().make_user_address_space();
        let _hook = GlobalAllocHook;

        // Load Sprout
        let load_module_start = runtime.mono_ticks();
        let (user_entry, stack_info, mut regions, _aux_info) =
            crate::task::loader::load_module(runtime, aspace, mod_desc)
                .expect("Failed to load sprout");
        log_scheduler_entry_step(
            runtime,
            boot_timing_hz,
            scheduler_entry_window_start,
            "load_init_module",
            load_module_start,
        );

        // Prepare Module Registry Page
        let boot_registry_prepare_start = runtime.mono_ticks();
        let reg_phys = crate::memory::alloc_frame().expect("OOM Registry");
        let reg_virt = reg_phys + runtime.phys_to_virt_offset();

        // Fill registry
        unsafe {
            let count = modules.len();
            let ptr = reg_virt as *mut usize;
            *ptr = count; // First word = count

            // Array of ModuleEntry { ptr: usize, len: usize } starts at offset 8 (64-bit)
            // Strings start after array. Max 16 modules typical, but let's calculate.
            // entry_size = 16 bytes.
            let array_start = ptr.add(1) as *mut usize;
            let mut string_offset_bytes = 8 + (count * 16);

            for (i, m) in modules.iter().enumerate() {
                let name_bytes = m.name.as_bytes();
                let name_len = name_bytes.len();

                // Safety check for page overflow
                if string_offset_bytes + name_len > 4096 {
                    kdebug!("Warning: Module registry page overflow, truncating list.");
                    *ptr = i; // Update count
                    break;
                }

                // Copy string
                let string_dst = (reg_virt as *mut u8).add(string_offset_bytes);
                core::ptr::copy_nonoverlapping(name_bytes.as_ptr(), string_dst, name_len);

                // Write Entry (ptr, len)
                let entry_slot = array_start.add(i * 2);
                *entry_slot = 0x600000 + string_offset_bytes; // User virtual address
                *entry_slot.add(1) = name_len;

                string_offset_bytes += name_len;
            }
        }
        log_scheduler_entry_step(
            runtime,
            boot_timing_hz,
            scheduler_entry_window_start,
            "prepare_boot_registry",
            boot_registry_prepare_start,
        );

        // Map Registry to fixed user address 0x600000
        // We map it read-only for user
        let boot_registry_map_start = runtime.mono_ticks();
        runtime
            .tasking()
            .map_page(
                aspace,
                0x600000,
                reg_phys,
                MapPerms {
                    user: true,
                    read: true,
                    write: false,
                    exec: false,
                    kind: MapKind::Normal,
                },
                &GlobalAllocHook,
            )
            .unwrap();
        log_scheduler_entry_step(
            runtime,
            boot_timing_hz,
            scheduler_entry_window_start,
            "map_boot_registry",
            boot_registry_map_start,
        );

        regions.push(VmRegionInfo {
            start: 0x600000,
            end: 0x601000,
            prot: VmProt::USER | VmProt::READ,
            flags: VmMapFlags::empty(),
            backing_kind: VmBackingKind::Unknown,
            _reserved: [0; 7],
        });

        // Flush TLB by reloading CR3
        let activate_aspace_start = runtime.mono_ticks();
        runtime.tasking().activate_address_space(aspace);
        log_scheduler_entry_step(
            runtime,
            boot_timing_hz,
            scheduler_entry_window_start,
            "activate_init_aspace",
            activate_aspace_start,
        );

        kdebug!("Spawning sprout with registry at 0x600000...");
        let spawn_init_start = runtime.mono_ticks();
        unsafe {
            kdebug!("Spawning init process...");
            let mut entry = user_entry;
            entry.arg0 = StartupArg::BootRegistry.to_raw(); // arg0 = registry ptr
            // Spawn at Normal priority - all tasks share the same priority for fair scheduling
            crate::sched::spawn_user_task_full::<R>(
                entry,
                aspace,
                stack_info,
                regions,
                crate::task::TaskPriority::Normal,
            );
        }
        log_scheduler_entry_step(
            runtime,
            boot_timing_hz,
            scheduler_entry_window_start,
            "spawn_init_task",
            spawn_init_start,
        );
        boot_trace(runtime, b"[kernel:start] init process spawned\r\n");
        crate::boot_progress::push(crate::boot_progress::BootPhase::Init, "Spawning Sprout");
    } else {
        boot_trace(runtime, b"[kernel:start] no init module; fallback path\r\n");
        kdebug!("Sprout not found. Checking fallback...");

        let spawned_fallback = false;
        #[cfg(feature = "diagnostic-apps")]
        {
            if let Some(mod_desc) = modules.iter().find(|m| m.name.contains("threads_demo")) {
                kdebug!("Found threads_demo fallback...");
                let aspace = runtime.tasking().make_user_address_space();
                let load_fallback_module_start = runtime.mono_ticks();
                let (user_entry, stack_info, regions) =
                    crate::task::loader::load_module(runtime, aspace, mod_desc)
                        .expect("Failed to load threads_demo");
                log_scheduler_entry_step(
                    runtime,
                    boot_timing_hz,
                    scheduler_entry_window_start,
                    "load_fallback_module",
                    load_fallback_module_start,
                );
                let spawn_fallback_start = runtime.mono_ticks();
                unsafe {
                    crate::sched::spawn_user_task_full::<R>(
                        user_entry,
                        aspace,
                        stack_info,
                        regions,
                        crate::task::TaskPriority::Normal,
                    );
                }
                log_scheduler_entry_step(
                    runtime,
                    boot_timing_hz,
                    scheduler_entry_window_start,
                    "spawn_fallback_task",
                    spawn_fallback_start,
                );
                spawned_fallback = true;
            }
        }

        if !spawned_fallback {
            kdebug!("No modules found. Checking threads_supported...");
            if runtime.threads_supported() {
                kdebug!("Spawning initial threads...");
                kdebug!("Spawning Thread A...");
                crate::task::spawn::<R>(
                    thread_a,
                    StartupArg::Raw(1),
                    crate::task::TaskPriority::Normal,
                    crate::task::Affinity::Any,
                );
                kdebug!("Spawning Thread B...");
                crate::task::spawn::<R>(
                    thread_b,
                    StartupArg::Raw(2),
                    crate::task::TaskPriority::Normal,
                    crate::task::Affinity::Any,
                );
            }
        }
    }

    // Transition out of early-boot mode.
    let end_bringup_start = runtime.mono_ticks();
    crate::sched::end_bringup::<R>();
    log_scheduler_entry_step(
        runtime,
        boot_timing_hz,
        scheduler_entry_window_start,
        "end_bringup",
        end_bringup_start,
    );
    boot_trace(runtime, b"[kernel:start] end_bringup\r\n");
    crate::boot_progress::push(crate::boot_progress::BootPhase::Scheduler, "Entering Scheduler");
    crate::boot_progress::finish();
    let scheduler_entry_total = runtime.mono_ticks().wrapping_sub(scheduler_entry_window_start);
    crate::kinfo!(
        "[kernel:start] scheduler-entry total elapsed_ticks={} elapsed_us={}",
        scheduler_entry_total,
        boot_timing_us(scheduler_entry_total)
    );

    kinfo!("Entering scheduler loop.");
    boot_trace(runtime, b"[kernel:start] kinfo(scheduler loop) ok\r\n");
    boot_trace(runtime, b"[kernel:start] scheduler loop\r\n");
    static PAINTED: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);

    loop {
        if !crate::task::yield_now::<R>() {
            // First idle period: perform the deferred bootfb paint instrumentation
            // if we haven't done it yet. This provides a landmark for BDD latency tests.
            if !PAINTED.swap(true, core::sync::atomic::Ordering::SeqCst) {
                let elapsed = if let Some(_fb) = runtime.framebuffer() {
                    let start_ticks = runtime.mono_ticks();

                    // Simple gradient to verify framebuffer access
                    // DISABLED
                    /*for y in 0..fb.height as usize {
                        for x in 0..fb.width as usize {
                            let r = (x * 255 / fb.width as usize) as u8;
                            let g = (y * 255 / fb.height as usize) as u8;
                            let b = 128u8;
                            let color = (r as u32) << 16 | (g as u32) << 8 | (b as u32);
                            let addr = fb.addr + (y * fb.pitch as usize + x * 4) as u64;
                            unsafe {
                                *(addr as *mut u32) = color;
                            }
                        }
                    }*/

                    runtime.mono_ticks().wrapping_sub(start_ticks)
                } else {
                    0
                };
                kinfo!("deferred_bootfb_gradient elapsed_ticks={}", elapsed);
            }

            // No runnable work on this CPU — halt until the next interrupt
            // (timer tick, IPI, or device IRQ).  This is the same idle pattern
            // used by secondary CPUs in `run_scheduler` and prevents CPU 0 from
            // spinning in a tight loop that continuously acquires the scheduler
            // lock, which was the primary source of try_lock miss warnings under
            // SMP.
            crate::runtime_base().idle_flush_console();
            crate::runtime::<R>().wait_for_interrupt();
        }
    }
}

extern "C" fn thread_a(arg: usize) -> ! {
    let mut count: usize = 0;
    loop {
        // Only log the first few iterations to avoid flooding serial output
        if count < 5 {
            let ticks = runtime_base().mono_ticks();
            crate::kdebug!("Thread A (arg={}) ticks={}", arg, ticks);
        }
        count = count.wrapping_add(1);
        for _ in 0..1000000 {
            core::hint::black_box(());
        }
        unsafe {
            crate::sched::yield_now_current();
        }
    }
}

extern "C" fn thread_b(arg: usize) -> ! {
    let mut count: usize = 0;
    loop {
        // Only log the first few iterations to avoid flooding serial output
        if count < 5 {
            let ticks = runtime_base().mono_ticks();
            crate::kdebug!("Thread B (arg={}) ticks={}", arg, ticks);
        }
        count = count.wrapping_add(1);
        for _ in 0..1000000 {
            core::hint::black_box(());
        }
        unsafe {
            crate::sched::yield_now_current();
        }
    }
}

pub mod boot_info;

extern "C" fn kernel_secondary_entry<R: BootRuntime>(cpu_index: usize) -> ! {
    // CRITICAL: First, load the kernel's GDT/IDT and set GS_BASE on this secondary CPU
    // This must happen before ANY kernel code that might fault or use logging (which uses GS).
    let base = unsafe { RAW_RUNTIME_BASE.expect("RAW_RUNTIME_BASE not initialized") };
    base.init_secondary_cpu(cpu_index);
    crate::kdebug!(
        "SMP: kernel_secondary_entry arg_cpu={} runtime_cpu={}",
        cpu_index,
        base.current_cpu_index()
    );
    // Verification done via base properties later if needed

    crate::kdebug!("SMP: Entering kernel_secondary_entry for CPU {}", cpu_index);

    // Per-CPU init
    base.mono_ticks(); // ok for logging
    // IMPORTANT: per-CPU SIMD init
    base.simd_init_cpu();

    // Then:
    unsafe {
        crate::sched::cpu_online::<R>(cpu_index);
        crate::sched::enter_secondary(cpu_index);
    }
}

pub fn scan_pci() {
    let rt = runtime_base();
    let mut reg = crate::device_registry::REGISTRY.lock();

    for bus in 0..16 {
        // Bus range restricted for speed in QEMU
        for dev in 0..32 {
            for func in 0..8 {
                let vendor_device = match rt.pci_cfg_read32(bus, dev, func, 0x00) {
                    Ok(val) => val,
                    Err(_) => 0xFFFFFFFF,
                };
                let vendor_id = (vendor_device & 0xFFFF) as u16;
                let device_id = (vendor_device >> 16) as u16;

                if vendor_id == 0xFFFF {
                    if func == 0 {
                        break;
                    } // Next device
                    continue; // Next function
                }

                // Enable Memory Space (bit 1) and Bus Mastering (bit 2)
                let cmd = rt.pci_cfg_read32(bus, dev, func, 0x04).unwrap_or(0);
                let _ = rt.pci_cfg_write32(bus, dev, func, 0x04, cmd | 0x06);

                let class_rev = rt.pci_cfg_read32(bus, dev, func, 0x08).unwrap_or(0);
                let class_code = (class_rev >> 24) as u8;
                let subclass = (class_rev >> 16) as u8;
                let prog_if = (class_rev >> 8) as u8;

                let header_type =
                    (rt.pci_cfg_read32(bus, dev, func, 0x0C).unwrap_or(0) >> 16) as u8;

                let mut bars = [0u64; 6];
                let mut sizes = [0u64; 6];

                // For simplicity, only scan BARs for header type 0 (normal devices)
                if (header_type & 0x7F) == 0 {
                    let mut i = 0;
                    while i < 6 {
                        let offset = 0x10 + (i * 4) as u8;
                        let bar = rt.pci_cfg_read32(bus, dev, func, offset).unwrap_or(0);
                        if bar != 0 {
                            // Check size by writing 0xFFFFFFFF
                            let _ = rt.pci_cfg_write32(bus, dev, func, offset, 0xFFFFFFFF);
                            let size_mask = rt.pci_cfg_read32(bus, dev, func, offset).unwrap_or(0);
                            let _ = rt.pci_cfg_write32(bus, dev, func, offset, bar);

                            if bar & 1 == 0 {
                                // Memory space
                                let is_64 = (bar & 0x4) != 0;
                                let mut final_bar = (bar & 0xFFFFFFF0) as u64;
                                let final_size_mask = if is_64 && i < 5 {
                                    let next_offset = offset + 4;
                                    let bar_hi =
                                        rt.pci_cfg_read32(bus, dev, func, next_offset).unwrap_or(0);
                                    let _ =
                                        rt.pci_cfg_write32(bus, dev, func, next_offset, 0xFFFFFFFF);
                                    let size_mask_hi =
                                        rt.pci_cfg_read32(bus, dev, func, next_offset).unwrap_or(0);
                                    let _ = rt.pci_cfg_write32(bus, dev, func, next_offset, bar_hi);

                                    final_bar |= (bar_hi as u64) << 32;
                                    (size_mask & 0xFFFFFFF0) as u64 | ((size_mask_hi as u64) << 32)
                                } else {
                                    (size_mask & 0xFFFFFFF0) as u64 | 0xFFFFFFFF_00000000
                                };

                                let size = (!final_size_mask).wrapping_add(1);
                                bars[i as usize] = final_bar;
                                sizes[i as usize] = size;

                                crate::ktrace!(
                                    "  BAR{} (MEM{}): 0x{:08x} (size 0x{:x})",
                                    i,
                                    if is_64 { "64" } else { "32" },
                                    final_bar,
                                    size
                                );

                                if is_64 {
                                    i += 1; // Skip next slot
                                }
                            } else {
                                // I/O space
                                let size = (!(size_mask & 0xFFFFFFFC)).wrapping_add(1) as u64;
                                bars[i as usize] = (bar & 0xFFFFFFFC) as u64;
                                sizes[i as usize] = size;
                                crate::ktrace!(
                                    "  BAR{} (I/O):  0x{:04x} (size 0x{:x})",
                                    i,
                                    bars[i as usize],
                                    size
                                );
                            }
                        }
                        i += 1;
                    }
                }

                let resource_id =
                    0x2000_0000 | ((bus as u64) << 16) | ((dev as u64) << 8) | (func as u64);

                let entry = crate::device_registry::DeviceEntry {
                    kind: "pci_device",
                    ioport_ranges: &[],
                    resource_id,
                    mmio_bars: bars,
                    mmio_sizes: sizes,
                    vendor_id,
                    device_id,
                    class_code,
                    subclass,
                    prog_if,
                    pci_location: Some(crate::device_registry::PciLocation { bus, dev, func }),
                    msi_cap: None,
                    msix_cap: None,
                    irq_mode: crate::device_registry::IrqMode::Legacy,
                    irq_vector: 0,
                };

                if let Some(idx) = reg.register(entry) {
                    crate::kdebug!(
                        "PCI: Discovered 0x{:04x}:0x{:04x} at {:02x}:{:02x}.{} class={:02x}{:02x}{:02x} id={}",
                        vendor_id,
                        device_id,
                        bus,
                        dev,
                        func,
                        class_code,
                        subclass,
                        prog_if,
                        idx
                    );
                }

                if func == 0 && (header_type & 0x80) == 0 {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod cmdline_loglevel_tests {
    use super::parse_cmdline_loglevel;

    #[test]
    fn parses_numeric_loglevel() {
        assert_eq!(parse_cmdline_loglevel("foo=bar loglevel=5"), Some(5));
        assert_eq!(parse_cmdline_loglevel("loglevel=0"), Some(0));
    }

    #[test]
    fn parses_named_loglevel() {
        assert_eq!(parse_cmdline_loglevel("loglevel=trace"), Some(5));
        assert_eq!(parse_cmdline_loglevel("loglevel=warn"), Some(2));
        assert_eq!(parse_cmdline_loglevel("loglevel=ERROR"), Some(1));
    }

    #[test]
    fn parses_space_separated_form() {
        assert_eq!(parse_cmdline_loglevel("display=bootfb loglevel debug"), Some(4));
    }

    #[test]
    fn ignores_invalid_values() {
        assert_eq!(parse_cmdline_loglevel("loglevel=9"), None);
        assert_eq!(parse_cmdline_loglevel("loglevel=verbose"), None);
    }

    #[test]
    fn last_valid_occurrence_wins() {
        assert_eq!(parse_cmdline_loglevel("loglevel=2 loglevel=trace"), Some(5));
    }
}
