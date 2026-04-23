# ✅ Scenario: echo, pipe, and wc work together

> Last run: 2026-04-22 19:53:46

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8716ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 2103ms | - [📜](./02/serial.log) - |
| 3 | And I type "echo hello world | wc -w" on the serial console | ✅ | 2288ms | - [📜](./03/serial.log) - |
| 4 | Then the serial output should contain "2" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01Hk_top=0xffffffffb001f980 arg=0x0
[27058034817] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[27081002223] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[27082037664] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[27103246929] [[34mDEBUG[0m] [kernel] [CPU0] Initializing VFS...
[27107157594] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='set_cmdline' elapsed_ticks=3031908 elapsed_us=1515 total_ticks=3754245 total_us=1877
[27127788699] [[34mDEBUG[0m] [kernel::vfs] [CPU0] VFS: Created /dev/display directory
[27143019816] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted union filesystem at / (root)
[27145972161] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted devfs at /dev
[27147079707] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted procfs at /proc
[27148091817] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted sysfs at /sys
[27152096895] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /tmp
[27152940144] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /run
[27153662415] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /services
[27154362543] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /session
[27155389503] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /data
[27156180051] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='vfs_init' elapsed_ticks=48153732 elapsed_us=24076 total_ticks=53053473 total_us=26526
[27157216119] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[27178094394] [[34mDEBUG[0m] [kernel] [CPU0] Scanning PCI bus...
[27182195502] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x29c0 at 00:00.0 class=060000 id=1
[27201230034] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1050 at 00:01.0 class=030000 id=2
[27221943969] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1000 at 00:02.0 class=020000 id=3
[27227399892] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2918 at 00:1f.0 class=060100 id=4
[27232395762] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2922 at 00:1f.2 class=010601 id=5
[27235421895] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2930 at 00:1f.3 class=0c0500 id=6
[27238170135] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='scan_pci' elapsed_ticks=59558862 elapsed_us=29779 total_ticks=135044085 total_us=67522
[27239339061] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[27261071508] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='register_legacy_devices' elapsed_ticks=672606 elapsed_us=336 total_ticks=157928397 total_us=78964
[27262123977] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[27284297172] [[34mDEBUG[0m] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[27291975678] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[27292686795] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[27293816649] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[27294711642] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[27299136777] [[34mDEBUG[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[27302254881] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[27303183171] [[34mDEBUG[0m] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[27303859143] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[27305372688] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Registers initialized
[27306284544] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[27307258506] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[27308478813] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: All pins masked
[27310492011] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Init complete
[27345750201] [[34mDEBUG[0m] [bran::arch::x86_64::ioapic] [CPU0] LAPIC: calibrated 64101000 ticks/sec (delta=641010, ok=true) -> init_cnt=641010
[27351791280] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] LAPIC: calibrated [kernel timer (64101000 ticks/sec), in::sched] [CPU1]it_cnt=641 R010 for 100Hz
[27352559322] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='setup_preemption_timer' elapsed_ticks=67603437 elapsed_us=33801 total_ticks=249446076 total_us=124723
[27353499657] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[27376892730] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[27377878209] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[273791363EGIS01] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[27381258993] [[34mDEBUG[0m] [bran::arch::x86_64] [CPTRU0] SMP: Starting CPU 1 (APIC 1)
[27411695718] [[34mDEBUG[0m] [bran::arch::x8Y: I6_64] [CPU0] SMP: CPU 1 (APIC 1)nserting is online
[274131594 T33] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2ID (APIC 2)
[27414996708] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] SMP: CPU 1 initializing preemption timer (vec=32 init_cnt=641010)
[27417371520] [[34mDEBUG[0m] [kernel] [CPU1] SMP: kernel_secondary_entry arg_cpu=1 runtime_cpu=1
[27418436133] [[34mDEBUG[0m] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[=2
[27434017274202088281] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[274234788722] [[34mDEBUG[0m] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[27435479115] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: run_7] [schedule[r 34mDentrEBUG[y on CPU 1
[2740m36104168] [[34mDEBUG[0m] [] [kerkernenel::sched] [CPUl::tas1]k] [CPU1 S] SMP: bootstrap_cpu sMPtart on CPU 1: CPU 1 online (triggered by scheduler spawn)
[274252
[27437233065] [[34m9544DEBU7]G[0m] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle thread 2 and is now schedulable
[27438881613] [[34mDEBUG[0m] [bran::arc [h:[34:x86_64] [CPU2] SMP: CPU 2 initializing preemption timer (vec=32 minit_cnt=641010)
[27440122347] [[34mDEBUG[0m] [DEbran::arch::x86_64] BUG[0[CPU0] SMP: CPU 2 (APIC 2) is onlm] [ine
[27440924709] [[34mDEBUG[0m] [kernel] [CPU2] SMP: kernel_secondary_entrbry arg_cpu=2 runtime_cpu=2
[27441642657] [[34anm] SDEBUG[0m] ::archMP: [bran::arch:CPU:x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[27442058259] [[34mDEBUG[0m] [kernel] [CPU2] S 3 (APMPIC 3): Entering kernel is onli_seconnedary_ent
[274681449ry for CPU 2
[27454692870] [[34mDEBUG[0m] ::x86_657] [[34mDEBUG[0m] [bran::4::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80017090 kstack_top=0xffffffffb0032e40 arg=0x1
[ar27426811599]c [h::x86_64] [CPU3] SMP: CPU 3 initializing preemption timer (vec=32 init_cnt=641010)
[27470429943] [[34mDEBUG[[34mDE0m] [kernel] [CPBUG[0mU3] [k] SMP: kerneernel::scl_secondary_entry arg_cpu=3 runthed:im:spawn] [[CPU1] SCe_cpu=3
[274724HED:61225] [[34mDEBUG[ Task 2 assigned to CPU0m] [kernel] [ 1
CPU3] SMP: [274Entekernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[27455951061] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU2] INIT KERNEL CTX: 309418entry=0xffffffff80017090 kstack_top=0xffffffffb00445a0 arg=0x2
[27457005576] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] SCHED: Task 3 assigned ring 46] [[34kernel_secondary_entry for CPU 3
[274742to CPU 2
[2799094] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[27484396203] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[27485156292] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[27487574433] [[34mDEB4587017UG[0m] 10[bran::arch::x86_64::task] mDEBUG[CPU3] INIT KERNEL CTX: entry=0xfff] [[34mDEBfffff80017090UG[0m] [ker kstack_top=0xffffffffb0055d00 ne[0m] [kernel::schl::sched] [CPU2] REGISTRY: Aarg=0x3
[27489088pply7eind] [g 1 deferreCPd inserts
U1] [27459298REGIS383] [[34mDEBUG[0m] [kernel::scTRY: Applying 2 deferred inserts
[274319677137] [[34mDEBUG[0m] [7] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=1
[27432570198] [[34mDEBUG[0m]kernel::sched::spawn] [CPU3] SCHED: Task 4 assignedhed to CP] U 3
[27489651717] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='start_secondary_cpus' elapsed_ticks=112509639 elapsed_us=56254 [CPU2] REGISTRY: Inserting TItoD=3ta
[27459960231] [[34mDEBUG[l_ticks=30m] [k85858440 ernel::stotal_us=ched] 19[CPU2]2929
[27 REGISTRY: Inserts applied49
[27460467527134] 4999[[3] [[34mDEBUG[04mDEBUG[0m]m] [kernel:: [kernel::ssched] [CPU2] SMP: Secondachry CPU 2 online!
[274609869ed]93] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: run_scheduler entry on CPU 2
[27461553966] [[34mDEBUG[0m] [kernel::task] [C [PU2] SCPU3] RMP: EGbootstrap_cpu start ISon CPU 2
[27462187434] [[34mDEBUG[0m] [kTRY: Applying 1 deferredernel::t inserasts
k] [CPU2] SM[27498957090] [P: CPU 2 bootstrapped with idle thread 3 and is now schedulable
[27465733911] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=4
[27500703285] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[27502460667] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[27503509968] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: run_scheduler entry on CPU 3
[27505181253] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: bootstrap_cpu start on CPU 3
[27506320710] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[27508490559] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle thread 4 and is now schedulable
[27602401728] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[27626315277] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Enumerating 108 boot modules...
[27628125624] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[27749126592] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[27750484839] [[34mDEBUG[0m] [kernel] [CPU0] Found init module: /bin/sprout (cmdline: 'init'), loading...
[27752098770] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/sprout' (len=85144, base=0x200000)
[27764403414] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[27781281000] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20f000 filesz=0 memsz=0 align=1
[27788384811] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='load_init_module' elapsed_ticks=36500442 elapsed_us=18250 total_ticks=685243383 total_us=342621
[27791152917] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='prepare_boot_registry' elapsed_ticks=1020591 elapsed_us=510 total_ticks=687342909 total_us=343671
[27794077971] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='map_boot_registry' elapsed_ticks=170181 elapsed_us=85 total_ticks=690978684 total_us=345489
[27795189444] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='activate_init_aspace' elapsed_ticks=167706 elapsed_us=83 total_ticks=692039007 total_us=346019
[27796182249] [[34mDEBUG[0m] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[27796690812] [[34mDEBUG[0m] [kernel] [CPU0] Spawning init process...
[27798651309] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb00661e0 arg=0xffffffffb001fd20
[27812529096] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[27813159462] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=5
[27816935322] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[27820726593] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='spawn_init_task' elapsed_ticks=23992056 elapsed_us=11996 total_ticks=717524742 total_us=358762
[27829104237] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[27984481008] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED: early-boot bringup complete; resuming steady-state SMP scheduling
[27985186878] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='end_bringup' elapsed_ticks=1037784 elapsed_us=518 total_ticks=882081420 total_us=441040
[27985997424] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[28275122172] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1171783272 elapsed_us=585891
[28275882063] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[28294334475] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[28295381004] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] ENTER_USER: TID=5 entry_pc=200000 user_sp=800000
[28371169530] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=0
[28375394025] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=20
[28389462123] [[34mDEBUG[0m] [sprout] [CPU0] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7fef70 rip=0x20414c rflags=0x202
[28390617222] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[28391307681] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: About to create Supervisor...
[28392011241] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Listing /bin directory...
[28460625666] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Parsed command line: '    '
[28462050507] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: FORCING display=bootfb for diagnostic test!
[28462923984] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Supervisor created, calling run_forever...
[28463764494] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[28464517026] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[28465377600] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[28483069956] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU0] BootFs: EXACT match for 'bin/sh' at index 4
[28501272657] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU0] BootFs: EXACT match for 'bin/sh' at index 4
[28518944322] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'sh' (len=117640, base=0x200000)
[28523138457] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[28536901008] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x216000 filesz=0 memsz=0 align=1
[28541923245] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /bin/sh
[28543271493] [[34mDEBUG[0m] [[28554450738bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb009c500 arg=0xffffffffb006f660
[28546535292] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=6, applying inserts
[28547144472] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[28547661549] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=6
[28548190110] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[28548689268] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SETUP_STDIO: stdin=Fd(3) stdout=Fd(4) stderr=Fd(5)
[28577634459] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 6
[28583354877] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Task 6 woken, restoring IRQs
[28584507600] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Done for /bin/sh
[28590964116] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[28593040443] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 250ms so the prompt can take the foreground
[28593915669] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[28594959987] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=6 entry_pc=200000 user_sp=800000
[28597386312] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=0
[28599733206] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=100
[28610801175] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=21021
[28611281292] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'etc/motd' at index 105
[28615654320] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[28616633166] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
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
[28632360075] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'etc/profile' at index 104
[28672262355] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[28673054652] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [28679145264] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[28680022437] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[?25h[29425031229] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[29425965162] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[29429387493] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=4096 port=0xffffffffb0071470
[29431278558] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(0)
[29440611222] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=1 port_id=PortId(0) mode=Write
[29442229278] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=2 port_id=PortId(0) mode=Read
[29449318272] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/cambium' (len=124728, base=0x200000)
[29450240985] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[29464148274] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x217000 filesz=0 memsz=0 align=1
[29469417516] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb0081820 arg=0xffffffffb0071600
[29476114239] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[29476628148] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=7
[29477138097] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[29490155541] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 7 → task '/bin/cambium' (pid=7 from boot module)
[29492394822] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned cambium (PID=7)
[29493588696] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[29494909125] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/iso9660d' (len=80648, base=0x200000)
[29495361522] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[29495894043] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[29496365217] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=7 entry_pc=200000 user_sp=800000
[29497472994] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=0
[29498634726] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=100
[29515269861] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20e000 filesz=0 memsz=0 align=1
[29520363510] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb00b5440 arg=0xffffffffb0071600
[29521964010] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[29522496795] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=8
[29523076242] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[29526850089] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 8 → task '/bin/iso9660d' (pid=8 from boot module)
[29529473226] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned iso9660d (PID=8)
[29530278723] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[29531456658] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Deferring netd until /dev/net/virtio0/rx is ready...
[29532007659] [[34mall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=20
[29563090590] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Running registration + health supervision loop
[295639744DEBUG29] [[32mINFO [0m] [sprout::supervisor] [CPU3] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[29566504836] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU0] BRIDGE: handle=2 -> fd=3 node=0xffffffffb0069330 port=0xffffffffb0071470
[29569132725] [[34mDEB[UG[0m] [sprout::supervisor] [CPU0] SPROUT: Bridged resp_port 2 -> FD 3 for task 'cambium'
[29578176078] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ahci_disk' at index 57
[29587488282] [[34mDEBUG[0m] [iso9660d] [CPU2] iso9660d: starting ISO9660 VFS provider
[29596836324] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=8 entry_pc=200000 user_sp=800000
[29533507707] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=0
[29535865326] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[29536847637] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=100
[29553002358] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Task 9 (user thread) assigned to CPU 3
[29557846626] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[29558389740] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=9
[29558931501] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[29560276350] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Sending post-unlock Resched IPI to CPU 3 for task 9
[29561812830] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=0
[29562689805] [[34mDEBUG[0m] [kernel::sysc[29874756945] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[29878192443] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ata_disk' at index 58
[30048672753] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.ata' start='thingos_driver_start_safe'
[30051656613] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/chime' at index 63
[30265378704] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[30267808428] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/display_bootfb' at index 41
[30444342522] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[30447114192] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/display_virtio_gpu' at index 42
[30636618177] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[30640405686] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/hdaudio' at index 61
[30815562492] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[30818406366] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/pci_stubd' at index 62
[30994727214] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[30996647715] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ps2_kbd' at index 3
[31181983173] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[31186099824] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ps2_mouse' at index 40
[31357052991] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[31358936730] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/rtc_cmos' at index 2
[31525082061] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[31527622170] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/rtl8168d' at index 45
[31697698131] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[31699818513] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/virtio_netd' at index 44
[31875596082] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[31878549252] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/virtio_sound' at index 60
[32050525683] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[32052326856] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[32202936777] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=3 starting copyin
[32203787418] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=3 copyin ok
[32213422989] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/virtio_netd' at index 44
[32228684598] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'virtio_netd' (len=106272, base=0x200000)
[32229974667] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[32245949538] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x214000 filesz=0 memsz=0 align=1
[32276319339] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x20277c + bias 0x0 = PC 0x20277c
[32277832785] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/virtio_netd
[32280065169] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb00f8080 arg=0xffffffffb00bcb60
[32282844462] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=10, applying inserts
[32283820965] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[32284666920] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=10
[32285837331] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[32287275900] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[32297445543] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[32302894008] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 10
[32308929807] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 10 woken, restoring IRQs
[32309559183] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/virtio_netd
[32313719361] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/virtio_netd for pci-0000:00:02.0 (entry='thingos_driver_start_safe', pid=10)
[32320693086] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=4 starting copyin
[32321995167] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=4 copyin ok
[32325844617] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ahci_disk' at index 57
[32343392565] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'ahci_disk' (len=74072, base=0x200000)
[32344889808] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[32358923223] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x20e000 filesz=0 memsz=0 align=1
[32364916452] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[32365713402] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/ahci_disk
[32366407755] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb0110860 arg=0xffffffffb00bd5a0
[32382241914] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=11, applying inserts
[32382906831] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[32383806609] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=11
[32385656787] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[32386542507] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[32394403767] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[32400495963] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 11
[32401327992] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 11 woken, restoring IRQs
[32401892358] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/ahci_disk
[32403171438] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x4
[32403941988] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=11 entry_pc=201000 user_sp=800000
[32404896678] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/ahci_disk for pci-0000:00:1f.2 (entry='thingos_driver_start_safe', pid=11)
[32405335446] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=11 buf_len=0
[32407068573] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=11 buf_len=100
[32410557300] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver (boot_fd=4)
[32410967622] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=5 starting copyin
[32411670951] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=5 copyin ok
[32414455986] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ata_disk' at index 58
[32424469968] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'ata_disk' (len=61712, base=0x200000)
[32425953516] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[32434157019] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x20b000 filesz=0 memsz=0 align=1
[32440886214] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[32441704317] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/ata_disk
[32442364020] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb0124860 arg=0xffffffffb00dd500
[32444356527] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=12, applying inserts
[32444972241] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[32445485919] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=12
[32446056885] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[32446556835] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[32448186870] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[32452527525] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 12
[32453423574] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 12 woken, restoring IRQs
[32453990283] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/ata_disk
[32455243557] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x5
[32457962394] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=12 entry_pc=201000 user_sp=800000
[32462953743] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 'pci-0000:00:1f.2' (handle 0)
[32464234209] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=12 buf_len=0
[32465576946] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x20277c SP=0x800000 ARG0=0x3
[32470897701] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Claimed PCI device '/sys/devices/pci-0000:00:1f.2' handle=0
[32471668746] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=12 buf_len=100
[32473273635] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=10 entry_pc=20277c user_sp=800000
[32474764146] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: mapping BAR5 (phys=0x80880000, size=0x1000) for task 11
[32476818594] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=0
[32477790378] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[32478771765] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=100
[32479720977] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[32480317782] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[32482301049] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[32484376419] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[32486044470] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[32487362325] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: DMA virt=0x20c000
[32490642162] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: DMA phys=0x22f9000
[32491807623] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 0...
[32492731722] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[32495727858] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 1...
[32497110393] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[32498398020] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 2...
[32499852726] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[32504002311] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=4096 port=0xffffffffb00ce3b0
[32505753819] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(1)
[32507005839] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(1) mode=Write
[32508248058] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=4 port_id=PortId(1) mode=Read
[32512199874] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Starting VirtIO-NET driver service...
[32513453313] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Initializing hardware driver...
[32515006161] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Claiming NIC device at /sys/devices/pci-0000:00:02.0
[32516005533] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: device::new(/sys/devices/pci-0000:00:02.0)
[32516749188] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
[32517137499] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: claiming '/sys/devices/pci-0000:00:02.0'...
[32518439448] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 10 claimed device 'pci-0000:00:02.0' (handle 1)
[32520890655] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: claimed, handle=1
[32538207966] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/common_bar -> '4' (n=2)
[32540896872] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=11 fd=5 starting copyin
[32542091835] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=11 fd=5 copyin ok
[32547888549] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/common_offset -> '0x0' (n=4)
[32550496869] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device port=2 model='                                        ' rpc_port=3
[32552213199] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 3...
[32552901282] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[32553329952] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/notify_bar -> '4' (n=2)
[32553717900] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 4...
[32554451622] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[32555117430] [[34mDEBUG[0m] [ahci_disk] [32557673544] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Entering RPC service [CPU2] AHCI: Probing port 5.loop
[32558637837] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[32559571077] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=4 -> fd=5 node=0xffffffffb00d0270 port=0xffffffffb00ce3b0
[32560815507] [[34mDEBUG..
[32555798583] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port [0m] [virtio::device] [CPU1] READ_SYS: /sys/d5 - no device
[3evices/pc25i-0056500:00:02.0/virtio/notify_offset -> '0x3000' (38311] [n=7)[34mDEBUG
[3[0m] [ahci_disk] [C2561637504] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DIPU2] ASK: EnteHCI: Foring RPC service loop
[32563809432] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
[32569393956] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devicund 1 Ses/pci-0000:00:02.0/virtio/notify_multiplier -> '4' (n=2)
ATA disk(s)
[32571875160] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[32579768298] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/device_bar -> '4' (n=2)
[32585410011] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/device_offset -> '0x2000' (n=7)
[32586728262] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: mapping common BAR4...
[32587503102] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: mapping BAR4 (phys=0xc000004000, size=0x4000) for task 10
[32590317342] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x10001000
[32592717036] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: common_cfg at 0x10001000
[32594411883] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: notify_cfg at 0x10004000
[32596943016] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: device_cfg = Some(268447744)
[32597805009] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: allocating DMA command buffer...
[32600781840] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 1 pages phys=0x23b9000 -> user_va=0x10005000
[32602389567] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: cmd_buf virt=0x10005000 phys=0x23b9000
[32603225721] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: device::new complete
[32605324257] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Device features 0x30bf8024
[32607009237] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: MAC 52:54:00:12:34:56
[32607978216] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Link UP
[32612563005] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 4 pages phys=0x23ba000 -> user_va=0x10006000
[32631088050] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: RX queue 0 setup (size=32)
[32635791936] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 4 pages phys=0x23be000 -> user_va=0x1000a000
[32637239250] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: TX queue 1 setup (size=32)
[32642522418] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 16 pages phys=0x23c2000 -> user_va=0x1000e000
[32644292967] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[32645941350] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 1 pages phys=0x23d2000 -> user_va=0x1001e000
[32653675527] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Allocated TX buffer (1 page)
[32656824750] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: RX queue filled with 32 buffers
[32658426504] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: DRIVER_OK set, device is live
[32659406670] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Driver initialized successfully
[32660414457] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Driver initialized successfully
[32661299880] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: MAC 52:54:00:12:34:56
[32662130028] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Initial link state is UP
[32666382078] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=65536 port=0xffffffffb00d15b0
[32667115041] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(2)
[32667918129] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=3 port_id=PortId(2) mode=Write
[32668725408] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=4 port_id=PortId(2) mode=Read
[32669863347] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Created VFS provider port
[32677379955] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Task 13 (user thread) assigned to CPU 1
[32678989266] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[32679601482] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=13
[32680248315] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[32682673980] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=13 buf_len=0
[32684094630] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=13 buf_len=100
[32686258110] [[34mDEBUG[0m] [virtio_netd] [CPU2] VIRTIO_NETD: Entering VFS provider service loop at /dev/net/virtio0
[32696873154] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=262400 port=0xffffffffb00d2690
[32698479759] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(3)
[32702378775] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=5 port_id=PortId(3) mode=Write
[32708936997] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] vfs: mounted userland provider at /dev/net/virtio0 (flags: 0x0)
[32710344975] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Mounted at /dev/net/virtio0
[32711284980] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Provider thread live at /dev/net/virtio0
[33202759359] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=65536 port=0xffffffffb00d3890
[33203661117] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(4)
[33204441996] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(4) mode=Write
[33205191756] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=4 port_id=PortId(4) mode=Read
[33219548967] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU2] VIRTIO_NETD: lookup 'rx'
[33220656843] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU2] VIRTIO_NETD: lookup 'rx' -> handle=5
[33250002588] [[32mINFO [0m] [sprout::supervisor] [CPU3] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[33251519367] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: Loading module '/bin/netd' (len=10813488, base=0x200000)
[33252448416] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[33432867567] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/ata_disk for isa-01f0 (entry='thingos_driver_start_safe', pid=12)
[33467137869] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=6 starting copyin
[33469041771] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=6 copyin ok
[33479509569] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=6 starting copyin
[33480183957] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=6 copyin ok
[33487517547] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=6 starting copyin
[33488163951] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=6 copyin ok
[34688951814] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: TLS block: tp=0xc3e000 filesz=0 memsz=0 align=1
[34694051271] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb018b860 arg=0xffffffffb00bd400
[34695861288] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[34696424631] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=14
[34696994046] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[34701110796] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SCHED: TID 14 �� task '/bin/netd' (pid=14 from boot module)
[34702912827] [[34mDEBUG[0m] [sprout::supervisor] [CPU3] SPROUT: Spawned netd (PID=14)
[34704617871] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[34705367730] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=14 entry_pc=200000 user_sp=800000
[34707611697] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=14 buf_len=0
[34708856457] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=14 buf_len=100
[34712114052] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[34713005415] [[34mDEBUG[0m] [netd] [CPU2] NETD: binary v2 (with heap storage) starting...
[34714679835] [[34mDEBUG[0m] [netd] [CPU2] NETD: main entry point, arg=0
[34769453796] [[34mDEBUG[0m] [netd] [CPU2] NETD: get_args starting...
[34770792045] [[34mDEBUG[0m] [netd] [CPU2] NETD: argv_get len=17
[34773679446] [[34mDEBUG[0m] [netd] [CPU2] NETD: get_args returning 0 args
[34774871538] [[34mDEBUG[0m] [netd] [CPU2] NETD: Starting network service (Phase 3 �� /net/ VFS provider)
[34775890083] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[34776966147] [[34mDEBUG[0m] [netd] [CPU2] NETD: NIC probe loop starting (round=0)
[34779258063] [[34mDEBUG[0m] [netd] [CPU2] NETD: Probing /dev/net/virtio0...
[34791908217] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU2] VIRTIO_NETD: lookup 'rx'
[34792815123] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU2] VIRTIO_NETD: lookup 'rx' -> handle=5
[34800788880] [[34mDEBUG[0m] [netd] [CPU2] NETD: Found /dev/net/virtio0/rx, opening others...
[34813600932] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU2] VIRTIO_NETD: lookup 'tx'
[34814524701] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU2] VIRTIO_NETD: lookup 'tx' -> handle=6
[34835204580] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU2] VIRTIO_NETD: lookup 'events'
[34836197715] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU2] VIRTIO_NETD: lookup 'events' -> handle=8
[34854914820] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU2] VIRTIO_NETD: lookup 'mac'
[34855974384] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU2] VIRTIO_NETD: lookup 'mac' -> handle=3
[34890791133] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU2] VIRTIO_NETD: read mac
[34930192011] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU2] VIRTIO_NETD: lookup 'mtu'
[34931370606] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU2] VIRTIO_NETD: lookup 'mtu' -> handle=4
[34966188279] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU2] VIRTIO_NETD: read mtu
[34999354071] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU2] VIRTIO_NETD: lookup 'status'
[35000240946] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU2] VIRTIO_NETD: lookup 'status' -> handle=2
[35033595102] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU2] VIRTIO_NETD: read status
[35055866901] [[34mDEBUG[0m] [netd] [CPU2] NETD: Opened VFS NIC device at /dev/net/virtio0 (rx=4, tx=5, events=6, mtu=1500, link=up)
[35058150270] [[34mDEBUG[0m] [netd] [CPU2] NETD: Driver online at /dev/net/virtio0 �� MAC 52:54:00:12:34:56  MTU 1500
[35063942826] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=65536 port=0xffffffffb00dd110
[35064594939] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(5)
[35065353510] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(5) mode=Write
[35066075220] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=4 port_id=PortId(5) mode=Read
[35071578663] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=262400 port=0xffffffffb00dd290
[35072214738] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(6)
[35072869887] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(6) mode=Write
[35076333369] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] vfs: mounted userland provider at /net (flags: 0x0)
[35077777218] [[34mDEBUG[0m] [netd::vfs_provider] [CPU2] NetVfsProvider: mounted at /net (port w=3 r=4)
[35078930799] [[34mDEBUG[0m] [netd] [CPU2] NETD: Running DHCP...
[35080131042] [[34mDEBUG[0m] [netd::dhcp] [CPU2] DHCP: Starting discovery...
[35119294023] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=14 fd=5 starting copyin
[35120177334] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=14 fd=5 copyin ok
[35167485144] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU2] VIRTIO_NETD: write tx len=308
[35171817351] [[34mDEBUG[0m] [virtio_netd::driver] [CPU2] VirtIO-NET: TX complete after 1 iterations
[35190326589] [[34mDEBUG[0m] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[35274783423] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU2] VIRTIO_NETD: read rx queued=1
[35336914965] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=14 fd=5 starting copyin
[35337701982] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=14 fd=5 copyin ok
[35381143050] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU2] VIRTIO_NETD: write tx len=320
[35383453479] [[34mDEBUG[0m] [virtio_netd::driver] [CPU2] VirtIO-NET: TX complete after 0 iterations
[35395732680] [[34mDEBUG[0m] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[35457611112] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU2] VIRTIO_NETD: read rx queued=1
[35480480937] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=65536 port=0xffffffffb00e7670
[35481322305] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(7)
[35482098795] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=5 port_id=PortId(7) mode=Write
[35482838688] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=6 port_id=PortId(7) mode=Read
[35506846386] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[35507634096] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
e[35566097655] [[34mDEBUG[0m] [netd::dhcp] [CPU2] DHCP: Configuration received
[35567741451] [[34mDEBUG[0m] [netd] [CPU2] NETD: DHCP — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[35568538269] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
[35569266810] [[34mDEBUG[0m] [netd] [CPU2] NETD: entering VFS service loop
[35569929054] [[34mDEBUG[0m] [netd] [CPU2] NETD: creating SocketApi...
[35571159096] [[34mDEBUG[0m] [netd] [CPU2] NETD: allocating sockets_storage...
[35572494738] [[34mDEBUG[0m] [netd] [CPU2] NETD: pushing socket storage 0...
[35573495925] [[34mDEBUG[0m] [netd] [CPU2] NETD: pushing socket storage 64...
[35574311025] [[34mDEBUG[0m] [netd] [CPU2] NETD: pushing socket storage 128...
[35575089297] [[34mDEBUG[0m] [netd] [CPU2] NETD: pushing socket storage 192...
[35576010360] [[34mDEBUG[0m] [netd] [CPU2] NETD: creating SocketSet...
[35576726163] [[34mDEBUG[0m] [netd] [CPU2] NETD: getting link state...
[35577379068] [[34mDEBUG[0m] [netd] [CPU2] NETD: scanning NIC units...
[35578181265] [[34mDEBUG[0m] [netd] [CPU2] NETD: scanning for registered NIC units...
[35593632855] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU2] VIRTIO_NETD: lookup 'rx'
[35594538045] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU2] VIRTIO_NETD: lookup 'rx' -> handle=5
[35672080818] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[35673297759] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
c[35685561318] [[34mDEBUG[0m] [netd] [CPU2] NETD: NIC scan complete: [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]
[35686641540] [[34mDEBUG[0m] [netd] [CPU2] NETD: NIC units scanned.
[35687360115] [[34mDEBUG[0m] [netd] [CPU2] NETD: bridging request port to fd...
[35688423111] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=4 -> fd=7 node=0xffffffffb001fd30 port=0xffffffffb00dd110
[35689561974] [[34mDEBUG[0m] [netd] [CPU2] NETD: request fd=7
[35690391660] [[34mDEBUG[0m] [netd] [CPU2] NETD: setting up /dev/net watch...
[35693926950] [[34mDEBUG[0m] [netd] [CPU2] NETD: watch fd=Some(8)
[35868202161] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[35869341618] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
h[36010830702] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[36012077409] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
o[36035034486] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED: CPU 0 resched try_lock actionable misses (excluding idle timer-only misses) reached 100 in 2s (timer=96 ipi=4 pending=5 idle_timer=0 last_trigger=timer_tick suppressing until window reset)
[36176669133] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[36177493572] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
 [36346490499] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[36347376252] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
h[36516460299] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[36517268007] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
e[36684292260] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[36685074063] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
l[36853514841] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[36854258166] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
l[37020704028] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[37021483290] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
o[37189912881] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[37190630070] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
 [37358180970] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[37358974686] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
w[37527757080] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[37528560531] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
o[37696181391] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[37696920657] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
r[37865562075] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[37866289164] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
l[38034397038] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[38035306650] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
d[38203882299] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[38204601105] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
 [38371320603] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[38372148507] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
|[38539093626] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[38540191800] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
 [38705800155] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[38706542292] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
w[38873856945] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[38874566214] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
c[38996099703] [[34mDEBUG[0m] [kernel::sched] [CPU2] SCHED: CPU 2 resched try_lock actionable misses (excluding idle timer-only misses) reached 100 in 2s (timer=100 ipi=0 pending=1 idle_timer=0 last_trigger=timer_tick suppressing until window reset)
[39042026199] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[39042733290] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
 [39106565256] [[34mDEBUG[0m] [kernel::sched] [CPU1] SCHED: CPU 1 resched try_lock actionable misses (excluding idle timer-only misses) reached 100 in 2s (timer=100 ipi=0 pending=1 idle_timer=0 last_trigger=timer_tick suppressing until window reset)
[39212446185] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[39213210894] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
-[39381237654] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[39381943887] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
w[39552282957] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[39552981600] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok

[39569379696] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='echo'
[39571825524] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/echo' at index 23
[39579664080] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/echo' at index 23
[39586221609] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: Loading module 'echo' (len=40584, base=0x200000)
[39587157324] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[39592263117] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: TLS block: tp=0x207000 filesz=0 memsz=0 align=1
[39597248361] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Starting Phase 1 for /bin/echo
[39598197903] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb01dc4a0 arg=0xffffffffb019d120
[39599731314] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Phase 1 complete, ID=15, applying inserts
[39600347259] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[39600852687] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=15
[39601388706] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[39601836978] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[39603329997] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SETUP_STDIO: stdin=Inherit stdout=Fd(7) stderr=Inherit
[39607149681] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Phase 2 complete, waking task 15
[39608231850] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Task 15 woken, restoring IRQs
[39608765691] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Done for /bin/echo
[39611993520] [[34mDEBUG[0m] [sh] [CPU3] sh: spawned '/bin/echo' pid=15 idx=0 background=false pending_pgid=0
[39614535048] [[34mDEBUG[0m] [sh] [CPU3] sh: setpgid leader pid=15 -> pgid=15
[39616393443] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='wc'
[39618059679] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/wc' at index 14
[39621557811] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/wc' at index 14
[39633320892] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: Loading module 'wc' (len=56312, base=0x200000)
[39635293071] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[39644219505] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: TLS block: tp=0x20a000 filesz=0 memsz=0 align=1
[39649495842] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Starting Phase 1 for /bin/wc
[39654493527] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb0200640 arg=0xffffffffb019d620
[39657523686] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Phase 1 complete, ID=16, applying inserts
[39658098909] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[39658644333] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=16
[39659236848] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[39659740824] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[39661318356] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SETUP_STDIO: stdin=Fd(6) stdout=Inherit stderr=Inherit
[39666007161] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Phase 2 complete, waking task 16
[39666809094] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Task 16 woken, restoring IRQs
[39667347489] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Done for /bin/wc
[39668032272] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[39668771076] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=16 entry_pc=200000 user_sp=800000
[39670254030] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=16 buf_len=0
[39670843344] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[39671582544] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=16 buf_len=100
[39671986827] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=15 entry_pc=200000 user_sp=800000
[39673113414] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=15 buf_len=0
[39674310060] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=15 buf_len=100
[39709433247] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=15 fd=1 starting copyin
[39710175516] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=15 fd=1 copyin ok
[39721135839] 
```
</details>
