# ❌ Scenario: Repeat cd into https and cat example content three times

> Last run: 2026-04-23 17:57:17

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8313ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 252ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https example.com /https/ex" on the serial console | ✅ | 1680ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ✅ | 0ms | - - - |
| 5 | And I type "cat /https/ex/@index" on the serial console | ✅ | 392ms | - [📜](./05/serial.log) - |
| 6 | And I type "cat /https/ex/@index" on the serial console | ✅ | 391ms | - [📜](./06/serial.log) - |
| 7 | And I type "cat /https/ex/@index" on the serial console | ✅ | 389ms | - [📜](./07/serial.log) - |
| 8 | Then the command output should contain "<html" | ❌ | 6083ms | - [📜](./08/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[25730873718] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[25761744261] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[25812306135] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[25854813930] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[25876728009] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[25880070843] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[25918330845] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[25939612743] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[25987698792] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[25988734002] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26075259573] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26144603067] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26166717093] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26245629333] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26354704431] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26383817196] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26407575810] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[26505712266] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[26554107063] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[26689376274] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[26938675566] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=927958977 elapsed_us=463979
[26939398299] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27013836300] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27108673350] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27109777959] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27195203376] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[27197896638] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27200405595] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27210043674] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=18909
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[27439891050] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[27441078390] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[27481024527] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
mount -t https example.com /https/e[29944803735] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=9 PID=9
[29989607076] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=10 PID=10
[30093385113] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=12 PID=12
[30124909155] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[30125785767] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[30160308387] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
[30186763002] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[30187599024] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[30188329083] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
x
[30200201757] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='mount'
[30210072090] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/mount' with argv=[[47, 98, 105, 110, 47, 109, 111, 117, 110, 116], [45, 116], [104, 116, 116, 112, 115], [101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109], [47, 104, 116, 116, 112, 115, 47, 101, 120]]
[30243041235] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/mount' TID=13 PID=13
[?25l[30416424456] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/bin/httpsd' TID=14 PID=14
[30462930795] [[32mINFO [0m] [httpsd] [CPU2] HTTPSD_READY
[30464048637] [[32mINFO [0m] [httpsd] [CPU2] httpsd: entering main RPC loop for /https/ex
[30580832007] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[33103005012] [[32mINFO [0m] [netd] [CPU1] NETD: Starting network service...
[33131527473] [[32mINFO [0m] [netd] [CPU1] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[33318456699] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/netd' TID=16 PID=16
[33322518009] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=16)
[33458826060] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Lookup payload_len=4
[33459759300] [[32mINFO [0m] [httpsd] [CPU2] httpsd: dispatch_lookup path=''
[33460521996] [[32mINFO [0m] [httpsd] [CPU2] httpsd: resolve_path path=''
[33479317641] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Close payload_len=8
mounted type=https device=example.com target=/https/ex
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat[33738276825] [[32mINFO [0m] [netd] [CPU1] NETD: Network ready
 /https/ex/@index
[34686780315] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[34690147404] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/cat' with argv=[[47, 98, 105, 110, 47, 99, 97, 116], [47, 104, 116, 116, 112, 115, 47, 101, 120, 47, 64, 105, 110, 100, 101, 120]]
[34714056069] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/cat' TID=17 PID=17
[?25l[34749165594] [[32mINFO [0m] [cat] [CPU2] cat: opening '/https/ex/@index'
[34758475686] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Lookup payload_len=10
[34759151625] [[32mINFO [0m] [httpsd] [CPU2] httpsd: dispatch_lookup path='@index'
[34759771629] [[32mINFO [0m] [httpsd] [CPU2] httpsd: resolve_path path='@index'
[34768592727] [[32mINFO [0m] [cat] [CPU3] cat: opened '/https/ex/@index' fd=8
[34774418283] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Poll payload_len=12
[34788677649] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[34803643974] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/ex/@index
[35978117505] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[35980959201] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/cat' with argv=[[47, 98, 105, 110, 47, 99, 97, 116], [47, 104, 116, 116, 112, 115, 47, 101, 120, 47, 64, 105, 110, 100, 101, 120]]
[36005008380] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/cat' TID=18 PID=18
[?25l[36039016827] [[32mINFO [0m] [cat] [CPU3] cat: opening '/https/ex/@index'
[36046462947] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Lookup payload_len=10
[36047249931] [[32mINFO [0m] [httpsd] [CPU2] httpsd: dispatch_lookup path='@index'
[36048149676] [[32mINFO [0m] [httpsd] [CPU2] httpsd: resolve_path path='@index'
[36054984042] [[32mINFO [0m] [cat] [CPU3] cat: opened '/https/ex/@index' fd=9
[36063400098] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Poll payload_len=12
[36077974482] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@inde] [httpsd] [CPU2] httpsd: RPC op=Close payload_len=8
x
[36093526821] [[32mINFO [0m[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/ex/@index
[37257967494] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[37260630660] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/cat' with argv=[[47, 98, 105, 110, 47, 99, 97, 116], [47, 104, 116, 116, 112, 115, 47, 101, 120, 47, 64, 105, 110, 100, 101, 120]]
[37284016275] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/cat' TID=19 PID=19
[?25l[37316689938] [[32mINFO [0m] [cat] [CPU1] cat: opening '/https/ex/@index'
[37325048277] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Lookup payload_len=10
[37325793087] [[32mINFO [0m] [httpsd] [CPU2] httpsd: dispatch_lookup path='@index'
[37326393093] [[32mINFO [0m] [httpsd] [CPU2] httpsd: resolve_path path='@index'
[37332705135] [[32mINFO [0m] [cat] [CPU1] cat: opened '/https/ex/@index' fd=10
[37340913126] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Poll payload_len=12
[37355700789] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[37372094463] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[52507262280] [[31;1mERROR[0m] [kern
```
</details>
