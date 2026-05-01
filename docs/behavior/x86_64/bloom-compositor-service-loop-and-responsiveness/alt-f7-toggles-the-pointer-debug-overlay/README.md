# ✅ Scenario: Alt F7 toggles the pointer debug overlay

> Last run: 2026-04-30 17:57:15

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 10329ms | - [📜](./01/serial.log) - |
| 2 | Then the serial output should contain "bloom: registered bristle pointer sink" within 60s | ✅ | 2560ms | - [📜](./02/serial.log) - |
| 3 | And the serial output should contain "ps2_kbd: bristle pid=" within 60s | ✅ | 0ms | - - - |
| 4 | When I press Alt+F7 | ✅ | 262ms | - [📜](./04/serial.log) - |
| 5 | Then the serial output should contain "bloom: pointer debug overlay enabled" within 60s | ✅ | 0ms | - - - |
| 6 | And the serial output should contain "bloom: pointer debug overlay ready" within 60s | ✅ | 0ms | - - - |
| 7 | When I press Alt+F7 | ✅ | 258ms | - [📜](./07/serial.log) - |
| 8 | Then the serial output should contain "bloom: pointer debug overlay disabled" within 60s | ✅ | 309ms | - [📜](./08/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[32305609545] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[32561010603] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[32582789481] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[32617053051] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[32650959396] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[32671840443] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[32673611388] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[32713261482] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[32715024606] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[32735923506] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[32750595240] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[32751238509] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[32787020838] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[32788114458] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[32878085988] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[32959344825] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[32981797728] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[33060979974] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[33166916211] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[33217773138] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[33247985727] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[33358041288] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[33408525876] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[33547037700] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[33805873233] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=995996034 elapsed_us=497998
[33806698101] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[33883520880] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[33886156227] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[33924404943] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[33956716134] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[33966580659] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[33968352099] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[34062552843] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[34065845550] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[34075538343] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[34229766978] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[34232180367] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[34267881846] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[34270412220] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[34270494819] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[34272481551] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[34294423020] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[34309863885] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[34314251862] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[34315835994] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[34360694643] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[34367093739] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[34369060572] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[34380202428] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[34381224372] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[34383777351] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[34388733555] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[34393261683] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[34397317944] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[34405696842] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[34448843847] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[34455080253] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[34549434942] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[34559781102] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[34567567320] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[34607790591] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[34772639748] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[34937244012] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[35104725678] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[35105076237] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[35116686000] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[35122019823] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[35124929169] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[35283010389] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[35445541956] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[35553893595] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[35620739979] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[35621872803] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[35638403493] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[35881833636] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[35908489650] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1920x1080 @ 60000mHz
[35909579640] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[35910414804] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[35912296992] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[36256118349] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[36418394463] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[36583465248] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[36757642812] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[36758770389] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[36759547011] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[36760248591] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil nine-slice shadow renderer loaded
[36769261815] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Solarized Warm
[37248393270] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[37352740425] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[37438017111] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[37614020301] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[37617087948] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[37696316328] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Starting AHCI VFS driver
[37720615284] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=12)
[37734498978] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[37752700821] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Mounted atapi2 at /dev/storage/atapi2
[37754157969] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Provider loop online at /dev/storage/atapi2
[37758920826] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=14)
[37765840200] [[32mINFO [0m] [rtc_cmos] [CPU1] RTC: claimed /sys/devices/isa-0070 (handle=2)
[37787804802] [[32mINFO [0m] [rtc_cmos] [CPU1] RTC: 2026-05-01 00:57:26 = 1777597046 unix_secs
[37789330953] [[32mINFO [0m] [kernel::time] [CPU1] System clock anchored: unix_secs=1777597046, mono_ns=18894506499, offset=1777597027105493501ns
[37790845884] [[32mINFO [0m] [rtc_cmos] [CPU1] RTC: System clock anchored to 1777597046 unix_secs
[37820634324] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:57:26.014952135 unix_secs=1777597046.014952135
[37985128104] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=96x96 hotspot=21,12
[37995533499] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: bristle pid=8
[37995624480] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[37999019949] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[38037233190] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: starting
[38050612116] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: listening on /run/wayland-0
[38051591622] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[38056490835] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[38058070413] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[38058927093] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1920x1080 @ 60000mHz ready
[38068527585] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
[38074642188] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: connected to /run/wayland-0
[38111220807] [[32mINFO [0m] [clock] [CPU2] clock: connected to /run/wayland-0
[38378734746] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[38426129181] [[32mINFO [0m] [clock] [CPU2] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[38607933705] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[38611021152] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: bound wp_presentation
[39406610628] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: bristle pid=8
[39426079407] [[32mINFO [0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x47
[39431620338] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x47 (is_aux=false)
[39451767069] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: sample rate set to 60 Hz
[42047087874] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=3 rect=1920x1080 src_px=0xff40422a dst_px=0xff40422a damage=1920x1080+0,0 res_id=1
[42048657090] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 958,539 src_px=0x20181818 dst_before=0xff880604 dst_after=0xff7a0807
[42057415422] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[42062343246] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=30 frame=6
[42094308465] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[42140047719] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[42148252905] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[42228358392] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: imported dmabuf wl_buffer=111 480x320 format=0x34325241
[42354699123] [[32mINFO [0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x38
[42359370966] [[32mINFO [0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x41
[42477667848] [[32mINFO [0m] [kernel::irq::ps2] [CPU3] PS/2 take_scancode: popped 0x38 (is_aux=false)
[42479196408] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: read scancode 0x38
[42481672695] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: edge detected: Down { key: LeftAlt, mods: Mods(4), repeat: false }
[42484775982] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: sent LeftAlt to bristle (pid=8)
[42486591840] [[32mINFO [0m] [kernel::irq::ps2] [CPU3] PS/2 take_scancode: popped 0x41 (is_aux=false)
[42487624542] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: read scancode 0x41
[42488492871] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: edge detected: Down { key: F7, mods: Mods(4), repeat: false }
[42489478878] [[32mINFO [0m] [bristle] [CPU2] bristle: KeyDown received: LeftAlt
[42490988628] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: sent F7 to bristle (pid=8)
[42493521147] [[32mINFO [0m] [bristle] [CPU2] bristle: KeyDown received: F7
[42534315912] [[32mINFO [0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xc1
[42538327524] [[32mINFO [0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xb8
[42540348147] [[32mINFO [0m] [kernel::irq::ps2] [CPU3] PS/2 take_scancode: popped 0xc1 (is_aux=false)
[42543018771] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: read scancode 0xc1
[42544330389] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: edge detected: Up { key: F7, mods: Mods(4) }
[42546491394] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: sent F7 to bristle (pid=8)
[42548162448] [[32mINFO [0m] [kernel::irq::ps2] [CPU3] PS/2 take_scancode: popped 0xb8 (is_aux=false)
[42549119085] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: read scancode 0xb8
[42550025364] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: edge detected: Up { key: LeftAlt, mods: Mods(0) }
[42551243262] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: sent LeftAlt to bristle (pid=8)
[42631861965] [[32mINFO [0m] [bloom::input] [CPU1] bloom: KeyDown received: LeftAlt (raw=0x00e2, mods=Mods(4), repeat=false)
[42633584829] [[32mINFO [0m] [bloom::input] [CPU1] bloom: KeyDown received: F7 (raw=0x0040, mods=Mods(4), repeat=false)
[42634374024] [[32mINFO [0m] [bloom::input] [CPU1] bloom: pointer debug overlay enabled
[42698841933] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pointer debug overlay ready buffer=4 size=460x144
[43096855428] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 16:57:28 UTC-8 system_unix=1777597048.651894206
[43219605792] [[32mINFO [0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x38
[43223864409] [[32mINFO [0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x41
[43227341553] [[32mINFO [0m] [kernel::irq::ps2] [CPU3] PS/2 take_scancode: popped 0x38 (is_aux=false)
[43229276904] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: read scancode 0x38
[43231240701] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: edge detected: Down { key: LeftAlt, mods: Mods(4), repeat: false }
[43234357320] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: sent LeftAlt to bristle (pid=8)
[43236940857] [[32mINFO [0m] [kernel::irq::ps2] [CPU3] PS/2 take_scancode: popped 0x41 (is_aux=false)
[43238127108] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: read scancode 0x41
[43238712231] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: edge detected: Down { key: F7, mods: Mods(4), repeat: false }
[43238738103] [[32mINFO [0m] [bristle] [CPU2] bristle: KeyDown received: LeftAlt
[43241367081] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: sent F7 to bristle (pid=8)
[43244064666] [[32mINFO [0m] [bristle] [CPU2] bristle: KeyDown received: F7
[43397006466] [[32mINFO [0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xc1
[43401984648] [[32mINFO [0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xb8
[43402073814] [[32mINFO [0m] [kernel::irq::ps2] [CPU3] PS/2 take_scancode: popped 0xc1 (is_aux=false)
[43405544391] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: read scancode 0xc1
[43406416812] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: edge detected: Up { key: F7, mods: Mods(4) }
[43410164457] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: sent F7 to bristle (pid=8)
[43412006055] [[32mINFO [0m] [kernel::irq::ps2] [CPU3] PS/2 take_scancode: popped 0xb8 (is_aux=false)
[43413025590] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: read scancode 0xb8
[43413745419] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: edge detected: Up { key: LeftAlt, mods: Mods(0) }
[43416339912] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: sent LeftAlt to bristle (pid=8)
[43836692526] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:57:29.023754426 unix_secs=1777597049.023754426
[44762742321] [
```
</details>
