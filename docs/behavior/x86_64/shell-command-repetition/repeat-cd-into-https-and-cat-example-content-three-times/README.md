# ❌ Scenario: Repeat cd into https and cat example content three times

> Last run: 2026-04-23 14:40:47

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8709ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 252ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https example.com /https/ex" on the serial console | ✅ | 855ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ✅ | 0ms | - - - |
| 5 | And I type "cat /https/ex/@index" on the serial console | ✅ | 1314ms | - [📜](./05/serial.log) - |
| 6 | And I type "cat /https/ex/@index" on the serial console | ✅ | 391ms | - [📜](./06/serial.log) - |
| 7 | And I type "cat /https/ex/@index" on the serial console | ✅ | 391ms | - [📜](./07/serial.log) - |
| 8 | Then the command output should contain "<html" | ❌ | 6078ms | - [📜](./08/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[27092692242] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[27126787908] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[27161837670] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[27202370514] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[27230108862] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[27232699494] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[27276435648] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[27304771758] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[27352043499] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[27353040627] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[27444526263] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[27520481472] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[27550738083] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[27634294380] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[27747502266] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[27793767936] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[27825515289] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[27946671852] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[27995444400] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[28158791562] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[28464978630] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1083317433 elapsed_us=541658
[28465678164] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[28538636511] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[28608959907] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[28610006271] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[28700017797] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[28702234572] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[28713718143] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=22341
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28984389324] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[28985675961] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[29029073304] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet ��� retrying
mount -t https example.com /https/ex
[31287163512] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='mount'
[?25l[31610439399] [[32mINFO [0m] [httpsd] [CPU1] HTTPSD_READY
[31611652248] [[32mINFO [0m] [httpsd] [CPU1] httpsd: entering main RPC loop for /https/ex
[31936758843] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[31938294861] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[31945617627] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Lookup payload_len=4
[31947083982] [[32mINFO [0m] [httpsd] [CPU1] httpsd: dispatch_lookup path=''
[31948540569] [[32mINFO [0m] [httpsd] [CPU1] httpsd: resolve_path path=''
[31996960347] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
[32031523656] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[32032343046] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[32033122275] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
[32053557426] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Close payload_len=8
mounted type=https device=example.com target=/https/ex
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat [32354254086] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
/https/ex/@index
[33255182301] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[?25l[33326673336] [[32mINFO [0m] [cat] [CPU1] cat: opening '/https/ex/@index'
[33336135723] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=10
[33337318014] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path='@index'
[33338351343] [[32mINFO [0m] [httpsd] [CPU3] httpsd: resolve_path path='@index'
[33365800149] [[32mINFO [0m] [cat] [CPU1] cat: opened '/https/ex/@index' fd=8
[33372[33386621664] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[33400965081] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[36175314549] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[36228406137] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[36395270109] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=17)
cat /https/ex[37202065164] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
/@index
[37588178397] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[?25l[37657063983] [[32mINFO [0m] [cat] [CPU3] cat: opening '/https/ex/@index'
[37666431330] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=10
[37667260455] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path='@index'
[37667898774] [[32mINFO [0m] [httpsd] [CPU3] httpsd: resolve_path path='@index'
[37678733367] [[32mINFO [0m] [cat] [CPU3] cat: opened '/https/ex/@index' fd=9
[37687544796] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Poll payload_len=12
[37705146765] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Read payload_len=20
129417] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Poll payload_len=12
cat: error reading /https/ex/@index
[37724746752] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/ex/@index
[38871444249] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[?25l[38926315791] [[32mINFO [0m] [cat] [CPU1] cat: opening '/https/ex/@index'
[38933693667] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=10
[38934668322] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path='@index'
[38935372278] [[32mINFO [0m] [httpsd] [CPU3] httpsd: resolve_path path='@index'
[38942965809] [[32mINFO [0m] [cat] [CPU1] cat: opened '/https/ex/@index' fd=10
[38952631905] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Poll payload_len=12
[38961577743] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[38989373181] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[55836062634] [[
```
</details>
