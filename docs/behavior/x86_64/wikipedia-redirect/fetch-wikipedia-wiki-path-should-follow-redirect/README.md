# ❌ Scenario: Fetch Wikipedia /wiki path should follow redirect

> Last run: 2026-04-22 21:33:14

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8619ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 151ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https en.wikipedia.org /https/wp" on the serial console | ✅ | 886ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ✅ | 0ms | - [📜](./04/serial.log) - |
| 5 | And I type "cat /https/wp/wiki/@index" on the serial console | ✅ | 5504ms | - [📜](./05/serial.log) - |
| 6 | Then the command output should contain "Main Page" | ❌ | 6107ms | - [📜](./06/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26669520042] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26696575587] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26729830842] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26762771244] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26782953681] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26785536096] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26821344924] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26841373581] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26890782963] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26891852229] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26976743376] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[27045268668] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[27066797109] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[27139933557] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[27240379188] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[27267029229] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[27291025179] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[27406951176] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[27465481923] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[27623947395] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27920410056] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1007979093 elapsed_us=503989
[27921235188] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27999596163] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[28067947182] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[28068969357] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[28158416319] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[28160767899] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[28170886524] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=19701
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28427935932] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mounting /hosts early...
[28457095095] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned early /hosts mount (PID=9)
[28463825049] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[28611617529] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: starting (mount_point=/hosts)
[28629047370] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: mounted at /hosts
[28636148343] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: advertising hostname thingos.local
[28638046899] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: serving cache immediately; mDNS socket will attach when netd is ready
[28704269649] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
mount -t mounted type=mdns device=none target=/hosts
https en.wikipedia.org /https/wp
[30948176160] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='mount'
[31174348656] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[31220935581] [[32mINFO [0m] [httpsd] [CPU3] HTTPSD_READY
[31222221822] [[32mINFO [0m] [httpsd] [CPU3] httpsd: entering main RPC loop for /https/wp
[?25l[31323914160] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[31324879872] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[31350187935] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
[31376008257] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[31376902590] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[31377696504] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
[31517047815] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=4
[31518084015] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path=''
[31518836019] [[32mINFO [0m] [httpsd] [CPU3] httpsd: resolve_path path=''
[31541348421] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Close payload_len=8
mounted type=https device=en.wikipedia.org target=/https/wp
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/wp/wiki[32576707449] [[32mINFO [0m] [netd] [CPU1] NETD: Starting network service...
[32608506711] [[32mINFO [0m] [netd] [CPU1] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
/@index
[32993531868] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[?25l[33051411393] [[32mINFO [0m] [cat] [CPU2] cat: opening '/https/wp/wiki/@index'
[33059037429] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=15
[33060334164] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path='wiki/@index'
[33061461675] [[32mINFO [0m] [httpsd] [CPU3] httpsd: resolve_path path='wiki/@index'
[33066563376] [[32mINFO [0m] [httpsd] [CPU3] httpsd: lookup 'wiki/@index' -> handle 2
[33078893859] [[32mINFO [0m] [cat] [CPU2] cat: opened '/https/wp/wiki/@index' fd=8
[33083813994] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Poll payload_len=12
[33097126557] [[32mINFO [0m] [netd] [CPU1] NETD: Network ready
[33102285018] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Read payload_len=20
[33105062397] [[32mINFO [0m] [httpsd] [CPU3] httpsd: ensure_upstream handle=2 MISS https://en.wikipedia.org/wiki - opening network stream
[33120720864] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=0/120)
[33124165866] [[32mINFO [0m] [http] [CPU3] http: connect host=en.wikipedia.org port=443
[33125026011] [[32mINFO [0m] [http] [CPU3] http: opening /net/tcp/new
[49639014480] [[
```
</details>
