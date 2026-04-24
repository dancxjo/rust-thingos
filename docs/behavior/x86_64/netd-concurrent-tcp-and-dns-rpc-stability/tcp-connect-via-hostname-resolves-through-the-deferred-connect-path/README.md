# ❌ Scenario: TCP connect via hostname resolves through the deferred-connect path

> Last run: 2026-04-23 19:11:44

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I type "mount -t https none /https" on the serial console | ✅ | 645ms | - [📜](./01/serial.log) - |
| 2 | And I wait for the serial output to contain "HTTPSD_READY" | ✅ | 0ms | - - - |
| 3 | And I wait for the serial output to contain "mounted type=https" | ✅ | 0ms | - - - |
| 4 | And I type "cat /https/example.com/@index" on the serial console | ✅ | 5577ms | - [📜](./04/serial.log) - |
| 5 | Then the command output should contain "Example Domain" | ❌ | 6086ms | - [📜](./05/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[25961353605] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[25988285862] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26019210492] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26051854290] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26072084610] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26074577430] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26109884097] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26130694260] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26178286233] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26179270656] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26262486195] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26331928920] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26354037963] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26431572849] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26535504600] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26561920869] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26585615232] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[26680658367] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[26729011287] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[26857109994] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27095823018] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=895281024 elapsed_us=447640
[27096500244] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27173851452] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27238588806] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27240524454] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27324256212] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[27329368341] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27331313790] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27338953290] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=20064
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[27584192526] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[27586160118] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[27620366103] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[30014741823] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=9 PID=9
[30057377460] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=10 PID=10
[30181555239] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=12 PID=12
[30185107425] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[30185949651] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[30237392625] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
[30262071939] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[30262828068] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[30263573967] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
[33323833719] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[35951656092] [[32mINFO [0m] [netd] [CPU1] NETD: Starting network service...
[35989616454] [[32mINFO [0m] [netd] [CPU1] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[36162676638] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/netd' TID=13 PID=13
[36166467711] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=13)
[42246113217] [[32mINFO [0m] [netd] [CPU1] NETD: Network ready
mount -t https none /https
[43890050226] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='mount'
[43895794536] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/mount' with argv=[[47, 98, 105, 110, 47, 109, 111, 117, 110, 116], [45, 116], [104, 116, 116, 112, 115], [110, 111, 110, 101], [47, 104, 116, 116, 112, 115]]
[43927528590] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/mount' TID=15 PID=15
[44110055814] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/httpsd' TID=16 PID=16
[?25l[44156709828] [[32mINFO [0m] [httpsd] [CPU1] HTTPSD_READY
[44157858591] [[32mINFO [0m] [httpsd] [CPU1] httpsd: entering main RPC loop for /https
[44430277749] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Lookup payload_len=4
[44431319097] [[32mINFO [0m] [httpsd] [CPU1] httpsd: dispatch_lookup path=''
[44432023020] [[32mINFO [0m] [httpsd] [CPU1] httpsd: resolve_path path=''
[44442791613] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Close payload_len=8
mounted type=https device=none target=/https
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/example.com/@index
[46172998476] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[46175913696] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/cat' with argv=[[47, 98, 105, 110, 47, 99, 97, 116], [47, 104, 116, 116, 112, 115, 47, 101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109, 47, 64, 105, 110, 100, 101, 120]]
[46199332674] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/cat' TID=18 PID=18
[46235182389] [[32mINFO [0m] [cat] [CPU3] cat: opening '/https/example.com/@index'
[46241022300] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Lookup payload_len=22
[46242167037] [[32mINFO [0m] [httpsd] [CPU1] httpsd: dispatch_lookup path='example.com/@index'
[46243273923] [[32mINFO [0m] [httpsd] [CPU1] httpsd: resolve_path path='example.com/@index'
[46250490825] [[32mINFO [0m] [httpsd] [CPU1] httpsd: lookup 'example.com/@index' -> handle 2
[46271886507] [[32mINFO [0m] [cat] [CPU3] cat: opened '/https/example.com/@index' fd=8
[46279389651] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Poll payload_len=12
[46297486587] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Read payload_len=20
[?25l[46327236549] [[32mINFO [0m] [httpsd] [CPU1] httpsd: worker started for handle=2 url=https://example.com
[46328755506] [[32mINFO [0m] [httpsd] [CPU1] httpsd: ensure_upstream handle=2 MISS https://example.com - opening network stream
[46344068694] [[32mINFO [0m] [http] [CPU1] http: waiting for header data from port (attempt=0/120)
[46345441098] [[32mINFO [0m] [http] [CPU2] http: connect host=example.com port=443
[46346179341] [[32mINFO [0m] [http] [CPU2] http: opening /net/tcp/new
[60431908284] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU0] VFS RPC: tid=5 req_id=1 op=Lookup TIMEOUT
[60438124824] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: netd activation probe failed for PID 13 (attempt 1/3): ETIMEDOUT
[62864617629] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=18 req_id=5 op=Read TIMEOUT
cat: error reading /https/example.com/@index
[62880076050] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Close payload_len=8
[62889405975] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU2] VFS RPC: tid=20 req_id=2 op=Lookup TIMEOUT
[62894159328] [[32mINFO [0m] [http] [CPU1] http: received 64 bytes from background TLS thread
[62896220508] [[33mWARN [0m] [httpsd] [CPU1] httpsd: upstream open failed for handle=2 https://example.com: http connect failed: failed to open /net/tcp/new: ETIMEDOUT
[62897848068] [[32mINFO [0m] [httpsd] [CPU1] httpsd: worker exiting for handle=2
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[80343450792] [[31;1mERROR[0m] [kernel::vfs::provi
```
</details>
