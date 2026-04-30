# ✅ Scenario: libpistil exports the vector renderer

> Last run: 2026-04-30 10:32:08

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 19629ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 463ms | - [📜](./02/serial.log) - |
| 3 | And I type "test_dlopen" on the serial console | ✅ | 16944ms | - [📜](./03/serial.log) - |
| 4 | Then the serial output should contain "[test_dlopen] pistil_draw_vector_smoke: PASS" within 60s | ✅ | 1ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[61291066518] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[62042707914] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[62074022802] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[62139874962] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[62198550513] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[62235022113] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[62238350559] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[62300894106] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[62303843745] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[62336774973] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[62362293477] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[62363496228] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[62427392115] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[62429249256] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[62585559168] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[62724858504] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[62760037329] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[62872638048] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[63021261072] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[63095467644] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[63133949604] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[63317591568] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[63433050417] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[63745540056] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[64233499440] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1771461219 elapsed_us=885730
[64235051298] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[64412838072] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[64424886207] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[64498296225] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[64641729372] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[64669316085] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[64686867927] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[65033694891] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[65046820773] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[65058259002] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
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
[65226322722] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[65229467358] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[65423308368] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[65428613052] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[65431494579] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[65444282475] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[65584953753] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[65600571795] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[65603382306] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[65653859370] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[65690587380] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[65709882084] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[65754033015] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[65763549093] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[65772152853] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[65797874472] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[65830203714] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[65939695404] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[65983099116] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[66003249576] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[66144196668] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[66212017542] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
test_dlopen
[?25l[66978391524] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[67002490533] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[67039748226] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[67651125729] [[32mINFO [0m] [user.print] [CPU1] --- test_dlopen starting ---
[67653293499] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlopen_nonexistent: starting
[67671398058] [[32mINFO [0m] [user.print] [CPU1]   dlerror: dlopen: cannot read library file
[67685473713] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlopen_nonexistent: PASS
[67687341084] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlopen_null_path: starting
[67689073947] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlopen_null_path: PASS
[67690721670] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlerror_cleared_after_read: starting
[67695129150] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlerror_cleared_after_read: PASS
[67696550526] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlsym_unknown_symbol: starting
[67715888229] [[32mINFO [0m] [user.print] [CPU1]   dlerror: dlsym: symbol not found
[68780307090] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[70684689669] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlsym_unknown_symbol: PASS
[70686677688] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlsym_invalid_handle: starting
[70688716065] [[32mINFO [0m] [user.print] [CPU1]   dlerror: dlsym: invalid handle
[70691827734] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlsym_invalid_handle: PASS
[70701055128] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlclose_rtld_default: starting
[70703800497] [[32mINFO [0m] [user.print] [CPU1]   dlerror: dlclose: cannot close RTLD_DEFAULT
[70705164849] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlclose_rtld_default: PASS
[70706467425] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlclose_invalid_handle: starting
[70708016478] [[32mINFO [0m] [user.print] [CPU1]   dlerror: dlclose: invalid handle
[70709255496] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlclose_invalid_handle: PASS
[70710517977] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlclose_double_close: starting
[70716809163] [[32mINFO [0m] [user.print] [CPU1]   dlerror: dlclose: cannot close RTLD_DEFAULT
[70719641190] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlclose_double_close: PASS
[70721931390] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlopen_pistil_shared_library: starting
[71460760470] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[74384081904] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[74400142476] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[74412897405] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[74416221495] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[74591777238] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[75271605849] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=11)
[75278817306] [[32mINFO [0m] [bloom] [CPU2] bloom: ENTERING MAIN
[75280606500] [[32mINFO [0m] [bloom] [CPU2] bloom: compositor service starting
[75486225408] [[32mINFO [0m] [bloom] [CPU2] bloom: connected to /dev/display/card0 on try 0
[75737495493] [[32mINFO [0m] [bloom] [CPU2] bloom: output0 1920x1080 @ 60000mHz
[75739209777] [[32mINFO [0m] [bloom] [CPU2] bloom: display driver supports VBLANK (vsync)
[75740453514] [[32mINFO [0m] [bloom] [CPU2] bloom: creating service port...
[76375876830] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[77757026787] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=12)
[78610041807] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[78630353109] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pistil background renderer loaded from /lib/libpistil.so
[78632248497] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pistil font text renderer loaded with default /share/fonts/NotoSans-Regular.ttf
[78633840516] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[81031303320] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] pistil_draw_vector_smoke: PASS
[87560561517] [[32mINFO [0m] [bloom] [CPU2] bloom: initial wallpaper configured /share/wallpapers/flower.png
[88191685230] [[32mINFO [0m] [bloom::render] [CPU2] bloom: cursor ready buffer=2 size=96x96 hotspot=24,15
[88214019960] [[32mINFO [0m] [bloom] [CPU2] bloom: watching wallpaper config /session/desktop/wallpaper
[88456293684] [[32mINFO [0m] [bloom] [CPU2] bloom: Wayland server thread spawned
[88460735220] [[32mINFO [0m] [bloom::loop_types] [CPU2] bloom: service loop started
[88463949519] [[32mINFO [0m] [bloom::loop_types] [CPU2] bloom: output0 1920x1080 @ 60000mHz ready
[88498629186] [[32mINFO [0m] [bloom::services::wallpaper] [CPU2] bloom: preparing background /share/wallpapers/flower.png
[100559876274] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[100588861923] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: starting
[100633070637] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: listening on /run/wayland-0
[100716151503] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: connected to /run/wayland-0
[103634308536] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: pistil text renderer loaded with default /share/fonts/NotoSans-Regular.ttf
[104082348117] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[105954732774] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[108464150652] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[110397050247] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[112414402848] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[114276944199] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[116495367351] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[116500909635] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[117427084071] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Starting AHCI VFS driver
[117697836003] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: bristle pid=8
[117700830291] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[117801443397] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Mounted atapi2 at /dev/storage/atapi2
[117802566585] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Provider loop online at /dev/storage/atapi2
[117830221542] [[32mINFO [0m] [ps2_mouse] [CPU1] ps2_mouse: bristle pid=8
[119494062138] [[32mINFO [0m] [ps2_mouse] [CPU1] ps2_mouse: sample rate set to 60 Hz
[119792010162] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=3 rect=1920x1080 src_px=0xff40422a dst_px=0xff40422a damage=1920x1080+0,0 res_id=1
[119798270559] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 955,536 src_px=0x20181818 dst_before=0xff8c0404 dst_after=0xff7d0707
[119842311633] [[32mINFO [0m] [pistil::compositor] [CPU1] pistil: drawing debug text with /share/fonts/NotoSans-Regular.ttf
[119859065370] [[32mINFO [0m] [bloom::loop_types] [CPU2] First frame rendered
[119859911160] [[32mINFO [0m] [user.print] [CPU1] [test_dlopen] test_dlopen_pistil_shared_library: PASS
[119861480673] [[32mINFO [0m] [user.print] [CPU1] --- test_dlopen: all tests PASSED ---
[119876241837] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU2] bloom: registered titlebar drag zone surface=1 height=40 frame=6
[119896769586] [[32mINFO [0m] [bloom::services::input_service] [CPU2] bloom: registered bristle pointer sink
[119942521314] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU1] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[119971379418] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[?25h
```
</details>
