# ❌ Scenario: Repeat cd into https and cat example content three times

> Last run: 2026-04-23 15:01:11

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 9743ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 250ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https example.com /https/ex" on the serial console | ✅ | 868ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ✅ | 0ms | - - - |
| 5 | And I type "cat /https/ex/@index" on the serial console | ✅ | 1421ms | - [📜](./05/serial.log) - |
| 6 | And I type "cat /https/ex/@index" on the serial console | ✅ | 406ms | - [📜](./06/serial.log) - |
| 7 | And I type "cat /https/ex/@index" on the serial console | ✅ | 444ms | - [📜](./07/serial.log) - |
| 8 | Then the command output should contain "<html" | ❌ | 6083ms | - [📜](./08/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[30120830256] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[30159676602] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[30203003391] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[30250766667] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[30283418154] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[30286888236] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[30336241551] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[30367273101] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[30424144377] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[30425250669] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[30534607917] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[30620066235] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[30651552228] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[30746191740] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[30874193427] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[30915145371] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[30949196289] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[31095068631] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[31169161419] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[31369062054] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[31705253448] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1247102967 elapsed_us=623551
[31706355252] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[31796519634] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[31897144125] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[31905840747] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[32047860768] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[32052243498] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[32074486653] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=50919
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[32373901428] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[32375965281] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[32439986370] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
mount -t https example.com /https/ex
[34715336370] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='mount'
[?25l[35094924678] [[32mINFO [0m] [httpsd] [CPU1] HTTPSD_READY
[35096425419] [[32mINFO [0m] [httpsd] [CPU1] httpsd: entering main RPC loop for /https/ex
[35395624737] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Lookup payload_len=4
[35397018096] [[32mINFO [0m] [httpsd] [CPU1] httpsd: dispatch_lookup path=''
[35398197417] [[32mINFO [0m] [httpsd] [CPU1] httpsd: resolve_path path=''
[35437165995] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Close payload_len=8
mounted type=https device=example.com target=/https/ex
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hca[35848196637] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[35851170993] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[35947391931] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
t /htt[35995297602] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[35996832102] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[35998209324] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
ps/e[36211641609] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
x/@index
[36690428214] [[32mINFO [0m] [sh] [CPU1] sh: spawning job cmd='cat'
[?25l[36797829882] [[32mINFO [0m] [cat] [CPU1] cat: opening '/https/ex/@index'
[36815404593] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Lookup payload_len=10
[36816760497] [[32mINFO [0m] [httpsd] [CPU1] httpsd: dispatch_lookup path='@index'
[36817810788] [[32mINFO [0m] [httpsd] [CPU1] httpsd: resolve_path path='@index'
[36836498952] [[32mINFO [0m] [cat] [CPU1] cat: opened '/https/ex/@index' fd=8
[36851727066] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Poll payload_len=12
[36884115279] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[36919182861] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[39952398486] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[40033306764] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[40217042019] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=17)
cat /https/ex/@index
[41410495599] [[32mINFO [0m] [sh] [CPU1] sh: spawning job cmd='cat'
[?25l[41472200979] [[32mINFO [0m] [cat] [CPU3] cat: opening '/https/ex/@index'
[41481620400] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Lookup payload_len=10
[41482972245] [[32mINFO [0m] [httpsd] [CPU1] httpsd: dispatch_lookup path='@index'
[41484121404] [[32mINFO [0m] [httpsd] [CPU1] httpsd: resolve_path path='@index'
[41493766545] [[32mINFO [0m] [cat] [CPU3] cat: opened '/https/ex/@index' fd=9
[41511553314] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Poll payload_len=12
[41521678803] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[41542400196] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hca[41728433406] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
t /https/ex/@index
[42704659569] [[32mINFO [0m] [sh] [CPU1] sh: spawning job cmd='cat'
[?25l[42799915257] [[32mINFO [0m] [cat] [CPU1] cat: opening '/https/ex/@index'
[42813968208] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Lookup payload_len=10
[42814943259] [[32mINFO [0m] [httpsd] [CPU1] httpsd: dispatch_lookup path='@index'
[42816078228] [[32mINFO [0m] [httpsd] [CPU1] httpsd: resolve_path path='@index'
[42835360128] [[32mINFO [0m] [cat] [CPU1] cat: opened '/https/ex/@index' fd=10
[42853493958] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Poll payload_len=12
[42885983250] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[42923565366] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[59286836202] [[31;
```
</details>
