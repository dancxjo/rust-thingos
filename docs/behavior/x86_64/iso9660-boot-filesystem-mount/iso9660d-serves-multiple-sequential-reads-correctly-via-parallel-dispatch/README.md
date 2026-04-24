# ❌ Scenario: iso9660d serves multiple sequential reads correctly via parallel dispatch

> Last run: 2026-04-23 19:11:44

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8301ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 252ms | - [📜](./02/serial.log) - |
| 3 | And I type "cat /mnt/iso/etc/hostname" on the serial console | ✅ | 473ms | - [📜](./03/serial.log) - |
| 4 | Then the latest command output should contain "thingos" | ❌ | 6079ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[25985269761] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26012223831] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26043693126] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26077110312] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26098307829] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26100925224] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26137512984] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26159061291] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26206342536] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26207402166] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26293480818] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26364894765] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26387737431] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26461939350] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26567079033] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26595827346] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26618960181] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[26720001462] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[26779197093] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[26911854882] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27160044681] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=930361179 elapsed_us=465180
[27160865391] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27239133636] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27307790070] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27308717700] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27396905745] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[27403762551] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27409417068] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27418447089] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=21021
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[27651119958] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[27652903938] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[27689097678] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
cat /mnt/iso/etc/hostname
[29363853783] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[29369128569] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/cat' with argv=[[47, 98, 105, 110, 47, 99, 97, 116], [47, 109, 110, 116, 47, 105, 115, 111, 47, 101, 116, 99, 47, 104, 111, 115, 116, 110, 97, 109, 101]]
[29401603572] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/cat' TID=9 PID=9
[29449416018] [[32mINFO [0m] [cat] [CPU3] cat: opening '/mnt/iso/etc/hostname'
cat: failed to open /mnt/iso/etc/hostname
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[30144337113] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=10 PID=10
[30189024789] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=11 PID=11
[30238703385] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=12 PID=12
[30244643385] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[30267261156] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[30295322475] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
[30321696504] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[30322470321] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[30323230707] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
[31746959508] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[34420765764] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[34452410091] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[34635298467] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/netd' TID=14 PID=14
[34639430562] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=14)
[35033403834] [[32mINFO [0m] [ne
```
</details>
