# ❌ Scenario: Fetch Wikipedia /wiki path should follow redirect

> Last run: 2026-04-23 19:52:41

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8312ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 253ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https en.wikipedia.org /https/wp" on the serial console | ✅ | 1812ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ✅ | 0ms | - - - |
| 5 | And I type "cat /https/wp/wiki/@index" on the serial console | ✅ | 5871ms | - [📜](./05/serial.log) - |
| 6 | Then the command output should contain "Main Page" | ❌ | 6086ms | - [📜](./06/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[25855913919] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[25882468458] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[25913093613] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[25946785029] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[25967499855] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[25970163054] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26005900899] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26027259159] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26074763121] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26075880963] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26162417886] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26233815861] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26257685652] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26340254094] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26457468576] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26488941072] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26514182244] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[26613833301] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[26662692210] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[26793593937] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27036809877] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=939225738 elapsed_us=469612
[27037546734] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27109456242] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27176212998] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27178875108] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27258651651] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[27263954454] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27268022595] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27275779509] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=98142
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[27531378699] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[27532555149] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[27567334971] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
mount -t https en.wikipedia.org /http[30005012301] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=9 PID=9
[30050836233] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=10 PID=10
[30190046370] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=12 PID=12
[30217141251] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[30218015355] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[30248423073] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
s/wp
[30273282897] [[32mINFO [0m] [sh] [CPU1] sh: spawning job cmd='mount'
[30277849041] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[30278589297] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[30278968797] [[32mINFO [0m] [sh] [CPU1] sh: spawning '/bin/mount' with argv=[[47, 98, 105, 110, 47, 109, 111, 117, 110, 116], [45, 116], [104, 116, 116, 112, 115], [101, 110, 46, 119, 105, 107, 105, 112, 101, 100, 105, 97, 46, 111, 114, 103], [47, 104, 116, 116, 112, 115, 47, 119, 112]]
[30279478548] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
[30310569960] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/bin/mount' TID=13 PID=13
[30495391443] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/bin/httpsd' TID=14 PID=14
[?25l[30549603579] [[32mINFO [0m] [httpsd] [CPU2] HTTPSD_READY
[30550829463] [[32mINFO [0m] [httpsd] [CPU2] httpsd: entering main RPC loop for /https/wp
[30755354124] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[33502212876] [[32mINFO [0m] [netd] [CPU1] NETD: Starting network service...
[33562453188] [[32mINFO [0m] [netd] [CPU1] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[33711173034] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/netd' TID=16 PID=16
[33714596751] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=16)
[33782634996] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Lookup payload_len=4
[33785334891] [[32mINFO [0m] [httpsd] [CPU2] httpsd: dispatch_lookup path=''
[33786337398] [[32mINFO [0m] [httpsd] [CPU2] httpsd: resolve_path path=''
[33811699614] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Close payload_len=8
mounted type=https device=en.wikipedia.org target=/https/wp
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/wp/wiki/@index
[35387387772] [[32mINFO [0m] [sh] [CPU1] sh: spawning job cmd='cat'
[35390307612] [[32mINFO [0m] [sh] [CPU1] sh: spawning '/bin/cat' with argv=[[47, 98, 105, 110, 47, 99, 97, 116], [47, 104, 116, 116, 112, 115, 47, 119, 112, 47, 119, 105, 107, 105, 47, 64, 105, 110, 100, 101, 120]]
[35414235351] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/bin/cat' TID=18 PID=18
[?25l[35440101444] [[32mINFO [0m] [cat] [CPU3] cat: opening '/https/wp/wiki/@index'
[35446363590] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Lookup payload_len=15
[35447194629] [[32mINFO [0m] [httpsd] [CPU2] httpsd: dispatch_lookup path='wiki/@index'
[35447811102] [[32mINFO [0m] [httpsd] [CPU2] httpsd: resolve_path path='wiki/@index'
[35450831262] [[32mINFO [0m] [httpsd] [CPU2] httpsd: lookup 'wiki/@index' -> handle 2
[35477737185] [[32mINFO [0m] [cat] [CPU3] cat: opened '/https/wp/wiki/@index' fd=8
[35482568451] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Poll payload_len=12
[35492633286] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Read payload_len=20
[35513136846] [[32mINFO [0m] [httpsd] [CPU1] httpsd: worker started for handle=2 url=https://en.wikipedia.org/wiki
[35514774075] [[32mINFO [0m] [httpsd] [CPU1] httpsd: ensure_upstream handle=2 MISS https://en.wikipedia.org/wiki - opening network stream
[35533351656] [[32mINFO [0m] [http] [CPU1] http: waiting for header data from port (attempt=0/120)
[35626404462] [[32mINFO [0m] [http] [CPU2] http: connect host=en.wikipedia.org port=443
[35627267478] [[32mINFO [0m] [http] [CPU2] http: opening /net/tcp/new
[38794801881] [[32mINFO [0m] [netd] [CPU1] NETD: Network ready
[52198846656] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=18 req_id=5 op=Read TIMEOUT
cat: error reading /https/wp/wiki/@index
[53288420460] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU2] VFS RPC: tid=20 req_id=1 op=Lookup TIMEOUT
[53300405565] [[32mINFO [0m] [http] [CPU1] http: received 64 bytes from background TLS thread
[53302771962] [[33mWARN [0m] [httpsd] [CPU1] httpsd: upstream open failed for handle=2 https://en.wikipedia.org/wiki: http connect failed: failed to open /net/tcp/new: ETIMEDOUT
[53305035333] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Close payload_len=8
[53306823768] [[32mINFO [0m] [httpsd] [CPU1] httpsd: worker exiting for handle=2
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[61780424805] [[31;1mERROR[0m] [
```
</details>
