# ✅ Scenario: Self hostname is published in /hosts

> Last run: 2026-04-22 21:33:14

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8517ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 151ms | - [📜](./02/serial.log) - |
| 3 | And I type "cat /etc/hostname" on the serial console | ✅ | 342ms | - [📜](./03/serial.log) - |
| 4 | Then the serial output should contain "thingos" | ✅ | 0ms | - - - |
| 5 | When I type "ls /hosts" on the serial console | ✅ | 776ms | - [📜](./05/serial.log) - |
| 6 | And I wait for 2 seconds | ✅ | 2002ms | - [📜](./06/serial.log) - |
| 7 | Then the serial output should contain "thingos.local" | ✅ | 0ms | - [📜](./07/serial.log) - |
| 8 | When I type "attr_list /hosts/thingos.local" on the serial console | ✅ | 3729ms | - [📜](./08/serial.log) - |
| 9 | And I wait for 1 seconds | ✅ | 1001ms | - [📜](./09/serial.log) - |
| 10 | Then the serial output should contain "net.ip.ipv4" | ✅ | 0ms | - - - |
| 11 | And the serial output should contain "net.hostname" | ✅ | 0ms | - - - |
| 12 | When I type "attr_get /hosts/thingos.local net.hostname" on the serial console | ✅ | 846ms | - [📜](./12/serial.log) - |
| 13 | And I wait for 1 seconds | ✅ | 1001ms | - [📜](./13/serial.log) - |
| 14 | Then the serial output should contain "thingos.local" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26308947555] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26335012308] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26366551827] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26397993138] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26417282034] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26419819338] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26454254640] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26473746849] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26520202335] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26521275330] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26605029858] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26674838454] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26700833643] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26784366939] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26886074556] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26912514420] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26936807469] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[27057586182] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[27109868577] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[27273370806] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27559755135] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1018089600 elapsed_us=509044
[27560567001] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27662236239] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27727052958] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27730153407] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27811965225] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27814145865] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27825186708] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=20064
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28032301770] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mounting /hosts early...
[28057884459] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned early /hosts mount (PID=9)
[28060936926] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[28190972040] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: starting (mount_point=/hosts)
[28207621233] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: mounted at /hosts
[28214639475] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: advertising hostname thingos.local
[28216626438] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: serving cache immediately; mDNS socket will attach when netd is ready
[28342909914] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
cat /etmounted type=mdns device=none target=/hosts
c/hostname
[29317878810] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[29376995010] [[32mINFO [0m] [cat] [CPU3] cat: opening '/etc/hostname'
[29380701537] [[32mINFO [0m] [cat] [CPU3] cat: opened '/etc/hostname' fd=7
thingos
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hls /hosts
[30007116579] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[?25l[30790535655] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[30792066756] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing primary port (0x1F0)...
[30835360413] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing secondary port (0x170)...
[30864904092] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[30865878516] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Entering RPC service loop
[30867120570] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: No active devices to service
[31093114206] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
thingos.local  
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[32479550631] [[32mINFO [0m] [netd] [CPU3] NETD: Starting network service...
[32510572809] [[32mINFO [0m] [netd] [CPU3] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[34038805152] [[32mINFO [0m] [netd] [CPU3] NETD: Network ready
attr_list /hosts/thingos.local
[40277321931] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[?25l[50307686649] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU1] VFS RPC: tid=11 op=Lookup resp_port=3 TIMEOUT (5s) - tainting provider
net.ip.ipv4	Utf8	7
net.hostname	Utf8	13
count=2
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_get /hosts/thingos.local net.hostname
[56507128491] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_get'
[?25ltype=Utf8 len=13
value=thingos.local
[?25h[1;95mTHING[0m[1;
```
</details>
