use crate::runtime::ArchRuntime;
use core::arch::{asm, naked_asm};
use kernel::time::MonotonicClamp;
use kernel::{FrameAllocatorHook, IrqState, MapKind, MapPerms, UserEntry, UserTaskSpec};

pub mod paging;
pub mod simd;
pub mod syscall;
pub mod task;
pub mod trap;
pub mod vector;

struct DumbAlloc;
impl FrameAllocatorHook for DumbAlloc {
    fn alloc_frame(&self) -> Option<u64> {
        None
    }
}

static UART_MAPPED: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);

pub struct AArch64Runtime {
    serial: SerialPort,
}

impl AArch64Runtime {
    pub const fn new() -> Self {
        Self {
            serial: SerialPort::new(),
        }
    }
}

pub use paging::AArch64AddressSpace;
pub use task::AArch64Context;

/// Ensure execution is in EL1h (using SP_EL1) before normal kernel init.
///
/// Handles two boot scenarios:
///
/// 1. **Arriving at EL2** (some firmware/hypervisor configurations): sets
///    `HCR_EL2.RW=1` so that EL1 runs as AArch64, programs `SPSR_EL2` for
///    EL1h with D/A/I/F masked, sets `ELR_EL2` to the post-`eret`
///    continuation, and executes `eret` to drop to EL1h.
///
/// 2. **Already at EL1t** (SPSel=0): copies the current SP value into
///    `SP_EL1` and sets `SPSel=1`.
///
/// If `SPSel` is already 1 the function is a no-op (idempotent).
#[unsafe(naked)]
unsafe extern "C" fn switch_to_el1h() {
    naked_asm!(
        // Save current SP before any branching so both paths can initialize
        // SP_EL1 to a valid kernel stack address.
        "mov  x9, sp",

        // ── Detect current EL ───────────────────────────────────────────
        "mrs  x10, CurrentEL",
        "lsr  x10, x10, #2",   // bits [3:2] → x10
        "and  x10, x10, #3",   // isolate two-bit EL field
        "cmp  x10, #2",
        "bne  1f",             // not EL2 → already at EL1, go to common path

        // ── EL2 path ────────────────────────────────────────────────────
        // HCR_EL2.RW = 1  →  EL1/EL0 execute as AArch64.
        "mrs  x10, hcr_el2",
        "orr  x10, x10, #(1 << 31)",
        "msr  hcr_el2, x10",

        // SPSR_EL2:
        //   M[4:0] = 0b00101 = 5  (EL1h, using SP_EL1)
        //   D/A/I/F (bits [9:6]) = 0b1111  (all async aborts + IRQ + FIQ masked)
        //   → 0x3C5
        "mov  x10, #0x3C5",
        "msr  spsr_el2, x10",

        // ELR_EL2: resume at the common SP_EL1 setup block after eret.
        "adr  x10, 2f",
        "msr  elr_el2, x10",

        "isb",
        "eret",                // → EL1h, continues at label 2

        // ── EL1 path (fall-through) ──────────────────────────────────────
        "1:",
        // ── Common: install saved SP into SP_EL1 and select it ──────────
        "2:",
        "msr  spsel, #1",      // switch stack-pointer select to SP_EL1
        "mov  sp, x9",         // initialize SP_EL1 from the saved value
        "isb",
        "ret",
    );
}

impl ArchRuntime for AArch64Runtime {
    type Context = AArch64Context;
    type AddressSpace = AArch64AddressSpace;

    fn init(&self, hhdm_offset: u64) {
        unsafe {
            paging::init(hhdm_offset);
            vector::init();
            self.early_init();

            // Map UART MMIO into kernel space (HHDM)
            let uart_phys = 0x0900_0000u64;
            let uart_virt = uart_phys + hhdm_offset;
            let aspace = paging::active_address_space();

            let perms = MapPerms {
                read: true,
                write: true,
                exec: false,
                user: false,
                kind: MapKind::Device,
            };

            struct KernelAlloc;
            impl FrameAllocatorHook for KernelAlloc {
                fn alloc_frame(&self) -> Option<u64> {
                    kernel::memory::alloc_frame()
                }
            }

            let allocator: &dyn FrameAllocatorHook = if kernel::memory::is_frame_allocator_ready() {
                &KernelAlloc
            } else {
                &DumbAlloc
            };

            if self.map_page(aspace, uart_virt, uart_phys, perms, allocator).is_ok() {
                UART_MAPPED.store(true, core::sync::atomic::Ordering::Release);
            }
        }
    }

