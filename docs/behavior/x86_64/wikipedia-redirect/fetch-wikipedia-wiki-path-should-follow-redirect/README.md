# ❌ Scenario: Fetch Wikipedia /wiki path should follow redirect

> Last run: 2026-04-24 11:14:27

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8513ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 252ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https en.wikipedia.org /https/wp" on the serial console | ✅ | 10786ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ❌ | 5969ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[25973558424] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26002026270] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26040018444] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26093797125] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26125366344] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26129766036] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26170896444] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26192259324] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26244151857] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26245273692] [[32mINFO [0m] [kernel:[:boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26336662968] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26415867390] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26447355132] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26530218000] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
26651209761] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26696632446] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26727948027] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[26892885888] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[26963199186] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[27167140308] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27571810629] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1304607777 elapsed_us=652303
[27572619492] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27682936380] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[27689454969] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27734857887] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[27810017169] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[27830261679] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27832019160] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27997978800] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[28013494179] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[28027445523] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[28051496649] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[28055042037] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=35244
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28297530426] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[28302365652] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[28332608733] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[28337434059] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[28344749895] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=5 specs_ptr=10002f8 count=2 timeout=28 ticks
[28347378741] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=3 flags=1 token=1
[28348729068] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=4 flags=1 token=2
[28359772815] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet ��� retrying
[28360703613] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[28361892735] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[28363384005] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[28682095563] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[28695027108] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[28696435449] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[28701428514] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[28702568169] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[28707847509] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[28717835091] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
m[28757257089] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=5, bind_id=322371585)
o[28791131985] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[28792059450] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[28798262163] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=5)
[28800378321] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Starting VFS-native VirtIO GPU driver...
[28801447752] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: boot_arg=5
[28803496788] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Mapping bootstrap memfd 5 size=4096...
[28807162560] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: vm_map success at 0x400000001000
[28808881893] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Recovered handles: req_read=4, resp_write=5, svc=1, id=322371585
[28810499124] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting (drv_req_r=4, drv_resp_w=5, svc=1, id=322371585)
[28816086519] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=10)
[28819555710] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[28820561550] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[28821340845] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[28823683185] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[28824727404] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[28829929458] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[28831116666] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[28832233881] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[28833795210] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=5 specs_ptr=10002f8 count=2 timeout=42 ticks
[28857955467] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=3 flags=1 token=1
[28859189634] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=4 flags=1 token=2
[28862132310] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[28866058485] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[28867318194] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[28955216829] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[28967962221] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[28975094379] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[28976038146] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Virgl 3D not supported, using 2D only
[28984981773] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=1
[29004943473] [[32mINFO [0m] [bristle] [CPU1] bristle: published pid 10 to /run/bristle/pid
[29018964645] [[32mINFO [0m] [bristle] [CPU1] bristle: published device handles kbd_in=7 mouse_in=9
[29023859436] [[32mINFO [0m] [bristle] [CPU1] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[29025015690] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=6 token=1
[29025782610] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=7 token=2
[29026483398] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=8 token=3
[29027249757] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] sys_wait_many: tid=10 specs_ptr=1000000 count=3 timeout=0 ticks
[29027963976] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1]   spec[0]: kind=7 object=6 flags=1 token=1
[29028725946] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1]   spec[1]: kind=7 object=7 flags=1 token=2
[29029451352] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1]   spec[2]: kind=7 object=8 flags=1 token=3
[29030512566] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=6 token=1
[29031394854] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=7 token=2
[29033452899] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=8 token=3
[29034901962] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] sys_wait_many: blocking...
[29176506447] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[29178235086] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[29179468164] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[29181948543] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[29183042823] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[29186780832] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[29187900984] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[29189271111] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[29190437793] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[29191582497] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[29192757462] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[29195975226] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[29215415097] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[29219675892] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[29221411098] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[29222678265] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[29223885801] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=5 specs_ptr=10002f8 count=2 timeout=53 ticks
[29225120298] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=3 flags=1 token=1
[29226324402] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=4 flags=1 token=2
[29228396406] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[29239322310] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[29240480907] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[29554930416] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[29556775248] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[29557367301] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: frame pool ready (1 buffer)
[29557794024] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[29566483122] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[29567632710] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[29575134633] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[29575910298] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[29576575182] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[29577301215] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[29577953724] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[29578291776] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_BIND_READY (ID: 322371585)
[29578902342] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[29579902935] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Waiting for BIND_ASSIGNED...
[29581227456] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[29581932930] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[29584138683] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[29585135085] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[29585872305] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[29587440234] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: returning 1 results
unt -t https en[29614753938] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: BIND_READY from instance_id=0x13370001
[29616293157] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Registering DISPLAY_CARD at /dev/display/card0 (handle=7)
.[29641439718] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[29643447537] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Handshake complete �� task 'display' marked ready
[29649960714] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Sent MSG_BIND_ASSIGNED to req_port 3 (res=Ok(92))
[29651207718] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[29652302889] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[29656461747] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[29659103331] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[29667868197] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[29672735796] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[29673513936] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[29674447473] [[32mINFO [0m] [sprout::superv[29686781454] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[29688037566] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[29689539660] [[32mINFO [0m] [kernel::syscall::handlersisor] [CPU0] ::wait] [CPU3] collect_ready: polling spec[0] kind=7 objSPROUT: Calling pathect=a token=2
[29691382974_exists(/dev/display/card0)...
w[29681766279] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[29683745157] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_SERVICE_READY.
] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=58 ticks
[29694349410] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[29696869488] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[29702819850] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[29704216773] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[29716301769] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[29717118816] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[29717871546] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[29718684567] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=59 ticks
[29719416672] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[29720122608] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[29721234741] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[29722244541] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[29723403204] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
ikipedia.[30157730328] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: unblocked
[30158785866] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[30159603276] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[30168604719] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[30170047281] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[30171439980] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[30172138755] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=73 ticks
[30172919898] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[30173615703] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[30174660945] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[30175342890] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[30176012592] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
org /https[30686322414] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: unblocked
[30687559980] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[30688312215] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[30696234360] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[30697396554] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[30698723418] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[30699613956] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=89 ticks
[30700420047] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[30701221782] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[30702499806] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[30703290156] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[30704023878] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
/wp
[30909279192] [[32mINFO [0m] [sh] [CPU2] sh: spawning job cmd='mount'
[30916958556] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: unblocked
[30918962349] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[30920237601] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[30921930798] [[32mINFO [0m] [sh] [CPU2] sh: spawning '/bin/mount' with argv=["/bin/mount", "-t", "https", "en.wikipedia.org", "/https/wp"]
[30933794001] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[30935429250] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[30936685692] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[30937999620] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=96 ticks
[30939203757] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[30940453896] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[30942343113] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[30943524876] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[30944815902] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
[30990887532] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU2] SYSCALL SPAWN_PROCESS_EX: name='/bin/mount' TID=11 PID=11
[30997818621] [[32mINFO [0m] [sh] [CPU2] sh: cleaning up 0 pipes
[?25l[31014614532] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: unblocked
[31015754385] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[31016548794] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[31024470543] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[31025187666] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[31026912246] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[31028806743] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=99 ticks
[31030694739] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[31032558975] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[31033771692] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[31034469906] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[31035134559] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
[31080831837] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: unblocked
[31082369769] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[31083140979] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[31091935842] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[31093353291] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[31095319596] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[31096917126] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=101 ticks
[31098237126] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[31098988074] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[31100204388] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[31100900226] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[31101572733] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
[31146792897] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: unblocked
[31147882953] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[31148681454] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[31156080780] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[31156792557] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[31158497007] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[31159962339] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=103 ticks
[31161710316] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[31163337117] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[31164931215] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[31165630485] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[31166304642] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
[31200298371] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU2] SYSCALL SPAWN_PROCESS_EX: name='/bin/httpsd' TID=12 PID=12
[31208603679] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: unblocked
[31209700500] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[31210474383] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[31218621258] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[31219603965] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[31220353263] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[31221041049] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=104 ticks
[31221891228] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[0]: kind=7 object=8 flags=1 token=1
[31222574328] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[1]: kind=7 object=a flags=1 token=2
[31223799057] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[31224704511] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[31235243061] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[31236436737] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[31239072777] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[31242309021] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=105 ticks
[31243013043] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[0]: kind=7 object=8 flags=1 token=1
[31243730727] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[1]: kind=7 object=a flags=1 token=2
[31244810157] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[31245543087] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[31246362510] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: blocking...
[31266292332] [[32mINFO [0m] [httpsd] [CPU3] HTTPSD_READY
[31268399844] [[32mINFO [0m] [httpsd] [CPU3] httpsd: entering main RPC loop for /https/wp
[31271309025] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: unblocked
[31272385056] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[31273459767] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[31281874206] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[31282606113] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[31283318946] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[31283986965] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=106 ticks
[31284824010] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[0]: kind=7 object=8 flags=1 token=1
[31285809753] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[1]: kind=7 object=a flags=1 token=2
[31286901426] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[31287577728] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[31300705029] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[31302031365] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[31303244544] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[31304390040] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=107 ticks
[31305573717] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[0]: kind=7 object=8 flags=1 token=1
[31306783695] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[1]: kind=7 object=a flags=1 token=2
[31308786201] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[31309978029] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[31311088050] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: blocking...
[31343717856] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: unblocked
[31345562226] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[31346800947] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[31357271352] [[32mINFO [0m] [display_virtio_gpu] [CPU2] display_virtio_gpu: waiting on WaitSet...
[31358103117] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[31359226074] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[31360133343] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=109 ticks
[31360833702] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[0]: kind=7 object=8 flags=1 token=1
[31361499180] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2]   spec[1]: kind=7 object=a flags=1 token=2
[31362606198] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=8 token=1
[31363296459] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] collect_ready: polling spec[0] kind=7 object=a token=2
[31364024934] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU2] sys_wait_many: blocking...
[31396664904] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=14 PID=14
[31444449696] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=15 PID=15
[31505318559] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=16 PID=16
[31509514509] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=5 token=1
[31510408314] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=1 object=6 token=2
[31511555988] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=6 token=3
[31512255687] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=15 specs_ptr=1010228 count=3 timeout=0 ticks
[31512959940] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=5 flags=1 token=1
[31513687524] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=1 object=6 flags=1 token=2
[31514363694] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[2]: kind=7 object=6 flags=1 token=3
[31515600303] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=5 token=1
[31516331154] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=1 object=6 token=2
[31517048805] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=6 token=3
[31517747481] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
[31535079939] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: unblocked
[31536500094] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[31538414094] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[31552405929] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[31553465328] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[31555141563] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[31556640423] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=115 ticks
[31558094469] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[31559895510] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[31562030379] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[31564722717] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[31565423538] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
[31570424985] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[31571377926] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing primary port (0x1F0)...
[31573488639] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive(base=1f0, slave=false) - selecting drive a0
[31582142658] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive - initial status: ff
[31588920693] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[31590554952] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[31591786512] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[31593293985] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=4
[31593672528] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive(base=1f0, slave=true) - selecting drive b0
[31594279233] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path=''
[31594992429] [[32mINFO [0m] [httpsd] [CPU3] httpsd: resolve_path path=''
[31601786898] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive - initial status: ff
[31607365020] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[31608735246] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[31609938855] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[31610745474] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing secondary port (0x170)...
[31611090852] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=116 ticks
[31611941658] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive(base=170, slave=false) - selecting drive a0
[31612317693] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[31613469954] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[31615189089] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[31616404611] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[31617534795] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[31619780841] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive - initial status: ff
[31627831521] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive(base=170, slave=true) - selecting drive b0
[31635770034] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive - initial status: ff
[31645804971] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[31646624592] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Entering RPC service loop
[31660548645] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: VFS provider mounted at /dev/ata_ctl
[31687500009] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[31689126018] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[31690361505] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[31693474494] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: No active devices to service
[31694282664] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Entering main service loop...
[31695182310] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Calling svc.next_event()...
[31696322262] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=6 token=1
[31697127561] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=1 object=4 token=2
[31697857488] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] sys_wait_many: tid=16 specs_ptr=1010048 count=2 timeout=0 ticks
[31698593619] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1]   spec[0]: kind=7 object=6 flags=1 token=1
[31699274574] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1]   spec[1]: kind=1 object=4 flags=1 token=2
[31700229594] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=6 token=1
[31700957277] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=1 object=4 token=2
[31701479634] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[31702056837] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] sys_wait_many: blocking...
[31702623546] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[31703864016] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[31705027728] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=119 ticks
[31706352183] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[31707535266] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[31709551170] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[31710707358] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[31711863249] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[31743108870] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=6 token=1
[31744593045] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=7 token=2
[31745782959] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] sys_wait_many: tid=7 specs_ptr=1000600 count=2 timeout=3119 ticks
[31746497475] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1]   spec[0]: kind=7 object=6 flags=1 token=1
[31747184205] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1]   spec[1]: kind=7 object=7 flags=1 token=2
[31748569512] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=6 token=1
[31749266340] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=7 token=2
[31749931455] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] sys_wait_many: blocking...
[31752863406] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[31772765409] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[31778320926] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[31814751111] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[31815483810] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[31820220597] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[31820916303] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=122 ticks
[31821641214] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[31822340385] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[31823491095] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[31824166110] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[31824825483] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[31841124612] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[31842777417] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[31843508697] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[31857733710] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[31858486077] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[31859173731] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[31859883066] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=124 ticks
[31860617679] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[31861290351] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[31862281374] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[31862997309] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[31863686580] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[31885161462] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[31887159480] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[31887870993] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[31894619889] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[31895299656] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[31895994570] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[31896653184] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=125 ticks
[31897339518] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[31898054067] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[31899016248] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[31899723933] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[31900416372] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[31940899056] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[31942693794] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[31943419662] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[31956153702] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[31956998898] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[31957693647] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[31958456343] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=127 ticks
[31959174753] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[31959863958] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[31960954806] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[31961639028] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[31962290250] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[32007100554] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[32008612185] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32009710293] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32023042821] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[32023734105] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32024461029] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32025177789] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=129 ticks
[32025900786] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[32026590750] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[32027602101] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32028287610] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32028960315] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[32074315449] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[32076171072] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32077508364] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32093887023] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[32095136238] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32096327307] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32097624735] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=131 ticks
[32098842039] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[32100153063] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[32102348817] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32103582357] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32104749831] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[32148801993] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[32150461959] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32151734373] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32159316486] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[32160060999] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32160741459] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32161392153] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=133 ticks
[32162074362] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[32162774787] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[32164009086] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32164695882] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32165370072] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[32214807768] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[32216792058] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32217523701] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32224608603] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[32225446407] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32226153069] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32226836763] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=135 ticks
[32227550520] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[32228274639] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[32229304008] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32229965229] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32230607244] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[32271323799] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[32272567668] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32273301456] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32286353814] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[32287109217] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32287853565] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32288515017] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=137 ticks
[32289286293] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[32290021995] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[32291657574] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32292664338] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32293658991] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[32346748038] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[32348693289] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32349852546] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32359710768] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[32360756109] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32361812373] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32362839564] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=139 ticks
[32363938299] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[32365001493] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[32366721948] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32367655782] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32368308159] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[32413515981] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[32415213930] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32416380150] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32425624374] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[32426927379] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32427843525] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32428728090] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=141 ticks
[32429802372] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[32430824877] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[32432528634] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32433557739] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32434619118] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[32480121663] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[32482043847] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32483052327] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32492435514] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[32493580086] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32494616385] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32496151545] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=143 ticks
[32497217610] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[32498280804] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[32501206485] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32502257172] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32503297233] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[32578558947] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[32581610523] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32584478619] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32594123562] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[32595368322] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32596565661] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32597680764] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=146 ticks
[32598406566] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[32599143588] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[32600791740] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32601458571] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32602147380] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[32644568253] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[32646726981] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32648003190] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32657132442] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[32657832999] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32658535437] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32659213488] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=148 ticks
[32659958364] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[32660698422] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[32662130292] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32662802601] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32663571336] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[32710625079] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[32712339330] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32713617024] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32725164714] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[32726394228] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32727567939] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32728800192] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=150 ticks
[32730107883] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[32731342611] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[32733639840] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32734813815] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32736031284] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[32768513547] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[32770190739] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32771393853] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32789386344] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[32790676776] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32791919424] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32793064524] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=152 ticks
[32794256517] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[32795517381] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[32797753065] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32798983140] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32800112037] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[32842708041] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[32844987813] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32846236797] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32857705947] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[32858824515] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32859940509] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32861103297] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=154 ticks
[32862334956] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[32863477482] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[32865278391] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32866410192] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32867564895] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[32908547199] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[32909735859] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32910559704] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32918259495] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[32918976420] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32919722187] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32920843593] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=156 ticks
[32921703639] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[32922379611] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[32923779339] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32924484120] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32925207249] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[32974994877] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[32977078497] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32978335929] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32988701988] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[32989924803] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32991087558] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32992223451] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=158 ticks
[32993435277] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[32994598890] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[32996588427] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[32997737850] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[32998872720] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[33041234061] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[33043326096] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33044577423] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33053205735] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[33053935728] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33054637407] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33055342881] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=160 ticks
[33056030271] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[33056689116] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[33058210449] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33058922325] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33059661987] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[33106211721] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[33107248086] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33107991312] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33115667937] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[33116755320] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33117932727] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33119045553] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=162 ticks
[33120213258] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[33121367631] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[33123104157] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33124254306] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33125374722] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[33173527992] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[33175430607] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33176648538] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33187060203] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[33188272920] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33189410067] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33190549227] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=164 ticks
[33191734521] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[33192889917] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[33194809164] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33195955485] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33197106756] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[33238872018] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[33240461331] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33241180269] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33248245371] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[33249077895] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33249781554] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33250430664] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=166 ticks
[33251270877] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[33252075450] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[33253279488] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33253954899] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33254596320] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[33297029634] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[33298859088] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33299617923] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33313126803] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[33314164455] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33314850624] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33315533823] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=168 ticks
[33316227648] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[33316997208] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[33318061656] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33318988923] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33319687203] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[33361673235] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[33362854041] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33363765204] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33376416183] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[33377145285] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33378245274] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33379345857] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=170 ticks
[33380393409] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[33381086475] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[33382153167] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33382808646] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33383450694] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[33405837729] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[33407137236] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33407953524] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33416272131] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[33417101223] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33417850092] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33418509300] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=171 ticks
[33419201673] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[33419860386] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[33421045614] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33421701258] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33422341161] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[33494873742] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[33496521861] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33497273832] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33511206498] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[33512036052] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33512731560] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33513417696] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=174 ticks
[33514105680] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[33514764294] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[33516007305] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33516688260] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33517336842] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[33569623791] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[33571497729] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33572722755] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33583220880] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[33584418384] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33585588102] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33586731915] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=176 ticks
[33587991129] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[33589262652] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[33591251067] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33592418046] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33593538528] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[33636508323] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[33638279763] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33639493965] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33649802505] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[33651021096] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33652199031] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33653320503] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=178 ticks
[33654569256] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[33655715115] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[33657880641] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33659010363] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33660104181] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[33705782055] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[33707752617] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33708984045] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33719220942] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[33720447585] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33721582158] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33722829228] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=180 ticks
[33724039008] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[33725179290] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[33727004718] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33728148564] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33729277593] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[33758430618] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[33760195029] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33761400387] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33778256655] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[33779529630] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33780757098] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33781942656] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=182 ticks
[33783192993] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[33784395414] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[33786341193] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33787510845] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33788751348] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[33833991477] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[33835942998] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33837210891] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33848322585] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[33849528240] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33850704987] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33851868171] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=184 ticks
[33853195695] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[33854398248] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[33856422204] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33857558064] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33858749595] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[33900009429] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[33901846902] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33903088758] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33913515834] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[33914805639] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33916016343] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33917182431] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=186 ticks
[33918408249] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[33919605621] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[33922626507] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33923782200] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33924970497] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[33966024675] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[33967930986] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33968668503] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33975616422] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[33976313283] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33976987110] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33977668164] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=188 ticks
[33978349020] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[33979289553] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[33980446467] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[33981136233] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[33981780690] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[34022975877] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[34024640694] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34026089130] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34039699089] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[34040468187] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34041164850] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34041833859] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=190 ticks
[34042537188] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[34043243751] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[34044482703] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34045229163] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34045951665] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[34066835253] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[34068606363] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34069878348] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34080444882] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[34081644993] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34082936382] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34084220940] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=191 ticks
[34085460849] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[34086576711] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[34088703066] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34089887964] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34091038938] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[34131124467] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[34132761036] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34133981112] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34142356875] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[34143043242] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34143713736] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34144440726] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=193 ticks
[34145132604] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[34145813988] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[34147017168] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34147681821] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34148325552] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[34187676303] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[34189290135] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34190503941] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34202735787] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[34203476340] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34204156602] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34204840098] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=195 ticks
[34205517093] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[34206168480] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[34207233126] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34207935960] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34208587380] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[34231462353] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[34232552706] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34233283590] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34240678593] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[34241432808] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34242160194] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34243039710] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=196 ticks
[34243765842] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[34244492733] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[34245675057] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34246331361] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34246973838] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[34296651774] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[34298600622] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34299754071] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34310961102] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[34311652914] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34312339281] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34313005419] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=198 ticks
[34313724753] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[34314375249] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[34315689540] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34316344260] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34317046302] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[34362406320] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[34363443972] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34364236566] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34371367338] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[34372070535] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34372828809] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34373516133] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=200 ticks
[34374220947] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[34375131351] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[34376801646] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34377484383] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34378221768] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[34428336624] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[34430649363] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34431659889] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34439743305] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[34440455148] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34441322454] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34442337864] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=202 ticks
[34443023736] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[34443753597] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[34445225892] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34445877675] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34446578067] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[34495180071] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[34497061566] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34498289859] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34509183291] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[34509886719] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34510545663] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34511275095] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=204 ticks
[34511967006] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[34512663702] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[34513833288] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34514556978] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34515191535] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[34560764502] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[34562614152] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34563491424] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34570025028] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[34570725849] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34571412084] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34572085647] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=206 ticks
[34572790725] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[34573505010] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[34574564805] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34575216819] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34575858273] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[34617220572] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[34619388177] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34620146880] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34632418557] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[34633235967] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34633905438] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34634547816] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=208 ticks
[34635230124] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[34635887913] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[34636958334] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34637612493] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34638317406] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[34683114312] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[34684919577] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34686197238] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34701988101] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[34703202039] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34704457623] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34705679217] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=210 ticks
[34706905464] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[34708110954] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[34710339048] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34711506489] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34712698878] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[34782634392] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[34784464902] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34785187965] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34797602631] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[34798405917] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34799070933] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34799721165] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=213 ticks
[34800394266] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[34801079841] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[34802080401] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34802811516] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34803494880] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[34848728970] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[34850367057] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34851564033] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34864848744] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[34865539401] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34866199104] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34866869004] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=215 ticks
[34867564677] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[34868699448] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[34870461747] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34871586123] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34872222198] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[34915111902] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[34917185589] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34918519746] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34932384003] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[34933183692] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34933989750] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34934644041] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=217 ticks
[34935318990] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[34935999747] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[34937168178] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34937843853] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34938641529] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[34980440715] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[34982080617] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34982858658] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34995818418] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[34996632891] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[34997305662] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[34997951802] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=219 ticks
[34998649323] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[34999302921] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[35000265366] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35000917050] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35001552069] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[35048251656] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[35050197666] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35051457540] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35068142175] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[35069540781] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35070781911] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35072023470] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=221 ticks
[35073236847] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[35074453788] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[35076497907] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35077751709] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35078914134] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[35128673448] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[35130510426] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35131653183] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35142517377] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[35144247402] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35145380952] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35146428801] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=223 ticks
[35147525325] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[35148554265] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[35150609241] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35168765742] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35169903615] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[35223096414] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[35224864752] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35226118026] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35323476903] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[35324940189] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35326106673] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35327212635] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=228 ticks
[35328432480] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[35329591704] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[35331772740] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35332978263] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35334102177] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[35356992000] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[35360303616] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35361605763] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35375552058] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[35376967593] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35378146023] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35379185028] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=229 ticks
[35380332273] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[35381630361] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[35384134302] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35385809811] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35391890160] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[35411032305] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[35412346563] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35413243140] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35426729019] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[35427533691] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35428226361] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35428980213] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=231 ticks
[35429681265] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[35430432312] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[35431539594] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35432239293] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35433207414] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[35478448071] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[35480615280] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35481990687] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35497693143] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[35498483196] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35499275559] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35500142997] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=233 ticks
[35501187051] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[35502180912] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[35503681257] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35504359473] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35505197178] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[35551223235] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[35554483008] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35555793636] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35569243512] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[35570927964] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35572193547] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35573403096] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=235 ticks
[35574582549] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[35575799061] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[35577242448] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35578071243] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35578749954] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[35619053481] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[35621182443] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35622413475] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35633163456] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[35634520251] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35635703961] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35636836356] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=237 ticks
[35638096197] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[35639373363] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[35641843710] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35642994321] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35644160739] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[35672849454] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[35675036034] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35675805561] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35689229070] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[35690033346] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35690813961] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35691668760] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=239 ticks
[35692429476] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[35693184648] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[35694549990] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35695220484] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35695872069] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[35743553340] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[35745226638] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35746472520] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35762207514] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[35763141909] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35763885168] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35764575990] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=241 ticks
[35765285061] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[35765996046] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[35767063662] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35767725543] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35768373729] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[35783807202] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[35784923328] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35785692063] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35792687403] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[35793596883] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35794471185] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35795122605] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=242 ticks
[35795838936] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[35796497286] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[35797856787] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35798520054] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35799220215] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[35842085004] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[35843797044] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35845018704] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35861368224] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[35862689280] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35863931433] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35865118542] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=244 ticks
[35866323075] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[35867492991] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[35871897765] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35872931358] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35873734446] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[35917080243] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[35920864122] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35922475842] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35932314990] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[35933532129] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35934778704] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35935869321] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=246 ticks
[35937094578] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[35938179519] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[35946422622] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35954769114] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35956111521] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[35968889286] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[35970645183] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35971443024] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35979534129] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[35981363814] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35992552233] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35993515041] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=248 ticks
[35994307305] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[35995039641] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[35996725644] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[35997584106] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[35998528467] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[36039719529] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[36041518359] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36042817371] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36059426238] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[36060683274] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36061873188] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36063056469] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=250 ticks
[36064249056] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[36065451906] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[36067439100] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36068674488] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36069920040] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[36113666358] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[36115050576] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36115800798] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36124157751] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[36124981629] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36125662617] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36126318690] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=252 ticks
[36127033041] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[36127727427] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[36128822037] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36129481245] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36130182858] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[36180624810] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[36182353581] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36183450765] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36193214475] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[36194371884] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36195441183] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36196484808] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=254 ticks
[36197552490] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[36198679341] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[36200701878] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36201796488] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36202824834] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[36245948739] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[36247839672] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36249101757] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36260571171] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[36261735741] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36262898958] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36264058017] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=256 ticks
[36265347888] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[36266491866] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[36268788138] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36269975742] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36271101471] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[36312227358] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[36313928574] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36315216036] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36326564868] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[36327746994] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36328907010] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36330153618] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=258 ticks
[36331397190] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[36332613240] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[36334459722] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36335621586] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36336814536] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[36378956955] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[36381165018] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36382151652] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36394106199] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[36395346306] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36396493617] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36397688844] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=260 ticks
[36398861961] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[36400015245] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[36401916408] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36403026726] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36404093319] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[36444038136] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[36445579203] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36446306622] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36454892430] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[36455805078] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36456481611] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36457119897] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=262 ticks
[36457829199] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[36458471907] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[36459888564] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36460557738] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36461217639] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[36510823767] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[36513172179] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36514423044] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36524468739] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[36525627765] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36526700166] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36527767914] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=264 ticks
[36528841008] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[36529898031] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[36532156617] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36533237862] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36534307227] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[36577055724] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[36578977743] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36580226628] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36591592059] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[36592894008] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36594043035] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36595171800] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=266 ticks
[36596477907] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[36597679899] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[36599634687] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36600753123] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36601827900] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[36643062687] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[36644725227] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36645907320] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36656124846] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[36657260607] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36658329774] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36659401251] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=268 ticks
[36660489195] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[36661639806] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[36663695145] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36664747746] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36665857965] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[36708902670] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[36710904054] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36712147791] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36723027957] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[36724213548] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36725411151] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36726538959] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=270 ticks
[36727809624] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[36728954823] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[36731111505] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36732188724] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36733290396] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[36774836010] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[36776769513] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36777997146] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36788402937] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[36789577044] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36790725114] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36791817579] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=272 ticks
[36792970665] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[36794123652] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[36797216445] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36798346464] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36799448730] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[36832681050] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[36834673260] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36836105493] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36852550515] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[36853913316] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36855109467] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36856284234] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=274 ticks
[36857466921] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[36858654096] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[36860700492] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36861880308] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36863029863] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[36907183995] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[36909265107] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36910489143] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36920830287] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[36922159197] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36923422536] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36924531369] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=276 ticks
[36925817841] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[36926976405] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[36929282478] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36930431307] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36931576308] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[36972723678] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[36973963752] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36974714040] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36984605130] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[36985863783] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36987032049] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36988226385] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=278 ticks
[36989549784] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[36990837147] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[36993541332] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[36994740090] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[36995836515] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[37039260060] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[37041668367] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37042802082] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37052675748] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[37053790257] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37054828833] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37055850810] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=280 ticks
[37056893544] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[37058436657] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[37060712007] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37061761968] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37062911952] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[37105236465] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[37106963157] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37108080999] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37117473393] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[37118197710] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37118864838] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37119536586] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=282 ticks
[37120605819] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[37121463852] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[37123070721] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37123808139] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37124521599] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[37170726219] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[37171944777] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37172725392] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37180028787] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[37180734261] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37181402907] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37182043272] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=284 ticks
[37182933744] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[37183831443] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[37185037626] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37186082142] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37187030232] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[37236744633] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[37238054733] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37238925867] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37246553256] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[37247312322] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37248151083] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37248817848] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=286 ticks
[37249510683] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[37250154579] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[37252109301] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37253154411] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37254141474] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[37320031782] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[37321805829] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37322941227] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37344410532] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[37352243841] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37364315769] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37365471000] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=289 ticks
[37366514955] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[37367529738] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[37369509276] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37370556168] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37371643815] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[37403094069] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[37405344273] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37406655693] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37417332216] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[37418567274] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37419730326] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37420986900] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=291 ticks
[37422402534] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[37423664421] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[37426219149] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37427373720] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37428437277] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[37470023679] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[37472041464] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37473217947] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37504349421] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[37505637774] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37506723672] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37507724232] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=293 ticks
[37517980599] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[37519055442] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[37520863083] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37525329732] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37526367615] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[37568050311] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[37569890259] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37571088786] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37583517774] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[37584364158] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37585183053] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37585856385] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=295 ticks
[37586635911] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[37587495924] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[37589296635] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37590256572] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37591071540] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[37634646819] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[37636458486] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37637690574] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37648717887] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[37649903313] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37650715344] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37651379733] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=297 ticks
[37652107416] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[37652966175] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[37654763091] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37655435499] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37656066393] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[37700242470] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[37701993615] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37703194023] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37713823785] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[37715003568] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37716173715] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37717301424] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=299 ticks
[37718443719] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[37719606111] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[37721778171] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37722944820] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37724039133] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[37766440305] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[37768445484] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37769566923] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37779357297] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[37780434483] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37781461740] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37782519852] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=301 ticks
[37783599249] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[37784628123] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[37786404942] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37787520012] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37788563769] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[37832854554] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[37834632660] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37835785482] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37846040628] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[37847187807] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37848293076] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37849350924] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=303 ticks
[37850475267] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[37851570075] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[37854176712] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37855179153] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37856235087] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[37898270124] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[37900240983] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37901402946] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37911381816] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[37912496094] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37913597337] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37914651555] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=305 ticks
[37915733493] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[37916913837] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[37919106852] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37920319074] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[37921332933] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[37988360124] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[37990241190] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[37991245017] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38006236422] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[38007101913] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38007794616] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38008438182] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=308 ticks
[38009148342] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[38009954268] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[38011482036] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38012174541] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38012842032] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[38068027536] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[38071674531] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38072893155] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38084814768] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[38086204596] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38087306334] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38088447705] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=310 ticks
[38089602771] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[38090724078] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[38106285063] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38107503159] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38108640537] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[38120246406] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[38121973362] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38123223534] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38143974165] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[38145389799] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38146750422] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38147993004] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=312 ticks
[38149086426] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[38150129820] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[38152522353] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38153589342] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38154609603] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[38205070431] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[38211218430] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38212423524] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38221002039] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[38221870533] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38222889210] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38223567492] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=314 ticks
[38227613259] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[38228288208] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[38229947910] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38230742649] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38231401230] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[38235770529] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[38237407461] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38238432243] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38248708641] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[38249892912] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38250929805] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38252181363] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=315 ticks
[38253294915] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[38254056258] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[38261473998] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38262932466] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38286816018] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[38328695097] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[38330630613] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38331892863] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38342964429] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[38344124313] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38345283141] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38346460020] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=318 ticks
[38347686597] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[38348840937] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[38350650228] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38351750745] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38352600594] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[38394065325] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[38395477428] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38396230257] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38403306942] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[38404015914] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38404702578] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38405375910] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=320 ticks
[38406049737] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[38406699144] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[38407887903] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38408707656] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38409775404] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[38459937714] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[38461730142] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38462949921] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38474145468] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[38475345348] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38476627629] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38477800680] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=322 ticks
[38479056231] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[38480229711] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[38482016100] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38483136021] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38484320094] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[38526155415] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[38528229267] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38529482145] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38540210874] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[38541453324] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38542647726] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38543781936] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=324 ticks
[38544947034] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[38546131140] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[38548149123] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38549385765] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38550488163] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[38592545145] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[38594513199] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38595755715] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38606305023] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[38607522855] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38608748508] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38609899317] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=326 ticks
[38611268223] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[38612554992] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[38614547499] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38615721738] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38616873438] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[38658443901] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[38660540886] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38661791982] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38672403759] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[38673627267] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38674798734] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38675931558] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=328 ticks
[38677105599] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[38678269509] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[38680402068] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38681546673] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38682623133] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[38724362424] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[38726148648] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38727415386] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38737596843] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[38738834607] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38739993930] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38741139657] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=330 ticks
[38742267894] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[38743405305] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[38745872814] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38746989732] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38748076455] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[38790908343] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[38792713113] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38793920055] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38804168007] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[38805378810] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38806601163] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38807814177] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=332 ticks
[38808978285] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[38810153646] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[38812065501] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38813249079] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38814336330] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[38856817098] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[38858757465] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38860041990] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38870570970] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[38871883875] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38873108868] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38874268686] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=334 ticks
[38875614525] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[38876798400] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[38879192286] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38880331875] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38881467009] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[38922485547] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[38924324439] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38925596589] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38936164080] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[38937548694] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38938798536] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38939990628] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=336 ticks
[38941290894] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[38942447907] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[38946188028] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[38947362201] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[38948538090] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[39012959139] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[39014930790] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39016205019] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39030219855] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[39031038123] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39031803096] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39032453691] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=339 ticks
[39033123393] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[39033832695] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[39035361420] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39036041550] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39036721218] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[39087966456] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[39089951571] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39091124490] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39102466095] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[39103687458] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39104837409] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39105949476] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=341 ticks
[39107172027] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[39108281817] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[39110180241] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39111277425] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39112354182] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[39153984606] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[39156247845] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39157861809] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39170031780] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[39171196680] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39172370589] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39173506053] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=343 ticks
[39174683163] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[39175864728] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[39178172484] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39179417508] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39180753447] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[39244289601] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[39246184758] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39247452354] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39261566850] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[39262327302] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39263001162] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39263816130] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=346 ticks
[39264586746] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[39265389999] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[39266438508] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39267220542] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39267912255] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[39310687614] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[39312869739] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39314128986] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39331291362] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[39332729172] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39333996570] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39335253540] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=348 ticks
[39336501831] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[39337732797] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[39339805857] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39340970559] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39342113976] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[39385217058] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[39387074661] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39388340937] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39398871864] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[39400103424] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39401354025] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39402578655] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=350 ticks
[39403821237] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[39404942511] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[39407319567] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39408415299] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39409573929] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[39452065620] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[39453902268] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39455155905] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39465744615] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[39466937070] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39468220341] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39469436589] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=352 ticks
[39470576640] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[39471739230] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[39473540601] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39474686064] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39475825389] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[39518026284] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[39520048359] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39521307375] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39532233510] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[39533477775] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39534682242] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39535874829] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=354 ticks
[39537114342] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[39538274325] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[39540199248] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39541345437] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39542423415] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[39575179974] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[39576928083] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39578131296] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39595733001] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[39597096330] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39598292514] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39599443389] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=356 ticks
[39600675906] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[39601804077] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[39604291089] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39605427378] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39606587691] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[39649862604] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[39651264840] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39652085385] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39659649018] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[39660400494] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39661133061] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39661948854] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=358 ticks
[39662828832] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[39663601923] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[39664895292] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39665618454] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39666364848] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[39716072319] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[39718218375] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39719491713] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39730159821] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[39731382537] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39732550704] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39733654521] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=360 ticks
[39734872749] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[39736021281] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[39737978181] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39739117803] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39740289798] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[39782197752] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[39784239957] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39785527122] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39795907272] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[39797183052] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39798403524] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39799561296] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=362 ticks
[39800810181] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[39802028772] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[39804216606] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39805387941] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39806550102] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[39847832376] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[39849816270] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39851072151] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39859616016] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[39860646012] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39861587634] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39862498764] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=364 ticks
[39863492196] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[39864429330] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[39865983366] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39866909511] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39867818463] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[39913958832] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[39915913818] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39917170029] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39927840447] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[39929101509] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39930289344] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39931397253] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=366 ticks
[39932639076] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[39933759921] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[39935635806] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39936763548] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39937830570] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[39980279625] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[39982115844] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39983388456] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39994764381] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[39996010890] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[39997162590] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[39998254956] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=368 ticks
[39999450150] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[40000640955] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[40002550533] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40003704543] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40004748300] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[40036644021] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[40038663885] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40039907391] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40056583017] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[40057740228] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40058887506] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40059932385] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=370 ticks
[40061034684] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[40062132660] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[40063877502] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40065021216] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40066249344] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[40112383707] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[40114101423] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40115212632] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40125754053] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[40126812759] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40127856714] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40128863247] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=372 ticks
[40129923405] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[40131083850] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[40132652802] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40133699760] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40134715401] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[40178576295] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[40180237713] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40181377830] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40192020099] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[40193088243] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40194125862] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40195170147] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=374 ticks
[40196246508] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[40197332010] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[40199014317] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40200065235] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40201067973] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[40244571642] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[40246706874] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40247811945] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40258456260] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[40259555985] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40260616506] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40261656336] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=376 ticks
[40262738637] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[40263777741] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[40265774868] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40266783942] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40267773711] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[40310415222] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[40312288434] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40313450892] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40323345117] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[40324656966] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40326129855] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40327179189] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=378 ticks
[40328431242] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[40329618978] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[40331285379] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40332365601] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40333395531] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[40376348661] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[40378149801] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40379263815] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40389324657] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[40390373595] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40391435766] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40392788337] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=380 ticks
[40394056659] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[40395186282] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[40397189646] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40398224592] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40399220928] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[40442970810] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[40445540388] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40446800526] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40457984721] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[40459243209] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40460517702] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40461732630] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=382 ticks
[40462931916] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[40464097476] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[40466203437] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40467335304] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40468465257] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[40507529502] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[40510352421] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40511571639] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40524911229] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[40526227071] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40527433221] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40528548456] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=384 ticks
[40529827470] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[40530995043] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[40533992037] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40535139942] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40536291939] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[40574337474] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[40575476898] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40576400073] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40583941992] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[40584662943] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40585552689] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40586480286] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=386 ticks
[40587409038] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[40588208628] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[40589395968] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40590143187] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40590787347] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[40640699979] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[40642685061] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40643887119] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40654883346] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[40656018909] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40657081839] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40658173446] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=388 ticks
[40659361380] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[40660488858] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[40662456780] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40663608150] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40664731140] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[40706827887] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[40708574445] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40709830425] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40721224434] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[40722333696] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40723492524] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40724611290] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=390 ticks
[40725798993] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[40726983891] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[40728807108] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40729976166] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40731171129] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[40773208674] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[40775549331] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40776821646] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready:[40806864219] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[40808660376] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40809842931] [[ polling spec[0] kind=7 object=a token=2
[40787894400] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[4032789063722] [[32mINFO [0m] [kemINFO [0m] [kernel::syscall::handlers::wrnel::syscaaillt] [CPU0::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 ] collect_ready: polling spec[0] kind=7 object=a token=2
[40820876547] [[32mINFO [0m] [displtoken=1
[40790199978] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40791317061] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=392 ticks
[4079259742ay_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[40822078341] [[328]mINFO [0m] [kernel::sys [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=ca7 objell::cthandlers::wait] [CPU0]=8 f claollectgs_ready: polling=1 token=1
[40793826414] [[32mINFO [0m] [kernel::syscall::handlers:: spec[0] kind=7 object=8 token=1
[40823166186] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[40795722924] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40824261060] [[32mINFO [0m
[40796860104] [[32m] [kernINFO [0m] [kernel::syscall::handlers::wait] [Cel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 spPU0] colleecct_ready: polling spec[0] kins_ptd=r=107 00object0f8 count=2 timeout=393 ticks
[=a40825443285] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[40826572710] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[40828909308] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spe token=2
[40798004445] [[32mINFO c[0] kind=7 object=8 token=1
[40830017382] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40831122288] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[40905392814] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[40907254839] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40908538836] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40920319011] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[40921591854] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40922769360] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40923996399] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=396 ticks
[40925218257] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[40926435462] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[40928388666] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40929537264] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40930769649] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[40963233861] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[40964482878] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40965424929] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40979078547] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[40980106464] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40981161870] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40982037195] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=398 ticks
[40982795898] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[40983490746] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[40984740225] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[40985497476] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[40986132693] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[41037263025] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[41038503330] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41039208243] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41046550215] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[41047367031] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41048059239] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41048729238] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=400 ticks
[41049465930] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[41050143519] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[41051664060] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41052661188] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41053299309] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[41103513330] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[41105710833] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41106957969] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41118130680] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[41119418406] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41120587200] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41121667092] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=402 ticks
[41122831365] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[41123935248] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[41126306727] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41127454005] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41128571055] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[41161879605] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[41163868746] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41165247816] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41181932121] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[41183202192] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41184415404] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41185565916] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=404 ticks
[41186727516] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[41187839682] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[41189864991] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41191047810] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41192151231] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[41235688923] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[41237789076] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41239033704] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41250500610] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[41251703856] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41252830971] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41253955281] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=406 ticks
[41255108994] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[41256233832] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[41257955508] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41259114336] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41260215348] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[41301545934] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[41303372451] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41304604242] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41316459393] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[41317851234] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41318998116] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41320133085] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=408 ticks
[41321303694] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[41322444141] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[41325143409] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41326295373] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41327407407] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[41367551115] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[41369622624] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41370866658] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41382635976] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[41383873674] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41385059298] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41386202121] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=410 ticks
[41387381376] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[41388546408] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[41390627553] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41391843339] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41392973886] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[41433818811] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[41435255862] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41435974140] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41443539720] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[41444287170] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41445003402] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41445669705] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=412 ticks
[41446502889] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[41447629938] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[41448968550] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41449640430] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41450298450] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[41499656055] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[41501062350] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41501891937] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41508973044] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[41509945026] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41510949810] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41511910011] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=414 ticks
[41512976901] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[41513969343] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[41515037520] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41515697619] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41516330955] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[41566027635] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[41567993313] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41569282920] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41579946177] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[41581136553] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41582317953] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41583500310] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=416 ticks
[41584763418] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[41585963760] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[41587733187] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41588853240] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41590033386] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[41632693707] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[41634630345] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41635886787] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41646415404] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[41647682967] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41648858922] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41649959769] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=418 ticks
[41651143578] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[41652305838] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[41654200137] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41655361968] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41656476576] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[41698581309] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[41700972555] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41702453661] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41713439526] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[41714672274] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41715818100] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41717090481] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=420 ticks
[41718338574] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[41719426650] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[41721327516] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41722499412] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41723655864] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[41756097768] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[41758013451] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41759130435] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41773919682] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[41774642646] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41775372276] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41776141836] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=422 ticks
[41776850940] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[41777549319] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[41778562287] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41779202223] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41779828761] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[41820803013] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[41823144561] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41824454463] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41840830911] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[41842031583] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41843139459] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41844181533] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=424 ticks
[41845267893] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[41846345706] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[41848162290] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41849289834] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41850362235] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[41896410369] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[41898034266] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41899174911] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41909645151] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[41910684585] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41911709730] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41912748933] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=426 ticks
[41913836910] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[41914873440] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[41916588351] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41917662468] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41918683521] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[41962493628] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[41964150294] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41965285659] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41976131175] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[41977269015] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41978355837] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41979410418] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=428 ticks
[41980484799] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[41981551920] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[41983219938] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[41984258151] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[41985291546] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[42029211741] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[42031438944] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42032690469] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42043092003] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[42043851630] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42044576706] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42045220767] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=430 ticks
[42045937593] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[42046676991] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[42047914029] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42048561522] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42049243665] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[42094542666] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[42096436965] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42097592031] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42108441309] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[42109526415] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42110579610] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42111574956] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=432 ticks
[42112768203] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[42113895483] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[42116681310] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42117702198] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42118706685] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[42160568142] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[42162224940] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42163362846] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42174391017] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[42175498992] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42176577927] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42177597033] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=434 ticks
[42178704678] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[42180141993] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[42182663160] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42183689361] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42184698798] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[42227292459] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[42228815310] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42229539990] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42236700792] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[42237483750] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42238190841] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42238889352] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=436 ticks
[42239592978] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[42240271029] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[42241525755] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42242210241] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42242838792] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[42293206263] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[42294913650] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42296119173] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42303751875] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[42304508631] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42305276706] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42305942745] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=438 ticks
[42306640299] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[42307309770] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[42308496846] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42309171531] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42309836052] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[42359220189] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[42361173393] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42362402511] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42373285185] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[42374438997] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42375574428] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42376681545] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=440 ticks
[42377913303] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[42379037184] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[42381413481] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42382575873] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42383708202] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[42417187560] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[42418917057] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42420132645] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42433923774] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[42434789892] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42435461310] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42436156026] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=442 ticks
[42436876020] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[42437527209] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[42438869451] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42439587894] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42440250270] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[42493363737] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[42495136893] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42496346475] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42505572813] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[42506287956] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42506983266] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42507649536] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=444 ticks
[42508351215] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[42509064906] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[42510307125] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42510958644] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42511599405] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[42557045883] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[42558797226] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42560021427] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42571316337] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[42572494074] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42573668577] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42574821399] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=446 ticks
[42577318509] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[42578566932] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[42580830732] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42581916465] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42582953028] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[42623420169] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[42625865469] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42626997369] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42637037487] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[42638192487] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42639259509] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42640321251] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=448 ticks
[42641431503] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[42642515718] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[42644314713] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42645418563] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42646464564] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[42690157455] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[42692095083] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42693614238] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42703391808] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[42704447148] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42705485856] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42706513410] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=450 ticks
[42707571522] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[42708663030] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[42710356425] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42711372924] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42712365432] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[42755502504] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[42757317108] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42758429802] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42768790515] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[42769837935] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42770996763] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42772176744] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=452 ticks
[42773230599] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[42774353985] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[42776278743] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42777295374] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42778290753] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[42821647704] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[42823767558] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42825048354] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42835427646] [[32mINFO [0m] [[42855567876] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[42857892924] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42859140951] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42869462460] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[42870657093] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42871841397] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42872987784] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_dispmany: lay_virttid=io_gpu9 ] [CPU0] display_virtio_gpu: waiting on WaitSet...
[42836677323] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42837893802] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42839047350] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=454 ticks
[42840250002] [[32mINFOspecs_ptr=100 [0m] [k00f8 count=2 timeout=ernel::sysc45al5 ticks
[4287l::han41dlers::wait] [CPU09502]   spec[0]: kind=3]7  [obje[32mINFO ct=8[0m]  flags=1[k tokener=1
nel::syscal[4l:28:h4145an17dlers::wa63it] [CPU0]   spe] c[0]: [kind=7 object=8 flags=1 token=1
[4287[32mINFO [0m] [ker5398nel:104] [:sys[32mIcaNFll::O [0m] [kernel::syscall::hhandlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[42843585939] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: pandlers::waiolling spec[0] kind=7 object=8 token=1
[42844727112] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42845860134] [[32mINFO [0m] [ketrnel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[42877254354] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42878472549] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42879612402] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[42926820552] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[42928232820] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42929008848] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42936331416] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[42937393323] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42938398404] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42939266832] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=457 ticks
[42939993888] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[42940653228] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[42941769585] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42942495948] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[42943184790] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[42987065220] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[42989044626] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[42990289485] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43000909347] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[43002150609] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43003312704] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43004440215] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=459 ticks
[43005694149] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[43006869444] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[43008864723] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43010024508] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43011121329] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[43053368325] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[43055430792] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43056656478] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43067467575] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[43068702270] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43069806813] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43070912082] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=461 ticks
[43072116087] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[43073234325] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[43075829412] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43077023583] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43078168221] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[43119477918] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[43121502666] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43122714591] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43133325312] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[43134611487] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43135785891] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43136942244] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=463 ticks
[43138219377] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[43139391801] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[43141315536] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43142478753] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43143651144] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[43185135444] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[43187186196] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43188411684] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43199213640] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[43200451503] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43201532946] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43202644947] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=465 ticks
[43203926535] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[43205083713] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[43206889110] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43208016489] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43209118392] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[43251511611] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[43253563386] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43254790953] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43265605515] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[43266882747] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43267993956] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43269096420] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=467 ticks
[43270427211] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[43271608083] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[43273716453] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43274833404] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43275957153] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[43317236094] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[43319120922] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43319851443] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43327740027] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[43328466819] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43329157674] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43329799392] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=469 ticks
[43330617957] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[43331529186] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[43333334352] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43334008542] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43334661348] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[43383960114] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[43385984928] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43387209987] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43397669172] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[43398919641] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43399895946] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43400840010] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=471 ticks
[43402000653] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[43403149977] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[43405508850] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43406648670] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43407835944] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[43449976944] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[43451988096] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43453229787] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43463933304] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[43465137441] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43466105562] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43467207036] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=473 ticks
[43468382166] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[43469617785] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[43471608609] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43472764566] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43473934614] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[43515454026] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[43517339976] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43518550086] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43530209910] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[43531442559] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43532637126] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43533780411] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=475 ticks
[43534940559] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[43536075759] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[43537974018] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43539043020] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43540173996] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[43582296483] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[43584289617] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43585563879] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43596371742] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[43597575483] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43598695833] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43599830538] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=477 ticks
[43600984713] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[43602227097] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[43604223531] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43605329097] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43606398462] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[43647904410] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[43649672352] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43650926781] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43662346134] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[43663536708] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43664724840] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43665854232] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=479 ticks
[43667123148] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[43668297057] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[43670224488] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43671338964] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43672429746] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[43714346610] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[43716106302] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43717403565] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43728662373] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[43729873209] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43731108201] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43732348440] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=481 ticks
[43733435031] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[43734715233] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[43736599962] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43737783375] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43738915968] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[43771256694] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[43773208743] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43774474326] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43790130747] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[43790873379] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43791542685] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43792175361] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=483 ticks
[43792834503] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[43793521728] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[43794612642] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43795253040] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43795876014] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[43814306382] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[43816089504] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43817330601] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43827981846] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[43829052498] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43830203604] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43831329564] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=484 ticks
[43832478987] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[43833643095] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[43835511159] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43836677082] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43837814031] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[43902221847] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[43904019423] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43905288306] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43921988418] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[43923276243] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43924726824] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43925884596] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=487 ticks
[43927080912] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[43928242413] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[43930150473] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43931345106] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43932498621] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[43978720731] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[43980982254] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43982222691] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43992701511] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[43993945908] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[43995090216] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[43996205154] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=489 ticks
[43997414373] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[43998586830] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[44000412291] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44001527064] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44002625667] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[44053163319] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[44090516283] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44091847140] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44103841122] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[44105062749] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44106202404] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44107400436] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=492 ticks
[44110247478] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[44111380929] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[44118440586] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44119647990] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44120801604] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[44126389725] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[44128413582] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44129686755] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44159419524] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[44160841989] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44162099520] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44163212808] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=494 ticks
[44164368468] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[44165550396] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[44168336586] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44169424035] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44170463865] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[44193691443] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[44195058798] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44195810340] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44203586394] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[44204388228] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44205633417] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44216815764] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=495 ticks
[44218034190] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[44219156619] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[44221993299] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44223205554] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44224285479] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[44230847628] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[44232607782] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44233795914] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44263658472] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[44265088461] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44266328106] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44267527821] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=497 ticks
[44268832377] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[44270007705] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[44272474455] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44284695048] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44285926872] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[44299032294] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[44300798784] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44302052025] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44318884698] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[44320221858] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44321431671] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44322575847] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=499 ticks
[44323801071] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[44324978379] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[44326955607] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44328069126] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44329232970] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[44373977703] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[44375168310] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44375941896] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44383781013] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[44384543742] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44385268125] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44385910140] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=501 ticks
[44386617099] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[44387263899] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[44388475494] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44389119654] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44389798530] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[44440757856] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[44442494250] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44443759602] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44455968183] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[44457152157] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44458290360] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44459437242] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=503 ticks
[44460685071] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[44461825848] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[44463670845] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44464779249] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44465874981] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[44507273910] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[44509537710] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44510698716] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44520506052] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[44521682106] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44522730153] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44523764538] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=505 ticks
[44524858653] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[44525956497] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[44527812087] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44528837628] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44529854061] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[44572964997] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[44574746832] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44575922985] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44585725470] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[44586968976] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44588056326] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44589106089] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=507 ticks
[44590222281] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[44591448462] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[44593534953] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44594582901] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44595622962] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[44639443695] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[44641768974] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44643039210] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44652379497] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[44653716756] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44654875353] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44655982965] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=509 ticks
[44657173341] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[44658415197] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[44661126840] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44662263855] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44663536533] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[44697803667] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[44699459145] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44700267480] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44713824210] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[44714578491] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44715282975] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44715966042] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=511 ticks
[44716638648] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[44717294094] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[44718716823] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44719366791] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44720002173] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[44762788356] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[44765096244] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44766414396] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44785414872] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[44787113679] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44788657287] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44789905710] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=513 ticks
[44791178256] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[44792336589] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[44794329558] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44795531088] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44796632760] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[44828957085] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[44831113833] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44833012653] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44848566312] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[44849742465] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44850937494] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44852106387] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=515 ticks
[44853290064] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[44854841064] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[44857565181] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44858802285] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44859992034] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[44903332089] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[44905379904] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44906624070] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44917319997] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[44918537202] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44919696096] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44920792917] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=517 ticks
[44922047676] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[44923201026] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[44925356553] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44926481820] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44927583030] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[44960500365] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[44962211382] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44963410272] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44979054153] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[44980447479] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44981633136] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44982774309] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=519 ticks
[44983939803] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[44985113052] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[44987294682] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[44988400743] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[44989489710] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[45035819466] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[45038118543] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45039421779] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45050231094] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[45051470970] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45052651941] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45053810076] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=521 ticks
[45055039425] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[45056230791] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[45058606329] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45059932731] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45061435683] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[45092649393] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[45093914415] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45094757070] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45107748642] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[45108489954] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45109175001] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45109824705] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=523 ticks
[45110500479] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[45111178233] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[45112405734] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45113057946] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45113693658] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[45159655827] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[45161785614] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45163120530] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45179727813] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[45180984717] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45182145822] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45183290262] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=525 ticks
[45184495620] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[45185754636] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[45188166045] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45189292962] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45190444794] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[45233596155] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[45235530450] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45237000369] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45247536114] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[45248732958] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45249920364] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45251080644] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=527 ticks
[45252309102] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[45253508751] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[45256086810] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45257331768] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45258641802] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[45299999052] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[45302448576] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45303711123] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45314360058] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[45315649962] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45316835058] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45317994612] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=529 ticks
[45319318044] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[45320579898] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[45323041500] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45324287052] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45325820067] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[45354834888] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[45356902899] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45358143039] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45374061546] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[45375312609] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45376477707] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45377663892] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=531 ticks
[45378965676] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[45380175951] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[45382039923] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45383300358] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45384643755] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[45433043073] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[45434896419] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45435797880] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45442923636] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[45443641221] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45444298614] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45444939144] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=533 ticks
[45445607031] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[45446417775] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[45447623727] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45448299963] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45448961481] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[45497883255] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[45498962454] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45499729374] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45507030789] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[45507745767] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45508406394] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45509129985] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=535 ticks
[45509823546] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[45510565518] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[45511638909] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45512287458] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45512966532] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[45555330711] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[45556630581] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45557363511] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45571488369] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[45572806389] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45574023231] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45575191860] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=537 ticks
[45576357189] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[45577509846] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[45579408072] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45580547661] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45581698107] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[45630225696] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[45632290572] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45633420459] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45641744115] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[45642461799] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45643153677] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45643847073] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=539 ticks
[45644578914] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[45645262410] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[45646823145] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45647481627] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45648162714] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[45696547149] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[45698674857] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45699902292] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45710227200] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[45711489879] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45712610526] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45713745000] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=541 ticks
[45714922968] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[45716075064] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[45718304049] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45719419086] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45720585702] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[45762546390] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[45764602917] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45765481608] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45772916838] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[45773684418] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45774593469] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45775510869] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=543 ticks
[45776610957] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[45777519975] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[45779515716] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45780466545] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45781245477] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[45828628362] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[45830466660] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45831712971] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45842405598] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[45843592971] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45844912146] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45846445722] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=545 ticks
[45847915740] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[45849222672] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[45851850099] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45853041234] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45854176368] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[45894646908] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[45896509296] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45897766860] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45906504204] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[45907252479] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45907971780] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45908610792] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=547 ticks
[45909275709] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[45909918549] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[45911084505] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45911730777] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45912368601] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[45960036342] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[45961572591] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45962291496] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45970030062] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[45971125101] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45971913966] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45972571854] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=549 ticks
[45973253535] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[45973976961] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[45975228651] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[45975894624] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[45976540104] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[46026410892] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[46027520121] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46028252358] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46035251559] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[46035924891] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46036620003] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46037283864] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=551 ticks
[46037945184] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[46038640857] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[46040121501] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46040892018] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46041631746] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[46092793032] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[46094931135] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46096155996] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46106368176] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[46107553536] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46108727973] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46109865714] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=553 ticks
[46111023750] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[46112233134] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[46114350381] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46115516007] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46116599331] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[46159246386] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[46161076995] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46161816459] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46168695441] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[46169457906] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46170133812] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46170786948] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=555 ticks
[46171500078] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[46172150244] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[46173360222] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46174006098] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46174675404] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[46225249884] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[46226894307] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46228190085] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46239479748] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[46240695237] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46241859939] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46243006392] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=557 ticks
[46244251614] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[46245391863] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[46247322132] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46248458157] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46249584546] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[46290287043] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[46291888500] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46292596152] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46300211397] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[46300894431] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46301547369] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46302265845] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=559 ticks
[46302937791] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[46303631253] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[46304679696] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46305348144] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46306109322] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[46356352218] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[46357402047] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46358118213] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46365357984] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[46366209021] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46366932084] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46367875488] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=561 ticks
[46368556740] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[46369252974] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[46370483742] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46371127209] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46371790641] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[46391396238] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[46393396005] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46394670861] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46405225284] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[46406609931] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46407812385] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46408928577] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=562 ticks
[46409635767] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[46410504228] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[46412291145] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46412952201] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46413582204] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[46456425939] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[46458100854] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46459299282] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46470822123] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[46472028570] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46473226932] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46474383945] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=564 ticks
[46475532279] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[46476676521] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[46478908245] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46480036086] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46481143731] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[46512624444] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[46514913885] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46516118319] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46532632113] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[46533682107] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46534354185] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46534979205] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=566 ticks
[46535665176] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[46536329268] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[46537542513] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46538181921] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46538803443] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[46555689411] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[46556774352] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46557478308] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46566251853] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[46567425300] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46568527104] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46569589440] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=567 ticks
[46570734771] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[46571838093] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[46573519047] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46574663256] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46575725130] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[46621315950] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[46623004527] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46624198368] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46634186841] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[46635291681] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46636379064] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46637408862] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=569 ticks
[46638442224] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[46639459317] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[46641527955] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46642565178] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46643537787] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[46711296225] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[46713096540] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46714301139] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46731633861] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[46732935909] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46734107871] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46735262805] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=572 ticks
[46736470638] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[46737618708] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[46739632005] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46740755688] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46741920126] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[46810579497] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[46812483003] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46813713969] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46831578948] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[46832917230] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46834152057] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46835288511] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=575 ticks
[46836497598] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[46837668339] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[46839848451] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46841060079] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46842164853] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[46885927011] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[46887858633] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46888953639] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46898833773] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[46899553536] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46900209345] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46900837434] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=577 ticks
[46901560398] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[46902260196] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[46903470900] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46904135619] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46904784432] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[46950962817] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[46952552295] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46953774384] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46961734578] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[46962438006] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46963092198] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46963730385] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=579 ticks
[46964396820] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[46965040716] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[46966078995] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[46966722198] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[46967355402] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[47017519164] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[47019002811] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47019700596] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47026611060] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[47027301024] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47027988546] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47028694911] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=581 ticks
[47029388604] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[47030062035] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[47031228255] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47031931848] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47032570893] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[47073991272] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[47075511879] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47076784062] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47090937762] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[47092130415] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47093307228] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47094497208] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=583 ticks
[47095956699] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[47097171792] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[47098304583] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47098985439] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47099645175] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[47116943346] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[47118549654] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47119721220] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47126905188] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[47127570138] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47128226838] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47128856841] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=584 ticks
[47129633694] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[47130402297] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[47131401141] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47132119485] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47132763810] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[47174006748] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[47175558672] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47176651698] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47189128701] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[47189841435] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47190497343] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47191129623] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=586 ticks
[47191797246] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[47192464704] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[47193433617] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47194080054] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47194708242] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[47239429941] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[47240408193] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47241090039] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47254004523] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[47255181930] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47256321717] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47257471767] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=588 ticks
[47258689731] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[47259871758] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[47261603037] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47262724146] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47263832550] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[47315129532] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[47316603873] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47317800750] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47328936666] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[47330050350] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47331203568] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47332291710] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=590 ticks
[47333431728] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[47334583692] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[47336428821] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47337580488] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47338697703] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[47380823127] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[47383055313] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47384253972] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47395059888] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[47396166774] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47397340980] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47398491855] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=592 ticks
[47399730312] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[47401024968] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[47402728758] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47403710904] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47404344636] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[47446910247] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[47447978622] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47448693699] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47455822293] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[47456524929] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47457178857] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47457899280] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=594 ticks
[47458567959] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[47459241555] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[47460295509] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47460966729] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47461624650] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[47512957173] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[47514549687] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47515774911] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47525910531] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[47527062561] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47528203866] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47529376290] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=596 ticks
[47530540233] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[47531726814] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[47533524753] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47534649030] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47535813303] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[47579537379] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[47581336638] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47582653866] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47593360650] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[47594557791] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47595726453] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47596873236] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=598 ticks
[47598091002] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[47599256100] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[47602513266] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47603626191] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47604792081] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[47645631528] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[47647451808] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47648699538] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47659724277] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[47660487831] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47661147336] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47661776283] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=600 ticks
[47662437438] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[47663076681] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[47664165978] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47664805320] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47665428459] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[47712235395] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[47714504673] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47715736992] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47725556472] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[47726247624] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47726896470] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47727541884] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=602 ticks
[47728237623] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[47728871718] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[47729846241] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47730569139] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47731203432] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[47777766168] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[47779535166] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47780751447] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47791731174] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[47792885415] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47794017909] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47795095029] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=604 ticks
[47796265044] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[47797517724] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[47800377603] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47801511153] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47802692421] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[47844062079] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[47846106924] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47847346503] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47859852117] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[47861107701] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47862313620] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47863458720] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=606 ticks
[47864698431] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[47865871350] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[47867908539] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47869057962] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47870175276] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[47899858644] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[47901030474] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47901791586] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47914990068] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[47915702274] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47916382140] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47917150347] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=608 ticks
[47917905618] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[47918565717] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[47919776685] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47920450677] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47921090316] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[47966310018] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[47967943089] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47968724661] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47981990133] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[47982711744] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47983398606] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47984068704] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=610 ticks
[47984833578] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[47985512982] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[47986729593] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[47987440182] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[47988091437] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[48033109377] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[48034950810] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48036190719] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48051102264] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[48052370520] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48053536080] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48054673623] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=612 ticks
[48055880730] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[48057065892] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[48059018733] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48060161754] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48061296492] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[48108728745] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[48110658816] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48111921264] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48122326395] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[48123487269] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48124674444] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48125803803] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=614 ticks
[48126966657] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[48128120040] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[48130454394] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48131635299] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48132766407] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[48198618765] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[48200380503] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48201597312] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48217160640] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[48218389098] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48219695469] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48220862646] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=617 ticks
[48222026820] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[48223207659] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[48225132450] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48226336356] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48227448258] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[48273640206] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[48275463357] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48276680529] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48284760348] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[48285518160] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48286231191] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48286927260] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=619 ticks
[48287714211] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[48288416187] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[48289923726] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48290574684] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48291206865] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[48338834874] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[48340447221] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48341740524] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48349249872] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[48349939209] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48350587164] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48351214197] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=621 ticks
[48351893205] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[48352531953] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[48353716224] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48354377313] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48355057872] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[48405037131] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[48406647366] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48407910507] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48415386591] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[48416116683] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48416774241] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48417398535] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=623 ticks
[48418103415] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[48418800408] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[48419872512] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48420537891] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48421206273] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[48471457287] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[48473052408] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48473954628] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48480739527] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[48481494435] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48482185224] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48482822685] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=625 ticks
[48483516609] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[48484223601] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[48485394342] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48486031671] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48486727773] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[48537767157] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[48539512758] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48540340332] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48547042335] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[48548097675] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48548794899] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48549431667] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=627 ticks
[48550115064] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[48550774437] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[48552022761] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48552692463] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48553338801] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[48594488514] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[48596395551] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48597619389] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48612653826] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[48613839582] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48615006726] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48616179909] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=629 ticks
[48617378601] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[48618554688] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[48620640849] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48621788919] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48622958373] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[48670014030] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[48672028317] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48673262418] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48683627586] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[48684819414] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48685968210] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48687102453] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=631 ticks
[48688333089] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[48689510298] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[48691681434] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48692341566] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48692967873] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[48735978753] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[48738021948] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48739231398] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48749498589] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[48750673455] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48751812615] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48752931678] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=633 ticks
[48754091661] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[48755047605] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[48756356121] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48757129113] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48757826271] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[48802028418] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[48803065443] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48803792367] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48811353723] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[48812231622] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48812903832] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48813588120] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=635 ticks
[48814251750] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[48814891686] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[48816081402] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48816777306] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48817474563] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[48858417564] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[48859661400] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48860398818] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48873288156] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[48873988581] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48874653597] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48875279046] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=637 ticks
[48876324882] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[48877271322] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[48878391441] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48879076884] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48879719460] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[48925382979] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[48927684432] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48928961103] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48944962341] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[48946433019] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48947586930] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48948750411] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=639 ticks
[48949930326] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[48951073215] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[48953190891] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[48954317247] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[48955418391] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[49000355184] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[49002511866] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49003400424] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49010916438] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[49011619206] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49012271319] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49012905084] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=641 ticks
[49013657418] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[49014595410] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[49015666260] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49016317515] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49016940159] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[49058979255] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[49061005554] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49062332616] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49079056026] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[49080322269] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49081475586] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49082628507] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=643 ticks
[49083847230] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[49085025627] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[49087006221] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49088145150] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49089264774] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[49132485633] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[49134247866] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49135457283] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49146249537] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[49147455291] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49148605341] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49149739617] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=645 ticks
[49150874784] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[49152085785] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[49153922961] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49155022752] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49156175079] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[49199099037] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[49201094415] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49202318781] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49209605973] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[49210441038] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49211106648] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49211784072] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=647 ticks
[49212446085] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[49213081830] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[49214048730] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49214692923] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49215341505] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[49288625661] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[49290435942] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49291653048] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49306982406] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[49308265776] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49309444932] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49310585016] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=650 ticks
[49311751005] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[49312928280] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[49314816573] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49315956789] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49317094134] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[49363943442] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[49365692178] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49366930140] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49377521820] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[49378682397] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49379821656] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49380951774] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=652 ticks
[49382139378] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[49383299691] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[49385807229] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49386945267] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49388071161] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[49430195991] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[49431977232] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49433204304] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49443716421] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[49444899735] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49446057342] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49447162710] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=654 ticks
[49448313981] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[49449486867] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[49452365622] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49453480989] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49454571111] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[49496235426] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[49497997098] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49499218428] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49509455589] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[49510591581] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49511735790] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49512852774] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=656 ticks
[49513998303] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[49515243096] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[49517434857] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49518330609] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49518976848] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[49561717920] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[49562912850] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49563709635] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49570506480] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[49571249178] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49571982405] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49572737478] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=658 ticks
[49573423053] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[49574106351] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[49575190533] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49575844065] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49576515120] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[49618731063] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[49620668625] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49621838376] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49637854728] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[49639089126] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49640271219] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49641423117] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=660 ticks
[49642580526] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[49643777436] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[49645780008] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49647047340] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49648186005] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[49695165234] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[49697115831] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49698310728] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49708616628] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[49709778063] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49710936099] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49712068098] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=662 ticks
[49713269661] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[49714365327] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[49717582761] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49718395914] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49719063933] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[49760216649] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[49762109793] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49763346369] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49773294648] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[49774067541] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49774780836] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49775494923] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=664 ticks
[49776228282] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[49776939498] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[49778276625] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49779003153] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49779633156] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[49793388348] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[49794363696] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49795088442] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49802375337] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[49803113481] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49803780477] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49804464369] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=665 ticks
[49805153079] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[49805796645] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[49806991047] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49807632864] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49808362857] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[49848279228] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[49849446504] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49850172966] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49863838101] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[49864628484] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49865436291] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49866110613] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=667 ticks
[49866790314] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[49867476384] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[49868492487] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49869139254] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49869769917] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[49916642160] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[49919074062] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49920341262] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49937281416] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[49938586797] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49939791825] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49940920194] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=669 ticks
[49942090176] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[49943294280] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[49945500957] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49946670510] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[49947815544] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[49991268228] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[49992715641] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[49993464576] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50000447145] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[50001147174] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50001882843] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50002559508] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=671 ticks
[50003229276] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[50003987187] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[50005292205] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50006021043] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50006670615] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[50057851668] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[50059903509] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50060731347] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50067689925] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[50068444305] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50069103216] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50069738136] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=673 ticks
[50070401106] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[50071044111] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[50072238513] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50072880594] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50073609564] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[50123074881] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[50124187146] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50124913146] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50132401572] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[50133298380] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50134026558] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50134725795] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=675 ticks
[50135413779] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[50136109815] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[50137222476] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50137953360] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50138612106] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[50157084285] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[50158321719] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50159359173] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50166495291] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[50167224558] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50167888782] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50168531094] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=676 ticks
[50169206571] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[50169961941] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[50170963095] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50171605770] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50172267123] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[50215239228] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[50216944338] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50218173720] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50236056783] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[50237316459] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50238476079] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50239617483] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=678 ticks
[50240795220] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[50241991206] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[50243711859] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50244898374] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50246045025] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[50288890905] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[50291250702] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50292536844] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50303556435] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[50304699984] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50305892076] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50307056976] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=680 ticks
[50308274874] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[50309474259] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[50311149768] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50312306121] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50313402282] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[50354740062] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[50356283868] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50357434380] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50368285407] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[50369397837] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50370536700] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50371707012] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=682 ticks
[50372880063] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[50374037274] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[50375685261] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50376810033] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50377928040] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[50422618884] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[50424584892] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50425828431] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50433915444] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[50434679361] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50435380314] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50436046782] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=684 ticks
[50437129974] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[50438303058] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[50440452414] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50441572995] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50442671301] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[50487887538] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[50490207306] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50491451109] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50502234123] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[50503483239] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50504622663] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50505710607] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=686 ticks
[50506911147] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[50508116868] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[50510002785] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50511110892] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50512206558] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[50553707292] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[50555634723] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50556829851] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50567909964] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[50569160301] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50570278275] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50571406512] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=688 ticks
[50572650843] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[50573801619] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[50575789572] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50576882499] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50577980772] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[50619587601] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[50621640003] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50622878163] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50633563860] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[50634789315] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50635887621] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50636945997] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=690 ticks
[50638204881] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[50639373642] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[50641321104] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50642408454] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50643499467] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[50685605916] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[50687444280] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50688193248] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50695484961] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[50696182185] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50696847927] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50697654348] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=692 ticks
[50698335765] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[50698983258] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[50701462977] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50702124033] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50702755191] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[50751707094] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[50753890836] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50755103685] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50765690679] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[50767022262] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50768168220] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50769237420] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=694 ticks
[50770464591] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[50771617578] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[50773761357] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50774880255] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50775953085] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[50817672642] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[50819043231] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50819757285] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50827104999] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[50828061306] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50828940195] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50829594453] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=696 ticks
[50830260129] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[50830896831] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[50831937420] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50832601149] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50833291971] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[50850768111] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[50851884699] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50852575158] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50859977586] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[50860824828] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50861555547] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50862292899] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=697 ticks
[50863209870] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[50863995204] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[50865443607] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50866302861] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50867042292] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[50884458603] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[50885568624] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50886296340] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50894058468] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[50894919108] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50895700152] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50896347315] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=698 ticks
[50897059422] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[50897720940] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[50899310550] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50899964775] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50900683482] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[50948672379] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[50950649772] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50951626671] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50958840702] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[50959649730] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50960516673] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50961247623] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=700 ticks
[50961944517] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[50962652928] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[50964136938] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[50965039422] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[50965824987] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[51016178796] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[51018007524] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51019286142] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51027176145] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[51027864261] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51028628838] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51029329395] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=702 ticks
[51030440439] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[51031534257] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[51033069945] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51033726183] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51034390275] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[51082542357] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[51084380193] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51085527471] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51095991606] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[51097250787] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51098423541] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51099513663] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=704 ticks
[51100970844] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[51102039384] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[51103846827] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51104907876] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51105990309] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[51148075143] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[51149816850] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51150943899] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51160444137] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[51161148027] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51161838651] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51162507957] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=706 ticks
[51163218777] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[51163974939] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[51165191055] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51165945798] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51166594776] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[51214232025] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[51215937201] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51217136685] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51228060312] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[51229202475] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51230314443] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51231524883] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=708 ticks
[51232689090] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[51233898375] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[51235651698] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51236758683] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51237892827] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[51280564170] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[51282237237] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51283456323] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51293731401] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[51294871782] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51295979262] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51297087138] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=710 ticks
[51298240422] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[51299429313] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[51301293417] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51302410962] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51303536823] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[51347595090] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[51349812558] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51350962905] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51358472517] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[51359180136] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51359855646] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51360517362] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=712 ticks
[51361218381] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[51361867623] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[51362911347] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51363553164] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51364309029] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[51412039569] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[51413159556] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51413912913] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51421293066] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[51422037744] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51422708667] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51423349230] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=714 ticks
[51424019856] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[51424674741] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[51425897391] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51426551352] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51427183368] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[51468542268] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[51469903419] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51470795211] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51486260892] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[51487666098] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51488455161] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51489110541] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=716 ticks
[51489788724] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[51490629102] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[51492370776] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51493298670] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51493955667] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[51545096031] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[51547625118] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51548834469] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51558986160] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[51560153007] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51561282003] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51562433835] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=718 ticks
[51563636388] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[51564808812] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[51566474586] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51567597378] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51568674795] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[51611013234] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[51612761871] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51613478532] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51620490933] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[51621154365] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51621813441] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51622456974] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=720 ticks
[51623233101] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[51623893167] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[51624949530] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51625619001] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51626288010] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[51667522104] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[51668699181] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51669448512] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51682169055] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[51682981581] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51683674449] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51684430281] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=722 ticks
[51685174662] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[51685934322] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[51687108561] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51687782949] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51688428099] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[51734267310] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[51735756501] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51736468608] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51750174465] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[51750901587] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51751687680] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51752331180] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=724 ticks
[51753001047] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[51753669594] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[51754897491] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51755575641] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51756231912] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[51777382635] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[51779554695] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51780820278] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51791930520] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[51793065555] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51794205606] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51795311667] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=725 ticks
[51796493298] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[51797671860] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[51799573419] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51800673309] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51801723567] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[51833376342] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[51835100790] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51836273577] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51854215017] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[51855158322] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51855826836] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51856707573] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=727 ticks
[51857447994] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[51858397668] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[51859677903] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51860379945] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51861114921] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[51899027334] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[51900743796] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51901452273] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51914231160] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[51914977323] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51915637422] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51916321578] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=729 ticks
[51917013885] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[51917697843] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[51918692232] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51919358535] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51920013849] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[51965066076] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[51966692646] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51967537743] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51981909705] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[51982640787] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51983337021] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51984015072] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=731 ticks
[51984717576] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[51985426515] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[51986476542] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[51987156474] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[51987891681] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[52031603448] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[52033329348] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52034533683] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52051685037] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[52052926365] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52054087998] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52055263557] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=733 ticks
[52056469443] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[52057678926] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[52059403407] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52060582200] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52061722548] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[52106418408] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[52108262118] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52109465496] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52120055031] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[52121225343] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52122296094] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52123288866] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=735 ticks
[52124350872] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[52125438222] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[52127433534] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52128437526] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52129445346] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[52172875491] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[52174214697] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52174922085] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52182273825] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[52183080246] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52183818225] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52184479116] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=737 ticks
[52185172050] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[52185957945] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[52187106477] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52187830596] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52188562041] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[52228989054] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[52230187350] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52230965655] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52244707680] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[52245644880] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52246340355] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52247003820] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=739 ticks
[52247821890] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[52248594288] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[52250272866] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52251370941] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52252470138] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[52295500785] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[52297094421] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52298206851] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52314180864] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[52315548615] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52316714439] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52317877953] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=741 ticks
[52319052885] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[52320191220] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[52322170857] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52323314439] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52324484619] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[52395421419] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[52397692611] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52398961560] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52416338304] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[52417520397] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52418243064] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52418988336] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=744 ticks
[52419786078] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[52420537653] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[52422005262] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52422705225] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52423396080] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[52469953965] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[52471671483] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52472912877] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52482967680] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[52483921281] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52484643684] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52485480861] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=746 ticks
[52486282002] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[52487124096] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[52488319983] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52488998694] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52489631370] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[52503695706] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[52504797345] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52505924097] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52513928379] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[52514815749] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52515585936] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52516313454] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=747 ticks
[52517304741] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[52518198579] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[52519525278] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52520254182] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52520991501] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[52553937645] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[52555393011] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52556429673] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52565285487] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[52573839483] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52575143148] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52576290855] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=749 ticks
[52577470968] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[52578671541] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[52580678403] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52581832545] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52582928541] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[52603152294] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[52605354483] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52606761306] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52618524783] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[52619668035] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52620890487] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52622123928] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=750 ticks
[52623449142] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[52624636317] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[52626817683] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52628025186] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52629404685] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[52658405613] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[52660280475] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52661478870] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52678796280] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[52680055197] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52681218117] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52682324772] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=752 ticks
[52683537291] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[52684663944] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[52686671301] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52687859433] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52688980806] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[52734297264] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[52735391478] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52736137410] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52743081501] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[52743766350] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52744414239] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52745051502] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=754 ticks
[52745724306] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[52746389685] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[52747403214] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52748060541] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52748763540] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[52767383625] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[52768529286] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52769280498] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52776780771] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[52777456677] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52778117106] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52778758065] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=755 ticks
[52779463473] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[52780117005] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[52781250093] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52781897322] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52782554220] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[52823873124] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[52825452405] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52826163687] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52838815788] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[52839523902] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52840184001] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52840874130] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=757 ticks
[52841878452] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[52842538353] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[52843557624] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52844206503] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52844983884] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[52867601358] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[52869552483] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52870741506] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52881622827] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[52882739283] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52883850789] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52884953814] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=758 ticks
[52886103765] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[52887265530] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[52889035320] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52890134814] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52891237377] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[52932477411] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[52934398638] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52935596670] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52945697013] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[52946598969] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52947292827] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52948327344] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=760 ticks
[52949537619] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[52950754296] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[52952064627] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[52952744031] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[52953501975] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[52998546150] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[53000236410] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53001433353] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53012607846] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[53013811620] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53014926624] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53016059580] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=762 ticks
[53017216164] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[53018326977] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[53020385451] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53021539197] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53022671823] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[53064847539] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[53066752464] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53067974520] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53075234256] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[53075976426] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53076659691] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53077415028] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=764 ticks
[53078116212] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[53078893461] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[53080083012] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53080726116] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53081353743] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[53130546876] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[53133027519] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53133811830] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53142238545] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[53143369884] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53144540658] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53145618999] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=766 ticks
[53146754067] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[53147881116] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[53149556196] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53150698062] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53151793926] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[53197217535] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[53199083025] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53200299042] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53210528349] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[53211677739] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53212795482] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53213952858] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=768 ticks
[53215105911] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[53216308893] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[53218161084] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53219271666] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53220436929] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[53262223839] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[53263342539] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53264058177] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53271071997] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[53271748101] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53272408266] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53273086746] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=770 ticks
[53273750574] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[53274392787] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[53275401003] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53276060442] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53276713248] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[53318857713] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[53320491444] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53321682744] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53332873308] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[53333599440] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53334290130] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53334985803] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=772 ticks
[53335817997] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[53336463609] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[53337844692] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53338613427] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53339265309] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[53386461348] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[53388959250] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53390202657] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53407611972] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[53408898873] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53410071495] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53411220555] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=774 ticks
[53412426771] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[53413563654] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[53415469008] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53416601073] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53417789832] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[53461235058] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[53464177635] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53465412693] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53476538313] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[53477732286] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53478899826] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53480041428] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=776 ticks
[53481228735] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[53482396803] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[53484352350] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53485603776] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53487137286] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[53527257300] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[53529463416] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53531063652] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53546910879] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[53548053339] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53548936122] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53549623974] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=778 ticks
[53550401355] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[53551091451] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[53552175369] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53552819199] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53553483423] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[53593317195] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[53595301617] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53596531824] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53606889336] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[53608057272] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53609168415] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53610262629] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=780 ticks
[53611424229] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[53612570022] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[53614348986] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53615499069] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53616662286] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[53659277628] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[53660516712] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53661254724] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53668596036] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[53669274615] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53669995269] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53670639528] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=782 ticks
[53671308570] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[53672012460] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[53673133338] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53673777828] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53674406412] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[53692973103] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[53694284127] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53695303365] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53703365166] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[53704657314] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53705870823] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53707006122] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=783 ticks
[53707772679] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[53708593752] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[53710028427] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53710930911] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53711627211] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[53759820213] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[53761549644] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53762749128] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53774033016] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[53775313383] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53776431621] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53777680209] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=785 ticks
[53779290081] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[53780677599] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[53782362711] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53783525565] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53784633342] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[53815138674] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[53816825304] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53818055082] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53835443805] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[53836739187] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53837893164] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53839012425] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=787 ticks
[53840214978] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[53841115977] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[53843011629] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53844137061] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53844924342] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[53891287296] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[53893103748] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53893805559] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53900544687] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[53901221385] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53901944679] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53902700709] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=789 ticks
[53903375064] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[53904023976] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[53905116177] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53905827162] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53906451819] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[53956407615] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[53958370884] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53959611057] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53966617254] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[53967282402] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53967988305] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53968620783] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=791 ticks
[53969285139] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[53969925405] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[53970894483] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[53971560357] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[53972192670] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[54023069166] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[54024226146] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54024941091] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54032353551] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[54033022758] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54033674607] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54034338138] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=793 ticks
[54035001141] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[54035639889] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[54037077369] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54037720803] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54038341995] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[54089333793] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[54091209348] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54092442558] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54103661304] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[54104840790] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54106047369] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54107245962] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=795 ticks
[54108430827] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[54109624437] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[54111622752] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54112758216] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54113926020] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[54144809862] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[54146693304] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54147908661] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54164434302] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[54165790569] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54166991142] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54168153006] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=797 ticks
[54169359816] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[54170494521] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[54172476930] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54173594541] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54174705750] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[54221105136] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[54222972375] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54224205849] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54235543362] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[54236774922] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54237452808] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54238152441] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=799 ticks
[54238829139] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[54239507256] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[54240539661] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54241184448] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54241919160] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[54288091605] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[54289355406] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54290088336] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54298236366] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[54298992858] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54299658930] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54300495579] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=801 ticks
[54301428324] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[54302179107] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[54303386445] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54304087497] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54304754394] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[54354053490] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[54356354019] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54357120642] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54364355694] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[54365040114] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54365737404] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54366493566] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=803 ticks
[54367245141] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[54368006154] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[54369275763] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54370013907] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54370662786] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[54419089692] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[54420302970] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54421009632] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54428611611] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[54429325797] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54429979890] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54430615074] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=805 ticks
[54431278902] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[54431962794] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[54433203594] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54433905603] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54434555736] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[54452730255] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[54453911688] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54454706229] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54462579534] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[54463271346] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54464059617] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54464733114] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=806 ticks
[54465579498] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[54466321305] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[54467733573] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54468386148] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54469059480] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[54518937330] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[54521290395] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54522483675] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54533122842] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[54534239001] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54534991335] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54535630677] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=808 ticks
[54536297970] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[54536943417] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[54538190124] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54538876590] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54539556753] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[54584548557] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[54586463283] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54587729625] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54598140465] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[54599438553] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token[54618658479] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[54620472852] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54621600330] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54632090568] [[32mI=1
[54600591606] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54601716576] [[32mINFO [NFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[54633241674] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54634350309] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec0m] [kernel::sy[0sc] kind=7 object=a tokealln=::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=810 ticks
[54602883786] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[54604014102] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[54606215697] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] colle2
[54635465346] [[32mct_ready: polling spec[0] kind=7 object=8 token=1
[54607400298] [[32mIINFO [0mNFO [0m] [kernel::s] [keryscanel::syscall::handlers::wait]ll: [CP:haU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=811 ticks
[54636630708] [[32mINFO [0m] [kernel::syscalndlersl::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[54637837980] [[32mINFO ::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 objecttoken=2
[54608511540] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
=a flags=1 token=2
[54639505371] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54640647336] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54641765409] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[54684116256] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[54686134998] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54686915976] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54694857162] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[54695567124] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54696294576] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54696955071] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=813 ticks
[54697646487] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[54698315001] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[54699368922] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54700034466] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54700683213] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[54749775465] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[54751708143] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54752925414] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54763646883] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[54765096936] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54766314009] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54767463663] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=815 ticks
[54768734889] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[54769868439] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[54771844050] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54772981098] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54774082869] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[54840029673] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[54841847676] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54843056235] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54858505482] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[54859786542] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54860798223] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54861871680] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=818 ticks
[54863074365] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[54864278436] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[54866647473] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54867868638] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54868950081] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[54914993463] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[54916793019] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54917972142] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54928657872] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[54929865375] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54930988629] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54932122113] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=820 ticks
[54933310905] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[54934469733] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[54936580446] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54937682481] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54938766663] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[54981649305] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[54983745300] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54984937590] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54992778291] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[54993516237] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54994239828] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54994942563] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=822 ticks
[54995697570] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[54996351630] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[54997550982] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[54998199069] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[54998852964] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[55047453945] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[55048712400] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55049615082] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55058421627] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[55059198183] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55059927483] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55060568046] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=824 ticks
[55061242434] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[55061933454] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[55062935367] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55063577382] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55064212071] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[55137293805] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[55139270406] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55140497181] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55156203861] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[55157482710] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55158627480] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55159775616] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=827 ticks
[55161002754] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[55162183032] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[55164094524] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55165269390] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55166406042] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[55212249180] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[55214408568] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55215610593] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55226079909] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[55227328959] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55228466403] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55229621469] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=829 ticks
[55230745251] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[55231967835] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[55234000569] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55235170287] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55236361422] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[55278714645] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[55280513409] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55281733881] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55292310744] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[55293523626] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55294652688] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55295815674] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=831 ticks
[55296980343] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[55298089506] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[55300526523] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55301634498] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55302742572] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[55344511596] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[55346206080] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55347388833] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55357302726] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[55358036778] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55358710176] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55359384696] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=833 ticks
[55360054827] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[55360700637] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[55361776041] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55362450858] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55363150359] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[55410645114] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[55412476152] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55413685008] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55425606984] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[55426808415] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55428021726] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55429138050] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=835 ticks
[55430308131] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[55431503952] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[55433376009] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55434495600] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55435576911] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[55478545287] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[55481438034] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55482688074] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55493355225] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[55494509400] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55495625262] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55496774124] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=837 ticks
[55497982287] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[55499089107] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[55500853617] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55501956675] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55503068511] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[55542983232] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[55544934720] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55546243830] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55556897847] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[55558217814] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55559696082] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55560823527] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=839 ticks
[55562131614] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[55563277968] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[55565380365] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55566542229] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55567656078] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[55609082760] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[55611062265] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55612344051] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55622928306] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[55624156665] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55625317770] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55626441750] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=841 ticks
[55627671726] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[55628853522] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[55630862826] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55632006144] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55633173255] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[55674951255] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[55677007122] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55677969336] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55684950288] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[55685656257] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55686332031] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55686993252] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=843 ticks
[55687711365] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[55688383542] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[55689584346] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55690252299] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55690929162] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[55708176018] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[55709217960] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55709942112] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55717525743] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[55718208480] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55718869998] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55719579927] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=844 ticks
[55720292859] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[55720938735] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[55722092151] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55722729876] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55723559826] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[55765116198] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[55767021222] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55768252254] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55782249831] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[55782987249] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55783754037] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55784395788] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=846 ticks
[55785095916] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[55785748260] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[55787171880] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55787844354] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55788496632] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[55830724125] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[55831930143] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55832723727] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55846619070] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[55847789217] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55849002033] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55850190858] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=848 ticks
[55851393147] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[55852577088] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[55854502473] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55855366182] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55856006283] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[55895368650] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[55897147779] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55898387523] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55913436546] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[55914591744] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55915693845] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55916768160] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=850 ticks
[55917905406] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[55918952001] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[55920911838] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55922017668] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55923078057] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[55972841199] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[55974843969] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55976055564] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55987074132] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[55988858277] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55989965394] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55991235267] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=852 ticks
[55991974038] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[55992731223] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[55996352544] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[55997292483] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[55998223809] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[56038401243] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[56040191559] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56041406289] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56053126206] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[56054523789] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56056125774] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56057520981] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=854 ticks
[56058666972] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[56059818837] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[56061653505] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56062762602] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56063958555] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[56099708544] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[56103651747] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56104431240] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56112750111] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[56113831455] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56114533332] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56115186996] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=856 ticks
[56115908838] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[56116620615] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[56117939823] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56118599988] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56119285629] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[56139223305] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[56141249439] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56142474234] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56150848083] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[56151533262] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56152240881] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56152961469] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=857 ticks
[56153704926] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[56154352584] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[56155335093] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56155978824] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56156604900] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[56203996860] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[56205311712] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56206051473] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56213133570] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[56213909994] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56214589926] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56215217256] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=859 ticks
[56215881348] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[56216519733] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[56217528048] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56218194219] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56219133762] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[56260869324] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[56262664920] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56263917105] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56280460401] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[56281665957] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56282841384] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56284007736] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=861 ticks
[56285222961] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[56286366081] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[56287660935] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56288676411] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56289636612] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[56335746852] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[56337430743] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56338627554] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56350565337] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[56351682321] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56352821019] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56353901208] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=863 ticks
[56355106071] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[56356312914] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[56359286379] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56360424417] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56361513648] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[56401254360] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[56402346561] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56403079689] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56410476144] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[56411142975] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56411890458] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56412917649] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=865 ticks
[56413606095] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[56414408523] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[56415656847] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56416398984] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56417031198] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[56467185720] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[56468772987] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56469540039] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56476373613] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[56477035230] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56477724105] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56478401067] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=867 ticks
[56479092450] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[56479762647] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[56481411393] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56482063407] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56482790133] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[56525034786] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[56527006833] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56528249052] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56542742322] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[56543985168] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56545160364] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56546313582] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=869 ticks
[56547572598] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[56548730172] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[56550655590] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56551814682] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56552960541] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[56591263113] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[56592933045] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56594203809] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56611476933] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[56612683809] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56613866958] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56614983678] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=871 ticks
[56616209298] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[56617337205] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[56619266880] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56620446762] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56621576088] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[56666285676] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[56668268415] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56669501196] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56679691068] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[56680861050] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56681997867] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56683138248] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=873 ticks
[56684302785] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[56685531177] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[56687356440] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56688476559] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56689602750] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[56732202648] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[56734437144] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56735419092] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56743292100] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[56744088390] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56744783568] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56745429840] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=875 ticks
[56746119573] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[56746809537] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[56748012981] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56748723735] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56749386309] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[56799742230] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[56801693949] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56802908118] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56812477128] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[56813179731] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56813890881] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56814527715] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=877 ticks
[56815191741] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[56815834482] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[56816891934] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56817577080] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56818237806] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[56864477274] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[56866388172] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56867614749] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56878087530] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[56879279325] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56880438681] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56881550847] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=879 ticks
[56882711094] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[56883896586] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[56886940209] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56888042442] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56889143718] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[56929803249] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[56930844465] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56931741669] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56939194158] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[56939905902] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56940582501] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56941239135] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=881 ticks
[56941926624] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[56942706348] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[56943942231] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56944592364] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[56945217813] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[56986756365] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[56988234006] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[56989693035] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57007522176] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[57008899464] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57010088784] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57011255862] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=883 ticks
[57012462969] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[57013639056] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[57015609915] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57016751385] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57017858766] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[57063007551] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[57064804038] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57065552973] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57073056282] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[57073830462] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57074528577] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57075166797] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=885 ticks
[57075835905] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[57076550487] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[57077609754] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57078267642] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57078903024] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[57128743650] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[57130524693] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57131260164] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57138514455] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[57139184091] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57139981008] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57140899662] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=887 ticks
[57141605004] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[57142282428] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[57143980641] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57144684366] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57145351989] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[57195075366] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[57197250561] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57198502383] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57207881907] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[57208578801] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57209265432] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57209911440] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=889 ticks
[57210581703] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[57211234410] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[57212431518] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57213589389] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57214716702] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[57261034644] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[57262426386] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57263153343] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57270449082] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[57271125450] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57271811685] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57272451852] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=891 ticks
[57273121587] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[57273765615] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[57275080632] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57275727696] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57276358953] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[57326450082] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[57327600330] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57328333029] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57335510628] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[57336187887] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57336844191] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57337488714] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=893 ticks
[57338182308] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[57338838678] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[57339969720] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57340614771] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57341279787] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[57383547309] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[57385867671] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57387053328] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57400180926] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[57400916925] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57401938506] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57403092054] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=895 ticks
[57404310579] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[57405470166] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[57407128581] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57407924838] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57408635460] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[57427036260] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[57428783115] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57429616695] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57437133930] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[57437853330] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57438576261] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57439277379] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=896 ticks
[57440019120] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[57440687568] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[57441894543] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57442579161] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57443334795] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[57492381375] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[57494331741] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57495577095] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57506866164] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[57508108350] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57509283249] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57510397395] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=898 ticks
[57511352877] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[57512001756] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[57513061683] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57513763626] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57514426893] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[57558574161] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[57560532843] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57561890364] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57573214017] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[57574344828] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57575494614] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57576611334] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=900 ticks
[57577731618] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[57578933907] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[57580747686] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57581914137] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57582992181] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[57624479682] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[57626495685] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57627752391] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57638439012] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[57639623679] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57640813758] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57641948595] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=902 ticks
[57643185105] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[57644390034] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[57646583115] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57647747157] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57648890640] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[57690403584] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[57692128725] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57693382725] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57704126040] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[57705322884] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57706537944] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57707690271] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=904 ticks
[57708866094] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[57710019477] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[57712131444] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57713337594] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57714413328] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[57756858819] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[57758851887] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57760114665] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57770467722] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[57771660210] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57772783827] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57773932854] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=906 ticks
[57775099602] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[57776224605] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[57778510185] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57779652513] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57780771972] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[57822726456] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[57825248448] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57826459944] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57837002817] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[57838171446] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57839312718] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57840433860] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=908 ticks
[57841680072] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[57842816163] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[57844773756] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57845872722] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57846522789] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[57888629271] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[57889712397] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57890503341] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57898349289] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[57899042388] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57899708328] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57900358560] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=910 ticks
[57901106571] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[57901765086] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[57902803464] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57903453795] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57904169664] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[57922468857] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[57923784600] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57924532677] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57931303584] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[57932043015] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57932732583] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57933400998] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=911 ticks
[57934133400] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[57934814520] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[57936050304] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57936832305] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57937525404] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[57980130087] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[57981355344] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57982062798] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57997353909] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[57998102217] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[57998789871] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[57999445218] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=913 ticks
[58000255302] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[58000978959] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[58002413667] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58003121319] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58003814682] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[58043710791] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[58045670100] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58046877735] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58061116113] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[58061966820] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58062680049] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58063351005] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=915 ticks
[58064026977] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[58064719581] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[58065954771] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58066609491] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58067256093] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[58121267028] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[58123098099] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58124318868] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58134891606] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[58136077758] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58137218238] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58138384986] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=917 ticks
[58139616414] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[58140755343] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[58142653107] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58143873414] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58145008086] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[58186041903] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[58187957223] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58189181259] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58199765349] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[58200504021] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58201232034] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58201914804] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=919 ticks
[58202583747] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[58203223980] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[58204472601] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58205166426] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58205799465] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[58252352466] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[58253694510] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58254431433] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58261711200] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[58262387667] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58263045951] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58263947082] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=921 ticks
[58264926159] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[58265862963] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[58266983775] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58267629222] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58268458941] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[58308619908] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[58310393163] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58311576114] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58327770732] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[58328963583] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58330160031] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58331304207] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=923 ticks
[58332497520] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[58333615461] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[58335299616] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58336451250] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58337592192] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[58384155456] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[58385242443] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58386117174] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58393914810] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[58394693874] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58395394893] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58396034895] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=925 ticks
[58396704003] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[58397348559] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[58398579921] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58399253715] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58399924407] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[58451080776] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[58453014477] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58454264583] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58464921273] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[58466207514] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58467486033] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58468621530] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=927 ticks
[58469769006] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[58470935919] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[58472848632] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58473960402] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58475064153] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[58516470408] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[58517832714] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58518620226] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58525860657] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[58526852340] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58527636288] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58528286982] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=929 ticks
[58528979751] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[58529636022] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[58530770265] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58531413765] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58532045748] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[58573364784] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[58574520642] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58575258720] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58588302498] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[58589027871] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58589723445] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58590356484] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=931 ticks
[58591019784] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[58591661766] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[58592811321] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58593452874] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58594137063] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[58639630665] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[58641714879] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58642973334] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58659748422] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[58661075682] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58662283977] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58663428813] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=933 ticks
[58664607078] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[58665791844] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[58667590905] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58668721353] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58669847709] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[58714795590] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[58716529047] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58717357215] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58724375391] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[58725092118] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58725765582] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58726464060] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=935 ticks
[58727140890] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[58727861346] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[58728879759] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58729529100] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58730160258] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[58773203280] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[58775073489] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58776179913] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58790822244] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[58791550125] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58792221246] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58792865010] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=937 ticks
[58793582694] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[58794236457] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[58795444719] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58796119668] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58796772012] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[58837716036] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[58839790845] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58841023164] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58855434462] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[58856369055] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58857077070] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58857716742] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=939 ticks
[58858389315] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[58859034267] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[58860053373] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58860754227] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58861402314] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[58912606863] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[58913785161] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58914550893] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58922197686] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[58922959095] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58923720306] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58924681530] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=941 ticks
[58925515539] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[58926161448] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[58927214478] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[58927862103] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[58928514579] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[59005487310] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[59008651020] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59009867532] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59038722237] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[59040130875] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59041742496] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59042834631] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=944 ticks
[59054578374] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[59055674238] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[59057868309] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59058953943] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59081006292] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[59134142529] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[59161632519] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59162942619] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59187358923] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[59190826365] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59191587708] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59192316942] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=947 ticks
[59193077526] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[59193733500] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[59194877214] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59195592390] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59196400362] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[59247201123] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[59249111889] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59250173532] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59258047266] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[59258774652] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59259456663] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59260115442] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=949 ticks
[59260805934] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[59261528271] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[59262945588] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59263627863] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59264557209] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[59310108363] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[59312069256] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59313184260] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59323236126] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[59324488971] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59325546192] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59326571568] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=951 ticks
[59327670402] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[59328799563] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[59330505168] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59331523449] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59332488105] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[59376279600] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[59378052162] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59379246234] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59389508508] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[59390693406] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59392097457] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59393222592] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=953 ticks
[59394377163] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[59395482069] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[59397452730] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59398570407] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59399688381] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[59442029262] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[59443921515] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59445082422] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59455077066] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[59456359974] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59457385614] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59458391454] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=955 ticks
[59459461215] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[59460548961] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[59462241993] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59462918361] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59463561102] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[59532821832] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[59534693328] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59535924723] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59553140262] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[59554423797] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59555597112] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59556739902] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=958 ticks
[59557966644] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[59559133524] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[59561202921] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59562387357] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59563473552] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[59607086418] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[59609281215] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59610567192] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59622818079] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[59624069571] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59625233250] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59626338717] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=960 ticks
[59627514012] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[59628651720] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[59630605947] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59631707817] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59632800480] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[59667909312] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[59670883635] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59675523864] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59687428548] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[59688665124] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59689857744] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59691002349] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=962 ticks
[59692111380] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[59692871370] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[59694230343] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59694904434] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59695547406] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[59739813045] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[59741842875] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59743049883] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59754673671] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[59755534278] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59756219919] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59756993703] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=964 ticks
[59757838371] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[59758562820] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[59759671323] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59760340959] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59761064715] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[59805435525] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[59807321112] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59808451725] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59815875834] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[59816606058] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59817290742] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59817981465] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=966 ticks
[59818677204] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[59819374428] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[59820376044] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59821041918] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59821808772] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[59871080280] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[59872366323] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59873094270] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59880676350] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[59881380009] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59882042715] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59882725320] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=968 ticks
[59883452244] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[59884352319] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[59885824647] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59886537117] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59887335222] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[59938039623] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[59940037575] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59941252569] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59952737724] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[59953981362] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59955124185] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59956337958] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=970 ticks
[59957496984] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[59958725079] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[59960811207] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59961906642] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[59963027982] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[59993647956] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[59995886940] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[59997112956] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60015694134] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[60016987866] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60018201342] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60019357134] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=972 ticks
[60020634696] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[60021817944] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[60023797581] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60024915027] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60026161404] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[60069774138] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[60071795520] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60073019523] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60083544378] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[60084763959] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60085895133] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60086954631] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=974 ticks
[60088063926] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[60089142531] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[60091095867] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60092247996] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60093241032] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[60136170171] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[60138290751] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60139816671] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60150882660] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[60152093232] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60153215331] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60154352544] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=976 ticks
[60155570079] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[60156753063] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[60158968947] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60160106490] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60161252613] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[60201922011] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[60203791626] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60205074699] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60215650803] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[60216909786] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60218082837] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60219238002] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=978 ticks
[60220455108] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[60221653503] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[60223517574] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60224667228] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60225801735] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[60268137765] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[60270317646] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60271661010] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60279538770] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[60280271535] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60280932294] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60281571900] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=980 ticks
[60282240051] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[60282916254] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[60284327697] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60284977038] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60285709704] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[60335132187] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[60336949101] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60338086017] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60348085116] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[60349247640] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60350322945] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60351362016] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=982 ticks
[60352457847] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[60353492694] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[60355220739] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60356275056] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60357281259] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[60400018701] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[60401822778] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60403071894] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60414027993] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[60415163094] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60416298327] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60417442239] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=984 ticks
[60418610142] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[60419752206] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[60422175462] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60423305646] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60424422036] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[60466163571] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[60468110868] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60469294116] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60480317535] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[60481499331] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60482627865] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60483793194] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=986 ticks
[60485027625] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[60486177807] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[60488377653] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60489525558] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60490639440] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[60532653060] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[60534404502] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60535614744] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60547065711] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[60548239620] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60549408579] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60550513056] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=988 ticks
[60551658156] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[60552801177] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[60554745174] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60555907566] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60556970199] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[60598424997] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[60600298473] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60601518054] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60612550053] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[60613684989] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60614808210] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60615950274] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=990 ticks
[60617232852] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[60618388050] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[60620161338] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60621262581] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60622393359] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[60665010318] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[60667611147] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60668830365] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60680430426] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[60681637533] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60682832793] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60683967432] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=992 ticks
[60685258953] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[60686364717] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[60688501632] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60689607429] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60690781503] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[60722233473] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[60723946272] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60725153247] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60741871344] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[60743130822] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60744335520] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60745478277] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=994 ticks
[60746660304] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[60747863022] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[60750075705] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60751260669] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60752366697] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[60796484199] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[60798842874] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60800101428] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60810880515] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[60811652319] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60812442273] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60813195663] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=996 ticks
[60813976212] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[60814853385] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[60817060887] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60818248524] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60819395769] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[60862100310] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[60863674179] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60864398430] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60871752282] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[60872608005] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60873287046] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60873927774] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=998 ticks
[60874594605] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[60875269719] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[60876568863] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60877302387] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60877942158] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[60919957659] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[60921663825] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60923222184] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60935686053] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[60936472311] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60937160592] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60938201082] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1000 ticks
[60938921307] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[60939603714] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[60940657833] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60941335983] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60941969121] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[60962140800] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[60963458886] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60964243263] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60971993907] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[60972701790] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60973365717] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60974036310] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1001 ticks
[60974714988] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[60975365649] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[60976735281] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[60977414124] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[60978190779] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[61029156837] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[61031203134] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61032446640] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61042832268] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[61044056238] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61045176984] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61046279415] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1003 ticks
[61047532359] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[61048702011] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[61050651981] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61051765533] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61052868921] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[61085514600] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[61088200470] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61090103976] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61105997271] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[61107226521] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61108382907] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61109550381] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1005 ticks
[61110739602] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[61111928691] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[61113778473] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61114952712] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61116134145] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[61160253759] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[61162267617] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61163558049] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61174841739] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[61176185565] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61177401879] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61178595291] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1007 ticks
[61180959807] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[61182893178] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[61184306469] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61185159783] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61185807870] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[61226809182] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[61228763739] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61229938737] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61240685055] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[61241987433] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61243093395] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61244168139] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1009 ticks
[61245561102] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[61246605783] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[61248517110] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61261269003] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61262462118] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[61292939763] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[61295018268] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61296165414] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61343629578] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[61345068312] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61346812956] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61348056495] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1012 ticks
[61350978480] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[61352328477] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[61355798988] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61360329525] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61361362062] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[61384586934] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[61386644814] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61387895217] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61405056372] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[61406449467] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61407668124] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61408847313] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1014 ticks
[61410129957] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[61411249086] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[61413547338] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61414678545] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61415850870] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[61457701965] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[61460073708] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61461140565] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61469303808] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[61470029082] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61470826923] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61471554375] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1016 ticks
[61472392509] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[61473050232] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[61474454646] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61475131839] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61475809428] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[61524280917] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[61526259696] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61527496338] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61537956315] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[61539151938] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61540230675] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61541354820] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1018 ticks
[61542531336] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[61543668648] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[61547511300] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61548670359] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61549768731] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[61583651052] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[61585992732] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61589117238] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61600779537] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[61602081288] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61603379607] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61604514279] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1020 ticks
[61605695085] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[61606877937] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[61608830481] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61610013168] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61611161634] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[61655842149] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[61657933590] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61659164787] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61670959548] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[61672250277] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61673380065] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61674531534] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1022 ticks
[61675740918] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[61676853744] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[61678554366] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61679619507] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61680662703] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[61722274350] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[61724018400] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61725207423] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61735211571] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[61736351688] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61737432471] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61738512528] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1024 ticks
[61739620173] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[61740715674] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[61743817476] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61744915551] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61745936604] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[61788847725] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[61791012591] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61791792810] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61800256386] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[61801536324] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61802429964] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61803081021] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1026 ticks
[61803897639] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[61804601199] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[61806346536] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61807337823] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61808063427] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[61854128226] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[61856222571] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61857032127] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61864577709] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[61865289057] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61865948034] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61866588663] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1028 ticks
[61867310637] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[61868102802] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[61869578100] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61870488999] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61871189028] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[61920557457] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[61922297712] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61923079977] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61930729443] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[61931454453] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61932142404] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61932833061] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1030 ticks
[61933544046] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[61934236815] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[61935551997] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61936247340] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[61936904997] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[61986576927] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[61988663847] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[61989891843] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62000281035] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[62001503157] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62002625355] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62003751447] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1032 ticks
[62005017987] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[62006158500] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[62008979505] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62010078207] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62011174962] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[62052674706] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[62054554320] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62055907914] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62066901303] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[62068176159] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62069323008] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62070436098] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1034 ticks
[62071635747] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[62072782893] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[62074655577] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62075803185] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62076957096] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[62118726384] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[62121007773] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62122250355] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62132737920] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[62133996276] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62135150715] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62136270768] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1036 ticks
[62137646901] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[62138808930] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[62140687026] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62141815362] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62142972606] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[62177159616] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[62178979632] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62180345304] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62197556916] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[62198788080] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62200032708] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62201184474] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1038 ticks
[62202353697] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[62203524504] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[62205822954] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62206983630] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62208087678] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[62274802095] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[62276664120] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62277974979] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62294898303] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[62296226190] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62297414817] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62298587208] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1041 ticks
[62299753725] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[62300886054] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[62302711350] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62303871465] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62305024749] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[62349616626] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[62351576760] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62352863265] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62363277900] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[62364487977] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62365675152] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62366857443] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1043 ticks
[62368106757] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[62369297925] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[62371433289] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62372586969] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62373717549] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[62415775455] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[62417734830] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62418971142] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62429413134] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[62430726303] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62431923840] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62433076200] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1045 ticks
[62434312182] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[62435502723] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[62439591357] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62440804239] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62441948613] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[62472485658] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[62474765430] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62476040352] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62491666017] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[62492873223] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62493971001] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62495019510] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1047 ticks
[62496145569] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[62497214307] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[62498920440] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62500006734] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62501030361] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[62548211880] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[62550000579] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62551139475] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62561173224] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[62562319281] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62563381749] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62564430489] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1049 ticks
[62565563115] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[62566743525] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[62568584331] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62569727583] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62570874960] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[62614516767] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[62616577452] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62617839504] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62628411978] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[62629585194] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62630713926] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62631857508] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1051 ticks
[62633078442] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[62634202521] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[62636485164] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62637601521] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62638783284] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[62672761470] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[62674604388] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62675863206] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62691999414] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[62693230116] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62694606744] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62695754847] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1053 ticks
[62696912025] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[62698057587] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[62700286440] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62701431111] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62702595879] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[62746240194] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[62747573031] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62748316158] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62755994037] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[62756765742] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62757449502] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62758104486] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1055 ticks
[62758796826] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[62759482533] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[62761121742] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62761907175] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62762536980] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[62836955181] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[62839032993] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62840259273] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62857046373] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[62857836261] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62858567145] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62859216750] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1058 ticks
[62859890742] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[62860653537] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[62862070194] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62862749565] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62863411281] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[62911447962] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[62913515181] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62914770369] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62925711519] [[32mINFO [0m] [di[62945209338] [[32mINFO splay_virtio_gpu] [CPU0] d[0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[62947122183] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62948334702] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62958858369] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[62960097321] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[629613isplay_virtio_gpu: wait02052] [[32mINFO [ing on WaitSet...
[62926943046] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62962451508] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1061 ticks
[62963682276] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[62964858660] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[62966652144] [[320m] [kmINFO [0m] [kernel::syscall::hanernel::syscall::handlers::wait] [CPU0] colldlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62967940728] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62969076588] [[32mINFO [0m] [kernel::syect_rescall::hady: andlers::wait] [CPU0] sys_wait_many:polling spec[0] kind=7 object=8 token=1
[62928133158] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 blocking...
 object=a token=2
[62929265322] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1060 ticks
[62930492757] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[62931749529] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[62934012009] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[62935166151] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[62936275545] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[63044352657] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[63046369749] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63047629722] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63058262025] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[63059553975] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63060701121] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63062120418] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1064 ticks
[63063345972] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[63064588059] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[63066639768] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63067788861] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63068951913] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[63100207302] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[63102043389] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63103137207] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63118265793] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[63119520750] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63120605064] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63121684593] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1066 ticks
[63122778411] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[63123865233] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[63125593080] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63126650433] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63127693629] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[63175611246] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[63177422847] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63178673085] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63190058217] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[63191233941] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63192364686] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63193457382] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1068 ticks
[63194732073] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[63195754314] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[63198018180] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63199159881] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63200262840] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[63242084433] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[63244424034] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63245669289] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63256897539] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[63258073494] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63259227735] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63260469492] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1070 ticks
[63261534963] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[63262687785] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[63264748635] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63265896309] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63267023556] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[63307850001] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[63309829143] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63310957512] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63321739635] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[63322900080] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63323965947] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63325018185] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1072 ticks
[63326123982] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[63327202290] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[63329594031] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63330676266] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63331703160] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[63373869900] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[63375549600] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63376679355] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63386473260] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[63387585096] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63388637730] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63389650632] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1074 ticks
[63390719403] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[63391764810] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[63393529320] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63394716891] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63395840211] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[63439680645] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[63440803239] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63441555210] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63448785444] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[63449496363] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63450157749] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63450828573] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1076 ticks
[63451560018] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[63452206917] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[63453214176] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63453861141] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63454524078] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[63495191265] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[63497182617] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63498378669] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63514396506] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[63515677632] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63516886884] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63518056503] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1078 ticks
[63519275853] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[63520487877] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[63522284958] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63523300170] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63524462166] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[63572802480] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[63575566593] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63576814488] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63587938491] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[63588977793] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63590105073] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63591283635] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1080 ticks
[63592489026] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[63593789952] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[63595797342] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63596955246] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63598081503] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[63630503838] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[63632742096] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63633986130] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63646442937] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[63647733237] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63648978921] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63650159628] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1082 ticks
[63651385974] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[63652566384] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[63654934266] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63656141307] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63657371448] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[63705142776] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[63707781654] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63709019616] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63720510942] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[63721765239] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63722988021] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63724121769] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1084 ticks
[63725401377] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[63726539052] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[63728727282] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63729851922] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63730984812] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[63762181131] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[63764056323] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63765269601] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63780945525] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[63782227740] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63783403134] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63784587999] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1086 ticks
[63785779992] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[63786935421] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[63788794014] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63789459954] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63790164801] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[63837841221] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[63839762481] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63840971865] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63851800419] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[63852934464] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63854074614] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63855271425] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1088 ticks
[63856476849] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[63857731542] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[63859603071] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63860799486] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63861914358] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[63892641054] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[63894378933] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63895597425] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63912460953] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[63913753200] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63914938098] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63916107354] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1090 ticks
[63917267733] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[63918406068] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[63920179059] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63921394977] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63922535886] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[63969064665] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[63970912863] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63972177456] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63979845633] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[63980548632] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63981227244] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63981915657] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1092 ticks
[63982655088] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[63983330829] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[63984628125] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[63985300665] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[63986088573] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[64035414597] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[64037650743] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64039030539] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64049635815] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[64050959247] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64052136456] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64053269280] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1094 ticks
[64054561857] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[64055707749] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[64057637061] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64058765364] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64059869082] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[64101103968] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[64102996650] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64104233424] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64114542954] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[64115742174] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64116901794] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64118002245] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1096 ticks
[64119136092] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[64120291092] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[64122069462] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64123241787] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64124359134] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[64167047538] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[64169169207] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64170358032] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64181111874] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[64182310137] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64183423524] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64184531103] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1098 ticks
[64185784377] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[64186939245] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[64188821895] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64189935282] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64191029628] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[64232985927] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[64234975200] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64236204417] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64246771545] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[64248024423] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64249221267] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64250343993] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1100 ticks
[64251552255] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[64252716363] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[64254749460] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64255867500] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64256976300] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[64299049584] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[64300756410] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64301910717] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64311676605] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[64312729503] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64313744055] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64314790749] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1102 ticks
[64315871763] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[64316983236] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[64318948023] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64320045867] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64321074114] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[64365483996] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[64367175378] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64368327144] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64378347891] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[64379452500] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64380512526] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64381530741] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1104 ticks
[64382586081] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[64383654060] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[64385304753] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64386393060] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64387441701] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[64431144129] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[64432802907] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64433925963] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64443783426] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[64444863714] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64445914929] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64446925026] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1106 ticks
[64447987098] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[64449049995] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[64451007786] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64452024450] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64453067778] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[64498487163] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[64500070338] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64500964572] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64508571369] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[64509300636] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64510033566] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64510674360] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1108 ticks
[64511385246] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[64512092007] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[64513293867] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64513961292] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64514623206] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[64563327873] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[64565097135] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64566221973] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64576358880] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[64577421678] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64578473883] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64579481703] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1110 ticks
[64580565225] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[64581621291] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[64583409891] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64584570468] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64585601190] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[64629369156] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[64631150661] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64632266028] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64642100358] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[64643151078] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64644207639] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64645231596] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1112 ticks
[64646299575] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[64647368181] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[64649109129] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64650150147] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64651204002] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[64695888147] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[64697936292] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64699056543] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64709214405] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[64710355908] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64711430124] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64712483880] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1114 ticks
[64713616704] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[64714841400] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[64716938187] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64718007057] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64719043917] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[64761616755] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[64763578011] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64764704037] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64773313968] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[64774006572] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64774666011] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64775370924] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1116 ticks
[64776066663] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[64776712671] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[64777753689] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64778398047] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64779094347] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[64827723147] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[64829518380] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64830688164] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64838601531] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[64839350664] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64840021719] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64840662876] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1118 ticks
[64841386401] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[64842040758] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[64843312644] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64843970697] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64844654028] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[64894809474] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[64896826962] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64897715553] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64904619252] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[64905328455] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64905996507] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64906643802] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1120 ticks
[64907355942] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[64908011652] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[64909001718] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64909745934] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[64910500743] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[64984020420] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[64985990850] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[64987237755] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65003957766] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[65005197807] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65006396103] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65007523317] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1123 ticks
[65008708182] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[65009893806] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[65011854303] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65013167736] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65014325343] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[65058926988] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[65060649852] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65061899166] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65073080985] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[65074247139] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65075412336] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65076520608] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1125 ticks
[65077765368] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[65078916375] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[65081730054] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65082870204] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65084174067] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[65125155711] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[65126272101] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65126997936] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65134481610] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[65135231172] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65135923347] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65136595524] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1127 ticks
[65137316211] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[65138044587] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[65139158271] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65139837213] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65140502592] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[65183046885] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[65185003422] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65185719885] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65198682285] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[65199423201] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65200124418] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65200770525] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1129 ticks
[65201444946] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[65202095871] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[65203688484] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65204355711] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65204990334] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[65257226925] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[65258869104] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65259630381] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65266948593] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[65267648721] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65268312318] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65268950769] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1131 ticks
[65269650666] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[65270298456] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[65271529686] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65272176783] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65272937961] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[65313171198] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[65315257095] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65316526374] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65334194178] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[65335115967] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65335819527] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65336464578] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1133 ticks
[65337135369] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[65337781740] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[65338978980] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65339635482] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65340265056] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[65380008771] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[65381817435] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65382849774] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65396950377] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[65398222461] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65399329710] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65400400428] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1135 ticks
[65401516884] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[65402635848] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[65404453554] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65405510214] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65406561066] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[65455459146] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[65457100137] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65458214217] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65468162958] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[65469235689] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65470275618] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65471278752] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1137 ticks
[65472323631] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[65473354584] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[65475279903] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65476345902] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65477384016] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[65522320347] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[65524334766] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65525561046] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65536158699] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[65537306538] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65538434874] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65539562946] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1139 ticks
[65540793186] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[65541940992] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[65544403287] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65545056621] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65545730976] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[65588250486] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[65590165773] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65590926720] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65598278229] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[65599042608] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65599737588] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65600413692] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1141 ticks
[65601210015] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[65602008351] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[65603589942] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65604481569] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65605306404] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[65654121489] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[65656127526] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65657251869] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65667286212] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[65668401513] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65669439924] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65670446523] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1143 ticks
[65671526118] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[65672644488] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[65674471467] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65675490639] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65676501165] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[65719620087] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[65721280944] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65722397202] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65732320962] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[65733399468] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65734457646] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65735462397] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1145 ticks
[65736525987] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[65737576905] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[65740445133] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65741510769] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65742503574] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[65785840164] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[65788347669] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65789459769] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65799245457] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[65800358349] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65801391810] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65802410322] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1147 ticks
[65803497375] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[65804614623] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[65806219413] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65807289471] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65808317025] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[65852034930] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[65853792873] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65854971336] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65864708811] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[65865832098] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65866884435] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65867882388] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1149 ticks
[65868973137] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[65870003529] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[65871752298] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65872815327] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65873841990] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[65918330247] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[65920068489] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65921188179] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65928505203] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[65929224867] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65929903710] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65930568990] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1151 ticks
[65931364191] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[65932028283] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[65933010462] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65933663862] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65934425403] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[65984145876] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[65985811023] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65986610349] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65993989380] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[65994697329] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65995354062] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[65995983438] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1153 ticks
[65996647860] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[65997470187] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[65999079762] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[65999741379] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66000367719] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[66018032388] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[66019083207] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66019791057] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66027340071] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[66028021389] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66028697097] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66029348847] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1154 ticks
[66030037953] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[66030728214] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[66032254497] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66032903178] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66033557205] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[66083685129] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[66086385552] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66087625428] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66098343102] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[66099591987] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66100758768] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66101918091] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1156 ticks
[66103158462] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[66104296698] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[66106180668] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66107326296] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66108454500] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[66149268603] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[66151304439] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66152605266] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66164820744] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[66166052865] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66167198559] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66168390750] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1158 ticks
[66169624125] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[66170783382] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[66172658442] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66173843043] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66174995172] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[66206126613] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[66209314908] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66210608376] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66228230904] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[66229445964] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66230627826] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66231797049] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1160 ticks
[66233136750] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[66234439260] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[66236215683] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66237171363] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66237891885] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[66306643689] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[66308839773] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66310120305] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66325075443] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[66325848897] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66326536485] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66327199686] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1163 ticks
[66327949314] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[66328664985] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[66329862753] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66330511929] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66331154571] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[66381375786] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[66384054165] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66385276122] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66396377388] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[66397613733] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66398766588] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66399960528] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1165 ticks
[66401173542] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[66402366756] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[66404157369] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66405277059] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66406402656] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[66446938602] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[66448924047] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66450153561] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66460748574] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[66461987064] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66463138764] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66464265714] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1167 ticks
[66465524763] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[66466639635] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[66468401274] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66469524726] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66470621844] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[66513106836] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[66515158380] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66516361791] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66526855098] [[32mINFO [0m] [display_virtio_gpu] [CPU0] di[66546799770] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[66548984700] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66550200585] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66560906346] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[66562063260] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66563219547] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66564356463] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 counsplay_virtio_gpu: waiting on WaitSet...
[t=2 timeout=1170 ticks
[66565561425] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[66566768730] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[66568790640] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66569919735] [[32mINFO 66528070191] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66529219482] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[665303[0m] [kernel::syscall::han61dler942] [[32mINFO [0m]s::wait] [CPU0] collect_read [y:kernel::syscall::handle polrsling spec[0] kind=7 object=::wait]a token=2
[66571064109] [[32mINFO [0m] [kernel::sys [CPU0] sycall::handlers::wait] [CPU0] sys_wait_many: blocking.s_wait_many: tid=9 specs_ptr=10000f8 count=..
2 timeout=1169 ticks
[66531636600] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[66532762362] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[66534902379] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66536114898] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66537241452] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[66646154487] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[66648064164] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66649311828] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66659897403] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[66661157772] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66662319372] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66663503016] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1173 ticks
[66664674714] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[66665831826] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[66667531821] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66668717907] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66669882774] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[66703567689] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[66705300123] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66706542342] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66722874669] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[66724117416] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66725343597] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66726509091] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1175 ticks
[66727713195] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[66728906376] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[66730756257] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66731940627] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66733080843] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[66777516597] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[66779550882] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66780758616] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66791601327] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[66792807774] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66793979703] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66795148992] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1177 ticks
[66796421802] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[66797660259] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[66799831824] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66800996889] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66802114203] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[66834808029] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[66836629629] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66838004178] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66854058942] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[66855380526] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66856592814] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66857807775] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1179 ticks
[66859016730] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[66860191035] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[66862206345] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66863470872] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66864652107] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[66909466536] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[66911521743] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66912740829] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66922858365] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[66924081906] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66925223772] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66926393589] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1181 ticks
[66927570138] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[66928788597] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[66931019298] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66932126712] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66933291216] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[66975703509] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[66977582430] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66978836298] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66989245125] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[66990427647] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66991553574] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66992698443] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1183 ticks
[66993886113] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[66995037450] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[66996897594] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[66998029131] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[66999162945] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[67041684930] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[67043679120] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67044949389] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67055042010] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[67056280368] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67057459128] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67058621619] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1185 ticks
[67059768897] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[67060933071] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[67063377810] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67064556141] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67065636264] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[67108169634] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[67110090135] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67111341759] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67121584365] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[67122807774] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67124003199] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67125121899] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1187 ticks
[67126395336] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[67127550435] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[67129375137] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67130524164] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67131647946] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[67174054332] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[67176092940] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67177342419] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67187589513] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[67188770055] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67189937661] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67191067581] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1189 ticks
[67192214199] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[67193382927] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[67195644615] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67196799153] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67197958839] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[67239954606] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[67241956485] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67243203225] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67253827245] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[67255032504] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67256199978] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67257330954] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1191 ticks
[67258530306] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[67259713488] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[67261976463] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67263112356] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67264225644] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[67306682883] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[67308675126] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67309926882] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67320592350] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[67321809291] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67322962806] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67324117542] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1193 ticks
[67325465889] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[67326705864] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[67328340288] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67329466116] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67330596069] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[67364382789] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[67366265406] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67367606163] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67383519621] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[67384839291] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67385972808] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67387120614] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1195 ticks
[67388355408] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[67389546906] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[67391871558] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67392969303] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67394022399] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[67439140890] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[67441144617] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67442371425] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67453867008] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[67455061641] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67456186380] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67457373027] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1197 ticks
[67458527367] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[67459638576] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[67461415461] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67462466379] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67463566335] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[67494858750] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[67496984247] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67498193070] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67514830581] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[67516101411] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67517271690] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67518390225] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1199 ticks
[67519593900] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[67520741145] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[67522650756] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67523786286] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67525056720] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[67571294967] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[67573076868] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67574193588] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67584053922] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[67585226148] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67586275746] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67587281091] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1201 ticks
[67588374282] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[67589470113] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[67592248416] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67593261021] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67594254255] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[67636138944] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[67638198474] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67639161876] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67648085208] [[32mINFO [0m] [display_virtio_gpu] [CPU0] display_virtio_gpu: waiting on WaitSet...
[67649014059] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67649889879] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67650795960] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=1203 ticks
[67651710984] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=8 flags=1 token=1
[67652594031] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=a flags=1 token=2
[67654585614] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 token=1
[67655788629] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=a token=2
[67656857103] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[67702844748] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[67704959058] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=8 to
```
</details>
