# ❌ Scenario: 3xx redirects surface as symlinks after first fetch

> Last run: 2026-04-21 14:32:38

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 9325ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 2102ms | - [📜](./02/serial.log) - |
| 3 | And I type "ping -c 1 example.org" on the serial console | ✅ | 2125ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "1 packets transmitted, 1 received" | ✅ | 511ms | - [📜](./04/serial.log) - |
| 5 | And I type "cat /https/example.org > /dev/null" on the serial console | ✅ | 2798ms | - [📜](./05/serial.log) - |
| 6 | And I wait for 5 seconds | ✅ | 5000ms | - [📜](./06/serial.log) - |
| 7 | And I type "attr_get /https/example.org user.http.location" on the serial console | ✅ | 3425ms | - [📜](./07/serial.log) - |
| 8 | Then the serial output should contain "example.com" | ❌ | 301021ms | - [📜](./08/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[27524936670] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[29522407002] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[29526221736] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[29590899327] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[30169131795] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[30248193492] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[32099860155] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[32142567864] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[33142998306] [[32mINFO [0m] [iso9660d] [CPU1] iso9660d: no ISO9660 filesystem found yet — retrying
ping -c 1 example[40056123096] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
.org
[?25lPING example.org (172.66.157.237) 56 bytes of data
64 bytes from 172.66.157.237: icmp_seq=1 time=1484ms

--- example.org ping statistics ---
1 packets transmitted, 1 received, 0% packet loss
rtt min/avg/max = 1484/1484/1484 ms
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/example.org > /dev/null
[51881783514] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://example.org offset=0 len=32768 cached=0 start=0 eof=false
[51885536670] [[32mINFO [0m] [httpsd] [CPU3] httpsd: opening upstream stream for handle=2 https://example.org
[51908216613] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=0/120)
[51913285677] [[32mINFO [0m] [http] [CPU1] http: connect host=example.org port=443
[51915218916] [[32mINFO [0m] [http] [CPU1] http: opening /net/tcp/new
[52205832393] [[32mINFO [0m] [http] [CPU1] http: allocated tcp socket id=2
[52207871661] [[32mINFO [0m] [http] [CPU1] http: opening ctl path /net/tcp/2/ctl
[52313800473] [[32mINFO [0m] [http] [CPU1] http: opening data path /net/tcp/2/data
[52412146611] [[32mINFO [0m] [http] [CPU1] http: issuing connect command: connect example.org 443
[53521985979] [[32mINFO [0m] [http] [CPU2] http: waiting for socket readiness...
[53525201037] [[32mINFO [0m] [http] [CPU2] http: waiting for http connect socket readiness (fd=4)...
[54103810695] [[32mINFO [0m] [http] [CPU2] http: wait_fd_ready complete for http connect socket revents=0x0004
[54109071621] [[32mINFO [0m] [http] [CPU2] http: background task: TCP connected to example.org
[54112863420] [[32mINFO [0m] [http] [CPU2] http: background task: starting TLS handshake with example.org
[?25l[54958298637] [[32mINFO [0m] [http] [CPU2] http: waiting for http read readiness (fd=4)...
[55756354830] [[32mINFO [0m] [http] [CPU2] http: wait_fd_ready complete for http read revents=0x0001
[58447875420] [[32mINFO [0m] [http] [CPU2] http: background task: TLS handshake complete for example.org
[58452448197] [[32mINFO [0m] [http] [CPU2] http: background task: sending 126 byte request
[58463567316] [[32mINFO [0m] [http] [CPU2] http: background task: wrote 126 bytes (total=126)
[58465980045] [[32mINFO [0m] [http] [CPU2] http: background task: flushing TLS stream
[58695366246] [[32mINFO [0m] [http] [CPU2] http: background task: entering read loop
[59098141410] [[32mINFO [0m] [http] [CPU2] http: waiting for http read readiness (fd=4)...
[59782833198] [[32mINFO [0m] [http] [CPU2] http: wait_fd_ready complete for http read revents=0x0001
[60533439780] [[32mINFO [0m] [http] [CPU2] http: background task: read 861 bytes from TLS
[60537932334] [[32mINFO [0m] [http] [CPU2] http: background task: forwarded 861 bytes to foreground
[60541479174] [[32mINFO [0m] [http] [CPU3] http: received 861 bytes from background TLS thread
[60545447688] [[32mINFO [0m] [http] [CPU3] http: headers complete body_start=326
[60549299283] [[32mINFO [0m] [http] [CPU3] http: response header: HTTP/1.1 200 OK
[60551768673] [[32mINFO [0m] [http] [CPU3] http: response header: Date: Tue, 21 Apr 2026 21:34:22 GMT
[60555252120] [[32mINFO [0m] [http] [CPU3] http: response header: Content-Type: text/html
[60557640396] [[32mINFO [0m] [http] [CPU3] http: response header: Transfer-Encoding: chunked
[60559924359] [[32mINFO [0m] [http] [CPU3] http: response header: Connection: close
[60562042563] [[32mINFO [0m] [http] [CPU3] http: response header: Server: cloudflare
[60564259734] [[32mINFO [0m] [http] [CPU3] http: response header: last-modified: Sat, 18 Apr 2026 00:49:31 GMT
[60566987712] [[32mINFO [0m] [http] [CPU3] http: response header: allow: GET, HEAD
[60569365923] [[32mINFO [0m] [http] [CPU3] http: response header: Accept-Ranges: bytes
[60572453601] [[32mINFO [0m] [http] [CPU3] http: response header: Age: 999
[60574566591] [[32mINFO [0m] [http] [CPU3] http: response header: Cache-Control: max-age=14400
[60576489369] [[32mINFO [0m] [http] [CPU3] http: response header: cf-cache-status: HIT
[60578036277] [[32mINFO [0m] [http] [CPU3] http: response header: CF-RAY: 9eff832c397bc391-SEA
[60580284567] [[32mINFO [0m] [http] [CPU3] http: response header:
210
<!doctype html><html lang="en"><head><title>Example Domain</title><meta name="viewport" content="width=device-width, initial-scale=1"><style>body{background:#eee;width:60vw;margin:15vh auto;font-family:system-ui,sans-serif}h1{font-size:1.5em}div{opacity:0.8}a:link,a:visited{color:#348}</style></head><body><div><h1>Example Domain</h1><p>This domain is for use in documentation examples without needing permission. Avoid use in operations.</p><p><a href="https://iana.org/domains/example">Learn more</a></p></div></body></html>

[60624784110] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://example.org offset=535 len=32768 cached=535 start=0 eof=false
[60868125120] [[32mINFO [0m] [http] [CPU2] http: background task: read 5 bytes from TLS
[60871195374] [[32mINFO [0m] [http] [CPU2] http: background task: forwarded 5 bytes to foreground
0

[60935849403] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://example.org offset=540 len=32768 cached=540 start=0 eof=false
[61236476433] [[32mINFO [0m] [httpsd] [CPU3] httpsd: upstream EOF for handle=2 cached=540
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_get /https/example.org user.http.location
[?25lattr_get: failed: EIO
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;91mERR[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hqemu-system-x86_64: terminating on signal 15 from pid 133142 (<unknown process>)

```
</details>
