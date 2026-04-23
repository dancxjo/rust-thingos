# ❌ Scenario: Repeat cd into https and cat example content three times

> Last run: 2026-04-23 14:53:37

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 9119ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 254ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https example.com /https/ex" on the serial console | ✅ | 1985ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ✅ | 0ms | - - - |
| 5 | And I type "cat /https/ex/@index" on the serial console | ✅ | 387ms | - [📜](./05/serial.log) - |
| 6 | And I type "cat /https/ex/@index" on the serial console | ✅ | 391ms | - [📜](./06/serial.log) - |
| 7 | And I type "cat /https/ex/@index" on the serial console | ✅ | 391ms | - [📜](./07/serial.log) - |
| 8 | Then the command output should contain "<html" | ❌ | 6077ms | - [📜](./08/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[28065644112] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[28103090730] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[28142595426] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[28185910335] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[28214908491] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[28218017586] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[28265185509] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[28295470500] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[28348966338] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[28350066162] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[28449910599] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[28534234212] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[28565421918] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[28649138199] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[28775624658] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[28811776818] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[28844024484] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[28972017723] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[29025335922] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[29211211887] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[29541496215] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1159049727 elapsed_us=579524
[29542313394] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[29627343801] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[29701365903] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[29702461965] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[29800312113] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[29802595779] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[29818954803] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=21384
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[30096924858] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[30098471205] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[30156587274] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet ��� retrying
mount -t https example.com /https/ex
[32653585173] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='mount'
[32996837709] [[32mINFO [0m] [httpsd] [CPU3] HTTPSD_READY
[32997996801] [[32mINFO [0m] [httpsd] [CPU3] httpsd: entering main RPC loop for /https/ex
[?25l[33284494056] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=4
[33286086306] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path=''
[33287351196] [[32mINFO [0m] [httpsd] [CPU3] httpsd: resolve_path path=''
[33313205211] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Close payload_len=8
[33316520820] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
mounted type=https device=example.com target=/https/ex
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[33686090856] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[33687650139] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[33752852364] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
[33795620364] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[33796458729] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[33797284356] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
[36927117381] [[32mINFO [0m] [netd] [CPU1] NETD: Starting network service...
[36968954748] [[32mINFO [0m] [netd] [CPU1] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[37156204404] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=16)
cat /h[37553871465] [[32mINFO [0m] [netd] [CPU1] NETD: Network ready
ttps/ex/@index
[38327612103] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[?25l[38388162417] [[32mINFO [0m] [cat] [CPU2] cat: opening '/https/ex/@index'
[38395413342] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=10
[38397054003] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path='@index'
[38398285662] [[32mINFO [0m] [httpsd] [CPU3] httpsd: resolve_path path='@index'
[38407546881] [[32mINFO [0m] [cat] [CPU2] cat: opened '/https/ex/@index' fd=8
[38432732943] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Poll payload_len=12
[38450258913] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[38466374397] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/ex/@index
[39615416181] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[?25l[39677110902] [[32mINFO [0m] [cat] [CPU3] cat: opening '/https/ex/@index'
[39686672223] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=10
[39687425250] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path='@index'
[39688074261] [[32mINFO [0m] [httpsd] [CPU3] httpsd: resolve_path path='@index'
[39699634722] [[32mINFO [0m] [cat] [CPU3] cat: opened '/https/ex/@index' fd=9
[39708126909] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Poll payload_len=12
[39724700334] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[39745356156] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/ex/@index
[40904931540] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[?25l[40963113576] [[32mINFO [0m] [cat] [CPU1] cat: opening '/https/ex/@index'
[40978503258] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=10
[40979346276] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path='@index'
[40979970702] [[32mINFO [0m] [httpsd] [CPU3] httpsd: resolve_path path='@index'
[40986448008] [[32mINFO [0m] [cat] [CPU1] cat: opened '/https/ex/@index' fd=10
[40995882873] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Poll payload_len=12
[41018423721] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[41036935434] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[55181558949
```
</details>
