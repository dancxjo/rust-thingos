# ❌ Scenario: bloom configures the default Solarized Warm theme

> Last run: 2026-05-01 13:23:52

## Steps

| # | Step | Result | Duration | Before | After | Artifacts |
|---|------|--------|----------|--------|-------|-----------|
| 1 | Given the machine is booted | ✅ | 10948ms | <a href="./01/before.png"><img src="./01/before.png" width="120" /></a> | <a href="./01/after.png"><img src="./01/after.png" width="120" /></a> | [📜](./01/serial.log) |
| 2 | Then the serial output should contain "bloom: initial theme configured Solarized Warm" within 60s | ❌ | 61146ms | <a href="./02/before.png"><img src="./02/before.png" width="120" /></a> | - | [📜](./02/serial.log) [timeout.png](./02/timeout.png) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[33402991743] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[33651956184] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[33671353155] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[33701052957] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[33731689629] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[33750749505] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[33752394555] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[33788746332] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[33790362210] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[33808948569] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[33822387390] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[33822944166] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[33857050887] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[33857991222] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[33943049151] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[34020674424] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[34041310413] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[34111268169] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[34208590416] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[34233059586] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[34258551327] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[34355705703] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[34403700375] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[34531681998] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[34772657139] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=895440513 elapsed_us=447720
[34773431517] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[34848678447] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[34851937428] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[34887567330] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[34923661344] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[34932140628] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[34933570518] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[35034079080] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[35037224739] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[35043300138] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[35248720749] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[35250423648] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[35283166941] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[35285432490] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[35286827532] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[35287448097] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[35311709202] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[35319976692] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[35322975897] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[35324490135] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[35330645988] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[35333976117] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[35336008521] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[35349946368] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[35352619236] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[35357232438] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[35361967443] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[35382299469] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[35394087663] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[35405993469] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[35420010912] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[35425315827] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[35510451537] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[35520020712] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[35530493130] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: cursor queue (queue 1) configured
[35531537514] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[35532306282] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: composition paths: gpu_opaque_copy=enabled cpu_alpha_blend=enabled
[35541369402] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: host scanout 1280x800 enabled=true (bootfb was 1920x1080)
[35613844959] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[35784232869] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[35784888546] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor DMA buffer ready (16 pages at phys=0x2723000)
[35861443761] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[35869429431] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[35873836779] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[35876686428] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[35944742361] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[36121192371] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[36314882043] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[36316684305] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[36331599117] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[36332663433] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[36568495524] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[36588178968] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1280x800 @ 60000mHz
[36589429437] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[36590747259] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[36592090986] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports GPU blit (hardware transfer/flush)
[36593446791] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports direct scanout (zero-copy path to display)
[36594745902] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports partial flush (damage regions)
[36595815630] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports resource cache (pre-allocated buffer pool)
[36597216975] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[36904430442] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[36905539902] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[36906245079] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[36913439244] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Facet Frame
[37146803166] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[37292387121] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hwrng' name='hwrng' class=Other kind='dev.rng.HwRng' start='_RNvCs7wlZbVUrQWf_5hwrng20thingos_driver_start'
[37462394079] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[37480980636] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[37630420641] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[37894999626] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=48x48 hotspot=11,6
[37905013476] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[37907924406] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[37950888591] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[37953635181] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[37954612443] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1280x800 @ 60000mHz ready
[37957489944] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: starting
[37970120166] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[37978658454] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: listening on /run/wayland-0
[37979625156] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[38123175024] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[38217391047] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=1 rect=1280x800 src_px=0xff0b0a10 dst_px=0xff0b0a10 damage=1280x800+0,0 res_id=1 gpu_planes=0 cpu_planes=2
[38218810278] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 638,399 src_px=0x10202020 dst_before=0xff0b0a10 dst_after=0xff0c0b11
[38279403294] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[38284699629] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[38452122588] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[38551558089] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[38562662160] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[38631395484] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[38641762368] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=12)
[38652387411] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: connected to /run/wayland-0
[38663867748] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=13)
[38725344405] [[32mINFO [0m] [clock] [CPU1] clock: running at low scheduler priority tid=13
[38728708326] [[32mINFO [0m] [clock] [CPU1] clock: connected to /run/wayland-0
[38960130165] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[38998513752] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[39000769335] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: bound wp_presentation
[39006917103] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: wl_region smoke test: create+add+set_opaque+destroy
[39079419885] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=28 frame=6
[39093310509] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[39103760124] [[32mINFO [0m] [clock] [CPU1] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[39104793882] [[32mINFO [0m] [clock] [CPU1] clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf
[39109450842] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/xhci' name='xhci' class=Block kind='dev.usb.Xhci' start='thingos_driver_start_safe'
[39111049362] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 15 driver(s) found
[39165651096] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Starting AHCI VFS driver
[39196032777] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: imported dmabuf wl_buffer=111 480x320 format=0x34325241
[39205197240] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[39225608631] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Mounted atapi2 at /dev/storage/atapi2
[39227053206] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Provider loop online at /dev/storage/atapi2
[39251419911] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: bristle pid=8
[39311181558] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: bristle pid=8
[39314502447] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: claimed /sys/devices/isa-0070 (handle=2)
[39358549065] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: sample rate set to 60 Hz
[39647772054] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: 2026-05-01 20:30:27 = 1777667427 unix_secs
[39649541613] [[32mINFO [0m] [kernel::time] [CPU3] System clock anchored: unix_secs=1777667427, mono_ns=19824603315, offset=1777667407175396685ns
[39650722023] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: System clock anchored to 1777667427 unix_secs
[39674524494] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:30:27.011813455 unix_secs=1777667427.011813455
[40052380698] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=28 frame=6
[40060096725] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
[40075281972] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[44063839182] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=3 height=28 frame=6
[44082263709] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_popup obj=23 assigned to xdg_surface=21
[44105926161] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: imported dmabuf wl_buffer=121 160x96 format=0x34325241
[44628987117] [[32mINFO [0m] [bloom::render] [CPU1] bloom: flat window overlays ready size=1280x800
[45698364162] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:30:30.024446254 unix_secs=1777667430.024446254
[46147123539] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: frame callback done object=1000 data=23066
[46149297381] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: wp_presentation_feedback.presented object=1001 tv=23.070251226 refresh_ns=16666666 seq=0
[46479592731] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: frame callback done object=1002 data=23232
[46480632957] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: wp_presentation_feedback.presented object=1003 tv=23.237570647 refresh_ns=16666666 seq=1
[51723316557] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:30:33.037004473 unix_secs=1777667433.037004473
[57748534503] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:30:36.049521112 unix_secs=1777667436.049521112
[63773103405] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:30:39.061914331 unix_secs=1777667439.061914331
[69798557796] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:30:42.074606415 unix_secs=1777667442.074606415
[74606512167] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 12:30:44 UTC-8 system_unix=1777667444.477597742
[75823617177] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:30:45.087147292 unix_secs=1777667445.087147292
[81848941977] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:30:48.099794397 unix_secs=1777667448.099794397
[87873886452] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:30:51.112264786 unix_secs=1777667451.112264786
[93898844622] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:30:54.124750488 unix_secs=1777667454.124750488
[99924251295] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:30:57.137452702 unix_secs=1777667457.137452702
[105948804423] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:31:00.149744529 unix_secs=1777667460.149744529
[111973803942] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:31:03.162250591 unix_secs=1777667463.162250591
[117998961003] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:31:06.174823941 unix_secs=1777667466.174823941
[124024481625] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:31:09.187595109 unix_secs=1777667469.187595109
[130049328486] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:31:12.199983642 unix_secs=1777667472.199983642
[136074293949] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:31:15.212481141 unix_secs=1777667475.212481141
[142099433388] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:31:18.225044805 unix_secs=1777667478.225044805
[148124416704] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:31:21.237548343 unix_secs=1777667481.237548343
[149090160549] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 12:31:21 UTC-8 system_unix=1777667481.720137290
[154149438663] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:31:24.250063249 unix_secs=1777667484.250063249
[160174435641] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:31:27.262574047 unix_secs=1777667487.262574047
[166200020250] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:31:30.275326785 unix_secs=1777667490.275326785
[172224575127] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:31:33.287637355 unix_secs=1777667493.287637355
[178249736808] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:31:36.300211398 unix_secs=1777667496.300211398
[184275156153] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:31:39.312870234 unix_secs=1777667499.312870234
[190300033770] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:31:42.325360176 unix_secs=1777667502.325360176
[196324894425] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:31:45.337802664 unix_secs=1777667505.337802664
[202350270705] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:31:48.350440875 unix_secs=1777667508.350440875
[208375117500] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:31:51.362894005 unix_secs=1777667511.362894005
[214400358612] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:31:54.375533091 unix_secs=1777667514.375533091
[220425224844] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:31:57.387956769 unix_secs=1777667517.387956769
[223609364649] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 12:31:58 UTC-8 system_unix=1777667518.979836509
[226450397151] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:32:00.400545232 unix_secs=1777667520.400545232
[232475476200] [
```
</details>
