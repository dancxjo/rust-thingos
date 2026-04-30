# ❌ Scenario: first valid commit is visible above the compositor background

> Last run: 2026-04-30 15:09:02

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Then the Wayland hello client should be visible | ❌ | 32255ms | - [📜](./01/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[32125423209] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[32374382073] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[32395413039] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[32427374331] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[32460087198] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[32480393154] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[32482079916] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[32518804197] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[32520283323] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[32540935779] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[32555168250] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[32555897352] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[32591174814] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[32592103929] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[32681462088] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[32762688948] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[32785084893] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[32857700997] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[32959580643] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[32993315091] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[33016978896] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[33121496925] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[33178814790] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[33345172509] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[33641696649] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1028856972 elapsed_us=514428
[33642395952] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[33718804515] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[33722258724] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[33759752004] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[33791634327] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[33833061108] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[33835023354] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[33928301682] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[33932667054] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[33939490101] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[34111607541] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[34113715383] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[34154731017] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[34157966568] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[34159592709] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[34160240796] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[34183982910] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[34205187159] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[34211201706] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[34213138080] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[34222101342] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[34227946929] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[34230424272] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[34243372911] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[34247110524] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[34252830810] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[34258904592] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[34260353226] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[34272837522] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[34285158336] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[34312336212] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[34318521600] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[34408985061] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[34418796621] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[34424415630] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[34474561737] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[34640396109] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[34805717760] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[34967880090] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[34971924702] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[34976352840] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[34980171402] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[34984620924] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[35144813154] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[35304227970] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[35384603133] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[35401308525] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[35402213385] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[35611090713] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[35776546344] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[35930615457] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[36084460500] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[36172894263] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[36194086731] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1920x1080 @ 60000mHz
[36194915097] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[36195576252] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[36196188831] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[36499868526] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[36501127410] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[36502027320] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[36510867393] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Solarized Warm
[36971342694] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[37210556922] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[37384197180] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[37426146120] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=11)
[37449609549] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=12)
[37482602091] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[37484301954] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[37540105746] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Starting AHCI VFS driver
[37567612698] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[37582818933] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Mounted atapi2 at /dev/storage/atapi2
[37583615091] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Provider loop online at /dev/storage/atapi2
[37642927476] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=48x48 hotspot=24,24
[37653432597] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[37655385042] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[37686330594] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[37687959408] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[37688822754] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1920x1080 @ 60000mHz ready
[37691643066] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: starting
[37706925102] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: listening on /run/wayland-0
[37708080333] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[37751466852] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: claimed /sys/devices/isa-0070 (handle=2)
[37824772821] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: bristle pid=8
[37825561224] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: bristle pid=8
[37838548473] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: connected to /run/wayland-0
[37899369717] [[32mINFO [0m] [clock] [CPU3] clock: connected to /run/wayland-0
[38144436528] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[38170928961] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[38187423219] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: 2026-04-30 22:09:31 = 1777586971 unix_secs
[38188464138] [[32mINFO [0m] [kernel::time] [CPU3] System clock anchored: unix_secs=1777586971, mono_ns=19094081242, offset=1777586951905918758ns
[38189213469] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: System clock anchored to 1777586971 unix_secs
[38195561547] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:09:31.002247894 unix_secs=1777586971.002247894
[38211736035] [[32mINFO [0m] [clock] [CPU3] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[38227296228] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: sample rate set to 60 Hz
[38323333785] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=1 rect=1920x1080 src_px=0xffccccff dst_px=0xffccccff damage=1920x1080+0,0 res_id=1
[38324614020] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 945,521 src_px=0x10202020 dst_before=0xffccccff dst_after=0xffc1c1f1
[38332971270] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[38337500388] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=26 frame=6
[38348263965] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[38401455840] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
[38409571695] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: imported dmabuf wl_buffer=111 480x320 format=0x34325241
[39233863515] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:09:31 UTC-8 system_unix=1777586971.520530071
[42445097673] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[42452015727] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[42768771210] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=26 frame=6
[42776743911] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[43842789804] [[32mINFO [0m] [bloom::render] [CPU1] bloom: chrome overlay ready buffer=5 size=1920x1080
[44198674476] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:09:34.005182554 unix_secs=1777586974.005182554
[44779177179] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=3 height=26 frame=6
[44789597787] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_popup obj=23 assigned to xdg_surface=21
[44868809040] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: imported dmabuf wl_buffer=121 160x96 format=0x34325241
[45055964151] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: frame callback done object=1000 data=22520
[45785828946] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:09:34 UTC-8 system_unix=1777586974.798456437
[45925011198] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: frame callback done object=1001 data=22958
[50204524293] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:09:37.008121999 unix_secs=1777586977.008121999
[52089165678] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:09:37 UTC-8 system_unix=1777586977.950278665
[56210460471] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:09:40.011104773 unix_secs=1777586980.011104773
[58330736178] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:09:41 UTC-8 system_unix=1777586981.070914112
[62216576070] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:09:43.014144472 unix_secs=1777586983.014144472
[64739847117] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:09:44 UTC-8 system_unix=1777586984.275416732
[68227437234] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:09:46.019565616 unix_secs=1777586986.019565616
[70913506488] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:09:47 UTC-8 system_unix=1777586987.362400874
[74241142800] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:09:49.026429421 unix_secs=1777586989.026429421
[77221705011] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:09:50 UTC-8 system_unix=1777586990.516514061
[80268891417] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:09:52.040273452 unix_secs=1777586992.040273452
[83477807919] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:09:53 UTC-8 system_unix=1777586993.644383157
[86273969331] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:09:55.042856761 unix_secs=1777586995.042856761
[89705544951] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:09:56 UTC-8 system_unix=1777586996.758190128
[92280060246] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:09:58.045865952 unix_secs=1777586998.045865952
[95845014078] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:09:59 UTC-8 system_unix=1777586999.828022520
[98286174657] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:10:01.048910914 unix_secs=1777587001.048910914
[101929537017] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:10:02 UTC-8 system_unix=1777587002.870225184
[104291932140] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:10:04.051809951 unix_secs=1777587004.051809951
[108122245836] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:10:05 UTC-8 system_unix=1777587005.966776207
[110297878251] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:10:07.054785316 unix_secs=1777587007.054785316
[114262610319] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:10:09 UTC-8 system_unix=1777587009.036796254
[116304051072] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:10:10.057887352 unix_secs=1777587010.057887352
[120303318168] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:10:12 UTC-8 system_unix=1777587012.057298546
[122309805651] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:10:13.060767562 unix_secs=1777587013.060767562
[126792139221] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:10:15 UTC-8 system_unix=1777587015.301818121
[128316388959] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:10:16.064037205 unix_secs=1777587016.064037205
[132986513253] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:10:18 UTC-8 system_unix=1777587018.399003751
[134321550330] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:10:19.066650066 unix_secs=1777587019.066650066
[139093383561] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:10:21 UTC-8 system_unix=1777587021.452245492
[140327539077] [[32mINFO [0m]
```
</details>
