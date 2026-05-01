# ❌ Scenario: Alt+Tab cycles through multiple windows

> Last run: 2026-05-01 13:23:52

## Steps

| # | Step | Result | Duration | Before | After | Artifacts |
|---|------|--------|----------|--------|-------|-----------|
| 1 | Given the machine is booted | ✅ | 10845ms | <a href="./01/before.png"><img src="./01/before.png" width="120" /></a> | <a href="./01/after.png"><img src="./01/after.png" width="120" /></a> | [📜](./01/serial.log) |
| 2 | When I wait for the shell prompt | ✅ | 1134ms | <a href="./02/before.png"><img src="./02/before.png" width="120" /></a> | <a href="./02/after.png"><img src="./02/after.png" width="120" /></a> | [📜](./02/serial.log) [console_interactive.png](./02/console_interactive.png) |
| 3 | And I type "clock &" on the serial console | ✅ | 281ms | <a href="./03/before.png"><img src="./03/before.png" width="120" /></a> | <a href="./03/after.png"><img src="./03/after.png" width="120" /></a> | [📜](./03/serial.log) |
| 4 | And I wait for 2 seconds | ✅ | 2302ms | <a href="./04/before.png"><img src="./04/before.png" width="120" /></a> | <a href="./04/after.png"><img src="./04/after.png" width="120" /></a> | [📜](./04/serial.log) |
| 5 | And I type "clock &" on the serial console | ✅ | 585ms | <a href="./05/before.png"><img src="./05/before.png" width="120" /></a> | <a href="./05/after.png"><img src="./05/after.png" width="120" /></a> | [📜](./05/serial.log) |
| 6 | And I wait for 2 seconds | ✅ | 1518ms | <a href="./06/before.png"><img src="./06/before.png" width="120" /></a> | <a href="./06/after.png"><img src="./06/after.png" width="120" /></a> | [📜](./06/serial.log) |
| 7 | And I type "clock &" on the serial console | ✅ | 682ms | <a href="./07/before.png"><img src="./07/before.png" width="120" /></a> | <a href="./07/after.png"><img src="./07/after.png" width="120" /></a> | [📜](./07/serial.log) |
| 8 | And I wait for 5 seconds | ✅ | 4537ms | <a href="./08/before.png"><img src="./08/before.png" width="120" /></a> | <a href="./08/after.png"><img src="./08/after.png" width="120" /></a> | [📜](./08/serial.log) |
| 9 | Then the serial output should contain "First frame rendered" within 60s | ✅ | 477ms | <a href="./09/before.png"><img src="./09/before.png" width="120" /></a> | <a href="./09/after.png"><img src="./09/after.png" width="120" /></a> | [📜](./09/serial.log) |
| 10 | And the serial output should contain "bloom: registered bristle pointer sink" within 60s | ✅ | 5557ms | <a href="./10/before.png"><img src="./10/before.png" width="120" /></a> | - |  |
| 11 | When I press alt+tab | ✅ | 5817ms | - | - | [📜](./11/serial.log) |
| 12 | Then the latest output should contain "bloom: focus cycled from None" within 10s | ✅ | 15157ms | - | - |  |
| 13 | When I press alt+tab | ✅ | 742ms | - | <a href="./13/after.png"><img src="./13/after.png" width="120" /></a> |  |
| 14 | Then the latest output should contain "bloom: focus cycled from Some" within 10s | ❌ | 10119ms | <a href="./14/before.png"><img src="./14/before.png" width="120" /></a> | - |  |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[33149475348] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[33396268521] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[33415513758] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[33444924777] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[33475476540] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[33494096691] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[33495787479] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[33532650030] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[33534279075] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[33553892625] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[33567739095] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[33568339431] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[33602277687] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[33603332862] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[33687490584] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[33769330089] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[33789965055] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[33860298153] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[33958348842] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[33986150319] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[34010558571] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[34105909035] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[34151943837] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[34278968130] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[34516423293] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=893380422 elapsed_us=446690
[34517250240] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[34595465685] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[34598929530] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[34664083410] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[34700289393] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[34708678950] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[34710051453] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[34827108030] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[34830398823] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[34840197909] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[34998063276] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[34999760433] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[35032327011] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[35034661332] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[35036047431] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[35036091189] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[35058970155] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[35074579782] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[35079288750] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[35081816418] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[35095986321] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[35099777757] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[35101499796] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[35111488335] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[35113967658] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[35118576504] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[35124165582] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[35132372748] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[35143263375] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[35153936862] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[35181034713] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[35189021835] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[35277891330] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[35288370975] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[35298139206] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: cursor queue (queue 1) configured
[35299144386] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[35299794915] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: composition paths: gpu_opaque_copy=enabled cpu_alpha_blend=enabled
[35308118010] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: host scanout 1280x800 enabled=true (bootfb was 1920x1080)
[35347494237] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[35510184930] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[35554848186] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor DMA buffer ready (16 pages at phys=0x2723000)
[35614116450] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[35622598671] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[35627124159] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[35629104390] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[35669441940] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[35834649873] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[36014775159] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[36022779408] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[36118621275] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[36119727072] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[36262389009] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[36349960185] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[36369224562] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1280x800 @ 60000mHz
[36370202781] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[36370987917] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[36371693985] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports GPU blit (hardware transfer/flush)
[36372530205] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports direct scanout (zero-copy path to display)
[36373327023] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports partial flush (damage regions)
[36373947126] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports resource cache (pre-allocated buffer pool)
[36374656164] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[36669610890] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[36670656528] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[36671385168] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[36678653946] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Facet Frame
echo [36975921510] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hwrng' name='hwrng' class=Other kind='dev.rng.HwRng' start='_RNvCs7wlZbVUrQWf_5hwrng20thingos_driver_start'
BDD_C[37144309950] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
ONS[37268967450] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
OLE_[37435477299] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=48x48 hotspot=11,6
R[37444954635] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[37446876027] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[37489727385] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[37491590631] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[37492440447] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1280x800 @ 60000mHz ready
[37494680553] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: starting
E[37506666978] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[37509748782] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: listening on /run/wayland-0
[37510692450] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
ADY_3[37660545054] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
05[37765461756] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=1 rect=1280x800 src_px=0xff0b0a10 dst_px=0xff0b0a10 damage=1280x800+0,0 res_id=1 gpu_planes=0 cpu_planes=2
[37766779611] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 638,399 src_px=0x10202020 dst_before=0xff0b0a10 dst_after=0xff0c0b11
38[37808244375] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
84[37895827134] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
_1[37969013511] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
777[38073254241] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[38080409631] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
66[38148446160] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
70437[38304720267] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
0[38359615635] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=12)
[38371728615] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: connected to /run/wayland-0
[38387503110] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=13)
[38396831583] [[32mINFO [0m] [clock] [CPU1] clock: running at low scheduler priority tid=13
[38400089706] [[32mINFO [0m] [clock] [CPU1] clock: connected to /run/wayland-0
2810147
[38685187563] [[32mINFO [0m] [clock] [CPU1] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[38686300488] [[32mINFO [0m] [clock] [CPU1] clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf
BDD_CONSOLE_READY_3053884_1777667043702810147
[38744567994] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[?25l[38773769826] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/xhci' name='xhci' class=Block kind='dev.usb.Xhci' start='thingos_driver_start_safe'
[38775477510] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 15 driver(s) found
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[38807192457] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[38809410420] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: bound wp_presentation
[38815681014] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: wl_region smoke test: create+add+set_opaque+destroy
[38834618295] [[32mINFO [0m] [ahci_disk] [CPU3] AHCI: Starting AHCI VFS driver
[38873759562] [[32mINFO [0m] [ahci_disk] [CPU3] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[38889270552] [[32mINFO [0m] [ahci_disk] [CPU3] AHCI: Mounted atapi2 at /dev/storage/atapi2
[38890524024] [[32mINFO [0m] [ahci_disk] [CPU3] AHCI: Provider loop online at /dev/storage/atapi2
[38896855470] [[32mINFO [0m] [rtc_cmos] [CPU1] RTC: claimed /sys/devices/isa-0070 (handle=2)
[38915837367] [[32mINFO [0m] [rtc_cmos] [CPU1] RTC: 2026-05-01 20:24:03 = 1777667043 unix_secs
[38916896832] [[32mINFO [0m] [kernel::time] [CPU1] System clock anchored: unix_secs=1777667043, mono_ns=19458292804, offset=1777667023541707196ns
[38917667481] [[32mINFO [0m] [rtc_cmos] [CPU1] RTC: System clock anchored to 1777667043 unix_secs
[38931336444] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:24:03.006581784 unix_secs=1777667043.006581784
[38972408310] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: bristle pid=8
[39032324661] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=28 frame=6
[39036962976] [[32mINFO [0m] [ps2_mouse] [CPU3] ps2_mouse: bristle pid=8
[39040485924] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
[39047653260] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[39136826223] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: imported dmabuf wl_buffer=111 480x320 format=0x34325241
c[39719919789] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: read scancode 0x47
[39723051819] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: edge detected: Down { key: Unknown, mods: Mods(0), repeat: false }
[39727646805] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: sent Unknown to bristle (pid=8)
[39735044085] [[32mINFO [0m] [bristle] [CPU2] bristle: KeyDown received: Unknown
[39747663945] [[32mINFO [0m] [ps2_mouse] [CPU3] ps2_mouse: sample rate set to 60 Hz
lock &
[2] 19
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[40916784315] [[32mINFO [0m] [clock] [CPU1] clock: running at low scheduler priority tid=19
[40920037653] [[32mINFO [0m] [clock] [CPU1] clock: connected to /run/wayland-0
[41599097958] [[32mINFO [0m] [clock] [CPU1] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[41600143167] [[32mINFO [0m] [clock] [CPU1] clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf
[41959781127] [[32mINFO [0m] [bloom::input] [CPU1] bloom: KeyDown received: Unknown (raw=0xffff, mods=Mods(0), repeat=false)
[41966149434] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=28 frame=6
[41974168104] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[42792985587] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=3 height=28 frame=6
[42800318022] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[43260720921] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=4 height=28 frame=6
[43270967883] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_popup obj=23 assigned to xdg_surface=21
[43303606599] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: imported dmabuf wl_buffer=121 160x96 format=0x34325241
[44257388406] [[32mINFO [0m] [bloom::render] [CPU1] bloom: flat window overlays ready size=1280x800
[44959783836] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:24:06.021498381 unix_secs=1777667046.021498381
c[47617925784] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: frame callback done object=1000 data=23802
[47620806618] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: wp_presentation_feedback.presented object=1001 tv=23.806088938 refresh_ns=16666666 seq=0
[48065918505] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: frame callback done object=1002 data=24028
[48067466997] [[32mINFO [0m] [wayland_hello] [CPU3] wayland_hello: wp_presentation_feedback.presented object=1003 tv=24.031657501 refresh_ns=16666666 seq=1
loc[50989499352] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:24:09.036409533 unix_secs=1777667049.036409533
k &
[3] 20
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[51188484534] [[32mINFO [0m] [clock] [CPU2] clock: running at low scheduler priority tid=20
[51192103083] [[32mINFO [0m] [clock] [CPU2] clock: connected to /run/wayland-0
[51508536156] [[32mINFO [0m] [clock] [CPU2] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[51509647134] [[32mINFO [0m] [clock] [CPU2] clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf
[51563850756] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=5 height=28 frame=6
[51569657931] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[57019819428] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:24:12.051520401 unix_secs=1777667052.051520401
clock &
[61227504294] [[32mINFO [0m] [clock] [CPU3] clock: running at low scheduler priority tid=21
[61231219269] [[32mINFO [0m] [clock] [CPU3] clock: connected to /run/wayland-0
[61511310498] [[32mINFO [0m] [clock] [CPU3] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[61512308979] [[32mINFO [0m] [clock] [CPU3] clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf
[4] 21
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[61570070991] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=6 height=28 frame=6
[61574727522] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[63049705389] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:24:15.066511298 unix_secs=1777667055.066511298
[69079587720] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:24:18.081448817 unix_secs=1777667058.081448817
[74192598018] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-05-01 12:24:20 UTC-8 system_unix=1777667060.637380096
[74287790577] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 12:24:20 UTC-8 system_unix=1777667060.684962895
[74617758864] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 12:24:20 UTC-8 system_unix=1777667060.849963340
[74758668699] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-05-01 12:24:20 UTC-8 system_unix=1777667060.920120070
[75109854171] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:24:21.096573429 unix_secs=1777667061.096573429
[81139772109] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:24:24.111540780 unix_secs=1777667064.111540780
[87169658070] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:24:27.126482127 unix_secs=1777667067.126482127
[93199807206] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:24:30.141558840 unix_secs=1777667070.141558840
[99229700757] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:24:33.156508949 unix_secs=1777667073.156508949
[105259699809] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:24:36.171506412 unix_secs=1777667076.171506412
[111289934085] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:24:39.186584396 unix_secs=1777667079.186584396
[117319754211] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:24:42.201532590 unix_secs=1777667082.201532590
[123349850811] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:24:45.216574752 unix_secs=1777667085.216574752
[129379774359] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:24:48.231554544 unix_secs=1777667088.231554544
[135409699260] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:24:51.246509570 unix_secs=1777667091.246509570
[141439632807] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:24:54.261492695 unix_secs=1777667094.261492695
[147470533155] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 20:24:57.276897543 unix_secs=1777667097.276897543
[148740207561] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-05-01 12:24:57 UTC-8 system_unix=1777667097.911589876
[148833904692] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 12:24:57 UTC-8 system_unix=1777667097.958341224
[149165930265] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 12:24:58 UTC-8 system_unix=1777667098.124458208
[149296366329] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-05-01 12:24:58 UTC-8 system_unix=1777667098.189525842
[152475534
```
</details>
