# ✅ Scenario: Round trip with mutable attributes

> Last run: 2026-04-23 14:53:37

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8918ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 253ms | - [📜](./02/serial.log) - |
| 3 | And I type "ls /bin" on the serial console | ✅ | 219ms | - [📜](./03/serial.log) - |
| 4 | And I wait for 2 seconds | ✅ | 2000ms | - [📜](./04/serial.log) - |
| 5 | And I type "ipc_provider_demo &" on the serial console | ✅ | 374ms | - [📜](./05/serial.log) - |
| 6 | And I wait for 2 seconds | ✅ | 2001ms | - [📜](./06/serial.log) - |
| 7 | And I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 601ms | - [📜](./07/serial.log) - |
| 8 | Then the command output should contain "provider.name" | ✅ | 0ms | - - - |
| 9 | And the command output should contain "provider.is_demo" | ✅ | 0ms | - - - |
| 10 | And the command output should contain "provider.requests" | ✅ | 0ms | - - - |
| 11 | When I type "attr_set /run/cookbook/hello.txt user.comment utf8 This_is_a_test" on the serial console | ✅ | 1119ms | - [📜](./11/serial.log) - |
| 12 | And I wait for 1 seconds | ✅ | 1000ms | - [📜](./12/serial.log) - |
| 13 | And I type "attr_get /run/cookbook/hello.txt user.comment" on the serial console | ✅ | 796ms | - [📜](./13/serial.log) - |
| 14 | Then the command output should contain "type=Utf8 len=14" | ✅ | 0ms | - - - |
| 15 | And the command output should contain "value=This_is_a_test" | ✅ | 0ms | - [📜](./15/serial.log) - |
| 16 | When I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 601ms | - [📜](./16/serial.log) - |
| 17 | Then the command output should contain "user.comment" | ✅ | 0ms | - - - |
| 18 | When I type "attr_rm /run/cookbook/hello.txt user.comment" on the serial console | ✅ | 778ms | - [📜](./18/serial.log) - |
| 19 | And I wait for 1 seconds | ✅ | 1000ms | - - - |
| 20 | And I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 603ms | - [📜](./20/serial.log) - |
| 21 | Then the latest serial output should not contain "user.comment" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[27671557815] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[27707782245] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[27744389244] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[27786166815] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[27814261860] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[27816916215] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[27861935310] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[27890047218] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[27939488478] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[27940564905] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[28038558669] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[28122014250] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[28152989304] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[28242617997] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[28366105449] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[28408519326] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[28439753859] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[28563333711] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[28612614195] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[28777846911] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[29083993488] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1112930148 elapsed_us=556465
[29084745690] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[29158223094] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[29264790192] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[29266189458] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[29354792379] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[29358741951] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[29371616010] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=20889
[1;32m
        .-.
       /   \        [1;36mTHING-OS[1;32m
      |     |       [0;36m"People, places, things."[1;32m
       \   /        
        `-'        
       /   \        v0.1  ���  
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[29619583587] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[29621026149] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[29676921780] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
ls /bin
[30441745422] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
attr_get  attr_list  attr_rm  attr_set  basename  bloom  bristle  cambium  cat  clear  cp  cwd_test  date  dirname  dmesg  echo  env  env_roundtrip  false  fetchd  file  find  grep  head  httpsd  input_echo  ip  ipc_memfd_demo  ipc_pipe_demo  ipc_provider_demo  ipc_service_demo  iso9660d  iso_reader  kill  killall  ld_so  ln  loglevel  ls  mdns  mdnsd  mesocarp  mkdir  mount  mv  netd  nslookup  ping  placed  poll_mux  ps  pwd  reboot  rm  rmdir  setshell  sh  show_args  shutdown  sleep  sort  sprout  stat  tail  tee  terminal  test_dlopen  test_dyn_loader  test_exec  test_exec_env  test_futex  test_threads  test_vm_protect  top  touch  true  uname  vfs_hello  wayland_hello  wc  which  xargs  yes  
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[32440424478] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[32441368377] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing primary port (0x1F0)...
[32470502559] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing secondary port (0x170)...
[32497629615] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[32498477781] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Entering RPC service loop
[32499255591] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: No active devices to service
[34789785657] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
ipc_p[37647053928] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
ro[37745555661] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
vid[37894595376] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=14)
er_demo &
[38428894020] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ipc_provider_demo'
[2] 15
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[38471321064] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: starting up
[38478204468] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: port pair write_h=1 read_h=2
[38487110805] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: mounted at /run/cookbook
[38751371010] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
attr_list /run/cookbook/hello.txt
[46998848985] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[?25l[47066153805] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #1 op=Lookup
[47110471683] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #2 op=AttrList
provider.is_demo	Bool	1
provider.name	Utf8	17
provider.requests	U64	8
count=3
[47158369632] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #3 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_set /run/cookbook/hello.txt user.comment utf8 This_is_a_test
[50695647420] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_set'
[?25l[50770396182] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #4 op=Lookup
[50787231198] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #5 op=AttrSet
[50788568094] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: set attr 'user.comment' type=Utf8 len=14
attr_set: wrote 0
[50811062412] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #6 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_get /run/cookbook/hello.txt u[55993409175] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU0] VFS RPC: tid=5 req_id=1 op=Lookup TIMEOUT
[55999286970] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: netd activation probe failed for PID 14 (attempt 1/3): ETIMEDOUT
ser.comment
[56613313152] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_get'
[?25l[56703147237] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #7 op=Lookup
[56722617633] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #8 op=AttrGet
type=Utf8 len=14
value=This_is_a_test
[56744858181] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #9 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_list /run/cookbook/hello.txt
[58602929352] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[?25l[58663661628] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #10 op=Lookup
[58675242252] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #11 op=AttrList
provider.is_demo	Bool	1
provider.name	Utf8	17
provider.requests	U64	8
user.comment	Utf8	14
count=4
[58694345655] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #12 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_rm /run/cookbook/hello.txt user.comment
[61168360605] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_rm'
[?25l[61234155576] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #13 op=Lookup
[61250935416] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #14 op=AttrRemove
[61253284356] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: removed attr 'user.comment'
[61267828116] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #15 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_list /run/cookbook/hello.txt
[66448945728] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[66519677532] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #16 op=Lookup
[66537645306] [[32mINFO [0m] [ipc_provider_demo] [CPU3] ipc_provider_demo: request #17 op=AttrList
provider.is_demo	B
```
</details>
