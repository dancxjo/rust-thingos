# ❌ Scenario: Fetch Wikipedia /wiki path should follow redirect

> Last run: 2026-04-22 22:03:32

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8621ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 254ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https en.wikipedia.org /https/wp" on the serial console | ✅ | 2227ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ✅ | 0ms | - [📜](./04/serial.log) - |
| 5 | And I type "cat /https/wp/wiki/@index" on the serial console | ✅ | 4021ms | - [📜](./05/serial.log) - |
| 6 | Then the command output should contain "Main Page" | ❌ | 6081ms | - [📜](./06/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26703008178] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26730702273] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26765371842] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26798934624] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26818853523] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26821567146] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26859164541] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26879886132] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26932104771] [[32mINFO [0m] [ker[27298301679] [[32mINFO [0m] [kenel::srnel::boot_proched] [CPUgress] [0]CP SchU0] booteduler initialized
[26933202252] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[27021712707] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[27095935383] [[32mINFO_progr [0m]ess:  [mikernel::boot_plestrogress] [CPone=U0] boot_progress: mil"SMP Bring-up"
estone="PCI Bus Scanned"
[27120744717] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[27196935711] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[27331662798] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[27356572254] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[27480091650] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[27529615410] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[27685038051] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27987827274] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1034162679 elapsed_us=517081
[27988697649] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[28066259430] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[28134212370] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[28135223292] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[28218509451] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[28220699628] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[28231719813] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=20394
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28427419812] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mounting /hosts early...
[28454331774] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned early /hosts mount (PID=9)
[28458391500] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet ��� retrying
[28492404303] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[28599920448] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: starting (mount_point=/hosts)
[28617066159] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: mounted at /hosts
[28623972399] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: advertising hostname thingos.local
[28626975630] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: serving cache immediately; mDNS socket will attach when netd is ready
momounted type=mdns device=none target=/hosts
unt -t https en.wikipedia.org /ht[31046213550] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[31067453241] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Probing primary port (0x1F0)...
[31095455424] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Probing secondary port (0x170)...
[31123921554] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[31124801202] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Entering RPC service loop
[31125578385] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: No active devices to service
[31299155547] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[32682340977] [[32mINFO [0m] [netd] [CPU1] NETD: Starting network service...
[32713365795] [[32mINFO [0m] [netd] [CPU1] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[33179363448] [[32mINFO [0m] [netd] [CPU1] NETD: Network ready
tps/wp
[35701183881] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='mount'
[?25l[35976828360] [[32mINFO [0m] [httpsd] [CPU3] HTTPSD_READY
[35978007120] [[32mINFO [0m] [httpsd] [CPU3] httpsd: entering main RPC loop for /https/wp
[36278130999] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=4
[36279302961] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path=''
[36280194555] [[32mINFO [0m] [httpsd] [CPU3] httpsd: resolve_path path=''
[36308802552] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Close payload_len=8
mounted type=https device=en.wikipedia.org target=/https/wp
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/wp/wiki/@index
[37774388454] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[?25l[37828217196] [[32mINFO [0m] [cat] [CPU2] cat: opening '/https/wp/wiki/@index'
[37837450035] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=15
[37838319255] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path='wiki/@index'
[37840660308] [[32mINFO [0m] [httpsd] [CPU3] httpsd: resolve_path path='wiki/@index'
[37844136495] [[32mINFO [0m] [httpsd] [CPU3] httpsd: lookup 'wiki/@index' -> handle 2
[37853673627] [[32mINFO [0m] [cat] [CPU2] cat: opened '/https/wp/wiki/@index' fd=8
[37859703321] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Poll payload_len=12
[37883893575] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Read payload_len=20
[37886108766] [[32mINFO [0m] [httpsd] [CPU3] httpsd: ensure_upstream handle=2 MISS https://en.wikipedia.org/wiki - opening network stream
[37907445642] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=0/120)
[37911236055] [[32mINFO [0m] [http] [CPU3] http: connect host=en.wikipedia.org port=443
[37912037229] [[32mINFO [0m] [http] [CPU3] http: opening /net/tcp/new
[49553302491] [[31;1mERROR
```
</details>
