# ❌ Scenario: boot framebuffer driver survives the first compositor commit

> Last run: 2026-04-29 23:47:13

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 9542ms | - [📜](./01/serial.log) - |
| 2 | Then the serial output should contain "display_bootfb: imported buffer" within 60s | ❌ | 61454ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[29427051687] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[29703527370] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[29725721982] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[29763904104] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[29802946239] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[29825143557] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[29827409667] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[29869049034] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[29870799156] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[29893996077] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[29912780964] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[29913684867] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[29955892956] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[29957011293] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[30055934964] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[30145519503] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[30173639859] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[30259537044] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[30371953986] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[30406588740] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[30432447111] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[30549174909] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[30604237389] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[30766342959] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[31066719156] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1086846552 elapsed_us=543423
[31067436048] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[31149577041] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[31186197603] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[31233912303] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[31283807808] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[31295135256] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[31297548513] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[31395708069] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[31399039485] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[31407001956] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[31586435265] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[31588172253] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[31622315703] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[31626433476] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[31651706625] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[31661774760] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[31664994834] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[31666056609] [[32mINFO [0m] [iso9660d] [CPU2] ISO9660D: Starting VFS provider
[31666639983] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[31700023113] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=9)
[31703561571] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[31705147947] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[31714127808] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[31719507369] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[31721935311] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[31724790141] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[31726896531] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[31734207681] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[31739211537] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[31761830100] [[32mINFO [0m] [bristle] [CPU3] bristle: published pid 9 to /run/bristle/pid
[31774833651] [[32mINFO [0m] [bristle] [CPU3] bristle: published device handles kbd_in=3 mouse_in=5
[31785910068] [[32mINFO [0m] [bristle] [CPU3] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[31788034707] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[31884729426] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[31978236873] [[32mINFO [0m] [virtio_gpu] [CPU1] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[31992315531] [[32mINFO [0m] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[31998713373] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[32571693264] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[32730674724] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[32914862937] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[32957279883] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[33021141252] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=11)
[33025534509] [[32mINFO [0m] [bloom] [CPU2] bloom: ENTERING MAIN
[33026502630] [[32mINFO [0m] [bloom] [CPU2] bloom: compositor service starting
[33028822827] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Starting early audio stack
[33051813168] [[32mINFO [0m] [sprout::pipelines] [CPU3] SPROUT: Audio stack worker running
[33089046540] [[32mINFO [0m] [sprout::pipelines] [CPU3] SPROUT: Early audio device '/sys/devices/pci-0000:00:03.0' matched dev.sound.Virtio; spawning /drivers/virtio_sound
[33110947023] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[33116754561] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: VFS provider loop online
[33150438915] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[33355012647] [[32mINFO [0m] [bloom] [CPU2] bloom: connected to /dev/display/card0 on try 0
[33356965950] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[33381824751] [[32mINFO [0m] [bloom] [CPU2] bloom: output0 1920x1080 @ 60000mHz
[33382803135] [[32mINFO [0m] [bloom] [CPU2] bloom: creating service port...
[33543738195] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[33707329095] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[33713291568] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pistil background renderer loaded from /lib/libpistil.so
[33719925162] [[32mINFO [0m] [bloom] [CPU2] bloom: initial wallpaper configured /share/wallpapers/flower.png
[33882813987] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[34051253676] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[34212612753] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[34394730645] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[34578487020] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[34769954835] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[34771742214] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[35055073119] [[32mINFO [0m] [ahci_dis
```
</details>
