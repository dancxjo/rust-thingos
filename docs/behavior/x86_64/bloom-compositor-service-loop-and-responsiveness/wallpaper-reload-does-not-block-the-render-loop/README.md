# ❌ Scenario: wallpaper reload does not block the render loop

> Last run: 2026-04-25 11:42:22

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 9935ms | - [📜](./01/serial.log) - |
| 2 | Then the serial output should contain "bloom: service loop started" within 60s | ❌ | 61017ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[30336432258] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[30902963883] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[30933995796] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[30977097360] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[31020459162] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[31046746302] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[31050526848] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[31096132452] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[31098802647] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[31125412758] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[31145044128] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[31145906418] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[31195075329] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[31196634777] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[31307489070] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[31392542709] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[31420959141] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[31508559951] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[31679538891] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[31745287860] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[31778573508] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[31922598345] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[31986654282] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[32171431644] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[32511406356] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1287849255 elapsed_us=643924
[32512369164] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[32659951830] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[32665337859] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[32719989093] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[32777302140] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[32789008857] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[32790212202] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[32908446417] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[32913305568] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[32916504786] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[32924566884] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[33124266384] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[33125378154] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[33170238915] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[33176260062] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[33197543940] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[33202504038] [[32mINFO [0m] [iso9660d] [CPU2] ISO9660D: Starting VFS provider
[33220366773] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[33682228998] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Bl
ock kind='dev.storage.Ahci' start='thingos_driver_start_safe'[33918872526] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[34146094059] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[34381149231] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[34639355421] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[34875329247] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[35101656777] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[35327281209] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[35551781100] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[35773654092] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[35997901533] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[36235832160] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[36483041067] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[36485397927] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[36487783167] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices'
[36504398370] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:00.0/vendor'
[36510702162] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:00.0/vendor'
[36519822867] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:00.0/devicpci-0000:00:00.0/status'
e'
[36521449899] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:00.0/device'
[36525537939] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:00.0/class'
[36527794347] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:00.0/class'
[36534391146] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:00.0/status'
[36536263302] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file '[36542727969] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:00.0/kind'
[36545405457] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:00.0/kind'
[36549564480] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/vendor'
[36550891773] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] [36569875sysfs: matched device file023] [[32mINFO [0m] 'pci-00 [kernel::vfs::sysfs00:00:01.0/vendor'
[36554474484] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/device'
[36556028322] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: ma] [CPU1]tched device file 'pci-0000:00:01.0/device'
[36561964428] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sys sysfs: matched device file 'pfs: lookup path='devices/pci-0000:00:01.0/class'
[36564003102] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1ci] sy-0000:00:01.0/statsfs: matched device us'
file 'pci-0000:00:01.0/class'
[36567986334] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/status'
[36575028996] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/kind'
[36576593526] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:01.0/kind'
[36580871712] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/vendor'
[36582708921] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/vendor'
[36589161444] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/device'
[36591804513] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/device'
[36595820217] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/class'
[36597322905] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/class'
[36600638184] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/status'
[36602165853] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/status'
[36607723515] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/kind'
[36609395856] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/kind'
[36616001169] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.0/vendor'
[36619273878] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.0/vendor'
[36625594896] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.0/device'
[36627956937] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.0/device'
[36634303629] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.0/class'
[36636135063] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.0/class'
[36643718265] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.0/status'
[36645734862] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.0/status'
[36651369744] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.0/kind'
[36653443761] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.0/kind'
[36657551832] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/vendor'
[36659379273] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/vendor'
[36663258819] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/device'
[36664957956] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/device'
[36671855055] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/class'
[36676231251] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/class'
[36688033866] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/status'
[36690883449] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/status'
[36698047617] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/kind'
[36700139553] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/kind'
[36706459449] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.3/vendor'
[36710257254] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.3/vendor'
[36715947477] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.3/device'
[36717927312] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.3/device'
[36721737492] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.3/class'
[36724350267] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.3/class'
[36728113488] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.3/status'
[36729901593] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.3/status'
[36733631484] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.3/kind'
[36735471696] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.3/kind'
[36741261843] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/vendor'
[36744057273] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/vendor'
[36750605397] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/device'
[36753464913] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/device'
[36757871535] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/class'
[36759833583] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/class'
[36763214334] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/status'
[36766092891] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/status'
[36771675435] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/kind'
[36773779614] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched[36779507457] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/vendor'
[36782792244] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/vendor'
 device file 'isa-0070/kind'
[36788398152] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/device'
[36790570509] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/device'
[36794755833] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/class'
[36796843710] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/class'
[36800616501] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/status'
[36802540269] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/status'
[36807277122] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/kind'
[36813339486] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/kind'
[36819653706] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-01f0/vendor'
[36821883219] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-01f0/vendor'
[36825553809] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-01f0/device'
[36827811801] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-01f0/device'
[36832004319] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-01f0/class'
[36834085728] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-01f0/class'
[36838950621] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-01f0/status'
[36841578510] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-01f0/status'
[36846705852] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-01f0/kind'
[36849114423] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-01f0/kind'
[36854456265] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:00.0 kind=pci_device vendor=0x8086 device=0x29c0 class=0x060000 present=true
[36857616345] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:01.0 kind=pci_device vendor=0x1af4 device=0x1050 class=0x030000 present=true
[36860537505] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:02.0 kind=pci_device vendor=0x1af4 device=0x1000 class=0x020000 present=true
[36864385404] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/status'
[36866099061] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/status'
[36957407784] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=9 PID=9
[36979347801] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:1f.0 kind=pci_device vendor=0x8086 device=0x2918 class=0x060100 present=true
[36980833164] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:1f.2 kind=pci_device vendor=0x8086 device=0x2922 class=0x010601 present=true
[36984198537] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/status'
[36985968096] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/status'
[37017139698] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/common_bar'
[37030032468] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/common_offset'
[37036555875] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/notify_bar'
[37037671143] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=10 PID=10
[37041483039] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU3] sysfs:[37048157388] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path= lookup path=''devices/pcdevii-0000ces/pci-0000:00:02.0/virtio/notify_offset'
:00:02.0/virtio/notify_multiplier'
[37057407981] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/device_bar'
[37062873837] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Starting AHCI VFS driver
[37064919210] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/device_offset'
[37113208077] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[37169154693] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Mounted atapi2 at /dev/storage/atapi2
[37197753252] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:1f.3 kind=pci_device vendor=0x8086 device=0x2930 class=0x0c0500 present=true
[37199937819] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: discovered device slot=isa-0070 kind=rtc_cmos vendor=0x0000 device=0x0000 class=0x000000 present=true
[37202300850] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: discovered device slot=isa-0060 kind=ps2_controller vendor=0x0000 device=0x0000 class=0x000000 present=true
[37204348335] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: discovered device slot=isa-01f0 kind=dev.storage.ata vendor=0x0000 device=0x0000 class=0x000000 present=true
[37278044298] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices'
[38515325376] [[3
```
</details>
