# ❌ Scenario: Fetch Example Domain page

> Last run: 2026-04-21 15:39:05

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 18514ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 2207ms | - [📜](./02/serial.log) - |
| 3 | And I type "ping -c 1 example.com" on the serial console | ✅ | 2133ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "1 packets transmitted, 1 received" | ❌ | 301078ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[55261424856] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[59233910241] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[59242180041] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[59354490063] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[60399351078] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[60555504207] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[64331991867] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[64460475156] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[64510718712] [[32mINFO [0m] [iso9660d] [CPU3] iso9660d: no ISO9660 filesystem found yet �� retrying
ping[68649653919] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
 -c 1 example.com
[71586488193] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ping'
[?25lqemu-system-x86_64: terminating on signal 15 from pid 269903 ()

```
</details>
