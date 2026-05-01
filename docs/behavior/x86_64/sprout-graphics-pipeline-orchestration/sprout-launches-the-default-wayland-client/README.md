# ❌ Scenario: sprout launches the default Wayland client

> Last run: 2026-05-01 14:19:59

## Steps

| # | Step | Result | Duration | Before | After | Artifacts |
|---|------|--------|----------|--------|-------|-----------|
| 1 | Given the machine is booted | ✅ | 11795ms | <a href="./01/before.png"><img src="./01/before.png" width="120" /></a> | <a href="./01/after.png"><img src="./01/after.png" width="120" /></a> | [📜](./01/serial.log) |
| 2 | Then the serial output should contain "wayland-server: listening on /run/wayland-0" within 180s | ✅ | 931ms | <a href="./02/before.png"><img src="./02/before.png" width="120" /></a> | <a href="./02/after.png"><img src="./02/after.png" width="120" /></a> | [📜](./02/serial.log) |
| 3 | And the serial output should contain "SPROUT: Spawned wayland_hello" within 180s | ✅ | 121ms | <a href="./03/before.png"><img src="./03/before.png" width="120" /></a> | <a href="./03/after.png"><img src="./03/after.png" width="120" /></a> | [📜](./03/serial.log) |
| 4 | And the serial output should contain "wayland_hello: connected to /run/wayland-0" within 180s | ✅ | 128ms | <a href="./04/before.png"><img src="./04/before.png" width="120" /></a> | <a href="./04/after.png"><img src="./04/after.png" width="120" /></a> | [📜](./04/serial.log) |
| 5 | And the serial output should contain "wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf" within 180s | ✅ | 130ms | <a href="./05/before.png"><img src="./05/before.png" width="120" /></a> | <a href="./05/after.png"><img src="./05/after.png" width="120" /></a> |  |
| 6 | And the serial output should contain "wayland-server: xdg_surface obj=" within 180s | ❌ | 181417ms | <a href="./06/before.png"><img src="./06/before.png" width="120" /></a> | - | [📜](./06/serial.log) [timeout.png](./06/timeout.png) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[35834270505] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[36178051239] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[36208793445] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[36266994555] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[36320954274] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[36352084098] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[36355533126] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[36421090332] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[36424355022] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[36453796269] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[36478821621] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[36480298008] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[36540657285] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[36542705661] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[36683042913] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[36769230234] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[36794149227] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[36869781729] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[36976899069] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[37007317578] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[37031510439] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[37136287815] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[37189124742] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[37333016820] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[37592545881] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1019453886 elapsed_us=509726
[37593430644] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[37758959370] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[37764185448] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[37817336304] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[37874926254] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[37885842819] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[37887646368] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[38002775382] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[38007973707] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[38016340164] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[38177943540] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[38179716333] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[38215865094] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[38218546080] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[38220128628] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[38221192086] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[38254738599] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[38268007041] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[38273094717] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[38275562127] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[38284814568] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[38291927487] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[38295554715] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[38306850153] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[38309579088] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[38315192916] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[38319977817] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[38327954874] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[38342431182] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[38353916271] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[38385456021] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[38394093705] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[38502893649] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[38514295875] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[38525342163] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: cursor queue (queue 1) configured
[38526553692] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[38527542933] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: composition paths: gpu_opaque_copy=enabled cpu_alpha_blend=enabled
[38538794976] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: host scanout 1280x800 enabled=true (bootfb was 1920x1080)
[38568435708] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[38758609329] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[38800731321] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor DMA buffer ready (16 pages at phys=0x2723000)
[38877093057] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[38886743280] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[38893973679] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[38901171573] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[38934008454] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[39121421460] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[39315126807] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[39338906970] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[39340079229] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[39360945921] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[39610080609] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[39634884960] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1280x800 @ 60000mHz
[39636223902] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[39637049298] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[39637746654] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports GPU blit (hardware transfer/flush)
[39638420151] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports direct scanout (zero-copy path to display)
[39639141729] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports partial flush (damage regions)
[39639782754] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports resource cache (pre-allocated buffer pool)
[39640665471] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[39968192319] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[40132719660] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hwrng' name='hwrng' class=Other kind='dev.rng.HwRng' start='_RNvCs7wlZbVUrQWf_5hwrng20thingos_driver_start'
[40305893694] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[40412881575] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[40414219263] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[40415044560] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[40424029998] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Facet Frame
[40896493242] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[41467225569] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[41544147777] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=11)
[41566487721] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=12)
[41586467208] [[32mINFO [0m] [clock] [CPU3] clock: running at low scheduler priority tid=12
[41630843397] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[41774351256] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[41937249189] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[42034198404] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=48x48 hotspot=11,6
[42046564461] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[42049281417] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[42083640852] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[42085692627] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[42086740905] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1280x800 @ 60000mHz ready
[42094424658] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: starting
[42111099888] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: listening on /run/wayland-0
[42111938121] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[42215170404] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: connected to /run/wayland-0
[42224595765] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[42243894594] [[32mINFO [0m] [clock] [CPU3] clock: connected to /run/wayland-0
[42413398797] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[42522785019] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[42542928945] [[32mINFO [0m] [clock] [CPU3] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[42551962464] [[32mINFO [0m] [clock] [CPU3] clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf
[42615923460] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/xhci' name='xhci' class=Block kind='dev.usb.Xhci' start='thingos_driver_start_safe'
[42616199175] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[42618842706] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 15 driver(s) found
[42619167822] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: bound wp_presentation
[42626497914] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: wl_region smoke test: create+add+set_opaque+destroy
[42716636292] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Starting AHCI VFS driver
[42726067032] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=1 rect=1280x800 src_px=0xff0b0a10 dst_px=0xff0b0a10 damage=1280x800+0,0 res_id=1 gpu_planes=0 cpu_planes=2
[42727670370] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 638,399 src_px=0x10202020 dst_before=0xff0b0a10 dst_after=0xff0c0b11
[42757833987] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[42762903513] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[42776517762] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Mounted atapi2 at /dev/storage/atapi2
[42777556041] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Provider loop online at /dev/storage/atapi2
[42814256265] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=28 frame=6
[42825181443] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: claimed /sys/devices/isa-0070 (handle=2)
[42831356469] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: bristle pid=8
[42850692753] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[42861121974] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: 2026-05-01 21:20:24 = 1777670424 unix_secs
[42862498701] [[32mINFO [0m] [kernel::time] [CPU3] System clock anchored: unix_secs=1777670424, mono_ns=21431095422, offset=1777670402568904578ns
[42873296895] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[42875734407] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: System clock anchored to 1777670424 unix_secs
[42905749557] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:20:24.020801385 unix_secs=1777670424.020801385
[42912891747] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[42954019119] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: imported dmabuf wl_buffer=111 480x320 format=0x34325241
[43053643875] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: bristle pid=8
[43478378757] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=28 frame=6
[43492456689] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[43568371902] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
[44809775472] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: sample rate set to 60 Hz
[46698465693] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=3 height=28 frame=6
[46928248785] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: xdg_popup obj=23 assigned to xdg_surface=21
[46989412965] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: imported dmabuf wl_buffer=121 160x96 format=0x34325241
[47383747026] [[32mINFO [0m] [bloom::render] [CPU1] bloom: flat window overlays ready size=1280x800
[48919081662] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:20:27.028350963 unix_secs=1777670427.028350963
[49731322773] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: frame callback done object=1000 data=24840
[49735471038] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: wp_presentation_feedback.presented object=1001 tv=24.845365330 refresh_ns=16666666 seq=0
[50252854806] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: frame callback done object=1002 data=25108
[50257468239] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: wp_presentation_feedback.presented object=1003 tv=25.121903400 refresh_ns=16666666 seq=1
[54944626473] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:20:30.041118765 unix_secs=1777670430.041118765
[60967979886] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:20:33.052855597 unix_secs=1777670433.052855597
[66992561130] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:20:36.065135428 unix_secs=1777670436.065135428
[73017031164] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:20:39.077373481 unix_secs=1777670439.077373481
[74355813213] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-05-01 13:20:39 UTC-8 system_unix=1777670439.746108152
[79041942672] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:20:42.089786385 unix_secs=1777670442.089786385
[85066120557] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:20:45.101908476 unix_secs=1777670445.101908476
[91090552146] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:20:48.114134071 unix_secs=1777670448.114134071
[97115202756] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:20:51.126459459 unix_secs=1777670451.126459459
[103139650581] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:20:54.138684196 unix_secs=1777670454.138684196
[109164640266] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:20:57.151124820 unix_secs=1777670457.151124820
[115188751425] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:21:00.163221336 unix_secs=1777670460.163221336
[121213268880] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:21:03.175494121 unix_secs=1777670463.175494121
[127237825341] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:21:06.187760653 unix_secs=1777670466.187760653
[133262445954] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:21:09.200058090 unix_secs=1777670469.200058090
[139286934105] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:21:12.212304162 unix_secs=1777670472.212304162
[145311381864] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:21:15.224547412 unix_secs=1777670475.224547412
[148903742361] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-05-01 13:21:17 UTC-8 system_unix=1777670477.020239112
[151336117944] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:21:18.236887419 unix_secs=1777670478.236887419
[157360534782] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:21:21.249115440 unix_secs=1777670481.249115440
[163385236806] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:21:24.261453829 unix_secs=1777670484.261453829
[169409512767] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:21:27.273613771 unix_secs=1777670487.273613771
[175434037581] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:21:30.285860883 unix_secs=1777670490.285860883
[181458598728] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:21:33.298138057 unix_secs=1777670493.298138057
[187483246797] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:21:36.310455657 unix_secs=1777670496.310455657
[193507706964] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:21:39.322691713 unix_secs=1777670499.322691713
[199532222076] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:21:42.334947075 unix_secs=1777670502.334947075
[205556704782] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:21:45.347206693 unix_secs=1777670505.347206693
[211581329883] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:21:48.359515828 unix_secs=1777670508.359515828
[217605742761] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:21:51.371726409 unix_secs=1777670511.371726409
[223513681908] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-05-01 13:21:54 UTC-8 system_unix=1777670514.325556260
[223630490787] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:21:54.384100686 unix_secs=1777670514.384100686
[229655309367] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:21:57.396473824 unix_secs=1777670517.396473824
[235679404026] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:22:00.408553378 unix_secs=1777670520.408553378
[241704216237] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:22:03.420928266 unix_secs=1777670523.420928266
[247728750852] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:22:06.433177770 unix_secs=1777670526.433177770
[253753636158] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:22:09.445623492 unix_secs=1777670529.445623492
[259777696959] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:22:12.457701495 unix_secs=1777670532.457701495
[265802079807] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:22:15.469891566 unix_secs=1777670535.469891566
[271826679993] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:22:18.482170407 unix_secs=1777670538.482170407
[277851040005] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:22:21.494380839 unix_secs=1777670541.494380839
[283875582606] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:22:24.506638593 unix_secs=1777670544.506638593
[289900247175] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:22:27.518977758 unix_secs=1777670547.518977758
[295924527459] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:22:30.531126562 unix_secs=1777670550.531126562
[298089299211] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-05-01 13:22:31 UTC-8 system_unix=1777670551.613341828
[301949162526] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:22:33.543424428 unix_secs=1777670553.543424428
[307973871645] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:22:36.555762339 unix_secs=1777670556.555762339
[313998273864] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:22:39.567983479 unix_secs=1777670559.567983479
[320022710700] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:22:42.580217457 unix_secs=1777670562.580217457
[326047381308] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:22:45.592535535 unix_secs=1777670565.592535535
[332071780920] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:22:48.604739053 unix_secs=1777670568.604739053
[338096303853] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:22:51.617011558 unix_secs=1777670571.617011558
[344120906811] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:22:54.629302131 unix_secs=1777670574.629302131
[350145437202] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:22:57.641564769 unix_secs=1777670577.641564769
[356170013133] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:23:00.653855242 unix_secs=1777670580.653855242
[362194356513] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:23:03.666035595 unix_secs=1777670583.666035595
[368218970229] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:23:06.678320805 unix_secs=1777670586.678320805
[372604144638] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-05-01 13:23:08 UTC-8 system_unix=1777670588.870559859
[374243469633] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:23:09.690584350 unix_secs=1777670589.690584350
[380268076353] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:23:12.702879708 unix_secs=1777670592.702879708
[386292532296] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:23:15.715115913 unix_secs=1777670595.715115913
[392317235805] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:23:18.727441185 unix_secs=1777670598.727441185
[398341497510] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:23:21.739616769 unix_secs=1777670601.739616769
[404366251905] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:23:24.751970701 unix_secs=1777670604.751970701
[410390696958] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:23:27.764208094 unix_secs=1777670607.764208094
[416415275892] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:23:30.776483206 unix_secs=1777670610.776483206
[422439694677] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:23:33.788699281 unix_secs=1777670613.788699281
[428464804977] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:23:36.801064071 unix_secs=1777670616.801064071
[434489056287] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:23:39.813363108 unix_secs=1777670619.813363108
[440513395146] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:23:42.825548427 unix_secs=1777670622.825548427
[446537925405] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:23:45.837819282 unix_secs=1777670625.837819282
[447117270897] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-05-01 13:23:46 UTC-8 system_unix=1777670626.127311072
[452562565719] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:23:48.850113039 unix_secs=1777670628.850113039
[458587253256] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:23:51.862453408 unix_secs=1777670631.862453408
[464611445331] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:23:54.874581769 unix_secs=1777670634.874581769
[470636223453] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:23:57.886954380 unix_secs=1777670637.886954380
[476660742624] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:00.899173474 unix_secs=1777670640.899173474
[482685203022] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:03.911440633 unix_secs=1777670643.911440633
[488709785289] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:06.923734225 unix_secs=1777670646.923734225
[494734230672] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:09.935953683 unix_secs=1777670649.935953683
[500758640481] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:12.948180268 unix_secs=1777670652.948180268
[506783066460] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:15.960398818 unix_secs=1777670655.960398818
[512807658825] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:18.972682494 unix_secs=1777670658.972682494
[518832263994] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:21.984989203 unix_secs=1777670661.984989203
[521826180909] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-05-01 13:24:23 UTC-8 system_unix=1777670663.481743259
[524856776037] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:24.997246677 unix_secs=1777670664.997246677
[530881276992] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:28.009498623 unix_secs=1777670668.009498623
[536905747455] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:31.021736907 unix_secs=1777670671.021736907
[542930391036] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:34.034041042 unix_secs=1777670674.034041042
[548955000198] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:37.046343346 unix_secs=1777670677.046343346
[554979509271] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:40.058603806 unix_secs=1777670680.058603806
[561004125990] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:43.070893009 unix_secs=1777670683.070893009
[567034922916] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:46.086295234 unix_secs=1777670686.086295234
[573053161527] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:49.095408451 unix_secs=1777670689.095408451
[579077588430] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:52.107646389 unix_secs=1777670692.107646389
[585102087801] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:55.119904737 unix_secs=1777670695.119904737
[591126827940] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:58.132221990 unix_secs=1777670698.132221990
[596333321991] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-05-01 13:25:00 UTC-8 system_unix=1777670700.735361765
[597151121193] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:25:01.144423578 unix_secs=1777670701.144423578
[603175708938] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:25:04.156704135 unix_secs=1777670704.156704135
[609200096967] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:25:07.168900327 unix_secs=1777670707.168900327
[615224761866] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:25:10.181224972 unix_secs=1777670710.181224972
[621249210153] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:25:13.193456178 unix_secs=1777670713.193456178
[627273835452] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:25:16.205774635 unix_secs=1777670716.205774635
[633298411812] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:25:19.218059037 unix_secs=1777670719.218059037
[639322911183] [
```
</details>
