# ✅ Scenario: bloom compositor reacts to wallpaper watch path

> Last run: 2026-04-30 02:28:06

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8915ms | - [📜](./01/serial.log) - |
| 2 | Then the serial output should contain "bloom: service loop started" within 60s | ✅ | 1031ms | - [📜](./02/serial.log) - |
| 3 | When I wait for the shell prompt | ✅ | 151ms | - [📜](./03/serial.log) - |
| 4 | And I type "echo /share/wallpapers/flower.png > /session/desktop/wallpaper" on the serial console | ✅ | 1014ms | - [📜](./04/serial.log) - |
| 5 | Then the serial output should contain "bloom: reacting to wallpaper change" within 60s | ✅ | 820ms | - [📜](./05/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[27690773121] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[27940495374] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[27959608809] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[27990665010] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[28021633299] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[28040160159] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[28041740496] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[28076784252] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[28078223019] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[28097089911] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[28110648918] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[28111147317] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[28145218464] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[28146255192] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[28226150502] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[28303718652] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[28325150304] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[28395460038] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[28493269497] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[28518930165] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[28541432337] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[28654592175] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[28708486785] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[28864911438] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[29143409163] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=977646549 elapsed_us=488823
[29143986399] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[29216549868] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[29220085455] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[29259603285] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[29299303704] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[29308638051] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[29310354480] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[29400004524] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[29402956440] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[29408772096] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[29565876747] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[29567751972] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[29604179979] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[29610746517] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[29631509259] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[29639999070] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[29642527035] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[29643490239] [[32mINFO [0m] [iso9660d] [CPU2] ISO9660D: Starting VFS provider
[29644873665] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[29696851041] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=9)
[29700281061] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[29702008578] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[29708495388] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[29712065724] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[29713840068] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[29716376151] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[29718647475] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[29724534807] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[29729944563] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[29746591050] [[32mINFO [0m] [bristle] [CPU3] bristle: published pid 9 to /run/bristle/pid
[29757751584] [[32mINFO [0m] [bristle] [CPU3] bristle: published device handles kbd_in=3 mouse_in=5
[29766972345] [[32mINFO [0m] [bristle] [CPU3] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[29778333453] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[29841451002] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[29929227636] [[32mINFO [0m] [virtio_gpu] [CPU1] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[29941020615] [[32mINFO [0m] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[29947300317] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[30737746215] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[30768250359] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[30771853134] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: VFS provider loop online
[30779851773] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[30803418822] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=11)
[30807328464] [[32mINFO [0m] [bloom] [CPU2] bloom: ENTERING MAIN
[30808182966] [[32mINFO [0m] [bloom] [CPU2] bloom: compositor service starting
[30810082116] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Starting early audio stack
[30832185714] [[32mINFO [0m] [sprout::pipelines] [CPU3] SPROUT: Audio stack worker running
[30857943600] [[32mINFO [0m] [sprout::pipelines] [CPU3] SPROUT: Early audio device '/sys/devices/pci-0000:00:03.0' matched dev.sound.Virtio; spawning /drivers/virtio_sound
[30973405386] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[31126656693] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[31233344274] [[32mINFO [0m] [virtio_sound] [CPU0] SND: rpc Lookup 'out0'
[31247028615] [[32mINFO [0m] [virtio_sound] [CPU0] SND: rpc 6
[31271339484] [[32mINFO [0m] [sprout::pipelines] [CPU3] SPROUT: Audio stack launched chime (PID=14)
[31283471175] [[32mINFO [0m] [chime] [CPU0] chime: Waiting for /dev/audio/card0/out0 ...
[31292977551] [[32mINFO [0m] [virtio_sound] [CPU0] SND: rpc Lookup 'out0'
[31294087902] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[31312210710] [[32mINFO [0m] [bloom] [CPU2] bloom: connected to /dev/display/card0 on try 0
[31317487740] [[32mINFO [0m] [chime] [CPU0] chime: Opened /dev/audio/card0/out0 (fd=15)
[31327612305] [[32mINFO [0m] [virtio_sound] [CPU0] SND: rpc DeviceCall handle=2 op=32769
[31333102845] [[32mINFO [0m] [bloom] [CPU2] bloom: output0 1920x1080 @ 60000mHz
[31333810464] [[32mINFO [0m] [bloom] [CPU2] bloom: creating service port...
[31383887370] [[32mINFO [0m] [virtio_sound] [CPU0] SND: rpc DeviceCall handle=2 op=32770
[31399729845] [[32mINFO [0m] [chime] [CPU0] chime: Configured stream (rate=44100Hz, fmt=1, ch=2)
[31410529788] [[32mINFO [0m] [virtio_sound] [CPU0] SND: rpc DeviceCall handle=2 op=32773
[31418052237] [[32mINFO [0m] [virtio_sound] [CPU0] SND: control SET_PARAMS begin for stream 0
[31420836216] [[32mINFO [0m] [virtio_sound] [CPU0] SND: control SET_PARAMS complete for stream 0
[31422285939] [[32mINFO [0m] [virtio_sound] [CPU0] SND: control PREPARE begin for stream 0
[31428552375] [[32mINFO [0m] [chime] [CPU0] chime: Generating classic chime...
[31486471797] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[31537577412] [[32mINFO [0m] [virtio_sound] [CPU0] SND: control PREPARE complete for stream 0
[31540199823] [[32mINFO [0m] [virtio_sound] [CPU0] SND: control START begin for stream 0
[31541672085] [[32mINFO [0m] [virtio_sound] [CPU0] SND: control START complete for stream 0
[31543023666] [[32mINFO [0m] [virtio_sound] [CPU0] SND: Hardware playback started
[31623596367] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pistil background renderer loaded from /lib/libpistil.so
[31630127727] [[32mINFO [0m] [bloom] [CPU2] bloom: initial wallpaper configured /share/wallpapers/flower.png
[31662073245] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[31834735185] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[32008291821] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[32228538111] [[32mINFO [0m] [bloom::display] [CPU2] bloom: imported buffer 1920x1080 as ID=1
[32232248928] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[32414441895] [[32mINFO [0m] [bloom::display] [CPU2] bloom: imported buffer 96x96 as ID=2
[32415614418] [[32mINFO [0m] [bloom::render] [CPU2] bloom: cursor ready buffer=2 size=96x96 hotspot=9,6
[32419374933] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[32423228904] [[32mINFO [0m] [bloom] [CPU2] bloom: watching wallpaper config /session/desktop/wallpaper
[32450350515] [[32mINFO [0m] [bloom] [CPU2] bloom: Wayland server thread spawned
[32451958044] [[32mINFO [0m] [bloom::loop_types] [CPU2] bloom: service loop started
[32452695891] [[32mINFO [0m] [bloom::loop_types] [CPU2] bloom: output0 1920x1080 @ 60000mHz ready
[32453529537] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: starting
[32465692710] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: listening on /run/wayland-0
[32527425909] [[32mINFO [0m] [bloom::display] [CPU2] bloom: imported buffer 460x144 as ID=3
[32528614767] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pointer debug overlay ready buffer=3 size=460x144
[32858704494] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[33058725579] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
e[33229406958] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
c[33231917202] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
ho /shar[33672238776] [[32mINFO [0m] [ahci_disk] [CPU3] AHCI: Starting AHCI VFS driver
[33705265110] [[32mINFO [0m] [ahci_disk] [CPU3] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[33717002121] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: bristle pid=9
[33718349676] [[32mINFO [0m] [ahci_disk] [CPU3] AHCI: Mounted atapi2 at /dev/storage/atapi2
[33719065941] [[32mINFO [0m] [ahci_disk] [CPU3] AHCI: Provider loop online at /dev/storage/atapi2
[33723267369] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: first commit copied buffer=1 rect=1920x1080 src_px=0xffccccff dst_px=0xffccccff damage=1920x1080+0,0 res_id=1
[33724351122] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: cursor plane blended buffer=2 at 970,545 src_px=0x20181818 dst_before=0xffccccff dst_after=0xffb5b5e2
e[33734643195] [[32mINFO [0m] [bloom::world] [CPU2] bloom: presented cursor buffer=2 at 951,534 size=96x96
[33736339725] [[32mINFO [0m] [bloom::loop_types] [CPU2] First frame rendered
[33744146667] [[32mINFO [0m] [bloom::services::wallpaper] [CPU2] bloom: preparing background /share/wallpapers/flower.png
/wallpape[34204982790] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: bristle pid=9
rs/flower.png > /session/desktop/wallpaper
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[37330770675] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: sample rate set to 200 Hz
[38350249179] [[32mINFO [0m] [bloom::display] [CPU2] bloom: imported buffer 1920x1080 as ID=4
[38660804160] [[32mINFO [0m] [bloom::services::input_service] [CPU2] bloom: registered bristle pointer sink
[38668604568] [[32mINFO [0m] [bristle] [CPU3] bristle: bloom sink registered (handle=7)
[38871135831] [
```
</details>
