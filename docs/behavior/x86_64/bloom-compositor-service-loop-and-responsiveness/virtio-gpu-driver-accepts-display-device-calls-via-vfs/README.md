# ❌ Scenario: virtio GPU driver accepts display device calls via VFS

> Last run: 2026-04-25 09:25:09

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 15559ms | - [📜](./01/serial.log) - |
| 2 | Then the serial output should contain "display_virtio_gpu: GPU initialized successfully" within 60s | ✅ | 624ms | - [📜](./02/serial.log) - |
| 3 | And the serial output should contain "bloom: output0" within 60s | ❌ | 61095ms | - [📜](./03/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[48461987607] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[48511902582] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[48586457898] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[48647293002] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[48694611537] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[48699343374] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[48762608928] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[48765559656] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[48801875199] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[48827841810] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[48829046805] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[48890375490] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[48891625233] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[49038526581] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[49156410798] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[49198198137] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[49328239653] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[49463001489] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[49516935270] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[49559404323] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[49846064499] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[49983542499] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[50262835194] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[50720321784] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1798522308 elapsed_us=899261
[50721565554] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[50935495578] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[50938812837] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[51024097212] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[51205505406] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[51227048004] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[51228968769] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[51425443014] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[51431744133] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[51435096570] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[51488820537] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[51764927148] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[51811315809] [[32mINFO [0m] [iso9660d] [CPU2] ISO9660D: Starting VFS provider
[52388232171] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=5, bind_id=322371585)
[52457965065] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[52470683892] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=5)
[52492933350] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=10)
[52516165317] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[52521015426] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready yet
[52619110632] [[32mINFO [0m] [bristle] [CPU1] bristle: published pid 10 to /run/bristle/pid
[52643519973] [[32mINFO [0m] [bristle] [CPU1] bristle: published device handles kbd_in=7 mouse_in=9
[52652018694] [[32mINFO [0m] [bristle] [CPU1] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[52680314478] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[52712688963] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[52725264504] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[52882380408] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[52883861316] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready yet
[53228221068] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[53229627429] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready yet
[53563407645] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[53565047745] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready yet
[53827149255] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[53855897271] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[54034617318] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=11)
[54057679764] [[32mINFO [0m] [bloom] [CPU2] bloom: ENTERING MAIN
[54059431833] [[32mINFO [0m] [bloom] [CPU2] bloom: compositor service starting
[54061247691] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[54091778466] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[
```
</details>
