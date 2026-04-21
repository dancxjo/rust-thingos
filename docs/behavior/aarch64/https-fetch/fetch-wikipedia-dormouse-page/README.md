# ❌ Scenario: Fetch Wikipedia Dormouse page

> Last run: 2026-04-20 20:31:23

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ❌ | 1002ms | - [📜](./01/serial.log) - |

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
[753960412] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
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
[844218203] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[844340426] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[846321581] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[849222156] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] make_user_address_space: created aspace phys=0x41008000
[853615377] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] activate_address_space: setting TTBR0 to 0x41008000
[889240571] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[891095300] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] activate_address_space: setting TTBR0 to 0x41008000
[891722646] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] enter_user: TTBR0=0x41008000 entry_pc=0x204334 user_sp=0x800000 tls_base=0x0
[894222504] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[961636862] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] make_user_address_space: created aspace phys=0x4106a000
[982837714] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] make_user_address_space: created aspace phys=0x41093000
[1011779776] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] make_user_address_space: created aspace phys=0x410bc000
[1459237931] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] make_user_address_space: created aspace phys=0x41b0d000
[1495929367] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] activate_address_space: setting TTBR0 to 0x4106a000
[1496475805] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] enter_user: TTBR0=0x4106a000 entry_pc=0x20b584 user_sp=0x800000 tls_base=0x0
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[1520885674] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] activate_address_space: setting TTBR0 to 0x410bc000
[1521227062] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] enter_user: TTBR0=0x410bc000 entry_pc=0x229ad4 user_sp=0x800000 tls_base=0x0
[1521688207] [[32mINFO [0m] [netd] [CPU0] NETD: binary v2 (with heap storage) starting...
[1531424961] [[32mINFO [0m] [netd] [CPU0] NETD: Starting network service (Phase 3 — /net/ VFS provider)
[1531788423] [[32mINFO [0m] [netd] [CPU0] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[1568729063] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] activate_address_space: setting TTBR0 to 0x41008000
[1569537882] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] activate_address_space: setting TTBR0 to 0xb6f7b000
[1570371368] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] activate_address_space: setting TTBR0 to 0x41b0d000
[1570733855] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] enter_user: TTBR0=0x41b0d000 entry_pc=0x222f74 user_sp=0x800000 tls_base=0x0
[1589509567] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] activate_address_space: setting TTBR0 to 0x41093000
[1589974646] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] enter_user: TTBR0=0x41093000 entry_pc=0x20c62c user_sp=0x800000 tls_base=0x0

```
</details>
