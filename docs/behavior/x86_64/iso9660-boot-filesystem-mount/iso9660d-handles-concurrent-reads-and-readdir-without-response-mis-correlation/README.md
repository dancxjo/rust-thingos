# ❌ Scenario: iso9660d handles concurrent reads and readdir without response mis-correlation

> Last run: 2026-04-23 19:11:44

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8307ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 253ms | - [📜](./02/serial.log) - |
| 3 | And I type "ls /mnt/iso && cat /mnt/iso/etc/hostname" on the serial console | ✅ | 714ms | - [📜](./03/serial.log) - |
| 4 | Then the latest command output should contain "thingos" | ❌ | 6086ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[25779039000] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[25806287595] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[25837264992] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[25870632975] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[25891665162] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[25894323345] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[25930181574] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[25951326390] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[25998897045] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26000118408] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26084921676] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26155990245] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26178670518] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26259323739] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26368226808] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26395027098] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26419330872] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[26518281537] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[26569662801] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[26704959699] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[26951828310] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=930138726 elapsed_us=465069
[26952635490] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27026166288] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27091651554] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27094603371] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27175527819] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[27179784753] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27183538008] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27191639574] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=23628
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[27395094111] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[27396735168] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[27434246895] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet �� retrying
ls /mnt/iso && cat /mnt/iso/etc/ho[29850904479] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=9 PID=9
[29895757947] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=10 PID=10
[30024097059] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=12 PID=12
[30027648453] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[30028954956] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[30081545769] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
stnam[30107268180] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[30108177363] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[30108971376] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
e
[30183544743] [[32mINFO [0m] [sh] [CPU1] sh: spawning job cmd='ls'
[30189302484] [[32mINFO [0m] [sh] [CPU1] sh: spawning '/bin/ls' with argv=[[47, 98, 105, 110, 47, 108, 115], [47, 109, 110, 116, 47, 105, 115, 111], [38], [38], [99, 97, 116], [47, 109, 110, 116, 47, 105, 115, 111, 47, 101, 116, 99, 47, 104, 111, 115, 116, 110, 97, 109, 101]]
[30254018025] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/bin/ls' TID=13 PID=13
[?25l/mnt/iso:
[30311545572] [[33mWARN [0m] [ls] [CPU1] ls: failed to open '/mnt/iso': ENOENT
ls: cannot access '/mnt/iso': No such file or directory

&:
[30317208042] [[33mWARN [0m] [ls] [CPU1] ls: failed to open '&': ENOENT
ls: cannot access '&': No such file or directory

&:
[30320951265] [[33mWARN [0m] [ls] [CPU1] ls: failed to open '&': ENOENT
ls: cannot access '&': No such file or directory

cat:
[30324667197] [[33mWARN [0m] [ls] [CPU1] ls: failed to open 'cat': ENOENT
ls: cannot access 'cat': No such file or directory

/mnt/iso/etc/hostname:
[30330015870] [[33mWARN [0m] [ls] [CPU1] ls: failed to open '/mnt/iso/etc/hostname': ENOENT
ls: cannot access '/mnt/iso/etc/hostname': No such file or directory
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[30898711767] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[33735815025] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[33788270010] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[33948702843] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/netd' TID=14 PID=14
[33952426761] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=14)
[34718026464] [[
```
</details>