    fn putchar(&self, c: u8) {
        // 1. Semihosting fallback (always works, very slow)
        let ch = c;
        unsafe {
            core::arch::asm!(
                "hlt #0xF000",
                in("w0") 0x03,
                in("x1") &ch,
                options(nostack, preserves_flags)
            );
        }

        // 2. PL011 UART0 via HHDM (only if mapped)
        if UART_MAPPED.load(core::sync::atomic::Ordering::Acquire) {
            let hhdm = unsafe { paging::get_hhdm_offset() };
            let uart_base = 0x0900_0000u64 + hhdm;
            let uart = uart_base as *mut u32;

            unsafe {
                // UARTFR (Flag Register) offset 0x18 (6 * 4). TXFF is bit 5.
                let mut timeout = 1000u32;
                while (core::ptr::read_volatile(uart.add(6)) & 0x20) != 0 && timeout > 0 {
                    timeout -= 1;
                }
                if timeout > 0 {
                    core::ptr::write_volatile(uart, c as u32);
                }
            }
        }
    }
    // getchar: default None (semihosting has no standard getchar)

    fn halt(&self) -> ! {
        hcf()
    }

    fn reboot(&self) -> ! {
        // PSCI SYSTEM_RESET via HVC
        unsafe {
            asm!(
                "mov x0, {fid}",
                "hvc #0",
                fid = in(reg) 0x8400_0009u64,
                options(noreturn)
            );
        }
    }

    fn wait_for_interrupt(&self) {
        unsafe {
            core::arch::asm!("wfe", options(nomem, nostack));
        }
    }

    fn mono_ticks(&self) -> u64 {
        self.serial.clamp.clamp(read_cntvct_el0())
    }

    fn mono_freq_hz(&self) -> u64 {
        read_cntfrq_el0()
    }

    fn irq_disable(&self) -> IrqState {
        let daif: u64;
        unsafe {
            asm!("mrs {}, daif", out(reg) daif, options(nomem, nostack));
            asm!("msr daifset, #2", options(nomem, nostack));
        }
        IrqState((daif >> 7) as usize & 1)
    }

    fn irq_restore(&self, state: IrqState) {
        if state.0 == 0 {
            unsafe {
                asm!("msr daifclr, #2", options(nomem, nostack));
            }
        } else {
            unsafe {
                asm!("msr daifset, #2", options(nomem, nostack));
            }
        }
    }

    fn simd_init_cpu(&self) {
        simd::init_cpu();
    }
    fn simd_state_layout(&self) -> (usize, usize) {
        simd::STATE_LAYOUT
    }
    unsafe fn simd_save(&self, dst: *mut u8) {
        unsafe { simd::save(dst) }
    }
    unsafe fn simd_restore(&self, src: *const u8) {
        unsafe { simd::restore(src) }
    }

    unsafe fn early_init(&self) {
        unsafe {
            switch_to_el1h();
            // MAIR Index 0: Normal Memory (0xFF), Index 1: Device-nGnRE (0x04)
            let mair: u64 = 0xff04;
            asm!("msr mair_el1, {}", in(reg) mair);
        }
    }

    fn fence_full(&self) {
        unsafe {
            asm!("dmb sy", options(nostack, preserves_flags));
        }
    }

    fn icache_invalidate(&self) {
        unsafe {
            asm!("ic ialluis", options(nostack, preserves_flags));
            asm!("dsb ish", options(nostack, preserves_flags));
            asm!("isb", options(nostack, preserves_flags));
        }
    }

    fn threads_supported(&self) -> bool {
        true
    }

    // Tasking
    fn init_kernel_context(
        &self,
        entry: extern "C" fn(usize) -> !,
        stack_top: u64,
        arg: usize,
    ) -> Self::Context {
        task::init_kernel_context(entry, stack_top, arg)
    }

    fn init_user_context(
        &self,
        spec: UserTaskSpec<Self::AddressSpace>,
        kstack_top: u64,
    ) -> Self::Context {
        task::init_user_context(spec, kstack_top)
    }

    unsafe fn switch(&self, from: &mut Self::Context, to: &Self::Context, _to_tid: u64) {
        unsafe { task::switch(from, to) }
    }

    unsafe fn switch_with_tls(
        &self,
        from: &mut Self::Context,
        to: &Self::Context,
        to_tid: u64,
        from_user_fs_base: *mut u64,
        to_user_fs_base: u64,
    ) {
        unsafe {
            *from_user_fs_base = self.get_user_tls_base();
            self.set_user_tls_base(to_user_fs_base);
            task::switch(from, to);
        }
    }

    fn get_user_tls_base(&self) -> u64 {
        let base: u64;
        unsafe {
            asm!("mrs {}, tpidr_el0", out(reg) base, options(nomem, nostack));
        }
        base
    }

    fn set_user_tls_base(&self, base: u64) {
        unsafe {
            asm!("msr tpidr_el0, {}", in(reg) base, options(nomem, nostack));
        }
    }

