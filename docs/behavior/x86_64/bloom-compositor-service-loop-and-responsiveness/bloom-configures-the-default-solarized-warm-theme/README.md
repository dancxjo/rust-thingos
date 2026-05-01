# ❌ Scenario: bloom configures the default Solarized Warm theme

> Last run: 2026-05-01 13:12:24

## Steps

| # | Step | Result | Duration | Before | After | Artifacts |
|---|------|--------|----------|--------|-------|-----------|
| 1 | Given the machine is booted | ✅ | 11361ms | <a href="./01/before.png"><img src="./01/before.png" width="120" /></a> | <a href="./01/after.png"><img src="./01/after.png" width="120" /></a> | [📜](./01/serial.log) |
| 2 | Then the serial output should contain "bloom: initial theme configured Solarized Warm" within 60s | ❌ | 61127ms | <a href="./02/before.png"><img src="./02/before.png" width="120" /></a> | - | [📜](./02/serial.log) [timeout.png](./02/timeout.png) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[34676336145] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[34943045940] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[34963286754] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[34993608606] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[35026284447] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[35045466720] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[35047223805] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[35088781398] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[35090519970] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[35110392537] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[35125233165] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[35125810170] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[35160735324] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[35161757202] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[35250552447] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[35326885308] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[35349149946] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[35421452022] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[35521074369] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[35546670951] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[35568749271] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[35666527677] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[35716880298] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[35848838058] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[36090824748] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=909298401 elapsed_us=454649
[36091615494] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[36167869584] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[36172704084] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[36210728895] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[36248893065] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[36257271204] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[36258678819] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[36358489068] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[36361889487] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[36364503153] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[36529497576] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[36531327822] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[36564407781] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[36566660394] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[36568082364] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[36568665441] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[36592904238] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[36603373455] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[36606887196] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[36608405097] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[36615243522] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[36618854910] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[36620379873] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[36630884730] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[36633556377] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[36638204856] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[36643402785] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[36656980800] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[36668118663] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[36678128751] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[36705618114] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[36711559863] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[36806147961] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[36820509891] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[36831684747] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: cursor queue (queue 1) configured
[36833167833] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[36834409095] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: composition paths: gpu_opaque_copy=enabled cpu_alpha_blend=enabled
[36846858576] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: host scanout 1280x800 enabled=true (bootfb was 1920x1080)
[36879124260] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[37073272203] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[37109224053] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor DMA buffer ready (16 pages at phys=0x2723000)
[37177338561] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[37186344294] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[37191399762] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[37194593436] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[37240702125] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[37410831150] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[37594449057] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[37616228166] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[37617458670] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[37623427083] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[37884241329] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[37981984788] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[38004715683] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1280x800 @ 60000mHz
[38005797687] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[38006627439] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[38007464550] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports GPU blit (hardware transfer/flush)
[38008290111] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports direct scanout (zero-copy path to display)
[38010284301] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports partial flush (damage regions)
[38011119036] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports resource cache (pre-allocated buffer pool)
[38011985847] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[38370094554] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hwrng' name='hwrng' class=Other kind='dev.rng.HwRng' start='_RNvCs7wlZbVUrQWf_5hwrng20thingos_driver_start'
[38533059576] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[38637303144] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[38639359803] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[38642575818] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[38651039955] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Facet Frame
[38978802324] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[39005439231] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[39160548504] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[39309787935] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[39572084376] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=48x48 hotspot=11,6
[39582508020] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[39584570025] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[39629970468] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[39631870476] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[39632768901] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1280x800 @ 60000mHz ready
[39634135365] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: starting
[39646523565] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[39655553157] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: listening on /run/wayland-0
[39656695353] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[39819594474] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[39898933173] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=1 rect=1280x800 src_px=0xff0b0a10 dst_px=0xff0b0a10 damage=1280x800+0,0 res_id=1 gpu_planes=0 cpu_planes=2
[39900417414] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 638,399 src_px=0x10202020 dst_before=0xff0b0a10 dst_after=0xff0c0b11
[39929443818] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=12)
[39942935043] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: connected to /run/wayland-0
[39950064759] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=13)
[39983371527] [[32mINFO [0m] [clock] [CPU1] clock: running at low scheduler priority tid=13
[39986688687] [[32mINFO [0m] [clock] [CPU1] clock: connected to /run/wayland-0
[40237636461] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[40272331869] [[32mINFO [0m] [clock] [CPU1] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[40273397274] [[32mINFO [0m] [clock] [CPU1] clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf
[40296236244] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[40298351049] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: bound wp_presentation
[40298395005] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[40304592900] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: wl_region smoke test: create+add+set_opaque+destroy
[40308200427] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[40423642182] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=28 frame=6
[40433409951] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[40447341231] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[40474364271] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[40560181926] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/xhci' name='xhci' class=Block kind='dev.usb.Xhci' start='thingos_driver_start_safe'
[40562632770] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 15 driver(s) found
[40572265833] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: imported dmabuf wl_buffer=111 480x320 format=0x34325241
[40619150550] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Starting AHCI VFS driver
[40653751677] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[40671454659] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Mounted atapi2 at /dev/storage/atapi2
[40673310447] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Provider loop online at /dev/storage/atapi2
[40704481620] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: bristle pid=8
[40786122003] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: bristle pid=8
[40819809492] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: sample rate set to 60 Hz
[40862970522] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: claimed /sys/devices/isa-0070 (handle=2)
[40883012808] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: 2026-05-01 20:18:46 = 1777666726 unix_secs
[40884209421] [[32mINFO [0m] [kernel::time] [CPU3] System clock anchored: unix_secs=1777666726, mono_ns=20441957679, offset=1777666705558042321ns
[40885047456] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: System clock anchored to 1777666726 unix_secs
[40897903167] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:18:46.005547646 unix_secs=1777666726.005547646
[41189799849] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=28 frame=6
[41195917686] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[41197657248] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
[45008650500] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=3 height=28 frame=6
[45019185783] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_popup obj=23 assigned to xdg_surface=21
[45048182982] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: imported dmabuf wl_buffer=121 160x96 format=0x34325241
[45640296141] [[32mINFO [0m] [bloom::render] [CPU1] bloom: flat window overlays ready size=1280x800
[46896392499] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:18:49.006132937 unix_secs=1777666729.006132937
[47305750272] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: frame callback done object=1000 data=23633
[47308784721] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: wp_presentation_feedback.presented object=1001 tv=23.636234803 refresh_ns=16666666 seq=0
[47637371232] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: frame callback done object=1002 data=23814
[47638334304] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: wp_presentation_feedback.presented object=1003 tv=23.817333045 refresh_ns=16666666 seq=1
[52897562355] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:18:52.006747070 unix_secs=1777666732.006747070
[58898464680] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:18:55.007216878 unix_secs=1777666735.007216878
[64899687270] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:18:58.007838650 unix_secs=1777666738.007838650
[70900657014] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:19:01.008295670 unix_secs=1777666741.008295670
[74797925631] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 12:19:02 UTC-8 system_unix=1777666742.956313489
[76902113739] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:19:04.008930676 unix_secs=1777666744.008930676
[82902953331] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:19:07.009472011 unix_secs=1777666747.009472011
[88904199186] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:19:10.010064760 unix_secs=1777666750.010064760
[94905087189] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:19:13.010532538 unix_secs=1777666753.010532538
[100906228203] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:19:16.011101263 unix_secs=1777666756.011101263
[106907420103] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:19:19.011702790 unix_secs=1777666759.011702790
[112908958206] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:19:22.012448329 unix_secs=1777666762.012448329
[118909670682] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:19:25.012821067 unix_secs=1777666765.012821067
[124910922708] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:19:28.013433913 unix_secs=1777666768.013433913
[130911943833] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:19:31.013956124 unix_secs=1777666771.013956124
[136912885065] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:19:34.014444808 unix_secs=1777666774.014444808
[142914080628] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:19:37.015026353 unix_secs=1777666777.015026353
[148914976386] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:19:40.015501771 unix_secs=1777666780.015501771
[148986120690] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 12:19:40 UTC-8 system_unix=1777666780.050830911
[154916611509] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:19:43.016275063 unix_secs=1777666783.016275063
[160917825321] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:19:46.016879345 unix_secs=1777666786.016879345
[166918864563] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:19:49.017403652 unix_secs=1777666789.017403652
[172919652147] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:19:52.017812294 unix_secs=1777666792.017812294
[178921059702] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:19:55.018484111 unix_secs=1777666795.018484111
[184921771056] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:19:58.018881500 unix_secs=1777666798.018881500
[190924611603] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:20:01.020255062 unix_secs=1777666801.020255062
[196925074104] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:20:04.020492071 unix_secs=1777666804.020492071
[202925381571] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:20:07.020673954 unix_secs=1777666807.020673954
[208926303366] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:20:10.021146154 unix_secs=1777666810.021146154
[214927399698] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:20:13.021694897 unix_secs=1777666813.021694897
[220928600013] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:20:16.022279842 unix_secs=1777666816.022279842
[223177513845] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 12:20:17 UTC-8 system_unix=1777666817.146556792
[226929751323] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:20:19.022861750 unix_secs=1777666819.022861750
[232930816140] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:20:22.023400082 unix_secs=1777666822.023400082
[238931986062] [
```
</details>
