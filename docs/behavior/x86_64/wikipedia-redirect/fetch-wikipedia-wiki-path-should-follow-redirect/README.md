# ❌ Scenario: Fetch Wikipedia /wiki path should follow redirect

> Last run: 2026-04-23 11:14:17

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 9528ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 255ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https en.wikipedia.org /https/wp" on the serial console | ✅ | 1120ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ✅ | 0ms | - - - |
| 5 | And I type "cat /https/wp/wiki/@index" on the serial console | ✅ | 856ms | - [📜](./05/serial.log) - |
| 6 | Then the command output should contain "Main Page" | ❌ | 6103ms | - [📜](./06/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[29436440715] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[29466036039] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[29507384016] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[29545830171] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[29569318086] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[29572462227] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[29624557380] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[29650890654] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[29703978018] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[29705739459] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[29800243209] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[29876817498] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[29900727021] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[29984371989] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[30089548038] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[30144076380] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[30180782907] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[30348852996] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[30432136878] [[32mINFO [0m] [kernel::boot_pr] [CPU0] boot_progress: milestone="Spawning Sprout"
ogress[30638647941] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[30974620545] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1246941630 elapsed_us=623470
[30975833097] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[31081551996] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[31198981869] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[31201669950] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[31300494885] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[31303124754] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[31317765105] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=30096
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[31589042694] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mounting /hosts early...
[31637008854] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned early /hosts mount (PID=9)
[31679598159] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet �� retrying
[31726078659] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[31840322745] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: starting (mount_point=/hosts)
[31873872195] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: mounted at /hosts
[31892056416] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: advertising hostname thingos.local
[31894987509] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: serving cache immediately; mDNS socket will attach when netd is ready
mount -t mounted type=mdns device=none target=/hosts
https en.wikipedia.org /https/wp
[34346536686] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='mount'
[?25l[34777739700] [[32mINFO [0m] [httpsd] [CPU1] HTTPSD_READY
[34779105273] [[32mINFO [0m] [httpsd] [CPU1] httpsd: entering main RPC loop for /https/wp
[35084987784] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Lookup payload_len=4
[35086273101] [[32mINFO [0m] [httpsd] [CPU1] httpsd: dispatch_lookup path=''
[35087886240] [[32mINFO [0m] [httpsd] [CPU1] httpsd: resolve_path path=''
[35375669505] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Close payload_len=8
mounted type=https device=en.wikipedia.org target=/https/wp
[35507503284] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[35548010190] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Probing primary port (0x1F0)...
[35675126388] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Probing secondary port (0x170)...
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hc[35796926220] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[35798486493] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Entering RPC service loop
[35799888234] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: No active devices to service
at /ht[36151730703] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
tps/wp/wiki/@index
[37134561684] [[32mINFO [0m] [sh] [CPU1] sh: spawning job cmd='cat'
[?25l[37371528909] [[32mINFO [0m] [cat] [CPU1] cat: opening '/https/wp/wiki/@index'
[37390333761] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Lookup payload_len=15
[37391754345] [[32mINFO [0m] [httpsd] [CPU1] httpsd: dispatch_lookup path='wiki/@index'
[37392919047] [[32mINFO [0m] [httpsd] [CPU1] httpsd: resolve_path path='wiki/@index'
[37398666591] [[32mINFO [0m] [httpsd] [CPU1] httpsd: lookup 'wiki/@index' -> handle 2
[37419774777] [[32mINFO [0m] [cat] [CPU1] cat: opened '/https/wp/wiki/@index' fd=8
[37444578534] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Poll payload_len=12
[37490466651] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Read payload_len=20
[37494113877] [[32mINFO [0m] [httpsd] [CPU1] httpsd: ensure_upstream handle=2 MISS https://en.wikipedia.org/wiki - opening network stream
[37520564961] [[32mINFO [0m] [http] [CPU1] http: waiting for header data from port (attempt=0/120)
[38327870889] [[32mINFO [0m] [http] [CPU2] http: connect host=en.wikipedia.org port=443
[38329379814] [[32mINFO [0m] [http] [CPU2] http: opening /net/tcp/new
[38331157194] [[32mINFO [0m] [netd] [CPU3] NETD: Starting network service...
[38335526361] [[32mINFO [0m] [http] [CPU1] http: received 61 bytes from background TLS thread
[38337805803] [[33mWARN [0m] [httpsd] [CPU1] httpsd: upstream open failed for handle=2 https://en.wikipedia.org/wiki: http connect failed: failed to open /net/tcp/new: ENOENT
cat: error reading /https/wp/wiki/@index
[38374886319] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Close payload_len=8
[38398646946] [[32mINFO [0m] [netd] [CPU3] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[39538672503] [[32mINFO 
```
</details>
