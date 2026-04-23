# ✅ Scenario: Scheduler loop entry is reached before deferred boot framebuffer paint

> Last run: 2026-04-22 21:17:04

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 0ms | - - - |
| 2 | When I wait for the system to boot | ✅ | 8513ms | - [📜](./02/serial.log) - |
| 3 | Then the serial output should contain "scheduler-entry total elapsed_ticks=" | ✅ | 0ms | - - - |
| 4 | And the serial log shows "Entering scheduler loop." after "Scheduler initialized" | ✅ | 501ms | - [📜](./04/serial.log) - |
| 5 | And the serial log shows "deferred_bootfb_gradient elapsed_ticks=" after "Entering scheduler loop." | ✅ | 501ms | - [📜](./05/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26532138501] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26559014460] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26592200415] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26624601432] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26644210791] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26646762219] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26681954541] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26701909179] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26749980081] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26751041526] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26834937921] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26903096319] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26924616774] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26999779818] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[27098988939] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[27128673462] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[27153115671] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[27266427309] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[27320960898] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[27478698192] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27776891406] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1005641670 elapsed_us=502820
[27777754719] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27855163776] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27960059127] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27961341672] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[28054074147] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[28057502814] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[28071472308] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=37125
[1;32m
        .-.
       /   \        [1;36mTHING-OS[1;32m
      |     |       [0;36m"People, places, things."[1;32m
       \   /        
        `-'        
       /   \        v0.1  ���  
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28294643124] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mounting /hosts early...
[28320030453] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned early /hosts mount (PID=9)
[28366624209] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet �� retrying
[28439789499] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: starting (mount_point=/hosts)
[28455681936] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: mounted at /hosts
[28461958602] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: advertising hostname thingos.local
[28463842110] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: serving cache immediately; mDNS socket will attach when netd is ready
[28898750166] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
mounted type=mdns device=none target=/hosts
[30840746178] [[32mINFO [0m] [ata_disk] [CPU2] ATA
```
</details>
