# ❌ Scenario: sprout launches the default Wayland client

> Last run: 2026-04-30 21:37:18

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 10938ms | - [📜](./01/serial.log) - |
| 2 | Then the serial output should contain "wayland-server: listening on /run/wayland-0" within 180s | ✅ | 1123ms | - [📜](./02/serial.log) - |
| 3 | And the serial output should contain "SPROUT: Spawned wayland_hello" within 180s | ✅ | 1ms | - - - |
| 4 | And the serial output should contain "wayland_hello: connected to /run/wayland-0" within 180s | ✅ | 1ms | - - - |
| 5 | And the serial output should contain "wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf" within 180s | ✅ | 307ms | - [📜](./05/serial.log) - |
| 6 | And the serial output should contain "wayland-server: xdg_surface obj=" within 180s | ❌ | 182645ms | - [📜](./06/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[34241056476] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[34516902618] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[34539793101] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[34574854545] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[34607150952] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[34626622668] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[34628423280] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[34668000147] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[34669691562] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[34690020816] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[34704759606] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[34705395945] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[34741239027] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[34742291001] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[34833448056] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[34918486548] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[34940690862] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[35015369268] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[35119681575] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[35151816249] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[35174946015] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[35274692838] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[35328594213] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[35461868079] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[35711428005] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=948598596 elapsed_us=474299
[35712240960] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[35807170674] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[35810926602] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[35848329429] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[35878175685] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[35887879632] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[35889497061] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[35988619656] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[35992378158] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[36003594627] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[36176757342] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[36179162085] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[36215471556] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[36218566527] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[36219947148] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[36222206031] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[36242719425] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[36270877005] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[36274532910] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[36276310224] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[36282875805] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[36286902267] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[36288540552] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[36299742435] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[36302373525] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[36307284090] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[36311775687] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[36312388464] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[36323078946] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[36335710917] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[36363646143] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[36370733223] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[36468578487] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[36484002522] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[36492524805] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[36555645060] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[36735721473] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[36901394211] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[37076273718] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[37086329346] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[37094830047] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[37099635144] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[37102485453] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[37251833157] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[37417542756] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[37491777741] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[37526355999] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[37527829911] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[37680558069] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[37777973871] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[37798547160] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1920x1080 @ 60000mHz
[37799419878] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[37800076875] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[37800676155] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[38172962553] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[38330072583] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[38435882133] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[38436987798] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[38437889259] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[38445430584] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Facet Frame
[39016803771] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[39114931845] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[39354509634] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[39451897089] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=96x96 hotspot=21,12
[39462612123] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[39464869752] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[39517435584] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[39518952396] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: starting
[39519081228] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[39519892467] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1920x1080 @ 60000mHz ready
[39537758403] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: listening on /run/wayland-0
[39539338212] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[39561352017] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=12)
[39576115920] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: connected to /run/wayland-0
[39584012490] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=13)
[39615975960] [[32mINFO [0m] [clock] [CPU1] clock: running at low scheduler priority tid=13
[39620464686] [[32mINFO [0m] [clock] [CPU1] clock: connected to /run/wayland-0
[39977110272] [[32mINFO [0m] [clock] [CPU1] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[39988819002] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[40124408841] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=1 rect=1920x1080 src_px=0xff0b0a10 dst_px=0xff0b0a10 damage=1920x1080+0,0 res_id=1
[40126927302] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 958,539 src_px=0x20181818 dst_before=0xff0b0a10 dst_after=0xff0d0c11
[40199975277] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[40209524784] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
[40427012142] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[40988504535] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[40990560699] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: bound wp_presentation
[41036803863] [[32mINFO [0m] [clock] [CPU1] clock: waiting for system clock anchor
[41765192238] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[41767221309] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[41827050870] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Starting AHCI VFS driver
[41857110438] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[41875936410] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Mounted atapi2 at /dev/storage/atapi2
[41876930832] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Provider loop online at /dev/storage/atapi2
[41906252124] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: claimed /sys/devices/isa-0070 (handle=2)
[41929706874] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: 2026-05-01 04:37:46 = 1777610266 unix_secs
[41931687369] [[32mINFO [0m] [kernel::time] [CPU3] System clock anchored: unix_secs=1777610266, mono_ns=20965578331, offset=1777610245034421669ns
[41941459494] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: System clock anchored to 1777610266 unix_secs
[41960974110] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: bristle pid=8
[41962823265] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:37:46.014531847 unix_secs=1777610266.014531847
[41973210444] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: bristle pid=8
[41980827042] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x47 (is_aux=false)
[41986958343] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfa (is_aux=true)
[41989079946] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xaa (is_aux=true)
[41991017343] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x00 (is_aux=true)
[41996551311] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfa (is_aux=true)
[42001560942] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfa (is_aux=true)
[42003134844] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: sample rate set to 60 Hz
[42007951392] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfa (is_aux=true)
[42012186084] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfa (is_aux=true)
[42016903467] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfa (is_aux=true)
[42019064835] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x00 (is_aux=true)
[42021219867] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x03 (is_aux=true)
[42023448060] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x3c (is_aux=true)
[42028464456] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfa (is_aux=true)
[43652067228] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[43658515659] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[44027938713] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=28 frame=6
[44039523792] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[44192757642] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=28 frame=6
[44199476475] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[44210053008] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: imported dmabuf wl_buffer=111 480x320 format=0x34325241
[46920470124] [[32mINFO [0m] [bloom::render] [CPU1] bloom: flat window overlays ready size=1920x1080
[47180184711] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:37:48 UTC-8 system_unix=1777610268.623799079
[47961454101] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:37:49.014878317 unix_secs=1777610269.014878317
[48360412419] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=3 height=28 frame=6
[48374158734] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_popup obj=23 assigned to xdg_surface=21
[48404433000] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: imported dmabuf wl_buffer=121 160x96 format=0x34325241
[49557546918] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: frame callback done object=1000 data=24771
[49559583084] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: wp_presentation_feedback.presented object=1001 tv=24.774938446 refresh_ns=16666666 seq=0
[51075366837] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: frame callback done object=1002 data=25531
[51076640175] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: wp_presentation_feedback.presented object=1003 tv=25.535700795 refresh_ns=16666666 seq=1
[53226105966] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:37:51 UTC-8 system_unix=1777610271.647246754
[53961954321] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:37:52.015338984 unix_secs=1777610272.015338984
[59228474415] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:37:54 UTC-8 system_unix=1777610274.648442512
[59963040324] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:37:55.015846708 unix_secs=1777610275.015846708
[65229006612] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:37:57 UTC-8 system_unix=1777610277.648734152
[65963725212] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:37:58.016229594 unix_secs=1777610278.016229594
[71965076568] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:01.016902467 unix_secs=1777610281.016902467
[72072157971] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:38:01 UTC-8 system_unix=1777610281.070175769
[77965308729] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:04.017014934 unix_secs=1777610284.017014934
[78073098015] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:38:04 UTC-8 system_unix=1777610284.070669304
[83966139972] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:07.017445818 unix_secs=1777610287.017445818
[84075184182] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:38:07 UTC-8 system_unix=1777610287.071794805
[89967060249] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:10.017897426 unix_secs=1777610290.017897426
[90076566657] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:38:10 UTC-8 system_unix=1777610290.072371087
[95968037022] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:13.018354660 unix_secs=1777610293.018354660
[96951160086] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:38:13 UTC-8 system_unix=1777610293.509658941
[101968716234] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:16.018734394 unix_secs=1777610296.018734394
[102952733136] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:38:16 UTC-8 system_unix=1777610296.510560438
[107969488638] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:19.019125150 unix_secs=1777610299.019125150
[109766160174] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:38:19 UTC-8 system_unix=1777610299.917122355
[113970289620] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:22.019528562 unix_secs=1777610302.019528562
[116573110929] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:38:23 UTC-8 system_unix=1777610303.320594762
[119971296027] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:25.020015447 unix_secs=1777610305.020015447
[122574021999] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:38:26 UTC-8 system_unix=1777610306.321214835
[125972029557] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:28.020398200 unix_secs=1777610308.020398200
[129451005585] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:38:29 UTC-8 system_unix=1777610309.759697454
[131972940495] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:31.020849412 unix_secs=1777610311.020849412
[135451244511] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:38:32 UTC-8 system_unix=1777610312.759792679
[137974226412] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:34.021371822 unix_secs=1777610314.021371822
[141485507559] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:38:35 UTC-8 system_unix=1777610315.776937436
[143974915953] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:37.021807821 unix_secs=1777610317.021807821
[147488211354] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:38:38 UTC-8 system_unix=1777610318.778338454
[149976042282] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:40.022387155 unix_secs=1777610320.022387155
[153490702365] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:38:41 UTC-8 system_unix=1777610321.779440343
[155976572169] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:43.022636886 unix_secs=1777610323.022636886
[160331389119] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:38:45 UTC-8 system_unix=1777610325.199918212
[161977265340] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:46.022997810 unix_secs=1777610326.022997810
[166334012427] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:38:48 UTC-8 system_unix=1777610328.201196008
[167978005371] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:49.023380481 unix_secs=1777610329.023380481
[172336301346] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:38:51 UTC-8 system_unix=1777610331.202323423
[173978978877] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:52.023865353 unix_secs=1777610332.023865353
[179220475929] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:38:54 UTC-8 system_unix=1777610334.644143068
[179979847344] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:55.024297821 unix_secs=1777610335.024297821
[185316345093] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:38:57 UTC-8 system_unix=1777610337.692144244
[185980826064] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:58.024747152 unix_secs=1777610338.024747152
[191349611574] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:39:00 UTC-8 system_unix=1777610340.708823668
[191981679120] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:01.025187573 unix_secs=1777610341.025187573
[197414973855] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:39:03 UTC-8 system_unix=1777610343.741388648
[197986519962] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:04.027600915 unix_secs=1777610344.027600915
[203427952464] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:39:06 UTC-8 system_unix=1777610346.747901350
[204021818781] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:07.043531008 unix_secs=1777610347.043531008
[209591833275] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:39:09 UTC-8 system_unix=1777610349.829885662
[210051670818] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:10.060184709 unix_secs=1777610350.060184709
[215890070319] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:39:12 UTC-8 system_unix=1777610352.979203652
[216052446687] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:13.060574871 unix_secs=1777610353.060574871
[221909741856] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:39:15 UTC-8 system_unix=1777610355.988801590
[222067090872] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:16.067891254 unix_secs=1777610356.067891254
[228087916551] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:19.078309869 unix_secs=1777610359.078309869
[228166530768] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:39:19 UTC-8 system_unix=1777610359.117233913
[234088786404] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:22.078738641 unix_secs=1777610362.078738641
[234298622019] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:39:22 UTC-8 system_unix=1777610362.183454703
[240089722686] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:25.079209339 unix_secs=1777610365.079209339
[240343633728] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:39:25 UTC-8 system_unix=1777610365.205689347
[246090525318] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:28.079599551 unix_secs=1777610368.079599551
[247009517568] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:39:28 UTC-8 system_unix=1777610368.538964138
[252091241424] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:31.079989119 unix_secs=1777610371.079989119
[253277757810] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:39:31 UTC-8 system_unix=1777610371.673009464
[258092235093] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:34.080459982 unix_secs=1777610374.080459982
[259353567330] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:39:34 UTC-8 system_unix=1777610374.710433579
[264093237078] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:37.080990163 unix_secs=1777610377.080990163
[265618487442] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:39:37 UTC-8 system_unix=1777610377.843218339
[270101628192] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:40.085143992 unix_secs=1777610380.085143992
[271921399497] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:39:40 UTC-8 system_unix=1777610380.994583715
[276128575701] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:43.098631062 unix_secs=1777610383.098631062
[277923829953] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:39:43 UTC-8 system_unix=1777610383.995896343
[282129444762] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:46.099048878 unix_secs=1777610386.099048878
[284329149126] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:39:47 UTC-8 system_unix=1777610387.198480376
[288129946995] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:49.099346970 unix_secs=1777610389.099346970
[290429717853] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:39:50 UTC-8 system_unix=1777610390.248986648
[294130969341] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:52.099821612 unix_secs=1777610392.099821612
[296699043762] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:39:53 UTC-8 system_unix=1777610393.383702287
[300131797119] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:55.100255763 unix_secs=1777610395.100255763
[302703438312] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:39:56 UTC-8 system_unix=1777610396.385744346
[306133015947] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:58.100854815 unix_secs=1777610398.100854815
[308752392828] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:39:59 UTC-8 system_unix=1777610399.410343638
[312133634703] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:01.101156636 unix_secs=1777610401.101156636
[315087980196] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:40:02 UTC-8 system_unix=1777610402.578020139
[318134409846] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:04.101576795 unix_secs=1777610404.101576795
[321312219723] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:40:05 UTC-8 system_unix=1777610405.690111853
[324135293163] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:07.101986262 unix_secs=1777610407.101986262
[327386201703] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:40:08 UTC-8 system_unix=1777610408.727042948
[330136096356] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:10.102391802 unix_secs=1777610410.102391802
[333494827248] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:40:11 UTC-8 system_unix=1777610411.781391327
[336138858132] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:13.103776749 unix_secs=1777610413.103776749
[339623096043] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:40:14 UTC-8 system_unix=1777610414.845441080
[342171481806] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:16.120087398 unix_secs=1777610416.120087398
[345654381600] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:40:17 UTC-8 system_unix=1777610417.861147482
[348172259226] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:19.120481833 unix_secs=1777610419.120481833
[351685861461] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:40:20 UTC-8 system_unix=1777610420.876888650
[354173221941] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:22.120955254 unix_secs=1777610422.120955254
[357723719826] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:40:23 UTC-8 system_unix=1777610423.895788397
[360175127532] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:25.121877987 unix_secs=1777610425.121877987
[363957416691] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:40:27 UTC-8 system_unix=1777610427.012655210
[366208499811] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:28.138626463 unix_secs=1777610428.138626463
[369992803257] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:40:30 UTC-8 system_unix=1777610430.030360439
[372209440779] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:31.139073600 unix_secs=1777610431.139073600
[376059145539] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:40:33 UTC-8 system_unix=1777610433.063552717
[378210252717] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:34.139480394 unix_secs=1777610434.139480394
[382096489500] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:40:36 UTC-8 system_unix=1777610436.082378609
[384211160223] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:37.139927365 unix_secs=1777610437.139927365
[388971269346] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:40:39 UTC-8 system_unix=1777610439.519817587
[390211964373] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:40.140356797 unix_secs=1777610440.140356797
[394972954167] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:40:42 UTC-8 system_unix=1777610442.520586044
[396212659326] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:43.140704917 unix_secs=1777610443.140704917
[400974052248] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:40:45 UTC-8 system_unix=1777610445.520837161
[402213585378] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:46.141168504 unix_secs=1777610446.141168504
[406975057665] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:40:48 UTC-8 system_unix=1777610448.521526963
[408214414905] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:49.141584538 unix_secs=1777610449.141584538
[412976578674] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:40:51 UTC-8 system_unix=1777610451.522342247
[414215232684] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:52.141993230 unix_secs=1777610452.141993230
[418976903433] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:40:54 UTC-8 system_unix=1777610454.522475059
[420216250311] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:55.142477541 unix_secs=1777610455.142477541
[424978327422] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:40:57 UTC-8 system_unix=1777610457.523156231
[426216988923] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:40:58.142872141 unix_secs=1777610458.142872141
[431824734033] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:41:00 UTC-8 system_unix=1777610460.946368843
[432217996551] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:01.143304923 unix_secs=1777610461.143304923
[438218731203] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:04.143721402 unix_secs=1777610464.143721402
[438662616447] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:41:04 UTC-8 system_unix=1777610464.365460035
[444219635673] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:07.144171707 unix_secs=1777610467.144171707
[444664657305] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:41:07 UTC-8 system_unix=1777610467.366409794
[450220399695] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:10.144577296 unix_secs=1777610470.144577296
[451510454538] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:41:10 UTC-8 system_unix=1777610470.789250034
[456221299413] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:13.145013411 unix_secs=1777610473.145013411
[457510498698] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:41:13 UTC-8 system_unix=1777610473.789273995
[462223039014] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:16.145866810 unix_secs=1777610476.145866810
[463511831145] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:41:16 UTC-8 system_unix=1777610476.789727038
[468256677273] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:19.162699453 unix_secs=1777610479.162699453
[470320198491] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:41:20 UTC-8 system_unix=1777610480.194293462
[474257582436] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:22.163154048 unix_secs=1777610482.163154048
[476321751345] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:41:23 UTC-8 system_unix=1777610483.194900467
[480258385299] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:25.163563416 unix_secs=1777610485.163563416
[483199304808] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:41:26 UTC-8 system_unix=1777610486.633665863
[486259348872] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:28.164027778 unix_secs=1777610488.164027778
[489202900098] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:41:29 UTC-8 system_unix=1777610489.635405510
[492260015544] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:31.164385749 unix_secs=1777610491.164385749
[495212523135] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:41:32 UTC-8 system_unix=1777610492.637752374
[498260981724] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:34.164852405 unix_secs=1777610494.164852405
[501402650727] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:41:35 UTC-8 system_unix=1777610495.735146828
[504261842898] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:37.165282068 unix_secs=1777610497.165282068
[507404662017] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:41:38 UTC-8 system_unix=1777610498.736383160
[510262593027] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:40.165660069 unix_secs=1777610500.165660069
[513405027597] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:41:41 UTC-8 system_unix=1777610501.736634392
[516263511984] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:43.166109169 unix_secs=1777610503.166109169
[519406258470] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:41:44 UTC-8 system_unix=1777610504.737117416
[522264328443] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:46.166536968 unix_secs=1777610506.166536968
[525407038761] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:41:47 UTC-8 system_unix=1777610507.737558101
[528265229514] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:49.166955114 unix_secs=1777610509.166955114
[531409664412] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:41:50 UTC-8 system_unix=1777610510.739009130
[534266028483] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:52.167382632 unix_secs=1777610512.167382632
[538252143297] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:41:54 UTC-8 system_unix=1777610514.160032802
[540266911965] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:55.167828118 unix_secs=1777610515.167828118
[544386477954] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:41:57 UTC-8 system_unix=1777610517.227424415
[546267725091] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:41:58.168232800 unix_secs=1777610518.168232800
[550390226694] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:42:00 UTC-8 system_unix=1777610520.229169277
[552268772319] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:01.168734090 unix_secs=1777610521.168734090
[557268098046] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:42:03 UTC-8 system_unix=1777610523.668046757
[558269389161] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:04.169077672 unix_secs=1777610524.169077672
[564111650091] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:42:07 UTC-8 system_unix=1777610527.089812533
[564270372369] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:07.169551819 unix_secs=1777610527.169551819
[570271176288] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:10.169958745 unix_secs=1777610530.169958745
[570984743274] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:42:10 UTC-8 system_unix=1777610530.526558544
[576271977270] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:13.170369631 unix_secs=1777610533.170369631
[576988604511] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:42:13 UTC-8 system_unix=1777610533.528251315
[582272868936] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:16.170810415 unix_secs=1777610536.170810415
[582989636196] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:42:16 UTC-8 system_unix=1777610536.528991953
[588273987840] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:19.171333567 unix_secs=1777610539.171333567
[589033956753] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:42:19 UTC-8 system_unix=1777610539.550910078
[594274806147] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:22.171739404 unix_secs=1777610542.171739404
[595216669707] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:42:22 UTC-8 system_unix=1777610542.642308646
[600276369792] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:25.172503011 unix_secs=1777610545.172503011
[601232337651] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:42:25 UTC-8 system_unix=1777610545.650088482
[606276427812] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:28.172560384 unix_secs=1777610548.172560384
[607461027921] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:42:28 UTC-8 system_unix=1777610548.764443863
[612277434747] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:31.173044580 unix_secs=1777610551.173044580
[613496369904] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:42:31 UTC-8 system_unix=1777610551.782353857
[618277928301] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:34.173341137 unix_secs=1777610554.173341137
[619534879161] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:42:34 UTC-8 system_unix=1777610554.801379878
[624278819043] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:37.173780552 unix_secs=1777610557.173780552
[625600434723] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:42:37 UTC-8 system_unix=1777610557.834118158
[630279713184] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:40.174223101 unix_secs=1777610560.174223101
[631705053441] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:42:40 UTC-8 system_unix=1777610560.886553231
[636281910660] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:42:43.175283279 unix_secs=1777610563.175283279
[638011639782] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-04-30 20:42:44 UTC-8 system_unix=1777610564.039815348

```
</details>
