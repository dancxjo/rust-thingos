# ❌ Scenario: Response headers are published as xattrs

> Last run: 2026-04-21 14:40:41

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8206ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 2102ms | - [📜](./02/serial.log) - |
| 3 | And I type "ping -c 1 example.com" on the serial console | ✅ | 2130ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "1 packets transmitted, 1 received" | ✅ | 608ms | - [📜](./04/serial.log) - |
| 5 | And I type "cat /https/example.com > /dev/null" on the serial console | ✅ | 2790ms | - [📜](./05/serial.log) - |
| 6 | And I wait for the shell prompt | ✅ | 2000ms | - [📜](./06/serial.log) - |
| 7 | And I type "attr_list /https/example.com" on the serial console | ✅ | 2488ms | - [📜](./07/serial.log) - |
| 8 | Then the serial output should contain "user.http.status_code" within 30s | ❌ | 31011ms | - [📜](./08/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[24280983408] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26103536019] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26107393455] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26164315254] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26694696501] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[26768145096] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28242531108] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[28274562987] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[28404539460] [[32mINFO [0m] [iso9660d] [CPU1] iso9660d: no ISO9660 filesystem found yet — retrying
ping -c 1 exam[36077261616] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
ple.com
[?25lPING example.com (172.66.147.243) 56 bytes of data
64 bytes from 172.66.147.243: icmp_seq=1 time=1729ms

--- example.com ping statistics ---
1 packets transmitted, 1 received, 0% packet loss
rtt min/avg/max = 1729/1729/1729 ms
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/example.com > /dev/null
[?25l[48657881712] [[32mINFO [0m] [httpsd] [CPU1] httpsd: read handle=2 url=https://example.com offset=0 len=32768 cached=0 start=0 eof=false
[48660482211] [[32mINFO [0m] [httpsd] [CPU1] httpsd: opening upstream stream for handle=2 https://example.com
[48680547927] [[32mINFO [0m] [http] [CPU1] http: waiting for header data from port (attempt=0/120)
[48685536735] [[32mINFO [0m] [http] [CPU1] http: connect host=example.com port=443
[48687586299] [[32mINFO [0m] [http] [CPU1] http: opening /net/tcp/new
[48839272185] [[32mINFO [0m] [http] [CPU1] http: allocated tcp socket id=2
[48841537965] [[32mINFO [0m] [http] [CPU1] http: opening ctl path /net/tcp/2/ctl
[48914832252] [[32mINFO [0m] [http] [CPU1] http: opening data path /net/tcp/2/data
[48984907290] [[32mINFO [0m] [http] [CPU1] http: issuing connect command: connect example.com 443
[49485322062] [[32mINFO [0m] [http] [CPU1] http: waiting for socket readiness...
[49488210321] [[32mINFO [0m] [http] [CPU1] http: waiting for http connect socket readiness (fd=4)...
[49813069647] [[32mINFO [0m] [http] [CPU1] http: wait_fd_ready complete for http connect socket revents=0x0004
[49817132112] [[32mINFO [0m] [http] [CPU1] http: background task: TCP connected to example.com
[49820688324] [[32mINFO [0m] [http] [CPU1] http: background task: starting TLS handshake with example.com
[50293243869] [[32mINFO [0m] [http] [CPU1] http: waiting for http read readiness (fd=4)...
[50976396570] [[32mINFO [0m] [http] [CPU1] http: wait_fd_ready complete for http read revents=0x0001
[52148083383] [[32mINFO [0m] [http] [CPU1] http: background task: TLS handshake complete for example.com
[52150489842] [[32mINFO [0m] [http] [CPU1] http: background task: sending 126 byte request
[52152755523] [[32mINFO [0m] [http] [CPU1] http: background task: wrote 126 bytes (total=126)
[52154338830] [[32mINFO [0m] [http] [CPU1] http: background task: flushing TLS stream
[52416014244] [[32mINFO [0m] [http] [CPU1] http: background task: entering read loop
[53161698711] [[32mINFO [0m] [http] [CPU1] http: background task: read 832 bytes from TLS
[53164329570] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 832 bytes to foreground
[53172019164] [[32mINFO [0m] [http] [CPU1] http: received 832 bytes from background TLS thread
[53174768130] [[32mINFO [0m] [http] [CPU1] http: headers complete body_start=297
[53176800963] [[32mINFO [0m] [http] [CPU1] http: response header: HTTP/1.1 200 OK
[53178327444] [[32mINFO [0m] [http] [CPU1] http: response header: Date: Tue, 21 Apr 2026 21:40:57 GMT
[53179926129] [[32mINFO [0m] [http] [CPU1] http: response header: Content-Type: text/html
[53181424065] [[32mINFO [0m] [http] [CPU1] http: response header: Transfer-Encoding: chunked
[53182999419] [[32mINFO [0m] [http] [CPU1] http: response header: Connection: close
[53184312192] [[32mINFO [0m] [http] [CPU1] http: response header: Server: cloudflare
[53186783265] [[32mINFO [0m] [http] [CPU1] http: response header: last-modified: Sat, 18 Apr 2026 00:49:31 GMT
[53188466793] [[32mINFO [0m] [http] [CPU1] http: response header: allow: GET, HEAD
[53189884605] [[32mINFO [0m] [http] [CPU1] http: response header: Accept-Ranges: bytes
[53191238727] [[32mINFO [0m] [http] [CPU1] http: response header: Age: 3720
[53192494608] [[32mINFO [0m] [http] [CPU1] http: response header: cf-cache-status: HIT
[53193848565] [[32mINFO [0m] [http] [CPU1] http: response header: CF-RAY: 9eff8cd29ee70913-SEA
[53195593374] [[32mINFO [0m] [http] [CPU1] http: response header:
210
<!doctype html><html lang="en"><head><title>Example Domain</title><meta name="viewport" content="width=device-width, initial-scale=1"><style>body{background:#eee;width:60vw;margin:15vh auto;font-family:system-ui,sans-serif}h1{font-size:1.5em}div{opacity:0.8}a:link,a:visited{color:#348}</style></head><body><div><h1>Example Domain</h1><p>This domain is for use in documentation examples without needing permission. Avoid use in operations.</p><p><a href="https://iana.org/domains/example">Learn more</a></p></div></body></html>

[53220802965] [[32mINFO [0m] [httpsd] [CPU1] httpsd: read handle=2 url=https://example.com offset=535 len=32768 cached=535 start=0 eof=false
[53314610613] [[32mINFO [0m] [http] [CPU1] http: background task: read 5 bytes from TLS
[53316706146] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 5 bytes to foreground
0

[53341614084] [[32mINFO [0m] [httpsd] [CPU1] httpsd: read handle=2 url=https://example.com offset=540 len=32768 cached=540 start=0 eof=false
[53611772775] [[32mINFO [0m] [httpsd] [CPU1] httpsd: upstream EOF for handle=2 cached=540
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_list /https/example.com
[?25l
```
</details>
