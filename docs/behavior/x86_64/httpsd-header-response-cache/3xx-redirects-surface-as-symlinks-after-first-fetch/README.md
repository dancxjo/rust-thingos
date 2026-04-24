# ❌ Scenario: 3xx redirects surface as symlinks after first fetch

> Last run: 2026-04-23 19:11:44

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8318ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 254ms | - [📜](./02/serial.log) - |
| 3 | And I type "ping -c 1 iana.org" on the serial console | ✅ | 357ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "1 packets transmitted, 1 received" | ❌ | 6109ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26034891663] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26061664926] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26092007139] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26125024959] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26146046289] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26148535941] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26184185841] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26204751606] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26251283619] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26252291241] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26333628849] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26403000294] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26424868041] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26496697392] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26600426919] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26626521636] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26650007175] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[26750680275] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[26799614358] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[26926922286] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27166208388] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=892635876 elapsed_us=446317
[27166898484] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27237469083] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27309286851] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27310919328] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27413853027] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[27420501273] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
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
[27467623095] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27476567118] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=19899
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[27701767599] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[27702954774] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[27734333781] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet ��� retrying
ping -c 1 iana.org
[29053181718] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ping'
[29059015623] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/ping' with argv=[[47, 98, 105, 110, 47, 112, 105, 110, 103], [45, 99], [49], [105, 97, 110, 97, 46, 111, 114, 103]]
[29095580712] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/ping' TID=9 PID=9
ping: iana.org: cannot open /net/dns/lookup
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;91mERR[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[30090219225] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=10 PID=10
[30129696762] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=11 PID=11
[30734107602] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=13 PID=13
[30742653744] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[30743555238] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing primary port (0x1F0)...
[30772153203] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing secondary port (0x170)...
[30799731666] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[30800636097] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Entering RPC service loop
[30801651276] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: No active devices to service
[30982520415] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[33783755511] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[33819474018] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[34003465122] [[32m
```
</details>
