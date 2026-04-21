# ❌ Scenario: Launch ps and verify reaping

> Last run: 2026-04-20 19:17:36

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 15864ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 8225ms | - [📜](./02/serial.log) - |
| 3 | And I type "ps" on the serial console | ✅ | 1157ms | - - - |
| 4 | Then the serial output should contain "PID" | ❌ | 121077ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
UEFI firmware (version  built at 23:58:55 on Oct  8 2025)
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;031;100t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI Misc Device" from PciRoot(0x0)/Pci(0x3,0x0)
BdsDxe: starting Boot0002 "UEFI Misc Device" from PciRoot(0x0)/Pci(0x3,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01HKMAIN
[kernel:mem:init] enter
[kernel:mem:init] phys_memory_map ok
[kernel:mem:init] modules ok
[kernel:mem:init] phys_to_virt_offset ok
[kernel:mem:init] memory map logging done
[kernel:mem:init] boot_frame_alloc init ok
[kernel:mem:init] frame allocator build ok
[kernel:mem:init] frame allocator log ok
[kernel:mem:init] FRAME_ALLOCATOR init ok
[kernel:mem:init] tasking init ok
[815915453] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
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
[945113852] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[945375675] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[947763870] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[951495319] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] make_user_address_space: created aspace phys=0x41008000
[959206218] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] activate_address_space: setting TTBR0 to 0x41008000
[981342122] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[982322714] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] activate_address_space: setting TTBR0 to 0x41008000
[982562945] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] enter_user: TTBR0=0x41008000 entry_pc=0x204334 user_sp=0x800000
[983489848] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[1008768023] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] make_user_address_space: created aspace phys=0x4106a000
[1024981089] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] make_user_address_space: created aspace phys=0x41093000
[1039034884] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] make_user_address_space: created aspace phys=0x410bc000
[1327736545] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] make_user_address_space: created aspace phys=0x41b0c000
[1355818810] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] activate_address_space: setting TTBR0 to 0x4106a000
[1356221048] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] enter_user: TTBR0=0x4106a000 entry_pc=0x20b584 user_sp=0x800000
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[1369012237] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] activate_address_space: setting TTBR0 to 0x410bc000
[1369352104] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] enter_user: TTBR0=0x410bc000 entry_pc=0x2291c8 user_sp=0x800000
[1396322165] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] activate_address_space: setting TTBR0 to 0xb6f7d000
[1396646709] [[31;1mERROR[0m] [bran] [CPU0] panicked at thingos/bran/src/arch/aarch64/trap.rs:16:9:
Unhandled Sync EL0 Exception. ESR=0x82000005 EC=0x20 ELR=0x202028 FAR=0x202028 SPSR=0x0
FAULT

```
</details>
