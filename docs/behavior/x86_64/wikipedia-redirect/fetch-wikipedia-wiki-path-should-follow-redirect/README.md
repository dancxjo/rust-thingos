# ❌ Scenario: Fetch Wikipedia /wiki path should follow redirect

> Last run: 2026-04-24 08:41:25

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 10231ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 355ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https en.wikipedia.org /https/wp" on the serial console | ✅ | 998ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ✅ | 0ms | - - - |
| 5 | And I type "cat /https/wp/wiki/@index" on the serial console | ✅ | 536ms | - [📜](./05/serial.log) - |
| 6 | Then the command output should contain "Main Page" | ❌ | 6095ms | - [📜](./06/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[31770547017] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[31801511346] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[31843767318] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[31882043160] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[31904592357] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[31907690661] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[31950627192] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[31973797152] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[32032069773] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[32033633016] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[32131215369] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[32219791197] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[32248317090] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[32352591414] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[32480241915] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[32543019333] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[32580964713] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[32751198942] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[32839151004] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[33046585341] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[33356532957] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1298936463 elapsed_us=649468
[33357620472] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[33441428922] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[33447941703] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[33501594225] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[33553022844] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[33569429190] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[33572148159] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[33677961570] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[3368279[33697960428] [[32mINFO [0m9568] [[32] [sh] [CPU3] SH: starting v0.1.0-debug
mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[33685175799] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[33709564977] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=39369
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[33972330678] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[33974784492] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[34067557491] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[34551354387] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=5, bind_id=322371585)
[34599897156] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[34612414419] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=5)
[34617505197] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Starting VFS-native VirtIO GPU driver...
[34618943568] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: boot_arg=5
[34620806319] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Mapping bootstrap memfd 5 size=4096...
[34625139384] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: vm_map success at 0x400000001000
[34628943459] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Recovered handles: req_read=4, resp_write=5, svc=1, id=322371585
[34630758228] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting (drv_req_r=4, drv_resp_w=5, svc=1, id=322371585)
[34663937583] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=10)
[34765799739] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[34777461147] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[34784160213] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[34785156945] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Virgl 3D not supported, using 2D only
[34795275306] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=1
[34925503668] [[32mINFO [0m] [bristle] [CPU1] bristle: published pid 10 to /run/bristle/pid
[34942754121] [[32mINFO [0m] [bristle] [CPU1] bristle: published device handles kbd_in=7 mouse_in=9
[34948421673] [[32mINFO [0m] [bristle] [CPU1] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[35587858845] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: frame pool ready (1 buffer)
[35611995342] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_BIND_READY (ID: 322371585)
[35614470045] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Waiting for BIND_ASSIGNED...
mount -t[35638152792] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: BIND_READY from instance_id=0x13370001
 [35639878923] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Registering DISPLAY_CARD at /dev/display/card0 (handle=7)
https en.w[35699872329] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[35702241531] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Handshake complete ��� task 'display' marked ready
i[35752538052] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[35755865937] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_SERVICE_READY.
kipedia.org /https/wp
[36900369594] [[32mINFO [0m] [sh] [CPU2] sh: spawning job cmd='mount'
[36918484548] [[32mINFO [0m] [sh] [CPU2] sh: spawning '/bin/mount' with argv=["/bin/mount", "-t", "https", "en.wikipedia.org", "/https/wp"]
[37017330471] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU2] SYSCALL SPAWN_PROCESS_EX: name='/bin/mount' TID=11 PID=11
[37331572179] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU2] SYSCALL SPAWN_PROCESS_EX: name='/bin/httpsd' TID=12 PID=12
[37351432404] [[32mINFO [0m] [sh] [CPU2] sh: cleaning up 0 pipes
[?25l[37450283805] [[32mINFO [0m] [httpsd] [CPU3] HTTPSD_READY
[37452278985] [[32mINFO [0m] [httpsd] [CPU3] httpsd: entering main RPC loop for /https/wp
[37698427338] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=4
[37700254383] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path=''
[37701584415] [[32mINFO [0m] [httpsd] [CPU3] httpsd: resolve_path path=''
[37750000596] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Close payload_len=8
mounted type=https device=en.wikipedia.org target=/https/wp
[37856613003] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=14 PID=14
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[37933393542] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=15 PID=15
[38008317006] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=16 PID=16
[38020545222] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[38022231588] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing primary port (0x1F0)...
cat[38098534551] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing secondary port (0x170)...
[38151281289] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[38153047680] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Entering RPC service loop
 [38170196856] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: VFS provider mounted at /dev/ata_ctl
/[38251758666] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: No active devices to service
https/wp/wiki/@index
[39348781494] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[39354064893] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/cat' with argv=["/bin/cat", "/https/wp/wiki/@index"]
[39397908462] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/cat' TID=18 PID=18
[39401925552] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[39458786829] [[32mINFO [0m] [cat] [CPU3] cat: opening '/https/wp/wiki/@index'
[39472391739] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Lookup payload_len=15
[39473441931] [[32mINFO [0m] [httpsd] [CPU2] httpsd: dispatch_lookup path='wiki/@index'
[?25l[39474682038] [[32mINFO [0m] [httpsd] [CPU2] httpsd: resolve_path path='wiki/@index'
[39480256662] [[32mINFO [0m] [httpsd] [CPU2] httpsd: lookup 'wiki/@index' -> handle 2
[39497357493] [[32mINFO [0m] [cat] [CPU3] cat: opened '/https/wp/wiki/@index' fd=8
[39506443911] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Poll payload_len=12
[39522206064] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Read payload_len=20
[39553415154] [[32mINFO [0m] [httpsd] [CPU1] httpsd: worker started for handle=2 url=https://en.wikipedia.org/wiki
[39556797027] [[32mINFO [0m] [httpsd] [CPU1] httpsd: ensure_upstream handle=2 MISS https://en.wikipedia.org/wiki - opening network stream
[39583260849] [[32mINFO [0m] [http] [CPU1] http: waiting for header data from port (attempt=0/120)
[39588155871] [[32mINFO [0m] [http] [CPU2] http: connect host=en.wikipedia.org port=443
[39589067661] [[32mINFO [0m] [http] [CPU2] http: opening /net/tcp/new
[39596792796] [[32mINFO [0m] [http] [CPU1] http: received 61 bytes from background TLS thread
[39600257895] [[33mWARN [0m] [httpsd] [CPU1] httpsd: upstream open failed for handle=2 https://en.wikipedia.org/wiki: http connect failed: failed to open /net/tcp/new: ENOENT
cat: error reading /https/wp/wiki/@index
[39642103677] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Close payload_len=8
[39645242208] [[32mINFO [0m] [httpsd] [CPU1] httpsd: worker exiting for handle=2
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[52236991587] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU0] VFS RPC: tid=5 req_id=1 op=Lookup TIMEOUT
[52314035763] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[57015349347] [[32mINFO [0m] [netd] [CPU3] NETD: Starting network service...
[57091694880] [[32mINFO [0m] [netd] [CPU3] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[57295312734] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/netd' TID=21 PID=21
[57300483306] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=21)
[58522445784] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready

```
</details>
