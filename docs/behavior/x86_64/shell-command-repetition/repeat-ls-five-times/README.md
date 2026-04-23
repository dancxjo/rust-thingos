# ✅ Scenario: Repeat ls five times

> Last run: 2026-04-22 19:29:20

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 17836ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 2409ms | - [📜](./02/serial.log) - |
| 3 | And I type "ls" on the serial console | ✅ | 1158ms | - [📜](./03/serial.log) - |
| 4 | And I type "ls" on the serial console | ✅ | 1157ms | - [📜](./04/serial.log) - |
| 5 | And I type "ls" on the serial console | ✅ | 1157ms | - [📜](./05/serial.log) - |
| 6 | And I type "ls" on the serial console | ✅ | 1153ms | - [📜](./06/serial.log) - |
| 7 | And I type "ls" on the serial console | ✅ | 1156ms | - [📜](./07/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H] [kernel] [CPU0] Scanning PCI bus...
[55049464338] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x29c0 at 00:00.0 class=060000 id=1
[55128361431] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1050 at 00:01.0 class=030000 id=2
[55180291881] [[[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Registers initialized
[55424968533] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[55427174682] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[55429434918] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: All pins masked
[55433231733] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Init complete
[55473501138] [[34mDEBUG[0m] [bran::arch::x86_64::ioapic] [CPU0] LAPIC: calibrated 71742000 ticks/sec (delta=717420, ok=true) -> init_cnt=717420
[55476530142] [[34mDEBUG[0m] [bran[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1000 at 00:02.0 class=020000 id=3
[551::arch::x86_64] [CPU0] LAPIC: calibrated timer (71742000 ticks/sec), init_cnt=717420 for 100Hz
[55477881591] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='setup_preemption_timer' elapsed_ticks=121557282 elapsed_us=60778 total_ticks=800800539 total_us=400400
[55479406224] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[55555583523] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Detected 48 CPUs. 2709659] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2918 at 00:1f.0 class=060100 id=4
[55208764182] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2922 at 00:1f.2 class=010601 id=5
[55213826316] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2930 at 00:Starting 3 secondaries...
[55557100929] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[55559335260] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[55562722743] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[55623839601] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] SMP: CPU 1 initializing preemption timer (vec=32 init_cnt=717420)
[55625258040] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[55626495903] [[34mDEBUG[0m] [kernel] [CPU1] SMP: kernel_secondary_entry arg_cpu=1 runtime_cpu=1
[55636724583] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[55637780517] [[34mDEBUG[0m] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[55639495032] 1[[34mDEBUG[0m] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[55654300581] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80017090 kstack_top=0xffffffffb0032e40 arg=0x1
[55665870645] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Task 2 assigned to CPU 1
[55691672817] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] SMP: CPU 2 initializing preemption timer (vec=32 init_cnt=717420)
[55693258203] [[34mDEBUG[0m] [kernel] [CPU2] SMP: kernel_secondary_entry arg_cpu=2 runtime_cpu=2
[55694186724] [[f.3 class=0c0500 id=6
[55227792378] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='scan_pci' elapsed_ticks=193614828 elapsed_us=96807 total_ticks=550621632 total_us=275310
[55231226952] [[32mINFO [0m] [kernel::boot_progr34mDEBUG[0m] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[55706232087] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 2 deferred inserts
[55708292145] [[34mDEBUG[0m] [kerne2
[55759898205] [[34mDEBUG[0m] [kernel::task] [CPU2l::sched] [CPU1] REGISTRY: Inserting TID=1
[55709474205] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting] SMP: CPU 2 bootstrapped with idle thread 3 and is now schedulable
[55775409888] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[55776707151] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[55822258536] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[55823149239] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] SMP: CPU 3 initializing preemption timer (vec=32 init_cnt=717420)
[55824198969] [[34e TID=2
[55712179446] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[55733170350] [[34mDEBUG[0m] [kernel::sched] mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[55824810294] [[34mDEBUG[0m] [kernel] [CPU3] SMP: kernel_secondary_entry arg_cpu=3 runtime_cpu=3
[55825442904] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[55825977438] [[34mDEBUG[0m] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[55826552265] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='start_secondary_cpus' elapsed_ticks=271486215 elapsed_us=135743 total_ticks=1149280869 total_us=574640
[55827968031] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[55929479826] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[55938911259] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80017090 kstack_top=0xfffffff[fb0055d00 arg=0x3
[55940496843] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SCPU2] SMP: CPU 2 onlinCHED: Task 4 assigned to CPU 3
[55943267985] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[55944369195] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=4
[55945600359] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[55946449251] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: Secondary CPU 3ss online!
[55947379356] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: run_scheduler entry on CPU 3
[55948427766] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: bootstrap_cpu start on CPU 3
e (triggered by scheduler spawn)
[55733964792] [[34mDEBUG[0m] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[55735097550] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU2] INIT KERNEL CTX: entry=0xffffffff80017090 kstack_top=0xffffffffb00445a0 arg=0x2
[55736034882] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: run_scheduler entry on CPU 1
[55736664060] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] SCHED: Task 3 assigned to CPU 2
[55737337887] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: bootstrap_cpu start on CPU 1
[55739962047] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Applying 1 deferred inserts
[55740965511] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserting TID=3
[55742222712] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserts applied
[55756348857] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle thread 2 and is now schedulable
[55757155773] [[34mDEBUG[0m] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[55758047004] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: run_scheduler entry on CPU 2
[55758912231] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: bootstrap_cpu start on CPU ] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[55289180298] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='register_legacy_devices' elapsed_ticks=1068870 elapsed_us=534 total_ticks=612055752 total_us=306027
[55290909729] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[55355112417] [[34mDEBUG[0m] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[55377392664] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[55378484007] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[55380024348] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[55381375335] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[55396963215] [[34mDEBUG[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[55406336931] [[34mDEBUG[0m] [bran::a[55955131419] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[55973695404] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle thread 4 and is now schedulable
[56061774549] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Enumerating 108 boot modules...
[56081405061] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
rch] [CPU0] IOAPIC: MADT parsed OK
[55408176615] [[34mDEBUG[0m] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[55409571723] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[55423370277] [56478860691] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[56481329949] [[34mDEBUG[0m] [kernel] [CPU0] Found init module: /bin/sprout (cmdline: 'init'), loading...
[56507416416] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/sprout' (len=85144, base=0x200000)
[56537738466] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[56592581793] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20f000 filesz=0 memsz=0 align=1
[56616147159] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='load_init_module' elapsed_ticks=110635866 elapsed_us=55317 total_ticks=1939014792 total_us=969507
[56619771120] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='prepare_boot_registry' elapsed_ticks=1647888 elapsed_us=823 total_ticks=1942694226 total_us=971347
[56621621496] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='map_boot_registry' elapsed_ticks=322146 elapsed_us=161 total_ticks=1944563742 total_us=972281
[56641284909] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='activate_init_aspace' elapsed_ticks=758670 elapsed_us=379 total_ticks=1964075091 total_us=982037
[56642990712] [[34mDEBUG[0m] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[56644253325] [[34mDEBUG[0m] [kernel] [CPU0] Spawning init process...
[56657145897] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb00661e0 arg=0xffffffffb001fd20
[56717826792] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[56718994860] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=5
[56720101680] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[56721198930] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='spawn_init_task' elapsed_ticks=77124069 elapsed_us=38562 total_ticks=2044131606 total_us=1022065
[56724208035] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[57165680517] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED: early-boot bringup complete; resuming steady-state SMP scheduling
[57166881717] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='end_bringup' elapsed_ticks=2098800 elapsed_us=1049 total_ticks=2489820894 total_us=1244910
[57175902630] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[58116867732] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=3439402527 elapsed_us=1719701
[58118187039] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[58187566404] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0[58201149699] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] ENTER_USER: TID=5 entry_pc=200000 user_sp=800000
] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[58464117888] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=0
[58491440106] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=20
[58534731783] [[34mDEBUG[0m] [sprout] [CPU0] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7fef70 rip=0x20414c rflags=0x202
[58544560569] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[58561817358] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: About to create Supervisor...
[58563811779] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Listing /bin directory...
[58762009218] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Parsed command line: '    '
[58781801826] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: FORCING display=bootfb for diagnostic test!
[58784936859] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Supervisor created, calling run_forever...
[58786918707] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[58788316059] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[58790027373] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[58854630945] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU0] BootFs: EXACT match for 'bin/sh' at index 4
[58969680264] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU0] BootFs: EXACT match for 'bin/sh' at index 4
[59011001181] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'sh' (len=117640, base=0x200000)
[59012885877] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[59035660332] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x216000 filesz=0 memsz=0 align=1
[59051993649] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /bin/sh
[59054366448] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb009c500 arg=0xffffffffb006f660
[59083950486] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=6, applying inserts
[59085221118] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[59086450203] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=6
[59087629392] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[59088538707] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
) stdout=Fd(4) stderr=Fd(5)
[59112331674] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SETUP_STDIO: stdin=Fd(3[59167419465] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 6
[59190194184] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Task 6 woken, restoring IRQs
[59191511841] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Done for /bin/sh
[59200784346] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[59212261878] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[59215148982] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 250ms so the prompt can take the foreground
[59215797894] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=6 entry_pc=200000 user_sp=800000
[59219731428] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=0
[59232355479] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed[59235131802] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=100
_ticks=33858
[59278731534] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'etc/motd' at index 105
[59286558078] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[59288198706] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
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
[59325135309] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'etc/profile' at index 104
[59424724326] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[59431277103] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [59461749303] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[59462976969] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[?25h[60361042104] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[60365346195] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[60382414191] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=4096 port=0xffffffffb0071470
[60386189490] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(0)
[60443718324] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=1 port_id=PortId(0) mode=Write
[60446836758] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=2 port_id=PortId(0) mode=Read
[60470825118] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/cambium' (len=124728, base=0x200000)
[60473096673] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[60527645772] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x217000 filesz=0 memsz=0 align=1
[60629592903] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb0081820 arg=0xffffffffb0071600
[60632946297] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[60633915705] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=7
[60634979856] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[60654047553] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 7 → task '/bin/cambium' (pid=7 from boot module)
[60677803956] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned cambium (PID=7)
[60680802336] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[60726086190] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[60727407642] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=7 entry_pc=200000 user_sp=800000
[60729500766] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=0
[60730728333] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/iso9660d' (len=80648, base=0x200000)
[60745118280] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=100
[60762987450] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[60803124063] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20e000 filesz=0 memsz=0 align=1
[60983533314] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb00b5440 arg=0xffffffffb0071600
[60999403839] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[61000592829] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=8
[61001789376] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[61053266340] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 8 → task '/bin/iso9660d' (pid=8 from boot module)
[61057107507] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned iso9660d (PID=8)
[61069167258] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[61070528178] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Deferring netd until /dev/net/virtio0/rx is ready...
[61081731744] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=8 entry_pc=200000 user_sp=800000
[61084544169] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=0
[61101162705] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=100
[61117681944] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[61243668519] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Task 9 (user thread) assigned to CPU 3
[61260974544] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[61262151027] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=9
[61263402816] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[61265737995] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Sending post-unlock Resched IPI to CPU 3 for task 9
[61270236918] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Running registration + health supervision loop
[61274166195] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=0
[61286368737] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU0] BRIDGE: handle=2 -> fd=3 node=0xffffffffb0069330 port=0xffffffffb0071470
[61291031670] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Bridged resp_port 2 -> FD 3 for task 'cambium'
[61309403760] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=20
[61313787546] [[32mINFO [0m] [sprout::supervisor] [CPU3] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[61363054830] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ahci_disk' at index 57
[61364993316] [[34mDEBUG[0m] [iso9660d] [CPU2] iso9660d: starting ISO9660 VFS provider
[61480862982] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[62624709246] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[62638983825] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ata_disk' at index 58
[63110210889] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.ata' start='thingos_driver_start_safe'
[63118641894] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/chime' at index 63
[63619364028] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[63623460648] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/display_bootfb' at index 41
[64001715558] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[64030058235] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/display_virtio_gpu' at index 42
[64492454433] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[64499330907] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/hdaudio' at index 61
[64880585451] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[64897871643] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/pci_stubd' at index 62
[65246675472] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[65250910791] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ps2_kbd' at index 3
[65696207214] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[65701228428] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ps2_mouse' at index 40
[66046997148] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[66060869886] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/rtc_cmos' at index 2
[66206536836] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[66208038336] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
l[66379400703] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[66380762316] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
s[66554043765] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[66555263841] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok

[66586757589] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[66592005381] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/ls' at index 5
[66627480216] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[66641889864] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/rtl8168d' at index 45
[66688149066] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/ls' at index 5
[66723101313] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: Loading module 'ls' (len=62192, base=0x200000)
[66725132298] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[66757155234] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: TLS block: tp=0x20b000 filesz=0 memsz=0 align=1
[66783811149] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Starting Phase 1 for /bin/ls
[66788264730] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb01ed340 arg=0xffffffffb008c2c0
[66793259379] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Phase 1 complete, ID=10, applying inserts
[66794316666] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[66795226938] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=10
[66796347156] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[66797330292] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[66800081403] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[66829001085] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Phase 2 complete, waking task 10
[66831194430] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Task 10 woken, restoring IRQs
[66832174563] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Done for /bin/ls
[66873489177] [[34mDEBUG[0m] [sh] [CPU3] sh: foreground handoff start job_id=1 pgid=10 shell_pgid=5 cmd='ls'
[66881625987] [[34mDEBUG[0m] [sh] [CPU3] sh: foreground handoff done job_id=1 pgid=10 cmd='ls'
[66883245825] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[66884371950] [[34mDEBUG[0m[] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[?25l66850493787] [[34mDEBUG[0m] [sh] [CPU3] sh: spawned '/bin/ls' pid=10 idx=0 background=false pending_pgid=0
[66858531069] [[34mDEBUG[0m] [sh] [CPU3] sh: setpgid leader pid=10 -> pgid=10
[67039595997] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[67051893084] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=10 entry_pc=200000 user_sp=800000
[67068614877] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=0
[67071138420] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=100
[67136177295] [[34mDEBUG[0m] [ls] [CPU1] ls: listing path '.'
[67590986100] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=10 fd=1 starting copyin
[67592283528] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=10 fd=1 copyin ok
[34mbin[0m  [34mboot[0m  [34mdata[0m  [34mdev[0m  [34mdrivers[0m  [34metc[0m  [34mlib[0m  [34mmnt[0m  motd  [34mproc[0m  [34mrun[0m  [34mservices[0m  [34msession[0m  [34mshare[0m  [34msys[0m  [34mtmp[0m  version  
[67655374248] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[67673936418] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/virtio_netd' at index 44
[67730329524] [[34mDEBUG[0m] [kernel::inbox] [CPU3] inbox::close: inbox closed, 0 messages dropped on close
[67785442395] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[67786790808] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[?25h[67805397693] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[67806683934] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [67824167169] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[67825564884] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[?25h[68232145245] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[68237954532] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/virtio_sound' at index 60
[68761177551] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[68764864773] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[69426099501] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=3 starting copyin
[69438377712] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=3 copyin ok
[69459626148] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/virtio_netd' at index 44
[69517694697] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'virtio_netd' (len=106272, base=0x200000)
[69519860982] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[69557291793] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x214000 filesz=0 memsz=0 align=1
[69593277534] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x20277c + bias 0x0 = PC 0x20277c
[69595134312] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/virtio_netd
[69596958948] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb00f7f60 arg=0xffffffffb00bcf60
[69600337026] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=11, applying inserts
[69601404510] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[69602367219] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=11
[69603672897] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[69604460046] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[69613275897] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[69631163481] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 11
[69632898456] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 11 woken, restoring IRQs
[69633892779] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/virtio_netd
[69648447726] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x20277c SP=0x800000 ARG0=0x3
[69650023773] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=11 entry_pc=20277c user_sp=800000
[69661523217] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=11 buf_len=0
[69692122731] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/virtio_netd for pci-0000:00:02.0 (entry='thingos_driver_start_safe', pid=11)
[69716958135] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=11 buf_len=100
[69741386649] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=4 starting copyin
[69742907355] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=4 copyin ok
[69749546790] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ahci_disk' at index 57
[69835737906] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'ahci_disk' (len=74072, base=0x200000)
[69837661674] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[69910825677] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x20e000 filesz=0 memsz=0 align=1
[69933774273] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[69935409984] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/ahci_disk
[70028519352] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[70030022700] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[70040533563] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb0110740 arg=0xffffffffb00bc600
l[70101218781] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=12, applying inserts
[70102471560] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[70106871219] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=12
[70119266283] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[70124508993] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[70130642373] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[70149537084] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 12
[70172904483] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 12 woken, restoring IRQs
[70189713000] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/ahci_disk
[70195162422] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/ahci_disk for pci-0000:00:1f.2 (entry='thingos_driver_start_safe', pid=12)
[70221492330] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x4
[70222897701] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=12 entry_pc=201000 user_sp=800000
[70231409391] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=12 buf_len=0
[70244165706] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=5 starting copyin
[70245568074] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=5 copyin ok
[70250400627] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=12 buf_len=100
[70251878037] [[34mDEBUG[0m] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[70254293208] [[34mDEBUG[0m] [virtio_netd] [CPU2] VIRTIO_NETD: Initializing hardware driver...
[70268953557] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ata_disk' at index 58
[70285704621] [[34mDEBUG[0m] [ahci_disk] [CPU3] AHCI: Starting AHCI/SATA disk driver (boot_fd=4)
[70293540471] [[34mDEBUG[0m] [virtio_netd::driver] [CPU2] VirtIO-NET: Claiming NIC device at /sys/devices/pci-0000:00:02.0
[70295842980] [[34mDEBUG[0m] [virtio::device] [CPU2] VirtIO: device::new(/sys/devices/pci-0000:00:02.0)
[70310406969] [[34mDEBUG[0m] [virtio::device] [CPU2] VirtIO: claiming '/sys/devices/pci-0000:00:02.0'...
[70315155867] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'ata_disk' (len=61712, base=0x200000)
[70316998785] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[70330707117] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 'pci-0000:00:02.0' (handle 0)
[70334541783] [[34mDEBUG[0m] [virtio::device] [CPU2] VirtIO: claimed, handle=0
[70350211371] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x20b000 filesz=0 memsz=0 align=1
[70389586542] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: task 12 claimed device 'pci-0000:00:1f.2' (handle 1)
[70401616197] [[34mDEBUG[0m] [ahci_disk] [CPU3] AHCI: Claimed PCI device '/sys/devices/pci-0000:00:1f.2' handle=1
[70409857089] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[70411585068] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/ata_disk
[70416673437] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: mapping BAR5 (phys=0x80880000, size=0x1000) for task 12
[70421949279] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb0124740 arg=0xffffffffb00ce2e0
[70427379627] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=13, applying inserts
[70428464238] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[70429194561] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[70434023979] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=13
[70435490202] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[70469337609] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[70472217585] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[70474434921] [[34mDEBUG[0m] [virtio::device] [CPU2] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/common_bar -> '4' (n=2)
[70481388351] [[34mDEBUG[0m] [ahci_disk] [CPU3] AHCI: Mapped ABAR at 0x10000000
[70504795779] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 13
[70515542328] [[34mDEBUG[0m] [ahci_disk] [CPU3] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[70528774602] [[34mDEBUG[0m] [virtio::device] [CPU2] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/common_offset -> '0x0' (n=4)
[70530624516] [[34mDEBUG[0m] [ahci_disk] [CPU3] AHCI: Ports implemented: 0x3f
[70532142912] [[34mDEBUG[0m] [ahci_disk] [CPU3] AHCI: DMA virt=0x20c000
[70545688752] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 13 woken, restoring IRQs
[70546833027] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/ata_disk
[70593943035] [[34mDEBUG[0m] [ahci_disk] [CPU3] AHCI: DMA phys=0x235b000
[70596593958] [[34mDEBUG[0m] [ahci_disk] [CPU3] AHCI: Probing port 0...
[70598399190] [[34mDEBUG[0m] [ahci_disk] [CPU3] AHCI: Port 0 - no device
[70599612435] [[34mDEBUG[0m] [ahci_disk] [CPU3] AHCI: Probing port 1...
[70600691073] [[34mDEBUG[0m] [ahci_disk] [CPU3] AHCI: Port 1 - no device
[70609647471] [[34mDEBUG[0m] [ahci_disk] [CPU3] AHCI: Probing port 2...
[70625108796] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x5
[70626628710] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=13 entry_pc=201000 user_sp=800000
[70627403451] [[34mDEBUG[0m] [virtio::device] [CPU2] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/notify_bar -> '4' (n=2)
[70635637347] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=13 buf_len=0
[70643010537] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=13 buf_len=100
[70671307344] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[70673633448] [[34mDEBUG[0m] [ahci_disk] [CPU3] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[70679873715] [[34mDEBUG[0m] [virtio::device] [CPU2] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/notify_offset -> '0x3000' (n=7)
[70697548383] [[34mDEBUG[0m] [kernel::ipc] [CPU3] CREATE_PORT: capacity=4096 port=0xffffffffb00d0170
[70699353945] [[34mDEBUG[0m] [kernel::ipc] [CPU3] CREATE_PORT: appended id=PortId(1)
[70717079829] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU3] ALLOC_HANDLE: handle=3 port_id=PortId(1) mode=Write
[70720835757] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU3] ALLOC_HANDLE: handle=4 port_id=PortId(1) mode=Read
[70729192875] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing primary port (0x1F0)...
[70729952403] [[34mDEBUG[0m] [virtio::device] [CPU2] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/notify_multiplier -> '4' (n=2)
[70739685522] [[34mDEBUG[0m] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[70785140250] [[34mDEBUG[0m] [virtio::device] [CPU2] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/device_bar -> '4' (n=2)
[70880908263] [[34mDEBUG[0m] [virtio::device] [CPU2] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/device_offset -> '0x2000' (n=7)
[70889204892] [[34mDEBUG[0m] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[70896416019] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: mapping BAR4 (phys=0xc000004000, size=0x4000) for task 11
[70898857854] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x10001000
[70901538939] [[34mDEBUG[0m] [virtio::device] [CPU2] VirtIO: common_cfg at 0x10001000
[70903182174] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Probing secondary port (0x170)...
[70910315058] [[34mDEBUG[0m] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x10004000
[70919593800] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=12 fd=5 starting copyin
[70920514137] [[34mDEBUG[0m] [virtio::device] [CPU2] VirtIO: device_cfg = Some(268447744)
[70921144503] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=12 fd=5 copyin ok
[70922420745] [[34mDEBUG[0m] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[70927810833] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: DMA alloc 1 pages phys=0x241b000 -> user_va=0x10005000
[70939929720] [[34mDEBUG[0m] [ahci_disk] [CPU3] AHCI: Registered ATAPI block device port=2 model='                                        ' rpc_port=3
[70944928758] [[34mDEBUG[0m] [ahci_disk] [CPU3] AHCI: Probing port 3...
[70968806997] [[34mDEBUG[0m] [ahci_disk] [CPU3] AHCI: Port 3 - no device
[70974383634] [[34mDEBUG[0m] [ahci_disk] [CPU3] AHCI: Probing port 4...
[70975575990] [[34mDEBUG[0m] [ahci_disk] [CPU3] AHCI: Port 4 - no device
[70976776695] [[34mDEBUG[0m] [ahci_disk] [CPU3] AHCI: Probing port 5...
[70977837711] [[34mDEBUG[0m] [ahci_disk] [CPU3] AHCI: Port 5 - no device
[70979183385] [[34mDEBUG[0m] [ahci_disk] [CPU3] AHCI: Found 1 SATA disk(s)
[70980511404] [[34mDEBUG[0m] [ahci_disk] [CPU3] AHCI: Entering RPC service loop
[70982327064] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU3] BRIDGE: handle=4 -> fd=5 node=0xffffffffb00ce410 port=0xffffffffb00d0170
[71005625823] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=65536 port=0xffffffffb00d1530
[71023913037] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(2)
[71027447865] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(2) mode=Write
[71028734040] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=4 port_id=PortId(2) mode=Read
[71054418963] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[71055791268] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[71066960019] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[71068420599] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: Entering RPC service loop
s[71070621171] [[32mINFO [0m] [ata_disk] [CPU1] ATA_DISK: No active devices to service
[71072283414] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[71073533916] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[71076304563] [[34mDEBUG[0m] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x10005000 phys=0x241b000

[71078562918] [[34mDEBUG[0m] [virtio::device] [CPU2] VirtIO: device::new complete
[71079241728] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[71082668679] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/ata_disk for isa-01f0 (entry='thingos_driver_start_safe', pid=13)
[71083461009] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/ls' at index 5
[71089552974] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/ls' at index 5
[71096374008] [[34mDEBUG[0m] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[71099646321] [[34mDEBUG[0m] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[71111491605] [[34mDEBUG[0m] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[71121114999] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: Loading module 'ls' (len=62192, base=0x200000)
[71129707440] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[71161864785] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: TLS block: tp=0x20b000 filesz=0 memsz=0 align=1
[71172993507] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Starting Phase 1 for /bin/ls
[71182596639] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb015aa40 arg=0xffffffffb00d1b00
[71198763702] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Phase 1 complete, ID=14, applying inserts
[71200145907] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[71216938419] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=14
[71221630194] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[71223173769] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[71226865314] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[71243141409] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: DMA alloc 4 pages phys=0x243d000 -> user_va=0x10006000
[71244067785] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Phase 2 complete, waking task 14
[71249940036] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Task 14 woken, restoring IRQs
[71255094834] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Done for /bin/ls
[71259440901] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[71261012790] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=14 entry_pc=200000 user_sp=800000
[71263157823] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=14 buf_len=0
[71265415188] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=14 buf_len=100
[71273073762] [[34mDEBUG[0m] [sh] [CPU3] sh: spawned '/bin/ls' pid=14 idx=0 background=false pending_pgid=0
[71275233084] [[34mDEBUG[0m] [sh] [CPU3] sh: setpgid leader pid=14 -> pgid=14
[71281160214] [[34mDEBUG[0m] [sh] [CPU3] sh: foreground handoff start job_id=2 pgid=14 shell_pgid=5 cmd='ls'
[71282495625] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=6 starting copyin
[71284687683] [[34mDEBUG[0m] [sh] [CPU3] sh: foreground handoff done job_id=2 pgid=14 cmd='ls'
[71318157735] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=6 copyin ok
[71333148777] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[71334378885] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[?25l[71352824004] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=6 starting copyin
[71354079225] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=6 copyin ok
[71393665728] [[34mDEBUG[0m] [ls] [CPU2] ls: listing path '.'
[71435768316] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=6 starting copyin
[71437062411] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=6 copyin ok
[71981718567] [[34mDEBUG[0m] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[72002394684] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: DMA alloc 4 pages phys=0x2482000 -> user_va=0x1000a000
[72014963328] [[34mDEBUG[0m] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[72028501842] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: DMA alloc 16 pages phys=0x2486000 -> user_va=0x1000e000
[72059078553] [[34mDEBUG[0m] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[72061133661] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: DMA alloc 1 pages phys=0x2496000 -> user_va=0x1001e000
[72079817568] [[34mDEBUG[0m] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[72085935306] [[34mDEBUG[0m] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[72094317900] [[34mDEBUG[0m] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[72100126197] [[34mDEBUG[0m] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[72102026139] [[34mDEBUG[0m] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[72228159333] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=14 fd=1 starting copyin
[72232228563] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=14 fd=1 copyin ok
[34mbin[0m  [34mboot[0m  [34mdata[0m  [34mdev[0m  [34mdrivers[0m  [34metc[0m  [34mlib[0m  [34mmnt[0m  motd  [34mproc[0m  [34mrun[0m  [34mservices[0m  [34msession[0m  [34mshare[0m  [34msys[0m  [34mtmp[0m  version  
[72274048572] [[34mDEBUG[0m] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[72287608734] [[34mDEBUG[0m] [virtio_netd] [CPU2] VIRTIO_NETD: Initial link state is UP
[72317932698] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=65536 port=0xffffffffb00d22b0
[72319152378] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(3)
[72320520096] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(3) mode=Write
[72321594411] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=4 port_id=PortId(3) mode=Read
[72352786704] [[34mDEBUG[0m] [virtio_netd] [CPU2] VIRTIO_NETD: Created VFS provider port
[72421083933] [[34mDEBUG[0m] [kernel::inbox] [CPU3] inbox::close: inbox closed, 0 messages dropped on close
[72502627791] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[72504189780] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[?25h[72522702087] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[72523779438] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user thread) assigned to CPU 3
[72536564826] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Applying 1 deferred inserts
[72555186627] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserting TID=15
[72556870122] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserts applied
[72558005190] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] SCHED: Sending post-unlock Resched IPI to CPU 3 for task 15
[72564746760] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [72568960827] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[72570081078] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[?25h[72599578920] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=15 buf_len=0
[72601626603] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=15 buf_len=100
[72614938869] [[34mDEBUG[0m] [virtio_netd] [CPU3] VIRTIO_NETD: Entering VFS provider service loop at /dev/net/virtio0
[72654423303] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=262400 port=0xffffffffb00d1ed0
[72656192994] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(4)
[72671035008] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=5 port_id=PortId(4) mode=Write
[72705579804] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] vfs: mounted userland provider at /dev/net/virtio0 (flags: 0x0)
[72711000219] [[34mDEBUG[0m] [virtio_netd] [CPU2] VIRTIO_NETD: Mounted at /dev/net/virtio0
[72713085852] [[34mDEBUG[0m] [virtio_netd] [CPU2] VIRTIO_NETD: Provider thread live at /dev/net/virtio0
[72998249841] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU3] VIRTIO_NETD: lookup 'rx'
[73010922864] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU3] VIRTIO_NETD: lookup 'rx' -> handle=5
[73132412463] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=65536 port=0xffffffffb00d2810
[73133784504] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(5)
[73135012599] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=5 port_id=PortId(5) mode=Write
[73136229573] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=6 port_id=PortId(5) mode=Read
[73138649826] [[32mINFO [0m] [sprout::supervisor] [CPU1] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[73152662352] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module '/bin/netd' (len=10813488, base=0x200000)
[73154217774] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[76117350969] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[76136080977] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
l[76141069389] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[76142322795] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
s[76278870195] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[76287555036] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok

[76299814437] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[76304207529] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/ls' at index 5
[76334519316] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/ls' at index 5
[76365939045] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: Loading module 'ls' (len=62192, base=0x200000)
[76368048306] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[76389391419] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: TLS block: tp=0x20b000 filesz=0 memsz=0 align=1
[76413519897] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Starting Phase 1 for /bin/ls
[76415730039] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb01aaa40 arg=0xffffffffb00d29e0
[76442382390] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Phase 1 complete, ID=16, applying inserts
[76443771657] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[76444599858] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=16
[76445708988] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[76446511812] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[76449186396] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[76475467926] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Phase 2 complete, waking task 16
[76477253622] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Task 16 woken, restoring IRQs
[76478270517] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Done for /bin/ls
[76505686653] [[34mDEBUG[0m] [sh] [CPU3] sh: spawned '/bin/ls' pid=16 idx=0 background=false pending_pgid=0
[76508633025] [[34mDEBUG[0m] [sh] [CPU3] sh: setpgid leader pid=16 -> pgid=16
[76510183695] [[34mDEBUG[0m] [sh] [CPU3] sh: foreground handoff start job_id=3 pgid=16 shell_pgid=5 cmd='ls'
[76512447825] [[34mDEBUG[0m] [sh] [CPU3] sh: foreground handoff done job_id=3 pgid=16 cmd='ls'
[76520912721] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[76522023171] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[?25lls[77950117839] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0xc3e000 filesz=0 memsz=0 align=1

[78018879774] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb01cea40 arg=0xffffffffb00bcfc0
[78022502019] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[78023501853] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=17
[78024694209] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[78075625419] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: TID 17 → task '/bin/netd' (pid=17 from boot module)
[78101440560] [[34mDEBUG[0m] [sprout::supervisor] [CPU1] SPROUT: Spawned netd (PID=17)
[78111317823] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[78112520937] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=17 entry_pc=200000 user_sp=800000
[78120586533] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=17 buf_len=0
[78135933348] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=17 buf_len=100
[78140849424] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[78142412370] [[34mDEBUG[0m] [netd] [CPU2] NETD: binary v2 (with heap storage) starting...
[78145308681] [[34mDEBUG[0m] [netd] [CPU2] NETD: main entry point, arg=0
[78256113177] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[78257509572] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=16 entry_pc=200000 user_sp=800000
[78259709649] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=16 buf_len=0
[78261684006] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=16 buf_len=100
[78288728364] [[34mDEBUG[0m] [netd] [CPU2] NETD: get_args starting...
[78291429117] [[34mDEBUG[0m] [netd] [CPU2] NETD: argv_get len=17
[78307541928] [[34mDEBUG[0m] [netd] [CPU2] NETD: get_args returning 0 args
[78319490667] [[34mDEBUG[0m] [netd] [CPU2] NETD: Starting network service (Phase 3 — /net/ VFS provider)
[78321687180] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[78323710641] [[34mDEBUG[0m] [netd] [CPU2] NETD: NIC probe loop starting (round=0)
[78338235987] [[34mDEBUG[0m] [netd] [CPU2] NETD: Probing /dev/net/virtio0...
[78422674473] [[34mDEBUG[0m] [ls] [CPU1] ls: listing path '.'
[78425560521] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU3] VIRTIO_NETD: lookup 'rx'
[78427316550] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU3] VIRTIO_NETD: lookup 'rx' -> handle=5
[78452726352] [[34mDEBUG[0m] [netd] [CPU2] NETD: Found /dev/net/virtio0/rx, opening others...
[78504344457] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU3] VIRTIO_NETD: lookup 'tx'
[78505940634] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU3] VIRTIO_NETD: lookup 'tx' -> handle=6
[78650728959] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU3] VIRTIO_NETD: lookup 'events'
[78652768425] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU3] VIRTIO_NETD: lookup 'events' -> handle=8
[78722002524] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU3] VIRTIO_NETD: lookup 'mac'
[78723795678] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU3] VIRTIO_NETD: lookup 'mac' -> handle=3
[78894025188] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU3] VIRTIO_NETD: read mac
[79025087592] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU3] VIRTIO_NETD: lookup 'mtu'
[79025789139] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=16 fd=1 starting copyin
[79039916670] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU3] VIRTIO_NETD: lookup 'mtu' -> handle=4
[79040505522] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=16 fd=1 copyin ok
[34mbin[0m  [34mboot[0m  [34mdata[0m  [34mdev[0m  [34mdrivers[0m  [34metc[0m  [34mlib[0m  [34mmnt[0m  motd  [34mproc[0m  [34mrun[0m  [34mservices[0m  [34msession[0m  [34mshare[0m  [34msys[0m  [34mtmp[0m  version  
[79094837382] [[34mDEBUG[0m] [kernel::inbox] [CPU3] inbox::close: inbox closed, 0 messages dropped on close
[79121129043] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[79122442080] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[?25h[79135439295] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[79136647062] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [79154060436] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[79155445281] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[?25h[79159088052] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[79160288460] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
l[79175997714] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[79177059918] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
s[79179732225] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [[CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[79180693845] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok

[79183513926] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
79215043941] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/ls' at index 5
[79227306312] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/ls' at index 5
[79247080077] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: Loading module 'ls' (len=62192, base=0x200000)
[79254748617] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[79296700626] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: TLS block: tp=0x20b000 filesz=0 memsz=0 align=1
[79322862927] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Starting Phase 1 for /bin/ls
[79324381224] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb020c640 arg=0xffffffffb00d2c00
[79326997068] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Phase 1 complete, ID=18, applying inserts
[79327892358] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[79342428033] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=18
[79343862312] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[79344566037] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[79353903816] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[79361054916] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Phase 2 complete, waking task 18
[79374636726] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Task 18 woken, restoring IRQs
[79376186901] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Done for /bin/ls
[79391812401] [[34mDEBUG[0m] [sh] [CPU3] sh: spawned '/bin/ls' pid=18 idx=0 background=false pending_pgid=0
[79394209785] [[34mDEBUG[0m] [sh] [CPU3] sh: setpgid leader pid=18 -> pgid=18
[79395709668] [[34mDEBUG[0m] [sh] [CPU3] sh: foreground handoff start job_id=4 pgid=18 shell_pgid=5 cmd='ls'
[79397696532] [[34mDEBUG[0m] [sh] [CPU3] sh: foreground handoff done job_id=4 pgid=18 cmd='ls'
[79398886215] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[79419294900] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[?25l[79437108960] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[79446697209] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=18 entry_pc=200000 user_sp=800000
[79450891410] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=18 buf_len=0
[79473070314] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=18 buf_len=100
[79545486438] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU3] VIRTIO_NETD: read mtu
[79731199218] [[34mDEBUG[0m] [ls] [CPU1] ls: listing path '.'
[79734146316] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU3] VIRTIO_NETD: lookup 'status'
[79735783413] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU3] VIRTIO_NETD: lookup 'status' -> handle=2
[79849761816] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU3] VIRTIO_NETD: read status
[79910120103] [[34mDEBUG[0m] [netd] [CPU2] NETD: Opened VFS NIC device at /dev/net/virtio0 (rx=4, tx=5, events=6, mtu=1500, link=up)
[79918095939] [[34mDEBUG[0m] [netd] [CPU2] NETD: Driver online at /dev/net/virtio0 — MAC 52:54:00:12:34:56  MTU 1500
[79937519838] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=65536 port=0xffffffffb00d9a70
[79949241999] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(6)
[79950730167] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(6) mode=Write
[79952090031] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=4 port_id=PortId(6) mode=Read
[79962815262] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=262400 port=0xffffffffb00d9bf0
[79974605304] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(7)
[79975970085] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(7) mode=Write
[79992327591] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] vfs: mounted userland provider at /net (flags: 0x0)
[80010342159] [[34mDEBUG[0m] [netd::vfs_provider] [CPU2] NetVfsProvider: mounted at /net (port w=3 r=4)
[80012902761] [[34mDEBUG[0m] [netd] [CPU2] NETD: Running DHCP...
[80018989710] [[34mDEBUG[0m] [netd::dhcp] [CPU2] DHCP: Starting discovery...
[80132253498] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=17 fd=5 starting copyin
[80143871016] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=17 fd=5 copyin ok
[80157551067] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=18 fd=1 starting copyin
[80158896213] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=18 fd=1 copyin ok
[34mbin[0m  [34mboot[0m  [34mdata[0m  [34mdev[0m  [34mdrivers[0m  [34metc[0m  [34mlib[0m  [34mmnt[0m  motd  [34mproc[0m  [34mrun[0m  [34mservices[0m  [34msession[0m  [34mshare[0m  [34msys[0m  [34mtmp[0m  version  
[80223594429] [[34mDEBUG[0m] [kernel::inbox] [CPU3] inbox::close: inbox closed, 0 messages dropped on close
[80239034799] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[80240459013] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[?25h[80244575301] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[80254967991] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [80259278121] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[80260508658] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[?25h[81453295110] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[81464739774] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
l[81643061379] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[81644399694] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
s[81780786186] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[81782162682] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok

[81785868351] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[81789235539] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/ls' at index 5
[81806127711] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/ls' at index 5
[81839633502] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: Loading module 'ls' (len=62192, base=0x200000)
[81841586277] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[81862979022] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: TLS block: tp=0x20b000 filesz=0 memsz=0 align=1
[81872249514] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Starting Phase 1 for /bin/ls
[81895540749] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb021c640 arg=0xffffffffb00d2c80
[81898691919] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Phase 1 complete, ID=19, applying inserts
[81899672547] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[81900573678] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=19
[81901833981] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[81902675217] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[81906100221] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[81935569188] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Phase 2 complete, waking task 19
[81937277928] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Task 19 woken, restoring IRQs
[81938310993] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Done for /bin/ls
[81940951587] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[81949502052] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=19 entry_pc=200000 user_sp=800000
[81951871749] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=19 buf_len=0
[81953610288] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=19 buf_len=100
[81977393223] [[34mDEBUG[0m] [sh] [CPU3] sh: spawned '/bin/ls' pid=19 idx=0 background=false pending_pgid=0
[81980394012] [[34mDEBUG[0m] [sh] [CPU3] sh: setpgid leader pid=19 -> pgid=19
[81981987714] [[34mDEBUG[0m] [sh] [CPU3] sh: foreground handoff start job_id=5 pgid=19 shell_pgid=5 cmd='ls'
[81998021028] [[34mDEBUG[0m] [sh] [CPU3] sh: foreground handoff done job_id=5 pgid=19 cmd='ls'
[82000431381] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[82001521767] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[?25l[82113820206] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU3] VIRTIO_NETD: write tx len=308
[82132654824] [[34mDEBUG[0m] [virtio_netd::driver] [CPU3] VirtIO-NET: TX complete after 0 iterations
[82158783696] [[34mDEBUG[0m] [ls] [CPU1] ls: listing path '.'
[82166067192] [[34mDEBUG[0m] [virtio_netd::driver] [CPU3] VirtIO-NET: RX frame! desc=0 len=600
[82390334268] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU3] VIRTIO_NETD: read rx queued=1
[82798604856] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=17 fd=5 starting copyin
[82800087414] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=17 fd=5 copyin ok
[82868689893] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU3] VIRTIO_NETD: write tx len=320
[82888448577] [[34mDEBUG[0m] [virtio_netd::driver] [CPU3] VirtIO-NET: TX complete after 0 iterations
[82895129559] [[34mDEBUG[0m] [virtio_netd::driver] [CPU3] VirtIO-NET: RX frame! desc=1 len=600
[83072892672] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU3] VIRTIO_NETD: read rx queued=1
[83353629843] [[34mDEBUG[0m] [netd::dhcp] [CPU2] DHCP: Configuration received
[83356887042] [[34mDEBUG[0m] [netd] [CPU2] NETD: DHCP — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[83358570537] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
[83359780944] [[34mDEBUG[0m] [netd] [CPU2] NETD: entering VFS service loop
[83375628567] [[34mDEBUG[0m] [netd] [CPU2] NETD: creating SocketApi...
[83377890222] [[34mDEBUG[0m] [netd] [CPU2] NETD: allocating sockets_storage...
[83379638661] [[34mDEBUG[0m] [netd] [CPU2] NETD: pushing socket storage 0...
[83393141568] [[34mDEBUG[0m] [netd] [CPU2] NETD: pushing socket storage 64...
[83394654585] [[34mDEBUG[0m] [netd] [CPU2] NETD: pushing socket storage 128...
[83395982703] [[34mDEBUG[0m] [netd] [CPU2] NETD: pushing socket storage 192...
[83397447540] [[34mDEBUG[0m] [netd] [CPU2] NETD: creating SocketSet...
[83398701netd111] [[34mDEBUG[0m] [netd] [CPU2] NETD: getting link state...
[83399824860] [[34mDEBUG[0m] [netd] [CPU2] NETD: scanning NIC units...
[83414256948] [[34mDEBUG[0m] [] [CPU2] NETD: scanning for registered NIC units...
[83477578965] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU3] VIRTIO_NETD: lookup 'rx'
[83479119339] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU3] VIRTIO_NETD: lookup 'rx' -> handle=5
[83855387583] [[34mDEBUG[0m] [netd] [CPU2] NETD: NIC scan complete: [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]
[83857646697] [[34mDEBUG[0m] [netd] [CPU2] NETD: NIC units scanned.
[8387
```
</details>
