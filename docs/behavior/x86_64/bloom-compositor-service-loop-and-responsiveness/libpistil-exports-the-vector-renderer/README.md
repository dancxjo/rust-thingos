# ✅ Scenario: libpistil exports the vector renderer

> Last run: 2026-04-30 10:54:48

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 10534ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 355ms | - [📜](./02/serial.log) - |
| 3 | And I type "test_dlopen" on the serial console | ✅ | 9539ms | - [📜](./03/serial.log) - |
| 4 | Then the serial output should contain "[test_dlopen] pistil_draw_vector_smoke: PASS" within 60s | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[32806427478] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[33233417250] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[33254216292] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[33290547972] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[33325115340] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[33345472677] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[33347441523] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[33385005852] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[33386836560] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[33408151458] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[33425427354] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[33426232488] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[33467358111] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[33468564690] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[33562764279] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[33646313250] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[33668878320] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[33743888079] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[33849453264] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[33882323244] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[33906604908] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[34024227831] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[34100987151] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[34257532815] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[34550375145] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1060506612 elapsed_us=530253
[34551270897] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[34629761925] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[34633528941] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[34674486924] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[34744547409] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[34754409492] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[34756081965] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[34899494157] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[34903135608] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[34910713728] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[35078891694] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[35080878327] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[35140735311] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[35143507311] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[35145307263] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[35145077121] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[35236511079] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[35237489232] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[35257319922] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[35259219699] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[35266603251] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[35272175565] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[35274161274] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[35285355600] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[35288557062] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[35293978764] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[35300086965] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[35472717456] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[35487519870] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[35498188737] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[35566391223] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[35572320102] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[35784843369] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
te[35827616385] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[35833995681] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[36519665622] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[37270701567] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[37995651870] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[38749211985] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
st_dlopen
[?25l[39309262773] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[39318689883] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[39325560186] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[39328385415] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[39427366242] [[32mINFO [0m] [user.print] [CPU1] --- test_dlopen starting ---
[39428551635] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlopen_nonexistent: starting
[39432860676] [[32mINFO [0m] [user.print] [CPU1]   dlerror: dlopen: cannot read library file
[39433904301] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlopen_nonexistent: PASS
[39434754777] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlopen_null_path: starting
[39437831565] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlopen_null_path: PASS
[39438756621] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlerror_cleared_after_read: starting
[39441355833] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlerror_cleared_after_read: PASS
[39442211457] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlsym_unknown_symbol: starting
[39443548914] [[32mINFO [0m] [user.print] [CPU1]   dlerror: dlsym: symbol not found
[39444335931] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlsym_unknown_symbol: PASS
[39445110969] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlsym_invalid_handle: starting
[39446014146] [[32mINFO [0m] [user.print] [CPU1]   dlerror: dlsym: invalid handle
[39446782419] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlsym_invalid_handle: PASS
[39447554718] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlclose_rtld_default: starting
[39448631673] [[32mINFO [0m] [user.print] [CPU1]   dlerror: dlclose: cannot close RTLD_DEFAULT
[39449420373] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlclose_rtld_default: PASS
[39450246363] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlclose_invalid_handle: starting
[39451161156] [[32mINFO [0m] [user.print] [CPU1]   dlerror: dlclose: invalid handle
[39451922466] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlclose_invalid_handle: PASS
[39452697075] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlclose_double_close: starting
[39453567615] [[32mINFO [0m] [user.print] [CPU1]   dlerror: dlclose: cannot close RTLD_DEFAULT
[39454730139] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlclose_double_close: PASS
[39455993445] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlopen_pistil_shared_library: starting
[39800205687] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=11)
[39807613824] [[32mINFO [0m] [bloom] [CPU2] bloom: ENTERING MAIN
[39808780374] [[32mINFO [0m] [bloom] [CPU2] bloom: compositor service starting
[39911903724] [[32mINFO [0m] [bloom] [CPU2] bloom: connected to /dev/display/card0 on try 0
[39970167699] [[32mINFO [0m] [bloom] [CPU2] bloom: output0 1920x1080 @ 60000mHz
[39971185122] [[32mINFO [0m] [bloom] [CPU2] bloom: display driver supports VBLANK (vsync)
[39971946333] [[32mINFO [0m] [bloom] [CPU2] bloom: creating service port...
[40259528859] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[40952627826] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[41016247668] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pistil background renderer loaded from /lib/libpistil.so
[41017484904] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pistil font text renderer loaded with default /share/fonts/NotoSans-Regular.ttf
[41018487807] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[41892431592] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=12)
[42007598919] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[43597064247] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[45914739453] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[48144992643] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] pistil_draw_vector_smoke: PASS
[48161957943] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[48372130191] [[32mINFO [0m] [bloom] [CPU2] bloom: initial wallpaper configured /share/wallpapers/flower.png
[48669233448] [[32mINFO [0m] [bloom::render] [CPU2] bloom: cursor ready buffer=2 size=96x96 hotspot=24,15
[48713591025] [[32mINFO [0m] [bloom] [CPU2] bloom: watching wallpaper config /session/desktop/wallpaper
[48816266796] [[32mINFO [0m] [bloom] [CPU2] bloom: Wayland server thread spawned
[48823079316] [[32mINFO [0m] [bloom::loop_types] [CPU2] bloom: service loop started
[48824734530] [[32mINFO [0m] [bloom::loop_types] [CPU2] bloom: output0 1920x1080 @ 60000mHz ready
[48839891100] [[32mINFO [0m] [bloom::services::wallpaper] [CPU2] bloom: preparing background /share/wallpapers/flower.png
[49114956924] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[49681834818] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: starting
[49698948420] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: listening on /run/wayland-0
[49873151922] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: connected to /run/wayland-0
[52467378381] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: pistil text renderer loaded with default /share/fonts/NotoSans-Regular.ttf
[63442326486] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[64602149370] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[64605514743] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[65450331936] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: bristle pid=8
[65573901261] [[32mINFO [0m] [ps2_mouse] [CPU1] ps2_mouse: bristle pid=8
[65705894067] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Starting AHCI VFS driver
[66053171085] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[66091190517] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Mounted atapi2 at /dev/storage/atapi2
[66092692116] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Provider loop online at /dev/storage/atapi2
[67062526476] [[32mINFO
```
</details>
