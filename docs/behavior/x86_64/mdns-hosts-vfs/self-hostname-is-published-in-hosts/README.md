# ✅ Scenario: Self hostname is published in /hosts

> Last run: 2026-04-22 21:44:56

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8412ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 253ms | - [📜](./02/serial.log) - |
| 3 | And I type "cat /etc/hostname" on the serial console | ✅ | 342ms | - [📜](./03/serial.log) - |
| 4 | Then the command output should contain "thingos" | ✅ | 0ms | - - - |
| 5 | When I type "ls /hosts" on the serial console | ✅ | 727ms | - [📜](./05/serial.log) - |
| 6 | And I wait for 2 seconds | ✅ | 2001ms | - [📜](./06/serial.log) - |
| 7 | Then the command output should contain "thingos.local" | ✅ | 0ms | - [📜](./07/serial.log) - |
| 8 | When I type "attr_list /hosts/thingos.local" on the serial console | ✅ | 3473ms | - [📜](./08/serial.log) - |
| 9 | And I wait for 1 seconds | ✅ | 1001ms | - [📜](./09/serial.log) - |
| 10 | Then the command output should contain "net.ip.ipv4" | ✅ | 0ms | - - - |
| 11 | And the command output should contain "net.hostname" | ✅ | 0ms | - - - |
| 12 | When I type "attr_get /hosts/thingos.local net.hostname" on the serial console | ✅ | 851ms | - [📜](./12/serial.log) - |
| 13 | And I wait for 1 seconds | ✅ | 1001ms | - [📜](./13/serial.log) - |
| 14 | Then the command output should contain "thingos.local" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26120182209] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26147033847] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26179367247] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26211833769] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26231665350] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26234706399] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26270474703] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26290103070] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26338842156] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26339995374] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26428535991] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26496412602] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26517708360] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26596481736] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26695790121] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26721719640] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26744349918] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[26856481971] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[26907033945] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[27060909645] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27347231274] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=987018351 elapsed_us=493509
[27348083796] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27422300895] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27491617395] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27492689961] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27582156954] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27585215955] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27594841263] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=31185
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[27803996484] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mounting /hosts early...
[27831873366] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned early /hosts mount (PID=9)
[27848303934] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[27890786121] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[27999076842] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: starting (mount_point=/hosts)
[28016983203] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: mounted at /hosts
[28024097013] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: advertising hostname thingos.local
[28026030747] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: serving cache immediately; mDNS socket will attach when netd is ready
catmounted type=mdns device=none target=/hosts
 /etc/hostname
[29313538485] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[29377979664] [[32mINFO [0m] [cat] [CPU3] cat: opening '/etc/hostname'
[29381509740] [[32mINFO [0m] [cat] [CPU3] cat: opened '/etc/hostname' fd=7
thingos
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hls /hosts
[30016773171] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[?25l[30419552889] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[30420546453] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing primary port (0x1F0)...
[30448622292] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing secondary port (0x170)...
[30474673119] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[30475552338] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Entering RPC service loop
[30477261210] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: No active devices to service
[30791525094] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
thingos.local  
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[32164488807] [[32mINFO [0m] [netd] [CPU3] NETD: Starting network service...
[32197829004] [[32mINFO [0m] [netd] [CPU3] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[32627383074] [[32mINFO [0m] [netd] [CPU3] NETD: Network ready
attr_list /hosts/thingos.local
[40123047756] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[?25l[49263256119] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU2] VFS RPC: tid=11 op=Lookup resp_port=3 TIMEOUT (5s) - tainting provider
net.ip.ipv4	Utf8	7
net.hostname	Utf8	13
count=2
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_get /hosts/thingos.local net.hostname
[55504554699] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_get'
[?25ltype=Utf8 len=13
value=thingos.local
[?25h[1;95mTHING[0m[
```
</details>
