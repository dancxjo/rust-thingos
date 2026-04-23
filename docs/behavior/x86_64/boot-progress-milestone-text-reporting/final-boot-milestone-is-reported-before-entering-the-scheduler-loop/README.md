# ✅ Scenario: Final boot milestone is reported before entering the scheduler loop

> Last run: 2026-04-22 21:17:04

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 0ms | - - - |
| 2 | When I wait for the system to boot | ✅ | 8501ms | - - - |
| 3 | Then the serial log shows "Entering Scheduler" after "Spawning Sprout" | ✅ | 501ms | - [📜](./03/serial.log) - |
| 4 | And the serial log shows "Entering scheduler loop." after "Entering Scheduler" | ✅ | 501ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26328905229] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26355447228] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26386374531] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26417765253] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26436674154] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26439192615] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26472931287] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26492066667] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26537776848] [[3[26876735523] [[32mINFO [0m] 2mINFO [0m[kernel::boo] [kernel::sched] [CPU0] Scheduler initializt_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
ed
[26538773316] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26619671562] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26685445050] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26706080280] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26777069880] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26913424824] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26936425659] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[27051313806] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[27099183177] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[27257181765] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27560374512] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1001878581 elapsed_us=500939
[27561598977] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27640779639] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27710816463] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27712095114] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27796751466] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27798931446] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27806198211] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=20130
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28067989884] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mounting /hosts early...
[28094449053] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned early /hosts mount (PID=9)
[28103392383] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[28133129343] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[28233614574] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: starting (mount_point=/hosts)
[28253349696] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: mounted at /hosts
[28260105522] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: advertising hostname thingos.local
[28262168352] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: serving cache immediately; mDNS socket will attach when netd is ready
mounted type=mdns device=none target=/hosts
[30812589786] [[32mIN
```
</details>
