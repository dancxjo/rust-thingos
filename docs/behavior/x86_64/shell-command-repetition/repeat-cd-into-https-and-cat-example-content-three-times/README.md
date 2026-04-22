# ✅ Scenario: Repeat cd into https and cat example content three times

> Last run: 2026-04-21 17:41:05

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8220ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 2103ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https example.com /https/ex" on the serial console | ✅ | 2894ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ✅ | 0ms | - - - |
| 5 | And I type "cat /https/ex/@index" on the serial console | ✅ | 2075ms | - [📜](./05/serial.log) - |
| 6 | And I type "cat /https/ex/@index" on the serial console | ✅ | 2076ms | - [📜](./06/serial.log) - |
| 7 | And I type "cat /https/ex/@index" on the serial console | ✅ | 2075ms | - [📜](./07/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[24464881353] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26248409088] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26251297380] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26305610892] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26835196410] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[26908706550] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[1;32m
        .-.
       /   \        [1;36mTHING-OS[1;32m
      |     |       [0;36m"People, places, things."[1;32m
       \   /        
        `-'        
       /   \        v0.1  ��  
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28381888392] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[28423561881] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[28435769307] [[32mINFO [0m] [iso9660d] [CPU3] iso9660d: no ISO9660 filesystem found yet — retrying
[30257636376] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
mount -t https example.com /https/ex
[39940809447] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='mount'
[?25l[40194103752] [[32mINFO [0m] [httpsd] [CPU2] httpsd: entering main RPC loop for /https/ex
[40480267014] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Lookup payload_len=4
[40482005982] [[32mINFO [0m] [httpsd] [CPU2] httpsd: dispatch_lookup path=''
[40484109501] [[32mINFO [0m] [httpsd] [CPU2] httpsd: resolve_path path=''
[40500884820] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Close payload_len=8
mounted type=https device=example.com target=/https/ex
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/ex/@index
[46776692127] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[?25l[46827611094] [[32mINFO [0m] [cat] [CPU1] cat: opening '/https/ex/@index'
[46838696355] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Lookup payload_len=10
[46840340448] [[32mINFO [0m] [httpsd] [CPU2] httpsd: dispatch_lookup path='@index'
[46841775552] [[32mINFO [0m] [httpsd] [CPU2] httpsd: resolve_path path='@index'
[46847536692] [[32mINFO [0m] [cat] [CPU1] cat: opened '/https/ex/@index' fd=6
[46855430325] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Poll payload_len=12
[46866002436] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[46880425053] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/ex/@index
[53614470228] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[?25l[53665541754] [[32mINFO [0m] [cat] [CPU2] cat: opening '/https/ex/@index'
[53670607947] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=10
[53672061201] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path='@index'
[53673380607] [[32mINFO [0m] [httpsd] [CPU3] httpsd: resolve_path path='@index'
[53682108216] [[32mINFO [0m] [cat] [CPU2] cat: opened '/https/ex/@index' fd=7
[53686470684] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Poll payload_len=12
[53703075360] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[53716164348] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/ex/@index

```
</details>
