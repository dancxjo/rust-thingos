# ❌ Scenario: Fetch Wikipedia /wiki path should follow redirect

> Last run: 2026-04-22 22:12:03

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 9015ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 253ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https en.wikipedia.org /https/wp" on the serial console | ✅ | 941ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ✅ | 0ms | - [📜](./04/serial.log) - |
| 5 | And I type "cat /https/wp/wiki/@index" on the serial console | ✅ | 570ms | - [📜](./05/serial.log) - |
| 6 | Then the command output should contain "Main Page" | ❌ | 6083ms | - [📜](./06/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[28065634509] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[28094738496] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[28131429150] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[28167207024] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[28188731934] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[28191695334] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[28230903756] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[28253343063] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[28308351093] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[28309546584] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[28404736206] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[28481932545] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[28504900677] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[28580449260] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[28687367643] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[28727895834] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[28753613757] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[28881336099] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[28940123091] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[29112863274] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[29441609274] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1109365719 elapsed_us=554682
[29442509745] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[29532225459] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[29612201124] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[29613297879] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[29717863824] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[29720158479] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[29731767417] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=22176
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[30014250651] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mounting /hosts early...
[30052982619] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned early /hosts mount (PID=9)
[30063982674] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[30252009381] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[30271314942] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: starting (mount_point=/hosts)
[30300762987] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: mounted at /hosts
[30314390865] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: advertising hostname thingos.local
[30318021195] [[32mINFO [0m] [mesocarp] [CPU2] mesocarp: serving cache immediately; mDNS socket will attach when netd is ready
mount -t hmounted type=mdns device=none target=/hosts
ttps en.wikipedia.org /https/wp
[32568072075] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='mount'
[?25l[32891364693] [[32mINFO [0m] [httpsd] [CPU1] HTTPSD_READY
[32893443561] [[32mINFO [0m] [httpsd] [CPU1] httpsd: entering main RPC loop for /https/wp
[33100275021] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[33101334057] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Probing primary port (0x1F0)...
[33143000616] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Probing secondary port (0x170)...
[33173556075] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[33174468162] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: Entering RPC service loop
[33175387146] [[32mINFO [0m] [ata_disk] [CPU2] ATA_DISK: No active devices to service
[33243519837] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Lookup payload_len=4
[33245357805] [[32mINFO [0m] [httpsd] [CPU1] httpsd: dispatch_lookup path=''
[33246611673] [[32mINFO [0m] [httpsd] [CPU1] httpsd: resolve_path path=''
[33281630019] [[32mINFO [0m] [httpsd] [CPU1] httpsd: RPC op=Close payload_len=8
mounted type=https device=en.wikipedia.org target=/https/wp
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[33452829795] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
cat /https/wp/wiki/@index
[34780817808] [[32mINFO [0m] [sh] [CPU1] sh: spawning job cmd='cat'
[34873164282] [[32mINFO [0m] [cat] [CPU1] cat: opening '/https/wp/wiki/@index'
[34886980359] [[32mINFO [0m] [httpsd] [CPU0] httpsd: RPC op=Lookup payload_len=15
[34888405959] [[32mINFO [0m] [httpsd] [CPU0] httpsd: dispatch_lookup path='wiki/@index'
[34889619930] [[32mINFO [0m] [httpsd] [CPU0] httpsd: resolve_path path='wiki/@index'
[?25l[34895111196] [[32mINFO [0m] [httpsd] [CPU0] httpsd: lookup 'wiki/@index' -> handle 2
[34922179644] [[32mINFO [0m] [cat] [CPU1] cat: opened '/https/wp/wiki/@index' fd=8
[34930002888] [[32mINFO [0m] [httpsd] [CPU0] httpsd: RPC op=Poll payload_len=12
[34942840911] [[32mINFO [0m] [httpsd] [CPU0] httpsd: RPC op=Read payload_len=20
[34946077056] [[32mINFO [0m] [httpsd] [CPU0] httpsd: ensure_upstream handle=2 MISS https://en.wikipedia.org/wiki - opening network stream
[34975064916] [[32mINFO [0m] [http] [CPU0] http: waiting for header data from port (attempt=0/120)
[35129090568] [[32mINFO [0m] [netd] [CPU3] NETD: Starting network service...
[35129539995] [[32mINFO [0m] [http] [CPU2] http: connect host=en.wikipedia.org port=443
[35130370275] [[32mINFO [0m] [http] [CPU2] http: opening /net/tcp/new
[35135924175] [[32mINFO [0m] [http] [CPU0] http: received 61 bytes from background TLS thread
[35138686440] [[33mWARN [0m] [httpsd] [CPU0] httpsd: upstream open failed for handle=2 https://en.wikipedia.org/wiki: http connect failed: failed to open /net/tcp/new: ENOENT
cat: error reading /https/wp/wiki/@index
[35172742242] [[32mINFO [0m] [httpsd] [CPU0] httpsd: RPC op=Close payload_len=8
[35178951720] [[32mINFO [0m] [netd] [CPU3] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[35735840283] [[32mINFO [0m] [netd] [CPU3] NETD: Network ready
[52450500432] [[31;1
```
</details>
