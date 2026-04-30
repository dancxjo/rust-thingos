# ❌ Scenario: first valid commit is visible above the compositor background

> Last run: 2026-04-30 15:03:08

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Then the Wayland hello client should be visible | ❌ | 31170ms | - [📜](./01/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[32623654008] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[32898052671] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[32918255139] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[32951395092] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[32984147394] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[33003853443] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[33005743848] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[33042842151] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[33044419914] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[33064388445] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[33078894783] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[33079407603] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[33116158284] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[33117196002] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[33206340981] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[33288406899] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[33310664211] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[33385067694] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[33484663014] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[33511211712] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[33534684315] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[33640109151] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[33687612057] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[33828436719] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[34076584410] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=939147561 elapsed_us=469573
[34077467457] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[34159643067] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[34163108694] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[34232783805] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[34265467896] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[34277535006] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[34279978689] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[34376065515] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[34379922159] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[34387754313] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[34555409229] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[34557100083] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[34589027352] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[34591214394] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[34592741634] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[34594039260] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[34616179983] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[34626001179] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[34629102816] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[34630557885] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[34636743504] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[34640473758] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[34642229622] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[34657843044] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[34660528320] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[34664985399] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[34669400601] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[34679368119] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[34694610621] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[34705468149] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[34722373092] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[34725879804] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[34812723462] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[34824175122] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[34830060144] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[34897744596] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[35075522592] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[35239618161] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[35404973340] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[35405738808] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[35414931321] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[35420126775] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[35423137068] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[35579000622] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[35738525790] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[35820256395] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[35835945090] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[35836863711] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[36006512553] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[36167196219] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[36328863879] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[36486633447] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[36573871356] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[36596672277] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1920x1080 @ 60000mHz
[36597486123] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[36598221429] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[36599032833] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[36905620620] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[36906546963] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[36907302828] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[36915218010] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Solarized Warm
[37464424470] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[37628008077] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[37797705957] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[37799322396] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[37873024035] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Starting AHCI VFS driver
[37901371563] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[37918705902] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Mounted atapi2 at /dev/storage/atapi2
[37919527107] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Provider loop online at /dev/storage/atapi2
[37949643468] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: bristle pid=8
[37950593340] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: claimed /sys/devices/isa-0070 (handle=2)
[37970494551] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: 2026-04-30 22:03:38 = 1777586618 unix_secs
[37971871245] [[32mINFO [0m] [kernel::time] [CPU3] System clock anchored: unix_secs=1777586618, mono_ns=18985706305, offset=1777586599014293695ns
[37972713075] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: System clock anchored to 1777586618 unix_secs
[37986759327] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:03:38.006912692 unix_secs=1777586618.006912692
[38006314434] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: bristle pid=8
[38045043399] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: sample rate set to 60 Hz
[38051255088] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[38209293210] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=48x48 hotspot=24,24
[38220813345] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[38222992170] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[38256025566] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[38257787007] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[38258609961] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1920x1080 @ 60000mHz ready
[38261401794] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: starting
[38276800023] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: listening on /run/wayland-0
[38277606048] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[38353780245] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=16)
[38372324430] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: connected to /run/wayland-0
[38380747614] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=17)
[38386656957] [[32mINFO [0m] [clock] [CPU2] clock: connected to /run/wayland-0
[38624935602] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=1 rect=1920x1080 src_px=0xffccccff dst_px=0xffccccff damage=1920x1080+0,0 res_id=1
[38626258275] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 945,521 src_px=0x10202020 dst_before=0xffccccff dst_after=0xffc1c1f1
[38683399392] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[38691012690] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
[38722835184] [[32mINFO [0m] [clock] [CPU2] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[39114097110] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[39155741097] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 14:03:38 UTC-8 system_unix=1777586618.588536916
[42384349689] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=30 frame=6
[42395384724] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[42441178692] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[42447785193] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[43462016004] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[43631935842] [[32mINFO [0m] [bloom::render] [CPU1] bloom: chrome overlay ready buffer=5 size=1920x1080
[44017324791] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:03:41.022734627 unix_secs=1777586621.022734627
[45364490655] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=30 frame=6
[45371350926] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[45478588848] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 14:03:41 UTC-8 system_unix=1777586621.753346146
[47745952023] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=3 height=30 frame=6
[47866671732] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_popup obj=23 assigned to xdg_surface=21
[50048578140] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:03:44.038528117 unix_secs=1777586624.038528117
[51587988573] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 14:03:44 UTC-8 system_unix=1777586624.808053071
[56080243626] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:03:47.054363005 unix_secs=1777586627.054363005
[57720410946] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 14:03:47 UTC-8 system_unix=1777586627.874271303
[62111742132] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:03:50.070097144 unix_secs=1777586630.070097144
[63890044548] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 14:03:50 UTC-8 system_unix=1777586630.959058882
[68121141462] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:03:53.073269503 unix_secs=1777586633.073269503
[70231697580] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 14:03:54 UTC-8 system_unix=1777586634.128706259
[74149247865] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:03:56.088851264 unix_secs=1777586636.088851264
[76467382893] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 14:03:57 UTC-8 system_unix=1777586637.246746189
[80172519636] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:03:59.100485087 unix_secs=1777586639.100485087
[82720363407] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 14:04:00 UTC-8 system_unix=1777586640.374088572
[86203851888] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:04:02.116176656 unix_secs=1777586642.116176656
[88940591553] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 14:04:03 UTC-8 system_unix=1777586643.484150175
[92235594264] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:04:05.132029331 unix_secs=1777586645.132029331
[95177263203] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 14:04:06 UTC-8 system_unix=1777586646.602520832
[98267051553] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:04:08.147774756 unix_secs=1777586648.147774756
[101477934459] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 14:04:09 UTC-8 system_unix=1777586649.752880764
[104298664272] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:04:11.163573097 unix_secs=1777586651.163573097
[107644034982] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 14:04:12 UTC-8 system_unix=1777586652.836056162
[110330203599] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:04:14.179342018 unix_secs=1777586654.179342018
[113822025867] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 14:04:15 UTC-8 system_unix=1777586655.924908186
[116361720651] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:04:17.195115080 unix_secs=1777586657.195115080
[119946004860] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 14:04:18 UTC-8 system_unix=1777586658.986936837
[122393245128] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:04:20.210872897 unix_secs=1777586660.210872897
[126079032090] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 14:04:22 UTC-8 system_unix=1777586662.053347096
[128424932823] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:04:23.226694783 unix_secs=1777586663.226694783
[132248216364] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 14:04:25 UTC-8 system_unix=1777586665.138009424
[134456487297] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:04:26.242472416 unix_secs=1777586666.242472416
[138386701167] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 14:04:28 UTC-8 system_unix=1777586668.207160003
[140487987882] [[32mINFO [0m] [kernel::
```
</details>
