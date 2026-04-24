# ❌ Scenario: Round trip with mutable attributes

> Last run: 2026-04-24 08:41:25

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 9927ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 356ms | - [📜](./02/serial.log) - |
| 3 | And I type "ls /bin" on the serial console | ✅ | 442ms | - [📜](./03/serial.log) - |
| 4 | And I wait for 2 seconds | ✅ | 2001ms | - [📜](./04/serial.log) - |
| 5 | And I type "ipc_provider_demo &" on the serial console | ✅ | 382ms | - [📜](./05/serial.log) - |
| 6 | And I wait for 2 seconds | ✅ | 2001ms | - - - |
| 7 | And I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 1996ms | - [📜](./07/serial.log) - |
| 8 | Then the command output should contain "provider.name" | ✅ | 1ms | - - - |
| 9 | And the command output should contain "provider.is_demo" | ✅ | 0ms | - - - |
| 10 | And the command output should contain "provider.requests" | ✅ | 0ms | - - - |
| 11 | When I type "attr_set /run/cookbook/hello.txt user.comment utf8 This_is_a_test" on the serial console | ✅ | 1228ms | - [📜](./11/serial.log) - |
| 12 | And I wait for 1 seconds | ✅ | 1000ms | - [📜](./12/serial.log) - |
| 13 | And I type "attr_get /run/cookbook/hello.txt user.comment" on the serial console | ✅ | 861ms | - [📜](./13/serial.log) - |
| 14 | Then the command output should contain "type=Utf8 len=14" | ❌ | 6089ms | - [📜](./14/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[30816425838] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[30846799962] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[30891360984] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[30928027185] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[30951399798] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[30954731610] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[30997704045] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[31021029600] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[31080020070] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[31082089896] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[31231987446] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[31322078865] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[31359140571] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[31468848444] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[31595612136] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[31661289000] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[31699271604] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[31871014494] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[31958463009] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[32188833567] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[32480682630] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1362184890 elapsed_us=681092
[32481552939] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[32583839442] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[32586001701] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[32636542851] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[32684436246] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[32698062771] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[32699070657] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[32806938681] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[32811557526] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[32813856471] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[32822142243] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[32829136164] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=43032
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[33064668813] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[33067408506] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[33133337292] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[33448777197] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=5, bind_id=322371585)
[33482709117] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[33496170642] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=5)
[33498220767] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Starting VFS-native VirtIO GPU driver...
[33499750680] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: boot_arg=5
[33501771501] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Mapping bootstrap memfd 5 size=4096...
[33507376056] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: vm_map success at 0x400000001000
[33508914450] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=10)
[33509527953] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Recovered handles: req_read=4, resp_write=5, svc=1, id=322371585
[33511408656] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting (drv_req_r=4, drv_resp_w=5, svc=1, id=322371585)
[33676197456] [[32mINFO [0m] [bristle] [CPU1] bristle: published pid 10 to /run/bristle/pid
[33684415182] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[33689876550] [[32mINFO [0m] [bristle] [CPU1] bristle: published device handles kbd_in=7 mouse_in=9
[33695603667] [[32mINFO [0m] [bristle] [CPU1] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[33709261179] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[33723005085] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[33724683762] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Virgl 3D not supported, using 2D only
[33739670514] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=1
[34383228231] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: frame pool ready (1 buffer)
[34401957546] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_BIND_READY (ID: 322371585)
[34403639490] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Waiting for BIND_ASSIGNED...
ls /bi[34422954786] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: BIND_READY from instance_id=0x13370001
[34424547696] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Registering DISPLAY_CARD at /dev/display/card0 (handle=7)
n
[34438733934] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[34449132003] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/ls' with argv=["/bin/ls", "/bin"]
[34476854610] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[34479051816] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Handshake complete — task 'display' marked ready
[34533520329] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/ls' TID=11 PID=11
[34553748768] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[34557256701] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_SERVICE_READY.
[34573980771] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25lattr_get  attr_list  attr_rm  attr_set  basename  bloom  bristle  cambium  cat  clear  cp  cwd_test  date  dirname  dmesg  echo  env  env_roundtrip  false  fetchd  file  find  grep  head  httpsd  input_echo  ip  ipc_memfd_demo  ipc_pipe_demo  ipc_provider_demo  ipc_service_demo  iso9660d  iso_reader  kill  killall  ld_so  ln  loglevel  ls  mdns  mdnsd  mesocarp  mkdir  mount  mv  netd  nslookup  ping  placed  poll_mux  ps  pwd  reboot  rm  rmdir  setshell  sh  show_args  shutdown  sleep  sort  sprout  stat  tail  tee  terminal  test_dlopen  test_dyn_loader  test_exec  test_exec_env  test_futex  test_threads  test_vm_protect  top  touch  true  uname  vfs_hello  wayland_hello  wc  which  xargs  yes  
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[36571647684] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=12 PID=12
[36663880803] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=13 PID=13
[36862730091] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=15 PID=15
[36867403287] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[36868470672] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[36981541509] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
[37036673223] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[37038312498] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[37058383461] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: VFS provider mounted at /dev/ata_ctl
[37116340503] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
ipc_provider_demo &
[42800283009] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ipc_provider_demo'
[42805989237] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/ipc_provider_demo' with argv=["/bin/ipc_provider_demo"]
[42861991854] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/ipc_provider_demo' TID=16 PID=16
[42867354882] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[42868118766] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: starting up
[2] 16
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[42878518287] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: port pair write_h=3 read_h=4
[42890407626] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: mounted at /run/cookbook
attr_list /run/cookbook/hel[51033156966] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU0] VFS RPC: tid=5 req_id=1 op=Lookup TIMEOUT
lo[51132650019] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
.txt
[51388241619] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[51393864621] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_list' with argv=["/bin/attr_list", "/run/cookbook/hello.txt"]
[51437220054] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_list' TID=17 PID=17
[51441057690] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25l[51542413725] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #1 op=Lookup
[51582592809] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #2 op=AttrList
provider.is_demo	Bool	1
provider.name	Utf8	17
provider.requests	U64	8
count=3
[51632871741] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #3 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[55704746196] [[32mINFO [0m] [netd] [CPU3] NETD: Starting network service...
[55768751577] [[32mINFO [0m] [netd] [CPU3] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[55999613901] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/netd' TID=18 PID=18
[56010016821] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=18)
attr_set /[56708767368] [[32mINFO [0m] [netd] [CPU3] NETD: Network ready
run/cookbook/hello.txt user.comment utf8 This_is_a_test
[59684721987] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_set'
[59690705976] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_set' with argv=["/bin/attr_set", "/run/cookbook/hello.txt", "user.comment", "utf8", "This_is_a_test"]
[59751684960] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_set' TID=20 PID=20
[59755969317] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25l[59866067217] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #4 op=Lookup
[59911879005] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #5 op=AttrSet
[59914326747] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: set attr 'user.comment' type=Utf8 len=14
attr_set: wrote 0
[59941219767] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #6 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_get /run/cookbook/hello.txt user.comment
[65998656174] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_get'
[66006088104] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_get' with argv=["/bin/attr_get", "/run/cookbook/hello.txt", "user.comment"]
[66055325160] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_get' TID=21 PID=21
[66058844313] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25l[66135071475] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #7 op=Lookup
[66173484960] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_prtype=Utf8 len=14
ovider_demo: request #8 op=AttrGet
value=This_is_a_test
[66210942963] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #9 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hG
```
</details>
