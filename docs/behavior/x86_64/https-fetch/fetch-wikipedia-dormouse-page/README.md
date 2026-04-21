# ❌ Scenario: Fetch Wikipedia Dormouse page

> Last run: 2026-04-20 20:43:59

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 14408ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 2105ms | - [📜](./02/serial.log) - |
| 3 | And I type "ping -c 1 en.wikipedia.org" on the serial console | ✅ | 2398ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "1 packets transmitted, 1 received" | ✅ | 714ms | - [📜](./04/serial.log) - |
| 5 | And I type "cat /https/en.wikipedia.org/wiki/Dormouse" on the serial console | ✅ | 3171ms | - [📜](./05/serial.log) - |
| 6 | Then the serial output should contain "Gliridae" | ❌ | 121085ms | - [📜](./06/serial.log) - |

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
[42446780145] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
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
[45846305637] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[45854056380] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[45952162674] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[46816092609] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[46954811112] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[49666618485] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[49759721649] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[52711457964] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: sending frame len=304 (IPv4 0.0.0.0 -> 255.255.255.255, proto=17)
[52850666121] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: TX complete after 1 iterations
[52880767896] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[53095655316] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: sending frame len=316 (IPv4 0.0.0.0 -> 255.255.255.255, proto=17)
[53195372637] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: TX complete after 11 iterations
[53250249954] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[53512847487] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
ping -c 1 en.wikipedia.org
[?25l[59149994376] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: sending frame len=42
[59252978829] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: TX complete after 0 iterations
[59288039844] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=74
[59552557053] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: sending frame len=76 (IPv4 10.0.2.15 -> 10.0.2.3, proto=17)
[59636038869] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: TX complete after 0 iterations
[59708585904] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=131
PING en.wikipedia.org (198.35.26.224) 56 bytes of data
[63073150239] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: sending frame len=42
[63152064624] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: TX complete after 0 iterations
[63179609823] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=74
[63381145938] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: sending frame len=98 (IPv4 10.0.2.15 -> 198.35.26.224, proto=1)
[63458443158] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: TX complete after 0 iterations
[63625684980] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=108
64 bytes from 198.35.26.224: icmp_seq=1 time=1614ms

--- en.wikipedia.org ping statistics ---
1 packets transmitted, 1 received, 0% packet loss
rtt min/avg/max = 1614/1614/1614 ms
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/en.wikipedia.org/wiki/Dormouse
[?25l[71525562009] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=0 len=32768 cached=0 start=0 eof=false
[71535828276] [[32mINFO [0m] [httpsd] [CPU3] httpsd: opening upstream stream for handle=2 https://en.wikipedia.org/wiki/Dormouse
[71578399497] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=0/120)
[71581738503] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[71602215069] [[32mINFO [0m] [http] [CPU3] http: connect host=en.wikipedia.org port=443
[71627323878] [[32mINFO [0m] [http] [CPU3] http: opening /net/tcp/new
[71750264058] [[32mINFO [0m] [http] [CPU3] http: allocated tcp socket id=2
[71762988363] [[32mINFO [0m] [http] [CPU3] http: opening ctl path /net/tcp/2/ctl
[71815148328] [[32mINFO [0m] [http] [CPU3] http: opening data path /net/tcp/2/data
[71930275164] [[32mINFO [0m] [http] [CPU3] http: issuing connect command: connect en.wikipedia.org 443
[72258663378] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: sending frame len=76 (IPv4 10.0.2.15 -> 10.0.2.3, proto=17)
[72346185417] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: TX complete after 1 iterations
[72371786619] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=131
[72670552449] [[32mINFO [0m] [http] [CPU3] http: waiting for socket readiness...
[72678499014] [[32mINFO [0m] [http] [CPU3] http: waiting for http connect socket readiness (fd=5)...
[72722016675] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: sending frame len=66 (IPv4 10.0.2.15 -> 198.35.26.224, proto=6)
[72805749357] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: TX complete after 1 iterations
[73187907078] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[73362828495] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: sending frame len=54 (IPv4 10.0.2.15 -> 198.35.26.224, proto=6)
[73466908449] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: TX complete after 0 iterations
[74888972565] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[74892320910] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=1/120)
[74898496662] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[78222967251] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[78225977610] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=2/120)
[78228646881] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[81516126879] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[81529403835] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=3/120)
[81537905460] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[84812105763] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[84815343987] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=4/120)
[84817354611] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[88115299173] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[88129462113] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=5/120)
[88136462073] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[91445786949] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[91453576830] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=6/120)
[91459379847] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[94741087815] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[94785648309] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=7/120)
[94788858153] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[98098118019] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[98104904832] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=8/120)
[98107202952] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[101404004625] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[101408067453] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=9/120)
[101421347478] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[104733267342] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[104736281100] [[32mINFO [0m] [http] [CPU3] http: https header read timed out after 10 iterations
[104765085810] [[32mINFO [0m] [http] [CPU3] http: https headers not completed, initial buffer=0
[104799006081] [[32mINFO [0m] [http] [CPU3] http: waiting for http read_chunk readiness (fd=3)...
[225387589449] [[32mINFO [0m] [httpsd] [CPU3] httpsd: upstream read failed for handle=2 https://en.wikipedia.org/wiki/Dormouse: http read_chunk: timed out waiting for readiness
cat: error reading /https/en.wikipedia.org/wiki/Dormouse
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[468851993511] [[32mINFO [0m] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=70
[473522331810] [
```
</details>
