# ❌ Scenario: grep filters piped input correctly

> Last run: 2026-04-24 12:44:06

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 16490ms | - - - |
| 2 | When I wait for the shell prompt | ❌ | 61054ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[50883177147] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[50933696385] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[51000661635] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[51067818450] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[51106755447] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[51115134510] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[51190558683] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[51232460730] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[51329990778] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[51332108520] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[51515865225] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[51639918165] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[51695004933] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[51806485698] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[52013359629] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[52133787255] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[52228984929] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[52486725951] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[52644554127] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[52969938153] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[53652342975] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=2272756959 elapsed_us=1136378
[53653995780] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[
```
</details>
