# ❌ Scenario: F11 toggles fullscreen in Bloom

> Last run: 2026-04-30 16:15:30

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 12682ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 252ms | - [📜](./02/serial.log) - |
| 3 | And I press f11 | ✅ | 257ms | - [📜](./03/serial.log) - |
| 4 | Then the serial output should contain "BLOOM_FULLSCREEN_TOGGLE_TRIGGERED" within 10s | ❌ | 12319ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[40103348538] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[40372255143] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[40394763750] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[40427879052] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[40461850869] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[40482504843] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[40484398515] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[40524502392] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[40526690094] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[40547497617] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[40563716226] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[40564641942] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[40605740340] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[40606956390] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[40710672057] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[40797876570] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[40820698182] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[40897416747] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[41005461486] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[41033172972] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[41057409525] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[41157115032] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[41209124979] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[41348292150] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[41612482164] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=983654793 elapsed_us=491827
[41613516252] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[41702833062] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[41707919814] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[41749673790] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[41782113219] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[41793956391] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[41796338166] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[41895868113] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[41900429373] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[41902559985] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[42066391521] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[42068879721] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[42105318816] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[42109027059] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[42111284820] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[42112573107] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[42135740097] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[42149582343] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[42202365216] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[42205283604] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[42216277356] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[42225220356] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[42228152307] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[42242339007] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[42242530077] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[42245645937] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[42250452651] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[42254646390] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[42261585135] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[42267612816] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[42311712234] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[42317806905] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[42423977541] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[42436057851] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[42438452925] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[42443981019] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
PS/2 byte received: 0x57
[42493380171] [[32mINFO [0m] [kernel::irq::ps2] [CPU0] buffer_scancode: 0x57
[42495179397] [[32mINFO [0m] [kernel::irq::ps2] [CPU0] PS/2 update_pause: ext=false scan=0x57 rel=false
[42604727418] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
PS/2 byte received: 0xd7
[42663338025] [[32mINFO [0m] [kernel::irq::ps2] [CPU0] buffer_scancode: 0xd7
[42664199886] [[32mINFO [0m] [kernel::irq::ps2] [CPU0] PS/2 update_pause: ext=false scan=0x57 rel=true
[42772283994] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[42947095290] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[43043480106] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[43052258568] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[43057890810] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[43060034127] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[43134267594] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[43300010424] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[43461541728] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[43489680333] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[43491422832] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[43516867515] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[43785730956] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[43807124064] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1920x1080 @ 60000mHz
[43808297313] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[43809101754] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[43809863823] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[44163568845] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[44164720776] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[44165615736] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[44166343881] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil nine-slice shadow renderer loaded
[44176197450] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Solarized Warm
[44681867538] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[44868618069] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[44967997899] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[45289977876] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=96x96 hotspot=21,12
[45306833649] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[45310570503] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[45328028790] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[45517493109] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[45531050103] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=11)
[45563237445] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=12)
[45573164901] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[45575774277] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[45577422165] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1920x1080 @ 60000mHz ready
[45589904085] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: starting
[45610903305] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: listening on /run/wayland-0
[45612776253] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[45724152210] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: connected to /run/wayland-0
[45748041339] [[32mINFO [0m] [clock] [CPU3] clock: connected to /run/wayland-0
[45833709801] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[46062679245] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[46100589447] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[46272578154] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[46327157811] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Starting AHCI VFS driver
[46490982054] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[46505840436] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: bound wp_presentation
[46551945792] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[46615493496] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: bristle pid=8
[46618239954] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Mounted atapi2 at /dev/storage/atapi2
[46618463397] [[32mINFO [0m] [clock] [CPU3] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[46622227080] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Provider loop online at /dev/storage/atapi2
[46649634306] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: claimed /sys/devices/isa-0070 (handle=2)
[46805316393] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: bristle pid=8
PS/2 byte received: 0x47
[46888161639] [[32mINFO [0m] [kernel::irq::ps2] [CPU0] buffer_scancode: 0x47
[46889232819] [[32mINFO [0m] [kernel::irq::ps2] [CPU0] PS/2 update_pause: ext=false scan=0x47 rel=false
[46913808975] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: 2026-04-30 23:17:11 = 1777591031 unix_secs
[46927642212] [[32mINFO [0m] [kernel::time] [CPU3] System clock anchored: unix_secs=1777591031, mono_ns=23463527967, offset=1777591007536472033ns
[46934840106] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: System clock anchored to 1777591031 unix_secs
[46947426636] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: sample rate set to 60 Hz
[46958931195] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 23:17:11.014771526 unix_secs=1777591031.014771526
[47080901307] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=1 rect=1920x1080 src_px=0xffccccff dst_px=0xffccccff damage=1920x1080+0,0 res_id=1
[47082839265] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 958,539 src_px=0x20181818 dst_before=0xffccccff dst_after=0xffb5b5e2
[47109440004] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[47116598958] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=30 frame=6
[47161329501] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[47210038491] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
[48020630229] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: imported dmabuf wl_buffer=111 480x320 format=0x34325241
[52979213595] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 23:17:14.025963347 unix_secs=1777591034.025963347
[53285653443] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[53294500149] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[53940534978] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=30 frame=6
[53958820311] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: xdg_popup obj=23 assigned to xdg_surface=21
[54023595681] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: imported dmabuf wl_buffer=121 160x96 format=0x34325241
[54933417858] [[32mINFO [0m] [bloom::render] [CPU1] bloom: chrome overlay ready buffer=5 size=1920x1080
[55655503497] [[32mINFO [0m] [bloom::render] [CPU1] bloom: shadow overlay ready buffer=6 size=1920x1080
[55770772167] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 15:17:15 UTC-8 system_unix=1777591035.419591495
[57912745572] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=3 height=30 frame=6
[57949849584] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[58605477348] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: frame callback done object=1000 data=28873
[58608439461] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: wp_presentation_feedback.presented object=1001 tv=29.297480949 refresh_ns=16666666 seq=0
[58613636796] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: frame callback done object=1002 data=29264
[58615129848] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: wp_presentation_feedback.presented object=1003 tv=29.303259447 refresh_ns=16666666 seq=1
[59002969476] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 23:17:17.037901512 unix_secs=1777591037.037901512
[61992841251] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 15:17:18 UTC-8 system_unix=1777591038.532543716
[65026938570] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 23:17:20.049872496 unix_secs=1777591040.049872496
[68144931855] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 15:17:21 UTC-8 system_unix=1777591041.608705211
[71050611390] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 23:17:23.061714467 unix_secs=1777591043.061714467
[74476553580] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 15:17:24 UTC-8 system_unix=1777591044.774534042
[77074313415] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 23:17:26.073587490 unix_secs=1777591046.073587490
[80734598373] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 15:17:27 UTC-8 system_unix=1777591047.903535319
[83098234560] [
```
</details>
