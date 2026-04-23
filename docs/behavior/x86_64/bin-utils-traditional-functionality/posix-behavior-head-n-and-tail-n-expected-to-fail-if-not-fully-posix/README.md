# ✅ Scenario: POSIX behavior - head -n and tail -n (expected to fail if not fully POSIX)

> Last run: 2026-04-22 21:17:04

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8417ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 253ms | - [📜](./02/serial.log) - |
| 3 | And I type "echo -e '1\n2\n3' | head -n 2 | tail -n 1" on the serial console | ✅ | 1190ms | - [📜](./03/serial.log) - |
| 4 | Then the serial output should contain "2" within 5s | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26354272956] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26379202047] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26409498621] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26440114602] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26458653012] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26461085607] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26494151772] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26513042259] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26557866588] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26558815569] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: mile[268960386stone="Tasking Initialized"
[26636789685] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26705617092] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26726040033] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26797010394] [[32mINFO 42] [[32mINFO [0m] [k[er0mne] l::boot_prog[kernerel:ss:b] oo[Ct_PUpr0]og breooss] [CPU0] bot_otpr_pogroregrsses: s:mi mleileststonone=e="B"SMPSP Timer O Bring-up"
K"
[26924620371] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26946762315] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[27055535991] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[27101934387] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[27253425375] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27533530167] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=955000794 elapsed_us=477500
[27534275109] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27605200557] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27669769842] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27670706415] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27751429431] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27754998051] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27766091463] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=27159
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28004812077] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mounting /hosts early...
[28031653848] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned early /hosts mount (PID=9)
[28035186894] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[28166806569] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: starting (mount_point=/hosts)
[28181945484] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: mounted at /hosts
[28188342600] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: advertising hostname thingos.local
[28190304285] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: serving cache immediately; mDNS socket will attach when netd is ready
echo -mounted type=mdns device=none target=/hosts
e '1\[28983077838] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
n2\n3' | head -n 2 | tail -n[30591537834] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[30593012241] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Probing primary port (0x1F0)...
[30620192130] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Probing secondary port (0x170)...
[30644621700] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[30645419904] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Entering RPC service loop
[30646270149] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: No active devices to service
[30900113046] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
 1
[31688093679] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='echo'
[31718961054] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='head'
[31818515553] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='tail'
[?25l[32224752021] [[32mINFO [0m] [netd] [CPU1] NETD: Starting network service...

```
</details>
