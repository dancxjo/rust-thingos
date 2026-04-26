# ❌ Scenario: Boot reaches an interactive serial shell before background bring-up continues

> Last run: 2026-04-25 20:10:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ❌ | 31013ms | - [📜](./01/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01Hot_progress: milestone="Framebuffer Initialized"
[53668547724] [53054413302] [[32mINFO [0m] [kernel::boot_progress] [CPU0] bo[[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[53712706641] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[53783257176] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[53850821871] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[53891143053] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[53896679892] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[53969721861] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[53973389118] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[54015131643] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[54046997169] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[54048542823] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[54123563274] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[54125876904] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[54304748289] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[54467383113] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[54514926048] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[54638171313] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[54811833945] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[54894565011] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[54950388141] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[55205091579] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[55338684654] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[55641265251] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[56231349603] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=2062896693 elapsed_us=1031448er loop.

[56232925485] [[32mINFO [0m] [kernel] [CPU0] Entering schedul[56577579366] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[56588926878] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[56686065579] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[56790900210] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[56831734707] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[56833724805] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[57065636397] [[32mINFO [0m] [kernel::syscall::handlers::proce
ss] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6[57074448849] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[57080748087] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[57091676433] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[57330728268] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[57334221252] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[57414458145] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[57419334819] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[57492529248] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[57Starting full pipeline (graphics + input)
[57496993719] [[32mI500364174] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: NFO [0m] [iso9660d] [CPU2] ISO9660D: Starting VFS provider
[57495656823] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[57591042003] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[57641267838] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[57717362307] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[57771676644] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=10)
[57775187250] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[58097428323] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[58156165221] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[58182675111] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[58209474246] [[32mINFO [0m] [bristle] [CPU1] bristle: published pid 10 to /run/bristle/pid
[58247070651] [[32mINFO [0m] [bristle] [CPU1] bristle: published device handles kbd_in=7 mouse_in=9
[58262584941] [[32mINFO [0m] [bristle] [CPU1] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[58442487015] [[32mINFO [0m] [cambium::catalog] [CPU2] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[58796257641] [[32mINFO [0m] [cambium::catalog] [CPU2] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[59105615118] [[32mINFO [0m] [cambium::catalog] [CPU2] DEVD CAio kind='dev.sound.Chime' start='thingos_driver_start_safe'
TALOG: registered driver '/drivers/chime' name='chime' class=Aud[59426406963] [[32mINFO [0m] [cambium::catalog] [CPU2] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[59756609946] [[32mINFO [0m] [cambium::catalog] [CPU2] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[60085156362] [[32mINFO [0m] [cambium::catalog] [CPU2] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[60396509844] [[32mINFO [0m] [cambium::catalog] [CPU2] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[60608373177] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[60685486389] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[60693611946] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[60782528400] [[32mINFO [0m] [cambium::catalog] [CPU2] DEVD CATALOG: rend='drv.Ps2Keyboard' start='thingos_driver_start_safe'
gistered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input ki[60842137587] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=11)
[61170679911] [[32mINFO [0m] [cambium::catalog] [CPU2] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[61512560802tered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kin] [[32mINFO [0m] [cambium::catalog] [CPU2] DEVD CATALOG: regisd='dev.rtc.Cmos' start='thingos_driver_start_safe'
[61572472269] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[61574542788] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[61835309415] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[61918180467] [[32mINFO [0m] [cambium::catalog] [CPU2] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[61990723542] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1920x1080 @ 60000mHz
[61992600153] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[62111409327] [[33mWARN [0m] [bloom::render] [CPU1] bloom: failed to find pistil_prepare_background
[62114307453] [[32mINFO [0m] [bloom] [CPU1] bloom: preparing background /session/desktop/wallpaper
[62314989462] [[32mINFO [0m] [cambium::catalog] [CPU2] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[62650962660] [[32mINFO [0m] [cambium::catalog] [CPU2] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[62658149235] [[32mINFO [0m] [cambium::catalog] [CPU2] DEVD CATALOG: 13 driver(s) found
[63165482160] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU2] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=12 PID=12
[63260888163] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU2] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=13 PID=13
[63650079042] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Starting AHCI VFS driver
[63863104734] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[63927577494] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Mounted atapi2 at /dev/storage/atapi2
[63973313745] [[32mINFO [0m] [bloom::render] [CPU2] bloom: using periwinkle fallback background
[64119185328] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[65870892516] [[32mINFO [0m] [bloom::display] [CPU2] bloom: imported buffer 1920x1080 as ID=0
[65874108795] [[32mINFO [0m] [bloom] [CPU2] bloom: background buffer_id=Some(0)
[65908006758] [[32mINFO [0m] [bloom] [CPU2] bloom: registered with bristle (pid=10)
[65919981039] [[32mINFO [0m] [bristle] [CPU1] bristle: bloom sink registered (fd=7)
[66026395017] [[32mINFO [0m] [bloom] [CPU2] bloom: Wayland server thread spawned
[66029009013] [[32mINFO [0m] [bloom::loop_types] [CPU2] bloom: service loop started
[66037858821] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: starting
[66137663559] [[32mINFO [0m] [bloom::wayland] [CPU1] wayland-server: listening on /run/wayland-0
[66283375092] [[32mINFO [0m] [bloom::loop_types] [CPU2] First frame rendered
[71124179445] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[71224493340] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[71525108391] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/netd' TID=17 PID=17
[71545918917] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Tracking netd activation (PID=17)
[73377037998] [[32mINFO [0m] [netd] [
```
</details>
