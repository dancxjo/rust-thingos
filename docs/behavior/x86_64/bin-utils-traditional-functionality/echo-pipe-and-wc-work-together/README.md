# ❌ Scenario: echo, pipe, and wc work together

> Last run: 2026-04-24 11:58:04

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ❌ | 1002ms | - [📜](./01/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[49665137412] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[49716936918] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[49789989744] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[49854210285] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[49891786032] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[49896398310] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[49968480243] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[50009118093] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[50105408199] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[50107507626] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[50280392085] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[50403079254] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[50447533092] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[50571158385] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[50727534033] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[50830542345] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[50886382107] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[51157881984] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[51251419560] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[51640927404] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[52396146561] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=2247308085 elapsed_us=1123654
[52397622222] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[52597653867] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[52601405901] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[52854607014] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[52950832836] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[52969443318] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[52970995638] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[53164647096] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[53171586534] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[53182151748] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[53204868585] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
    [36mps[53242582734] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=40095
[0m            observe living processes
    [36mcat /version[0m  inspect the genome

[2m--------------------------------------------------------------[0m
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[53613547020] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[53615717793] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[53639903427] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[53645288961] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[53649252954] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=5 specs_ptr=10002f8 count=2 timeout=34 ticks
[53651944203] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=3 flags=1 token=1
[53653447914] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=4 flags=1 token=2
[53666759949] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[53668257753] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[53670048927] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[53834056584] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[54001908312] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[54088371282] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[54106698360] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
e[54136921938] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[54138678825] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[54144692514] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[54146375052] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
c[54204822837] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=5, bind_id=322371585)
ho[54291700881] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[54293369625] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[54316663665] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=5)
[54319116027] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Starting VFS-native VirtIO GPU driver...
[54323709198] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: boot_arg=5
[54325845255] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Mapping bootstrap memfd 5 size=4096...
[54331255506] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: vm_map success at 0x400000001000
[54352205193] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Recovered handles: req_read=4, resp_write=5, svc=1, id=322371585
[54354057582] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting (drv_req_r=4, drv_resp_w=5, svc=1, id=322371585)
[54375898434] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=10)
[54398606394] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[54415925949] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[54449848728] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[54503838906] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[54505940313] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[54519989799] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[54521900367] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[54535538343] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[54536766702] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=5 specs_ptr=10002f8 count=2 timeout=56 ticks
[54540756600] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=3 flags=1 token=1
[54542003307] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=4 flags=1 token=2
[54548396397] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[54549820215] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[54554817702] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[54913743885] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[54923042394] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[54924757272] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[54930819702] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[54SPROUT: Loop iteration: run_health_vine
[54957543762] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[54961558575] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[54963220917] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[54964420863] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[54965646450] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=5 specs_ptr=10002f8 count=2 timeout=68 ticks
[54966827421] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=3 flags=1 token=1
[54967952358] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU932132079] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[54932752083] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[54935983410] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[54937338951] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[54938828142] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[54940298622] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[54941890344] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[54944345808] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[54948087612] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[54949303464] [[32mINFO [0m] [sprout::supervisor] [CPU0] 0]   spec[1]: kind=7 object=4 flags=1 token=2
[54971908860] [[32mINFO [0m] [kernel::syscall::handlers::w[54987634317] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[54999898107] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Virgl 3D not supported, using 2D only
[55031584608] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=1
ait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[54973210809] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[54974478669] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[55143426888] [[32mINFO [0m] [bristle] [CPU1] bristle: published pid 10 to /run/bristle/pid
[55186239372] [[32mINFO [0m] [bristle] [CPU1] bristle: published device handles kbd_in=7 mouse_in=9
[55196818809] [[32mINFO [0m] [bristle] [CPU1] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[55199113200] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=6 token=1
[55200522531] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=7 token=2
[55202100162] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=8 token=3
[55203697428] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] sys_wait_many: tid=10 specs_ptr=1000000 count=3 timeout=0 ticks
[55205106561] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1]   spec[0]: kind=7 object=6 flags=1 token=1
[5520643401[55223639163] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=7 token=2
9] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1]   spec[1]: kind=7 object=7 flags=1 token=2
[55216676427] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1]   spec[2]: kind=7 object=8 flags=1 token=3
[55221671868] [[32mINFO [0m] [kernel::syscall::handlers::wait[55227090831] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=8 token=3
[55228598469] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] sys_wait_many: blocking...
] [CPU1] collect_ready: polling spec[0] kind=7 object=6 token=1
[55290416874] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[55292329620] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[55293669651] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[55296580878] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[55297811382] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[55309481073] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[55310792328] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[55312058703] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[55334459169] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[55335994758] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[55337328981] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[55341682737] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[55343217600] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[55363761519] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[55365591534] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[55366920180] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[55368215067] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=5 specs_ptr=10002f8 count=2 timeout=80 ticks
[55369511175] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=3 flags=1 token=1
[55372190709] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=4 flags=1 token=2
[55374621192] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[55384021506] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[55385257620] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[55682656986] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[55684805913] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[55686136935] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[55688916690] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[55700790057] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[55705353231] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[55706650989] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[55707907299] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[55709083254] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[55710278448] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[55711767969] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[55741955049] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[55743325473] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[55755333942] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[55757092017] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[55758387927] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[55759651398] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=5 specs_ptr=10002f8 count=2 timeout=92 ticks
[55760911008] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=3 flags=1 token=1
[55762144284] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=4 flags=1 token=2
[55764113691] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[55765328454] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[55766529918] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[56110013817] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[56111879307] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[56113176306] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[56115750471] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[56116960614] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[56121312489] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[56122593021] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[56123828244] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[56124976908] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[56126105541] [[32mINFO [0m] [s[56159510484] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[56161346406] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[56162647299] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[56163917634] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=5 specs_ptr=10002f8 count=2 timeout=104 ticks
[56165294262] [[prout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[56127279813] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[56130796821] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[56132174637] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=3 flags=1 token=1
[56184995757] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=4 flags=1 token=2
[56187194745] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[56191405050] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[56192667102] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
 hello world | wc -w
[56406610557] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='echo'
[56422114881] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/echo' with argv=["/bin/echo", "hello", "world"]
[56515780398] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[56517607047] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[56518967208] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[56520281202] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/echo' TID=11 PID=11
[56532104937] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[56533318644] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[56566332669] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[56567984385] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[56574694605] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[56578369188] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[56580526596] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='wc'
[56587902558] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[56599327785] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[56605037181] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/wc' with argv=["/bin/wc", "-w"]
[56611370310] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[56612801553] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[56616862863] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[56618546358] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[56630675013] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[56632237530] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=5 specs_ptr=10002f8 count=2 timeout=117 ticks
[56633648841] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=3 flags=1 token=1
[56634993129] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=4 flags=1 token=2
[56649237711] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[56650709445] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[56655319182] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[56672055627] [[32mINFO [0m] [kernel::ipc::pipe] [CPU2] PIPE_ENQUEUE: tail=12 len=12 n=12
[56778565866] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/wc' TID=12 PID=12
[56901046158] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 1 pipes
[56903667018] [[32mINFO [0m] [sh] [CPU3] sh: closing pipe 0 ends: read=6 write=7
[?25l[56939556696] [[32mINFO [0m] [wc] [CPU2] wc: counting from stdin (no files)
[56949911601] [[32mINFO [0m] [wc] [CPU2] wc: read 12 bytes from fd 0
[56973552768] [[32mINFO [0m] [kernel::ipc::pipe] [CPU2] PIPE_DEQUEUE: EOF reached (writers=0)
[56979953448] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[56981748516] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[56983165833] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
       2
[56991279576] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[56992549383] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[57011881014] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[57013343508] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[57014692185] all::handlers::wait] [CPU0]   spec[0]: kind=7 object=3 flags=1 token=1
[57042600054] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=4 flags=1 token=2
[57054042507] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[57071932599] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[57073330776] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[57015977304] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[57017273346] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[57018658785] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[57022357920] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[57023895753] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[57030397380] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[57032143047] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[57033466017] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[57034753974] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=5 specs_ptr=10002f8 count=2 timeout=129 ticks
[57036034539] [[32mINFO [0m] [kernel::sysc[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[57139235175] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: frame pool ready (1 buffer)
[57169475187] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_BIND_READY (ID: 322371585)
[57171295698] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Waiting for BIND_ASSIGNED...
[57171931938] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[57173499108] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[57174859731] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[57177688590] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: returning 1 results
[57192759459] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: BIND_READY from instance_id=0x13370001
[57203035494] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: 
```
</details>
