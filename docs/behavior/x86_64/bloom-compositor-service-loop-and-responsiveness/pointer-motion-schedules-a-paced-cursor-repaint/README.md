# ❌ Scenario: pointer motion schedules a paced cursor repaint

> Last run: 2026-04-30 18:01:45

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 11158ms | - [📜](./01/serial.log) - |
| 2 | Then the pointer debug overlay should update after mouse movement | ❌ | 61117ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[34437234315] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[34470396411] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] enter
[34473469866] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] phys_memory_map ok
[34474333971] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] modules ok
[34475184942] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] phys_to_virt_offset ok
[34475789139] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] memory map logging done
[34500740967] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] boot_frame_alloc init ok
[34703588370] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] frame allocator build ok
[34704443466] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] frame allocator log ok
[34705663938] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] FRAME_ALLOCATOR init ok
[34706630607] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] tasking init ok
[34707194478] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[34727485782] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[34728119910] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] enter
[34728657876] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] set expand hook
[34729342131] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] expand hook ok
[34730139972] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock begin
[34731169143] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock ok
[34731734994] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] reserve_region begin
[34761823437] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] reserve_region ok
[34762569798] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] inner allocator init begin
[34765536861] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] inner allocator init ok
[34766335527] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] heap top store ok
[34766969556] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] init done
[34767675030] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[34791131265] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] devfs: set_boot_fb width=1920 height=1080 pitch=7680 resource_id=0xfb000000 phys=0x80000000
[34793589963] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs set_boot_fb elapsed_ticks=2413884 elapsed_us=1206
[34799303550] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs FbNode::new elapsed_ticks=4840209 elapsed_us=2420
[34800306024] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] register begin
[34804641894] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] register ok
[34805301465] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs register fb0 elapsed_ticks=4966071 elapsed_us=2483
[34805901669] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[34826356719] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[34828376451] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[34865467593] [[34mDEBUG[0m] [kernel] [CPU0] Seeding entropy pool...
[34866819372] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed begin
[34867837785] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] fill_entropy done
[34870182336] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] add_sample(timer) ok
[34870770132] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] mark_seeded(timer) ok
[34871263713] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[34871890053] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[34872432705] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed done
[34873528206] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] entropy::seed_from_hardware elapsed_ticks=6451434 elapsed_us=3225
[34874143293] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[34894441065] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[34903515603] [[35mTRACE[0m] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[34904697003] [[35mTRACE[0m] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[34905414621] [[35mTRACE[0m] [kernel::sched] [CPU0]   Allocating scheduler...
[34909388415] [[35mTRACE[0m] [kernel::sched] [CPU0]   Leaking scheduler...
[34909934631] [[35mTRACE[0m] [kernel::sched] [CPU0]   Initializing boot task...
[34912062669] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED: allocating CpuScheduler for cpu0 (total=1)
[34912940535] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[34913488302] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[34914407022] [[35mTRACE[0m] [kernel::sched] [CPU0]   Creating boot task...
[34927011108] [[35mTRACE[0m] [kernel::sched] [CPU0]   Creating idle tasks...
[34928427798] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff8008b390 kstack_top=0xffffffffb0020660 arg=0x0
[34930434759] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[34950919443] [[35mTRACE[0m] [kernel::sched] [CPU0]   Boot task initialized
[34953409425] [[34mDEBUG[0m] [kernel::sched::state] [CPU0] SCHED[cpu0]: current=Some(0) idle=Some(1) runnable=0 need_resched=true
[34954730415] [[35mTRACE[0m] [kernel::sched] [CPU0]   Storing scheduler pointer...
[34955900628] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[34956849015] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[34977493221] [[34mDEBUG[0m] [kernel] [CPU0] Initializing VFS...
[34981308978] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='set_cmdline' elapsed_ticks=3005640 elapsed_us=1502 total_ticks=3638811 total_us=1819
[35001369711] [[34mDEBUG[0m] [kernel::vfs] [CPU0] VFS: Created /dev/display directory
[35002276551] [[34mDEBUG[0m] [kernel::vfs] [CPU0] VFS: Created /dev/storage directory
[35044640664] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted union filesystem at / (root)
[35047185954] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted devfs at /dev
[35048310594] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted procfs at /proc
[35049225519] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted sysfs at /sys
[35053349331] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /tmp
[35054186013] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /run
[35054945508] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /services
[35055698370] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /session
[35056533468] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /data
[35057037312] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='vfs_init' elapsed_ticks=75020286 elapsed_us=37510 total_ticks=79632102 total_us=39816
[35057695794] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[35078346534] [[34mDEBUG[0m] [kernel] [CPU0] Scanning PCI bus...
[35082301023] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x29c0 at 00:00.0 class=060000 id=1
[35087713221] [[35mTRACE[0m] [kernel] [CPU0]   BAR0 (MEM32): 0x80000000 (size 0x800000)
[35099944770] [[35mTRACE[0m] [kernel] [CPU0]   BAR2 (MEM64): 0xc000000000 (size 0x4000)
[35103617637] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (MEM32): 0x80883000 (size 0x1000)
[35104359411] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1050 at 00:01.0 class=030000 id=2
[35108756100] [[35mTRACE[0m] [kernel] [CPU0]   BAR0 (I/O):  0x6060 (size 0x20)
[35112404085] [[35mTRACE[0m] [kernel] [CPU0]   BAR1 (MEM32): 0x80882000 (size 0x1000)
[35127557058] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (MEM64): 0xc000004000 (size 0x4000)
[35128413144] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1000 at 00:02.0 class=020000 id=3
[35130962262] [[35mTRACE[0m] [kernel] [CPU0]   BAR1 (MEM32): 0x80881000 (size 0x1000)
[35139474381] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (MEM64): 0xc000008000 (size 0x4000)
[35140268955] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1059 at 00:03.0 class=040100 id=4
[35141124744] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2918 at 00:1f.0 class=060100 id=5
[35143603209] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (I/O):  0x6040 (size 0x20)
[35146658019] [[35mTRACE[0m] [kernel] [CPU0]   BAR5 (MEM32): 0x80880000 (size 0x1000)
[35147422299] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2922 at 00:1f.2 class=010601 id=6
[35150379462] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (I/O):  0x6000 (size 0x40)
[35151094374] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2930 at 00:1f.3 class=0c0500 id=7
[35153399160] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='scan_pci' elapsed_ticks=74431599 elapsed_us=37215 total_ticks=175987779 total_us=87993
[35154072360] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[35176229946] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='register_legacy_devices' elapsed_ticks=713427 elapsed_us=356 total_ticks=198805266 total_us=99402
[35177036268] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[35200469700] [[34mDEBUG[0m] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[35207491110] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[35208229815] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[35209199223] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[35210024289] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[35213851167] [[34mDEBUG[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[35215347519] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[35216127672] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[35216731836] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[35217295872] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[35217872118] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[35218809747] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[35219684115] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[35220190698] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[35220678834] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[35221184691] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[35221667580] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[35222488686] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[35223403578] [[34mDEBUG[0m] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[35224368795] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[35233072743] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Registers initialized
[35234105247] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[35235110922] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[35236127949] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: All pins masked
[35237574339] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> NMI
[35238354855] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[35239240410] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ4 -> GSI 4 -> 0x24
[35239832298] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Init complete
[35274282318] [[34mDEBUG[0m] [bran::arch::x86_64::ioapic] [CPU0] LAPIC: calibrated 62572000 ticks/sec (delta=625720, ok=true) -> init_cnt=625720
[35275842492] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62572000 ticks/sec), init_cnt=625720 for 100Hz
[35276504571] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='setup_preemption_timer' elapsed_ticks=75234423 elapsed_us=37617 total_ticks=299096358 total_us=149548
[35277246873] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[35298875766] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[35299784619] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[35301166626] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[35303181474] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[35333206128] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[35334583779] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[35342796489] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] SMP: CPU 1 initializing preemption timer (vec=32 init_cnt=625720)
[35344326204] [[34mDEBUG[0m] [kernel] [CPU1] SMP: kernel_secondary_entry arg_cpu=1 runtime_cpu=1
[35345084544] [[34mDEBUG[0m] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[35346369960] [[34mDEBUG[0m] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[35349781005] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff8008b390 kstack_top=0xffffffffb00375a0 arg=0x1
[35350467438] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Task 2 assigned to CPU 1
[35353431762] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 2 deferred inserts
[35354419353] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=1
[35355022395] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=2
[35356332924] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[35357116905] [[34mDEBUG[0m] [kernel::sched::lifecycle] [CPU1] SMP: Secondary CPU 1 online!
[35357833962] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: run_scheduler entry on CPU 1
[35358461754] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: bootstrap_cpu start on CPU 1
[35359035690] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] SMP: CPU 2 initializing preemption timer (vec=32 init_cnt=625720)
[35359539567] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle thread 2 and is now schedulable
[35359279197] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[35359954278] [[34mDEBUG[0m] [kernel] [CPU2] SMP: kernel_secondary_entry arg_cpu=2 runtime_cpu=2
[35360417235] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[35360922795] [[34mDEBUG[0m] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[35371126626] [[34mDEBUG[0m] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[35372423262] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU2] INIT KERNEL CTX: entry=0xffffffff8008b390 kstack_top=0xffffffffb0048d20 arg=0x2
[35374300830] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] SCHED: Task 3 assigned to CPU 2
[35376172458] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Applying 1 deferred inserts
[35377292247] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserting TID=3
[35377827408] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserts applied
[35378207205] [[34mDEBUG[0m] [kernel::sched::lifecycle] [CPU2] SMP: Secondary CPU 2 online!
[35378574264] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: run_scheduler entry on CPU 2
[35378933469] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: bootstrap_cpu start on CPU 2
[35384764602] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle thread 3 and is now schedulable
[35385642204] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[35386211817] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[35386101531] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] SMP: CPU 3 initializing preemption timer (vec=32 init_cnt=625720)
[35387175021] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[35387852610] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='start_secondary_cpus' elapsed_ticks=89216457 elapsed_us=44608 total_ticks=410444925 total_us=205222
[35388089418] [[34mDEBUG[0m] [kernel] [CPU3] SMP: kernel_secondary_entry arg_cpu=3 runtime_cpu=3
[35389253229] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[35389804527] [[34mDEBUG[0m] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[35391241545] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[35392906725] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff8008b390 kstack_top=0xffffffffb005a4a0 arg=0x3
[35393664306] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SCHED: Task 4 assigned to CPU 3
[35395234446] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[35395839666] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=4
[35396464125] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[35397317241] [[34mDEBUG[0m] [kernel::sched::lifecycle] [CPU3] SMP: Secondary CPU 3 online!
[35397816630] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: run_scheduler entry on CPU 3
[35398295922] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: bootstrap_cpu start on CPU 3
[35398786005] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle thread 4 and is now schedulable
[35472731547] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[35537594730] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Enumerating 214 boot modules...
[35538767517] [[35mTRACE[0m] [kernel] [CPU0]   Module[0]: name='/bin/sprout' cmdline='init' size=89912 bytes
[35539349175] [[35mTRACE[0m] [kernel] [CPU0]   Module[1]: name='/bin/bristle' cmdline='' size=68744 bytes
[35539999176] [[35mTRACE[0m] [kernel] [CPU0]   Module[2]: name='/drivers/rtc_cmos' cmdline='init' size=45904 bytes
[35540579844] [[35mTRACE[0m] [kernel] [CPU0]   Module[3]: name='/drivers/ps2_kbd' cmdline='' size=72264 bytes
[35541213774] [[35mTRACE[0m] [kernel] [CPU0]   Module[4]: name='/bin/sh' cmdline='init' size=127864 bytes
[35541976998] [[35mTRACE[0m] [kernel] [CPU0]   Module[5]: name='/bin/ls' cmdline='init' size=62136 bytes
[35542728606] [[35mTRACE[0m] [kernel] [CPU0]   Module[6]: name='/bin/ln' cmdline='init' size=47568 bytes
[35543474142] [[35mTRACE[0m] [kernel] [CPU0]   Module[7]: name='/bin/ps' cmdline='init' size=50544 bytes
[35546748072] [[35mTRACE[0m] [kernel] [CPU0]   Module[8]: name='/bin/kill' cmdline='init' size=55584 bytes
[35557055127] [[35mTRACE[0m] [kernel] [CPU0]   Module[9]: name='/bin/killall' cmdline='init' size=67656 bytes
[35557593192] [[35mTRACE[0m] [kernel] [CPU0]   Module[10]: name='/bin/cat' cmdline='init' size=50832 bytes
[35558227287] [[35mTRACE[0m] [kernel] [CPU0]   Module[11]: name='/bin/head' cmdline='init' size=56112 bytes
[35558747730] [[35mTRACE[0m] [kernel] [CPU0]   Module[12]: name='/bin/tail' cmdline='init' size=56088 bytes
[35559284244] [[35mTRACE[0m] [kernel] [CPU0]   Module[13]: name='/bin/top' cmdline='init' size=65488 bytes
[35559789771] [[35mTRACE[0m] [kernel] [CPU0]   Module[14]: name='/bin/wc' cmdline='init' size=56248 bytes
[35560344501] [[35mTRACE[0m] [kernel] [CPU0]   Module[15]: name='/bin/yes' cmdline='init' size=44520 bytes
[35560849830] [[35mTRACE[0m] [kernel] [CPU0]   Module[16]: name='/bin/which' cmdline='init' size=50448 bytes
[35561351562] [[35mTRACE[0m] [kernel] [CPU0]   Module[17]: name='/bin/cp' cmdline='init' size=46312 bytes
[35561996184] [[35mTRACE[0m] [kernel] [CPU0]   Module[18]: name='/bin/mv' cmdline='init' size=47048 bytes
[35562770067] [[35mTRACE[0m] [kernel] [CPU0]   Module[19]: name='/bin/rm' cmdline='init' size=47128 bytes
[35563592691] [[35mTRACE[0m] [kernel] [CPU0]   Module[20]: name='/bin/rmdir' cmdline='init' size=47144 bytes
[35564399475] [[35mTRACE[0m] [kernel] [CPU0]   Module[21]: name='/bin/mkdir' cmdline='init' size=47144 bytes
[35565220086] [[35mTRACE[0m] [kernel] [CPU0]   Module[22]: name='/bin/mount' cmdline='init' size=57688 bytes
[35565894441] [[35mTRACE[0m] [kernel] [CPU0]   Module[23]: name='/bin/echo' cmdline='init' size=44616 bytes
[35566414818] [[35mTRACE[0m] [kernel] [CPU0]   Module[24]: name='/bin/printf' cmdline='init' size=93952 bytes
[35566933017] [[35mTRACE[0m] [kernel] [CPU0]   Module[25]: name='/bin/grep' cmdline='init' size=85240 bytes
[35567435277] [[35mTRACE[0m] [kernel] [CPU0]   Module[26]: name='/bin/pwd' cmdline='init' size=39184 bytes
[35567972583] [[35mTRACE[0m] [kernel] [CPU0]   Module[27]: name='/bin/touch' cmdline='init' size=46160 bytes
[35568476163] [[35mTRACE[0m] [kernel] [CPU0]   Module[28]: name='/bin/setshell' cmdline='init' size=47048 bytes
[35568984990] [[35mTRACE[0m] [kernel] [CPU0]   Module[29]: name='/bin/dmesg' cmdline='init' size=39328 bytes
[35569534011] [[35mTRACE[0m] [kernel] [CPU0]   Module[30]: name='/bin/stat' cmdline='init' size=46200 bytes
[35570036337] [[35mTRACE[0m] [kernel] [CPU0]   Module[31]: name='/bin/file' cmdline='init' size=51104 bytes
[35570559849] [[35mTRACE[0m] [kernel] [CPU0]   Module[32]: name='/bin/dirname' cmdline='init' size=50096 bytes
[35571066168] [[35mTRACE[0m] [kernel] [CPU0]   Module[33]: name='/bin/basename' cmdline='init' size=50104 bytes
[35571574632] [[35mTRACE[0m] [kernel] [CPU0]   Module[34]: name='/bin/sleep' cmdline='init' size=56824 bytes
[35572076199] [[35mTRACE[0m] [kernel] [CPU0]   Module[35]: name='/bin/sort' cmdline='init' size=66336 bytes
[35572609809] [[35mTRACE[0m] [kernel] [CPU0]   Module[36]: name='/bin/env' cmdline='init' size=45832 bytes
[35573117349] [[35mTRACE[0m] [kernel] [CPU0]   Module[37]: name='/bin/uname' cmdline='' size=44632 bytes
[35573658714] [[35mTRACE[0m] [kernel] [CPU0]   Module[38]: name='/bin/true' cmdline='' size=20744 bytes
[35574176781] [[35mTRACE[0m] [kernel] [CPU0]   Module[39]: name='/bin/false' cmdline='' size=20744 bytes
[35574691878] [[35mTRACE[0m] [kernel] [CPU0]   Module[40]: name='/bin/input_echo' cmdline='' size=43016 bytes
[35575274658] [[35mTRACE[0m] [kernel] [CPU0]   Module[41]: name='/drivers/ps2_mouse' cmdline='' size=71096 bytes
[35575819653] [[35mTRACE[0m] [kernel] [CPU0]   Module[42]: name='/drivers/display_bootfb' cmdline='' size=91736 bytes
[35576241294] [[35mTRACE[0m] [kernel] [CPU0]   Module[43]: name='/drivers/display_virtio_gpu' cmdline='' size=147560 bytes
[35576891130] [[35mTRACE[0m] [kernel] [CPU0]   Module[44]: name='/bin/cambium' cmdline='' size=144768 bytes
[35578616568] [[35mTRACE[0m] [kernel] [CPU0]   Module[45]: name='/drivers/virtio_netd' cmdline='' size=123920 bytes
[35579215551] [[35mTRACE[0m] [kernel] [CPU0]   Module[46]: name='/drivers/rtl8168d' cmdline='' size=61392 bytes
[35581221093] [[35mTRACE[0m] [kernel] [CPU0]   Module[47]: name='/bin/netd' cmdline='' size=10819752 bytes
[35581728138] [[35mTRACE[0m] [kernel] [CPU0]   Module[48]: name='/bin/mesocarp' cmdline='' size=123496 bytes
[35582255181] [[35mTRACE[0m] [kernel] [CPU0]   Module[49]: name='/bin/mdns' cmdline='' size=123496 bytes
[35582770905] [[35mTRACE[0m] [kernel] [CPU0]   Module[50]: name='/bin/mdnsd' cmdline='' size=123496 bytes
[35583318408] [[35mTRACE[0m] [kernel] [CPU0]   Module[51]: name='/bin/fetchd' cmdline='' size=76752 bytes
[35583821922] [[35mTRACE[0m] [kernel] [CPU0]   Module[52]: name='/bin/httpsd' cmdline='' size=536128 bytes
[35584320288] [[35mTRACE[0m] [kernel] [CPU0]   Module[53]: name='/bin/find' cmdline='' size=57896 bytes
[35584815981] [[35mTRACE[0m] [kernel] [CPU0]   Module[54]: name='/bin/ip' cmdline='' size=55472 bytes
[35585308242] [[35mTRACE[0m] [kernel] [CPU0]   Module[55]: name='/bin/iso_reader' cmdline='' size=37424 bytes
[35585809050] [[35mTRACE[0m] [kernel] [CPU0]   Module[56]: name='/bin/ping' cmdline='' size=65184 bytes
[35586437667] [[35mTRACE[0m] [kernel] [CPU0]   Module[57]: name='/bin/nslookup' cmdline='' size=56424 bytes
[35586966360] [[35mTRACE[0m] [kernel] [CPU0]   Module[58]: name='/drivers/ahci_disk' cmdline='' size=53784 bytes
[35587787697] [[35mTRACE[0m] [kernel] [CPU0]   Module[59]: name='/drivers/ata_disk' cmdline='' size=64448 bytes
[35589034833] [[35mTRACE[0m] [kernel] [CPU0]   Module[60]: name='/bin/iso9660d' cmdline='' size=78544 bytes
[35589572799] [[35mTRACE[0m] [kernel] [CPU0]   Module[61]: name='/drivers/virtio_sound' cmdline='' size=99296 bytes
[35590121391] [[35mTRACE[0m] [kernel] [CPU0]   Module[62]: name='/drivers/hdaudio' cmdline='' size=67696 bytes
[35590647411] [[35mTRACE[0m] [kernel] [CPU0]   Module[63]: name='/drivers/pci_stubd' cmdline='' size=58168 bytes
[35591198841] [[35mTRACE[0m] [kernel] [CPU0]   Module[64]: name='/drivers/chime' cmdline='' size=59536 bytes
[35594147886] [[35mTRACE[0m] [kernel] [CPU0]   Module[65]: name='/bin/vfs_hello' cmdline='' size=39560 bytes
[35603206485] [[35mTRACE[0m] [kernel] [CPU0]   Module[66]: name='/bin/show_args' cmdline='' size=42488 bytes
[35604057225] [[35mTRACE[0m] [kernel] [CPU0]   Module[67]: name='/bin/clock' cmdline='' size=82936 bytes
[35604714981] [[35mTRACE[0m] [kernel] [CPU0]   Module[68]: name='/bin/env_roundtrip' cmdline='' size=48232 bytes
[35605240110] [[35mTRACE[0m] [kernel] [CPU0]   Module[69]: name='/bin/cwd_test' cmdline='' size=43696 bytes
[35605806555] [[35mTRACE[0m] [kernel] [CPU0]   Module[70]: name='/bin/date' cmdline='' size=67032 bytes
[35606335644] [[35mTRACE[0m] [kernel] [CPU0]   Module[71]: name='/bin/wayland_hello' cmdline='' size=82360 bytes
[35606869617] [[35mTRACE[0m] [kernel] [CPU0]   Module[72]: name='/bin/terminal' cmdline='' size=91056 bytes
[35607371514] [[35mTRACE[0m] [kernel] [CPU0]   Module[73]: name='/bin/tee' cmdline='' size=44968 bytes
[35607864534] [[35mTRACE[0m] [kernel] [CPU0]   Module[74]: name='/bin/xargs' cmdline='' size=55016 bytes
[35608447644] [[35mTRACE[0m] [kernel] [CPU0]   Module[75]: name='/bin/placed' cmdline='' size=38048 bytes
[35608956372] [[35mTRACE[0m] [kernel] [CPU0]   Module[76]: name='/bin/bloom' cmdline='' size=462464 bytes
[35609790645] [[35mTRACE[0m] [kernel] [CPU0]   Module[77]: name='/bin/clear' cmdline='' size=20872 bytes
[35611515456] [[35mTRACE[0m] [kernel] [CPU0]   Module[78]: name='/bin/loglevel' cmdline='' size=41536 bytes
[35612022237] [[35mTRACE[0m] [kernel] [CPU0]   Module[79]: name='/bin/poll_mux' cmdline='' size=39056 bytes
[35614875318] [[35mTRACE[0m] [kernel] [CPU0]   Module[80]: name='/bin/ipc_service_demo' cmdline='' size=47592 bytes
[35615395233] [[35mTRACE[0m] [kernel] [CPU0]   Module[81]: name='/bin/ipc_pipe_demo' cmdline='' size=43936 bytes
[35615903202] [[35mTRACE[0m] [kernel] [CPU0]   Module[82]: name='/bin/ipc_provider_demo' cmdline='' size=84440 bytes
[35616431169] [[35mTRACE[0m] [kernel] [CPU0]   Module[83]: name='/bin/vfs_test_provider' cmdline='' size=70648 bytes
[35616947355] [[35mTRACE[0m] [kernel] [CPU0]   Module[84]: name='/bin/ipc_memfd_demo' cmdline='' size=38232 bytes
[35617455093] [[35mTRACE[0m] [kernel] [CPU0]   Module[85]: name='/bin/test_exec' cmdline='' size=62592 bytes
[35617956990] [[35mTRACE[0m] [kernel] [CPU0]   Module[86]: name='/bin/test_vm_protect' cmdline='' size=38224 bytes
[35618465652] [[35mTRACE[0m] [kernel] [CPU0]   Module[87]: name='/bin/test_exec_env' cmdline='' size=58904 bytes
[35618971608] [[35mTRACE[0m] [kernel] [CPU0]   Module[88]: name='/bin/test_threads' cmdline='' size=82376 bytes
[35619475749] [[35mTRACE[0m] [kernel] [CPU0]   Module[89]: name='/bin/test_futex' cmdline='' size=52992 bytes
[35619995136] [[35mTRACE[0m] [kernel] [CPU0]   Module[90]: name='/bin/ld_so' cmdline='' size=378600 bytes
[35620489674] [[35mTRACE[0m] [kernel] [CPU0]   Module[91]: name='/bin/test_dyn_loader' cmdline='' size=57760 bytes
[35622165249] [[35mTRACE[0m] [kernel] [CPU0]   Module[92]: name='/bin/test_dlopen' cmdline='' size=64688 bytes
[35622788586] [[35mTRACE[0m] [kernel] [CPU0]   Module[93]: name='/bin/reboot' cmdline='' size=32048 bytes
[35624924412] [[35mTRACE[0m] [kernel] [CPU0]   Module[94]: name='/bin/shutdown' cmdline='' size=32048 bytes
[35627664864] [[35mTRACE[0m] [kernel] [CPU0]   Module[95]: name='/bin/attr_list' cmdline='' size=47400 bytes
[35628181248] [[35mTRACE[0m] [kernel] [CPU0]   Module[96]: name='/bin/attr_get' cmdline='' size=47488 bytes
[35628685059] [[35mTRACE[0m] [kernel] [CPU0]   Module[97]: name='/bin/attr_set' cmdline='' size=69600 bytes
[35629185867] [[35mTRACE[0m] [kernel] [CPU0]   Module[98]: name='/bin/attr_rm' cmdline='' size=47008 bytes
[35629719807] [[35mTRACE[0m] [kernel] [CPU0]   Module[99]: name='/lib/libpistil.so' cmdline='' size=826992 bytes
[35630252328] [[35mTRACE[0m] [kernel] [CPU0]   Module[100]: name='/share/wallpapers/clouds.bmp' cmdline='' size=786486 bytes
[35630798577] [[35mTRACE[0m] [kernel] [CPU0]   Module[101]: name='/share/wallpapers/flower.bmp' cmdline='' size=8386614 bytes
[35631347235] [[35mTRACE[0m] [kernel] [CPU0]   Module[102]: name='/share/wallpapers/flower.png' cmdline='' size=2652468 bytes
[35631900777] [[35mTRACE[0m] [kernel] [CPU0]   Module[103]: name='/share/wallpapers/leather.bmp' cmdline='' size=1179702 bytes
[35632443891] [[35mTRACE[0m] [kernel] [CPU0]   Module[104]: name='/share/wallpapers/linen.bmp' cmdline='' size=4718646 bytes
[35632999545] [[35mTRACE[0m] [kernel] [CPU0]   Module[105]: name='/share/fonts/DSEG7Classic-Regular.ttf' cmdline='' size=23272 bytes
[35633545134] [[35mTRACE[0m] [kernel] [CPU0]   Module[106]: name='/share/fonts/Hack-Regular.ttf' cmdline='' size=309408 bytes
[35634068844] [[35mTRACE[0m] [kernel] [CPU0]   Module[107]: name='/share/fonts/Inter-Regular.ttf' cmdline='' size=876576 bytes
[35634598098] [[35mTRACE[0m] [kernel] [CPU0]   Module[108]: name='/share/fonts/Iosevka-Regular.ttf' cmdline='' size=10457376 bytes
[35635149000] [[35mTRACE[0m] [kernel] [CPU0]   Module[109]: name='/share/fonts/JetBrainsMono-Regular.ttf' cmdline='' size=270224 bytes
[35635686372] [[35mTRACE[0m] [kernel] [CPU0]   Module[110]: name='/share/fonts/NotoSans-Regular.ttf' cmdline='' size=569208 bytes
[35636249748] [[35mTRACE[0m] [kernel] [CPU0]   Module[111]: name='/share/fonts/NotoSansSymbol-Regular.ttf' cmdline='' size=258156 bytes
[35636808669] [[35mTRACE[0m] [kernel] [CPU0]   Module[112]: name='/share/fonts/NotoSansSymbol2-Regular.ttf' cmdline='' size=656852 bytes
[35637368976] [[35mTRACE[0m] [kernel] [CPU0]   Module[113]: name='/share/fonts/NotoSerif-Regular.ttf' cmdline='' size=616196 bytes
[35637916677] [[35mTRACE[0m] [kernel] [CPU0]   Module[114]: name='/share/themes/solarized_warm.toml' cmdline='' size=1832 bytes
[35638494342] [[35mTRACE[0m] [kernel] [CPU0]   Module[115]: name='/share/cursors/future/alias.svg' cmdline='' size=9033 bytes
[35639221992] [[35mTRACE[0m] [kernel] [CPU0]   Module[116]: name='/share/cursors/future/all-scroll.svg' cmdline='' size=5170 bytes
[35640138600] [[35mTRACE[0m] [kernel] [CPU0]   Module[117]: name='/share/cursors/future/bottom_left_corner.svg' cmdline='' size=1441 bytes
[35641030359] [[35mTRACE[0m] [kernel] [CPU0]   Module[118]: name='/share/cursors/future/bottom_right_corner.svg' cmdline='' size=3037 bytes
[35641958319] [[35mTRACE[0m] [kernel] [CPU0]   Module[119]: name='/share/cursors/future/bottom_side.svg' cmdline='' size=5432 bytes
[35642823942] [[35mTRACE[0m] [kernel] [CPU0]   Module[120]: name='/share/cursors/future/cell.svg' cmdline='' size=3707 bytes
[35649922506] [[35mTRACE[0m] [kernel] [CPU0]   Module[121]: name='/share/cursors/future/center_ptr.svg' cmdline='' size=9766 bytes
[35650511226] [[35mTRACE[0m] [kernel] [CPU0]   Module[122]: name='/share/cursors/future/col-resize.svg' cmdline='' size=16463 bytes
[35651060676] [[35mTRACE[0m] [kernel] [CPU0]   Module[123]: name='/share/cursors/future/color-picker.svg' cmdline='' size=6296 bytes
[35651603526] [[35mTRACE[0m] [kernel] [CPU0]   Module[124]: name='/share/cursors/future/context-menu.svg' cmdline='' size=9011 bytes
[35652166836] [[35mTRACE[0m] [kernel] [CPU0]   Module[125]: name='/share/cursors/future/copy.svg' cmdline='' size=3527 bytes
[35652725493] [[35mTRACE[0m] [kernel] [CPU0]   Module[126]: name='/share/cursors/future/crosshair.svg' cmdline='' size=16918 bytes
[35653536897] [[35mTRACE[0m] [kernel] [CPU0]   Module[127]: name='/share/cursors/future/default.svg' cmdline='' size=3079 bytes
[35654760405] [[35mTRACE[0m] [kernel] [CPU0]   Module[128]: name='/share/cursors/future/dnd-move.svg' cmdline='' size=3569 bytes
[35655761922] [[35mTRACE[0m] [kernel] [CPU0]   Module[129]: name='/share/cursors/future/dnd-no-drop.svg' cmdline='' size=4753 bytes
[35661241638] [[35mTRACE[0m] [kernel] [CPU0]   Module[130]: name='/share/cursors/future/down-arrow.svg' cmdline='' size=1820 bytes
[35675075205] [[35mTRACE[0m] [kernel] [CPU0]   Module[131]: name='/share/cursors/future/draft.svg' cmdline='' size=3136 bytes
[35676182883] [[35mTRACE[0m] [kernel] [CPU0]   Module[132]: name='/share/cursors/future/fleur.svg' cmdline='' size=28469 bytes
[35681872974] [[35mTRACE[0m] [kernel] [CPU0]   Module[133]: name='/share/cursors/future/help.svg' cmdline='' size=5873 bytes
[35682456447] [[35mTRACE[0m] [kernel] [CPU0]   Module[134]: name='/share/cursors/future/left-arrow.svg' cmdline='' size=1785 bytes
[35682996954] [[35mTRACE[0m] [kernel] [CPU0]   Module[135]: name='/share/cursors/future/left_side.svg' cmdline='' size=3848 bytes
[35683532676] [[35mTRACE[0m] [kernel] [CPU0]   Module[136]: name='/share/cursors/future/no-drop.svg' cmdline='' size=3861 bytes
[35684063151] [[35mTRACE[0m] [kernel] [CPU0]   Module[137]: name='/share/cursors/future/not-allowed.svg' cmdline='' size=1567 bytes
[35684598609] [[35mTRACE[0m] [kernel] [CPU0]   Module[138]: name='/share/cursors/future/openhand.svg' cmdline='' size=3435 bytes
[35685266958] [[35mTRACE[0m] [kernel] [CPU0]   Module[139]: name='/share/cursors/future/pencil.svg' cmdline='' size=6527 bytes
[35686009953] [[35mTRACE[0m] [kernel] [CPU0]   Module[140]: name='/share/cursors/future/pirate.svg' cmdline='' size=6663 bytes
[35687065458] [[35mTRACE[0m] [kernel] [CPU0]   Module[141]: name='/share/cursors/future/pointer.svg' cmdline='' size=2922 bytes
[35688463008] [[35mTRACE[0m] [kernel] [CPU0]   Module[142]: name='/share/cursors/future/progress-01.svg' cmdline='' size=11030 bytes
[35689080603] [[35mTRACE[0m] [kernel] [CPU0]   Module[143]: name='/share/cursors/future/progress-02.svg' cmdline='' size=12198 bytes
[35689722618] [[35mTRACE[0m] [kernel] [CPU0]   Module[144]: name='/share/cursors/future/progress-03.svg' cmdline='' size=12733 bytes
[35690367834] [[35mTRACE[0m] [kernel] [CPU0]   Module[145]: name='/share/cursors/future/progress-04.svg' cmdline='' size=13777 bytes
[35691292230] [[35mTRACE[0m] [kernel] [CPU0]   Module[146]: name='/share/cursors/future/progress-05.svg' cmdline='' size=13835 bytes
[35693558538] [[35mTRACE[0m] [kernel] [CPU0]   Module[147]: name='/share/cursors/future/progress-06.svg' cmdline='' size=14907 bytes
[35707807377] [[35mTRACE[0m] [kernel] [CPU0]   Module[148]: name='/share/cursors/future/progress-07.svg' cmdline='' size=14805 bytes
[35714140407] [[35mTRACE[0m] [kernel] [CPU0]   Module[149]: name='/share/cursors/future/progress-08.svg' cmdline='' size=15996 bytes
[35722392057] [[35mTRACE[0m] [kernel] [CPU0]   Module[150]: name='/share/cursors/future/progress-09.svg' cmdline='' size=16010 bytes
[35723018496] [[35mTRACE[0m] [kernel] [CPU0]   Module[151]: name='/share/cursors/future/progress-10.svg' cmdline='' size=17080 bytes
[35723572764] [[35mTRACE[0m] [kernel] [CPU0]   Module[152]: name='/share/cursors/future/progress-11.svg' cmdline='' size=17096 bytes
[35724328002] [[35mTRACE[0m] [kernel] [CPU0]   Module[153]: name='/share/cursors/future/progress-12.svg' cmdline='' size=17079 bytes
[35726485311] [[35mTRACE[0m] [kernel] [CPU0]   Module[154]: name='/share/cursors/future/progress-13.svg' cmdline='' size=15900 bytes
[35731598958] [[35mTRACE[0m] [kernel] [CPU0]   Module[155]: name='/share/cursors/future/progress-14.svg' cmdline='' size=16001 bytes
[35732173686] [[35mTRACE[0m] [kernel] [CPU0]   Module[156]: name='/share/cursors/future/progress-15.svg' cmdline='' size=14931 bytes
[35732771811] [[35mTRACE[0m] [kernel] [CPU0]   Module[157]: name='/share/cursors/future/progress-16.svg' cmdline='' size=14873 bytes
[35733335517] [[35mTRACE[0m] [kernel] [CPU0]   Module[158]: name='/share/cursors/future/progress-17.svg' cmdline='' size=13846 bytes
[35733920013] [[35mTRACE[0m] [kernel] [CPU0]   Module[159]: name='/share/cursors/future/progress-18.svg' cmdline='' size=13835 bytes
[35734543779] [[35mTRACE[0m] [kernel] [CPU0]   Module[160]: name='/share/cursors/future/progress-19.svg' cmdline='' size=12645 bytes
[35735211072] [[35mTRACE[0m] [kernel] [CPU0]   Module[161]: name='/share/cursors/future/progress-20.svg' cmdline='' size=12741 bytes
[35735851107] [[35mTRACE[0m] [kernel] [CPU0]   Module[162]: name='/share/cursors/future/progress-21.svg' cmdline='' size=11660 bytes
[35736499194] [[35mTRACE[0m] [kernel] [CPU0]   Module[163]: name='/share/cursors/future/progress-22.svg' cmdline='' size=11612 bytes
[35737118175] [[35mTRACE[0m] [kernel] [CPU0]   Module[164]: name='/share/cursors/future/progress-23.svg' cmdline='' size=11674 bytes
[35737743030] [[35mTRACE[0m] [kernel] [CPU0]   Module[165]: name='/share/cursors/future/progress.svg' cmdline='' size=11642 bytes
[35738327658] [[35mTRACE[0m] [kernel] [CPU0]   Module[166]: name='/share/cursors/future/right-arrow.svg' cmdline='' size=3432 bytes
[35738892684] [[35mTRACE[0m] [kernel] [CPU0]   Module[167]: name='/share/cursors/future/right_ptr.svg' cmdline='' size=5155 bytes
[35739654159] [[35mTRACE[0m] [kernel] [CPU0]   Module[168]: name='/share/cursors/future/right_side.svg' cmdline='' size=6447 bytes
[35740423191] [[35mTRACE[0m] [kernel] [CPU0]   Module[169]: name='/share/cursors/future/row-resize.svg' cmdline='' size=15825 bytes
[35741314224] [[35mTRACE[0m] [kernel] [CPU0]   Module[170]: name='/share/cursors/future/size_bdiag.svg' cmdline='' size=16323 bytes
[35742430515] [[35mTRACE[0m] [kernel] [CPU0]   Module[171]: name='/share/cursors/future/size_fdiag.svg' cmdline='' size=16589 bytes
[35745085332] [[35mTRACE[0m] [kernel] [CPU0]   Module[172]: name='/share/cursors/future/size_hor.svg' cmdline='' size=16255 bytes
[35745644319] [[35mTRACE[0m] [kernel] [CPU0]   Module[173]: name='/share/cursors/future/size_ver.svg' cmdline='' size=15822 bytes
[35746235679] [[35mTRACE[0m] [kernel] [CPU0]   Module[174]: name='/share/cursors/future/text.svg' cmdline='' size=5584 bytes
[35746876539] [[35mTRACE[0m] [kernel] [CPU0]   Module[175]: name='/share/cursors/future/top_left_corner.svg' cmdline='' size=7121 bytes
[35747424636] [[35mTRACE[0m] [kernel] [CPU0]   Module[176]: name='/share/cursors/future/top_right_corner.svg' cmdline='' size=7139 bytes
[35748118527] [[35mTRACE[0m] [kernel] [CPU0]   Module[177]: name='/share/cursors/future/top_side.svg' cmdline='' size=10651 bytes
[35748738828] [[35mTRACE[0m] [kernel] [CPU0]   Module[178]: name='/share/cursors/future/up-arrow.svg' cmdline='' size=7653 bytes
[35749292898] [[35mTRACE[0m] [kernel] [CPU0]   Module[179]: name='/share/cursors/future/vertical-text.svg' cmdline='' size=5621 bytes
[35749843470] [[35mTRACE[0m] [kernel] [CPU0]   Module[180]: name='/share/cursors/future/wait-01.svg' cmdline='' size=15308 bytes
[35750381931] [[35mTRACE[0m] [kernel] [CPU0]   Module[181]: name='/share/cursors/future/wait-02.svg' cmdline='' size=6672 bytes
[35750932503] [[35mTRACE[0m] [kernel] [CPU0]   Module[182]: name='/share/cursors/future/wait-03.svg' cmdline='' size=6686 bytes
[35751515778] [[35mTRACE[0m] [kernel] [CPU0]   Module[183]: name='/share/cursors/future/wait-04.svg' cmdline='' size=7827 bytes
[35752210494] [[35mTRACE[0m] [kernel] [CPU0]   Module[184]: name='/share/cursors/future/wait-05.svg' cmdline='' size=7835 bytes
[35752770339] [[35mTRACE[0m] [kernel] [CPU0]   Module[185]: name='/share/cursors/future/wait-06.svg' cmdline='' size=8976 bytes
[35753780700] [[35mTRACE[0m] [kernel] [CPU0]   Module[186]: name='/share/cursors/future/wait-07.svg' cmdline='' size=8989 bytes
[35754319821] [[35mTRACE[0m] [kernel] [CPU0]   Module[187]: name='/share/cursors/future/wait-08.svg' cmdline='' size=10126 bytes
[35754889665] [[35mTRACE[0m] [kernel] [CPU0]   Module[188]: name='/share/cursors/future/wait-09.svg' cmdline='' size=10140 bytes
[35755426410] [[35mTRACE[0m] [kernel] [CPU0]   Module[189]: name='/share/cursors/future/wait-10.svg' cmdline='' size=11277 bytes
[35756074662] [[35mTRACE[0m] [kernel] [CPU0]   Module[190]: name='/share/cursors/future/wait-11.svg' cmdline='' size=11291 bytes
[35756802213] [[35mTRACE[0m] [kernel] [CPU0]   Module[191]: name='/share/cursors/future/wait-12.svg' cmdline='' size=11277 bytes
[35758710603] [[35mTRACE[0m] [kernel] [CPU0]   Module[192]: name='/share/cursors/future/wait-13.svg' cmdline='' size=10140 bytes
[35762488278] [[35mTRACE[0m] [kernel] [CPU0]   Module[193]: name='/share/cursors/future/wait-14.svg' cmdline='' size=10126 bytes
[35763060597] [[35mTRACE[0m] [kernel] [CPU0]   Module[194]: name='/share/cursors/future/wait-15.svg' cmdline='' size=8991 bytes
[35763597078] [[35mTRACE[0m] [kernel] [CPU0]   Module[195]: name='/share/cursors/future/wait-16.svg' cmdline='' size=8978 bytes
[35764129368] [[35mTRACE[0m] [kernel] [CPU0]   Module[196]: name='/share/cursors/future/wait-17.svg' cmdline='' size=7842 bytes
[35764698981] [[35mTRACE[0m] [kernel] [CPU0]   Module[197]: name='/share/cursors/future/wait-18.svg' cmdline='' size=7829 bytes
[35765241039] [[35mTRACE[0m] [kernel] [CPU0]   Module[198]: name='/share/cursors/future/wait-19.svg' cmdline='' size=6822 bytes
[35765773263] [[35mTRACE[0m] [kernel] [CPU0]   Module[199]: name='/share/cursors/future/wait-20.svg' cmdline='' size=6679 bytes
[35766304167] [[35mTRACE[0m] [kernel] [CPU0]   Module[200]: name='/share/cursors/future/wait-21.svg' cmdline='' size=5542 bytes
[35766835533] [[35mTRACE[0m] [kernel] [CPU0]   Module[201]: name='/share/cursors/future/wait-22.svg' cmdline='' size=5528 bytes
[35767368252] [[35mTRACE[0m] [kernel] [CPU0]   Module[202]: name='/share/cursors/future/wait-23.svg' cmdline='' size=5546 bytes
[35767916547] [[35mTRACE[0m] [kernel] [CPU0]   Module[203]: name='/share/cursors/future/wait.svg' cmdline='' size=5525 bytes
[35768449167] [[35mTRACE[0m] [kernel] [CPU0]   Module[204]: name='/share/cursors/future/wayland-cursor.svg' cmdline='' size=8222 bytes
[35768989773] [[35mTRACE[0m] [kernel] [CPU0]   Module[205]: name='/share/cursors/future/x-cursor.svg' cmdline='' size=6822 bytes
[35769525528] [[35mTRACE[0m] [kernel] [CPU0]   Module[206]: name='/share/cursors/future/zoom-in.svg' cmdline='' size=5441 bytes
[35770053825] [[35mTRACE[0m] [kernel] [CPU0]   Module[207]: name='/share/cursors/future/zoom-out.svg' cmdline='' size=5403 bytes
[35770628949] [[35mTRACE[0m] [kernel] [CPU0]   Module[208]: name='/share/fonts/unifont.hex' cmdline='' size=8355852 bytes
[35771175363] [[35mTRACE[0m] [kernel] [CPU0]   Module[209]: name='/etc/locale.conf' cmdline='' size=85 bytes
[35771683002] [[35mTRACE[0m] [kernel] [CPU0]   Module[210]: name='/etc/profile' cmdline='' size=68 bytes
[35772183843] [[35mTRACE[0m] [kernel] [CPU0]   Module[211]: name='/etc/motd' cmdline='' size=610 bytes
[35772754215] [[35mTRACE[0m] [kernel] [CPU0]   Module[212]: name='/etc/fstab' cmdline='' size=127 bytes
[35773537173] [[35mTRACE[0m] [kernel] [CPU0]   Module[213]: name='/etc/hostname' cmdline='' size=8 bytes
[35774746359] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[36037685739] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[36039078933] [[34mDEBUG[0m] [kernel] [CPU0] Found init module: /bin/sprout (cmdline: 'init'), loading...
[36040656366] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/sprout' (len=89912, base=0x200000)
[36041457573] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[36054578142] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[36057098385] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[36061882296] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [49, 3b, 6e, 18, 75, 13, 49, 8d]
[36066840909] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20c000 exec=false
[36068721843] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20f000 exec=false
[36076463643] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x210000 filesz=0 memsz=0 align=1
[36084564186] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='load_init_module' elapsed_ticks=44147004 elapsed_us=22073 total_ticks=1107150363 total_us=553575
[36086351862] [[34mDEBUG[0m] [kernel] [CPU0] Warning: Module registry page overflow, truncating list.
[36087175377] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='prepare_boot_registry' elapsed_ticks=1880802 elapsed_us=940 total_ticks=1109768484 total_us=554884
[36088218375] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='map_boot_registry' elapsed_ticks=170940 elapsed_us=85 total_ticks=1110821811 total_us=555410
[36089202303] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='activate_init_aspace' elapsed_ticks=178200 elapsed_us=89 total_ticks=1111771584 total_us=555885
[36090021495] [[34mDEBUG[0m] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[36090591702] [[34mDEBUG[0m] [kernel] [CPU0] Spawning init process...
[36092588070] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb006a9a0 arg=0xffffffffb0020a20
[36107709858] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[36108634617] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=5
[36115637382] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[36116274942] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='spawn_init_task' elapsed_ticks=25767258 elapsed_us=12883 total_ticks=1138868940 total_us=569434
[36117207753] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[36258859956] [[34mDEBUG[0m] [kernel::sched::lifecycle] [CPU0] SCHED: early-boot bringup complete; resuming steady-state SMP scheduling
[36259689213] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='end_bringup' elapsed_ticks=1360194 elapsed_us=680 total_ticks=1282282023 total_us=641141
[36260391057] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[36520898601] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1543256286 elapsed_us=771628
[36521840817] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[36539691903] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[36540719952] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] ENTER_USER: TID=5 entry_pc=200000 user_sp=800000
[36594118374] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpus_online=4
[36605105196] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=0 curr=Some(5) runq=1 runq_avg=2 runq_samples=1 ctxsw=1 idle2busy=0 tick=1 ipi=0 enq=3 deq=1 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[36606480834] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=1 curr=Some(2) runq=0 runq_avg=0 runq_samples=40 ctxsw=0 idle2busy=0 tick=35 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[36607357512] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=2 curr=Some(3) runq=0 runq_avg=0 runq_samples=41 ctxsw=0 idle2busy=0 tick=35 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[36608200464] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=3 curr=Some(4) runq=0 runq_avg=0 runq_samples=40 ctxsw=0 idle2busy=0 tick=35 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[36621950475] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=0
[36627593607] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=20
[36639572079] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[36641759715] [[34mDEBUG[0m] [sprout] [CPU0] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7fef70 rip=0x2032e5 rflags=0x202
[36643990185] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[36645436938] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: About to create Supervisor...
[36646784064] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Listing /bin directory...
[36649448286] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/bin' tid=5
[36682690143] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=3
[36690405576] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[36724571664] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Parsed command line: '    '
[36728687292] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Supervisor created, calling run_forever...
[36730505592] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[36743176833] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[36745158186] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[36746836236] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/run/sprout/shell' tid=5
[36752389773] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/etc/default/shell' tid=5
[36755137386] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/bin/sh' tid=5
[36759251760] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=3
[36763329900] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[36769326528] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[36772405758] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[36802254390] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN: page cache miss for '/bin/sh', caching 127864 bytes
[36807678831] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'sh' (len=127864, base=0x200000)
[36808354077] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[36809458983] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[36810046218] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[36812313681] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [0f, 84, 32, 06, 00, 00, 48, 8b]
[36825307893] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=214000 exec=false
[36827283339] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=217000 exec=false
[36828654720] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x218000 filesz=0 memsz=0 align=1
[36839875215] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /bin/sh
[36841899534] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb00a3d40 arg=0xffffffffb0074560
[36845518512] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=6, applying inserts
[36846345822] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[36846851448] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=6
[36850610115] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[36851219460] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[36859961358] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SETUP_STDIO: stdin=Fd(3) stdout=Fd(4) stderr=Fd(5)
[36879550851] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 6
[36887813325] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Task 6 woken, restoring IRQs
[36888603411] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Done for /bin/sh
[36890386764] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[36898372170] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[36900446022] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=3
[36901264983] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[36902227791] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=6 entry_pc=200000 user_sp=800000
[36903110706] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=0
[36904527957] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=4
[36904803870] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=100
[36908995299] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=5
[36912393210] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[36913972227] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[36917175735] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/run/motd' tid=6
[36919983870] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/etc/motd' tid=6
[36923253906] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=6
[36925561794] [[34mDEBUG[0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=19767
[36931794240] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=610
[1;32m
        .-.
       /   \        [1;36mTHING-OS[1;32m
      |     |       [0;36m"People, places, things."[1;32m
       \   /        
        `-'        
       /   \        v0.1  ��  
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
[36937302303] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/etc/profile' tid=6
[36966576570] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=6
[36978080766] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=97
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [36984431319] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=6
[?25h[36986767455] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[37070267058] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[37072595934] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[37077274344] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/cambium' (len=144768, base=0x200000)
[37077917448] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[37078986747] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[37079593386] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[37081650969] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [8d, 3c, f0, 48, 83, c7, 70, 48]
[37093058409] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=217000 exec=false
[37100288412] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[37101739884] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x21b000 filesz=0 memsz=0 align=1
[37109134293] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb00bd7a0 arg=0xffffffffb00ad320
[37111689054] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[37114246125] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=7
[37114953942] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[37122152628] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 7 → task '/bin/cambium' (pid=7 from boot module)
[37127843445] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[37128882252] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=7 entry_pc=200000 user_sp=800000
[37131357186] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=0
[37133098959] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=100
[37134285342] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned cambium (PID=7)
[37137022692] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[37139488947] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[37142255931] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Deferring netd until /dev/net/virtio0/rx is ready...
[37144714530] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[37147231242] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[37149736239] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[37152089634] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[37153456428] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[37165078104] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN: page cache miss for '/bin/bristle', caching 68744 bytes
[37165161957] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[37166512449] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers' tid=7
[37168790769] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'bristle' (len=68744, base=0x200000)
[37169410245] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_readdir: fd=3 len=4096
[37169357346] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[37170788358] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[37171173666] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_readdir: natural phase begin offset=0
[37171610058] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[37174185741] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [cc, fc, ff, ff, 45, 31, ff, e9]
[37186437354] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=209000 exec=false
[37187818800] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20b000 exec=false
[37189027788] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20c000 filesz=0 memsz=0 align=1
[37193133681] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /bin/bristle
[37193795661] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb00e9360 arg=0xffffffffb00d78e0
[37195303926] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=8, applying inserts
[37195869447] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[37196356098] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=8
[37196893404] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[37197380451] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[37199244093] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SETUP_STDIO: stdin=Null stdout=Inherit stderr=Inherit
[37200368799] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[37204768755] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ahci_disk' tid=7
[37204291311] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 8
[37213294635] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Task 8 woken, restoring IRQs
[37214140689] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Done for /bin/bristle
[37215042843] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/bristle' TID=8 PID=8
[37215497550] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[37216558830] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=8 entry_pc=200000 user_sp=800000
[37217902095] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=0
[37219485501] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=100
[37238554089] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/session/active_ui' tid=8
[37250442834] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=3
[37260477837] [[35mTRACE[0m] [bristle] [CPU2] bristle: active_ui set to 'bloom'
[37263519579] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[37267417275] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/run/bristle/pid' tid=8
[37267655667] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[37271472975] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[37273791786] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[37276206297] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/fb0' tid=5
[37277251935] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_open: path='/dev/fb0' len=8 flags=0x0
[37279382580] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] devfs: lookup entry path='fb0' len=3
[37280800227] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] devfs: dynamic registry hit path='fb0'
[37282925163] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_open: resolved node for /dev/fb0 mode=0o20666 size=32 ino=4
[37283804679] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_open: process info present for /dev/fb0
[37284574833] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_open: handle_table.open('/dev/fb0') -> 3
[37293189120] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] FbNode::read: off=0 n=32 buf_len=32 total=32
[37294980756] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=3
[37297618974] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[37299542775] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/drivers/display_virtio_gpu' tid=5
[37302595539] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=3
[37304410275] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=3
[37304685396] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[37307279295] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[37309590813] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[37312375122] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=4096
[37314678258] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(0)
[37315496361] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=4096
[37316472765] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(1)
[37323809226] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=1 port_id=PortId(1) mode=Write
[37323809226] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=1 port_id=PortId(0) mode=Write
[37326319107] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=2 port_id=PortId(1) mode=Read
[37326342801] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=2 port_id=PortId(0) mode=Read
[37329149253] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=4096
[37329617424] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(2)
[37330672566] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(2) mode=Write
[37331393319] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=4 port_id=PortId(2) mode=Read
[37335689457] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/run/bristle/kbd_in' tid=8
[37343516430] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[37345523292] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=3
[37346270511] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=4096
[37346858208] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/run/bristle/mouse_in' tid=8
[37346899590] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(3)
[37347948132] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=3 port_id=PortId(3) mode=Write
[37348635027] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=4 port_id=PortId(3) mode=Read
[37350671127] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=3
[37352233545] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[37352977728] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/proc/self/inbox' tid=8
[37357027059] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[37361524101] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=2 -> fd=4 node=0xffffffffb00f1a70 port=0xffffffffb00f46b0
[37363278546] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[37364254356] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=4 -> fd=5 node=0xffffffffb01f9410 port=0xffffffffb00f71b0
[37366569273] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[37373980347] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[37396553964] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN: page cache miss for '/drivers/display_virtio_gpu', caching 147560 bytes
[37398217725] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'display_virtio_gpu' (len=147560, base=0x200000)
[37398897096] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[37400086977] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[37401139149] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[37408880817] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [00, 00, 00, ba, 12, 00, 00, 00]
[37419358350] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=216000 exec=false
[37421729301] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[37423398606] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x21c000 filesz=0 memsz=0 align=1
[37427610000] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/display_virtio_gpu
[37428359001] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb022daa0 arg=0xffffffffb021d880
[37429824465] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=9, applying inserts
[37430365269] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[37430837334] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=9
[37431434931] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[37431895380] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[37434273789] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SETUP_STDIO: stdin=Null stdout=Inherit stderr=Inherit
[37449384324] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 9
[37450612881] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Task 9 woken, restoring IRQs
[37451279943] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Done for /drivers/display_virtio_gpu
[37452014688] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/drivers/display_virtio_gpu' TID=9 PID=9
[37453701648] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x3
[37454937663] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=9 entry_pc=200000 user_sp=800000
[37456742466] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=0
[37458521166] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=100
[37463058204] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[37464361803] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[37465107570] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[37467395493] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0 mounted=false, service_ready=false)...
[37467593757] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Starting VFS-native VirtIO GPU driver...
[37469175282] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: boot_arg=3
[37471031004] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Mapping bootstrap memfd 3 size=4096...
[37473623649] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready for bloom yet
[37474726608] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: vm_map success at 0x400000001000
[37476506793] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Recovered handles: req_read=2, resp_write=3, svc=0, id=322371585
[37478287209] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3, svc=0, id=322371585)
[37479618759] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Scanning /sys/devices for PCI GPU...
[37480890183] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices' tid=9
[37483649016] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[37483714851] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices'
[37488621489] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[37489804770] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_readdir: fd=4 len=4096
[37491073092] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_readdir: natural phase begin offset=0
[37493802720] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[37496066487] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/proc/self/inbox' tid=5
[37496273364] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU3] sysfs: readdir found 12 slots
[37496984349] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[37503055161] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU3] sysfs: readdir wrote 173 bytes
[37504292496] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[37505061297] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[37507780398] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ata_disk' tid=7
[37507739775] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37545116862] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/platform-fb000000/vendor' tid=9
[37547723961] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/platform-fb000000/vendor'
[37552584036] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'platform-fb000000/vendor'
[37557696396] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37561622505] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/platform-fb000000/device' tid=9
[37563785721] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/platform-fb000000/device'
[37566363615] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'platform-fb000000/device'
[37570229334] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37572016614] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/platform-fb000000/class' tid=9
[37574384760] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/platform-fb000000/class'
[37576557150] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'platform-fb000000/class'
[37579605393] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37581461511] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/vendor' tid=9
[37583408148] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:00.0/vendor'
[37585058049] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:00.0/vendor'
[37588492755] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37590362040] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/device' tid=9
[37594757376] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:00.0/device'
[37596634911] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:00.0/device'
[37601186832] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37602310185] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/class' tid=9
[37603934643] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:00.0/class'
[37604910453] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:00.0/class'
[37606689714] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37607763699] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/vendor' tid=9
[37609113564] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/vendor'
[37610301300] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:01.0/vendor'
[37613190681] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37614949053] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/device' tid=9
[37617502098] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/device'
[37619551431] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:01.0/device'
[37622996103] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37624427148] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/class' tid=9
[37626102261] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/class'
[37627452456] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:01.0/class'
[37629242145] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37636792215] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: task 9 claimed device 'pci-0000:00:01.0' (handle 0)
[37638388227] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/virtio/common_bar' tid=9
[37639895469] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/common_bar'
[37647523155] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37650258063] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/virtio/common_offset' tid=9
[37652812857] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/common_offset'
[37657204332] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37659382662] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/virtio/notify_bar' tid=9
[37661117736] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/notify_bar'
[37665447468] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37667154789] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/virtio/notify_offset' tid=9
[37669982163] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/notify_offset'
[37673882004] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37675522599] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/virtio/notify_multiplier' tid=9
[37678449633] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/notify_multiplier'
[37681693269] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37683565062] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[37686662079] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: mapping BAR2 (phys=0xc000000000, size=0x4000) for task 9
[37687842390] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[37689513840] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10000000
[37691382828] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[37693035435] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: DMA alloc 1 pages phys=0x2329000 -> user_va=0x10004000
[37693439586] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/chime' tid=7
[37700304312] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[37705243983] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: DMA alloc 4 pages phys=0x232a000 -> user_va=0x10005000
[37707789306] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[37708923681] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Virgl 3D not supported, using 2D only
[37709831577] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/firmware/framebuffer' tid=9
[37712766432] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='firmware/framebuffer'
[37716213480] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37720560834] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=1
[37831083213] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[37833210525] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[37835378889] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[37837130496] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[37839120264] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0 mounted=false, service_ready=false)...
[37842132141] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready for bloom yet
[37844353239] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[37846686372] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[37850968485] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[37853214432] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[37868461488] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[37870781751] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[37871917281] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/display_bootfb' tid=7
[38042076468] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[38044814940] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[38045885262] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/display_virtio_gpu' tid=7
[38183882715] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[38186415564] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[38189115921] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[38190933726] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[38192397375] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0 mounted=false, service_ready=false)...
[38194213959] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready for bloom yet
[38195600256] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[38197164027] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[38198509008] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[38199839535] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[38202076704] [[34mDEBUG[0m] [kernel::syscall::handlers::memory] [CPU3] sys_vm_map: large backing file mapping offset=0 len=8294400 total_size=8294400
[38220254787] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[38223501756] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[38224709688] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/hdaudio' tid=7
[38289399918] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: frame pool ready (1 buffer)
[38295130665] [[34mDEBUG[0m] [kernel::ipc] [CPU3] CREATE_PORT: capacity=65536
[38295790929] [[34mDEBUG[0m] [kernel::ipc] [CPU3] CREATE_PORT: appended id=PortId(4)
[38296485282] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU3] ALLOC_HANDLE: handle=5 port_id=PortId(4) mode=Write
[38297237682] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU3] ALLOC_HANDLE: handle=6 port_id=PortId(4) mode=Read
[38299240155] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU3] BRIDGE: handle=5 -> fd=5 node=0xffffffffb00ad330 port=0xffffffffb0108310
[38301827058] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/proc/5/inbox' tid=9
[38332818249] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU3] SENDMSG: thing=6 data_len=44 caps_len=1 node=Pointer { addr: 0xffffffffb0108750, metadata: DynMetadata(0xffffffff802c6ec0) }
[38341254336] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_BIND_READY (ID: 322371585)
[38342613969] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Waiting for BIND_ASSIGNED...
[38353549113] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU0] RECVMSG: thing=4 data_len=44 caps_len=1 node=Pointer { addr: 0xffffffffb00d91b0, metadata: DynMetadata(0xffffffff802c6ec0) }
[38366158050] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: BIND_READY from instance_id=0x13370001
[38369287374] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Registering DISPLAY_CARD at /dev/display/card0 (handle=5)
[38378530674] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=262400
[38379258687] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(5)
[38381823843] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=7 port_id=PortId(5) mode=Write
[38388091236] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] vfs: mounted userland provider at /dev/display/card0 (flags: 0x0) port=0xffffffffb0108310
[38394760932] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[38397945960] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Handshake complete — task 'display' marked ready
[38399886261] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[38401923483] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[38402943447] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/pci_stubd' tid=7
[38406564042] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[38407118508] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Sent MSG_BIND_ASSIGNED to display pid 9 (res=Ok(()))
[38408006439] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU3] SENDMSG: thing=6 data_len=44 caps_len=0 node=Pointer { addr: 0xffffffffb0108750, metadata: DynMetadata(0xffffffff802c6ec0) }
[38408830713] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[38409945090] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_SERVICE_READY.
[38411161305] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU3] BRIDGE: handle=2 -> fd=7 node=0xffffffffb00f7590 port=0xffffffffb00f5430
[38412454047] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU3] BRIDGE: handle=3 -> fd=8 node=0xffffffffb0119610 port=0xffffffffb01f8ff0
[38413517769] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU3] BRIDGE: handle=6 -> fd=9 node=0xffffffffb01197b0 port=0xffffffffb0108310
[38415677685] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[38423386716] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU0] RECVMSG: thing=4 data_len=44 caps_len=0 node=Pointer { addr: 0xffffffffb00d91b0, metadata: DynMetadata(0xffffffff802c6ec0) }
[38426861451] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: SERVICE_READY from instance_id=0x13370001
[38428963254] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Service 'display' reported ready
[38430500361] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[38431999848] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[38563662390] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[38566050336] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[38567125839] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ps2_kbd' tid=7
[38731577478] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[38733966414] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[38735041158] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ps2_mouse' tid=7
[38743703427] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[38746071045] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[38748312735] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[38750612571] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[38752860102] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0 mounted=true, service_ready=true)...
[38755461723] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/bloom' (len=462464, base=0x200000)
[38756074335] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[38757308139] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[38759512671] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[38764024530] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [29, 44, 24, 60, 48, 8b, 44, 24]
[38807729070] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=253000 exec=false
[38812298316] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=25a000 exec=false
[38814564360] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x25d000 filesz=0 memsz=0 align=1
[38819420013] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb01398a0 arg=0xffffffffb00d91c0
[38823237618] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[38823747336] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=10
[38825818614] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[38829489105] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 10 → task '/bin/bloom' (pid=10 from boot module)
[38849040384] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[38851792914] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[38853482448] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[38855020083] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[38856311472] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[38908620399] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[38910529647] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=10 entry_pc=200000 user_sp=800000
[38915102424] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=0
[38916546603] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=100
[38919258609] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[38920212606] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[38922115023] [[34mDEBUG[0m] [bloom] [CPU1] bloom: connect try 0...
[38922909696] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/dev/display/card0' tid=10
[38925185277] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=Lookup len=4 tid=10
[38936170746] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[38938466820] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[38942083686] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/rtc_cmos' tid=7
[38949780870] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=Lookup
[38951716881] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: vfs_lookup path=''
[38953310088] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=Lookup req_id=1
[38957664438] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=Lookup req_id=1
[39064503225] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=Lookup id=1 -> OK(8)
[39082980123] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=48 tid=10
[39112852350] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[39114034443] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_GET_INFO requested
[39116196141] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: Returning dimensions 1920x1080
[39117102024] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=2
[39118948506] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=2
[39171548493] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=2 -> OK(48)
[39174482589] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[39176712828] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[39178106847] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[39178872975] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[39181634184] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[39183028929] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[39184580622] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[39186376152] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[39189099576] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[39199033599] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1920x1080 @ 60000mHz
[39200233446] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[39201241530] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[39202748046] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[39208593402] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=65536
[39209256306] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(6)
[39209860173] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=5 port_id=PortId(6) mode=Write
[39210536970] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=6 port_id=PortId(6) mode=Read
[39213743415] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/services/bloom' tid=10
[39218727009] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[39229736799] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/components' tid=10
[39233868465] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[39235065540] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/status' tid=10
[39240977985] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[39243020058] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[39243970854] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/rtl8168d' tid=7
[39404887005] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[39407057481] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[39408072363] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/virtio_netd' tid=7
[39502345245] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[39504470709] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[39506214891] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[39508325769] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[39510118230] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[39511770606] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[39513213498] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[39514479345] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[39576499149] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[39578945505] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[39579953820] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/virtio_sound' tid=7
[39675263100] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[39678108789] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/lib/libpistil.so' tid=10
[39833576277] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[39835211955] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[39837003030] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[39838948809] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[39840615177] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[39842347050] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[39843817365] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[39845632695] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[39987957141] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[40050704001] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[40051819137] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[40052539659] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[40053404820] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil nine-slice shadow renderer loaded
[40054251171] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/desktop/theme' tid=10
[40057312086] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/desktop/theme' tid=10
[40063915749] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[40066094772] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Solarized Warm
[40162346367] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[40163975973] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[40165411473] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[40166790246] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[40168418466] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[40170198981] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[40171995468] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[40173334740] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[40478184318] [[34mDEBUG[0m] [kernel::syscall::handlers::memory] [CPU1] sys_vm_map: large backing file mapping offset=0 len=8294400 total_size=8294400
[40505566134] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[40507430304] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[40509319455] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[40511178939] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[40512716376] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[40514532300] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[40516251534] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[40517828175] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[40569413445] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=88 tid=10
[40598063253] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[40601330616] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_IMPORT_BUFFER requested: memfd=10, size=1920x1080
[40602881814] [[34mDEBUG[0m] [kernel::syscall::handlers::memory] [CPU3] sys_vm_map: large backing file mapping offset=0 len=8294400 total_size=8294400
[40652064486] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[40655788536] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[40658211066] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[40659563901] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/device_snapshot' tid=7
[40661320854] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='device_snapshot'
[40669130601] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[40674610911] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:00.0 kind=pci_device vendor=0x8086 device=0x29c0 class=0x060000 present=true
[40676636814] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:01.0 kind=pci_device vendor=0x1af4 device=0x1050 class=0x030000 present=true
[40678425645] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:02.0 kind=pci_device vendor=0x1af4 device=0x1000 class=0x020000 present=true
[40681191507] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: isolation mode skipping driver '/drivers/virtio_netd' class=Net for pci-0000:00:02.0
[40681310340] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] DISP: imported buffer as ID=1
[40682058582] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:03.0 kind=pci_device vendor=0x1af4 device=0x1059 class=0x040100 present=true
[40682921202] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=3
[40683195927] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: isolation mode skipping driver '/drivers/virtio_sound' class=Audio for pci-0000:00:03.0
[40683870051] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:1f.0 kind=pci_device vendor=0x8086 device=0x2918 class=0x060100 present=true
[40684632087] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:1f.2 kind=pci_device vendor=0x8086 device=0x2922 class=0x010601 present=true
[40684799496] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=3
[40691353197] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=3 -> OK(12)
[40693306962] [[34mDEBUG[0m] [bloom::display] [CPU1] bloom: imported buffer 1920x1080 as ID=1
[40694619504] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/desktop/wallpaper' tid=10
[40697456250] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/desktop/wallpaper' tid=10
[40701746976] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=8
[40703361435] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[40713077988] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/share/cursors/future/default.svg' tid=10
[40726839978] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.2/status' tid=7
[40728666660] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/status'
[40730116218] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/status'
[40732117173] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[40746098118] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: page cache miss for '/drivers/ahci_disk', caching 53784 bytes
[40747485240] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'ahci_disk' (len=53784, base=0x200000)
[40747988787] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[40748908002] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[40749376470] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=200000 exec=true
[40751579715] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   COPIED at 0x201420: [24, 0c, e8, 59, 0e, 00, 00, bf]
[40754812296] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=207000 exec=false
[40755858462] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=208000 exec=false
[40757015409] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x209000 filesz=0 memsz=0 align=1
[40766228217] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[40767100605] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/ahci_disk
[40768336620] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb01698a0 arg=0xffffffffb01264c0
[40789871133] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=11, applying inserts
[40790673231] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[40791187734] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=11
[40791925416] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[40792405962] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[40794765759] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[40801662231] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 11
[40804933059] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 11 woken, restoring IRQs
[40805907252] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/ahci_disk
[40806734859] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x3
[40807057533] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=11 PID=11
[40810074591] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=11 entry_pc=201000 user_sp=800000
[40811280510] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=11 buf_len=0
[40812811083] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=11 buf_len=100
[40815140850] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/ahci_disk for pci-0000:00:1f.2 (entry='thingos_driver_start_safe', pid=11)
[40816441347] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Starting AHCI VFS driver
[40820309838] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: sent DRIVER_READY to Sprout (pid=5) for driver pid=11
[40822015674] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:1f.3 kind=pci_device vendor=0x8086 device=0x2930 class=0x0c0500 present=true
[40823494173] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=isa-0070 kind=dev.rtc.Cmos vendor=0x0000 device=0x0000 class=0x000000 present=true
[40824421440] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU0] RECVMSG: thing=4 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00d91b0, metadata: DynMetadata(0xffffffff802c6ec0) }
[40831029228] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0070/status' tid=7
[40833348336] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/status'
[40835627646] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/status'
[40838237748] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: DRIVER_READY from PID 11 — no matching ManagedTask (already exited?)
[40839157722] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=4
[40840939689] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[40853253573] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: page cache miss for '/drivers/rtc_cmos', caching 45904 bytes
[40855542354] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'rtc_cmos' (len=45904, base=0x200000)
[40855556610] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 'pci-0000:00:1f.2' (handle 1)
[40856671086] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[40858482357] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[40858709331] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[40859312901] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: mapping BAR5 (phys=0x80880000, size=0x1000) for task 11
[40859607822] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=200000 exec=true
[40860228552] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10009000
[40863059127] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   COPIED at 0x201420: [0f, 84, ea, 03, 00, 00, b8, 00]
[40864158258] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: DMA alloc 1 pages phys=0x35c9000 -> user_va=0x1000a000
[40866747174] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=206000 exec=false
[40868948604] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=207000 exec=false
[40870252731] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x208000 filesz=0 memsz=0 align=1
[40874206626] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=65536
[40874856792] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(7)
[40875447690] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=1 port_id=PortId(7) mode=Write
[40875941337] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=2 port_id=PortId(7) mode=Read
[40877768481] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[40878946317] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/rtc_cmos
[40880399472] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb01abc00 arg=0xffffffffb0129320
[40881643143] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=262400
[40882189524] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(8)
[40882862922] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(8) mode=Write
[40883721912] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=12, applying inserts
[40884528696] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[40885574961] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=12
[40886504538] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[40887332805] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[40887727650] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] vfs: mounted userland provider at /dev/storage/atapi2 (flags: 0x0) port=0xffffffffb0129630
[40889792592] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Mounted atapi2 at /dev/storage/atapi2
[40890619935] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Provider loop online at /dev/storage/atapi2
[40892498262] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[40904824356] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 12
[40906842537] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 12 woken, restoring IRQs
[40907440200] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/rtc_cmos
[40908058752] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/rtc_cmos' TID=12 PID=12
[40908476862] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x4
[40909428582] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=12 entry_pc=201000 user_sp=800000
[40909702086] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/rtc_cmos for isa-0070 (entry='thingos_driver_start_safe', pid=12)
[40910473956] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=12 buf_len=0
[40911632223] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=12 buf_len=100
[40912473096] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: sent DRIVER_READY to Sprout (pid=5) for driver pid=12
[40913506953] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=isa-0060 kind=drv.Ps2Keyboard vendor=0x0000 device=0x0000 class=0x000000 present=true
[40914410559] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0060/status' tid=7
[40915581630] [[34mDEBUG[0m] [rtc_cmos] [CPU3] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffef0 rip=0x201c27 rflags=0x206
[40915602222] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/status'
[40916418048] [[34mDEBUG[0m] [rtc_cmos] [CPU3] Starting... arg=4
[40917438804] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/status'
[40925301021] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=9
[40951838532] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU0] RECVMSG: thing=4 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00d91b0, metadata: DynMetadata(0xffffffff802c6ec0) }
[40955879118] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: DRIVER_READY from PID 12 — no matching ManagedTask (already exited?)
[40956683196] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: task 12 claimed device 'isa-0070' (handle 2)
[40957471896] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[40958652867] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: claimed /sys/devices/isa-0070 (handle=2)
[40982378481] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: 2026-05-01 01:04:41 = 1777597481 unix_secs
[40993465194] [[32mINFO [0m] [kernel::time] [CPU3] System clock anchored: unix_secs=1777597481, mono_ns=20496565831, offset=1777597460503434169ns
[40996949598] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: System clock anchored to 1777597481 unix_secs
[40998216204] [[34mDEBUG[0m] [rtc_cmos] [CPU3] RTC: Entering maintenance loop.
[41015296641] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 01:04:41.010293212 unix_secs=1777597481.010293212
[41020320957] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=5
[41035208643] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: page cache miss for '/drivers/ps2_kbd', caching 72264 bytes
[41036449773] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'ps2_kbd' (len=72264, base=0x200000)
[41037051429] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[41038249032] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[41038728225] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=200000 exec=true
[41040881475] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   COPIED at 0x201420: [83, ff, 06, 77, 46, 48, 8d, 15]
[41044551867] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=209000 exec=false
[41046740691] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=20c000 exec=false
[41048109993] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x20d000 filesz=0 memsz=0 align=1
[41053642872] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[41054363163] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/ps2_kbd
[41055456882] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb0246fa0 arg=0xffffffffb01b5d60
[41057088303] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=13, applying inserts
[41057778729] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[41058221820] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=13
[41058690420] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[41059235448] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[41061320850] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[41065879866] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 13
[41067397635] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 13 woken, restoring IRQs
[41069638467] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/ps2_kbd
[41070235371] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ps2_kbd' TID=13 PID=13
[41073436602] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x5
[41074455675] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=13 entry_pc=201000 user_sp=800000
[41075974632] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=13 buf_len=0
[41077432275] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=13 buf_len=100
[41080478142] [[34mDEBUG[0m] [ps2_kbd] [CPU1] ps2_kbd: online — waiting for bristle pid
[41081441544] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/run/bristle/pid' tid=13
[41085692175] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/run/bristle/pid' tid=13
[41088948021] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[41091581520] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: bristle pid=8
[41093945112] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: task subscribed to vector 0x21
[41095185747] [[34mDEBUG[0m] [ps2_kbd] [CPU1] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[41096747241] [[34mDEBUG[0m] [ps2_kbd] [CPU1] ps2_kbd: using interrupt-driven loop (IRQ vector 0x21)
[41159982633] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=88 tid=10
[41165791887] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/ps2_kbd for isa-0060 (entry='thingos_driver_start_safe', pid=13)
[41169504849] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: sent DRIVER_READY to Sprout (pid=5) for driver pid=13
[41170879200] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=isa-0064 kind=drv.Ps2Mouse vendor=0x0000 device=0x0000 class=0x000000 present=true
[41172434820] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0064/status' tid=7
[41174573385] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0064/status'
[41177119269] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0064/status'
[41177954763
```
</details>
