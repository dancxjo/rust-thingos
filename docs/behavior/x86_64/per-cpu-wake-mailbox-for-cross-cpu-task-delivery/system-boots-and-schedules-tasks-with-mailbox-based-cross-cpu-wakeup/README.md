# ✅ Scenario: System boots and schedules tasks with mailbox-based cross-CPU wakeup

> Last run: 2026-04-25 20:10:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 0ms | - - - |
| 2 | When I wait for the system to boot | ✅ | 17676ms | - - - |
| 3 | Then the serial output should contain "Scheduler initialized" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[54208334070] [[32mINFO [0m] [kernel::boot_progress] [CPU0] bo[[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: mot_progress: milestone="Framebuffer Initialized"
[54809911398] ] [CPU0] Initializing global allocator...
[54936809631] [[32mIilestone="Memory Map OK"
[54859337544] [[32mINFO [0m] [kernelNFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[55008453126] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[55052761071] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[55058477001] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[55134535071] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[55138152135] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[55181935842] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[55214115693] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[55215767277] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[55294899429] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[55297257411] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[55487631243] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[55619698332] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[55671063723] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[55796205762] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[55951562997] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[56018456736] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
ot_progress: milestone="Modules Scanned"
[56103062037] [[32mINFO [0m] [kernel::boot_progress] [CPU0] bo[56374215810] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[56502157305] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[56868695400] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[57527399094] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=2185241487 elapsed_us=1092620
[57529010583] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[57739469601] 
```
</details>
