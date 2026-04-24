# ✅ Scenario: Round trip with mutable attributes

> Last run: 2026-04-24 08:20:54

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 10236ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 355ms | - [📜](./02/serial.log) - |
| 3 | And I type "ls /bin" on the serial console | ✅ | 541ms | - [📜](./03/serial.log) - |
| 4 | And I wait for 2 seconds | ✅ | 2001ms | - [📜](./04/serial.log) - |
| 5 | And I type "ipc_provider_demo &" on the serial console | ✅ | 377ms | - [📜](./05/serial.log) - |
| 6 | And I wait for 2 seconds | ✅ | 2001ms | - - - |
| 7 | And I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 667ms | - [📜](./07/serial.log) - |
| 8 | Then the command output should contain "provider.name" | ✅ | 0ms | - - - |
| 9 | And the command output should contain "provider.is_demo" | ✅ | 0ms | - - - |
| 10 | And the command output should contain "provider.requests" | ✅ | 0ms | - - - |
| 11 | When I type "attr_set /run/cookbook/hello.txt user.comment utf8 This_is_a_test" on the serial console | ✅ | 1178ms | - [📜](./11/serial.log) - |
| 12 | And I wait for 1 seconds | ✅ | 1001ms | - [📜](./12/serial.log) - |
| 13 | And I type "attr_get /run/cookbook/hello.txt user.comment" on the serial console | ✅ | 863ms | - [📜](./13/serial.log) - |
| 14 | Then the command output should contain "type=Utf8 len=14" | ✅ | 0ms | - - - |
| 15 | And the command output should contain "value=This_is_a_test" | ✅ | 0ms | - - - |
| 16 | When I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 665ms | - [📜](./16/serial.log) - |
| 17 | Then the command output should contain "user.comment" | ✅ | 0ms | - - - |
| 18 | When I type "attr_rm /run/cookbook/hello.txt user.comment" on the serial console | ✅ | 10786ms | - [📜](./18/serial.log) - |
| 19 | And I wait for 1 seconds | ✅ | 999ms | - - - |
| 20 | And I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 4618ms | - [📜](./20/serial.log) - |
| 21 | Then the latest serial output should not contain "user.comment" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[31538251074] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[31583911623] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[31657152978] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[31714432827] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[31749507339] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[31754340222] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[31817323395] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[31852685997] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[31929140958] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[31930490889] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[32051711307] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[32146793217] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[32174423424] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[32256485151] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[32366715612] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[32432451348] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[32475588816] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[32626121121] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[327boot_progr13636098] [[32emINFO [0m] [kernel::boot_progress] [CPU0] ss: milestone="Spawning Sprout"
[32944841160] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[33349918272] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1395106185 elapsed_us=697553
[33351318660] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[33450372219] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[33452445774] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[33522605127] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[33580372254] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[33592190643] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[33593380458] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[33751152765] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[33761132691] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[33764174598] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[33764896242] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[33785451348] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=39006
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[34096604850] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[34099166937] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[34176415350] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[34493776053] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=5, bind_id=322371585)
[34543278297] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[34555192254] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=5)
[34557118497] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Starting VFS-native VirtIO GPU driver...
[34558798098] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: boot_arg=5
[34560846573] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Mapping bootstrap memfd 5 size=4096...
[34564954842] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: vm_map success at 0x400000001000
[34567067469] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Recovered handles: req_read=4, resp_write=5, svc=1, id=322371585
[34568928603] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting (drv_req_r=4, drv_resp_w=5, svc=1, id=322371585)
[34575656478] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=10)
[34763784396] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[34782175395] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[34792744470] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[34794341769] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Virgl 3D not supported, using 2D only
[34807319415] [[32mINFO [0m] [bristle] [CPU1] bristle: published pid 10 to /run/bristle/pid
[34808763330] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=1
[34826497497] [[32mINFO [0m] [bristle] [CPU1] bristle: published device handles kbd_in=7 mouse_in=9
[34836216294] [[32mINFO [0m] [bristle] [CPU1] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[35632142139] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: frame pool ready (1 buffer)
[35655662790] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_BIND_READY (ID: 322371585)
[35657504355] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Waiting for BIND_ASSIGNED...
ls /bi[35681853471] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: BIND_READY from instance_id=0x13370001
n[35683629531] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Registering DISPLAY_CARD at /dev/display/card0 (handle=7)

[35696449503] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[35704611393] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[35708328810] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Handshake complete — task 'display' marked ready
[35722235670] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/ls' with argv=["/bin/ls", "/bin"]
[35733168636] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[35736291360] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_SERVICE_READY.
[35806043262] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/ls' TID=11 PID=11
[35874725601] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25lattr_get  attr_list  attr_rm  attr_set  basename  bloom  bristle  cambium  cat  clear  cp  cwd_test  date  dirname  dmesg  echo  env  env_roundtrip  false  fetchd  file  find  grep  head  httpsd  input_echo  ip  ipc_memfd_demo  ipc_pipe_demo  ipc_provider_demo  ipc_service_demo  iso9660d  iso_reader  kill  killall  ld_so  ln  loglevel  ls  mdns  mdnsd  mesocarp  mkdir  mount  mv  netd  nslookup  ping  placed  poll_mux  ps  pwd  reboot  rm  rmdir  setshell  sh  show_args  shutdown  sleep  sort  sprout  stat  tail  tee  terminal  test_dlopen  test_dyn_loader  test_exec  test_exec_env  test_futex  test_threads  test_vm_protect  top  touch  true  uname  vfs_hello  wayland_hello  wc  which  xargs  yes  
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[38029778985] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=12 PID=12
[38104570185] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=13 PID=13
[38191852479] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=14 PID=14
[38200509039] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[38202396177] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Probing primary port (0x1F0)...
[38278234302] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Probing secondary port (0x170)...
[38340776595] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[38346149325] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Entering RPC service loop
[38389503174] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: VFS provider mounted at /dev/ata_ctl
[38447379795] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: No active devices to service
ipc_provider_demo &
[44100137928] [[32mINFO [0m] [sh] [CPU1] sh: spawning job cmd='ipc_provider_demo'
[44104646949] [[32mINFO [0m] [sh] [CPU1] sh: spawning '/bin/ipc_provider_demo' with argv=["/bin/ipc_provider_demo"]
[44158871460] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/bin/ipc_provider_demo' TID=16 PID=16
[44165076153] [[32mINFO [0m] [sh] [CPU1] sh: cleaning up 0 pipes
[2] 16
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[44180127354] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: starting up
[44190455694] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: port pair write_h=3 read_h=4
[44205770664] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: mounted at /run/cookbook
attr_list /run/cookbook/[52226927412] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU0] VFS RPC: tid=5 req_id=1 op=Lookup TIMEOUT
hell[52378568715] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
o.txt
[52706832948] [[32mINFO [0m] [sh] [CPU1] sh: spawning job cmd='attr_list'
[52712124234] [[32mINFO [0m] [sh] [CPU1] sh: spawning '/bin/attr_list' with argv=["/bin/attr_list", "/run/cookbook/hello.txt"]
[52752031761] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_list' TID=17 PID=17
[52756156926] [[32mINFO [0m] [sh] [CPU1] sh: cleaning up 0 pipes
[?25l[52841536572] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #1 op=Lookup
[52872363291] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #2 op=AttrList
provider.is_demo	Bool	1
provider.name	Utf8	17
provider.requests	U64	8
count=3
[52919765481] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #3 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_set /run/cookbook/hello.txt user.comment utf8 This_is_a_test
[56619272820] [[32mINFO [0m] [sh] [CPU1] sh: spawning job cmd='attr_set'
[56625205098] [[32mINFO [0m] [sh] [CPU1] sh: spawning '/bin/attr_set' with argv=["/bin/attr_set", "/run/cookbook/hello.txt", "user.comment", "utf8", "This_is_a_test"]
[56682250482] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_set' TID=18 PID=18
[56687015682] [[32mINFO [0m] [sh] [CPU1] sh: cleaning up 0 pipes
[?25l[56760771903] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #4 op=Lookup
[56821108773] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #5 op=AttrSet
[56823565359] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: set attr 'user.comment' type=Utf8 len=14
attr_set: wrote 0
[56858524338] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #6 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[57106245207] [[32mINFO [0m] [netd] [CPU1] NETD: Starting network service...
[57170491191] [[32mINFO [0m] [netd] [CPU1] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[57426253962] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/netd' TID=19 PID=19
[57431519805] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=19)
[58096027671] [[32mINFO [0m] [netd] [CPU1] NETD: Network ready
attr_get /run/cookbook/hello.txt user.comment
[62764685181] [[32mINFO [0m] [sh] [CPU2] sh: spawning job cmd='attr_get'
[62770362831] [[32mINFO [0m] [sh] [CPU2] sh: spawning '/bin/attr_get' with argv=["/bin/attr_get", "/run/cookbook/hello.txt", "user.comment"]
[62823573813] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU2] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_get' TID=21 PID=21
[62895367788] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #7 op=Lookup
[62928071448] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #8 op=AttrGet
type=Utf8 len=14
value=This_is_a_test
[62959764219] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #9 op=Close
[63001849944] [[32mINFO [0m] [sh] [CPU2] sh: cleaning up 0 pipes
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_list /run/cookbook/hello.txt
[64959324870] [[32mINFO [0m] [sh] [CPU2] sh: spawning job cmd='attr_list'
[64967492931] [[32mINFO [0m] [sh] [CPU2] sh: spawning '/bin/attr_list' with argv=["/bin/attr_list", "/run/cookbook/hello.txt"]
[65013629901] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU2] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_list' TID=22 PID=22
[65089015464] [[32mINFO [0m] [ipc_provider_demo] [CPU0] ipc_provider_demo: request #10 op=Lookup
[65123731497] [[32mINFO [0m] [ipc_provider_demo] [CPU0] ipc_provider_demo: request #11 op=AttrList
provider.is_demo	Bool	1
provider.name	Utf8	17
provider.requests	U64	8
user.comment	Utf8	14
count=4
[65159714730] [[32mINFO [0m] [sh] [CPU2] sh: cleaning up 0 pipes
[?25l[65174189058] [[32mINFO [0m] [ipc_provider_demo] [CPU0] ipc_provider_demo: request #12 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_rm /run/cookbook/hello.txt user.comment
[67725856638] [[32mINFO [0m] [sh] [CPU2] sh: spawning job cmd='attr_rm'
[67731645498] [[32mINFO [0m] [sh] [CPU2] sh: spawning '/bin/attr_rm' with argv=["/bin/attr_rm", "/run/cookbook/hello.txt", "user.comment"]
[67776309216] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU2] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_rm' TID=23 PID=23
[67855684776] [[32mINFO [0m] [ipc_provider_demo] [CPU0] ipc_provider_demo: request #13 op=Lookup
[67885779885] [[32mINFO [0m] [sh] [CPU2] sh: cleaning up 0 pipes
[?25l[67898055918] [[32mINFO [0m] [ipc_provider_demo] [CPU0] ipc_provider_demo: request #14 op=AttrRemove
[67912489986] [[32mINFO [0m] [ipc_provider_demo] [CPU0] ipc_provider_demo: removed attr 'user.comment'
[67953484335] [[32mINFO [0m] [ipc_provider_demo] [CPU0] ipc_provider_demo: request #15 op=Close
[?25h[1;95mTH
```
</details>
