# ✅ Scenario: boot framebuffer driver paints the first compositor commit

> Last run: 2026-04-30 01:34:56

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8717ms | - [📜](./01/serial.log) - |
| 2 | Then the serial output should contain "display_bootfb: imported buffer" within 60s | ✅ | 817ms | - [📜](./02/serial.log) - |
| 3 | And the serial output should contain "First frame rendered" within 60s | ✅ | 1431ms | - [📜](./03/serial.log) - |
| 4 | And the bloom first frame should contain visible pixels | ✅ | 1511ms | - [📜](./04/serial.log) - |
| 5 | And the serial output should not contain "task='display_bootfb'" | ✅ | 301ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26878271937] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[27135301677] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[27156521238] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[27191986536] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[27226054812] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[27247225599] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[27248959221] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[27287305980] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[27288914961] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[27309916029] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[27324913968] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[27325558161] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[27361091373] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[27362120379] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[27447827484] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[27530933199] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[27553643667] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[27635555640] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[27736974771] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[27765416844] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[27792443250] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[27915677064] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[28020184896] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[28186611684] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[28484365701] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1101210627 elapsed_us=550605
[28485102195] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[28573682181] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[28577904333] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[28621646988] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[28663972656] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[28675939875] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[28677779229] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[28778244594] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[28785771267] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[28789516734] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28974276474] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[28976192883] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[29016023025] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[29021495184] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[29050010022] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[29065652154] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[29068273311] [[32mINFO [0m] [iso9660d] [CPU2] ISO9660D: Starting VFS provider
[29096959452] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[29099356440] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[29159059083] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=9)
[29163858306] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[29166164346] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[29178439752] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[29183335599] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_bootfb'
[29185705230] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[29190101556] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[29193153429] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[29199346341] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[29205221331] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_bootfb' (boot_fd=3, bind_id=322371585)
[29233434516] [[32mINFO [0m] [bristle] [CPU3] bristle: published pid 9 to /run/bristle/pid
[29241522717] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=bootfb)
[29248537131] [[32mINFO [0m] [bristle] [CPU3] bristle: published device handles kbd_in=3 mouse_in=5
[29261517780] [[32mINFO [0m] [bristle] [CPU3] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[29294404854] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: Starting VFS-native bootfb driver (v0.4.1) TID=10 PID=10
[29295544443] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: boot_arg=3
[29296630407] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: Mapping bootstrap memfd 3 size=4096...
[29298981822] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: vm_map success at 0x400000001000
[29299991688] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: Recovered handles: req_read=4, resp_write=5, svc=1, id=322371585
[29301982446] [[32mINFO [0m] [display_bootfb::driver] [CPU1] display_bootfb: probing /dev/fb0...
[29685193065] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: Sending MSG_BIND_READY handshake (class_mask=0x3) to supervisor port...
[29697213447] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: Sent MSG_BIND_READY (result=Ok(())), waiting for MSG_BIND_ASSIGNED...
[29783865144] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[29837384280] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: entering VFS provider service loop
[29878279398] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=11)
[29885314503] [[32mINFO [0m] [bloom] [CPU2] bloom: ENTERING MAIN
[29886577413] [[32mINFO [0m] [bloom] [CPU2] bloom: compositor service starting
[29895225525] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Starting early audio stack
[29914698099] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[29925162366] [[32mINFO [0m] [sprout::pipelines] [CPU3] SPROUT: Audio stack worker running
[29973395166] [[32mINFO [0m] [sprout::pipelines] [CPU3] SPROUT: Early audio device '/sys/devices/pci-0000:00:03.0' matched dev.sound.Virtio; spawning /drivers/virtio_sound
[30039135786] [[32mINFO [0m] [bloom] [CPU2] bloom: connected to /dev/display/card0 on try 0
[30060690627] [[32mINFO [0m] [bloom] [CPU2] bloom: output0 1920x1080 @ 60000mHz
[30061697787] [[32mINFO [0m] [bloom] [CPU2] bloom: creating service port...
[30114293682] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[30289519788] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[30409001997] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pistil background renderer loaded from /lib/libpistil.so
[30416991462] [[32mINFO [0m] [bloom] [CPU2] bloom: initial wallpaper configured /share/wallpapers/flower.png
[30478529928] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[30652275654] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[30833050908] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[31071099939] [[32mINFO [0m] [display_bootfb::driver] [CPU1] display_bootfb: imported buffer 1 (1920x1080 @ 0x4000011d5000)
[31079120985] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[31079734653] [[32mINFO [0m] [bloom::display] [CPU2] bloom: imported buffer 1920x1080 as ID=1
[31253773716] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[31347200775] [[32mINFO [0m] [display_bootfb::driver] [CPU1] display_bootfb: imported buffer 2 (96x96 @ 0x4000019e7000)
[31355781468] [[32mINFO [0m] [bloom::display] [CPU2] bloom: imported buffer 96x96 as ID=2
[31356822948] [[32mINFO [0m] [bloom::render] [CPU2] bloom: cursor ready buffer=2 size=96x96 hotspot=9,6
[31367509272] [[32mINFO [0m] [bloom] [CPU2] bloom: watching wallpaper config /session/desktop/wallpaper
[31397348697] [[32mINFO [0m] [bloom] [CPU2] bloom: Wayland server thread spawned
[31402932792] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: starting
[31414338252] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: listening on /run/wayland-0
[31420786122] [[32mINFO [0m] [bloom::loop_types] [CPU2] bloom: service loop started
[31421706921] [[32mINFO [0m] [bloom::loop_types] [CPU2] bloom: output0 1920x1080 @ 60000mHz ready
[31431579168] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[31432852737] [[32mINFO [0m] [bloom::services::wallpaper] [CPU2] bloom: preparing background /share/wallpapers/flower.png
[31602294669] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[31792020645] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[31966275462] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[31968106896] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 12 driver(s) found
[32248125459] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Starting AHCI VFS driver
[32275256079] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[32291080041] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Mounted atapi2 at /dev/storage/atapi2
[32291897319] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Provider loop online at /dev/storage/atapi2
[33683349117] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: bristle pid=9
[33715380303] [[32mINFO [0m] [ps2_mouse] [CPU1] ps2_mouse: bristle pid=9
[35525289558] [[32mINFO [0m] [display_bootfb::driver] [CPU1] display_bootfb: imported buffer 3 (1920x1080 @ 0x40000339d000)
[35533852530] [[32mINFO [0m] [bloom::display] [CPU2] bloom: imported buffer 1920x1080 as ID=3
[35728538340] [[32mINFO [0m] [display_bootfb::driver] [CPU1] display_bootfb: imported buffer 4 (460x144 @ 0x400003bc7000)
[35733607206] [[32mINFO [0m] [bloom::display] [CPU2] bloom: imported buffer 460x144 as ID=4
[35734560609] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pointer debug overlay ready buffer=4 size=460x144
[35813533668] [[32mINFO [0m] [bloom::world] [CPU2] bloom: presented cursor buffer=2 at 951,534 size=96x96
[35815098099] [[32mINFO [0m] [bloom::loop_types] [CPU2] First frame rendered
[35822443701] [[32mINFO [0m] [bloom::services::input_service] [CPU2] bloom: registered bristle pointer sink
[35827480128] [[32mINFO [0m] [bristle] [CPU3] bristle: bloom sink registered (handle=7)
[40109012097] [[32mINFO [0m] [sprout::
```
</details>
