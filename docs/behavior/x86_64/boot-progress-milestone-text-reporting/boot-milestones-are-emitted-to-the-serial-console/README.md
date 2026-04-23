# ✅ Scenario: Boot milestones are emitted to the serial console

> Last run: 2026-04-22 19:52:24

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 0ms | - - - |
| 2 | When I wait for the system to boot | ✅ | 8613ms | - [📜](./02/serial.log) - |
| 3 | Then the serial output should contain "boot_progress: milestone=" | ✅ | 0ms | - [📜](./03/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01Ht /tmp
[26933717844] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /run
[26934423615] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /services
[26935144104] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /session
[26935943397] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /data
[26936470374] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='vfs_init' elapsed_ticks=46631970 elapsed_us=23315 total_ticks=51658233 total_us=25829
[26937286365] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[26958388644] [[34mDEBUG[0m] [kernel] [CPU0] Scanning PCI bus...
[26962281621] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x29c0 at 00:00.0 class=060000 id=1
[26979796371] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1050 at 00:01.0 class=030000 id=2
[26999IOAP216673] [[IC34mDEBUG[: hhdm=0xff0mff] [kern800000000el00] 0
[2706591[C90PU45] [[340] PCI: DmDiscovereEBd 0xUG1af4[:0x1000 at 00:00m2.0 class=020000 i] d=[bran:3
[27:arch] [CPU000] IOAPI35C: Dis69abling80 l2]egacy P [[34mICDEBUG[...0m] 
[[ker2706nel]69 [42CPU0] PC47I: D4]is [covere[3 [CPU0] SMP4md 0x8086:0x291: CPU 1 (APDEB8 at 00:1fIC 1) isU.0 clas online
[27184304928] [[34mGs=06DEBUG[0m] 0100[bra[0mn id=4
[270::arch:] :x86_64] [CPU0][bran::arch] [ SM0CPU0] IOAPIC: PIP: StartC disabing CPU 74584le56d ] [[34mDEBOKUG[0m] [k
[2 (APIC ern27el06] [CP82U0]2)
[27 PCI:9591874 Discovered69] [ 0[3450224]xmD808 [[34mDEBUG[06:0mEx29] [BUG[bran::arch::x86_64] [CPU1] SMP: CPU 1 initializing pr22 eemption timer (veat 00:1f.2 classc=32 init_cnt=010601=625123)
[271 i8924d=5
9516] [[34mDEBUG[[0m] [kernel] [CPU1] SMP: kernel_secon0m] [bran::arch] [C27dary_entry arP01003g_cpu=1 runtime_cpu=1
[27189954693] [[34mDEBUG0311] [0m] [kernel[[34]UmD [0] CPU1] EBUG[0SIOAPIC: RSDP virt=m] [kernel] [CPU0] PMP: CI: Discovered 0x8086Entering kernel_secondary_entry for CPU 0x7f77e01
[27191003202] [[34mDEBUG[0m] [kernel::sched] [CPU1] S14:0x2930 aMP: CPU 1 online (triggered by scheduler s
t 00:1pawn)
f.[2707231[27194345607] [[34mDEBUG[0m] [bran::arch::x86_64::task3 class=0c05] [CPU1] INIT K00 id=6
[8471] [[34mDEB27012469ERNEL C803] [[34mDEBUGUG[TX: en0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='scan_pci' elapsed_ticks=53604606 elapsed_us=26802 total_ticks=1try=270x64ff35ff38ffff800170 total_us=6390 kstack821_to
[2701p=0xff3307ff475]ffffb0 [[32mINFO [0m] [kernel::032e40 arg=0x1
[2boot_p71ro95gr2170es38] s][ [CPU0] boot_p[0m] [bran::arrog[34mDEress: BUG[0mmilestone="P] [kerCI Bneusl: Sca:snnched:ed:spawn"
[27] 035510898][CPU1] S [CHED[: Task34mDEBUG 2 ass[ig0mne] d to C[kerPUne 1
[27198275l] [CPU0313]]  [[ker[nel:start] schedul34mDEBUG[er0m] -e[kernentl:ry:s step='regched] ister_[CPU1] REGISTRY: leApgacy_devicplying 2 es' elapsedeferrd_edtick is=64ns20erts81 ela
[2719ps93ed60_us=321 056] to[ta[3l_tick4ms=15DE06987BU57 totG[0m]al_us=7 [ke5349rn
el::[2sched] [CPU701]36 REGIS37TRY:42 Inserti44] [ng TID[32m=1INFO
 [2[0m] [kernel::b71oot_pr99og968279] [[3ress] [4mDECPU0] BUGbo[0otm]_progress: [ke milrneselto::ne="Legacschedy ] [CPU1] REGISTRY: InsDeviceertings" T
ID[2705757c318=20] [[34m
[27201350055] [[34mDEBUG[0DEBUGm] [kernel[0m::] sc[kernelhed] [] CPUh::x[C86PU0] Sys_64:tem :ain1] REGIitSTiaRYli: zed. SInetsertting ups applied p
[27reemption20211 t0309] [[34mDimEBerUG ([0m10] 0H[kernel::schedz)] [CPU1...] SMP:
[ Sec27on06dary CPU 151 o86nlin11e!5] [
[2[723402mDEB81UG6410] [0m] [bran::a[[34mDEBUG[0m]rc [h]ke [CPrnU0el] ::task] [CPU1cp] SMP: run_scheduli] [er entry on CPU 1
[27203501358] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: bootstrap_cpu start on CPU 1
[27204577719] [[34mDCPU0] MADTEBUG[0m] [kernel::t: ask] [CPU1] toSMP: CPU 1 bootal tsletrapped with idle ngththread 2 and is  144
[now schedulable
[27208300218] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] SM27P: CPU 2 ini07tializ54ing 1806preemption timer2] [ (vec=32 init_cnt=62[5123)
[27209468418]34 [mDEBUG[34mDEBUG[0m] [bran:[0m]:arch::x [br86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[2721an::arch] 0125646] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[27210507027] [[34mDEBUG[0m] [kernel] [CPU2] SMP:[C kernel_PU0]se IOAcondary_entry arg_cpu=2 runtPIime_C: Mcpu=ADT 2pa
[27211443897] [[34mDEBrsedUG[0m] [kernel] [CPU2] SMP: Enter Oing kernel_secondaryK
[_entry for CPU 2
27076453668] [27225841104[] [[34mDEBUG[34m[0m] [kernel:DEBUG[0m] [bran::arch] [CPU0] SMP: Found :sched] [CPU2] SMP: CPU 2 onl4 ine (triggered by scheduler spawn)
[27227973432] [[34mDEBUGCP[0m] [bran::arch::x8Us (CP6_64::task] [CPU2] INIT KERNEL CTX: entry=0xffffffff80U_COUNT017090 kstack_top=0xffffffffb00445a0 arg=0x2
[27229427379] [[34mDEBUG[0m] [kernel::sched::spawn]  n[CPU2] SCHED: Task 3 assigned to CPU 2
[27231855552ow] [[34mDEBUG[0 = 4)
m] [kernel::sched] [[2CPU2] RE70GISTRY: Applying77 1 deferred inserts
[27232793379] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserting TID=3
[27233809284] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserts applied
[27234341475] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[27234970818] [[34mDEBUG[0m] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[27235548780] [[34mDEBUG[0m] [bran::arch::x22491186_64] [CPU3] SMP: CPU 3 initializing preemption timer (vec=32 init_cnt=625123)
[27236714241] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[27237185547] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: run_scheduler entry on CPU 2
[27237783078] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[27238114266] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: bootstrap_cpu start on CPU 2
[27238752354] [[34mDEBUG[0m] [kernel] [CPU3] SMP: kernel_secondary_entry arg_cpu]=3 runtime_cpu=3
[27239527128] [[34mDEBUG[0m] [br [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='start_secondary_cpus' elapsed_ticks=89684133 elapsed_us=44842 total_ticks=353506725 total_us=1an76[272606948::arch] [CPU0] IO753
[27240211251] [AP[34mDEBUG[0m] [79] [[kernel]34 ImDEBC: Found UG[0m][CPU3] SMP: Entering kernel_secondary_entry  [kernefal::sched] or CPU 3
[27240847194] [[32mINFO [0m]t[C phys 0xPU3] REGf [kernel::boot_progress] [CPUISTRY: A0] boot_progress: milestone="SMP Bring-up"
[27248688489] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[27250665453]ec [[34mDEBUG[0m] [branpplyin0::arch::x86_64::task] [CPU3] INIT KERNE00L CTX: entry=0xffffffff80017090 kstack_top=0xffffffffb0055d00 arg=0x3g 1 d
[27253289448] [[34mDEBUG[eferr0m] [kernel::sched::spawn] [CPU3] SCHED: Task 4 assigned to CPU 3
00, ed insertGSs
[I base 0
[270786272626704925200] [[34mDE0] [[BUG[0m] [bra34mnDEBU::arch] [G[0CPU0] IOAPIm] [kernel::C: Rtaskegiste] [Crs initiPU2]al SMP: ized
[270795CPU 216992] [ boo[27347442507] [[32mINFO [0m]tstrapp [kernel::boot_progress] [CPU0] ed with idlboot_progress: milestone="Boot Info OK"e thread 3 and
 [34mis nDEow schBUG[0m] [bran:edu:arch] lab[CPUle
[20] 726349IOAPIC: 9978Loca] [[3l APIC en4mDEabled (SVR=0x1FFBUG, [0m] [TPR=0)
[270kern8046el::sched]2574 [CP] [[34U3] REmDEBUG[GIST0m] [braRY: In::arch] [CPU0] IOAPIC: version 0nsertx20, 24 redir entries
[270ing T87492828] [[34mID=4DE
[27BU264761G337][0m] [b [[34ran::arch] [CPUmDEB0] IOAPIUG[C:0m All pin] [kes maskedrnel:
[2:sch708962ed] 7004] [[[CPU3]34m REGISDEBUGTRY:[0m] [br Insertan::ars applied
[2ch] [CPU0] IOAPIC: Init com72plete
[271239657207792169] [3][34mDEBU [[34mDG[0m]EB [keUGrnel[0m] [b::schran:ed] [C:aPU3] Srch::x86_6MP: S4::ioapic] [econCPU0] LAPIC: dary CPUcalibrat 3 oned 625line12300 ticks/sec (delta=6!
[2251272663,84 ok=true) -> init_c4858nt=625] [[34mDEBUG[123
[0m] 27125911[k395] [[34mDEBUG[0m]ernel: :task] [CPU3] SMP: run_scheduler e[brntry an::on Carch::x86_64] [CPU0PU 3] LA
[27PIC: cali267815brated t91imer6] [ (625123[34m00 ticks/secDEBU), init_cnt=G[0625123 fm] [or 100Hzkerne
[27126654l::t984] [[ask]34 [CPmDU3EBUG] SMP[0m: bo] [kotsternerap_cpl] [u start on CPCPU0] [kU 3ernel:st
[2727art]1001 scheduler-802]entry step='setup_preemption_timer' elap sed_ticks=68508957 elapsed_us=34254 tot[al_tic[34mks=241835286 DEBUGtotal_us=[0m]1209 [ke17
rnel::[27127task]567566]  [CPU[[32mINFO [03] m] [kerSMP:ne CPU 3l::boot_progress boot] [CPU0]strap boot_progress: mileped wstone="Bith idle threadSP 4 and T isimer OK"
[2 now714 sche921dulabl98e23] [[
34mDEBUG[0m] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[27150406866] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[27151680996] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[27153868962] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[27183388122] [[34mDEBUG[0m] [bran::arch::x86_64][27405501123] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Enumerating 108 boot modules...
[27407140068] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[27524244594] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[27525572580] [[34mDEBUG[0m] [kernel] [CPU0] Found init module: /bin/sprout (cmdline: 'init'), loading...
[27527210007] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/sprout' (len=85144, base=0x200000)
[27538598340] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[27559642308] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20f000 filesz=0 memsz=0 align=1
[27572648763] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='load_init_module' elapsed_ticks=45669756 elapsed_us=22834 total_ticks=687806922 total_us=343903
[27577485804] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='prepare_boot_registry' elapsed_ticks=3491532 elapsed_us=1745 total_ticks=692664984 total_us=346332
[27579132570] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='map_boot_registry' elapsed_ticks=240933 elapsed_us=120 total_ticks=694318449 total_us=347159
[27580902954] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='activate_init_aspace' elapsed_ticks=243507 elapsed_us=121 total_ticks=696026793 total_us=348013
[27582464679] [[34mDEBUG[0m] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[27583307829] [[34mDEBUG[0m] [kernel] [CPU0] Spawning init process...
[27586802793] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80052250 kstack_top=0xffffffffb00661e0 arg=0xffffffffb001fd20
[27605816997] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[27606397071] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=5
[27616419270] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[27617015448] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='spawn_init_task' elapsed_ticks=33802890 elapsed_us=16901 total_ticks=732203175 total_us=366101
[27617848500] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[27773111586] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED: early-boot bringup complete; resuming steady-state SMP scheduling
[27773813034] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='end_bringup' elapsed_ticks=1121439 elapsed_us=560 total_ticks=888998220 total_us=444499
[27774622887] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[2806733
```
</details>
