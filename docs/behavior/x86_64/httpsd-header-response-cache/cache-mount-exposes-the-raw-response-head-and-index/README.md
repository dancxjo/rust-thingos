# ✅ Scenario: Cache mount exposes the raw response head and index

> Last run: 2026-04-21 14:40:41

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8721ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 2101ms | - [📜](./02/serial.log) - |
| 3 | And I type "ping -c 1 example.com" on the serial console | ✅ | 2130ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "1 packets transmitted, 1 received" | ✅ | 507ms | - [📜](./04/serial.log) - |
| 5 | And I type "cat /https/example.com > /dev/null" on the serial console | ✅ | 2793ms | - [📜](./05/serial.log) - |
| 6 | And I wait for the shell prompt | ✅ | 2000ms | - [📜](./06/serial.log) - |
| 7 | And I type "cat /run/httpsd/cache/index" on the serial console | ✅ | 2437ms | - [📜](./07/serial.log) - |
| 8 | Then the serial output should contain "example.com" | ✅ | 0ms | - - - |
| 9 | When I type "cat /run/httpsd/cache/example.com/headers" on the serial console | ✅ | 3162ms | - [📜](./09/serial.log) - |
| 10 | Then the serial output should contain "HTTP/" | ✅ | 0ms | - - - |
| 11 | And the serial output should contain "Content-Type:" | ✅ | 0ms | - - - |
| 12 | When I type "cat /run/httpsd/cache/example.com/status" on the serial console | ✅ | 3109ms | - [📜](./12/serial.log) - |
| 13 | Then the serial output should contain "200" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[25724825940] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[27718367259] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[27721328316] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[27778031820] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[28340118378] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[28472140455] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[30116955495] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[30156926910] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[30321312186] [[32mINFO [0m] [iso9660d] [CPU1] iso9660d: no ISO9660 filesystem found yet — retrying
ping -c 1 example.com[38912508624] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready

[?25lPING example.com (172.66.147.243) 56 bytes of data
64 bytes from 172.66.147.243: icmp_seq=1 time=1560ms

--- example.com ping statistics ---
1 packets transmitted, 1 received, 0% packet loss
rtt min/avg/max = 1560/1560/1560 ms
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/example.com > /dev/null
[?25l[50207085357] [[32mINFO [0m] [httpsd] [CPU1] httpsd: read handle=2 url=https://example.com offset=0 len=32768 cached=0 start=0 eof=false
[50211493893] [[32mINFO [0m] [httpsd] [CPU1] httpsd: opening upstream stream for handle=2 https://example.com
[50287414056] [[32mINFO [0m] [http] [CPU1] http: waiting for header data from port (attempt=0/120)
[50319689046] [[32mINFO [0m] [http] [CPU1] http: connect host=example.com port=443
[50323295088] [[32mINFO [0m] [http] [CPU1] http: opening /net/tcp/new
[50483032710] [[32mINFO [0m] [http] [CPU1] http: allocated tcp socket id=2
[50485121280] [[32mINFO [0m] [http] [CPU1] http: opening ctl path /net/tcp/2/ctl
[50575782642] [[32mINFO [0m] [http] [CPU1] http: opening data path /net/tcp/2/data
[50713852728] [[32mINFO [0m] [http] [CPU1] http: issuing connect command: connect example.com 443
[51230875971] [[32mINFO [0m] [http] [CPU1] http: waiting for socket readiness...
[51233904942] [[32mINFO [0m] [http] [CPU1] http: waiting for http connect socket readiness (fd=4)...
[52028596752] [[32mINFO [0m] [http] [CPU1] http: wait_fd_ready complete for http connect socket revents=0x0004
[52032190188] [[32mINFO [0m] [http] [CPU1] http: background task: TCP connected to example.com
[52035087753] [[32mINFO [0m] [http] [CPU1] http: background task: starting TLS handshake with example.com
[52439824668] [[32mINFO [0m] [http] [CPU1] http: waiting for http read readiness (fd=4)...
[53349427230] [[32mINFO [0m] [http] [CPU1] http: wait_fd_ready complete for http read revents=0x0001
[54053457315] [[32mINFO [0m] [http] [CPU1] http: background task: TLS handshake complete for example.com
[54055429791] [[32mINFO [0m] [http] [CPU1] http: background task: sending 126 byte request
[54057862254] [[32mINFO [0m] [http] [CPU1] http: background task: wrote 126 bytes (total=126)
[54060548322] [[32mINFO [0m] [http] [CPU1] http: background task: flushing TLS stream
[54428646195] [[32mINFO [0m] [http] [CPU1] http: background task: entering read loop
[54683751573] [[32mINFO [0m] [http] [CPU1] http: waiting for http read readiness (fd=4)...
[55556821716] [[32mINFO [0m] [http] [CPU1] http: wait_fd_ready complete for http read revents=0x0001
[55842618282] [[32mINFO [0m] [http] [CPU1] http: background task: read 832 bytes from TLS
[55846116447] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 832 bytes to foreground
[55857222102] [[32mINFO [0m] [http] [CPU1] http: received 832 bytes from background TLS thread
[55860638460] [[32mINFO [0m] [http] [CPU1] http: headers complete body_start=297
[55862880282] [[32mINFO [0m] [http] [CPU1] http: response header: HTTP/1.1 200 OK
[55864692906] [[32mINFO [0m] [http] [CPU1] http: response header: Date: Tue, 21 Apr 2026 21:41:49 GMT
[55866686238] [[32mINFO [0m] [http] [CPU1] http: response header: Content-Type: text/html
[55868625615] [[32mINFO [0m] [http] [CPU1] http: response header: Transfer-Encoding: chunked
[55871384382] [[32mINFO [0m] [http] [CPU1] http: response header: Connection: close
[55872891855] [[32mINFO [0m] [http] [CPU1] http: response header: Server: cloudflare
[55874415927] [[32mINFO [0m] [http] [CPU1] http: response header: last-modified: Sat, 18 Apr 2026 00:49:31 GMT
[55876587888] [[32mINFO [0m] [http] [CPU1] http: response header: allow: GET, HEAD
[55878418167] [[32mINFO [0m] [http] [CPU1] http: response header: Accept-Ranges: bytes
[55880480073] [[32mINFO [0m] [http] [CPU1] http: response header: Age: 3772
[55882397274] [[32mINFO [0m] [http] [CPU1] http: response header: cf-cache-status: HIT
[55884690477] [[32mINFO [0m] [http] [CPU1] http: response header: CF-RAY: 9eff8e17bbb1c3ca-SEA
[55887285894] [[32mINFO [0m] [http] [CPU1] http: response header:
210
<!doctype html><html lang="en"><head><title>Example Domain</title><meta name="viewport" content="width=device-width, initial-scale=1"><style>body{background:#eee;width:60vw;margin:15vh auto;font-family:system-ui,sans-serif}h1{font-size:1.5em}div{opacity:0.8}a:link,a:visited{color:#348}</style></head><body><div><h1>Example Domain</h1><p>This domain is for use in documentation examples without needing permission. Avoid use in operations.</p><p><a href="https://iana.org/domains/example">Learn more</a></p></div></body></html>

[55933765107] [[32mINFO [0m] [httpsd] [CPU1] httpsd: read handle=2 url=https://example.com offset=535 len=32768 cached=535 start=0 eof=false
[56063559585] [[32mINFO [0m] [http] [CPU1] http: background task: read 5 bytes from TLS
[56066089101] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 5 bytes to foreground
0

[56120306814] [[32mINFO [0m] [httpsd] [CPU1] httpsd: read handle=2 url=https://example.com offset=540 len=32768 cached=540 start=0 eof=false
[56317748817] [[32mINFO [0m] [httpsd] [CPU1] httpsd: upstream EOF for handle=2 cached=540
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /run/httpsd/cache/index
[?25lcat: failed to open /run/httpsd/cache/index
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /run/httpsd/cache/example.com/headers
cat: failed to open /run/httpsd/cache/example.com/headers
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /run/httpsd/cache/example.com/status

```
</details>
