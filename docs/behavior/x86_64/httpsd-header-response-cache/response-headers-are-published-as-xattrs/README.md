# ❌ Scenario: Response headers are published as xattrs

> Last run: 2026-04-21 14:38:13

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8712ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 2102ms | - [📜](./02/serial.log) - |
| 3 | And I type "ping -c 1 example.com" on the serial console | ✅ | 2127ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "1 packets transmitted, 1 received" | ✅ | 303ms | - [📜](./04/serial.log) - |
| 5 | And I type "cat /https/example.com > /dev/null" on the serial console | ✅ | 2796ms | - [📜](./05/serial.log) - |
| 6 | And I wait for the shell prompt | ✅ | 2000ms | - [📜](./06/serial.log) - |
| 7 | And I type "attr_list /https/example.com" on the serial console | ✅ | 2486ms | - [📜](./07/serial.log) - |
| 8 | Then the serial output should contain "user.http.status_code" within 30s | ❌ | 31034ms | - [📜](./08/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[25697569788] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[27636014274] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[27639016383] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[27699094467] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[28260286296] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[28339652253] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[30038985339] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[30090379242] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[30195334983] [[32mINFO [0m] [iso9660d] [CPU1] iso9660d: no ISO9660 filesystem found yet �� retrying
ping -c 1 example.c[38662528674] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
om
[?25lPING example.com (104.20.23.154) 56 bytes of data
64 bytes from 104.20.23.154: icmp_seq=1 time=1311ms

--- example.com ping statistics ---
1 packets transmitted, 1 received, 0% packet loss
rtt min/avg/max = 1311/1311/1311 ms
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/example.com > /dev/null
[?25l[49408994217] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://example.com offset=0 len=32768 cached=0 start=0 eof=false
[49412941182] [[32mINFO [0m] [httpsd] [CPU3] httpsd: opening upstream stream for handle=2 https://example.com
[49433095470] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=0/120)
[49435202487] [[32mINFO [0m] [http] [CPU1] http: connect host=example.com port=443
[49436897202] [[32mINFO [0m] [http] [CPU1] http: opening /net/tcp/new
[53648888766] [[32mINFO [0m] [http] [CPU1] http: allocated tcp socket id=2
[53650996707] [[32mINFO [0m] [http] [CPU1] http: opening ctl path /net/tcp/2/ctl
[53753015349] [[32mINFO [0m] [http] [CPU1] http: opening data path /net/tcp/2/data
[53817675615] [[32mINFO [0m] [http] [CPU1] http: issuing connect command: connect example.com 443
[54347086662] [[32mINFO [0m] [http] [CPU1] http: waiting for socket readiness...
[54352412730] [[32mINFO [0m] [http] [CPU1] http: waiting for http connect socket readiness (fd=4)...
[55217517993] [[32mINFO [0m] [http] [CPU1] http: wait_fd_ready complete for http connect socket revents=0x0004
[55222153998] [[32mINFO [0m] [http] [CPU1] http: background task: TCP connected to example.com
[55225564053] [[32mINFO [0m] [http] [CPU1] http: background task: starting TLS handshake with example.com
[55806820674] [[32mINFO [0m] [http] [CPU1] http: waiting for http read readiness (fd=4)...
[56324257077] [[32mINFO [0m] [http] [CPU1] http: wait_fd_ready complete for http read revents=0x0001
[57737153595] [[32mINFO [0m] [http] [CPU1] http: background task: TLS handshake complete for example.com
[57739994862] [[32mINFO [0m] [http] [CPU1] http: background task: sending 126 byte request
[57742901997] [[32mINFO [0m] [http] [CPU1] http: background task: wrote 126 bytes (total=126)
[57745743297] [[32mINFO [0m] [http] [CPU1] http: background task: flushing TLS stream
[57809578497] [[32mINFO [0m] [http] [CPU1] http: background task: entering read loop
[58634599650] [[32mINFO [0m] [http] [CPU1] http: background task: read 832 bytes from TLS
[58637637465] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 832 bytes to foreground
[58639475136] [[32mINFO [0m] [http] [CPU3] http: received 832 bytes from background TLS thread
[58643115498] [[32mINFO [0m] [http] [CPU3] http: headers complete body_start=297
[58646073090] [[32mINFO [0m] [http] [CPU3] http: response header: HTTP/1.1 200 OK
[58648445559] [[32mINFO [0m] [http] [CPU3] http: response header: Date: Tue, 21 Apr 2026 21:38:30 GMT
[58651824957] [[32mINFO [0m] [http] [CPU3] http: response header: Content-Type: text/html
[58654333881] [[32mINFO [0m] [http] [CPU3] http: response header: Transfer-Encoding: chunked
[58656639459] [[32mINFO [0m] [http] [CPU3] http: response header: Connection: close
[58659101292] [[32mINFO [0m] [http] [CPU3] http: response header: Server: cloudflare
[58661466567] [[32mINFO [0m] [http] [CPU3] http: response header: last-modified: Sat, 18 Apr 2026 00:49:31 GMT
[58664055351] [[32mINFO [0m] [http] [CPU3] http: response header: allow: GET, HEAD
[58666088415] [[32mINFO [0m] [http] [CPU3] http: response header: Accept-Ranges: bytes
[58667530251] [[32mINFO [0m] [http] [CPU3] http: response header: Age: 3574
[58668836688] [[32mINFO [0m] [http] [CPU3] http: response header: cf-cache-status: HIT
[58670221764] [[32mINFO [0m] [http] [CPU3] http: response header: CF-RAY: 9eff893f5e94908e-SEA
[58672189158] [[32mINFO [0m] [http] [CPU3] http: response header:
210
<!doctype html><html lang="en"><head><title>Example Domain</title><meta name="viewport" content="width=device-width, initial-scale=1"><style>body{background:#eee;width:60vw;margin:15vh auto;font-family:system-ui,sans-serif}h1{font-size:1.5em}div{opacity:0.8}a:link,a:visited{color:#348}</style></head><body><div><h1>Example Domain</h1><p>This domain is for use in documentation examples without needing permission. Avoid use in operations.</p><p><a href="https://iana.org/domains/example">Learn more</a></p></div></body></html>

[58706991915] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://example.com offset=535 len=32768 cached=535 start=0 eof=false
[58962944337] [[32mINFO [0m] [http] [CPU1] http: background task: read 5 bytes from TLS
[58966237275] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 5 bytes to foreground
0

[59004113091] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://example.com offset=540 len=32768 cached=540 start=0 eof=false
[59107410681] [[32mINFO [0m] [httpsd] [CPU3] httpsd: upstream EOF for handle=2 cached=540
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_list /https/example.com

```
</details>
