# ✅ Scenario: Cache mount exposes the raw response head and index

> Last run: 2026-04-21 14:30:14

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8510ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 2203ms | - [📜](./02/serial.log) - |
| 3 | And I type "ping -c 1 example.com" on the serial console | ✅ | 2130ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "1 packets transmitted, 1 received" | ✅ | 507ms | - [📜](./04/serial.log) - |
| 5 | And I type "cat /https/example.com > /dev/null" on the serial console | ✅ | 2789ms | - [📜](./05/serial.log) - |
| 6 | And I wait for 2 seconds | ✅ | 2001ms | - [📜](./06/serial.log) - |
| 7 | And I type "cat /run/httpsd/cache/index" on the serial console | ✅ | 2435ms | - [📜](./07/serial.log) - |
| 8 | Then the serial output should contain "example.com" | ✅ | 0ms | - - - |
| 9 | When I type "cat /run/httpsd/cache/example.com/headers" on the serial console | ✅ | 3154ms | - [📜](./09/serial.log) - |
| 10 | Then the serial output should contain "HTTP/" | ✅ | 0ms | - - - |
| 11 | And the serial output should contain "Content-Type:" | ✅ | 0ms | - - - |
| 12 | When I type "cat /run/httpsd/cache/example.com/status" on the serial console | ✅ | 3104ms | - [📜](./12/serial.log) - |
| 13 | Then the serial output should contain "200" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[25153532085] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[27138710478] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[27143351829] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[27205693053] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[27787946769] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27913313604] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[29660161938] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[29711089584] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[30387474546] [[32mINFO [0m] [iso9660d] [CPU1] iso9660d: no ISO9660 filesystem found yet ��� retrying
ping -c 1 example.co[38365333875] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
m
[?25lPING example.com (172.66.147.243) 56 bytes of data
64 bytes from 172.66.147.243: icmp_seq=1 time=1635ms

--- example.com ping statistics ---
1 packets transmitted, 1 received, 0% packet loss
rtt min/avg/max = 1635/1635/1635 ms
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/example.com > /dev/null
[49654617177] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://example.com offset=0 len=32768 cached=0 start=0 eof=false
[49656958857] [[32mINFO [0m] [httpsd] [CPU3] httpsd: opening upstream stream for handle=2 https://example.com
[49673917953] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=0/120)
[49675388499] [[32mINFO [0m] [http] [CPU1] http: connect host=example.com port=443
[49677969429] [[32mINFO [0m] [http] [CPU1] http: opening /net/tcp/new
[49840277817] [[32mINFO [0m] [http] [CPU1] http: allocated tcp socket id=2
[49843001076] [[32mINFO [0m] [http] [CPU1] http: opening ctl path /net/tcp/2/ctl
[49917869133] [[32mINFO [0m] [http] [CPU1] http: opening data path /net/tcp/2/data
[49997334981] [[32mINFO [0m] [http] [CPU1] http: issuing connect command: connect example.com 443
[50402344344] [[32mINFO [0m] [http] [CPU1] http: waiting for socket readiness...
[50405606229] [[32mINFO [0m] [http] [CPU1] http: waiting for http connect socket readiness (fd=4)...
[51310907307] [[32mINFO [0m] [http] [CPU1] http: wait_fd_ready complete for http connect socket revents=0x0004
[51315281589] [[32mINFO [0m] [http] [CPU1] http: background task: TCP connected to example.com
[51318676893] [[32mINFO [0m] [http] [CPU1] http: background task: starting TLS handshake with example.com
[?25l[51663414231] [[32mINFO [0m] [http] [CPU1] http: waiting for http read readiness (fd=4)...
[52451048265] [[32mINFO [0m] [http] [CPU1] http: wait_fd_ready complete for http read revents=0x0001
[53214819834] [[32mINFO [0m] [http] [CPU1] http: background task: TLS handshake complete for example.com
[53216782476] [[32mINFO [0m] [http] [CPU1] http: background task: sending 126 byte request
[53219346312] [[32mINFO [0m] [http] [CPU1] http: background task: wrote 126 bytes (total=126)
[53221053402] [[32mINFO [0m] [http] [CPU1] http: background task: flushing TLS stream
[53495384448] [[32mINFO [0m] [http] [CPU1] http: background task: entering read loop
[53745884280] [[32mINFO [0m] [http] [CPU1] http: waiting for http read readiness (fd=4)...
[54357273498] [[32mINFO [0m] [http] [CPU1] http: wait_fd_ready complete for http read revents=0x0001
[54796406148] [[32mINFO [0m] [http] [CPU1] http: background task: read 832 bytes from TLS
[54798955398] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 832 bytes to foreground
[54800523591] [[32mINFO [0m] [http] [CPU3] http: received 832 bytes from background TLS thread
[54803302257] [[32mINFO [0m] [http] [CPU3] http: headers complete body_start=297
[54805461612] [[32mINFO [0m] [http] [CPU3] http: response header: HTTP/1.1 200 OK
[54807240774] [[32mINFO [0m] [http] [CPU3] http: response header: Date: Tue, 21 Apr 2026 21:31:21 GMT
[54809004855] [[32mINFO [0m] [http] [CPU3] http: response header: Content-Type: text/html
[54810730260] [[32mINFO [0m] [http] [CPU3] http: response header: Transfer-Encoding: chunked
[54812296473] [[32mINFO [0m] [http] [CPU3] http: response header: Connection: close
[54813762432] [[32mINFO [0m] [http] [CPU3] http: response header: Server: cloudflare
[54815062929] [[32mINFO [0m] [http] [CPU3] http: response header: last-modified: Sat, 18 Apr 2026 00:51:00 GMT
[54816775992] [[32mINFO [0m] [http] [CPU3] http: response header: allow: GET, HEAD
[54818522451] [[32mINFO [0m] [http] [CPU3] http: response header: Accept-Ranges: bytes
[54819950955] [[32mINFO [0m] [http] [CPU3] http: response header: Age: 1786
[54821264751] [[32mINFO [0m] [http] [CPU3] http: response header: cf-cache-status: HIT
[54822805026] [[32mINFO [0m] [http] [CPU3] http: response header: CF-RAY: 9eff7ec66d97b455-SEA
[54824591613] [[32mINFO [0m] [http] [CPU3] http: response header:
210
<!doctype html><html lang="en"><head><title>Example Domain</title><meta name="viewport" content="width=device-width, initial-scale=1"><style>body{background:#eee;width:60vw;margin:15vh auto;font-family:system-ui,sans-serif}h1{font-size:1.5em}div{opacity:0.8}a:link,a:visited{color:#348}</style></head><body><div><h1>Example Domain</h1><p>This domain is for use in documentation examples without needing permission. Avoid use in operations.</p><p><a href="https://iana.org/domains/example">Learn more</a></p></div></body></html>

[54865387137] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://example.com offset=535 len=32768 cached=535 start=0 eof=false
[54950752791] [[32mINFO [0m] [http] [CPU1] http: background task: read 5 bytes from TLS
[54953091666] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 5 bytes to foreground
0

[54985900431] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://example.com offset=540 len=32768 cached=540 start=0 eof=false
[55236637368] [[32mINFO [0m] [httpsd] [CPU3] httpsd: upstream EOF for handle=2 cached=540
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /run/httpsd/cache/index
[?25lcat: failed to open /run/httpsd/cache/index
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /run/httpsd/cache/example.com/headers
cat: failed to open /run/httpsd/cache/example.com/headers
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /run/httpsd/cache/example.com/status

```
</details>
