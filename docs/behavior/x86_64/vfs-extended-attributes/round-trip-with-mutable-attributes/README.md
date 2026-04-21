# ❌ Scenario: Round trip with mutable attributes

> Last run: 2026-04-20 21:03:15

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 9527ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 2102ms | - [📜](./02/serial.log) - |
| 3 | And I type "ls /bin" on the serial console | ✅ | 1414ms | - [📜](./03/serial.log) - |
| 4 | And I wait for 2 seconds | ✅ | 2002ms | - - - |
| 5 | And I type "ipc_provider_demo &" on the serial console | ✅ | 2031ms | - [📜](./05/serial.log) - |
| 6 | And I wait for 2 seconds | ✅ | 2003ms | - - - |
| 7 | And I type "attr_list /run/cookbook/hello.txt" on the serial console | ✅ | 2756ms | - [📜](./07/serial.log) - |
| 8 | Then the serial output should contain "provider.name" | ❌ | 121093ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[kernel:mem:init] enter
[kernel:mem:init] phys_memory_map ok
[kernel:mem:init] modules ok
[kernel:mem:init] phys_to_virt_offset ok
[kernel:mem:init] memory map logging done
[kernel:mem:init] boot_frame_alloc init ok
[kernel:mem:init] frame allocator build ok
[kernel:mem:init] frame allocator log ok
[kernel:mem:init] FRAME_ALLOCATOR init ok
[kernel:mem:init] tasking init ok
[27610078320] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[kernel:global_alloc] enter
[kernel:global_alloc] set expand hook
[kernel:global_alloc] expand hook ok
[kernel:global_alloc] kernel_heap lock begin
[kernel:global_alloc] kernel_heap lock ok
[kernel:global_alloc] reserve_region begin
[kernel:global_alloc] reserve_region ok
[kernel:global_alloc] inner allocator init begin
[kernel:global_alloc] inner allocator init ok
[kernel:global_alloc] heap top store ok
[kernel:global_alloc] init done
[kernel:devfs] set_boot_fb begin
[kernel:devfs] set_boot_fb ok
[kernel:devfs] register begin
[kernel:devfs] register ok
[kernel:entropy] seed begin
[kernel:entropy] fill_entropy done
[kernel:entropy] add_sample(timer) ok
[kernel:entropy] mark_seeded(timer) ok
[kernel:entropy] seed done
[30140716188] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[30144241083] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[30222068448] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[30902348400] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[31025358012] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[32993787519] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[33045477036] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[35356339095] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: sending frame len=304 (IPv4 0.0.0.0 -> 255.255.255.255, proto=17)
[35436029739] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: TX complete after 1 iterations
[35462180985] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[35633423265] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: sending frame len=316 (IPv4 0.0.0.0 -> 255.255.255.255, proto=17)
[35701326672] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: TX complete after 11 iterations
[35718862806] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[35953167393] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
ls /bin
[?25lattr_get  attr_list  attr_rm  attr_set  basename  bloom  bristle  cambium  cat  clear  cp  cwd_test  date  dirname  dmesg  echo  env  env_roundtrip  false  fetchd  file  find  grep  head  httpsd  input_echo  ip  ipc_memfd_demo  ipc_pipe_demo  ipc_provider_demo  ipc_service_demo  iso9660d  iso_reader  kill  killall  ld_so  ln  loglevel  ls  mkdir  mount  mv  netd  nslookup  ping  placed  poll_mux  ps  pwd  reboot  rm  rmdir  setshell  sh  show_args  shutdown  sleep  sort  sprout  stat  tail  tee  terminal  test_dlopen  test_dyn_loader  test_exec  test_exec_env  test_futex  test_threads  test_vm_protect  top  touch  true  uname  vfs_hello  wayland_hello  wc  which  xargs  yes  
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hipc_provider_demo &
[2] 14
[52668381051] [[32mINFO [0m] [ipc_provider_demo] [CPU2] ipc_provider_demo: starting up
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[52682857194] [[32mINFO [0m] [ipc_provider_demo] [CPU2] ipc_provider_demo: port pair write_h=1 read_h=2
[52694049903] [[32mINFO [0m] [ipc_provider_demo] [CPU2] ipc_provider_demo: mounted at /run/cookbook
attr_list /run/cookbook/hello.txt

```
</details>
