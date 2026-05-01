# ✅ Scenario: Alt+Tab cycles through multiple windows

> Last run: 2026-05-01 11:59:18

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 12880ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 360ms | - [📜](./02/serial.log) - |
| 3 | And I type "clock &" on the serial console | ✅ | 187ms | - [📜](./03/serial.log) - |
| 4 | And I wait for 2 seconds | ✅ | 1999ms | - [📜](./04/serial.log) - |
| 5 | And I type "clock &" on the serial console | ✅ | 180ms | - [📜](./05/serial.log) - |
| 6 | And I wait for 2 seconds | ✅ | 2001ms | - [📜](./06/serial.log) - |
| 7 | And I type "clock &" on the serial console | ✅ | 182ms | - [📜](./07/serial.log) - |
| 8 | And I wait for 5 seconds | ✅ | 5003ms | - [📜](./08/serial.log) - |
| 9 | When I press alt+tab | ✅ | 264ms | - [📜](./09/serial.log) - |
| 10 | Then the latest output should contain "bloom: focus cycled from None" within 10s | ✅ | 0ms | - - - |
| 11 | When I press alt+tab | ✅ | 263ms | - [📜](./11/serial.log) - |
| 12 | Then the latest output should contain "bloom: focus cycled from Some" within 10s | ✅ | 0ms | - - - |
| 13 | When I press shift+alt+tab | ✅ | 277ms | - [📜](./13/serial.log) - |
| 14 | Then the latest output should contain "bloom: focus cycled from Some" within 10s | ✅ | 0ms | - - - |
| 15 | And the output contains "forward=false" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[40274081826] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[40699004610] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[40730260824] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[40786016865] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[40836927054] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[40867515777] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[40870959195] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[40929860664] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[40932916794] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[40962796446] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[40986273603] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[40987390950] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[41039867484] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[41041354233] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[41173439538] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[41287141368] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[41313510579] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[41396039949] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[41502380040] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[41550703293] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[41579335017] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[41710387653] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[41766940314] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[41926647609] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[42221758590] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1155314523 elapsed_us=577657
[42222626523] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[42313256568] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[42317484330] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[42396667335] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[42440062401] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[42450237159] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[42451812216] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[42561447753] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[42567444183] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[42579467832] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[42732075309] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[42734982576] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[42789514713] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[42792004431] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[42795749172] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[42798558957] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[42826263612] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[42857414127] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[42864268029] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[42867282645] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[42878906169] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[42894310008] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[42897887340] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[42914504655] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[42920929359] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[42929064420] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[42936618483] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[42944994510] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[42966079365] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[42982304673] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[43026106629] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[43034868162] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[43150800132] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[43165113387] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[43178604876] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: cursor queue (queue 1) configured
[43179943092] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[43181175477] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: composition paths: gpu_opaque_copy=enabled cpu_alpha_blend=enabled
[43185837816] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[43198736460] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: host scanout 1280x800 enabled=true (bootfb was 1920x1080)
[43418276748] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
cloc[43673370510] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
k[43737520695] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor DMA buffer ready (16 pages at phys=0x2723000)
 &[43869173139] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[43894238685] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'

