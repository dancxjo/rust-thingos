# ✅ Scenario: Startup chime opens the audio output through the audio provider

> Last run: 2026-04-30 01:55:04

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 0ms | - - - |
| 2 | When I wait for the serial output to contain "chime: Opened /dev/audio/card0/out0" | ✅ | 10950ms | - [📜](./02/serial.log) - |
| 3 | Then the serial output should contain "SPROUT: Audio stack launched chime" | ✅ | 0ms | - - - |
| 4 | And the serial output should not contain "DISP: vfs_lookup path='out0'" | ✅ | 300ms | - [📜](./04/serial.log) - |
| 5 | And the serial output should not contain "KERNEL PAGE FAULT" | ✅ | 300ms | - [📜](./05/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[30692649669] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[31058272014] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[31080245559] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[31115364753] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[31150422336] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[31171839501] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[31173719346] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[31226910891] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[31229297550] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[31251421410] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[31269133368] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[31269839205] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[31315220937] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[31316382702] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[31427156838] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[31540645422] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[31566293154] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[31643036040] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[31749380982] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[31802174547] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[31836937110] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[31979579313] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[32040837741] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[32219522412] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[32601109431] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1259499186 elapsed_us=629749
[32602235325] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[32708298117] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[32712765327] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[32765993898] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[32832636276] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[32851866663] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[32853789738] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[32968811931] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[32972826744] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[32985615729] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[33147051465] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[33149979324] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[33204536772] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[33209667513] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[33247748061] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[33250229760] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[33252970938] [[32mINFO [0m] [iso9660d] [CPU2] ISO9660D: Starting VFS provider
[33253658889] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[33257353074] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[33315485313] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=9)
[33321060234] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[33323667135] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[33337917987] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[33343872540] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[33346214847] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[33350721393] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[33354361227] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[33362452893] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[33372892410] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[33410109447] [[32mINFO [0m] [bristle] [CPU3] bristle: published pid 9 to /run/bristle/pid
[33431378211] [[32mINFO [0m] [bristle] [CPU3] bristle: published device handles kbd_in=3 mouse_in=5
[33449443731] [[32mINFO [0m] [bristle] [CPU3] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[33457127583] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[33638255211] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[33852403695] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[33963200271] [[32mINFO [0m] [virtio_gpu] [CPU1] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[33979626780] [[32mINFO [0m] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[33991424445] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[34939304565] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[34970888634] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[35048835162] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=11)
[35055134961] [[32mINFO [0m] [bloom] [CPU2] bloom: ENTERING MAIN
[35056905642] [[32mINFO [0m] [bloom] [CPU2] bloom: compositor service starting
[35060327016] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Starting early audio stack
[35094824391] [[32mINFO [0m] [sprout::pipelines] [CPU3] SPROUT: Audio stack worker running
[35121079488] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[35140260111] [[32mINFO [0m] [sprout::pipelines] [CPU3] SPROUT: Early audio device '/sys/devices/pci-0000:00:03.0' matched dev.sound.Virtio; spawning /drivers/virtio_sound
[35250758037] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[35257034637] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: VFS provider loop online
[35411587629] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[35412911919] [[32mINFO [0m] [bloom] [CPU2] bloom: connected to /dev/display/card0 on try 0
[35450718270] [[32mINFO [0m] [bloom] [CPU2] bloom: output0 1920x1080 @ 60000mHz
[35452144530] [[32mINFO [0m] [bloom] [CPU2] bloom: creating service port...
[35524919430] [[32mINFO [0m] [virtio_sound] [CPU0] SND: rpc Lookup 'out0'
[35548936962] [[32mINFO [0m] [virtio_sound] [CPU0] SND: rpc 6
[35577674154] [[32mINFO [0m] [sprout::pipelines] [CPU3] SPROUT: Audio stack launched chime (PID=14)
[35588586066] [[32mINFO [0m] [chime] [CPU0] chime: Waiting for /dev/audio/card0/out0 ...
[35603920473] [[32mINFO [0m] [virtio_sound] [CPU0] SND: rpc Lookup 'out0'
[35636490615] [[32mINFO [0m] [chime] [CPU0] chime: Opened /dev/audio/card0/out0 (fd=8)
[35655131886] [[32mINFO [0m] [virtio_sound] [CPU0] SND: rpc DeviceCall handle=2 op=32769
[35656211679] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[35739475332] [[32mINFO [0m] [virtio_sound] [CPU0] SND: rpc DeviceCall handle=2 op=32770
[35766950241] [[32mINFO [0m] [chime] [CPU0] chime: Configured stream (rate=44100Hz, fmt=1, ch=2)
[35779334547] [[32mINFO [0m] [virtio_sound] [CPU0] SND: rpc DeviceCall handle=2 op=32773
[35792630907] [[32mINFO [0m] [virtio_sound] [CPU0] SND: control SET_PARAMS begin for stream 0
[35797523520] [[32mINFO [0m] [virtio_sound] [CPU0] SND: control SET_PARAMS complete for stream 0
[35800086927] [[32mINFO [0m] [virtio_sound] [CPU0] SND: control PREPARE begin for stream 0
[35815943922] [[32mINFO [0m] [virtio_sound] [CPU0] SND: control PREPARE complete for stream 0
[35818790139] [[32mINFO [0m] [virtio_sound] [CPU0] SND: control START begin for stream 0
[35821261476] [[32mINFO [0m] [virtio_sound] [CPU0] SND: control START complete for stream 0
[35823755847] [[32mINFO [0m] [virtio_sound] [CPU0] SND: Hardware playback started
[35849101431] [[32mINFO [0m] [chime] [CPU0] chime: Generating classic chime...
[35888185674] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[35899663173] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pistil background renderer loaded from /lib/libpistil.so
[35908205850] [[32mINFO [0m] [bloom] [CPU2] bloom: initial wallpaper configured /share/wallpapers/flower.png
[36111828159] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[36350068227] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[36561696468] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[36780489735] [[32mINFO [0m] [bloom::display] [CPU2] bloom: imported buffer 1920x1080 as ID=1
[36915599820] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[37069591911] [[32mINFO [0m] [bloom::display] [CPU2] bloom: imported buffer 96x96 as ID=2
[37070937255] [[32mINFO [0m] [bloom::render] [CPU2] bloom: cursor ready buffer=2 size=96x96 hotspot=9,6
[37081971762] [[32mINFO [0m] [bloom] [CPU2] bloom: watching wallpaper config /session/desktop/wallpaper
[37112512899] [[32mINFO [0m] [bloom] [CPU2] bloom: Wayland server thread spawned
[37115324268] [[32mINFO [0m] [bloom::loop_types] [CPU2] bloom: service loop started
[37116538635] [[32mINFO [0m] [bloom::loop_types] [CPU2] bloom: output0 1920x1080 @ 60000mHz ready
[37116735678] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: starting
[37141025460] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: listening on /run/wayland-0
[37213993806] [[32mINFO [0m] [bloom::display] [CPU2] bloom: imported buffer 460x144 as ID=3
[37215208107] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pointer debug overlay ready buffer=3 size=460x144
[37220225460] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[37785144045] [
```
</details>
