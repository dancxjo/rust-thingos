# ❌ Scenario: Fetch Wikipedia /wiki path should follow redirect

> Last run: 2026-04-21 17:41:05

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8321ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 2103ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https en.wikipedia.org /https/wp" on the serial console | ✅ | 3154ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ✅ | 0ms | - - - |
| 5 | And I type "cat /https/wp/wiki/@index" on the serial console | ✅ | 2331ms | - [📜](./05/serial.log) - |
| 6 | Then the serial output should contain "Main Page" | ❌ | 301075ms | - [📜](./06/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[24684953139] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26495214999] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26498148204] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26551719150] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[27077870490] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27147185538] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28606808076] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[28641347460] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[28660483632] [[32mINFO [0m] [iso9660d] [CPU3] iso9660d: no ISO9660 filesystem found yet ��� retrying
[30497154501] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
mount -t https en.wikipedia.org /https/wp
[41129047179] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='mount'
[?25l[41386241457] [[32mINFO [0m] [httpsd] [CPU2] httpsd: entering main RPC loop for /https/wp
[41680908258] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Lookup payload_len=4
[41682674319] [[32mINFO [0m] [httpsd] [CPU2] httpsd: dispatch_lookup path=''
[41684967654] [[32mINFO [0m] [httpsd] [CPU2] httpsd: resolve_path path=''
[41700462012] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Close payload_len=8
mounted type=https device=en.wikipedia.org target=/https/wp
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/wp/wiki/@index
[48802899783] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[?25l[48852686619] [[32mINFO [0m] [cat] [CPU1] cat: opening '/https/wp/wiki/@index'
[48863563881] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Lookup payload_len=15
[48865282356] [[32mINFO [0m] [httpsd] [CPU2] httpsd: dispatch_lookup path='wiki/@index'
[48867153687] [[32mINFO [0m] [httpsd] [CPU2] httpsd: resolve_path path='wiki/@index'
[48870982380] [[32mINFO [0m] [httpsd] [CPU2] httpsd: lookup 'wiki/@index' -> handle 2
[48878776980] [[32mINFO [0m] [cat] [CPU1] cat: opened '/https/wp/wiki/@index' fd=6
[48885328107] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Poll payload_len=12
[48894893718] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Read payload_len=20
[48897411882] [[32mINFO [0m] [httpsd] [CPU2] httpsd: ensure_upstream handle=2 MISS https://en.wikipedia.org/wiki - opening network stream
[48913187895] [[32mINFO [0m] [http] [CPU2] http: waiting for header data from port (attempt=0/120)
[48917545677] [[32mINFO [0m] [http] [CPU2] http: connect host=en.wikipedia.org port=443
[48919277880] [[32mINFO [0m] [http] [CPU2] http: opening /net/tcp/new
[49194913317] [[32mINFO [0m] [http] [CPU3] http: allocated tcp socket id=1
[49197898233] [[32mINFO [0m] [http] [CPU3] http: opening ctl path /net/tcp/1/ctl
[49259995587] [[32mINFO [0m] [http] [CPU3] http: opening data path /net/tcp/1/data
[49324531641] [[32mINFO [0m] [http] [CPU3] http: issuing connect command: connect en.wikipedia.org 443
[50437749351] [[32mINFO [0m] [http] [CPU3] http: waiting for socket readiness...
[50441378394] [[32mINFO [0m] [http] [CPU3] http: waiting for http connect socket readiness (fd=6)...
[53672499144] [[32mINFO [0m] [http] [CPU3] http: wait_fd_ready complete for http connect socket revents=0x0004
[53675241642] [[32mINFO [0m] [http] [CPU3] http: background task: TCP connected to en.wikipedia.org
[53677855011] [[32mINFO [0m] [http] [CPU3] http: background task: starting TLS handshake with en.wikipedia.org
[54163583595] [[32mINFO [0m] [http] [CPU3] http: waiting for http read readiness (fd=6)...
[54674649183] [[32mINFO [0m] [http] [CPU3] http: wait_fd_ready complete for http read revents=0x0001
[57067850895] [[32mINFO [0m] [http] [CPU3] http: background task: TLS handshake complete for en.wikipedia.org
[57070911546] [[32mINFO [0m] [http] [CPU3] http: background task: sending 180 byte request
[57073908540] [[32mINFO [0m] [http] [CPU3] http: background task: wrote 180 bytes (total=180)
[57076252068] [[32mINFO [0m] [http] [CPU3] http: background task: flushing TLS stream
[57670391097] [[32mINFO [0m] [http] [CPU3] http: background task: entering read loop
[58786828122
```
</details>
