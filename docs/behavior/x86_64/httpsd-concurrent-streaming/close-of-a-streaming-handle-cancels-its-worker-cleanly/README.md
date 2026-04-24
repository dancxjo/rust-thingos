# ❌ Scenario: Close of a streaming handle cancels its worker cleanly

> Last run: 2026-04-23 19:11:44

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8307ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 253ms | - [📜](./02/serial.log) - |
| 3 | And I type "ping -c 1 example.com" on the serial console | ✅ | 405ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "1 packets transmitted, 1 received" | ❌ | 6008ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[25987611969] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26014477401] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26045041638] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26078311017] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26098863648] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26101453851] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26137913637] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26159162898] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26207056194] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26208693093] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26300041878] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26367915882] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26389748979] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26461928361] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26566389960] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26593658718] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26616842637] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[26711870064] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[26759985582] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[26887044294] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27123657594] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=892903902 elapsed_us=446451
[27124300896] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27196264062] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27293707881] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27294736656] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27381460821] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[27385403496] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27387364455] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27394064742] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=20130
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[27673471287] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[27674975823] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[27716450454] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
ping -c 1 example.com
[29169989772] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ping'
[29175445299] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/ping' with argv=[[47, 98, 105, 110, 47, 112, 105, 110, 103], [45, 99], [49], [101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109]]
[29214255444] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/ping' TID=9 PID=9
ping: example.com: cannot open /net/dns/lookup
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;91mERR[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[30116890716] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=10 PID=10
[30156739635] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=11 PID=11
[30203151231] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=12 PID=12
[30206663850] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[30207494856] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[30258839655] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
[30283941006] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[30284683209] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[30285493623] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
[35826362088] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[38373201999] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[38405427357] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[38583563832] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/netd' TID=14 PID=14
[38586214755] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=14)
[38873561364] [[32mI
```
</details>
