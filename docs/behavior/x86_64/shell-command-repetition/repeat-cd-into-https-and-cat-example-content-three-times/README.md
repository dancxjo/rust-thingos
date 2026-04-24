# ❌ Scenario: Repeat cd into https and cat example content three times

> Last run: 2026-04-23 21:19:50

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8615ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 253ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https example.com /https/ex" on the serial console | ✅ | 1684ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ✅ | 0ms | - - - |
| 5 | And I type "cat /https/ex/@index" on the serial console | ✅ | 442ms | - [📜](./05/serial.log) - |
| 6 | And I type "cat /https/ex/@index" on the serial console | ✅ | 391ms | - [📜](./06/serial.log) - |
| 7 | And I type "cat /https/ex/@index" on the serial console | ✅ | 595ms | - [📜](./07/serial.log) - |
| 8 | Then the command output should contain "<html" | ❌ | 6078ms | - [📜](./08/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26751919887] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26778507426] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26812088160] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26845256394] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26865859383] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26868509481] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26906124036] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26928450186] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26977222173] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26978385027] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[27066133743] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[27137629266] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[27160352637] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[27241996254] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[27351416895] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[27383571369] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[27407793501] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[27511197615] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[27562768299] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[27699135156] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27952244199] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=952738017 elapsed_us=476369
[27952978086] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[28030315830] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[28099782381] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[28100746641] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[28224457074] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[28228434861] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[28230609066] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[28236807291] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[28241104023] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=19536
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28443380427] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[28444952151] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[28496972922] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet �� retrying
mount -t https example.com /https/ex
[30994128528] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='mount'
[30999673188] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/mount' with argv=["/bin/mount", "-t", "https", "example.com", "/https/ex"]
[31040173692] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/mount' TID=9 PID=9
[31114859523] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=10 PID=10
[31244368188] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/httpsd' TID=11 PID=11
[31253985048] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25l[31305929127] [[32mINFO [0m] [httpsd] [CPU2] HTTPSD_READY
[31307058585] [[32mINFO [0m] [httpsd] [CPU2] httpsd: entering main RPC loop for /https/ex
[31425520401] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[31583268915] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=14 PID=14
[31621810902] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=15 PID=15
[31652947689] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[31653930858] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[31691312565] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
[31721029593] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[31722050283] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[31722962139] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
[34111210683] [[32mINFO [0m] [netd] [CPU1] NETD: Starting network service...
[34141411293] [[32mINFO [0m] [netd] [CPU1] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[34323589014] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/netd' TID=16 PID=16
[34327874031] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=16)
[34439054958] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Lookup payload_len=4
[34440203688] [[32mINFO [0m] [httpsd] [CPU2] httpsd: dispatch_lookup path=''
[34441065021] [[32mINFO [0m] [httpsd] [CPU2] httpsd: resolve_path path=''
[34460953065] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Close payload_len=8
mounted type=https device=example.com target=/https/ex
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[34852962540] [[32mINFO [0m] [netd] [CPU1] NETD: Network ready
[35102499003] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: netd activation probe succeeded for PID 16 (/net/icmp/new is responsive)
cat /https/ex/@index
[35685198615] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[35688100866] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/cat' with argv=["/bin/cat", "/https/ex/@index"]
[35713086519] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/cat' TID=18 PID=18
[35715759816] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25l[35751315402] [[32mINFO [0m] [cat] [CPU3] cat: opening '/https/ex/@index'
[35794209957] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Lookup payload_len=10
[35795028687] [[32mINFO [0m] [httpsd] [CPU2] httpsd: dispatch_lookup path='@index'
[35795632455] [[32mINFO [0m] [httpsd] [CPU2] httpsd: resolve_path path='@index'
[35802789726] [[32mINFO [0m] [cat] [CPU3] cat: opened '/https/ex/@index' fd=8
[35830746270] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Poll payload_len=12
[35862186162] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[35880604485] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/ex/@index
[37150257870] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[37153011291] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/cat' with argv=["/bin/cat", "/https/ex/@index"]
[37178486697] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/cat' TID=19 PID=19
[37181984796] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25l[37217340369] [[32mINFO [0m] [cat] [CPU1] cat: opening '/https/ex/@index'
[37228410945] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Lookup payload_len=10
[37229282112] [[32mINFO [0m] [httpsd] [CPU2] httpsd: dispatch_lookup path='@index'
[37230009399] [[32mINFO [0m] [httpsd] [CPU2] httpsd: resolve_path path='@index'
[37241527224] [[32mINFO [0m] [cat] [CPU1] cat: opened '/https/ex/@index' fd=9
[37267677150] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Poll payload_len=12
[37279197549] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[37308921210] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/ex/@index
[38433718356] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[38436551373] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/cat' with argv=["/bin/cat", "/https/ex/@index"]
[38462094099] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/cat' TID=20 PID=20
[38465776041] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25l[38717172648] [[32mINFO [0m] [cat] [CPU2] cat: opening '/https/ex/@index'
[38729975592] [[32mINFO [0m] [httpsd] [CPU0] httpsd: RPC op=Lookup payload_len=10
[38732043966] [[32mINFO [0m] [httpsd] [CPU0] httpsd: dispatch_lookup path='@index'
[38733240183] [[32mINFO [0m] [httpsd] [CPU0] httpsd: resolve_path path='@index'
[38991626553] [[32mINFO [0m] [cat] [CPU2] cat: opened '/https/ex/@index' fd=10
[39000353403] [[32mINFO [0m] [httpsd] [CPU0] httpsd: RPC op=Poll payload_len=12
[39104696730] [[32mINFO [0m] [httpsd] [CPU0] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[392037
```
</details>
