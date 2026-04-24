# ❌ Scenario: Fetch Wikipedia /wiki path should follow redirect

> Last run: 2026-04-23 21:03:05

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8316ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 253ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https en.wikipedia.org /https/wp" on the serial console | ✅ | 1864ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ✅ | 0ms | - - - |
| 5 | And I type "cat /https/wp/wiki/@index" on the serial console | ✅ | 5563ms | - [📜](./05/serial.log) - |
| 6 | Then the command output should contain "Main Page" | ❌ | 6094ms | - [📜](./06/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[25877786022] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[25903098672] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[25933111743] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[25964649414] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[25983732225] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[25986169341] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26020429908] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26039911260] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26085248442] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26086187490] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26166213975] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26233351650] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26254388160] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26325172632] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26423203785] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26451013413] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26475848685] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[26576359458] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[26624982813] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[26757221601] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27001230894] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=895151466 elapsed_us=447575
[27001852251] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27071063448] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27137336754] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27138175746] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27225334521] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[27234166245] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27236118195] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[27237006159] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27247557051] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=43164
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[27466680252] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[27468503535] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[27502471293] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet ��� retrying
mount -t https en.wikipedia.org /h[29882126637] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=9 PID=9
[29931275121] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=10 PID=10
[30075965568] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=12 PID=12
[30079246956] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[30080105715] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[30134600496] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
ttps/[30161593737] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[30162425007] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[30163232979] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
wp
[30280891740] [[32mINFO [0m] [sh] [CPU1] sh: spawning job cmd='mount'
[30286778742] [[32mINFO [0m] [sh] [CPU1] sh: spawning '/bin/mount' with argv=["/bin/mount", "-t", "https", "en.wikipedia.org", "/https/wp"]
[30318543816] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/bin/mount' TID=13 PID=13
[30323776956] [[32mINFO [0m] [sh] [CPU1] sh: cleaning up 0 pipes
[?25l[30437179575] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[30510801915] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/bin/httpsd' TID=14 PID=14
[30573618438] [[32mINFO [0m] [httpsd] [CPU2] HTTPSD_READY
[30574862703] [[32mINFO [0m] [httpsd] [CPU2] httpsd: entering main RPC loop for /https/wp
[33093555891] [[32mINFO [0m] [netd] [CPU1] NETD: Starting network service...
[33124143723] [[32mINFO [0m] [netd] [CPU1] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[33305683521] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/netd' TID=16 PID=16
[33308704209] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=16)
[33737459910] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Lookup payload_len=4
[33738460239] [[32mINFO [0m] [httpsd] [CPU2] httpsd: dispatch_lookup path=''
[33739133538] [[32mINFO [0m] [httpsd] [CPU2] httpsd: resolve_path path=''
[33783142536] [[32mINFO [0m] [netd] [CPU1] NETD: Network ready
[33984201273] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Close payload_len=8
mounted type=https device=en.wikipedia.org target=/https/wp
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[34113464121] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: netd activation probe succeeded for PID 16 (/net/icmp/new is responsive)
cat /https/wp/wiki/@index
[35629962786] [[32mINFO [0m] [sh] [CPU1] sh: spawning job cmd='cat'
[35632752738] [[32mINFO [0m] [sh] [CPU1] sh: spawning '/bin/cat' with argv=["/bin/cat", "/https/wp/wiki/@index"]
[35656856862] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/bin/cat' TID=18 PID=18
[35682503373] [[32mINFO [0m] [cat] [CPU3] cat: opening '/https/wp/wiki/@index'
[35690716611] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Lookup payload_len=15
[35691582630] [[32mINFO [0m] [httpsd] [CPU2] httpsd: dispatch_lookup path='wiki/@index'
[35692223556] [[32mINFO [0m] [httpsd] [CPU2] httpsd: resolve_path path='wiki/@index'
[35695579755] [[32mINFO [0m] [httpsd] [CPU2] httpsd: lookup 'wiki/@index' -> handle 2
[35701416234] [[32mINFO [0m] [cat] [CPU3] cat: opened '/https/wp/wiki/@index' fd=8
[35708959836] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Poll payload_len=12
[35733578562] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Read payload_len=20
[35841869778] [[32mINFO [0m] [httpsd] [CPU1] httpsd: worker started for handle=2 url=https://en.wikipedia.org/wiki
[35843835258] [[32mINFO [0m] [httpsd] [CPU1] httpsd: ensure_upstream handle=2 MISS https://en.wikipedia.org/wiki - opening network stream
[35861591799] [[32mINFO [0m] [http] [CPU1] http: waiting for header data from port (attempt=0/120)
[35864192694] [[32mINFO [0m] [http] [CPU2] http: connect host=en.wikipedia.org port=443
[35864950308] [[32mINFO [0m] [http] [CPU2] http: opening /net/tcp/new
[44970654795] [[32mINFO [0m] [http] [CPU2] http: allocated tcp socket id=2
[44971593876] [[32mINFO [0m] [http] [CPU2] http: opening ctl path /net/tcp/2/ctl
[44987898945] [[32mINFO [0m] [http] [CPU2] http: opening data path /net/tcp/2/data
[45004471875] [[32mINFO [0m] [http] [CPU2] http: issuing connect command: connect en.wikipedia.org 443
[48353337714] [[32mINFO [0m] [http] [CPU2] http: waiting for socket readiness...
[48354658143] [[32mINFO [0m] [http] [CPU2] http: waiting for http connect socket readiness (fd=8)...
[50169990849] [[32mINFO [0m] [sh] [CPU1] sh: cleaning up 0 pipes
[?25l[51113662413] [[32mINFO [0m] [http] [CPU2] http: wait_fd_ready complete for http connect socket revents=0x0004
[51115304823] [[32mINFO [0m] [http] [CPU2] http: background task: TCP connected to en.wikipedia.org
[51116818467] [[32mINFO [0m] [http] [CPU2] http: background task: starting TLS handshake with en.wikipedia.org
[51372415875] [[32mINFO [0m] [http] [CPU2] http: waiting for http read readiness (fd=8)...
[51833748582] [[32mINFO [0m] [http] [CPU2] http: wait_fd_ready complete for http read revents=0x0001
[52269281394] [[31;1mERROR[0m] [kernel::vfs::provider] [CPU3] VFS RPC: tid=18 req_id=5 op=Read TIMEOUT
cat: error reading /https/wp/wiki/@index
[52280087772] [[32mINFO [0m] [httpsd] [CPU2] httpsd: RPC op=Close payload_len=8
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[52615169475] [[32mINFO [0m] [http] [CPU2] http: background task: TLS handshake complete for en.wikipedia.org
[52616121855] [[32mINFO [0m] [http] [CPU2] http: background task: sending 180 byte request
[52617609825] [[32mINFO [0m] [http] [CPU2] http: background task: wrote 180 bytes (total=180)
[52618316916] [[32mINFO [0m] [http] [CPU2] http: background task: flushing TLS stream
[52681050147] [[32mINFO [0m] [http] [CPU2] http: background task: entering read loop
[52697968125] [[32mINFO [0m] [http] [CPU2] http: waiting for http read readiness (fd=8)...
[55374997821] [[32mINFO [0m] [http] [CPU2] http: wait_fd_ready complete for http read revents=0x0001
[55555116111] [[32mINFO [0m] [http] [CPU2] http: background task: read 1632 bytes from TLS
[55557058425] [[32mINFO [0m] [http] [CPU2] http: background task: forwarded 1632 bytes to foreground
[68749677513] [[32mINFO [0m] [htt
```
</details>
