# ❌ Scenario: pointer motion events are coalesced to latest-per-frame

> Last run: 2026-04-30 10:54:48

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 13391ms | - [📜](./01/serial.log) - |
| 2 | Then the pointer debug overlay should update after mouse movement | ❌ | 42663ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[41897553291] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[42331421286] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[42352357245] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[42394385682] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[42431165799] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[42452399451] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[42454474755] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[42495167517] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[42497137188] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[42520758588] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[42537375276] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[42538178265] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[42579702891] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[42581063646] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[42681542574] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[42792786003] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[42825453990] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[42903340029] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[43013449512] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[43046404368] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[43070219148] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[43198311585] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[43297179486] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[43482001860] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[43842519633] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1239490626 elapsed_us=619745
[43843483266] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[43934141493] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[43940879433] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[43986101610] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[44117535462] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[44130389292] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[44132817234] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[44328365037] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[44332313388] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[44341758417] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[44501924016] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[44504141187] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[44587440315] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[44590525749] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[44598664308] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[44600704929] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[44697171156] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[44740434915] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[44746207209] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[44749492458] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[44769653148] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[44776309512] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[44778565953] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[44792849574] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[44797368990] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[44803923120] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[44814783288] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[45044373660] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[45084965871] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[45128743836] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[45150917625] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[45161509899] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[45545257131] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[45592110465] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[45615864756] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[46737377907] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[47714417850] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[48809248125] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[49985134386] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[49991382342] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[50004616101] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[50010808089] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[50017265166] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[50575897086] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[50733833700] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[50735185545] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[50755045836] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[51558382623] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[51657235773] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1920x1080 @ 60000mHz
[51658343682] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports VBLANK (vsync)
[51659254284] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[51688393317] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[53013066315] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=11)
[53823110319] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[53840137131] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[55159519767] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[55668379404] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/NotoSans-Regular.ttf
[55669711614] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[59191723470] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[59734745862] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[60126258357] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[61021871724] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[61361639196] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=96x96 hotspot=24,15
[61379332839] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[61441359177] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[61442893512] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[61443744549] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1920x1080 @ 60000mHz ready
[61448549976] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: starting
[61454354544] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
[61464189996] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: listening on /run/wayland-0
[61478409036] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: connected to /run/wayland-0
[63764581551] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: pistil text renderer loaded with default /share/fonts/NotoSans-Regular.ttf
[72981907416] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[73627568190] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[73630181592] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[73887361845] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Starting AHCI VFS driver
[73969608768] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[74004184947] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Mounted atapi2 at /dev/storage/atapi2
[74005394562] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Provider loop online at /dev/storage/atapi2
[75028473960] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: bristle pid=8
[75133289880] [[32mINFO [0m] [ps2_mouse] [CPU3] ps2_mouse: bristle pid=8
[75298165041] [[32mINFO [0m] [ps2_mouse] [CPU3] ps2_mouse: sample rate set to 60 Hz
[76339947783] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=3 rect=1920x1080 src_px=0xff40422a dst_px=0xff40422a damage=1920x1080+0,0 res_id=1
[76341755886] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 955,536 src_px=0x20181818 dst_before=0xff8c0404 dst_after=0xff7d0707
[76374409782] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[76379212140] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=40 frame=6
[76393043331] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[76587151509] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[76746712878] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[77039385192] [[32mINFO [0m] [bloom::input] [CPU1] bloom: pointer debug overlay enabled
[77268066843] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pointer debug overlay ready buffer=4 size=460x144
[81529923321] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=40 frame=6
[81569218104] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_popup obj=23 assigned to xdg_surface=21
[84680144472] [[32mINFO [0m] [bloom::render] [CPU1] bloom: chrome overlay ready buffer=6 size=1920x1080
[87555091536] [[3
```
</details>
