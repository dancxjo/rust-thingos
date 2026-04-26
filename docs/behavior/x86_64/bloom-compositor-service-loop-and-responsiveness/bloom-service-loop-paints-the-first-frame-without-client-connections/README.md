# ✅ Scenario: bloom service loop paints the first frame without client connections

> Last run: 2026-04-26 15:25:14

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 13313ms | - - - |
| 2 | Then the serial output should contain "bloom: service loop started" within 60s | ✅ | 921ms | - [📜](./02/serial.log) - |
| 3 | And the serial output should contain "bloom: output0" within 60s | ✅ | 1ms | - - - |
| 4 | And the serial output should contain "First frame rendered" within 60s | ✅ | 1956ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[39390187815] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[40240372590] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[40302050217] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[40401914289] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[40498958940] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[40559466552] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[40564903764] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[40675229133] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[40679909655] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[40741002852] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[40784658354] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[40786202952] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[40894682103] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[40897662201] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[41146519590] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[41317634985] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[41387218059] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[41525030052] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[41695427070] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[41783360553] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[41852420874] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[42157948767] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[42280041144] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[42678538914] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[43387958526] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=2430031263 elapsed_us=1215015
[43389429435] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[43590987660] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[43601754339] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[43815890481] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[43913815770] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[43933937685] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[43937078790] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[44126177799] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[44133216204] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[44232895014] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
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
[44372475609] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [44377904802] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[?25h[44449159491] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[44493223302] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[44493205581] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[44500076412] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[44506442079] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[44552149257] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[44571381855] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[44575501971] [[32mINFO [0m] [iso9660d] [CPU2] ISO9660D: Starting VFS provider
[44606188869] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[44643279087] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=10)
[44676169164] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[44872125375] [32mINFO [[0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[44892724239] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[44904567114] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[44970370170] [[32mINFO [0m] [bristle] [CPU1] bristle: published pid 10 to /run/bristle/pid
[44991788193] [[32mINFO [0m] [bristle] [CPU1] bristle: published device handles kbd_in=7 mouse_in=9
[45002106963] [[32mINFO [0m] [bristle] [CPU1] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[45346014120] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class='dev.storaBlock kind=ge.Ahci' start='thingos_driver_start_safe'
[45626926653] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[45888482343] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[45949489344] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[46048474494] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=11)
[46049961507] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[46055731128] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[46098201072] [[32mINFO [0m] [bloom] [CPU2] bloom: ENTERING MAIN
[46099765305] [[32mINFO [0m] [bloom] [CPU2] bloom: compositor service starting
[46149862935] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[46393591728] [[32mINFO [0m] [bloom] [CPU2] bloom: connected to /dev/display/card0 on try 0
[46411877259] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[46429391085] [[32mINFO [0m] [bloom] [CPU2] bloom: output0 1920x1080 @ 60000mHz
[46430545491] [[32mINFO [0m] [bloom] [CPU2] bloom: creating service port...
[46451475906] [[32mINFO [0m] [bloom::render] [CPU2] bloom: pistil background renderer ready
[46460389998] [[32mINFO [0m] [bloom] [CPU2] bloom: initial wallpaper configured /share/wallpapers/flower.bmp
[46477387473] [[32mINFO [0m] [bloom] [CPU2] bloom: registered with bristle (pid=10)
[46486958892] [[32mINFO [0m] [bloom] [CPU2] bloom: watching wallpaper config /session/desktop/wallpaper
[46536831495] [[32mINFO [0m] [bloom] [CPU2] bloom: Wayland server thread spawned
[46538672664] [[32mINFO [0m] [bloom::loop_types] [CPU2] bloom: service loop started
[46540582110] [[32mINFO [0m] [bloom::loop_types] [CPU2] bloom: output0 1920x1080 @ 60000mHz ready
[46554437556] [[32mINFO [0m] [bloom::services::wallpaper] [CPU2] bloom: preparing background /share/wallpapers/flower.bmp
[46587283149] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: starting
[46608576300] [[32mINFO [0m] [bristle] [CPU1] bristle: bloom sink registered (fd=7)
[46611947349] [[32mINFO [0m] [bloom::wayland] [CPU3] wayland-server: listening on /run/wayland-0
[49082389125] [[31;1mERROR[0m] [kernel::trap] [CPU1] user_page_fault tid=7 task='/bin/cambium' va=0x0000000000000003 rip=0x000000000021045c err=0x0004 p=0 u=1 w=0 i=0 fs=0x000000000021a000 task_fs=0x000000000021a000
[49129669512] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Task 'cambium' (PID 7) died with code -1. Restarting...
[49237453023] [[32mINFO [0m] [cambium] [CPU2] CAMBIUM: main started
[49266008187] [[32mINFO [0m] [cambium] [CPU2] CAMBIUM: starting device discovery manager (daemon mode)
[49576281579] [[32mINFO [0m] [cambium::catalog] [CPU2] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[49784599194] [[32mINFO [0m] [cambium::catalog] [CPU2] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[49988219094] [[32mINFO [0m] [cambium::catalog] [CPU2] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[50193310266] [[32mINFO [0m] [cambium::catalog] [CPU2] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[50429402925] [[32mINFO [0m] [cambium::catalog] [CPU2] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[50644439439] [[32mINFO [0m] [cambium::catalog] [CPU2] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[50851881432] [[32mINFO [0m] [cambium::catalog] [CPU2] DEVD CATALOG: regisd driver '/driveterers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[51054842784] [[32mINFO [0m] [cambium::catalog] [CPU2] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[51263085951] [[32mINFO [0m] [cambium::catalog] [CPU2] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[51543152496] [[32mINFO [0m] [cambium::catalog] [CPU2] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[51745646766] [[32mINFO [0m] [cambium::catalog] [CPU2] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[51966755346] [[32mINFO [0m] [cambium::catalog] [CPU2] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[52186403148] [[32mINFO [0m] [cambium::catalog] [CPU2] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[52189285434] [[32mINFO [0m] [cambium::catalog] [CPU2] DEVD CATALOG: 13 driver(s) found
[52499722143] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Starting AHCI VFS driver
[52536697587] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[52555193658] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Mounted atapi2 at /dev/storage/atapi2
[52656779736] [[31;1mERROR[0m] [bran::arch::x86_64::idt] [CPU1] KERNEL PAGE FAULT at 0x23 RIP=0x23 CS=0x8 ERR=0x10 RSP=0xffffffffb019c2a0
[52658167551] [[31;1mERROR[0m] [bran::arch::x86_64::idt] [CPU1] KERNEL PAGE FAULT at 0x1 RIP=0xffffffff8006dea8 CS=0x8 ERR=0x0 RSP=0xffffffffb019af18
[52658998623] [[31;1mERROR[0m] [bran::arch::x86_64::idt] [CPU1] Registers:
[52660205565] [[31;1mERROR[0m] [bran::arch::x86_64::idt] [CPU1]   RAX: 0xffffffffb019b312 RBX: 0xffffffffb019b728 RCX: 0x0000000000000001 RDX: 0x0000000000000001
[52662739800] [[31;1mERROR[0m] [bran::arch::x86_64::idt] [CPU1]   RSI: 0x0000000000000001 RDI: 0xffffffffb019b312 RBP: 0xffffffffb019b1d8 RSP: 0xffffffffb019af18
[52663798209] [[31;1mERROR[0m] [bran::arch::x86_64::idt] [CPU1]   R8:  0x0000000000000001 R9:  0xffffffff802d52f8 R10: 0x0000000000000010 R11: 0x000000000000005d
[52665097617] [[31;1mERROR[0m] [bran::arch::x86_64::idt] [CPU1]   R12: 0xffffffff802a13a7 R13: 0xffffffff802a13a6 R14: 0xffffffffb019b2f0 R15: 0x0000000000000002
[52666066266] [[31;1mERROR[0m] [bran::arch::x86_64::idt] [CPU1]   RIP: 0xffffffff8006dea8 CS:  0x0000000000000008 RFLAGS: 0x0000000000010002
[52667013762] [[31;1mERROR[0m] [bran::arch::x86_64::idt] [CPU1] Backtrace (current rbp=0xffffffffb019b1d8):
[52668286440] [[31;1mERROR[0m] [bran::arch::x86_64::idt] [CPU1]   [00] 0xffffffff80207696 (frame=0xffffffffb019b1d8)
[52670354451] [[31;1mERROR[0m] [bran::arch::x86_64::idt] [CPU1] Stack Dump:
[52671849747] [[31;1mERROR[0m] [bran::arch::x86_64::idt] [CPU1]   [+00] 0x0000000000000001
[52673579772] [[31;1mERROR[0m] [bran::arch::x86_64::idt] [CPU1]   [+08] 0x0000000000000001
[52675344711] [[31;1mERROR[0m] [bran::arch::x86_64::idt] [CPU1]   [+10] 0x0000000000000001
[52676860269] [[31;1mERROR[0m] [bran::arch::x86_64::idt] [CPU1]   [+18] 0x0000000000000001
[52678138062] [[31;1mERROR[0m] [bran::arch::x86_64::idt] [CPU1]   [+20] 0x0000000000000001
[52679567193] [[31;1mERROR[0m] [bran::arch::x86_64::idt] [CPU1]   [+28] 0x0000000000000001
[52680782814] [[31;1mERROR[0m] [bran::arch::x86_64::idt] [CPU1]   [+30] 0xffffffffb019b312
[52681619892] [[31;1mERROR[0m] [bran::arch::x86_64::idt] [CPU1]   [+38] 0x0000000000000000
[52683858216] [[31;1mERROR[0m] [bran::arch::x86_64::idt] [CPU1] Code at RIP:
[52687822374] [[31;1mERROR[0m] [bran::arch::x86_64::idt] [CPU1]   8a 09 88 4c 24 0f 48 8b 4c 24 38 48 89 44 24 58 
[52689325425] [[31;1mERROR[0m] [bran] [CPU1] panicked at thingos/bran/src/arch/x86_64/idt.rs:1519:5:
KERNEL PAGE FAULT
[52800734910] [[32mINFO [0m] [bloom::display] [CPU2] bloom: imported buffer 1920x1080 as ID=0
[52894669773] [[32mINFO [0m] [bloom:
```
</details>
