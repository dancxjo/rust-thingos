# ✅ Scenario: Repeat cd into https and cat example content three times

> Last run: 2026-04-20 11:59:00

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 15529ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 2000ms | - - - |
| 3 | And I type "cd /https" on the serial console | ✅ | 1526ms | - [📜](./03/serial.log) - |
| 4 | And I type "cat www.example.com" on the serial console | ✅ | 2028ms | - [📜](./04/serial.log) - |
| 5 | And I type "cd /https" on the serial console | ✅ | 1515ms | - [📜](./05/serial.log) - |
| 6 | And I type "cat www.example.com" on the serial console | ✅ | 2033ms | - [📜](./06/serial.log) - |
| 7 | And I type "cd /https" on the serial console | ✅ | 1528ms | - [📜](./07/serial.log) - |
| 8 | And I type "cat www.example.com" on the serial console | ✅ | 2029ms | - [📜](./08/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[45393948303] [[32mINFO [0m] [kernel] [CPU0] thing-os kernel starting...
[45412306203] [[33mWARN [0m] [bran::mem] [CPU0] memory_map: Limine reports 230 entries; only first 64 fit in cache (rest dropped)
[45822386676] [[32mINFO [0m] [kernel::memory] [CPU0] Frame allocator initialized with 479141 free frames
[45875718174] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[49340638512] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[49346501028] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[49457423829] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[50069103051] [[32mINFO [0m] [sprout] [CPU2] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[50409320247] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcd /https
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/https[0m [1;96m>[0m [?25hcat www.example.com
[?25lcd /https
cat www.example.com
cd /https
cat www.example.com

```
</details>
