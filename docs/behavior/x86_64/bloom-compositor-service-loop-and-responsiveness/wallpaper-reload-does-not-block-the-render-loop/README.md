# ❌ Scenario: wallpaper reload does not block the render loop

> Last run: 2026-04-25 09:25:09

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 13627ms | - [📜](./01/serial.log) - |
| 2 | Then the serial output should contain "bloom: service loop started" within 60s | ❌ | 61043ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[42377501430] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[42424960644] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[42490844055] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[42547791825] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[42579832152] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[42583632201] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[42644049393] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[42647177958] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[42680521884] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[42707714808] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[42708971811] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[42771946866] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[42774024711] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[42918481650] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[43034296074] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[43075865844] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[43180903854] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[43314302196] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[43404109353] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[43448180952] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[43604007018] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[43684301430] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[43880255133] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[44258480409] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1446436266 elapsed_us=723218
[44260283694] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[44373264771] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[44472943515] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[44551758537] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[44745687591] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[44768368623] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[44770113762] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[44937837318] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[44944412634] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[44948313564] [[32mINFO [0m] [sprout::
supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground[44960754135] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[45286538913] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[45309825132] [[32mINFO [0m] [iso9660d] [CPU2] ISO9660D: Starting VFS provider
[45813353388] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=5, bind_id=322371585)
[45879615573] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[45896367429] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=5)
[45924616914] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=10)
[45947952501] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[45949518021] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready yet
[46140809913] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[46156020537] [[32mINFO [0m] [bristle] [CPU1] bristle: published pid 10 to /run/bristle/pid
[46158189825] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[46169475165] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[46180276659] [[32mINFO [0m] [bristle] [CPU1] bristle: published device handles kbd_in=7 mouse_in=9
[46189916289] [[32mINFO [0m] [bristle] [CPU1] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[46302296898] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[46303780479] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready yet
[46655938263] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[46657552128] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready yet
[47093499090] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[47094982242] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready yet
[47190242649] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[47209538046] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[47229932673] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[47237201187] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[47318469165] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=11)
[47340551973] [[32mINFO [0m] [bloom] [CPU2] bloom: ENTERING MAIN
[47342290215] [[32mINFO [0m] [bloom] [CPU2] bloom: compositor service starting
[50235318606] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=12 PID=12
[50328758007] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=13 PID=13
[50339525148] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Starting AHCI VFS driver
[50412484551] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[
```
</details>
