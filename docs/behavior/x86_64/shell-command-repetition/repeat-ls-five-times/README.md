# ✅ Scenario: Repeat ls five times

> Last run: 2026-04-20 11:28:55

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 15012ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 2000ms | - - - |
| 3 | And I type "ls" on the serial console | ✅ | 1153ms | - [📜](./03/serial.log) - |
| 4 | And I type "ls" on the serial console | ✅ | 1151ms | - [📜](./04/serial.log) - |
| 5 | And I type "ls" on the serial console | ✅ | 1142ms | - [📜](./05/serial.log) - |
| 6 | And I type "ls" on the serial console | ✅ | 1155ms | - [📜](./06/serial.log) - |
| 7 | And I type "ls" on the serial console | ✅ | 1157ms | - [📜](./07/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[43542311406] [[32mINFO [0m] [kernel] [CPU0] thing-os kernel starting...
[43560792231] [[33mWARN [0m] [bran::mem] [CPU0] memory_map: Limine reports 230 entries; only first 64 fit in cache (rest dropped)
[44018652414] [[32mINFO [0m] [kernel::memory] [CPU0] Frame allocator initialized with 479141 free frames
[44063761467] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[47576682912] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[47585489655] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[47688240237] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[48289363353] [[32mINFO [0m] [sprout] [CPU2] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[48664948794] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hls
[?25l[34mbin[0m  [34mboot[0m  [34mdata[0m  [34mdev[0m  [34mdrivers[0m  [34metc[0m  [34mhttps[0m  [34mlib[0m  [34mmnt[0m  motd  [34mnet[0m  [34mproc[0m  [34mrun[0m  [34mservices[0m  [34msession[0m  [34mshare[0m  [34msys[0m  [34mtmp[0m  version  
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hls
[?25l[34mbin[0m  [34mboot[0m  [34mdata[0m  [34mdev[0m  [34mdrivers[0m  [34metc[0m  [34mhttps[0m  [34mlib[0m  [34mmnt[0m  motd  [34mnet[0m  [34mproc[0m  [34mrun[0m  [34mservices[0m  [34msession[0m  [34mshare[0m  [34msys[0m  [34mtmp[0m  version  
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hls
[34mbin[0m  [34mboot[0m  [34mdata[0m  [34mdev[0m  [34mdrivers[0m  [34metc[0m  [34mhttps[0m  [?25l[34mlib[0m  [34mmnt[0m  motd  [34mnet[0m  [34mproc[0m  [34mrun[0m  [34mservices[0m  [34msession[0m  [34mshare[0m  [34msys[0m  [34mtmp[0m  version  
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hls
[?25l[34mbin[0m  [34mboot[0m  [34mdata[0m  [34mdev[0m  [34mdrivers[0m  [34metc[0m  [34mhttps[0m  [34mlib[0m  [34mmnt[0m  motd  [34mnet[0m  [34mproc[0m  [34mrun[0m  [34mservices[0m  [34msession[0m  [34mshare[0m  [34msys[0m  [34mtmp[0m  version  
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hls
[?25l[34mbin[0m  [34mboot[0m  [34mdata[0m  [34mdev[0m  [34mdrivers[0m  [34metc[0m  [34mhttps[0m  [34mlib[0m  [34mmnt[0m  motd  [34mnet[0m  [34mproc[0m  [34mrun[0m  [34mservices[0m  [34msession[0m  [34mshare[0m  [34msys[0m  [34mtmp[0m  version  

```
</details>
