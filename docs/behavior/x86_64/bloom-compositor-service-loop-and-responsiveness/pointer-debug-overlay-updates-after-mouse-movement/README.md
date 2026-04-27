# ❌ Scenario: pointer debug overlay updates after mouse movement

> Last run: 2026-04-27 12:09:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8918ms | - [📜](./01/serial.log) - |
| 2 | Then the pointer debug overlay should update after mouse movement | ❌ | 7467ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[27514697892] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[27774515934] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[27793702200] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[27826004250] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[27857133282] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[27876283776] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[27877972089] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[27913288722] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[27914836422] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[27933967215] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[27948315747] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[27948853350] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[27984168894] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[27985213542] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[28070592495] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[28156874097] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[28178488833] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[28251941256] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[28357614549] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[28387857366] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[28412345346] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[28524048135] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[28575665844] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[28724408757] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[29006238855] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1001428593 elapsed_us=500714
[29006995710] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[29080481331] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[29083527924] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[29120686716] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[29159243982] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[29172232122] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[29173862421] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[29270531334] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[29273880570] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[29280431268] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[29426353836] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[29427981957] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[29461783824] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[29464731483] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[29489021100] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[29497893414] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[29500114149] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[29501662245] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[29502588291] [[32mINFO [0m] [iso9660d] [CPU2] ISO9660D: Starting VFS provider
[29560669578] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=9)
[29565903675] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[29568365145] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[29578751301] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[29585543130] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[29588355555] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[29593836195] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[29597236548] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[29604482820] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[29611829973] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[29643033057] [[32mINFO [0m] [bristle] [CPU3] bristle: published pid 9 to /run/bristle/pid
[29659569456] [[32mINFO [0m] [bristle] [CPU3] bristle: published device handles kbd_in=3 mouse_in=5
[29672491662] [[32mINFO [0m] [bristle] [CPU3] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[29682457200] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[29780799939] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[29864760552] [[32mINFO [0m] [virtio_gpu] [CPU1] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[29875373220] [[32mINFO [0m] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[29882479077] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[30418796034] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[30572759250] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[30753383298] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[30816022512] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[30855151734] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[30859198062] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: VFS provider loop online
[30886546383] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=11)
[30889874136] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Starting early audio stack
[30895237923] [[32mINFO [0m] [bloom] [CPU2] bloom: ENTERING MAIN
[30896681673] [[32mINFO [0m] [bloom] [CPU2] bloom: compositor service starting
[30916110390] [[32mINFO [0m] [sprout::pipelines] [CPU3] SPROUT: Audio stack worker running
[30945797850] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[30951113160] [[32mINFO [0m] [sprout::pipelines] [CPU3] SPROUT: Early audio device '/sys/devices/pci-0000:00:03.0' matched dev.sound.Virtio; spawning /drivers/virtio_sound
[31205807094] [[32mINFO [0m] [bloom] [CPU2] bloom: connected to /dev/display/card0 on try 0
[31209838044] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[31243442637] [[32mINFO [0m] [bloom] [CPU2] bloom: output0 1920x1080 @ 60000mHz
[31244982483] [[32mINFO [0m] [bloom] [CPU2] bloom: creating service port...
[31454729427] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[31562857095] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pistil background renderer loaded from /lib/libpistil.so
[31571356773] [[32mINFO [0m] [bloom] [CPU2] bloom: initial wallpaper configured /share/wallpapers/flower.png
[31670552661] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[31866247083] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[32046091737] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[32204435901] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[32397323046] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[32672364879] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[32892914472] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[32894938164] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[33403897197] [[32mINFO [0m] [ahci_disk] [CPU3] AHCI: Starting AHCI VFS driver
[33483413634] [[32mINFO [0m] [ahci_disk] [CPU3] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[33507826440] [[32mINFO [0m] [ahci_disk] [CPU3] AHCI: Mounted atapi2 at /dev/storage/atapi2
[33509118687] [[32mINFO [0m] [ahci_disk] [CPU3] AHCI: Provider loop online at /dev/storage/atapi2
[36167307264] [[32mINFO [0m] [bloom::display] [CPU2] bloom: imported buffer 1920x1080 as ID=1
[36202527405] [[32mINFO [0m] [bloom::display] [CPU2] bloom: imported buffer 32x32 as ID=2
[36203863443] [[32mINFO [0m] [bloom::render] [CPU2] bloom: cursor ready buffer=2 size=32x32 hotspot=3,2
[36210693618] [[32mINFO [0m] [bloom] [CPU2] bloom: watching wallpaper config /session/desktop/wallpaper
[36243717576] [[32mINFO [0m] [bloom] [CPU2] bloom: Wayland server thread spawned
[36245087835] [[32mINFO [0m] [bloom::loop_types] [CPU2] bloom: service loop started
[36245886633] [[32mINFO [0m] [bloom::loop_types] [CPU2] bloom: output0 1920x1080 @ 60000mHz ready
[36287535009] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: starting
[36300644325] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: listening on /run/wayland-0
[36314068032] [[32mINFO [0m] [bloom::display] [CPU2] bloom: imported buffer 460x96 as ID=3
[36315460401] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pointer debug overlay ready buffer=3 size=460x96
[36786474747] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: first commit copied buffer=1 rect=1920x1080 src_px=0xff40422a dst_px=0xff40422a damage=1920x1080+0,0 res_id=1
[36797439756] [[32mINFO [0m] [bloom::world] [CPU2] bloom: presented cursor buffer=2 at 957,538 size=32x32
[36798871791] [[32mINFO [0m] [bloom::loop_types] [CPU2] First frame rendered
[36810186336] [[32mINFO [0m] [bloom::services::wallpaper] [CPU2] bloom: preparing background /share/wallpapers/flower.png
[39822314070] [[32mINFO [0m] [bloom::display] [CPU2] bloom: imported buffer 1920x1080 as ID=4
[40298229312] [[32mINFO [0m] [bloom::services::input_service] [CPU2] bloom: registered bristle pointer sink
[40307794659] [[32mINFO [0m] [bristle] [CPU3] bristle: bloom sink registered (fd=6)
[41132735952] [[32mINFO [0m] [sprout::pipelines] [CPU3] SPROUT: Early audio driver started, but /dev/audio/card0/out0 did not appear
[41159115525] [[31;1mERROR[0m] [kernel::trap] [CPU3] user_page_fault tid=15 task='ahci_disk' va=0x0000000000010047 rip=0x0000000000209ce0 err=0x0004 p=0 u=1 w=0 i=0 fs=0x0000000000209000 task_fs=0x0000000000209000
[41213417256] [[32mINFO [0m] [cambium::spawn] [CPU1] CAMBIUM: driver for pci-0000:00:1f.2 exited with code -1
[50280226788] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[53603957286] [[32mINF
```
</details>
