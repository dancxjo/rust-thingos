# ✅ Scenario: libpistil exports the vector renderer

> Last run: 2026-04-30 02:28:06

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8610ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 254ms | - [📜](./02/serial.log) - |
| 3 | And I type "test_dlopen" on the serial console | ✅ | 2282ms | - [📜](./03/serial.log) - |
| 4 | Then the serial output should contain "[test_dlopen] pistil_draw_vector_smoke: PASS" within 60s | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26409504132] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[26666463219] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26686555962] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26719667436] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26752449108] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26772159282] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26773986954] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26812134195] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[26813752350] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26833827900] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26848897482] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[26849426472] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[26886307767] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26887343670] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26973889503] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[27059890539] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[27083695023] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[27167299038] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[27281443629] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[27309554151] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[27334759650] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[27452260209] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[27504358035] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[27665699919] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27962461824] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1054630038 elapsed_us=527315
[27963201882] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[28045725906] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[28049148270] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[28092636198] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[28136884578] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[28147721283] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[28149425073] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[28336448844] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[28340108082] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[28349219019] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28512227205] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[28514826417] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[28568334168] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[28575184902] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[28628960184] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[28649177403] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[28651875318] [[32mINFO [0m] [iso9660d] [CPU2] ISO9660D: Starting VFS provider
[28654607025] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[28657079649] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[28719020055] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=9)
[28725215805] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[28728176070] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[28743539022] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[28750498557] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[28753984941] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[28757894484] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[28761147459] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[28767605988] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[28776434577] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[28794059547] [[32mINFO [0m] [bristle] [CPU3] bristle: published pid 9 to /run/bristle/pid
[28813521990] [[32mINFO [0m] [bristle] [CPU3] bristle: published device handles kbd_in=3 mouse_in=5
[28829824386] [[32mINFO [0m] [bristle] [CPU3] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[28857066480] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[28968207180] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[29074104906] [[32mINFO [0m] [virtio_gpu] [CPU1] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[29085326853] [[32mINFO [0m] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[29091739050] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
test_dlopen
[29708635803] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[29783323746] [[32mINFO [0m] [user.print] [CPU2] --- test_dlopen starting ---
[?25l[29791583514] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlopen_nonexistent: starting
[29798420058] [[32mINFO [0m] [user.print] [CPU2]   dlerror: dlopen: cannot read library file
[29799238161] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlopen_nonexistent: PASS
[29800348413] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlopen_null_path: starting
[29801320164] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlopen_null_path: PASS
[29802042336] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlerror_cleared_after_read: starting
[29804542911] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlerror_cleared_after_read: PASS
[29805541194] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlsym_unknown_symbol: starting
[29806691178] [[32mINFO [0m] [user.print] [CPU2]   dlerror: dlsym: symbol not found
[29807336658] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlsym_unknown_symbol: PASS
[29807970654] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlsym_invalid_handle: starting
[29808719193] [[32mINFO [0m] [user.print] [CPU2]   dlerror: dlsym: invalid handle
[29809529673] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlsym_invalid_handle: PASS
[29810551749] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlclose_rtld_default: starting
[29811880461] [[32mINFO [0m] [user.print] [CPU2]   dlerror: dlclose: cannot close RTLD_DEFAULT
[29812502346] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlclose_rtld_default: PASS
[29813193135] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlclose_invalid_handle: starting
[29813999952] [[32mINFO [0m] [user.print] [CPU2]   dlerror: dlclose: invalid handle
[29814598671] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlclose_invalid_handle: PASS
[29815231017] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlclose_double_close: starting
[29815918902] [[32mINFO [0m] [user.print] [CPU2]   dlerror: dlclose: cannot close RTLD_DEFAULT
[29816557716] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlclose_double_close: PASS
[29817295167] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlopen_pistil_shared_library: starting
[29922849297] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[29960469033] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[30015308928] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=12)
[30018164055] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[30022525170] [[32mINFO [0m] [bloom] [CPU3] bloom: ENTERING MAIN
[30023842827] [[32mINFO [0m] [bloom] [CPU3] bloom: compositor service starting
[30023961264] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: VFS provider loop online
[30024863484] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Starting early audio stack
[30113802774] [[32mINFO [0m] [sprout::pipelines] [CPU1] SPROUT: Audio stack worker running
[30143198910] [[32mINFO [0m] [sprout::pipelines] [CPU1] SPROUT: Early audio device '/sys/devices/pci-0000:00:03.0' matched dev.sound.Virtio; spawning /drivers/virtio_sound
[30203298015] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[30251778975] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] pistil_draw_vector_smoke: PASS
[30364224726] [[32mINFO [0m] [bloom] [CPU3] bloom: connected to /dev/display/card0 on try 0
[30401401074] [[32mINFO [0m] [bloom] [CPU3] bloom: output0 1920x1080 @ 60000mHz
[30402831657] [[32mINFO [0m] [bloom] [CPU3] bloom: creating service port...
[30487667298] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[30659566971] [[32mINFO [0m] [virtio_sound] [CPU0] SND: rpc Lookup 'out0'
[30748491081] [[32mINFO [0m] [bloom::render] [CPU3] bloom: pistil background renderer loaded from /lib/libpistil.so
[30756476190] [[32mINFO [0m] [bloom] [CPU3] bloom: initial wallpaper configured /share/wallpapers/flower.png
[30866045298] [[32mINFO [0m] [virtio_sound] [CPU0] SND: rpc 6
[30920877108] [[32mINFO [0m] [sprout::pipelines] [CPU1] SPROUT: Audio stack launched chime (PID=15)
[30931014576] [[32mINFO [0m] [chime] [CPU0] chime: Waiting for /dev/audio/card0/out0 ...
[30938462577] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[30948602652] [[32mINFO [0m] [virtio_sound] [CPU0] SND: rpc Lookup 'out0'
[30974784852] [[32mINFO [0m] [chime] [CPU0] chime: Opened /dev/audio/card0/out0 (fd=17)
[30997160469] [[32mINFO [0m] [virtio_sound] [CPU0] SND: rpc DeviceCall handle=2 op=32769
[31041663873] [[32mINFO [0m] [virtio_sound] [CPU0] SND: rpc DeviceCall handle=2 op=32770
[31073810295] [[32mINFO [0m] [chime] [CPU0] chime: Configured stream (rate=44100Hz, fmt=1, ch=2)
[31094093382] [[32mINFO [0m] [virtio_sound] [CPU0] SND: rpc DeviceCall handle=2 op=32773
[31110663672] [[32mINFO [0m] [virtio_sound] [CPU0] SND: control SET_PARAMS begin for stream 0
[31115507280] [[32mINFO [0m] [virtio_sound] [CPU0] SND: control SET_PARAMS complete for stream 0
[31118014059] [[32mINFO [0m] [virtio_sound] [CPU0] SND: control PREPARE begin for stream 0
[31128768297] [[32mINFO [0m] [chime] [CPU0] chime: Generating classic chime...
[31136329653] [[32mINFO [0m] [virtio_sound] [CPU0] SND: control PREPARE complete for stream 0
[31138982754] [[32mINFO [0m] [virtio_sound] [CPU0] SND: control START begin for stream 0
[31141399047] [[32mINFO [0m] [virtio_sound] [CPU0] SND: control START complete for stream 0
[31143614634] [[32mINFO [0m] [virtio_sound] [CPU0] SND: Hardware playback started
[31224739623] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[31550074116] [[32mINFO [0m] [bloom::display] [CPU3] bloom: imported buffer 1920x1080 as ID=1
[31553516313] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[31766940084] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[31902834447] [[32mINFO [0m] [bloom::display] [CPU3] bloom: imported buffer 96x96 as ID=2
[31904373270] [[32mINFO [0m] [bloom::render] [CPU3] bloom: cursor ready buffer=2 size=96x96 hotspot=9,6
[31916044446] [[32mINFO [0m] [bloom] [CPU3] bloom: watching wallpaper config /session/desktop/wallpaper
[31952718468] [[32mINFO [0m] [bloom] [CPU3] bloom: Wayland server thread spawned
[31954719192] [[32mINFO [0m] [bloom::loop_types] [CPU3] bloom: service loop started
[31955806575] [[32mINFO [0m] [bloom::loop_types] [CPU3] bloom: output0 1920x1080 @ 60000mHz ready
[31991220657] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: starting
[32003743002] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: listening on /run/wayland-0
[32027136438] [[32mINFO [0m] [bloom::display] [CPU3] bloom: imported buffer 460x144 as ID=3
[32028764757] [[32mINFO [0m] [bloom::render] [CPU3] bloom: pointer debug overlay ready buffer=3 size=460x144
[32029666845] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[32420396184] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[32646865812] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[32845450209] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[33083771886] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[33085670178] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[33406333818] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: first commit copied buffer=1 rect=1920x1080 src_px=0xffccccff dst_px=0xffccccff damage=1920x1080+0,0 res_id=1
[33408289662] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: cursor plane blended buffer=2 at 970,545 src_px=0x20181818 dst_before=0xffccccff dst_after=0xffb5b5e2
[33418375023] [[32mINFO [0m] [bloom::world] [CPU3] bloom: presented cursor buffer=2 at 951,534 size=96x96
[33420683406] [[32mINFO [0m] [bloom::loop_types] [CPU3] First frame rendered
[33435077148] [[32mINFO [0m] [bloom::services::wallpaper] [CPU3] bloom: preparing background /share/wallpapers/flower.png
[33638335203] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: bristle pid=9
[34053123027] [[32mINFO [0m] [pistil::font] [CPU2] pistil: opening font /share/fonts/NotoSans-Regular.ttf
[34058883507] [[32mINFO [0m] [pistil::font] [CPU2] pistil: stat font /share/fonts/NotoSans-Regular.ttf
[34061478099] [[32mINFO [0m] [pistil::font] [CPU2] pistil: reading font 569208 bytes from /share/fonts/NotoSans-Regular.ttf
[34197406782] [[32mINFO [0m] [pistil::font] [CPU2] pistil: font read returned 569208 of 569208 bytes
[34199178519] [[32mINFO [0m] [pistil::font] [CPU2] pistil: parsing font /share/fonts/NotoSans-Regular.ttf
[34423301319] [[32mINFO [0m] [ahci_disk] [CPU3] AHCI: Starting AHCI VFS driver
[34472207550] [[32mINFO [0m] [ahci_disk] [CPU3] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[34515138702] [[32mINFO [0m] [ps2_mouse] [CPU3] ps2_mouse: bristle pid=9
[34531689621] [[32mINFO [0m] [ahci_disk] [CPU3] AHCI: Mounted atapi2 at /dev/storage/atapi2
[34533135450] [[32mINFO [0m] [ahci_disk] [CPU3] AHCI: Provider loop online at /dev/storage/atapi2
[34890746748] [[32mINFO [0m] [pistil::font] [CPU2] pistil: parsed font /share/fonts/NotoSans-Regular.ttf
[34891950555] [[32mINFO [0m] [pistil::compositor] [CPU2] pistil: drawing debug text with /share/fonts/NotoSans-Regular.ttf
[34900435944] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlopen_pistil_shared_library: PASS
[34901135313] [[32mINFO [0m] [user.print] [CPU2] --- test_dlopen: all tests PASSED ---
[?25h[1;95mTHING[0
```
</details>
