# ❌ Scenario: Round trip with mutable attributes

> Last run: 2026-04-23 21:03:05

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8313ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 252ms | - [📜](./02/serial.log) - |
| 3 | And I type "ls /bin" on the serial console | ✅ | 230ms | - [📜](./03/serial.log) - |
| 4 | And I wait for 2 seconds | ✅ | 2001ms | - [📜](./04/serial.log) - |
| 5 | And I type "ipc_provider_demo &" on the serial console | ✅ | 375ms | - [📜](./05/serial.log) - |
| 6 | And I wait for 2 seconds | ✅ | 2001ms | - [📜](./06/serial.log) - |
| 7 | And I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 604ms | - [📜](./07/serial.log) - |
| 8 | Then the command output should contain "provider.name" | ✅ | 0ms | - - - |
| 9 | And the command output should contain "provider.is_demo" | ✅ | 0ms | - - - |
| 10 | And the command output should contain "provider.requests" | ✅ | 0ms | - - - |
| 11 | When I type "attr_set /run/cookbook/hello.txt user.comment utf8 This_is_a_test" on the serial console | ✅ | 1119ms | - [📜](./11/serial.log) - |
| 12 | And I wait for 1 seconds | ✅ | 1001ms | - [📜](./12/serial.log) - |
| 13 | And I type "attr_get /run/cookbook/hello.txt user.comment" on the serial console | ✅ | 795ms | - [📜](./13/serial.log) - |
| 14 | Then the command output should contain "type=Utf8 len=14" | ❌ | 6075ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[25879265709] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[25904451738] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[25935443688] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[25967981820] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[25987254810] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[25989946785] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26024182503] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26043874989] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26092860123] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26093923152] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26179961478] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26250392553] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26271367518] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26342183604] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26441146644] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26467582746] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26491470588] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[26588629848] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[26636857962] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[26765657589] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27012655527] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=898287852 elapsed_us=449143
[27013377105] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27084718188] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27156123621] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27157100421] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27246875007] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[27251251599] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27253571796] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27256093623] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[27267162879] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=20724
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[27458888589] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[27460115430] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[27497889210] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet �� retrying
ls /bin
[28448889381] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[28454680683] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/ls' with argv=["/bin/ls", "/bin"]
[28490884818] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/ls' TID=9 PID=9
attr_get  attr_list  attr_rm  attr_set  basename  bloom  bristle  cambium  cat  clear  cp  cwd_test  date  dirname  dmesg  echo  env  env_roundtrip  false  fetchd  file  find  grep  head  httpsd  input_echo  ip  ipc_memfd_demo  ipc_pipe_demo  ipc_provider_demo  ipc_service_demo  iso9660d  iso_reader  kill  killall  ld_so  ln  loglevel  ls  mdns  mdnsd  mesocarp  mkdir  mount  mv  netd  nslookup  ping  placed  poll_mux  ps  pwd  reboot  rm  rmdir  setshell  sh  show_args  shutdown  sleep  sort  sprout  stat  tail  tee  terminal  test_dlopen  test_dyn_loader  test_exec  test_exec_env  test_futex  test_threads  test_vm_protect  top  touch  true  uname  vfs_hello  wayland_hello  wc  which  xargs  yes  
[28773657957] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[29902519119] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=10 PID=10
[29951743998] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=11 PID=11
[30001722564] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=12 PID=12
[30006602340] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[30007542543] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[30034216311] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
[30060392307] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[30061146819] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[30061864404] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
[30587996076] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[33160530876] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[33213101889] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[33373157433] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/netd' TID=14 PID=14
[33376145055] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=14)
[34352929215] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
[34612920672] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: netd activation probe succeeded for PID 14 (/net/icmp/new is responsive)
ipc_provider_demo &
[36482148483] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ipc_provider_demo'
[36484672686] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/ipc_provider_demo' with argv=["/bin/ipc_provider_demo"]
[36513967644] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/ipc_provider_demo' TID=16 PID=16
[36517316352] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: starting up
[36517938996] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[2] 16
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[36523675023] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: port pair write_h=1 read_h=2
[36530140449] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: mounted at /run/cookbook
attr_list /run/cookbook/hello.txt
[45019947060] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[45022610358] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_list' with argv=["/bin/attr_list", "/run/cookbook/hello.txt"]
[45045506187] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_list' TID=17 PID=17
[45047216412] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25l[45082920696] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #1 op=Lookup
[45107251299] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #2 op=AttrList
provider.is_demo	Bool	1
provider.name	Utf8	17
provider.requests	U64	8
count=3
[45128112381] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #3 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_set /run/cookbook/hello.txt user.comment utf8 This_is_a_test
[48713160837] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_set'
[48716051571] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/attr_set' with argv=["/bin/attr_set", "/run/cookbook/hello.txt", "user.comment", "utf8", "This_is_a_test"]
[48747074178] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/attr_set' TID=18 PID=18
[48749584257] [[32mINFO [0m] [sh] [CPU3] sh: cleaning up 0 pipes
[?25l[48793604541] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #4 op=Lookup
[48840122793] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #5 op=AttrSet
[48841426986] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: set attr 'user.comment' type=Utf8 len=14
attr_set: wrote 0
[48854681469] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #6 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_get /run/cookbook/hello.txt user.comment

```
</details>
