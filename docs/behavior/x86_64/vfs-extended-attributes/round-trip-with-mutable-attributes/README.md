# ✅ Scenario: Round trip with mutable attributes

> Last run: 2026-04-23 21:19:50

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8509ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 254ms | - [📜](./02/serial.log) - |
| 3 | And I type "ls /bin" on the serial console | ✅ | 280ms | - [📜](./03/serial.log) - |
| 4 | And I wait for 2 seconds | ✅ | 2000ms | - [📜](./04/serial.log) - |
| 5 | And I type "ipc_provider_demo &" on the serial console | ✅ | 376ms | - [📜](./05/serial.log) - |
| 6 | And I wait for 2 seconds | ✅ | 2000ms | - [📜](./06/serial.log) - |
| 7 | And I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 597ms | - [📜](./07/serial.log) - |
| 8 | Then the command output should contain "provider.name" | ✅ | 0ms | - - - |
| 9 | And the command output should contain "provider.is_demo" | ✅ | 0ms | - - - |
| 10 | And the command output should contain "provider.requests" | ✅ | 0ms | - - - |
| 11 | When I type "attr_set /run/cookbook/hello.txt user.comment utf8 This_is_a_test" on the serial console | ✅ | 1122ms | - [📜](./11/serial.log) - |
| 12 | And I wait for 1 seconds | ✅ | 1000ms | - [📜](./12/serial.log) - |
| 13 | And I type "attr_get /run/cookbook/hello.txt user.comment" on the serial console | ✅ | 849ms | - [📜](./13/serial.log) - |
| 14 | Then the command output should contain "type=Utf8 len=14" | ✅ | 0ms | - - - |
| 15 | And the command output should contain "value=This_is_a_test" | ✅ | 0ms | - [📜](./15/serial.log) - |
| 16 | When I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 603ms | - [📜](./16/serial.log) - |
| 17 | Then the command output should contain "user.comment" | ✅ | 0ms | - - - |
| 18 | When I type "attr_rm /run/cookbook/hello.txt user.comment" on the serial console | ✅ | 775ms | - [📜](./18/serial.log) - |
| 19 | And I wait for 1 seconds | ✅ | 1000ms | - [📜](./19/serial.log) - |
| 20 | And I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 655ms | - [📜](./20/serial.log) - |
| 21 | Then the latest serial output should not contain "user.comment" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26535639570] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26562736266] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26595712770] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26629697094] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26650246887] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26653102476] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26690186820] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26710968636] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26761049601] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26762233773] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26850112179] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26919937275] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26942054469] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[27016262328] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[27122373399] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[27151950210] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[27176229894] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[27278703243] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[27330764538] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[27464795688] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27729085098] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=945696345 elapsed_us=472848
[27729851127] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27808576092] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27926616663] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27928080741] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[28051410519] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[28057591188] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[28061300883] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[28091228253] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=33198
[28092606597] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28292540694] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[28294613655] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[28374766629] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
ls /bin
[29092708491] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[29101430721] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/ls' with argv=["/bin/ls", "/bin"]
[29140901493] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/ls' TID=9 PID=9
attr_get  attr_list  attr_rm  attr_set  basename  bloom  bristle  cambium  cat  clear  cp  cwd_test  date  dirname  dmesg  echo  env  env_roundtrip  false  fetchd  file  find  grep  head  httpsd  input_echo  ip  ipc_memfd_demo  ipc_pipe_demo  ipc_provider_demo  ipc_service_demo  iso9660d  iso_reader  kill  killall  ld_so  ln  loglevel  ls  mdns  mdnsd  mesocarp  mkdir  mount  mv  netd  nslookup  ping  placed  poll_mux  ps  pwd  reboot  rm  rmdir  setshell  sh  show_args  shutdown  sleep  sort  sprout  stat  tail  tee  terminal  test_dlopen  test_dyn_loader  test_exec  test_exec_env  test_futex  test_threads  test_vm_protect  top  touch  true  uname  vfs_hello  wayland_hello  wc  which  xargs  yes  
[29449276560] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[30933301212] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=10 PID=10
[30985861863] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=11 PID=11
[31314259482] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[31752294354] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=13 PID=13
[31759602171] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[31760535345] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing primary port (0x1F0)...
[31790717607] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing secondary port (0x170)...
[31826213001] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[31827083904] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Entering RPC service loop
[31827978072] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: No active devices to service
[34084181670] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[34120123686] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[34299499311] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/netd' TID=14 PID=14
[34302988335] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=14)
ipc_provider_demo &
[37253172924] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ipc_provider_demo'
[37295393256] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/ipc_provider_demo' with argv=["/bin/ipc_provider_demo"]
[37326977985] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/ipc_provider_demo' TID=15 PID=15
[37329763152] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[2] 15
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[37341740238] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: starting up
[37347962817] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: port pair write_h=1 read_h=2
[37359793812] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: mounted at /run/cookbook
attr_list /run/cookbook/hello.txt
[45805403985] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[45808305675] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_list' with argv=["/bin/attr_list", "/run/cookbook/hello.txt"]
[45835368777] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_list' TID=17 PID=17
[45840676398] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25l[45875588715] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #1 op=Lookup
[45890968827] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #2 op=AttrList
provider.is_demo	Bool	1
provider.name	Utf8	17
provider.requests	U64	8
count=3
[45911107077] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #3 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_set /run/cookbook/hello.txt user.comment utf8 This_is_a_test
[49511341704] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_set'
[49518102579] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_set' with argv=["/bin/attr_set", "/run/cookbook/hello.txt", "user.comment", "utf8", "This_is_a_test"]
[49549881348] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_set' TID=18 PID=18
[49551958401] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25l[49601868525] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #4 op=Lookup
[49625201439] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #5 op=AttrSet
[49626728679] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: set attr 'user.comment' type=Utf8 len=14
attr_set: wrote 0
[49648874055] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #6 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_get /run/cookbook/hello.t[54576091845] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU2] VFS RPC: tid=14 req_id=44 op=Poll TIMEOUT
xt user.comment
[55430194710] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_get'
[55433063367] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_get' with argv=["/bin/attr_get", "/run/cookbook/hello.txt", "user.comment"]
[55462415349] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_get' TID=19 PID=19
[55464502962] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25l[55586959131] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #7 op=Lookup
[55607944953] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #8 op=AttrGet
type=Utf8 len=14
value=This_is_a_test
[55632555231] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #9 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_list /run/cookbook/hello.txt
[57595910064] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[57598978272] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_list' with argv=["/bin/attr_list", "/run/cookbook/hello.txt"]
[57624636531] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_list' TID=20 PID=20
[57626478987] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25l[57657051144] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #10 op=Lookup
[57668324670] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #11 op=AttrList
provider.is_demo	Bool	1
provider.name	Utf8	17
provider.requests	U64	8
user.comment	Utf8	14
count=4
[57687923634] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #12 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_rm /run/cookbook/hello.txt user.comment
[60153445374] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_rm'
[60156560409] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_rm' with argv=["/bin/attr_rm", "/run/cookbook/hello.txt", "user.comment"]
[60182205138] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_rm' TID=21 PID=21
[60184364328] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25l[60233792784] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #13 op=Lookup
[60257887536] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #14 op=AttrRemove
[60260400024] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: removed attr 'user.comment'
[60283236189] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #15 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_list /run/cookbook/hello.txt
[65439805728] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[65443651680] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_list' with argv=["/bin/attr_list", "/run/cookbook/hello.txt"]
[65473514007] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_list' TID=22 PID=22
[65475522321] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25l[65613687018] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #16 op=Lookup
[65637804342] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
[65652166833] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #17 op=AttrList
provider.is_demo	Bool	1
provider.name	Utf8	17
provider.requests	U64	8
count=3
[65677211853] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #18 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[658212
```
</details>
