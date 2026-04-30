# ❌ Scenario: displayed Wayland windows are visible through the session filesystem

> Last run: 2026-04-30 15:09:02

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Then the Wayland hello client should be visible | ❌ | 32886ms | - [📜](./01/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[34710567408] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[34981880439] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[35003703405] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[35037403071] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[35071662054] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[35092189737] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[35093983749] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[35133384528] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[35135024397] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[35156672199] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[35171462403] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[35172001887] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[35215826448] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[35216936667] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[35311844733] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[35396179533] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[35419032066] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[35494102083] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[35605849026] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[35639016699] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[35662784058] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[35776064841] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[35831312517] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[35980481196] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[36261194871] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1022690460 elapsed_us=511345
[36261911400] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[36347178747] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[36352573224] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[36399309408] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[36468428799] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[36480755817] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[36483725388] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[36588795375] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[36592316475] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[36599742861] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[36767655804] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[36769401240] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[36806587752] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[36809566365] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[36811237815] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[36811549236] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[36844820661] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[36846855507] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[36851040468] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[36852710664] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[36867701574] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[36871941909] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[36873895278] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[36885896289] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[36888848535] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[36894751212] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[36900018606] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[36915590316] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[36927476190] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[36947152605] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[36964343658] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[36970034541] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[37091202621] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[37108321107] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[37114767459] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[37152082473] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[37328213835] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[37500418890] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[37674416868] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[37759361013] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[37769876133] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[37775018094] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[37779120390] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[37865665893] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[38031655134] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[38193207525] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[38196203397] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[38292185217] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[38293174095] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[38443119594] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[38612064579] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[38636791215] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1920x1080 @ 60000mHz
[38637837744] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[38638595061] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[38639458176] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[38972573739] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[38973550407] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[38974335774] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[38982110739] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Solarized Warm
[39488311599] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[39663272781] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[39839957949] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[39958391286] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[40068093945] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[40238706453] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[40241163798] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[40278518049] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=11)
[40300849743] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=12)
[40311816270] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Starting AHCI VFS driver
[40340084202] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[40357904532] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Mounted atapi2 at /dev/storage/atapi2
[40358782695] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Provider loop online at /dev/storage/atapi2
[40532899275] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=48x48 hotspot=24,24
[40544561508] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[40547079375] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[40556197869] [[32mINFO [0m] [rtc_cmos] [CPU2] RTC: claimed /sys/devices/isa-0070 (handle=2)
[40578244476] [[32mINFO [0m] [rtc_cmos] [CPU2] RTC: 2026-04-30 22:10:35 = 1777587035 unix_secs
[40579257444] [[32mINFO [0m] [kernel::time] [CPU2] System clock anchored: unix_secs=1777587035, mono_ns=20289473869, offset=1777587014710526131ns
[40584390297] [[32mINFO [0m] [rtc_cmos] [CPU2] RTC: System clock anchored to 1777587035 unix_secs
[40585509426] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[40587292350] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[40588632480] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1920x1080 @ 60000mHz ready
[40589006865] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: starting
[40606169637] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: listening on /run/wayland-0
[40607101392] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[40607127297] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:10:35.012895889 unix_secs=1777587035.012895889
[40622128272] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: connected to /run/wayland-0
[40647421683] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: bristle pid=8
[40650374589] [[32mINFO [0m] [clock] [CPU3] clock: connected to /run/wayland-0
[40887555258] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: bristle pid=8
[40964433345] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[40997212740] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[41006477985] [[32mINFO [0m] [clock] [CPU3] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[41037759939] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: fallback sample rate 40 Hz also failed; continuing with device default
[41267827887] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=1 rect=1920x1080 src_px=0xffccccff dst_px=0xffccccff damage=1920x1080+0,0 res_id=1
[41269338099] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 945,521 src_px=0x10202020 dst_before=0xffccccff dst_after=0xffc1c1f1
[41303144454] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[41308138146] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=26 frame=6
[41319846876] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[41363672724] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
[41398706085] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: imported dmabuf wl_buffer=111 480x320 format=0x34325241
[41966166264] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:10:35 UTC-8 system_unix=1777587035.691504176
[45026626722] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[45033465840] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[45377258850] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=26 frame=6
[45397625526] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[46632541329] [[32mINFO [0m] [bloom::render] [CPU1] bloom: chrome overlay ready buffer=5 size=1920x1080
[46641002859] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:10:38.030939764 unix_secs=1777587038.030939764
[47649302217] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=3 height=26 frame=6
[47671868046] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: xdg_popup obj=23 assigned to xdg_surface=21
[47712144018] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU3] wayland-server: imported dmabuf wl_buffer=121 160x96 format=0x34325241
[47923623132] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: frame callback done object=1000 data=23951
[48213808500] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:10:38 UTC-8 system_unix=1777587038.817037103
[48809183940] [[32mINFO [0m] [wayland_hello] [CPU2] wayland_hello: frame callback done object=1001 data=24396
[52665648651] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:10:41.043271702 unix_secs=1777587041.043271702
[54340093335] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:10:41 UTC-8 system_unix=1777587041.880337096
[58695909162] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:10:44.058427994 unix_secs=1777587044.058427994
[60542911770] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:10:44 UTC-8 system_unix=1777587044.981784626
[64726300716] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:10:47.073622006 unix_secs=1777587047.073622006
[66672419253] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:10:48 UTC-8 system_unix=1777587048.046505747
[70756585086] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:10:50.088764785 unix_secs=1777587050.088764785
[72811199769] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:10:51 UTC-8 system_unix=1777587051.115620356
[76790054484] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:10:53.105494567 unix_secs=1777587053.105494567
[79081772880] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:10:54 UTC-8 system_unix=1777587054.250970453
[82817345721] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:10:56.119157444 unix_secs=1777587056.119157444
[85345520601] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:10:57 UTC-8 system_unix=1777587057.383005271
[88848010911] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:10:59.134465603 unix_secs=1777587059.134465603
[91616470638] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:11:00 UTC-8 system_unix=1777587060.518352778
[94878258816] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:11:02.149596617 unix_secs=1777587062.149596617
[97847818893] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:11:03 UTC-8 system_unix=1777587063.634201129
[100908647037] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:11:05.164796618 unix_secs=1777587065.164796618
[103878336606] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:11:06 UTC-8 system_unix=1777587066.649338298
[106938848280] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:11:08.179898527 unix_secs=1777587068.179898527
[110083224900] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:11:09 UTC-8 system_unix=1777587069.751831301
[112969366323] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:11:11.195156492 unix_secs=1777587071.195156492
[116245918173] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:11:12 UTC-8 system_unix=1777587072.833055904
[118999662738] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:11:14.210303000 unix_secs=1777587074.210303000
[122412422847] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:11:15 UTC-8 system_unix=1777587075.916526486
[125030207214] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:11:17.225573159 unix_secs=1777587077.225573159
[128551713081] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:11:18 UTC-8 system_unix=1777587078.985988288
[131060591904] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:11:20.240758756 unix_secs=1777587080.240758756
[134790904050] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:11:22 UTC-8 system_unix=1777587082.105563379
[137091582441] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:11:23.256252292 unix_secs=1777587083.256252292
[140865310740] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:11:25 UTC-8 system_unix=1777587085.142729087
[143124953037] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-04-30 22:11:26.272924984 unix_secs=1777587086.272924984
[147188587590] [[32mINFO [0m] [clock] [CPU3] clock: tick local=2026-04-30 14:11:28 UTC-8 system_unix=1777587088.304398153
[149151998985] [[32mINFO [0m] [k
```
</details>
