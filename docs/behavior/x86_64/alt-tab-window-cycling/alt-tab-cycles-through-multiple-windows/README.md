# ❌ Scenario: Alt+Tab cycles through multiple windows

> Last run: 2026-05-01 12:03:04

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 10540ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 253ms | - [📜](./02/serial.log) - |
| 3 | And I type "clock &" on the serial console | ✅ | 180ms | - [📜](./03/serial.log) - |
| 4 | And I wait for 2 seconds | ✅ | 2000ms | - [📜](./04/serial.log) - |
| 5 | And I type "clock &" on the serial console | ✅ | 179ms | - [📜](./05/serial.log) - |
| 6 | And I wait for 2 seconds | ✅ | 2000ms | - [📜](./06/serial.log) - |
| 7 | And I type "clock &" on the serial console | ✅ | 179ms | - [📜](./07/serial.log) - |
| 8 | And I wait for 5 seconds | ✅ | 5002ms | - [📜](./08/serial.log) - |
| 9 | When I press alt+tab | ✅ | 262ms | - [📜](./09/serial.log) - |
| 10 | Then the latest output should contain "bloom: focus cycled from None" within 10s | ✅ | 0ms | - - - |
| 11 | When I press alt+tab | ✅ | 258ms | - - - |
| 12 | Then the latest output should contain "bloom: focus cycled from Some" within 10s | ❌ | 11026ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[32885897682] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[33137156217] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[33156552264] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[33186937740] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[33218077860] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[33236957424] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[33238635705] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[33276165153] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[33278264283] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[33297866217] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[33312393081] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[33313039782] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[33347715555] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[33348738918] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[33435062991] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[33514960116] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[33536248185] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[33609363513] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[33708566760] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[33734051703] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[33755454447] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[33850987170] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[33902840400] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[34036488618] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[34273822671] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=904845150 elapsed_us=452422
[34274688393] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[34350080919] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[34353301917] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[34416196056] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[34453824537] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[34462477632] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[34463896434] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[34553896740] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[34557580200] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[34567031466] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[34709426829] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[34711106793] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[34747493286] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[34749805827] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[34750062831] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[34751867139] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[34773834777] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[34785202122] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[34788420579] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[34789965012] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[34801311072] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[34805173953] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[34806839727] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[34816660956] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[34818878655] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[34823272638] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[34827867657] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[34843696734] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[34855774734] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[34866166797] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[34888448364] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[34892690481] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[34984468761] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[34995378957] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[35005762935] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: cursor queue (queue 1) configured
[35007137913] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[35008491606] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: composition paths: gpu_opaque_copy=enabled cpu_alpha_blend=enabled
[35018171496] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: host scanout 1280x800 enabled=true (bootfb was 1920x1080)
[35060236365] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[35225819244] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[35270271135] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor DMA buffer ready (16 pages at phys=0x2723000)
[35329320642] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[35337503256] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[35341995678] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[35344331484] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[35387905278] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
cl[35570742174] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
oc[35744651778] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
k [35767872360] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[35768927931] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[35793532632] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
&
[35934264399] [[32mINFO [0m] [clock] [CPU2] clock: running at low scheduler priority tid=11
[1] 11
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[36020959722] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[36040759425] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1280x800 @ 60000mHz
[36041652735] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[36042264291] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[36042949470] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports GPU blit (hardware transfer/flush)
[36043607490] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports direct scanout (zero-copy path to display)
[36044418861] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports partial flush (damage regions)
[36045214788] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports resource cache (pre-allocated buffer pool)
[36046001640] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[36351450465] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[36352518279] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[36353197254] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[36360431217] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Facet Frame
[36598340064] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[36752167980] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hwrng' name='hwrng' class=Other kind='dev.rng.HwRng' start='_RNvCs7wlZbVUrQWf_5hwrng20thingos_driver_start'
[36901848126] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[37051821807] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[37202350284] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[37286127846] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[37487101806] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=48x48 hotspot=11,6
[37496542809] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[37498732194] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[37529294187] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[37531099320] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[37532126049] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1280x800 @ 60000mHz ready
[37532305668] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: starting
[37542518739] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[37544143263] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: listening on /run/wayland-0
[37545012087] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[37572841284] [[32mINFO [0m] [clock] [CPU2] clock: connected to /run/wayland-0
[37692846543] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[37851746988] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[37853703360] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=1 rect=1280x800 src_px=0xff0b0a10 dst_px=0xff0b0a10 damage=1280x800+0,0 res_id=1 gpu_planes=0 cpu_planes=2
[37855403487] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 638,399 src_px=0x10202020 dst_before=0xff0b0a10 dst_after=0xff0c0b11
[37857517467] [[32mINFO [0m] [clock] [CPU2] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[37858502550] [[32mINFO [0m] [clock] [CPU2] clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf
[38011938327] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[38019127278] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[38084113848] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=13)
[38103134256] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=14)
[38110918857] [[32mINFO [0m] [clock] [CPU2] clock: running at low scheduler priority tid=14
[38113234236] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: connected to /run/wayland-0
[38114070192] [[32mINFO [0m] [clock] [CPU2] clock: connected to /run/wayland-0
[38178712572] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=28 frame=6
[38190115788] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[38193030381] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[38376288588] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[38443945386] [[32mINFO [0m] [clock] [CPU2] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[38445031680] [[32mINFO [0m] [clock] [CPU2] clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf
[38451381408] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[38532319221] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/xhci' name='xhci' class=Block kind='dev.usb.Xhci' start='thingos_driver_start_safe'
[38534022681] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 15 driver(s) found
[38592982593] [[32mINFO [0m] [ahci_disk] [CPU3] AHCI: Starting AHCI VFS driver
[38630018757] [[32mINFO [0m] [ahci_disk] [CPU3] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[38646021150] [[32mINFO [0m] [ahci_disk] [CPU3] AHCI: Mounted atapi2 at /dev/storage/atapi2
[38647392729] [[32mINFO [0m] [ahci_disk] [CPU3] AHCI: Provider loop online at /dev/storage/atapi2
[38664311895] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: bristle pid=8
[38708687259] [[32mINFO [0m] [ps2_mouse] [CPU3] ps2_mouse: bristle pid=8
[38741903970] [[32mINFO [0m] [rtc_cmos] [CPU1] RTC: claimed /sys/devices/isa-0070 (handle=2)
[38746947888] [[32mINFO [0m] [ps2_mouse] [CPU3] ps2_mouse: sample rate set to 60 Hz
[38763864117] [[32mINFO [0m] [rtc_cmos] [CPU1] RTC: 2026-05-01 19:03:15 = 1777662195 unix_secs
[38765023044] [[32mINFO [0m] [kernel::time] [CPU1] System clock anchored: unix_secs=1777662195, mono_ns=19382343634, offset=1777662175617656366ns
[38765992419] [[32mINFO [0m] [rtc_cmos] [CPU1] RTC: System clock anchored to 1777662195 unix_secs
[38783597358] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:03:15.008252937 unix_secs=1777662195.008252937
[38865980670] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[38867439336] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: bound wp_presentation
[38874187242] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: wl_region smoke test: create+add+set_opaque+destroy
[38909766225] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=28 frame=6
[38920911546] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[38985048036] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: imported dmabuf wl_buffer=111 480x320 format=0x34325241
[39057354897] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
[42563259816] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=3 height=28 frame=6
[42571821831] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
clock &
[2] 19
[43070768136] [[32mINFO [0m] [clock] [CPU1] clock: running at low scheduler priority tid=19
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[43074427737] [[32mINFO [0m] [clock] [CPU1] clock: connected to /run/wayland-0
[43378311504] [[32mINFO [0m] [clock] [CPU1] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[43379447892] [[32mINFO [0m] [clock] [CPU1] clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf
[43440026883] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=4 height=28 frame=6
[43448931999] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[44809776000] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:03:18.022434212 unix_secs=1777662198.022434212
[44939716635] [[32mINFO [0m] [bloom::render] [CPU1] bloom: flat window overlays ready size=1280x800
[48005242296] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=5 height=28 frame=6
[48025153869] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_popup obj=23 assigned to xdg_surface=21
[48055122522] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: imported dmabuf wl_buffer=121 160x96 format=0x34325241
[48305248134] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: frame callback done object=1000 data=24146
[48308456031] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: wp_presentation_feedback.presented object=1001 tv=24.150379671 refresh_ns=16666666 seq=0
[48742941555] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: frame callback done object=1002 data=24359
[48744770547] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: wp_presentation_feedback.presented object=1003 tv=24.368703711 refresh_ns=16666666 seq=1
clock &
[3] 20
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[50260471206] [[32mINFO [0m] [clock] [CPU2] clock: running at low scheduler priority tid=20
[50265342138] [[32mINFO [0m] [clock] [CPU2] clock: connected to /run/wayland-0
[50559039663] [[32mINFO [0m] [clock] [CPU2] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[50560534926] [[32mINFO [0m] [clock] [CPU2] clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf
[50616156690] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=6 height=28 frame=6
[50623989867] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[50837825148] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:03:21.036534818 unix_secs=1777662201.036534818
[56866282209] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:03:24.050749554 unix_secs=1777662204.050749554
[62894595093] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:03:27.064907778 unix_secs=1777662207.064907778
[66914030931] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: read scancode 0x38
[66917754816] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: edge detected: Down { key: LeftAlt, mods: Mods(4), repeat: false }
[66921034818] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: sent LeftAlt to bristle (pid=8)
[66925316502] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: read scancode 0x0f
[66926743785] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: edge detected: Down { key: Tab, mods: Mods(4), repeat: false }
[66929628711] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: sent Tab to bristle (pid=8)
[66935414766] [[32mINFO [0m] [bristle] [CPU2] bristle: KeyDown received: LeftAlt
[66939288273] [[32mINFO [0m] [bristle] [CPU2] bristle: KeyDown received: Tab
[66943555569] [[32mINFO [0m] [bloom::input] [CPU1] bloom: KeyDown received: LeftAlt (raw=0x00e2, mods=Mods(4), repeat=false)
[66945164880] [[32mINFO [0m] [bloom::input] [CPU1] bloom: KeyDown received: Tab (raw=0x002b, mods=Mods(4), repeat=false)
[66948036573] [[32mINFO [0m] [bloom::input] [CPU1] bloom: focus cycled from None to Some(6) (forward=true)
[67088798865] [
```
</details>
