# ❌ Scenario: Repeat cd into https and cat example content three times

> Last run: 2026-04-22 21:33:14

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8518ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 252ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https example.com /https/ex" on the serial console | ✅ | 1266ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ✅ | 0ms | - [📜](./04/serial.log) - |
| 5 | And I type "cat /https/ex/@index" on the serial console | ✅ | 389ms | - [📜](./05/serial.log) - |
| 6 | And I type "cat /https/ex/@index" on the serial console | ✅ | 391ms | - [📜](./06/serial.log) - |
| 7 | And I type "cat /https/ex/@index" on the serial console | ✅ | 10399ms | - [📜](./07/serial.log) - |
| 8 | Then the command output should contain "<html" | ❌ | 6015ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26311155849] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26357472405] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26393789961] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26428098411] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26448788157] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26451515475] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26487991629] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26508666822] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26557527051] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26558602323] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26643313620] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26715308796] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26745790104] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26820628560] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26921200977] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26953903416] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26979574281] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[27094761837] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[27145277676] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[27302166276] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27598050216] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1018653504 elapsed_us=509326
[27598910691] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27711811974] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27781161903] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27782244336] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27869559465] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27871809702] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27880066071] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=19899
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28074985059] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mounting /hosts early...
[28102844253] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned early /hosts mount (PID=9)
[28105401819] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[28129238775] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[28233724035] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: starting (mount_point=/hosts)
[28250647920] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: mounted at /hosts
[28257703551] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: advertising hostname thingos.local
[28259736054] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: serving cache immediately; mDNS socket will attach when netd is ready
mounted type=mdns device=none target=/hosts
mount -t https example.com /https/e[30738931740] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[30757114575] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[30784499559] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
[30817648455] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[30818497842] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[30819333402] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
[30843721260] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
x
[31308311892] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='mount'
[?25l[32200863849] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[32233911138] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[32425634043] [[32mINFO [0m] [httpsd] [CPU3] HTTPSD_READY
[32426817555] [[32mINFO [0m] [httpsd] [CPU3] httpsd: entering main RPC loop for /https/ex
[32732356503] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=4
[32733449265] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path=''
[32734274793] [[32mINFO [0m] [httpsd] [CPU3] httpsd: resolve_path path=''
[32746193172] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Close payload_len=8
mounted type=https device=example.com target=/https/ex
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hc[32933482824] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
at /https/ex/@index
[33987171768] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[?25l[34045091091] [[32mINFO [0m] [cat] [CPU2] cat: opening '/https/ex/@index'
[34050807648] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=10
[34051933344] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path='@index'
[34052694126] [[32mINFO [0m] [httpsd] [CPU3] httpsd: resolve_path path='@index'
[34063320225] [[32mINFO [0m] [cat] [CPU2] cat: opened '/https/ex/@index' fd=8
[34074174453] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Poll payload_len=12
[34089664653] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[34106625102] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/ex/@index
[35278905321] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='cat'
[35345619243] [[32mINFO [0m] [cat] [CPU3] cat: opening '/https/ex/@index'
[35355115224] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Lookup payload_len=10
[35356065228] [[32mINFO [0m] [httpsd] [CPU3] httpsd: dispatch_lookup path='@index'
[35356828320] [[32mINFO [0m] [httpsd] [CPU3] httpsd: resolve_path path='@index'
[35368647402] [[32mINFO [0m] [cat] [CPU3] cat: opened '/https/ex/@index' fd=9
[35381226672] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Poll payload_len=12
[35400280146] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Read payload_len=20
cat: error reading /https/ex/@index
[35426357538] [[32mINFO [0m] [httpsd] [CPU3] httpsd: RPC op=Close payload_len=8
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/ex/@index
[36595226877] [[32mINFO [0m] [sh] [CPU
```
</details>
