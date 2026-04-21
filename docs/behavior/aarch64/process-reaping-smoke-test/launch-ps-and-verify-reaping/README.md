# ❌ Scenario: Launch ps and verify reaping

> Last run: 2026-04-20 18:36:07

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ❌ | 1001ms | - [📜](./01/serial.log) - |

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
[854545882] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
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
[970894981] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[971307045] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[973618468] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[976811013] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] make_user_address_space: created aspace phys=0x41008000
[981795647] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] activate_address_space: setting TTBR0 to 0x41008000
[1002184751] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[1003070208] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] activate_address_space: setting TTBR0 to 0x41008000
[1003247098] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] enter_user: TTBR0=0x41008000 entry_pc=0x204334 user_sp=0x800000
[1004218773] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[1026702179] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] make_user_address_space: created aspace phys=0x4106a000
[1036771165] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] make_user_address_space: created aspace phys=0x41093000
[1046935137] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] make_user_address_space: created aspace phys=0x410bc000
[1331788905] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] make_user_address_space: created aspace phys=0x41b0c000
[1371481804] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] activate_address_space: setting TTBR0 to 0x4106a000
[1371794876] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] enter_user: TTBR0=0x4106a000 entry_pc=0x20b584 user_sp=0x800000
[1387547614] [[31;1mERROR[0m] [bran] [CPU0] panicked at thingos/kernel/src/sched/mod.rs:2947:13:
scheduler invariant violated: terminate_current tid mismatch (cpu=0, scheduler_current=3, terminating_tid=0)
FAULT

```
</details>
