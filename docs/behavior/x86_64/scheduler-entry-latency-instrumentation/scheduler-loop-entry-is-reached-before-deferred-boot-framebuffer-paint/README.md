# ❌ Scenario: Scheduler loop entry is reached before deferred boot framebuffer paint

> Last run: 2026-04-21 17:41:05

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 0ms | - - - |
| 2 | When I wait for the system to boot | ✅ | 8304ms | - [📜](./02/serial.log) - |
| 3 | Then the serial output should contain "scheduler-entry total elapsed_ticks=" | ❌ | 301038ms | - [📜](./03/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[24409079739] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26305432791] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26308597788] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26364109992] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26891238363] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[26967600957] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28496766375] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[28540842891] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[28558563660] [[32mINFO [0m] [iso9660d] [CPU3] iso9660d: no ISO9660 filesystem found yet — retrying
[30504323883] [
```
</details>
