use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;
use alloc::{format, vec};
use core::mem::size_of;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use abi::trace::TraceEvent;
use kernel::{kdebug, kinfo};

pub const IRQ_TIMER_VECTOR: u8 = 0x20;
pub const IRQ_PAUSE_DUMP_VECTOR: u8 = 0x31;
pub const IRQ_RESCHED_VECTOR: u8 = 0x30;
pub const IRQ_TLB_SHOOTDOWN_VECTOR: u8 = 0x41;

static IRQ12_COUNT: AtomicU64 = AtomicU64::new(0);
static IRQ1_COUNT: AtomicU64 = AtomicU64::new(0);
static IRQ4_COUNT: AtomicU64 = AtomicU64::new(0);
static HOTKEY_SHELL_TID: AtomicU64 = AtomicU64::new(0);
static PAUSE_DUMP_ACTIVE: AtomicBool = AtomicBool::new(false);
static PAUSE_DUMP_OWNER_CPU: AtomicU64 = AtomicU64::new(u64::MAX);
static PS2_REBOOT_EXTENDED_PREFIX: AtomicBool = AtomicBool::new(false);
static PS2_REBOOT_CTRL_DOWN: AtomicBool = AtomicBool::new(false);
static PS2_REBOOT_ALT_DOWN: AtomicBool = AtomicBool::new(false);

const MAX_PAUSE_CPUS: usize = 256;
static PAUSE_CPU_VALID: [AtomicBool; MAX_PAUSE_CPUS] =
    [const { AtomicBool::new(false) }; MAX_PAUSE_CPUS];
static PAUSE_CPU_RIP: [AtomicU64; MAX_PAUSE_CPUS] = [const { AtomicU64::new(0) }; MAX_PAUSE_CPUS];
static PAUSE_CPU_RSP: [AtomicU64; MAX_PAUSE_CPUS] = [const { AtomicU64::new(0) }; MAX_PAUSE_CPUS];
static PAUSE_CPU_RBP: [AtomicU64; MAX_PAUSE_CPUS] = [const { AtomicU64::new(0) }; MAX_PAUSE_CPUS];
static PAUSE_CPU_RFLAGS: [AtomicU64; MAX_PAUSE_CPUS] =
    [const { AtomicU64::new(0) }; MAX_PAUSE_CPUS];
static PAUSE_CPU_CS: [AtomicU64; MAX_PAUSE_CPUS] = [const { AtomicU64::new(0) }; MAX_PAUSE_CPUS];

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct IdtEntry {
    offset_low: u16,
    selector: u16,
    ist: u8,
    type_attr: u8,
    offset_middle: u16,
    offset_high: u32,
    reserved: u32,
}

impl IdtEntry {
    const fn missing() -> Self {
        Self {
            offset_low: 0,
            selector: 0,
            ist: 0,
            type_attr: 0,
            offset_middle: 0,
            offset_high: 0,
            reserved: 0,
        }
    }

    fn set_handler(&mut self, handler: u64, sel: u16, ist: u8, type_attr: u8) {
        self.offset_low = (handler & 0xFFFF) as u16;
        self.selector = sel;
        self.ist = ist;
        self.type_attr = type_attr;
        self.offset_middle = ((handler >> 16) & 0xFFFF) as u16;
        self.offset_high = (handler >> 32) as u32;
        self.reserved = 0;
    }
}

#[repr(C, align(16))]
pub struct Idt {
    entries: [IdtEntry; 256],
}

pub static mut IDT: Idt = Idt { entries: [IdtEntry::missing(); 256] };

#[repr(C, packed)]
struct IdtDescriptor {
    size: u16,
    offset: u64,
}

unsafe extern "C" {
    fn nmi_handler_shim();
    fn breakpoint_handler_shim();
    fn double_fault_handler_shim();
    fn gp_handler_shim();
    fn pf_handler_shim();
    fn generic_handler_shim();
    fn irq_common_handler_shim();
    fn irq_timer_handler_shim();
    fn irq_resched_handler_shim();
    fn irq_tlb_shootdown_handler_shim();
    fn irq_keyboard_handler_shim();
    fn irq_pause_dump_handler_shim();
    fn irq_mouse_handler_shim();
    fn irq_serial_handler_shim();
    fn invalid_opcode_handler_shim();
    fn div0_handler_shim();
}

