# ❌ Scenario: 3-command pipeline with cat as middle filter

> Last run: 2026-04-24 17:40:56

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 19152ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 568ms | - [📜](./02/serial.log) - |
| 3 | And I type "printf 'a\nb\nc\n' | cat | wc -l" on the serial console | ✅ | 1461ms | - [📜](./03/serial.log) - |
| 4 | Then the command output should strictly be "3" | ❌ | 6078ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[59128906089] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[59209614948] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[59334842556] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[59449487427] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[59512810830] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[59517470067] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[59655154911] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[59719999119] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[59777614380] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[59790351687] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[59911140201] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[59915488710] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[60178812177] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[60299455326] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[60345344895] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[60527435661] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[60713032413] [[32mINFO [0m] [kernel::boot_progress] [CPU0] b[60856534530] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
oot_progress: milestone="SMP Bring-up"
[60933609264] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[61171564542] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[61334069346] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[61601279421] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[62342605347] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=2353755096 elapsed_us=1176877
[62343921156] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[62676453378] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[62704767279] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[62858881239] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[63177698727] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[63213750204] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[63219392115] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[63497009565] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[63506332296] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[63529106850] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[63535240263] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[63580014597] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=36366
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[64318745799] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[64321083321] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
printf '[65068071189] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet ��� retrying
a\n[65273036070] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[65305138833] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
b\[65360642622] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[65362543950] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
nc\[65617805550] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=5, bind_id=322371585)
n' [65824644270] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[65826428580] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[65991321165] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=5)
[66018863295] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Starting VFS-native VirtIO GPU driver...
[66020469372] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: boot_arg=5
[66022347501] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Mapping bootstrap memfd 5 size=4096...
[66051007902] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=10)
[66055665753] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[66056948925] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[66060704259] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: vm_map success at 0x400000001000
[66062655813] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Recovered handles: req_read=4, resp_write=5, svc=1, id=322371585
[66066084513] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting (drv_req_r=4, drv_resp_w=5, svc=1, id=322371585)
[66100778931] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[66105666264] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[66137420448] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[66170576010] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[66630448137] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[66632001777] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[66719897178] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[66721452864] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[66722748576] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[66742208082] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[66743550819] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[66750867348] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[66755121378] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[66756368745] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[66760465530] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[66807456243] [[32mINFO [0m] [bristle] [CPU1] bristle: published pid 10 to /run/bristle/pid
[66831801399] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[66846041163] [[32mINFO [0m] [bristle] [CPU1] bristle: published device handles kbd_in=7 mouse_in=9
[66862715007] [[32mINFO [0m] [bristle] [CPU1] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[66966744900] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[67007175312] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[67008855606] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Virgl 3D not supported, using 2D only
[67061166282] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=1
[67086198432] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[67090757316] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[67101358038] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[67108203954] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[67109408850] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[67110495903] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[67111600776] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[67112742873] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[67135288341] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[67136808024] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[67197071469] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[67627487994] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[67628871486] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[67646277963] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[67663619661] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[67665186468] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[67666391034] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[67667569662] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[67669478052] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[67698441162] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[67708326543] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[67727725197] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[68060545773] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[68071725579] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[68076128934] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[68077461771] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[68078592252] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[68079709632] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[68080886841] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[68082114276] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[68109260637] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[68110494771] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[68113934658] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[68423024043] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[68424352557] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[68428297212] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[68443232121] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[68444504568] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[68451560529] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[68452326921] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[68453620158] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[68457456639] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: path_exists returned false
[68458647147] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: run_health_vine
[68462129472] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[68567601465] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: frame pool ready (1 buffer)
[68596097889] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_BIND_READY (ID: 322371585)
[68598469203] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Waiting for BIND_ASSIGNED...
| tac | wc -
[68656448850] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: BIND_READY from instance_id=0x13370001
[68658215571] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Registering DISPLAY_CARD at /dev/display/card0 (handle=7)
[68715458724] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='printf'
[68739203181] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[68760279555] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Handshake complete — task 'display' marked ready
[68774694615] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/printf' with argv=["/bin/printf", "a\\nb\\nc\\n"]
[68777972934] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Sent MSG_BIND_ASSIGNED to req_port 3 (res=Ok(92))
[68782024278] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[68783209176] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_netd_if_ready
[68790804324] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: verify_netd_liveness
[68792167686] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[68793357963] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[68794538736] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[68795742114] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[68797492368] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Calling path_exists(/dev/display/card0)...
[68963453559] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/printf' TID=11 PID=11
[68988497325] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[68991555534] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_SERVICE_READY.
[69074984121] [[32mINFO [0m] [kernel::ipc::pipe] [CPU2] PIPE_ENQUEUE: tail=6 len=6 n=6
[69105727185] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='tac'
[69112867329] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/tac' with argv=["/bin/tac"]
[69131652777] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 2 pipes
[69133051845] [[32mINFO [0m] [sh] [CPU3] sh: closing pipe 0 ends: read=6 write=7
[69136263075] [[32mINFO [0m] [sh] [CPU3] sh: closing pipe 1 ends: read=8 write=9
sh: spawn failed: no such file or directory
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hl[72239059101] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=12 PID=12
[72395109732] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=13 PID=13
[73103351145] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[73104982896] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[73111383345] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=15 PID=15
[73118522697] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: identify_drive(base=1f0, slave=false) - selecting drive a0
[73216400499] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: identify_drive - initial status: ff
[73236462486] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: identify_drive(base=1f0, slave=true) - selecting drive b0
[73274776146] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: identify_drive - initial status: ff
[73351048716] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
[73352585427] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: identify_drive(base=170, slave=false) - selecting drive a0
[73444240551] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: identify_drive - initial status: ff
[73551416070] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: identify_drive(base=170, slave=true) - selecting drive b0
[73575711660] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: identify_drive - initial status: ff
[73589896710] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[73591393326] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[73625372238] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: VFS provider mounted at /dev/ata_ctl
[73885030659] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
[73888160907] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering main service loop...
[73897965669] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Calling svc.next_event()...

```
</details>
