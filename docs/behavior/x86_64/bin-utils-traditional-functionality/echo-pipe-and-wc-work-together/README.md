# ❌ Scenario: echo, pipe, and wc work together

> Last run: 2026-04-24 11:50:15

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 15866ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 463ms | - [📜](./02/serial.log) - |
| 3 | And I type "echo hello world | wc -w" on the serial console | ✅ | 10449ms | - [📜](./03/serial.log) - |
| 4 | Then the command output should strictly be "2" | ❌ | 6057ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[49337296635] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[49389505572] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[49454615628] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[49517424000] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[49557162765] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[49562416893] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[49632257004] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[49672304715] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[49768559082] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[49770684612] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[49937850864] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[50053477716] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[50095875588] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[50215415052] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[50381166594] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[50471502213] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[50519963076] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[50754370260] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[50933462778] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[51262178088] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[51859566165] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=2047607496 elapsed_us=1023803
[51861247086] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[52082025237] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[52086193929] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[52274800182] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[52363598925] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[52383784530] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[52385590587] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[52581888150] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[52586459574] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[52596155865] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[52600191600] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[52617197952] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=38445
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[52971622803] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[52974062988] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[52997349471] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[53002257990] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[53005961316] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=5 specs_ptr=10002f8 count=2 timeout=31 ticks
[53008942932] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=3 flags=1 token=1
[53010596958] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=4 flags=1 token=2
[53021174712] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[53022518604] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[53029835265] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[53117949291] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[53401339332] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[53432910828] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[53450826924] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[53457397917] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[53458773291] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[53467090347] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[53468699658] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
ec[53572315533] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=5, bind_id=322371585)
h[53642288205] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[53644503726] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[53666207331] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=5)
[53668670286] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Starting VFS-native VirtIO GPU driver...
[53670187989] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: boot_arg=5
[53672123670] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Mapping bootstrap memfd 5 size=4096...
[53676315033] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: vm_map success at 0x400000001000
[53678430333] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Recovered handles: req_read=4, resp_write=5, svc=1, id=322371585
[53691904464] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting (drv_req_r=4, drv_resp_w=5, svc=1, id=322371585)
[53712545634] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=10)
[53721769332] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[53724781704] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[53726487342] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[53731368108] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[53732791992] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[53746189398] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[53748296811] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[53749731750] [[32mINFO [0m] [ker ticks
[53758371315] [nel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[53751142401] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=5 specs_ptr=10002f8 count=2 timeout=51[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=3 flags=1 token=1
[53759812194] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=4 flags=1 token=2
[53765748399] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[53772463470] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[53781419901] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[53892326400] [[32mINFO [0m] [bristle] [CPU1] bristle: published pid 10 to /run/bristle/pid
[53921733822] [[32mINFO [0m] [bristle] [CPU1] bristle: published device handles kbd_in=7 mouse_in=9
[53936942169] [[32mINFO [0m] [bristle] [CPU1] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[53939200161] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=6 token=1
[53940690045] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=7 token=2
[53942106207] [[32mINFO [0m] [kernel::syscall::handlers::wait]   spec[1]: kind=7 object=7 flags=1 token=2
[53950465503] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1]   spec[2]: kind=7 object=8 flags=1 token=3
[53954972676] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=6 token=1
[53956310727] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=7 token=2
[53957535555] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=8 token=3
[53964066684] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] sys_wait_many: blocking...
[53991060255] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[CPU1] collect_ready: polling spec[0] kind=7 object=8 token=3
[53943571341] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] sys_wait_many: tid=10 specs_ptr=1000000 count=3 timeout=0 ticks
[53945653179] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1]   spec[0]: kind=7 object=6 flags=1 token=1
[53946949518] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] [54015679905] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[54027119718] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[54028713750] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Virgl 3D not supported, using 2D only
[54054515592] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=1
[54094232379] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[54096370284] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[54097722690] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[54101254218] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[54102480399] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[54122652672] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[54124306962] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[54125711013] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[54127111500] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[54128343753] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[54136619394] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[54142862367] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[54147366735] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[54154432398] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[54156679368] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[54158002404] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[54159340158] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=5 specs_ptr=10002f8 count=2 timeout=63 ticks
[54160589043] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=3 flags=1 token=1
[54161805687] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=4 flags=1 token=2
[54163962930] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[54165243528] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[54166566960] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[54471009153] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[54473048025] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[54474481116] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[54477364590] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[54481984590] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[54498305202] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[54499781589] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[54503223093] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[54504662850] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[54506045682] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[54507466365] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[54511104879] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[54512450025] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[54516089496] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[54528885378] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[54530333385] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[54531750405] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=5 specs_ptr=10002f8 count=2 timeout=74 ticks
[54533138352] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=3 flags=1 token=1
[54534595632] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=4 flags=1 token=2
[54536681925] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[54538049841] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[54545789166] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[54833977473] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[54838945887] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[54840351093] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[54858526899] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[54859979658] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[54890810436] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[54892623258] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[54894036516] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[54898979949] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[54900905598] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[54902360040] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[54906115869] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[54907983537] [[32mINFO [0m] [sprout::supervisor] [CPU0] S0PROU] kind=7 objT: Loop iteration: run_health_vine
[54913768965] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[54915624786] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[ect=3 token=1
[54926891910] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[54928232766] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=5 specs_ptr=10002f8 count=2 timeout=86 ticks
[54929592630] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=3 flags=1 token=1
[54936290640] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=4 flags=1 token=2
[54938521605] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[54939882393] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[54941192988] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[55117007451] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: frame pool ready (1 buffer)
[55170840186] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[55172683170] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[55190873826] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[55195964901] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_BIND_READY (ID: 322371585)
[55200330537] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Waiting for BIND_ASSIGNED...
[55206843549] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: returning 1 results
[55232269422] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: BIND_READY from instance_id=0x13370001
[55261070139] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Registering DISPLAY_CARD at /dev/display/card0 (handle=7)
o hello world | wc -w
[55339054089] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[55341206745] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Handshake complete — task 'display' marked ready
[55348708305] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Sent MSG_BIND_ASSIGNED to req_port 3 (res=Ok(92))
[55350163869] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[55351452618] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[55355940024] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[55357984902] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[55359456801] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[55360712187] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[55362034431] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[55383807666] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[55393587810] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[55398373866] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_SERVICE_READY.
[55404883314] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[55406613141] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[55407940269] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[55422916494] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[55426167060] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[55428286980] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[55429506033] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=91 ticks
[55430833425] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[55440790812] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[55442944194] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[55444415961] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[55445671677] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
[55458034038] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='echo'
[55468320699] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: unblocked
[55497751914] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[55502670762] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[55516977747] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[55517946429] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[55518797136] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[55519617153] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=93 ticks
[55520542407] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[55521397272] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[55523109246] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
G[55525118847] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[55534723398] [[31;1mERROR[0m] [bran::arch::x86_64::idt] [CPU2] KERNEL PAGE FAULT at 0x0 RIP=0xffffffffb01d0888 CS=0x100000000000000 ERR=0x0 RSP=0x0
[55536396003] [[31;1mERROR[0m] [bran::arch::x86_64::idt] [CPU2] Registers:
[55537452003] [[31;1mERROR[0m] [bran::arch::x86_64::idt] [CPU2]   RAX: 0x000000000000000b RBX: 0xffffffffb000e770 RCX: 0x00ffffffb000e770 RDX: 0x0000000000000009
[55551507330] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[55552899402] [[31;1mERROR[0m] [bran::arch::x86_64::idt] [CPU2]   RSI: 0xffffffffb000e770 RDI: 0xffffffff[55563221835] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[55573191135] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[55574902416] [[31;1mERROR[0m] [bran::arch::x86_64::idt] [CPU2] Backtrace b01d0c48 RBP: 0xffffffff800f5993 RSP: 0x000000000000000b
[55554952959] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[55557537255] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[55558449837] [[31;1mERROR[0m] [bran::arch::x86_64::idt] [CPU2] KERNEL GPF at RIP=0xffffff8027a0f0 CS=0x8 ERR=0x0 RSP=0xffffffffb01cfb50
[55559474223] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=95 ticks
[55560913287] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token(c=1
urrent rbp=0xffffffffb01cfbb8):
[55576883076] [[31;1mERROR[0m] [bran::arch::x86_64::idt] [CPU2]   [00] 0xffffffff8021ec36 (frame=0xffffffffb01cfbb8)
[55578977223] [[31;1mERROR[0m] [bran] [CPU2] panicked at thingos/bran/src/arch/x86_64/idt.rs:1537:9:
KERNEL GPF
[55587901545] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[55647362925] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[55648929204] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[5565030830collect_ready: polling spec[0] kind=7 object=8 token=1
[55674601653] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
7] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[55651614447] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=97 ticks
[55653013152] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[55663789764] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[55668520479] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] [55705927398] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/echo' with argv=["/bin/echo", "hello", "world"]
[55803533148] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/echo' TID=11 PID=11
[55818405885] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='wc'
[55825072743] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/wc' with argv=["/bin/wc", "-w"]
[55898782236] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/wc' TID=12 PID=12
[55962561204] [[32mINFO [0m] [wc] [CPU3] wc: counting from stdin (no files)
[56042571948] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 1 pipes
[56044292469] [[32mINFO [0m] [sh] [CPU3] sh: closing pipe 0 ends: read=6 write=7
[?25l[56073355701] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[56074919769] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56076283857] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56077554192] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=110 ticks
[56078892012] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[56080178220] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[56083938240] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56085332457] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56100536877] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[56102264625] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56103653001] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56104984650] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=111 ticks
[56106425892] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[56107778166] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[56110105029] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56111446215] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56116734696] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
[56128496193] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: unblocked
[56131873677] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56143892178] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56182397766] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[56183855112] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56185525638] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56186878605] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=114 ticks
[56188271997] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[56189602095] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[56191587540] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56192903151] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56194285125] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
[56259912423] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: unblocked
[56262125304] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56263891299] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56277112617] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[56278633752] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56280159705] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56294703432] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[56296116624] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56301918354] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56303454504] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=117 ticks
[56304871590] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[56306252244] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[56308399983] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56309713845] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56311159113] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
[56324718153] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: unblocked
[56326729272] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56328192030] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56369315574] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[56370928218] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56372353125] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56373747177] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=119 ticks
[56375149149] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[56376579369] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[56392719669] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56394246975] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56410258905] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[56413351599] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56415738258] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56417970873] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=121 ticks
[56421233022] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[56423647896] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[56425864374] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56427214635] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56428581363] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
[56490008982] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: unblocked
[56492160087] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56493623637] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56507164296] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[56508512115] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56510070738] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56523158505] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[56524682445] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56526431643] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56528211201] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=124 ticks
[56530180740] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[56531651847] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[56533836579] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56535383883] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56536712166] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
[56579213196] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: unblocked
[56581286124] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56582594838] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56598149487] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[56599463415] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56600747016] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56601871062] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=126 ticks
[56607903759] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[56609243196] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[56612647608] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56616482076] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56640524259] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[56643573360] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56651497683] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56659757913] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=128 ticks
[56661082170] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[56662433223] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[56664866049] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56666344482] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56667730746] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
[56696094213] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: unblocked
[56698081770] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56701169217] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56744185542] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[56749455774] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56750921370] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56752442967] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=131 ticks
[56753890545] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[56755331160] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[56757545889] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56758912122] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56760260139] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
[56832433383] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: unblocked
[56834541291] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56835895116] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56859791736] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[56861300793] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56862640824] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56864162124] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=134 ticks
[56865625839] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[56867085726] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[56869381305] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[56870735229] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[56872365759] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
[58471895031] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=13 PID=13
[59797585386] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=15 PID=15
[59923400394] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=16 PID=16
[59934977487] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[59937022398] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing primary port (0x1F0)...
[59939967912] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=5 token=1
[59941836933] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=1 object=6 token=2
[59944029585] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=6 token=3
[59945789706] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=15 specs_ptr=1010228 count=3 timeout=0 ticks
[59947212468] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=5 flags=1 token=1
[59948567382] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=1 object=6 flags=1 token=2
[59949701130] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive(base=1f0, slave=false) - selecting drive a0
[59950421190] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[2]: kind=7 object=6 flags=1 token=3
[59952989778] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=5 token=1
[59954359311] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=1 object=6 token=2
[59955891138] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=6 token=3
[59957262849] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
[60008618472] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive - initial status: ff
[60031261917] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive(base=1f0, slave=true) - selecting drive b0
[60044742483] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive - initial status: ff
[60065621352] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing secondary port (0x170)...
[60066538323] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive(base=170, slave=false) - selecting drive a0
[60085598067] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive - initial status: ff
[60098914458] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive(base=170, slave=true) - selecting drive b0
[60115121022] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive - initial status: ff
[60129712962] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[60131165424] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Entering RPC service loop
[60150923019] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: VFS provider mounted at /dev/ata_ctl
[60219054357] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: No active devices to service
[60223624362] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Entering main service loop...
[60225056034] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Calling svc.next_event()...
[60226937364] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=6 token=1
[60228370785] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=1 object=4 token=2
[60229720089] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] sys_wait_many: tid=16 specs_ptr=1010048 count=2 timeout=0 ticks
[60231048306] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1]   spec[0]: kind=7 object=6 flags=1 token=1
[60232337847] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1]   spec[1]: kind=1 object=4 flags=1 token=2
[60234230694] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=6 token=1
[60235479249] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=1 object=4 token=2
[60236663784] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] sys_wait_many: blocking...
[6038982
```
</details>
