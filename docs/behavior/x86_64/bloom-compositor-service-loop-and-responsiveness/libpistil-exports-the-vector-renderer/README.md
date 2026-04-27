# ✅ Scenario: libpistil exports the vector renderer

> Last run: 2026-04-27 10:20:00

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8308ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 254ms | - [📜](./02/serial.log) - |
| 3 | And I type "test_dlopen" on the serial console | ✅ | 1437ms | - [📜](./03/serial.log) - |
| 4 | Then the serial output should contain "[test_dlopen] pistil_draw_vector_smoke: PASS" within 60s | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[25742043690] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[25991055783] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26010455130] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26041244856] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26072275680] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26091022353] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26092682715] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26127817881] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[26129340831] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26148612963] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26162159331] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[26162682282] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[26197718514] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26198640633] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26279835285] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26360646675] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26381740341] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[26459820585] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[26557190682] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[26582556495] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[26604319599] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[26713083375] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[26764181334] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[26911915371] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27187219026] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=968639727 elapsed_us=484319
[27187767717] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27258949212] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[27262997619] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27299071404] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[27337460733] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[27347123265] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27348546720] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27443006316] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[27448161147] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[27451162233] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[27628868388] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[27630516540] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[27662779353] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[27667112220] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[27689899083] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[27703848018] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[27706602792] [[32mINFO [0m] [iso9660d] [CPU2] ISO9660D: Starting VFS provider
[27708741885] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[27711076239] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[27741776634] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=9)
[27744934899] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[27746457486] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[27753181830] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[27756520374] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[27758421075] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[27760865814] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[27762838059] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[27767212539] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[27771796899] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[27798622071] [[32mINFO [0m] [bristle] [CPU3] bristle: published pid 9 to /run/bristle/pid
[27808764225] [[32mINFO [0m] [bristle] [CPU3] bristle: published device handles kbd_in=3 mouse_in=5
[27821401740] [[32mINFO [0m] [bristle] [CPU3] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[27832734402] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[27903707040] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[27994002894] [[32mINFO [0m] [virtio_gpu] [CPU1] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[28004515638] [[32mINFO [0m] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[28011153258] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
test_dlopen
[28770044391] [[32mINFO [0m] [user.print] [CPU2] --- test_dlopen starting ---
[?25l[28771059504] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlopen_nonexistent: starting
[28774808799] [[32mINFO [0m] [user.print] [CPU2]   dlerror: dlopen: cannot read library file
[28775582946] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlopen_nonexistent: PASS
[28776196383] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlopen_null_path: starting
[28790358630] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlopen_null_path: PASS
[28791074532] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlerror_cleared_after_read: starting
[28792860657] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlerror_cleared_after_read: PASS
[28793620746] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlsym_unknown_symbol: starting
[28794664767] [[32mINFO [0m] [user.print] [CPU2]   dlerror: dlsym: symbol not found
[28795256094] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlsym_unknown_symbol: PASS
[28795870587] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlsym_invalid_handle: starting
[28796730204] [[32mINFO [0m] [user.print] [CPU2]   dlerror: dlsym: invalid handle
[28797304470] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlsym_invalid_handle: PASS
[28797893817] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlclose_rtld_default: starting
[28798637241] [[32mINFO [0m] [user.print] [CPU2]   dlerror: dlclose: cannot close RTLD_DEFAULT
[28799232528] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlclose_rtld_default: PASS
[28799876622] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlclose_invalid_handle: starting
[28800632223] [[32mINFO [0m] [user.print] [CPU2]   dlerror: dlclose: invalid handle
[28801204740] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlclose_invalid_handle: PASS
[28801883253] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlclose_double_close: starting
[28803596448] [[32mINFO [0m] [user.print] [CPU2]   dlerror: dlclose: cannot close RTLD_DEFAULT
[28804231566] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlclose_double_close: PASS
[28804831737] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlopen_pistil_shared_library: starting
[28826571048] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[28885028403] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=12)
[28887349128] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Starting early audio stack
[28889765190] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[28891536696] [[32mINFO [0m] [bloom] [CPU3] bloom: ENTERING MAIN
[28892430666] [[32mINFO [0m] [bloom] [CPU3] bloom: compositor service starting
[28980040650] [[32mINFO [0m] [sprout::pipelines] [CPU1] SPROUT: Audio stack worker running
[29000930442] [[32mINFO [0m] [sprout::pipelines] [CPU1] SPROUT: Early audio device '/sys/devices/pci-0000:00:03.0' matched dev.sound.Virtio; spawning /drivers/virtio_sound
[29035612980] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[29039120517] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: VFS provider loop online
[29117841984] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] pistil_draw_vector_smoke: PASS
[29162910414] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[29163143031] [[32mINFO [0m] [bloom] [CPU3] bloom: connected to /dev/display/card0 on try 0
[29187028431] [[32mINFO [0m] [bloom] [CPU3] bloom: output0 1920x1080 @ 60000mHz
[29187804096] [[32mINFO [0m] [bloom] [CPU3] bloom: creating service port...
[29343321480] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[29456921142] [[32mINFO [0m] [bloom::render] [CPU3] bloom: pistil background renderer loaded from /lib/libpistil.so
[29463083727] [[32mINFO [0m] [bloom] [CPU3] bloom: initial wallpaper configured /share/wallpapers/flower.png
[29541286533] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[29733431046] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[29920837584] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[30104541720] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[30296222583] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[30476595270] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[30664696161] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[30842006415] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[31040270712] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[31223865189] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[31225521459] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[31488170076] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Starting AHCI VFS driver
[31515701349] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[31530564813] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Mounted atapi2 at /dev/storage/atapi2
[31531300416] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Provider loop online at /dev/storage/atapi2
[32061072813] [[32mINFO [0m] [pistil::font] [CPU2] pistil: opening font /share/fonts/NotoSans-Regular.ttf
[32064160029] [[32mINFO [0m] [pistil::font] [CPU2] pistil: stat font /share/fonts/NotoSans-Regular.ttf
[32065793760] [[32mINFO [0m] [pistil::font] [CPU2] pistil: reading font 569208 bytes from /share/fonts/NotoSans-Regular.ttf
[32199188637] [[32mINFO [0m] [pistil::font] [CPU2] pistil: font read returned 569208 of 569208 bytes
[32200449633] [[32mINFO [0m] [pistil::font] [CPU2] pistil: parsing font /share/fonts/NotoSans-Regular.ttf
[32720961108] [[32mINFO [0m] [
```
</details>
