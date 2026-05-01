# ✅ Scenario: bloom service loop processes multiple wallpaper events

> Last run: 2026-05-01 12:08:41

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 11254ms | - [📜](./01/serial.log) - |
| 2 | Then the serial output should contain "bloom: service loop started" within 60s | ✅ | 1225ms | - [📜](./02/serial.log) - |
| 3 | When I wait for the shell prompt | ✅ | 151ms | - [📜](./03/serial.log) - |
| 4 | And I type "echo /share/wallpapers/flower.png > /session/desktop/wallpaper" on the serial console | ✅ | 1014ms | - [📜](./04/serial.log) - |
| 5 | And I type "echo /share/wallpapers/clouds.bmp > /session/desktop/wallpaper" on the serial console | ✅ | 1017ms | - [📜](./05/serial.log) - |
| 6 | Then the serial output should contain "bloom: reacting to wallpaper change" within 60s | ✅ | 0ms | - - - |
| 7 | And the serial output should contain "/share/wallpapers/clouds.bmp" within 60s | ✅ | 306ms | - [📜](./07/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[35204617635] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[35458264809] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[35478158694] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[35509017687] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[35540994720] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[35560263057] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[35562059643] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[35600200449] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[35601971493] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[35621278374] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[35636548035] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[35637258096] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[35672803947] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[35673927036] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[35763635160] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[35845006098] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[35867683137] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[35940901128] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[36049629165] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[36084458850] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[36107739030] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[36214176801] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[36268203477] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[36400252482] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[36644093145] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=949822203 elapsed_us=474911
[36644899764] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[36722396502] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[36725892852] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[36763174635] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[36800186973] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[36809032260] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[36810827823] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[36910632891] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[36914008626] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[36918983178] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[1;32m
        .-.
       /   \        [1;36mTHING-OS[1;32m
      |     |       [0;36m"People, places, things."[1;32m
       \   /        
        `-'        
       /   \        v0.1  ���  
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[37075254744] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[37077716676] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[37114742610] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[37117283841] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[37118708121] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[37119432801] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[37141452447] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[37153681158] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[37156992642] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[37190142429] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[37198720878] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[37204076217] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[37206712026] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[37219269120] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[37222463124] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[37226156913] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[37229137374] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[37235821095] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[37245442509] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[37256513217] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[37299301842] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[37305944709] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[37394245911] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[37407043146] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[37417096134] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: cursor queue (queue 1) configured
[37418232324] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[37419071679] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: composition paths: gpu_opaque_copy=enabled cpu_alpha_blend=enabled
[37428272904] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: host scanout 1280x800 enabled=true (bootfb was 1920x1080)
[37446875004] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[37616529126] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[37682239617] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor DMA buffer ready (16 pages at phys=0x2723000)
[37748198697] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[37756193211] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[37760948940] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[37767635730] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[37789232481] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[37968116934] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[38157797799] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[38191632963] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[38294168946] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[38296085883] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[38509522161] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[38619072459] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[38721328173] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hwrng' name='hwrng' class=Other kind='dev.rng.HwRng' start='_RNvCs7wlZbVUrQWf_5hwrng20thingos_driver_start'
[38877000261] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[38967140520] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1280x800 @ 60000mHz
[38968269285] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[38968945257] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[38969627400] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports GPU blit (hardware transfer/flush)
[38970474576] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports direct scanout (zero-copy path to display)
[38971270404] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports partial flush (damage regions)
[38972110122] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports resource cache (pre-allocated buffer pool)
[38973690030] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[39302854305] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[39303935979] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[39304666764] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[39313603197] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Facet Frame
[39638266767] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[39794906877] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[39947281803] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[40101085167] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[40239913131] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=11)
[40269327252] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=12)
[40276637478] [[32mINFO [0m] [clock] [CPU3] clock: running at low scheduler priority tid=12
[40290240045] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[40295110482] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[40456666140] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[40761211821] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=48x48 hotspot=11,6
[40777974105] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[40781750031] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[40836591543] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: starting
[40849488306] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: listening on /run/wayland-0
[40850547804] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[40856199351] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[40858067976] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[40859213307] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1280x800 @ 60000mHz ready
[40874582034] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/xhci' name='xhci' class=Block kind='dev.usb.Xhci' start='thingos_driver_start_safe'
[40889965677] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 15 driver(s) found
[40920686202] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: connected to /run/wayland-0
[40983957729] [[32mINFO [0m] [clock] [CPU3] clock: connected to /run/wayland-0
[41012202825] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Starting AHCI VFS driver
[41052220110] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[41073980607] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: bristle pid=8
[41120870769] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: claimed /sys/devices/isa-0070 (handle=2)
[41145324066] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: 2026-05-01 19:13:17 = 1777662797 unix_secs
[41147083659] [[32mINFO [0m] [kernel::time] [CPU3] System clock anchored: unix_secs=1777662797, mono_ns=20573301111, offset=1777662776426698889ns
[41149404285] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: System clock anchored to 1777662797 unix_secs
[41180257305] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:13:17.016002178 unix_secs=1777662797.016002178
[41258333655] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: bristle pid=8
[41278222986] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Mounted atapi2 at /dev/storage/atapi2
[41279214174] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Provider loop online at /dev/storage/atapi2
[41344573545] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=1 rect=1280x800 src_px=0xff0b0a10 dst_px=0xff0b0a10 damage=1280x800+0,0 res_id=1 gpu_planes=0 cpu_planes=2
[41346046863] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 638,399 src_px=0x10202020 dst_before=0xff0b0a10 dst_after=0xff0c0b11
[41355631086] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[41390847465] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: sample rate set to 60 Hz
[41392502184] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[41403803166] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[41405213586] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: bound wp_presentation
[41409985485] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: wl_region smoke test: create+add+set_opaque+destroy
ec[41603136630] [[32mINFO [0m] [clock] [CPU3] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[41604296052] [[32mINFO [0m] [clock] [CPU3] clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf
h[41627366550] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=28 frame=6
[41638871043] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[41646533313] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[41656949235] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
o[41733266553] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: imported dmabuf wl_buffer=111 480x320 format=0x34325241
 /shar[42076760121] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=28 frame=6
e/[42119134332] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
wallp[42412974351] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
apers/flower.png > /session/desktop/wallpaper
e[?25lch[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hecho /[45151052727] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: reacting to wallpaper change: /share/wallpapers/flower.png
[45194001369] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=3 height=28 frame=6
share[45405358317] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: xdg_popup obj=23 assigned to xdg_surface=21
/[45460178082] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: imported dmabuf wl_buffer=121 160x96 format=0x34325241
wallpap[45828509067] [[32mINFO [0m] [bloom::render] [CPU1] bloom: flat window overlays ready size=1280x800
ers/clouds.bmp > /session[47194764507] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:13:20.023996959 unix_secs=1777662800.023996959
/desktop/wallpape[48052230006] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: reacting to wallpaper change: /share/wallpapers/flower.png
[48064080108] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: frame callback done object=1000 data=24006
[48067222071] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: wp_presentation_feedback.presented object=1001 tv=24.010769359 refresh_ns=16666666 seq=0
r
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[48889435650] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: reacting to wallpaper change: /share/wallpapers/clouds.bmp
[49151532111]
```
</details>
