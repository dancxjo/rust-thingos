# ✅ Scenario: Cache mount exposes the raw response head and index

> Last run: 2026-04-21 14:38:13

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 10353ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 2101ms | - [📜](./02/serial.log) - |
| 3 | And I type "ping -c 1 example.com" on the serial console | ✅ | 2129ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "1 packets transmitted, 1 received" | ✅ | 406ms | - [📜](./04/serial.log) - |
| 5 | And I type "cat /https/example.com > /dev/null" on the serial console | ✅ | 2795ms | - [📜](./05/serial.log) - |
| 6 | And I wait for the shell prompt | ✅ | 2001ms | - [📜](./06/serial.log) - |
| 7 | And I type "cat /run/httpsd/cache/index" on the serial console | ✅ | 2435ms | - [📜](./07/serial.log) - |
| 8 | Then the serial output should contain "example.com" | ✅ | 0ms | - - - |
| 9 | When I type "cat /run/httpsd/cache/example.com/headers" on the serial console | ✅ | 3159ms | - [📜](./09/serial.log) - |
| 10 | Then the serial output should contain "HTTP/" | ✅ | 0ms | - - - |
| 11 | And the serial output should contain "Content-Type:" | ✅ | 0ms | - - - |
| 12 | When I type "cat /run/httpsd/cache/example.com/status" on the serial console | ✅ | 3101ms | - [📜](./12/serial.log) - |
| 13 | Then the serial output should contain "200" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[30858411276] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[33199526415] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[33202518063] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[33259909716] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[33866974383] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[33969776940] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[35719499673] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[35847555348] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[36956364951] [[32mINFO [0m] [iso9660d] [CPU1] iso9660d: no ISO9660 filesystem found yet �� retrying
ping -c 1 example.com[44273049777] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready

[?25lPING example.com (104.20.23.154) 56 bytes of data
64 bytes from 104.20.23.154: icmp_seq=1 time=1548ms

--- example.com ping statistics ---
1 packets transmitted, 1 received, 0% packet loss
rtt min/avg/max = 1548/1548/1548 ms
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/example.com > /dev/null
[55064010309] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://example.com offset=0 len=32768 cached=0 start=0 eof=false
[55066537053] [[32mINFO [0m] [httpsd] [CPU3] httpsd: opening upstream stream for handle=2 https://example.com
[55083983460] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=0/120)
[55085417739] [[32mINFO [0m] [http] [CPU1] http: connect host=example.com port=443
[55087284351] [[32mINFO [0m] [http] [CPU1] http: opening /net/tcp/new
[55226081625] [[32mINFO [0m] [http] [CPU1] http: allocated tcp socket id=2
[55227724299] [[32mINFO [0m] [http] [CPU1] http: opening ctl path /net/tcp/2/ctl
[55299890448] [[32mINFO [0m] [http] [CPU1] http: opening data path /net/tcp/2/data
[55371550344] [[32mINFO [0m] [http] [CPU1] http: issuing connect command: connect example.com 443
[55885194783] [[32mINFO [0m] [http] [CPU1] http: waiting for socket readiness...
[55887359649] [[32mINFO [0m] [http] [CPU1] http: waiting for http connect socket readiness (fd=4)...
[?25l[56691454149] [[32mINFO [0m] [http] [CPU1] http: wait_fd_ready complete for http connect socket revents=0x0004
[56694476025] [[32mINFO [0m] [http] [CPU1] http: background task: TCP connected to example.com
[56697736392] [[32mINFO [0m] [http] [CPU1] http: background task: starting TLS handshake with example.com
[57089007591] [[32mINFO [0m] [http] [CPU1] http: waiting for http read readiness (fd=4)...
[57835906095] [[32mINFO [0m] [http] [CPU1] http: wait_fd_ready complete for http read revents=0x0001
[59266842888] [[32mINFO [0m] [http] [CPU1] http: background task: TLS handshake complete for example.com
[59269004256] [[32mINFO [0m] [http] [CPU1] http: background task: sending 126 byte request
[59271302574] [[32mINFO [0m] [http] [CPU1] http: background task: wrote 126 bytes (total=126)
[59273076621] [[32mINFO [0m] [http] [CPU1] http: background task: flushing TLS stream
[59546822907] [[32mINFO [0m] [http] [CPU1] http: background task: entering read loop
[59803897197] [[32mINFO [0m] [http] [CPU1] http: waiting for http read readiness (fd=4)...
[60558887730] [[32mINFO [0m] [http] [CPU1] http: wait_fd_ready complete for http read revents=0x0001
[60826700979] [[32mINFO [0m] [http] [CPU1] http: background task: read 832 bytes from TLS
[60830414832] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 832 bytes to foreground
[60832321572] [[32mINFO [0m] [http] [CPU3] http: received 832 bytes from background TLS thread
[60835564977] [[32mINFO [0m] [http] [CPU3] http: headers complete body_start=297
[60837793005] [[32mINFO [0m] [http] [CPU3] http: response header: HTTP/1.1 200 OK
[60839476401] [[32mINFO [0m] [http] [CPU3] http: response header: Date: Tue, 21 Apr 2026 21:39:23 GMT
[60841150161] [[32mINFO [0m] [http] [CPU3] http: response header: Content-Type: text/html
[60842640309] [[32mINFO [0m] [http] [CPU3] http: response header: Transfer-Encoding: chunked
[60844214805] [[32mINFO [0m] [http] [CPU3] http: response header: Connection: close
[60845743563] [[32mINFO [0m] [http] [CPU3] http: response header: Server: cloudflare
[60847890840] [[32mINFO [0m] [http] [CPU3] http: response header: last-modified: Sat, 18 Apr 2026 00:49:31 GMT
[60849692805] [[32mINFO [0m] [http] [CPU3] http: response header: allow: GET, HEAD
[60851291292] [[32mINFO [0m] [http] [CPU3] http: response header: Accept-Ranges: bytes
[60852666996] [[32mINFO [0m] [http] [CPU3] http: response header: Age: 3626
[60854084874] [[32mINFO [0m] [http] [CPU3] http: response header: cf-cache-status: HIT
[60855410682] [[32mINFO [0m] [http] [CPU3] http: response header: CF-RAY: 9eff8a852c940999-SEA
[60857224494] [[32mINFO [0m] [http] [CPU3] http: response header:
210
<!doctype html><html lang="en"><head><title>Example Domain</title><meta name="viewport" content="width=device-width, initial-scale=1"><style>body{background:#eee;width:60vw;margin:15vh auto;font-family:system-ui,sans-serif}h1{font-size:1.5em}div{opacity:0.8}a:link,a:visited{color:#348}</style></head><body><div><h1>Example Domain</h1><p>This domain is for use in documentation examples without needing permission. Avoid use in operations.</p><p><a href="https://iana.org/domains/example">Learn more</a></p></div></body></html>

[60899386152] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://example.com offset=535 len=32768 cached=535 start=0 eof=false
[60921487407] [[32mINFO [0m] [http] [CPU1] http: background task: read 5 bytes from TLS
[60923785626] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 5 bytes to foreground
0

[60961287387] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://example.com offset=540 len=32768 cached=540 start=0 eof=false
[61210149990] [[32mINFO [0m] [httpsd] [CPU3] httpsd: upstream EOF for handle=2 cached=540
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /run/httpsd/cache/index
[?25lcat: failed to open /run/httpsd/cache/index
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /run/httpsd/cache/example.com/headers
cat: failed to open /run/httpsd/cache/example.com/headers
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /run/httpsd/cache/example.com/status

```
</details>
