# ❌ Scenario: bloom compositor reacts to theme watch path

> Last run: 2026-05-01 13:23:52

## Steps

| # | Step | Result | Duration | Before | After | Artifacts |
|---|------|--------|----------|--------|-------|-----------|
| 1 | Given the machine is booted | ✅ | 11257ms | <a href="./01/before.png"><img src="./01/before.png" width="120" /></a> | <a href="./01/after.png"><img src="./01/after.png" width="120" /></a> | [📜](./01/serial.log) |
| 2 | Then the serial output should contain "bloom: service loop started" within 60s | ✅ | 825ms | <a href="./02/before.png"><img src="./02/before.png" width="120" /></a> | <a href="./02/after.png"><img src="./02/after.png" width="120" /></a> | [📜](./02/serial.log) |
| 3 | When I wait for the shell prompt | ✅ | 987ms | <a href="./03/before.png"><img src="./03/before.png" width="120" /></a> | <a href="./03/after.png"><img src="./03/after.png" width="120" /></a> | [📜](./03/serial.log) [console_interactive.png](./03/console_interactive.png) |
| 4 | And I type "echo Solarized Warm > /session/desktop/theme" on the serial console | ✅ | 887ms | <a href="./04/before.png"><img src="./04/before.png" width="120" /></a> | <a href="./04/after.png"><img src="./04/after.png" width="120" /></a> | [📜](./04/serial.log) |
| 5 | Then the serial output should contain "bloom: reacting to theme change: Solarized Warm -> Solarized Warm" within 60s | ❌ | 61377ms | <a href="./05/before.png"><img src="./05/before.png" width="120" /></a> | - | [📜](./05/serial.log) [timeout.png](./05/timeout.png) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[34422463812] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[34671745581] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[34691284386] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[34722169449] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[34753362303] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[34772502402] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[34774120986] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[34811009838] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[34812984525] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[34832785218] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[34847775039] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[34848387453] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[34883822061] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[34884801501] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[34972881141] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[35053578879] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[35075837841] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[35150038935] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[35248465890] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[35274179919] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[35296721262] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[35396366577] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[35448239838] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[35580641052] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[35830336905] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=924999702 elapsed_us=462499
[35831352249] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[36019610418] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[36025436073] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[36076857003] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[36120565635] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[36129487284] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[36131077719] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[36261735741] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[36266803947] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[36275453907] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[36418516959] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[36420955395] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[36458901699] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[36461259318] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[36463225392] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[36463735473] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[36486128250] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[36502227927] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[36507046488] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[36509486739] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[36517095549] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[36521847879] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[36524146659] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[36534893901] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[36541227360] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[36546652989] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[36552302457] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[36555808278] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[36567718638] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[36578271015] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[36612533958] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[36618369117] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[36712777299] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[36724552755] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[36736923498] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: cursor queue (queue 1) configured
[36738539772] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[36739922637] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: composition paths: gpu_opaque_copy=enabled cpu_alpha_blend=enabled
[36750831282] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: host scanout 1280x800 enabled=true (bootfb was 1920x1080)
[36765559248] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[36931353591] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[37001147799] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor DMA buffer ready (16 pages at phys=0x2723000)
[37060609377] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[37068519774] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[37073016915] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[37075738788] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[37099435461] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[37263753681] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[37460377845] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[37513345254] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[37563876768] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[37565049555] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[37712558730] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[37801645959] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[37821249609] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1280x800 @ 60000mHz
[37822176744] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[37822797837] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[37823602938] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports GPU blit (hardware transfer/flush)
[37824336990] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports direct scanout (zero-copy path to display)
[37824993954] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports partial flush (damage regions)
[37825781004] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports resource cache (pre-allocated buffer pool)
[37826609139] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[38184199581] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hwrng' name='hwrng' class=Other kind='dev.rng.HwRng' start='_RNvCs7wlZbVUrQWf_5hwrng20thingos_driver_start'
[38339186094] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[38443705542] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[38444917038] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[38445607266] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[38452813080] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Facet Frame
[38765540055] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[38774051118] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[38926599855] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[39076823577] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[39268370592] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[39429207840] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[39586022289] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[39829454610] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=48x48 hotspot=11,6
[39839548056] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[39841647648] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[39855202233] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=11)
[39880440600] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[39882412284] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[39883452378] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1280x800 @ 60000mHz ready
[39884073075] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: starting
[39893069304] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=13)
[39893405475] [[32mINFO [0m] [clock] [CPU1] clock: running at low scheduler priority tid=13
[39898154802] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: listening on /run/wayland-0
[39899080122] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[39910419054] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/xhci' name='xhci' class=Block kind='dev.usb.Xhci' start='thingos_driver_start_safe'
[39912017376] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 15 driver(s) found
[39970066554] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Starting AHCI VFS driver
[39997477080] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[40059988452] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: bristle pid=8
[40063465563] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: connected to /run/wayland-0
[40068680619] [[32mINFO [0m] [clock] [CPU1] clock: connected to /run/wayland-0
[40126796655] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: claimed /sys/devices/isa-0070 (handle=2)
[40152467124] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: 2026-05-01 20:31:39 = 1777667499 unix_secs
[40154129070] [[32mINFO [0m] [kernel::time] [CPU3] System clock anchored: unix_secs=1777667499, mono_ns=20076887803, offset=1777667478923112197ns
[40155600309] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: System clock anchored to 1777667499 unix_secs
[40170005073] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:31:39.006784734 unix_secs=1777667499.006784734
[40265124999] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=1 rect=1280x800 src_px=0xff0b0a10 dst_px=0xff0b0a10 damage=1280x800+0,0 res_id=1 gpu_planes=0 cpu_planes=2
[40267764999] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 638,399 src_px=0x10202020 dst_before=0xff0b0a10 dst_after=0xff0c0b11
[40318287306] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[40363582908] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[40396048275] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Mounted atapi2 at /dev/storage/atapi2
[40397245812] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Provider loop online at /dev/storage/atapi2
[40409520690] [[32mINFO [0m] [clock] [CPU1] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[40410729942] [[32mINFO [0m] [clock] [CPU1] clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf
[40424040723] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[40425752169] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: bound wp_presentation
[40431031179] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: wl_region smoke test: create+add+set_opaque+destroy
[40432802520] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[40444656879] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[40499947026] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: bristle pid=8
[40502133276] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=28 frame=6
[40515928365] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[40577391129] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: imported dmabuf wl_buffer=111 480x320 format=0x34325241
ec[40881450753] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: sample rate set to 60 Hz
ho[40952750751] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=28 frame=6
[40960606929] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
 BDD_CONSOLE_READ[41577931593] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
Y_3053884_1777667500928511308
BDD_CONSOLE_READY_3053884_1777667500928511308
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hecho Solarized Warm > /[45611280264] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=3 height=28 frame=6
[45636074748] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_popup obj=23 assigned to xdg_surface=21
s[45667351389] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: imported dmabuf wl_buffer=121 160x96 format=0x34325241
ession/de[46176516969] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:31:42.011275427 unix_secs=1777667502.011275427
sk[46265268918] [[32mINFO [0m] [bloom::render] [CPU1] bloom: flat window overlays ready size=1280x800
top/theme
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[47965774857] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: frame callback done object=1000 data=23968
[47968248471] [[32mINFO [0m] [bloom::services::theme_service] [CPU1] bloom: reacting to theme change: Solarized Warm -> Facet Frame
[47969021793] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: wp_presentation_feedback.presented object=1001 tv=23.977968844 refresh_ns=16666666 seq=0
[48711787938] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: frame callback done object=1002 data=24349
[48712786749] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: wp_presentation_feedback.presented object=1003 tv=24.354146272 refresh_ns=16666666 seq=1
[48728165112] [[32mINFO [0m] [bloom::services::theme_service] [CPU1] bloom: reacting to theme change: Solarized Warm -> Facet Frame
[48734440128] [[32mINFO [0m] [bloom::services::theme_service] [CPU1] bloom: reacting to theme change: Solarized Warm -> Facet Frame
[49426857117] [[32mINFO [0m] [bloom::services::theme_service] [CPU1] bloom: reacting to theme change: Solarized Warm -> Facet Frame
[52183005105] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:31:45.014570051 unix_secs=1777667505.014570051
[58190962170] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:31:48.018535136 unix_secs=1777667508.018535136
[64198612434] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:31:51.022362875 unix_secs=1777667511.022362875
[70206261477] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:31:54.026206322 unix_secs=1777667514.026206322
[74809901298] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 12:31:56 UTC-8 system_unix=1777667516.327396361
[76214111358] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:31:57.030114828 unix_secs=1777667517.030114828
[82221982524] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:32:00.034041534 unix_secs=1777667520.034041534
[88229759706] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:32:03.037916018 unix_secs=1777667523.037916018
[94237454091] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:32:06.041783637 unix_secs=1777667526.041783637
[100245197580] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:32:09.045657444 unix_secs=1777667529.045657444
[106253337003] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:32:12.049648220 unix_secs=1777667532.049648220
[112261565790] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:32:15.053843430 unix_secs=1777667535.053843430
[118275352338] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:32:18.060713192 unix_secs=1777667538.060713192
[124276798668] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:32:21.061374746 unix_secs=1777667541.061374746
[130284467808] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:32:24.065242052 unix_secs=1777667544.065242052
[136291667985] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:32:27.068898752 unix_secs=1777667547.068898752
[142299623268] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:32:30.072869628 unix_secs=1777667550.072869628
[148307328048] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:32:33.076695750 unix_secs=1777667553.076695750
[149082175077] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 12:32:33 UTC-8 system_unix=1777667553.463935488
[154314996297] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:32:36.080567181 unix_secs=1777667556.080567181
[160323503109] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:32:39.084776235 unix_secs=1777667559.084776235
[166330957551] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:32:42.088520435 unix_secs=1777667562.088520435
[172338249369] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:32:45.092191011 unix_secs=1777667565.092191011
[178346120865] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:32:48.096130455 unix_secs=1777667568.096130455
[184354159770] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:32:51.100125933 unix_secs=1777667571.100125933
[190361582631] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:32:54.103843370 unix_secs=1777667574.103843370
[196369465248] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:32:57.107780174 unix_secs=1777667577.107780174
[202378864512] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:33:00.112459329 unix_secs=1777667580.112459329
[208385165769] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:33:03.115638057 unix_secs=1777667583.115638057
[214392928101] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:33:06.119500529 unix_secs=1777667586.119500529
[220401465537] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:33:09.123754166 unix_secs=1777667589.123754166
[223380554694] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 12:33:10 UTC-8 system_unix=1777667590.612924079
[226408512825] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:33:12.127293419 unix_secs=1777667592.127293419
[232416935091] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:33:15.131495526 unix_secs=1777667595.131495526
[238423822692] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:33:18.134954474 unix_secs=1777667598.134954474
[244432371876]
```
</details>