core::arch::global_asm!(
    r#"
    .global breakpoint_handler_shim
    breakpoint_handler_shim:
        int3
        iretq

    .global double_fault_handler_shim
    double_fault_handler_shim:
        mov $0x3f8, %dx
        mov $0x44, %al
        out %al, %dx
        mov $0x46, %al
        out %al, %dx
        testb $3, 16(%rsp)
        jz 1f
        swapgs
    1:
        cli
        mov %rsp, %rdi
        call rust_double_fault_handler
    2:  hlt
        jmp 2b

    .global gp_handler_shim
    gp_handler_shim:
        mov $0x3f8, %dx
        mov $0x47, %al
        out %al, %dx
        testb $3, 16(%rsp)
        jz 1f
        swapgs
    1:
        cli
        mov %rsp, %rdi
        call rust_gp_handler
    2:  hlt
        jmp 2b

    .global invalid_opcode_handler_shim
    invalid_opcode_handler_shim:
        push $0
        testb $3, 16(%rsp)
        jz 1f
        swapgs
    1:
        cli
        mov %rsp, %rdi
        call rust_invalid_opcode_handler
    2:  hlt
        jmp 2b

    .global div0_handler_shim
    div0_handler_shim:
        push $0
        testb $3, 16(%rsp)
        jz 1f
        swapgs
    1:
        cli
        mov %rsp, %rdi
        call rust_div0_handler
    2:  hlt
        jmp 2b

    .global nmi_handler_shim
    nmi_handler_shim:
        testb $3, 8(%rsp)
        jz 1f
        swapgs
    1:
        push %rax
        push %rcx
        push %rdx
        push %rbx
        push %rbp
        push %rsi
        push %rdi
        push %r8
        push %r9
        push %r10
        push %r11
        push %r12
        push %r13
        push %r14
        push %r15

        mov %rsp, %rdi
        call rust_nmi_handler

        pop %r15
        pop %r14
        pop %r13
        pop %r12
        pop %r11
        pop %r10
        pop %r9
        pop %r8
        pop %rdi
        pop %rsi
        pop %rbp
        pop %rbx
        pop %rdx
        pop %rcx
        pop %rax

        testb $3, 8(%rsp)
        jz 2f
        swapgs
    2:
        iretq

    pf_handler_shim:
        // Check if coming from user mode (CS bit 0-1)
        testb $3, 16(%rsp)
        jz 1f
        swapgs
    1:
        // PF pushes error code, so stack has: ERR, RIP, CS, RFLAGS, RSP, SS
        push %rax
        push %rcx
        push %rdx
        push %rbx
        push %rbp
        push %rsi
        push %rdi
        push %r8
        push %r9
        push %r10
        push %r11
        push %r12
        push %r13
        push %r14
        push %r15

        mov %rsp, %rsi // Pointer to IrqRegisterSnapshot (15 registers)
        lea 120(%rsp), %rdi // Pointer to InterruptStackFrame
        call rust_pf_handler

        pop %r15
        pop %r14
        pop %r13
        pop %r12
        pop %r11
        pop %r10
        pop %r9
        pop %r8
        pop %rdi
        pop %rsi
        pop %rbp
        pop %rbx
        pop %rdx
        pop %rcx
        pop %rax

        testb $3, 16(%rsp) // CS of frame
        jz 2f
        swapgs
    2:
        add $8, %rsp // Pop error code
        iretq

    .global generic_handler_shim
    generic_handler_shim:
        mov $0x3f8, %dx
        mov $0x3F, %al
        out %al, %dx
    2:  hlt
        jmp 2b

    irq_common_handler_shim:
        // IRQ stubs (generic) don't push error code.
        // Stack has: RIP, CS, RFLAGS, RSP, SS
        testb $3, 8(%rsp)
        jz 1f
        swapgs
    1:
        push %rax
        push %rcx
        push %rdx
        push %rbx
        push %rbp
        push %rsi
        push %rdi
        push %r8
        push %r9
        push %r10
        push %r11
        push %r12
        push %r13
        push %r14
        push %r15

        mov $0, %rdi // Vector will be resolved via LAPIC ISR
        mov %rsp, %rsi
        call rust_irq_handler

        pop %r15
        pop %r14
        pop %r13
        pop %r12
        pop %r11
        pop %r10
        pop %r9
        pop %r8
        pop %rdi
        pop %rsi
        pop %rbp
        pop %rbx
        pop %rdx
        pop %rcx
        pop %rax

        testb $3, 8(%rsp)
        jz 2f
        swapgs
    2:
        iretq

    irq_timer_handler_shim:
        testb $3, 8(%rsp)
        jz 1f
        swapgs
    1:
        push %rax
        push %rcx
        push %rdx
        push %rbx
        push %rbp
        push %rsi
        push %rdi
        push %r8
        push %r9
        push %r10
        push %r11
        push %r12
        push %r13
        push %r14
        push %r15

        mov $0x20, %rdi
        mov %rsp, %rsi
        call rust_irq_handler

        pop %r15
        pop %r14
        pop %r13
        pop %r12
        pop %r11
        pop %r10
        pop %r9
        pop %r8
        pop %rdi
        pop %rsi
        pop %rbp
        pop %rbx
        pop %rdx
        pop %rcx
        pop %rax

        testb $3, 8(%rsp)
        jz 2f
        swapgs
    2:
        iretq

    .global irq_pause_dump_handler_shim
    irq_pause_dump_handler_shim:
        testb $3, 8(%rsp)
        jz 1f
        swapgs
    1:
        push %rax
        push %rcx
        push %rdx
        push %rbx
        push %rbp
        push %rsi
        push %rdi
        push %r8
        push %r9
        push %r10
        push %r11
        push %r12
        push %r13
        push %r14
        push %r15

        mov %rsp, %rdi
        call rust_pause_dump_ipi_handler

        pop %r15
        pop %r14
        pop %r13
        pop %r12
        pop %r11
        pop %r10
        pop %r9
        pop %r8
        pop %rdi
        pop %rsi
        pop %rbp
        pop %rbx
        pop %rdx
        pop %rcx
        pop %rax

        testb $3, 8(%rsp)
        jz 2f
        swapgs
    2:
        iretq

    .global irq_keyboard_handler_shim
    irq_keyboard_handler_shim:
        testb $3, 8(%rsp)
        jz 1f
        swapgs
    1:
        push %rax
        push %rcx
        push %rdx
        push %rbx
        push %rbp
        push %rsi
        push %rdi
        push %r8
        push %r9
        push %r10
        push %r11
        push %r12
        push %r13
        push %r14
        push %r15

        mov $0x21, %rdi
        mov %rsp, %rsi
        call rust_irq_handler

        pop %r15
        pop %r14
        pop %r13
        pop %r12
        pop %r11
        pop %r10
        pop %r9
        pop %r8
        pop %rdi
        pop %rsi
        pop %rbp
        pop %rbx
        pop %rdx
        pop %rcx
        pop %rax

        testb $3, 8(%rsp)
        jz 2f
        swapgs
    2:
        iretq

    .global irq_resched_handler_shim
    irq_resched_handler_shim:
        testb $3, 8(%rsp)
        jz 1f
        swapgs
    1:
        push %rax
        push %rcx
        push %rdx
        push %rbx
        push %rbp
        push %rsi
        push %rdi
        push %r8
        push %r9
        push %r10
        push %r11
        push %r12
        push %r13
        push %r14
        push %r15

        mov $0x30, %rdi
        mov %rsp, %rsi
        call rust_irq_handler

        pop %r15
        pop %r14
        pop %r13
        pop %r12
        pop %r11
        pop %r10
        pop %r9
        pop %r8
        pop %rdi
        pop %rsi
        pop %rbp
        pop %rbx
        pop %rdx
        pop %rcx
        pop %rax

        testb $3, 8(%rsp)
        jz 2f
        swapgs
    2:
        iretq
    .global irq_tlb_shootdown_handler_shim
    irq_tlb_shootdown_handler_shim:
        testb $3, 8(%rsp)
        jz 1f
        swapgs
    1:
        push %rax
        push %rcx
        push %rdx
        push %rbx
        push %rbp
        push %rsi
        push %rdi
        push %r8
        push %r9
        push %r10
        push %r11
        push %r12
        push %r13
        push %r14
        push %r15

        mov $0x41, %rdi
        mov %rsp, %rsi
        call rust_irq_handler

        pop %r15
        pop %r14
        pop %r13
        pop %r12
        pop %r11
        pop %r10
        pop %r9
        pop %r8
        pop %rdi
        pop %rsi
        pop %rbp
        pop %rbx
        pop %rdx
        pop %rcx
        pop %rax

        testb $3, 8(%rsp)
        jz 2f
        swapgs
    2:
        iretq

    .global irq_mouse_handler_shim
    irq_mouse_handler_shim:
        testb $3, 8(%rsp)
        jz 1f
        swapgs
    1:
        push %rax
        push %rcx
        push %rdx
        push %rbx
        push %rbp
        push %rsi
        push %rdi
        push %r8
        push %r9
        push %r10
        push %r11
        push %r12
        push %r13
        push %r14
        push %r15

        mov $0x2C, %rdi
        mov %rsp, %rsi
        call rust_irq_handler

        pop %r15
        pop %r14
        pop %r13
        pop %r12
        pop %r11
        pop %r10
        pop %r9
        pop %r8
        pop %rdi
        pop %rsi
        pop %rbp
        pop %rbx
        pop %rdx
        pop %rcx
        pop %rax

        testb $3, 8(%rsp)
        jz 2f
        swapgs
    2:
        iretq
    .global irq_serial_handler_shim
    irq_serial_handler_shim:
        testb $3, 8(%rsp)
        jz 1f
        swapgs
    1:
        push %rax
        push %rcx
        push %rdx
        push %rbx
        push %rbp
        push %rsi
        push %rdi
        push %r8
        push %r9
        push %r10
        push %r11
        push %r12
        push %r13
        push %r14
        push %r15

        mov $0x24, %rdi
        mov %rsp, %rsi
        call rust_irq_handler

        pop %r15
        pop %r14
        pop %r13
        pop %r12
        pop %r11
        pop %r10
        pop %r9
        pop %r8
        pop %rdi
        pop %rsi
        pop %rbp
        pop %rbx
        pop %rdx
        pop %rcx
        pop %rax

        testb $3, 8(%rsp)
        jz 2f
        swapgs
    2:
        iretq
"#,
    options(att_syntax)
);

