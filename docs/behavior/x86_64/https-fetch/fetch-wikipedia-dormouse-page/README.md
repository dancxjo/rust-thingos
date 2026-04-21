# ❌ Scenario: Fetch Wikipedia Dormouse page

> Last run: 2026-04-20 20:26:57

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 11779ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 2308ms | - [📜](./02/serial.log) - |
| 3 | And I type "ping -c 1 en.wikipedia.org" on the serial console | ✅ | 2409ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "1 packets transmitted, 1 received" | ✅ | 305ms | - [📜](./04/serial.log) - |
| 5 | And I type "cat /https/en.wikipedia.org/wiki/Dormouse" on the serial console | ✅ | 3174ms | - [📜](./05/serial.log) - |
| 6 | Then the serial output should contain "Gliridae" | ❌ | 121035ms | - [📜](./06/serial.log) - |

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
[34692643788] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
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
[37627737873] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[37632935274] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[37731591810] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[38511777744] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[38628176895] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[41460108756] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[41517353823] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[43937562801] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: TX complete after 0 iterations
[43971375327] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[44318898162] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: TX complete after 0 iterations
[44395372659] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[44931401592] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
ping -c 1 en.wikipedia.org
[?25l[51344985978] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: TX complete after 0 iterations
[51364848447] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=74
[51625211187] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: TX complete after 0 iterations
[51959355657] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=131
PING en.wikipedia.org (198.35.26.224) 56 bytes of data
[53568897525] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: TX complete after 0 iterations
[53589637431] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=74
[53851861074] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: TX complete after 1 iterations
[54345319677] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=108
64 bytes from 198.35.26.224: icmp_seq=1 time=786ms

--- en.wikipedia.org ping statistics ---
1 packets transmitted, 1 received, 0% packet loss
rtt min/avg/max = 786/786/786 ms
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/en.wikipedia.org/wiki/Dormouse
[?25l[62363756730] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=0 len=32768 cached=0 start=0 eof=false
[62372468169] [[32mINFO [0m] [httpsd] [CPU3] httpsd: opening upstream stream for handle=2 https://en.wikipedia.org/wiki/Dormouse
[62426924241] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=0/1000)
[62435008812] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[62444580099] [[32mINFO [0m] [http] [CPU3] http: connect host=en.wikipedia.org port=443
[62450497197] [[32mINFO [0m] [http] [CPU3] http: opening /net/tcp/new
[62672355339] [[32mINFO [0m] [http] [CPU3] http: allocated tcp socket id=2
[62674706325] [[32mINFO [0m] [http] [CPU3] http: opening ctl path /net/tcp/2/ctl
[62793868962] [[32mINFO [0m] [http] [CPU3] http: opening data path /net/tcp/2/data
[62909992926] [[32mINFO [0m] [http] [CPU3] http: issuing connect command: connect en.wikipedia.org 443
[63275483568] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: TX complete after 1 iterations
[63301699527] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=131
[63647577972] [[32mINFO [0m] [http] [CPU3] http: waiting for socket readiness...
[63650527644] [[32mINFO [0m] [http] [CPU3] http: waiting for http connect socket readiness (fd=5)...
[63769209570] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: TX complete after 1 iterations
[64437115908] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[65642055897] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: TX complete after 0 iterations
[74746270302] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[74751666990] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=1/1000)
[74769718683] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[87056872881] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[87059799981] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=2/1000)
[87072941274] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[99401554296] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[99404443314] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=3/1000)
[99414743472] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[111721210458] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[111724174353] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=4/1000)
[111726407496] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[124064357340] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[124068025719] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=5/1000)
[124070491215] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[136403407602] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[136406316486] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=6/1000)
[136408609326] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[148713709617] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[148722083037] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=7/1000)
[148726213251] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[161030098416] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[161033130984] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=8/1000)
[161048961282] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[173336274177] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[173339360271] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=9/1000)
[173341692051] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[185613617343] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[185617024725] [[32mINFO [0m] [http] [CPU3] http: https header read timed out after 10 iterations
[185621459958] [[32mINFO [0m] [http] [CPU3] http: https headers not completed, initial buffer=0
[185626443618] [[32mINFO [0m] [http] [CPU3] http: waiting for http read_chunk readiness (fd=3)...
[305871294498] [[32mINFO [0m] [httpsd] [CPU3] httpsd: upstream read failed for handle=2 https://en.wikipedia.org/wiki/Dormouse: http read_chunk: timed out waiting for readiness
cat: error reading /https/en.wikipedia.org/wiki/Dormouse
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[459811242693
```
</details>
