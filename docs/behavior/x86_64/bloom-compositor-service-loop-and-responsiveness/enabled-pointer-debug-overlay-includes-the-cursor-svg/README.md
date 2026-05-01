# ❌ Scenario: enabled pointer debug overlay includes the cursor svg

> Last run: 2026-04-30 17:57:15

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 10335ms | - [📜](./01/serial.log) - |
| 2 | Then the pointer debug overlay should include the cursor svg | ❌ | 5203ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[32216003985] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[32472876480] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[32493940413] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[32528512269] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[32561885862] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[32582754798] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[32584547193] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[32624415978] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[32626167024] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[32647091697] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[32661866424] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[32662509396] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[32698183254] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[32699231664] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[32791655589] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[32872719363] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[32895541965] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[32974131135] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[33076032066] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[33103457178] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[33128000235] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[33228841635] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[33279781260] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[33417243156] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[33672664938] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=952164642 elapsed_us=476082
[33673516833] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[33761567631] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[33765219939] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[33804828156] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[33839536170] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[33849937011] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[33851708286] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[33947457027] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[33950886519] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[33955530213] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[34135070343] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[34136784132] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[34168711566] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[34171041432] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[34172482971] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[34173318960] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[34200669921] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[34231107108] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[34236354900] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[34237996155] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[34244361261] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[34247724720] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[34249286544] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[34256137443] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[34260513540] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[34262859609] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[34267471095] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[34269770304] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[34274810856] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[34282158801] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[34323213837] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[34328314581] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[34416261165] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[34426217232] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[34432047177] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[34475007732] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[34641383007] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[34804194744] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[34974457848] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[34986061341] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[34995631209] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[35001700404] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[35005091022] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[35160707373] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[35326698660] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[35431575828] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[35507607465] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[35508734151] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[35523574515] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[35745540798] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[35767667529] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1920x1080 @ 60000mHz
[35768733462] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[35769382671] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[35770312017] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[36058438086] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[36212551188] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[36377258709] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[36378412521] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[36379360710] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[36380202342] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil nine-slice shadow renderer loaded
[36388000044] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Solarized Warm
[36852605889] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[37015003179] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[37116264306] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[37343766768] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=96x96 hotspot=21,12
[37353889914] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[37356064449] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[37400957286] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[37402755291] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[37403655729] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1920x1080 @ 60000mHz ready
[37406060802] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: starting
[37414648359] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[37425180573] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: listening on /run/wayland-0
[37426341249] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[37576022946] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[37578513291] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[37666110834] [[32mINFO [0m] [ahci_disk] [CPU3] AHCI: Starting AHCI VFS driver
[37667816340] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=13)
[37689720651] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=14)
[37700757765] [[32mINFO [0m] [ahci_disk] [CPU3] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[37704273915] [[32mINFO [0m] [clock] [CPU2] clock: connected to /run/wayland-0
[37746383829] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: claimed /sys/devices/isa-0070 (handle=2)
[37768774890] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: 2026-05-01 00:57:41 = 1777597061 unix_secs
[37770128682] [[32mINFO [0m] [kernel::time] [CPU3] System clock anchored: unix_secs=1777597061, mono_ns=18884903449, offset=1777597042115096551ns
[37771095120] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: System clock anchored to 1777597061 unix_secs
[37783449363] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: connected to /run/wayland-0
[37787056857] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:57:41.007821809 unix_secs=1777597061.007821809
[37791614553] [[32mINFO [0m] [ahci_disk] [CPU3] AHCI: Mounted atapi2 at /dev/storage/atapi2
[37792401933] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: bristle pid=8
[37792617786] [[32mINFO [0m] [ahci_disk] [CPU3] AHCI: Provider loop online at /dev/storage/atapi2
[38025839511] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=1 rect=1920x1080 src_px=0xffccccff dst_px=0xffccccff damage=1920x1080+0,0 res_id=1
[38027311575] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 958,539 src_px=0x20181818 dst_before=0xffccccff dst_after=0xffb5b5e2
[38046466887] [[32mINFO [0m] [clock] [CPU2] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[38070964470] [[32mINFO [0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x47
[38074421121] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x47 (is_aux=false)
[38082830709] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: bristle pid=8
[38106470853] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[38130106707] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[38133504882] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 16:57:41 UTC-8 system_unix=1777597061.175701504
[38134935300] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=30 frame=6
[38148941952] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[38168208342] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: sample rate set to 60 Hz
[38193561615] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
[39508413021] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[39509900430] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: bound wp_presentation
[41639852001] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[41646470910] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[41691541056] [[32mINFO [0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x38
[41699611767] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0x38 (is_aux=false)
[41701372944] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: read scancode 0x38
[41704005222] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: edge detected: Down { key: LeftAlt, mods: Mods(4), repeat: false }
[41706817185] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: sent LeftAlt to bristle (pid=8)
[41710491438] [[32mINFO [0m] [bristle] [CPU2] bristle: KeyDown received: LeftAlt
[41865207780] [[32mINFO [0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x41
[41870600541] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0x41 (is_aux=false)
[41872617798] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: read scancode 0x41
[41874089070] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: edge detected: Down { key: F7, mods: Mods(4), repeat: false }
[41877010197] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: sent F7 to bristle (pid=8)
[41880591258] [[32mINFO [0m] [bristle] [CPU2] bristle: KeyDown received: F7
[42037954134] [[32mINFO [0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xc1
[42045735864] [[32mINFO [0m] [bloom::input] [CPU1] bloom: KeyDown received: LeftAlt (raw=0x00e2, mods=Mods(4), repeat=false)
[42047335209] [[32mINFO [0m] [bloom::input] [CPU1] bloom: KeyDown received: F7 (raw=0x0040, mods=Mods(4), repeat=false)
[42048369627] [[32mINFO [0m] [bloom::input] [CPU1] bloom: pointer debug overlay enabled
[42071512725] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0xc1 (is_aux=false)
[42072830910] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: read scancode 0xc1
[42073942911] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: edge detected: Up { key: F7, mods: Mods(4) }
[42076129227] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: sent F7 to bristle (pid=8)
[42212551821] [[32mINFO [0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xb8
[42219740409] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0xb8 (is_aux=false)
[42221074071] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: read scancode 0xb8
[42222005628] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: edge detected: Up { key: LeftAlt, mods: Mods(0) }
[42223912731] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: sent LeftAlt to bristle (pid=8)
[42228122112] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=30 frame=6
[42235877079] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[42308589675] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: imported dmabuf wl_buffer=111 480x320 format=0x34325241
[43029124809] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pointer debug overlay ready buffer=5 size=460x144
[43818356538] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:57:44.024167537 unix_secs=1777597064.024167537
[44271243819] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 16:57:44 UTC-8 system_unix=1777597064.250362588
[46280258343] [[32mINFO [0m] [bloom::render] [CPU1] bloom: chrome overlay ready buffer=6 size=1920x1080
[46853689476] [[32mINFO [0m] [bloom::render] [CPU1] bloom: shadow overlay ready buffer=7 size=1920x1080
[48751192743] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=3 height=30 frame=6
[48760209597] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_popup obj=23 assigned to xdg_surface=21
[48790175940] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: imported dmabuf wl_buffer=121 160x96 format=0x34325241
[49850846298] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:57:47.040463435 unix_secs=1777597067.040463435
[50492478564] [[32m
```
</details>