pub unsafe fn init() {
    let handler = generic_handler_shim as *const () as u64;
    unsafe {
        let base = core::ptr::addr_of_mut!(IDT.entries) as *mut IdtEntry;
        for i in 0..256 {
            (*base.add(i)).set_handler(handler, crate::arch::x86_64::gdt::KERNEL_CODE_SEL, 0, 0x8E);
        }

        IDT.entries[3].set_handler(
            breakpoint_handler_shim as *const () as u64,
            crate::arch::x86_64::gdt::KERNEL_CODE_SEL,
            0,
            0x8E,
        );
        IDT.entries[0].set_handler(
            div0_handler_shim as *const () as u64,
            crate::arch::x86_64::gdt::KERNEL_CODE_SEL,
            0,
            0x8E,
        );
        IDT.entries[6].set_handler(
            invalid_opcode_handler_shim as *const () as u64,
            crate::arch::x86_64::gdt::KERNEL_CODE_SEL,
            0,
            0x8E,
        );
        IDT.entries[8].set_handler(
            double_fault_handler_shim as *const () as u64,
            crate::arch::x86_64::gdt::KERNEL_CODE_SEL,
            1,
            0x8E,
        );
        IDT.entries[2].set_handler(
            nmi_handler_shim as *const () as u64,
            crate::arch::x86_64::gdt::KERNEL_CODE_SEL,
            0,
            0x8E,
        );
        IDT.entries[13].set_handler(
            gp_handler_shim as *const () as u64,
            crate::arch::x86_64::gdt::KERNEL_CODE_SEL,
            0,
            0x8E,
        );
        IDT.entries[14].set_handler(
            pf_handler_shim as *const () as u64,
            crate::arch::x86_64::gdt::KERNEL_CODE_SEL,
            0,
            0x8E,
        );

        // Hardware IRQs/MSI vectors
        for vector in 0x20..=0xEF {
            IDT.entries[vector as usize].set_handler(
                irq_common_handler_shim as *const () as u64,
                crate::arch::x86_64::gdt::KERNEL_CODE_SEL,
                0,
                0x8E,
            );
        }

        // Dedicated Timer Vector
        IDT.entries[IRQ_TIMER_VECTOR as usize].set_handler(
            irq_timer_handler_shim as *const () as u64,
            crate::arch::x86_64::gdt::KERNEL_CODE_SEL,
            0,
            0x8E,
        );

        // Dedicated Keyboard Vector (0x21) - Bypass Common Shim/ISR lookup
        IDT.entries[0x21].set_handler(
            irq_keyboard_handler_shim as *const () as u64,
            crate::arch::x86_64::gdt::KERNEL_CODE_SEL,
            0,
            0x8E,
        );

        IDT.entries[IRQ_PAUSE_DUMP_VECTOR as usize].set_handler(
            irq_pause_dump_handler_shim as *const () as u64,
            crate::arch::x86_64::gdt::KERNEL_CODE_SEL,
            0,
            0x8E,
        );

        // Dedicated Reschedule IPI Vector
        IDT.entries[IRQ_RESCHED_VECTOR as usize].set_handler(
            irq_resched_handler_shim as *const () as u64,
            crate::arch::x86_64::gdt::KERNEL_CODE_SEL,
            0,
            0x8E,
        );

        // Dedicated TLB Shootdown Vector
        IDT.entries[IRQ_TLB_SHOOTDOWN_VECTOR as usize].set_handler(
            irq_tlb_shootdown_handler_shim as *const () as u64,
            crate::arch::x86_64::gdt::KERNEL_CODE_SEL,
            0,
            0x8E,
        );

        // Dedicated Mouse Vector (0x2C) - Bypass Common Shim/ISR lookup
        IDT.entries[0x2C].set_handler(
            irq_mouse_handler_shim as *const () as u64,
            crate::arch::x86_64::gdt::KERNEL_CODE_SEL,
            0,
            0x8E,
        );

        // Dedicated Serial Vector (0x24) - Bypass Common Shim/ISR lookup
        IDT.entries[0x24].set_handler(
            irq_serial_handler_shim as *const () as u64,
            crate::arch::x86_64::gdt::KERNEL_CODE_SEL,
            0,
            0x8E,
        );

        let idtr = IdtDescriptor {
            size: (size_of::<Idt>() - 1) as u16,
            offset: core::ptr::addr_of!(IDT) as u64,
        };

        core::arch::asm!("lidt [{}]", in(reg) &idtr);
    }
}

/// Load the kernel IDT on secondary CPUs.
/// The IDT entries are already set up by the BSP.
pub unsafe fn load_on_secondary() {
    let idtr = IdtDescriptor {
        size: (size_of::<Idt>() - 1) as u16,
        offset: unsafe { core::ptr::addr_of!(IDT) } as u64,
    };

    unsafe {
        core::arch::asm!("lidt [{}]", in(reg) &idtr);
    }
}

