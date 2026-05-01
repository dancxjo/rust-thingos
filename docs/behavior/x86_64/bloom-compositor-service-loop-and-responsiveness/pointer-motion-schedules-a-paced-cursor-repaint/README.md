# ❌ Scenario: pointer motion schedules a paced cursor repaint

> Last run: 2026-04-30 17:57:15

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 10532ms | - [📜](./01/serial.log) - |
| 2 | Then the pointer debug overlay should update after mouse movement | ❌ | 34940ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[32659708818] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[32931251892] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[32952971931] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[32987520951] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[33022705518] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[33047307546] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[33049320612] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[33092127585] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[33093934896] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[33115210953] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[33130385904] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[33131108373] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[33170884824] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[33172039098] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[33267264426] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[33351442773] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[33379651635] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[33469582905] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[33573934911] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[33604164825] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[33630646896] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[33737296857] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[33859465002] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[33999021309] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[34254402138] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1060755795 elapsed_us=530377
[34255386198] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[34336486932] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[34340674698] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[34381045380] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[34414010004] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[34425131664] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[34429477203] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[34528149348] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[34532795187] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[34535621340] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[34703144223] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[34704491877] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[34741578102] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[34744024491] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[34744732605] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[34746108408] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[34770532665] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[34781182227] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[34784455398] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[34786327356] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[34825277322] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[34829699553] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[34831345692] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[34842479067] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[34842458145] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[34845648354] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[34850906244] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[34855117143] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[34857615738] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[34865389548] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[34909456197] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[34915564794] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[35045652510] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[35060672592] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[35069064030] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[35072217015] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[35240436066] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[35407375278] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[35580195156] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[35760721491] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[35777121798] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[35789013678] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[35796475902] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[35798621859] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[35929481676] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[36093823722] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[36193536060] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[36274859115] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[36276120045] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[36291921369] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[36532170378] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[36558527907] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1920x1080 @ 60000mHz
[36559722870] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[36560474577] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[36561333402] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[36894350064] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[36896004189] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[36897212682] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[36903001641] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[37059732270] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[37146397002] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil nine-slice shadow renderer loaded
[37154150550] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Solarized Warm
[37702617084] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[37874518605] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[38035258734] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[38202760530] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[38205748680] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[38220022203] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=96x96 hotspot=21,12
[38232076278] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[38235745053] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[38313475530] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[38317258188] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[38318179977] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: starting
[38318771601] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1920x1080 @ 60000mHz ready
[38334405747] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
[38346137874] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: listening on /run/wayland-0
[38348140017] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[38511320034] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=12)
[38527129311] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: connected to /run/wayland-0
[38537124813] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=13)
[38769744585] [[32mINFO [0m] [clock] [CPU1] clock: connected to /run/wayland-0
[38835618360] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[38868292155] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[38869880412] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: bound wp_presentation
[39063407526] [[32mINFO [0m] [clock] [CPU1] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[40742193459] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Starting AHCI VFS driver
[40774404231] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[40789077285] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Mounted atapi2 at /dev/storage/atapi2
[40790083488] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Provider loop online at /dev/storage/atapi2
[40807785612] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: claimed /sys/devices/isa-0070 (handle=2)
[40837822641] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: 2026-05-01 00:57:58 = 1777597078 unix_secs
[40839313284] [[32mINFO [0m] [kernel::time] [CPU3] System clock anchored: unix_secs=1777597078, mono_ns=20419501030, offset=1777597057580498970ns
[40840164981] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: System clock anchored to 1777597078 unix_secs
[40856779425] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: bristle pid=8
[40858917561] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:57:58.009190566 unix_secs=1777597078.009190566
[40867557357] [[32mINFO [0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x47
[40867697112] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: bristle pid=8
[40872147261] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x47 (is_aux=false)
[40887746922] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: sample rate set to 60 Hz
[42658738617] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=3 rect=1920x1080 src_px=0xff40422a dst_px=0xff40422a damage=1920x1080+0,0 res_id=1
[42661519758] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 958,539 src_px=0x20181818 dst_before=0xff880604 dst_after=0xff7a0807
[42671207106] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[42676159614] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=30 frame=6
[42690753039] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[42730189755] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[42738137574] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[42809049195] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: imported dmabuf wl_buffer=111 480x320 format=0x34325241
[43030078872] [[32mINFO [0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x38
[43038034083] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0x38 (is_aux=false)
[43040225547] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: read scancode 0x38
[43055594505] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: edge detected: Down { key: LeftAlt, mods: Mods(4), repeat: false }
[43058449731] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: sent LeftAlt to bristle (pid=8)
[43073147667] [[32mINFO [0m] [bristle] [CPU2] bristle: KeyDown received: LeftAlt
[43080136473] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 16:57:59 UTC-8 system_unix=1777597079.118608937
[43195560705] [[32mINFO [0m] [bloom::input] [CPU1] bloom: KeyDown received: LeftAlt (raw=0x00e2, mods=Mods(4), repeat=false)
[43202603466] [[32mINFO [0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x41
[43202693457] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=30 frame=6
[43207781661] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0x41 (is_aux=false)
[43208978043] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: read scancode 0x41
[43209896994] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: edge detected: Down { key: F7, mods: Mods(4), repeat: false }
[43212772647] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: sent F7 to bristle (pid=8)
[43217162901] [[32mINFO [0m] [bristle] [CPU2] bristle: KeyDown received: F7
[43227292746] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[43277812512] [[32mINFO [0m] [bloom::input] [CPU1] bloom: KeyDown received: F7 (raw=0x0040, mods=Mods(4), repeat=false)
[43279337937] [[32mINFO [0m] [bloom::input] [CPU1] bloom: pointer debug overlay enabled
[43378796340] [[32mINFO [0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xc1
[43432024779] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0xc1 (is_aux=false)
[43433391606] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: read scancode 0xc1
[43434801465] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: edge detected: Up { key: F7, mods: Mods(4) }
[43437693981] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: sent F7 to bristle (pid=8)
[43553531241] [[32mINFO [0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xb8
[43558889913] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0xb8 (is_aux=false)
[43561162029] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: read scancode 0xb8
[43562376231] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: edge detected: Up { key: LeftAlt, mods: Mods(0) }
[43565267823] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: sent LeftAlt to bristle (pid=8)
[43717291893] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pointer debug overlay ready buffer=4 size=460x144
[46030870248] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=3 height=30 frame=6
[46053059877] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_popup obj=23 assigned to xdg_surface=21
[46095044490] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: imported dmabuf wl_buffer=121 160x96 format=0x34325241
[46792652841] [[32mINFO [0m] [bloom::render] [CPU1] bloom: chrome overlay ready buffer=7 size=1920x1080
[46859989077] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:58:01.010397511 unix_secs=1777597081.010397511
[47361684414] [[32mINFO [0m] [bloom::render] [CPU1] bloom: shadow overlay ready buffer=8 size=1920x1080
[48072472965] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: frame callback done object=1000 data=24028
[48074692809] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: wp_presentation_feedback.presented object=1001 tv=24.034347298 refresh_ns=16666666 seq=0
[49307806185] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 16:58:02 UTC-8 system_unix=1777597082.233913250
[49932040620] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: frame callback done object=1002 data=24958
[49935349629] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: wp_presentation_feedback.presented object=1003 tv=24.961926973 refresh_ns=16666666 seq=1
[52862443755] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:58:04.011680158 unix_secs=1777597084.011680158
[55375898160] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 16:58:05 UTC-8 system_unix=1777597085.268053238
[58865335584] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:58:07.013126997 unix_secs=1777597087.013126997
[61409269878] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 16:58:08 UTC-8 system_unix=1777597088.284903437
[64867702284] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:58:10.014286026 unix_secs=1777597090.014286026
[67446610473] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 16:58:11 UTC-8 system_unix=1777597091.303590482
[70870332093] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:58:13.015594809 unix_secs=1777597093.015594809
[73448538009] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 16:58:14 UTC-8 system_unix=1777597094.304513429
[76872859569] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:58:16.016867968 unix_secs=1777597096.016867968
[79450201116] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 16:58:17 UTC-8 system_unix=1777597097.305396281
[82875235839] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:58:19.018058842 unix_secs=1777597099.018058842
[85488607512] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 16:58:20 UTC-8 system_unix=1777597100.324423325
[88878086352] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:58:22.019432371 unix_secs=1777597102.019432371
[91489254615] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 16:58:23 UTC-8 system_unix=1777597103.324905243
[94880275479] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:58:25.020598231 unix_secs=1777597105.020598231
[97524355434] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 16:58:26 UTC-8 system_unix=1777597106.342384340
[100882951488] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:58:28.021921369 unix_secs=1777597108.021921369
[103559761239] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 16:58:29 UTC-8 system_unix=1777597109.360005056
[106885548165] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:58:31.023210682 unix_secs=1777597111.023210682
[109595708310] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 16:58:32 UTC-8 system_unix=1777597112.378117554
[112888022874] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:58:34.024452739 unix_secs=1777597114.024452739
[115631725704] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 16:58:35 UTC-8 system_unix=1777597115.396112985
[118890663903] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:58:37.025772049 unix_secs=1777597117.025772049
[121667540973] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 16:58:38 UTC-8 system_unix=1777597118.414035569
[124893267609] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:58:40.027078110 unix_secs=1777597120.027078110
[127668652254] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 16:58:41 UTC-8 system_unix=1777597121.414577234
[130895827887] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:58:43.028346781 unix_secs=1777597123.028346781
[133671364629] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 16:58:44 UTC-8 system_unix=1777597124.415885555
[136898185050] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:58:46.029537490 unix_secs=1777597126.029537490
[139707810198] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 16:58:47 UTC-8 system_unix=1777597127.433976785
[142900767471] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:58:49.030829410 unix_secs=1777597129.030829410
[145743886398] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 16:58:50 UTC-8 system_unix=1777597130.452035526
[148903314813] [
```
</details>
