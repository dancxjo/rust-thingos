// x86_64 Local APIC Access

use core::ptr;

const MSR_APIC_BASE: u32 = 0x1B;
const MSR_X2APIC_ID: u32 = 0x802;
const APIC_BASE_X2APIC_ENABLE: u64 = 1 << 10;

fn rdmsr(msr: u32) -> u64 {
    let low: u32;
    let high: u32;
    unsafe {
        core::arch::asm!(
            "rdmsr",
            in("ecx") msr,
            out("eax") low,
            out("edx") high,
            options(nostack, preserves_flags)
        );
    }
    ((high as u64) << 32) | (low as u64)
}

pub fn base_phys() -> u64 {
    let apic_base = rdmsr(MSR_APIC_BASE);
    // Mask out reserved bits (low 12 bits) to get physical base address
    apic_base & 0xFFFF_F000
}

pub fn id(base_phys: u64, hhdm: u64) -> u32 {
    if rdmsr(MSR_APIC_BASE) & APIC_BASE_X2APIC_ENABLE != 0 {
        return rdmsr(MSR_X2APIC_ID) as u32;
    }

    let id_reg = base_phys + hhdm + 0x20;
    let val = unsafe { ptr::read_volatile(id_reg as *const u32) };
    val >> 24
}
