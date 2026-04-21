# ✅ Scenario: Launch ps and verify reaping

> Last run: 2026-04-20 22:43:29

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 13387ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 2307ms | - [📜](./02/serial.log) - |
| 3 | And I type "ps" on the serial console | ✅ | 1160ms | - [📜](./03/serial.log) - |
| 4 | Then the serial output should contain "PID" | ✅ | 1ms | - - - |
| 5 | And the serial output should contain "sh" | ✅ | 1ms | - - - |
| 6 | And the serial output should contain "ps" | ✅ | 1ms | - - - |

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
[39840806247] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
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
[42983804457] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[42994557375] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[43087159830] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[43888449132] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[44112933513] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[46715161020] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[46782663345] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[46990630302] [[32mINFO [0m] [iso9660d] [CPU1] iso9660d: no ISO9660 filesystem found yet ��� retrying
ps
  PID  PPID STAT COMMAND
[?25l    5     0 S    
    6     5 S    /bin/sh
    7     5 S    /bin/cambium
    8     5 R    /bin/netd
    9     5 S    /bin/httpsd
   10     5 S    /bin/iso9660d
   12     7 R    /drivers/virtio_netd
   13     7 S    /drivers/ahci_disk
   14     6 R    /bin/ps
[?25h[1;95mTHING[0m
```
</details>
