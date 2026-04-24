# ❌ Scenario: Fetch Wikipedia /wiki path should follow redirect

> Last run: 2026-04-23 15:53:33

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 12273ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 355ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https en.wikipedia.org /https/wp" on the serial console | ✅ | 1253ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ✅ | 0ms | - - - |
| 5 | And I type "cat /https/wp/wiki/@index" on the serial console | ✅ | 2087ms | - [📜](./05/serial.log) - |
| 6 | Then the command output should contain "Main Page" | ❌ | 6018ms | - [📜](./06/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[38057493540] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[38099217552] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[38160299034] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[38231483763] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[38262190560] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[38265933420] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[38341819560] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[38375459859] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[38449202451] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[38450991843] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[38588658405] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[38695462080] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[38737068051] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[38831061027] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[38974530309] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[39044296434] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[39081739851] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[39255967476] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[39358758747] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[39588561639] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[40058095407] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1574006247 elapsed_us=787003
[40059994458] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[40189538202] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[40347717234] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[40349827386] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[40513628331] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[40524211695] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[40533552807] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[40551247110] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=52470
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[40843176561] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[40845887643] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[40947170880] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
mount -t https en.wikipedia.org /https/wp
[43580870685] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='mount'
[43593921525] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/mount' with argv=[[47, 98, 105, 110, 47, 109, 111, 117, 110, 116], [45, 116], [104, 116, 116, 112, 115], [101, 110, 46, 119, 105, 107, 105, 112, 101, 100, 105, 97, 46, 111, 114, 103], [47, 104, 116, 116, 112, 115, 47, 119, 112]]
[43658758143] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/mount' TID=9 PID=9
[44031315801] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/httpsd' TID=10 PID=10
[?25l[44521574460] [[32mINFO [0m] [httpsd] [CPU1] HTTPSD_READY
[44523618579] [[32mINFO [0m] [httpsd] [CPU1] httpsd: entering main RPC loop for /https/wp
[44753695371] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Lookup payload_len=4
[44756183274] [[32mINFO [0m] [httpsd] [CPU1] httpsd: dispatch_lookup path=''
[44758031769] [[32mINFO [0m] [httpsd] [CPU1] httpsd: resolve_path path=''
[45144254661] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Close payload_len=8
mounted type=https device=en.wikipedia.org target=/https/wp
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /[45710050815] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=12 PID=12
[45793231374] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=13 PID=13
[46045790274] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=15 PID=15
[46055721657] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[46057707993] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[46169076657] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[46209941382] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
https/wp/w[46296426099] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[46298728146] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[46300530507] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
iki/@index
[46859962083] [[32mINFO [0m] [sh] [CPU1] sh: spawning job cmd='cat'
[46873583196] [[32mINFO [0m] [sh] [CPU1] sh: spawning '/bin/cat' with argv=[[47, 98, 105, 110, 47, 99, 97, 116], [47, 104, 116, 116, 112, 115, 47, 119, 112, 47, 119, 105, 107, 105, 47, 64, 105, 110, 100, 101, 120]]
[46928036133] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/bin/cat' TID=16 PID=16
[?25l[47024036433] [[32mINFO [0m] [cat] [CPU1] cat: opening '/https/wp/wiki/@index'
[47048896587] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Lookup payload_len=15
[47050692909] [[32mINFO [0m] [httpsd] [CPU1] httpsd: dispatch_lookup path='wiki/@index'
[47053445010] [[32mINFO [0m] [httpsd] [CPU1] httpsd: resolve_path path='wiki/@index'
[47063778894] [[32mINFO [0m] [httpsd] [CPU1] httpsd: lookup 'wiki/@index' -> handle 2
[47090316537] [[32mINFO [0m] [cat] [CPU1] cat: opened '/https/wp/wiki/@index' fd=8
[47117832300] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Poll payload_len=12
[47162430579] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Read payload_len=20
[47167535448] [[32mINFO [0m] [httpsd] [CPU1] httpsd: ensure_upstream handle=2 MISS https://en.wikipedia.org/wiki - opening network stream
[47209202733] [[32mINFO [0m] [http] [CPU1] http: waiting for header data from port (attempt=0/120)
[47216497449] [[32mINFO [0m] [http] [CPU2] http: connect host=en.wikipedia.org port=443
[47218279713] [[32mINFO [0m] [http] [CPU2] http: opening /net/tcp/new
[47225906376] [[32mINFO [0m] [http] [CPU1] http: received 61 bytes from background TLS thread
[47230248615] [[33mWARN [0m] [httpsd] [CPU1] httpsd: upstream open failed for handle=2 https://en.wikipedia.org/wiki: http connect failed: failed to open /net/tcp/new: ENOENT
cat: error reading /https/wp/wiki/@index
[47278609983] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[51874261428] [[32mINFO [0m] [netd] [CPU3] NETD: Starting network service...
[51949310490] [[32mINFO [0m] [netd] [CPU3] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[52217862708] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/netd' TID=18 PID=18
[52222872372] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=18)
[53080719846] [[32mINFO [0m] [netd] [CPU3] NETD: Network ready
[70316391618] [[31;1mER
```
</details>
