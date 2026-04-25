# ✅ Scenario: POSIX behavior - head -n and tail -n (expected to fail if not fully POSIX)

> Last run: 2026-04-24 17:40:56

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 16789ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 453ms | - [📜](./02/serial.log) - |
| 3 | And I type "echo -e '1\n2\n3' | head -n 2 | tail -n 1" on the serial console | ✅ | 1308ms | - [📜](./03/serial.log) - |
| 4 | Then the command output should strictly be "2" | ✅ | 1ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[52214868618] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[52263727032] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[52328629254] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[52391604771] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[52428633213] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[52433330103] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[52499350035] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[52540430580] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[52568261130] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[52569407451] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[52649287647] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[52651323681] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initializ[53433790014] [[32mINFO [0m] [kernel::boot_progred"
[52805250762] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[52984276290] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[53039929998] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[53142815385] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[53286402576] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[53362559283] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
ess] [CPU0] boot_progress: milestone="Modules Scanned"
[53612141022] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[53753694423] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[54001445091] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[54471611040] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1782456093 elapsed_us=891228
[54473094258] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[54685717350] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[54688802322] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[54779557107] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[54875776659] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[54901074129] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[54902719179] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[55192904844] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[55198939488] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
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
[55391389515] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[55407995775] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[55474107843] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=37422
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[56112836835] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[56114971077] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
e[56357525235] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
cho -e[56683782342] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[56685198768] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[56697051411] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[56718532860] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
 '[56830228620] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=5, bind_id=322371585)
1[56932928811] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[56934570627] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[56966950986] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=5)
[56975293386] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Starting VFS-native VirtIO GPU driver...
[56976507720] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=10)
[56977138020] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: boot_arg=5
[56980401159] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Mapping bootstrap memfd 5 size=4096...
[56984204805] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: vm_map success at 0x400000001000
[56986098312] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Recovered handles: req_read=4, resp_write=5, svc=1, id=322371585
[56987724123] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting (drv_req_r=4, drv_resp_w=5, svc=1, id=322371585)
[56991581031] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[56992875951] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[56997251751] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[57015657831] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[57016945161] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[57033423744] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[57212201211] [[32mINFO [0m] [bristle] [CPU1] bristle: published pid 10 to /run/bristle/pid
[57237073938] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[57246967899] [[32mINFO [0m] [bristle] [CPU1] bristle: published device handles kbd_in=7 mouse_in=9
[57254112993] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[57255195624] [[32mINFO [0m] [bristle] [CPU1] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[57286959180] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[57288526779] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Virgl 3D not supported, using 2D only
[57301950387] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=1
[57362175288] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[57363512943] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[57389536215] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[57392931255] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[57412428843] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[57413912556] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[57426581850] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[57427856145] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[57431763213] [[32mINFO [0m] [sprout::superviso[57441086472] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
r] [CPU0] SPROUT: path_exists returned false
[57435107565] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[57756740250] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[57758062065] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[57786383292] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[57787828494] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[57789715137] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[57791112555] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[57792317517] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[57793527627] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[57819836943] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[57821138496] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[57824642931] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[58122693882] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[58124198913] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[58145582946] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[58147885488] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[58149334782] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[58150486911] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[58151679927] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[58152920034] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[58156583265] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exist[58194335166] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
s returned false
[58180563408] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[58696498476] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[58697927112] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[58701931431] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[58703122236] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[58737613935] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[58739811570] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[58740943866] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[58749697116] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[58767414915] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[58768739172] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[58773776490] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[58865575494] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: frame pool ready (1 buffer)
[58913282703] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_BIND_READY (ID: 322371585)
[58927878966] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Waiting for BIND_ASSIGNED...
\n2\n3' | head[59016716055] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: BIND_READY from instance_id=0x13370001
[59020705458] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Registering DISPLAY_CARD at /dev/display/card0 (handle=7)
 -n 2 | tai[59079428694] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[59081789580] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Handshake complete — task 'display' marked ready
[59088132246] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Sent MSG_BIND_ASSIGNED to req_port 3 (res=Ok(92))
[59089532040] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[59090684895] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[59097693468] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[59099021553] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[59119173366] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[59122271538] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_SERVICE_READY.
[59144074605] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[59147808258] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[59149148322] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[59150364834] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
l -n 1
[59392311297] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='echo'
[59483554944] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/echo' with argv=["/bin/echo", "-e", "1\\n2\\n3"]
[59565543213] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/echo' TID=11 PID=11
[59591951661] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='head'
[59671604421] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/head' with argv=["/bin/head", "-n", "2"]
[59747587647] [[32mINFO [0m] [kernel::ipc::pipe] [CPU2] PIPE_ENQUEUE: tail=6 len=6 n=6
[59750171415] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/head' TID=12 PID=12
[59941872144] [[32mINFO [0m] [kernel::ipc::pipe] [CPU3] PIPE_ENQUEUE: tail=2 len=2 n=2
[59950159236] [[32mINFO [0m] [kernel::ipc::pipe] [CPU3] PIPE_ENQUEUE: tail=4 len=4 n=2
[60064802655] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='tail'
[60089511999] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/tail' with argv=["/bin/tail", "-n", "1"]
[60149273547] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/tail' TID=13 PID=13
[60164176248] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 2 pipes
[60165527136] [[32mINFO [0m] [sh] [CPU3] sh: closing pipe 0 ends: read=6 write=7
[60167788197] [[32mINFO [0m] [sh] [CPU3] sh: closing pipe 1 ends: read=8 write=9
[?25l[60360873606] [[32mINFO [0m] [kernel::ipc::pipe] [CPU1] PIPE_DEQUEUE: EOF reached (writers=0)
2

```
</details>
