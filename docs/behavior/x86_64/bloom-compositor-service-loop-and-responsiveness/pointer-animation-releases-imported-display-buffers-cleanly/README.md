# ❌ Scenario: pointer animation releases imported display buffers cleanly

> Last run: 2026-04-30 17:57:15

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 11449ms | - [📜](./01/serial.log) - |
| 2 | Then the pointer debug overlay should update after mouse movement | ❌ | 35436ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[35602599186] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[35886233625] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[35909064378] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[35946401601] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[35981921514] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[36003629046] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[36005997324] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[36048784662] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[36050821884] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[36074700024] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[36089578800] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[36090241143] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[36127084818] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[36128173752] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[36221838312] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[36309683883] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[36333418176] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[36425623080] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[36530725407] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[36581682060] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[36610596726] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[36724896318] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[36798144768] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[36998742792] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[37337491158] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1187024520 elapsed_us=593512
[37338290550] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[37419283110] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[37422853677] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[37491649932] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[37523781405] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[37534220889] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[37535824821] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[37682785899] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[37688601324] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[37701051498] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[37860538650] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[37863398496] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[37914182526] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[37918036596] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[37918287891] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[37921457112] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[37953923073] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[37980141870] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[37985972277] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[37988669334] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[38000415354] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[38008767885] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[38011486689] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[38027788524] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[38031746445] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[38040943941] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[38048650332] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[38061565047] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[38079355908] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[38099164752] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[38125188024] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[38131619691] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[38266722582] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[38286859611] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[38297837853] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[38364748785] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[38593772415] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[38789158254] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[38968034094] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[38979109554] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[38984086185] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[38989595568] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[39019530990] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[39221383839] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[39400168929] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[39423689415] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[39425390400] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[39442742163] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[39673812585] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[39777587784] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[39802277031] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1920x1080 @ 60000mHz
[39803250861] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[39803973660] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[39804738963] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[40175065755] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[40351041951] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[40352985948] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[40354308060] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[40355791410] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil nine-slice shadow renderer loaded
[40364278779] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Solarized Warm
[41046083760] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[41221769721] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[41463934182] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[41528183862] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[41700907017] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[41779608123] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=11)
[41811280137] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=12)
[41888820072] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[41891561019] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[41952336888] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Starting AHCI VFS driver
[41981746389] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[42152854755] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=96x96 hotspot=21,12
[42166300935] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[42170015382] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[42208880604] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[42211559841] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[42213271650] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1920x1080 @ 60000mHz ready
[42213410877] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: starting
[42219857823] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Mounted atapi2 at /dev/storage/atapi2
[42221541318] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Provider loop online at /dev/storage/atapi2
[42231185931] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: listening on /run/wayland-0
[42232571634] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[42242574792] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
[42265659117] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: connected to /run/wayland-0
[42296863092] [[32mINFO [0m] [clock] [CPU3] clock: connected to /run/wayland-0
[42665642448] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[42692987568] [[32mINFO [0m] [clock] [CPU3] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[42753726576] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[42756626418] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: bound wp_presentation
[43747179729] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: claimed /sys/devices/isa-0070 (handle=2)
[43780877910] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: 2026-05-01 00:59:28 = 1777597168 unix_secs
[43782135375] [[32mINFO [0m] [kernel::time] [CPU3] System clock anchored: unix_secs=1777597168, mono_ns=21890907340, offset=1777597146109092660ns
[43783146330] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: System clock anchored to 1777597168 unix_secs
[43797061341] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:59:28.006374841 unix_secs=1777597168.006374841
[44657265180] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: bristle pid=8
[44673076470] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: bristle pid=8
[44673874344] [[32mINFO [0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x47
[44679616443] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x47 (is_aux=false)
[44708611530] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: sample rate set to 60 Hz
[46745830791] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=3 rect=1920x1080 src_px=0xff40422a dst_px=0xff40422a damage=1920x1080+0,0 res_id=1
[46748560419] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 958,539 src_px=0x20181818 dst_before=0xff880604 dst_after=0xff7a0807
[46758211830] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[46764564264] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=30 frame=6
[46781038095] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[46855410492] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[46880683839] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[47012601273] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: imported dmabuf wl_buffer=111 480x320 format=0x34325241
[47038370280] [[32mINFO [0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x38
[47044437990] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0x38 (is_aux=false)
[47046527616] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: read scancode 0x38
[47066369856] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: edge detected: Down { key: LeftAlt, mods: Mods(4), repeat: false }
[47070841422] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: sent LeftAlt to bristle (pid=8)
[47209526100] [[32mINFO [0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x41
[47215260543] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0x41 (is_aux=false)
[47218580409] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: read scancode 0x41
[47220280470] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: edge detected: Down { key: F7, mods: Mods(4), repeat: false }
[47222565456] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: sent F7 to bristle (pid=8)
[47331704640] [[32mINFO [0m] [bristle] [CPU2] bristle: KeyDown received: LeftAlt
[47336888544] [[32mINFO [0m] [bristle] [CPU2] bristle: KeyDown received: F7
[47341938369] [[32mINFO [0m] [bloom::input] [CPU1] bloom: KeyDown received: LeftAlt (raw=0x00e2, mods=Mods(4), repeat=false)
[47345056473] [[32mINFO [0m] [bloom::input] [CPU1] bloom: KeyDown received: F7 (raw=0x0040, mods=Mods(4), repeat=false)
[47346639747] [[32mINFO [0m] [bloom::input] [CPU1] bloom: pointer debug overlay enabled
[47385741183] [[32mINFO [0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xc1
[47397783840] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0xc1 (is_aux=false)
[47399746053] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: read scancode 0xc1
[47401665894] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: edge detected: Up { key: F7, mods: Mods(4) }
[47404895604] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: sent F7 to bristle (pid=8)
[47441866131] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pointer debug overlay ready buffer=4 size=460x144
[47561065827] [[32mINFO [0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xb8
[47673027237] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0xb8 (is_aux=false)
[47674911537] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: read scancode 0xb8
[47676516690] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: edge detected: Up { key: LeftAlt, mods: Mods(0) }
[47685007953] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: sent LeftAlt to bristle (pid=8)
[48066755319] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 16:59:30 UTC-8 system_unix=1777597170.139479018
[49777447797] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=30 frame=6
[49794206517] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[49814985693] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:59:31.016507329 unix_secs=1777597171.016507329
[51116264892] [[32mINFO [0m] [bloom::render] [CPU1] bloom: chrome overlay ready buffer=6 size=1920x1080
[51908789625] [[32mINFO [0m] [bloom::render] [CPU1] bloom: shadow overlay ready buffer=7 size=1920x1080
[52402890903] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=3 height=30 frame=6
[52414566303] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_popup obj=23 assigned to xdg_surface=21
[52521152178] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: imported dmabuf wl_buffer=121 160x96 format=0x34325241
[53145382125] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: frame callback done object=1000 data=26563
[53148380010] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: wp_presentation_feedback.presented object=1001 tv=26.567625925 refresh_ns=16666666 seq=0
[54300363744] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 16:59:33 UTC-8 system_unix=1777597173.258913297
[55248912510] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: frame callback done object=1002 data=27616
[55250508621] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: wp_presentation_feedback.presented object=1003 tv=27.620192292 refresh_ns=16666666 seq=1
[55835823153] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:59:34.026938632 unix_secs=1777597174.026938632
[60583057788] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 16:59:36 UTC-8 system_unix=1777597176.400226857
[61855535049] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:59:37.036798606 unix_secs=1777597177.036798606
[66668523669] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 16:59:39 UTC-8 system_unix=1777597179.443090329
[67875608361] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:59:40.046846895 unix_secs=1777597180.046846895
[72886074294] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 16:59:42 UTC-8 system_unix=1777597182.551878858
[73896134136] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:59:43.057082920 unix_secs=1777597183.057082920
[79073616963] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 16:59:45 UTC-8 system_unix=1777597185.645472834
[79916322288] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:59:46.067177442 unix_secs=1777597186.067177442
[85321072122] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 16:59:48 UTC-8 system_unix=1777597188.769389636
[85936335540] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:59:49.077199792 unix_secs=1777597189.077199792
[91341069930] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 16:59:51 UTC-8 system_unix=1777597191.779377617
[91956519666] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:59:52.087297399 unix_secs=1777597192.087297399
[97491459978] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 16:59:54 UTC-8 system_unix=1777597194.854447769
[97976660991] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:59:55.097391162 unix_secs=1777597195.097391162
[103709726835] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 16:59:57 UTC-8 system_unix=1777597197.963610320
[103997042127] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:59:58.107556848 unix_secs=1777597198.107556848
[109894577892] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 17:00:01 UTC-8 system_unix=1777597201.055958282
[110017262883] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 01:00:01.117674783 unix_secs=1777597201.117674783
[116037536670] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 01:00:04.127797090 unix_secs=1777597204.127797090
[116046263025] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 17:00:04 UTC-8 system_unix=1777597204.131800683
[122057807223] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 01:00:07.137938571 unix_secs=1777597207.137938571
[122431966965] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 17:00:07 UTC-8 system_unix=1777597207.324870222
[128077962111] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 01:00:10.148023291 unix_secs=1777597210.148023291
[128613450867] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 17:00:10 UTC-8 system_unix=1777597210.415571385
[134098378425] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 01:00:13.158209916 unix_secs=1777597213.158209916
[134835803982] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 17:00:13 UTC-8 system_unix=1777597213.526532750
[140118337920] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 01:00:16.168209150 unix_secs=1777597216.168209150
[140884439586] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 17:00:16 UTC-8 system_unix=1777597216.551082872
[146138616921] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 01:00:19.178352247 unix_secs=1777597219.178352247
[146906001462] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 17:00:19 UTC-8 system_unix=1777597219.561750356
[152159372211] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 01:00:22.188723028 unix_secs=1777597222.188723028
[152958157308] [[3
```
</details>