[43975792938] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[43990510080] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[43993550040] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[1] 10
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[44054447415] [[32mINFO [0m] [clock] [CPU1] clock: running at low scheduler priority tid=10
[44173171812] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[44400775650] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[44471489964] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=11)
[44495530827] [[32mINFO [0m] [bloom] [CPU2] bloom: ENTERING MAIN
[44497478883] [[32mINFO [0m] [bloom] [CPU2] bloom: compositor service starting
[44619111174] [[32mINFO [0m] [bloom] [CPU2] bloom: connected to /dev/display/card0 on try 0
[44639773134] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hwrng' name='hwrng' class=Other kind='dev.rng.HwRng' start='_RNvCs7wlZbVUrQWf_5hwrng20thingos_driver_start'
[44691433974] [[32mINFO [0m] [bloom] [CPU2] bloom: output0 1280x800 @ 60000mHz
[44693776281] [[32mINFO [0m] [bloom] [CPU2] bloom: display driver does not support VBLANK
[44695062357] [[32mINFO [0m] [bloom] [CPU2] bloom: display driver supports linear dmabuf import
[44696444694] [[32mINFO [0m] [bloom] [CPU2] bloom: display driver supports GPU blit (hardware transfer/flush)
[44697743079] [[32mINFO [0m] [bloom] [CPU2] bloom: display driver supports direct scanout (zero-copy path to display)
[44702277378] [[32mINFO [0m] [bloom] [CPU2] bloom: display driver supports partial flush (damage regions)
[44703402612] [[32mINFO [0m] [bloom] [CPU2] bloom: display driver supports resource cache (pre-allocated buffer pool)
[44704566027] [[32mINFO [0m] [bloom] [CPU2] bloom: creating service port...
[44847264594] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[45047309043] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[45186181227] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pistil background renderer loaded from /lib/libpistil.so
[45188486640] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[45189778260] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[45200967306] [[32mINFO [0m] [bloom] [CPU2] bloom: initial theme configured Facet Frame
[45286495947] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[45483307716] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[45569943078] [[32mINFO [0m] [bloom] [CPU2] bloom: initial wallpaper configured /share/wallpapers/flower.png
[45679494432] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[45776465625] [[32mINFO [0m] [bloom::render] [CPU2] bloom: cursor ready buffer=2 size=48x48 hotspot=11,6
[45793710765] [[32mINFO [0m] [bloom] [CPU2] bloom: watching wallpaper config /session/desktop/wallpaper
[45797700894] [[32mINFO [0m] [bloom] [CPU2] bloom: watching theme config /session/desktop/theme
[45855887682] [[32mINFO [0m] [bloom] [CPU2] bloom: Wayland server thread spawned
[45859030701] [[32mINFO [0m] [bloom::loop_types] [CPU2] bloom: service loop started
[45859540683] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: starting
[45860432673] [[32mINFO [0m] [bloom::loop_types] [CPU2] bloom: output0 1280x800 @ 60000mHz ready
[45899312481] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[46008540105] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: listening on /run/wayland-0
[46009983657] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[46022734296] [[32mINFO [0m] [clock] [CPU1] clock: connected to /run/wayland-0
[46208425956] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[46264653072] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=1 rect=1280x800 src_px=0xff0b0a10 dst_px=0xff0b0a10 damage=1280x800+0,0 res_id=1 gpu_planes=0 cpu_planes=2
[46266889647] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 638,399 src_px=0x10202020 dst_before=0xff0b0a10 dst_after=0xff0c0b11
[46279439415] [[32mINFO [0m] [bloom::loop_types] [CPU2] First frame rendered
[46400599872] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/xhci' name='xhci' class=Block kind='dev.usb.Xhci' start='thingos_driver_start_safe'
[46404044016] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 15 driver(s) found
[46571946531] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Starting AHCI VFS driver
[46720970340] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[46897806648] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=14)
[46981007964] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: connected to /run/wayland-0
[47083036407] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=15)
[47248345155] [[32mINFO [0m] [clock] [CPU3] clock: running at low scheduler priority tid=15
[47287935024] [[32mINFO [0m] [clock] [CPU3] clock: connected to /run/wayland-0
[47415878037] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Mounted atapi2 at /dev/storage/atapi2
[47420360394] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Provider loop online at /dev/storage/atapi2
[47504768982] [[32mINFO [0m] [clock] [CPU1] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[47513017893] [[32mINFO [0m] [clock] [CPU1] clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf
[47666364207] [[32mINFO [0m] [rtc_cmos] [CPU1] RTC: claimed /sys/devices/isa-0070 (handle=2)
[47708748318] [[32mINFO [0m] [rtc_cmos] [CPU1] RTC: 2026-05-01 18:59:31 = 1777661971 unix_secs
[47710937571] [[32mINFO [0m] [kernel::time] [CPU1] System clock anchored: unix_secs=1777661971, mono_ns=23855213266, offset=1777661947144786734ns
[47713024557] [[32mINFO [0m] [rtc_cmos] [CPU1] RTC: System clock anchored to 1777661971 unix_secs
[47744135901] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 18:59:31.013087107 unix_secs=1777661971.013087107
[47938815177] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: bristle pid=8
[47958192810] [[32mINFO [0m] [ps2_mouse] [CPU3] ps2_mouse: bristle pid=8
[47990695797] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[47998926690] [[32mINFO [0m] [clock] [CPU3] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[48000883755] [[32mINFO [0m] [clock] [CPU3] clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf
[48035748189] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[48038358984] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: bound wp_presentation
[48049202883] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: wl_region smoke test: create+add+set_opaque+destroy
[48093980715] [[32mINFO [0m] [ps2_mouse] [CPU3] ps2_mouse: sample rate set to 60 Hz
[48139282389] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU2] bloom: registered titlebar drag zone surface=1 height=28 frame=6
[48153798660] [[32mINFO [0m] [bloom::services::input_service] [CPU2] bloom: registered bristle pointer sink
[48162107994] [[32mINFO [0m] [bloom::services::wallpaper] [CPU2] bloom: preparing background /share/wallpapers/flower.png
[48165470760] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[48428163597] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[48540849159] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: imported dmabuf wl_buffer=111 480x320 format=0x34325241
clock &
[2] 19
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[51133348596] [[32mINFO [0m] [clock] [CPU1] clock: running at low scheduler priority tid=19
[51138262593] [[32mINFO [0m] [clock] [CPU1] clock: connected to /run/wayland-0
[51599545998] [[32mINFO [0m] [clock] [CPU1] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[51601533390] [[32mINFO [0m] [clock] [CPU1] clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf
[53745418782] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 18:59:34.016203383 unix_secs=1777661974.016203383
[54309522036] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU2] bloom: registered titlebar drag zone surface=2 height=28 frame=6
[54317887899] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[55211929014] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU2] bloom: registered titlebar drag zone surface=3 height=28 frame=6
[55224485844] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
cloc[58064753439] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU2] bloom: registered titlebar drag zone surface=4 height=28 frame=6
[58087369692] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
k[58177070292] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU2] bloom: registered titlebar drag zone surface=5 height=28 frame=6
 &[58220519841] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_popup obj=23 assigned to xdg_surface=21

[3] 20
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[58468330338] [[32mINFO [0m] [clock] [CPU2] clock: running at low scheduler priority tid=20
[58474155564] [[32mINFO [0m] [clock] [CPU2] clock: connected to /run/wayland-0
[58994740794] [[32mINFO [0m] [clock] [CPU2] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[58996964004] [[32mINFO [0m] [clock] [CPU2] clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf
[59638488756] [[32mINFO [0m] [bloom::render] [CPU2] bloom: flat window overlays ready size=1280x800
[59751485178] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 18:59:37.020458521 unix_secs=1777661977.020458521
[63286789170] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU2] bloom: registered titlebar drag zone surface=6 height=28 frame=6
[63306688104] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[63474618603] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: frame callback done object=1000 data=31592
[63477421788] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: wp_presentation_feedback.presented object=1001 tv=31.732123054 refresh_ns=16666666 seq=0
[63486920772] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: imported dmabuf wl_buffer=121 160x96 format=0x34325241
[65671321848] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: frame callback done object=1002 data=32790
[65676893832] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: wp_presentation_feedback.presented object=1003 tv=32.796977119 refresh_ns=16666666 seq=1
[65761507185] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 18:59:40.025474673 unix_secs=1777661980.025474673
[71768081946] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 18:59:43.028771227 unix_secs=1777661983.028771227
[74188320360] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 10:59:44 UTC-8 system_unix=1777661984.237808447
[74581953237] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-05-01 10:59:44 UTC-8 system_unix=1777661984.435035108
[74626817298] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-05-01 10:59:44 UTC-8 system_unix=1777661984.457323374
[74721201588] [[32mINFO [0m] [clock] [CPU1] clock: tick local=2026-05-01 10:59:44 UTC-8 system_unix=1777661984.504289915
[74970095541] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: read scancode 0x38
[74974708050] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: edge detected: Down { key: LeftAlt, mods: Mods(4), repeat: false }
[74978907333] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: sent LeftAlt to bristle (pid=8)
[74993148714] [[32mINFO [0m] [bristle] [CPU2] bristle: KeyDown received: LeftAlt
[75002620671] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: read scancode 0x0f
[75004532394] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: edge detected: Down { key: Tab, mods: Mods(4), repeat: false }
[75007867341] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: sent Tab to bristle (pid=8)
[75016385961] [[32mINFO [0m] [bristle] [CPU2] bristle: KeyDown received: Tab
[75027285036] [[32mINFO [0m] [bloom::input] [CPU2] bloom: KeyDown received: LeftAlt (raw=0x00e2, mods=Mods(4), repeat=false)
[75030013938] [[32mINFO [0m] [bloom::input] [CPU2] bloom: KeyDown received: Tab (raw=0x002b, mods=Mods(4), repeat=false)
[75034462668] [[32mINFO [0m] [bloom::input] [CPU2] bloom: focus cycled from None to Some(6) (forward=true)
[75147013161] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: read scancode 0x8f
[75149351211] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: edge detected: Up { key: Tab, mods: Mods(4) }
[75160487952] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: sent Tab to bristle (pid=8)
[75162778482] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: read scancode 0xb8
[75163788216] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: edge detected: Up { key: LeftAlt, mods: Mods(0) }
[75166970769] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: sent LeftAlt to bristle (pid=8)
[75841417905] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: read scancode 0x38
[75843309366] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: edge detected: Down { key: LeftAlt, mods: Mods(4), repeat: false }
[75848504622] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: sent LeftAlt to bristle (pid=8)
[75852662853] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: read scancode 0x0f
[75854101323] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: edge detected: Down { key: Tab, mods: Mods(4), repeat: false }
[75856220847] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: sent Tab to bristle (pid=8)
[75867587697] [[32mINFO [0m] [bristle] [CPU2] bristle: KeyDown received: LeftAlt
[75875606796] [[32mINFO [0m] [bristle] [CPU2] bristle: KeyDown received: Tab
[75886011531] [[32mINFO [0m] [bloom::input] [CPU2] bloom: KeyDown received: LeftAlt (raw=0x00e2, mods=Mods(4), repeat=false)
[75890400135] [[32mINFO [0m] [bloom::input] [CPU2] bloom: KeyDown received: Tab (raw=0x002b, mods=Mods(4), repeat=false)
[75892436928] [[32mINFO [0m] [bloom::input] [CPU2] bloom: focus cycled from Some(6) to Some(3) (forward=true)
[76030847640] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: read scancode 0x8f
[76032934329] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: edge detected: Up { key: Tab, mods: Mods(4) }
[76038230994] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: sent Tab to bristle (pid=8)
[76043347908] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: read scancode 0xb8
[76046086446] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: edge detected: Up { key: LeftAlt, mods: Mods(0) }
[76053826629] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: sent LeftAlt to bristle (pid=8)
[76711418157] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: read scancode 0x2a
[76713243486] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: edge detected: Down { key: LeftShift, mods: Mods(1), repeat: false }
[76717038783] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: sent LeftShift to bristle (pid=8)
[76720519986] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: read scancode 0x38
[76721582982] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: edge detected: Down { key: LeftAlt, mods: Mods(5), repeat: false }
[76723457118] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: sent LeftAlt to bristle (pid=8)
[76734776316] [[32mINFO [0m] [bristle] [CPU2] bristle: KeyDown received: LeftShift
[76742223723] [[32mINFO [0m] [bristle] [CPU2] bristle: KeyDown received: LeftAlt
[76749173358] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: read scancode 0x0f
[76751384127] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: edge detected: Down { key: Tab, mods: Mods(5), repeat: false }
[76754894997] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: sent Tab to bristle (pid=8)
[76764542943] [[32mINFO [0m] [bristle] [CPU2] bristle: KeyDown received: Tab
[76778446305] [[32mINFO [0m] [bloom::input] [CPU2] bloom: KeyDown received: LeftShift (raw=0x00e1, mods=Mods(1), repeat=false)
[76781910051] [[32mINFO [0m] [bloom::input] [CPU2] bloom: KeyDown received: LeftAlt (raw=0x00e2, mods=Mods(5), repeat=false)
[76784478540] [[32mINFO [0m] [bloom::input] [CPU2] bloom: KeyDown received: Tab (raw=0x002b, mods=Mods(5), repeat=false)
[76786324395] [[32mINFO [0m] [bloom::input] [CPU2] bloom: focus cycled from Some(3) to Some(6) (forward=false)
[76915977369] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: read scancode 0x8f
[76917799167] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: edge detected: Up { key: Tab, mods: Mods(5) }
[76930816215] [[32mINFO [0m] [ps2_kbd] [CPU2] ps2_kbd: sent Tab to bristle (pid=8)

```
</details>
