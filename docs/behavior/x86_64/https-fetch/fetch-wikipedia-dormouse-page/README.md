# ❌ Scenario: Fetch Wikipedia Dormouse page

> Last run: 2026-04-20 22:47:42

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 15636ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 2204ms | - [📜](./02/serial.log) - |
| 3 | And I type "ping -c 1 en.wikipedia.org" on the serial console | ✅ | 2390ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "1 packets transmitted, 1 received" | ✅ | 411ms | - [📜](./04/serial.log) - |
| 5 | And I type "cat /https/en.wikipedia.org/wiki/Dormouse" on the serial console | ✅ | 3173ms | - [📜](./05/serial.log) - |
| 6 | Then the serial output should contain "Gliridae" | ❌ | 301017ms | - [📜](./06/serial.log) - |

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
[46446949296] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
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
[49848413505] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[49855817781] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[49957626873] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[50890521693] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[51105611337] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[53991008181] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[54054537966] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[55053463377] [[32mINFO [0m] [iso9660d] [CPU1] iso9660d: no ISO9660 filesystem found yet — retrying
ping -c 1 en.wikipedia.org
[?25l[63251695155] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
[63533782224] [[32mINFO [0m] [netd] [CPU2] NETD: did_work=true, pushing notifications
[63614897775] [[32mINFO [0m] [netd] [CPU2] NETD: did_work=true, pushing notifications
[63717650502] [[32mINFO [0m] [netd] [CPU2] NETD: did_work=true, pushing notifications
[64605475836] [[32mINFO [0m] [netd] [CPU2] NETD: did_work=true, pushing notifications
[64734129339] [[32mINFO [0m] [netd] [CPU2] NETD: did_work=true, pushing notifications
[64841345943] [[32mINFO [0m] [netd] [CPU2] NETD: did_work=true, pushing notifications
[64943430519] [[32mINFO [0m] [netd] [CPU2] NETD: did_work=true, pushing notifications
[65040352146] [[32mINFO [0m] [netd] [CPU2] NETD: did_work=true, pushing notifications
[65208874203] [[32mINFO [0m] [netd] [CPU1] NETD: did_work=true, pushing notifications
[65320634643] [[32mINFO [0m] [netd] [CPU1] NETD: did_work=true, pushing notifications
[65476066491] [[32mINFO [0m] [netd] [CPU1] NETD: did_work=true, pushing notifications
[65490415584] [[32mINFO [0m] [netd::socket_api] [CPU1] SOCKET_API: vfs_notify node=0x10101 revents=0x4
[65616519012] [[32mINFO [0m] [netd] [CPU1] NETD: did_work=true, pushing notifications
PING en.wikipedia.org (198.35.26.224) 56 bytes of data
[65725786104] [[32mINFO [0m] [netd] [CPU1] NETD: did_work=true, pushing notifications
[65833208232] [[32mINFO [0m] [netd] [CPU1] NETD: did_work=true, pushing notifications
[65964723396] [[32mINFO [0m] [netd] [CPU1] NETD: did_work=true, pushing notifications
[66139843143] [[32mINFO [0m] [netd] [CPU1] NETD: did_work=true, pushing notifications
[66345004968] [[32mINFO [0m] [netd] [CPU1] NETD: did_work=true, pushing notifications
[66465665112] [[32mINFO [0m] [netd] [CPU1] NETD: did_work=true, pushing notifications
VFS RPC: op=Read returned error 11
[66608626161] [[32mINFO [0m] [netd] [CPU1] NETD: did_work=true, pushing notifications
VFS RPC: op=Read returned error 11
[66871318800] [[32mINFO [0m] [netd] [CPU1] NETD: did_work=true, pushing notifications
[66888033234] [[32mINFO [0m] [netd::socket_api] [CPU1] SOCKET_API: vfs_notify node=0x10101 revents=0x5
64 bytes from 198.35.26.224: icmp_seq=1 time=655ms
[67048743663] [[32mINFO [0m] [netd] [CPU1] NETD: did_work=true, pushing notifications
[67056568425] [[32mINFO [0m] [netd::socket_api] [CPU1] SOCKET_API: vfs_notify node=0x10101 revents=0x4
[67205911872] [[32mINFO [0m] [netd] [CPU1] NETD: did_work=true, pushing notifications
[67395032001] [[32mINFO [0m] [netd] [CPU1] NETD: did_work=true, pushing notifications
VFS RPC: op=Stat returned error 2

--- en.wikipedia.org ping statistics ---
1 packets transmitted, 1 received, 0% packet loss
rtt min/avg/max = 655/655/655 ms
[67498417734] [[32mINFO [0m] [netd] [CPU1] NETD: did_work=true, pushing notifications
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/en.wikipedia.org/wiki/Dormouse
[?25l[74838501573] [[32mINFO [0m] [httpsd] [CPU1] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=0 len=32768 cached=0 start=0 eof=false
[74844435171] [[32mINFO [0m] [httpsd] [CPU1] httpsd: opening upstream stream for handle=2 https://en.wikipedia.org/wiki/Dormouse
[74879722236] [[32mINFO [0m] [http] [CPU1] http: waiting for header data from port (attempt=0/120)
[74891073411] [[32mINFO [0m] [http] [CPU1] http: waiting for http header read readiness (fd=3)...
[74915333394] [[32mINFO [0m] [http] [CPU1] http: connect host=en.wikipedia.org port=443
[74918130375] [[32mINFO [0m] [http] [CPU1] http: opening /net/tcp/new
[75167802270] [[32mINFO [0m] [http] [CPU1] http: allocated tcp socket id=2
[75170282814] [[32mINFO [0m] [http] [CPU1] http: opening ctl path /net/tcp/2/ctl
[75227406342] [[32mINFO [0m] [netd] [CPU3] NETD: did_work=true, pushing notifications
[75249189807] [[32mINFO [0m] [netd::socket_api] [CPU3] SOCKET_API: vfs_notify node=0x10201 revents=0x11
[75333897144] [[32mINFO [0m] [http] [CPU1] http: opening data path /net/tcp/2/data
[75384760011] [[32mINFO [0m] [netd] [CPU3] NETD: did_work=true, pushing notifications
[75449682165] [[32mINFO [0m] [http] [CPU1] http: issuing connect command: connect en.wikipedia.org 443
[75479803443] [[32mINFO [0m] [netd] [CPU3] NETD: did_work=true, pushing notifications
[75599752470] [[32mINFO [0m] [netd] [CPU3] NETD: did_work=true, pushing notifications
[75985986780] [[32mINFO [0m] [http] [CPU1] http: waiting for socket readiness...
[75992944830] [[32mINFO [0m] [http] [CPU1] http: waiting for http connect socket readiness (fd=5)...
[76164156453] [[32mINFO [0m] [netd] [CPU3] NETD: did_work=true, pushing notifications
[76168958613] [[32mINFO [0m] [netd::socket_api] [CPU3] SOCKET_API: vfs_notify node=0x10201 revents=0x0
[76261868694] [[32mINFO [0m] [netd] [CPU3] NETD: did_work=true, pushing notifications
[77931478911] [[32mINFO [0m] [http] [CPU1] http: wait_fd_ready complete for http connect socket revents=0x0004
[77953710978] [[32mINFO [0m] [http] [CPU1] http: background task: TCP connected to en.wikipedia.org
[77960474031] [[32mINFO [0m] [http] [CPU1] http: background task: starting TLS handshake with en.wikipedia.org
[77967787458] [[32mINFO [0m] [netd] [CPU3] NETD: did_work=true, pushing notifications
[77973204936] [[32mINFO [0m] [netd::socket_api] [CPU3] SOCKET_API: vfs_notify node=0x10201 revents=0x4
[78422306490] [[32mINFO [0m] [netd] [CPU3] NETD: did_work=true, pushing notifications
[78498875433] [[32mINFO [0m] [http] [CPU1] http: waiting for http read readiness (fd=5)...
[78533184642] [[32mINFO [0m] [netd] [CPU3] NETD: did_work=true, pushing notifications
[78682245345] [[32mINFO [0m] [netd] [CPU3] NETD: did_work=true, pushing notifications
[79449220059] [[32mINFO [0m] [netd] [CPU3] NETD: did_work=true, pushing notifications
[79453045485] [[32mINFO [0m] [netd::socket_api] [CPU3] SOCKET_API: vfs_notify node=0x10201 revents=0x5
[255198197232] [[32mINFO [0m] [http] [CPU1] http: header read slice wait: http header read: timed out waiting for readiness
[255215678553] [[32mINFO [0m] [http] [CPU1] http: https header read timed out after 1 iterations
[255220260174] [[32mINFO [0m] [http] [CPU1] http: https headers not completed, initial buffer=0
[255234246762] [[32mINFO [0m] [http] [CPU1] http: waiting for http read_chunk readiness (fd=3)...
[3758
```
</details>
