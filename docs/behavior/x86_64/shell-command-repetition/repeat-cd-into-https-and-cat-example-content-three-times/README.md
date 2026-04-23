# ❌ Scenario: Repeat cd into https and cat example content three times

> Last run: 2026-04-23 11:14:17

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 13507ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 359ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https example.com /https/ex" on the serial console | ✅ | 1344ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ✅ | 0ms | - - - |
| 5 | And I type "cat /https/ex/@index" on the serial console | ✅ | 518ms | - [📜](./05/serial.log) - |
| 6 | And I type "cat /https/ex/@index" on the serial console | ✅ | 658ms | - [📜](./06/serial.log) - |
| 7 | And I type "cat /https/ex/@index" on the serial console | ✅ | 553ms | - [📜](./07/serial.log) - |
| 8 | Then the command output should contain "<html" | ❌ | 6088ms | - [📜](./08/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[41932166727] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[41964239691] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[42019621677] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[42070189887] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[42099248796] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[42104233644] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[42159477360] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[42188004573] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[42267203286] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[42268353633] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[42420953388] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[42527657568] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[42563490519] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[42658988064] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[42787307607] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[42858548304] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[42907600956] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[43161171669] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[43274566203] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[43552915164] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[44049905823] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1757047446 elapsed_us=878523
[44062416618] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[44236091229] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[44466414663] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[44487776421] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[44648556645] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[44652554430] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[44685105828] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=48015
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[45062266854] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mounting /hosts early...
[45192835215] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned early /hosts mount (PID=9)
[45272406630] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[45273224832] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet ��� retrying
mou[45637192029] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: starting (mount_point=/hosts)
[45694309716] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: mounted at /hosts
n[45709857336] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: advertising hostname thingos.local
t[45715369986] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: serving cache immediately; mDNS socket will attach when netd is ready
 -t https exmounted type=mdns device=none target=/hosts
ample.com /https/ex
[47460354612] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='mount'
[?25l[47878512198] [[32mINFO [0m] [httpsd] [CPU1] HTTPSD_READY
[47880532425] [[32mINFO [0m] [httpsd] [CPU1] httpsd: entering main RPC loop for /https/ex
[48187669728] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Lookup payload_len=4
[48197107332] [[32mINFO [0m] [httpsd] [CPU1] httpsd: dispatch_lookup path=''
[48198876297] [[32mINFO [0m] [httpsd] [CPU1] httpsd: resolve_path path=''
[49571306499] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[49573170405] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Probing primary port (0x1F0)...
[49634286603] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Probing secondary port (0x170)...
[49700295843] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Close payload_len=8
[49715594412] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[49717145049] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Entering RPC service loop
[49718931141] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: No active devices to service
mounted type=https device=example.com target=/https/ex
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /ht[50287240377] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
tps/ex/@index
[51069583521] [[32mINFO [0m] [sh] [CPU1] sh: spawning job cmd='cat'
[?25l[51220905120] [[32mINFO [0m] [cat] [CPU1] cat: opening '/https/ex/@index'
[51241180584] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Lookup payload_len=10
[51243042972] [[32mINFO [0m] [httpsd] [CPU1] httpsd: dispatch_lookup path='@index'
[51245464215] [[32mINFO [0m] [httpsd] [CPU1] httpsd: resolve_path path='@index'
[51403850916] [[32mINFO [0m] [cat] [CPU1] cat: opened '/https/ex/@index' fd=8
[51426468951] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Poll payload_len=12
[51474242094] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[51513090618] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/ex/@index
[52763486325] [[32mINFO [0m] [sh] [CPU1] sh: spawning job cmd='cat'
[?25l[53434692663] [[32mINFO [0m] [netd] [CPU3] NETD: Starting network service...
[53577416310] [[32mINFO [0m] [cat] [CPU2] cat: opening '/https/ex/@index'
[53604017412] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Lookup payload_len=10
[53605289232] [[32mINFO [0m] [httpsd] [CPU1] httpsd: dispatch_lookup path='@index'
[53607121821] [[32mINFO [0m] [httpsd] [CPU1] httpsd: resolve_path path='@index'
[53621394849] [[32mINFO [0m] [netd] [CPU3] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[53624417055] [[32mINFO [0m] [cat] [CPU2] cat: opened '/https/ex/@index' fd=9
[53638056285] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Poll payload_len=12
[53682573318] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[53712047565] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/ex/@index
[54900585498] [[32mINFO [0m] [sh] [CPU1] sh: spawning job cmd='cat'
[?25l[55288241217] [[32mINFO [0m] [netd] [CPU3] NETD: Network ready
[55301869062] [[32mINFO [0m] [cat] [CPU0] cat: opening '/https/ex/@index'
[55340928489] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Lookup payload_len=10
[55342308285] [[32mINFO [0m] [httpsd] [CPU1] httpsd: dispatch_lookup path='@index'
[55343378541] [[32mINFO [0m] [httpsd] [CPU1] httpsd: resolve_path path='@index'
[55355803008] [[32mINFO [0m] [cat] [CPU0] cat: opened '/https/ex/@index' fd=10
[55382592441] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Poll payload_len=12
[55405160712] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[55472668020] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[
```
</details>
