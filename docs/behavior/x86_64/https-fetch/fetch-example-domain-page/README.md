# ✅ Scenario: Fetch Example Domain page

> Last run: 2026-04-21 14:37:54

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8414ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 2102ms | - [📜](./02/serial.log) - |
| 3 | And I type "ping -c 1 example.com" on the serial console | ✅ | 2128ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "1 packets transmitted, 1 received" | ✅ | 405ms | - [📜](./04/serial.log) - |
| 5 | And I type "cat /https/example.com" on the serial console | ✅ | 2191ms | - [📜](./05/serial.log) - |
| 6 | Then the serial output should contain "Example Domain" | ✅ | 1829ms | - [📜](./06/serial.log) - |
| 7 | And the serial output should contain "documentation examples" | ✅ | 0ms | - - - |
| 8 | And the serial output should contain "permission" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[24584909679] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26592729042] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26596390095] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26654472471] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[27278519499] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27381336312] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28963851939] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[28994855769] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[29135582982] [[32mINFO [0m] [iso9660d] [CPU1] iso9660d: no ISO9660 filesystem found yet �� retrying
ping -c 1 example.com
[?25l[38249634489] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
PING example.com (172.66.147.243) 56 bytes of data
64 bytes from 172.66.147.243: icmp_seq=1 time=1470ms

--- example.com ping statistics ---
1 packets transmitted, 1 received, 0% packet loss
rtt min/avg/max = 1470/1470/1470 ms
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/example.com
[?25l[46675387341] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://example.com offset=0 len=32768 cached=0 start=0 eof=false
[46678007607] [[32mINFO [0m] [httpsd] [CPU3] httpsd: opening upstream stream for handle=2 https://example.com
[46697259213] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=0/120)
[46699686528] [[32mINFO [0m] [http] [CPU1] http: connect host=example.com port=443
[46704076749] [[32mINFO [0m] [http] [CPU1] http: opening /net/tcp/new
[50990272674] [[32mINFO [0m] [http] [CPU1] http: allocated tcp socket id=2
[50991946467] [[32mINFO [0m] [http] [CPU1] http: opening ctl path /net/tcp/2/ctl
[51098458158] [[32mINFO [0m] [http] [CPU1] http: opening data path /net/tcp/2/data
[51187149816] [[32mINFO [0m] [http] [CPU1] http: issuing connect command: connect example.com 443
[51617942772] [[32mINFO [0m] [http] [CPU1] http: waiting for socket readiness...
[51620514132] [[32mINFO [0m] [http] [CPU1] http: waiting for http connect socket readiness (fd=4)...
[52072347525] [[32mINFO [0m] [http] [CPU1] http: wait_fd_ready complete for http connect socket revents=0x0004
[52075231032] [[32mINFO [0m] [http] [CPU1] http: background task: TCP connected to example.com
[52078021578] [[32mINFO [0m] [http] [CPU1] http: background task: starting TLS handshake with example.com
[52202854011] [[32mINFO [0m] [http] [CPU1] http: waiting for http read readiness (fd=4)...
[53410071297] [[32mINFO [0m] [http] [CPU1] http: wait_fd_ready complete for http read revents=0x0001
[54210595824] [[32mINFO [0m] [http] [CPU1] http: background task: TLS handshake complete for example.com
[54212719341] [[32mINFO [0m] [http] [CPU1] http: background task: sending 126 byte request
[54215251167] [[32mINFO [0m] [http] [CPU1] http: background task: wrote 126 bytes (total=126)
[54217028778] [[32mINFO [0m] [http] [CPU1] http: background task: flushing TLS stream
[54531550260] [[32mINFO [0m] [http] [CPU1] http: background task: entering read loop
[54894657642] [[32mINFO [0m] [http] [CPU1] http: waiting for http read readiness (fd=4)...
[55626387663] [[32mINFO [0m] [http] [CPU1] http: wait_fd_ready complete for http read revents=0x0001
[559516
```
</details>
