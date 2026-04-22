# ❌ Scenario: Cache mount exposes the raw response head and index

> Last run: 2026-04-21 17:41:05

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8318ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 2102ms | - [📜](./02/serial.log) - |
| 3 | And I type "ping -c 1 example.com" on the serial console | ✅ | 2128ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "1 packets transmitted, 1 received" | ✅ | 608ms | - [📜](./04/serial.log) - |
| 5 | And I type "mount -t https example.com /https/ex" on the serial console | ✅ | 2894ms | - [📜](./05/serial.log) - |
| 6 | And I wait for the serial output to contain "mounted type=https" | ❌ | 301021ms | - [📜](./06/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[24752051346] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26552445084] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26555448084] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26610295833] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[27150656511] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27227758140] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28712586804] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[28745191662] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[28767668061] [[32mINFO [0m] [iso9660d] [CPU3] iso9660d: no ISO9660 filesystem found yet — retrying
[30850428345] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
ping -c 1 example.com
[37735476273] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ping'
[?25lPING example.com (104.20.23.154) 56 bytes of data
64 bytes from 104.20.23.154: icmp_seq=1 time=1656ms

--- example.com ping statistics ---
1 packets transmitted, 1 received, 0% packet loss
rtt min/avg/max = 1656/1656/1656 ms
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hmount -t https example.com /https/ex
[49267803750] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='mount'
[?25l[49519907118] [[32mINFO [0m] [httpsd] [CPU3] httpsd: entering main RPC loop for /https/ex
[49795002741] [[32
```
</details>
