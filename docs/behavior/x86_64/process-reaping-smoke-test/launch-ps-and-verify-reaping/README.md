# ❌ Scenario: Launch ps and verify reaping

> Last run: 2026-04-20 11:07:52

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 16468ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 995ms | - [📜](./02/serial.log) - |
| 3 | And I type "ps" on the serial console | ✅ | 1007ms | - [📜](./03/serial.log) - |
| 4 | Then the serial output should contain "PID" | ❌ | 121073ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[=3h[2Jt0;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[2J[01;01H[01;01H[47261698671] [[32mINFO ] [kernel] [CPU0] thing-os kernel starting...
[47285399139] [[33mWARN [0m] [bran::mem] [CPU0] memory_map: Limine reports 230 entries; only first 64 fit in cache (rest dropped)
[47790451863] [[32mINFO [0m] [kernel::memory] [CPU0] Frame allocator initialized with 479141 free frames
[47856220863] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[52197442374] [INFO ] [kernel] [CPU0] Initializing SIMD...
[52203082041] [[32mINFO ] [kernel] [CPU0] Initializing tasking...
[52325832999] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[52989630045] [INFO ] [sprout] [CPU2] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[53375017938] [[32mINFO ] [kernel] [CPU0] Entering scheduler loop.

        .-.
       /   \        THING-OS
      |     |       "People, places, things."
       \   /        
        `-'        
       /   \        v0.1  •  ACT IV
      |     |       2026-04-16
       \   /
        `-'

--------------------------------------------------------------[0m
 sprout has taken root. the system is awake.

  try:
    ls /bin       browse available shoots
    ps            observe living processes
    cat /version  inspect the genome

--------------------------------------------------------------
THING-
```
</details>
