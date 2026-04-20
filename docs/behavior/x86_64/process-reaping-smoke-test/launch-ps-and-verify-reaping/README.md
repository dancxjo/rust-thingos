# ✅ Scenario: Launch ps and verify reaping

> Last run: 2026-04-20 11:19:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 17378ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 2107ms | - [📜](./02/serial.log) - |
| 3 | And I type "ps" on the serial console | ✅ | 1160ms | - [📜](./03/serial.log) - |
| 4 | Then the serial output should contain "PID" | ✅ | 6ms | - - - |
| 5 | And the serial output should contain "sh" | ✅ | 3ms | - - - |
| 6 | And the serial output should contain "ps" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[50149910481] [[32mINFO [0m] [kernel] [CPU0] thing-os kernel starting...
[50180385189] [[33mWARN [0m] [bran::mem] [CPU0] memory_map: Limine reports 230 entries; only first 64 fit in cache (rest dropped)
[50760361674] [[32mINFO [0m] [kernel::memory] [CPU0] Frame allocator initialized with 479141 free frames
[50831566830] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[54841962219] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[54856007085] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[54968825010] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[55898957235] [[32mINFO [0m] [sprout] [CPU2] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[56333852157] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[1;32m
        .-.
       /   \        [1;36mTHING-OS[1;32m
      |     |       [0;36m"People, places, things."[1;32m
       \   /        
        `-'        
       /   \        v0.1  •  ACT IV
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hps
  PID  PPID STAT COMMAND
[?25l    5     0 S    
    6     5 S    /bin/sh
    7     5 S    /bin/cambium
    8     5 S    /bin/netd
    9     5 S    /bin/httpsd
   11     7 R    /drivers/virtio_netd
   12     7 S    /drivers/ahci_disk
   
```
</details>
