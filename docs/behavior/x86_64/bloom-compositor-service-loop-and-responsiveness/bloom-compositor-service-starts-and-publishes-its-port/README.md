# ❌ Scenario: bloom compositor service starts and publishes its port

> Last run: 2026-04-24 17:40:56

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 17393ms | - [📜](./01/serial.log) - |
| 2 | Then the log should match pattern "bloom: compositor service starting" | ❌ | 1006ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[53662986168] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[53776898868] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[53901498882] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[54062925741] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[54122963037] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[54129744867] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[54249744285] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[54285763752] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[54342313641] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[54343561338] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[54525898383] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[54527867097] [[32mINFO [0m] [kernel::boot[55258243689] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[54772167417] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[54874974957] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[54916799949] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[55021765095] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[55164334995] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[55311010854] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[55511313165] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[55701106626] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[55973009301] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[56484872565] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1903304370 elapsed_us=951652
[56492681817] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[56641235871] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[56652944964] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[56754294894] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[56884598727] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[56910179106] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[56919165864] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[57164861193] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[57174198840] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[57187737882] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[57215311131] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[57254788635] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=35178
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
[57545336838] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[57547452699] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[57813803190] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[58164619458] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[58169025585] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[58186786977] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[58193188119] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[58297393209] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=5, bind_id=322371585)
[58426569630] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[58431449208] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[58464330639] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=5)
[58468048188] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Starting VFS-native VirtIO GPU driver...
[58469554869] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: boot_arg=5
[58471314000] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Mapping bootstrap memfd 5 size=4096...
[58486653225] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: vm_map success at 0x400000001000
[58488671274] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Recovered handles: req_read=4, resp_write=5, svc=1, id=322371585
[58490282466] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting (drv_req_r=4, drv_resp_w=5, svc=1, id=322371585)
[58552332960] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=10)
[58577975676] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[58579488759] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[58580816844] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[58600405215] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[58601733333] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[58626626322] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[59001104316] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[59002503384] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[59019282861] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[59020608009] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[59021958963] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[59023186332] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[59036663631] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[59040927726] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[59072724810] [[32mINFO [0m] [bristle] [CPU1] bristle: published pid 10 to /run/bristle/pid
[59083017444] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[59093376936] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[59097140883] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[59123711130] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[59150877324] [[32mINFO [0m] [bristle] [CPU1] bristle: published device handles kbd_in=7 mouse_in=9
[59178171162] [[32mINFO [0m] [bristle] [CPU1] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[59198202426] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[59229726666] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[59231393430] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Virgl 3D not supported, using 2D only
[59285422680] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=1
[59410372395] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[59411695101] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[59415570324] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[59444539473] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[59445922206] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[59447129049] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[59448314937] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[59449565010] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[59481375987] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[59482781061] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[59488014828] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[
```
</details>
