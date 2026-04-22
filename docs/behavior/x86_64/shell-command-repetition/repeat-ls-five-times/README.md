# ✅ Scenario: Repeat ls five times

> Last run: 2026-04-21 17:41:05

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8220ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 2103ms | - [📜](./02/serial.log) - |
| 3 | And I type "ls" on the serial console | ✅ | 1153ms | - [📜](./03/serial.log) - |
| 4 | And I type "ls" on the serial console | ✅ | 1156ms | - [📜](./04/serial.log) - |
| 5 | And I type "ls" on the serial console | ✅ | 1155ms | - [📜](./05/serial.log) - |
| 6 | And I type "ls" on the serial console | ✅ | 1153ms | - [📜](./06/serial.log) - |
| 7 | And I type "ls" on the serial console | ✅ | 1153ms | - [📜](./07/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[24371018991] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26169486717] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26172392367] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26225235267] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26752025388] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[26822960043] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28258438065] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[28294487826] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[28309004097] [[32mINFO [0m] [iso9660d] [CPU3] iso9660d: no ISO9660 filesystem found yet — retrying
[30483332946] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
ls
[34204807593] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[?25l[34mbin[0m  [34mboot[0m  [34mdata[0m  [34mdev[0m  [34mdrivers[0m  [34metc[0m  [34mlib[0m  [34mmnt[0m  motd  [34mnet[0m  [34mproc[0m  [34mrun[0m  [34mservices[0m  [34msession[0m  [34mshare[0m  [34msys[0m  [34mtmp[0m  version  
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hls
[38000170857] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[?25l[34mbin[0m  [34mboot[0m  [34mdata[0m  [34mdev[0m  [34mdrivers[0m  [34metc[0m  [34mlib[0m  [34mmnt[0m  motd  [34mnet[0m  [34mproc[0m  [34mrun[0m  [34mservices[0m  [34msession[0m  [34mshare[0m  [34msys[0m  [34mtmp[0m  version  
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hls
[41825561745] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[?25l[34mbin[0m  [34mboot[0m  [34mdata[0m  [34mdev[0m  [34mdrivers[0m  [34metc[0m  [34mlib[0m  [34mmnt[0m  motd  [34mnet[0m  [34mproc[0m  [34mrun[0m  [34mservices[0m  [34msession[0m  [34mshare[0m  [34msys[0m  [34mtmp[0m  version  
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hls
[45613775724] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[?25l[34mbin[0m  [34mboot[0m  [34mdata[0m  [34mdev[0m  [34mdrivers[0m  [34metc[0m  [34mlib[0m  [34mmnt[0m  motd  [34mnet[0m  [34mproc[0m  [34mrun[0m  [34mservices[0m  [34msession[0m  [34mshare[0m  [34msys[0m  [34mtmp[0m  version  
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hls
[49415794395] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[?25l[34mbin[0m  [34mboot[0m  [34mdata[0m  [34mdev[0m  [34mdrivers[0m  [34metc[0m  [34mlib[0m  [34mmnt[0m  motd  [34mnet[0m  [34mproc[0m  [34mrun[0m  [34mservices[0m  [34msession[0m  [34mshare[0m  [34msys[0m  [34mtmp[0m  ve
```
</details>
