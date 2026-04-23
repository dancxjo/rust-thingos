# ❌ Scenario: Repeat cd into https and cat example content three times

> Last run: 2026-04-22 21:44:56

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8415ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 252ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https example.com /https/ex" on the serial console | ✅ | 1677ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ✅ | 0ms | - [📜](./04/serial.log) - |
| 5 | And I type "cat /https/ex/@index" on the serial console | ✅ | 394ms | - [📜](./05/serial.log) - |
| 6 | And I type "cat /https/ex/@index" on the serial console | ✅ | 393ms | - [📜](./06/serial.log) - |
| 7 | And I type "cat /https/ex/@index" on the serial console | ✅ | 389ms | - [📜](./07/serial.log) - |
| 8 | Then the command output should contain "<html" | ❌ | 6076ms | - [📜](./08/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26153932629] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26181029655] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26214754665] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26248214619] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26267744217] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26270820642] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26306122491] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26326232889] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26375008539] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26376079620] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26462132565] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26530599579] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26552418849] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26630353629] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26728980564] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26755323408] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26778185214] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[26890619811] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[26941809840] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[27094190013] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27378964305] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=982031490 elapsed_us=491015
[27379802472] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27450202329] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27517829031] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27518812068] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27606714663] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27608882730] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27619568163] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=19536
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[27829034112] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mounting /hosts early...
[27857916273] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned early /hosts mount (PID=9)
[27860344017] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[27892829448] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[28002560850] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: starting (mount_point=/hosts)
[28020524994] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: mounted at /hosts
[28028700711] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: advertising hostname thingos.local
[28030755093] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: serving cache immediately; mDNS socket will attach when netd is ready
mounmounted type=mdns device=none target=/hosts
t -t https example.com /https/ex[30404236005] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[30405396252] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Probing primary port (0x1F0)...
[30434477502] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Probing secondary port (0x170)...
[30460557864] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[30462339270] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Entering RPC service loop
[30463521264] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: No active devices to service
[30847043634] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[32251576005] [[32mINFO [0m] [netd] [CPU1] NETD: Starting network service...
[32283052098] [[32mINFO [0m] [netd] [CPU1] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[32756849895] [[32mINFO [0m] [netd] [CPU1] NETD: Network ready

[33198118668] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='mount'
[?25l[33469426584] [[32mINFO [0m] [httpsd] [CPU3] HTTPSD_READY
[33470628015] [[32mINFO [0m] [httpsd] [CPU3] httpsd: entering main RPC loop for /https/ex
[33743911047] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=4
[33745064067] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path=''
[33745986384] [[32mINFO [0m] [httpsd] [CPU3] httpsd: resolve_path path=''
[33782417526] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Close payload_len=8
mounted type=https device=example.com target=/https/ex
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/ex/@index
[35005218600] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[?25l[35060953323] [[32mINFO [0m] [cat] [CPU2] cat: opening '/https/ex/@index'
[35067393702] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=10
[35068234509] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path='@index'
[35068937409] [[32mINFO [0m] [httpsd] [CPU3] httpsd: resolve_path path='@index'
[35076046368] [[32mINFO [0m] [cat] [CPU2] cat: opened '/https/ex/@index' fd=8
[35086134633] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Poll payload_len=12
[35103191541] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[35118069591] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/ex/@index
[36298983963] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[?25l[36356546457] [[32mINFO [0m] [cat] [CPU3] cat: opening '/https/ex/@index'
[36367503018] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=10
[36368355441] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path='@index'
[36369057417] [[32mINFO [0m] [httpsd] [CPU3] httpsd: resolve_path path='@index'
[36379792812] [[32mINFO [0m] [cat] [CPU3] cat: opened '/https/ex/@index' fd=9
[36387694398] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Poll payload_len=12
[36408427671] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[36423619419] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/ex/@index
[37588487739] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[?25l[37648322844] [[32mINFO [0m] [cat] [CPU1] cat: opening '/https/ex/@index'
[37659473247] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Lookup payload_len=10
[37660311414] [[32mINFO [0m] [httpsd] [CPU1] httpsd: dispatch_lookup path='@index'
[37661038107] [[32mINFO [0m] [httpsd] [CPU1] httpsd: resolve_path path='@index'
[37672465182] [[32mINFO [0m] [cat] [CPU1] cat: opened '/https/ex/@index' fd=10
[37680617634] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Poll payload_len=12
[37701802941] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[37721950761] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[49293346806] [[31;1mER
```
</details>
