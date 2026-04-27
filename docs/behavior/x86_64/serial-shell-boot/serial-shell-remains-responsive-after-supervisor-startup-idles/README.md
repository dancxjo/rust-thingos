# ✅ Scenario: Serial shell remains responsive after supervisor startup idles

> Last run: 2026-04-27 08:36:49

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 10536ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 254ms | - [📜](./02/serial.log) - |
| 3 | And I wait for the serial output to contain "SPROUT: Continuing supervisor startup" | ✅ | 0ms | - - - |
| 4 | Then the serial shell is still responsive | ✅ | 477ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[32028625332] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[32478605478] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[32518425258] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[32586210591] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[32647653621] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[32685611178] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[32688838380] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[32756459472] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[32758521609] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[32787787857] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[32806041774] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[32806776783] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[32850733080] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[32851897419] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[32956356246] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[33039336759] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[33077243133] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[33166902516] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[33278673384] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[33347867124] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[33387458445] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[33559737816] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[33632049759] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[33827560140] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[34180838538] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1300851684 elapsed_us=650425
[34181879589] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[34303665858] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[34309392909] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[34358271816] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[34405045257] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[34416767616] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[34422840441] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[34555549446] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[34562088825] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[34623650259] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[34764313419] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[34768005393] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[34835750070] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[34865763042] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[34899319323] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[34917362799] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[34922571816] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[34926325170] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[34970020437] [[32mINFO [0m] [iso9660d] [CPU2] ISO9660D: Starting VFS provider
[34998509634] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=9)
[35006669610] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[35010977364] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[35039364261] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[35079868692] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[35085844464] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[35100476829] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[35135651892] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[35147473020] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[35156902374] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[35216697252] [[32mINFO [0m] [bristle] [CPU3] bristle: published pid 9 to /run/bristle/pid
[35233275594] [[32mINFO [0m] [bristle] [CPU3] bristle: published device handles kbd_in=3 mouse_in=5
[35248821036] [[32mINFO [0m] [bristle] [CPU3] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[35262788319] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[35390822115] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
echo [35700435144] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
SHE[35917614282] [[32mINFO [0m] [virtio_gpu] [CPU1] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[35933583279] [[32mINFO [0m] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
LL_ALIV[36192533916] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
E_PR[36416176335] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
OBE
[36684315162] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[?25lSHELL_ALIVE_PROBE
[?25h
```
</details>
