# ❌ Scenario: Launch ps and verify reaping

> Last run: 2026-04-22 19:53:46

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ❌ | 1001ms | - [📜](./01/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H [kernel:entropy] mark_seeded(timer) ok
[26966565450] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[26967091338] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[26967630756] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed done
[26968703652] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] entropy::seed_from_hardware elapsed_ticks=5988906 elapsed_us=2994
[26969372199] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[26989748379] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[27017460921] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80017090 kstack_top=0xffffffffb001f980 arg=0x0
[27019817220] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[27043080438] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[27044245800] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[27066879015] [[34mDEBUG[0m] [kernel] [CPU0] Initializing VFS...
[27070675071] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='set_cmdline' elapsed_ticks=3003792 elapsed_us=1501 total_ticks=3623202 total_us=1811
[27093278652] [[34mDEBUG[0m] [kernel::vfs] [CPU0] VFS: Created /dev/display directory
[27107916231] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted union filesystem at / (root)
[27110301339] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted devfs at /dev
[27111331764] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted procfs at /proc
[27112330971] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted sysfs at /sys
[27116435445] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /tmp
[27117247608] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /run
[27117963642] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /services
[27118681524] [[34mDEBUG[0m] [kernelc0 at 00:00.0 class=060000 id=1
[27162780744] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1050 at 00:01.0 class=030000 id=2
[27179899263] [[34mDEBUG[0m] [ker] IOAPneIC: Ll]oc [alCP APIC enaU0bl] PCedI: ( DSVisR=co0xve1FreF,d  T0xPR1a=0f4:0x1000 at )00:0
[2.0 2726cl05as06s=41026]0000 i [[34mDd=EB3
[27UG[180m] [bran::a12rch]31704]  [[[34mDEBUGCP[U0] IO0m] [keAPrnIC: veel] rsio[CPU0] Pn CI: Discov0x20ered,  0x824 redir entries
[27261754080] [[34mDE086:0x2BU91G[0m] [bran::arch] 8 at 00:1f.0 cla[CssPU=00]60 IOAPI10C:0  Aidll=4 p
in[2s 71ma89sk37ed51
14[2] 72[71[3314m7381DEBUG] [[0m[3] 4m[kerneDEl]BU [G[0m] [bran::arch] [CCPPU0]U0 I] PCI: Discovered 0x8086:0OAx2922 at 00PIC: In:1f.2 citlass c=0om10pl60ete1 
[id27=53057
[2719171257964044] [[3] 4mDE[[34mDEBUBUGG[0[0m]m] [ [bran::kearrnch::elx86_] 64[C::PUio0]apic P] [CCI: PUDi0] LAPIC: cascovered librated 0x80862546:920x290030 a tict U1] SCHED: Task 2 assigned to CPU 1
[27381887181] [[34mDEBUG[0m]ks/sec  [kernel::sched] [CPU1] REG(dISTRY: App0ellying 2 defert0red inserts
[27383063763] [[34mDa:EBUG[0m] [kernel=1:625:sched] [CPU1] REGISTRY: Inserting TID=1
[27383665f.3 class=0c0500 id=6
[27194084742] [[34mDEBUG[0m] [kerne492l] [CPU0] [kernel:start] scheduler-entry step='scan_pci' elapsed_ticks=52168446 elapsed_us2,=26084 total_ticks=127302681 total_us=63651
[27194898423 ok=true) -> init_c87] n] [[32mINFO [0m] [kernel::boot_progres[[34mDEBs]UGt=6 [CPU0] boot_progress: milestone="PCI Bus Scanned"
[27216316941] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry ste[250m] p='register_legacy_devices' elapsed_ticks=632247 elapsed_us=316 total_ticks=149538312 total_us=74769
[27217201506] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progr[kernel::sched] [CPU1] REGISTRY: Inserting TID=2
[27385227342] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[27386058084] [[34mDEBUG[0m] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[27386753988] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: run_scheduler entry on CPU 1
[27387392802] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: bootstrap_cpu start on CPU 1
[27388603143] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle thread 2 and is now schedulable
[27393705339] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[27394209612] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] SMP: CPe492
[27307U 2 in48itia06lizi29ng]  preemption timer (vec=3[[32 4mDEinitBUG_cnt=6[0m] [25492)
[27395451204] [bran::arch[34mDE::BUG[0m] [bran::arch::x8x8ss: m6_64] [6_6iC4] [CPU0] SMP:lestone PU0]Starting CPU 3 (APIC 3)
[27395992470] [[34mDEBUG[0m] [kernel] [CP="L LU2] SMP: kernel_secondAPICary_entry arg_cpu=2 runtime_cpu=2
[2739e:7051143] [[34mDEBUG[0m] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[27403543200] [[34mDEBUG[0m] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[27405574350] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU2] INIT KERNEL CTX: entry=0xffffffff80017090 kstack_top=0xffffffffb00445a0 arg=0x2
[27419498667] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] SCHED:  calibraTask 3 assigned to CPU 2
[27420709668] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] SMP: CPU 3 initializing preemption timer (vec=32 init_cnt=625492)
[27422310795] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[27435620124] [[34mDEBUG[0m] [kernel] [CPU3] SMP: kernel_secondary_entry arg_cpu=3 runtime_cpu=3
[27436798983] [[34mgtDEBUGed timer (62549200 tickacy Device[s/0s"m] [kernel::schedsec), init
] [CPU2] REGISTRY: Applying 1 deferr[27238630_c188] [[34mDEBUG[0m] [kernel] [CPeU0]nt=625492 for 1 S00ysHzte
m [2in73it08ialized.d inse214 Srts5et8ti
2] ng[[27 up 437883990] [[34mDEBUG[0m] [[34mkeprrneeeDEBUGlmption timer [0m](100Hz] [ker [)...
[27246207318] [[34mDEBUG[0m] [bran::arcCnelh] [] CP[CU0PU] 0] [kIOernel:starAPIC: hhdm=0t]xf sffchf800000ed00ul00er00-e
[27246828114] [[34mDntry EBstUGep[='0mse] tu[bp_raprn:ee:amprctih]on [_tCPimU0er] ' IOelAPIC:apsed_ Dtiisckabs=li68ng93 l44eg92ac ey laPIC...
[2724ps7826760ed] [[34_umDs=EB34UG467 t[0m] [otbralan_t::icarksch=2] 41[C43PU600]85 I tOAotPIalC:_u Ps=IC12 d07is18ab
[273090le55d OK
[2724866951488] 4][ [[3[32mINFO4m DE[0BUm]G [kernel[0::boom]t_ [prbroganre::ssar] ch[C] PU[C0]PU b0]oo It_OAprPIogC:ress:  RSDP virt=0x7mif7le7est01on4e=
["B27SP25 T24im18er90 O8]K" [
[34[27333mD93EB27UG69[] 0m[] [34mD[bEBraUGn:[:a0mrc] h:[k:xer86ne_6l]4: [:aCPcpU0i]]  [KeCPU0] MADrnT:el t: otDeal lengtteh ct14ed4 4
[ CPUs27. 25St56ar08ti35ng8] 3 [ s[ec34onmDEBUdaries...
[2733482462G7][0 [m][34m [DEBUGbr[0m] [branan::::ararchch::x] 86[C_6PU0]4] I [OACPPIU0C:] SMP: St MarADtiT parsed ngOK 3
 s[2ec72on56da52ry34 C48PU] s.[..[3
4m[2DEBUG[0m] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[27257216844] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[27258633600] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Registers initialized
[27259532025] [[34mDEBUG[0m] [bran::arch] [CPU07336025926] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[27338266890] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[27368708037] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[27369708861] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC PU2)
[273]371080869] S [MP: Entering kernel_secondary_entry [3for CPU 4mDEBUG[0m] [b3ran::arch::x86_64] [CPU
[1]27 S43MP85: 30CP75U 7]1  [in[it34iamDliEBziUGng[ pre0mem] pt[bioran n::arch::x86_64] [CPU0] SMP: Secondary CPU startup completeti
me[2r 74(v38ec90=3552 71in] it[_c[3nt4m=6DE25BU49G2)[0
m][2 [73ke72rn55el45::83sc] he[d][3 [4mCPDEU2BUG[0m] [] keRErnel] [CPU1GISTRY: Inse] SMP:rting TI kernel_D=3secondary_entry 
[27439594050] [[34mDarg_cEBUGpu=1 r[0mun] ti[kerme_cpu=1nel]
[2 [CPU0] K7373erne28l: S78ec76] onda[[3ry C4mDEBUGPU b[0rim] [kernngel] [CPU-up 1] ScompMP: leEntete.ring
[27 ker44nel_se0193co363]ndar [[y_entry 34mDfor EBUGCPU 1
[[0m] [ke2737rnel45] [C04PU0]355] [ [[34kernel:start] scmDEBheUG[0m] dule[kerr-ennel::stry chedst] [Cep='PU1]st SMP: CPU art_1 onlineseco (trndariggered y_cpby schus' eduleler spawnapse)
[27d_37789682tick1] [s[34mD=106699989 eEBUGlapsed_us[0m]=5 [3349 totbran:al:arc_tih::x86_64::cks=373400task] [CPU1] 610 toINIT KEtal_us=1RNEL86700
[ CTX: e27nt441071ry=0xfffffff592]f80017090 k [[34mDstEBacUG[k_0m] [ktoernep=l::sched0xfff] ff[CPUfffb2] R00EG32ISe40 TRarY:g= I0xns1er
[ts27378737133] [[3 a4mppDEliBUedG
[0[2m]74 [41ke670047]rn [el[::32scmIheNFd:O :s[pawn] [CP0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[27442106175] [[34mDEBUG[0m] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[27443081919] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: run_scheduler entry on CPU 2
[27443930151] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: bootstrap_cpu start on CPU 2
[27445522962] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[27447356343] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80017090 kstack_top=0xffffffffb0055d00 arg=0x3
[27449299185] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SCHED: Task 4 assigned to CPU 3
[27452302416] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[27453180348] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle thread 3 and is now schedulable
[27454531665] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=4
[27455981355] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[27456909447] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[27457802757] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: run_scheduler entry on CPU 3
[27458664849] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: bootstrap_cpu start on CPU 3
[27460758633] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle thread 4 and is now schedulable
[27567345630] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[27592230303] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Enumerating 108 boot modules...
[27594773118] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[27721963302] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[27723379266] [[34mDEBUG[0m] [kernel] [CPU0] Found init module: /bin/sprout (cmdline: 'init'), loading...
[27725028210] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/sprout' (len=85144, base=0x200000)
[27738310545] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[27755824338] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20f000 filesz=0 memsz=0 align=1
[27763835253] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='load_init_module' elapsed_ticks=39051309 elapsed_us=19525 total_ticks=697046691 total_us=348523
[27765682692] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='prepare_boot_registry' elapsed_ticks=994257 elapsed_us=497 total_ticks=698913336 total_us=349456
[27766656225] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='map_boot_registry' elapsed_ticks=153681 elapsed_us=76 total_ticks=699890565 total_us=349945
[27768158088] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='activate_init_aspace' elapsed_ticks=433488 elapsed_us=216 total_ticks=701225217 total_us=350612
[27769270716] [[34mDEBUG[0m] [kernel] [CPU0] Spkernel::schedawning sprout with registry at 0x600000...
[27778765806] [[34mDEBUG[0m] [kernel] [CPU0] Spawning init process...
[27781006242] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb00661e0 arg=0xffffffffb001fd20
[27800880987] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[27801730308] [[34mDEBUG[0m] [] [CPU0] REGISTRY: Inserting TID=5
[27806622756] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[27807260712] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='spawn_init_task' elapsed_ticks=28581399 elapsed_us=14290 total_ticks=740481951 total_us=370240
[27808139898] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[28009691589] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED: early-boot bringup complete; resuming steady-state SMP scheduling
[28010709540] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='end_bringup' elapsed_ticks=1503975 elapsed_us=751 total_ticks=943924773 total_us=471962
[28011825567] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[28309957170] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1242979023 elapsed_us=621489
[28310665812] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[28328240490] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[28329322032] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] ENTER_USER: TID=5 entry_pc=200000 user_sp=800000
[28376006604] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=0
[28380084183] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=20
[28395047010] [[34mDEBUG[0m] [sprout] [CPU0] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7fef70 rip=0x20414c rflags=0x202
[28399202700] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[28399957113] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: About to create Supervisor...
[28400641335] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Listing /bin directory...
[28469429208] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Parsed command line: '    '
[28470878832] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: FORCING display=bootfb for diagnostic test!
[28471946844] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Supervisor created, calling run_forever...
[28472906715] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[28473784317] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[28474588164] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[28490641773] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU0] BootFs: EXACT match for 'bin/sh' at index 4
[28508215527] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU0] BootFs: EXACT match for 'bin/sh' at index 4
[28527390309] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'sh' (len=117640, base=0x200000)
[28528276557] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[28540063695] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x216000 filesz=0 memsz=0 align=1
[28544995017] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /bin/sh
[28550205651] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb009c500 arg=0xffffffffb006f660
[28553256831] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=6, applying inserts
[28555307814] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[28558576530] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=6
[28559082849] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[28559666091] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[28568052216] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SETUP_STDIO: stdin=Fd(3) stdout=Fd(4) stderr=Fd(5)
[28582618251] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 6
[28588719291] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Task 6 woken, restoring IRQs
[28589370942] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Done for /bin/sh
[28597877682] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[28600058553] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 250ms so the prompt can take the foreground
[28603831938] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[28605169296] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=6 entry_pc=200000 user_sp=800000
[28608983634] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=0
[28611015444] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=100
[28613522091] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=19635
[28621561320] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'etc/motd' at index 105
[28627431030] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[28628463897] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
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
[28640309973] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'etc/profile' at index 104
[28700705847] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[28701482304] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [28706478042] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[28707153057] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[?25h[29459266551] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[29460148014] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[29463497382] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=4096 port=0xffffffffb0071530
[29465236053] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(0)
[29469880836] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=1 port_id=PortId(0) mode=Write
[29471456124] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=2 port_id=PortId(0) mode=Read
[29477930856] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/cambium' (len=124728, base=0x200000)
[29485065390] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[29498003139] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x217000 filesz=0 memsz=0 align=1
[29503250535] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb00818e0 arg=0xffffffffb00716c0
[29505004452] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[29505521892] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=7
[29506359036] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[29523364167] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 7 → task '/bin/cambium' (pid=7 from boot module)
[29525460096] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned cambium (PID=7)
[29526635754] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[29526995652] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[29527999215] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/iso9660d' (len=80648, base=0x200000)
[29528443131] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=7 entry_pc=200000 user_sp=800000
[29529301725] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[29530345449] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=0
[29531422338] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=100
[29537189715] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20e000 filesz=0 memsz=0 align=1
[29546288442] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb00b5300 arg=0xffffffffb00716c0
[29553455580] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[29554133466] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=8
[29554648266] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[29558427327] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 8 → task '/bin/iso9660d' (pid=8 from boot module)
[29560208370] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned iso9660d (PID=8)
[29561344659] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[29562142995] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Deferring netd until /dev/net/virtio0/rx is ready...
[29562472500] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=8 entry_pc=200000 user_sp=800000
[29563498008] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=0
[29564703399] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=100
[29571870834] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[29573673756] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Task 9 (user thread) assigned to CPU 3
[29577093150] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[29577622701] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=9
[29578151856] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[29579500005] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Sending post-unlock Resched IPI to CPU 3 for task 9
[29580843732] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Running registration + health supervision loop
[29581726977] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=0
[29584285368] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU0] BRIDGE: handle=2 -> fd=3 node=0xffffffffb008b9b0 port=0xffffffffb0071530
[29586191712] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=20
[29587349055] [[32mINFO [0m] [sprout::supervisor] [CPU3] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[29587697997] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Bridged resp_port 2 -> FD 3 for task 'cambium'
[29602057089] [[34mDEBUG[0m] [iso9660d] [CPU2] iso9660d: starting ISO9660 VFS provider
[29616549567] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[29617557684] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ahci_disk' at index 57
[29880447762] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[29883833067] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ata_disk' at index 58
[30051564774] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.ata' start='thingos_driver_start_safe'
[30054012153] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/chime' at index 63
[30220181376] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[30222549324] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/display_bootfb' at index 41
[30397422198] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[30399564855] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/display_virtio_gpu' at index 42
[30580990671] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[30583773957] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/hdaudio' at index 61
[30755308188] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[30758049234] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/pci_stubd' at index 62
[30927299271] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[30929209641] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ps2_kbd' at index 3
[31100087964] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[31102172970] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ps2_mouse' at index 40
[31268354073] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[31270257810] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/rtc_cmos' at index 2
[31435280481] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[31437645525] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/rtl8168d' at index 45
[31603645326] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[31606348917] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/virtio_netd' at index 44
[31779628419] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[31782276900] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/virtio_sound' at index 60
[31954356687] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[31956419385] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[32105875758] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=3 starting copyin
[32106646341] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=3 copyin ok
[32113592016] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/virtio_netd' at index 44
[32129078421] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'virtio_netd' (len=106272, base=0x200000)
[32130098022] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[32141463717] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x214000 filesz=0 memsz=0 align=1
[32154276363] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x20277c + bias 0x0 = PC 0x20277c
[32155189308] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/virtio_netd
[32156210823] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb00f8160 arg=0xffffffffb008c2c0
[32157952167] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=10, applying inserts
[32158741263] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[32159326782] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=10
[32159877222] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[32160321501] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[32162127657] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[32166935790] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 10
[32172863943] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 10 woken, restoring IRQs
[32173437285] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/virtio_netd
[32178189252] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/virtio_netd for pci-0000:00:02.0 (entry='thingos_driver_start_safe', pid=10)
[32184185022] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=4 starting copyin
[32184959103] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=4 copyin ok
[32188870956] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ahci_disk' at index 57
[32199608826] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'ahci_disk' (len=74072, base=0x200000)
[32200615821] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[32209857768] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x20e000 filesz=0 memsz=0 align=1
[32216369955] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[32217305175] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/ahci_disk
[32218034871] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb010ff60 arg=0xffffffffb00ce260
[32232946713] [[34mDEBUG[0m] 
[32235828669] [[34mDEBUG[0m] [kernel::sched::spawn] [ker[CPU1]ne SPAWN_FROM_PATH: Inserts l::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=11, applying inserts
[32233595applied, entering 229] [[34mDPhase 2
EBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[32234112603] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=11
[32234773659] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied[32247830406] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[32251817400] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 11
[32252681604] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 11 woken, restoring IRQs
[32254527624] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/ahci_disk
[32255773011] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x4
[32257060572] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=11 entry_pc=201000 user_sp=800000
[32258264511] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/ahci_disk for pci-0000:00:1f.2 (entry='thingos_driver_start_safe', pid=11)
[32258676318] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=11 buf_len=0
[32259832671] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=11 buf_len=100
[32263069773] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver (boot_fd=4)
[32264705550] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=5 starting copyin
[32265358455] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=5 copyin ok
[32267938659] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ata_disk' at index 58
[32276705340] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'ata_disk' (len=61712, base=0x200000)
[32277888588] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[32285382459] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x20b000 filesz=0 memsz=0 align=1
[32291387271] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[32294373144] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/ata_disk
[32295461187] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb0123f60 arg=0xffffffffb00dd780
[32297752344] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=12, applying inserts
[32298810951] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[32299892625] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 'pci-0000:00:1f.2' (handle 0)
[32300794416] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=12
[32301544308] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[32302285653] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[32303179821] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Claimed PCI device '/sys/devices/pci-0000:00:1f.2' handle=0
[32305517673] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: mapping BAR5 (phys=0x80880000, size=0x1000) for task 11
[32311442196] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[32312499747] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[32316247260] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 12
[32316862314] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[32317244058] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 12 woken, restoring IRQs
[32317820469] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/ata_disk
[32318217096] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x5
[32318791692] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[32319218877] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=12 entry_pc=201000 user_sp=800000
[32320907916] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[32321679588] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: DMA virt=0x20c000
[32322728625] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=12 buf_len=0
[32324507556] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=12 buf_len=100
[32328326316] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[32331036078] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[32331729177] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: DMA phys=0x22f9000
[32334257010] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 0...
[32335464117] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[32336315484] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x20277c SP=0x800000 ARG0=0x3
[32341781604] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 1...
[32343364284] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=10 entry_pc=20277c user_sp=800000
[32343860142] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[32344537830] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 2...
[32345318940] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[32346121698] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=0
[32347428861] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=100
[32347836345] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=4096 port=0xffffffffb00ce610
[32348725563] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(1)
[32349461694] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(1) mode=Write
[32350248084] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=4 port_id=PortId(1) mode=Read
[32362712745] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
[32370633834] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Starting VirtIO-NET driver service...
[32371086726] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=11 fd=5 starting copyin
[32371620567] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Initializing hardware driver...
[32371956507] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=11 fd=5 copyin ok
[32373267498] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Claim[32383953624] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU2] Bing NIC device at /sys/devices/pci-0000:00:02.0
[32374678512] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: device::new(/sys/devices/pci-0000:00:02.0)RIDGE: handle=4 -> fd=5 node=0xffffffffb
[3237568900d05b0 port=0xffffffffb00ce610
[32389392915] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/common_bar -> '4' (n=2)
[32391615993] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[32392673082] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
434] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: claiming '/sys/devices/pci-0000:00:02.0'...
[32376739362] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device port=2 model='                                        ' rpc_port=3
[32377122525] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 10 claimed device 'pci-0000:00:02.0' (handle 1)
[32378243535] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 3...
[32378688012] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: claimed, handle=1
[32379032268] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[3237[32394016941] [[32mIN9690486] [[34mDEBUG[0m] [ahci_diskFO]  [CPU2] A[0HCI: Probing pm] [ata_disk] [ort 4CP...
[32U3380304847] [[34mDEBUG[0m] [ahci] ATA_DISK: No active devices to service_disk] [CPU2] A
[32395796532] [[34mDEBUG[0m] [virtio::device]HCI: Port 4 - no device
 [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/common_offset -> '0x0' (n=4)
[32401579518] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/notify_bar -> '4' (n=2)
[32380906305] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 5...
[32381533338] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[32382224292] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[32383000221] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[32407683000] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/notify_offset -> '0x3000' (n=7)
[32413555647] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/notify_multiplier -> '4' (n=2)
[32415326427] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[32420857458] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/device_bar -> '4' (n=2)
[32424972426] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/device_offset -> '0x2000' (n=7)
[32426018955] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: mapping common BAR4...
[32426845605] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: mapping BAR4 (phys=0xc000004000, size=0x4000) for task 10
[32428613217] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x10001000
[32429993376] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: common_cfg at 0x10001000
[32431002417] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: notify_cfg at 0x10004000
[32432545563] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: device_cfg = Some(268447744)
[32433380430] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: allocating DMA command buffer...
[32436693663] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 1 pages phys=0x23b9000 -> user_va=0x10005000
[32438412369] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: cmd_buf virt=0x10005000 phys=0x23b9000
[32439260667] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: device::new complete
[32441531991] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Device features 0x30bf8024
[32443449324] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: MAC 52:54:00:12:34:56
[32444425497] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Link UP
[32461710204] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 4 pages phys=0x23ba000 -> user_va=0x10006000
[32466204375] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: RX queue 0 setup (size=32)
[32472523050] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 4 pages phys=0x23be000 -> user_va=0x1000a000
[32474635479] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: TX queue 1 setup (size=32)
[32484407076] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 16 pages phys=0x23c2000 -> user_va=0x1000e000
[32489774526] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[32491296255] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 1 pages phys=0x23d2000 -> user_va=0x1001e000
[32494110297] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Allocated TX buffer (1 page)
[32496973740] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: RX queue filled with 32 buffers
[32498589453] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: DRIVER_OK set, device is live
[32499668487] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Driver initialized successfully
[32500593642] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Driver initialized successfully
[32501474049] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: MAC 52:54:00:12:34:56
[32502272616] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Initial link state is UP
[32506462692] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=65536 port=0xffffffffb00d17f0
[32508115992] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(2)
[32509480773] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=3 port_id=PortId(2) mode=Write
[32510596404] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=4 port_id=PortId(2) mode=Read
[32513292240] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Created VFS provider port
[32527496133] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Task 13 (user thread) assigned to CPU 1
[32529088614] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[32529705483] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=13
[32530395942] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[32533290339] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=13 buf_len=0
[32534133522] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=13 buf_len=100
[32535402240] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Entering VFS provider service loop at /dev/net/virtio0
[32542893900] [[34mDEBUG[0m] [cambium::spawn] [CPU2] CAMBIUM: launched driver /drivers/ata_disk for isa-01f0 (entry='thingos_driver_start_safe', pid=12)
[32573928387] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=7 fd=6 starting copyin
[32574626436] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=7 fd=6 copyin ok
[32583281577] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=7 fd=6 starting copyin
[32585469114] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=7 fd=6 copyin ok
[32592617343] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=7 fd=6 starting copyin
[32593241109] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=7 fd=6 copyin ok
[32615810469] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=262400 port=0xffffffffb00d2bb0
[32616643026] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(3)
[32618464989] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=5 port_id=PortId(3) mode=Write
[32623693410] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] vfs: mounted userland provider at /dev/net/virtio0 (flags: 0x0)
[32625318726] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Mounted at /dev/net/virtio0
[32626153395] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Provider thread live at /dev/net/virtio0
[32815131987] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'rx'
[32816082321] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'rx' -> handle=5
[32840661084] [[32mINFO [0m] [sprout::supervisor] [CPU3] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[32842152354] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: Loading module '/bin/netd' (len=10813488, base=0x200000)
[32843056620] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[32928765573] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=65536 port=0xffffffffb00d3ff0
[32929569849] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(4)
[32930270076] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(4) mode=Write
[32930961492] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=4 port_id=PortId(4) mode=Read
[34190106159] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: TLS block: tp=0xc3e000 filesz=0 memsz=0 align=1
[34194777573] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb018af60 arg=0xffffffffb00716a0
[34196289765] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[34196805819] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=14
[34197375531] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[34205131125] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SCHED: TID 14 → task '/bin/netd' (pid=14 from boot module)
[34208373573] [[34mDEBUG[0m] [sprout::supervisor] [CPU3] SPROUT: Spawned netd (PID=14)
[34208930316] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[34209620643] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=14 entry_pc=200000 user_sp=800000
[34210841940] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=14 buf_len=0
[34211962125] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=14 buf_len=100
[34214549391] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[34215328686] [[34mDEBUG[0m] [netd] [CPU2] NETD: binary v2 (with heap storage) starting...
[34216994064] [[34mDEBUG[0m] [netd] [CPU2] NETD: main entry point, arg=0
[34264179906] [[34mDEBUG[0m] [netd] [CPU2] NETD: get_args starting...
[34265392227] [[34mDEBUG[0m] [netd] [CPU2] NETD: argv_get len=17
[34267837659] [[34mDEBUG[0m] [netd] [CPU2] NETD: get_args returning 0 args
[34268928639] [[34mDEBUG[0m] [netd] [CPU2] NETD: Starting network service (Phase 3 — /net/ VFS provider)
[34269849669] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[34270812147] [[34mDEBUG[0m] [netd] [CPU2] NETD: NIC probe loop starting (round=0)
[34273019088] [[34mDEBUG[0m] [netd] [CPU2] NETD: Probing /dev/net/virtio0...
[34369798650] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'rx'
[34370602068] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'rx' -> handle=5
[34379287173] [[34mDEBUG[0m] [netd] [CPU2] NETD: Found /dev/net/virtio0/rx, opening others...
[34385184900] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'tx'
[34386072402] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'tx' -> handle=6
[34400911314] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'events'
[34402708758] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'events' -> handle=8
[34411590477] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'mac'
[34412484546] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'mac' -> handle=3
[34513966542] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: read mac
[34535969193] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'mtu'
[34536875835] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'mtu' -> handle=4
[34552306437] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: read mtu
[34618663464] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'status'
[34620055536] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'status' -> handle=2
[34651363131] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=65536 port=0xffffffffb00dd530
[34652610432] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(5)
[34653792987] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=5 port_id=PortId(5) mode=Write
[34654948878] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=6 port_id=PortId(5) mode=Read
[35467483161] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: read status
[35482464633] [[34mDEBUG[0m] [netd] [CPU2] NETD: Opened VFS NIC device at /dev/net/virtio0 (rx=4, tx=5, events=6, mtu=1500, link=up)
[35484908646] [[34mDEBUG[0m] [netd] [CPU2] NETD: Driver online at /dev/net/virtio0 ��� MAC 52:54:00:12:34:56  MTU 1500
[35490549336] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=65536 port=0xffffffffb00e0310
[35491331040] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(6)
[35492509404] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(6) mode=Write
[35493636816] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=4 port_id=PortId(6) mode=Read
[[35516538453] [[34mDEBUG[035500748679]m] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=262400 port=0xffffffffb00e7c70
[35501403168] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(7)
[35502143358] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(7) mode=Write
[35505703035] [[34mDEBUG[0m] [kernel::syscall::handlers [kernel::syscall::handlers::vfs] [CPU2] v:fs: mounted us:verland provider afs]t /net [CPU3] sys_fs_write:  (tid=6 fdflags: 0=1 starting copyin
x0)
[35507474838] [[34mDEBUG[0m] [netd::vfs_provider] [CPU2] NetVfsProvider: mounted at /net (port w=3 r=4)
[35508839091] [[34mDEBUG[0m] [netd] [CPU2] NETD: Running DHCP...
[35510005377] [[34mDEBUG[0m] [netd::dhcp] [CPU2] DHCP: Starting discovery...
[35518193172] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
l[35564749836] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=14 fd=5 starting copyin
[35565544575] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=14 fd=5 copyin ok
[35622364503] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: write tx len=308
[35626308927] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: TX complete after 1 iterations
[35631159036] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: RX frame! desc=0 len=600
[35670239154] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: read rx queued=1
[35679869907] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[35680834860] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
s[35848534392] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[35849380710] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
 [36019692093] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[36020523165] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
/[36188744394] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[36189464850] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
b[36357704790] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[36358410198] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
i[36526355052] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[36527039604] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
n[36692321622] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[36693020496] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
 [36862968945] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[36863639208] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
|[37029556344] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[37030312242] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
 [37059433818] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=14 fd=5 starting copyin
[37060193478] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=14 fd=5 copyin ok
[37196697087] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[37197421536] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
g[37214382942] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: write tx len=320
[37216487517] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: TX complete after 0 iterations
[37221087354] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: RX frame! desc=1 len=600
[37249935492] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: read rx queued=1
[37298425197] [[34mDEBUG[0m] [netd::dhcp] [CPU2] DHCP: Configuration received
[37299889737] [[34mDEBUG[0m] [netd] [CPU2] NETD: DHCP ��� IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[37300659627] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
[37301293425] [[34mDEBUG[0m] [netd] [CPU2] NETD: entering VFS service loop
[37301969562] [[34mDEBUG[0m] [netd] [CPU2] NETD: creating SocketApi...
[37302908742] [[34mDEBUG[0m] [netd] [CPU2] NETD: allocating sockets_storage...
[37303908939] [[34mDEBUG[0m] [netd] [CPU2] NETD: pushing socket storage 0...
[37304833797] [[34mDEBUG[0m] [netd] [CPU2] NETD: pushing socket storage 64...
[37305610716] [[34mDEBUG[0m] [netd] [CPU2] NETD: pushing socket storage 128...
[37306377075] [[34mDEBUG[0m] [netd] [CPU2] NETD: pushing socket storage 192...
[37307253291] [[34mDEBUG[0m] [netd] [CPU2] NETD: creating SocketSet...
[37308213822] [[34mDEBUG[0m] [netd] [CPU2] NETD: getting link state...
[37309151979] [[34mDEBUG[0m] [netd] [CPU2] NETD: scanning NIC units...
[37309849632] [[34mDEBUG[0m] [netd] [CPU2] NETD: scanning for registered NIC units...
[37317960306] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'rx'
[37318808901] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'rx' -> handle=5
[37364039724] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[37364767209] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
r[37421743623] [[34mDEBUG[0m] [netd] [CPU2] NETD: NIC scan complete: [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]
[37422824109] [[34mDEBUG[0m] [netd] [CPU2] NETD: NIC units scanned.
[37423483020] [[34mDEBUG[0m] [netd] [CPU2] NETD: bridging request port to fd...
[37424490411] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=4 -> fd=7 node=0xffffffffb001fd30 port=0xffffffffb00e0310
[37425813117] [[34mDEBUG[0m] [netd] [CPU2] NETD: request fd=7
[37426450182] [[34mDEBUG[0m] [netd] [CPU2] NETD: setting up /dev/net watch...
[37429445163] [[34mDEBUG[0m] [netd] [CPU2] NETD: watch fd=Some(8)
[37533582042] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[37534294974] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
e[37702739151] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[37703441094] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
p[37869274740] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[37869959358] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
 [38039180355] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[38040452637] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
s[38211065310] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[38211724980] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
h[38379313599] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[38380062402] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
 [38549217729] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[38549973990] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
|[38718528453] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[38719158720] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
 [38886997479] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[38887711038] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
w[39056909892] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[39057642525] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
c[39108856347] [[34mDEBUG[0m] [kernel::sched] [CPU1] SCHED: CPU 1 resched try_lock actionable misses (excluding idle timer-only misses) reached 100 in 2s (timer=100 ipi=0 pending=0 idle_timer=0 last_trigger=timer_tick suppressing until window reset)
[39227734194] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[39228632916] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
 [39398193120] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[39398984955] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
-[39567449988] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[39568527108] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
l[39736244724] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[39736936107] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok

[39754647603] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ls'
[39756736569] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/ls' at index 5
[39765189948] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/ls' at index 5
[39775011144] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: Loading module 'ls' (len=62192, base=0x200000)
[39775884918] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[39782600385] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: TLS block: tp=0x20b000 filesz=0 memsz=0 align=1
[39787509630] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Starting Phase 1 for /bin/ls
[39788133000] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb01ea260 arg=0xffffffffb018ef20
[39789681690] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Phase 1 complete, ID=15, applying inserts
[39790259322] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[39790815801] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=15
[39791342085] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[39791777850] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[39797357424] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SETUP_STDIO: stdin=Inherit stdout=Fd(7) stderr=Inherit
[39805865385] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Phase 2 complete, waking task 15
[39808269831] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Task 15 woken, restoring IRQs
[39809449086] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Done for /bin/ls
[39813591675] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[39814341930] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=15 entry_pc=200000 user_sp=800000
[39815692752] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=15 buf_len=0
[39817603320] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=15 buf_len=100
[39846845115] [[34mDEBUG[0m] [ls] [CPU3] ls: listing path '/bin'
[39930587004] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/attr_get' at index 93
[39933947823] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED: CPU 0 resched try_lock actionable misses (excluding idle timer-only misses) reached 100 in 2s (timer=98 ipi=2 pending=4 idle_timer=0 last_trigger=timer_tick suppressing until window reset)
[39938277555] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/attr_list' at index 92
[39941518188] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/attr_rm' at index 95
[39944703513] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/attr_set' at index 94
[39947136306] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/basename' at index 32
[39951656481] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/bloom' at index 74
[39953835999] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/bristle' at index 1
[39956447322] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/cambium' at index 43
[39960159294] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/cat' at index 10
[39965774046] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/clear' at index 75
[39970140078] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/cp' at index 17
[39973998471] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/cwd_test' at index 67
[39976936593] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/date' at index 68
[39979425750] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/dirname' at index 31
[39981941373] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/dmesg' at index 28
[39985211574] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/echo' at index 23
[39987693669] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/env' at index 35
[39990658356] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/env_roundtrip' at index 66
[39996399036] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/false' at index 38
[40001478792] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/fetchd' at index 50
[40004790309] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/file' at index 30
[40007574651] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/find' at index 52
[40009960782] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/grep' at index 24
[40012170099] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/head' at index 11
[40014832902] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/httpsd' at index 51
[40017776304] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/input_echo' at index 39
[40020727131] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/ip' at index 53
[40023737721] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/ipc_memfd_demo' at index 81
[40030900635] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/ipc_pipe_demo' at index 79
[40037111994] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/ipc_provider_demo' at index 80
[40040340747] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/ipc_service_demo' at index 78
[40043451228] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/iso9660d' at index 59
[40046179107] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/iso_reader' at index 54
[40049691726] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/kill' at index 8
[40051927542] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/killall' at index 9
[40054959615] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/ld_so' at index 87
[40057375974] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/ln' at index 6
[40063117446] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/loglevel' at index 76
[40066092594] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/ls' at index 5
[40070380581] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/mdns' at index 48
[40073109318] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/mdnsd' at index 49
[40075739088] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/mesocarp' at index 47
[40078168053] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/mkdir' at index 21
[40080501714] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/mount' at index 22
[40083034629] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/mv' at index 18
[40085789502] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/netd' at index 46
[40088751186] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/nslookup' at index 56
[40092224205] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/ping' at index 55
[40095395802] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/placed' at index 73
[40098591720] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/poll_mux' at index 77
[40100930859] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/ps' at index 7
[40106553135] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/pwd' at index 25
[40110036813] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/reboot' at index 90
[40112339949] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/rm' at index 19
[40115711856] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/rmdir' at index 20
[40118288265] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/setshell' at index 27
[40120496889] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/sh' at index 4
[40123340301] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/show_args' at index 65
[40127302941] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/shutdown' at index 91
[40130070651] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/sleep' at index 33
[40133328015] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/sort' at index 34
[40138645470] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/sprout' at index 0
[40141395690] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/stat' at index 29
[40143663780] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/tail' at index 12
[40146588900] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/tee' at index 71
[40149665061] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/terminal' at index 70
[40152762375] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/test_dlopen' at index 89
[40156257570] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/test_dyn_loader' at index 88
[40159514109] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/test_exec' at index 82
[40162703493] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/test_exec_env' at index 84
[40166531229] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/test_futex' at index 86
[40169609634] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/test_threads' at index 85
[40172733282] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/test_vm_protect' at index 83
[40175045823] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/top' at index 13
[40177792809] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/touch' at index 26
[40180392384] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/true' at index 37
[40184079342] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/uname' at index 36
[40186940904] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/vfs_hello' at index 64
[40189897770] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/wayland_hello' at index 69
[40192712736] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/wc' at index 14
[40195587102] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/which' at index 16
[40199591520] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/xargs' at index 72
[40202261517] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/yes' at index 15
[40207750869] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=15 fd=1 starting copyin
[40208483997] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=15 fd=1 copyin ok
[40218797091] [[34mDEBUG[0m] [sh] [CPU3] sh: spawned '/bin/ls' pid=15 idx=0 background=false pending_pgid=0
[40221189723] [[34mDEBUG[0m] [sh] [CPU3] sh: setpgid leader pid=15 -> pgid=15
[40223177841] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='grep'
[40225285287] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/grep' at index 24
[40228091673] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/grep' at index 24
[40240868811] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: Loading module 'grep' (len=78216, base=0x200000)
[40241802414] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[40251281532] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: TLS block: tp=0x20f000 filesz=0 memsz=0 align=1
[40255827216] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Starting Phase 1 for /bin/grep
[40259590041] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb01fa260 arg=0xffffffffb0198340
[40262645412] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Phase 1 complete, ID=16, applying inserts
[40263383820] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[40264583799] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=16
[40265615247] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[40266164004] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[40267787109] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SETUP_STDIO: stdin=Fd(6) stdout=Fd(9) stderr=Inherit
[40271629959] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Phase 2 complete, waking task 16
[40272376650] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Task 16 woken, restoring IRQs
[40272901812] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Done for /bin/grep
[40275478188] [[34mDEBUG[0m] [sh] [CPU3] sh: spawned '/bin/grep' pid=16 idx=1 background=false pending_pgid=15
[40279734264] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[40280519169] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=16 entry_pc=200000 user_sp=800000
[40281627804] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=16 buf_len=0
[40283122374] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=16 buf_len=100
[40291966671] [[34mDEBUG[0m] [sh] [CPU3] sh: setpgid member pid=16 -> pgid=15
[40292654061] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='wc'
[40294222914] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/wc' at index 14
[40296726195] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/wc' at index 14
[40305033516] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: Loading module 'wc' (len=56312, base=0x200000)
[40306172577] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[40315263120] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: TLS block: tp=0x20a000 filesz=0 memsz=0 align=1
[40320146229] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Starting Phase 1 for /bin/wc
[40320784416] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb01caf60 arg=0xffffffffb019aa80
[40322319015] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Phase 1 complete, ID=17, applying inserts
[40322876484] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[40323361320] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=17
[40323983502] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[40324430091] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[40325800845] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SETUP_STDIO: stdin=Fd(8) stdout=Inherit stderr=Inherit
[40329499584] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Phase 2 complete, waking task 17
[40330363095] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Task 17 woken, restoring IRQs
[40331843442] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Done for /bin/wc
[40335318540] [[34mDEBUG[0m] [sh] [CPU3] sh: spawned '/bin/wc' pid=17 idx=2 background=false pending_pgid=15
[40340010315] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[40341614643] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=17 entry_pc=200000 user_sp=800000
[40343324802] [[34mDEBUG[0m] [sh] [CPU3] sh: setpgid member pid=17 -> pgid=15
[40343990148] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=17 buf_len=0
[40346106108] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=17 buf_len=100
[40351777290] [[34mDEBUG[0m] [sh] [CPU3] sh: foreground handoff start job_id=1 pgid=15 shell_pgid=5 cmd='ls /bin | grep sh | wc -l'
[40352155107] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=16 fd=1 starting copyin
[40352907177] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=16 fd=1 copyin ok
[40353435078] [[34mDEBUG[0m] [sh] [CPU3] sh: foreground handoff done job_id=1 pgid=15 cmd='ls /bin | grep sh | wc -l'
[40354172298] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[40354896945] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[?25l[40368975537] [[34mDEBUG[0m] [kernel::inbox] [CPU3] inbox::close: inbox closed, 0 messages dropped on close
[40378372749] [[34mDEBUG[0m] [kernel::inbox] [CPU3] inbox::close: inbox closed, 0 messages dropped on close
[40402744173] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=17 fd=1 starting copyin
[40403926959] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=17 fd=1 copyin ok
       0
[40413692220] [[34mDEBUG[0m] [kernel::inbox] [CPU3] inbox::close: inbox closed, 0 messages dropped on close
[40421333766] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[40422369768] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[?25h[40424655546] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[40425338877] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [40426898787] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[40427900601] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[?25h[43353422343] [[34mDE
```
</details>