    unsafe fn enter_user(&self, entry: UserEntry) -> ! {
        // Debug: Read current TTBR0
        let ttbr0: u64;
        unsafe {
            asm!("mrs {}, ttbr0_el1", out(reg) ttbr0, options(nomem, nostack));
        }
        kernel::kinfo!(
            "enter_user: TTBR0={:#x} entry_pc={:#x} user_sp={:#x}",
            ttbr0,
            entry.entry_pc,
            entry.user_sp
        );

        // Switch to EL1h (using SP_EL1) so we can safely set SP_EL0 for user mode.
        // We first save the current SP, then switch SPSel=1 and restore SP to SP_EL1.
        // After this, SP_EL0 can be safely written for the user task.
        //
        // SPSR: EL0t (mode 0), all interrupts unmasked
        let spsr: u64 = 0;
        let ksp: u64;
        unsafe {
            asm!("mov {}, sp", out(reg) ksp, options(nomem, nostack));
        }

        unsafe {
            asm!(
                "msr spsel, #1",
                "mov sp, {ksp}",
                "msr sp_el0, {sp}",
                "msr elr_el1, {pc}",
                "msr spsr_el1, {spsr}",
                "mov x0, {arg}",
                "eret",
                ksp = in(reg) ksp,
                sp = in(reg) entry.user_sp,
                pc = in(reg) entry.entry_pc,
                spsr = in(reg) spsr,
                arg = in(reg) entry.arg0,
                options(noreturn)
            );
        }
    }

    // Paging - use ProxyAllocator for real page table allocation
    fn make_user_address_space(&self) -> Self::AddressSpace {
        let aspace = paging::make_user_address_space(self.active_address_space(), &ProxyAllocator);
        kernel::kinfo!(
            "make_user_address_space: created aspace phys={:#x}",
            aspace.0
        );
        aspace
    }

    fn active_address_space(&self) -> Self::AddressSpace {
        paging::active_address_space()
    }

    fn activate_address_space(&self, aspace: Self::AddressSpace) {
        kernel::kinfo!("activate_address_space: setting TTBR0 to {:#x}", aspace.0);
        unsafe {
            asm!(
                "msr ttbr0_el1, {ttbr}",
                "isb",
                "tlbi vmalle1is",
                "dsb ish",
                "isb",
                ttbr = in(reg) aspace.0,
                options(nostack)
            );
        }
    }

    fn map_page(
        &self,
        aspace: Self::AddressSpace,
        virt: u64,
        phys: u64,
        perms: MapPerms,
        allocator: &dyn FrameAllocatorHook,
    ) -> Result<(), ()> {
        paging::map_page(aspace, virt, phys, perms, allocator)
    }

    fn unmap_page(&self, aspace: Self::AddressSpace, virt: u64) -> Result<Option<u64>, ()> {
        paging::unmap_page(aspace, virt)
    }

    fn translate(&self, aspace: Self::AddressSpace, virt: u64) -> Option<u64> {
        paging::translate(aspace, virt)
    }

    fn tlb_flush_page(&self, virt: u64) {
        paging::tlb_flush_page(virt)
    }

    fn aspace_to_raw(&self, aspace: Self::AddressSpace) -> u64 {
        aspace.0
    }
}
struct ProxyAllocator;
impl FrameAllocatorHook for ProxyAllocator {
    fn alloc_frame(&self) -> Option<u64> {
        kernel::memory::alloc_frame()
    }
}

pub struct SerialPort {
    pub clamp: MonotonicClamp,
}

impl SerialPort {
    pub const fn new() -> Self {
        Self {
            clamp: MonotonicClamp::new(),
        }
    }

    fn putchar(&self, c: u8) {
        // 2. PL011 UART0 (standard on QEMU virt)
        // We use the HHDM mapping if initialized, or fall back to physical if very early.
        // Limine usually maps the first 4GiB of physical memory at HHDM_OFFSET.
        let hhdm = unsafe { paging::get_hhdm_offset() };
        let uart_base = 0x09000000u64 + hhdm;
        let uart = uart_base as *mut u32;

        unsafe {
            // UARTFR (Flag Register) is at offset 0x18. TXFF is bit 5.
            // NON-BLOCKING: If the FIFO is full for too long (e.g. no one reading
            // the serial socket), drop the byte rather than hanging the kernel.
            let mut timeout = 1000u32;
            while (core::ptr::read_volatile(uart.add(6)) & (1 << 5)) != 0 && timeout > 0 {
                timeout -= 1;
            }

            if timeout > 0 {
                // UARTDR (Data Register) is at offset 0x00.
                core::ptr::write_volatile(uart, c as u32);
            }
        }
    }
}

pub fn hcf() -> ! {
    loop {
        unsafe {
            asm!("wfi", options(nomem, nostack));
        }
    }
}

#[inline]
fn read_cntfrq_el0() -> u64 {
    let val: u64;
    unsafe {
        asm!("mrs {}, cntfrq_el0", out(reg) val, options(nomem, nostack));
    }
    val
}

#[inline]
fn read_cntvct_el0() -> u64 {
    let val: u64;
    unsafe {
        asm!("mrs {}, cntvct_el0", out(reg) val, options(nomem, nostack));
    }
    val
}
