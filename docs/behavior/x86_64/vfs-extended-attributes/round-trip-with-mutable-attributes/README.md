# ✅ Scenario: Round trip with mutable attributes

> Last run: 2026-04-22 21:21:24

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8710ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 253ms | - [📜](./02/serial.log) - |
| 3 | And I type "ls /bin" on the serial console | ✅ | 231ms | - [📜](./03/serial.log) - |
| 4 | And I wait for 2 seconds | ✅ | 2000ms | - [📜](./04/serial.log) - |
| 5 | And I type "ipc_provider_demo &" on the serial console | ✅ | 377ms | - [📜](./05/serial.log) - |
| 6 | And I wait for 2 seconds | ✅ | 2000ms | - - - |
| 7 | And I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 602ms | - [📜](./07/serial.log) - |
| 8 | Then the serial output should contain "provider.name" | ✅ | 1ms | - [📜](./08/serial.log) - |
| 9 | And the serial output should contain "provider.is_demo" | ✅ | 0ms | - - - |
| 10 | And the serial output should contain "provider.requests" | ✅ | 0ms | - - - |
| 11 | When I type "attr_set /run/cookbook/hello.txt user.comment utf8 This_is_a_test" on the serial console | ✅ | 1176ms | - [📜](./11/serial.log) - |
| 12 | And I wait for 1 seconds | ✅ | 1001ms | - [📜](./12/serial.log) - |
| 13 | And I type "attr_get /run/cookbook/hello.txt user.comment" on the serial console | ✅ | 797ms | - [📜](./13/serial.log) - |
| 14 | Then the serial output should contain "type=Utf8 len=14" | ✅ | 0ms | - - - |
| 15 | And the serial output should contain "value=This_is_a_test" | ✅ | 0ms | - - - |
| 16 | When I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 601ms | - [📜](./16/serial.log) - |
| 17 | Then the serial output should contain "user.comment" | ✅ | 0ms | - - - |
| 18 | When I type "attr_rm /run/cookbook/hello.txt user.comment" on the serial console | ✅ | 774ms | - [📜](./18/serial.log) - |
| 19 | And I wait for 1 seconds | ✅ | 1001ms | - [📜](./19/serial.log) - |
| 20 | And I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 597ms | - [📜](./20/serial.log) - |
| 21 | Then the latest serial output should not contain "user.comment" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[27078491847] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[27107015859] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[27144761457] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[27180829005] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[27201906732] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[27204721137] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[27243466503] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[27264954420] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[27316887873] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[27318380331] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[27406864287] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[27480486891] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[27502917486] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[27577623678] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[27679209987] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[27729303030] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[27753492888] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[27871458648] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[27922407777] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[28084228491] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[28394970054] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1055344653 elapsed_us=527672
[28395851682] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[28471247904] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[28544791143] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[28545923406] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[28637006640] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[28639252785] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[28656257784] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=28116
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28874414448] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mounting /hosts early...
[28918075527] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned early /hosts mount (PID=9)
[28928198376] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet �� retrying
[29107123650] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: starting (mount_point=/hosts)
[29125530456] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: mounted at /hosts
[29134334955] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: advertising hostname thingos.local
[29136473949] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: serving cache immediately; mDNS socket will attach when netd is ready
[29259637506] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
ls /binmounted type=mdns device=none target=/hosts

[29756748582] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
attr_get  attr_list  attr_rm  attr_set  basename  bloom  bristle  cambium  cat  clear  cp  cwd_test  date  dirname  dmesg  echo  env  env_roundtrip  false  fetchd  file  find  grep  head  httpsd  input_echo  ip  ipc_memfd_demo  ipc_pipe_demo  ipc_provider_demo  ipc_service_demo  iso9660d  iso_reader  kill  killall  ld_so  ln  loglevel  ls  mdns  mdnsd  mesocarp  mkdir  mount  mv  netd  nslookup  ping  placed  poll_mux  ps  pwd  reboot  rm  rmdir  setshell  sh  show_args  shutdown  sleep  sort  sprout  stat  tail  tee  terminal  test_dlopen  test_dyn_loader  test_exec  test_exec_env  test_futex  test_threads  test_vm_protect  top  touch  true  uname  vfs_hello  wayland_hello  wc  which  xargs  yes  
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[31899052284] [[32mINFO [0m] [ata_disk] [CPU0] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[31900763928] [[32mINFO [0m] [ata_disk] [CPU0] ATA_DISK: Probing primary port (0x1F0)...
[31956919599] [[32mINFO [0m] [ata_disk] [CPU0] ATA_DISK: Probing secondary port (0x170)...
[31958627415] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[31992232437] [[32mINFO [0m] [ata_disk] [CPU0] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[31993185840] [[32mINFO [0m] [ata_disk] [CPU0] ATA_DISK: Entering RPC service loop
[31994100600] [[32mINFO [0m] [ata_disk] [CPU0] ATA_DISK: No active devices to service
[33473665962] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[33525925785] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[34383665613] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
ipc_provider_demo &
[37781211963] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ipc_provider_demo'
[2] 18
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[37845836952] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: starting up
[37853642871] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: port pair write_h=1 read_h=2
[37862189145] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: mounted at /run/cookbook
attr_list /run/cookbook/hello.txt
[46323038058] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[?25l[46395313008] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #1 op=Lookup
[46408286694] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #2 op=AttrList
provider.is_demo	Bool	1
provider.name	Utf8	17
provider.requests	U64	8
count=3
[46436306697] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: r[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_set /run/cookbook/hello.txt user.comment utf8 This_is_a_test
[50050715847] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_set'
equest #3 op=C[?25llose
[50224029141] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #4 op=Lookup
[50256220377] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #5 op=AttrSet
[50258729400] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: set attr 'user.comment' type=Utf8 len=14
attr_set: wrote 0
[50283144549] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #6 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[50952531696] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU2] VFS RPC: tid=11 op=Lookup resp_port=3 TIMEOUT (5s) - tainting provider
attr_get /run/cookbook/hello.txt user.comment
[56135125596] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_get'
[?25l[56204574294] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #7 op=Lookup
[56223667071] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #8 op=AttrGet
type=Utf8 len=14
value=This_is_a_test
[56248128519] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #9 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_list /run/cookbook/hello.txt
[58121454567] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[?25l[58176995118] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #10 op=Lookup
[58189458987] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #11 op=AttrList
provider.is_demo	Bool	1
provider.name	Utf8	17
provider.requests	U64	8
user.comment	Utf8	14
count=4
[58208070987] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #12 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_rm /run/cookbook/hello.txt user.comment
[60678890877] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_rm'
[?25l[60742839663] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #13 op=Lookup
[60758941320] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #14 op=AttrRemove
[60761475654] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: removed attr 'user.comment'
[60790453515] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #15 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_list /run/cookbook/hello.txt
[65946120339] 
```
</details>
