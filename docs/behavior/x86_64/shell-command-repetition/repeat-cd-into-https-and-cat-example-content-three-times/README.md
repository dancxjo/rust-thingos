# ❌ Scenario: Repeat cd into https and cat example content three times

> Last run: 2026-04-23 21:03:05

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8518ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 150ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https example.com /https/ex" on the serial console | ✅ | 808ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ✅ | 0ms | - - - |
| 5 | And I type "cat /https/ex/@index" on the serial console | ✅ | 851ms | - [📜](./05/serial.log) - |
| 6 | And I type "cat /https/ex/@index" on the serial console | ✅ | 439ms | - [📜](./06/serial.log) - |
| 7 | And I type "cat /https/ex/@index" on the serial console | ✅ | 389ms | - [📜](./07/serial.log) - |
| 8 | Then the command output should contain "<html" | ❌ | 6082ms | - [📜](./08/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26412772518] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26439101073] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26469661152] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26502882615] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26525963277] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26529135270] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26564774379] [[32mINFO [0m] [k[269822090ernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26584792542] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26635004286] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26636028111] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26719771617]34] [[32mIN [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26788972155] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26810558973] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26882416902] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
FO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[27016463001] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[27039505449] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[27137125587] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[27184329612] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[27315873948] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27564086847] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=907220523 elapsed_us=453610
[27565068003] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27636366120] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27701275866] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27702132117] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27785347194] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[27792840900] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27793228419] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[27794855385] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27801709947] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=26961
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28016132100] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[28017618453] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[28050303270] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
mount -t https example.com /https/ex
[30344983779] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='mount'
[30350921799] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/mount' with argv=["/bin/mount", "-t", "https", "example.com", "/https/ex"]
[30386212857] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/mount' TID=9 PID=9
[30511573191] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=10 PID=10
[30580857483] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/httpsd' TID=11 PID=11
[30589597302] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25l[30666808458] [[32mINFO [0m] [httpsd] [CPU2] HTTPSD_READY
[30668061204] [[32mINFO [0m] [httpsd] [CPU2] httpsd: entering main RPC loop for /https/ex
[30694998576] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=14 PID=14
[30731735859] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=15 PID=15
[30737499474] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[30738477264] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[30793102362] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
[30824136222] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[30825329733] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[30826088370] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
[30939106836] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Lookup payload_len=4
[30940043805] [[32mINFO [0m] [httpsd] [CPU2] httpsd: dispatch_lookup path=''
[30940693740] [[32mINFO [0m] [httpsd] [CPU2] httpsd: resolve_path path=''
[30967837263] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Close payload_len=8
mounted type=https device=example.com target=/https/ex
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[31121443584] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[33688958160] [[32mINFO [0m] [netd] [CPU1] NETD: Starting network service...
[33720090261] [[32mINFO [0m] [netd] [CPU1] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
ct /hatpst/ex/@index
[33767192052] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ct'
[33770255211] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/ct' with argv=["/bin/ct", "/hatpst/ex/@index"]
[33773111394] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
sh: spawn failed: no such file or directory
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hc[33907970283] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/netd' TID=16 PID=16
[33910937478] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=16)
at /https/ex/@index
[35045297826] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[35047990824] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/cat' with argv=["/bin/cat", "/https/ex/@index"]
[35072421945] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/cat' TID=17 PID=17
[35075077224] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25l[35100943449] [[32mINFO [0m] [cat] [CPU2] cat: opening '/https/ex/@index'
[35110314393] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Lookup payload_len=10
[35111024784] [[32mINFO [0m] [httpsd] [CPU2] httpsd: dispatch_lookup path='@index'
[35111613141] [[32mINFO [0m] [httpsd] [CPU2] httpsd: resolve_path path='@index'
[35123897853] [[32mINFO [0m] [cat] [CPU2] cat: opened '/https/ex/@index' fd=8
[35132255994] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Poll payload_len=12
[35152985406] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[35171894538] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/ex/@index
[36390417855] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[36393150222] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/cat' with argv=["/bin/cat", "/https/ex/@index"]
[36420509136] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/cat' TID=18 PID=18
[36422381028] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25l[36458451975] [[32mINFO [0m] [cat] [CPU3] cat: opening '/https/ex/@index'
[36465947958] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Lookup payload_len=10
[36467056989] [[32mINFO [0m] [httpsd] [CPU2] httpsd: dispatch_lookup path='@index'
[36467652342] [[32mINFO [0m] [httpsd] [CPU2] httpsd: resolve_path path='@index'
[36478613754] [[32mINFO [0m] [cat] [CPU3] cat: opened '/https/ex/@index' fd=9
[36482834817] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Poll payload_len=12
[36488515866] [[32mINFO [0m]cat: error reading /https/ex/@index
[36497786721] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Close payload_len=8
 [httpsd] [CPU2] httpsd: RPC op=Read payload_len=20
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[53192564997] [[31;1mER
```
</details>
