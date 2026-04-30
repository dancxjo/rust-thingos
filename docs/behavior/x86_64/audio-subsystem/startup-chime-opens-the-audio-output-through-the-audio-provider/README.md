# ❌ Scenario: Startup chime opens the audio output through the audio provider

> Last run: 2026-04-30 01:50:43

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 0ms | - - - |
| 2 | When I wait for the serial output to contain "chime: Opened /dev/audio/card0/out0" | ❌ | 32566ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[29549250522] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[29838374379] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[29869596966] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[29932546842] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[29984099508] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[30014738160] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[30017406045] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[30075685101] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[30078123438] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[30109603887] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[30133492983] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[30134376888] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[30192752799] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[30194328912] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[30288398844] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[30374610222] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[30400102458] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[30483012120] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[30604345728] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[30663322173] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[30698712132] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[30867736482] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[30947614236] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[31160372463] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[31548309573] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1329025368 elapsed_us=664512
[31549131834] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[31650071772] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[31655535186] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[31705648062] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[31762820034] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[31777634097] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[31779614955] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[31891284480] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[31894703412] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[31907673567] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[32049705864] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[32051672532] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[32089518846] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[32092736049] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[32129441124] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[32152258248] [[32mINFO [0m] [iso9660d] [CPU2] ISO9660D: Starting VFS provider
[32152497531] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[32158407534] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[32161123236] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[32223148947] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=9)
[32229475938] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[32234413134] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[32245501365] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[32252005302] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[32255750208] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[32259986748] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[32265482535] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[32276467047] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[32282599470] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[32326490163] [[32mINFO [0m] [bristle] [CPU3] bristle: published pid 9 to /run/bristle/pid
[32339030691] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[32348134962] [[32mINFO [0m] [bristle] [CPU3] bristle: published device handles kbd_in=3 mouse_in=5
[32364262194] [[32mINFO [0m] [bristle] [CPU3] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[32506302774] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[32639300001] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[32799888759] [[32mINFO [0m] [virtio_gpu] [CPU1] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[32812345665] [[32mINFO [0m] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[32819510097] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[33719883879] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[33789577932] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=11)
[33791212389] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[33796446057] [[32mINFO [0m] [bloom] [CPU2] bloom: ENTERING MAIN
[33797636004] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Starting early audio stack
[33797818032] [[32mINFO [0m] [bloom] [CPU2] bloom: compositor service starting
[33821604663] [[32mINFO [0m] [sprout::pipelines] [CPU3] SPROUT: Audio stack worker running
[33866039592] [[32mINFO [0m] [sprout::pipelines] [CPU3] SPROUT: Early audio device '/sys/devices/pci-0000:00:03.0' matched dev.sound.Virtio; spawning /drivers/virtio_sound
[34011664797] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[34018154808] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: VFS provider loop online
[34057033824] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[34233320088] [[32mINFO [0m] [bloom] [CPU2] bloom: connected to /dev/display/card0 on try 0
[34328422557] [[32mINFO [0m] [bloom] [CPU2] bloom: output0 1920x1080 @ 60000mHz
[34329841029] [[32mINFO [0m] [bloom] [CPU2] bloom: creating service port...
[34353949080] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[34599152280] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[34749751938] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pistil background renderer loaded from /lib/libpistil.so
[34762111230] [[32mINFO [0m] [bloom] [CPU2] bloom: initial wallpaper configured /share/wallpapers/flower.png
[34843744122] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[35074161012] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[35324967843] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[35571597237] [[32mINFO [0m] [bloom::display] [CPU2] bloom: imported buffer 1920x1080 as ID=1
[35683265244] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[35929571799] [[32mINFO [0m] [bloom::display] [CPU2] bloom: imported buffer 96x96 as ID=2
[35931352677] [[32mINFO [0m] [bloom::render] [CPU2] bloom: cursor ready buffer=2 size=96x96 hotspot=9,6
[35943077841] [[32mINFO [0m] [bloom] [CPU2] bloom: watching wallpaper config /session/desktop/wallpaper
[35944455228] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[35982750375] [[32mINFO [0m] [bloom] [CPU2] bloom: Wayland server thread spawned
[35984938143] [[32mINFO [0m] [bloom::loop_types] [CPU2] bloom: service loop started
[35986086510] [[32mINFO [0m] [bloom::loop_types] [CPU2] bloom: output0 1920x1080 @ 60000mHz ready
[35998752174] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: starting
[36012352728] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: listening on /run/wayland-0
[36026809863] [[32mINFO [0m] [bloom::services::wallpaper] [CPU2] bloom: preparing background /share/wallpapers/flower.png
[36189440826] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[36465742137] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[36684464190] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[36687188967] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[37035318012] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Starting AHCI VFS driver
[37081566720] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[37105157991] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Mounted atapi2 at /dev/storage/atapi2
[37106352525] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Provider loop online at /dev/storage/atapi2
[38606373663] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: bristle pid=9
[38635919058] [[32mINFO [0m] [ps2_mouse] [CPU1] ps2_mouse: bristle pid=9
[41818208454] [[32mINFO [0m] [bloom::display] [CPU2] bloom: imported buffer 1920x1080 as ID=3
[42497858502] [[32mINFO [0m] [bloom::display] [CPU2] bloom: imported buffer 460x144 as ID=4
[42507028740] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pointer debug overlay ready buffer=4 size=460x144
[42856390566] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: first commit copied buffer=3 rect=1920x1080 src_px=0xff40422a dst_px=0xff40422a damage=1920x1080+0,0 res_id=1
[42858460392] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: cursor plane blended buffer=2 at 970,545 src_px=0x20181818 dst_before=0xffa30827 dst_after=0xff920a25
[42871723917] [[32mINFO [0m] [bloom::world] [CPU2] bloom: presented cursor buffer=2 at 951,534 size=96x96
[42874430907] [[32mINFO [0m] [bloom::loop_types] [CPU2] First frame rendered
[42908674710] [[32mINFO [0m] [bristle] [CPU3] bristle: bloom sink registered (handle=7)
[42911157960] [[32mINFO [0m] [bloom::services::input_service] [CPU2] bloom: registered bristle pointer sink
[44238017109] [[32mINFO [0m] [sprout::pipelines] [CPU3] SPROUT: Early audio driver started, but /dev/audio/card0/out0 did not appear
[52609220367] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[56257394028] [
```
</details>
