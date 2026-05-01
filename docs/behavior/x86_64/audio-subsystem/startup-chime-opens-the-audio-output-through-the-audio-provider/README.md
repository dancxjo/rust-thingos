# ❌ Scenario: Startup chime opens the audio output through the audio provider

> Last run: 2026-04-30 19:15:08

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 0ms | - - - |
| 2 | When I wait for the serial output to contain "chime: Opened /dev/audio/card0/out0" | ❌ | 32209ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[38958419643] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[39236809689] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[39257354730] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[39289108815] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[39323135082] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[39343348671] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[39345219111] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[39385490166] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[39387313482] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[39408001314] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[39423034959] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[39423676776] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[39461954829] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[39463314231] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[39564488766] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[39651608106] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[39675073350] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[39767048442] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[39923502927] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[39969165456] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[40011879138] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[40182055221] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[40278669222] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[40473692259] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[40743243783] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1256241459 elapsed_us=628120
[40744103862] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[40824679005] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[40830494892] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[40872566559] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[40908200289] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[40919360559] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[40921123980] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[41022735732] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[41026623429] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[41033362326] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[1;32m
        .-.
       /   \        [1;36mTHING-OS[1;32m
      |     |       [0;36m"People, places, things."[1;32m
       \   /        
        `-'        
       /   \        v0.1  ��  
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[41196969528] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[41198834028] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[41233496601] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[41235994239] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[41237534184] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[41238817356] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[41264530395] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[41271623184] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[41274870120] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[41276442570] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[41283520575] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[41287210536] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[41303590911] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[41314078674] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[41316663102] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[41321331249] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[41327498652] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[41332131126] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[41344644099] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[41355420513] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[41379703167] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[41386526313] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[41500840161] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[41512705575] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[41521156281] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[41571944337] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[41746202982] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[41910233376] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[42075735999] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[42082180569] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[42091813665] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[42099255396] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[42101410131] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[42287723808] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[42465625521] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[42557555733] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[42654997605] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[42656209332] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[42674925975] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[43010581119] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[43084144290] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1920x1080 @ 60000mHz
[43088809962] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[43092747918] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[43095234006] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[43528829289] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[43530484866] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[43534403352] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[43549965657] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Facet Frame
[44171269395] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[44394633426] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[44701529763] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[44728864356] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[44785081407] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=11)
[44849903703] [[32mINFO [0m] [clock] [CPU3] clock: running at low scheduler priority tid=12
[44861398032] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=12)
[45096217749] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[45271644990] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[45554440107] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=96x96 hotspot=21,12
[45565224045] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[45568523616] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[45609559083] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[45612255282] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[45613611879] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1920x1080 @ 60000mHz ready
[45623353017] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: starting
[45637897206] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: listening on /run/wayland-0
[45639313698] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[45657437232] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[45660211773] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[45693458085] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: connected to /run/wayland-0
[45734908032] [[32mINFO [0m] [clock] [CPU3] clock: connected to /run/wayland-0
[45903778272] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: bristle pid=8
[46076279304] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: claimed /sys/devices/isa-0070 (handle=1)
[46092306513] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: bristle pid=8
[46107246471] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: 2026-05-01 02:17:30 = 1777601850 unix_secs
[46108958214] [[32mINFO [0m] [kernel::time] [CPU3] System clock anchored: unix_secs=1777601850, mono_ns=23054266422, offset=1777601826945733578ns
[46109149515] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Starting AHCI VFS driver
[46110373287] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: System clock anchored to 1777601850 unix_secs
[46133024751] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[46138293168] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 02:17:30.013588525 unix_secs=1777601850.013588525
[46151905470] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Mounted atapi2 at /dev/storage/atapi2
[46152943320] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Provider loop online at /dev/storage/atapi2
[46183409217] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[46190064261] [[32mINFO [0m] [clock] [CPU3] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[46199645580] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x47 (is_aux=false)
[46205614719] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfa (is_aux=true)
[46208073747] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xaa (is_aux=true)
[46210695960] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x00 (is_aux=true)
[46215799212] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfa (is_aux=true)
[46219831647] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfa (is_aux=true)
[46221030537] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: sample rate set to 60 Hz
[46225717923] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfa (is_aux=true)
[46230355050] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfa (is_aux=true)
[46234452528] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfa (is_aux=true)
[46237264062] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x00 (is_aux=true)
[46240357845] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x03 (is_aux=true)
[46243657647] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x3c (is_aux=true)
[46251381561] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfa (is_aux=true)
[46264453059] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[46266593505] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: bound wp_presentation
[46620258333] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=1 rect=1920x1080 src_px=0xff0b0a10 dst_px=0xff0b0a10 damage=1920x1080+0,0 res_id=1
[46622456529] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 958,539 src_px=0x20181818 dst_before=0xff0b0a10 dst_after=0xff0d0c11
[46660949808] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[46668294750] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=28 frame=6
[46699403058] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[46821013998] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
[46832400978] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: imported dmabuf wl_buffer=111 480x320 format=0x34325241
[49138144770] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 18:17:31 UTC-8 system_unix=1777601851.505664550
[51215318715] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[51223643757] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[51754410015] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=28 frame=6
[51779644191] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[52149618411] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 02:17:33.020469540 unix_secs=1777601853.020469540
[53377990215] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=3 height=28 frame=6
[53428840674] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: xdg_popup obj=23 assigned to xdg_surface=21
[53540993220] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: imported dmabuf wl_buffer=121 160x96 format=0x34325241
[55072021158] [[32mINFO [0m] [bloom::render] [CPU1] bloom: flat window overlays ready size=1920x1080
[55554744069] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 18:17:34 UTC-8 system_unix=1777601854.722719298
[57725445294] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: frame callback done object=1000 data=28817
[57729897126] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: wp_presentation_feedback.presented object=1001 tv=28.832188737 refresh_ns=16666666 seq=0
[58163987871] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 02:17:36.027673393 unix_secs=1777601856.027673393
[59700363096] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: frame callback done object=1002 data=29835
[59701673988] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: wp_presentation_feedback.presented object=1003 tv=29.843601100 refresh_ns=16666666 seq=1
[61865662617] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 18:17:37 UTC-8 system_unix=1777601857.878315901
[64178405148] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 02:17:39.034888384 unix_secs=1777601859.034888384
[68114446785] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 18:17:41 UTC-8 system_unix=1777601861.002599399
[70192942380] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 02:17:42.042134874 unix_secs=1777601862.042134874
[74430411033] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 18:17:44 UTC-8 system_unix=1777601864.160512190
[76207436415] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 02:17:45.049387683 unix_secs=1777601865.049387683
[80813859423] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 18:17:47 UTC-8 system_unix=1777601867.352300025
[82221758322] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 02:17:48.056567826 unix_secs=1777601868.056567826
[87373592823] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 18:17:50 UTC-8 system_unix=1777601870.632273909
[88222270488] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 02:17:51.056798053 unix_secs=1777601871.056798053
[94085802954] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 18:17:53 UTC-8 system_unix=1777601873.988210064
[94251066789] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 02:17:54.071172757 unix_secs=1777601874.071172757
[100265256333] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 02:17:57.078297345 unix_secs=1777601877.078297345
[100590827799] [[
```
</details>
