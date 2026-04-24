# ❌ Scenario: Repeat cd into https and cat example content three times

> Last run: 2026-04-23 15:53:33

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 12066ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 355ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https example.com /https/ex" on the serial console | ✅ | 966ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ✅ | 0ms | - - - |
| 5 | And I type "cat /https/ex/@index" on the serial console | ✅ | 446ms | - [📜](./05/serial.log) - |
| 6 | And I type "cat /https/ex/@index" on the serial console | ✅ | 503ms | - [📜](./06/serial.log) - |
| 7 | And I type "cat /https/ex/@index" on the serial console | ✅ | 551ms | - [📜](./07/serial.log) - |
| 8 | Then the command output should contain "<html" | ❌ | 6098ms | - [📜](./08/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[37511288463] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[37561029198] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[37605901674] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[37655592513] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[37685805333] [[32mINFO [0m] [kernel] [C[38397937974] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
PU0] Initializing SIMD...
[37689425862] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[37748955354] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[37780745310] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[37860069225] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[37861770540] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[37989883074] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[38106307536] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[38140581039] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[38236003014] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[38466634701] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[38509448604] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[38714130444] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[38819756382] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[39058799538] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[39475820340] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1582582749 elapsed_us=791291
[39476832351] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[39627361125] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[39843421101] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[39845580588] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[40031880240] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[40040721435] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[40045488417] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[40068191724] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=42438
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[40363679565] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[40366544130] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[40472785452] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
mount -t https example.com /https/ex
[42649149939] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='mount'
[42661069968] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/mount' with argv=[[47, 98, 105, 110, 47, 109, 111, 117, 110, 116], [45, 116], [104, 116, 116, 112, 115], [101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109], [47, 104, 116, 116, 112, 115, 47, 101, 120]]
[42730592553] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/mount' TID=9 PID=9
[43031976075] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/httpsd' TID=10 PID=10
[?25l[43253555433] [[32mINFO [0m] [httpsd] [CPU1] HTTPSD_READY
[43256030862] [[32mINFO [0m] [httpsd] [CPU1] httpsd: entering main RPC loop for /https/ex
[43442089317] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Lookup payload_len=4
[43443559104] [[32mINFO [0m] [httpsd] [CPU1] httpsd: dispatch_lookup path=''
[43444620681] [[32mINFO [0m] [httpsd] [CPU1] httpsd: resolve_path path=''
[43618005354] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Close payload_len=8
mounted type=https device=example.com target=/https/ex
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/ex/@index
[44965070205] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[44971579323] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/cat' with argv=[[47, 98, 105, 110, 47, 99, 97, 116], [47, 104, 116, 116, 112, 115, 47, 101, 120, 47, 64, 105, 110, 100, 101, 120]]
[45010300764] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/cat' TID=12 PID=12
[45012220572] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=13 PID=13
[?25l[45075061251] [[32mINFO [0m] [cat] [CPU3] cat: opening '/https/ex/@index'
[45116538357] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Lookup payload_len=10
[45118267293] [[32mINFO [0m] [httpsd] [CPU2] httpsd: dispatch_lookup path='@index'
[45119881686] [[32mINFO [0m] [httpsd] [CPU2] httpsd: resolve_path path='@index'
[45136265526] [[32mINFO [0m] [cat] [CPU3] cat: opened '/https/ex/@index' fd=8
[45151468329] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Poll payload_len=12
[45175705311] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[45198125148] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /ht[45720693513] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
tps/ex/@index
[46459203186] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[46466565882] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/cat' with argv=[[47, 98, 105, 110, 47, 99, 97, 116], [47, 104, 116, 116, 112, 115, 47, 101, 120, 47, 64, 105, 110, 100, 101, 120]]
[46516972887] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/cat' TID=15 PID=15
[46553347731] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=16 PID=16
[46583582166] [[32mINFO [0m] [cat] [CPU3] cat: opening '/https/ex/@index'
[46598645016] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Lookup payload_len=10
[46600601124] [[32mINFO [0m] [httpsd] [CPU2] httpsd: dispatch_lookup path='@index'
[46602188952] [[32mINFO [0m] [httpsd] [CPU2] httpsd: resolve_path path='@index'
[?25l[46664065668] [[32mINFO [0m] [cat] [CPU3] cat: opened '/https/ex/@index' fd=9
[46677076743] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Poll payload_len=12
[46724242455] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[46755501309] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[46794154671] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=17 PID=17
[46806593559] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[46808484987] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Probing primary port (0x1F0)...
[46931552778] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Probing secondary port (0x170)...
[47006300187] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[47008166964] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Entering RPC service loop
[47009920617] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: No active devices to service
ahptt/ tsc/e
[48649339695] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ahptt/'
[48657807660] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/ahptt/' with argv=[[47, 98, 105, 110, 47, 97, 104, 112, 116, 116, 47], [116, 115, 99, 47, 101]]
sh: spawn failed: no such file or directory
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hxedni@/x[50981434053] [[32mINFO [0m] [netd] [CPU3] NETD: Starting network service...
[51065761659] [[32mINFO [0m] [netd] [CPU3] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[51327353286] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/netd' TID=18 PID=18
[51333458616] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=18)
[52116310125] [[32mINF
```
</details>
