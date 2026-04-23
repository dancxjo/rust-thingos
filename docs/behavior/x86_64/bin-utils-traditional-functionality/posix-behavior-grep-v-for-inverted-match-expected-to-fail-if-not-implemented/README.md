# ❌ Scenario: POSIX behavior - grep -v for inverted match (expected to fail if not implemented)

> Last run: 2026-04-22 21:04:38

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 9531ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 252ms | - [📜](./02/serial.log) - |
| 3 | And I type "echo -e 'a\nb\nc' | grep -v b" on the serial console | ✅ | 585ms | - [📜](./03/serial.log) - |
| 4 | Then the serial output should contain "a" | ✅ | 1ms | - - - |
| 5 | And the serial output should contain "c" | ✅ | 0ms | - - - |
| 6 | And the latest serial output should not contain "b" | ❌ | 1008ms | - [📜](./06/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[29495532792] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[29527248399] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[29565057192] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[29603807244] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[29626650405] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[29629892721] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[29670344550] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[29694295884] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[29753175639] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[29754450825] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[29855006742] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[29933211693] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[29958867840] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[30040956495] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[30156445869] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[30188013009] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[30216076209] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[30352553484] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[30412132113] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[30595300923] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[30940812210] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1161684282 elapsed_us=580842
[30942535701] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[31036298568] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[31117049469] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[31118312577] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[31226569770] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[31230602931] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[31247914863] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=25641
[1;32m
        .-.
       /   \        [1;36mTHING-OS[1;32m
      |     |       [0;36m"People, places, things."[1;32m
       \   /        
        `-'        
       /   \        v0.1  •  
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[31518564297] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mounting /hosts early...
[31574207280] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned early /hosts mount (PID=9)
[31582990923] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet �� retrying
[31827406281] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: starting (mount_point=/hosts)
[31852269570] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: mounted at /hosts
[31860914316] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: advertising hostname thingos.local
[31866333939] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: serving cache immediately; mDNS socket will attach when netd is ready
echo -e mounted type=mdns device=none target=/hosts
'a\nb[32711865945] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
\nc' | grep -v b
[33615639288] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='echo'
[33705050016] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='grep'
[?25la
c

[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[34842254106] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[34843596744] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing primary port (0x1F0)...
[34883649933] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing secondary port (0x170)...
[34917122889] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[34918925151] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Entering RPC service loop
[34919961318] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: No active devices to service
[35262710769] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[37089233280] [[32mINFO [0
```
</details>
