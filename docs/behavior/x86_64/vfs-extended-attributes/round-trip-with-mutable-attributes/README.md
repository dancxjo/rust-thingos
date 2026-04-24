# ✅ Scenario: Round trip with mutable attributes

> Last run: 2026-04-23 17:31:42

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8316ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 151ms | - [📜](./02/serial.log) - |
| 3 | And I type "ls /bin" on the serial console | ✅ | 231ms | - [📜](./03/serial.log) - |
| 4 | And I wait for 2 seconds | ✅ | 2000ms | - [📜](./04/serial.log) - |
| 5 | And I type "ipc_provider_demo &" on the serial console | ✅ | 375ms | - [📜](./05/serial.log) - |
| 6 | And I wait for 2 seconds | ✅ | 2000ms | - [📜](./06/serial.log) - |
| 7 | And I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 601ms | - [📜](./07/serial.log) - |
| 8 | Then the command output should contain "provider.name" | ✅ | 0ms | - - - |
| 9 | And the command output should contain "provider.is_demo" | ✅ | 0ms | - - - |
| 10 | And the command output should contain "provider.requests" | ✅ | 0ms | - - - |
| 11 | When I type "attr_set /run/cookbook/hello.txt user.comment utf8 This_is_a_test" on the serial console | ✅ | 1119ms | - [📜](./11/serial.log) - |
| 12 | And I wait for 1 seconds | ✅ | 1001ms | - [📜](./12/serial.log) - |
| 13 | And I type "attr_get /run/cookbook/hello.txt user.comment" on the serial console | ✅ | 791ms | - [📜](./13/serial.log) - |
| 14 | Then the command output should contain "type=Utf8 len=14" | ✅ | 0ms | - - - |
| 15 | And the command output should contain "value=This_is_a_test" | ✅ | 0ms | - [📜](./15/serial.log) - |
| 16 | When I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 599ms | - [📜](./16/serial.log) - |
| 17 | Then the command output should contain "user.comment" | ✅ | 0ms | - - - |
| 18 | When I type "attr_rm /run/cookbook/hello.txt user.comment" on the serial console | ✅ | 780ms | - [📜](./18/serial.log) - |
| 19 | And I wait for 1 seconds | ✅ | 1000ms | - - - |
| 20 | And I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 606ms | - [📜](./20/serial.log) - |
| 21 | Then the latest serial output should not contain "user.comment" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[25816297518] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[25842319173] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[25871543346] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[25903404516] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[25923498513] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[25925974338] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[25962685716] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[25985222109] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26033748873] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26034660333] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26115199737] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26182785651] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26204604525] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26278349625] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26383081164] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26409030483] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26432764842] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[26527509327] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[26573415957] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[26704882479] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[26947686018] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=892013826 elapsed_us=446006
[26948388786] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27021358947] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27084993474] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27085859229] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27167006097] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[27172823898] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27174623685] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
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
[27184079076] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=19569
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[27448902822] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[27450099303] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[27482498175] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
ls /bin
[28133643252] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[28139150523] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/ls' with argv=[[47, 98, 105, 110, 47, 108, 115], [47, 98, 105, 110]]
[28174890876] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/ls' TID=9 PID=9
attr_get  attr_list  attr_rm  attr_set  basename  bloom  bristle  cambium  cat  clear  cp  cwd_test  date  dirname  dmesg  echo  env  env_roundtrip  false  fetchd  file  find  grep  head  httpsd  input_echo  ip  ipc_memfd_demo  ipc_pipe_demo  ipc_provider_demo  ipc_service_demo  iso9660d  iso_reader  kill  killall  ld_so  ln  loglevel  ls  mdns  mdnsd  mesocarp  mkdir  mount  mv  netd  nslookup  ping  placed  poll_mux  ps  pwd  reboot  rm  rmdir  setshell  sh  show_args  shutdown  sleep  sort  sprout  stat  tail  tee  terminal  test_dlopen  test_dyn_loader  test_exec  test_exec_env  test_futex  test_threads  test_vm_protect  top  touch  true  uname  vfs_hello  wayland_hello  wc  which  xargs  yes  
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[29931592482] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=10 PID=10
[29972696028] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=11 PID=11
[30030873873] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=12 PID=12
[30039871059] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[30059704191] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[30089348751] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
[30113825412] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[30114580716] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[30115308993] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
ipc_provider_demo &
[36127608891] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ipc_provider_demo'
[36130204968] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/ipc_provider_demo' with argv=[[47, 98, 105, 110, 47, 105, 112, 99, 95, 112, 114, 111, 118, 105, 100, 101, 114, 95, 100, 101, 109, 111]]
[36160039773] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/ipc_provider_demo' TID=14 PID=14
[2] 14
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[36228730032] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[38706089499] [[32mINFO [0m] [netd] [CPU3] NETD: Starting network service...
[38734395678] [[32mINFO [0m] [netd] [CPU3] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[36178864887] [[32mINFO [0m] [ipc_provider_demo] [CPU2] ipc_provider_demo: starting up
[36186956289] [[32mINFO [0m] [ipc_provider_demo] [CPU2] ipc_provider_demo: port pair write_h=1 read_h=2
[36197263806] [[32mINFO [0m] [ipc_provider_demo] [CPU2] ipc_provider_demo: mounted at /run/cookbook
[38915919999] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/netd' TID=15 PID=15
[38918496936] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=15)
attr_list /run/cookbook/hello.txt
[44692900461] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[44695631475] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_list' with argv=[[47, 98, 105, 110, 47, 97, 116, 116, 114, 95, 108, 105, 115, 116], [47, 114, 117, 110, 47, 99, 111, 111, 107, 98, 111, 111, 107, 47, 104, 101, 108, 108, 111, 46, 116, 120, 116]]
[44718538722] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_list' TID=16 PID=16
[44752916472] [[32mINFO [0m] [ipc_provider_demo] [CPU2] ipc_provider_demo: request #1 op=Lookup
[?25l[44776279251] [[32mINFO [0m] [ipc_provider_demo] [CPU2] ipc_provider_demo: request #2 op=AttrList
provider.is_demo	Bool	1
provider.name	Utf8	17
provider.requests	U64	8
count=3
[44792712723] [[32mINFO [0m] [ipc_provider_demo] [CPU2] ipc_provider_demo: request #3 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr[45135743070] [[32mINFO [0m] [netd] [CPU3] NETD: Network ready
_set /run/cookbook/hello.txt user.comment utf8 This_is_a_test
[48391511916] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_set'
[48394949526] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_set' with argv=[[47, 98, 105, 110, 47, 97, 116, 116, 114, 95, 115, 101, 116], [47, 114, 117, 110, 47, 99, 111, 111, 107, 98, 111, 111, 107, 47, 104, 101, 108, 108, 111, 46, 116, 120, 116], [117, 115, 101, 114, 46, 99, 111, 109,
[48396207882] [[32mINFO [0m] [user.print] [CPU3] 109, 101, 110, 116], [117, 116, 102, 56], [84, 104, 105, 115, 95, 105, 115, 95, 97, 95, 116, 101, 115, 116]]
[48427774263] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_set' TID=17 PID=17
[?25l[48469651527] [[32mINFO [0m] [ipc_provider_demo] [CPU2] ipc_provider_demo: request #4 op=Lookup
[48493779675] [[32mINFO [0m] [ipc_provider_demo] [CPU2] ipc_provider_demo: request #5 op=AttrSet
[48495308466] [[32mINFO [0m] [ipc_provider_demo] [CPU2] ipc_provider_demo: set attr 'user.comment' type=Utf8 len=14
attr_set: wrote 0
[48518306694] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #6 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_get /run/cookbook/hello.txt user.comment
[54298713117] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_get'
[54301477296] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_get' with argv=[[47, 98, 105, 110, 47, 97, 116, 116, 114, 95, 103, 101, 116], [47, 114, 117, 110, 47, 99, 111, 111, 107, 98, 111, 111, 107, 47, 104, 101, 108, 108, 111, 46, 116, 120, 116], [117, 115, 101, 114, 46, 99, 111, 109,
[54302386380] [[32mINFO [0m] [user.print] [CPU3] 109, 101, 110, 116]]
[54329281512] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_get' TID=18 PID=18
[54367502112] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #7 op=Lookup
[54384752037] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #8 op=AttrGet
type=Utf8 len=14
value=This_is_a_test
[54406912758] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #9 op=Close
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_list /run/cookbook/hello.txt
[56273131893] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[56275788888] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_list' with argv=[[47, 98, 105, 110, 47, 97, 116, 116, 114, 95, 108, 105, 115, 116], [47, 114, 117, 110, 47, 99, 111, 111, 107, 98, 111, 111, 107, 47, 104, 101, 108, 108, 111, 46, 116, 120, 116]]
[56300620893] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_list' TID=19 PID=19
[?25l[56331983268] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #10 op=Lookup
[56341301676] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #11 op=AttrList
provider.is_demo	Bool	1
provider.name	Utf8	17
provider.requests	U64	8
user.comment	Utf8	14
count=4
[56357893449] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #12 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_rm /run/cookbook/hello.txt user.comment
[58848940755] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_rm'
[58851879141] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_rm' with argv=[[47, 98, 105, 110, 47, 97, 116, 116, 114, 95, 114, 109], [47, 114, 117, 110, 47, 99, 111, 111, 107, 98, 111, 111, 107, 47, 104, 101, 108, 108, 111, 46, 116, 120, 116], [117, 115, 101, 114, 46, 99, 111, 109, 109, 1
[58853341107] [[32mINFO [0m] [user.print] [CPU3] 01, 110, 116]]
[58875595779] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_rm' TID=20 PID=20
[?25l[58912750182] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #13 op=Lookup
[58931194344] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #14 op=AttrRemove
[58933175037] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: removed attr 'user.comment'
[58939168266] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #15 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_list /run/cookbook/hello.txt
[64139052450] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[64141791813] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_list' with argv=[[47, 98, 105, 110, 47, 97, 116, 116, 114, 95, 108, 105, 115, 116], [47, 114, 117, 110, 47, 99, 111, 111, 107, 98, 111, 111, 107, 47, 104, 101, 108, 108, 111, 46, 116, 120, 116]]
[64166460468] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_list' TID=21 PID=21
[64168087236] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU0] VFS RPC: tid=5 req_id=1 op=Lookup TIMEOUT
[?25l[64171686051] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: netd activation probe failed for PID 15 (attempt 1/3): ETIMEDOUT
[64204655823] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #16 op=Lookup
[64223065170] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #17 op=AttrList
provider.is_demo	Bool	1
provider.name	Utf8	17
provider.requests	U64	8
count=3
[64242748086] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #18 op=Close
[?25h[1;95mTHIN
```
</details>
