# ✅ Scenario: Launch ps and verify reaping

> Last run: 2026-04-24 17:40:56

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 22034ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 678ms | - [📜](./02/serial.log) - |
| 3 | And I type "ps" on the serial console | ✅ | 1780ms | - [📜](./03/serial.log) - |
| 4 | Then the command output should contain "PID" | ✅ | 5ms | - - - |
| 5 | And the command output should contain "sh" | ✅ | 1ms | - - - |
| 6 | And the command output should contain "ps" | ✅ | 1ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[68817772221] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[68926330737] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[69050800533] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[69157869759] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[69228749766] [[32mINFO [0m] [kernel] [CPU0] [70514603973] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
Initializing SIMD...
[69233702967] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[69292862364] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[69326483193] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[69353984205] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[69355271997] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[69420697764] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[69422816727] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[69660173913] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[69863360424] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[69925889649] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[70036659858] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[70265738829] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[70411672221] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[70786612809] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[70926392856] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[71191405098] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[71773851171] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=2318124567 elapsed_us=1159062
[71775323103] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[72096516888] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[72100212525] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[72312358338] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[72719038260] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[72757578597] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[72759494016] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[73164233670] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[73195417449] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[73199883405] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[73253157549] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[73302328572] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=36432
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[73658174898] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[73698625869] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[74136792873] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[74350944591] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[74352419328] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[74367376677] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[74369150460] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
ps
[74511472596] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=5, bind_id=322371585)
[74525979792] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ps'
[74559173337] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/ps' with argv=["/bin/ps"]
[74620015668] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[74642050593] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[74734532268] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/ps' TID=10 PID=10
[74816858853] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=11)
[74878690788] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[74880423255] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[74919505782] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[74924185248] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[74925452052] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[74934655917] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=5)
[74936544837] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Starting VFS-native VirtIO GPU driver...
[74968491675] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: boot_arg=5
[74971179657] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Mapping bootstrap memfd 5 size=4096...
[75055295832] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[75209842224] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: vm_map success at 0x400000001000
[75212167569] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Recovered handles: req_read=4, resp_write=5, svc=1, id=322371585
[75213711540] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting (drv_req_r=4, drv_resp_w=5, svc=1, id=322371585)
  PID  PPID STAT COMMAND
[75587580453] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[75588855012] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[75604378938] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[75636081015] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[75637480017] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[75638719200] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[75639884760] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[75641099358] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[75683609892] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[75698069172] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[75727737690] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 11 to /run/bristle/pid
[75744711108] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[75792768612] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=7 mouse_in=9
[75887013378] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[76082468055] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[76083850689] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[76093086399] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[76094389998] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[76095467679] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[76096675446] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[76097920107] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[76099169652] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[76129005282] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[76142029557] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[76172843670] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[76188861870] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[76231893936] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[76271922309] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[76275668865] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Virgl 3D not supported, using 2D only
[76326130155] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=1
[76561950465] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[76563375933] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[76594861728] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[76596396129] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[76597679697] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[76598955048] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[76600151760] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[76652512200] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[76683948693] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[76690858893] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[76713626979] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[77056130547] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[77066524623] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[77071463040] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[77072753802] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[77073897615] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[77075005623] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[77117884338] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[77146816758] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[77162339661] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[77167758888] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[77199091398] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
    5     0 R    
[77534767860] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[77536110828] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[77568681333] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[77570165640] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[77571357303] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[77572506528] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[77606028390] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[77607470490] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[77611244700] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[77634257877] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[77656977849] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[77996598966] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[77997906162] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[78023269335] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[78035591238] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[78083517303] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[78085312371] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[78087203502] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[78088523172] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
    6     5 R    /bin/sh
[78202578102] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[78204058878] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
    7     5 R    /bin/cambium
[78371717820] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
    8     5 S    /bin/iso9660d
    9     5 R    /drivers/display_virtio_gpu
[78811875945] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: frame pool ready (1 buffer)
[78831782304] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[78843794469] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[78868635582] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[78870082830] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[78871255584] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[78872356134] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[78873435630] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[78874580862] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[78880337349] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[78907986234] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_BIND_READY (ID: 322371585)
[78915864291] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Waiting for BIND_ASSIGNED...
[78928592754] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[78941157372] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[79036949211] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25l[79062989280] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: BIND_READY from instance_id=0x13370001
[79065424053] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Registering DISPLAY_CARD at /dev/display/card0 (handle=7)
[79146430605] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[79148536335] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Handshake complete — task 'display' marked ready
[79178913000] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Sent MSG_BIND_ASSIGNED to req_port 3 (res=Ok(92))
[79180373844] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[79181646456] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[79216605237] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[79235306436] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[79236388539] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[79247734797] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_SERVICE_READY.
[79272296796] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[79310512116] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[79312389783] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[79313558577] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
   10     6 R    /bin/ps
   11     5 S    /bin/bristle
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m 
```
</details>
