# ❌ Scenario: sprout launches the default Wayland client

> Last run: 2026-04-30 21:38:33

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 12252ms | - [📜](./01/serial.log) - |
| 2 | Then the serial output should contain "wayland-server: listening on /run/wayland-0" within 180s | ✅ | 1636ms | - [📜](./02/serial.log) - |
| 3 | And the serial output should contain "SPROUT: Spawned wayland_hello" within 180s | ✅ | 1ms | - - - |
| 4 | And the serial output should contain "wayland_hello: connected to /run/wayland-0" within 180s | ✅ | 1ms | - - - |
| 5 | And the serial output should contain "wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf" within 180s | ✅ | 103ms | - [📜](./05/serial.log) - |
| 6 | And the serial output should contain "wayland-server: xdg_surface obj=" within 180s | ❌ | 182220ms | - [📜](./06/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[38217434442] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[38535775476] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[38559911775] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[38605093428] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[38648955807] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[38670508140] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[38672739006] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[38723132019] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[38725215441] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[38752043385] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[38768549919] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[38769793986] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[38821912074] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[38823338037] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[38944130313] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[39038984589] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[39066176853] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[39148041801] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[39260615724] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[39297321492] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[39320945796] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[39429341919] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[39494406864] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[39655038951] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[39937454964] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1086366765 elapsed_us=543183
[39939694113] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[40035942540] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[40039737045] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[40092373860] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[40143205773] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[40154449896] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[40156896450] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[40282391556] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[40286185566] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[40299520008] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[40453163586] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[40455261726] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[40496499648] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[40499195517] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[40500918216] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[40505026617] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[40539160497] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[40542555108] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[40545887052] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[40547477685] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[40555817742] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[40563714708] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[40567038600] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[40602968769] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[40607173596] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[40612605396] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[40619918625] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[40632390645] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[40648417425] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[40667497299] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[40692285183] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[40698813243] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[40833090177] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[40848113295] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[40862393715] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: cursor queue (queue 1) configured
[40863836706] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[40880372610] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[41063890560] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[41263502478] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[41473967337] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[41487552645] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor DMA buffer ready (16 pages at phys=0x2b21000)
[41606708583] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[41621982732] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[41630829339] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[41637262557] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[41697626718] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[41886369261] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[42055944084] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[42091205145] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[42093130563] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[42112044942] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hwrng' name='hwrng' class=Other kind='dev.rng.HwRng' start='_RNvCs7wlZbVUrQWf_5hwrng20thingos_driver_start'
[42403774809] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[42429703272] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1920x1080 @ 60000mHz
[42431294862] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[42432615588] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[42435114975] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[42814587948] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[42816030840] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[42816879468] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[42822940545] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[42999059796] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[43164705606] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[43334279670] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[43431645906] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Facet Frame
[44141176992] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[44349560805] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[44447771478] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=11)
[44478928989] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=12)
[44561700777] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[44562670020] [[32mINFO [0m] [clock] [CPU3] clock: running at low scheduler priority tid=12
[44565020247] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 14 driver(s) found
[44594656326] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[44681108967] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Starting AHCI VFS driver
[44717650098] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[44736205305] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Mounted atapi2 at /dev/storage/atapi2
[44740007433] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Provider loop online at /dev/storage/atapi2
[45049311450] [[32mINFO [0m] [rtc_cmos] [CPU2] RTC: claimed /sys/devices/isa-0070 (handle=2)
[45054003093] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: bristle pid=8
[45091855446] [[32mINFO [0m] [rtc_cmos] [CPU2] RTC: 2026-05-01 04:39:47 = 1777610387 unix_secs
[45097312359] [[32mINFO [0m] [kernel::time] [CPU2] System clock anchored: unix_secs=1777610387, mono_ns=22548362133, offset=1777610364451637867ns
[45099074328] [[32mINFO [0m] [rtc_cmos] [CPU2] RTC: System clock anchored to 1777610387 unix_secs
[45115630428] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:47.008375416 unix_secs=1777610387.008375416
[45178044153] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=96x96 hotspot=21,12
[45188497431] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[45191281509] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[45237850680] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[45241433490] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[45242452893] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: starting
[45243011253] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1920x1080 @ 60000mHz ready
[45265414986] [[33mWARN [0m] [virtio_gpu] [CPU3] VirtioGpu: Command 0x101 failed with resp_type=0x1203
[45267039873] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: listening on /run/wayland-0
[45267385515] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor resource create failed: Command failed
[45268392642] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[45283153971] [[32mINFO [0m] [ps2_mouse] [CPU1] ps2_mouse: bristle pid=8
[45319095591] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: connected to /run/wayland-0
[45351150306] [[32mINFO [0m] [ps2_mouse] [CPU1] ps2_mouse: sample rate set to 60 Hz
[45624230058] [[32mINFO [0m] [clock] [CPU3] clock: connected to /run/wayland-0
[45764030532] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[45816641277] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[45818794791] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: bound wp_presentation
[46030530678] [[32mINFO [0m] [clock] [CPU3] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[46032159756] [[32mINFO [0m] [clock] [CPU3] clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf
[46126223352] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=1 rect=1920x1080 src_px=0xff0b0a10 dst_px=0xff0b0a10 damage=1920x1080+0,0 res_id=1
[46129026834] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 958,539 src_px=0x20181818 dst_before=0xff0b0a10 dst_after=0xff0d0c11
[46141492683] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[46147060509] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=28 frame=6
[46163471442] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[46252330212] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
[46356290607] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: imported dmabuf wl_buffer=111 480x320 format=0x34325241
[49901389131] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[49911169506] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[50390937267] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=28 frame=6
[50397554196] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[51135995988] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:50.018776887 unix_secs=1777610390.018776887
[51416652837] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=3 height=28 frame=6
[51429834885] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_popup obj=23 assigned to xdg_surface=21
[51488717412] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: imported dmabuf wl_buffer=121 160x96 format=0x34325241
[52913993979] [[32mINFO [0m] [bloom::render] [CPU1] bloom: flat window overlays ready size=1920x1080
[54864308037] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: frame callback done object=1000 data=27424
[54882915219] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: wp_presentation_feedback.presented object=1001 tv=27.427457227 refresh_ns=16666666 seq=0
[55414963857] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: frame callback done object=1002 data=27700
[55416046818] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: wp_presentation_feedback.presented object=1003 tv=27.704321700 refresh_ns=16666666 seq=1
[57160522551] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:53.031852464 unix_secs=1777610393.031852464
[63186832104] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:56.044992984 unix_secs=1777610396.044992984
[69213054801] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:59.058106098 unix_secs=1777610399.058106098
[74394399321] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 20:40:01 UTC-8 system_unix=1777610401.647887804
[75239257302] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:02.071207332 unix_secs=1777610402.071207332
[81265539267] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:05.084338910 unix_secs=1777610405.084338910
[87291882381] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:08.097505055 unix_secs=1777610408.097505055
[93317943246] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:11.110563323 unix_secs=1777610411.110563323
[99344091363] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:14.123638734 unix_secs=1777610414.123638734
[105370394283] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:17.136786333 unix_secs=1777610417.136786333
[111396576819] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:20.149875225 unix_secs=1777610420.149875225
[117422990751] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:23.163017000 unix_secs=1777610423.163017000
[123449021850] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:26.176086834 unix_secs=1777610426.176086834
[129475619328] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:29.189383082 unix_secs=1777610429.189383082
[135501586308] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:32.202371076 unix_secs=1777610432.202371076
[141527855469] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:35.215489388 unix_secs=1777610435.215489388
[147554146014] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:38.228628522 unix_secs=1777610438.228628522
[148961638023] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 20:40:38 UTC-8 system_unix=1777610438.932011824
[153580357062] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:41.241739739 unix_secs=1777610441.241739739
[159606517521] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:44.254816767 unix_secs=1777610444.254816767
[165632574888] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:47.267865020 unix_secs=1777610447.267865020
[171658710498] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:50.280948549 unix_secs=1777610450.280948549
[177685007379] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:53.294093904 unix_secs=1777610453.294093904
[183711393987] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:56.307261947 unix_secs=1777610456.307261947
[189737656020] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:59.320389828 unix_secs=1777610459.320389828
[195764075859] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:02.333580476 unix_secs=1777610462.333580476
[201790052343] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:05.346611387 unix_secs=1777610465.346611387
[207816301374] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:08.359711086 unix_secs=1777610468.359711086
[213842574924] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:11.372845403 unix_secs=1777610471.372845403
[219868834185] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:14.385970166 unix_secs=1777610474.385970166
[223231830327] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 20:41:16 UTC-8 system_unix=1777610476.067187770
[225894808458] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:17.398990319 unix_secs=1777610477.398990319
[231921321720] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:20.412211277 unix_secs=1777610480.412211277
[237947487690] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:23.425288503 unix_secs=1777610483.425288503
[243975727182] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:26.439393795 unix_secs=1777610486.439393795
[250000676640] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:29.451891558 unix_secs=1777610489.451891558
[256031483037] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:32.467301505 unix_secs=1777610492.467301505
[262052456853] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:35.477813807 unix_secs=1777610495.477813807
[268078577976] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:38.490839768 unix_secs=1777610498.490839768
[274104580761] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:41.503878879 unix_secs=1777610501.503878879
[280130921565] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:44.517028145 unix_secs=1777610504.517028145
[286157129907] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:47.530127861 unix_secs=1777610507.530127861
[292183281423] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:50.543228864 unix_secs=1777610510.543228864
[297235546674] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 20:41:53 UTC-8 system_unix=1777610513.069019758
[298209518112] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:53.556336879 unix_secs=1777610513.556336879
[304235819481] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:56.569479990 unix_secs=1777610516.569479990
[310261849293] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:59.582516330 unix_secs=1777610519.582516330
[316288093572] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:02.595639971 unix_secs=1777610522.595639971
[322314388539] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:05.608777191 unix_secs=1777610525.608777191
[328340695452] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:08.621919494 unix_secs=1777610528.621919494
[334366703385] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:11.634948062 unix_secs=1777610531.634948062
[340393029273] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:14.648105462 unix_secs=1777610534.648105462
[346419415353] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:17.661268488 unix_secs=1777610537.661268488
[352445686527] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:20.674362594 unix_secs=1777610540.674362594
[358471714425] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:23.687440002 unix_secs=1777610543.687440002
[364498093410] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:26.700612417 unix_secs=1777610546.700612417
[370524357159] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:29.713738005 unix_secs=1777610549.713738005
[371768812074] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 20:42:30 UTC-8 system_unix=1777610550.335789045
[376550637375] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:32.726873922 unix_secs=1777610552.726873922
[382576776945] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:35.739952766 unix_secs=1777610555.739952766
[388602866223] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:38.753018739 unix_secs=1777610558.753018739
[394629102714] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:41.766127695 unix_secs=1777610561.766127695
[400655262810] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:44.779220696 unix_secs=1777610564.779220696
[406681635228] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:47.792380950 unix_secs=1777610567.792380950
[412707792651] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:50.805470601 unix_secs=1777610570.805470601
[418733893545] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:53.818539792 unix_secs=1777610573.818539792
[424760695293] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:56.831789163 unix_secs=1777610576.831789163
[430786473909] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:59.844809993 unix_secs=1777610579.844809993
[436812608562] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:43:02.857894562 unix_secs=1777610582.857894562
[442838862543] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:43:05.871017559 unix_secs=1777610585.871017559
[446294045703] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 20:43:07 UTC-8 system_unix=1777610587.598407707
[448865104512] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:43:08.884131218 unix_secs=1777610588.884131218
[454891439145] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:43:11.897292215 unix_secs=1777610591.897292215
[460917609372] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:43:14.910372791 unix_secs=1777610594.910372791
[466943699343] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:43:17.923438632 unix_secs=1777610597.923438632
[472969945734] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:43:20.936565722 unix_secs=1777610600.936565722
[478996194732] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:43:23.949686063 unix_secs=1777610603.949686063
[485023751388] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:43:26.963429064 unix_secs=1777610606.963429064
[491048514561] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:43:29.975852379 unix_secs=1777610609.975852379
[497075076168] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:43:32.989083666 unix_secs=1777610612.989083666
[503101139442] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:43:36.002147099 unix_secs=1777610616.002147099
[509127257595] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:43:39.015219804 unix_secs=1777610619.015219804
[515153762475] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:43:42.028420187 unix_secs=1777610622.028420187
[520799399769] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 20:43:44 UTC-8 system_unix=1777610624.850850094
[521179877724] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:43:45.041507610 unix_secs=1777610625.041507610
[527206098738] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:43:48.054623315 unix_secs=1777610628.054623315
[533232294441] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:43:51.067709765 unix_secs=1777610631.067709765
[539258318544] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:43:54.080751285 unix_secs=1777610634.080751285
[545284692579] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:43:57.093908586 unix_secs=1777610637.093908586
[551310816309] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:44:00.106998617 unix_secs=1777610640.106998617
[557337095634] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:44:03.120128412 unix_secs=1777610643.120128412
[563363508411] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:44:06.133266177 unix_secs=1777610646.133266177
[569389681047] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:44:09.146411021 unix_secs=1777610649.146411021
[575416125768] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:44:12.159609605 unix_secs=1777610652.159609605
[581442012261] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:44:15.172596890 unix_secs=1777610655.172596890
[587468298879] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:44:18.185715795 unix_secs=1777610658.185715795
[593494402974] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:44:21.198794325 unix_secs=1777610661.198794325
[595332934812] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 20:44:22 UTC-8 system_unix=1777610662.117638372
[599520589701] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:44:24.211890560 unix_secs=1777610664.211890560
[605546842890] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:44:27.225011759 unix_secs=1777610667.225011759
[611573224086] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:44:30.238186500 unix_secs=1777610670.238186500
[617599340688] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:44:33.251254338 unix_secs=1777610673.251254338
[623625718914] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:44:36.264427743 unix_secs=1777610676.264427743
[629651756580] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:44:39.277473554 unix_secs=1777610679.277473554
[635677995777] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:44:42.290584506 unix_secs=1777610682.290584506
[641704267446] [
```
</details>
