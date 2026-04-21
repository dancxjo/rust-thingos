# ❌ Scenario: 3xx redirects surface as symlinks after first fetch

> Last run: 2026-04-21 14:40:41

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 9432ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 2103ms | - [📜](./02/serial.log) - |
| 3 | And I type "ping -c 1 example.org" on the serial console | ✅ | 2125ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "1 packets transmitted, 1 received" | ✅ | 405ms | - [📜](./04/serial.log) - |
| 5 | And I type "attr_get /https/example.org user.http.location" on the serial console | ✅ | 3406ms | - [📜](./05/serial.log) - |
| 6 | Then the serial output should contain "example.com" within 30s | ❌ | 31080ms | - [📜](./06/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[27738260781] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[30026325549] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[30029914398] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[30089541405] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[30648784254] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[30726362898] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[1;32m
        .-.
       /   \        [1;36mTHING-OS[1;32m
      |     |       [0;36m"People, places, things."[1;32m
       \   /        
        `-'        
       /   \        v0.1  ���  
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[32411481333] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[32461281369] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[32651373348] [[32mINFO [0m] [iso9660d] [CPU1] iso9660d: no ISO9660 filesystem found yet ��� retrying
ping -c 1 example.org
[41496849471] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
[?25lPING example.org (104.20.26.136) 56 bytes of data
64 bytes from 104.20.26.136: icmp_seq=1 time=1452ms

--- example.org ping statistics ---
1 packets transmitted, 1 received, 0% packet loss
rtt min/avg/max = 1452/1452/1452 ms
a[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_get /https/example.org user.http.location
attr_get: failed: EIO
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;91mERR[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hqemu-system-x86_64: terminating on signal 15 from pid 159988 ()

```
</details>
