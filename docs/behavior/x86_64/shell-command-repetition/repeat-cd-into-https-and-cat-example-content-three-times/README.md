# ❌ Scenario: Repeat cd into https and cat example content three times

> Last run: 2026-04-24 08:41:25

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 10540ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 253ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https example.com /https/ex" on the serial console | ✅ | 870ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ✅ | 0ms | - - - |
| 5 | And I type "cat /https/ex/@index" on the serial console | ✅ | 499ms | - [📜](./05/serial.log) - |
| 6 | And I type "cat /https/ex/@index" on the serial console | ✅ | 449ms | - [📜](./06/serial.log) - |
| 7 | And I type "cat /https/ex/@index" on the serial console | ✅ | 445ms | - [📜](./07/serial.log) - |
| 8 | Then the command output should contain "<html" | ❌ | 6091ms | - [📜](./08/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[32467944762] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[32498254635] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[32537880309] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[32591215899] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[32624728653] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[32629342218] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[32691933714] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[32725414392] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[32811226437] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[32813257125] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[32962547112] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[33053427891] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[33078351042] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[33159390198] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[33269881590] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[33331262943] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[33358925292] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[33477264084] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[33554964729] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[33786363765] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[34217161011] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1369549599 elapsed_us=684774
[34218596940] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[34346196159] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[34350449298] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[34395420312] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[34458143511] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[34476103233] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[34478954103] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[34630108293] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[34639346115] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[34644680862] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[1;32m
        .-.
       /   \        [1;36mTHING-OS[1;32m
      |     |       [0;36m"People, places, things."[1;32m
       \   /        
        `-'        
       /   \        v0.1  ��  
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
[34714434315] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[34727833800] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=23100
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[35052926271] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[35055210036] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[35132838576] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
moun[35507885028] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=5, bind_id=322371585)
t[35579395665] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[35592003282] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=5)
[35594158380] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Starting VFS-native VirtIO GPU driver...
[35595672750] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: boot_arg=5
[35597723205] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Mapping bootstrap memfd 5 size=4096...
[35601590508] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: vm_map success at 0x400000001000
[35603690232] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Recovered handles: req_read=4, resp_write=5, svc=1, id=322371585
[35605446591] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting (drv_req_r=4, drv_resp_w=5, svc=1, id=322371585)
[35613377613] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=10)
[35768842494] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[35781025665] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[35788669455] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[35789650611] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Virgl 3D not supported, using 2D only
[35798434947] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=1
[35838328020] [[32mINFO [0m] [bristle] [CPU1] bristle: published pid 10 to /run/bristle/pid
[35859311796] [[32mINFO [0m] [bristle] [CPU1] bristle: published device handles kbd_in=7 mouse_in=9
[35869807314] [[32mINFO [0m] [bristle] [CPU1] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[36454430331] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: frame pool ready (1 buffer)
[36478899996] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_BIND_READY (ID: 322371585)
[36480796473] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Waiting for BIND_ASSIGNED...
 -t https [36505631415] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: BIND_READY from instance_id=0x13370001
[36507518718] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Registering DISPLAY_CARD at /dev/display/card0 (handle=7)
example.[36542364375] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[36544656588] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Handshake complete — task 'display' marked ready
[36565028610] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[36569988081] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_SERVICE_READY.
com /https/ex
[37288191930] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='mount'
[37300509213] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/mount' with argv=["/bin/mount", "-t", "https", "example.com", "/https/ex"]
[37382370399] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/mount' TID=11 PID=11
[37394333592] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25l[37697076714] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU2] SYSCALL SPAWN_PROCESS_EX: name='/bin/httpsd' TID=12 PID=12
[37800997872] [[32mINFO [0m] [httpsd] [CPU3] HTTPSD_READY
[37803394068] [[32mINFO [0m] [httpsd] [CPU3] httpsd: entering main RPC loop for /https/ex
[38038794255] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=4
[38040968724] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path=''
[38042376966] [[32mINFO [0m] [httpsd] [CPU3] httpsd: resolve_path path=''
[38091655866] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Close paylomounted type=https device=example.com target=/https/ex
ad_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/ex/@ind[39115872411] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=14 PID=14
e[39199088049] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=15 PID=15
[39279720513] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=16 PID=16
[39295206225] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[39296997267] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing primary port (0x1F0)...
x
[39330372015] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[39335704551] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/cat' with argv=["/bin/cat", "/https/ex/@index"]
[39359059443] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing secondary port (0x170)...
[39384096609] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/cat' TID=18 PID=18
[39387902631] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25l[39399450222] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[39400379040] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Entering RPC service loop
[39412976658] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: VFS provider mounted at /dev/ata_ctl
[39444848223] [[32mINFO [0m] [cat] [CPU3] cat: opening '/https/ex/@index'
[39447859473] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: No active devices to service
[39495904701] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=10
[39497466426] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path='@index'
[39498752766] [[32mINFO [0m] [httpsd] [CPU3] httpsd: resolve_path path='@index'
[39524462340] [[32mINFO [0m] [cat] [CPU3] cat: opened '/https/ex/@index' fd=8
[39542644515] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Poll payload_len=12
[39572813511] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[39597983337] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/ex/@index
[40928012961] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[40933206765] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/cat' with argv=["/bin/cat", "/https/ex/@index"]
[40973838246] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/cat' TID=19 PID=19
[40979023239] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25l[41045296941] [[32mINFO [0m] [cat] [CPU1] cat: opening '/https/ex/@index'
[41060647782] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Lookup payload_len=10
[41062050249] [[32mINFO [0m] [httpsd] [CPU1] httpsd: dispatch_lookup path='@index'
[41063316162] [[32mINFO [0m] [httpsd] [CPU1] httpsd: resolve_path path='@index'
[41081275785] [[32mINFO [0m] [cat] [CPU1] cat: opened '/https/ex/@index' fd=9
[41094966693] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Poll payload_len=12
[41127022233] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[41163677841] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/ex/@index
[42399109335] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[42405088737] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/cat' with argv=["/bin/cat", "/https/ex/@index"]
[42446145852] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/cat' TID=20 PID=20
[42450543630] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25l[42520165677] [[32mINFO [0m] [cat] [CPU2] cat: opening '/https/ex/@index'
[42531162432] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Lookup payload_len=10
[42533250738] [[32mINFO [0m] [httpsd] [CPU1] httpsd: dispatch_lookup path='@index'
[42534577635] [[32mINFO [0m] [httpsd] [CPU1] httpsd: resolve_path path='@index'
[42578427705] [[32mINFO [0m] [cat] [CPU2] cat: opened '/https/ex/@index' fd=10
[42587864583] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Poll payload_len=12
[42604528230] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[42622656516] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[54003481950] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU0] VFS RPC: tid=5 req_id=1 op=Lookup TIMEOUT
[54122240469] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[59071409166] [[32mINFO [0m] [netd] [CPU3] NETD: Starting network service...
[59141347353] [[32mINFO [0m] [netd] [CPU3] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[59350516302] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/netd' TID=21 PID=21
[59358542133] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=21)
[
```
</details>
