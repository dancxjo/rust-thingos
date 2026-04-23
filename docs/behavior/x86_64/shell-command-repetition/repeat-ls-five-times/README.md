# ✅ Scenario: Repeat ls five times

> Last run: 2026-04-22 21:33:14

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8404ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 252ms | - [📜](./02/serial.log) - |
| 3 | And I type "ls" on the serial console | ✅ | 253ms | - [📜](./03/serial.log) - |
| 4 | And I type "ls" on the serial console | ✅ | 870ms | - [📜](./04/serial.log) - |
| 5 | And I type "ls" on the serial console | ✅ | 5229ms | - [📜](./05/serial.log) - |
| 6 | And I type "ls" on the serial console | ✅ | 99ms | - [📜](./06/serial.log) - |
| 7 | And I type "ls" on the serial console | ✅ | 101ms | - [📜](./07/serial.log) - |
| 8 | Then the command output should contain "bin" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26130893811] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26156356710] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26187348759] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26218865805] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26238123747] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26240735598] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26275126845] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26294655915] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26340690486] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26341750050] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26422367037] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26489979516] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26511373713] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26582918901] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26682246294] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26709250359] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26733634719] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[26851250085] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[26904146940] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[27063471039] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27370482051] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1008766143 elapsed_us=504383
[27371306655] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27446027466] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27518847378] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27519906744] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27609838047] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27612015552] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27621289839] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=20130
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[27826470045] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mounting /hosts early...
[27851986635] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned early /hosts mount (PID=9)
[27860028306] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[27887769393] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[28001497194] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: starting (mount_point=/hosts)
[28018072863] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: mounted at /hosts
[28025807766] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: advertising hostname thingos.local
[28028015763] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: serving cache immediately; mDNS socket will attach when netd is ready
ls
[28471551108] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
mounted type=mdns device=none target=/hosts
[?25l[34mbin[0m  [34mboot[0m  [34mdata[0m  [34mdev[0m  [34mdrivers[0m  [34metc[0m  [34mhosts[0m  [34mlib[0m  [34mmnt[0m  motd  [34mproc[0m  [34mrun[0m  [34mservices[0m  [34msession[0m  [34mshare[0m  [34msys[0m  [34mtmp[0m  version  
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hls
[29301481275] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[?25l[30508015098] [[32mINFO [0m] [sprout::supervisor] [CPU3] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[34mbin[0m  [34mboot[0m  [34mdata[0m  [34mdev[0m  [34mdrivers[0m  [34metc[0m  [34mhosts[0m  [34mlib[0m  [34mmnt[0m  motd  [34mproc[0m  [34mrun[0m  [34mservices[0m  [34msession[0m  [34mshare[0m  [34msys[0m  [34mtmp[0m  version  
[31730774394] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[31731777792] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Probing primary port (0x1F0)...
[31759068297] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Probing secondary port (0x170)...
[31783366098] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[31784209248] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Entering RPC service loop
[31785018705] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: No active devices to service
[31896770268] [[32mINFO [0m] [netd] [CPU3] NETD: Starting network service...
[31937633673] [[32mINFO [0m] [netd] [CPU3] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hls
[32168421780] [[32mINFO [0m] [sh] [CPU1] sh: spawning job cmd='ls'
[?25l[32676772524] [[32mINFO [0m] [netd] [CPU3] NETD: Network ready
[49075269375] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU1] VFS RPC: tid=11 op=Lookup resp_port=3 TIMEOUT (5s) - tainting provider
[49076125890] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU2] VFS RPC: tid=19 op=Stat resp_port=5 TIMEOUT (5s) - tainting provider
[34mbin[0m  [34mboot[0m  [34mdata[0m  [34mdev[0m  [34mdrivers[0m  [34metc[0m  [34mlib[0m  [34mmnt[0m  motd  net  [34mproc[0m  [34mrun[0m  [34mservices[0m  [34msession[0m  [34mshare[0m  [34msys[0m  [34mtmp[0m  version  
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hls
[49396782798] [[32mINFO [0m] [sh] [CPU1] sh: spawning job cmd='ls'
[?25l[34mbin[0m  [34mboot[0m  [34mdata[0m  [34mdev[0m  [34mdrivers[0m  [34metc[0m  hosts  [34mlib[0m  [34mmnt[0m  motd  net  [34mproc[0m  [34mrun[0m  [34mservices[0m  [34msession[0m  [34mshare[0m  [34msys[0m  [34mtmp[0m  version  
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hls
[49725406665] [[32mINFO [0m] [sh
```
</details>
