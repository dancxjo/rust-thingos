# ✅ Scenario: Launch ps and verify reaping

> Last run: 2026-04-23 11:14:17

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 14015ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 359ms | - [📜](./02/serial.log) - |
| 3 | And I type "ps" on the serial console | ✅ | 306ms | - [📜](./03/serial.log) - |
| 4 | Then the command output should contain "PID" | ✅ | 2ms | - - - |
| 5 | And the command output should contain "sh" | ✅ | 0ms | - - - |
| 6 | And the command output should contain "ps" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[43769269467] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[43813224015] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[43871982363] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[43928750217] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[43963031904] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[43967761266] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[44030013027] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[44066387112] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[44148568101] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[44150401416] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[44298262932] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[4[4386864962] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone=44642518734] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
"PCI Bus Scanned"
[44417392008] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[44514950667] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[44714572155] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[44754984648] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[44952469947] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[45077352474] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[45354275901] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[45790988892] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1606253286 elapsed_us=803126
[45792242958] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[45940442592] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[46061179626] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[46066335084] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[46233148929] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[46236923535] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[46328575227] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=51579
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[46652234409] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mounting /hosts early...
[46743325101] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned early /hosts mount (PID=9)
[46745576130] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[47004183402] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: starting (mount_point=/hosts)
[47037389256] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: mounted at /hosts
[47054922882] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: advertising hostname thingos.local
[47058477543] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: serving cache immediately; mDNS socket will attach when netd is ready
ps
[47286793554] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[47336284908] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ps'
  PID  PPID STAT COMMAND
    5     0 S    
    6     5 R    /bin/sh
    7     5 R    /bin/cambium
    8     5 S    /bin/iso9660d
mounted type=mdns device=none target=/hosts
    9     5 R    /bin/mount -t mdns none /hosts
   11     1 R    /bin/mdns none /hosts
   12     6 R    /bin/ps
[?25l[?25h[1;95mTHING[0m
```
</details>
