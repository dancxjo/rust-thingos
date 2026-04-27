# ✅ Scenario: Boot reaches an interactive serial shell before background bring-up continues

> Last run: 2026-04-27 08:36:49

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 10941ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 356ms | - [📜](./02/serial.log) - |
| 3 | And I type "echo shell-ready" on the serial console | ✅ | 430ms | - [📜](./03/serial.log) - |
| 4 | Then the command output should contain "shell-ready" | ✅ | 11ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[33517464519] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[33988163550] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[34017502662] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[34054798008] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[34099504956] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[34128120015] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[34130545383] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[34179939354] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[34181740527] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[34210583847] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[34230714672] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[34231886700] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[34298422191] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[34300343055] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[34434184785] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[34524684447] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[34558374543] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[34652689071] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[34764292365] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[34837826562] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[34889504826] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[35062278669] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[35124984378] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[35291162049] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[35613553965] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1272592827 elapsed_us=636296
[35614392462] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[35742275613] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[35749575444] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[35795302290] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[35854729350] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[35867342544] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[35870118636] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[36008417214] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[36015510399] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[36085095981] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[1;32m
        .-.
       /   \        [1;36mTHING-OS[1;32m
      |     |       [0;36m"People, places, things."[1;32m
       \   /        
        `-'        
       /   \        v0.1  ���  
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[36215232174] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[36217779114] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[36285715191] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[36316195806] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[36359186655] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[36396509358] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[36404018409] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[36406218684] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[36411297384] [[32mINFO [0m] [iso9660d] [CPU2] ISO9660D: Starting VFS provider
[36462308421] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=9)
[36472960128] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[36477672495] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[36492127287] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[36501349566] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[36504707151] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[36509906367] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[36513190230] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[36519574839] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[36525701718] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[36609879603] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[36610758624] [[32mINFO [0m] [bristle] [CPU3] bristle: published pid 9 to /run/bristle/pid
[36632189781] [[32mINFO [0m] [bristle] [CPU3] bristle: published device handles kbd_in=3 mouse_in=5
[36640924881] [[32mINFO [0m] [bristle] [CPU3] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[36782339814] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[37091148633] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
echo she[37409433237] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
ll[37608738783] [[32mINFO [0m] [virtio_gpu] [CPU1] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[37622646534] [[32mINFO [0m] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[37631140173] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
-ready
[?25lshell-ready

```
</details>
