# ✅ Scenario: Round trip with mutable attributes

> Last run: 2026-04-23 19:11:44

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8305ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 252ms | - [📜](./02/serial.log) - |
| 3 | And I type "ls /bin" on the serial console | ✅ | 281ms | - [📜](./03/serial.log) - |
| 4 | And I wait for 2 seconds | ✅ | 2000ms | - [📜](./04/serial.log) - |
| 5 | And I type "ipc_provider_demo &" on the serial console | ✅ | 373ms | - [📜](./05/serial.log) - |
| 6 | And I wait for 2 seconds | ✅ | 2001ms | - [📜](./06/serial.log) - |
| 7 | And I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 608ms | - [📜](./07/serial.log) - |
| 8 | Then the command output should contain "provider.name" | ✅ | 0ms | - - - |
| 9 | And the command output should contain "provider.is_demo" | ✅ | 0ms | - - - |
| 10 | And the command output should contain "provider.requests" | ✅ | 0ms | - - - |
| 11 | When I type "attr_set /run/cookbook/hello.txt user.comment utf8 This_is_a_test" on the serial console | ✅ | 1168ms | - [📜](./11/serial.log) - |
| 12 | And I wait for 1 seconds | ✅ | 1001ms | - [📜](./12/serial.log) - |
| 13 | And I type "attr_get /run/cookbook/hello.txt user.comment" on the serial console | ✅ | 800ms | - [📜](./13/serial.log) - |
| 14 | Then the command output should contain "type=Utf8 len=14" | ✅ | 0ms | - - - |
| 15 | And the command output should contain "value=This_is_a_test" | ✅ | 0ms | - [📜](./15/serial.log) - |
| 16 | When I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 653ms | - [📜](./16/serial.log) - |
| 17 | Then the command output should contain "user.comment" | ✅ | 0ms | - - - |
| 18 | When I type "attr_rm /run/cookbook/hello.txt user.comment" on the serial console | ✅ | 827ms | - [📜](./18/serial.log) - |
| 19 | And I wait for 1 seconds | ✅ | 1001ms | - - - |
| 20 | And I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 703ms | - [📜](./20/serial.log) - |
| 21 | Then the latest serial output should not contain "user.comment" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[25877774373] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[25905755601] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[25938013167] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[25972684122] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[25993652586] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[25996397658] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26033562192] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26055165510] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26102654754] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26103857835] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26188008858] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26260209228] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26283023250] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26356096734] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26461373235] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26488699809] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26512667082] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[26612979987] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[26665846515] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[26800033392] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27043821453] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=918497811 elapsed_us=459248
[27044547090] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27116265198] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27185433990] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27186447354] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27274401330] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[27278775117] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27280874907] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27292219383] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=20592
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[27498934848] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[27500383680] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[27538552833] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
ls /bin
[28416053853] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[28421853240] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/ls' with argv=[[47, 98, 105, 110, 47, 108, 115], [47, 98, 105, 110]]
[28459865412] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/ls' TID=9 PID=9
attr_get  attr_list  attr_rm  attr_set  basename  bloom  bristle  cambium  cat  clear  cp  cwd_test  date  dirname  dmesg  echo  env  env_roundtrip  false  fetchd  file  find  grep  head  httpsd  input_echo  ip  ipc_memfd_demo  ipc_pipe_demo  ipc_provider_demo  ipc_service_demo  iso9660d  iso_reader  kill  killall  ld_so  ln  loglevel  ls  mdns  mdnsd  mesocarp  mkdir  mount  mv  netd  nslookup  ping  placed  poll_mux  ps  pwd  reboot  rm  rmdir  setshell  sh  show_args  shutdown  sleep  sort  sprout  stat  tail  tee  terminal  test_dlopen  test_dyn_loader  test_exec  test_exec_env  test_futex  test_threads  test_vm_protect  top  touch  true  uname  vfs_hello  wayland_hello  wc  which  xargs  yes  
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[30021523686] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=10 PID=10
[30216132507] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=12 PID=12
[30983783094] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[33637286925] [[32mINFO [0m] [netd] [CPU1] NETD: Starting network service...
[33670608906] [[32mINFO [0m] [netd] [CPU1] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[33852152202] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/netd' TID=13 PID=13
[33855053265] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=13)
ipc_pro[35917914846] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=15 PID=15
[35921806470] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[35922667176] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[35979481989] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
[36005381280] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[36006191265] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[36006959736] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
vider_demo &
[36563467347] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ipc_provider_demo'
[36566329569] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/ipc_provider_demo' with argv=[[47, 98, 105, 110, 47, 105, 112, 99, 95, 112, 114, 111, 118, 105, 100, 101, 114, 95, 100, 101, 109, 111]]
[36595202061] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/ipc_provider_demo' TID=16 PID=16
[2] 16
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[36604003788] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: starting up
[36610164327] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: port pair write_h=1 read_h=2
[36616847091] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: mounted at /run/cookbook
attr_list /run/cookbook/hello.txt
[45162140958] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[45165293514] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_list' with argv=[[47, 98, 105, 110, 47, 97, 116, 116, 114, 95, 108, 105, 115, 116], [47, 114, 117, 110, 47, 99, 111, 111, 107, 98, 111, 111, 107, 47, 104, 101, 108, 108, 111, 46, 116, 120, 116]]
[45189224916] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_list' TID=17 PID=17
[?25l[45242649276] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #1 op=Lookup
[45256206600] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #2 op=AttrList
provider.is_demo	Bool	1
provider.name	Utf8	17
provider.requests	U64	8
count=3
[45281666166] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #3 op=Close
[45304991127] [[32mINFO [0m] [netd] [CPU1] NETD: Network ready
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_set /run/cookbook/hello.txt user.comment utf8 This_is_a_test
[48848421930] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_set'
[48851606826] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_set' with argv=[[47, 98, 105, 110, 47, 97, 116, 116, 114, 95, 115, 101, 116], [47, 114, 117, 110, 47, 99, 111, 111, 107, 98, 111, 111, 107, 47, 104, 101, 108, 108, 111, 46, 116, 120, 116], [117, 115, 101, 114, 46, 99, 111, 109,
[48852800799] [[32mINFO [0m] [user.print] [CPU3] 109, 101, 110, 116], [117, 116, 102, 56], [84, 104, 105, 115, 95, 105, 115, 95, 97, 95, 116, 101, 115, 116]]
[48885446973] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_set' TID=18 PID=18
[?25l[48921876564] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #4 op=Lookup
[48949225116] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #5 op=AttrSet
[48950554752] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: set attr 'user.comment' type=Utf8 len=14
attr_set: wrote 0
[48963086766] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #6 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_get /run/cookbook/hello.txt user.comment
[54945275286] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_get'
[54948631749] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_get' with argv=[[47, 98, 105, 110, 47, 97, 116, 116, 114, 95, 103, 101, 116], [47, 114, 117, 110, 47, 99, 111, 111, 107, 98, 111, 111, 107, 47, 104, 101, 108, 108, 111, 46, 116, 120, 116], [117, 115, 101, 114, 46, 99, 111, 109,
[54949592280] [[32mINFO [0m] [user.print] [CPU3] 109, 101, 110, 116]]
[54974879553] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_get' TID=19 PID=19
[?25l[55007263509] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #7 op=Lookup
[55030431621] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #8 op=AttrGet
type=Utf8 len=14
value=This_is_a_test
[55043925651] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #9 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_list /run/cookbook/hello.txt
[56932047447] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[56935948080] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_list' with argv=[[47, 98, 105, 110, 47, 97, 116, 116, 114, 95, 108, 105, 115, 116], [47, 114, 117, 110, 47, 99, 111, 111, 107, 98, 111, 111, 107, 47, 104, 101, 108, 108, 111, 46, 116, 120, 116]]
[56959347951] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_list' TID=20 PID=20
[?25l[57115248366] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #10 op=Lookup
[57181655586] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #11 op=AttrList
provider.is_demo	Bool	1
provider.name	Utf8	17
provider.requests	U64	8
user.comment	Utf8	14
count=4
[57200252076] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #12 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_rm /run/cookbook/hello.txt user.comment
[59664176385] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_rm'
[59667032733] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_rm' with argv=[[47, 98, 105, 110, 47, 97, 116, 116, 114, 95, 114, 109], [47, 114, 117, 110, 47, 99, 111, 111, 107, 98, 111, 111, 107, 47, 104, 101, 108, 108, 111, 46, 116, 120, 116], [117, 115, 101, 114, 46, 99, 111, 109, 109, 1
[59668025076] [[32mINFO [0m] [user.print] [CPU3] 01, 110, 116]]
[59697765006] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_rm' TID=21 PID=21
[59732382732] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #13 op=Lookup
[59895801108] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #14 op=AttrRemove
[59898232812] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: removed attr 'user.comment'
[?25l[59920307766] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #15 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25ha[63358838070] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU0] VFS RPC: tid=5 req_id=1 op=Lookup TIMEOUT
[63364021017] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: netd activation probe failed for PID 13 (attempt 1/3): ETIMEDOUT
ttr_list /run/cookbook/hello.txt
[65112063258] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[65114923335] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_list' with argv=[[47, 98, 105, 110, 47, 97, 116, 116, 114, 95, 108, 105, 115, 116], [47, 114, 117, 110, 47, 99, 111, 111, 107, 98, 111, 111, 107, 47, 104, 101, 108, 108, 111, 46, 116, 120, 116]]
[65140964856] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_list' TID=22 PID=22
[?25l[65182827600] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #16 op=Lookup
[65196503823] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #17 op=AttrList
provider.is_demo	Bool	1
provider.name	Utf8	17
provider.requests	U64	8
count=3
[65214181791] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #18 op=Close
[?25h[1;95mTHING[0m[1
```
</details>
