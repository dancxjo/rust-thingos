# ❌ Scenario: Fetch Wikipedia /wiki path should follow redirect

> Last run: 2026-04-23 21:19:50

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8615ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 252ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https en.wikipedia.org /https/wp" on the serial console | ✅ | 1862ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ✅ | 0ms | - - - |
| 5 | And I type "cat /https/wp/wiki/@index" on the serial console | ✅ | 5615ms | - [📜](./05/serial.log) - |
| 6 | Then the command output should contain "Main Page" | ❌ | 6091ms | - [📜](./06/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26835808923] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26863329405] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26896926276] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26931176184] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26952525567] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26955449532] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26993351682] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[27014715552] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[27066511263] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[27067823310] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[27157732173] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[27231449190] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[27255074847] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[27335462682] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[27445907445] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[27473951109] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[27498922539] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[27602052060] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[27651660696] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[27790188063] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[28049543841] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=960111702 elapsed_us=480055
[28050315480] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[28135222566] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[28247825496] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[28248751740] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[28344837708] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[28349110251] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[28351363458] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[28359275637] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[28366968993] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=21978
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28594378065] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[28595823960] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[28636391487] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet ��� retrying
mount -t https en.wikipedia.org /https/w[31185494142] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=9 PID=9
[31240494549] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=10 PID=10
[31385306106] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=12 PID=12
[31390043553] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[31390936500] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[31451576442] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
p
[31465246428] [[32mINFO [0m] [sh] [CPU1] sh: spawning job cmd='mount'
[31474621959] [[32mINFO [0m] [sh] [CPU1] sh: spawning '/bin/mount' with argv=["/bin/mount", "-t", "https", "en.wikipedia.org", "/https/wp"]
[31486246902] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[31487047977] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[31487803677] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
[31508656377] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/bin/mount' TID=13 PID=13
[31700088783] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/bin/httpsd' TID=14 PID=14
[31709198070] [[32mINFO [0m] [sh] [CPU1] sh: cleaning up 0 pipes
[?25l[31766732283] [[32mINFO [0m] [httpsd] [CPU2] HTTPSD_READY
[31768249689] [[32mINFO [0m] [httpsd] [CPU2] httpsd: entering main RPC loop for /https/wp
[31812744645] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[34572617046] [[32mINFO [0m] [netd] [CPU1] NETD: Starting network service...
[34611782469] [[32mINFO [0m] [netd] [CPU1] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[34791096087] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/netd' TID=16 PID=16
[34793850630] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=16)
[34990560099] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Lookup payload_len=4
[34991917257] [[32mINFO [0m] [httpsd] [CPU2] httpsd: dispatch_lookup path=''
[34992777963] [[32mINFO [0m] [httpsd] [CPU2] httpsd: resolve_path path=''
[35011510149] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Close payload_len=8
mounted type=https device=en.wikipedia.org target=/https/wp
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/wp/wiki/@index
[36521821743] [[32mINFO [0m] [sh] [CPU1] sh: spawning job cmd='cat'
[36525075015] [[32mINFO [0m] [sh] [CPU1] sh: spawning '/bin/cat' with argv=["/bin/cat", "/https/wp/wiki/@index"]
[36550430961] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/bin/cat' TID=18 PID=18
[36552701130] [[32mINFO [0m] [sh] [CPU1] sh: cleaning up 0 pipes
[?25l[36580027572] [[32mINFO [0m] [cat] [CPU3] cat: opening '/https/wp/wiki/@index'
[36589037694] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Lookup payload_len=15
[36589948659] [[32mINFO [0m] [httpsd] [CPU2] httpsd: dispatch_lookup path='wiki/@index'
[36590802600] [[32mINFO [0m] [httpsd] [CPU2] httpsd: resolve_path path='wiki/@index'
[36594982248] [[32mINFO [0m] [httpsd] [CPU2] httpsd: lookup 'wiki/@index' -> handle 2
[36607515450] [[32mINFO [0m] [cat] [CPU3] cat: opened '/https/wp/wiki/@index' fd=8
[36614803269] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Poll payload_len=12
[36645716118] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Read payload_len=20
[36764984058] [[32mINFO [0m] [httpsd] [CPU1] httpsd: worker started for handle=2 url=https://en.wikipedia.org/wiki
[36766595580] [[32mINFO [0m] [httpsd] [CPU1] httpsd: ensure_upstream handle=2 MISS https://en.wikipedia.org/wiki - opening network stream
[36785219394] [[32mINFO [0m] [http] [CPU1] http: waiting for header data from port (attempt=0/120)
[36790579782] [[32mINFO [0m] [http] [CPU2] http: connect host=en.wikipedia.org port=443
[36791719866] [[32mINFO [0m] [http] [CPU2] http: opening /net/tcp/new
[36818595528] [[32mINFO [0m] [netd] [CPU1] NETD: Network ready
[40292274396] [[32mINFO [0m] [http] [CPU2] http: allocated tcp socket id=2
[40293222552] [[32mINFO [0m] [http] [CPU2] http: opening ctl path /net/tcp/2/ctl
[40398878850] [[32mINFO [0m] [http] [CPU2] http: opening data path /net/tcp/2/data
[40594295709] [[32mINFO [0m] [http] [CPU2] http: issuing connect command: connect en.wikipedia.org 443
[45769390392] [[32mINFO [0m] [http] [CPU2] http: waiting for socket readiness...
[45770739300] [[32mINFO [0m] [http] [CPU2] http: waiting for http connect socket readiness (fd=8)...
[48372924930] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU0] VFS RPC: tid=5 req_id=2 op=Lookup TIMEOUT
[48376887174] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: netd activation probe failed for PID 16 (attempt 1/3): ETIMEDOUT
[48378679206] [[32mINFO [0m] [http] [CPU2] http: wait_fd_ready complete for http connect socket revents=0x0004
[48380451174] [[32mINFO [0m] [http] [CPU2] http: background task: TCP connected to en.wikipedia.org
[48382238553] [[32mINFO [0m] [http] [CPU2] http: background task: starting TLS handshake with en.wikipedia.org
[51242896683] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: netd activation probe succeeded for PID 16 (/net/icmp/new is responsive)
[53486640900] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=18 req_id=5 op=Read TIMEOUT
cat: error reading /https/wp/wiki/@index
[53503159578] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[66528294162] [[31;1mERROR
```
</details>
