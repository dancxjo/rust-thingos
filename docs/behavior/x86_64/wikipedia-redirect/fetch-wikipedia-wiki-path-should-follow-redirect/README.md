# ❌ Scenario: Fetch Wikipedia /wiki path should follow redirect

> Last run: 2026-04-23 17:57:17

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8217ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 253ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https en.wikipedia.org /https/wp" on the serial console | ✅ | 1604ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ✅ | 0ms | - - - |
| 5 | And I type "cat /https/wp/wiki/@index" on the serial console | ✅ | 5507ms | - [📜](./05/serial.log) - |
| 6 | Then the command output should contain "Main Page" | ❌ | 6089ms | - [📜](./06/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[25504906284] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[25531395450] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[25561651467] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[25595300148] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[25615754010] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[25618297584] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[25657001403] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[25677896574] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[25723734399] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[25724742780] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[25806393162] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[25876704249] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[25898516457] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[25970381712] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26074382169] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26102300103] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26126479071] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[26225819664] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[26274992238] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[26404886376] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[26648872140] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=903126708 elapsed_us=451563
[26649709482] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[26722100988] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[26789204970] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[26790199194] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[26875084434] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[26879272530] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[26882835837] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[26891993832] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=20922
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[27107079516] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[27108318237] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[27141945600] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet �� retrying
mount -t https en.wikipedia.org /h[29535383559] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=9 PID=9
[29577335634] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=10 PID=10
[29703619407] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=12 PID=12
[29730407289] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[29731259679] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[29765245455] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
ttps/[29789271336] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[29790803097] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[29791511112] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[29792278956] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
wp
[29953893420] [[32mINFO [0m] [sh] [CPU1] sh: spawning job cmd='mount'
[29959025877] [[32mINFO [0m] [sh] [CPU1] sh: spawning '/bin/mount' with argv=[[47, 98, 105, 110, 47, 109, 111, 117, 110, 116], [45, 116], [104, 116, 116, 112, 115], [101, 110, 46, 119, 105, 107, 105, 112, 101, 100, 105, 97, 46, 111, 114, 103], [47, 104, 116, 116, 112, 115, 47, 119, 112]]
[29988055317] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/bin/mount' TID=13 PID=13
[?25l[30164405997] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/bin/httpsd' TID=14 PID=14
[30221248497] [[32mINFO [0m] [httpsd] [CPU2] HTTPSD_READY
[30222338355] [[32mINFO [0m] [httpsd] [CPU2] httpsd: entering main RPC loop for /https/wp
[32276894562] [[32mINFO [0m] [netd] [CPU1] NETD: Starting network service...
[32306348514] [[32mINFO [0m] [netd] [CPU1] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[32485043052] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/netd' TID=16 PID=16
[32488046580] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=16)
[32781029721] [[32mINFO [0m] [netd] [CPU1] NETD: Network ready
[32823166134] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Lookup payload_len=4
[32824087791] [[32mINFO [0m] [httpsd] [CPU2] httpsd: dispatch_lookup path=''
[32824902627] [[32mINFO [0m] [httpsd] [CPU2] httpsd: resolve_path path=''
[32841346230] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Close payload_len=8
mounted type=https device=en.wikipedia.org target=/https/wp
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/wp/wiki/@index
[34377213453] [[32mINFO [0m] [sh] [CPU1] sh: spawning job cmd='cat'
[34380021456] [[32mINFO [0m] [sh] [CPU1] sh: spawning '/bin/cat' with argv=[[47, 98, 105, 110, 47, 99, 97, 116], [47, 104, 116, 116, 112, 115, 47, 119, 112, 47, 119, 105, 107, 105, 47, 64, 105, 110, 100, 101, 120]]
[34403021004] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/bin/cat' TID=17 PID=17
[?25l[34443296151] [[32mINFO [0m] [cat] [CPU2] cat: opening '/https/wp/wiki/@index'
[34448512791] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Lookup payload_len=15
[34449179028] [[32mINFO [0m] [httpsd] [CPU1] httpsd: dispatch_lookup path='wiki/@index'
[34449767913] [[32mINFO [0m] [httpsd] [CPU1] httpsd: resolve_path path='wiki/@index'
[34452923967] [[32mINFO [0m] [httpsd] [CPU1] httpsd: lookup 'wiki/@index' -> handle 2
[34462080972] [[32mINFO [0m] [cat] [CPU2] cat: opened '/https/wp/wiki/@index' fd=8
[34471015425] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Poll payload_len=12
[34489448532] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Read payload_len=20
[34491217629] [[32mINFO [0m] [httpsd] [CPU1] httpsd: ensure_upstream handle=2 MISS https://en.wikipedia.org/wiki - opening network stream
[34508379378] [[32mINFO [0m] [http] [CPU1] http: waiting for header data from port (attempt=0/120)
[34509857151] [[32mINFO [0m] [http] [CPU3] http: connect host=en.wikipedia.org port=443
[34510595031] [[32mINFO [0m] [http] [CPU3] http: opening /net/tcp/new
[49637390418] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU0] VFS RPC: tid=5 req_id=1 op=Lookup TIMEOUT
[49641074868] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: netd activation probe failed for PID 16 (attempt 1/3): ETIMEDOUT
[50952976602] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU2] VFS RPC: tid=17 req_id=5 op=Read TIMEOUT
cat: error reading /https/wp/wiki/@index
[51033400473] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=18 req_id=2 op=Lookup TIMEOUT
[51037873062] [[32mINFO [0m] [http] [CPU1] http: received 64 bytes from background TLS thread
[51039942426] [[33mWARN [0m] [httpsd] [CPU1] httpsd: upstream open failed for handle=2 https://en.wikipedia.org/wiki: http connect failed: failed to open /net/tcp/new: ETIMEDOUT
[51046720329] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[67030432425] [[31;1mERROR
```
</details>
