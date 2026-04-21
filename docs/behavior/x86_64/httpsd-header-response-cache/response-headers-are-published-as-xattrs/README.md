# ❌ Scenario: Response headers are published as xattrs

> Last run: 2026-04-21 14:36:54

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8303ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 2103ms | - [📜](./02/serial.log) - |
| 3 | And I type "ping -c 1 example.com" on the serial console | ✅ | 2126ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "1 packets transmitted, 1 received" | ✅ | 405ms | - [📜](./04/serial.log) - |
| 5 | And I type "attr_list /https/example.com" on the serial console | ✅ | 2485ms | - [📜](./05/serial.log) - |
| 6 | Then the serial output should contain "user.http.status_code" within 30s | ❌ | 31087ms | - [📜](./06/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[24373634934] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26308483047] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26312181654] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26380491192] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26955354723] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27039572505] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28611905322] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[28648563339] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[28779198954] [[32mINFO [0m] [iso9660d] [CPU1] iso9660d: no ISO9660 filesystem found yet �� retrying
ping -c 1 example.[37030881030] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
com
[?25lPING example.com (172.66.147.243) 56 bytes of data
64 bytes from 172.66.147.243: icmp_seq=1 time=1427ms

--- example.com ping statistics ---
1 packets transmitted, 1 received, 0% packet loss
rtt min/avg/max = 1427/1427/1427 ms
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_list /https/example.com

```
</details>
