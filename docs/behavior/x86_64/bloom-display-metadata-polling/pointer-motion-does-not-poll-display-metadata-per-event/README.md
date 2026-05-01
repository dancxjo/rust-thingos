# ✅ Scenario: pointer motion does not poll display metadata per event

> Last run: 2026-05-01 11:21:23

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 11458ms | - [📜](./01/serial.log) - |
| 2 | Then the serial output should contain "bloom: registered bristle pointer sink" within 60s | ✅ | 1435ms | - [📜](./02/serial.log) - |
| 3 | And the serial output should contain "ps2_mouse: bristle pid=" within 60s | ✅ | 0ms | - - - |
| 4 | And the serial output should contain "ps2_mouse: sample rate set to" within 60s | ✅ | 109ms | - [📜](./04/serial.log) - |
| 5 | When I wait for the shell prompt | ✅ | 151ms | - [📜](./05/serial.log) - |
| 6 | And I type "loglevel 5" on the serial console | ✅ | 284ms | - [📜](./06/serial.log) - |
| 7 | Then the latest serial output should contain "Log level set to 5" | ✅ | 0ms | - - - |
| 8 | When I wait for 1 seconds | ✅ | 1001ms | - [📜](./08/serial.log) - |
| 9 | And I type "echo poll-baseline" on the serial console | ✅ | 411ms | - [📜](./09/serial.log) - |
| 10 | And I move the mouse 20 times | ✅ | 454ms | - [📜](./10/serial.log) - |
| 11 | And I wait for 1 seconds | ✅ | 1005ms | - [📜](./11/serial.log) - |
| 12 | Then the latest serial output should contain "bloom: pointer moved" | ✅ | 5ms | - - - |
| 13 | And the latest serial output should contain at most 8 occurrences of "DISP: DISPLAY_OP_GET_INFO requested" | ✅ | 6ms | - - - |
| 14 | And the latest serial output should contain at most 12 occurrences of "VFS: sys_fs_open path='/session/wayland/" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[35860036443] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[36125021757] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[36145372197] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[36181600125] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[36215895705] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[36235916805] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[36237844599] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[36278161986] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[36279954018] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[36300282579] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[36314979492] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[36315631374] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[36353356974] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[36354489864] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[36450695490] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[36537077709] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[36559501308] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[36632463285] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[36736573005] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[36763523016] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[36786975786] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[36891025908] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[36944259528] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[37082176692] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[37331382594] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=956386761 elapsed_us=478193
[37332250263] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[37409691198] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[37414926945] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[37454588391] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[37492663197] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[37501924086] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[37503729681] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[37603770567] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[37608913881] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[37611102738] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[37771755351] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[37774764027] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[37815745836] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[37818403260] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[37847957598] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[37850596509] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[37869925830] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[37886671284] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[37895935341] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[37897994970] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[37908233418] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[37915178037] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[37917768933] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[37936907448] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[37941376737] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[37946514177] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[37951386594] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[37954305378] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[37973455212] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[37986591522] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[38019277824] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[38026944747] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[38152393521] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[38167781619] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[38178274596] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: cursor queue (queue 1) configured
[38179623471] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[38180705442] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: composition paths: gpu_opaque_copy=enabled cpu_alpha_blend=enabled
[38192202543] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: host scanout 1280x800 enabled=true (bootfb was 1920x1080)
[38205416040] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[38386817469] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[38470026573] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor DMA buffer ready (16 pages at phys=0x2723000)
[38547705570] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[38560602564] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[38567332518] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[38570822400] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[38577607959] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[38755086447] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[38985943920] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[39095113464] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[39156635100] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[39162081618] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[39272241030] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[39491524974] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hwrng' name='hwrng' class=Other kind='dev.rng.HwRng' start='_RNvCs7wlZbVUrQWf_5hwrng20thingos_driver_start'
[39728749071] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[39750043146] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[39911368794] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[40074515580] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1280x800 @ 60000mHz
[40075782912] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[40076739318] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[40077489243] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports GPU blit (hardware transfer/flush)
[40078188282] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports direct scanout (zero-copy path to display)
[40078842474] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports partial flush (damage regions)
[40079471091] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports resource cache (pre-allocated buffer pool)
[40080300579] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[40401445095] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[40402719390] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[40403763081] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[40412511843] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Facet Frame
[40664592837] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[40828640424] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[40993601088] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[41153433552] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=11)
[41177755146] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[41177756136] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=12)
[41193844791] [[32mINFO [0m] [clock] [CPU3] clock: running at low scheduler priority tid=12
[41286341382] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/clouds.bmp
[41532391593] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=96x96 hotspot=21,12
[41545681122] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[41706466263] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/xhci' name='xhci' class=Block kind='dev.usb.Xhci' start='thingos_driver_start_safe'
[41708270505] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 15 driver(s) found
[41716906209] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[41719714641] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[41755727541] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: starting
[41769893913] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: listening on /run/wayland-0
[41770945161] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[41776775469] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[41780048970] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[41781156945] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1280x800 @ 60000mHz ready
[41839527576] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: connected to /run/wayland-0
[41912778831] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Starting AHCI VFS driver
[41952562179] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[41971899882] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Mounted atapi2 at /dev/storage/atapi2
[41973527046] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Provider loop online at /dev/storage/atapi2
[41997993114] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: claimed /sys/devices/isa-0070 (handle=2)
[42024405423] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: 2026-05-01 18:21:57 = 1777659717 unix_secs
[42025909134] [[32mINFO [0m] [kernel::time] [CPU3] System clock anchored: unix_secs=1777659717, mono_ns=21012787075, offset=1777659695987212925ns
[42026917845] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: System clock anchored to 1777659717 unix_secs
[42028874316] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 18:21:57.000404201 unix_secs=1777659717.000404201
[42031437624] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: bristle pid=8
[42145434474] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=1 rect=1280x800 src_px=0xff0b0a10 dst_px=0xff0b0a10 damage=1280x800+0,0 res_id=1 gpu_planes=0 cpu_planes=2
[42147034941] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 638,399 src_px=0x20181818 dst_before=0xff0b0a10 dst_after=0xff0d0c11
[42165933018] [[32mINFO [0m] [clock] [CPU3] clock: connected to /run/wayland-0
[42175809093] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[42184997316] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: bristle pid=8
[42265564671] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[42308111208] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[42309912447] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[42316129713] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[42321295929] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: bound wp_presentation
[42369673863] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: sample rate set to 60 Hz
[42535608819] [[32mINFO [0m] [clock] [CPU3] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[42537307428] [[32mINFO [0m] [clock] [CPU3] clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf
[42554140365] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=28 frame=6
[42570448602] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[42725833986] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: imported dmabuf wl_buffer=111 480x320 format=0x34325241
[43140785556] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=28 frame=6
[43196258688] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[43279267053] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/clouds.bmp
loglevel 5
[43916399406] [[32mINFO [0m] [user.print] [CPU3] Log level set to 5
[43922764380] [[35mTRACE[0m] [kernel::signal] [CPU3] signal: queued child event ppid=6 child_pid=18 status=0x0 wake_tids=1 queue_len=1
[43932147897] [[34mDEBUG[0m] [sh] [CPU3] sh: spawned '/bin/loglevel' pid=18 idx=0 background=false pending_pgid=0
[43934620191] [[34mDEBUG[0m] [sh] [CPU3] sh: setpgid leader pid=18 -> pgid=18
[43936361040] [[34mDEBUG[0m] [sh] [CPU3] sh: cleaning up 0 pipes
[43938231348] [[34mDEBUG[0m] [sh] [CPU3] sh: foreground handoff start job_id=1 pgid=18 shell_pgid=5 cmd='loglevel 5'
[43939947612] [[34mDEBUG[0m] [sh] [CPU3] sh: foreground handoff done job_id=1 pgid=18 cmd='loglevel 5'
[43941325329] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=6
[?25l[43964895744] [[34mDEBUG[0m] [kernel::inbox] [CPU3] inbox::close: inbox closed, 0 messages dropped on close
[43974621801] [[35mTRACE[0m] [kernel::sched::lifecycle] [CPU3] waitpid: queued status delivered parent_pid=6 parent_tid=6 target_pid=-1 child_pid=18 status=0x0 flags=0x6
[43987093194] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=6
[?25h[43989572649] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=95
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [43991195490] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=6
[?25h[43992119028] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[44041244247] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU3] SENDMSG: thing=6 data_len=16 caps_len=1 node=Pointer { addr: 0xffffffffb0205d90, metadata: DynMetadata(0xffffffff802d8c70) }
[44051645913] [[34mDEBUG[0m] [pistil::font] [CPU3] pistil: opening font /share/fonts/DSEG7Classic-Regular.ttf
[44052067884] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU1] RECVMSG: thing=19 data_len=16 caps_len=1 node=Pointer { addr: 0xffffffffb03a5270, metadata: DynMetadata(0xffffffff802d9528) }
[44053400127] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/share/fonts/DSEG7Classic-Regular.ttf' tid=12
[44057697486] [[34mDEBUG[0m] [pistil::font] [CPU3] pistil: stat font /share/fonts/DSEG7Classic-Regular.ttf
[44060107146] [[34mDEBUG[0m] [pistil::font] [CPU3] pistil: reading font 23272 bytes from /share/fonts/DSEG7Classic-Regular.ttf
[44066376486] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[44067946890] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[44069534982] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[44071315530] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[44073985296] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[44076547317] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[44078747823] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[44080882791] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[44091126915] [[34mDEBUG[0m] [pistil::font] [CPU3] pistil: font read returned 23272 of 23272 bytes
[44092434870] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=8
[44093746257] [[34mDEBUG[0m] [pistil::font] [CPU3] pistil: parsing font /share/fonts/DSEG7Classic-Regular.ttf
[44097439815] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=21
[44099059158] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=88 tid=10
[44144868075] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[44146412574] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_IMPORT_BUFFER requested: memfd=12, size=1280x800
[44147552955] [[34mDEBUG[0m] [kernel::syscall::handlers::memory] [CPU3] sys_vm_map: large backing file mapping offset=0 len=4096000 total_size=4096000
[44185802100] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] DISP: imported buffer as ID=3
[44187646998] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=9
[44190870999] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=9
[44194622208] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=9 -> OK(12)
[44196222675] [[34mDEBUG[0m] [bloom::display] [CPU1] bloom: imported buffer 1280x800 as ID=3
[44197369326] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=52 tid=10
[44203612134] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[44206391724] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_RELEASE_BUFFER requested: id=1 size=4096000
[44235999126] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=10
[44238442347] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=10
[44241202566] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=10
[44248452237] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=10 -> OK(8)
[44254699236] [[34mDEBUG[0m] [pistil::font] [CPU3] pistil: parsed font /share/fonts/DSEG7Classic-Regular.ttf
[44288964390] [[34mDEBUG[0m] [pistil::font] [CPU3] pistil: opening font /share/fonts/Inter-Regular.ttf
[44290162620] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/share/fonts/Inter-Regular.ttf' tid=12
[44293026723] [[34mDEBUG[0m] [pistil::font] [CPU3] pistil: stat font /share/fonts/Inter-Regular.ttf
[44294285508] [[34mDEBUG[0m] [pistil::font] [CPU3] pistil: reading font 876576 bytes from /share/fonts/Inter-Regular.ttf
[44311593348] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=200 tid=10
[44369138121] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[44370682455] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_COMMIT requested: planes=2
[44408052744] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[44411028420] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[44412963243] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[44415259977] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[44416902156] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[44418724878] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[44420192850] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[44421665673] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[44625772026] [[34mDEBUG[0m] [pistil::font] [CPU3] pistil: font read returned 876576 of 876576 bytes
[44627110143] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=8
[44628698202] [[34mDEBUG[0m] [pistil::font] [CPU3] pistil: parsing font /share/fonts/Inter-Regular.ttf
[44746362738] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[44749000560] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[44750705010] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[44752311153] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[44753988774] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[44755529907] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[44756949666] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[44758390149] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[44936498145] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT transfer begin seq=3 res_id=1 damage=1280x800+0,0
[44939450061] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT transfer end seq=3 res_id=1
[44940656310] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT flush begin seq=3 res_id=1
[44942760192] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT flush end seq=3 res_id=1
[44943969378] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT complete (seq=3)
[44947013001] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=11
[44949764772] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=11
[44955354213] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=11 -> OK(8)
[44957413182] [[35mTRACE[0m] [bloom::frame_clock] [CPU1] bloom: frame interval 1133503717ns (target 16666666ns)
[44958654708] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=48 tid=10
[44964023379] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[44965563753] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_GET_INFO requested
[44966875239] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: Returning dimensions 1280x800
[44968349316] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=12
[44970828507] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=12
[44973375777] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=12 -> OK(48)
[44980993992] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/components' tid=10
[44984648280] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=21
[44986510668] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/status' tid=10
[44990078958] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=21
[44993169210] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/index' tid=10
[44997765615] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=21
[44999542995] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/index' tid=10
[45004823622] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=21
[45006532857] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/events/latest' tid=10
[45012315381] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=21
[45085938381] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[45088041966] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[45089810766] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[45091436907] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[45092922468] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[45094552701] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[45096144852] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[45097717995] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[45222471921] [[34mDEBUG[0m] [pistil::font] [CPU2] pistil: parsed font /share/fonts/Inter-Regular.ttf
[45249866277] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU1] wayland-server: xdg_surface.ack_configure serial=1 accepted
[45251619864] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU1] wayland-server: wl_surface obj=10 registered frame callback cb=1000
[45253288443] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU1] wayland-server: wp_presentation.feedback surface_obj=10 fb=1001
[45254314545] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU1] wayland-server: surface 1 ready for mapping
[45264264936] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=64
[45265072083] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(14)
[45266641233] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=17 port_id=PortId(14) mode=Write
[45267296745] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=18 port_id=PortId(14) mode=Read
[45272907207] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=88 tid=10
[45399979614] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[45401087226] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_IMPORT_BUFFER requested: memfd=10, size=480x320
[45406812825] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] DISP: imported buffer as ID=4
[45407759925] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=13
[45409763718] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=13
[45412511727] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=13 -> OK(12)
[45413811795] [[34mDEBUG[0m] [bloom::display] [CPU1] bloom: imported buffer 480x320 as ID=4
[45417620985] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=3 height=28 frame=6
[45418762323] [[34mDEBUG[0m] [bloom::services::wayland_cmd] [CPU1] wayland-cmd: created surface bloom_id=3
[45426553095] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[45429123201] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[45431389476] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[45433524180] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[45435652185] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[45437941032] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[45440031351] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[45442061973] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[45639394053] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU1] BRIDGE: handle=17 -> fd=23 node=0xffffffffb016c670 port=0xffffffffb02cf0b0
[45640929279] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=23
[45642376725] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU1] BRIDGE: handle=18 -> fd=23 node=0xffffffffb016c670 port=0xffffffffb02cf0b0
[45643004616] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=23
[45647231421] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU1] wayland-server: xdg_surface obj=21 created for surface=3
[45649640388] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: xdg_popup obj=23 assigned to xdg_surface=21
[45650777304] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU1] wayland-server: xdg_popup.configure obj=23 x=0 y=0 width=160 height=96
[45653705955] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU1] wayland-server: xdg_surface.configure serial=2 sent to obj=21
[45658818513] [[34mDEBUG[0m] [kernel::syscall::handlers::memory] [CPU1] sys_vm_map: large backing file mapping offset=0 len=4096000 total_size=4096000
[45675573372] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU2] SENDMSG: thing=6 data_len=76 caps_len=1 node=Pointer { addr: 0xffffffffb01c16f0, metadata: DynMetadata(0xffffffff802d8c70) }
[45711850305] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU1] RECVMSG: thing=17 data_len=76 caps_len=1 node=Pointer { addr: 0xffffffffb01d2df0, metadata: DynMetadata(0xffffffff802d9528) }
[45716521257] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: imported dmabuf wl_buffer=121 160x96 format=0x34325241
[45722665329] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU1] wayland-server: xdg_surface.ack_configure serial=2 accepted
[45723846795] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU1] wayland-server: wl_surface obj=20 registered frame callback cb=1002
[45724846134] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU1] wayland-server: wp_presentation.feedback surface_obj=20 fb=1003
[45726088452] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU1] wayland-server: surface 3 ready for mapping
[45734093691] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=88 tid=10
[45744829944] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[45745805886] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_IMPORT_BUFFER requested: memfd=13, size=1280x800
[45746520270] [[34mDEBUG[0m] [kernel::syscall::handlers::memory] [CPU3] sys_vm_map: large backing file mapping offset=0 len=4096000 total_size=4096000
[45773943039] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[45776645574] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[45778514430] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[45781148028] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[45783452979] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[45785250654] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[45786918936] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[45788508612] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[45900125469] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] DISP: imported buffer as ID=5
[45901221564] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=14
[45903395043] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=14
[45906355011] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=14 -> OK(12)
[45907677618] [[34mDEBUG[0m] [bloom::display] [CPU1] bloom: imported buffer 1280x800 as ID=5
[46104050322] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[46106676990] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[46109103150] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[46111414899] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[46113680019] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[46116019983] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[46118182704] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[46120450167] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[46156264935] [[34mDEBUG[0m] [kernel::syscall::handlers::memory] [CPU1] sys_vm_map: large backing file mapping offset=0 len=4096000 total_size=4096000
[46197177444] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=88 tid=10
[46210095360] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[46211646096] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_IMPORT_BUFFER requested: memfd=14, size=1280x800
[46212920094] [[34mDEBUG[0m] [kernel::syscall::handlers::memory] [CPU3] sys_vm_map: large backing file mapping offset=0 len=4096000 total_size=4096000
[46248575373] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] DISP: imported buffer as ID=6
[46250146173] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=15
[46252988925] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=15
[46258919685] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=15 -> OK(12)
[46261259517] [[34mDEBUG[0m] [bloom::display] [CPU1] bloom: imported buffer 1280x800 as ID=6
[46262460387] [[32mINFO [0m] [bloom::render] [CPU1] bloom: flat window overlays ready size=1280x800
[46299564762] [[34mDEBUG[0m] [pistil::font] [CPU1] pistil: opening font /share/fonts/Inter-Regular.ttf
[46301446224] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/share/fonts/Inter-Regular.ttf' tid=10
[46306945707] [[34mDEBUG[0m] [pistil::font] [CPU1] pistil: stat font /share/fonts/Inter-Regular.ttf
[46310023749] [[34mDEBUG[0m] [pistil::font] [CPU1] pistil: reading font 876576 bytes from /share/fonts/Inter-Regular.ttf
[46447473006] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[46449510789] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[46451205438] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[46452820722] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[46454225796] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[46455944535] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[46457334099] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[46459059537] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[46591066005] [[34mDEBUG[0m] [pistil::font] [CPU1] pistil: font read returned 876576 of 876576 bytes
[46592367096] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=25
[46593716268] [[34mDEBUG[0m] [pistil::font] [CPU1] pistil: parsing font /share/fonts/Inter-Regular.ttf
[46625397951] [[34mDEBUG[0m] [pistil::font] [CPU3] pistil: parsed font /share/fonts/Inter-Regular.ttf
[46801445394] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[46804105557] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[46806531849] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[46809004539] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[46818822072] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[46836682695] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[46839405327] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[46844760930] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[46851073962] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU1] wayland-server: xdg_surface.ack_configure serial=1 accepted
[46855344624] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU1] wayland-server: wl_surface obj=10 registered frame callback cb=1000
[46860919644] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU1] wayland-server: surface 2 ready for mapping
[47192457642] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[47197157634] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[47202170994] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[47206089678] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[47208128088] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[47210237646] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[47212669878] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[47215163523] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[47504457660] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
e[47506499700] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[47567366385] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[47569900488] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[47572413009] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[47573755812] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
c[47575731192] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[47577635457] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
[47577831246] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
h[47579750493] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[47580613641] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[47584640961] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[47586088902] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[47587425798] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[47665214256] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
o[47667672591] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[47709050037] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
 [47711196555] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[47775010866] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
p[47776784286] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[47807320869] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
o[47808657963] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[47865819408] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
l[47867676879] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[47940440031] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[47942419635] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[47944386699] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[47945098740] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
l[47946178302] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[47947757880] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[47949420948] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[47951729298] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[47953983825] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[47956135524] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[47976504444] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
-[47978486127] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[48031795878] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 18:22:00.002905505 unix_secs=1777659720.002905505
[48035772972] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
b[48037186461] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[48071734920] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
a[48073868502] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[48146331585] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
s[48147835989] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[48183008016] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
e[48185079492] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[48239159298] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
l[48240630306] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[48281168958] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
i[48282605613] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[48288276597] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[48290926926] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[48293455980] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[48295920882] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[48298344600] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[48300512964] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[48305014923] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[48307976607] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[48342814509] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
n[48344886876] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[48361588209] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU1] wayland-server: wl_surface obj=10 registered frame callback cb=1001
[48418193637] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
e[48419600064] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[48442677756] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1

[48445604196] [[34mDEBUG[0m] [sh] [CPU3] sh: spawning job cmd='echo'
[48446761242] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/bin/echo' tid=6
[48450557364] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=6
[48451924389] [[34mDEBUG[0m] [pistil::font] [CPU1] pistil: parsed font /share/fonts/Inter-Regular.ttf
[48452194725] [[34mDEBUG[0m] [sh] [CPU3] sh: spawning '/bin/echo' with argv=["/bin/echo", "poll-baseline"]
[48460459674] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN: page cache miss for '/bin/echo', caching 44616 bytes
[48462186927] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: Loading module 'echo' (len=44616, base=0x200000)
[48463989387] [[35mTRACE[0m] [kernel::task::loader] [CPU3]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[48466540419] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[48467496891] [[35mTRACE[0m] [kernel::task::loader] [CPU3] Segment: vaddr=200000 exec=true
[48468586287] [[35mTRACE[0m] [bloom::display] [CPU1] bloom: committing bounded damage rects=1
[48469842795] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=356 tid=10
[48470356044] [[35mTRACE[0m] [kernel::task::loader] [CPU3]   COPIED at 0x201420: [fd, 80, 00, 00, 00, 0f, 82, 75]
[48473342247] [[35mTRACE[0m] [kernel::task::loader] [CPU3] Segment: vaddr=206000 exec=false
[48474751908] [[35mTRACE[0m] [kernel::task::loader] [CPU3] Segment: vaddr=207000 exec=false
[48476243772] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: TLS block: tp=0x208000 filesz=0 memsz=0 align=1
[48481546773] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Starting Phase 1 for /bin/echo
[48482749788] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb02f2e40 arg=0xffffffffb02cef40
[48485709954] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Phase 1 complete, ID=19, applying inserts
[48486442323] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[48487987515] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=19
[48489433014] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[48490676157] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[48493838019] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[48498647472] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Phase 2 complete, waking task 19
[48500448513] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Task 19 woken, restoring IRQs
[48501706902] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[48502081518] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Done for /bin/echo
[48502486197] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=19 entry_pc=200000 user_sp=800000
[48503795439] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=19 buf_len=0
[48503258562] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/echo' TID=19 PID=19
[48504942519] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=19 buf_len=100
[48516849810] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[48518152881] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_COMMIT requested: planes=5
[48534502797] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU1] vfs: /dev/console write by PID=19 len=14
poll-baseline
[48536112966] [[35mTRACE[0m] [kernel::signal] [CPU1] signal: queued child event ppid=6 child_pid=19 status=0x0 wake_tids=1 queue_len=1
[48642114147] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT transfer begin seq=4 res_id=1 damage=482x322+40,60
[48644801370] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT transfer end seq=4 res_id=1
[48645980988] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT flush begin seq=4 res_id=1
[48647609769] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT flush end seq=4 res_id=1
[48648698109] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT complete (seq=4)
[48649459221] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=16
[48651735594] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=16
[48655372128] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[48655585803] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=16 -> OK(8)
[48657607086] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[48658787232] [[35mTRACE[0m] [bloom::frame_clock] [CPU1] bloom: frame interval 1850843643ns (target 16666666ns)
[48659740602] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=48 tid=10
[48659950548] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[48662190390] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[48662416473] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[48663716607] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_GET_INFO requested
[48664271106] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[48664491381] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: Returning dimensions 1280x800
[48665524116] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=17
[48666112473] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[48667032909] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=17
[48670786263] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=17 -> OK(48)
[48671023269] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[48673304955] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[48679974684] [[34mDEBUG[0m] [sh] [CPU3] sh: spawned '/bin/echo' pid=19 idx=0 background=false pending_pgid=0
[48681567297] [[34mDEBUG[0m] [sh] [CPU3] sh: setpgid leader pid=19 -> pgid=19
[48681519645] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/components' tid=10
[48682169580] [[34mDEBUG[0m] [sh] [CPU3] sh: cleaning up 0 pipes
[48682791531] [[34mDEBUG[0m] [sh] [CPU3] sh: foreground handoff start job_id=2 pgid=19 shell_pgid=5 cmd='echo poll-baseline'
[48683919867] [[34mDEBUG[0m] [sh] [CPU3] sh: foreground handoff done job_id=2 pgid=19 cmd='echo poll-baseline'
[48684928182] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=6
[?25l[48686340285] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=25
[48687214059] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/status' tid=10
[48687338667] [[34mDEBUG[0m] [kernel::inbox] [CPU3] inbox::close: inbox closed, 0 messages dropped on close
[48691965663] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=25
[48692636421] [[35mTRACE[0m] [kernel::sched::lifecycle] [CPU3] waitpid: queued status delivered parent_pid=6 parent_tid=6 target_pid=-1 child_pid=19 status=0x0 flags=0x6
[48694028559] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=6
[48694086870] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/index' tid=10
[?25h[48696531708] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=95
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [48699748383] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=6
[?25h[48700395645] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=25
[48700777488] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[48702737688] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/index' tid=10
[48705834672] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=25
[48707261196] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/events/latest' tid=10
[48710550504] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=25
[48718361802] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=88 tid=10
[48727402614] [[35mTRACE[0m] [bloom::wayland] [CPU1] wayland-server: frame callback done surface=1 ts=24328ms callbacks=1
[48730547184] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: frame callback done object=1000 data=24328
[48733257045] [[35mTRACE[0m] [bloom::wayland] [CPU1] wayland-server: wp_presentation_feedback.presented surface=1 ts_ns=24361837764 feedbacks=1
[48734612124] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: wp_presentation_feedback.presented object=1001 tv=24.361837764 refresh_ns=16666666 seq=0
[48753986391] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[48755088030] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_IMPORT_BUFFER requested: memfd=15, size=160x96
[48757117431] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] DISP: imported buffer as ID=7
[48758039616] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=18
[48759978201] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=18
[48764198736] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=18 -> OK(12)
[48767255922] [[34mDEBUG[0m] [bloom::display] [CPU1] bloom: imported buffer 160x96 as ID=7
[48770306343] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=88 tid=10
[48785531586] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[48786671142] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_IMPORT_BUFFER requested: memfd=16, size=520x220
[48790835016] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] DISP: imported buffer as ID=8
[48791726280] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=19
[48793844154] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=19
[48796643577] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=19 -> OK(12)
[48797947176] [[34mDEBUG[0m] [bloom::display] [CPU1] bloom: imported buffer 520x220 as ID=8
[48803478867] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=88 tid=10
[48822633420] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[48824289657] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_IMPORT_BUFFER requested: memfd=17, size=520x220
[48830478477] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x28 (is_aux=true)
[48830872200] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] DISP: imported buffer as ID=9
[48831729936] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x08 (is_aux=true)
[48832043337] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=20
[48832740990] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xfd (is_aux=true)
[48834487977] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=20
[48838798173] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=20 -> OK(12)
[48840977559] [[34mDEBUG[0m] [bloom::display] [CPU1] bloom: imported buffer 520x220 as ID=9
[48841243275] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x28 (is_aux=true)
[48842417778] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=52 tid=10
[48844443912] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x08 (is_aux=true)
[48846969864] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfd (is_aux=true)
[48850582671] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[48851617254] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_RELEASE_BUFFER requested: id=8 size=457600
[48856422351] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU2] RECVMSG: thing=3 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00f2530, metadata: DynMetadata(0xffffffff802c6ed0) }
[48857900916] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=16
[48859359252] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=21
[48861390534] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=21
[48866988192] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=21 -> OK(8)
[48873356136] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x28 (is_aux=true)
[48874210374] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x08 (is_aux=true)
[48874871529] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xfd (is_aux=true)
[48881028570] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x28 (is_aux=true)
[48883751961] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x08 (is_aux=true)
[48885834822] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfd (is_aux=true)
[48894135939] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU2] RECVMSG: thing=3 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00f2530, metadata: DynMetadata(0xffffffff802c6ed0) }
[48916329528] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=460 tid=10
[48918898743] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x28 (is_aux=true)
[48919857525] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x08 (is_aux=true)
[48920453406] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x28 (is_aux=true)
[48920738691] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xfd (is_aux=true)
[48924360804] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x08 (is_aux=true)
[48926706015] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[48928108119] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfd (is_aux=true)
[48928606419] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_COMMIT requested: planes=7
[48935635650] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU2] RECVMSG: thing=3 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00f2530, metadata: DynMetadata(0xffffffff802c6ed0) }
[48959513493] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x28 (is_aux=true)
[48960329979] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x08 (is_aux=true)
[48960851082] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xfd (is_aux=true)
[48966006045] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x28 (is_aux=true)
[48967880214] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x08 (is_aux=true)
[48969942516] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfd (is_aux=true)
[48974915847] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU2] RECVMSG: thing=3 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00f2530, metadata: DynMetadata(0xffffffff802c6ed0) }
[49000183188] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x28 (is_aux=true)
[49001343996] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x08 (is_aux=true)
[49002194175] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xfd (is_aux=true)
[49009497768] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x28 (is_aux=true)
[49011893436] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x08 (is_aux=true)
[49014153738] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfd (is_aux=true)
[49020201945] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU2] RECVMSG: thing=3 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00f2530, metadata: DynMetadata(0xffffffff802c6ed0) }
[49020217059] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[49024901673] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[49027195536] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[49029651858] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[49031881404] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[49034237670] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[49036757352] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[49038373593] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[49039941291] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x28 (is_aux=true)
[49042327257] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x08 (is_aux=true)
[49043222316] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xfd (is_aux=true)
[49053860658] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x28 (is_aux=true)
[49057274904] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x08 (is_aux=true)
[49060444455] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfd (is_aux=true)
[49067623638] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU2] RECVMSG: thing=3 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00f2530, metadata: DynMetadata(0xffffffff802c6ed0) }
[49079023719] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x28 (is_aux=true)
[49079821659] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x08 (is_aux=true)
[49080407607] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xfd (is_aux=true)
[49084616130] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x28 (is_aux=true)
[49086389286] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x08 (is_aux=true)
[49088216397] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfd (is_aux=true)
[49122143730] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x28 (is_aux=true)
[49123339815] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xfd (is_aux=true)
[49126141548] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x28 (is_aux=true)
[49128972222] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfd (is_aux=true)
[49139838297] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU2] RECVMSG: thing=3 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00f2530, metadata: DynMetadata(0xffffffff802c6ed0) }
[49165334493] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x28 (is_aux=true)
[49166200050] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x08 (is_aux=true)
[49166735838] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xfd (is_aux=true)
[49173012537] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x28 (is_aux=true)
[49176424275] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x08 (is_aux=true)
[49179217263] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfd (is_aux=true)
[49189213689] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU2] RECVMSG: thing=3 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00f2530, metadata: DynMetadata(0xffffffff802c6ed0) }
[49205023230] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x28 (is_aux=true)
[49206007323] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x08 (is_aux=true)
[49206109128] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU1] PS/2 byte received: 0xfd (is_aux=true)
[49214952270] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x28 (is_aux=true)
[49216167561] [[35mTRACE[0m] [ps2_kbd] [CPU1] ps2_kbd: yield on AUX data (mouse packet)
[49218101262] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x08 (is_aux=true)
[49222292724] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfd (is_aux=true)
[49232453622] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU2] RECVMSG: thing=3 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00f2530, metadata: DynMetadata(0xffffffff802c6ed0) }
[49247393646] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x28 (is_aux=true)
[49248677841] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x08 (is_aux=true)
[49249557654] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xfd (is_aux=true)
[49255450398] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x28 (is_aux=true)
[49257148842] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x08 (is_aux=true)
[49258927212] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfd (is_aux=true)
[49265722605] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU2] RECVMSG: thing=3 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00f2530, metadata: DynMetadata(0xffffffff802c6ed0) }
[49287211314] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU3] PS/2 byte received: 0x28 (is_aux=true)
[49288324833] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU3] PS/2 byte received: 0x08 (is_aux=true)
[49288915665] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU3] PS/2 byte received: 0xfd (is_aux=true)
[49292363274] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x28 (is_aux=true)
[49295726766] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x08 (is_aux=true)
[49295949186] [[35mTRACE[0m] [ps2_kbd] [CPU1] ps2_kbd: yield on AUX data (mouse packet)
[49298426496] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfd (is_aux=true)
[49303685772] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU2] RECVMSG: thing=3 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00f2530, metadata: DynMetadata(0xffffffff802c6ed0) }
[49330061154] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x28 (is_aux=true)
[49330854012] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x08 (is_aux=true)
[49331456493] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xfd (is_aux=true)
[49337992374] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x28 (is_aux=true)
[49341837402] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x08 (is_aux=true)
[49344333456] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfd (is_aux=true)
[49350201318] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU2] RECVMSG: thing=3 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00f2530, metadata: DynMetadata(0xffffffff802c6ed0) }
[49371880404] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x28 (is_aux=true)
[49373013030] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x08 (is_aux=true)
[49373977356] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xfd (is_aux=true)
[49382708100] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x28 (is_aux=true)
[49385880456] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x08 (is_aux=true)
[49388981070] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfd (is_aux=true)
[49392385251] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[49393845501] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[49395317004] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[49396884570] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[49396807251] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU2] RECVMSG: thing=3 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00f2530, metadata: DynMetadata(0xffffffff802c6ed0) }
[49399752171] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[49401256377] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[49402688016] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[49404051312] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[49415218050] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x28 (is_aux=true)
[49416316620] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x08 (is_aux=true)
[49417115583] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xfd (is_aux=true)
[49422330870] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x28 (is_aux=true)
[49424232957] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x08 (is_aux=true)
[49425918432] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfd (is_aux=true)
[49432267500] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU2] RECVMSG: thing=3 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00f2530, metadata: DynMetadata(0xffffffff802c6ed0) }
[49458331725] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x28 (is_aux=true)
[49459989678] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x28 (is_aux=true)
[49459816923] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x08 (is_aux=true)
[49461064653] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xfd (is_aux=true)
[49461785109] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x08 (is_aux=true)
[49464069501] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfd (is_aux=true)
[49472495523] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU2] RECVMSG: thing=3 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00f2530, metadata: DynMetadata(0xffffffff802c6ed0) }
[49499932218] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x28 (is_aux=true)
[49501058013] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x08 (is_aux=true)
[49502030127] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xfd (is_aux=true)
[49506169746] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x28 (is_aux=true)
[49507932540] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x08 (is_aux=true)
[49509637320] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfd (is_aux=true)
[49514781030] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU2] RECVMSG: thing=3 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00f2530, metadata: DynMetadata(0xffffffff802c6ed0) }
[49543263726] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x28 (is_aux=true)
[49544292171] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x08 (is_aux=true)
[49545094665] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xfd (is_aux=true)
[49552240221] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x28 (is_aux=true)
[49554262791] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x08 (is_aux=true)
[49556548503] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfd (is_aux=true)
[49562541270] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU2] RECVMSG: thing=3 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00f2530, metadata: DynMetadata(0xffffffff802c6ed0) }
[49583560587] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x28 (is_aux=true)
[49584377040] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x08 (is_aux=true)
[49584909660] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xfd (is_aux=true)
[49590857151] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x28 (is_aux=true)
[49592984496] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x08 (is_aux=true)
[49594669971] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfd (is_aux=true)
[49601187768] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU2] RECVMSG: thing=3 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00f2530, metadata: DynMetadata(0xffffffff802c6ed0) }
[49626442734] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU3] PS/2 byte received: 0x28 (is_aux=true)
[49627484181] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU3] PS/2 byte received: 0x08 (is_aux=true)
[49627539390] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xfd (is_aux=true)
[49630746066] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x28 (is_aux=true)
[49632032868] [[35mTRACE[0m] [ps2_kbd] [CPU1] ps2_kbd: yield on AUX data (mouse packet)
[49632632940] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x08 (is_aux=true)
[49635988248] [[35mTRACE[0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfd (is_aux=true)
[49641153408] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU2] RECVMSG: thing=3 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00f2530, metadata: DynMetadata(0xffffffff802c6ed0) }
[49641945738] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT transfer begin seq=5 res_id=1 damage=1280x800+0,0
[49644491853] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT transfer end seq=5 res_id=1
[49645228215] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT flush begin seq=5 res_id=1
[49646020446] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT flush end seq=5 res_id=1
[49646655168] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT complete (seq=5)
[49647555375] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=22
[49649583159] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=22
[49653771618] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=22 -> OK(8)
[49657781184] [[35mTRACE[0m] [bloom::frame_clock] [CPU1] bloom: frame interval 499488297ns (target 16666666ns)
[49659007101] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=48 tid=10
[49668183675] [[35mTRACE[0m] [bloom::wayland] [CPU1] wayland-server: frame callback done surface=3 ts=24827ms callbacks=1
[49669699893] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: frame callback done object=1002 data=24827
[49669767180] [[35mTRACE[0m] [bloom::wayland] [CPU1] wayland-server: wp_presentation_feedback.presented surface=3 ts_ns=24833344371 feedbacks=1
[49671421206] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: wp_presentation_feedback.presented object=1003 tv=24.833344371 refresh_ns=16666666 seq=1
[49671887760] [[35mTRACE[0m] [bloom::wayland] [CPU1] wayland-server: frame callback done surface=2 ts=24827ms callbacks=2
[49704284256] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[49705943331] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_GET_INFO requested
[49706802783] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: Returning dimensions 1280x800
[49707425625] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=23
[49709357775] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=23
[49712040180] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=23 -> OK(48)
[49718571078] [[35mTRACE[0m] [bloom::input] [CPU1] bloom: pointer moved dx=8 dy=3 pos=648,403
[49719747825] [[35mTRACE[0m] [bloom::input] [CPU1] bloom: pointer moved dx=8 dy=3 pos=656,406
[49721159829] [[35mTRACE[0m] [bloom::input] [CPU1] bloom: pointer moved dx=8 dy=3 pos=664,409
[49721987436] [[35mTRACE[0m] [bloom::input] [CPU1] bloom: pointer moved dx=8 dy=3 pos=672,412
[49722753234] [[35mTRACE[0m] [bloom::input] [CPU1] bloom: pointer moved dx=8 dy=3 pos=680,415
[49724075016] [[35mTRACE[0m] [bloom::input] [CPU1] bloom: pointer moved dx=8 dy=3 pos=688,418
[49725728976] [[35mTRACE[0m] [bloom::input] [CPU1] bloom: pointer moved dx=8 dy=3 pos=696,421
[49726519359] [[35mTRACE[0m] [bloom::input] [CPU1] bloom: pointer moved dx=8 dy=3 pos=704,424
[49727917140] [[35mTRACE[0m] [bloom::input] [CPU1] bloom: motion coalesce pre=10 post=1 pos=720,430
[49729413690] [[35mTRACE[0m] [bloom::input] [CPU1] bloom: cursor smoothing visible=720,430 target=720,430
[49734556344] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[49736226342] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[49737694545] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[49739266269] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[49740712263] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[49742307516] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[49743770835] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[49745234154] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[49763944527] [[35mTRACE[0m] [bloom::display] [CPU1] bloom: committing bounded damage rects=2
[49766011119] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=476 tid=10
[49772571717] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[49774181028] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_COMMIT requested: planes=7
[49818773796] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT transfer begin seq=6 res_id=1 damage=272x222+544,304
[49820369808] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT transfer end seq=6 res_id=1
[49821246420] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT flush begin seq=6 res_id=1
[49822226619] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT flush end seq=6 res_id=1
[49823256846] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT complete (seq=6)
[49824326640] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=24
[49827587337] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=24
[49832761242] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=24 -> OK(8)
[49848749511] [[35mTRACE[0m] [bloom::frame_clock] [CPU1] bloom: frame interval 95344904ns (target 16666666ns)
[49860814938] [[35mTRACE[0m] [bloom::input] [CPU1] bloom: motion coalesce pre=19 post=2 pos=792,457
[49861725837] [[35mTRACE[0m] [bloom::input] [CPU1] bloom: cursor smoothing visible=792,457 target=792,457
[49908010251] [[35mTRACE[0m] [bloom::display] [CPU1] bloom: committing bounded damage rects=2
[49909545609] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=476 tid=10
[49941833667] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[49943295072] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_COMMIT requested: planes=7
[49990584435] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT transfer begin seq=7 res_id=1 damage=264x219+624,334
[49992272088] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT transfer end seq=7 res_id=1
[49993160382] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT flush begin seq=7 res_id=1
[49994518266] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT flush end seq=7 res_id=1
[49995346401] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT complete (seq=7)
[49996196976] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=25
[49998343989] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=25
[50001466350] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=25 -> OK(8)
[50004416121] [[35mTRACE[0m] [bloom::frame_clock] [CPU1] bloom: frame interval 77980056ns (target 16666666ns)
[50071264815] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[50073625800] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[50076151620] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[50078647212] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[50080259427] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[50082000870] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[50083720269] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[50085379806] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[50280686742] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=48 tid=10
[50316171708] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[50318006937] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_GET_INFO requested
[50319339081] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: Returning dimensions 1280x800
[50320584204] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=26
[50324363199] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=26
[50328767907] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=26 -> OK(48)
[50411965692] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[50413812471] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[50415532365] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[50417021820] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[50418838107] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[50421158436] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[50423362407] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[50425367784] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[50752330299] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[50755084809] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[50757682107] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[50760279504] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[50762277918] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[50763941316] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[50765367609] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[50767145583] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[50993766615] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=48 tid=10
[51030235905] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[51031610322] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_GET_INFO requested
[51032256924] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: Returning dimensions 1280x800
[51033025461] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=27
[51035045160] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=27
[51038767593] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=27 -> OK(48)
[51047637927] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/components' tid=10
[51050533083] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=25
[51051599214] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/status' tid=10
[51065069913] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=25
[51067001436] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/index' tid=10
[51069891345] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=25
[51071214711] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/index' tid=10
[51075398055] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=25
[51076582524] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/events/latest' tid=10
[51079130289] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=25
[51088878225] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[51090588648] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[51092521755] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[51094166343] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[51095583429] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[51097116642] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[51098501223] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[51099891282] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[51431015097] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[51433657803] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[51436018425] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[51437991099] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[51439408350] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[51440832366] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[51442184904] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[51443478933] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[51770267406] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[51772841472] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[51775355742] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[51776105403] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=48 tid=10
[51778523049] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[51780033822] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[51781522188] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[51782896242] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[51784321050] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[51807060162] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[51809016402] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_GET_INFO requested
[51810294063] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: Returning dimensions 1280x800
[51811399266] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=28
[51814221096] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=28
[51817687383] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=28 -> OK(48)
[52109485263] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[52111994550] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[52114694412] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[52116165750] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[52117587885] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[52119053877] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[52120426248] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[52121719782] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[52448036619] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[52450964742] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[52453213164] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[52454672622] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[52456048161] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[52457506893] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[52458881310] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[52460199396] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[52586001402] [[35mTRACE[0m] [ps2_mouse] [CPU2] ps2_mouse: irq_wakes=21 poll_timeouts=235 last_drain_bytes=0
[52591121154] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=48 tid=10
[52620260451] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[52621824585] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_GET_INFO requested
[52622979585] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: Returning dimensions 1280x800
[52624170390] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=29
[52626594372] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=29
[52630062738] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=29 -> OK(48)
[52789784487] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[52792388583] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[52794550248] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[52796212986] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[52797595059] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[52799054880] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[52800381150] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[52801673925] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[53125806426] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[53128330398] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[53130648417] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[53132875092] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[53134973925] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[53136453876] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[53137813311] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[53139159942] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[53402848950] [
```
</details>
