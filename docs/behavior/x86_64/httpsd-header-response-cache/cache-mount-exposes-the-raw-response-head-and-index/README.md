# ❌ Scenario: Cache mount exposes the raw response head and index

> Last run: 2026-04-23 19:11:44

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8317ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 252ms | - [📜](./02/serial.log) - |
| 3 | And I type "ping -c 1 example.com" on the serial console | ✅ | 405ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "1 packets transmitted, 1 received" | ❌ | 6100ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26012631513] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26039084214] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26069133651] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26101614297] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26121997308] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26124452409] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26160151347] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26180944548] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26226571041] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26227565661] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26310796314] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26380371336] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26402540439] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26474552775] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26578314180] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26604699924] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26627624001] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[26723500155] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[26771892279] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[26903960754] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27140886300] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=892092234 elapsed_us=446046
[27141831189] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27212858970] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27283103661] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27284089833] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27367979298] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[27373260882] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27375219069] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27385643406] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=19932
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[27616157514] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[27617370264] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[27652551234] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
ping -c 1 example.com
[29211807966] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ping'
[29218521816] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/ping' with argv=[[47, 98, 105, 110, 47, 112, 105, 110, 103], [45, 99], [49], [101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109]]
[29254241709] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/ping' TID=9 PID=9
ping: example.com: cannot open /net/dns/lookup
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;91mERR[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[30069907923] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=10 PID=10
[30260421741] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=12 PID=12
[30296007324] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=13 PID=13
[30301833375] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[30302690715] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing primary port (0x1F0)...
[30330193806] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing secondary port (0x170)...
[30354393036] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[30355099698] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Entering RPC service loop
[30355813950] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: No active devices to service
[31302959688] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[33953768244] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[34008649389] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[34163957751] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/netd' TID=14 PID=14
[34166777931] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=14)
[34882241955] [[32mINFO [0m
```
</details>
