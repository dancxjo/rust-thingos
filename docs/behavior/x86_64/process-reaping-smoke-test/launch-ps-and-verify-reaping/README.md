# ✅ Scenario: Launch ps and verify reaping

> Last run: 2026-04-21 17:41:05

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8215ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 2102ms | - [📜](./02/serial.log) - |
| 3 | And I type "ps" on the serial console | ✅ | 1154ms | - [📜](./03/serial.log) - |
| 4 | Then the serial output should contain "PID" | ✅ | 0ms | - - - |
| 5 | And the serial output should contain "sh" | ✅ | 0ms | - - - |
| 6 | And the serial output should contain "ps" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[24513299943] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26293176327] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26296014195] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26348703579] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26867897892] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[26937284946] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28376990433] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[28426009854] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[28439730693] [[32mINFO [0m] [iso9660d] [CPU3] iso9660d: no ISO9660 filesystem found yet — retrying
[30245340741] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
ps
[34187453652] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ps'
  PID  PPID STAT COMMAND
[?25l    5     0 S    
    6     5 S    /bin/sh
    7     5 S    /bin/cambium
    8     5 R    /bin/netd
    9     5 S    /bin/iso9660d
   11     7 R    /drivers/virtio_netd
   12     7 S    /drivers/ahci_disk
   13   
```
</details>
