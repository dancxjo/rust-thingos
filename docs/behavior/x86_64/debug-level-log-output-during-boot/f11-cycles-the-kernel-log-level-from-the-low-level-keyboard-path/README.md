# ❌ Scenario: F11 cycles the kernel log level from the low-level keyboard path

> Last run: 2026-04-30 21:36:48

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 0ms | - - - |
| 2 | When I wait for the system to boot | ✅ | 10845ms | - [📜](./02/serial.log) - |
| 3 | And I press f11 | ✅ | 257ms | - [📜](./03/serial.log) - |
| 4 | Then the serial output should contain "F11 hotkey: log level set to 4 (DEBUG)" | ❌ | 32752ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[34124125827] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[34378954401] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[34398674112] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[34428867990] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[34460324118] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[34479906483] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[34481729898] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[34521534399] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[34523582643] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[34543294368] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[34558058898] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[34558801596] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[34594079586] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[34595245080] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[34686069627] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[34769290545] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[34792594353] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[34872778875] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[34974123591] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[35000052450] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[35023026852] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[35121734076] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[35171625357] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[35307886152] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[35560594113] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=944505639 elapsed_us=472252
[35561709150] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[35651132814] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[35655243954] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[35694347370] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[35762824416] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[35775890370] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[35778024810] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[35875860372] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[35881879374] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[35889593289] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[36048915012] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[36050619099] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[36085882833] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[36088142046] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[36089576655] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[36090249129] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[36114810303] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[36123075054] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[36126178242] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[36127670733] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[36134428671] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[36138185919] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[36139866543] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[36154783137] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[36157559163] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[36162112206] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[36166642611] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[36173567265] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[36185089347] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[36204165624] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[36216729021] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[36223540023] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[36320145147] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[36331300170] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[36339433152] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[36408557955] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[36574275441] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[36739654809] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[36890215461] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[36898902051] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[36903832119] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[36907255737] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[36909306852] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[37085458740] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[37247684496] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[37366333818] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[37435884948] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[37437747237] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[37453764579] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[37685374089] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[37782765108] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[37806181347] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1920x1080 @ 60000mHz
[37807746438] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[37808773332] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[37809633675] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[38143537674] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[38144644395] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[38145510051] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[38153041839] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Facet Frame
[38698360965] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[38855610519] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[39044838228] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[39239249346] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[39341444472] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[39448122615] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[39450966192] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[39521987076] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Starting AHCI VFS driver
[39555116997] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[39572967522] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Mounted atapi2 at /dev/storage/atapi2
[39573979500] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Provider loop online at /dev/storage/atapi2
[39584357307] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: claimed /sys/devices/isa-0070 (handle=2)
[39604075863] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: bristle pid=8
[39608483376] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: 2026-05-01 04:38:26 = 1777610306 unix_secs
[39609766251] [[32mINFO [0m] [kernel::time] [CPU3] System clock anchored: unix_secs=1777610306, mono_ns=19804728421, offset=1777610286195271579ns
[39610736484] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: System clock anchored to 1777610306 unix_secs
[39642258018] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:26.014689224 unix_secs=1777610306.014689224
[39831955251] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: bristle pid=8
[39856466298] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x57 (is_aux=false)
[39862564335] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0xd7 (is_aux=false)
[39864061050] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: read scancode 0xd7
[39866207832] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: edge detected: Up { key: F11, mods: Mods(0) }
[39870769554] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: sent F11 to bristle (pid=8)
[39872498457] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0x47 (is_aux=false)
[39873491262] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: read scancode 0x47
[39874861851] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: edge detected: Down { key: Unknown, mods: Mods(0), repeat: false }
[39876247290] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: sent Unknown to bristle (pid=8)
[39880183827] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=96x96 hotspot=21,12
[39883060899] [[32mINFO [0m] [bristle] [CPU2] bristle: KeyDown received: Unknown
[39887876424] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfa (is_aux=true)
[39890130126] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xaa (is_aux=true)
[39892597734] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x00 (is_aux=true)
[39894712770] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[39898828398] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[39899144901] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfa (is_aux=true)
[39905194230] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfa (is_aux=true)
[39906480339] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: sample rate set to 60 Hz
[39910723776] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfa (is_aux=true)
[39915474687] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfa (is_aux=true)
[39919717497] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfa (is_aux=true)
[39921842631] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x00 (is_aux=true)
[39924119202] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x03 (is_aux=true)
[39926396994] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x3c (is_aux=true)
[39931774674] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0xfa (is_aux=true)
[39940615473] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[39942374406] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[39943217094] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1920x1080 @ 60000mHz ready
[39945012624] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: starting
[39962496750] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: listening on /run/wayland-0
[39964127610] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[40193300631] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=16)
[40209820827] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: connected to /run/wayland-0
[40221448014] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=17)
[40226020296] [[32mINFO [0m] [clock] [CPU2] clock: running at low scheduler priority tid=17
[40229869119] [[32mINFO [0m] [clock] [CPU2] clock: connected to /run/wayland-0
[40518506523] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=1 rect=1920x1080 src_px=0xff0b0a10 dst_px=0xff0b0a10 damage=1920x1080+0,0 res_id=1
[40521389469] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 958,539 src_px=0x20181818 dst_before=0xff0b0a10 dst_after=0xff0d0c11
[40564708140] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[40572672063] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
[40605069219] [[32mINFO [0m] [clock] [CPU2] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[41097894387] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 20:38:26 UTC-8 system_unix=1777610306.741925404
[41114329674] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[43727293896] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[43733921451] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[44131396386] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=28 frame=6
[44142589854] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[44893067274] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[44897347044] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: bound wp_presentation
[44963946984] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=28 frame=6
[44988809250] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[45208400820] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: imported dmabuf wl_buffer=111 480x320 format=0x34325241
[45643870074] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:29.017071283 unix_secs=1777610309.017071283
[47814797283] [[32mINFO [0m] [bloom::render] [CPU1] bloom: flat window overlays ready size=1920x1080
[47994064668] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 20:38:30 UTC-8 system_unix=1777610310.191797363
[51648299010] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:32.019339953 unix_secs=1777610312.019339953
[51989233725] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=3 height=28 frame=6
[52037943672] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_popup obj=23 assigned to xdg_surface=21
[52084282272] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: imported dmabuf wl_buffer=121 160x96 format=0x34325241
[54290958546] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: frame callback done object=1000 data=27106
[54296677446] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: wp_presentation_feedback.presented object=1001 tv=27.121263493 refresh_ns=16666666 seq=0
[54943664919] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 20:38:33 UTC-8 system_unix=1777610313.666850928
[56751751848] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: frame callback done object=1002 data=28363
[56753334330] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: wp_presentation_feedback.presented object=1003 tv=28.374013041 refresh_ns=16666666 seq=1
[57652709103] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:35.021566285 unix_secs=1777610315.021566285
[61640608656] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 20:38:37 UTC-8 system_unix=1777610317.014976643
[63657602778] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:38.023989115 unix_secs=1777610318.023989115
[68318908119] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 20:38:40 UTC-8 system_unix=1777610320.354479128
[69661950633] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:41.026153241 unix_secs=1777610321.026153241
[75260930382] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 20:38:43 UTC-8 system_unix=1777610323.825405928
[75666593046] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:44.028489364 unix_secs=1777610324.028489364
[81670868235] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:47.030648293 unix_secs=1777610327.030648293
[82167103821] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 20:38:47 UTC-8 system_unix=1777610327.278205251
[87675516192] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:50.032958725 unix_secs=1777610330.032958725
[89020522338] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 20:38:50 UTC-8 system_unix=1777610330.704980641
[93680020467] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:53.035232411 unix_secs=1777610333.035232411
[95677269072] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 20:38:54 UTC-8 system_unix=1777610334.033435172
[99684695418] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:56.037532267 unix_secs=1777610336.037532267
[102468354450] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 20:38:57 UTC-8 system_unix=1777610337.428960321
[105689264043] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:38:59.039806943 unix_secs=1777610339.039806943
[109264194171] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 20:39:00 UTC-8 system_unix=1777610340.826921663
[111693805245] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:02.042090398 unix_secs=1777610342.042090398
[115941968505] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 20:39:04 UTC-8 system_unix=1777610344.165795201
[117698430861] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:05.044401968 unix_secs=1777610345.044401968
[122774865477] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 20:39:07 UTC-8 system_unix=1777610347.582334717
[123702828810] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:08.046611602 unix_secs=1777610348.046611602
[129678986976] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 20:39:11 UTC-8 system_unix=1777610351.034393454
[129707342028] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:11.048860274 unix_secs=1777610351.048860274
[135739928412] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:14.065153070 unix_secs=1777610354.065153070
[136470430404] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 20:39:14 UTC-8 system_unix=1777610354.430014914
[141753787395] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 04:39:17.072119027 unix_secs=1777610357.072119027
[143301647247] [
```
</details>
