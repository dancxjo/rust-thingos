# ✅ Scenario: Final boot milestone is reported before entering the scheduler loop

> Last run: 2026-04-24 17:34:52

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 0ms | - - - |
| 2 | When I wait for the system to boot | ✅ | 18599ms | - - - |
| 3 | Then the serial log shows "Entering Scheduler" after "Spawning Sprout" | ✅ | 504ms | - [📜](./03/serial.log) - |
| 4 | And the serial log shows "Entering scheduler loop." after "Entering Scheduler" | ✅ | 494ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[57678757224] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[57726342036] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[57790869675] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[57854780874] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[57889197894] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[57894180069] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[57957497466] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[58005919752] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[58033541115] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[58034759772] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[58101013839] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[58103086503] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[58264299786] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[58435827318] [[32mINFO [0m] [kerne[59038794111l::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[58505860050] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[58688842938] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[58826724396] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[58932700398] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[59280888876] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[59458596285] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[59814526167] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[60580139334] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=2438977893 elapsed_us=1219488
[60592769061] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[60883828962] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[60887359797] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[61002692124] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[61145353104] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[61161444894] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[61162924383] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[61611827970] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[61658389848] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[61680606504] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[61704617172] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[61792386645] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=38478
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[62273387814] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[62275617063] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[62544502218] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[62861526717] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[62862955980] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[62880516963] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[62882283288] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[63016145346] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=5, bind_id=322371585)
[63158278458] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[63159778704] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[63218435049] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=5)
[63230941422] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Starting VFS-native VirtIO GPU driver...
[63232187964] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: boot_arg=5
[63253020204] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Mapping bootstrap memfd 5 size=4096...
[63257233380] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: vm_map success at 0x400000001000
[63267430644] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=10)
[63285950508] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Recovered handles: req_read=4, resp_write=5, svc=1, id=322371585
[63289840845] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting (drv_req_r=4, drv_resp_w=5, svc=1, id=322371585)
[63290536749] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[63297041808] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[63307618935] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[63362834040] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[63364257660] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[63380798250] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[63774258933] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[63777689877] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[63780149103] [[32mINFO [0m] [bristle] [CPU1] bristle: published pid 10 to /run/bristle/pid
[63795213438] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[63796576305] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[63797807667] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[63799014906] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[63800285835] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[63801560196] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[63854505297] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[63855901296] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[63875330937] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[63899848617] [[32mINFO [0m] [bristle] [CPU1] bristle: published device handles kbd_in=7 mouse_in=9
[
```
</details>
