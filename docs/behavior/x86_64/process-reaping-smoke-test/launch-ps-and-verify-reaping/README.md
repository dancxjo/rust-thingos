# ✅ Scenario: Launch ps and verify reaping

> Last run: 2026-04-20 20:26:24

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 10849ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 2207ms | - [📜](./02/serial.log) - |
| 3 | And I type "ps" on the serial console | ✅ | 1158ms | - [📜](./03/serial.log) - |
| 4 | Then the serial output should contain "PID" | ✅ | 1ms | - - - |
| 5 | And the serial output should contain "sh" | ✅ | 2ms | - - - |
| 6 | And the serial output should contain "ps" | ✅ | 0ms | - - - |

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
[31563296259] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
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
[34562789151] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[34566796242] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[34659800835] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[35438576184] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[35566932753] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[37957419555] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[38069196858] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[40146477459] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: sending frame len=304 (IPv4 0.0.0.0 -> 255.255.255.255, proto=17)
[40212610647] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: TX complete after 0 iterations
[40235892543] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[40383484647] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: received frame len=590 (IPv4 10.0.2.2 -> 255.255.255.255, proto=17)
[40449564309] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: sending frame len=316 (IPv4 0.0.0.0 -> 255.255.255.255, proto=17)
[40527371676] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: TX complete after 12 iterations
[40540748061] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[40690808037] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: received frame len=590 (IPv4 10.0.2.2 -> 255.255.255.255, proto=17)
[40806926457] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
ps
  PID  PPID STAT COMMAND
[?25l    5     0 S    
    6     5 S    /bin/sh
    7     5 S    /bin/cambium
    8     5 S    /bin/netd
    9     5 S    /bin/httpsd
   11     7 R    /drivers/virtio_netd
   12     7 S    /drivers/ahci_disk
   13  
```
</details>
