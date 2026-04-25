# ❌ Scenario: bloom compositor service starts and publishes its port

> Last run: 2026-04-24 18:32:22

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 22587ms | - [📜](./01/serial.log) - |
| 2 | Then the log should match pattern "bloom: compositor service starting" | ❌ | 1032ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[70874541210] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[70975721223] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[71099535276] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[71197950648] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[71256703650] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[71262037044] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[71382766653] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[71385613728] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[71451093087] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[71491791756] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[71493117003] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[71602045152] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[71603982483] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[71796691296] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[71927964900] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[71963458479] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[72061022814] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[72310865253] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[72412990287] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[72478876239] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[72735583536] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[72844493799] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[73110922962] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[73566023883] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1906283082 elapsed_us=953141
[73567371702] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[73757575848] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[73762153179] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[73869160233] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[74046447915] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[74073155508] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[74084101344] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[74288459883] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[74297290452] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[74301378921] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[74320969008] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[74729431920] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[75089208150] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet �� retrying
[75663490254] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=5, bind_id=322371585)
[75774285708] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[75809563005] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=5)
[75871294092] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=10)
[75886467723] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[75892512927] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[75912553695] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[76243931874] [[32mINFO [0m] [bristle] [CPU1] bristle: published pid 10 to /run/bristle/pid
[76268847798] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[76289525400] [[32mINFO [0m] [bristle] [CPU1] bristle: published device handles kbd_in=7 mouse_in=9
[76293848862] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[76295401479] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[76296866316] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[76300115034] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[76314056247] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[76323294267] [[32mINFO [0m] [bristle] [CPU1] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[76657620534] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[76679587677] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[76683428679] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[77025101835] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[77026442427] [[32mINFO [0m] [sprout::supervisor] [CPU0
```
</details>