#[repr(C)]
pub struct InterruptStackFrame {
    pub error_code: u64,
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

#[repr(C)]
pub struct IrqRegisterSnapshot {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rbp: u64,
    pub rbx: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rax: u64,
}

fn ps2_irq_trace_enter(_vector: u8, _count: u64, _irq_snapshot: *const IrqRegisterSnapshot) -> u64 {
    if kernel::is_runtime_initialized() { kernel::runtime_base().mono_ticks() } else { 0 }
}

fn ps2_irq_trace_exit(_vector: u8, _count: u64, _start_ticks: u64) {
    // Keep hard IRQ paths free of formatted console logging. Entry/exit are
    // already recorded in the progress ring around the interrupt dispatch.
}

const PS2_STATUS_PORT: u16 = 0x64;
const PS2_DATA_PORT: u16 = 0x60;
const PS2_STATUS_OUTPUT_FULL: u8 = 0x01;
const PS2_STATUS_AUX_DATA: u8 = 0x20;
const PS2_SCANCODE_CTRL: u8 = 0x1D;
const PS2_SCANCODE_ALT: u8 = 0x38;
const PS2_SCANCODE_DELETE: u8 = 0x53;
const PS2_SCANCODE_F12: u8 = 0x58;
const PS2_SCANCODE_RELEASE_MASK: u8 = 0x80;
const PS2_SCANCODE_KEY_MASK: u8 = 0x7F;
const HOTKEY_SHELL_LOCKED: u64 = u64::MAX;

#[inline]
fn raw_inb(port: u16) -> u8 {
    let value: u8;
    unsafe {
        core::arch::asm!("in al, dx", out("al") value, in("dx") port, options(nostack, preserves_flags))
    }
    value
}

fn detect_ctrl_alt_del_reboot(byte: u8) -> bool {
    match byte {
        0xE0 => {
            PS2_REBOOT_EXTENDED_PREFIX.store(true, Ordering::Release);
            return false;
        }
        0xE1 => {
            PS2_REBOOT_EXTENDED_PREFIX.store(false, Ordering::Release);
            return false;
        }
        _ => {}
    }

    let extended = PS2_REBOOT_EXTENDED_PREFIX.swap(false, Ordering::AcqRel);
    let released = (byte & PS2_SCANCODE_RELEASE_MASK) != 0;
    let scancode = byte & PS2_SCANCODE_KEY_MASK;

    match (extended, scancode) {
        (false, PS2_SCANCODE_CTRL) | (true, PS2_SCANCODE_CTRL) => {
            PS2_REBOOT_CTRL_DOWN.store(!released, Ordering::Release);
            false
        }
        (false, PS2_SCANCODE_ALT) | (true, PS2_SCANCODE_ALT) => {
            PS2_REBOOT_ALT_DOWN.store(!released, Ordering::Release);
            false
        }
        (true, PS2_SCANCODE_DELETE) if !released => {
            PS2_REBOOT_CTRL_DOWN.load(Ordering::Acquire)
                && PS2_REBOOT_ALT_DOWN.load(Ordering::Acquire)
        }
        _ => false,
    }
}

fn reboot_from_ctrl_alt_del() -> ! {
    let runtime = kernel::runtime_base();
    runtime.serial_putbuf_sync(b"PS/2 hotkey Ctrl+Alt+Del detected; forcing immediate reboot\r\n");
    runtime.reboot();
}

fn capture_ps2_keyboard(
    max_reads: usize,
    consume_action_hotkeys: bool,
    trace_bytes: bool,
) -> (bool, bool, bool, Option<u8>, usize) {
    let mut pause_dump = false;
    let mut f12_press = false;
    let mut ctrl_alt_del = false;
    let mut log_level = None;
    let mut captured = 0usize;

    for _ in 0..max_reads {
        let status = raw_inb(PS2_STATUS_PORT);
        if status & PS2_STATUS_OUTPUT_FULL == 0 {
            break;
        }
        let is_aux = (status & PS2_STATUS_AUX_DATA) != 0;
        let byte = raw_inb(PS2_DATA_PORT);
        captured += 1;
        if trace_bytes {
            kernel::ktrace!("PS/2 byte received: 0x{:02x} (is_aux={})", byte, is_aux);
        }

        if is_aux {
            kernel::irq::ps2::buffer_scancode(byte, true);
            continue;
        }

        if detect_ctrl_alt_del_reboot(byte) {
            ctrl_alt_del = true;
        }
        if kernel::irq::ps2::buffer_scancode(byte, false) {
            pause_dump = true;
        }
        if consume_action_hotkeys {
            if kernel::irq::ps2::take_terminal_hotkey() {
                f12_press = true;
            }
            if let Some(level) = kernel::irq::ps2::take_log_level_hotkey() {
                log_level = Some(level);
            }
        }
    }

    (pause_dump, f12_press, ctrl_alt_del, log_level, captured)
}

fn capture_pause_reboot_hotkey(max_reads: usize) -> bool {
    let mut reboot = false;

    for _ in 0..max_reads {
        let status = raw_inb(PS2_STATUS_PORT);
        if status & PS2_STATUS_OUTPUT_FULL == 0 {
            break;
        }
        let is_aux = (status & PS2_STATUS_AUX_DATA) != 0;
        let byte = raw_inb(PS2_DATA_PORT);

        if is_aux {
            kernel::irq::ps2::buffer_scancode(byte, true);
            continue;
        }

        let released = (byte & PS2_SCANCODE_RELEASE_MASK) != 0;
        let scancode = byte & PS2_SCANCODE_KEY_MASK;
        if detect_ctrl_alt_del_reboot(byte) {
            reboot = true;
        }
        let _ = kernel::irq::ps2::buffer_scancode(byte, false);
        if !released && scancode == PS2_SCANCODE_F12 {
            reboot = true;
        }
    }

    reboot
}

fn capture_ps2_keyboard_irq() -> (bool, bool, bool) {
    let (pause, mut f12, ctrl_alt_del, mut log_level, _) = capture_ps2_keyboard(32, true, false);
    f12 |= kernel::irq::ps2::take_terminal_hotkey();
    log_level = log_level.or_else(kernel::irq::ps2::take_log_level_hotkey);
    if let Some(level) = log_level {
        announce_log_level_hotkey(level);
    }
    (pause, f12, ctrl_alt_del)
}

fn poll_ps2_keyboard_fallback() -> (bool, bool) {
    let (pause_dump, mut f12_press, ctrl_alt_del, mut log_level, captured) =
        capture_ps2_keyboard(8, true, false);
    f12_press |= kernel::irq::ps2::take_terminal_hotkey();
    log_level = log_level.or_else(kernel::irq::ps2::take_log_level_hotkey);
    if let Some(level) = log_level {
        announce_log_level_hotkey(level);
    }
    if f12_press {
        activate_terminal_and_spawn_shell();
    }
    if captured != 0 && !ctrl_alt_del {
        kernel::irq::dispatch_irq(0x21);
    }
    (pause_dump, ctrl_alt_del)
}

fn announce_log_level_hotkey(level: u8) {
    let name = kernel::logging::log_level_name(level);
    kernel::kinfo!("F1 hotkey: log level set to {} ({})", level, name);
}

fn try_spawn_shell(path: &str) -> Option<u64> {
    kernel::irq::ps2::set_fb_input_enabled(true);
    let tty_path = "/dev/tty0".to_string();
    let res = unsafe {
        kernel::sched::spawn_process_from_path_current(
            path,
            alloc::vec![path.as_bytes().to_vec()],
            alloc::collections::BTreeMap::new(),
            kernel::sched::StdioSpec::Path(tty_path.clone()),
            kernel::sched::StdioSpec::Path(tty_path.clone()),
            kernel::sched::StdioSpec::Path(tty_path),
            0,
            alloc::vec![],
            Some("/".to_string()),
            alloc::vec![],
            None,
        )
    };
    res.ok().map(|r| r.child_tid)
}

fn hotkey_shell_is_alive(tid: u64) -> bool {
    match unsafe { kernel::sched::task_status_current(tid) } {
        Some((state, _)) => state != kernel::task::TaskState::Dead,
        None => false,
    }
}

fn try_lock_hotkey_shell_spawn() -> bool {
    loop {
        let current = HOTKEY_SHELL_TID.load(Ordering::Acquire);

        if current == HOTKEY_SHELL_LOCKED {
            return false;
        }

        if current != 0 {
            if hotkey_shell_is_alive(current) {
                return false;
            }

            if HOTKEY_SHELL_TID
                .compare_exchange(current, 0, Ordering::AcqRel, Ordering::Acquire)
                .is_err()
            {
                continue;
            }
        }

        if HOTKEY_SHELL_TID
            .compare_exchange(0, HOTKEY_SHELL_LOCKED, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            return true;
        }
    }
}

fn unlock_hotkey_shell_spawn(spawned_tid: Option<u64>) {
    HOTKEY_SHELL_TID.store(spawned_tid.unwrap_or(0), Ordering::Release);
}

pub fn activate_terminal_and_spawn_shell() {
    crate::console::activate_onscreen_terminal();

    if !try_lock_hotkey_shell_spawn() {
        let existing = HOTKEY_SHELL_TID.load(Ordering::Acquire);
        if existing != 0 && existing != HOTKEY_SHELL_LOCKED {
            kdebug!("F12 hotkey: shell already running (tid={})", existing);
        }
        return;
    }

    if let Some(tid) = try_spawn_shell("/bin/sh") {
        unlock_hotkey_shell_spawn(Some(tid));
        kdebug!("F12 hotkey: spawned {} as tid {}", "/bin/sh", tid);
        return;
    }

    unlock_hotkey_shell_spawn(None);

    kdebug!("F12 hotkey: failed to spawn /bin/sh");
}

fn capture_control_state() -> (u64, u64, u64, u64) {
    let rsp: u64;
    let rbp: u64;
    let cr2: u64;
    let cr3: u64;
    unsafe {
        core::arch::asm!("mov {}, rsp", out(reg) rsp, options(nomem, nostack, preserves_flags));
        core::arch::asm!("mov {}, rbp", out(reg) rbp, options(nomem, nostack, preserves_flags));
        core::arch::asm!("mov {}, cr2", out(reg) cr2, options(nomem, nostack, preserves_flags));
        core::arch::asm!("mov {}, cr3", out(reg) cr3, options(nomem, nostack, preserves_flags));
    }
    (rsp, rbp, cr2, cr3)
}

fn print_stack_words(base_rsp: u64) {
    kernel::kprint!("Stack Snapshot:\n");
    for i in 0..8u64 {
        let addr = base_rsp.wrapping_add(i * 8);
        let value = unsafe { core::ptr::read_volatile(addr as *const u64) };
        kernel::kprint!("  [0x{:016x}] = 0x{:016x}\n", addr, value);
    }
}

fn print_frame_pointer_walk(mut rbp: u64) {
    kernel::kprint!("Frame Walk:\n");
    for depth in 0..8 {
        if rbp == 0 || (rbp & 0x7) != 0 {
            break;
        }

        let next_rbp = unsafe { core::ptr::read_volatile(rbp as *const u64) };
        let ret_addr = unsafe { core::ptr::read_volatile((rbp + 8) as *const u64) };
        kernel::kprint!("  #{} rbp=0x{:016x} ret=0x{:016x}\n", depth, rbp, ret_addr);

        if next_rbp <= rbp {
            break;
        }
        rbp = next_rbp;
    }
}

fn print_register_snapshot(snapshot: &IrqRegisterSnapshot) {
    kernel::kprint!("Interrupted Context:\n");
    // Frame is immediately after the IrqRegisterSnapshot on the stack
    unsafe {
        let frame_ptr = (snapshot as *const _ as *const u64).add(15);
        let rip = *frame_ptr.add(0);
        let cs = *frame_ptr.add(1);
        let rflags = *frame_ptr.add(2);
        let rsp = *frame_ptr.add(3);
        let ss = *frame_ptr.add(4);

        kernel::kprint!("  RIP=0x{:016x}  CS=0x{:016x}  RFLAGS=0x{:016x}\n", rip, cs, rflags);
        kernel::kprint!("  RSP=0x{:016x}  SS=0x{:016x}\n", rsp, ss);
    }
    kernel::kprint!(
        "  RAX=0x{:016x}  RBX=0x{:016x}  RCX=0x{:016x}  RDX=0x{:016x}\n",
        snapshot.rax,
        snapshot.rbx,
        snapshot.rcx,
        snapshot.rdx
    );
    kernel::kprint!(
        "  RSI=0x{:016x}  RDI=0x{:016x}  RBP=0x{:016x}\n",
        snapshot.rsi,
        snapshot.rdi,
        snapshot.rbp
    );
    kernel::kprint!(
        "  R8 =0x{:016x}  R9 =0x{:016x}  R10=0x{:016x}  R11=0x{:016x}\n",
        snapshot.r8,
        snapshot.r9,
        snapshot.r10,
        snapshot.r11
    );
    kernel::kprint!(
        "  R12=0x{:016x}  R13=0x{:016x}  R14=0x{:016x}  R15=0x{:016x}\n",
        snapshot.r12,
        snapshot.r13,
        snapshot.r14,
        snapshot.r15
    );
}

fn store_pause_cpu_snapshot(cpu: usize, snapshot: &IrqRegisterSnapshot, rsp: u64, rbp: u64) {
    if cpu >= MAX_PAUSE_CPUS {
        return;
    }

    unsafe {
        let frame_ptr = (snapshot as *const _ as *const u64).add(15);
        let rip = *frame_ptr.add(0);
        let cs = *frame_ptr.add(1);
        let rflags = *frame_ptr.add(2);

        PAUSE_CPU_RIP[cpu].store(rip, Ordering::SeqCst);
        PAUSE_CPU_RSP[cpu].store(rsp, Ordering::SeqCst);
        PAUSE_CPU_RBP[cpu].store(rbp, Ordering::SeqCst);
        PAUSE_CPU_RFLAGS[cpu].store(rflags, Ordering::SeqCst);
        PAUSE_CPU_CS[cpu].store(cs, Ordering::SeqCst);
    }
    PAUSE_CPU_VALID[cpu].store(true, Ordering::SeqCst);
}

fn clear_pause_cpu_snapshots(cpu_total: usize) {
    let limit = cpu_total.min(MAX_PAUSE_CPUS);
    for cpu in 0..limit {
        PAUSE_CPU_VALID[cpu].store(false, Ordering::SeqCst);
        PAUSE_CPU_RIP[cpu].store(0, Ordering::SeqCst);
        PAUSE_CPU_RSP[cpu].store(0, Ordering::SeqCst);
        PAUSE_CPU_RBP[cpu].store(0, Ordering::SeqCst);
        PAUSE_CPU_RFLAGS[cpu].store(0, Ordering::SeqCst);
        PAUSE_CPU_CS[cpu].store(0, Ordering::SeqCst);
    }
}

fn wait_for_pause_cpu_snapshots(current_cpu: usize, cpu_total: usize) {
    let limit = cpu_total.min(MAX_PAUSE_CPUS);
    for _ in 0..200_000 {
        let mut all_seen = true;
        for cpu in 0..limit {
            if cpu == current_cpu {
                continue;
            }
            if !PAUSE_CPU_VALID[cpu].load(Ordering::SeqCst) {
                all_seen = false;
                break;
            }
        }
        if all_seen {
            return;
        }
        core::hint::spin_loop();
    }
}

fn trigger_pause_dump(snapshot: Option<&IrqRegisterSnapshot>) -> ! {
    let already_active = PAUSE_DUMP_ACTIVE.swap(true, Ordering::SeqCst);
    unsafe {
        core::arch::asm!("cli", options(nomem, nostack, preserves_flags));
        kernel::logging::force_unlock();
    }

    if !already_active {
        let runtime = kernel::runtime_base();
        let current_cpu = runtime.current_cpu_index();
        PAUSE_DUMP_OWNER_CPU.store(current_cpu as u64, Ordering::SeqCst);
        let cpu_total = runtime.cpu_total_count();
        clear_pause_cpu_snapshots(cpu_total);

        let (rsp, rbp, _, _) = capture_control_state();
        if let Some(snapshot) = snapshot {
            store_pause_cpu_snapshot(current_cpu, snapshot, rsp, rbp);
        }

        for cpu in 0..cpu_total {
            if cpu != current_cpu {
                let apic_id = unsafe { crate::arch::x86_64::CPU_IDS[cpu].0 };
                crate::arch::x86_64::ioapic::send_nmi_ipi(apic_id);
            }
        }
        wait_for_pause_cpu_snapshots(current_cpu, cpu_total);

        kinfo!("PS/2 hotkey Alt+F12 detected on CPU {}; forcing kernel pause", current_cpu);

        // Dump the last-progress-marker ring so freeze analysis can see recent activity.
        kernel::trace::progress_ring::dump();

        render_pause_dump_screen(snapshot);
        kernel::kprint!(
            " Press F12 to reboot immediately.                                             \n\n"
        );
        kernel::kprint!(
            " Scheduler task dump skipped in pause mode to avoid lock wedging.              \n"
        );

        crate::console::flush_sync();
        crate::console::serial_flush_sync();
    }

    loop {
        unsafe {
            core::arch::asm!("hlt", options(nomem, nostack));
        }
    }
}

fn current_task_name() -> [u8; 32] {
    unsafe { kernel::sched::current_task_name_current() }
}

fn current_task_name_str(name: &[u8; 32]) -> &str {
    let len = name.iter().position(|&b| b == 0).unwrap_or(name.len());
    core::str::from_utf8(&name[..len]).unwrap_or("unknown")
}

fn print_irq_trace_summary() {
    let mut events = [TraceEvent::Empty; 12];
    let count = if let Some(ring) = kernel::trace::irq_ring::IRQ_RING.try_lock() {
        ring.read_all(&mut events)
    } else {
        kernel::kprint!("Recent IRQ/Trace Events:\n");
        kernel::kprint!("  <unavailable: ring lock busy during pause dump>\n");
        return;
    };

    kernel::kprint!("Recent IRQ/Trace Events:\n");
    if count == 0 {
        kernel::kprint!("  <none>\n");
        return;
    }

    for event in &events[..count] {
        match *event {
            TraceEvent::Empty => {}
            TraceEvent::TimerTick { timestamp } => {
                kernel::kprint!("  timer      t={}\n", timestamp);
            }
            TraceEvent::Irq { vector, timestamp } => {
                kernel::kprint!("  irq  0x{:02x}  t={}\n", vector, timestamp);
            }
            TraceEvent::ContextSwitch { from, to, timestamp } => {
                kernel::kprint!("  switch {} -> {}  t={}\n", from, to, timestamp);
            }
            TraceEvent::PreemptDisable { depth, timestamp } => {
                kernel::kprint!("  preempt depth={}  t={}\n", depth, timestamp);
            }
            TraceEvent::Heartbeat { cpu, tid, runq_len, timestamp } => {
                kernel::kprint!(
                    "  heartbeat cpu={} tid={} runq={} t={}\n",
                    cpu,
                    tid,
                    runq_len,
                    timestamp
                );
            }
            TraceEvent::SchedSwitch { cpu, from_tid, to_tid, timestamp } => {
                kernel::kprint!(
                    "  sched_switch cpu={} {} -> {} t={}\n",
                    cpu,
                    from_tid,
                    to_tid,
                    timestamp
                );
            }
            TraceEvent::VfsRpc { enter, op_tag, timestamp } => {
                kernel::kprint!("  vfs_rpc enter={} op={} t={}\n", enter, op_tag, timestamp);
            }
            TraceEvent::InputSummary { source, events, bytes, drops, timestamp } => {
                kernel::kprint!(
                    "  input source={} events={} bytes={} drops={} t={}\n",
                    source,
                    events,
                    bytes,
                    drops,
                    timestamp
                );
            }
        }
    }
}

fn print_pause_cpu_summary(cpu_total: usize) {
    let limit = cpu_total.min(MAX_PAUSE_CPUS);
    kernel::kprint!("CPU Snapshot Summary:\n");
    for cpu in 0..limit {
        if PAUSE_CPU_VALID[cpu].load(Ordering::SeqCst) {
            kernel::kprint!(
                "  CPU{} rip=0x{:016x} rsp=0x{:016x} rbp=0x{:016x} cs=0x{:016x} rflags=0x{:016x}\n",
                cpu,
                PAUSE_CPU_RIP[cpu].load(Ordering::SeqCst),
                PAUSE_CPU_RSP[cpu].load(Ordering::SeqCst),
                PAUSE_CPU_RBP[cpu].load(Ordering::SeqCst),
                PAUSE_CPU_CS[cpu].load(Ordering::SeqCst),
                PAUSE_CPU_RFLAGS[cpu].load(Ordering::SeqCst),
            );
        } else {
            kernel::kprint!("  CPU{} <no interrupt snapshot>\n", cpu);
        }
    }
}

fn render_pause_dump_screen(snapshot: Option<&IrqRegisterSnapshot>) {
    let runtime = kernel::runtime_base();
    let cpu = runtime.current_cpu_index();
    let cpu_total = runtime.cpu_total_count();
    let tid = unsafe { kernel::sched::current_tid_current() };
    let task_name = current_task_name();
    let task_name = current_task_name_str(&task_name);
    let ticks = runtime.mono_ticks();
    let irq1 = IRQ1_COUNT.load(Ordering::Relaxed);
    let irq12 = IRQ12_COUNT.load(Ordering::Relaxed);
    let irq4 = IRQ4_COUNT.load(Ordering::Relaxed);
    let (rsp, rbp, cr2, cr3) = capture_control_state();

    kernel::kprint!("\x1b[0m\x1b[2J\x1b[H\x1b[?25l");
    kernel::kprint!("\x1b[37;44;1m");
    kernel::kprint!(
        "                                                                                \n"
    );
    kernel::kprint!(
        "  Thing-OS Kernel Pause And Dump                                               \n"
    );
    kernel::kprint!(
        "                                                                                \n"
    );
    kernel::kprint!("\x1b[0m\x1b[37;44m");
    kernel::kprint!(
        " Alt+F12 was pressed on a PS/2 keyboard. The kernel has stopped all CPUs.      \n"
    );
    kernel::kprint!(
        " Power cycle or reboot is required to continue.                                 \n"
    );
    kernel::kprint!(
        "                                                                                \n"
    );
    kernel::kprint!(
        " CPU      : {:<3} / {:<3}                                                      \n",
        cpu,
        cpu_total
    );
    kernel::kprint!(" TID      : {:<16}                                                   \n", tid);
    kernel::kprint!(" Task     : {:<60}\n", task_name);
    kernel::kprint!(" Ticks    : {:<60}\n", ticks);
    kernel::kprint!(
        " IRQ cnts : kbd={:<10} mouse={:<10} serial={:<10}             \n",
        irq1,
        irq12,
        irq4
    );
    kernel::kprint!(" RSP/RBP  : 0x{:016x} / 0x{:016x}                              \n", rsp, rbp);
    kernel::kprint!(" CR2/CR3  : 0x{:016x} / 0x{:016x}                              \n", cr2, cr3);
    kernel::kprint!(
        "                                                                                \n"
    );
    kernel::kprint!("\x1b[0m\n");
    if let Some(snapshot) = snapshot {
        print_register_snapshot(snapshot);
        kernel::kprint!("\n");
        print_stack_words(snapshot as *const IrqRegisterSnapshot as u64);
    } else {
        print_stack_words(rsp);
    }
    kernel::kprint!("\n");
    print_frame_pointer_walk(rbp);
    kernel::kprint!("\n");
    print_pause_cpu_summary(cpu_total);
    kernel::kprint!("\n");
    print_irq_trace_summary();
    kernel::kprint!("\n");
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_pause_dump_ipi_handler(snapshot: &IrqRegisterSnapshot) -> ! {
    unsafe {
        core::arch::asm!("cli", options(nomem, nostack, preserves_flags));
    }

    let runtime = kernel::runtime_base();
    let cpu = runtime.current_cpu_index();
    let (rsp, rbp, _, _) = capture_control_state();
    store_pause_cpu_snapshot(cpu, snapshot, rsp, rbp);

    crate::arch::x86_64::hcf()
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_nmi_handler(snapshot: &IrqRegisterSnapshot) {
    if PAUSE_DUMP_ACTIVE.load(Ordering::SeqCst) {
        let runtime = kernel::runtime_base();
        let cpu = runtime.current_cpu_index();
        let (rsp, rbp, _, _) = capture_control_state();
        store_pause_cpu_snapshot(cpu, snapshot, rsp, rbp);
        if PAUSE_DUMP_OWNER_CPU.load(Ordering::SeqCst) == cpu as u64 {
            if capture_pause_reboot_hotkey(32) {
                unsafe {
                    kernel::logging::force_unlock();
                }
                runtime.reboot();
            }
            return;
        }
        crate::arch::x86_64::hcf();
    }

    IRQ1_COUNT.fetch_add(1, Ordering::Relaxed);
    let (pause_dump, _f12_press, ctrl_alt_del, _log_level, captured) =
        capture_ps2_keyboard(32, false, false);
    if ctrl_alt_del {
        reboot_from_ctrl_alt_del();
    }

    if captured != 0 {
        kernel::irq::dispatch_irq(0x21);
    }

    if pause_dump {
        trigger_pause_dump(Some(snapshot));
    }
}

/// Hardware IRQ handler - dispatches to kernel and sends EOI
#[unsafe(no_mangle)]
pub extern "C" fn rust_irq_handler(vector: u64, irq_snapshot: *const IrqRegisterSnapshot) {
    // Dedicated shims pass the exact vector. Generic shared stubs still fall back
    // to LAPIC ISR probing until they are split out into per-vector handlers.
    let resolved = if vector != 0 {
        vector as u8
    } else {
        crate::arch::x86_64::ioapic::lapic_in_service_vector().unwrap_or(0)
    };

    if resolved == 0 {
        return;
    }

    // Record IRQ entry in the progress ring for freeze post-mortems.
    let irq_entry_ns = kernel::trace::now_or_zero();
    let irq_cpu = kernel::runtime_base().current_cpu_index();
    kernel::trace::progress_ring::push(
        kernel::trace::progress_ring::ProgressTag::IrqEntry,
        irq_cpu,
        resolved as u64,
        irq_entry_ns,
    );

    let mut pause_dump = false;
    let mut f12_press = false;
    let mut ctrl_alt_del = false;
    let mut ps2_irq_count = 0u64;
    let mut ps2_irq_start_ticks = 0u64;

    if resolved == 0x21 {
        let count = IRQ1_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
        ps2_irq_count = count;
        ps2_irq_start_ticks = ps2_irq_trace_enter(resolved, count, irq_snapshot);
        let (pause, f12, reboot) = capture_ps2_keyboard_irq();
        pause_dump = pause;
        f12_press = f12;
        ctrl_alt_del = reboot;
    }

    if resolved == 0x2C {
        let count = IRQ12_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
        ps2_irq_count = count;
        ps2_irq_start_ticks = ps2_irq_trace_enter(resolved, count, irq_snapshot);
        let _ = capture_ps2_keyboard(32, false, false);
    }

    // Send EOI to Local APIC early to avoid wedging during context switch
    crate::arch::x86_64::ioapic::send_eoi();

    if (resolved >= 0x20 && resolved <= 0x2F) || (resolved >= 0xF0) {
        crate::arch::x86_64::pic::send_eoi(resolved);
    }

    if ctrl_alt_del {
        reboot_from_ctrl_alt_del();
    }

    if pause_dump {
        let snapshot = if irq_snapshot.is_null() { None } else { Some(unsafe { &*irq_snapshot }) };
        trigger_pause_dump(snapshot);
    }

    if f12_press {
        activate_terminal_and_spawn_shell();
    }

    if resolved == 0x24 {
        let count = IRQ4_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
        /*
        if count <= 3 || (count % 128 == 0) {
            kinfo!("IRQ4 (serial) fired (count={})", count);
        }
        */
    }

    // IRQ_TIMER_VECTOR or IRQ_RESCHED_VECTOR is our preemption heartbeat
    if resolved == IRQ_TIMER_VECTOR {
        let (fallback_pause, fallback_reboot) = poll_ps2_keyboard_fallback();
        if fallback_reboot {
            reboot_from_ctrl_alt_del();
        }
        if !pause_dump && fallback_pause {
            let snapshot =
                if irq_snapshot.is_null() { None } else { Some(unsafe { &*irq_snapshot }) };
            trigger_pause_dump(snapshot);
        }
        kernel::sched::on_tick::<crate::arch::CurrentRuntime>();
        crate::console::flush_deferred();
        crate::console::serial_flush_deferred();
        crate::console::blink_cursor();
        // Boot display path disabled.
        // let now_ticks: u64;
        // unsafe {
        //     let low: u32;
        //     let high: u32;
        //     core::arch::asm!("rdtsc", out("eax") low, out("edx") high, options(nostack, nomem));
        //     now_ticks = ((high as u64) << 32) | (low as u64);
        // }
        // crate::theme::try_tick(now_ticks);
    } else if resolved == 0x24 {
        // Serial interrupt (COM1 IRQ4) — handles both RX and TX:
        // 1. Poll received bytes into the serial RX buffer
        crate::RUNTIME.arch.poll_serial();
        // 2. Drain the deferred TX ring into the UART FIFO (THRE interrupt).
        // Console RX is transferred into the TTY from the scheduler tick path;
        // doing scheduler wakeups from the UART interrupt path is not safe.
        crate::console::serial_drain_irq();
    } else if resolved == IRQ_RESCHED_VECTOR {
        kernel::sched::on_resched_ipi::<crate::arch::CurrentRuntime>();
    } else if resolved == IRQ_TLB_SHOOTDOWN_VECTOR {
        // Full TLB flush on local CPU (including Global pages)
        crate::arch::x86_64::paging::tlb_flush_all();
    } else {
        kernel::irq::dispatch_irq(resolved);
    }

    if resolved == 0x21 || resolved == 0x2C {
        ps2_irq_trace_exit(resolved, ps2_irq_count, ps2_irq_start_ticks);
    }

    // Record IRQ exit in the progress ring.
    kernel::trace::progress_ring::push(
        kernel::trace::progress_ring::ProgressTag::IrqExit,
        irq_cpu,
        resolved as u64,
        kernel::trace::now_or_zero(),
    );
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_pf_handler(frame: &InterruptStackFrame, snapshot: &IrqRegisterSnapshot) {
    let cr2: u64;
    unsafe {
        core::arch::asm!("mov {}, cr2", out(reg) cr2);
    }

    let is_user = frame.cs & 3 != 0;

    if is_user {
        unsafe {
            unsafe extern "C" {
                fn kernel_handle_page_fault(rip: u64, addr: u64, err: u64);
            }
            kernel_handle_page_fault(frame.rip, cr2, frame.error_code);
        }
        return;
    }

    // Kernel mode fault - safe to panic, but avoid kerror! which can deadlock
    kernel::kerror!(
        "KERNEL PAGE FAULT at 0x{:x} RIP=0x{:x} CS=0x{:x} ERR=0x{:x} RSP=0x{:x}",
        cr2,
        frame.rip,
        frame.cs,
        frame.error_code,
        frame.rsp
    );
    kernel::kerror!("Registers:");
    kernel::kerror!(
        "  RAX: 0x{:016x} RBX: 0x{:016x} RCX: 0x{:016x} RDX: 0x{:016x}",
        snapshot.rax,
        snapshot.rbx,
        snapshot.rcx,
        snapshot.rdx
    );
    kernel::kerror!(
        "  RSI: 0x{:016x} RDI: 0x{:016x} RBP: 0x{:016x} RSP: 0x{:016x}",
        snapshot.rsi,
        snapshot.rdi,
        snapshot.rbp,
        frame.rsp
    );
    kernel::kerror!(
        "  R8:  0x{:016x} R9:  0x{:016x} R10: 0x{:016x} R11: 0x{:016x}",
        snapshot.r8,
        snapshot.r9,
        snapshot.r10,
        snapshot.r11
    );
    kernel::kerror!(
        "  R12: 0x{:016x} R13: 0x{:016x} R14: 0x{:016x} R15: 0x{:016x}",
        snapshot.r12,
        snapshot.r13,
        snapshot.r14,
        snapshot.r15
    );
    kernel::kerror!(
        "  RIP: 0x{:016x} CS:  0x{:016x} RFLAGS: 0x{:016x}",
        frame.rip,
        frame.cs,
        frame.rflags
    );

    print_stack_trace();

    // Dump first few words of stack
    unsafe {
        let stack_ptr = frame.rsp as *const u64;
        if !stack_ptr.is_null() && (frame.rsp >= 0xffffffff80000000) {
            kernel::kerror!("Stack Dump:");
            for i in 0..8 {
                let val = *stack_ptr.add(i);
                kernel::kerror!("  [+{:02x}] 0x{:016x}", i * 8, val);
            }
        }
    }

    // Dump bytes at RIP if possible
    unsafe {
        let rip_ptr = frame.rip as *const u8;
        if !rip_ptr.is_null() && (frame.rip >= 0xffffffff80000000) {
            kernel::kerror!("Code at RIP:");
            let mut line = alloc::string::String::new();
            for i in 0..16 {
                use core::fmt::Write;
                let _ = write!(line, "{:02x} ", *rip_ptr.add(i));
            }
            kernel::kerror!("  {}", line);
        }
    }

    panic!("KERNEL PAGE FAULT");
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_gp_handler(frame: &InterruptStackFrame) -> ! {
    if frame.cs & 3 != 0 {
        unsafe {
            unsafe extern "C" {
                fn kernel_handle_exception(rip: u64, error_code: u64, rsp: u64, cs: u64, kind: u64);
            }
            kernel_handle_exception(frame.rip, frame.error_code, frame.rsp, frame.cs, 13);
            loop {
                core::arch::asm!("hlt");
            }
        }
    } else {
        kernel::kerror!(
            "KERNEL GPF at RIP=0x{:x} CS=0x{:x} ERR=0x{:x} RSP=0x{:x}",
            frame.rip,
            frame.cs,
            frame.error_code,
            frame.rsp
        );
        print_stack_trace();
        panic!("KERNEL GPF");
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_invalid_opcode_handler(frame: &InterruptStackFrame) -> ! {
    if frame.cs & 3 != 0 {
        // Diagnostic: Print the bytes at the faulting RIP
        unsafe {
            let rip = frame.rip as *const u8;
            let mut bytes = [0u8; 8];
            core::ptr::copy_nonoverlapping(rip, bytes.as_mut_ptr(), 8);
            kernel::kdebug!("USER-UD: rip=0x{:x} bytes={:02x?}", frame.rip, bytes);
        }

        unsafe {
            unsafe extern "C" {
                fn kernel_handle_exception(rip: u64, error_code: u64, rsp: u64, cs: u64, kind: u64);
            }
            kernel_handle_exception(frame.rip, frame.error_code, frame.rsp, frame.cs, 6);
            loop {
                core::arch::asm!("hlt");
            }
        }
    } else {
        panic!(
            "KERNEL INVALID OPCODE at RIP=0x{:x} CS=0x{:x} RSP=0x{:x}",
            frame.rip, frame.cs, frame.rsp
        );
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_div0_handler(frame: &InterruptStackFrame) -> ! {
    if frame.cs & 3 != 0 {
        unsafe {
            unsafe extern "C" {
                fn kernel_handle_exception(rip: u64, error_code: u64, rsp: u64, cs: u64, kind: u64);
            }
            kernel_handle_exception(frame.rip, frame.error_code, frame.rsp, frame.cs, 0);
            loop {
                core::arch::asm!("hlt");
            }
        }
    } else {
        panic!(
            "KERNEL DIVIDE BY ZERO at RIP=0x{:x} CS=0x{:x} RSP=0x{:x}",
            frame.rip, frame.cs, frame.rsp
        );
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_double_fault_handler(frame: &InterruptStackFrame) -> ! {
    panic!(
        "DOUBLE FAULT at RIP=0x{:x} CS=0x{:x} ERR=0x{:x} RSP=0x{:x}",
        frame.rip, frame.cs, frame.error_code, frame.rsp
    );
}

fn print_stack_trace() {
    let mut rbp: u64;
    unsafe {
        core::arch::asm!("mov {}, rbp", out(reg) rbp);
    }
    kernel::kerror!("Backtrace (current rbp=0x{:x}):", rbp);
    let mut depth = 0;
    // Walk up the stack. Kernel stack is usually in high memory (HHDM or kernel space).
    while rbp >= 0xffffffff80000000 && depth < 32 {
        let prev_rbp_ptr = rbp as *const u64;
        let ret_addr_ptr = (rbp + 8) as *const u64;

        unsafe {
            // Basic sanity check before dereferencing: must be 8-byte aligned and in kernel memory
            if (rbp % 8 != 0) || rbp < 0xffffffff80000000 {
                break;
            }

            let prev_rbp = *prev_rbp_ptr;
            let ret_addr = *ret_addr_ptr;

            kernel::kerror!("  [{:02}] 0x{:016x} (frame=0x{:x})", depth, ret_addr, rbp);

            if prev_rbp <= rbp {
                // Prevent infinite loops or downward walks.
                // In a proper frame-pointer chain, the previous RBP is always at a higher address.
                break;
            }
            rbp = prev_rbp;
        }
        depth += 1;
    }
}
