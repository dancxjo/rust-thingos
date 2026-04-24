# ✅ Scenario: Round trip with mutable attributes

> Last run: 2026-04-23 17:57:17

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8311ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 252ms | - [📜](./02/serial.log) - |
| 3 | And I type "ls /bin" on the serial console | ✅ | 231ms | - [📜](./03/serial.log) - |
| 4 | And I wait for 2 seconds | ✅ | 2000ms | - [📜](./04/serial.log) - |
| 5 | And I type "ipc_provider_demo &" on the serial console | ✅ | 371ms | - [📜](./05/serial.log) - |
| 6 | And I wait for 2 seconds | ✅ | 2000ms | - [📜](./06/serial.log) - |
| 7 | And I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 600ms | - [📜](./07/serial.log) - |
| 8 | Then the command output should contain "provider.name" | ✅ | 0ms | - - - |
| 9 | And the command output should contain "provider.is_demo" | ✅ | 0ms | - - - |
| 10 | And the command output should contain "provider.requests" | ✅ | 0ms | - - - |
| 11 | When I type "attr_set /run/cookbook/hello.txt user.comment utf8 This_is_a_test" on the serial console | ✅ | 1120ms | - [📜](./11/serial.log) - |
| 12 | And I wait for 1 seconds | ✅ | 1000ms | - [📜](./12/serial.log) - |
| 13 | And I type "attr_get /run/cookbook/hello.txt user.comment" on the serial console | ✅ | 796ms | - [📜](./13/serial.log) - |
| 14 | Then the command output should contain "type=Utf8 len=14" | ✅ | 0ms | - - - |
| 15 | And the command output should contain "value=This_is_a_test" | ✅ | 0ms | - [📜](./15/serial.log) - |
| 16 | When I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 604ms | - [📜](./16/serial.log) - |
| 17 | Then the command output should contain "user.comment" | ✅ | 0ms | - - - |
| 18 | When I type "attr_rm /run/cookbook/hello.txt user.comment" on the serial console | ✅ | 780ms | - [📜](./18/serial.log) - |
| 19 | And I wait for 1 seconds | ✅ | 1000ms | - - - |
| 20 | And I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 601ms | - [📜](./20/serial.log) - |
| 21 | Then the latest serial output should not contain "user.comment" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[25814621679] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[25841261292] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[25871582484] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[25904195889] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[25924445151] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[25926955890] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[25962467124] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[25983361338] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26029443462] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26030501541] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26115466443] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26186318697] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26209275939] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26282412552] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26389406076] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26417095221] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26442106416] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[26555358159] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[26605008738] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[26742508353] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[26988938955] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=936855480 elapsed_us=468427
[26990269185] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27061285251] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27126226512] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27127144638] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27210579033] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[27215952423] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27217932258] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27228485922] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=19503
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[27470445156] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[27471679752] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[27512615955] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
ls /bin
[28456806576] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[28462741824] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/ls' with argv=[[47, 98, 105, 110, 47, 108, 115], [47, 98, 105, 110]]
[28499739246] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/ls' TID=9 PID=9
attr_get  attr_list  attr_rm  attr_set  basename  bloom  bristle  cambium  cat  clear  cp  cwd_test  date  dirname  dmesg  echo  env  env_roundtrip  false  fetchd  file  find  grep  head  httpsd  input_echo  ip  ipc_memfd_demo  ipc_pipe_demo  ipc_provider_demo  ipc_service_demo  iso9660d  iso_reader  kill  killall  ld_so  ln  loglevel  ls  mdns  mdnsd  mesocarp  mkdir  mount  mv  netd  nslookup  ping  placed  poll_mux  ps  pwd  reboot  rm  rmdir  setshell  sh  show_args  shutdown  sleep  sort  sprout  stat  tail  tee  terminal  test_dlopen  test_dyn_loader  test_exec  test_exec_env  test_futex  test_threads  test_vm_protect  top  touch  true  uname  vfs_hello  wayland_hello  wc  which  xargs  yes  
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[30022730925] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=10 PID=10
[30063907071] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=11 PID=11
[30251685651] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=13 PID=13
[30257513286] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[30258338682] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing primary port (0x1F0)...
[30310170759] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing secondary port (0x170)...
[30335282241] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[30336068928] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Entering RPC service loop
[30336793641] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: No active devices to service
[30603364473] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[33204901620] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[33277318140] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[33412078326] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/netd' TID=14 PID=14
[33415107033] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=14)
ipc_provider_demo &
[36437271123] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ipc_provider_demo'
[36440144961] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/ipc_provider_demo' with argv=[[47, 98, 105, 110, 47, 105, 112, 99, 95, 112, 114, 111, 118, 105, 100, 101, 114, 95, 100, 101, 109, 111]]
[36469110975] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/ipc_provider_demo' TID=15 PID=15
[36475335006] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: starting up
[36482029716] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: port pair write_h=1 read_h=2
[36490121283] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: mounted at /run/cookbook
[2] 15
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[37269115653] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
attr_list /run/cookbook/hello.txt
[45000440199] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[45003463131] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_list' with argv=[[47, 98, 105, 110, 47, 97, 116, 116, 114, 95, 108, 105, 115, 116], [47, 114, 117, 110, 47, 99, 111, 111, 107, 98, 111, 111, 107, 47, 104, 101, 108, 108, 111, 46, 116, 120, 116]]
[45026676354] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_list' TID=16 PID=16
[?25l[45062738952] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #1 op=Lookup
[45072935754] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #2 op=AttrList
provider.is_demo	Bool	1
provider.name	Utf8	17
provider.requests	U64	8
count=3
[45110604462] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #3 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_set /run/cookbook/hello.txt user.comment utf8 This_is_a_test
[48695581407] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_set'
[48698710962] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_set' with argv=[[47, 98, 105, 110, 47, 97, 116, 116, 114, 95, 115, 101, 116], [47, 114, 117, 110, 47, 99, 111, 111, 107, 98, 111, 111, 107, 47, 104, 101, 108, 108, 111, 46, 116, 120, 116], [117, 115, 101, 114, 46, 99, 111, 109,
[48700066866] [[32mINFO [0m] [user.print] [CPU3] 109, 101, 110, 116], [117, 116, 102, 56], [84, 104, 105, 115, 95, 105, 115, 95, 97, 95, 116, 101, 115, 116]]
[48728420136] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_set' TID=17 PID=17
[?25l[48763531047] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #4 op=Lookup
[48780410778] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #5 op=AttrSet
[48782190006] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: set attr 'user.comment' type=Utf8 len=14
attr_set: wrote 0
[48794526198] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #6 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_get /run/cookbook/hello.txt user.comment
[54618804141] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_get'
[54621683061] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_get' with argv=[[47, 98, 105, 110, 47, 97, 116, 116, 114, 95, 103, 101, 116], [47, 114, 117, 110, 47, 99, 111, 111, 107, 98, 111, 111, 107, 47, 104, 101, 108, 108, 111, 46, 116, 120, 116], [117, 115, 101, 114, 46, 99, 111, 109,
[54622828260] [[32mINFO [0m] [user.print] [CPU3] 109, 101, 110, 116]]
[54647533809] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_get' TID=18 PID=18
[?25l[54689520072] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #7 op=Lookup
[54708023799] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #8 op=AttrGet
type=Utf8 len=14
value=This_is_a_test
[54731125779] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #9 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_list /run/cookbook/hello.txt
[56613256887] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[56616070599] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_list' with argv=[[47, 98, 105, 110, 47, 97, 116, 116, 114, 95, 108, 105, 115, 116], [47, 114, 117, 110, 47, 99, 111, 111, 107, 98, 111, 111, 107, 47, 104, 101, 108, 108, 111, 46, 116, 120, 116]]
[56639254914] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_list' TID=19 PID=19
[?25l[56669963691] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #10 op=Lookup
[56679743175] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #11 op=AttrList
provider.is_demo	Bool	1
provider.name	Utf8	17
provider.requests	U64	8
user.comment	Utf8	14
count=4
[56698025505] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #12 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_rm /run/cookbook/hello.txt[58463084295] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU0] VFS RPC: tid=5 req_id=1 op=Lookup TIMEOUT
[58468840089] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: netd activation probe failed for PID 14 (attempt 1/3): ETIMEDOUT
 user.comment
[59178548880] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_rm'
[59181626856] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_rm' with argv=[[47, 98, 105, 110, 47, 97, 116, 116, 114, 95, 114, 109], [47, 114, 117, 110, 47, 99, 111, 111, 107, 98, 111, 111, 107, 47, 104, 101, 108, 108, 111, 46, 116, 120, 116], [117, 115, 101, 114, 46, 99, 111, 109, 109, 1
[59182763046] [[32mINFO [0m] [user.print] [CPU3] 01, 110, 116]]
[59207178195] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_rm' TID=20 PID=20
[?25l[59247432948] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #13 op=Lookup
[59263796361] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #14 op=AttrRemove
[59266593606] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: removed attr 'user.comment'
[59302555785] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #15 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_list /run/cookbook/hello.txt
[64459623789] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[64462771956] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_list' with argv=[[47, 98, 105, 110, 47, 97, 116, 116, 114, 95, 108, 105, 115, 116], [47, 114, 117, 110, 47, 99, 111, 111, 107, 98, 111, 111, 107, 47, 104, 101, 108, 108, 111, 46, 116, 120, 116]]
[64490201523] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_list' TID=21 PID=21
[?25l[64537626384] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #16 op=Lookup
[64556189478] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #17 op=AttrList
provider.is_demo	Bool	1
provider.na
```
</details>
