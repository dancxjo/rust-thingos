# ❌ Scenario: Sprout receives DRIVER_READY and logs the state transition

> Last run: 2026-04-30 21:36:48

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When the system finishes bringing up supervised services | ❌ | 92220ms | - [📜](./01/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[37124407386] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[37406820594] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[37429851327] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[37466647053] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[37503965730] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[37527393519] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[37530143046] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[37574120100] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[37576420332] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[37599428427] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[37615674756] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[37616505036] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[37659210072] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[37660456878] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[37762338471] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[37859169249] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[37884515658] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[37968132675] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[38086552383] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[38128500465] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[38153988246] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[38273773296] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[38331282495] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[38489622303] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[38776364946] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1092496119 elapsed_us=546248
[38778254427] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[38878154403] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[38885577192] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[38934672117] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[38976483744] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[38989921311] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[38992071393] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[39123379086] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[39132702840] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[39185943258] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[39344219112] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[39347787336] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[39395008026] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[39398068479] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[39399569847] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[39399816060] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[39434905026] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[39448736052] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[39454974735] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[39457129536] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[39470544201] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[39476074935] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[39479393019] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[39495709209] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[39499451376] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[39505398504] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[39511673058] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[39531321225] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[39548285007] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[39567632742] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[39592258167] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[39596830284] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[39726675153] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[39745473042] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[39758700861] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: cursor queue (queue 1) configured
[39760248759] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[39838115856] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[40041273162] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[40227680460] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[40367523768] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor DMA buffer ready (16 pages at phys=0x2b21000)
[40428505458] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[40458436128] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[40471950519] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[40478655129] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[40480691493] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[40647967635] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[40849606215] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[40885455237] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[41065999590] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[41067327378] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[41092269405] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hwrng' name='hwrng' class=Other kind='dev.rng.HwRng' start='_RNvCs7wlZbVUrQWf_5hwrng20thingos_driver_start'
[41380261659] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[41407965687] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1920x1080 @ 60000mHz
[41409215100] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[41409942420] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[41410598658] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[41811646404] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[41812939047] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[41814007059] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[41825094828] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Facet Frame
[42405186912] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[42642357681] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[42878849508] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[42984617082] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=96x96 hotspot=21,12
[43000012869] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[43006017615] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[43072826082] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[43075698600] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[43076971080] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1920x1080 @ 60000mHz ready
[43079336685] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: starting
[43089273315] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
[43104609273] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: listening on /run/wayland-0
[43105772457] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[43241744766] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=12)
[43277092518] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: connected to /run/wayland-0
[43286753565] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=13)
[43693864830] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[43718961990] [[32mINFO [0m] [clock] [CPU1] clock: running at low scheduler priority tid=13
[43724834307] [[32mINFO [0m] [clock] [CPU1] clock: connected to /run/wayland-0
[43748248731] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[43750381059] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: bound wp_presentation
[44088711117] [[32mINFO [0m] [clock] [CPU1] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[44089901625] [[32mINFO [0m] [clock] [CPU1] clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf
[44349626541] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[44527639332] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[44706182961] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[46008619899] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[47047373670] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[47049725250] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 14 driver(s) found
[47128718769] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Starting AHCI VFS driver
[47172444198] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[47196370089] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Mounted atapi2 at /dev/storage/atapi2
[47198238483] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Provider loop online at /dev/storage/atapi2
[47240549697] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: bristle pid=8
[47245461516] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: claimed /sys/devices/isa-0070 (handle=2)
[47306937546] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: 2026-05-01 04:39:41 = 1777610381 unix_secs
[47309027205] [[32mINFO [0m] [kernel::time] [CPU3] System clock anchored: unix_secs=1777610381, mono_ns=23654267092, offset=1777610357345732908ns
[47310566523] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: System clock anchored to 1777610381 unix_secs
[47315425311] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:41.002177934 unix_secs=1777610381.002177934
[47420003499] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: bristle pid=8
[47476269423] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: sample rate set to 60 Hz
[48329810430] [[33mWARN [0m] [virtio_gpu] [CPU3] VirtioGpu: Command 0x101 failed with resp_type=0x1203
[48331663248] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor resource create failed: Command failed
[48703914402] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=3 rect=1920x1080 src_px=0xff030209 dst_px=0xff030209 damage=1920x1080+0,0 res_id=1
[48705473586] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 958,539 src_px=0x20181818 dst_before=0xff341a66 dst_after=0xff301a5c
[48714548223] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[48720789084] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=28 frame=6
[48740022870] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[48817986624] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[48829547547] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[48866059341] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: imported dmabuf wl_buffer=111 480x320 format=0x34325241
[49296960867] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=28 frame=6
[49308311613] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[52000764981] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=3 height=28 frame=6
[52017528090] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_popup obj=23 assigned to xdg_surface=21
[52082563665] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: imported dmabuf wl_buffer=121 160x96 format=0x34325241
[53342761824] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:44.017005992 unix_secs=1777610384.017005992
[53637283062] [[32mINFO [0m] [bloom::render] [CPU1] bloom: flat window overlays ready size=1920x1080
[55715273526] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: frame callback done object=1000 data=27849
[55718459445] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: wp_presentation_feedback.presented object=1001 tv=27.853786587 refresh_ns=16666666 seq=0
[56308196307] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: frame callback done object=1002 data=28147
[56310102849] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: wp_presentation_feedback.presented object=1003 tv=28.149900355 refresh_ns=16666666 seq=1
[59373120906] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:47.032230281 unix_secs=1777610387.032230281
[65403585951] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:50.047467836 unix_secs=1777610390.047467836
[71434119504] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:53.062733029 unix_secs=1777610393.062733029
[74256169185] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:39:54 UTC-8 system_unix=1777610394.473031143
[77464514490] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:56.077946147 unix_secs=1777610396.077946147
[83495070582] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:59.093223500 unix_secs=1777610399.093223500
[89525599746] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:02.108486894 unix_secs=1777610402.108486894
[95556255960] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:05.123788750 unix_secs=1777610405.123788750
[101586634050] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:08.138986144 unix_secs=1777610408.138986144
[107617142622] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:11.154232559 unix_secs=1777610411.154232559
[113647580508] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:14.169460709 unix_secs=1777610414.169460709
[119677980873] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:17.184673663 unix_secs=1777610417.184673663
[125708415756] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:20.199898150 unix_secs=1777610420.199898150
[131738942082] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:23.215158227 unix_secs=1777610423.215158227
[137769592224] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:26.230453912 unix_secs=1777610426.230453912
[143800180161] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:29.245760882 unix_secs=1777610429.245760882
[148842122451] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:40:31 UTC-8 system_unix=1777610431.766418247
[149830484331] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:32.260909238 unix_secs=1777610432.260909238
[155861032041] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:35.276181559 unix_secs=1777610435.276181559
[161891528040] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:38.291433221 unix_secs=1777610438.291433221
[167921895174] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:41.306633305 unix_secs=1777610441.306633305
[173952453840] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:44.321899586 unix_secs=1777610444.321899586
[179983061115] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:47.337201376 unix_secs=1777610447.337201376
[186013392543] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:50.352382534 unix_secs=1777610450.352382534
[192044649984] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:53.367981257 unix_secs=1777610453.367981257
[198074380911] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:56.382878252 unix_secs=1777610456.382878252
[204105250107] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:59.398283926 unix_secs=1777610459.398283926
[210135529758] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:02.413430434 unix_secs=1777610462.413430434
[216166143699] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:05.428714717 unix_secs=1777610465.428714717
[222196503144] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:08.443906402 unix_secs=1777610468.443906402
[223482702696] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:41:09 UTC-8 system_unix=1777610469.082182898
[228226990629] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:11.459145178 unix_secs=1777610471.459145178
[234257844117] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:14.474565734 unix_secs=1777610474.474565734
[240287869140] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:17.489613292 unix_secs=1777610477.489613292
[246318471003] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:20.504894902 unix_secs=1777610480.504894902
[252340639980] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:23.515977740 unix_secs=1777610483.515977740
[258346302984] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:26.518822211 unix_secs=1777610486.518822211
[264382730799] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:29.537005973 unix_secs=1777610489.537005973
[270387480033] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:32.539380640 unix_secs=1777610492.539380640
[276404650467] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:35.548000886 unix_secs=1777610495.548000886
[282435112740] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:38.563218905 unix_secs=1777610498.563218905
[288465569469] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:41.578451345 unix_secs=1777610501.578451345
[294495929640] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:44.593653838 unix_secs=1777610504.593653838
[297584684892] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:41:46 UTC-8 system_unix=1777610506.137611687
[300526525299] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:47.608927049 unix_secs=1777610507.608927049
[306556931571] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:50.624154292 unix_secs=1777610510.624154292
[312587416548] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:53.639397160 unix_secs=1777610513.639397160
[318618184533] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:56.654749076 unix_secs=1777610516.654749076
[324648382278] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:59.669877451 unix_secs=1777610519.669877451
[330678983547] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:02.685165199 unix_secs=1777610522.685165199
[336709411368] [
```
</details>
