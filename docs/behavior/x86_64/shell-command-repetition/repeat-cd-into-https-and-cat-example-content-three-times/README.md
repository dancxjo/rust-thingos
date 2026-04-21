# ✅ Scenario: Repeat cd into https and cat example content three times

> Last run: 2026-04-21 05:30:04

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 10033ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 2102ms | - [📜](./02/serial.log) - |
| 3 | And I type "cd /https" on the serial console | ✅ | 1511ms | - [📜](./03/serial.log) - |
| 4 | And I type "cat www.example.com" on the serial console | ✅ | 2023ms | - [📜](./04/serial.log) - |
| 5 | And I type "cd /https" on the serial console | ✅ | 1511ms | - [📜](./05/serial.log) - |
| 6 | And I type "cat www.example.com" on the serial console | ✅ | 2026ms | - [📜](./06/serial.log) - |
| 7 | And I type "cd /https" on the serial console | ✅ | 1512ms | - [📜](./07/serial.log) - |
| 8 | And I type "cat www.example.com" on the serial console | ✅ | 2029ms | - [📜](./08/serial.log) - |

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
[29318381499] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
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
[31896074859] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[31899372153] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[31961962923] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[32716352658] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[32851702488] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[35145666237] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[35211128733] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[35465462109] [[32mINFO [0m] [iso9660d] [CPU1] iso9660d: no ISO9660 filesystem found yet — retrying
cd /https
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/https[0m [1;96m>[0m [?25h[41885894160] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
cat www.example.com
[?25l[48115122165] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://www.example.com offset=0 len=32768 cached=0 start=0 eof=false
[48119807406] [[32mINFO [0m] [httpsd] [CPU3] httpsd: opening upstream stream for handle=2 https://www.example.com
[48150517899] [[32mINFO [0m] [http] [CPU2] http: connect host=www.example.com port=443
[48152855289] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=0/120)
[48155444073] [[32mINFO [0m] [http] [CPU2] http: opening /net/tcp/new
[48157653951] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[48402785376] [[32mINFO [0m] [http] [CPU2] http: allocated tcp socket id=1
[48407150451] [[32mINFO [0m] [http] [CPU2] http: opening ctl path /net/tcp/1/ctl
[48499989186] [[32mINFO [0m] [http] [CPU2] http: opening data path /net/tcp/1/data
[48562863987] [[32mINFO [0m] [http] [CPU2] http: issuing connect command: connect www.example.com 443
[49493365746] [[32mINFO [0m] [http] [CPU2] http: waiting for socket readiness...
[49498996173] [[32mINFO [0m] [http] [CPU2] http: waiting for http connect socket readiness (fd=5)...
cd /https
cat www.example.com
cd /https
cat www.example.com

```
</details>
