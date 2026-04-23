# ✅ Scenario: Round trip with mutable attributes

> Last run: 2026-04-23 11:14:17

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 11151ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 254ms | - [📜](./02/serial.log) - |
| 3 | And I type "ls /bin" on the serial console | ✅ | 338ms | - [📜](./03/serial.log) - |
| 4 | And I wait for 2 seconds | ✅ | 2000ms | - [📜](./04/serial.log) - |
| 5 | And I type "ipc_provider_demo &" on the serial console | ✅ | 381ms | - [📜](./05/serial.log) - |
| 6 | And I wait for 2 seconds | ✅ | 2000ms | - - - |
| 7 | And I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 667ms | - [📜](./07/serial.log) - |
| 8 | Then the command output should contain "provider.name" | ✅ | 0ms | - - - |
| 9 | And the command output should contain "provider.is_demo" | ✅ | 0ms | - - - |
| 10 | And the command output should contain "provider.requests" | ✅ | 0ms | - - - |
| 11 | When I type "attr_set /run/cookbook/hello.txt user.comment utf8 This_is_a_test" on the serial console | ✅ | 1184ms | - [📜](./11/serial.log) - |
| 12 | And I wait for 1 seconds | ✅ | 998ms | - [📜](./12/serial.log) - |
| 13 | And I type "attr_get /run/cookbook/hello.txt user.comment" on the serial console | ✅ | 865ms | - [📜](./13/serial.log) - |
| 14 | Then the command output should contain "type=Utf8 len=14" | ✅ | 2ms | - - - |
| 15 | And the command output should contain "value=This_is_a_test" | ✅ | 1ms | - [📜](./15/serial.log) - |
| 16 | When I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 607ms | - [📜](./16/serial.log) - |
| 17 | Then the command output should contain "user.comment" | ✅ | 0ms | - - - |
| 18 | When I type "attr_rm /run/cookbook/hello.txt user.comment" on the serial console | ✅ | 788ms | - [📜](./18/serial.log) - |
| 19 | And I wait for 1 seconds | ✅ | 1008ms | - - - |
| 20 | And I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 600ms | - [📜](./20/serial.log) - |
| 21 | Then the latest serial output should not contain "user.comment" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[34625256270] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[34654733223] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[34691783478] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[34728626295] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[34750908060] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[34753826481] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[34793947716] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[34816449591] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[34882743027] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[34884543177] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[35035444587] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[35139614565] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[35174927502] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[35270012811] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[35394791058] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[35447878158] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[35485040316] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[35661691758] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[35744291913] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[35947282140] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[36266348613] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1347575592 elapsed_us=673787
[36267644985] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[36366289575] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[36461837313] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[36463642479] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[36617539761] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[36624675252] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[36640585674] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=39072
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[36959858133] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mounting /hosts early...
[37017949188] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned early /hosts mount (PID=9)
[37032601947] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet �� retrying
[37249142634] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: starting (mount_point=/hosts)
[37281020469] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: mounted at /hosts
[37305419514] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: advertising hostname thingos.local
[37317327333] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: serving cache immediately; mDNS socket will attach when netd is ready
[37367892078] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
ls /bin
[37776922326] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
mounted type=mdns device=none target=/hosts
attr_get  attr_list  attr_rm  attr_set  basename  bloom  bristle  cambium  cat  clear  cp  cwd_test  date  dirname  dmesg  echo  env  env_roundtrip  false  fetchd  file  find  grep  head  httpsd  input_echo  ip  ipc_memfd_demo  ipc_pipe_demo  ipc_provider_demo  ipc_service_demo  iso9660d  iso_reader  kill  killall  ld_so  ln  loglevel  ls  mdns  mdnsd  mesocarp  mkdir  mount  mv  netd  nslookup  ping  placed  poll_mux  ps  pwd  reboot  rm  rmdir  setshell  sh  show_args  shutdown  sleep  sort  sprout  stat  tail  tee  terminal  test_dlopen  test_dyn_loader  test_exec  test_exec_env  test_futex  test_threads  test_vm_protect  top  touch  true  uname  vfs_hello  wayland_hello  wc  which  xargs  yes  
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[41361049059] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[43653930474] [[32mINFO [0m] [netd] [CPU3] NETD: Starting network service...
[43714049115] [[32mINFO [0m] [netd] [CPU3] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[44031445722] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[44033258610] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Probing primary port (0x1F0)...
[44136888180] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Probing secondary port (0x170)...
[44192120808] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[44193602343] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Entering RPC service loop
[44194999134] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: No active devices to service
[44719161795] [[32mINFO [0m] [netd] [CPU3] NETD: Network ready
ipc_provider_demo &
[46131001554] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ipc_provider_demo'
[2] 18
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[46206096189] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: starting up
[46215602268] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: port pair write_h=1 read_h=2
[46226065677] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: mounted at /run/cookbook
attr_list /run/cookbook/hello.txt
[54744925422] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[?25l[54855120804] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #1 op=Lookup
[54879731016] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #2 op=AttrList
provider.is_demo	Bool	1
provider.name	Utf8	17
provider.requests	U64	8
count=3
[54936553683] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #3 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_set /run/cookbook/hello.txt user.comment utf8 This_is_a_test
[58664669019] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_set'
[?25l[58784278542] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #4 op=Lookup
[58813408665] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #5 op=AttrSet
[58815744042] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: set attr 'user.comment' type=Utf8 len=14
attr_set: wrote 0
[58841756193] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #6 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[61240597176] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU2] VFS RPC: tid=11 req_id=1 op=Lookup TIMEOUT
attr_get /run/cookbook/hello.txt user.comment
[64812556710] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_get'
[?25l[64937825733] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #7 op=Lookup
[64972149066] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #8 op=AttrGet
type=Utf8 len=14
value=This_is_a_test
[65014687650] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #9 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_list /run/cookbook/hello.txt
[67004643288] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[?25l[67073529369] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #10 op=Lookup
[67097388963] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #11 op=AttrList
provider.is_demo	Bool	1
provider.name	Utf8	17
provider.requests	U64	8
user.comment	Utf8	14
count=4
[67118218992] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #12 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_rm /run/cookbook/hello.txt user.comment
[69606637386] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_rm'
[?25l[69674092389] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #13 op=Lookup
[69689639646] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #14 op=AttrRemove
[69692045742] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: removed attr 'user.comment'
[69706327911] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #15 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_list /run/cookbook/hello.txt
[74904567870] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[?25l[74994258339] [[3
```
</details>
