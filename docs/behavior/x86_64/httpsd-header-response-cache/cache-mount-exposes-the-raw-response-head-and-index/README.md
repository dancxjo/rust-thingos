# ✅ Scenario: Cache mount exposes the raw response head and index

> Last run: 2026-04-21 14:32:38

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 9232ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 2103ms | - [📜](./02/serial.log) - |
| 3 | And I type "ping -c 1 example.com" on the serial console | ✅ | 2133ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "1 packets transmitted, 1 received" | ✅ | 305ms | - [📜](./04/serial.log) - |
| 5 | And I type "cat /https/example.com > /dev/null" on the serial console | ✅ | 2799ms | - [📜](./05/serial.log) - |
| 6 | And I wait for 5 seconds | ✅ | 5001ms | - [📜](./06/serial.log) - |
| 7 | And I type "cat /run/httpsd/cache/index" on the serial console | ✅ | 2440ms | - [📜](./07/serial.log) - |
| 8 | Then the serial output should contain "example.com" | ✅ | 0ms | - - - |
| 9 | When I type "cat /run/httpsd/cache/example.com/headers" on the serial console | ✅ | 3154ms | - [📜](./09/serial.log) - |
| 10 | Then the serial output should contain "HTTP/" | ✅ | 0ms | - - - |
| 11 | And the serial output should contain "Content-Type:" | ✅ | 0ms | - - - |
| 12 | When I type "cat /run/httpsd/cache/example.com/status" on the serial console | ✅ | 3103ms | - [📜](./12/serial.log) - |
| 13 | Then the serial output should contain "200" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[27146357535] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[29298939516] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[29302351056] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[29364099237] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[29978027343] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[30085343343] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[31862121225] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[31904072376] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[32107212258] [[32mINFO [0m] [iso9660d] [CPU1] iso9660d: no ISO9660 filesystem found yet ��� retrying
ping -c 1 example.com
[?25l[41097818487] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
PING example.com (104.20.23.154) 56 bytes of data
64 bytes from 104.20.23.154: icmp_seq=1 time=527ms

--- example.com ping statistics ---
1 packets transmitted, 1 received, 0% packet loss
rtt min/avg/max = 527/527/527 ms
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/example.com > /dev/null
[51124359495] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://example.com offset=0 len=32768 cached=0 start=0 eof=false
[51130882803] [[32mINFO [0m] [httpsd] [CPU3] httpsd: opening upstream stream for handle=2 https://example.com
[51174515337] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=0/120)
[51177386337] [[32mINFO [0m] [http] [CPU1] http: connect host=example.com port=443
[51179601033] [[32mINFO [0m] [http] [CPU1] http: opening /net/tcp/new
[51458982630] [[32mINFO [0m] [http] [CPU1] http: allocated tcp socket id=2
[51463988334] [[32mINFO [0m] [http] [CPU1] http: opening ctl path /net/tcp/2/ctl
[51556318044] [[32mINFO [0m] [http] [CPU1] http: opening data path /net/tcp/2/data
[51643744185] [[32mINFO [0m] [http] [CPU1] http: issuing connect command: connect example.com 443
[52073198034] [[32mINFO [0m] [http] [CPU1] http: waiting for socket readiness...
[52077014385] [[32mINFO [0m] [http] [CPU1] http: waiting for http connect socket readiness (fd=4)...
[?25l[53459594463] [[32mINFO [0m] [http] [CPU1] http: wait_fd_ready complete for http connect socket revents=0x0004
[53463252645] [[32mINFO [0m] [http] [CPU1] http: background task: TCP connected to example.com
[53467993854] [[32mINFO [0m] [http] [CPU1] http: background task: starting TLS handshake with example.com
[53999938608] [[32mINFO [0m] [http] [CPU1] http: waiting for http read readiness (fd=4)...
[54587637126] [[32mINFO [0m] [http] [CPU1] http: wait_fd_ready complete for http read revents=0x0001
[56426254698] [[32mINFO [0m] [http] [CPU1] http: background task: TLS handshake complete for example.com
[56429620566] [[32mINFO [0m] [http] [CPU1] http: background task: sending 126 byte request
[56433299340] [[32mINFO [0m] [http] [CPU1] http: background task: wrote 126 bytes (total=126)
[56436631350] [[32mINFO [0m] [http] [CPU1] http: background task: flushing TLS stream
[56869925508] [[32mINFO [0m] [http] [CPU1] http: background task: entering read loop
[56948781681] [[32mINFO [0m] [http] [CPU1] http: waiting for http read readiness (fd=4)...
[57465386946] [[32mINFO [0m] [http] [CPU1] http: wait_fd_ready complete for http read revents=0x0001
[57965058492] [[32mINFO [0m] [http] [CPU1] http: background task: read 832 bytes from TLS
[57969846594] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 832 bytes to foreground
[57975255129] [[32mINFO [0m] [http] [CPU3] http: received 832 bytes from background TLS thread
[57979218528] [[32mINFO [0m] [http] [CPU3] http: headers complete body_start=297
[57982954029] [[32mINFO [0m] [http] [CPU3] http: response header: HTTP/1.1 200 OK
[57984939078] [[32mINFO [0m] [http] [CPU3] http: response header: Date: Tue, 21 Apr 2026 21:33:50 GMT
[57986269275] [[32mINFO [0m] [http] [CPU3] http: response header: Content-Type: text/html
[57987523836] [[32mINFO [0m] [http] [CPU3] http: response header: Transfer-Encoding: chunked
[57988925082] [[32mINFO [0m] [http] [CPU3] http: response header: Connection: close
[57993327744] [[32mINFO [0m] [http] [CPU3] http: response header: Server: cloudflare
[57995755686] [[32mINFO [0m] [http] [CPU3] http: response header: last-modified: Sat, 18 Apr 2026 00:49:31 GMT
[58000778352] [[32mINFO [0m] [http] [CPU3] http: response header: allow: GET, HEAD
[58003248666] [[32mINFO [0m] [http] [CPU3] http: response header: Accept-Ranges: bytes
[58005561306] [[32mINFO [0m] [http] [CPU3] http: response header: Age: 3293
[58007783988] [[32mINFO [0m] [http] [CPU3] http: response header: cf-cache-status: HIT
[58010143752] [[32mINFO [0m] [http] [CPU3] http: response header: CF-RAY: 9eff8263dfa2a4ad-SEA
[58013093820] [[32mINFO [0m] [http] [CPU3] http: response header:
210
<!doctype html><html lang="en"><head><title>Example Domain</title><meta name="viewport" content="width=device-width, initial-scale=1"><style>body{background:#eee;width:60vw;margin:15vh auto;font-family:system-ui,sans-serif}h1{font-size:1.5em}div{opacity:0.8}a:link,a:visited{color:#348}</style></head><body><div><h1>Example Domain</h1><p>This domain is for use in documentation examples without needing permission. Avoid use in operations.</p><p><a href="https://iana.org/domains/example">Learn more</a></p></div></body></html>

[58078496553] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://example.com offset=535 len=32768 cached=535 start=0 eof=false
[58266991596] [[32mINFO [0m] [http] [CPU1] http: background task: read 5 bytes from TLS
[58270554177] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 5 bytes to foreground
0

[58316560137] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://example.com offset=540 len=32768 cached=540 start=0 eof=false
[58563235500] [[32mINFO [0m] [httpsd] [CPU3] httpsd: upstream EOF for handle=2 cached=540
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /run/httpsd/cache/index
[?25lcat: failed to open /run/httpsd/cache/index
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /run/httpsd/cache/example.com/headers
cat: failed to open /run/httpsd/cache/example.com/headers
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /run/httpsd/cache/example.com/status

```
</details>
