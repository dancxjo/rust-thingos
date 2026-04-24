# ❌ Scenario: Response headers are published as xattrs

> Last run: 2026-04-23 19:11:44

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8309ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 252ms | - [📜](./02/serial.log) - |
| 3 | And I type "ping -c 1 example.com" on the serial console | ✅ | 410ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "1 packets transmitted, 1 received" | ❌ | 6105ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[25892101983] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[25918469808] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[25948762257] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[25981809579] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26001966474] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26004468105] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26039352075] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26059963215] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26106820344] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26107858953] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26189411985] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26258639088] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26280620289] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26353499997] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26463325944] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26489871276] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26514875739] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[26612147232] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[26665241097] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[26796571131] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27043980348] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=915080364 elapsed_us=457540
[27044743341] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27117588663] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27181229625] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27182378388] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27268369359] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[27275446473] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27278965923] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27312096306] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=44979
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[27489989967] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[27492091176] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[27527820342] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
ping -c 1 example.com
[29213935443] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ping'
[29219100735] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/ping' with argv=[[47, 98, 105, 110, 47, 112, 105, 110, 103], [45, 99], [49], [101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109]]
[29253559137] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/ping' TID=9 PID=9
ping: example.com: cannot open /net/dns/lookup
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;91mERR[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[29930211135] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=10 PID=10
[29971794072] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=11 PID=11
[30159147909] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=13 PID=13
[30165527238] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[30166472424] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing primary port (0x1F0)...
[30221434716] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing secondary port (0x170)...
[30247649355] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[30248407068] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Entering RPC service loop
[30249179268] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: No active devices to service
[30834949992] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[33416853987] [[32mINFO 
```
</details>
