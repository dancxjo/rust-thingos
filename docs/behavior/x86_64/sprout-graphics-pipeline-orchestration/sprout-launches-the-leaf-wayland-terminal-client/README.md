# ❌ Scenario: sprout launches the Leaf Wayland terminal client

> Last run: 2026-05-01 14:19:59

## Steps

| # | Step | Result | Duration | Before | After | Artifacts |
|---|------|--------|----------|--------|-------|-----------|
| 1 | Given the machine is booted | ✅ | 10948ms | <a href="./01/before.png"><img src="./01/before.png" width="120" /></a> | <a href="./01/after.png"><img src="./01/after.png" width="120" /></a> | [📜](./01/serial.log) |
| 2 | Then the serial output should contain "SPROUT: Spawned leaf" within 180s | ❌ | 181200ms | <a href="./02/before.png"><img src="./02/before.png" width="120" /></a> | - | [📜](./02/serial.log) [timeout.png](./02/timeout.png) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[33208130307] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[33456402375] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[33476476638] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[33507223728] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[33539602470] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[33559310895] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[33561139557] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[33598325739] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[33599998839] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[33619525863] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[33633835620] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[33634443018] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[33669276465] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[33670279698] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[33757212489] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[33835402557] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[33856449792] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[33927673461] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[34026342966] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[34052766627] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[34075629390] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[34174291899] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[34223372799] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[34352996502] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[34595851488] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=905282928 elapsed_us=452641
[34596793638] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[34675199922] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[34679052276] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[34721326299] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[34793791956] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[34803307869] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[34804824846] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[34896583524] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[34899874086] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[34904696607] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[35104331196] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[35106136527] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[35140254963] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[35142648453] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[35144138766] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[35144891925] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[35170859559] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[35180382534] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[35183768367] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[35186184594] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[35199005490] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[35204671788] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[35207411316] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[35218237494] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[35221283262] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[35228463600] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[35233605759] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[35242521402] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[35253908151] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[35264091951] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[35292659226] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[35299542036] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[35384699163] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[35395302756] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[35405768112] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: cursor queue (queue 1) configured
[35406694488] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[35407375938] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: composition paths: gpu_opaque_copy=enabled cpu_alpha_blend=enabled
[35415493047] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: host scanout 1280x800 enabled=true (bootfb was 1920x1080)
[35456560227] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[35618947023] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[35670181800] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor DMA buffer ready (16 pages at phys=0x2723000)
[35732984826] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[35740915914] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[35745300888] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[35747345832] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[35778714741] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[35945155554] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[36119482212] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[36138113517] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[36218163729] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[36219264312] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[36321021363] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[36540630522] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[36560859027] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1280x800 @ 60000mHz
[36562503780] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[36563214270] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[36563947761] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports GPU blit (hardware transfer/flush)
[36564799128] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports direct scanout (zero-copy path to display)
[36565879449] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports partial flush (damage regions)
[36566544300] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports resource cache (pre-allocated buffer pool)
[36567435201] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[36870897558] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[36871959498] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[36873153306] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[36881609028] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Facet Frame
[37080561078] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hwrng' name='hwrng' class=Other kind='dev.rng.HwRng' start='_RNvCs7wlZbVUrQWf_5hwrng20thingos_driver_start'
[37232991279] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[37383568365] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[37575925167] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[37663268577] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[37861694772] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=48x48 hotspot=11,6
[37872551772] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[37874899755] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[37920596538] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[37922365404] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[37923362037] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1280x800 @ 60000mHz ready
[37929275373] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: starting
[37934280120] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[37943929551] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: listening on /run/wayland-0
[37944887739] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[38086249773] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[38171106732] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=1 rect=1280x800 src_px=0xff0b0a10 dst_px=0xff0b0a10 damage=1280x800+0,0 res_id=1 gpu_planes=0 cpu_planes=2
[38172451647] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 638,399 src_px=0x10202020 dst_before=0xff0b0a10 dst_after=0xff0c0b11
[38247160215] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[38253144963] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[38407575723] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[38471670072] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=12)
[38488095657] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: connected to /run/wayland-0
[38503575132] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=13)
[38572463721] [[32mINFO [0m] [clock] [CPU1] clock: running at low scheduler priority tid=13
[38576240769] [[32mINFO [0m] [clock] [CPU1] clock: connected to /run/wayland-0
[38776667160] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[38797060401] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[38798508012] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: bound wp_presentation
[38803352115] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: wl_region smoke test: create+add+set_opaque+destroy
[38867014131] [[32mINFO [0m] [clock] [CPU1] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[38868100194] [[32mINFO [0m] [clock] [CPU1] clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf
[38875783221] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=28 frame=6
[38885824494] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[38892584742] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/xhci' name='xhci' class=Block kind='dev.usb.Xhci' start='thingos_driver_start_safe'
[38894227383] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 15 driver(s) found
[38900268495] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[38907814968] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[38955901974] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Starting AHCI VFS driver
[38994804948] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[39023855673] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: imported dmabuf wl_buffer=111 480x320 format=0x34325241
[39028126830] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Mounted atapi2 at /dev/storage/atapi2
[39029509893] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Provider loop online at /dev/storage/atapi2
[39051567258] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: bristle pid=8
[39116302071] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: bristle pid=8
[39138004620] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: claimed /sys/devices/isa-0070 (handle=2)
[39161995191] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: sample rate set to 60 Hz
[39529538334] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: 2026-05-01 21:24:02 = 1777670642 unix_secs
[39530754516] [[32mINFO [0m] [kernel::time] [CPU3] System clock anchored: unix_secs=1777670642, mono_ns=19765235011, offset=1777670622234764989ns
[39531547209] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: System clock anchored to 1777670642 unix_secs
[39543951876] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:02.005505968 unix_secs=1777670642.005505968
[39593760558] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=28 frame=6
[39599590239] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[39602179584] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
[43617583053] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=3 height=28 frame=6
[43642194519] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_popup obj=23 assigned to xdg_surface=21
[43681317702] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: imported dmabuf wl_buffer=121 160x96 format=0x34325241
[44166618639] [[32mINFO [0m] [bloom::render] [CPU1] bloom: flat window overlays ready size=1280x800
[45554074665] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:05.011717049 unix_secs=1777670645.011717049
[45836270568] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: frame callback done object=1000 data=22903
[45838126059] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: wp_presentation_feedback.presented object=1001 tv=22.906160343 refresh_ns=16666666 seq=0
[46186776240] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: frame callback done object=1002 data=23089
[46188310575] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: wp_presentation_feedback.presented object=1003 tv=23.092176459 refresh_ns=16666666 seq=1
[51566548803] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:08.017966774 unix_secs=1777670648.017966774
[57578914239] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:11.024172014 unix_secs=1777670651.024172014
[63591334686] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:14.030380142 unix_secs=1777670654.030380142
[69603777573] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:17.036603648 unix_secs=1777670657.036603648
[74362633323] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 13:24:19 UTC-8 system_unix=1777670659.415275906
[75616214487] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:20.042819713 unix_secs=1777670660.042819713
[81628661169] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:23.049045166 unix_secs=1777670663.049045166
[87641390430] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:26.055385508 unix_secs=1777670666.055385508
[93653617101] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:29.061520162 unix_secs=1777670669.061520162
[99666062364] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:32.067747694 unix_secs=1777670672.067747694
[105678441165] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:35.073937853 unix_secs=1777670675.073937853
[111690801288] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:38.080118327 unix_secs=1777670678.080118327
[117703317798] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:41.086368580 unix_secs=1777670681.086368580
[123715777482] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:44.092600270 unix_secs=1777670684.092600270
[129728059626] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:47.098760927 unix_secs=1777670687.098760927
[135740895807] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:50.105121565 unix_secs=1777670690.105121565
[141753124326] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:53.111268065 unix_secs=1777670693.111268065
[147765631464] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:56.117523268 unix_secs=1777670696.117523268
[149098166910] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 13:24:56 UTC-8 system_unix=1777670696.783524153
[153777984657] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:24:59.123694997 unix_secs=1777670699.123694997
[159790352832] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:25:02.129903059 unix_secs=1777670702.129903059
[165802792716] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:25:05.136121153 unix_secs=1777670705.136121153
[171815307576] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:25:08.142364937 unix_secs=1777670708.142364937
[177827747889] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:25:11.148586975 unix_secs=1777670711.148586975
[183840181866] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:25:14.154803501 unix_secs=1777670714.154803501
[189852524367] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:25:17.160991004 unix_secs=1777670717.160991004
[195864980784] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:25:20.167215979 unix_secs=1777670720.167215979
[201877657773] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:25:23.173512398 unix_secs=1777670723.173512398
[207890250678] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:25:26.179804198 unix_secs=1777670726.179804198
[213902281098] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:25:29.185869650 unix_secs=1777670729.185869650
[219914856480] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:25:32.192140198 unix_secs=1777670732.192140198
[223840566444] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 13:25:34 UTC-8 system_unix=1777670734.154789309
[225927221685] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:25:35.198334449 unix_secs=1777670735.198334449
[231940015065] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:25:38.204587886 unix_secs=1777670738.204587886
[237952098384] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:25:41.210775670 unix_secs=1777670741.210775670
[243964620603] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:25:44.217024470 unix_secs=1777670744.217024470
[249977065932] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:25:47.223256507 unix_secs=1777670747.223256507
[255989594190] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:25:50.229491992 unix_secs=1777670750.229491992
[262002076908] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:25:53.235747590 unix_secs=1777670753.235747590
[268014353706] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:25:56.241905030 unix_secs=1777670756.241905030
[274026930705] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:25:59.248174802 unix_secs=1777670759.248174802
[280039348776] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:26:02.254384465 unix_secs=1777670762.254384465
[286051662105] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:26:05.260554247 unix_secs=1777670765.260554247
[292064123901] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:26:08.266776251 unix_secs=1777670768.266776251
[298076547582] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:26:11.272997200 unix_secs=1777670771.272997200
[298624269669] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 13:26:11 UTC-8 system_unix=1777670771.546592709
[304089366834] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:26:14.279315003 unix_secs=1777670774.279315003
[310101464937] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:26:17.285445202 unix_secs=1777670777.285445202
[316114232445] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:26:20.291809849 unix_secs=1777670780.291809849
[322126648206] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:26:23.297991758 unix_secs=1777670783.297991758
[328138935696] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:26:26.304159445 unix_secs=1777670786.304159445
[334151133690] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:26:29.310296293 unix_secs=1777670789.310296293
[340163725341] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:26:32.316566576 unix_secs=1777670792.316566576
[346176105726] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:26:35.322766240 unix_secs=1777670795.322766240
[352188531123] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:26:38.328990472 unix_secs=1777670798.328990472
[358200925731] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:26:41.335191587 unix_secs=1777670801.335191587
[364213540812] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:26:44.341491604 unix_secs=1777670804.341491604
[370225943307] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:26:47.347678117 unix_secs=1777670807.347678117
[373355291925] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 13:26:48 UTC-8 system_unix=1777670808.912187838
[376238630823] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:26:50.353991713 unix_secs=1777670810.353991713
[382252297845] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:26:53.360810473 unix_secs=1777670813.360810473
[388263148581] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:26:56.366296363 unix_secs=1777670816.366296363
[394275787785] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:26:59.372598211 unix_secs=1777670819.372598211
[400288085439] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:27:02.378768323 unix_secs=1777670822.378768323
[406300614786] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:27:05.385019846 unix_secs=1777670825.385019846
[412312980453] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:27:08.391211094 unix_secs=1777670828.391211094
[418325699451] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:27:11.397541256 unix_secs=1777670831.397541256
[424337945130] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:27:14.403679787 unix_secs=1777670834.403679787
[430350698316] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:27:17.410037092 unix_secs=1777670837.410037092
[436363001580] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:27:20.416194433 unix_secs=1777670840.416194433
[442375508223] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:27:23.422462489 unix_secs=1777670843.422462489
[448128277872] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 13:27:26 UTC-8 system_unix=1777670846.298682924
[448387710507] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:27:26.428561882 unix_secs=1777670846.428561882
[454400237445] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:27:29.434823536 unix_secs=1777670849.434823536
[460412803026] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:27:32.441073260 unix_secs=1777670852.441073260
[466425155031] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:27:35.447284573 unix_secs=1777670855.447284573
[472437512184] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:27:38.453466334 unix_secs=1777670858.453466334
[478449912996] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:27:41.459679857 unix_secs=1777670861.459679857
[484462432872] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:27:44.465922932 unix_secs=1777670864.465922932
[490474981557] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:27:47.472181880 unix_secs=1777670867.472181880
[496487255715] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:27:50.478332786 unix_secs=1777670870.478332786
[502499615838] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:27:53.484527698 unix_secs=1777670873.484527698
[508512214650] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:27:56.490800044 unix_secs=1777670876.490800044
[514524543027] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:27:59.496988207 unix_secs=1777670879.496988207
[520537034061] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:28:02.503237667 unix_secs=1777670882.503237667
[522912967137] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 13:28:03 UTC-8 system_unix=1777670883.690852524
[526549509189] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:28:05.509458088 unix_secs=1777670885.509458088
[532561916205] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:28:08.515663345 unix_secs=1777670888.515663345
[538574480037] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:28:11.521941631 unix_secs=1777670891.521941631
[544586776140] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:28:14.528102569 unix_secs=1777670894.528102569
[550599252918] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:28:17.534336783 unix_secs=1777670897.534336783
[556611659043] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:28:20.540537519 unix_secs=1777670900.540537519
[562624006593] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:28:23.546727580 unix_secs=1777670903.546727580
[568636565244] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:28:26.552983805 unix_secs=1777670906.552983805
[574648918602] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:28:29.559182017 unix_secs=1777670909.559182017
[580661398614] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:28:32.565418690 unix_secs=1777670912.565418690
[586673993070] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:28:35.571700127 unix_secs=1777670915.571700127
[592686229410] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:28:38.577840110 unix_secs=1777670918.577840110
[597643244583] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 13:28:41 UTC-8 system_unix=1777670921.056090808
[598698683451] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:28:41.584065646 unix_secs=1777670921.584065646
[604711290051] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:28:44.590317796 unix_secs=1777670924.590317796
[610723730067] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:28:47.596563544 unix_secs=1777670927.596563544
[616736133222] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:28:50.602771606 unix_secs=1777670930.602771606
[622748702532] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 21:28:53.609028920 unix_secs=1777670933.609028920
[628761038301] [
```
</details>
