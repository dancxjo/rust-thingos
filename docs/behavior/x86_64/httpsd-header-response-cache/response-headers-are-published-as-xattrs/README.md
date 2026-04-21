# ❌ Scenario: Response headers are published as xattrs

> Last run: 2026-04-21 14:32:38

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8613ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 2103ms | - [📜](./02/serial.log) - |
| 3 | And I type "ping -c 1 example.com" on the serial console | ✅ | 2129ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "1 packets transmitted, 1 received" | ✅ | 406ms | - [📜](./04/serial.log) - |
| 5 | And I type "cat /https/example.com > /dev/null" on the serial console | ✅ | 2790ms | - [📜](./05/serial.log) - |
| 6 | And I wait for 5 seconds | ✅ | 5000ms | - [📜](./06/serial.log) - |
| 7 | And I type "attr_list /https/example.com" on the serial console | ✅ | 2489ms | - [📜](./07/serial.log) - |
| 8 | Then the serial output should contain "user.http.status_code" within 30s | ❌ | 31039ms | - [📜](./08/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[25487947716] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[27421054848] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[27423988284] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[27482455440] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[28038667140] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[28115625054] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[29653271703] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[29683123569] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[30239078760] [[32mINFO [0m] [iso9660d] [CPU1] iso9660d: no ISO9660 filesystem found yet �� retrying
ping -c 1 example.com
[?25l[38889740175] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
PING example.com (172.66.147.243) 56 bytes of data
64 bytes from 172.66.147.243: icmp_seq=1 time=1479ms

--- example.com ping statistics ---
1 packets transmitted, 1 received, 0% packet loss
rtt min/avg/max = 1479/1479/1479 ms
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/example.com > /dev/null
[49343335701] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://example.com offset=0 len=32768 cached=0 start=0 eof=false
[49345686654] [[32mINFO [0m] [httpsd] [CPU3] httpsd: opening upstream stream for handle=2 https://example.com
[49362602487] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=0/120)
[49363918626] [[32mINFO [0m] [http] [CPU1] http: connect host=example.com port=443
[49365512856] [[32mINFO [0m] [http] [CPU1] http: opening /net/tcp/new
[49537180506] [[32mINFO [0m] [http] [CPU1] http: allocated tcp socket id=2
[49538989896] [[32mINFO [0m] [http] [CPU1] http: opening ctl path /net/tcp/2/ctl
[49609925277] [[32mINFO [0m] [http] [CPU1] http: opening data path /net/tcp/2/data
[49683455118] [[32mINFO [0m] [http] [CPU1] http: issuing connect command: connect example.com 443
[50068896999] [[32mINFO [0m] [http] [CPU1] http: waiting for socket readiness...
[50073096909] [[32mINFO [0m] [http] [CPU1] http: waiting for http connect socket readiness (fd=4)...
[50468321541] [[32mINFO [0m] [http] [CPU1] http: wait_fd_ready complete for http connect socket revents=0x0004
[50471064402] [[32mINFO [0m] [http] [CPU1] http: background task: TCP connected to example.com
[50473870062] [[32mINFO [0m] [http] [CPU1] http: background task: starting TLS handshake with example.com
[?25l[50853119691] [[32mINFO [0m] [http] [CPU1] http: waiting for http read readiness (fd=4)...
[51725563824] [[32mINFO [0m] [http] [CPU1] http: wait_fd_ready complete for http read revents=0x0001
[52793655651] [[32mINFO [0m] [http] [CPU1] http: background task: TLS handshake complete for example.com
[52795695942] [[32mINFO [0m] [http] [CPU1] http: background task: sending 126 byte request
[52798179786] [[32mINFO [0m] [http] [CPU1] http: background task: wrote 126 bytes (total=126)
[52799826651] [[32mINFO [0m] [http] [CPU1] http: background task: flushing TLS stream
[53223554472] [[32mINFO [0m] [http] [CPU1] http: background task: entering read loop
[53499776781] [[32mINFO [0m] [http] [CPU1] http: waiting for http read readiness (fd=4)...
[54182847444] [[32mINFO [0m] [http] [CPU1] http: wait_fd_ready complete for http read revents=0x0001
[54567553161] [[32mINFO [0m] [http] [CPU1] http: background task: read 832 bytes from TLS
[54570350373] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 832 bytes to foreground
[54572224014] [[32mINFO [0m] [http] [CPU3] http: received 832 bytes from background TLS thread
[54575024559] [[32mINFO [0m] [http] [CPU3] http: headers complete body_start=297
[54577174212] [[32mINFO [0m] [http] [CPU3] http: response header: HTTP/1.1 200 OK
[54578757321] [[32mINFO [0m] [http] [CPU3] http: response header: Date: Tue, 21 Apr 2026 21:32:54 GMT
[54580498401] [[32mINFO [0m] [http] [CPU3] http: response header: Content-Type: text/html
[54582061413] [[32mINFO [0m] [http] [CPU3] http: response header: Transfer-Encoding: chunked
[54583617363] [[32mINFO [0m] [http] [CPU3] http: response header: Connection: close
[54585019335] [[32mINFO [0m] [http] [CPU3] http: response header: Server: cloudflare
[54586445034] [[32mINFO [0m] [http] [CPU3] http: response header: last-modified: Sat, 18 Apr 2026 00:51:00 GMT
[54588535122] [[32mINFO [0m] [http] [CPU3] http: response header: allow: GET, HEAD
[54590337549] [[32mINFO [0m] [http] [CPU3] http: response header: Accept-Ranges: bytes
[54591915774] [[32mINFO [0m] [http] [CPU3] http: response header: Age: 1878
[54594528318] [[32mINFO [0m] [http] [CPU3] http: response header: cf-cache-status: HIT
[54596212176] [[32mINFO [0m] [http] [CPU3] http: response header: CF-RAY: 9eff81085dc1c4cb-SEA
[54597939000] [[32mINFO [0m] [http] [CPU3] http: response header:
210
<!doctype html><html lang="en"><head><title>Example Domain</title><meta name="viewport" content="width=device-width, initial-scale=1"><style>body{background:#eee;width:60vw;margin:15vh auto;font-family:system-ui,sans-serif}h1{font-size:1.5em}div{opacity:0.8}a:link,a:visited{color:#348}</style></head><body><div><h1>Example Domain</h1><p>This domain is for use in documentation examples without needing permission. Avoid use in operations.</p><p><a href="https://iana.org/domains/example">Learn more</a></p></div></body></html>

[54643235889] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://example.com offset=535 len=32768 cached=535 start=0 eof=false
[54796604148] [[32mINFO [0m] [http] [CPU1] http: background task: read 5 bytes from TLS
[54799342620] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 5 bytes to foreground
0

[54836110032] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://example.com offset=540 len=32768 cached=540 start=0 eof=false
[54900419310] [[32mINFO [0m] [httpsd] [CPU3] httpsd: upstream EOF for handle=2 cached=540
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_list /https/example.com

```
</details>
