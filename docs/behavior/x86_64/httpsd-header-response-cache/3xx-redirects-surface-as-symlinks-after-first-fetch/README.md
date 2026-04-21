# ❌ Scenario: 3xx redirects surface as symlinks after first fetch

> Last run: 2026-04-21 15:34:08

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ❌ | 1821ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[28365279822] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[30865916037] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[30869605404] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[30937800465] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[31590559176] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[31688340189] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[34203681930] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[34276162965] [[32mINFO [0m] [iso9660d] [CPU3] iso9660d: no ISO9660 filesystem found yet �� retrying
[34279184346] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[36939536370] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
ping -c 1 iana.org
[?25lPING iana.org (192.0.43.8) 56 bytes of data
64 bytes from 192.0.43.8: icmp_seq=1 time=750ms

--- iana.org ping statistics ---
1 packets transmitted, 1 received, 0% packet loss
rtt min/avg/max = 750/750/750 ms
m[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hmount -t https none /https
[?25l[50732319924] [[32mINFO [0m] [httpsd] [CPU3] httpsd: entering main RPC loop for /https
[51019607760] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=4
[51022756389] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path=''
mounted type=https device=none target=/https
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_get /https/iana.org user.http.location
[?25l[61168901145] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=12
[61172505009] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path='iana.org'
[61206060432] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=AttrGet payload_len=28
[61213137546] [[32mINFO [0m] [httpsd] [CPU3] httpsd: ensure_cached_entry handle=2 key=CacheKey { host: "iana.org", path: "" }
[61218348411] [[32mINFO [0m] [httpsd] [CPU3] httpsd: ensure_upstream handle=2 MISS https://iana.org - opening network stream
[61244220246] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=0/120)
[61251377319] [[32mINFO [0m] [http] [CPU3] http: connect host=iana.org port=443
[61254298017] [[32mINFO [0m] [http] [CPU3] http: opening /net/tcp/new
[61385201559] [[32mINFO [0m] [http] [CPU3] http: allocated tcp socket id=2
[61388185089] [[32mINFO [0m] [http] [CPU3] http: opening ctl path /net/tcp/2/ctl
[61502948265] [[32mINFO [0m] [http] [CPU3] http: opening data path /net/tcp/2/data
[61586404836] [[32mINFO [0m] [http] [CPU3] http: issuing connect command: connect iana.org 443
[62159965494] [[32mINFO [0m] [http] [CPU3] http: waiting for socket readiness...
[62163042612] [[32mINFO [0m] [http] [CPU3] http: waiting for http connect socket readiness (fd=7)...
[62918789835] [[32mINFO [0m] [http] [CPU3] http: wait_fd_ready complete for http connect socket revents=0x0004
[62922419571] [[32mINFO [0m] [http] [CPU3] http: background task: TCP connected to iana.org
[62925793128] [[32mINFO [0m] [http] [CPU3] http: background task: starting TLS handshake with iana.org
[63355740888] [[32mINFO [0m] [http] [CPU3] http: waiting for http read readiness (fd=7)...
[64560935274] [[32mINFO [0m] [http] [CPU3] http: wait_fd_ready complete for http read revents=0x0001
[65543622804] [[32mINFO [0m] [http] [CPU3] http: background task: TLS handshake complete for iana.org
[65546291613] [[32mINFO [0m] [http] [CPU3] http: background task: sending 168 byte request
[65549779581] [[32mINFO [0m] [http] [CPU3] http: background task: wrote 168 bytes (total=168)
[65552166933] [[32mINFO [0m] [http] [CPU3] http: background task: flushing TLS stream
[65778200367] [[32mINFO [0m] [http] [CPU3] http: background task: entering read loop
[66012838023] [[32mINFO [0m] [http] [CPU3] http: waiting for http read readiness (fd=7)...
[66969318867] [[32mINFO [0m] [http] [CPU3] http: wait_fd_ready complete for http read revents=0x0001
[67142867847] [[32mINFO [0m] [http] [CPU3] http: background task: read 505 bytes from TLS
[67146635193] [[32mINFO [0m] [http] [CPU3] http: background task: forwarded 505 bytes to foreground
[67158055239] [[32mINFO [0m] [http] [CPU3] http: received 505 bytes from background TLS thread
[67162237098] [[32mINFO [0m] [http] [CPU3] http: headers complete body_start=276
type=Utf8 len=21
value=https://www.iana.org/
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[67241779110] [[32mINFO [0m] [http] [CPU3] http: waiting for http read readiness (fd=7)...
cat /https/iana.org
[?25l
```
</details>
