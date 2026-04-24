# ❌ Scenario: 3-command pipeline with cat as middle filter

> Last run: 2026-04-24 11:50:15

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 17374ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 459ms | - [📜](./02/serial.log) - |
| 3 | And I type "printf 'a\nb\nc\n' | cat | wc -l" on the serial console | ✅ | 726ms | - [📜](./03/serial.log) - |
| 4 | Then the command output should strictly be "3" | ❌ | 6042ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[54014846655] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[54066960948] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[54140127525] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[54212084553] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[54253685871] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[54259656000] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[54330694374] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[54370436868] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[54465179571] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[54467389449] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[54634289622] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[54751162323] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[54793177197] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[54912403491] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[55116871821] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[55241574366] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[55311893967] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[55563089835] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[55684144824] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[56045963028] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[56637707082] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=2129499471 elapsed_us=1064749
[56639144727] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[56840679687] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[56844346779] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[56943469473] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[57054803454] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[57083771580] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[57085520052] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[57407463795] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[57429093777] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[57454437843] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[57459253632] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[57483022245] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=40524
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[57845168205] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[57854554098] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[57914140647] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[57926137071] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[57932473929] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=5 specs_ptr=10002f8 count=2 timeout=35 ticks
[57942736170] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=3 flags=1 token=1
[57944276709] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=4 flags=1 token=2
[57967682652] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[57969292128] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[57975333471] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[57983517108] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
pr[58348435530] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[58385098497] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
i[58397508774] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[58405472367] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[58407174177] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[58413624060] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[58415364744] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
n[58467351294] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=5, bind_id=322371585)
t[58524020280] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[58525791588] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[58545940728] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=5)
[58547922906] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Starting VFS-native VirtIO GPU driver...
[58567280442] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: boot_arg=5
[58569687693] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Mapping bootstrap memfd 5 size=4096...
[58575441177] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=10)
[58577483679] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: vm_map success at 0x400000001000
[58579490244] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Recovered handles: req_read=4, resp_write=5, svc=1, id=322371585
[58603412868] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting (drv_req_r=4, drv_resp_w=5, svc=1, id=322371585)
[58605960039] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[58608244662] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[58617520566] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[58622167956] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[58623691533] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[58641013530] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[58650626496] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[58652149842] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[58653522246] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=5 specs_ptr=10002f8 count=2 timeout=53 ticks
[58654833633] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=3 flags=1 token=1
[58656101262] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=4 flags=1 token=2
[58664495505] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[58665798510] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[58667310504] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[58864037826] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[58900073562] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[58935403725] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[58937148303] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Virgl 3D not supported, using 2D only
[58953188976] [[32mINFO [0m] [bristle] [CPU1] bristle: published pid 10 to /run/bristle/pid
[58960112211] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=1
[58984442451] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[58986331140] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[58987670544] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[59000295717] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[59005650990] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[59008081902] [[32mINFO [0m] [bristle] [CPU1] bristle: published device handles kbd_in=7 mouse_in=9
[59017439547] [[32mINFO [0m] [bristle] [CPU1] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[59019519174] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=6 token=1
[59020922103] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=7 token=2
[59022262695] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=8 token=3
[59023218672] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[59023905171] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] sys_wait_many: tid=10 specs_ptr=1000000 count=3 timeout=0 ticks
[59024873853] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[59026787985] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1]   spec[0]: kind=7 object=6 flags=1 token=1
[59027731125] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[59028328491] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1]   spec[1]: kind=7 object=7 flags=1 token=2
[59029348818] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[59030011392] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1]   spec[2]: kind=7 object=8 flags=1 token=3
[59030920047] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[59031995583] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=6 token=1
[59032969446] [[32mINFO [0m] [sprout::super[kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=8 token=3
[59036269347] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] sys_wait_many: blocking...
[59036960103] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[59039092464] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[59043772029] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[59046037974] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[59059697697] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[59060963082] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=5 visor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[59033612979] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=7 token=2
[59034928095] [[32mINFO [0m] specs_ptr=10002f8 count=2 timeout=65 ticks
[59062268958] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=3 flags=1 token=1
[59063490486] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=4 flags=1 token=2
[59065569981] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[59066831406] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[59076985077] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[59410525515] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[59412490830] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[59413788885] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[59416666089] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[59442277158] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[59446734237] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[59448025659] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[59449239993] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[59460968490] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[59462494509] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[59463801012] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[59485946982] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[59494857279] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[59498683134] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[59516928900] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[59518584147] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[59519923518] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=5 specs_ptr=10002f8 count=2 timeout=78 ticks
[59521210914] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=3 flags=1 token=1
[59522485968] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=4 flags=1 token=2
[59524781415] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[59533085337] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[59538035436] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[59834350488] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[59836390515] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[59837765856] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[59840520366] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[59841834756] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[59856516918] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[59859937269] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[59861273373] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[59862635679] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[59864808432] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[59866713918] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[59877363315] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[59889397062] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[59893757451] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[59895498399] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[59896873443] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[59898193113] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: tid=5 specs_ptr=10002f8 count=2 timeout=89 ticks
[59899491333] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[0]: kind=7 object=3 flags=1 token=1
[59900767014] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0]   spec[1]: kind=7 object=4 flags=1 token=2
[59909286723] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[59914189434] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[59915470494] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: blocking...
[60201178455] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: unblocked
[60203145552] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[60215517252] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[60218332944] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[60219117453] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[60223274529] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[60225725439] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[60243535011] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: frame pool ready (1 buffer)
[60246874149] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[60248351724] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[60249670272] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[60251731122] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[60256035147] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[60257481999] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[60271461327] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[60273405819] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=3 token=1
[60274835478] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] collect_ready: polling spec[0] kind=7 object=4 token=2
[60285851043] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU0] sys_wait_many: returning 1 results
f '[60327213111] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: BIND_READY from instance_id=0x13370001
a\nb\nc[60342564612] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Registering DISPLAY_CARD at /dev/display/card0 (handle=7)
\n' | cat | wc -l
[60395179647] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[60405443241] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Handshake complete — task 'display' marked ready
[60412429011] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='printf'
[60413479368] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Sent MSG_BIND_ASSIGNED to req_port 3 (res=Ok(92))
[60414887247] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[60416179758] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[60419561862] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[60420838797] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[60422448240] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[60423765699] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[60431600262] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/printf' with argv=["/bin/printf", "a\\nb\\nc\\n"]
[60433556106] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[60435010383] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[60460977555] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 2 pipes
[60464166147] [[32mINFO [0m] [sh] [CPU3] sh: closing pipe 0 ends: read=6 write=7
[60483224802] [[32mINFO [0m] [sh] [CPU3] sh: closing pipe 1 ends: read=8 write=9
sh: spawn failed: no such file or directory
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[63696732528] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=11 PID=11
[63840180888] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=12 PID=12
[63934455552] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=13 PID=13
[63949936644] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[63951805533] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing primary port (0x1F0)...
[63955342803] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive(base=1f0, slave=false) - selecting drive a0
[63974275761] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive - initial status: ff
[64006872897] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive(base=1f0, slave=true) - selecting drive b0
[64033709817] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive - initial status: ff
[64053377850] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=5 token=1
[64057345869] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=1 object=6 token=2
[64075734723] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing secondary port (0x170)...
[64077111285] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=6 token=3
[64078451217] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=12 specs_ptr=1010228 count=3 timeout=0 ticks
[64079366241] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive(base=170, slave=false) - selecting drive a0
[64079981955] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=5 flags=1 token=1
[64081318290] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=1 object=6 flags=1 token=2
[64082683368] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[2]: kind=7 object=6 flags=1 token=3
[64085377191] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=5 token=1
[64086790086] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=1 object=6 token=2
[64088021151] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=6 token=3
[64089278715] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: blocking...
[64096370943] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive - initial status: ff
[64117518993] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive(base=170, slave=true) - selecting drive b0
[64165805781] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: identify_drive - initial status: ff
[64182938094] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[64184609049] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Entering RPC service loop
[64212951924] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: VFS provider mounted at /dev/ata_ctl
[64272058983] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: No active devices to service
[64273681131] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Entering main service loop...
[64276218765] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Calling svc.next_event()...
[64278126429] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=6 token=1
[64279587372] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=1 object=4 token=2
[64280988288] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] sys_wait_many: tid=13 specs_ptr=1010048 count=2 timeout=0 ticks
[64282392306] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1]   spec[0]: kind=7 object=6 flags=1 token=1
[64283727090] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1]   spec[1]: kind=1 object=4 flags=1 token=2
[64292485884] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=6 token=1
[64294336953] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=1 object=4 token=2
[64299527028] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] sys_wait_many: blocking...
[64569086175] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=6 token=1
[64570671594] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=7 token=2
[64572993276] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] sys_wait_many: tid=7 specs_ptr=1000600 count=2 timeout=3220 ticks
[64574546619] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1]   spec[0]: kind=7 object=6 flags=1 token=1
[64575917802] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1]   spec[1]: kind=7 object=7 flags=1 token=2
[64594254252] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=6 token=1
[64596451953] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] collect_ready: polling spec[0] kind=7 object=7 token=2
[64598186433] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU1] sys_wait_many: blocking...
[76969982067] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_BIND_READY (ID: 322371585)
[76972059780] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Waiting for BIND_ASSIGNED...
[76977302820] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[76981293543] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_SERVICE_READY.
[76987659771] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_g[76995340620] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[77002353186] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[77007373179] [[32mINFO [0m] [kernel::syscallpu: waiting on WaitSet...
[76989546513] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[76991845656] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[76993248981] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=592 ticks
::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[77008710372] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[77037153897] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[77040466503] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[77042323413] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_rea[77056284954] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[77057848362] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[77066063250] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
dy: polling spec[0] kind=7 object=a token=2
[77087136126] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[77089445961] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[77092823412] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[77115670929] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: waiting on WaitSet...
[77117266776] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[77118813156] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=a token=2
[77120268225] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] sys_wait_many: tid=9 specs_ptr=10000f8 count=2 timeout=596 ticks
[77122307955] [[32mINFO [0m] [kernel::syscall::handlers::wait]G [CPU3]   spec[0]: kind=7 object=8 flags=1 token=1
[77123658711] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3]   spec[1]: kind=7 object=a flags=1 token=2
[77126297292] [[32mINFO [0m] [kernel::syscall::handlers::wait] [CPU3] collect_ready: polling spec[0] kind=7 object=8 token=1
[7
```
</details>
