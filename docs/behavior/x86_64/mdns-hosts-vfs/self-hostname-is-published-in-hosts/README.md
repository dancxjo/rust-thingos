# ❌ Scenario: Self hostname is published in /hosts

> Last run: 2026-04-21 17:41:05

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8323ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 2001ms | - [📜](./02/serial.log) - |
| 3 | And I type "cat /etc/hostname" on the serial console | ✅ | 1920ms | - [📜](./03/serial.log) - |
| 4 | Then the serial output should contain "thingos" | ✅ | 0ms | - - - |
| 5 | When I type "ls /hosts" on the serial console | ✅ | 1511ms | - [📜](./05/serial.log) - |
| 6 | And I wait for 2 seconds | ✅ | 2001ms | - - - |
| 7 | Then the serial output should contain "thingos.local" | ❌ | 301045ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[24576881340] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26367402204] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26370368838] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26423965491] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26937838950] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27008866005] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28496657574] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[28538639217] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[28548675903] [[32mINFO [0m] [iso9660d] [CPU3] iso9660d: no ISO9660 filesystem found yet — retrying
[30607412154] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
cat /etc/hostname
[36735482652] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[?25l[36801943695] [[32mINFO [0m] [cat] [CPU1] cat: opening '/etc/hostname'
[36806378268] [[32mINFO [0m] [cat] [CPU1] cat: opened '/etc/hostname' fd=5
thingos
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hls /hosts

```
</details>
