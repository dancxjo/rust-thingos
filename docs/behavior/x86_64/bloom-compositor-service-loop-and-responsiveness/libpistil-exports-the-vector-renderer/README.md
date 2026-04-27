# ✅ Scenario: libpistil exports the vector renderer

> Last run: 2026-04-27 09:53:39

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8529ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 252ms | - [📜](./02/serial.log) - |
| 3 | And I type "test_dlopen" on the serial console | ✅ | 10242ms | - [📜](./03/serial.log) - |
| 4 | Then the serial output should contain "[test_dlopen] pistil_draw_vector_smoke: PASS" within 60s | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26273838624] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[26531220342] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[26551030341] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26582834190] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[26614123866] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[26633273469] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26634891756] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26670262311] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[26671712562] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26691383268] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26705309103] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[26705825355] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[26741063844] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26742042921] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[26824544835] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26905113048] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[26926232058] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[27007176603] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[27105393183] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[27131328642] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[27156067884] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[27267865383] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[27316628262] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[27465207099] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[27744190287] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=982429734 elapsed_us=491214
[27744863190] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27816515001] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[27819533511] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[27855108402] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[27894691242] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[27904456041] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[27905897184] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[27999454560] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[28003775943] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[28011095145] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28198016319] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[28199556693] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[28234578768] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[28239412806] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[28264300614] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[28271972784] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[28274190351] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[28275499065] [[32mINFO [0m] [iso9660d] [CPU2] ISO9660D: Starting VFS provider
[28275640173] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[28305049245] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=9)
[28309723926] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[28311968157] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[28318799520] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[28323126876] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[28324702362] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[28327397241] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[28329467925] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[28333999221] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[28338727197] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[28356875316] [[32mINFO [0m] [bristle] [CPU3] bristle: published pid 9 to /run/bristle/pid
[28370172270] [[32mINFO [0m] [bristle] [CPU3] bristle: published device handles kbd_in=3 mouse_in=5
[28381445664] [[32mINFO [0m] [bristle] [CPU3] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[28389100773] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[28482936141] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[28576128570] [[32mINFO [0m] [virtio_gpu] [CPU1] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[28585963791] [[32mINFO [0m] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[28592409351] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
test_d[29177601453] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
lope[29340317391] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
n
[29469313632] [[32mINFO [0m] [user.print] [CPU2] --- test_dlopen starting ---
[29470707057] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlopen_nonexistent: starting
[?25l[29475901950] [[32mINFO [0m] [user.print] [CPU2]   dlerror: dlopen: cannot read library file
[29476873899] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlopen_nonexistent: PASS
[29477936829] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlopen_null_path: starting
[29479363749] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlopen_null_path: PASS
[29480371239] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlerror_cleared_after_read: starting
[29489721888] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlerror_cleared_after_read: PASS
[29490796665] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlsym_unknown_symbol: starting
[29492315259] [[32mINFO [0m] [user.print] [CPU2]   dlerror: dlsym: symbol not found
[29493409638] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlsym_unknown_symbol: PASS
[29494543650] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlsym_invalid_handle: starting
[29495767818] [[32mINFO [0m] [user.print] [CPU2]   dlerror: dlsym: invalid handle
[29496732705] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlsym_invalid_handle: PASS
[29497732176] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlclose_rtld_default: starting
[29498939580] [[32mINFO [0m] [user.print] [CPU2]   dlerror: dlclose: cannot close RTLD_DEFAULT
[29499907107] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlclose_rtld_default: PASS
[29500890144] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlclose_invalid_handle: starting
[29502057585] [[32mINFO [0m] [user.print] [CPU2]   dlerror: dlclose: invalid handle
[29502986403] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlclose_invalid_handle: PASS
[29503954194] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlclose_double_close: starting
[29505015474] [[32mINFO [0m] [user.print] [CPU2]   dlerror: dlclose: cannot close RTLD_DEFAULT
[29505995640] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlclose_double_close: PASS
[29507002371] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] test_dlopen_pistil_shared_library: starting
[29527933413] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[29535302280] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[29585703543] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=12)
[29588059116] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Starting early audio stack
[29591589588] [[32mINFO [0m] [bloom] [CPU3] bloom: ENTERING MAIN
[29592424818] [[32mINFO [0m] [bloom] [CPU3] bloom: compositor service starting
[29619573093] [[32mINFO [0m] [sprout::pipelines] [CPU1] SPROUT: Audio stack worker running
[29642142354] [[32mINFO [0m] [sprout::pipelines] [CPU1] SPROUT: Early audio device '/sys/devices/pci-0000:00:03.0' matched dev.sound.Virtio; spawning /drivers/virtio_sound
[29675169942] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[29679274977] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: VFS provider loop online
[29801800512] [[32mINFO [0m] [bloom] [CPU3] bloom: connected to /dev/display/card0 on try 0
[29802555486] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[29823375186] [[32mINFO [0m] [bloom] [CPU3] bloom: output0 1920x1080 @ 60000mHz
[29824178538] [[32mINFO [0m] [bloom] [CPU3] bloom: creating service port...
[29834123385] [[32mINFO [0m] [user.print] [CPU2] [test_dlopen] pistil_draw_vector_smoke: PASS
[30005055531] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[30102327717] [[32mINFO [0m] [bloom::render] [CPU3] bloom: pistil background renderer loaded from /lib/libpistil.so
[30108928971] [[32mINFO [0m] [bloom] [CPU3] bloom: initial wallpaper configured /share/wallpapers/flower.png
[30263601720] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[30447075780] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[30647660769] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[30835184193] [[32mINFO [0m] [cambium::c
```
</details>
