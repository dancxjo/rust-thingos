# ❌ Scenario: Response headers are published as xattrs

> Last run: 2026-04-22 22:12:03

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8914ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 253ms | - [📜](./02/serial.log) - |
| 3 | And I type "ping -c 1 example.com" on the serial console | ✅ | 398ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "1 packets transmitted, 1 received" | ❌ | 6102ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[27665401071] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[27693234459] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[27727953660] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[27762250032] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[27783312876] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[27786115929] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[27824977587] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[27845910180] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[27898518120] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[27899735193] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[27991288842] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[28066259793] [[32mINFO [0m] [kernel::[28279460088] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="boot_progress]SM [CPP U0Br] bootin_pg-rogresup"
s: milestone="PCI Bus Scanned"
[28089247032] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[28164983550] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[28315100583] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[28338824778] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[28465065816] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[28524484263] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[28695299292] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[29009926803] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1088120946 elapsed_us=544060
[29010866808] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[29101099335] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[29183402028] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[29184480402] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[29295881307] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[29299504542] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[29311704576] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=27786
[1;32m
        .-.
       /   \        [1;36mTHING-OS[1;32m
      |     |       [0;36m"People, places, things."[1;32m
       \   /        
        `-'        
       /   \        v0.1  ��  
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[29667736890] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mounting /hosts early...
[29700706695] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned early /hosts mount (PID=9)
[29720815410] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[29752104492] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[29868685968] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: starting (mount_point=/hosts)
[29888216952] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: mounted at /hosts
[29895279513] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: advertising hostname thingos.local
[29897703594] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: serving cache immediately; mDNS socket will attach when netd is ready
ping -c mounted type=mdns device=none target=/hosts
1 example.com
[31168965927] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ping'
ping: example.com: cannot open /net/dns/lookup
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;91mERR[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[32620493334] [[32mINFO [0m] [ata_disk] [CPU0] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[32650772418] [[32mINFO [0m] [ata_disk] [CPU0] ATA_DISK: Probing primary port (0x1F0)...
[32692273581] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[32694594405] [[32mINFO [0m] [ata_disk] [CPU0] ATA_DISK: Probing secondary port (0x170)...
[32724696675] [[32mINFO [0m] [ata_disk] [CPU0] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[32725585860] [[32mINFO [0m] [ata_disk] [CPU0] ATA_DISK: Entering RPC service loop
[32726458347] [[32mINFO [0m] [ata_disk] [CPU0] ATA_DISK: No active devices to service
[34242945957] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[34309008657] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[35256100440] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready

```
</details>
