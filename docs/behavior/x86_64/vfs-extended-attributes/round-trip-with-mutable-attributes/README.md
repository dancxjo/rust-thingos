# ✅ Scenario: Round trip with mutable attributes

> Last run: 2026-04-21 17:41:05

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8216ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 2102ms | - [📜](./02/serial.log) - |
| 3 | And I type "ls /bin" on the serial console | ✅ | 1411ms | - [📜](./03/serial.log) - |
| 4 | And I wait for 2 seconds | ✅ | 2001ms | - - - |
| 5 | And I type "ipc_provider_demo &" on the serial console | ✅ | 2025ms | - [📜](./05/serial.log) - |
| 6 | And I wait for 2 seconds | ✅ | 2000ms | - - - |
| 7 | And I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 2744ms | - [📜](./07/serial.log) - |
| 8 | Then the serial output should contain "provider.name" | ✅ | 0ms | - - - |
| 9 | And the serial output should contain "provider.is_demo" | ✅ | 0ms | - - - |
| 10 | And the serial output should contain "provider.requests" | ✅ | 0ms | - - - |
| 11 | When I type "attr_set /run/cookbook/hello.txt user.comment utf8 This_is_a_test" on the serial console | ✅ | 4383ms | - [📜](./11/serial.log) - |
| 12 | And I wait for 1 seconds | ✅ | 1001ms | - - - |
| 13 | And I type "attr_get /run/cookbook/hello.txt user.comment" on the serial console | ✅ | 3355ms | - [📜](./13/serial.log) - |
| 14 | Then the serial output should contain "type=Utf8 len=14" | ✅ | 0ms | - - - |
| 15 | And the serial output should contain "value=This_is_a_test" | ✅ | 0ms | - - - |
| 16 | When I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 2746ms | - [📜](./16/serial.log) - |
| 17 | Then the serial output should contain "user.comment" | ✅ | 0ms | - - - |
| 18 | When I type "attr_rm /run/cookbook/hello.txt user.comment" on the serial console | ✅ | 3303ms | - [📜](./18/serial.log) - |
| 19 | And I wait for 1 seconds | ✅ | 1000ms | - - - |
| 20 | And I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 2740ms | - [📜](./20/serial.log) - |
| 21 | Then the latest serial output should not contain "user.comment" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[24479376372] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26271586110] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26274450477] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26327210283] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26846858511] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[26916470658] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28394823105] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[28432510755] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[28445136159] [[32mINFO [0m] [iso9660d] [CPU3] iso9660d: no ISO9660 filesystem found yet — retrying
[30347423040] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
ls /bin
[35045671947] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[?25lattr_get  attr_list  attr_rm  attr_set  basename  bloom  bristle  cambium  cat  clear  cp  cwd_test  date  dirname  dmesg  echo  env  env_roundtrip  false  fetchd  file  find  grep  head  httpsd  input_echo  ip  ipc_memfd_demo  ipc_pipe_demo  ipc_provider_demo  ipc_service_demo  iso9660d  iso_reader  kill  killall  ld_so  ln  loglevel  ls  mdns  mdnsd  mesocarp  mkdir  mount  mv  netd  nslookup  ping  placed  poll_mux  ps  pwd  reboot  rm  rmdir  setshell  sh  show_args  shutdown  sleep  sort  sprout  stat  tail  tee  terminal  test_dlopen  test_dyn_loader  test_exec  test_exec_env  test_futex  test_threads  test_vm_protect  top  touch  true  uname  vfs_hello  wayland_hello  wc  which  xargs  yes  
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hipc_provider_demo &
[48304964400] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ipc_provider_demo'
[2] 14
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[48349505655] [[32mINFO [0m] [ipc_provider_demo] [CPU2] ipc_provider_demo: starting up
[48356377509] [[32mINFO [0m] [ipc_provider_demo] [CPU2] ipc_provider_demo: port pair write_h=1 read_h=2
[48362570685] [[32mINFO [0m] [ipc_provider_demo] [CPU2] ipc_provider_demo: mounted at /run/cookbook
attr_list /run/cookbook/hello.txt
[63930553434] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[?25l[64002084663] [[32mINFO [0m] [ipc_provider_demo] [CPU2] ipc_provider_demo: request #1 op=Lookup
[64013707593] [[32mINFO [0m] [ipc_provider_demo] [CPU2] ipc_provider_demo: request #2 op=AttrList
provider.is_demo	Bool	1
provider.name	Utf8	17
provider.requests	U64	8
count=3
[64036598505] [[32mINFO [0m] [ipc_provider_demo] [CPU2] ipc_provider_demo: request #3 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_set /run/cookbook/hello.txt user.comment utf8 This_is_a_test
[78377705934] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_set'
[?25l[78446922774] [[32mINFO [0m] [ipc_provider_demo] [CPU2] ipc_provider_demo: request #4 op=Lookup
[78462124389] [[32mINFO [0m] [ipc_provider_demo] [CPU2] ipc_provider_demo: request #5 op=AttrSet
[78464022615] [[32mINFO [0m] [ipc_provider_demo] [CPU2] ipc_provider_demo: set attr 'user.comment' type=Utf8 len=14
attr_set: wrote 0
[78476579313] [[32mINFO [0m] [ipc_provider_demo] [CPU2] ipc_provider_demo: request #6 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_get /run/cookbook/hello.txt user.comment
[92726423691] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_get'
[?25l[92786320605] [[32mINFO [0m] [ipc_provider_demo] [CPU2] ipc_provider_demo: request #7 op=Lookup
[92802092988] [[32mINFO [0m] [ipc_provider_demo] [CPU2] ipc_provider_demo: request #8 op=AttrGet
type=Utf8 len=14
value=This_is_a_test
[92820817980] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #9 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_list /run/cookbook/hello.txt
[101772650364] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_list'
[101831185434] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #10 op=Lookup
[?25l[101841927429] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #11 op=AttrList
provider.is_demo	Bool	1
provider.name	Utf8	17
provider.requests	U64	8
user.comment	Utf8	14
count=4
[101862274305] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #12 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_rm /run/cookbook/hello.txt user.comment
[112657474578] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='attr_rm'
[?25l[112721833653] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #13 op=Lookup
[112742616459] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #14 op=AttrRemove
[112746306651] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: removed attr 'user.comment'
[112761667920] [[32mINFO [0m] [ipc_provider_demo] [CPU1] ipc_provider_demo: request #15 op=Close
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_list /run/cookbook/hello.txt
[124978879542] [[32
```
</details>
