# ❌ Scenario: Fetch Wikipedia Dormouse page

> Last run: 2026-04-20 19:24:28

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 10434ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 2205ms | - [📜](./02/serial.log) - |
| 3 | And I type "cat /https/en.wikipedia.org/wiki/Dormouse" on the serial console | ✅ | 3149ms | - [📜](./03/serial.log) - |
| 4 | Then the serial output should contain "Gliridae" | ❌ | 121090ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
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
[30217393470] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
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
[33063777549] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[33068007786] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[33137524959] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[33943604079] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[34138783602] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/en.wikipedia.org/wiki/Dormouse
[?25l[48505536288] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=0 len=32768 cached=0 start=0 eof=false
[48510643929] [[32mINFO [0m] [httpsd] [CPU3] httpsd: opening upstream stream for handle=2 https://en.wikipedia.org/wiki/Dormouse
[48547029960] [[32mINFO [0m] [http] [CPU2] http: connect host=en.wikipedia.org port=443
[48550102062] [[32mINFO [0m] [http] [CPU2] http: opening /net/tcp/new
[48690233526] [[32mINFO [0m] [http] [CPU3] http: allocated tcp socket id=1
[48693095451] [[32mINFO [0m] [http] [CPU3] http: opening ctl path /net/tcp/1/ctl
[48812567694] [[32mINFO [0m] [http] [CPU3] http: opening data path /net/tcp/1/data
[50432988903
```
</details>
