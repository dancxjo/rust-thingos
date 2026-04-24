# ❌ Scenario: bloom service loop starts

> Last run: 2026-04-24 11:14:27

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8206ms | - - - |
| 2 | Then the log should match pattern "bloom: service loop started" | ❌ | 1008ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[25383941979] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[25409259315] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[25442349999] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[25474150086] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[25493580519] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[25496108814] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[25530628497] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[25550440344] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[25599712776] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[25600824249] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[25683687810] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[25751550726] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[25773171501] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[25844716854] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[25943680950] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[25973848428] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[25996404192] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[26115632235] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[26163427224] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[26323043802] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[26624957766] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1003965633 elapsed_us=501982
[26625728283] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[26700234726] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[26702058636] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[26741761728] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[26780716215] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[26792137284] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[26793404121] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[26878845675] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[26884045914] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[26887702809] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[26888100624] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[26895422367] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=20526
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[27111013083] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[27113115183] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[27126377652] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[27129154635] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[27131046360] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=5 specs_ptr=10002f8 count=2 timeout=24 ticks
[27132469617] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=3 flags=1 token=1
[27133263531] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=4 flags=1 token=2
[27140297877] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[27140963916] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[27141767070] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[27167791233] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet ��� retrying
[27480388320] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[27488756262] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[27489476355] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[27492275481] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[27492933138] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[27495803379] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[27496659069] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[27520151901] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=5, bind_id=322371585)
[27547090032] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[27547973178] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[27555018084] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=5)
[27556078902] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Starting VFS-native VirtIO GPU driver...
[27557035176] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: boot_arg=5
[27558063687] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Mapping bootstrap memfd 5 size=4096...
[27560262576] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: vm_map success at 0x400000001000
[27561350520] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Recovered handles: req_read=4, resp_write=5, svc=1, id=322371585
[27562217166] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting (drv_req_r=4, drv_resp_w=5, svc=1, id=322371585)
[27562558551] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=10)
[27565115754] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[27565862346] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[27566584782] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[27569133207] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[27569849241] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[27574261506] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[27575159172] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[27575841348] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[27576543489] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=5 specs_ptr=10002f8 count=2 timeout=38 ticks
[27577269522] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=3 flags=1 token=1
[27577921998] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=4 flags=1 token=2
[27579250644] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[27580949220] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[27601559007] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[27664882806] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[27678029775] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[27683428212] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[27684185397] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Virgl 3D not supported, using 2D only
[27690632772] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=1
[27703867950] [[32mINFO [0m] [bristle] [CPU1] bristle: published pid 10 to /run/bristle/pid
[27715058877] [[32mINFO [0m] [bristle] [CPU1] bristle: published device handles kbd_in=7 mouse_in=9
[27720066165] [[32mINFO [0m] [bristle] [CPU1] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[27721419759] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=6 token=1
[27722189946] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=7 token=2
[27722934558] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=8 token=3
[27723649800] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] sys_wait_many: tid=10 specs_ptr=1000000 count=3 timeout=0 ticks
[27724422726] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1]   spec[0]: kind=7 object=6 flags=1 token=1
[27725078403] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1]   spec[1]: kind=7 object=7 flags=1 token=2
[27725777013] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1]   spec[2]: kind=7 object=8 flags=1 token=3
[27726842649] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=6 token=1
[27727525782] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=7 token=2
[27728183307] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=8 token=3
[27728863338] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] sys_wait_many: blocking...
[27909604932] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[27910902393] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[27911659116] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[27913157316] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[27914107716] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[27916223544] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[27916928721] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[27917637957] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[27918303369] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[27918923208] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[27919602381] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[27921190572] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[27921869811] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vinel
[27923995803] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[2::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=4 flags=1 token=2
[27928586301] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[27929249931] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_rea7924822090] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[27925505817] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[27926165025] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPdy: polling spec[0] kind=7 object=4 token=2
[27929952105] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
U0] sys_wait_many: tid=5 specs_ptr=10002f8 count=2 timeout=48 ticks
[27926923629] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=3 flags=1 token=1
[27927587160] [[32mINFO [0m] [kerne[28198905042] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: frame pool ready (1 buffer)
[28213221828] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_BIND_READY (ID: 322371585)
[28214156322] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Waiting for BIND_ASSIGNED...
[28216368510] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[28217579841] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[28218300033] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[28219818990] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: returning 1 results
[28227017115] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: BIND_READY from instance_id=0x13370001
[28227807894] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Registering DISPLAY_CARD at /dev/display/card0 (handle=7)
[28260808455] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[28261935735] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Handshake complete — task 'display' marked ready
[28266025392] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Sent MSG_BIND_ASSIGNED to req_port 3 (res=Ok(92))
[28266784524] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[28267424262] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[28269402876] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[28270100331] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[28270903749] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[28271585133] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[28272302025] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[28273583415] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[28312339440] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[28314070059] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_SERVICE_READY.
[28316659701] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[28317566607] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[28318285116] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[28318969866] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=51 ticks
[28319651745] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[28320297687] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[28321426221] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[28322121102] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[28322778231] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
[28406215299] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: unblocked
[28407790488] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[28410262848] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[28419675273] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[28420411932] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[28421076354] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[28421705466] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=54 ticks
[28422377379] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[28423043088] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[28424088396] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[28424726979] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[28425353913] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
[28442394123] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: unblocked
[28443511701] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[28444283472] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[28454512680] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[28455835584] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[28457196339] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[28457990187] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=55 ticks
[28458666423] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[28459319625] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[28460338170] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[28461009225] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[28469472438] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[28470603447] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[28473155172] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[28474625619] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=56 ticks
[28476772335] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[28479202818] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[28480270500] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[28480933272] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[28481577729] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
[29325186891] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: unblocked
[29327465112] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[29328181608] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[29334944100] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[29335605750] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[29336286276] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[29336918985] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=82 ticks
[29337593373] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[29338241757] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[29339371578] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[29340041775] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[29340701016] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
[29768486880] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=11 PID=11
[29787441717] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: unblocked
[29788443234] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[29790455013] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[29798906412] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[29799596937] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[29800257729] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[29800925253] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=96 ticks
[29801599839] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[29802247959] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[29803215420] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[29803882911] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[29804523837] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
[29812775718] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=12 PID=12
[29855122902] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=13 PID=13
[29861127153] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[29862052506] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing primary port (0x1F0)...
[29863915092] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive(base=1f0, slave=false) - selecting drive a0
[29872375401] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive - initial status: ff
[29880363711] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive(base=1f0, slave=true) - selecting drive b0
[29891148045] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive - initial status: ff
[29901016266] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing secondary port (0x170)...
[29901425862] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=5 token=1
[29901954786] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive(base=170, slave=false) - selecting drive a0
[29902278384] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=1 object=6 token=2
[29903375337] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=6 token=3
[29904040782] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=12 specs_ptr=1010228 count=3 timeout=0 ticks
[29904726357] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=5 flags=1 token=1
[29905380087] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=1 object=6 flags=1 token=2
[29906050317] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[2]: kind=7 object=6 flags=1 token=3
[29907255873] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=5 token=1
[29907918810] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=1 object=6 token=2
[29908394373] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive - initial status: ff
[29908816146] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=6 token=3
[29909492844] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
[29910839508] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: unblocked
[29911734105] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[29912431593] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[29915748852] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive(base=170, slave=true) - selecting drive b0
[29920146135] [[32mINFO [0m] [display_virtio_gp
```
</details>
