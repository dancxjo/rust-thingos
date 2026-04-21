# ❌ Scenario: Fetch Wikipedia Dormouse page

> Last run: 2026-04-20 20:20:36

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 11663ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 2204ms | - [📜](./02/serial.log) - |
| 3 | And I type "ping -c 1 en.wikipedia.org" on the serial console | ✅ | 2394ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "1 packets transmitted, 1 received" | ✅ | 406ms | - [📜](./04/serial.log) - |
| 5 | And I type "cat /https/en.wikipedia.org/wiki/Dormouse" on the serial console | ✅ | 3166ms | - [📜](./05/serial.log) - |
| 6 | Then the serial output should contain "Gliridae" | ❌ | 121024ms | - [📜](./06/serial.log) - |

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
[34508777523] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
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
[37217202594] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[37221365148] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[37319391714] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[38114229549] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[38237158476] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[40616202660] [[32mINFO [0m] [netd] [CPU2] NETD: binary v2 (with heap storage) starting...
[40681642848] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service (Phase 3 �� /net/ VFS provider)
[40688245785] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[43288846392] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: sending frame len=304
[43535394177] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: received frame len=590
[43594422894] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: sending frame len=316
[43810160493] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: received frame len=590
[43934932008] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready — entering VFS service loop
[43937476869] [[32mINFO [0m] [netd] [CPU2] NETD: creating SocketApi...
[43950331128] [[32mINFO [0m] [netd] [CPU2] NETD: allocating sockets_storage...
[43956631917] [[32mINFO [0m] [netd] [CPU2] NETD: pushing socket storage 0...
[43960693656] [[32mINFO [0m] [netd] [CPU2] NETD: pushing socket storage 64...
[43964534394] [[32mINFO [0m] [netd] [CPU2] NETD: pushing socket storage 128...
[43967294844] [[32mINFO [0m] [netd] [CPU2] NETD: pushing socket storage 192...
[43969844127] [[32mINFO [0m] [netd] [CPU2] NETD: creating SocketSet...
[43972071726] [[32mINFO [0m] [netd] [CPU2] NETD: getting link state...
[43974288336] [[32mINFO [0m] [netd] [CPU2] NETD: scanning NIC units...
[43979415975] [[32mINFO [0m] [netd] [CPU2] NETD: scanning for registered NIC units...
[43984028418] [[32mINFO [0m] [netd] [CPU2] NETD: checking nic unit 0 at /dev/net/virtio0/rx
[44010991926] [[32mINFO [0m] [netd] [CPU2] NETD: checking nic unit 1 at /dev/net/virtio1/rx
[44019956607] [[32mINFO [0m] [netd] [CPU2] NETD: checking nic unit 2 at /dev/net/virtio2/rx
[44031216141] [[32mINFO [0m] [netd] [CPU2] NETD: checking nic unit 3 at /dev/net/virtio3/rx
[44044964403] [[32mINFO [0m] [netd] [CPU2] NETD: checking nic unit 4 at /dev/net/virtio4/rx
[44059348212] [[32mINFO [0m] [netd] [CPU2] NETD: checking nic unit 5 at /dev/net/virtio5/rx
[44068685232] [[32mINFO [0m] [netd] [CPU2] NETD: checking nic unit 6 at /dev/net/virtio6/rx
[44076541080] [[32mINFO [0m] [netd] [CPU2] NETD: checking nic unit 7 at /dev/net/virtio7/rx
[44084046336] [[32mINFO [0m] [netd] [CPU2] NETD: checking nic unit 8 at /dev/net/virtio8/rx
[44096562840] [[32mINFO [0m] [netd] [CPU2] NETD: checking nic unit 9 at /dev/net/virtio9/rx
[44111402478] [[32mINFO [0m] [netd] [CPU2] NETD: checking nic unit 10 at /dev/net/virtio10/rx
[44123613798] [[32mINFO [0m] [netd] [CPU2] NETD: checking nic unit 11 at /dev/net/virtio11/rx
[44132366322] [[32mINFO [0m] [netd] [CPU2] NETD: checking nic unit 12 at /dev/net/virtio12/rx
[44141380107] [[32mINFO [0m] [netd] [CPU2] NETD: checking nic unit 13 at /dev/net/virtio13/rx
[44153453454] [[32mINFO [0m] [netd] [CPU2] NETD: checking nic unit 14 at /dev/net/virtio14/rx
[44167987545] [[32mINFO [0m] [netd] [CPU2] NETD: checking nic unit 15 at /dev/net/virtio15/rx
[44176712349] [[32mINFO [0m] [netd] [CPU2] NETD: NIC scan complete: [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]
[44179602324] [[32mINFO [0m] [netd] [CPU2] NETD: NIC units scanned.
[44181299415] [[32mINFO [0m] [netd] [CPU2] NETD: bridging request port to fd...
[44183717655] [[32mINFO [0m] [netd] [CPU2] NETD: request fd=6
[44185255818] [[32mINFO [0m] [netd] [CPU2] NETD: setting up /dev/net watch...
[44191523574] [[32mINFO [0m] [netd] [CPU2] NETD: watch fd=Some(7)
[44193781071] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
ping -c 1 en.wikipedia.org
[?25l[50054553021] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=21
[50063785101] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: lookup path='dns/lookup'
[50076274512] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[50132702697] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=19
[50136448098] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: op_poll handle=16 revents=0x0000
[50189101776] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[50235570330] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=15
[50245842702] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=43
[50265492090] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=15
[50417817153] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: sending frame len=42
[50666935836] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: received frame len=64
[50711254902] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: sending frame len=76
[50907710370] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: received frame len=121
[51075473163] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[51131783868] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=21
[51136407267] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: lookup path='dns/lookup'
[51196699917] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[51258175914] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=19
[51261964644] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: op_poll handle=16 revents=0x0001
[51318920136] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[51394924416] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=27
[51480175593] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[51546471669] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=19
[51549000492] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: lookup path='icmp/new'
[51603111318] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[51662607810] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=19
[51666823791] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: op_poll handle=19 revents=0x0001
[51692742552] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=27
[51759687507] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[51829946223] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=21
[51834357597] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: lookup path='icmp/1/ctl'
[51901040367] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[51969036240] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=19
[51979738866] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: op_poll handle=33554689 revents=0x0001
[52105328979] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[52192988463] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=15
[52218553035] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=37
[52238712834] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=15
PING en.wikipedia.org (198.35.26.224) 56 bytes of data
[52325610183] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[52445584323] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=22
[52458135576] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: lookup path='icmp/1/data'
[52487737731] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=19
[52491666249] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: op_poll handle=33554690 revents=0x0004
[52511227857] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=15
[52639863738] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[52777832943] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=99
[52879806045] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: sending frame len=42
[53107593264] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[53276380443] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: received frame len=64
[53349205371] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: sending frame len=98
[53571313422] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=15
[53712250680] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[53770953786] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=22
[53773559928] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: lookup path='icmp/1/data'
[53839602960] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[53895409293] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=19
[53896815423] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: op_poll handle=33554690 revents=0x0004
[53935544190] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[54023380521] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: received frame len=98
[54113608230] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=27
64 bytes from 198.35.26.224: icmp_seq=1 time=907ms
[54164606331] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[54210563946] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=21
[54212917341] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: lookup path='icmp/1/ctl'
[54260925972] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[54312720396] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=19
[54315636309] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: op_poll handle=33554689 revents=0x0001
[54330151062] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=15
[54391251948] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[54437449770] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=32
[54495776115] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[54541490421] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=15

--- en.wikipedia.org ping statistics ---
1 packets transmitted, 1 received, 0% packet loss
rtt min/avg/max = 907/907/907 ms
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hc[54658591845] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
at /https/en.wikipedia.org/wiki/Dormouse
[?25l[61785152352] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=0 len=32768 cached=0 start=0 eof=false
[61790353317] [[32mINFO [0m] [httpsd] [CPU3] httpsd: opening upstream stream for handle=2 https://en.wikipedia.org/wiki/Dormouse
[61827167523] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=0/1000)
[61831132836] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[61837275027] [[32mINFO [0m] [http] [CPU3] http: connect host=en.wikipedia.org port=443
[61840152726] [[32mINFO [0m] [http] [CPU3] http: opening /net/tcp/new
[61883674083] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=18
[61888719948] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: lookup path='tcp/new'
[61904621592] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=19
[61907487741] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: op_poll handle=12 revents=0x0001
[61919653950] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=27
[61935560775] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[61938045015] [[32mINFO [0m] [http] [CPU3] http: allocated tcp socket id=2
[61940534601] [[32mINFO [0m] [http] [CPU3] http: opening ctl path /net/tcp/2/ctl
[61995825441] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=20
[61998373998] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: lookup path='tcp/2/ctl'
[62010118962] [[32mINFO [0m] [http] [CPU3] http: opening data path /net/tcp/2/data
[62057699418] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[62117837661] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=21
[62120731200] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: lookup path='tcp/2/data'
[62134321227] [[32mINFO [0m] [http] [CPU3] http: issuing connect command: connect en.wikipedia.org 443
[62186523564] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[62252856798] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=19
[62257255170] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: op_poll handle=66049 revents=0x0001
[62324822703] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[62386021731] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=15
[62458715319] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[62516344473] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=55
[62652537321] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: sending frame len=76
[62882659785] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: received frame len=121
[62996893113] [[32mINFO [0m] [netd::socket_api] [CPU2] SOCKET_API: connecting socket handle=2 endpoint=198.35.26.224:443 local_port=49155 state=Closed
[63004411833] [[32mINFO [0m] [netd::socket_api] [CPU2] SOCKET_API: connect initiated for handle=2 endpoint=198.35.26.224:443
[63058543152] [[32mINFO [0m] [netd::vfs_device] [CPU2] VFS_NIC: sending frame len=66
[63218203719] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[63286043997] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=15
[63295274229] [[32mINFO [0m] [http] [CPU3] http: waiting for socket readiness...
[63302132058] [[32mINFO [0m] [http] [CPU3] http: waiting for http connect socket readiness (fd=5)...
[63366051672] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[63427023231] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=19
[63429798069] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: op_poll handle=66050 revents=0x0000
[63501049557] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[63558543444] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: handling RPC n=19
[63561397449] [[32mINFO [0m] [netd::vfs_provider] [CPU2] NETD: op_poll handle=66050 revents=0x0000
[63609435780] [[32mINFO [0m] [netd] [CPU2] NETD: main loop iteration
[73964584089] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[73976501280] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=1/1000)
[73978982814] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[86137990587] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[86151924111] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=2/1000)
[86167617195] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[98310404610] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[98329942590] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=3/1000)
[98342505492] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[110492574225] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[110500721892] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=4/1000)
[110513211435] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[122668518885] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[122671413414] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=5/1000)
[122673709851] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[134834377326] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[134838110187] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=6/1000)
[134841463020] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[146936605761] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[146939521146] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=7/1000)
[146941816362] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[159081100200] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[159091153287] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=8/1000)
[159098336364] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[171254509668] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[171274040817] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=9/1000)
[171294189957] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[183470197317] [[32mINFO [0m] [http] [CPU3] http: header read slice wait: http header read: timed out waiting for readiness
[183474097818] [[32mINFO [0m] [http] [CPU3] http: https header read timed out after 10 iterations
[183489181293] [[32mINFO [0m] [http] [CPU3] http: https headers not completed, initial buffer=0
[183494032755] [[32mINFO [0m] [http] [CPU3] http: waiting for http read_chunk readiness (fd=3)...
[303662922651] [[32mINFO [0m] [httpsd] [CPU3] httpsd: upstream read failed for handle=2 https://en.wikipedia.org/wiki/Dormouse: http read_chunk: timed out waiting for readiness
cat: error readin
```
</details>
