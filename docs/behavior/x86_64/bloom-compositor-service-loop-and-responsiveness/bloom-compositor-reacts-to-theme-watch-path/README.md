# ❌ Scenario: bloom compositor reacts to theme watch path

> Last run: 2026-05-01 13:12:24

## Steps

| # | Step | Result | Duration | Before | After | Artifacts |
|---|------|--------|----------|--------|-------|-----------|
| 1 | Given the machine is booted | ✅ | 10934ms | <a href="./01/before.png"><img src="./01/before.png" width="120" /></a> | <a href="./01/after.png"><img src="./01/after.png" width="120" /></a> | [📜](./01/serial.log) |
| 2 | Then the serial output should contain "bloom: service loop started" within 60s | ✅ | 719ms | <a href="./02/before.png"><img src="./02/before.png" width="120" /></a> | <a href="./02/after.png"><img src="./02/after.png" width="120" /></a> | [📜](./02/serial.log) |
| 3 | When I wait for the shell prompt | ✅ | 999ms | <a href="./03/before.png"><img src="./03/before.png" width="120" /></a> | <a href="./03/after.png"><img src="./03/after.png" width="120" /></a> | [📜](./03/serial.log) [console_interactive.png](./03/console_interactive.png) |
| 4 | And I type "echo Solarized Warm > /session/desktop/theme" on the serial console | ✅ | 1270ms | <a href="./04/before.png"><img src="./04/before.png" width="120" /></a> | <a href="./04/after.png"><img src="./04/after.png" width="120" /></a> | [📜](./04/serial.log) |
| 5 | Then the serial output should contain "bloom: reacting to theme change: Solarized Warm -> Solarized Warm" within 60s | ❌ | 60668ms | <a href="./05/before.png"><img src="./05/before.png" width="120" /></a> | - | [📜](./05/serial.log) [timeout.png](./05/timeout.png) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[33515235369] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[33762811137] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[33782790921] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[33814637373] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[33846809271] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[33866032299] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[33868158390] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[33906425982] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[33908051364] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[33927980361] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[33942511119] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[33943158249] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[33979018260] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[33980083500] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[34067747868] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[34146051852] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[34167399288] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[34242328725] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[34343934966] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[34369567122] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[34391820210] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[34489554528] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[34537994403] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[34666632198] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[34905929091] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=905280222 elapsed_us=452640
[34906609815] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[35023343091] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[35026940322] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[35064276555] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[35102168310] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[35110526187] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[35111985480] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[35203698090] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[35207038053] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[35221800009] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[35375911362] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[35377577565] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[35410260039] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[35412650625] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[35414409624] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[35415099951] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[35443465365] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[35450558352] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[35453710941] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[35455301805] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[35476202619] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[35480474601] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[35482347186] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[35492356878] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[35495534745] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[35500472733] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[35504668749] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[35506422138] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[35517108957] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[35528213391] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[35565189132] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[35571402669] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[35659856067] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[35672060094] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[35682520665] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: cursor queue (queue 1) configured
[35684645007] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[35686545939] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: composition paths: gpu_opaque_copy=enabled cpu_alpha_blend=enabled
[35697456498] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: host scanout 1280x800 enabled=true (bootfb was 1920x1080)
[35718027906] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[35882474595] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[35956736178] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor DMA buffer ready (16 pages at phys=0x2723000)
[36024323511] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[36033073989] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[36038828430] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[36040370454] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[36056229924] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[36228591234] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[36409825221] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[36440519181] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[36509034309] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[36510137664] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[36668067447] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[36691001490] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1280x800 @ 60000mHz
[36691981524] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[36692730987] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[36693460386] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports GPU blit (hardware transfer/flush)
[36694112532] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports direct scanout (zero-copy path to display)
[36694768770] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports partial flush (damage regions)
[36695395473] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports resource cache (pre-allocated buffer pool)
[36696059367] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[36984983904] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[37135544787] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hwrng' name='hwrng' class=Other kind='dev.rng.HwRng' start='_RNvCs7wlZbVUrQWf_5hwrng20thingos_driver_start'
[37290057321] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[37460710650] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[37461809253] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[37463728797] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[37470704898] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Facet Frame
[37716041286] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[37811994231] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[37899273819] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[38165684766] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=48x48 hotspot=11,6
[38176021488] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[38178817842] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[38189251056] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[38339353206] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[38535549822] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[38537754156] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[38538659478] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1280x800 @ 60000mHz ready
[38539422867] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: starting
[38554097769] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[38554803507] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: listening on /run/wayland-0
[38555664015] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[38742627825] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=12)
[38743512555] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[38755140534] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: connected to /run/wayland-0
[38766629088] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=13)
[38842012638] [[32mINFO [0m] [clock] [CPU1] clock: running at low scheduler priority tid=13
[38845898553] [[32mINFO [0m] [clock] [CPU1] clock: connected to /run/wayland-0
[38893197948] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=1 rect=1280x800 src_px=0xff0b0a10 dst_px=0xff0b0a10 damage=1280x800+0,0 res_id=1 gpu_planes=0 cpu_planes=2
[38894648595] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 638,399 src_px=0x10202020 dst_before=0xff0b0a10 dst_after=0xff0c0b11
[38911533936] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[39157569264] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[39162769602] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[39175488693] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[39179549277] [[32mINFO [0m] [clock] [CPU1] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[39180936201] [[32mINFO [0m] [clock] [CPU1] clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf
[39205035540] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[39206491467] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: bound wp_presentation
[39211250529] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: wl_region smoke test: create+add+set_opaque+destroy
[39269778867] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=28 frame=6
[39288162243] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[39297434154] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/xhci' name='xhci' class=Block kind='dev.usb.Xhci' start='thingos_driver_start_safe'
[39299135007] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 15 driver(s) found
[39354591738] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Starting AHCI VFS driver
[39381250194] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: imported dmabuf wl_buffer=111 480x320 format=0x34325241
[39390764655] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[39406523442] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Mounted atapi2 at /dev/storage/atapi2
[39407447244] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Provider loop online at /dev/storage/atapi2
[39428475834] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: bristle pid=8
[39507060021] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: bristle pid=8
[39565932417] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: sample rate set to 60 Hz
[39851091357] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: claimed /sys/devices/isa-0070 (handle=2)
[39868875255] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: 2026-05-01 20:19:58 = 1777666798 unix_secs
[39869879379] [[32mINFO [0m] [kernel::time] [CPU3] System clock anchored: unix_secs=1777666798, mono_ns=19934794209, offset=1777666778065205791ns
[39870606963] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: System clock anchored to 1777666798 unix_secs
e[39875474397] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:19:58.001707799 unix_secs=1777666798.001707799
cho BDD_CONSOLE_RE[40031374878] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=28 frame=6
[40043907090] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
[40044600255] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
ADY_3040849_1777666799625046888
BDD_CONSOLE_READY_3040849_1777666799625046888
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hecho Solarized Warm[43979562792] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=3 height=28 frame=6
[43992986103] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_popup obj=23 assigned to xdg_surface=21
 [44020497147] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: imported dmabuf wl_buffer=121 160x96 format=0x34325241
> /session[44535113733] [[32mINFO [0m] [bloom::render] [CPU1] bloom: flat window overlays ready size=1280x800
/desktop/theme
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[45898990698] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:20:01.014585689 unix_secs=1777666801.014585689
[46155210519] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: frame callback done object=1000 data=23070
[46157563320] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: wp_presentation_feedback.presented object=1001 tv=23.074097689 refresh_ns=16666666 seq=0
[46173861822] [[32mINFO [0m] [bloom::services::theme_service] [CPU1] bloom: reacting to theme change: Solarized Warm -> Facet Frame
[46947583947] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: frame callback done object=1002 data=23468
[46948630113] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: wp_presentation_feedback.presented object=1003 tv=23.471884012 refresh_ns=16666666 seq=1
[46962355704] [[32mINFO [0m] [bloom::services::theme_service] [CPU1] bloom: reacting to theme change: Solarized Warm -> Facet Frame
[46971729717] [[32mINFO [0m] [bloom::services::theme_service] [CPU1] bloom: reacting to theme change: Solarized Warm -> Facet Frame
[47626441203] [[32mINFO [0m] [bloom::services::theme_service] [CPU1] bloom: reacting to theme change: Solarized Warm -> Facet Frame
[51924518316] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:20:04.027413271 unix_secs=1777666804.027413271
[57950268849] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:20:07.040288405 unix_secs=1777666807.040288405
[63976245597] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:20:10.053278710 unix_secs=1777666810.053278710
[70002029856] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:20:13.066148960 unix_secs=1777666813.066148960
[74720338539] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 12:20:15 UTC-8 system_unix=1777666815.424207652
[76027879983] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:20:16.079091332 unix_secs=1777666816.079091332
[82053408624] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:20:19.091850785 unix_secs=1777666819.091850785
[88079148564] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:20:22.104747617 unix_secs=1777666822.104747617
[94104947376] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:20:25.117641281 unix_secs=1777666825.117641281
[100131671739] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:20:28.130970380 unix_secs=1777666828.130970380
[106156782600] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:20:31.143562804 unix_secs=1777666831.143562804
[112182447993] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:20:34.156388207 unix_secs=1777666834.156388207
[118207959441] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:20:37.169150531 unix_secs=1777666837.169150531
[124233833328] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:20:40.182085396 unix_secs=1777666840.182085396
[130259653194] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:20:43.194991649 unix_secs=1777666843.194991649
[136285315386] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:20:46.207814578 unix_secs=1777666846.207814578
[142311147528] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:20:49.220745532 unix_secs=1777666849.220745532
[148336995015] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:20:52.233647776 unix_secs=1777666852.233647776
[148804903797] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 12:20:52 UTC-8 system_unix=1777666852.467467642
[154362755481] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:20:55.246529411 unix_secs=1777666855.246529411
[160388694378] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:20:58.259483828 unix_secs=1777666858.259483828
[166414323801] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:21:01.272313654 unix_secs=1777666861.272313654
[172440292695] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:21:04.285285643 unix_secs=1777666864.285285643
[178465773981] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:21:07.298051564 unix_secs=1777666867.298051564
[184491815244] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:21:10.311041836 unix_secs=1777666870.311041836
[190517339496] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:21:13.323832952 unix_secs=1777666873.323832952
[196543174146] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:21:16.336738925 unix_secs=1777666876.336738925
[202568822841] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:21:19.349577941 unix_secs=1777666879.349577941
[208594739826] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:21:22.362465781 unix_secs=1777666882.362465781
[214620883983] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:21:25.375619221 unix_secs=1777666885.375619221
[220646296761] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:21:28.388307080 unix_secs=1777666888.388307080
[222891785292] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 12:21:29 UTC-8 system_unix=1777666889.510458022
[226674634560] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:21:31.402433938 unix_secs=1777666891.402433938
[232697922699] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:21:34.414118086 unix_secs=1777666894.414118086
[238723837275] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:21:37.427066728 unix_secs=1777666897.427066728
[244749404592] [
```
</details>
