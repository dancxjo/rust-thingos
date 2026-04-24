# ✅ Scenario: grep filters piped input correctly

> Last run: 2026-04-24 11:46:14

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 10952ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 353ms | - [📜](./02/serial.log) - |
| 3 | And I type "echo hello | grep hello | wc -l" on the serial console | ✅ | 1273ms | - [📜](./03/serial.log) - |
| 4 | Then the command output should strictly be "1" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[33789270537] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[33822527376] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[33863246571] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[33907773405] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[33933291084] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[33936963720] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[33984704556] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[34011520917] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[34076072910] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[34077305658] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[34218698415] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[34322213574] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[34360646529] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[34470044433] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[34583715804] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[34656861492] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[34699688100] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[34887242544] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[34966924839] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[35188628013] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[35595417066] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1491082626 elapsed_us=745541
[35596791417] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[35733400890] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[35738322081] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[35801914731] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[35864793228] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[35888124294] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[35889926259] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[36077343126] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[36082740045] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[36085033611] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[36095956776] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[36103482129] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=41481
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[36387636549] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[36390006411] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[36415973682] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[36420955494] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[36424735545] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=5 specs_ptr=10002f8 count=2 timeout=30 ticks
[36427295421] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=3 flags=1 token=1
[36428732241] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=4 flags=1 token=2
[36440918481] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[36442418991] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[36444084831] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[36449316288] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet �� retrying
[36807174558] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[36822161904] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[36823698978] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[36833078172] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[36834409887] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[36839870331] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[36845019420] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[36877265403] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=5, bind_id=322371585)
[36923350431] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[36925129494] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[36931908882] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=5)
[36933813411] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Starting VFS-native VirtIO GPU driver...
[36935384409] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: boot_arg=5
[36937392690] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Mapping bootstrap memfd 5 size=4096...
[36941611707] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: vm_map success at 0x400000001000
[36943772943] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Recovered handles: req_read=4, resp_write=5, svc=1, id=322371585
[36945653844] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting (drv_req_r=4, drv_resp_w=5, svc=1, id=322371585)
[36958356204] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=10)
[36976480728] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[36978053640] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[36979514286] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[36984136035] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[36985556850] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[36994651320] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[37010830560] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[37013632161] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[37014949587] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=5 specs_ptr=10002f8 count=2 timeout=48 ticks
[37016307735] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=3 flags=1 token=1
[37017626712] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=4 flags=1 token=2
[37022043267] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[37023904137] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[37030075533] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[37123112961] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[37142338398] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[37154835168] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[37156785204] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Virgl 3D not supported, using 2D only
[37171033020] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=1
[37202063019] [[32mINFO [0m] [bristle] [CPU1] bristle: published pid 10 to /run/bristle/pid
[37223717289] [[32mINFO [0m] [bristle] [CPU1] bristle: published device handles kbd_in=7 mouse_in=9
[37234779153] [[32mINFO [0m] [bristle] [CPU1] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[37236872805] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=6 token=1
[37238197392] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=7 token=2
[37239393873] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=8 token=3
[37240706382] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] sys_wait_many: tid=10 specs_ptr=1000000 count=3 timeout=0 ticks
[37242026250] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1]   spec[0]: kind=7 object=6 flags=1 token=1
[37243253685] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1]   spec[1]: kind=7 object=7 flags=1 token=2
[37244651829] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1]   spec[2]: kind=7 object=8 flags=1 token=3
[37246481382] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=6 token=1
[37247736471] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=7 token=2
[37249001460] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=8 token=3
[37250235132] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] sys_wait_many: blocking...
[37332484992] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[37334763015] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[37336128390] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[37339667409] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[37341022488] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[37345814352] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[37347208503] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[37348684197] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[37350138771] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[37351497018] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[37353774975] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[37382254701] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[37384372905] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[37393577991] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[37398615243] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[37400144958] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[37401509079] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=5 specs_ptr=10002f8 count=2 timeout=60 ticks
[37402890393] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=3 flags=1 token=1
[37404270585] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=4 flags=1 token=2
[37406425386] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[37407783138] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[37409109441] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[37732352361] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[37734538809] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[37735824621] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[37738662555] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[37739846892] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[37744271796] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[37746717426] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[37751786457] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[37762723779] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[37764216798] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[37765821522] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[37769924907] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[37771200060] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[37775005323] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[37776742773] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[37778145207] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[37780249815] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=5 specs_ptr=10002f8 count=2 timeout=71 ticks
[37794420180] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=3 flags=1 token=1
[37795752192] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=4 flags=1 token=2
[37799501619] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[37803034434] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[37804379943] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[38006566917] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: frame pool ready (1 buffer)
[38030541021] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_BIND_READY (ID: 322371585)
[38032551975] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Waiting for BIND_ASSIGNED...
[38036014929] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[38037939324] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[38039541441] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
ech[38050176747] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: returning 1 results
o hello | grep hel[38096543859] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: BIND_READY from instance_id=0x13370001
[38098275468] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Registering DISPLAY_CARD at /dev/display/card0 (handle=7)
[38125834131] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[38128333254] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Handshake complete — task 'display' marked ready
[38136081984] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Sent MSG_BIND_ASSIGNED to req_port 3 (res=Ok(92))
[38137990374] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[38139340140] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
l[38150211429] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[38152908156] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[38153375370] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_SERVICE_READY.
[38153999631] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[38155311315] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[38156556834] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[38157867297] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[38158485552] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[38159211453] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[38160188220] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[38161620420] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[38162889435] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=74 ticks
[38164309788] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[38165623815] [[32mINFO o[38208195663[0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[38167766109] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[38169083997] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[38170356642] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: unblocked
[38211409170] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[38213747220] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[38229410604] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[38232327474] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[38233671201] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[38234939523] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=76 ticks
[38236257972] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[38242296840] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[38245847343] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[38250217962] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[38266732977] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[38268215370] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[38269344696] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[38270485242] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=77 ticks
[38271821907] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[38273214903] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[38275226484] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
 [38276598228] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[38277946608] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
|[38301760926] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: unblocked
[38306092275] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[38307524574] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[38320619964] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[38322462585] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[38323854624] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[38325205446] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=79 ticks
[38326625337] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[38327929134] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[38330500989] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[38331827787] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[38333153595] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
 w[38416998741] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: unblocked
[38419304583] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[38420639301] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[38435458380] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[38436911568] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[38438263380] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[38439542064] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=82 ticks
[38440901004] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[38442206880] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[38444849718] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[38446356729] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[38463062088] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
c[38465628003] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[38467958232] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[38469753135] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=83 ticks
[38471487912] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[38473412802] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[38476194240] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[38477493153] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[38478736296] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
 -l
[38700766896] [[32mINFO [0m] [sh] [CPU2] sh: spawning job cmd='echo'
[38708714616] [[32mINFO [0m] [sh] [CPU2] sh: spawning '/bin/echo' with argv=["/bin/echo", "hello"]
[38785354245] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU2] SYSCALL SPAWN_PROCESS_EX: name='/bin/echo' TID=11 PID=11
[38854530957] [[32mINFO [0m] [kernel::ipc::pipe] [CPU2] PIPE_ENQUEUE: tail=6 len=6 n=6
[38879388471] [[32mINFO [0m] [sh] [CPU2] sh: spawning job cmd='grep'
[38882935212] [[32mINFO [0m] [sh] [CPU2] sh: spawning '/bin/grep' with argv=["/bin/grep", "hello"]
[38934882756] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU2] SYSCALL SPAWN_PROCESS_EX: name='/bin/grep' TID=12 PID=12
[38994188310] [[32mINFO [0m] [sh] [CPU2] sh: spawning job cmd='wc'
[38998561206] [[32mINFO [0m] [sh] [CPU2] sh: spawning '/bin/wc' with argv=["/bin/wc", "-l"]
[39010377516] [[32mINFO [0m] [grep] [CPU3] grep: main started, args=["/bin/grep", "hello"]
[39013856013] [[32mINFO [0m] [grep] [CPU3] grep_fd: fd=0 pattern='hello' invert=false
[39043610298] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU2] SYSCALL SPAWN_PROCESS_EX: name='/bin/wc' TID=13 PID=13
[39061234872] [[32mINFO [0m] [sh] [CPU2] sh: cleaning up 2 pipes
[39062709708] [[32mINFO [0m] [sh] [CPU2] sh: closing pipe 0 ends: read=6 write=7
[39065395710] [[32mINFO [0m] [sh] [CPU2] sh: closing pipe 1 ends: read=8 write=9
[?25l[39105762762] [[32mINFO [0m] [kernel::ipc::pipe] [CPU3] PIPE_DEQUEUE: EOF reached (writers=0)
[39109464339] [[32mINFO [0m] [grep] [CPU3] grep: read_all(fd=0) reached EOF, total bytes=6
[39114107340] [[32mINFO [0m] [kernel::ipc::pipe] [CPU3] PIPE_ENQUEUE: tail=6 len=6 n=6
[39146135754] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: unblocked
[39148884456] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39150429780] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39162838572] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[39164154777] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39165479067] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39166934829] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=104 ticks
[39168308619] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[0]: kind=7 object=8 flags=1 token=1
[39169926741] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[1]: kind=7 object=a flags=1 token=2
[39172778766] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39174031314] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39186547785] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[39187883658] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39189220488] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39190559166] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=105 ticks
[39191918040] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[0]: kind=7 object=8 flags=1 token=1
[39193224708] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[1]: kind=7 object=a flags=1 token=2
[39195073797] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39196383996] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39197628756] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: blocking...
[39219894252] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: unblocked
[39224454918] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39228858933] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39239967129] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[39241004484] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39242439093] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39255211380] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[39262008819] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39263896881] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39265812729] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=107 ticks
[39268088145] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[0]: kind=7 object=8 flags=1 token=1
[39269863149] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[1]: kind=7 object=a flags=1 token=2
[39272145429] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39273501828] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39287364270] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[39288227913] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39289220619] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39290025192] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=108 ticks
[39290905566] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[0]: kind=7 object=8 flags=1 token=1
[39291727827] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[1]: kind=7 object=a flags=1 token=2
[39292958826] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39293785509] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39294755379] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: blocking...
[39350298867] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: unblocked
[39352204617] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39353504883] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39365713662] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[39367256247] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39368928093] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39371138829] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=111 ticks
[39372418437] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[0]: kind=7 object=8 flags=1 token=1
[39375221490] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[1]: kind=7 object=a flags=1 token=2
[39377669529] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39379294284] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39380945208] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: blocking...
[39417000315] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: unblocked
[39419093934] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39421687569] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39434809359] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[39436759989] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39438632112] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39440158626] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=113 ticks
[39441737412] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[0]: kind=7 object=8 flags=1 token=1
[39443024412] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[1]: kind=7 object=a flags=1 token=2
[39445322136] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39446631642] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39449908839] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: blocking...
[39482085456] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: unblocked
[39484201911] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39485715720] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39498033201] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[39499633239] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39503777247] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39508176939] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=115 ticks
[39512382987] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[0]: kind=7 object=8 flags=1 token=1
[39513633522] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[1]: kind=7 object=a flags=1 token=2
[39515683053] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39516923985] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39518484390] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: blocking...
[39548378331] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: unblocked
[39550498647] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39551966652] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39564804015] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[39566347821] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39567675114] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39569013396] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=117 ticks
[39570397317] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[0]: kind=7 object=8 flags=1 token=1
[39571780182] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[1]: kind=7 object=a flags=1 token=2
[39573885186] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39575252772] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39576586203] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: blocking...
[39617485149] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: unblocked
[39619402218] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39620794356] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39641521194] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[39643131825] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39644503470] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39645683418] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=119 ticks
[39647095257] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[0]: kind=7 object=8 flags=1 token=1
[39648078063] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[1]: kind=7 object=a flags=1 token=2
[39649907451] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39650792148] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39651655494] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: blocking...
[39678701238] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: unblocked
[39680888379] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39682286259] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39694917966] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[39696366699] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39701544960] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39703070748] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=121 ticks
[39704550963] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[0]: kind=7 object=8 flags=1 token=1
[39705813642] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[1]: kind=7 object=a flags=1 token=2
[39708079059] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39709729092] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39711101595] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: blocking...
[39778499310] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: unblocked
[39780476076] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39782288601] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39802978710] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[39805754967] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39809058234] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39810356883] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=124 ticks
[39811708200] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[0]: kind=7 object=8 flags=1 token=1
[39813016584] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[1]: kind=7 object=a flags=1 token=2
[39815525475] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39816805842] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39818083602] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: blocking...
[39848594973] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: unblocked
[39850579626] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39852003939] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39865719729] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[39867095829] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39868489980] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39869774109] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=126 ticks
[39871179051] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[0]: kind=7 object=8 flags=1 token=1
[39872486973] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[1]: kind=7 object=a flags=1 token=2
[39874572309] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39875860431] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39877142877] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: blocking...
[39908566764] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: unblocked
[39911122581] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39912514851] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39925980633] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[39927817809] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39930415173] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39932196216] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=128 ticks
[39933860769] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[0]: kind=7 object=8 flags=1 token=1
[39935135526] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[1]: kind=7 object=a flags=1 token=2
[39937715367] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39939012630] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39940289037] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: blocking...
[39975053514] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: unblocked
[39977044668] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39978491091] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39992463060] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[39993831702] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[39995135499] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[39996417120] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=129 ticks
[39997720290] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[0]: kind=7 object=8 flags=1 token=1
[39999104970] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[1]: kind=7 object=a flags=1 token=2
[40001891556] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[40006353519] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[40032570633] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[40034079789] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[40035422229] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[40036670454] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=131 ticks
[40037987352] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[0]: kind=7 object=8 flags=1 token=1
[40039307022] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[1]: kind=7 object=a flags=1 token=2
[40041239073] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[40042488585] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[40043788422] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: blocking...
[40075576365] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: unblocked
[40077429843] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[40078835049] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[40092303636] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[40093733361] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[40094624493] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[40096801437] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=133 ticks
[40098523080] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[0]: kind=7 object=8 flags=1 token=1
[40099568982] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[1]: kind=7 object=a flags=1 token=2
[40102369791] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[40103577888] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[40104790770] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: blocking...
[40139979726] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: unblocked
[40142132019] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[40143627546] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[40157318355] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[40163526315] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[40164897828] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[40166218092] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=135 ticks
[40167624354] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[0]: kind=7 object=8 flags=1 token=1
[40168937061] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[1]: kind=7 object=a flags=1 token=2
[40171174956] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[40172498916] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[40174222836] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: blocking...
[40205365497] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: unblocked
[40207324971] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[40208660613] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[40220499198] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[40221956643] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[40223340993] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[40224878397] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=137 ticks
[40227602283] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[0]: kind=7 object=8 flags=1 token=1
[40229312178] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[1]: kind=7 object=a flags=1 token=2
[40232041872] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[40236087441] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[40237471395] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: blocking...
[40271210232] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: unblocked
[40273318635] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[40274740308] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[40289243478] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[40290498864] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[40291766592] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[40293029931] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=139 ticks
[40294330758] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[0]: kind=7 object=8 flags=1 token=1
[40295920038] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[1]: kind=7 object=a flags=1 token=2
[40304479413] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[40305779250] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[40311739050] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: blocking...
[40335893532] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: unblocked
[40338048630] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[40339425291] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[40352583909] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[40353974562] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[40355547804] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[40367509710] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[40369196934] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[40370519244] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[40371868119] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=141 ticks
[40373369388] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[0]: kind=7 object=8 flags=1 token=1
[40374729615] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[1]: kind=7 object=a flags=1 token=2
[40376770797] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[40378084791] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[40379472903] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: blocking...
[40440495579] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: unblocked
[40442539533] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[40443870126] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[40455700395] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[40457264694] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[40458614130] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[40459821435] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=144 ticks
[40461097842] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[0]: kind=7 object=8 flags=1 token=1
[40462356363] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[1]: kind=7 object=a flags=1 token=2
[40464311415] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[40465555977] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[40466740380] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: blocking...
[40501096053] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: unblocked
[40503053316] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[40504497957] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[40515911964] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[40518095970] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[40521306969] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[40522613109] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=146 ticks
[40524912813] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[0]: kind=7 object=8 flags=1 token=1
[40526916045] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[1]: kind=7 object=a flags=1 token=2
[40529799387] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[40530673161] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[40531480506] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: blocking...
[40566791232] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: unblocked
[40568661639] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[40569978801] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[40573721265] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=14 PID=14
[40647027399] [[32mINFO [0m] [wc] [CPU1] wc: counting from stdin (no files)
[40662685404] [[32mINFO [0m] [wc] [CPU1] wc: read 6 bytes from fd 0
[40665594189] [[32mINFO [0m] [kernel::ipc::pipe] [CPU1] PIPE_DEQUEUE: EOF reached (writers=0)
       1
[40778033076] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=15 PID=15
[40865817498] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=16 PID=16
[40933816638] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[40935630417] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing primary port (0x1F0)...
[40939102380] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive(base=1f0, slave=false) - selecting drive a0
[
```
</details>
