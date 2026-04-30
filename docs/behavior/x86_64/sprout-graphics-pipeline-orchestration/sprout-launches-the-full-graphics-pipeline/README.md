# ❌ Scenario: sprout launches the full graphics pipeline

> Last run: 2026-04-30 00:47:16

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8515ms | - [📜](./01/serial.log) - |
| 2 | Then the serial output should contain "SPROUT: Starting full pipeline (graphics + input)" within 60s | ✅ | 205ms | - [📜](./02/serial.log) - |
| 3 | And the serial output should contain "SPROUT: Spawned bristle" within 60s | ✅ | 0ms | - - - |
| 4 | And the serial output should contain "SPROUT: Spawned bloom" within 120s | ✅ | 509ms | - [📜](./04/serial.log) - |
| 5 | And the serial output should contain "bloom: compositor service starting" within 120s | ✅ | 0ms | - - - |
| 6 | And the serial output should contain "bloom: service loop started" within 180s | ❌ | 181446ms | - [📜](./06/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26411022297] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[26676507726] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26696462529] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26731000032] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26763858033] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26783361660] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26785071951] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26821994727] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[26823582423] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26843052918] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26858056038] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[26858650170] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[26895217008] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26896292973] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26982727233] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[27063903966] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[27086325750] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[27160314291] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[27266897658] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[27293568786] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[27317426169] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[27419351289] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[27478149204] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[27615257835] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27869182011] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=952849095 elapsed_us=476424
[27870184782] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27955241424] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[27958745892] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27998318139] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[28040168343] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[28052071443] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[28053998511] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[28157839644] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[28161890097] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[28165304475] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28325938608] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[28328163270] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[28365434295] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[28369242099] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[28392207525] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[28404129237] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[28406647731] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[28408460553] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[28408765935] [[32mINFO [0m] [iso9660d] [CPU2] ISO9660D: Starting VFS provider
[28470662814] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=9)
[28475465436] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[28477594299] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[28484250069] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[28490504559] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[28492399650] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[28495501518] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[28498828776] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[28504375119] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[28509039273] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[28524751860] [[32mINFO [0m] [bristle] [CPU3] bristle: published pid 9 to /run/bristle/pid
[28535968593] [[32mINFO [0m] [bristle] [CPU3] bristle: published device handles kbd_in=3 mouse_in=5
[28545776754] [[32mINFO [0m] [bristle] [CPU3] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[28561335858] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[28629196959] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[28736658621] [[32mINFO [0m] [virtio_gpu] [CPU1] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[28749167370] [[32mINFO [0m] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[28756104795] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[29334299478] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[29495011590] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[29665490976] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[29857598892] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[29883200754] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[29959579947] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=11)
[29961848961] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[29963869485] [[32mINFO [0m] [bloom] [CPU2] bloom: ENTERING MAIN
[29964648186] [[32mINFO [0m] [bloom] [CPU2] bloom: compositor service starting
[29965431111] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: VFS provider loop online
[29970841098] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Starting early audio stack
[29989562163] [[32mINFO [0m] [sprout::pipelines] [CPU3] SPROUT: Audio stack worker running
[30015779772] [[32mINFO [0m] [sprout::pipelines] [CPU3] SPROUT: Early audio device '/sys/devices/pci-0000:00:03.0' matched dev.sound.Virtio; spawning /drivers/virtio_sound
[30077954973] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[30248212170] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[30424746231] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[30595207368] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[30762622209] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[30945894144] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[31055019963] [[32mINFO [0m] [bloom] [CPU2] bloom: connected to /dev/display/card0 on try 0
[31077327501] [[32mINFO [0m] [bloom] [CPU2] bloom: output0 1920x1080 @ 60000mHz
[31078135473] [[32mINFO [0m] [bloom] [CPU2] bloom: creating service port...
[31141648263] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[31335457560] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[31382399994] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pistil background renderer loaded from /lib/libpistil.so
[31391041275] [[32mINFO [0m] [bloom] [CPU2] bloom: initial wallpaper configured /share/wallpapers/flower.png
[31518404841] [[32mINFO [0m] [cambium::catalog]
```
</details>
