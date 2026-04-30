# ❌ Scenario: displayed Wayland windows are visible through the session filesystem

> Last run: 2026-04-30 15:03:08

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Then the Wayland hello client should be visible | ❌ | 32852ms | - [📜](./01/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[32149050450] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[32393287311] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[32413595478] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[32443351941] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[32474976600] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[32493672189] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[32495227842] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[32530257540] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[32531676342] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[32550954150] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[32564452041] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[32565033138] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[32599198698] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[32600073858] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[32691193755] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[32775625971] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[32796923181] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[32879254716] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[32980945437] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[33006372894] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[33029827281] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[33127090161] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[33176538516] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[33307937487] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[33555435804] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=935496276 elapsed_us=467748
[33556156722] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[33632123382] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[33635542446] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[33699092295] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[33729778995] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[33740492907] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[33742002888] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[33829874199] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[33833041242] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[33839375724] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[34011814683] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[34013440197] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[34044519597] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[34046964006] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[34048522992] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[34048612257] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[34070037705] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[34080722445] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[34083814644] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[34085249682] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[34091521893] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[34095135030] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[34096881324] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[34111714758] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[34114211208] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[34118658948] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[34123100319] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[34130052297] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[34140445713] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[34151602617] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[34175932758] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[34182768774] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[34272425220] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[34281568893] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[34288009833] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[34337404101] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[34495619664] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[34653096885] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[34817048541] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[34844761149] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[34854924159] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[34860211188] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[34860447072] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[34984901721] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[35138325651] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[35265896589] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[35303062377] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[35303921994] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[35318468658] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[35534750064] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[35557410867] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1920x1080 @ 60000mHz
[35558289591] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[35558947908] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[35559560553] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[35854435221] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[35855372223] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[35856114921] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[35863430460] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Solarized Warm
[36259026441] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[36409767636] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[36557966016] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[36727920999] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[36925870509] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=48x48 hotspot=24,24
[36935535186] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[36937436217] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[36981037665] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[36982546722] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[36983220417] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1920x1080 @ 60000mHz ready
[36985122438] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: starting
[36996306501] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[37002208188] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: listening on /run/wayland-0
[37003291116] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[37156505859] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[37255019637] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=1 rect=1920x1080 src_px=0xffccccff dst_px=0xffccccff damage=1920x1080+0,0 res_id=1
[37257131637] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 945,521 src_px=0x10202020 dst_before=0xffccccff dst_after=0xffc1c1f1
[37329422262] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[37336662924] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
[37580701455] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=12)
[37596054969] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: connected to /run/wayland-0
[37602925767] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=13)
[37738622295] [[32mINFO [0m] [clock] [CPU1] clock: connected to /run/wayland-0
[37951879779] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[37979435274] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[38120317521] [[32mINFO [0m] [clock] [CPU1] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[39661530876] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[39663128538] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[39717917217] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Starting AHCI VFS driver
[39751552137] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[39772912179] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Mounted atapi2 at /dev/storage/atapi2
[39773667219] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Provider loop online at /dev/storage/atapi2
[39787563618] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: claimed /sys/devices/isa-0070 (handle=2)
[39798420189] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: bristle pid=8
[39820090497] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: 2026-04-30 22:04:37 = 1777586677 unix_secs
[39821378982] [[32mINFO [0m] [kernel::time] [CPU3] System clock anchored: unix_secs=1777586677, mono_ns=19910542377, offset=1777586657089457623ns
[39822184017] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: System clock anchored to 1777586677 unix_secs
[39833181531] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:04:37.004645723 unix_secs=1777586677.004645723
[40042311045] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: bristle pid=8
[40076692854] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: fallback sample rate 40 Hz also failed; continuing with device default
[41399540457] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=30 frame=6
[41409163653] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[41455354149] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[41461063116] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[41736001065] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 14:04:37 UTC-8 system_unix=1777586677.955162263
[41751928119] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=30 frame=6
[41759191452] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[42875000025] [[32mINFO [0m] [bloom::render] [CPU1] bloom: chrome overlay ready buffer=5 size=1920x1080
[43789670925] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=3 height=30 frame=6
[43800089520] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_popup obj=23 assigned to xdg_surface=21
[45859317504] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:04:40.019017408 unix_secs=1777586680.019017408
[47744762043] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 14:04:40 UTC-8 system_unix=1777586680.961618402
[51888039324] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:04:43.033423198 unix_secs=1777586683.033423198
[54033515019] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 14:04:44 UTC-8 system_unix=1777586684.105904602
[57916847472] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:04:46.047822635 unix_secs=1777586686.047822635
[60294540438] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 14:04:47 UTC-8 system_unix=1777586687.236498970
[63945461151] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:04:49.062139045 unix_secs=1777586689.062139045
[66553663770] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 14:04:50 UTC-8 system_unix=1777586690.365912285
[69974297910] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:04:52.076555015 unix_secs=1777586692.076555015
[72882833793] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 14:04:53 UTC-8 system_unix=1777586693.530671289
[76003724478] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:04:55.091253004 unix_secs=1777586695.091253004
[79170500310] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 14:04:56 UTC-8 system_unix=1777586696.674341709
[82031596680] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:04:58.105215026 unix_secs=1777586698.105215026
[85430943378] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 14:04:59 UTC-8 system_unix=1777586699.804720240
[88060499373] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:05:01.119652529 unix_secs=1777586701.119652529
[91691302428] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 14:05:02 UTC-8 system_unix=1777586702.934736481
[94089088764] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:05:04.133967784 unix_secs=1777586704.133967784
[97982325309] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 14:05:06 UTC-8 system_unix=1777586706.080345932
[100118124282] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:05:07.148451520 unix_secs=1777586707.148451520
[104243685480] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 14:05:09 UTC-8 system_unix=1777586709.210901739
[106146719613] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:05:10.162762963 unix_secs=1777586710.162762963
[110501012622] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 14:05:12 UTC-8 system_unix=1777586712.339734353
[112175635473] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:05:13.177202000 unix_secs=1777586713.177202000
[116792371773] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 14:05:15 UTC-8 system_unix=1777586715.485383205
[118204127679] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:05:16.191477556 unix_secs=1777586716.191477556
[123118515927] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 14:05:18 UTC-8 system_unix=1777586718.648495047
[124232984271] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:05:19.205899318 unix_secs=1777586719.205899318
[129379290645] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 14:05:21 UTC-8 system_unix=1777586721.778680595
[130261870728] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:05:22.220326657 unix_secs=1777586722.220326657
[135640324611] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 14:05:24 UTC-8 system_unix=1777586724.908998852
[136309298070] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:05:25.244050921 unix_secs=1777586725.244050921
[141928699308] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 14:05:28 UTC-8 system_unix=1777586728.053568588
[142319279685] [[32mIN
```
</details>
