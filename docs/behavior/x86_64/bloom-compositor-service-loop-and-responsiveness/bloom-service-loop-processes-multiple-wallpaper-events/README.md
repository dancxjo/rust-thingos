# ❌ Scenario: bloom service loop processes multiple wallpaper events

> Last run: 2026-04-25 11:42:22

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 10026ms | - - - |
| 2 | Then the serial output should contain "bloom: service loop started" within 60s | ❌ | 61044ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[30646680372] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[31193617290] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[31219214202] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[31259903862] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[31301425089] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[31326677415] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[31330011042] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[31378563480] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[31380615387] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[31407804681] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[31426597752] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[31427370414] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[31473534510] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[31474905000] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[31581867966] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[31664934114] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[31692727308] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[31777314558] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[31894652559] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[31953104370] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[31982499087] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[32114679828] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[32189085159] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[32371334193] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[32712541752] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1211291598 elapsed_us=605645
[32713465719] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[32832245094] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[32835798270] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[32888374233] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[32945966790] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[32960232558] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[32961479100] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[33085640313] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[33091984992] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[33094774548] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[33099285780] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[33292175334] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[33293297268] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[33338974416] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[33348147657] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[33362578755] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[33369818328] [[32mINFO [0m] [iso9660d] [CPU2] ISO9660D: Starting VFS provider
[33411373743] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[33844839963] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[34077783234] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[34302278241] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[34533947448] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[34812695610] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[35046388047] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[35302448610] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[35529702549] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[35756851845] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[35969107086] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[36190974963] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[36435521100] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[36666126585] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[36668356824] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[36670578219] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices'
[36685938465] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:00.0/vendor'
[36690080163] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:00.0/vendor'
[36700137837] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:00.0/device'
[36702112260] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:00.0/device'
[36706016787] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:00.0/class'
[36707733744] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:00.0/class'
[36711763242] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:00.0/status'
[36713137395] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:00.0/status'
[36717453003] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:00.0/kind'
[36719954436] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:00.0/kind'
[36724919385] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/vendor'
[36727337856] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:01.0/vendor'
[36734487801] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/device'
[36737353686] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:01.0/device'
[36743255340] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/class'
[36744650283] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:01.0/class'
[36747995394] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/status'
[36749279457] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:01.0/status'
[36753215730] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/kind'
[36755535267] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:01.0/kind'
[36760936113] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/vendor'
[36763637658] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/vendor'
[36772029954] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/device'
[36774593559] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/device'
[36780857223] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/class'
[36783392778] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/class'
[36789594732] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/status'
[36792197673] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/status'
[36796559184] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/kind'
[36799103451] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/kind'
[36803255379] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.0/vendor'
[36804793080] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.0/vendor'
[36808088229] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.0/device'
[36809629230] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.0/device'
[36813035886] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.0/class'
[36815239230] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.0/class'
[36818701524] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.0/status'
[36820741551] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.0/status'
[36825205824] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.0/kind'
[36828010890] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.0/kind'
[36835076784] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/vendor'
[36838356324] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/vendor'
[36843040872] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/device'
[36844638468] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/device'
[36847870488] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/class'
[36849522303] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/class'
[36852790788] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/status'
[36854760822] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/status'
[36858563148] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/kind'
[36860412732] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/kind'
[36864413355] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.3/vendor'
[36866878323] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.3/vendor'
[36872123112] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.3/device'
[36874352889] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.3/device'
[36877627941] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.3/class'
[36879454491] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.3/class'
[36883496529] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.3/status'
[36885509001] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.3/status'
[36889465635] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.3/kind'
[36891323337] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.3/kind'
[36895806354] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/vendor'
[36899866542] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/vendor'
[36906697707] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/device'
[36908545212] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/device'
[36911670378] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/class'
[36914156994] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/class'
[36917848242] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/status'
[36921267042] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/status'
[36927234564] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/kind'
[36931177140] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/kind'
[36936891618] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/vendor'
[36938845977] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/vendor'
[36942185313] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/device'
[36944023182] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/device'
[36947845044] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/class'
[36949776831] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/class'
[36953003571] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/status'
[36956717952] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/status'
[36962698608] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/kind'
[36966481002] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/kind'
[36973866270] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-01f0/vendor'
[36975872241] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-01f0/vendor'
[36979033938] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-01f0/device'
[36981023013] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-01f0/device'
[36984268266] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup p[36993652641] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-01f0/status'
ath='devices/isa-01f0/class'
[36986351094] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-01f0/class'
[36989963868] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-01f0/status'
[37002355467] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-01f0/kind'
[37005395988] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-01f0/kind'
[37010031927] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:00.0 kind=pci_device vendor=0x8086 device=0x29c0 class=0x060000 present=true
[37012491780] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:01.0 kind=pci_device vendor=0x1af4 device=0x1050 class=0x030000 present=true
[37014620511] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:02.0 kind=pci_device vendor=0x1af4 device=0x1000 class=0x020000 present=true
[37017592557] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/status'
[37020822762] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/status'
[37113703275] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=9 PID=9
[37133993919] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:1f.0 kind=pci_device vendor=0x8086 device=0x2918 class=0x060100 present=true
[37136062491] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:1f.2 kind=pci_device vendor=0x8086 device=0x2922 class=0x010601 present=true
[37140949758] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/status'
[37143828579] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/status'
[37176638499] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/common_bar'
[37189875030] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/common_offset'
[37195402233] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/notify_bar'
[37200971148] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=10 PID=10
[37201908447] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/notify_offset'
[37209689649] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/notify_multiplier'
[37218868500] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/device_bar'
[37222644624] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Starting AHCI VFS driver
[37228873473] [[32mINFO [0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/device_offset'
[37280227578] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[37328591520] [[32mINFO [
```
</details>
