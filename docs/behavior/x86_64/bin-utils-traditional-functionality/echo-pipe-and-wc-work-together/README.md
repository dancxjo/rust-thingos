# ❌ Scenario: echo, pipe, and wc work together

> Last run: 2026-04-24 12:44:06

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 15155ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ❌ | 61103ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[46725694521] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[46775883858] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[46842869436] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[46903760442] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[46939902933] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[46945076475] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[47053405839] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[47089078872] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[47185633737] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[47188176849] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[47406862602] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[47525118432] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[47567629296] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[47686544136] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[47833183893] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[47912887671] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[48045403791] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[48277564434] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[48424150764] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[48713084772] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[
```
</details>
