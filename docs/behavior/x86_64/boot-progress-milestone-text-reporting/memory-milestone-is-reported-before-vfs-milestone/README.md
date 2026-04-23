# ❌ Scenario: Memory milestone is reported before VFS milestone

> Last run: 2026-04-22 19:52:24

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 0ms | - - - |
| 2 | When I wait for the system to boot | ✅ | 8714ms | - [📜](./02/serial.log) - |
| 3 | Then the serial log shows "boot_progress: milestone=\"Memory Map OK\"" after "kernel:start" | ❌ | 1510ms | - [📜](./03/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[27011611440] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHm] [kernel] [CPU0] PCI: Discovered 0x8086:0x2918 at 00:1f.0 class=06ED: Task 1 assigned to CP0100 id=4
[27U 0
[2703511834533] [[32mI5754NFO [0m] [kernel:552]:s [ch[34mDEBUG[ed] 0m] [C[kernel] [PUCP0]U0 Sch] edulPCI: Discovered 0xer initialized
[27036228285] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[27057358086] [[34mDEBUG[8086:0x2922 at 00:1f.2 class=010601 0mid=5] [kernel] [CPU
[271885350] Initializing V066] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2930 at 00:1f.3 class=0c0500 id=6
[27191FS...
[27061270929] 094612] [[34mDEBUG[0[[34mDEBUG[0m] [km] [kernel]er [nel]CPU0] [ker [CPU0] ne[kerl:starnel:start] t] scheduscheler-entry stedulep=r-e'scan_pcntry step='set_cmdline' elapsed_tii' elapsed_tcks=30357icks=5872699803 elapsed_us=1517 elapsed_us=29363 total_ticks=133849320 total_ticks=3 tot772362 totalal_u_us=1886s=66924
[
[27080945100]27192000264] [[32mINFO [ [0m] [k[ernel::boot_progress] [CP3U0] boot_progress: milestone="PCI Bus Scanned"
[27213974700] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='register_legacy_devices' elapsed_ticks4mDE=6BUG[0m] [kernel::vfs] [CPU0] VFS: Created /dev/display directory
[27096692865] [[34mDEBUG[0m] [kerne6838l::vfs] [CPU0] vfs: mounted union filesystem 2 elapseatd_ / (rootus=334)
[27 t0991ot4710al_ticks8] [[34mDEBUG[0m] =1[k56erne72l:52:vfs] [CPU170] vfs:  tmootunalte_ud devfs s=at /78de36v2
[27
[27215004710016282] 285][ [[34mD[32mEBINUGFO [0m] [kerne[0ml:] :boot_progress[kernel::vfs] ] [CP[CPU0U0] ] vfs:bo mountot_progress:ed procfs m at /ilesproctone
[27="Legacy D101172054]evices [[34mDEBUG"[
[0m27237] [kernel::vfs] [CPU0]591579] vfs: mounted sysfs at / [[sys
[27134m05559899] DE[BUG[0m] [kernel][3 [CPU0] Sy4mstem initialDEBUGized. Sett[0im] [kernel::vfs] [ng up preeCPU0] vfs: mounted tmpfs at /tmp
[mption timer (1027106527426] [[34mDEBUG[0Hz)...
[0m] [27245499468] [[34mDEBUG[0m] [brankernel::vf::arch] [CPs]U0] IOAPIC: hhdm=0xffff800000000000
[2724612359 [CPU0] 7] [[34mDEBUG[0m] vfs: mounted tmpfs a[brat /run
[27107332296] [[34mDn::arcEBUGh] [CPU0] IOAPIC: Disabling legacy PIC.[0m] ..
[kernel::v[27247285956fs] [CP] [[34mDEU0] BUG[0m] [bran::arch] [CPU0] IOAPICvfs: mou: PIC disabled OK
[27248127258] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Rnted tSDP virt=0x7f77e014mpfs 
[27252161409] [[at /services
[27108085884] [[34m34mDEBUG[0m] DEBUG[bra[0m]n::arch::x86_64::acpi] [CPU0] MA [kernDT: total el::vfs] [CPU0] vfs: mounlented tmpfs at /segth 144
[2725ssion
[27108916923] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /data
[27109421691] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='vfs_init' elapsed_ticks=47270784 elapsed_us=23635 total_ticks=52198146 total_us=26099
[271104055355] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[27131783382] [[34mDEBUG[0m] [kernel] [CPU0] Scanning PCI bus...
[27136131000] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x29c0 at 00:00.0 158700] [[34mDcEBUGlass=060000[0m] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[27256121046] [[34mDEBUG[0m] [bran::arch] [CPU0] SMP: Found 4 CPUs  (CPU_COUNT now = 4)
[27256864272] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: idFound at ph=1ys 0xfec00000, GSI 
[27155base16 0
[2728634] [[34mDEBUG[0583m]96 [kernel] [C85PU8] [[34m0DEBUG[0m] [bran::arch] [CPU0] IOAPIC: Registers i] Pnitialized
[27259454046] [[CI34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Local API: Discovered 0x1aC enablf4:0x1050 at 00:01.ed (0 claSVR=0x1FF, TPR=0)ss=030000 id=2
[271
[27268004007734] [[3394m921] [[34mDEBUG[0m] DEBUG[0m[bra] n::arch] [CP[kerU0] IOAPIneC: versionl] 0x20, [CPU 24 redir ent0] PCI:ries D
isco[2726189254vered 08]x1af [[34mDEBU4:G[00x10m] [bran00 at 00::arch] [CPU0] IO:0APIC:2.0 class=0 All pins mas20ke00d
[27260 id=3
[23934471814507289] [[34mDEBUG[0m] [bra5]n::arch] [ [CPU0] IO[APIC34mDEB: UG[0Init complete
[27298524825] [[34mDEBUG[0m] [bran::arch::x86_64::ioapic] [CPU0] LAPIC: calibrated 62754900 ticks/sec (delta=627549, ok=true) -> init_cnt=627549
[27300431862] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62754900 ticks/sec), init_cnt=627549 for 100Hz
[27301248150] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='setup_preemption_timer' elapsed_ticks=62894568 elapsed_us=31447 total_ticks=243998172 total_us=121999
[27302182050] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[27323721150] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[27324626835] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[27325833777] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[27328150608] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[27357599775] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[27358701315] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: StartInserting TID=2
in[2g 73CP74U 242 21(A70PI] C [2)[3
4m[2DE73BU60G13[001m]16 [k] [[34mDEBUGer[ne0ml:] :s[bchraedn:] :a[CrcPUh:1]:x R86EG_6IS4]TR [Y:CP IU1ns] erSMtsP: a CppPUli 1 initializing ped
re[2em73750185pt94] io[[34mDEBUnG t[0imem] [r (veckernel=3::2 initsc_chent=627549)
d][2 [CPU1] SMP:73617 Secondary CPU 1 online!
[2709100] [[34mDEBUG[0m] [kernel] 37571[C55PU211]]  S[[34mMP: kernel_seconDEdary_entry aBUrgG_c[0pum]=1 [ rkeunrntielme_cpu::ta=1sk
] [CPU1] SMP[2: 7362784run_scheduler entry on CPU 1
[2786737] 6337[[34mD43EB9] [UG[34mDEBUG[[00mm] [kerne] [kernel] [Cl::tPU1]ask] [ SCPMPU1] SMP: b: ooEntstetrapri_cngpu kern start on CPU 1
[2el73_sec77on39da8620] ry[[3_e4mntDEryBUG for[0m] [ke CPU rnel::tas1k]
[273627411894939] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] SMP: CPU 3 ini [CP40151U1]tial SizM73] P:in C[[34mDEBUG[0m] [kernel::sched] [CPU1] SgPU  M1 preemptioP:n ti CPbootU 1 onlmer (sivene (c=triggered32 by schedul inier spawn)t_cnt=62754
[279)
[27415390662] [[34mDEBUG[0m] [bran3674::11tra731]rch:: [x8[34m6_DEBUG64[0] [Cm]PU0] SMP: Secondary [bran C::PU staarch:rt:x86_64:up com:tplask]et [CPeU1
[274208] INIT KERNEL 24CTX: entry=0xfff046]ff [ff[f834mDEB0017090UG[0m] [ker kstack_top=0apped nelxffwifffff] [CPU3]fb S00MP32e4: 0 kernel_seconarg=0x1
dary_ent[2ry arg_cpu=3 runtime_cpu=3
[27422028282] [[34mDEBUG[0m]7368 [ke3276rn13el] th ][CPU3] SMPidle [ t: Enterinhre[g kernel_s34econdary_entry for CPmDU EBUG3
[274[0m2500339] [kernel::7] [[34sched::spawad 2 an] [CPU1] SCHnd is noEDw : scTahedulabsk 2 assignedle
[27382751319] [mD[34EBmDEBUG[UG0m] [bran::arch:[0m] [:x86_64] [CPU2] SMP: CPU 2 initializing preemption timer (vec=32kernel::sched]  in to CPU 1it_cnt=627549)
[27383888598][C
[27 [[34mDEBUG[0m] [bran::a371301639] [[PU3]3rc4mh:DEBUG[0m] :x86_64[ker] [CPUne0] SMP: CPU 2 (APIC 2) is online
[273846526l::sch14ed] ] [C[[3PU4mDEBU1]G REGIS[0TRY: Applying 2 dm] [kefererred insenertl] [sCP
[U2] 27372325728] [[34mDEBUG[0m] SMP: kern[kernel_secelon::scdahed]ry_entry a [CPU1] REGISTRYrg: In_cpu=2 runtime_ SMP: CPU 3 osertincngpline u=2
[27385332051] [[34mDEBUG[0m] [bran::(triggarch::xered by 86_ TID=1
[27372925569] [[34mDE64BUG[0]schedumler sp [awn)CP
[2U074] 25SMP:9595 S06] ta[rt[34ming CPU] [kernel::sDEBched] [CPU1] REGIS 3TRY: U (APICG 3[0m] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
)[27429
[27933859838883444] [[] 34[mDEBUG[0[3m] [4mkernel] [CPU2]DE SMP: EnBUG[0m] [bran::archteri::ngx8 k6_64::ertanel_skse] co[CPU3]nd IarNIT KERNEL Cy_enTX: try entrfoy=0xr ffffffff800170CPU 90 kstack_2
[27to3931p=820x15ff7]ff [ff[34ffmDEBUG[0m] [kernelb0055d00 a::scrghed] [CP=0U2] x3
SMP:[2 C74PU 23182 o2814] [nl[3ine (trigg4mered by DEBUGschedule[0r spawn)
[27395055303m]] [[34mDEBU [keG[0m] [bran::rnelar::ch::x86_64::tasksched::spawn] [CPU2] I] [CNIT KERNEL CTX: entry=0xffffffff80017090 PU3] Skstack_top=0xfffCHED: fffffb00Task445a0 arg=0x2
[ 4 assigne2739d to6439 CPU 3
38[2749] [[34mDEBUG[0m] [ker327488nel::sched::60] spawn] [CP[U2] SCHED: Task 3 assigned to CPU 2
[27398849214] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Applying 1 deferred inserts[34m
[27DE3998BU84490] [[34mDEBUG[0m] [kerG[0nel::sm]ched [kernel] [CPU2]]  R[CEGISTRY: PUInsert0]ing  [TID=ke3
[274009rnel62:start20] 4]sche [[du34mDEBler-UG[0m] [kernel::schentredy ] step[CPU='2] REGISTRstart_seY: Inserts appliedcond
[ary_27401803cpus' el902]apsed_t [[34mDEBicksUG[0m] =108[ker57ne8283 elal::psscedhe_ud] [CPU2] SMP: Secs=5428ondary9 total_t CPU 2 onlinicks=37464375e!3 to
[2tal_us=18732740265511
[27437] 4810[[34mDEBUG[0m] [kernel::task] 2572] [[34m[CPU2] SMP: run_scheduDEBUG[0ler entry on CPU 2
m] [[27403461624] [kern[34mDEel::BUG[0m] [kernscheel::task] [CPU2]d] [ SMP: bootstrap_cpu CPU3]st Rart on CPU 2
[27409150098] [[34mDEGISEBUG[0m] [kTRY:ernel: Apply:task] [CPUing 12] S defMP: CPU 2 bootserretrapped wd inith idle tserthread 3 and is now s
[274schedulable
[4927519427] [[411334mDEBU25986] G[0m] [ker[nel::sc[34mDEBUG[0m]hed] [CP [brU3] an::REarGIch::x86_64] [CPU0] SMP: CPSTRY: IU 3 (APIC nser3) is online
[ting TID=4
[27452672874] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[27454341123] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[27455366334] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: run_scheduler entry on CPU 3
[27456473154] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: bootstrap_cpu start on CPU 3
[27457473252] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[27469141260] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle thread 4 and is now schedulable
[27569434332] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[27595418433] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Enumerating 108 boot modules...
[27597657351] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[27713126265] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[27714602322] [[34mDEBUG[0m] [kernel] [CPU0] Found init module: /bin/sprout (cmdline: 'init'), loading...
[27718893246] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/sprout' (len=85144, base=0x200000)
[27727124766] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[27744704394] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20f000 filesz=0 memsz=0 align=1
[27754111671] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='load_init_module' elapsed_ticks=35456784 elapsed_us=17728 total_ticks=696863277 total_us=348431
[27756063588] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='prepare_boot_registry' elapsed_ticks=1045869 elapsed_us=522 total_ticks=698838063 total_us=349419
[27757086225] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='map_boot_registry' elapsed_ticks=165495 elapsed_us=82 total_ticks=699869511 total_us=349934
[27758248848] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='activate_init_aspace' elapsed_ticks=174141 elapsed_us=87 total_ticks=700980390 total_us=350490
[27759271320] [[34mDEBUG[0m] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[27759790641] [[34mDEBUG[0m] [kernel] [CPU0] Spawning init process...
[27763324809] [[3[34mDEBUG[0m4mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb00661e0 arg=0xffffffffb001fd20
[27788480841] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[27789510573] ] [[kernel::sched] [CPU0] REGISTRY: Inserting TID=5
[27794023851] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[27795116547] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='spawn_init_task' elapsed_ticks=35352075 elapsed_us=17676 total_ticks=737868483 total_us=368934
[27796622568] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[27958534428] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED: early-boot bringup complete; resuming steady-state SMP scheduling
[27959337252] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='end_bringup' elapsed_ticks=2431209 elapsed_us=1215 total_ticks=902110935 total_us=451055
[27963466014] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[28255095462] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1197615771 elapsed_us=598807
[28255852416] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[28274219424] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[28275397920] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] ENTER_USER: TID=5 entry_pc=200000 user_sp=800000
[28325240163] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=0
[28331625102] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=20
[28346427648] [[34mDEBUG[0m] [sprout] [CPU0] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7fef70 rip=0x20414c rflags=0x202
[28348526679] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[28349758371] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: About to create Supervisor...
[28350548061] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Listing /bin directory...
[28462633980] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Parsed command line: '    '
[28464086541] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: FORCING display=bootfb for diagnostic test!
[28465009551] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Supervisor created, calling run_forever...
[28465962327] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[28466678889] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[28467529629] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[28484968809] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU0] BootFs: EXACT match for 'bin/sh' at index 4
[28504098018] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU0] BootFs: EXACT match for 'bin/sh' at index 4
[28523548383] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'sh' (len=117640, base=0x200000)
[28524711765] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[28537605393] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x216000 filesz=0 memsz=0 align=1
[28547880339] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /bin/sh
[28549171827] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb009c500 arg=0xffffffffb006f660
[28552570563] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=6, applying inserts
[28556464695] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[28557732324] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=6
[28558321275] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[28558848450] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[28565046906] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SETUP_STDIO: stdin=Fd(3) stdout=Fd(4) stderr=Fd(5)
[28582563867] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 6
[28586749983] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Task 6 woken, restoring IRQs
[28589206272] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Done for /bin/sh
[28595084133] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[28597129869] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 250ms so the prompt can take the foreground
[28597486434] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[28598242002] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=6 entry_pc=200000 user_sp=800000
[28599224049] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=0
[28600572429] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=100
[28604058945] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=20229
[28619883963] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'etc/motd' at index 105
[28624162347] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[28625082915] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
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
[28637470290] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'etc/profile' at index 104
[28683730317] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[28684602936] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [28689802911] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[28690521750] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[?25h[29462530977] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[29463483027] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[29467083360] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=4096 port=0xffffffffb0071470
[29469574827] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(0)
[29478387774] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=1 port_id=PortId(0) mode=Write
[29480020548] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=2 port_id=PortId(0) mode=Read
[29489153661] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/cambium' (len=124728, base=0x200000)
[29490132771] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[29506644420] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x217000 filesz=0 memsz=0 align=1
[29512255509] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb0081820 arg=0xffffffffb0071600
[29515761561] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[29516335497] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=7
[29516886564] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[29526227544] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 7 ��� task '/bin/cambium' (pid=7 from boot module)
[29528468541] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned cambium (PID=7)
[29529788739] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[29530555197] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[29531433228] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/iso9660d' (len=80648, base=0x200000)
[29531885988] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=7 entry_pc=200000 user_sp=800000
[29532777021] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[29533726398] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=0
[29535669042] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=100
[29554359879] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20e000 filesz=0 memsz=0 align=1
[29559141546] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb00b5460 arg=0xffffffffb0071600
[29560622025] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[29561137188] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=8
[29561646708] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[29565297399] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 8 ��� task '/bin/iso9660d' (pid=8 from boot module)
[29567102202] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned iso9660d (PID=8)
[29567727618] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[29568436557] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Deferring netd until /dev/net/virtio0/rx is ready...
[29568789492] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=8 entry_pc=200000 user_sp=800000
ervis[29570965149] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=0
[29572596537] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=100
[29583709716] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Task 9 (user thread) assigned to CPU 3
[29587706841] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRYor] : Applying 1 deferred inserts
[29588325393] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=9
[29588849103] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[29590243287] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=0
[29590815870] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Sending post-unlock Resched IPI to CPU 3 for task 9
[29591556093] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=20
[29592672516] [[34mDEBUG[0m] [sprout::sup[CPU0] SPROUT: Running registration + health supervision loop
[29593463526] [[32mINFO [0m] [sprout::supervisor] [CPU3] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[29596501803] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU0] BRIDGE: handle=2 -> fd=3 node=0xffffffffb0069330 port=0xffffffffb0071470
[29599319673] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Bridged resp_port 2 -> FD 3 for task 'cambium'
[29601671484] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[29609799153] [[34mDEBUG[0m] [iso9660d] [CPU2] iso9660d: starting ISO9660 VFS provider
[29617201548] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet �� retrying
[29672548455] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ahci_disk' at index 57
[29964090486] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[29967569742] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ata_disk' at index 58
[30155511936] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.ata' start='thingos_driver_start_safe'
[30158221038] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/chime' at index 63
[30356293968] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[30359007459] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/display_bootfb' at index 41
[30549619056] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[30552161310] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/display_virtio_gpu' at index 42
[30740343282] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[30742927149] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/hdaudio' at index 61
[30917971128] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[30921126753] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/pci_stubd' at index 62
[31112776299] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[31115386203] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ps2_kbd' at index 3
[31299284643] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[31304245830] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ps2_mouse' at index 40
[31478606049] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[31480723197] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/rtc_cmos' at index 2
[31689927423] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[31694206698] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/rtl8168d' at index 45
[31909030098] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[31912611423] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/virtio_netd' at index 44
[32155385889] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[32160673380] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/virtio_sound' at index 60
[32426332125] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[32429375880] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[32647273296] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=3 starting copyin
[32648114400] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=3 copyin ok
[32657101620] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/virtio_netd' at index 44
[32673428601] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'virtio_netd' (len=106272, base=0x200000)
[32674582908] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[32688672522] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x214000 filesz=0 memsz=0 align=1
[32700196353] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x20277c + bias 0x0 = PC 0x20277c
[32701321686] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/virtio_netd
[32702258457] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb00f7c00 arg=0xffffffffb00ccec0
[32703934428] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=10, applying inserts
[32704665510] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[32705273040] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=10
[32705873772] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[32706335013] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[32708197104] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[32715130074] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 10
[32730716898] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 10 woken, restoring IRQs
[32731753164] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/virtio_netd
[32738283831] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/virtio_netd for pci-0000:00:02.0 (entry='thingos_driver_start_safe', pid=10)
[32748484395] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=4 starting copyin
[32749716648] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=4 copyin ok
[32756061855] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ahci_disk' at index 57
[32772622476] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'ahci_disk' (len=74072, base=0x200000)
[32774608515] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[32790532236] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x20e000 filesz=0 memsz=0 align=1
[32800700361] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[32802043725] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/ahci_disk
[32803270038] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb01103e0 arg=0xffffffffb00cda00
[32826636645] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=11, applying inserts
[32827747227] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[32828847942] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=11
[32830816227] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[32831746959] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, Inherit stdout=Inherit stderr=Inherit
entering Phase 2
[32838965016] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=[32845419123] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 11
[32846390181] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 11 woken, restoring IRQs
[32846970750] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/ahci_disk
[32848860330] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x4
[32849924910] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/ahci_disk for pci-0000:00:1f.2 (entry='thingos_driver_start_safe', pid=11)
[32850337839] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=11 entry_pc=201000 user_sp=800000
[32851845807] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=11 buf_len=0
[32853619557] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=11 buf_len=100
[32856660243] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=5 starting copyin
[32857431552] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=5 copyin ok
[32858524050] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver (boot_fd=4)
[32860750989] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ata_disk' at index 58
[32871055074] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'ata_disk' (len=61712, base=0x200000)
[32872622739] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[32880570657] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x20b000 filesz=0 memsz=0 align=1
[32886771423] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[32887597479] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/ata_disk
[32888371725] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb01243e0 arg=0xffffffffb00cde20
[32890426404] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=12, applying inserts
[32891050401] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[32892227412] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=12
[32893675089] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[32894383302] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[32906856213] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[32914235112] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 12
[32915666124] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 12 woken, restoring IRQs
[32916629130] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/ata_disk
[32917525806] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x5
[32918755848] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=12 entry_pc=201000 user_sp=800000
[32920614804] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=12 buf_len=0
[32921520423] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 'pci-0000:00:1f.2' (handle 0)
[32922499335] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=12 buf_len=100
[32923183293] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Claimed PCI device '/sys/devices/pci-0000:00:1f.2' handle=0
[32925736734] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: mapping BAR5 (phys=0x80880000, size=0x1000) for task 11
[32926644828] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[32928025614] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[32932387587] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[32933892684] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[32935717287] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x20277c SP=0x800000 ARG0=0x3
[32943445326] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[32947150929] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=10 entry_pc=20277c user_sp=800000
[32948354340] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[32950267317] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: DMA virt=0x20c000
[32952902367] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: DMA phys=0x22f9000
[32953262034] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=0
[32954043243] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 0...
[32955060138] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[32955520488] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=100
[32956274142] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 1...
[32957422344] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[32959187910] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 2...
[32960302188] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[32963240904] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=4096 port=0xffffffffb00cde30
[32964432633] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(1)
[32965487214] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(1) mode=Write
[32966588193] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=4 port_id=PortId(1) mode=Read
[32977906995] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
[32990469732] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=11 fd=5 starting copyin
[32991600939] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=11 fd=5 copyin ok
[32998563873] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Starting VirtIO-NET driver service...
[33000396627] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device port=2 model='                                        ' rpc_port=3
[33001200969] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Initializing hardware driver...
[33002892450] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 3...
[33004194201] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Claiming NIC device at /sys/devices/pci-0000:00:02.0
[33005088468] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[33006314022] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: device::new(/sys/devices/pci-0000:00:02.0)
[33007651974] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 4...
[33008201457] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: claiming '/sys/devices/pci-0000:00:02.0'...
[33008842845] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[33009879078] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 5...
[33010563201] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 10 claimed device 'pci-0000:00:02.0' (handle 1)
[33011431893] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[33012456477] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[33013380180] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[33013751100] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: claimed, handle=1
[33014508087] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=4 -> fd=5 node=0xffffffffb00cddb0 port=0xffffffffb00cde30
[33024912492] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[33027142764] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[33029420622] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
[33032434215] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/common_bar -> '4' (n=2)
[33042504363] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=65536 port=0xffffffffb00d04f0
[33043228713] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/[33060892755] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sysdevices/d/pci-0000:00:02.0/virtio/common_offset -> '0x0' (n=4)
[33045231186] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortIevices/pci-0000:0d(2)
[33047137035] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(2) mode=Write
[33048401364] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=4 port_id=PortId(2) mo0:02.0/virtio/notify_offset -de=Read
[33052527057] [[34mDEBUG[0m] [virtio::device]> '0x3000' (n=7)
 [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/notify_bar -> '4' (n=2)
[33069620100] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/notify_multiplier -> '4' (n=2)
[33072456681] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[33081342228] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/device_bar -> '4' (n=2)
[33090038949] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/device_offset -> '0x2000' (n=7)
[33092122866] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: mapping common BAR4...
[33093453789] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: mapping BAR4 (phys=0xc000004000, size=0x4000) for task 10
[33095270175] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x10001000
[33096594729] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: common_cfg at 0x10001000
[33097481967] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: notify_cfg at 0x10004000
[33098925354] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: device_cfg = Some(268447744)
[33099776061] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: allocating DMA command buffer...
[33102979107] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 1 pages phys=0x23b9000 -> user_va=0x10005000
[33104767509] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: cmd_buf virt=0x10005000 phys=0x23b9000
[33106147371] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: device::new complete
[33108389589] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Device features 0x30bf8024
[33111474165] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: MAC 52:54:00:12:34:56
[33112762122] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Link UP
[33126240774] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 4 pages phys=0x23ba000 -> user_va=0x10006000
[33136251258] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: RX queue 0 setup (size=32)
[33146318997] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 4 pages phys=0x23be000 -> user_va=0x1000a000
[33150541908] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: TX queue 1 setup (size=32)
[33159610374] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 16 pages phys=0x23c2000 -> user_va=0x1000e000
[33162546648] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[33164214864] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 1 pages phys=0x23d2000 -> user_va=0x1001e000
[33167291982] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Allocated TX buffer (1 page)
[33170214231] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: RX queue filled with 32 buffers
[33172016229] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: DRIVER_OK set, device is live
[33173546175] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Driver initialized successfully
[33175333587] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Driver initialized successfully
[33177168288] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: MAC 52:54:00:12:34:56
[33178603821] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Initial link state is UP
[33185699712] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=65536 port=0xffffffffb00d1ef0
[33186966978] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(3)
[33188675916] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=3 port_id=PortId(3) mode=Write
[33190351920] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=4 port_id=PortId(3) mode=Read
[33193056270] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Created VFS provider port
[33206405925] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Task 13 (user thread) assigned to CPU 1
[33209671209] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[33210876237] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=13
[33212058726] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[33218965824] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=13 buf_len=0
[33220607145] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=13 buf_len=100
[33222843852] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Entering VFS provider service loop at /dev/net/virtio0
[33249790200] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=262400 port=0xffffffffb00d2fd0
[33250593354] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(4)
[33252565104] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=5 port_id=PortId(4) mode=Write
[33257898003] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] vfs: mounted userland provider at /dev/net/virtio0 (flags: 0x0)
[33259590936] [[34mDEBUG[0m] [virtio_netd] [CPU2] VIRTIO_NETD: Mounted at /dev/net/virtio0
[33260852493] [[34mDEBUG[0m] [virtio_netd] [CPU2] VIRTIO_NETD: Provider thread live at /dev/net/virtio0
[33417439011] [[3
```
</details>
