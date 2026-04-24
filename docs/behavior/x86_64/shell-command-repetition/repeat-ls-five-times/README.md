# ✅ Scenario: Repeat ls five times

> Last run: 2026-04-23 19:11:44

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8316ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 252ms | - [📜](./02/serial.log) - |
| 3 | And I type "ls" on the serial console | ✅ | 149ms | - [📜](./03/serial.log) - |
| 4 | And I type "ls" on the serial console | ✅ | 201ms | - [📜](./04/serial.log) - |
| 5 | And I type "ls" on the serial console | ✅ | 99ms | - [📜](./05/serial.log) - |
| 6 | And I type "ls" on the serial console | ✅ | 98ms | - [📜](./06/serial.log) - |
| 7 | And I type "ls" on the serial console | ✅ | 100ms | - [📜](./07/serial.log) - |
| 8 | Then the command output should contain "bin" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26019109578] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26045593266] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26075758005] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26108852088] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26129562822] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26132171076] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26168165430] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26189195934] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26236474407] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26237485593] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26321673972] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26388342354] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26410350879] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26489287803] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26594717424] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26621093664] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26645104926] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[26740545183] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[26789789499] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[26919359511] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27163383984] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=904362393 elapsed_us=452181
[27165159648] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27237917817] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27339985299] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27343137393] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27453348054] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[27457565421] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27460671447] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
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
[27478673838] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=32670
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[27709252923] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[27710452704] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[27752087352] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet �� retrying
ls
[28180657626] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[28186794372] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/ls' with argv=[[47, 98, 105, 110, 47, 108, 115]]
[28224490008] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/ls' TID=9 PID=9
[34mbin[0m  [34mboot[0m  [34mdata[0m  [34mdev[0m  [34mdrivers[0m  [34metc[0m  [34mlib[0m  [34mmnt[0m  motd  [34mproc[0m  [34mrun[0m  [34mservices[0m  [34msession[0m  [34mshare[0m  [34msys[0m  [34mtmp[0m  version  
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hls
[28667758779] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[28670548467] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/ls' with argv=[[47, 98, 105, 110, 47, 108, 115]]
[28698890418] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/ls' TID=10 PID=10
[?25l[34mbin[0m  [34mboot[0m  [34mdata[0m  [34mdev[0m  [34mdrivers[0m  [34metc[0m  [34mlib[0m  [34mmnt[0m  motd  [34mproc[0m  [34mrun[0m  [34mservices[0m  [34msession[0m  [34mshare[0m  [34msys[0m  [34mtmp[0m  version  
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hls
[29334399336] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[29337045111] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/ls' with argv=[[47, 98, 105, 110, 47, 108, 115]]
[29364216651] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/ls' TID=11 PID=11
[?25l[34mbin[0m  [34mboot[0m  [34mdata[0m  [34mdev[0m  [34mdrivers[0m  [34metc[0m  [34mlib[0m  [34mmnt[0m  motd  [34mproc[0m  [34mrun[0m  [34mservices[0m  [34msession[0m  [34mshare[0m  [34msys[0m  [34mtmp[0m  version  
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hls
[29664072075] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[29666753094] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/ls' with argv=[[47, 98, 105, 110, 47, 108, 115]]
[29694775176] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/ls' TID=12 PID=12
[34mbin[0m  [34mboot[0m  [34mdata[0m  [34mdev[0m  [34mdrivers[0m  [34metc[0m  [34mlib[0m  [34mmnt[0m  motd  [34mproc[0m  [34mrun[0m  [34mservices[0m  [34msession[0m  [34mshare[0m  [34msys[0m  [34mtmp[0m  version  
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hls
[29990915295] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[29994624462] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/ls' with argv=[[47, 98, 105, 110, 47, 108, 115]]
[30022688586] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/ls' TID=13 PID=13
[?25l[34mbin[0m  [34mboot[
```
</details>
