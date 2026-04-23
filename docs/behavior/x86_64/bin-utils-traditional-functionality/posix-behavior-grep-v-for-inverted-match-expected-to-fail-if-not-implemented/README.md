# ❌ Scenario: POSIX behavior - grep -v for inverted match (expected to fail if not implemented)

> Last run: 2026-04-22 21:17:04

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8513ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 253ms | - [📜](./02/serial.log) - |
| 3 | And I type "echo -e 'a\nb\nc' | grep -v b" on the serial console | ✅ | 537ms | - [📜](./03/serial.log) - |
| 4 | Then the serial output should contain "a" within 5s | ✅ | 0ms | - - - |
| 5 | And the serial output should contain "c" within 5s | ✅ | 0ms | - - - |
| 6 | And the latest serial output should not contain "b" | ❌ | 1006ms | - [📜](./06/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26512973157] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26538158262] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26569053654] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26599448007] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26618478348] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26620920084] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26654401323] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26673321510] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26717591241] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26718540222] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26796623667] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26863617165] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26884446633] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26954410395] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[27052918299] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[27077799144] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[27099248418] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[27208041696] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[27254448441] [[32mINFO [0m] [ke] [CPUrnel::boot_progress0] boot_progress: milestone="Spawning Sprout"
[27404265306] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27682701387] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=944889660 elapsed_us=472444
[27683415771] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27752717421] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27840434688] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27841318461] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27921806451] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27923884890] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27941101815] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=28974
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28244898297] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mounting /hosts early...
[28283314455] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned early /hosts mount (PID=9)
[28287167436] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet �� retrying
[28401360999] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: starting (mount_point=/hosts)
[28416538491] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: mounted at /hosts
[28422593991] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: advertising hostname thingos.local
[28424510334] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: serving cache immediately; mDNS socket will attach when netd is ready
[28557286626] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
echo -e mounted type=mdns device=none target=/hosts
'a\nb\nc' | grep -v b
[30294982872] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='echo'
[30360055539] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='grep'
[?25la
c

[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[30745347897] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[30746299947] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing primary port (0x1F0)...
[30772719054] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing secondary port (0x170)...
[30798848949] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[30799649595] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Entering RPC service loop
[30800456247] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: No active devices to service
[31540545003] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[32879977020] [[32mINFO [0m] [netd] [CPU3] NETD: Starting network service...
[32908371936] [[32mINFO [0m] [netd] [CPU3] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[33283051923] 
```
</details>
