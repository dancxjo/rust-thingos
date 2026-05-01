# ❌ Scenario: pointer movement commits bounded damage

> Last run: 2026-04-30 17:57:15

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 10334ms | - [📜](./01/serial.log) - |
| 2 | Then the pointer debug overlay should update after mouse movement | ❌ | 34959ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[32418205710] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[32668759332] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[32689537386] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[32723527749] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[32757128250] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[32777677647] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[32779541355] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[32818985001] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[32820775713] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[32841915711] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[32856441387] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[32857424688] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[32892399672] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[32893480026] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[32982486537] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[33064057455] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[33087107691] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[33161464875] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[33263884005] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[33295759200] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[33320203818] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[33420924273] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[33469631877] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[33603068169] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[33852195795] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=937422288 elapsed_us=468711
[33853028352] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[33930399492] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[33933960027] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[33998789979] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[34032684015] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[34042452312] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[34043966946] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[34136599959] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[34139781489] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[34146115806] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[34296100905] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[34297846176] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[34329758067] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[34332098427] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[34332593460] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[34334107104] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[34353990066] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[34368314838] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[34371454557] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[34372923882] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[34380601365] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[34389254229] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[34391065005] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[34401192012] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[34404282990] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[34408806267] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[34413459399] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[34428278115] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[34440035817] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[34453107579] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[34467295071] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[34473169995] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[34563341703] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[34575583053] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[34586129127] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[34631851485] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[34800847488] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[34964766078] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[35136178968] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[35141913609] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[35150291319] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[35155070445] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[35159548545] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[35312025309] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[35471242653] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[35549359956] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[35641148136] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[35642377551] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[35657455845] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[35883432255] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[35905722039] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1920x1080 @ 60000mHz
[35906669634] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[35907319602] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[35908042764] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[35991067464] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[36146144793] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[36470300328] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[36614498349] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[36615663909] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[36616571838] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[36617390436] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil nine-slice shadow renderer loaded
[36625200183] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Solarized Warm
[37100823705] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[37266021705] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[37500486573] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[37502461524] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[37603182342] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Starting AHCI VFS driver
[37645160058] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[37662097737] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Mounted atapi2 at /dev/storage/atapi2
[37663210002] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Provider loop online at /dev/storage/atapi2
[37697189409] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: claimed /sys/devices/isa-0070 (handle=2)
[37729459383] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: 2026-05-01 00:58:42 = 1777597122 unix_secs
[37731104400] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: bristle pid=8
[37733253690] [[32mINFO [0m] [kernel::time] [CPU3] System clock anchored: unix_secs=1777597122, mono_ns=18866441385, offset=1777597103133558615ns
[37734302199] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: System clock anchored to 1777597122 unix_secs
[37746145173] [[32mINFO [0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x47
[37749837477] [[32mINFO [0m] [kernel::irq::ps2] [CPU2] PS/2 take_scancode: popped 0x47 (is_aux=false)
[37745234670] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 00:58:42.005211129 unix_secs=1777597122.005211129
[37757498229] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: bristle pid=8
[37770027570] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: sample rate set to 60 Hz
[37788891855] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[37996395294] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=96x96 hotspot=21,12
[38003747463] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[38005683969] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[38039662716] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[38043666738] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[38045062836] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1920x1080 @ 60000mHz ready
[38068667043] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=16)
[38090115756] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=17)
[38310141969] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: starting
[38323729026] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: listening on /run/wayland-0
[38324836308] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[38383962078] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: connected to /run/wayland-0
[38413452495] [[32mINFO [0m] [clock] [CPU2] clock: connected to /run/wayland-0
[38562869697] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=1 rect=1920x1080 src_px=0xffccccff dst_px=0xffccccff damage=1920x1080+0,0 res_id=1
[38564363376] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 958,539 src_px=0x20181818 dst_before=0xffccccff dst_after=0xffb5b5e2
[38648308875] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[38655413676] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
[38774985546] [[32mINFO [0m] [clock] [CPU2] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[39195032547] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 16:58:42 UTC-8 system_unix=1777597122.728855094
[39206611620] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[42151086747] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[42158550555] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[42345190701] [[32mINFO [0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x38
[42351398628] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0x38 (is_aux=false)
[42352745193] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: read scancode 0x38
[42355185444] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: edge detected: Down { key: LeftAlt, mods: Mods(4), repeat: false }
[42358067895] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: sent LeftAlt to bristle (pid=8)
[42363133032] [[32mINFO [0m] [bristle] [CPU2] bristle: KeyDown received: LeftAlt
[42518491026] [[32mINFO [0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x41
[42523862469] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0x41 (is_aux=false)
[42525181116] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: read scancode 0x41
[42525931338] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: edge detected: Down { key: F7, mods: Mods(4), repeat: false }
[42528066009] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: sent F7 to bristle (pid=8)
[42534735474] [[32mINFO [0m] [bristle] [CPU2] bristle: KeyDown received: F7
[42567729336] [[32mINFO [0m] [bloom::input] [CPU1] bloom: KeyDown received: LeftAlt (raw=0x00e2, mods=Mods(4), repeat=false)
[42569606013] [[32mINFO [0m] [bloom::input] [CPU1] bloom: KeyDown received: F7 (raw=0x0040, mods=Mods(4), repeat=false)
[42570580239] [[32mINFO [0m] [bloom::input] [CPU1] bloom: pointer debug overlay enabled
[42573881559] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=30 frame=6
[42583323156] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[42679828158] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pointer debug overlay ready buffer=4 size=460x144
[42693753762] [
```
</details>
