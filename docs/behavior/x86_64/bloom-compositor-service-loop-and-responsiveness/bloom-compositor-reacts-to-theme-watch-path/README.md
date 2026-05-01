# ❌ Scenario: bloom compositor reacts to theme watch path

> Last run: 2026-05-01 12:08:41

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 11256ms | - [📜](./01/serial.log) - |
| 2 | Then the serial output should contain "bloom: service loop started" within 60s | ✅ | 1128ms | - [📜](./02/serial.log) - |
| 3 | When I wait for the shell prompt | ✅ | 149ms | - [📜](./03/serial.log) - |
| 4 | And I type "echo Solarized Warm > /session/desktop/theme" on the serial console | ✅ | 728ms | - [📜](./04/serial.log) - |
| 5 | Then the serial output should contain "bloom: reacting to theme change: Solarized Warm -> Solarized Warm" within 60s | ❌ | 61697ms | - [📜](./05/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[35219652765] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[35478980889] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[35499459864] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[35530925958] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[35563411884] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[35583183933] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[35584920558] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[35623767135] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[35625582003] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[35645768697] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[35660333346] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[35661063867] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[35696796267] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[35697952290] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[35786028729] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[35869195626] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[35890852503] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[35967003501] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[36071705274] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[36099705180] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[36123418287] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[36223468710] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[36275775096] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[36411645237] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[36676704669] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=958515756 elapsed_us=479257
[36677540262] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[36759130452] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[36764287494] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[36804293757] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[36847356183] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[36856397094] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[36860353431] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[36960466323] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[36964031742] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[36966338079] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[37142217849] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[37143948006] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[37177251408] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[37179628761] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[37179974403] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[37181723304] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[37206549006] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[37257418869] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[37261691841] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[37263883371] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[37272376845] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[37277016711] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[37278745350] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[37286477052] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[37290498267] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[37293351909] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[37299164727] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[37303059519] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[37307498019] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[37316196324] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[37367780307] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[37372770072] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[37470237717] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[37481050860] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[37491581622] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: cursor queue (queue 1) configured
[37493967192] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[37494966333] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: composition paths: gpu_opaque_copy=enabled cpu_alpha_blend=enabled
[37505009388] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: host scanout 1280x800 enabled=true (bootfb was 1920x1080)
[37529395266] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[37721131833] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[37762049688] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor DMA buffer ready (16 pages at phys=0x2723000)
[37835450796] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[37845078018] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[37850597565] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[37852999305] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[37898474691] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[38077360959] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[38274311625] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[38279347128] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[38382954093] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[38384112690] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[38537801115] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[38631687204] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[38658610683] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1280x800 @ 60000mHz
[38660035491] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[38660901345] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[38661809373] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports GPU blit (hardware transfer/flush)
[38662694235] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports direct scanout (zero-copy path to display)
[38663543292] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports partial flush (damage regions)
[38664193128] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports resource cache (pre-allocated buffer pool)
[38664917709] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[38745933369] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hwrng' name='hwrng' class=Other kind='dev.rng.HwRng' start='_RNvCs7wlZbVUrQWf_5hwrng20thingos_driver_start'
[39147200895] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[39148818753] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[39150383910] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[39159076671] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Facet Frame
[39503047254] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[39673805886] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[39841373715] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[40006356291] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[40147116174] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[40306607913] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[40404502611] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=48x48 hotspot=11,6
[40414845834] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[40416998259] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[40466371077] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[40468528782] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[40469429088] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1280x800 @ 60000mHz ready
[40470527889] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: starting
[40490199486] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: listening on /run/wayland-0
[40491267300] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[40556977362] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[40644560022] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=12)
[40691854533] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: connected to /run/wayland-0
[40708209828] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=13)
[40763320059] [[32mINFO [0m] [clock] [CPU1] clock: running at low scheduler priority tid=13
[40768096017] [[32mINFO [0m] [clock] [CPU1] clock: connected to /run/wayland-0
[40846558863] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=1 rect=1280x800 src_px=0xff0b0a10 dst_px=0xff0b0a10 damage=1280x800+0,0 res_id=1 gpu_planes=0 cpu_planes=2
[40848071451] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 638,399 src_px=0x10202020 dst_before=0xff0b0a10 dst_after=0xff0c0b11
[41041760034] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[41110137915] [[32mINFO [0m] [clock] [CPU1] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[41111413035] [[32mINFO [0m] [clock] [CPU1] clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf
[41121951189] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[41187008808] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[41224350486] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[41226959994] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: bound wp_presentation
[41234479902] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: wl_region smoke test: create+add+set_opaque+destroy
[41238574014] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
ec[41257294749] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
h[41328597783] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=28 frame=6
o[41373296811] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[41383855326] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/xhci' name='xhci' class=Block kind='dev.usb.Xhci' start='thingos_driver_start_safe'
[41386400880] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 15 driver(s) found
 [41450176251] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Starting AHCI VFS driver
[41463867192] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: imported dmabuf wl_buffer=111 480x320 format=0x34325241
[41499361695] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[41517066030] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Mounted atapi2 at /dev/storage/atapi2
[41518310988] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Provider loop online at /dev/storage/atapi2
[41584627458] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: bristle pid=8
[41593390212] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: claimed /sys/devices/isa-0070 (handle=2)
[41625481062] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: bristle pid=8
[41630421294] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: 2026-05-01 19:10:44 = 1777662644 unix_secs
[41631826071] [[32mINFO [0m] [kernel::time] [CPU3] System clock anchored: unix_secs=1777662644, mono_ns=20815735924, offset=1777662623184264076ns
[41632851381] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: System clock anchored to 1777662644 unix_secs
Sola[41649034317] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: sample rate set to 60 Hz
[41653124601] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:10:44.009607142 unix_secs=1777662644.009607142
rized W[42045108534] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=28 frame=6
[42053888778] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[42054716583] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
arm > /session/desktop/theme
[?25l[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[46279638603] [[32mINFO [0m] [bloom::services::theme_service] [CPU1] bloom: reacting to theme change: Solarized Warm -> Facet Frame
[46304408832] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=3 height=28 frame=6
[46315777464] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_popup obj=23 assigned to xdg_surface=21
[46364171997] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: imported dmabuf wl_buffer=121 160x96 format=0x34325241
[46893258819] [[32mINFO [0m] [bloom::render] [CPU1] bloom: flat window overlays ready size=1280x800
[47663764434] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:10:47.016055147 unix_secs=1777662647.016055147
[49055883096] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: frame callback done object=1000 data=24514
[49059204546] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: wp_presentation_feedback.presented object=1001 tv=24.518626275 refresh_ns=16666666 seq=0
[49068699669] [[32mINFO [0m] [bloom::services::theme_service] [CPU1] bloom: reacting to theme change: Solarized Warm -> Facet Frame
[49859738148] [[32mINFO [0m] [bloom::services::theme_service] [CPU1] bloom: reacting to theme change: Solarized Warm -> Facet Frame
[49863139590] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: frame callback done object=1002 data=24920
[49864110351] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: wp_presentation_feedback.presented object=1003 tv=24.924309877 refresh_ns=16666666 seq=1
[49867783977] [[32mINFO [0m] [bloom::services::theme_service] [CPU1] bloom: reacting to theme change: Solarized Warm -> Facet Frame
[53677042518] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:10:50.022745141 unix_secs=1777662650.022745141
[59690544243] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:10:53.029495772 unix_secs=1777662653.029495772
[65704103256] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:10:56.036271847 unix_secs=1777662656.036271847
[71717633394] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:10:59.043022924 unix_secs=1777662659.043022924
[74370958728] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 11:11:00 UTC-8 system_unix=1777662660.368659835
[77731170066] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:11:02.049809146 unix_secs=1777662662.049809146
[83744589192] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:11:05.056515508 unix_secs=1777662665.056515508
[89758183647] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:11:08.063283398 unix_secs=1777662668.063283398
[95771579706] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:11:11.070014477 unix_secs=1777662671.070014477
[101785175382] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:11:14.076786377 unix_secs=1777662674.076786377
[107798661630] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:11:17.083538956 unix_secs=1777662677.083538956
[113812052013] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:11:20.090249575 unix_secs=1777662680.090249575
[119825618550] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:11:23.097025369 unix_secs=1777662683.097025369
[125839415889] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:11:26.103844360 unix_secs=1777662686.103844360
[131852883822] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:11:29.110600172 unix_secs=1777662689.110600172
[137866154646] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:11:32.117302459 unix_secs=1777662692.117302459
[143879504373] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:11:35.123977306 unix_secs=1777662695.123977306
[148710732423] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 11:11:37 UTC-8 system_unix=1777662697.539386186
[149892998475] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:11:38.130725990 unix_secs=1777662698.130725990
[155906613753] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:11:41.137527953 unix_secs=1777662701.137527953
[161920105677] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:11:44.144259676 unix_secs=1777662704.144259676
[167933601528] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:11:47.151010093 unix_secs=1777662707.151010093
[173947227531] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:11:50.157819052 unix_secs=1777662710.157819052
[179960744766] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:11:53.164573990 unix_secs=1777662713.164573990
[185974133631] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:11:56.171275088 unix_secs=1777662716.171275088
[191987861175] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:11:59.178105992 unix_secs=1777662719.178105992
[198001068177] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:12:02.184745958 unix_secs=1777662722.184745958
[204014600229] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:12:05.191506968 unix_secs=1777662725.191506968
[210028007343] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:12:08.198226316 unix_secs=1777662728.198226316
[216041657766] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:12:11.205029318 unix_secs=1777662731.205029318
[222054969807] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:12:14.211710270 unix_secs=1777662734.211710270
[223052481135] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 11:12:14 UTC-8 system_unix=1777662734.710159925
[228068479683] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:12:17.218466132 unix_secs=1777662737.218466132
[234082159344] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:12:20.225287697 unix_secs=1777662740.225287697
[240095660013] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:12:23.232033049 unix_secs=1777662743.232033049
[246109387326] [
```
</details>
