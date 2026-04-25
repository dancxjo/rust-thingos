# ❌ Scenario: /proc/sched/stat contains per-CPU counters

> Last run: 2026-04-25 13:10:40

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 0ms | - - - |
| 2 | When I wait for the system to boot | ✅ | 17587ms | - [📜](./02/serial.log) - |
| 3 | And the shell command "cat /proc/sched/stat" succeeds | ✅ | 363ms | - [📜](./03/serial.log) - |
| 4 | Then the output contains "context_switches:" | ❌ | 1011ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[54071910123] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[54630298338] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[54681119757] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[54759864819] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[54840862395] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[54892808751] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[54898627575] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[54983589276] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[54987285474] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[55040948160] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[55074601626] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[55076142000] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[55158751065] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[55161308961] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[55359024996] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[55496338788] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[55552804461] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[55680434502] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[55873171299] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[56027176953] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[56110683981] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[56342928972] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[56458518039] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[56754684723] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[57309518013] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=2093247651 elapsed_us=1046623
[57311160687] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
MAIN (arg0=6291456)
[57516517212] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING [57532069089] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[57620347950] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
ca[57738764622] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
t[57768014832] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[57770047302] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
 /[58073187315] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
proc[58090115985] [[32mINFO [0m] [sprout::pipelines] [CPU0] S0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
PROUT: Spawned serial shell '/bin/sh' (PID=6)
[58096118817] [[32mINFO [0m] [sprout::supervisor] [CPU[58112330859] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
/[1;32m
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
sc[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat[58300181445] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
 [58302350106] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
/proc/sche[58380409098] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[58396238505] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
d[58425622695] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[58448194167] [[32mINFO [0m] [iso9660d] [CPU2] ISO9660D: Starting VFS provider
/[58472494146] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
stat
[58754857524] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[58784020548] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/cat' with argv=["/bin/cat", "/proc/sched/stat"]

[58875674418] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/cat' TID=9 PID=9
[58971333663] [[32mINFO [0m] [cat] [CPU3] cat: opening '/proc/sched/stat'
[58979344875] [[32mINFO [0m] [cat] [CPU3] cat: opened '/proc/sched/stat' fd=6
online_cpus: 4

cpu: 0
runnable_count: 0
context_switches: 6
wakeups: 1
steals_in: 0
steals_out: 0
timer_interrupts: 20
idle_total_us: 0
idle_episodes: 0
dispatch_count: 6
resched_ipi_received: 0
mailbox_pushes: 0
mailbox_tasks_drained: 0

cpu: 1
runnable_count: 0
context_switches: 1
wakeups: 1
steals_in: 0
steals_out: 0
timer_interrupts: 71
idle_total_us: 0
idle_episodes: 0
dispatch_count: 1
resched_ipi_received: 1
mailbox_pushes: 1
mailbox_tasks_drained: 1

cpu: 2
runnable_count: 0
context_switches: 2
wakeups: 1
steals_in: 0
steals_out: 0
timer_interrupts: 68
idle_total_us: 0
idle_episodes: 1
dispatch_count: 2
resched_ipi_received: 0
mailbox_pushes: 1
mailbox_tasks_drained: 1

cpu: 3
runnable_count: 1
context_switches: 2
wakeups: 2
steals_in: 0
steals_out: 0
timer_interrupts: 69
idle_total_us: 0
idle_episodes: 0
dispatch_count: 2
resched_ipi_received: 1
mailbox_pushes: 1
mailbox_tasks_drained: 1

[59071067484] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m]S[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[59154935697] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[0m [1;93m/[0m [1;96m>[0m [?25h
[1;95mTHING[0m[1;96m-O[59505656397] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[59827548594] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[60188493321] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[60545027301] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[60876682680] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[61214266905] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[61551877992] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[61880806482] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'

```
</details>
