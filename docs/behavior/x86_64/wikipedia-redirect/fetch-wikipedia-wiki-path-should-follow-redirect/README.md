# ❌ Scenario: Fetch Wikipedia /wiki path should follow redirect

> Last run: 2026-04-23 19:11:44

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8311ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 150ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https en.wikipedia.org /https/wp" on the serial console | ✅ | 1816ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ✅ | 0ms | - - - |
| 5 | And I type "cat /https/wp/wiki/@index" on the serial console | ✅ | 5472ms | - [📜](./05/serial.log) - |
| 6 | Then the command output should contain "Main Page" | ❌ | 6079ms | - [📜](./06/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[25706955516] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[25733264865] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[25764517812] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[25797777258] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[25818246168] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[25820778060] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[25856394696] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[25877606040] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[25923616719] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[25924636584] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26008514994] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26080138194] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26104515921] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26179470504] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26285451456] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26314641177] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26340632142] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[26450876364] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[26505932937] [[32mINFO [0m] [kernel::boot_pro] gress] [CPU0boot_progress: milestone="Spawning Sprout"
[26635522452] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[26881260549] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=935328372 elapsed_us=467664
[26881988958] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[26958698481] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27023439993] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27024362376] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27107001669] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[27112095285] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27114106140] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27123793389] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=19998
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[27376567680] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[27378543852] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[27430687944] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet �� retrying
mount -t https en.wikipedia.org /https/w[29812562472] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=9 PID=9
[29859072012] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=10 PID=10
[30017267181] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=12 PID=12
[30046395753] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[30047403441] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[30081180921] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
p
[30091448376] [[32mINFO [0m] [sh] [CPU1] sh: spawning job cmd='mount'
[30097404546] [[32mINFO [0m] [sh] [CPU1] sh: spawning '/bin/mount' with argv=[[47, 98, 105, 110, 47, 109, 111, 117, 110, 116], [45, 116], [104, 116, 116, 112, 115], [101, 110, 46, 119, 105, 107, 105, 112, 101, 100, 105, 97, 46, 111, 114, 103], [47, 104, 116, 116, 112, 115, 47, 119, 112]]
[30109382028] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[30110160003] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[30110910621] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
[30129540210] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/bin/mount' TID=13 PID=13
[?25l[30318786960] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/bin/httpsd' TID=14 PID=14
[30379939953] [[32mINFO [0m] [httpsd] [CPU2] HTTPSD_READY
[30381298827] [[32mINFO [0m] [httpsd] [CPU2] httpsd: entering main RPC loop for /https/wp
[30425117415] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[33022885929] [[32mINFO [0m] [netd] [CPU1] NETD: Starting network service...
[33054467490] [[32mINFO [0m] [netd] [CPU1] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[33239832351] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/netd' TID=16 PID=16
[33243423345] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=16)
[33453465309] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Lookup payload_len=4
[33454480092] [[32mINFO [0m] [httpsd] [CPU2] httpsd: dispatch_lookup path=''
[33455178471] [[32mINFO [0m] [httpsd] [CPU2] httpsd: resolve_path path=''
[33472486542] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Close payload_len=8
mounted type=https device=en.wikipedia.org target=/https/wp
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/wp/wiki/@index
[35069367300] [[32mINFO [0m] [sh] [CPU1] sh: spawning job cmd='cat'
[35072683536] [[32mINFO [0m] [sh] [CPU1] sh: spawning '/bin/cat' with argv=[[47, 98, 105, 110, 47, 99, 97, 116], [47, 104, 116, 116, 112, 115, 47, 119, 112, 47, 119, 105, 107, 105, 47, 64, 105, 110, 100, 101, 120]]
[35098231344] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/bin/cat' TID=18 PID=18
[?25l[35130357966] [[32mINFO [0m] [cat] [CPU3] cat: opening '/https/wp/wiki/@index'
[35140250871] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Lookup payload_len=15
[35141043531] [[32mINFO [0m] [httpsd] [CPU1] httpsd: dispatch_lookup path='wiki/@index'
[35141657232] [[32mINFO [0m] [httpsd] [CPU1] httpsd: resolve_path path='wiki/@index'
[35146278189] [[32mINFO [0m] [httpsd] [CPU1] httpsd: lookup 'wiki/@index' -> handle 2
[35179680855] [[32mINFO [0m] [cat] [CPU3] cat: opened '/https/wp/wiki/@index' fd=8
[35187285078] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Poll payload_len=12
[35196007671] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Read payload_len=20
[35217102624] [[32mINFO [0m] [httpsd] [CPU1] httpsd: worker started for handle=2 url=https://en.wikipedia.org/wiki
[35218682268] [[32mINFO [0m] [httpsd] [CPU1] httpsd: ensure_upstream handle=2 MISS https://en.wikipedia.org/wiki - opening network stream
[35233976712] [[32mINFO [0m] [http] [CPU1] http: waiting for header data from port (attempt=0/120)
[35237177349] [[32mINFO [0m] [http] [CPU2] http: connect host=en.wikipedia.org port=443
[35238112008] [[32mINFO [0m] [http] [CPU2] http: opening /net/tcp/new
[40522665975] [[32mINFO [0m] [netd] [CPU1] NETD: Network ready
[51655176738] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=18 req_id=5 op=Read TIMEOUT
cat: error reading /https/wp/wiki/@index
[51670330107] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[53083315263] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU2] VFS RPC: tid=20 req_id=1 op=Lookup TIMEOUT
[60708475509] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU0] VFS RPC: tid=5 req_id=2 op=Lookup TIMEOUT
[60711872826] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: netd activation probe failed for PID 16 (attempt 1/3): ETIMEDOUT
[68151612672] [[
```
</details>
