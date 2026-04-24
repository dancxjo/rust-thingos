# ❌ Scenario: Two simultaneous large fetches complete concurrently

> Last run: 2026-04-23 19:11:44

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8312ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 251ms | - [📜](./02/serial.log) - |
| 3 | And I type "ping -c 1 example.com" on the serial console | ✅ | 406ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "1 packets transmitted, 1 received" | ❌ | 6107ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[25959385584] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[25985099448] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26015274285] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26047330815] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26067153453] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26069968353] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26108810079] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26129418183] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26176045269] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26177061603] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26258591073] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26327761482] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26349292134] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26420656680] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26524246287] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26553406407] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26577548349] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[26677124529] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[26725676604] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[26854267407] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27100662633] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=902880594 elapsed_us=451440
[27101307288] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27168367413] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27233291316] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27234178323] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27316375812] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[27321623142] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27323761179] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27337339953] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=20163
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[27548191341] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[27550087686] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[27589970067] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet �� retrying
ping -c 1 example.com
[29190339684] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ping'
[29195868273] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/ping' with argv=[[47, 98, 105, 110, 47, 112, 105, 110, 103], [45, 99], [49], [101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109]]
[29233405641] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/ping' TID=9 PID=9
ping: example.com: cannot open /net/dns/lookup
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;91mERR[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[30037477338] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=10 PID=10
[30228706464] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=12 PID=12
[30261229383] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=13 PID=13
[30268033488] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[30268870632] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing primary port (0x1F0)...
[30295357983] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing secondary port (0x170)...
[30321251301] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[30321957072] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Entering RPC service loop
[30322702674] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: No active devices to service
[36211390050] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[38776618155] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[38823855246] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[38983717245] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/netd' TID=14 PID=14
[38986436544] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=14)
[39713049618] [[32mIN
```
</details>
