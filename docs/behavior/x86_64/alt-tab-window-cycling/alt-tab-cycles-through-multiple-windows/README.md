# ✅ Scenario: Alt+Tab cycles through multiple windows

> Last run: 2026-05-01 12:07:05

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 12056ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 254ms | - [📜](./02/serial.log) - |
| 3 | And I type "clock &" on the serial console | ✅ | 180ms | - [📜](./03/serial.log) - |
| 4 | And I wait for 2 seconds | ✅ | 2001ms | - [📜](./04/serial.log) - |
| 5 | And I type "clock &" on the serial console | ✅ | 181ms | - [📜](./05/serial.log) - |
| 6 | And I wait for 2 seconds | ✅ | 2001ms | - [📜](./06/serial.log) - |
| 7 | And I type "clock &" on the serial console | ✅ | 179ms | - [📜](./07/serial.log) - |
| 8 | And I wait for 5 seconds | ✅ | 5001ms | - [📜](./08/serial.log) - |
| 9 | When I press alt+tab | ✅ | 262ms | - [📜](./09/serial.log) - |
| 10 | Then the latest output should contain "bloom: focus cycled from None" within 10s | ✅ | 0ms | - - - |
| 11 | When I press alt+tab | ✅ | 258ms | - [📜](./11/serial.log) - |
| 12 | Then the latest output should contain "bloom: focus cycled from Some" within 10s | ✅ | 0ms | - - - |
| 13 | When I press shift+alt+tab | ✅ | 260ms | - [📜](./13/serial.log) - |
| 14 | Then the latest output should contain "bloom: focus cycled from Some" within 10s | ✅ | 0ms | - [📜](./14/serial.log) - |
| 15 | And the output contains "forward=false" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[37617510414] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[37880366733] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[37900477065] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[37932096774] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[37964774925] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[37986507603] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[37989327882] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[38048904927] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[38051552550] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[38081049831] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[38104129470] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[38105077329] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[38157517398] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[38158527759] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[38248855326] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[38329086906] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[38353046754] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[38432609358] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[38533456995] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[38588245014] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[38620246599] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[38765465673] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[38837004327] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[39030799467] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[39299072505] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1119432204 elapsed_us=559716
[39299985219] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[39381012033] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[39384307182] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[39432057489] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[39537871131] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[39553618962] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[39555926619] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[39658785804] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[39664226085] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[39673176708] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[39828500745] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[39831154011] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[39881057205] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[39884518575] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[39885299190] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[39887883882] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[39911159508] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[39931607166] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[39935004384] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[39936670455] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[39954471909] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[39963198231] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[39966104871] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[39987232197] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[39991783359] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[40002887826] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[40011184455] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[40061171073] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[40085789073] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[40102934025] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[40103516739] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[40106296329] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[40225806060] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[40237886700] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[40255131576] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: cursor queue (queue 1) configured
[40256369505] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[40257068280] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: composition paths: gpu_opaque_copy=enabled cpu_alpha_blend=enabled
[40269515583] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: host scanout 1280x800 enabled=true (bootfb was 1920x1080)
[40326137973] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[40504707177] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
cloc[40581701787] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor DMA buffer ready (16 pages at phys=0x2723000)
k[40657140150] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[40664035335] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[40665405429] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[40671784461] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[40673936259] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
 &
