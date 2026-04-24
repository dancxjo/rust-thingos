# ✅ Scenario: Round trip with mutable attributes

> Last run: 2026-04-23 19:52:41

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8315ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 252ms | - [📜](./02/serial.log) - |
| 3 | And I type "ls /bin" on the serial console | ✅ | 231ms | - [📜](./03/serial.log) - |
| 4 | And I wait for 2 seconds | ✅ | 2000ms | - [📜](./04/serial.log) - |
| 5 | And I type "ipc_provider_demo &" on the serial console | ✅ | 374ms | - [📜](./05/serial.log) - |
| 6 | And I wait for 2 seconds | ✅ | 2001ms | - [📜](./06/serial.log) - |
| 7 | And I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 654ms | - [📜](./07/serial.log) - |
| 8 | Then the command output should contain "provider.name" | ✅ | 0ms | - - - |
| 9 | And the command output should contain "provider.is_demo" | ✅ | 0ms | - - - |
| 10 | And the command output should contain "provider.requests" | ✅ | 0ms | - - - |
| 11 | When I type "attr_set /run/cookbook/hello.txt user.comment utf8 This_is_a_test" on the serial console | ✅ | 1115ms | - [📜](./11/serial.log) - |
| 12 | And I wait for 1 seconds | ✅ | 999ms | - [📜](./12/serial.log) - |
| 13 | And I type "attr_get /run/cookbook/hello.txt user.comment" on the serial console | ✅ | 796ms | - [📜](./13/serial.log) - |
| 14 | Then the command output should contain "type=Utf8 len=14" | ✅ | 0ms | - - - |
| 15 | And the command output should contain "value=This_is_a_test" | ✅ | 0ms | - [📜](./15/serial.log) - |
| 16 | When I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 602ms | - [📜](./16/serial.log) - |
| 17 | Then the command output should contain "user.comment" | ✅ | 0ms | - - - |
| 18 | When I type "attr_rm /run/cookbook/hello.txt user.comment" on the serial console | ✅ | 831ms | - [📜](./18/serial.log) - |
| 19 | And I wait for 1 seconds | ✅ | 1001ms | - [📜](./19/serial.log) - |
| 20 | And I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 604ms | - [📜](./20/serial.log) - |
| 21 | Then the latest serial output should not contain "user.comment" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26072544432] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26104285812] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26134520049] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26166992907] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26187250023] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26189710701] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26225586519] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26246580195] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26292710697] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26293691919] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26375298114] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26444290785] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26466602613] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26547265668] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26652482406] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26679373083] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26705081271] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[26801570136] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[26850341694] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[26979863361] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27220350630] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=905515413 elapsed_us=452757
[27221051451] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27319794975] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27385724388] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27386662182] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27469256859] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[27473061561] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27474965595] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27482151114] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=20559
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[27725645277] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[27726904029] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[27763578876] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet ��� retrying
ls /bin
[28471352745] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[28476750258] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/ls' with argv=[[47, 98, 105, 110, 47, 108, 115], [47, 98, 105, 110]]
[28512523743] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/ls' TID=9 PID=9
attr_get  attr_list  attr_rm  attr_set  basename  bloom  bristle  cambium  cat  clear  cp  cwd_test  date  dirname  dmesg  echo  env  env_roundtrip  false  fetchd  file  find  grep  head  httpsd  input_echo  ip  ipc_memfd_demo  ipc_pipe_demo  ipc_provider_demo  ipc_service_demo  iso9660d  iso_reader  kill  killall  ld_so  ln  loglevel  ls  mdns  mdnsd  mesocarp  mkdir  mount  mv  netd  nslookup  ping  placed  poll_mux  ps  pwd  reboot  rm  rmdir  setshell  sh  show_args  shutdown  sleep  sort  sprout  stat  tail  tee  terminal  test_dlopen  test_dyn_loader  test_exec  test_exec_env  test_futex  test_threads  test_vm_protect  top  touch  true  uname  vfs_hello  wayland_hello  wc  which  xargs  yes  
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[30137495982] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=10 PID=10
[30351627108] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=12 PID=12
[33410271312] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
ipc_provide[35988845376] [[32mINFO [0m] [netd] [CPU1] NETD: Starting network service...
[36015972399] [[32mINFO [0m] [netd] [CPU1] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
r_de[36200548791] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/netd' TID=13 PID=13
[36204545883] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=13)
mo &
[36452433831] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ipc_provider_demo'
[36456501114] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/ipc_provider_demo' with argv=[[47, 98, 105, 110, 47, 105, 112, 99, 95, 112, 114, 111, 118, 105, 100, 101, 114, 95, 100, 101, 109, 111]]
[36485610216] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/ipc_provider_demo' TID=15 PID=15
[2] 15
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[36496463784] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: starting up
[36502720980] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: port pair write_h=1 read_h=2
[36510066813] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: mounted at /run/cookbook
[38479353858] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=16 PID=16
[38538686010] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[38541324888] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing primary port (0x1F0)...
[38570597472] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing secondary port (0x170)...
[38597647242] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[38598462012] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Entering RPC service loop
[38599224345] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: No active devices to service
attr_list /run/cookbook/hello.txt
[45029121159] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[45032776635] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_list' with argv=[[47, 98, 105, 110, 47, 97, 116, 116, 114, 95, 108, 105, 115, 116], [47, 114, 117, 110, 47, 99, 111, 111, 107, 98, 111, 111, 107, 47, 104, 101, 108, 108, 111, 46, 116, 120, 116]]
[45056446743] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_list' TID=17 PID=17
[?25l[45244518957] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #1 op=Lookup
[45256087635] [[32mINFO [0m] [netd] [CPU1] NETD: Network ready
[45271003074] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #2 op=AttrList
provider.is_demo	Bool	1
provider.name	Utf8	17
provider.requests	U64	8
count=3
[45314777739] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #3 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_set /run/cookbook/hello.txt user.comment utf8 This_is_a_test
[48877864662] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_set'
[48881278446] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_set' with argv=[[47, 98, 105, 110, 47, 97, 116, 116, 114, 95, 115, 101, 116], [47, 114, 117, 110, 47, 99, 111, 111, 107, 98, 111, 111, 107, 47, 104, 101, 108, 108, 111, 46, 116, 120, 116], [117, 115, 101, 114, 46, 99, 111, 109,
[48882585279] [[32mINFO [0m] [user.print] [CPU3] 109, 101, 110, 116], [117, 116, 102, 56], [84, 104, 105, 115, 95, 105, 115, 95, 97, 95, 116, 101, 115, 116]]
[48914993688] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_set' TID=18 PID=18
[48957308565] [[32mINFO [0m] [ipc_provider_demo] [CPU0] ipc_provider_demo: request #4 op=Lookup
[48969448044] [[32mINFO [0m] [ipc_provider_demo] [CPU0] ipc_provider_demo: request #5 op=AttrSet
[48970667328] [[32mINFO [0m] [ipc_provider_demo] [CPU0] ipc_provider_demo: set attr 'user.comment' type=Utf8 len=14
[?25lattr_set: wrote 0
[48994498047] [[32mINFO [0m] [ipc_provider_demo] [CPU0] ipc_provider_demo: request #6 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_get /run/cookbook/hello.txt user.comment
[54795386151] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_get'
[54798236460] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_get' with argv=[[47, 98, 105, 110, 47, 97, 116, 116, 114, 95, 103, 101, 116], [47, 114, 117, 110, 47, 99, 111, 111, 107, 98, 111, 111, 107, 47, 104, 101, 108, 108, 111, 46, 116, 120, 116], [117, 115, 101, 114, 46, 99, 111, 109,
[54799160790] [[32mINFO [0m] [user.print] [CPU3] 109, 101, 110, 116]]
[54822498786] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_get' TID=19 PID=19
[?25l[54856109748] [[32mINFO [0m] [ipc_provider_demo] [CPU0] ipc_provider_demo: request #7 op=Lookup
[54870040071] [[32mINFO [0m] [ipc_provider_demo] [CPU0] ipc_provider_demo: request #8 op=AttrGet
type=Utf8 len=14
value=This_is_a_test
[54884794239] [[32mINFO [0m] [ipc_provider_demo] [CPU0] ipc_provider_demo: request #9 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_list /run/cookbook/hello.txt
[56782323279] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[56785366902] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_list' with argv=[[47, 98, 105, 110, 47, 97, 116, 116, 114, 95, 108, 105, 115, 116], [47, 114, 117, 110, 47, 99, 111, 111, 107, 98, 111, 111, 107, 47, 104, 101, 108, 108, 111, 46, 116, 120, 116]]
[56811327804] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_list' TID=20 PID=20
[?25l[56856790221] [[32mINFO [0m] [ipc_provider_demo] [CPU0] ipc_provider_demo: request #10 op=Lookup
[56880502800] [[32mINFO [0m] [ipc_provider_demo] [CPU0] ipc_provider_demo: request #11 op=AttrList
provider.is_demo	Bool	1
provider.name	Utf8	17
provider.requests	U64	8
user.comment	Utf8	14
count=4
[56906842839] [[32mINFO [0m] [ipc_provider_demo] [CPU0] ipc_provider_demo: request #12 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_rm /run/cookbook/hello.txt user.comment
[59356453563] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_rm'
[59359947240] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_rm' with argv=[[47, 98, 105, 110, 47, 97, 116, 116, 114, 95, 114, 109], [47, 114, 117, 110, 47, 99, 111, 111, 107, 98, 111, 111, 107, 47, 104, 101, 108, 108, 111, 46, 116, 120, 116], [117, 115, 101, 114, 46, 99, 111, 109, 109, 1
[59360934006] [[32mINFO [0m] [user.print] [CPU3] 01, 110, 116]]
[59387555337] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_rm' TID=21 PID=21
[?25l[59434203312] [[32mINFO [0m] [ipc_provider_demo] [CPU0] ipc_provider_demo: request #13 op=Lookup
[59471491233] [[32mINFO [0m] [ipc_provider_demo] [CPU0] ipc_provider_demo: request #14 op=AttrRemove
[59474211885] [[32mINFO [0m] [ipc_provider_demo] [CPU0] ipc_provider_demo: removed attr 'user.comment'
[59495394255] [[32mINFO [0m] [ipc_provider_demo] [CPU0] ipc_provider_demo: request #15 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[62889590841] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU0] VFS RPC: tid=5 req_id=1 op=Lookup TIMEOUT
[62895033333] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: netd activation probe failed for PID 13 (attempt 1/3): ETIMEDOUT
attr_list /run/cookbook/hello.txt
[64810393032] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[64813644126] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_list' with argv=[[47, 98, 105, 110, 47, 97, 116, 116, 114, 95, 108, 105, 115, 116], [47, 114, 117, 110, 47, 99, 111, 111, 107, 98, 111, 111, 107, 47, 104, 101, 108, 108, 111, 46, 116, 120, 116]]
[64838642154] [[32mINFO 
```
</details>
