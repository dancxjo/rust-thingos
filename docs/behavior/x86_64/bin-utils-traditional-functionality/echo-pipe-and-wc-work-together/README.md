# ✅ Scenario: echo, pipe, and wc work together

> Last run: 2026-04-22 21:17:04

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8416ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 254ms | - [📜](./02/serial.log) - |
| 3 | And I type "echo hello world | wc -w" on the serial console | ✅ | 456ms | - [📜](./03/serial.log) - |
| 4 | Then the serial output should contain "2" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26369597364] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26394400461] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26424901668] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26455254045] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26473830108] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26476273296] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26509611150] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26528499426] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26573705862] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26574664182] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26654029710] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26719872696] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26740149843] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26818823757] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26918236884] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26944167228] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26966532615] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[27076501452] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[27122989311] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[27269983785] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27548126991] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=954269382 elapsed_us=477134
[27548852991] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27618113787] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27683943936] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27684913938] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27765871188] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27767913096] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27777889194] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=20691
[1;32m
        .-.
       /   \        [1;36mTHING-OS[1;32m
      |     |       [0;36m"People, places, things."[1;32m
       \   /        
        `-'        
       /   \        v0.1  ��  
      |     |       2026-04-16
       \   /
        `-'
[0m
[2m--------------------------------------------------------------[0m
[1m sprout has taken root. the system is awake.[0m

  try:
    [36mls /bin[0m       browse available shoots
    [36mps[0m            observe living processes
    [36mcat /version[0m  inspect the genome

[2m--------------------------------------------------------------[0m
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[27984637593] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mounting /hosts early...
[28011947667] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned early /hosts mount (PID=9)
[28013256645] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[28132575141] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: starting (mount_point=/hosts)
[28153019994] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: mounted at /hosts
[28154379594] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[28159594452] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: advertising hostname thingos.local
[28161495483] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: serving cache immediately; mDNS socket will attach when netd is ready
echo mounted type=mdns device=none target=/hosts
hello world | wc -w
[29711942106] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='echo'
[29786292657] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='wc'
[?25l       2
[?25h[
```
</details>
