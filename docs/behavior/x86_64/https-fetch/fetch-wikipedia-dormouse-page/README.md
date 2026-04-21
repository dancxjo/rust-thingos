# ❌ Scenario: Fetch Wikipedia Dormouse page

> Last run: 2026-04-21 05:18:50

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 12581ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 2101ms | - [📜](./02/serial.log) - |
| 3 | And I type "ping -c 1 en.wikipedia.org" on the serial console | ✅ | 2386ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "1 packets transmitted, 1 received" | ✅ | 611ms | - [📜](./04/serial.log) - |
| 5 | And I type "cat /https/en.wikipedia.org/wiki/Dormouse" on the serial console | ✅ | 3163ms | - [📜](./05/serial.log) - |
| 6 | Then the serial output should contain "Dormouse" | ✅ | 1ms | - [📜](./06/serial.log) - |
| 7 | And the serial output should contain "Gliridae" | ❌ | 301014ms | - [📜](./07/serial.log) - |

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
[36800021544] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
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
[40007651673] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[40011860097] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[40107788028] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[40957907661] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[41053524732] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[43718596581] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[43796028342] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[44243668392] [[32mINFO [0m] [iso9660d] [CPU1] iso9660d: no ISO9660 filesystem found yet — retrying
ping -c 1 en.wikipedia.org
[?25l[52860657300] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
PING en.wikipedia.org (198.35.26.224) 56 bytes of data
VFS RPC: op=Read returned error 11
VFS RPC: op=Read returned error 11
64 bytes from 198.35.26.224: icmp_seq=1 time=840ms
VFS RPC: op=Stat returned error 2

--- en.wikipedia.org ping statistics ---
1 packets transmitted, 1 received, 0% packet loss
rtt min/avg/max = 840/840/840 ms
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/en.wikipedia.org/wiki/Dormouse
[?25l[65208861102] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=0 len=32768 cached=0 start=0 eof=false
[65214647388] [[32mINFO [0m] [httpsd] [CPU3] httpsd: opening upstream stream for handle=2 https://en.wikipedia.org/wiki/Dormouse
[65246652570] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=0/120)
[65252927553] [[32mINFO [0m] [http] [CPU1] http: connect host=en.wikipedia.org port=443
[65255995332] [[32mINFO [0m] [http] [CPU1] http: opening /net/tcp/new
[65257447959] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[65577745530] [[32mINFO [0m] [http] [CPU1] http: allocated tcp socket id=2
[65585817000] [[32mINFO [0m] [http] [CPU1] http: opening ctl path /net/tcp/2/ctl
[65686728921] [[32mINFO [0m] [http] [CPU1] http: opening data path /net/tcp/2/data
[65764832793] [[32mINFO [0m] [http] [CPU1] http: issuing connect command: connect en.wikipedia.org 443
[66427868331] [[32mINFO [0m] [http] [CPU1] http: waiting for socket readiness...
[66433830210] [[32mINFO [0m] [http] [CPU1] http: waiting for http connect socket readiness (fd=5)...
[245327126946] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[245333522940] [[32mINFO [0m] [http] [CPU3] http: https header read timed out after 1 iterations
[245339467230] [[32mINFO [0m] [http] [CPU3] http: https headers not completed, initial buffer=0
[245345118975] [[32mINFO [0m] [http] [CPU3] http: waiting for http read_chunk readiness (fd=3)...
[365910552018] 
```
</details>
