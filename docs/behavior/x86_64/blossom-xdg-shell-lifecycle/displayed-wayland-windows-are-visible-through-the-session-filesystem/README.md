# ✅ Scenario: displayed Wayland windows are visible through the session filesystem

> Last run: 2026-04-30 15:14:01

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Then the Wayland hello client should be visible | ✅ | 4634ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 152ms | - - - |
| 3 | And I type "find /session/wayland/windows" on the serial console | ✅ | 587ms | - [📜](./03/serial.log) - |
| 4 | Then the latest serial output should contain "/session/wayland/windows/index" | ✅ | 0ms | - - - |
| 5 | When I type "cat /session/wayland/windows/index" on the serial console | ✅ | 617ms | - [📜](./05/serial.log) - |
| 6 | Then the latest serial output should contain "Thing-OS Wayland Lab" | ✅ | 0ms | - [📜](./06/serial.log) - |
| 7 | And the latest serial output should contain "Clock" | ✅ | 0ms | - - - |
| 8 | And the latest serial output should contain "title=" | ✅ | 0ms | - - - |
| 9 | When I type "cat /session/wayland/components" on the serial console | ✅ | 565ms | - [📜](./09/serial.log) - |
| 10 | Then the latest serial output should contain "xdg_toplevel" | ✅ | 0ms | - - - |
| 11 | When I type "cat /session/wayland/events/latest" on the serial console | ✅ | 618ms | - [📜](./11/serial.log) - |
| 12 | Then the latest serial output should contain "surface_committed" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[33247400439] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[33495926937] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[33515961600] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[33547540950] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[33579337407] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[33598905813] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[33600542646] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[33636645801] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[33638194920] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[33658049964] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[33672186108] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[33672732489] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[33708352722] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[33709333218] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[33798288612] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[33879241242] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[33901196142] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[33974629161] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[34079073633] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[34111816794] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[34134544224] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[34233076581] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[34284716730] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[34415375016] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[34663037475] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=933370449 elapsed_us=466685
[34663716351] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[34748692968] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[34752256176] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[34789979664] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[34820880600] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[34830865707] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[34834133268] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[34925377707] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[34928610651] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[34934692881] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[35077488600] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[35079249051] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[35115878721] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[35118172056] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[35119612671] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[35121441960] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[35142719700] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[35153390217] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[35156627979] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[35158240095] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[35179084050] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[35183245977] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[35185015569] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[35194928571] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[35198465379] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[35203153095] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[35208155103] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[35210446491] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[35222808555] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[35234818773] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[35258595570] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[35265328692] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[35360667210] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[35371529919] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[35377362669] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[35419973589] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[35585046684] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[35747192811] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[35921451093] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[35929171905] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[35940071343] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[35944795293] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[35946897426] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[36106345803] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[36277279170] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[36327424947] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[36377981244] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[36378922635] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[36538486116] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[36559161738] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1920x1080 @ 60000mHz
[36560355348] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[36561429399] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[36562489590] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[36831738900] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[36997020720] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[37213827057] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[37214766171] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[37215627537] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[37225685805] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Solarized Warm
[37770981402] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[38031674043] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[38092282569] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[38264620383] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[38370600906] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=11)
[38392601841] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=12)
[38561467230] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[38658621804] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=48x48 hotspot=24,24
[38668746468] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[38670962781] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[38709306240] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[38710929114] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[38711667225] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1920x1080 @ 60000mHz ready
[38717585049] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: starting
[38734648986] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: listening on /run/wayland-0
[38736041355] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[38741415405] [[32mINFO [0m] [clock] [CPU3] clock: connected to /run/wayland-0
[38754047607] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
[38864882793] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: connected to /run/wayland-0
[39043626171] [[32mINFO [0m] [clock] [CPU3] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[39166441677] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[39212064837] [[32mINFO [0m] [clock] [CPU3] clock: waiting for system clock anchor
[40106204787] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[40108388100] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[40171601745] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Starting AHCI VFS driver
[40208126673] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[40228098471] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Mounted atapi2 at /dev/storage/atapi2
[40229343891] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Provider loop online at /dev/storage/atapi2
[40247121948] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: bristle pid=8
[40251709542] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: claimed /sys/devices/isa-0070 (handle=2)
[40275401661] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: 2026-04-30 22:14:30 = 1777587270 unix_secs
[40276495182] [[32mINFO [0m] [kernel::time] [CPU3] System clock anchored: unix_secs=1777587270, mono_ns=20138094223, offset=1777587249861905777ns
[40278733011] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: System clock anchored to 1777587270 unix_secs
[40310678529] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:14:30.015961820 unix_secs=1777587270.015961820
[41395902735] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: bristle pid=8
[41494054932] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: sample rate set to 60 Hz
[42905583534] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=3 rect=1920x1080 src_px=0xff40422a dst_px=0xff40422a damage=1920x1080+0,0 res_id=1
[42906841494] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 945,521 src_px=0x10202020 dst_before=0xff6e0404 dst_after=0xff690606
[42915116112] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[42919309653] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=26 frame=6
[42948149376] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[43011947319] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[43019393670] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[44208896028] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[44233579731] [[32mINFO [0m] [bloom::render] [CPU1] bloom: chrome overlay ready buffer=5 size=1920x1080
[45297343113] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:14:32 UTC-8 system_unix=1777587272.509770401
[45871758273] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=26 frame=6
[45909618546] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[46010324943] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: imported dmabuf wl_buffer=111 480x320 format=0x34325241
[46312252734] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:14:33.017750175 unix_secs=1777587273.017750175
[47715736959] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=3 height=26 frame=6
[47729688963] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: xdg_popup obj=23 assigned to xdg_surface=21
[47790262080] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: imported dmabuf wl_buffer=121 160x96 format=0x34325241
[47993866602] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: frame callback done object=1000 data=23986
[48270924240] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: frame callback done object=1001 data=24130
[51530128617] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:14:35 UTC-8 system_unix=1777587275.626718279
[52315731369] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:14:36.019679028 unix_secs=1777587276.019679028
find /session/wayland/windows
/session/wayland/windows
/session/wayland/windows/1
/session/wayland/windows/1/geometry
/session/wayland/windows/1/info
/session/wayland/windows/1/state
/session/wayland/windows/1/title
/session/wayland/windows/2
/session/wayland/windows/2/geometry
/session/wayland/windows/2/info
/session/wayland/windows/2/state
/session/wayland/windows/2/title
/session/wayland/windows/3
/session/wayland/windows/3/geometry
/session/wayland/windows/3/info
/session/wayland/windows/3/state
/session/wayland/windows/3/title
/session/wayland/windows/index
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /session/wayland/[57767519337] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:14:38 UTC-8 system_unix=1777587278.745438455
windows/ind[58318639533] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:14:39.021168651 unix_secs=1777587279.021168651
ex
[?25l1 520x220+40,48 z=1 title="Clock"
2 480x320+76,80 z=2 title="Thing-OS Wayland Lab"
3 160x96+112,112 z=3 title=""
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /session/wayland/components
[?25lwl_display
wl_registry
wl_compositor
wl_shm
xdg_wm_base
xdg_surface
xdg_toplevel
xdg_popup
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /session/wayland/events/latest
surface_committed id=1 frame_serial=11
[?25l
```
</details>
