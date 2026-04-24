# ❌ Scenario: Repeat cd into https and cat example content three times

> Last run: 2026-04-23 19:11:44

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8406ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 253ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https example.com /https/ex" on the serial console | ✅ | 1782ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ✅ | 0ms | - - - |
| 5 | And I type "cat /https/ex/@index" on the serial console | ✅ | 391ms | - [📜](./05/serial.log) - |
| 6 | And I type "cat /https/ex/@index" on the serial console | ✅ | 445ms | - [📜](./06/serial.log) - |
| 7 | And I type "cat /https/ex/@index" on the serial console | ✅ | 496ms | - [📜](./07/serial.log) - |
| 8 | Then the command output should contain "<html" | ❌ | 6075ms | - [📜](./08/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26110668375] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26138099460] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26169884301] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26204990724] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26226057495] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26228660337] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26265460848] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26287185012] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26335304589] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26336363625] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26421614406] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26492066601] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26515138914] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26595094086] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26706735297] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26736907494] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26761863216] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[26864019699] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[26917919787] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[27051311298] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27296996463] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=939064368 elapsed_us=469532
[27297740448] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27371720607] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27438338367] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27439236198] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27525041544] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[27529160076] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27531136413] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27537987906] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=20493
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[27780077358] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[27781359969] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[27816850743] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
mount -t https example.com /https/e[30228879252] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=9 PID=9
[30272985897] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=10 PID=10
[30319770393] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=11 PID=11
[30323698515] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[30324564864] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Probing primary port (0x1F0)...
[30357940635] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Probing secondary port (0x170)...
[30391210641] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[30391980234] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Entering RPC service loop
[30392703000] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: No active devices to service
[30925248090] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
x
[33513422052] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='mount'
[33518931006] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/mount' with argv=[[47, 98, 105, 110, 47, 109, 111, 117, 110, 116], [45, 116], [104, 116, 116, 112, 115], [101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109], [47, 104, 116, 116, 112, 115, 47, 101, 120]]
[33551762706] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/mount' TID=13 PID=13
[?25l[33586645389] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[33615932262] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[33768902409] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/bin/httpsd' TID=15 PID=15
[33800260791] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/netd' TID=14 PID=14
[33803293854] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=14)
[33836598774] [[32mINFO [0m] [httpsd] [CPU3] HTTPSD_READY
[33838153602] [[32mINFO [0m] [httpsd] [CPU3] httpsd: entering main RPC loop for /https/ex
[34108530720] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=4
[34109594079] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path=''
[34110301830] [[32mINFO [0m] [httpsd] [CPU3] httpsd: resolve_path path=''
[34131756912] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Close payload_len=8
mounted type=https device=example.com target=/https/ex
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/ex/@index
[35315982867] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[35318901717] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/cat' with argv=[[47, 98, 105, 110, 47, 99, 97, 116], [47, 104, 116, 116, 112, 115, 47, 101, 120, 47, 64, 105, 110, 100, 101, 120]]
[35342029965] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/cat' TID=17 PID=17
[?25l[35372733000] [[32mINFO [0m] [cat] [CPU2] cat: opening '/https/ex/@index'
[35383167699] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=10
[35384104965] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path='@index'
[35384742294] [[32mINFO [0m] [httpsd] [CPU3] httpsd: resolve_path path='@index'
[35391955005] [[32mINFO [0m] [cat] [CPU2] cat: opened '/https/ex/@index' fd=8
[35401884507] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Poll payload_len=12
[35416027812] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[35427767562] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[35555588574] [[32mINFO [0m] [netd] [CPU1] NETD: Network ready
cat /https/ex/@index
[36616930020] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[36619720830] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/cat' with argv=[[47, 98, 105, 110, 47, 99, 97, 116], [47, 104, 116, 116, 112, 115, 47, 101, 120, 47, 64, 105, 110, 100, 101, 120]]
[36643886136] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/cat' TID=19 PID=19
[?25l[36684150723] [[32mINFO [0m] [cat] [CPU1] cat: opening '/https/ex/@index'
[36716235237] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=10
[36716983677] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path='@index'
[36717625659] [[32mINFO [0m] [httpsd] [CPU3] httpsd: resolve_path path='@index'
[36728544996] [[32mINFO [0m] [cat] [CPU1] cat: opened '/https/ex/@index' fd=9
[36765128697] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Poll payload_len=12
[36786945129] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[36802966530] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/ex/@index
[38077947204] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[38080664292] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/cat' with argv=[[47, 98, 105, 110, 47, 99, 97, 116], [47, 104, 116, 116, 112, 115, 47, 101, 120, 47, 64, 105, 110, 100, 101, 120]]
[38105496594] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/cat' TID=20 PID=20
[?25l[38135220354] [[32mINFO [0m] [cat] [CPU2] cat: opening '/https/ex/@index'
[38140783593] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=10
[38141639151] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path='@index'
[38142242457] [[32mINFO [0m] [httpsd] [CPU3] httpsd: resolve_path path='@index'
[38148582120] [[32mINFO [0m] [cat] [CPU2] cat: opened '/https/ex/@index' fd=10
[38152670853] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Poll payload_len=12
[38162647479] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[38175887904] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[52960869984] [[31;1mERROR
```
</details>