[40830212313] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[1] 10
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[40936483104] [[32mINFO [0m] [clock] [CPU1] clock: running at low scheduler priority tid=10
[41017191534] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[41091305475] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=11)
[41095033716] [[32mINFO [0m] [bloom] [CPU2] bloom: ENTERING MAIN
[41096050545] [[32mINFO [0m] [bloom] [CPU2] bloom: compositor service starting
[41142672120] [[32mINFO [0m] [bloom] [CPU2] bloom: connected to /dev/display/card0 on try 0
[41164149312] [[32mINFO [0m] [bloom] [CPU2] bloom: output0 1280x800 @ 60000mHz
[41165067207] [[32mINFO [0m] [bloom] [CPU2] bloom: display driver does not support VBLANK
[41165923887] [[32mINFO [0m] [bloom] [CPU2] bloom: display driver supports linear dmabuf import
[41166580257] [[32mINFO [0m] [bloom] [CPU2] bloom: display driver supports GPU blit (hardware transfer/flush)
[41168021598] [[32mINFO [0m] [bloom] [CPU2] bloom: display driver supports direct scanout (zero-copy path to display)
[41168712849] [[32mINFO [0m] [bloom] [CPU2] bloom: display driver supports partial flush (damage regions)
[41169489372] [[32mINFO [0m] [bloom] [CPU2] bloom: display driver supports resource cache (pre-allocated buffer pool)
[41170187982] [[32mINFO [0m] [bloom] [CPU2] bloom: creating service port...
[41179671720] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[41337673674] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hwrng' name='hwrng' class=Other kind='dev.rng.HwRng' start='_RNvCs7wlZbVUrQWf_5hwrng20thingos_driver_start'
[41485844565] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pistil background renderer loaded from /lib/libpistil.so
[41487002370] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[41487936204] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[41494816704] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[41495395194] [[32mINFO [0m] [bloom] [CPU2] bloom: initial theme configured Facet Frame
[41650434705] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[41804934798] [[32mINFO [0m] [bloom] [CPU2] bloom: initial wallpaper configured /share/wallpapers/flower.png
[41813069727] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[41962795248] [[32mINFO [0m] [bloom::render] [CPU2] bloom: cursor ready buffer=2 size=48x48 hotspot=11,6
[41966977371] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[41972272914] [[32mINFO [0m] [bloom] [CPU2] bloom: watching wallpaper config /session/desktop/wallpaper
[41974209717] [[32mINFO [0m] [bloom] [CPU2] bloom: watching theme config /session/desktop/theme
[42005447616] [[32mINFO [0m] [bloom] [CPU2] bloom: Wayland server thread spawned
[42007254333] [[32mINFO [0m] [bloom::loop_types] [CPU2] bloom: service loop started
[42008066562] [[32mINFO [0m] [bloom::loop_types] [CPU2] bloom: output0 1280x800 @ 60000mHz ready
[42008751279] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: starting
[42089901282] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: listening on /run/wayland-0
[42090884451] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[42124179108] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[42223830528] [[32mINFO [0m] [clock] [CPU1] clock: connected to /run/wayland-0
[42347977353] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=1 rect=1280x800 src_px=0xff0b0a10 dst_px=0xff0b0a10 damage=1280x800+0,0 res_id=1 gpu_planes=0 cpu_planes=2
[42349342728] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 638,399 src_px=0x10202020 dst_before=0xff0b0a10 dst_after=0xff0c0b11
[42389007573] [[32mINFO [0m] [bloom::loop_types] [CPU2] First frame rendered
[42499142532] [[32mINFO [0m] [clock] [CPU1] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[42500213547] [[32mINFO [0m] [clock] [CPU1] clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf
[42551088756] [[32mINFO [0m] [bloom::services::input_service] [CPU2] bloom: registered bristle pointer sink
[42558412710] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[42588537420] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[42747374208] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[42930334854] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/xhci' name='xhci' class=Block kind='dev.usb.Xhci' start='thingos_driver_start_safe'
[42932056398] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 15 driver(s) found
[42934824174] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU2] bloom: registered titlebar drag zone surface=1 height=28 frame=6
[42949113405] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[43024996806] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Starting AHCI VFS driver
[43054355619] [[32mINFO [0m] [rtc_cmos] [CPU2] RTC: claimed /sys/devices/isa-0070 (handle=1)
[43055701458] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[43071292968] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Mounted atapi2 at /dev/storage/atapi2
[43072148856] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Provider loop online at /dev/storage/atapi2
[43076112024] [[32mINFO [0m] [rtc_cmos] [CPU2] RTC: 2026-05-01 19:07:17 = 1777662437 unix_secs
[43077338007] [[32mINFO [0m] [kernel::time] [CPU2] System clock anchored: unix_secs=1777662437, mono_ns=21538501116, offset=1777662415461498884ns
[43078295865] [[32mINFO [0m] [rtc_cmos] [CPU2] RTC: System clock anchored to 1777662437 unix_secs
[43097246742] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:07:17.008744340 unix_secs=1777662437.008744340
[43131925386] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: bristle pid=8
[43155043074] [[32mINFO [0m] [ps2_mouse] [CPU1] ps2_mouse: bristle pid=8
[43248982491] [[32mINFO [0m] [ps2_mouse] [CPU1] ps2_mouse: sample rate set to 60 Hz
[43516959585] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=17)
[43527678381] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: connected to /run/wayland-0
[43554442932] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=18)
[43565210502] [[32mINFO [0m] [clock] [CPU3] clock: running at low scheduler priority tid=18
[43568364477] [[32mINFO [0m] [clock] [CPU3] clock: connected to /run/wayland-0
[43819768509] [[32mINFO [0m] [bloom::services::wallpaper] [CPU2] bloom: preparing background /share/wallpapers/flower.png
[43908815676] [[32mINFO [0m] [clock] [CPU3] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[43909951668] [[32mINFO [0m] [clock] [CPU3] clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf
[44605974864] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[46325484942] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU2] bloom: registered titlebar drag zone surface=2 height=28 frame=6
[46330930371] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[46935320619] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[46936795950] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: bound wp_presentation
[46941810894] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: wl_region smoke test: create+add+set_opaque+destroy
cl[47712091680] [[32mINFO [0m] [bloom::render] [CPU2] bloom: flat window overlays ready size=1280x800
ock &
[2] 19
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[48028624809] [[32mINFO [0m] [clock] [CPU1] clock: running at low scheduler priority tid=19
[48031938438] [[32mINFO [0m] [clock] [CPU1] clock: connected to /run/wayland-0
[48326608143] [[32mINFO [0m] [clock] [CPU1] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[48327816009] [[32mINFO [0m] [clock] [CPU1] clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf
[49102466754] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:07:20.012667449 unix_secs=1777662440.012667449
[49299847443] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU2] bloom: registered titlebar drag zone surface=3 height=28 frame=6
[49316147628] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[49445260524] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU2] bloom: registered titlebar drag zone surface=4 height=28 frame=6
[49452155808] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[50036478525] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: imported dmabuf wl_buffer=111 480x320 format=0x34325241
[51374122569] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU2] bloom: registered titlebar drag zone surface=5 height=28 frame=6
[51395023911] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_popup obj=23 assigned to xdg_surface=21
[51414259545] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: frame callback done object=1000 data=25656
[51416076195] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: wp_presentation_feedback.presented object=1001 tv=25.693783401 refresh_ns=16666666 seq=0
[51441039309] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: imported dmabuf wl_buffer=121 160x96 format=0x34325241
[51730189434] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: frame callback done object=1002 data=25857
[51731862699] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: wp_presentation_feedback.presented object=1003 tv=25.862945509 refresh_ns=16666666 seq=1
clock [55111087086] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:07:23.017005252 unix_secs=1777662443.017005252
&
[3] 20
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[55216098036] [[32mINFO [0m] [clock] [CPU2] clock: running at low scheduler priority tid=20
[55220690382] [[32mINFO [0m] [clock] [CPU2] clock: connected to /run/wayland-0
[55517357874] [[32mINFO [0m] [clock] [CPU2] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[55518509805] [[32mINFO [0m] [clock] [CPU2] clock: pistil generic text renderer loaded with /share/fonts/Inter-Regular.ttf
[55593207483] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU2] bloom: registered titlebar drag zone surface=6 height=28 frame=6
[55599780687] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[61118569842] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:07:26.020711964 unix_secs=1777662446.020711964
[67126332669] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:07:29.024608194 unix_secs=1777662449.024608194
[71869620273] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: read scancode 0x38
[71872777482] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: edge detected: Down { key: LeftAlt, mods: Mods(4), repeat: false }
[71876552913] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: sent LeftAlt to bristle (pid=8)
[71880127671] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: read scancode 0x0f
[71881292406] [[32mINFO [0m] [bristle] [CPU2] bristle: KeyDown received: LeftAlt
[71881453446] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: edge detected: Down { key: Tab, mods: Mods(4), repeat: false }
[71883738465] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: sent Tab to bristle (pid=8)
[71886290619] [[32mINFO [0m] [bristle] [CPU2] bristle: KeyDown received: Tab
[71894783532] [[32mINFO [0m] [bloom::input] [CPU2] bloom: KeyDown received: LeftAlt (raw=0x00e2, mods=Mods(4), repeat=false)
[71898985488] [[32mINFO [0m] [bloom::input] [CPU2] bloom: KeyDown received: Tab (raw=0x002b, mods=Mods(4), repeat=false)
[71902358583] [[32mINFO [0m] [bloom::input] [CPU2] bloom: focus cycled from None to Some(6) (forward=true)
[72056733507] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: read scancode 0x8f
[72058535406] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: edge detected: Up { key: Tab, mods: Mods(4) }
[72060842205] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: sent Tab to bristle (pid=8)
[72062755578] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: read scancode 0xb8
[72063896289] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: edge detected: Up { key: LeftAlt, mods: Mods(0) }
[72065217774] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: sent LeftAlt to bristle (pid=8)
[72728495829] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: read scancode 0x38
[72729848301] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: edge detected: Down { key: LeftAlt, mods: Mods(4), repeat: false }
[72733011417] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: sent LeftAlt to bristle (pid=8)
[72735963828] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: read scancode 0x0f
[72737077017] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: edge detected: Down { key: Tab, mods: Mods(4), repeat: false }
[72737288019] [[32mINFO [0m] [bristle] [CPU2] bristle: KeyDown received: LeftAlt
[72739053222] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: sent Tab to bristle (pid=8)
[72741796809] [[32mINFO [0m] [bristle] [CPU2] bristle: KeyDown received: Tab
[72746614809] [[32mINFO [0m] [bloom::input] [CPU2] bloom: KeyDown received: LeftAlt (raw=0x00e2, mods=Mods(4), repeat=false)
[72748630449] [[32mINFO [0m] [bloom::input] [CPU2] bloom: KeyDown received: Tab (raw=0x002b, mods=Mods(4), repeat=false)
[72750168777] [[32mINFO [0m] [bloom::input] [CPU2] bloom: focus cycled from Some(6) to Some(4) (forward=true)
[72910563792] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: read scancode 0x8f
[72911504490] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: edge detected: Up { key: Tab, mods: Mods(4) }
[72913380903] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: sent Tab to bristle (pid=8)
[72915261078] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: read scancode 0xb8
[72916047897] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: edge detected: Up { key: LeftAlt, mods: Mods(0) }
[72917176827] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: sent LeftAlt to bristle (pid=8)
[73134259902] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 19:07:32.028570491 unix_secs=1777662452.028570491
[73582336938] 
```
</details>
