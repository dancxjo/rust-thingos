# ❌ Scenario: /proc/sched/stat contains per-CPU counters

> Last run: 2026-04-25 20:10:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 0ms | - - - |
| 2 | When I wait for the system to boot | ✅ | 17987ms | - [📜](./02/serial.log) - |
| 3 | And the shell command "cat /proc/sched/stat" succeeds | ✅ | 383ms | - [📜](./03/serial.log) - |
| 4 | Then the output contains "context_switches:" | ❌ | 1014ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[55495651200] [[32mINFO [0m] [kernel::boot_progress] [CPU0] bo[[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: mot_progress: milestone="Framebuffer Initialized"
[56075640225] ilestone="Memory Map OK"
[56122594209] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[56207988606] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[56281045029] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[56326202427] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[56332804506] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[56419996083] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[56423848041] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[56473095030] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[56505049755] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[56506642698] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[56585035782] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[56587380201] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[56791035741] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[56957708511] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[57005249268] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[57122717190] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[57280008291] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
ot_progress: milestone="Boot Info OK"
[57370937877] [[32mINFO [0m] [kernel::boot_progress] [CPU0] bo[57428405430] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[57653551395] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[57760790769] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[58130607183] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[58746811860] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=2110269678 elapsed_us=1055134
[58748198883] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
c[59002098078] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
at [59139456024] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
/p[59243076717] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
ro[59365446360] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[59390470128] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[59392364658] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
c/[59650115514] [[32mINFO [0m] [kernel::syscall::handlers::proceess] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
sch[59682545043] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[59699317293] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[59710406283] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
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
d/s[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m t[?25hcat /proc/sched/st[59962653267] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
a[59964471930] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
t
[60051908037] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[60063240138] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[60068680353] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[60085565727] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/cat' with argv=["/bin/cat", "/proc/sched/stat"]

[60124508961] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[60127793715] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[60128975808] [[32mINFO [0m] [iso9660d] [CPU2] ISO9660D: Starting VFS provider
[60129805230] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[60143753076] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[60205850727] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/cat' TID=9 PID=9
[60224008779] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25l[60256555623] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[60322519125] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[60334170006] [[32mINFO [0m] [cat] [CPU3] cat: opening '/proc/sched/stat'
[60374900388] [[32mINFO [0m] [cat] [CPU3] cat: opened '/proc/sched/stat' fd=6
[60458001780] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=11)
online_cpus: 4

cpu: 0
runnable_count: 1
context_switches: 5
wakeups: 1
steals_in: 0
steals_out: 0
timer_interrupts: 23
idle_total_us: 0
idle_episodes: 0
dispatch_count: 5
resched_ipi_received: 0
mailbox_pushes: 0
mailbox_tasks_drained: 0

cpu: 1
runnable_count: 0
context_switches: 1
wakeups: 1
steals_in: 0
steals_out: 0
timer_interrupts: 68
idle_total_us: 0
idle_episodes: 0
dispatch_count: 1
resched_ipi_received: 0
mailbox_pushes: 1
mailbox_tasks_drained: 1

cpu: 2
runnable_count: 0
context_switches: 2
wakeups: 1
steals_in: 0
steals_out: 0
timer_interrupts: 70
idle_total_us: 0
idle_episodes: 1
dispatch_count: 2
resched_ipi_received: 1
mailbox_pushes: 1
mailbox_tasks_drained: 1

cpu: 3
runnable_count: 0
context_switches: 2
wakeups: 2
steals_in: 0
steals_out: 0
timer_interrupts: 61
idle_total_us: 0
idle_episodes: 0
dispatch_count: 2
resched_ipi_received: 1
mailbox_pushes: 1
mailbox_tasks_drained: 1

: 1

[60669707967] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[60843867051] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 11 to /run/bristle/pid
[60878241732] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=7 mouse_in=9
[60891308082] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[61028446710] [[32mINFO [0m] [virtio_gpu] [CPU1] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[61051691613] [[32mINFO [0m] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[61082391645] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[62315554455] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[
```
</details>
