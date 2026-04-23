# ❌ Scenario: Fetch Wikipedia /wiki path should follow redirect

> Last run: 2026-04-22 21:44:56

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8405ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 252ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https en.wikipedia.org /https/wp" on the serial console | ✅ | 1402ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ✅ | 0ms | - [📜](./04/serial.log) - |
| 5 | And I type "cat /https/wp/wiki/@index" on the serial console | ✅ | 4992ms | - [📜](./05/serial.log) - |
| 6 | Then the command output should contain "Main Page" | ❌ | 6081ms | - [📜](./06/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26261846523] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26288748156] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26323117194] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26356173129] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26376390084] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26379021075] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26415011502] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26435910270] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26483712882] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26484816138] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26568822192] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26641043055] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26663092203] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26735934060] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26836512582] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26862681087] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26886261369] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[26996978382] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[27047042187] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[27201867627] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27497382033] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=991208922 elapsed_us=495604
[27498253596] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27572874714] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27638819142] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27639795744] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27741916686] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27745482105] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27758650194] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=20031
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28017703428] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mounting /hosts early...
[28055772789] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[28072534908] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned early /hosts mount (PID=9)
[28203483363] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: starting (mount_point=/hosts)
[28219261851] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: mounted at /hosts
[28225966527] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: advertising hostname thingos.local
[28227912438] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: serving cache immediately; mDNS socket will attach when netd is ready
[28358349327] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
mount -tmounted type=mdns device=none target=/hosts
 https en.wikipedia.org /https/w[30684407523] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[30685460949] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[30720170778] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
p
[30730600890] [[32mINFO [0m] [sh] [CPU1] sh: spawning job cmd='mount'
[30747443067] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[30748296084] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[30749095179] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
[?25l[30841418388] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[32260996416] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[32291163069] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[32467966905] [[32mINFO [0m] [httpsd] [CPU3] HTTPSD_READY
[32469095637] [[32mINFO [0m] [httpsd] [CPU3] httpsd: entering main RPC loop for /https/wp
[32783306424] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=4
[32784961044] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path=''
[32785768686] [[32mINFO [0m] [httpsd] [CPU3] httpsd: resolve_path path=''
[32798075244] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Close payload_len=8
mounted type=https device=en.wikipedia.org target=/https/wp
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hca[33075837267] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
t /https/wp/wiki/@index
[34315944762] [[32mINFO [0m] [sh] [CPU1] sh: spawning job cmd='cat'
[?25l[34382516982] [[32mINFO [0m] [cat] [CPU2] cat: opening '/https/wp/wiki/@index'
[34394307288] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=15
[34395260559] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path='wiki/@index'
[34395947553] [[32mINFO [0m] [httpsd] [CPU3] httpsd: resolve_path path='wiki/@index'
[34399153338] [[32mINFO [0m] [httpsd] [CPU3] httpsd: lookup 'wiki/@index' -> handle 2
[34405228044] [[32mINFO [0m] [cat] [CPU2] cat: opened '/https/wp/wiki/@index' fd=8
[34410306546] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Poll payload_len=12
[34419964557] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Read payload_len=20
[34423307424] [[32mINFO [0m] [httpsd] [CPU3] httpsd: ensure_upstream handle=2 MISS https://en.wikipedia.org/wiki - opening network stream
[34440058818] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=0/120)
[34440870651] [[32mINFO [0m] [http] [CPU2] http: connect host=en.wikipedia.org port=443
[34441744854] [[32mINFO [0m] [http] [CPU2] http: opening /net/tcp/new
[49261379277] 
```
</details>
