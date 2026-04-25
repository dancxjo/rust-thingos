# ❌ Scenario: iso9660d mounts the boot filesystem at /media/cdrom

> Last run: 2026-04-25 09:08:47

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 16691ms | - - - |
| 2 | Then the log should match pattern "iso9660d: found ISO9660 on device (atapi|ata_|ahci)" | ❌ | 1023ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[51550015110] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[51598495872] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[51673188435] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[51775319211] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[51837468507] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[51842198892] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[51958604874] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[51962092479] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[52022629692] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[52099123329] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[52100413761] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[52200574668] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[52202633307] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[52442331810] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[52610150670] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[52660865268] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[52796959281] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[52959884373] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[53008416945] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[53050416573] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[53324383398] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[53460896346] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[53733397806] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[54336507819] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=2080857900 elapsed_us=1040428
[54337836333] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[54613276575] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[54627016422] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[54734448747] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[54910027821] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[54949266834] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[54951104505] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[55282224756] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[55341345444] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[55352817762] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[55376925417] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[55825225203] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[55853073771] [[32mINFO [0m] [iso9660d] [CPU2] ISO9660D: Starting VFS provider
[56369358210] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=5, bind_id=322371585)
[56464327062] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[56506151427] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=5)
[56550881607] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=10)
[56567774703] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[56576331702] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready yet
[56800348803] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[56817511938] [[32mINFO [0m] [bristle] [CPU1] bristle: published pid 10 to /run/bristle/pid
[56830892184] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[56842441788] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[56848900944] [[32mINFO [0m] [bristle] [CPU1] bristle: published device handles kbd_in=7 mouse_in=9
[56861183082] [[32mINFO [0m] [bristle] [CPU1] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[56969050776] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[56970580227] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready yet
[57344476431] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[57345993738] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready yet
[
```
</details>
