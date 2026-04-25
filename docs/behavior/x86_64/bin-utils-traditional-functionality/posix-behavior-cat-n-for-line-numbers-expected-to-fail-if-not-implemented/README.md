# ❌ Scenario: POSIX behavior - cat -n for line numbers (expected to fail if not implemented)

> Last run: 2026-04-24 17:40:56

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 17735ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 560ms | - [📜](./02/serial.log) - |
| 3 | And I type "echo hello | cat -n" on the serial console | ✅ | 1773ms | - [📜](./03/serial.log) - |
| 4 | Then the command output should contain "1  hello" | ❌ | 6030ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[54914584890] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[54973523616] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[55040739237] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[55128648102] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[55173226350] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[55177856976] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[55239796920] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[55275965943] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[55324209633] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[55325471685] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[55467073761] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[55469253180] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[55655089413] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[55790728455] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[55833658419] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[55933200345] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[56158084587] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[56266957494] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[56319811812] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[56524447782] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[56623643208] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[56997406488] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[58081489587] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=2563448415 elapsed_us=1281724
[58082932974] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[58495139010] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[58498500126] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[58630197054] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[58765476726] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[58809067779] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[58810641384] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[59058653544] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[59066942220] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[59071783914] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[59110880004] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[59148919632] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=38313
[1;32m
        .-.
       /   \        [1;36mTHING-OS[1;32m
      |     |       [0;36m"People, places, things."[1;32m
       \   /        
        `-'        
       /   \        v0.1  ���  
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[59572493178] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[59574690120] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[59937696786] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
ec[60084821082] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[60098693028] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[60111652722] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[60152540415] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
ho h[60369059685] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=5, bind_id=322371585)
el[60486023862] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[60487753623] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[60520668780] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=5)
[60536291277] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Starting VFS-native VirtIO GPU driver...
[60537904119] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: boot_arg=5
[60539658135] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Mapping bootstrap memfd 5 size=4096...
[60567337941] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=10)
[60594418599] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: vm_map success at 0x400000001000
[60599133012] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Recovered handles: req_read=4, resp_write=5, svc=1, id=322371585
[60600862443] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting (drv_req_r=4, drv_resp_w=5, svc=1, id=322371585)
[60619698249] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[60628095990] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[60648496689] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[60652647396] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[60653954559] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[60687052767] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[61135330410] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[61136681727] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[61269453828] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[61270891671] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[61272277242] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[61274468178] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[61275765606] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[61277218629] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[61280790648] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[61281994587] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[61310346966] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[61384696989] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[61396906362] [[32mINFO [0m] [bristle] [CPU1] bristle: published pid 10 to /run/bristle/pid
[61404108942] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[61423850367] [[32mINFO [0m] [bristle] [CPU1] bristle: published device handles kbd_in=7 mouse_in=9
[61430229168] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[61432139208] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Virgl 3D not supported, using 2D only
[61432735584] [[32mINFO [0m] [bristle] [CPU1] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[61465534548] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=1
[61674207903] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[61675559418] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[61706271990] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[61715770149] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[61717335075] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[61729303284] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[61733993376] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[61738667727] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[61743242385] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[61765185240] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[61774155630] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[62102922123] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[62104183383] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[62141215752] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[62142799191] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[62156849304] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[62157968631] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[62164303443] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[62182631544] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[62200027164] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[62227591668] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[62231855301] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[62573380782] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[62584889202] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[62608641282] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[62610153573] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[62611384308] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[62612562771] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[62641596765] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[62643107967] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[62646934581] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[62648096808] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[62702658414] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[63035047548] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[63036359265] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[63045520230] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[63046740141] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[63047878476] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[63069056325] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[63070644912] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[63088449369] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[63092247042] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[63093409500] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[63096954360] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[63641970876] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[63646493493] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[63660801138] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[63691488993] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[63705056646] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[63706245504] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[63707423076] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[63718307004] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[63722221398] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[63723432729] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[63726892284] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
lo | cat -
[64145830617] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[64147242951] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[64151141274] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[64154910666] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[64161471462] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[64162610787] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[64163715132] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[64219449426] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[64234611771] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='echo'
[64244465373] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[64259709492] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[64263578808] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[64271575269] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/echo' with argv=["/bin/echo", "hello"]
[64437846825] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/echo' TID=11 PID=11
[64462133043] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[64543558926] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/cat' with argv=["/bin/cat", "-"]
[64573954566] [[32mINFO [0m] [kernel::ipc::pipe] [CPU2] PIPE_ENQUEUE: tail=6 len=6 n=6
[64641864573] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[64643271165] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[64699981566] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/cat' TID=12 PID=12
[64878106161] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[64890169146] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[64891401960] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[64892603424] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[64893790599] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[64894991469] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[64945741443] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 1 pipes
[64947181101] [[32mINFO [0m] [sh] [CPU3] sh: closing pipe 0 ends: read=6 write=7
[64948674252] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[64950893172] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[64971084354] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[?25lhello
[65188830828] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: frame pool ready (1 buffer)
[65198651364] [[32mINFO [0m] [kernel::ipc::pipe] [CPU2] PIPE_DEQUEUE: EOF reached (writers=0)
[65262303117] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_BIND_READY (ID: 322371585)
[65264015322] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Waiting for BIND_ASSIGNED...
[65345745102] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: BIND_READY from instance_id=0x13370001
[65347403088] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Registering DISPLAY_CARD at /dev/display/card0 (handle=7)
[65472925584] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[65475999633] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Handshake complete — task 'display' marked ready
[65514360615] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Sent MSG_BIND_ASSIGNED to req_port 3 (res=Ok(92))
[65516054439] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[65536397586] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[65538464772] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[65564124747] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[65565646509] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[65566934301] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[65573792427] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_SERVICE_READY.
[65608979502] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[65610318840] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[65611539642] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hn[70546228698] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=13 PID=13
[72709830699] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=15 PID=15
[72968369265] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=16 PID=16
[72988513851] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[72991168998] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing primary port (0x1F0)...
[73007783541] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive(base=1f0, slave=false) - selecting drive a0
[73073998734] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive - initial status: ff
[73114648530] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive(base=1f0, slave=true) - selecting drive b0
[73146227682] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive - initial status: ff
[73211974902] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing secondary port (0x170)...
[73213460595] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive(base=170, slave=false) - selecting drive a0
[73226754249] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive - initial status: ff
[
```
</details>
