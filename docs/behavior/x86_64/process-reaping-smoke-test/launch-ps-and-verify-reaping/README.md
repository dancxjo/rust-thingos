# ✅ Scenario: Launch ps and verify reaping

> Last run: 2026-04-20 16:43:09

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 15237ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 2207ms | - [📜](./02/serial.log) - |
| 3 | And I type "ps" on the serial console | ✅ | 1155ms | - [📜](./03/serial.log) - |
| 4 | Then the serial output should contain "PID" | ✅ | 4ms | - - - |
| 5 | And the serial output should contain "sh" | ✅ | 1ms | - - - |
| 6 | And the serial output should contain "ps" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[bran:runtime] tasking init begin
[bran:runtime] tasking init ok
[bran:runtime] framebuffer begin
[bran:runtime] framebuffer ok
[kernel:mem:init] enter
[bran:runtime] phys_memory_map begin
[bran:mem] memory_map enter
[bran:mem] before MEMORY_MAP_REQUEST response
[bran:mem] got MEMORY_MAP_REQUEST response
[bran:mem] entries read
[bran:mem] truncating memory map to MAX_RANGES
[bran:mem] begin cache fill
[bran:mem] cache fill done
[bran:mem] publish done
[bran:runtime] phys_memory_map ok
[kernel:mem:init] phys_memory_map ok
[bran:runtime] modules begin
[bran:req] get_modules enter
[bran:req] before MODULE_REQUEST response
[bran:req] got MODULE_REQUEST response
[bran:req] modules list acquired
[bran:req] begin module cache fill
[bran:req] module cache fill done
[bran:req] get_modules publish done
[bran:runtime] modules ok
[kernel:mem:init] modules ok
[kernel:mem:init] phys_to_virt_offset ok
[kernel:mem:init] memory map logging done
[kernel:mem:init] boot_frame_alloc init ok
[kernel:mem:init] frame allocator build ok
[kernel:mem:init] frame allocator log ok
[kernel:mem:init] FRAME_ALLOCATOR init ok
[bran:runtime] tasking init skipped (already done)
[kernel:mem:init] tasking init ok
[44774496426] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
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
[bran:runtime] framebuffer begin
[bran:runtime] framebuffer ok
[kernel:devfs] set_boot_fb begin
[kernel:devfs] set_boot_fb ok
[kernel:devfs] register begin
[kernel:devfs] register ok
[kernel:entropy] seed begin
[kernel:entropy] fill_entropy done
[kernel:entropy] add_sample(timer) ok
[kernel:entropy] mark_seeded(timer) ok
[kernel:entropy] seed done
[48409319475] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[48421769154] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[kernel:task:init] begin
[kernel:task:init] registry init ok
[48531348294] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[kernel:task:init] sched init ok
[kernel:task:init] done
[bran:runtime] modules begin
[bran:req] get_modules enter
[bran:req] get_modules cache hit
[bran:runtime] modules ok
[kernel:vfs:init] begin
[kernel:vfs:init] mount::init ok
[kernel:vfs:init] root dirs populated
[kernel:vfs:init] mount / ok
[kernel:vfs:init] mount /dev ok
[kernel:vfs:init] mount /proc ok
[kernel:vfs:init] mount /sys ok
[kernel:vfs:init] mount /tmp ok
[kernel:vfs:init] mount /run ok
[kernel:vfs:init] mount /services ok
[kernel:vfs:init] mount /session ok
[kernel:vfs:init] mount /data ok
[kernel:vfs:init] done
[bran:runtime] phys_memory_map begin
[bran:mem] memory_map enter
[bran:mem] cache hit
[bran:runtime] phys_memory_map ok
[bran:runtime] modules begin
[bran:req] get_modules enter
[bran:req] get_modules cache hit
[bran:runtime] modules ok
[bran:runtime] framebuffer begin
[bran:runtime] framebuffer ok
[kernel:boot_info] set begin
[kernel:boot_info] set ok
[bran:runtime] modules begin
[bran:req] get_modules enter
[bran:req] get_modules cache hit
[bran:runtime] modules ok
[bran:runtime] framebuffer begin
[bran:runtime] framebuffer ok
[49505909277] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[49679634411] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[1;32m
        .-.
       /   \        [1;36mTHING-OS[1;32m
      |     |       [0;36m"People, places, things."[1;32m
       \   /        
        `-'        
       /   \        v0.1  •  ACT IV
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
[bran:runtime] modules begin
[bran:req] get_modules enter
[bran:req] get_modules cache hit
[bran:runtime] modules ok
[bran:runtime] modules begin
[bran:req] get_modules enter
[bran:req] get_modules cache hit
[bran:runtime] modules ok
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[bran:runtime] modules begin
[bran:req] get_modules enter
[bran:req] get_modules cache hit
[bran:runtime] modules ok
ps
  PID  PPID STAT COMMAND
[?25l    5     0 S    
    6     5 S    /bin/sh
    7     5 S    /bin/cambium
    8     5 S    /bin/netd
    9     5 S    /bin/httpsd
 
```
</details>
