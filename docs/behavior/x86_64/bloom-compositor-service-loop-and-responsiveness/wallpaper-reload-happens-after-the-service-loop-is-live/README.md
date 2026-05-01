# ✅ Scenario: wallpaper reload happens after the service loop is live

> Last run: 2026-05-01 12:08:41

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 10741ms | - [📜](./01/serial.log) - |
| 2 | Then the serial output should contain "bloom: service loop started" within 60s | ✅ | 1023ms | - [📜](./02/serial.log) - |
| 3 | When I wait for the shell prompt | ✅ | 158ms | - [📜](./03/serial.log) - |
| 4 | And I type "echo /share/wallpapers/flower.png > /session/desktop/wallpaper" on the serial console | ✅ | 1017ms | - [📜](./04/serial.log) - |
| 5 | Then the serial output should contain "bloom: reacting to wallpaper change" within 60s | ✅ | 409ms | - [📜](./05/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[33583736208] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[33826439064] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[33845583387] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[33875206068] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[33905404797] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[33924443487] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[33926129655] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[33963211359] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[33964793775] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[33983860152] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[33997675041] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[33998255742] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[34032509115] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[34033523403] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[34118770356] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[34197824067] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[34219272582] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[34289642112] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[34387410948] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[34412932125] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[34435086477] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[34528720941] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[34574077263] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[34699436343] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[34935943713] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=882833853 elapsed_us=441416
[34936709511] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[35010412701] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[35013708312] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[35049310263] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[35086981116] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[35095956720] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[35097741063] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[35190856305] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[35194571082] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[35197566855] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[35362387005] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[35364054264] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[35397184647] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[35399438415] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[35400074028] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[35401257012] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[35423301408] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[35434820256] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[35438019738] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[35439705213] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[35446079229] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[35478252744] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[35480141235] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[35491471257] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[35493947511] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[35496729939] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[35499256584] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[35503646541] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[35508352737] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[35518135026] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[35562642819] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[35568613509] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[35654269596] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[35664562725] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[35674553244] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: cursor queue (queue 1) configured
[35675692800] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[35676373920] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: composition paths: gpu_opaque_copy=enabled cpu_alpha_blend=enabled
[35686064304] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: host scanout 1280x800 enabled=true (bootfb was 1920x1080)
[35716584816] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[35876827536] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[35931248463] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor DMA buffer ready (16 pages at phys=0x2723000)
[35992783332] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[36000911298] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[36005309109] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[36007018674] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[36038637393] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[36198834738] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[36371475624] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[36385580913] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[36471142620] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[36472255281] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[36610314840] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[36698089560] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[36722228004] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1280x800 @ 60000mHz
[36723304398] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[36724128177] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[36724817778] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports GPU blit (hardware transfer/flush)
[36725507115] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports direct scanout (zero-copy path to display)
[36726182064] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports partial flush (damage regions)
[36726804972] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports resource cache (pre-allocated buffer pool)
[36727495563] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[37035036522] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[37036180368] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[37036881486] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[37044399810] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Facet Frame
[37343711493] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hwrng' name='hwrng' class=Other kind='dev.rng.HwRng' start='_RNvCs7wlZbVUrQWf_5hwrng20thingos_driver_start'
[37494354183] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[37645765608] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[37798706550] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[37960916928] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[38119458333] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[38244949380] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[38337189000] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=48x48 hotspot=11,6
[38346496254] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[38348478927] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[38392404006] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[38394295005] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[38395232568] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1280x800 @ 60000mHz ready
[38397003018] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: starting
[38414375175] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: listening on /run/wayland-0
[38415348972] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[38477746890] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[38637869688] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[38667879591] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=1 rect=1280x800 src_px=0xff0b0a10 dst_px=0xff0b0a10 damage=1280x800+0,0 res_id=1 gpu_planes=0 cpu_planes=2
[38669338290] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 638,399 src_px=0x10202020 dst_before=0xff0b0a10 dst_after=0xff0c0b11
[38720210529] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=12)
[38733491478] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: connected to /run/wayland-0
[38734870548] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[38744460810] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=13)
[38822129940] [[32mINFO [0m] [clock] [CPU1] clock: running at low scheduler priority tid=13
[38825556660] [[32mINFO [0m] [clock] [CPU1] clock: connected to /run/wayland-0
[39055059549] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[39078203703] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[39087140994] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[39098869227] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[39100350432] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: bound wp_presentation
[39105643236] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: wl_region smoke test: create+add+set_opaque+destroy
[39178202514] [[32mINFO [0m] [clock] [CPU1] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[39188345988] [[32mINFO [0m] [clock] [CPU1] clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf
[39197125968] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/xhci' name='xhci' class=Block kind='dev.usb.Xhci' start='thingos_driver_start_safe'
[39198905823] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 15 driver(s) found
e[39220599528] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=28 frame=6
c[39238827672] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[39298509789] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Starting AHCI VFS driver
[39341513442] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: imported dmabuf wl_buffer=111 480x320 format=0x34325241
[39347159247] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[39369952479] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Mounted atapi2 at /dev/storage/atapi2
[39371476089] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Provider loop online at /dev/storage/atapi2
[39400996767] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: bristle pid=8
[39435293436] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: claimed /sys/devices/isa-0070 (handle=2)
[39461891568] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: bristle pid=8
[39512395230] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: sample rate set to 60 Hz
ho /share/w[39846046152] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: 2026-05-01 19:13:32 = 1777662812 unix_secs
[39847331271] [[32mINFO [0m] [kernel::time] [CPU3] System clock anchored: unix_secs=1777662812, mono_ns=19923492319, offset=1777662792076507681ns
[39848258769] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: System clock anchored to 1777662812 unix_secs
[39856491972] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:13:32.003969290 unix_secs=1777662812.003969290
all[40013911080] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=28 frame=6
[40023476823] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[40027178796] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
papers/flower.png > /session/desktop/wallpaper
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[43526720424] [
```
</details>
