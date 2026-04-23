# ✅ Scenario: Self hostname is published in /hosts

> Last run: 2026-04-22 22:12:03

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 9012ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 253ms | - [📜](./02/serial.log) - |
| 3 | And I type "cat /etc/hostname" on the serial console | ✅ | 340ms | - [📜](./03/serial.log) - |
| 4 | Then the command output should contain "thingos" | ✅ | 0ms | - - - |
| 5 | When I type "ls /hosts" on the serial console | ✅ | 724ms | - [📜](./05/serial.log) - |
| 6 | And I wait for 2 seconds | ✅ | 2001ms | - [📜](./06/serial.log) - |
| 7 | Then the command output should contain "thingos.local" | ✅ | 0ms | - [📜](./07/serial.log) - |
| 8 | When I type "attr_list /hosts/thingos.local" on the serial console | ✅ | 3620ms | - [📜](./08/serial.log) - |
| 9 | And I wait for 1 seconds | ✅ | 1001ms | - [📜](./09/serial.log) - |
| 10 | Then the command output should contain "net.ip.ipv4" | ✅ | 0ms | - - - |
| 11 | And the command output should contain "net.hostname" | ✅ | 0ms | - - - |
| 12 | When I type "attr_get /hosts/thingos.local net.hostname" on the serial console | ✅ | 898ms | - [📜](./12/serial.log) - |
| 13 | And I wait for 1 seconds | ✅ | 1001ms | - [📜](./13/serial.log) - |
| 14 | Then the command output should contain "thingos.local" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[27870123039] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[27899341437] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[27934203660] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[27969291372] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[27993779253] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[27996873300] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[28035170856] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[28056860634] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[28108715349] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[28109867874] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[28204042812] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[28280396859] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[28305683241] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[28382693427] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[28492631838] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[28522790868] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[28548407349] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[28677217899] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[28737379209] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[28903712310] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[29216356422] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1084000104 elapsed_us=542000
[29217250623] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[29299726533] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[29372901096] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[29374190505] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[29464475370] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[29467119363] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[29477830206] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=21252
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[29682643089] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mounting /hosts early...
[29748643320] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned early /hosts mount (PID=9)
[29768816352] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet �� retrying
[29911745787] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: starting (mount_point=/hosts)
[29945802447] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: mounted at /hosts
[29959358154] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: advertising hostname thingos.local
[29963265321] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: serving cache immediately; mDNS socket will attach when netd is ready
[30177535938] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
cat mounted type=mdns device=none target=/hosts
/etc/hostname
[31268590353] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[31345875132] [[32mINFO [0m] [cat] [CPU3] cat: opening '/etc/hostname'
[31350099924] [[32mINFO [0m] [cat] [CPU3] cat: opened '/etc/hostname' fd=7
thingos
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hls /hosts
[31960150431] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[?25l[32683140402] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[32684200461] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing primary port (0x1F0)...
[32718195675] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing secondary port (0x170)...
[32761317699] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[32763522000] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Entering RPC service loop
[32764572654] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: No active devices to service
[33083141652] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
thingos.local  
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[34634640381] [[32mINFO [0m] [netd] [CPU3] NETD: Starting network service...
[34677805503] [[32mINFO [0m] [netd] [CPU3] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[35209964361] [[32mINFO [0m] [netd] [CPU3] NETD: Network ready
attr_list /hosts/thingos.local
[42053210298] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[?25l[51631991202] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=11 op=Lookup resp_port=3 TIMEOUT (5s) - tainting provider
net.ip.ipv4	Utf8	7
net.hostname	Utf8	13
count=2
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_get /hosts/thingos.local net.hostname
[57928390872] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_get'
[?25ltype=Utf8 len=13
value=thingos.local
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;9
```
</details>
