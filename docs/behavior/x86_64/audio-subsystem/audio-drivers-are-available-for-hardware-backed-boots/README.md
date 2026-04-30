# ✅ Scenario: Audio drivers are available for hardware-backed boots

> Last run: 2026-04-30 01:55:04

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 0ms | - - - |
| 2 | When I wait for the serial output to contain "DEVD CATALOG: registered driver '/drivers/virtio_sound'" | ✅ | 10042ms | - [📜](./02/serial.log) - |
| 3 | Then the serial output should contain "DEVD CATALOG: registered driver '/drivers/chime'" | ✅ | 0ms | - - - |
| 4 | And the serial output should contain "DEVD CATALOG: registered driver '/drivers/hdaudio'" | ✅ | 0ms | - - - |
| 5 | And the serial output should not contain "KERNEL PAGE FAULT" | ✅ | 300ms | - [📜](./05/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26779717866] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[27036325536] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[27056764449] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[27090250440] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[27124601031] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[27145703013] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[27147518772] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[27185807154] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[27187481310] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[27207459477] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[27221820582] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[27222351783] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[27258467280] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[27259446885] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[27344259525] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[27424779756] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[27446978196] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[27523172061] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[27627646299] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[27654027819] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[27676862994] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[27791235780] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[27844980339] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[27996360546] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[28292352924] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1012420794 elapsed_us=506210
[28293338271] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[28376110752] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[28379786688] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[28460497626] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[28508731053] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[28518641547] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[28520146116] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[28621607652] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[28624877688] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[28630913883] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28792072848] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[28794730965] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[28832380104] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[28842570339] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[28875825693] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[28876241097] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[28880822718] [[32mINFO [0m] [iso9660d] [CPU2] ISO9660D: Starting VFS provider
[28885496871] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[28887809808] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[28934259948] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=9)
[28939578624] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[28942147443] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[28955867853] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[28961169468] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[28963649781] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[28966807155] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[28970335911] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[28977303729] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[28986621741] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[29010330591] [[32mINFO [0m] [bristle] [CPU3] bristle: published pid 9 to /run/bristle/pid
[29030516460] [[32mINFO [0m] [bristle] [CPU3] bristle: published device handles kbd_in=3 mouse_in=5
[29048798196] [[32mINFO [0m] [bristle] [CPU3] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[29051378763] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[29229600972] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[29407234296] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[29576671377] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[29747190924] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[29771081439] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[29877931842] [[32mINFO [0m] [virtio_gpu] [CPU1] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[29888919786] [[32mINFO [0m] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[29895926742] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[30835228941] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[30905788353] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=11)
[30911730729] [[32mINFO [0m] [bloom] [CPU2] bloom: ENTERING MAIN
[30912821346] [[32mINFO [0m] [bloom] [CPU2] bloom: compositor service starting
[30915075411] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Starting early audio stack
[30944331660] [[32mINFO [0m] [sprout::pipelines] [CPU3] SPROUT: Audio stack worker running
[30982769235] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[31036537158] [[32mINFO [0m] [sprout::pipelines] [CPU3] SPROUT: Early audio device '/sys/devices/pci-0000:00:03.0' matched dev.sound.Virtio; spawning /drivers/virtio_sound
[31218468699] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[31223573700] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: VFS provider loop online
[31256931519] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[31382839983] [[32mINFO [0m] [bloom] [CPU2] bloom: connected to /dev/display/card0 on try 0
[31409904702] [[32mINFO [0m] [bloom] [CPU2] bloom: output0 1920x1080 @ 60000mHz
[31411164147] [[32mINFO [0m] [bloom] [CPU2] bloom: creating service port...
[31432348893] [[32mINFO [0m] [virtio_sound] [CPU0] SND: rpc Lookup 'out0'
[31449700161] [[32mINFO [0m] [virtio_sound] [CPU0] SND: rpc 6
[31466874054] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[31486523409] [[32mINFO [0m] [sprout::pipelines] [CPU3] SPROUT: Audio stack launched chime (PID=14)
[31497313419] [[32mINFO [0m] [chime] [CPU0] chime: Waiting for /dev/audio/card0/out0 ...
[31513511799] [[32mINFO [0m] [virtio_sound] [CPU0] SND: rpc Lookup 'out0'
[31540308162] [[32mINFO [0m] [chime] [CPU0] chime: Opened /dev/audio/card0/out0 (fd=8)
[31555104438] [[32mINFO [0m] [virtio_sound] [CPU0] SND: rpc DeviceCall handle=2 op=32769
[31588120938] [[32mINFO [0m] [virtio_sound] [CPU0] SND: rpc DeviceCall handle=2 op=32770
[31607768676] [[32mINFO [0m] [chime] [CPU0] chime: Configured stream (rate=44100Hz, fmt=1, ch=2)
[31618360422] [[32mINFO [0m] [virtio_sound] [CPU0] SND: rpc DeviceCall handle=2 op=32773
[31629656652] [[32mINFO [0m] [virtio_sound] [CPU0] SND: control SET_PARAMS begin for stream 0
[31632723573] [[32mINFO [0m] [virtio_sound] [CPU0] SND: control SET_PARAMS complete for stream 0
[31634436009] [[32mINFO [0m] [virtio_sound] [CPU0] SND: control PREPARE begin for stream 0
[31642879455] [[32mINFO [0m] [chime] [CPU0] chime: Generating classic chime...
[31673278890] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[31783734411] [[32mINFO [0m] [virtio_sound] [CPU0] SND: control PREPARE complete for stream 0
[31786839447] [[32mINFO [0m] [virtio_sound] [CPU0] SND: control START begin for stream 0
[31790114928] [[32mINFO [0m] [virtio_sound] [CPU0] SND: control START complete for stream 0
[31793029785] [[32mINFO [0m] [virtio_sound] [CPU0] SND: Hardware playback started
[31804506261] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pistil background renderer loaded from /lib/libpistil.so
[31817311911] [[32mINFO [0m] [bloom] [CPU2] bloom: initial wallpaper configured /share/wallpapers/flower.png
[31889509179] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[32121750606] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[32326021398] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[32601916776] [[32mINFO [0m] [bloom::display] [CPU2] bloom: imported buffer 1920x1080 as ID=1
[32607492951] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[32818121853] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[32819962065] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[32897640666] [[32mINFO [0m] [bloom::display] [CPU2] bloom: imported buffer 96x96 as ID=2
[32899405176] [[32mINFO [0m] [bloom::render] [CPU2] bloom: cursor ready buffer=2 size=96x96 hotspot=9,6
[32910223005] [[32mINFO [0m] [bloom] [CPU2] bloom: watching wallpaper config /session/desktop/wallpaper
[32940290328] [[32mINFO [0m] [bloom] [CPU2] bloom: Wayland server thread spawned
[32942551983] [[32mINFO [0m] [bloom::loop_types] [CPU2] bloom: service loop started
[32943614649] [[32mINFO [0m] [bloom::loop_types] [CPU2] bloom: output0 1920x1080 @ 60000mHz ready
[32945247984] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: starting
[32960833125] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: listening on /run/wayland-0
[32998950798] [[32mINFO [0m] [bloom::display] [CPU2] bloom: imported buffer 460x144 as ID=3
[33000574728] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pointer debug overlay ready buffer=3 size=460x144
[33835438857] [
```
</details>
