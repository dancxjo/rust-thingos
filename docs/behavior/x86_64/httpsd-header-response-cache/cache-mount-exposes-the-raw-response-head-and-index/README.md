# ❌ Scenario: Cache mount exposes the raw response head and index

> Last run: 2026-04-21 14:36:54

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8405ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 2102ms | - [📜](./02/serial.log) - |
| 3 | And I type "ping -c 1 example.com" on the serial console | ✅ | 2125ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "1 packets transmitted, 1 received" | ✅ | 303ms | - [📜](./04/serial.log) - |
| 5 | And I type "cat /https/example.com > /dev/null" on the serial console | ❌ | 1052ms | - [📜](./05/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[24665052027] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26558249619] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26561505960] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26619065880] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[27232618347] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27312073866] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28870822167] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[28901711718] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[29030318823] [[32mINFO [0m] [iso9660d] [CPU1] iso9660d: no ISO9660 filesystem found yet ��� retrying
ping -c 1 example.com
[?25l[38156910759] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
PING example.com (172.66.147.243) 56 bytes of data
64 bytes from 172.66.147.243: icmp_seq=1 time=1358ms

--- example.com ping statistics ---
1 packets transmitted, 1 received, 0% packet loss
rtt min/avg/max = 1358/1358/1358 ms
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcqemu-system-x86_64: terminating on signal 15 from pid 144246 ()

```
</details>
