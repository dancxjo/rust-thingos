# ✅ Scenario: Self hostname is published in /hosts

> Last run: 2026-04-22 20:35:10

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8614ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 254ms | - [📜](./02/serial.log) - |
| 3 | And I type "cat /etc/hostname" on the serial console | ✅ | 343ms | - [📜](./03/serial.log) - |
| 4 | Then the serial output should contain "thingos" | ✅ | 0ms | - - - |
| 5 | When I type "ls /hosts" on the serial console | ✅ | 673ms | - [📜](./05/serial.log) - |
| 6 | And I wait for 2 seconds | ✅ | 2002ms | - [📜](./06/serial.log) - |
| 7 | Then the serial output should contain "thingos.local" | ✅ | 0ms | - [📜](./07/serial.log) - |
| 8 | When I type "attr_list /hosts/thingos.local" on the serial console | ✅ | 3829ms | - [📜](./08/serial.log) - |
| 9 | And I wait for 1 seconds | ✅ | 1002ms | - [📜](./09/serial.log) - |
| 10 | Then the serial output should contain "net.ip.ipv4" | ✅ | 1ms | - - - |
| 11 | And the serial output should contain "net.hostname" | ✅ | 0ms | - - - |
| 12 | When I type "attr_get /hosts/thingos.local net.hostname" on the serial console | ✅ | 849ms | - [📜](./12/serial.log) - |
| 13 | And I wait for 1 seconds | ✅ | 1002ms | - [📜](./13/serial.log) - |
| 14 | Then the serial output should contain "thingos.local" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26772470406] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26799397581] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26835303462] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26868160638] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26887939188] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26890514046] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26925768276] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26946458022] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26994465531] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26995474308] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[27059814078] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[27127731510] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[27149448909] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[27222966474] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[27325967922] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[27354191865] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[27384317697] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[27501075822] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[27551736861] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[27708466929] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[28006041294] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=989717322 elapsed_us=494858
[28008559194] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[28084454475] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[28185124836] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[28186081935] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[28278518268] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[28282179519] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 250ms so the prompt can take the foreground
[28289129451] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=20262
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat[29207201892] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mounting /hosts early...
 [29234233017] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned early /hosts mount (PID=9)
[29262402576] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[29268917667] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
/et[29396702643] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: starting (mount_point=/hosts)
[29426917212] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: mounted at /hosts
[29441413023] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: advertising hostname thingos.local
c[29445192183] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: serving cache immediately; mDNS socket will attach when netd is ready
/hostname
[29975654082] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
mounted type=mdns device=none target=/hosts
[30057796791] [[32mINFO [0m] [cat] [CPU3] cat: opening '/etc/hostname'
[30062601030] [[32mINFO [0m] [cat] [CPU3] cat: opened '/etc/hostname' fd=7
thingos
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hls /hosts
[30682745940] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[?25l[32014027815] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[32015030751] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing primary port (0x1F0)...
[32046807375] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing secondary port (0x170)...
[32079937923] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[32080771041] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Entering RPC service loop
[32081614950] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: No active devices to service
thingos.local  
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[32661894243] [[32mINFO [0m] [sprout::supervisor] [CPU3] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[34119347559] [[32mINFO [0m] [netd] [CPU0] NETD: Starting network service...
[34206223623] [[32mINFO [0m] [netd] [CPU0] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[34741973475] [[32mINFO [0m] [netd] [CPU0] NETD: Network ready
attr_list /hosts/thingos.local
[40612631763] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[?25l[51056038374] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=11 op=3 TIMEOUT (5s) - tainting provider
[51242157351] [[33mWARN [0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=11 op=Lookup rejected (provider is already tainted/stalled)
[51243939582] [[33mWARN [0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=11 op=Lookup rejected (provider is already tainted/stalled)
[51398917383] [[33mWARN [0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=11 op=Lookup rejected (provider is already tainted/stalled)
net.ip.ipv4	Utf8	7
net.hostname	Utf8	13
count=2
[51566967111] [[33mWARN [0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=11 op=Lookup rejected (provider is already tainted/stalled)
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[52262069805] [[33mWARN [0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=11 op=Lookup rejected (provider is already tainted/stalled)
[53320223253] [[33mWARN [0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=11 op=Lookup rejected (provider is already tainted/stalled)
[53323218927] [[33mWARN [0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=11 op=Lookup rejected (provider is already tainted/stalled)
[54801153132] [[33mWARN [0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=11 op=Lookup rejected (provider is already tainted/stalled)
attr[55152331542] [[33mWARN [0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=11 op=Lookup rejected (provider is already tainted/stalled)
_get /h[55485369027] [[33mWARN [0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=11 op=Lookup rejected (provider is already tainted/stalled)
[55486960320] [[33mWARN [0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=11 op=Lookup rejected (provider is already tainted/stalled)
osts/thing[56020300545] [[33mWARN [0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=11 op=Lookup rejected (provider is already tainted/stalled)
os.local[56481603915] [[33mWARN [0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=11 op=Lookup rejected (provider is already tainted/stalled)
 net.[56712038097] [[33mWARN [0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=11 op=Lookup rejected (provider is already tainted/stalled)
hos[56869802748] [[33mWARN [0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=11 op=Lookup rejected (provider is already tainted/stalled)
tname
[57179588763] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_get'
[?25l[57218662380] [[33mWARN [0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=11 op=Lookup rejected (provider is already tainted/stalled)
[57382577241] [[33mWARN [0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=11 op=Lookup rejected (provider is already tainted/stalled)
[57541312257] [[33mWARN [0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=11 op=Lookup rejected (provider is already tainted/stalled)
[57544073862] [[33mWARN [0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=11 op=Lookup rejected (provider is already tainted/stalled)
type=Utf8 len=13
value=thingos.local
[57706042218] [[33mWARN [0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=11 op=Lookup rejected (provider is already tainted/stalled)
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[57970946847] [[33mWARN [0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=11 op=Lookup rejected (provider is already tainted/stalled)
[58197883062] [[33mWARN [0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=11 op=Lookup rejected (provider is already tainted/stalled)
[58527610647] [[33mWARN [0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=11 op=Lookup rejected (provider is already tainted/stalled)
[58727476005] [[33mWARN [0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=11 op=Lookup rejected (provider is already tainted/stalled)
[59916958464] [
```
</details>
