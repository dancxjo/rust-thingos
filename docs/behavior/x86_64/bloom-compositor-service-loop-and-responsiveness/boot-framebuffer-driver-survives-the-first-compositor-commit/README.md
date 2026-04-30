# ✅ Scenario: boot framebuffer driver survives the first compositor commit

> Last run: 2026-04-30 01:19:46

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 9739ms | - [📜](./01/serial.log) - |
| 2 | Then the serial output should contain "display_bootfb: imported buffer" within 60s | ✅ | 929ms | - [📜](./02/serial.log) - |
| 3 | And the serial output should contain "First frame rendered" within 60s | ✅ | 311ms | - [📜](./03/serial.log) - |
| 4 | And the serial output should not contain "task='display_bootfb'" | ✅ | 300ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[29879162016] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[30159669870] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[30181064694] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[30223376865] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[30257846718] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[30287477550] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[30289599285] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[30332643726] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[30335468097] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[30358516023] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[30374774925] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[30375401661] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[30418785606] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[30419911731] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[30515656644] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[30612522105] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[30639613719] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[30722110584] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[30855502491] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[30939343710] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[30972022653] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[31157247957] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[31245652383] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[31415675544] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[31709945157] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1265362725 elapsed_us=632681
[31710660366] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[31793367144] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[31796968830] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[31836176262] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[31877857110] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[31890381237] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[31892687211] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[31998096339] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[32002133163] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[32008041351] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[32166359346] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[32169063564] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[32205969939] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[32209657128] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[32233608033] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[32248229706] [[32mINFO [0m] [iso9660d] [CPU2] ISO9660D: Starting VFS provider
[32265373008] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[32267903250] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[32269512429] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[32312081505] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=9)
[32318467335] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[32320869207] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[32328325260] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[32332004628] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_bootfb'
[32334426399] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[32337811209] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[32340857142] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[32346378702] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[32352653553] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_bootfb' (boot_fd=3, bind_id=322371585)
[32366481939] [[32mINFO [0m] [bristle] [CPU3] bristle: published pid 9 to /run/bristle/pid
[32380065993] [[32mINFO [0m] [bristle] [CPU3] bristle: published device handles kbd_in=3 mouse_in=5
[32390046744] [[32mINFO [0m] [bristle] [CPU3] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[32390161386] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=bootfb)
[32518979493] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[32688142905] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[32873235252] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: Starting VFS-native bootfb driver (v0.4.1) TID=10 PID=10
[32874487932] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: boot_arg=3
[32875413681] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: Mapping bootstrap memfd 3 size=4096...
[32878186176] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: vm_map success at 0x400000001000
[32879046783] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: Recovered handles: req_read=4, resp_write=5, svc=1, id=322371585
[32881661373] [[32mINFO [0m] [display_bootfb::driver] [CPU1] display_bootfb: probing /dev/fb0...
[33241087836] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: Sending MSG_BIND_READY handshake (class_mask=0x3) to supervisor port...
[33252598236] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: Sent MSG_BIND_READY (result=Ok(())), waiting for MSG_BIND_ASSIGNED...
[33258783558] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[33275689722] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[33345839967] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=11)
[33349888242] [[32mINFO [0m] [bloom] [CPU2] bloom: ENTERING MAIN
[33350844021] [[32mINFO [0m] [bloom] [CPU2] bloom: compositor service starting
[33355928562] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Starting early audio stack
[33376116279] [[32mINFO [0m] [sprout::pipelines] [CPU3] SPROUT: Audio stack worker running
[33410421528] [[32mINFO [0m] [sprout::pipelines] [CPU3] SPROUT: Early audio device '/sys/devices/pci-0000:00:03.0' matched dev.sound.Virtio; spawning /drivers/virtio_sound
[33434132028] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[33545988465] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: entering VFS provider service loop
[33721432074] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[33823283109] [[32mINFO [0m] [bloom] [CPU2] bloom: connected to /dev/display/card0 on try 0
[33856105932] [[32mINFO [0m] [bloom] [CPU2] bloom: output0 1920x1080 @ 60000mHz
[33857526912] [[32mINFO [0m] [bloom] [CPU2] bloom: creating service port...
[33899696556] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[34082042115] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[34218633702] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pistil background renderer loaded from /lib/libpistil.so
[34226354745] [[32mINFO [0m] [bloom] [CPU2] bloom: initial wallpaper configured /share/wallpapers/flower.png
[34279685682] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[34450695972] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[34648091412] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[34990401468] [[32mINFO [0m] [display_bootfb::driver] [CPU1] display_bootfb: imported buffer 1 (1920x1080 @ 0x4000011d5000)
[35021514561] [[32mINFO [0m] [bloom::display] [CPU2] bloom: imported buffer 1920x1080 as ID=1
[35026533960] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[35278644753] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[35281763781] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 12 driver(s) found
[35412485955] [[32mINFO [0m] [display_bootfb::driver] [CPU1] display_bootfb: imported buffer 2 (96x96 @ 0x4000019e7000)
[35441884236] [[32mINFO [0m] [bloom::display] [CPU2] bloom: imported buffer 96x96 as ID=2
[35443762827] [[32mINFO [0m] [bloom::render] [CPU2] bloom: cursor ready buffer=2 size=96x96 hotspot=9,6
[35457224748] [[32mINFO [0m] [bloom] [CPU2] bloom: watching wallpaper config /session/desktop/wallpaper
[35503157613] [[32mINFO [0m] [bloom] [CPU2] bloom: Wayland server thread spawned
[35506171866] [[32mINFO [0m] [bloom::loop_types] [CPU2] bloom: service loop started
[35507355939] [[32mINFO [0m] [bloom::loop_types] [CPU2] bloom: output0 1920x1080 @ 60000mHz ready
[35548992336] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: starting
[35564388486] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: listening on /run/wayland-0
[35616813309] [[32mINFO [0m] [display_bootfb::driver] [CPU1] display_bootfb: imported buffer 3 (460x144 @ 0x400001c32000)
[35630415249] [[32mINFO [0m] [bloom::display] [CPU2] bloom: imported buffer 460x144 as ID=3
[35632207776] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pointer debug overlay ready buffer=3 size=460x144
[35699321394] [[32mINFO [0m] [bloom::world] [CPU2] bloom: presented cursor buffer=2 at 951,534 size=96x96
[35705133255] [[32mINFO [0m] [bloom::loop_types] [CPU2] First frame rendered
[35714483178] [[32mINFO [0m] [bloom::services::wallpaper] [CPU2] bloom: preparing background /share/wallpapers/flower.png
[35819124858] [[32mINFO [0m
```
</details>
