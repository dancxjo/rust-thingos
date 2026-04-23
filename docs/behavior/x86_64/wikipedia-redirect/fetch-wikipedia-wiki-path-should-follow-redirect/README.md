# ❌ Scenario: Fetch Wikipedia /wiki path should follow redirect

> Last run: 2026-04-22 21:21:24

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8709ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 252ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https en.wikipedia.org /https/wp" on the serial console | ✅ | 1354ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ✅ | 0ms | - [📜](./04/serial.log) - |
| 5 | And I type "cat /https/wp/wiki/@index" on the serial console | ✅ | 6126ms | - [📜](./05/serial.log) - |
| 6 | Then the serial output should contain "Main Page" | ❌ | 6040ms | - [📜](./06/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[27038242704] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[27068321016] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[27104869308] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[27138967944] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[27159990759] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[27162843114] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[27199798956] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[27220986969] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[27272416314] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[27273583887] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[27362491827] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[27436011207] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[27458689533] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[27543382218] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[27658702104] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[27692071440] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[27716066598] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[27834331206] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[27884360163] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[28052140908] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[28351132590] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1056201696 elapsed_us=528100
[28352086521] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[28430415453] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[28505250774] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[28506269583] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[28598036214] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[28600980243] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[28609221663] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=43362
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28866778941] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mounting /hosts early...
[28896642753] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned early /hosts mount (PID=9)
[28908116919] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[28976489355] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[29086866237] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: starting (mount_point=/hosts)
[29105526879] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: mounted at /hosts
[29112667650] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: advertising hostname thingos.local
[29114730051] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: serving cache immediately; mDNS socket will attach when netd is ready
mount -t mounted type=mdns device=none target=/hosts
https en.wikipedia.org /https/wp
[31584630099] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='mount'
[31887083514] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[31888352496] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[31916700849] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
[31951556835] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[31952436912] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[31953328902] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
[31983677682] [[32mINFO [0m] [httpsd] [CPU2] HTTPSD_READY
[?25l[31985668044] [[32mINFO [0m] [httpsd] [CPU2] httpsd: entering main RPC loop for /https/wp
[32132028093] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[33633459024] [[32mINFO [0m] [netd] [CPU1] NETD: Starting network service...
[33653597109] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Lookup payload_len=4
[33654646014] [[32mINFO [0m] [httpsd] [CPU2] httpsd: dispatch_lookup path=''
[33655417917] [[32mINFO [0m] [httpsd] [CPU2] httpsd: resolve_path path=''
[33665968017] [[32mINFO [0m] [netd] [CPU1] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[33684207348] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Close payload_len=8
mounted type=https device=en.wikipedia.org target=/https/wp
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /htt[34217738544] [[32mINFO [0m] [netd] [CPU1] NETD: Network ready
ps/wp/wiki/@index
[35174761314] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[?25l[35243932185] [[32mINFO [0m] [cat] [CPU2] cat: opening '/https/wp/wiki/@index'
[35253276564] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Lookup payload_len=15
[35254749189] [[32mINFO [0m] [httpsd] [CPU1] httpsd: dispatch_lookup path='wiki/@index'
[35255961015] [[32mINFO [0m] [httpsd] [CPU1] httpsd: resolve_path path='wiki/@index'
[35261447199] [[32mINFO [0m] [httpsd] [CPU1] httpsd: lookup 'wiki/@index' -> handle 2
[35286269667] [[32mINFO [0m] [cat] [CPU2] cat: opened '/https/wp/wiki/@index' fd=8
[35292976851] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Poll payload_len=12
[35302782702] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Read payload_len=20
[35306413626] [[32mINFO [0m] [httpsd] [CPU1] httpsd: ensure_upstream handle=2 MISS https://en.wikipedia.org/wiki - opening network stream
[35333150259] [[32mINFO [0m] [http] [CPU1] http: waiting for header data from port (attempt=0/120)
[35335200648] [[32mINFO [0m] [http] [CPU3] http: connect host=en.wikipedia.org port=443
[35336076369] [[32mINFO [0m] [http] [CPU3] http: opening /net/tcp/new
[53845476762] [[3
```
</details>
