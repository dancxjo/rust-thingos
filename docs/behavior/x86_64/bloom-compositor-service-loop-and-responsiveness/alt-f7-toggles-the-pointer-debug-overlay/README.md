# ❌ Scenario: Alt F7 toggles the pointer debug overlay

> Last run: 2026-04-30 18:01:45

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 11254ms | - [📜](./01/serial.log) - |
| 2 | Then the serial output should contain "bloom: registered bristle pointer sink" within 60s | ❌ | 61455ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[34562721633] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[34596899205] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] enter
[34600033281] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] phys_memory_map ok
[34600787298] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] modules ok
[34601500560] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] phys_to_virt_offset ok
[34602002391] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] memory map logging done
[34626861918] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] boot_frame_alloc init ok
[34845313800] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] frame allocator build ok
[34846118373] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] frame allocator log ok
[34847062602] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] FRAME_ALLOCATOR init ok
[34847817147] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] tasking init ok
[34848503547] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[34869990309] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[34870801779] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] enter
[34871368125] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] set expand hook
[34871891967] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] expand hook ok
[34872401289] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock begin
[34873242096] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock ok
[34873784385] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] reserve_region begin
[34902773103] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] reserve_region ok
[34903691031] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] inner allocator init begin
[34907135175] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] inner allocator init ok
[34907889258] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] heap top store ok
[34908416499] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] init done
[34909029540] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[34932726477] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] devfs: set_boot_fb width=1920 height=1080 pitch=7680 resource_id=0xfb000000 phys=0x80000000
[34935167784] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs set_boot_fb elapsed_ticks=2666961 elapsed_us=1333
[34941265260] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs FbNode::new elapsed_ticks=5240301 elapsed_us=2620
[34942117221] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] register begin
[34948286934] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] register ok
[34949582910] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs register fb0 elapsed_ticks=7323294 elapsed_us=3661
[34950554100] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[34974217443] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[34976334129] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[35013844206] [[34mDEBUG[0m] [kernel] [CPU0] Seeding entropy pool...
[35015287131] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed begin
[35016261027] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] fill_entropy done
[35018487438] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] add_sample(timer) ok
[35019061440] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] mark_seeded(timer) ok
[35019562545] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[35020124964] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[35020660752] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed done
[35021810802] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] entropy::seed_from_hardware elapsed_ticks=6220236 elapsed_us=3110
[35022417474] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[35042564073] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[35051609670] [[35mTRACE[0m] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[35052700353] [[35mTRACE[0m] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[35053342137] [[35mTRACE[0m] [kernel::sched] [CPU0]   Allocating scheduler...
[35057193732] [[35mTRACE[0m] [kernel::sched] [CPU0]   Leaking scheduler...
[35057817927] [[35mTRACE[0m] [kernel::sched] [CPU0]   Initializing boot task...
[35059990977] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED: allocating CpuScheduler for cpu0 (total=1)
[35060943456] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[35061525675] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[35062508448] [[35mTRACE[0m] [kernel::sched] [CPU0]   Creating boot task...
[35074595358] [[35mTRACE[0m] [kernel::sched] [CPU0]   Creating idle tasks...
[35075851206] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff8008b390 kstack_top=0xffffffffb0020660 arg=0x0
[35077886547] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[35098982622] [[35mTRACE[0m] [kernel::sched] [CPU0]   Boot task initialized
[35101249854] [[34mDEBUG[0m] [kernel::sched::state] [CPU0] SCHED[cpu0]: current=Some(0) idle=Some(1) runnable=0 need_resched=true
[35102478345] [[35mTRACE[0m] [kernel::sched] [CPU0]   Storing scheduler pointer...
[35103638955] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[35104676970] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[35125454133] [[34mDEBUG[0m] [kernel] [CPU0] Initializing VFS...
[35129320281] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='set_cmdline' elapsed_ticks=2933502 elapsed_us=1466 total_ticks=3761241 total_us=1880
[35148945084] [[34mDEBUG[0m] [kernel::vfs] [CPU0] VFS: Created /dev/display directory
[35150015340] [[34mDEBUG[0m] [kernel::vfs] [CPU0] VFS: Created /dev/storage directory
[35190694308] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted union filesystem at / (root)
[35193711069] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted devfs at /dev
[35194907880] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted procfs at /proc
[35195899596] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted sysfs at /sys
[35200148940] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /tmp
[35201000472] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /run
[35201742906] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /services
[35202533322] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /session
[35203363668] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /data
[35203886916] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='vfs_init' elapsed_ticks=73838655 elapsed_us=36919 total_ticks=78590688 total_us=39295
[35204532594] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[35224817298] [[34mDEBUG[0m] [kernel] [CPU0] Scanning PCI bus...
[35228893194] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x29c0 at 00:00.0 class=060000 id=1
[35233930710] [[35mTRACE[0m] [kernel] [CPU0]   BAR0 (MEM32): 0x80000000 (size 0x800000)
[35245977855] [[35mTRACE[0m] [kernel] [CPU0]   BAR2 (MEM64): 0xc000000000 (size 0x4000)
[35249508558] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (MEM32): 0x80883000 (size 0x1000)
[35250313428] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1050 at 00:01.0 class=030000 id=2
[35254244784] [[35mTRACE[0m] [kernel] [CPU0]   BAR0 (I/O):  0x6060 (size 0x20)
[35258274480] [[35mTRACE[0m] [kernel] [CPU0]   BAR1 (MEM32): 0x80882000 (size 0x1000)
[35273166159] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (MEM64): 0xc000004000 (size 0x4000)
[35274127944] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1000 at 00:02.0 class=020000 id=3
[35276394780] [[35mTRACE[0m] [kernel] [CPU0]   BAR1 (MEM32): 0x80881000 (size 0x1000)
[35285038734] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (MEM64): 0xc000008000 (size 0x4000)
[35285834397] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1059 at 00:03.0 class=040100 id=4
[35286659265] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2918 at 00:1f.0 class=060100 id=5
[35289195711] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (I/O):  0x6040 (size 0x20)
[35291746215] [[35mTRACE[0m] [kernel] [CPU0]   BAR5 (MEM32): 0x80880000 (size 0x1000)
[35292380772] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2922 at 00:1f.2 class=010601 id=6
[35296467822] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (I/O):  0x6000 (size 0x40)
[35297259921] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2930 at 00:1f.3 class=0c0500 id=7
[35299671000] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='scan_pci' elapsed_ticks=74208585 elapsed_us=37104 total_ticks=174366819 total_us=87183
[35300346411] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[35322167925] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='register_legacy_devices' elapsed_ticks=709236 elapsed_us=354 total_ticks=196856484 total_us=98428
[35323000086] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[35343465894] [[34mDEBUG[0m] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[35351462718] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[35352170700] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[35353188552] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[35354092587] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[35357934150] [[34mDEBUG[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[35359535937] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[35360335758] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[35360895900] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[35361394233] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[35361914709] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[35362747497] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[35363610711] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[35364180060] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[35364682749] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[35365196559] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[35365741026] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[35366605329] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[35367542265] [[34mDEBUG[0m] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[35368163325] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[35369739273] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Registers initialized
[35373328254] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[35374403493] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[35375438505] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: All pins masked
[35376839982] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> NMI
[35377644324] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[35378465265] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ4 -> GSI 4 -> 0x24
[35379012900] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Init complete
[35413624257] [[34mDEBUG[0m] [bran::arch::x86_64::ioapic] [CPU0] LAPIC: calibrated 62994700 ticks/sec (delta=629947, ok=true) -> init_cnt=629947
[35416746354] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62994700 ticks/sec), init_cnt=629947 for 100Hz
[35417740413] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='setup_preemption_timer' elapsed_ticks=73529874 elapsed_us=36764 total_ticks=292407258 total_us=146203
[35418751533] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[35441844867] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[35442774543] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[35444173215] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[35446259046] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[35479936503] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[35480889213] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[35481944949] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] SMP: CPU 1 initializing preemption timer (vec=32 init_cnt=629947)
[35484388467] [[34mDEBUG[0m] [kernel] [CPU1] SMP: kernel_secondary_entry arg_cpu=1 runtime_cpu=1
[35485430475] [[34mDEBUG[0m] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[35486966427] [[34mDEBUG[0m] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[35490285897] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff8008b390 kstack_top=0xffffffffb00375a0 arg=0x1
[35490994077] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Task 2 assigned to CPU 1
[35494844319] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 2 deferred inserts
[35495993742] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=1
[35496855933] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=2
[35498808114] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[35499706869] [[34mDEBUG[0m] [kernel::sched::lifecycle] [CPU1] SMP: Secondary CPU 1 online!
[35500614765] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: run_scheduler entry on CPU 1
[35501513157] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: bootstrap_cpu start on CPU 1
[35503043235] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle thread 2 and is now schedulable
[35504807052] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] SMP: CPU 2 initializing preemption timer (vec=32 init_cnt=629947)
[35504985417] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[35505776757] [[34mDEBUG[0m] [kernel] [CPU2] SMP: kernel_secondary_entry arg_cpu=2 runtime_cpu=2
[35505934068] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[35506335909] [[34mDEBUG[0m] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[35515133907] [[34mDEBUG[0m] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[35516320323] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU2] INIT KERNEL CTX: entry=0xffffffff8008b390 kstack_top=0xffffffffb0048d20 arg=0x2
[35523354570] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] SCHED: Task 3 assigned to CPU 2
[35525895801] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Applying 1 deferred inserts
[35526490362] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserting TID=3
[35527142013] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserts applied
[35527661664] [[34mDEBUG[0m] [kernel::sched::lifecycle] [CPU2] SMP: Secondary CPU 2 online!
[35528167026] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: run_scheduler entry on CPU 2
[35528666514] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: bootstrap_cpu start on CPU 2
[35529175935] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle thread 3 and is now schedulable
[35530064526] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[35530380699] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] SMP: CPU 3 initializing preemption timer (vec=32 init_cnt=629947)
[35530743567] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[35531834184] [[34mDEBUG[0m] [kernel] [CPU3] SMP: kernel_secondary_entry arg_cpu=3 runtime_cpu=3
[35532209823] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[35533106565] [[34mDEBUG[0m] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[35534586549] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='start_secondary_cpus' elapsed_ticks=92875035 elapsed_us=46437 total_ticks=409124034 total_us=204562
[35534907144] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[35535508668] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[35536048713] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff8008b390 kstack_top=0xffffffffb005a4a0 arg=0x3
[35537049702] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SCHED: Task 4 assigned to CPU 3
[35538805236] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[35539426065] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=4
[35540022804] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[35540514702] [[34mDEBUG[0m] [kernel::sched::lifecycle] [CPU3] SMP: Secondary CPU 3 online!
[35541029733] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: run_scheduler entry on CPU 3
[35541508563] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: bootstrap_cpu start on CPU 3
[35545151895] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle thread 4 and is now schedulable
[35596435281] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[35664107259] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Enumerating 214 boot modules...
[35665428348] [[35mTRACE[0m] [kernel] [CPU0]   Module[0]: name='/bin/sprout' cmdline='init' size=89912 bytes
[35666492961] [[35mTRACE[0m] [kernel] [CPU0]   Module[1]: name='/bin/bristle' cmdline='' size=68744 bytes
[35667385908] [[35mTRACE[0m] [kernel] [CPU0]   Module[2]: name='/drivers/rtc_cmos' cmdline='init' size=45904 bytes
[35667918891] [[35mTRACE[0m] [kernel] [CPU0]   Module[3]: name='/drivers/ps2_kbd' cmdline='' size=72264 bytes
[35668447419] [[35mTRACE[0m] [kernel] [CPU0]   Module[4]: name='/bin/sh' cmdline='init' size=127864 bytes
[35668968159] [[35mTRACE[0m] [kernel] [CPU0]   Module[5]: name='/bin/ls' cmdline='init' size=62136 bytes
[35669529753] [[35mTRACE[0m] [kernel] [CPU0]   Module[6]: name='/bin/ln' cmdline='init' size=47568 bytes
[35670047193] [[35mTRACE[0m] [kernel] [CPU0]   Module[7]: name='/bin/ps' cmdline='init' size=50544 bytes
[35670579780] [[35mTRACE[0m] [kernel] [CPU0]   Module[8]: name='/bin/kill' cmdline='init' size=55584 bytes
[35671097385] [[35mTRACE[0m] [kernel] [CPU0]   Module[9]: name='/bin/killall' cmdline='init' size=67656 bytes
[35671600767] [[35mTRACE[0m] [kernel] [CPU0]   Module[10]: name='/bin/cat' cmdline='init' size=50832 bytes
[35672209518] [[35mTRACE[0m] [kernel] [CPU0]   Module[11]: name='/bin/head' cmdline='init' size=56112 bytes
[35672959212] [[35mTRACE[0m] [kernel] [CPU0]   Module[12]: name='/bin/tail' cmdline='init' size=56088 bytes
[35673570240] [[35mTRACE[0m] [kernel] [CPU0]   Module[13]: name='/bin/top' cmdline='init' size=65488 bytes
[35677132590] [[35mTRACE[0m] [kernel] [CPU0]   Module[14]: name='/bin/wc' cmdline='init' size=56248 bytes
[35677662405] [[35mTRACE[0m] [kernel] [CPU0]   Module[15]: name='/bin/yes' cmdline='init' size=44520 bytes
[35678163411] [[35mTRACE[0m] [kernel] [CPU0]   Module[16]: name='/bin/which' cmdline='init' size=50448 bytes
[35678667255] [[35mTRACE[0m] [kernel] [CPU0]   Module[17]: name='/bin/cp' cmdline='init' size=46312 bytes
[35679181098] [[35mTRACE[0m] [kernel] [CPU0]   Module[18]: name='/bin/mv' cmdline='init' size=47048 bytes
[35679698670] [[35mTRACE[0m] [kernel] [CPU0]   Module[19]: name='/bin/rm' cmdline='init' size=47128 bytes
[35680221027] [[35mTRACE[0m] [kernel] [CPU0]   Module[20]: name='/bin/rmdir' cmdline='init' size=47144 bytes
[35680727379] [[35mTRACE[0m] [kernel] [CPU0]   Module[21]: name='/bin/mkdir' cmdline='init' size=47144 bytes
[35681233863] [[35mTRACE[0m] [kernel] [CPU0]   Module[22]: name='/bin/mount' cmdline='init' size=57688 bytes
[35681751336] [[35mTRACE[0m] [kernel] [CPU0]   Module[23]: name='/bin/echo' cmdline='init' size=44616 bytes
[35682302172] [[35mTRACE[0m] [kernel] [CPU0]   Module[24]: name='/bin/printf' cmdline='init' size=93952 bytes
[35682816741] [[35mTRACE[0m] [kernel] [CPU0]   Module[25]: name='/bin/grep' cmdline='init' size=85240 bytes
[35683338009] [[35mTRACE[0m] [kernel] [CPU0]   Module[26]: name='/bin/pwd' cmdline='init' size=39184 bytes
[35683840401] [[35mTRACE[0m] [kernel] [CPU0]   Module[27]: name='/bin/touch' cmdline='init' size=46160 bytes
[35684617518] [[35mTRACE[0m] [kernel] [CPU0]   Module[28]: name='/bin/setshell' cmdline='init' size=47048 bytes
[35686146540] [[35mTRACE[0m] [kernel] [CPU0]   Module[29]: name='/bin/dmesg' cmdline='init' size=39328 bytes
[35686681833] [[35mTRACE[0m] [kernel] [CPU0]   Module[30]: name='/bin/stat' cmdline='init' size=46200 bytes
[35688152940] [[35mTRACE[0m] [kernel] [CPU0]   Module[31]: name='/bin/file' cmdline='init' size=51104 bytes
[35688660843] [[35mTRACE[0m] [kernel] [CPU0]   Module[32]: name='/bin/dirname' cmdline='init' size=50096 bytes
[35689176006] [[35mTRACE[0m] [kernel] [CPU0]   Module[33]: name='/bin/basename' cmdline='init' size=50104 bytes
[35689703643] [[35mTRACE[0m] [kernel] [CPU0]   Module[34]: name='/bin/sleep' cmdline='init' size=56824 bytes
[35690236362] [[35mTRACE[0m] [kernel] [CPU0]   Module[35]: name='/bin/sort' cmdline='init' size=66336 bytes
[35690743968] [[35mTRACE[0m] [kernel] [CPU0]   Module[36]: name='/bin/env' cmdline='init' size=45832 bytes
[35691259857] [[35mTRACE[0m] [kernel] [CPU0]   Module[37]: name='/bin/uname' cmdline='' size=44632 bytes
[35691775548] [[35mTRACE[0m] [kernel] [CPU0]   Module[38]: name='/bin/true' cmdline='' size=20744 bytes
[35692270614] [[35mTRACE[0m] [kernel] [CPU0]   Module[39]: name='/bin/false' cmdline='' size=20744 bytes
[35692768881] [[35mTRACE[0m] [kernel] [CPU0]   Module[40]: name='/bin/input_echo' cmdline='' size=43016 bytes
[35693292987] [[35mTRACE[0m] [kernel] [CPU0]   Module[41]: name='/drivers/ps2_mouse' cmdline='' size=71096 bytes
[35693806236] [[35mTRACE[0m] [kernel] [CPU0]   Module[42]: name='/drivers/display_bootfb' cmdline='' size=91736 bytes
[35694339054] [[35mTRACE[0m] [kernel] [CPU0]   Module[43]: name='/drivers/display_virtio_gpu' cmdline='' size=147560 bytes
[35694888570] [[35mTRACE[0m] [kernel] [CPU0]   Module[44]: name='/bin/cambium' cmdline='' size=144768 bytes
[35695391028] [[35mTRACE[0m] [kernel] [CPU0]   Module[45]: name='/drivers/virtio_netd' cmdline='' size=123920 bytes
[35695902693] [[35mTRACE[0m] [kernel] [CPU0]   Module[46]: name='/drivers/rtl8168d' cmdline='' size=61392 bytes
[35696436402] [[35mTRACE[0m] [kernel] [CPU0]   Module[47]: name='/bin/netd' cmdline='' size=10819752 bytes
[35696965755] [[35mTRACE[0m] [kernel] [CPU0]   Module[48]: name='/bin/mesocarp' cmdline='' size=123496 bytes
[35697469434] [[35mTRACE[0m] [kernel] [CPU0]   Module[49]: name='/bin/mdns' cmdline='' size=123496 bytes
[35698021161] [[35mTRACE[0m] [kernel] [CPU0]   Module[50]: name='/bin/mdnsd' cmdline='' size=123496 bytes
[35698759536] [[35mTRACE[0m] [kernel] [CPU0]   Module[51]: name='/bin/fetchd' cmdline='' size=76752 bytes
[35699562063] [[35mTRACE[0m] [kernel] [CPU0]   Module[52]: name='/bin/httpsd' cmdline='' size=536128 bytes
[35701249452] [[35mTRACE[0m] [kernel] [CPU0]   Module[53]: name='/bin/find' cmdline='' size=57896 bytes
[35713770543] [[35mTRACE[0m] [kernel] [CPU0]   Module[54]: name='/bin/ip' cmdline='' size=55472 bytes
[35725200555] [[35mTRACE[0m] [kernel] [CPU0]   Module[55]: name='/bin/iso_reader' cmdline='' size=37424 bytes
[35725738224] [[35mTRACE[0m] [kernel] [CPU0]   Module[56]: name='/bin/ping' cmdline='' size=65184 bytes
[35726280183] [[35mTRACE[0m] [kernel] [CPU0]   Module[57]: name='/bin/nslookup' cmdline='' size=56424 bytes
[35726788218] [[35mTRACE[0m] [kernel] [CPU0]   Module[58]: name='/drivers/ahci_disk' cmdline='' size=53784 bytes
[35727318363] [[35mTRACE[0m] [kernel] [CPU0]   Module[59]: name='/drivers/ata_disk' cmdline='' size=64448 bytes
[35727826893] [[35mTRACE[0m] [kernel] [CPU0]   Module[60]: name='/bin/iso9660d' cmdline='' size=78544 bytes
[35728337568] [[35mTRACE[0m] [kernel] [CPU0]   Module[61]: name='/drivers/virtio_sound' cmdline='' size=99296 bytes
[35728873818] [[35mTRACE[0m] [kernel] [CPU0]   Module[62]: name='/drivers/hdaudio' cmdline='' size=67696 bytes
[35729383173] [[35mTRACE[0m] [kernel] [CPU0]   Module[63]: name='/drivers/pci_stubd' cmdline='' size=58168 bytes
[35729913582] [[35mTRACE[0m] [kernel] [CPU0]   Module[64]: name='/drivers/chime' cmdline='' size=59536 bytes
[35730455541] [[35mTRACE[0m] [kernel] [CPU0]   Module[65]: name='/bin/vfs_hello' cmdline='' size=39560 bytes
[35730971496] [[35mTRACE[0m] [kernel] [CPU0]   Module[66]: name='/bin/show_args' cmdline='' size=42488 bytes
[35731728351] [[35mTRACE[0m] [kernel] [CPU0]   Module[67]: name='/bin/clock' cmdline='' size=82936 bytes
[35732781018] [[35mTRACE[0m] [kernel] [CPU0]   Module[68]: name='/bin/env_roundtrip' cmdline='' size=48232 bytes
[35733823719] [[35mTRACE[0m] [kernel] [CPU0]   Module[69]: name='/bin/cwd_test' cmdline='' size=43696 bytes
[35737841898] [[35mTRACE[0m] [kernel] [CPU0]   Module[70]: name='/bin/date' cmdline='' size=67032 bytes
[35738348976] [[35mTRACE[0m] [kernel] [CPU0]   Module[71]: name='/bin/wayland_hello' cmdline='' size=82360 bytes
[35738861301] [[35mTRACE[0m] [kernel] [CPU0]   Module[72]: name='/bin/terminal' cmdline='' size=91056 bytes
[35739698379] [[35mTRACE[0m] [kernel] [CPU0]   Module[73]: name='/bin/tee' cmdline='' size=44968 bytes
[35740239546] [[35mTRACE[0m] [kernel] [CPU0]   Module[74]: name='/bin/xargs' cmdline='' size=55016 bytes
[35740741476] [[35mTRACE[0m] [kernel] [CPU0]   Module[75]: name='/bin/placed' cmdline='' size=38048 bytes
[35741285844] [[35mTRACE[0m] [kernel] [CPU0]   Module[76]: name='/bin/bloom' cmdline='' size=462464 bytes
[35741800875] [[35mTRACE[0m] [kernel] [CPU0]   Module[77]: name='/bin/clear' cmdline='' size=20872 bytes
[35742301815] [[35mTRACE[0m] [kernel] [CPU0]   Module[78]: name='/bin/loglevel' cmdline='' size=41536 bytes
[35742886872] [[35mTRACE[0m] [kernel] [CPU0]   Module[79]: name='/bin/poll_mux' cmdline='' size=39056 bytes
[35743409625] [[35mTRACE[0m] [kernel] [CPU0]   Module[80]: name='/bin/ipc_service_demo' cmdline='' size=47592 bytes
[35743921587] [[35mTRACE[0m] [kernel] [CPU0]   Module[81]: name='/bin/ipc_pipe_demo' cmdline='' size=43936 bytes
[35744430117] [[35mTRACE[0m] [kernel] [CPU0]   Module[82]: name='/bin/ipc_provider_demo' cmdline='' size=84440 bytes
[35744995374] [[35mTRACE[0m] [kernel] [CPU0]   Module[83]: name='/bin/vfs_test_provider' cmdline='' size=70648 bytes
[35745523110] [[35mTRACE[0m] [kernel] [CPU0]   Module[84]: name='/bin/ipc_memfd_demo' cmdline='' size=38232 bytes
[35746055598] [[35mTRACE[0m] [kernel] [CPU0]   Module[85]: name='/bin/test_exec' cmdline='' size=62592 bytes
[35746563798] [[35mTRACE[0m] [kernel] [CPU0]   Module[86]: name='/bin/test_vm_protect' cmdline='' size=38224 bytes
[35747092095] [[35mTRACE[0m] [kernel] [CPU0]   Module[87]: name='/bin/test_exec_env' cmdline='' size=58904 bytes
[35747651181] [[35mTRACE[0m] [kernel] [CPU0]   Module[88]: name='/bin/test_threads' cmdline='' size=82376 bytes
[35748164001] [[35mTRACE[0m] [kernel] [CPU0]   Module[89]: name='/bin/test_futex' cmdline='' size=52992 bytes
[35748751104] [[35mTRACE[0m] [kernel] [CPU0]   Module[90]: name='/bin/ld_so' cmdline='' size=378600 bytes
[35749275408] [[35mTRACE[0m] [kernel] [CPU0]   Module[91]: name='/bin/test_dyn_loader' cmdline='' size=57760 bytes
[35749791066] [[35mTRACE[0m] [kernel] [CPU0]   Module[92]: name='/bin/test_dlopen' cmdline='' size=64688 bytes
[35750301444] [[35mTRACE[0m] [kernel] [CPU0]   Module[93]: name='/bin/reboot' cmdline='' size=32048 bytes
[35750975568] [[35mTRACE[0m] [kernel] [CPU0]   Module[94]: name='/bin/shutdown' cmdline='' size=32048 bytes
[35751957483] [[35mTRACE[0m] [kernel] [CPU0]   Module[95]: name='/bin/attr_list' cmdline='' size=47400 bytes
[35752541517] [[35mTRACE[0m] [kernel] [CPU0]   Module[96]: name='/bin/attr_get' cmdline='' size=47488 bytes
[35753056020] [[35mTRACE[0m] [kernel] [CPU0]   Module[97]: name='/bin/attr_set' cmdline='' size=69600 bytes
[35753561349] [[35mTRACE[0m] [kernel] [CPU0]   Module[98]: name='/bin/attr_rm' cmdline='' size=47008 bytes
[35754067338] [[35mTRACE[0m] [kernel] [CPU0]   Module[99]: name='/lib/libpistil.so' cmdline='' size=826992 bytes
[35754602400] [[35mTRACE[0m] [kernel] [CPU0]   Module[100]: name='/share/wallpapers/clouds.bmp' cmdline='' size=786486 bytes
[35755146999] [[35mTRACE[0m] [kernel] [CPU0]   Module[101]: name='/share/wallpapers/flower.bmp' cmdline='' size=8386614 bytes
[35755760469] [[35mTRACE[0m] [kernel] [CPU0]   Module[102]: name='/share/wallpapers/flower.png' cmdline='' size=2652468 bytes
[35756332623] [[35mTRACE[0m] [kernel] [CPU0]   Module[103]: name='/share/wallpapers/leather.bmp' cmdline='' size=1179702 bytes
[35756874318] [[35mTRACE[0m] [kernel] [CPU0]   Module[104]: name='/share/wallpapers/linen.bmp' cmdline='' size=4718646 bytes
[35757405255] [[35mTRACE[0m] [kernel] [CPU0]   Module[105]: name='/share/fonts/DSEG7Classic-Regular.ttf' cmdline='' size=23272 bytes
[35757971007] [[35mTRACE[0m] [kernel] [CPU0]   Module[106]: name='/share/fonts/Hack-Regular.ttf' cmdline='' size=309408 bytes
[35758523625] [[35mTRACE[0m] [kernel] [CPU0]   Module[107]: name='/share/fonts/Inter-Regular.ttf' cmdline='' size=876576 bytes
[35759099805] [[35mTRACE[0m] [kernel] [CPU0]   Module[108]: name='/share/fonts/Iosevka-Regular.ttf' cmdline='' size=10457376 bytes
[35759662620] [[35mTRACE[0m] [kernel] [CPU0]   Module[109]: name='/share/fonts/JetBrainsMono-Regular.ttf' cmdline='' size=270224 bytes
[35760212202] [[35mTRACE[0m] [kernel] [CPU0]   Module[110]: name='/share/fonts/NotoSans-Regular.ttf' cmdline='' size=569208 bytes
[35760769176] [[35mTRACE[0m] [kernel] [CPU0]   Module[111]: name='/share/fonts/NotoSansSymbol-Regular.ttf' cmdline='' size=258156 bytes
[35761371954] [[35mTRACE[0m] [kernel] [CPU0]   Module[112]: name='/share/fonts/NotoSansSymbol2-Regular.ttf' cmdline='' size=656852 bytes
[35761957704] [[35mTRACE[0m] [kernel] [CPU0]   Module[113]: name='/share/fonts/NotoSerif-Regular.ttf' cmdline='' size=616196 bytes
[35762543058] [[35mTRACE[0m] [kernel] [CPU0]   Module[114]: name='/share/themes/solarized_warm.toml' cmdline='' size=1832 bytes
[35763081981] [[35mTRACE[0m] [kernel] [CPU0]   Module[115]: name='/share/cursors/future/alias.svg' cmdline='' size=9033 bytes
[35763613941] [[35mTRACE[0m] [kernel] [CPU0]   Module[116]: name='/share/cursors/future/all-scroll.svg' cmdline='' size=5170 bytes
[35764154184] [[35mTRACE[0m] [kernel] [CPU0]   Module[117]: name='/share/cursors/future/bottom_left_corner.svg' cmdline='' size=1441 bytes
[35764935360] [[35mTRACE[0m] [kernel] [CPU0]   Module[118]: name='/share/cursors/future/bottom_right_corner.svg' cmdline='' size=3037 bytes
[35766057129] [[35mTRACE[0m] [kernel] [CPU0]   Module[119]: name='/share/cursors/future/bottom_side.svg' cmdline='' size=5432 bytes
[35767392342] [[35mTRACE[0m] [kernel] [CPU0]   Module[120]: name='/share/cursors/future/cell.svg' cmdline='' size=3707 bytes
[35768236152] [[35mTRACE[0m] [kernel] [CPU0]   Module[121]: name='/share/cursors/future/center_ptr.svg' cmdline='' size=9766 bytes
[35768904336] [[35mTRACE[0m] [kernel] [CPU0]   Module[122]: name='/share/cursors/future/col-resize.svg' cmdline='' size=16463 bytes
[35769609810] [[35mTRACE[0m] [kernel] [CPU0]   Module[123]: name='/share/cursors/future/color-picker.svg' cmdline='' size=6296 bytes
[35770276971] [[35mTRACE[0m] [kernel] [CPU0]   Module[124]: name='/share/cursors/future/context-menu.svg' cmdline='' size=9011 bytes
[35770931856] [[35mTRACE[0m] [kernel] [CPU0]   Module[125]: name='/share/cursors/future/copy.svg' cmdline='' size=3527 bytes
[35771577798] [[35mTRACE[0m] [kernel] [CPU0]   Module[126]: name='/share/cursors/future/crosshair.svg' cmdline='' size=16918 bytes
[35772477477] [[35mTRACE[0m] [kernel] [CPU0]   Module[127]: name='/share/cursors/future/default.svg' cmdline='' size=3079 bytes
[35773599477] [[35mTRACE[0m] [kernel] [CPU0]   Module[128]: name='/share/cursors/future/dnd-move.svg' cmdline='' size=3569 bytes
[35774254461] [[35mTRACE[0m] [kernel] [CPU0]   Module[129]: name='/share/cursors/future/dnd-no-drop.svg' cmdline='' size=4753 bytes
[35774935812] [[35mTRACE[0m] [kernel] [CPU0]   Module[130]: name='/share/cursors/future/down-arrow.svg' cmdline='' size=1820 bytes
[35775650493] [[35mTRACE[0m] [kernel] [CPU0]   Module[131]: name='/share/cursors/future/draft.svg' cmdline='' size=3136 bytes
[35776298448] [[35mTRACE[0m] [kernel] [CPU0]   Module[132]: name='/share/cursors/future/fleur.svg' cmdline='' size=28469 bytes
[35776955643] [[35mTRACE[0m] [kernel] [CPU0]   Module[133]: name='/share/cursors/future/help.svg' cmdline='' size=5873 bytes
[35777596767] [[35mTRACE[0m] [kernel] [CPU0]   Module[134]: name='/share/cursors/future/left-arrow.svg' cmdline='' size=1785 bytes
[35778269967] [[35mTRACE[0m] [kernel] [CPU0]   Module[135]: name='/share/cursors/future/left_side.svg' cmdline='' size=3848 bytes
[35778961152] [[35mTRACE[0m] [kernel] [CPU0]   Module[136]: name='/share/cursors/future/no-drop.svg' cmdline='' size=3861 bytes
[35779612836] [[35mTRACE[0m] [kernel] [CPU0]   Module[137]: name='/share/cursors/future/not-allowed.svg' cmdline='' size=1567 bytes
[35780282274] [[35mTRACE[0m] [kernel] [CPU0]   Module[138]: name='/share/cursors/future/openhand.svg' cmdline='' size=3435 bytes
[35780933991] [[35mTRACE[0m] [kernel] [CPU0]   Module[139]: name='/share/cursors/future/pencil.svg' cmdline='' size=6527 bytes
[35781564621] [[35mTRACE[0m] [kernel] [CPU0]   Module[140]: name='/share/cursors/future/pirate.svg' cmdline='' size=6663 bytes
[35782300983] [[35mTRACE[0m] [kernel] [CPU0]   Module[141]: name='/share/cursors/future/pointer.svg' cmdline='' size=2922 bytes
[35782942173] [[35mTRACE[0m] [kernel] [CPU0]   Module[142]: name='/share/cursors/future/progress-01.svg' cmdline='' size=11030 bytes
[35783617089] [[35mTRACE[0m] [kernel] [CPU0]   Module[143]: name='/share/cursors/future/progress-02.svg' cmdline='' size=12198 bytes
[35784097668] [[35mTRACE[0m] [kernel] [CPU0]   Module[144]: name='/share/cursors/future/progress-03.svg' cmdline='' size=12733 bytes
[35784744435] [[35mTRACE[0m] [kernel] [CPU0]   Module[145]: name='/share/cursors/future/progress-04.svg' cmdline='' size=13777 bytes
[35786248080] [[35mTRACE[0m] [kernel] [CPU0]   Module[146]: name='/share/cursors/future/progress-05.svg' cmdline='' size=13835 bytes
[35791636485] [[35mTRACE[0m] [kernel] [CPU0]   Module[147]: name='/share/cursors/future/progress-06.svg' cmdline='' size=14907 bytes
[35805551925] [[35mTRACE[0m] [kernel] [CPU0]   Module[148]: name='/share/cursors/future/progress-07.svg' cmdline='' size=14805 bytes
[35824144125] [[35mTRACE[0m] [kernel] [CPU0]   Module[149]: name='/share/cursors/future/progress-08.svg' cmdline='' size=15996 bytes
[35824907283] [[35mTRACE[0m] [kernel] [CPU0]   Module[150]: name='/share/cursors/future/progress-09.svg' cmdline='' size=16010 bytes
[35825622360] [[35mTRACE[0m] [kernel] [CPU0]   Module[151]: name='/share/cursors/future/progress-10.svg' cmdline='' size=17080 bytes
[35826275133] [[35mTRACE[0m] [kernel] [CPU0]   Module[152]: name='/share/cursors/future/progress-11.svg' cmdline='' size=17096 bytes
[35826949059] [[35mTRACE[0m] [kernel] [CPU0]   Module[153]: name='/share/cursors/future/progress-12.svg' cmdline='' size=17079 bytes
[35827598235] [[35mTRACE[0m] [kernel] [CPU0]   Module[154]: name='/share/cursors/future/progress-13.svg' cmdline='' size=15900 bytes
[35828279784] [[35mTRACE[0m] [kernel] [CPU0]   Module[155]: name='/share/cursors/future/progress-14.svg' cmdline='' size=16001 bytes
[35828936616] [[35mTRACE[0m] [kernel] [CPU0]   Module[156]: name='/share/cursors/future/progress-15.svg' cmdline='' size=14931 bytes
[35829607440] [[35mTRACE[0m] [kernel] [CPU0]   Module[157]: name='/share/cursors/future/progress-16.svg' cmdline='' size=14873 bytes
[35830281069] [[35mTRACE[0m] [kernel] [CPU0]   Module[158]: name='/share/cursors/future/progress-17.svg' cmdline='' size=13846 bytes
[35831072145] [[35mTRACE[0m] [kernel] [CPU0]   Module[159]: name='/share/cursors/future/progress-18.svg' cmdline='' size=13835 bytes
[35832053862] [[35mTRACE[0m] [kernel] [CPU0]   Module[160]: name='/share/cursors/future/progress-19.svg' cmdline='' size=12645 bytes
[35833438245] [[35mTRACE[0m] [kernel] [CPU0]   Module[161]: name='/share/cursors/future/progress-20.svg' cmdline='' size=12741 bytes
[35834141013] [[35mTRACE[0m] [kernel] [CPU0]   Module[162]: name='/share/cursors/future/progress-21.svg' cmdline='' size=11660 bytes
[35834817381] [[35mTRACE[0m] [kernel] [CPU0]   Module[163]: name='/share/cursors/future/progress-22.svg' cmdline='' size=11612 bytes
[35835510480] [[35mTRACE[0m] [kernel] [CPU0]   Module[164]: name='/share/cursors/future/progress-23.svg' cmdline='' size=11674 bytes
[35836154475] [[35mTRACE[0m] [kernel] [CPU0]   Module[165]: name='/share/cursors/future/progress.svg' cmdline='' size=11642 bytes
[35836790022] [[35mTRACE[0m] [kernel] [CPU0]   Module[166]: name='/share/cursors/future/right-arrow.svg' cmdline='' size=3432 bytes
[35837429166] [[35mTRACE[0m] [kernel] [CPU0]   Module[167]: name='/share/cursors/future/right_ptr.svg' cmdline='' size=5155 bytes
[35838111804] [[35mTRACE[0m] [kernel] [CPU0]   Module[168]: name='/share/cursors/future/right_side.svg' cmdline='' size=6447 bytes
[35838791307] [[35mTRACE[0m] [kernel] [CPU0]   Module[169]: name='/share/cursors/future/row-resize.svg' cmdline='' size=15825 bytes
[35851074303] [[35mTRACE[0m] [kernel] [CPU0]   Module[170]: name='/share/cursors/future/size_bdiag.svg' cmdline='' size=16323 bytes
[35859595959] [[35mTRACE[0m] [kernel] [CPU0]   Module[171]: name='/share/cursors/future/size_fdiag.svg' cmdline='' size=16589 bytes
[35860241670] [[35mTRACE[0m] [kernel] [CPU0]   Module[172]: name='/share/cursors/future/size_hor.svg' cmdline='' size=16255 bytes
[35860909029] [[35mTRACE[0m] [kernel] [CPU0]   Module[173]: name='/share/cursors/future/size_ver.svg' cmdline='' size=15822 bytes
[35861601963] [[35mTRACE[0m] [kernel] [CPU0]   Module[174]: name='/share/cursors/future/text.svg' cmdline='' size=5584 bytes
[35862184677] [[35mTRACE[0m] [kernel] [CPU0]   Module[175]: name='/share/cursors/future/top_left_corner.svg' cmdline='' size=7121 bytes
[35862758613] [[35mTRACE[0m] [kernel] [CPU0]   Module[176]: name='/share/cursors/future/top_right_corner.svg' cmdline='' size=7139 bytes
[35863328094] [[35mTRACE[0m] [kernel] [CPU0]   Module[177]: name='/share/cursors/future/top_side.svg' cmdline='' size=10651 bytes
[35863900908] [[35mTRACE[0m] [kernel] [CPU0]   Module[178]: name='/share/cursors/future/up-arrow.svg' cmdline='' size=7653 bytes
[35864490783] [[35mTRACE[0m] [kernel] [CPU0]   Module[179]: name='/share/cursors/future/vertical-text.svg' cmdline='' size=5621 bytes
[35865225495] [[35mTRACE[0m] [kernel] [CPU0]   Module[180]: name='/share/cursors/future/wait-01.svg' cmdline='' size=15308 bytes
[35866173255] [[35mTRACE[0m] [kernel] [CPU0]   Module[181]: name='/share/cursors/future/wait-02.svg' cmdline='' size=6672 bytes
[35868151341] [[35mTRACE[0m] [kernel] [CPU0]   Module[182]: name='/share/cursors/future/wait-03.svg' cmdline='' size=6686 bytes
[35868793158] [[35mTRACE[0m] [kernel] [CPU0]   Module[183]: name='/share/cursors/future/wait-04.svg' cmdline='' size=7827 bytes
[35869672938] [[35mTRACE[0m] [kernel] [CPU0]   Module[184]: name='/share/cursors/future/wait-05.svg' cmdline='' size=7835 bytes
[35870232024] [[35mTRACE[0m] [kernel] [CPU0]   Module[185]: name='/share/cursors/future/wait-06.svg' cmdline='' size=8976 bytes
[35870769495] [[35mTRACE[0m] [kernel] [CPU0]   Module[186]: name='/share/cursors/future/wait-07.svg' cmdline='' size=8989 bytes
[35871469689] [[35mTRACE[0m] [kernel] [CPU0]   Module[187]: name='/share/cursors/future/wait-08.svg' cmdline='' size=10126 bytes
[35872014024] [[35mTRACE[0m] [kernel] [CPU0]   Module[188]: name='/share/cursors/future/wait-09.svg' cmdline='' size=10140 bytes
[35872812855] [[35mTRACE[0m] [kernel] [CPU0]   Module[189]: name='/share/cursors/future/wait-10.svg' cmdline='' size=11277 bytes
[35873585715] [[35mTRACE[0m] [kernel] [CPU0]   Module[190]: name='/share/cursors/future/wait-11.svg' cmdline='' size=11291 bytes
[35875108896] [[35mTRACE[0m] [kernel] [CPU0]   Module[191]: name='/share/cursors/future/wait-12.svg' cmdline='' size=11277 bytes
[35890988298] [[35mTRACE[0m] [kernel] [CPU0]   Module[192]: name='/share/cursors/future/wait-13.svg' cmdline='' size=10140 bytes
[35899703301] [[35mTRACE[0m] [kernel] [CPU0]   Module[193]: name='/share/cursors/future/wait-14.svg' cmdline='' size=10126 bytes
[35900508039] [[35mTRACE[0m] [kernel] [CPU0]   Module[194]: name='/share/cursors/future/wait-15.svg' cmdline='' size=8991 bytes
[35901096726] [[35mTRACE[0m] [kernel] [CPU0]   Module[195]: name='/share/cursors/future/wait-16.svg' cmdline='' size=8978 bytes
[35901632382] [[35mTRACE[0m] [kernel] [CPU0]   Module[196]: name='/share/cursors/future/wait-17.svg' cmdline='' size=7842 bytes
[35902163088] [[35mTRACE[0m] [kernel] [CPU0]   Module[197]: name='/share/cursors/future/wait-18.svg' cmdline='' size=7829 bytes
[35902709271] [[35mTRACE[0m] [kernel] [CPU0]   Module[198]: name='/share/cursors/future/wait-19.svg' cmdline='' size=6822 bytes
[35903236743] [[35mTRACE[0m] [kernel] [CPU0]   Module[199]: name='/share/cursors/future/wait-20.svg' cmdline='' size=6679 bytes
[35903763522] [[35mTRACE[0m] [kernel] [CPU0]   Module[200]: name='/share/cursors/future/wait-21.svg' cmdline='' size=5542 bytes
[35904410817] [[35mTRACE[0m] [kernel] [CPU0]   Module[201]: name='/share/cursors/future/wait-22.svg' cmdline='' size=5528 bytes
[35905099527] [[35mTRACE[0m] [kernel] [CPU0]   Module[202]: name='/share/cursors/future/wait-23.svg' cmdline='' size=5546 bytes
[35906378277] [[35mTRACE[0m] [kernel] [CPU0]   Module[203]: name='/share/cursors/future/wait.svg' cmdline='' size=5525 bytes
[35917837857] [[35mTRACE[0m] [kernel] [CPU0]   Module[204]: name='/share/cursors/future/wayland-cursor.svg' cmdline='' size=8222 bytes
[35918411562] [[35mTRACE[0m] [kernel] [CPU0]   Module[205]: name='/share/cursors/future/x-cursor.svg' cmdline='' size=6822 bytes
[35918962596] [[35mTRACE[0m] [kernel] [CPU0]   Module[206]: name='/share/cursors/future/zoom-in.svg' cmdline='' size=5441 bytes
[35919509604] [[35mTRACE[0m] [kernel] [CPU0]   Module[207]: name='/share/cursors/future/zoom-out.svg' cmdline='' size=5403 bytes
[35920044468] [[35mTRACE[0m] [kernel] [CPU0]   Module[208]: name='/share/fonts/unifont.hex' cmdline='' size=8355852 bytes
[35920592664] [[35mTRACE[0m] [kernel] [CPU0]   Module[209]: name='/etc/locale.conf' cmdline='' size=85 bytes
[35921117529] [[35mTRACE[0m] [kernel] [CPU0]   Module[210]: name='/etc/profile' cmdline='' size=68 bytes
[35921616819] [[35mTRACE[0m] [kernel] [CPU0]   Module[211]: name='/etc/motd' cmdline='' size=610 bytes
[35922124227] [[35mTRACE[0m] [kernel] [CPU0]   Module[212]: name='/etc/fstab' cmdline='' size=127 bytes
[35922620547] [[35mTRACE[0m] [kernel] [CPU0]   Module[213]: name='/etc/hostname' cmdline='' size=8 bytes
[35923253850] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[36188439870] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[36189919458] [[34mDEBUG[0m] [kernel] [CPU0] Found init module: /bin/sprout (cmdline: 'init'), loading...
[36192082608] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/sprout' (len=89912, base=0x200000)
[36192893352] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[36207278811] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[36209759718] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[36216231051] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [49, 3b, 6e, 18, 75, 13, 49, 8d]
[36223569525] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20c000 exec=false
[36225489696] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20f000 exec=false
[36229576482] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x210000 filesz=0 memsz=0 align=1
[36240282672] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='load_init_module' elapsed_ticks=48503961 elapsed_us=24251 total_ticks=1114967106 total_us=557483
[36242457735] [[34mDEBUG[0m] [kernel] [CPU0] Warning: Module registry page overflow, truncating list.
[36243045531] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='prepare_boot_registry' elapsed_ticks=1652013 elapsed_us=826 total_ticks=1117759302 total_us=558879
[36243913431] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='map_boot_registry' elapsed_ticks=166386 elapsed_us=83 total_ticks=1118627829 total_us=559313
[36245164098] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='activate_init_aspace' elapsed_ticks=439263 elapsed_us=219 total_ticks=1119812694 total_us=559906
[36246038136] [[34mDEBUG[0m] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[36246636063] [[34mDEBUG[0m] [kernel] [CPU0] Spawning init process...
[36248826900] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb006a9a0 arg=0xffffffffb0020a20
[36268382964] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[36278317713] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=5
[36279090870] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[36279862014] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='spawn_init_task' elapsed_ticks=33267399 elapsed_us=16633 total_ticks=1154563278 total_us=577281
[36280579467] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[36441357249] [[34mDEBUG[0m] [kernel::sched::lifecycle] [CPU0] SCHED: early-boot bringup complete; resuming steady-state SMP scheduling
[36442219341] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='end_bringup' elapsed_ticks=1414314 elapsed_us=707 total_ticks=1316923212 total_us=658461
[36442885017] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[36711483435] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1585838034 elapsed_us=792919
[36712287744] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[36731423586] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[36732681711] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] ENTER_USER: TID=5 entry_pc=200000 user_sp=800000
[36791502660] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpus_online=4
[36803120871] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=0 curr=Some(5) runq=1 runq_avg=2 runq_samples=1 ctxsw=1 idle2busy=0 tick=1 ipi=0 enq=3 deq=1 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[36805269072] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=1 curr=Some(2) runq=0 runq_avg=0 runq_samples=42 ctxsw=0 idle2busy=0 tick=36 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[36806686488] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=2 curr=Some(3) runq=0 runq_avg=0 runq_samples=42 ctxsw=0 idle2busy=0 tick=37 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[36807962202] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=3 curr=Some(4) runq=0 runq_avg=0 runq_samples=41 ctxsw=0 idle2busy=0 tick=36 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[36824447748] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=0
[36833492025] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=20
[36852471480] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[36855384522] [[34mDEBUG[0m] [sprout] [CPU0] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7fef70 rip=0x2032e5 rflags=0x202
[36857525562] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[36859195362] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: About to create Supervisor...
[36861068277] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Listing /bin directory...
[36864269376] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/bin' tid=5
[36900067644] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=3
[36910023612] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[36948734130] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Parsed command line: '    '
[36951444057] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Supervisor created, calling run_forever...
[36953071419] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[36965460180] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[36967738071] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[36969524592] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/run/sprout/shell' tid=5
[36975107466] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/etc/default/shell' tid=5
[36979309455] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/bin/sh' tid=5
[36984688785] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=3
[36986720364] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[36992339505] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[36997447410] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[37028213277] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN: page cache miss for '/bin/sh', caching 127864 bytes
[37034422524] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'sh' (len=127864, base=0x200000)
[37036005600] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[37037209176] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[37037893695] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[37040197722] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [0f, 84, 32, 06, 00, 00, 48, 8b]
[37056283506] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=214000 exec=false
[37058754381] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=217000 exec=false
[37060703625] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x218000 filesz=0 memsz=0 align=1
[37065957357] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /bin/sh
[37067753514] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb00a3d40 arg=0xffffffffb0074560
[37071424863] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=6, applying inserts
[37072595670] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[37077391461] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=6
[37078118286] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[37078668594] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[37091635020] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SETUP_STDIO: stdin=Fd(3) stdout=Fd(4) stderr=Fd(5)
[37105925142] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 6
[37116120459] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Task 6 woken, restoring IRQs
[37117176954] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Done for /bin/sh
[37119587703] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[37125048246] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[37126086030] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=6 entry_pc=200000 user_sp=800000
[37128764112] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[37128743751] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=0
[37130751207] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=100
[37132238847] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=3
[37134341508] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=4
[37135150569] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[37137281379] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=5
[37139772582] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/run/motd' tid=6
[37140007641] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[37144361265] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/etc/motd' tid=6
[37149732906] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=6
[37159491072] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=610
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
[37168086054] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/etc/profile' tid=6
[37201115325] [[34mDEBUG[0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=20757
[37222407948] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=6
[37233052626] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=97
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [37238981109] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=6
[?25h[37240604082] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[37356665445] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[37358744676] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[37362817173] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/cambium' (len=144768, base=0x200000)
[37363458594] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[37364509677] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[37365110277] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[37370653155] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [8d, 3c, f0, 48, 83, c7, 70, 48]
[37382695647] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=217000 exec=false
[37384474380] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[37385792070] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x21b000 filesz=0 memsz=0 align=1
[37391075931] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb00bd7c0 arg=0xffffffffb00745a0
[37393940958] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[37399799217] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=7
[37401067968] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[37408253817] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 7 → task '/bin/cambium' (pid=7 from boot module)
[37413669513] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[37414715349] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=7 entry_pc=200000 user_sp=800000
[37416802731] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=0
[37418712177] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=100
[37422924462] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[37426504599] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned cambium (PID=7)
[37429533339] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[37431777867] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Deferring netd until /dev/net/virtio0/rx is ready...
[37433806542] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[37435604382] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[37437157065] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[37438707108] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[37440038757] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[37451418510] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[37452425670] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN: page cache miss for '/bin/bristle', caching 68744 bytes
[37452942846] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers' tid=7
[37456180773] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'bristle' (len=68744, base=0x200000)
[37456691118] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_readdir: fd=3 len=4096
[37456890108] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[37458234198] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[37458424872] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_readdir: natural phase begin offset=0
[37459209975] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[37471739580] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [cc, fc, ff, ff, 45, 31, ff, e9]
[37475885865] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=209000 exec=false
[37477390566] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20b000 exec=false
[37478767260] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20c000 filesz=0 memsz=0 align=1
[37483082967] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /bin/bristle
[37483832133] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb00e9360 arg=0xffffffffb00c6a00
[37485460551] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=8, applying inserts
[37486171437] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[37486846980] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=8
[37487557338] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[37488172359] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[37489842555] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[37490428668] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SETUP_STDIO: stdin=Null stdout=Inherit stderr=Inherit
[37493342139] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ahci_disk' tid=7
[37508357535] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 8
[37509724494] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Task 8 woken, restoring IRQs
[37510418319] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Done for /bin/bristle
[37511156925] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/bristle' TID=8 PID=8
[37511439735] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[37512256617] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=8 entry_pc=200000 user_sp=800000
[37513185171] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=0
[37514210976] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=100
[37520444082] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[37525306830] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[37527835092] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[37528086552] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/session/active_ui' tid=8
[37531065759] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[37532757669] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/fb0' tid=5
[37533523830] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_open: path='/dev/fb0' len=8 flags=0x0
[37534861254] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] devfs: lookup entry path='fb0' len=3
[37536372126] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] devfs: dynamic registry hit path='fb0'
[37539358692] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=3
[37539347241] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_open: resolved node for /dev/fb0 mode=0o20666 size=32 ino=4
[37540486797] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_open: process info present for /dev/fb0
[37541270448] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_open: handle_table.open('/dev/fb0') -> 3
[37543287276] [[35mTRACE[0m] [bristle] [CPU2] bristle: active_ui set to 'bloom'
[37548380463] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/run/bristle/pid' tid=8
[37556431275] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] FbNode::read: off=0 n=32 buf_len=32 total=32
[37559384313] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=3
[37563282042] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[37565987151] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/drivers/display_virtio_gpu' tid=5
[37569825777] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=3
[37571839404] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[37573658760] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[37578331692] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=3
[37578927606] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=4096
[37580154711] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[37580664627] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(0)
[37582074519] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=4096
[37583103063] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(1)
[37587718806] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=1 port_id=PortId(1) mode=Write
[37587706959] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=1 port_id=PortId(0) mode=Write
[37590252249] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=2 port_id=PortId(1) mode=Read
[37590306204] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=2 port_id=PortId(0) mode=Read
[37592394609] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=4096
[37593211755] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(2)
[37593855486] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(2) mode=Write
[37594371969] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=4 port_id=PortId(2) mode=Read
[37597393713] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/run/bristle/kbd_in' tid=8
[37603993812] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[37605514155] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=3
[37606167720] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=4096
[37606701132] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(3)
[37606836201] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/run/bristle/mouse_in' tid=8
[37607485509] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=3 port_id=PortId(3) mode=Write
[37608141153] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=4 port_id=PortId(3) mode=Read
[37610888502] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=3
[37612545036] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[37613504049] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/proc/self/inbox' tid=8
[37620537339] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[37622764311] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=2 -> fd=4 node=0xffffffffb00c6b90 port=0xffffffffb00f5470
[37625538258] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=4 -> fd=5 node=0xffffffffb01f92b0 port=0xffffffffb01f71f0
[37627413021] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[37632406548] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[37638082251] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[37663533963] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN: page cache miss for '/drivers/display_virtio_gpu', caching 147560 bytes
[37665490236] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'display_virtio_gpu' (len=147560, base=0x200000)
[37666393314] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[37667546367] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[37668159837] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[37670461554] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [00, 00, 00, ba, 12, 00, 00, 00]
[37680989973] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=216000 exec=false
[37683626310] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[37685318352] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x21c000 filesz=0 memsz=0 align=1
[37689822555] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/display_virtio_gpu
[37690853178] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb022da20 arg=0xffffffffb01f9740
[37692590067] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=9, applying inserts
[37693141662] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[37693967652] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=9
[37694554491] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[37695027513] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[37697394702] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SETUP_STDIO: stdin=Null stdout=Inherit stderr=Inherit
[37716937698] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 9
[37718264562] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Task 9 woken, restoring IRQs
[37718792001] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Done for /drivers/display_virtio_gpu
[37719429462] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/drivers/display_virtio_gpu' TID=9 PID=9
[37721434938] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x3
[37722465363] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=9 entry_pc=200000 user_sp=800000
[37724836545] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=0
[37726835520] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=100
[37730272965] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[37732625964] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[37732788093] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[37734905637] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Starting VFS-native VirtIO GPU driver...
[37736337705] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: boot_arg=3
[37736418192] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0 mounted=false, service_ready=false)...
[37738113633] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Mapping bootstrap memfd 3 size=4096...
[37740454818] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready for bloom yet
[37741646712] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: vm_map success at 0x400000001000
[37743693669] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Recovered handles: req_read=2, resp_write=3, svc=0, id=322371585
[37745576154] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3, svc=0, id=322371585)
[37745981823] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[37747157316] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Scanning /sys/devices for PCI GPU...
[37748675580] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices' tid=9
[37749519390] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[37751584728] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices'
[37752538560] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[37756916538] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/proc/self/inbox' tid=5
[37757840142] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[37758108696] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_readdir: fd=4 len=4096
[37759785822] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_readdir: natural phase begin offset=0
[37764956328] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU3] sysfs: readdir found 12 slots
[37765729815] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[37768411263] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ata_disk' tid=7
[37768871943] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[37770772347] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU3] sysfs: readdir wrote 173 bytes
[37772885304] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37806078519] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/platform-fb000000/vendor' tid=9
[37808020074] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/platform-fb000000/vendor'
[37810946448] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'platform-fb000000/vendor'
[37813954893] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37817110452] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/platform-fb000000/device' tid=9
[37818655050] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/platform-fb000000/device'
[37819696530] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'platform-fb000000/device'
[37821658380] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37823871294] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/platform-fb000000/class' tid=9
[37825486908] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/platform-fb000000/class'
[37826860137] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'platform-fb000000/class'
[37828811427] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37830073446] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/vendor' tid=9
[37831382226] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:00.0/vendor'
[37832488155] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:00.0/vendor'
[37834057140] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37835125713] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/device' tid=9
[37836192471] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:00.0/device'
[37837218936] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:00.0/device'
[37839137094] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37840077825] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/class' tid=9
[37841254308] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:00.0/class'
[37842317040] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:00.0/class'
[37844065017] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37845307632] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/vendor' tid=9
[37847242323] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/vendor'
[37849050756] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:01.0/vendor'
[37851116523] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37853839188] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/device' tid=9
[37855198359] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/device'
[37856315970] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:01.0/device'
[37859508621] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37862302005] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/class' tid=9
[37865456442] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/class'
[37866930354] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:01.0/class'
[37869299490] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37875943083] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: task 9 claimed device 'pci-0000:00:01.0' (handle 0)
[37877458278] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/virtio/common_bar' tid=9
[37879016439] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/common_bar'
[37886760450] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37888648908] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/virtio/common_offset' tid=9
[37890696492] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/common_offset'
[37896330945] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37898592864] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/virtio/notify_bar' tid=9
[37900140267] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/notify_bar'
[37903476204] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37904655162] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/virtio/notify_offset' tid=9
[37905866361] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/notify_offset'
[37908959880] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37909999677] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/virtio/notify_multiplier' tid=9
[37911173289] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/notify_multiplier'
[37915288686] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37916689437] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[37919214729] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: mapping BAR2 (phys=0xc000000000, size=0x4000) for task 9
[37922367219] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10000000
[37927140141] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: DMA alloc 1 pages phys=0x2329000 -> user_va=0x10004000
[37932132645] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[37936864185] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: DMA alloc 4 pages phys=0x232a000 -> user_va=0x10005000
[37939213983] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[37940265429] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Virgl 3D not supported, using 2D only
[37941181773] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/firmware/framebuffer' tid=9
[37942586286] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='firmware/framebuffer'
[37944614730] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[37948572585] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37948499688] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[37949933670] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/chime' tid=7
[37952331615] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=1
[38091864888] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[38093988075] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[38095657545] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[38097266394] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[38098843398] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0 mounted=false, service_ready=false)...
[38101948863] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready for bloom yet
[38104098384] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[38107585923] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[38109631098] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[38111660367] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[38112795897] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[38115083259] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[38116154142] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/display_bootfb' tid=7
[38291666655] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[38295653319] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[38297266425] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/display_virtio_gpu' tid=7
[38405190021] [[34mDEBUG[0m] [kernel::syscall::handlers::memory] [CPU3] sys_vm_map: large backing file mapping offset=0 len=8294400 total_size=8294400
[38468995785] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[38473814577] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[38476676634] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[38479311189] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[38481295413] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0 mounted=false, service_ready=false)...
[38482781865] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[38484335703] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready for bloom yet
[38485852218] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[38486478822] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[38487053880] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/hdaudio' tid=7
[38491270653] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[38492814030] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[38494983879] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[38500400631] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: frame pool ready (1 buffer)
[38504866422] [[34mDEBUG[0m] [kernel::ipc] [CPU3] CREATE_PORT: capacity=65536
[38505615621] [[34mDEBUG[0m] [kernel::ipc] [CPU3] CREATE_PORT: appended id=PortId(4)
[38506275357] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU3] ALLOC_HANDLE: handle=5 port_id=PortId(4) mode=Write
[38506821969] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU3] ALLOC_HANDLE: handle=6 port_id=PortId(4) mode=Read
[38508632547] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU3] BRIDGE: handle=5 -> fd=5 node=0xffffffffb00745b0 port=0xffffffffb01065f0
[38511776787] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/proc/5/inbox' tid=9
[38541518796] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU3] SENDMSG: thing=6 data_len=44 caps_len=1 node=Pointer { addr: 0xffffffffb0106a10, metadata: DynMetadata(0xffffffff802c6ec0) }
[38550127737] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_BIND_READY (ID: 322371585)
[38551432656] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Waiting for BIND_ASSIGNED...
[38556282897] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU0] RECVMSG: thing=4 data_len=44 caps_len=1 node=Pointer { addr: 0xffffffffb00d90d0, metadata: DynMetadata(0xffffffff802c6ec0) }
[38566944504] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: BIND_READY from instance_id=0x13370001
[38569478310] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Registering DISPLAY_CARD at /dev/display/card0 (handle=5)
[38578268784] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=262400
[38579016630] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(5)
[38580761340] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=7 port_id=PortId(5) mode=Write
[38585965671] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] vfs: mounted userland provider at /dev/display/card0 (flags: 0x0) port=0xffffffffb01065f0
[38590064964] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[38592110667] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Handshake complete ��� task 'display' marked ready
[38600686542] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Sent MSG_BIND_ASSIGNED to display pid 9 (res=Ok(()))
[38602243020] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[38602380498] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[38603712246] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU3] SENDMSG: thing=6 data_len=44 caps_len=0 node=Pointer { addr: 0xffffffffb0106a10, metadata: DynMetadata(0xffffffff802c6ec0) }
[38604959019] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_SERVICE_READY.
[38605990368] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU3] BRIDGE: handle=2 -> fd=7 node=0xffffffffb00f5770 port=0xffffffffb00f36f0
[38607455370] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU3] BRIDGE: handle=3 -> fd=8 node=0xffffffffb01178f0 port=0xffffffffb01f8ff0
[38608721547] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU3] BRIDGE: handle=6 -> fd=9 node=0xffffffffb0117a70 port=0xffffffffb01065f0
[38611003992] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[38616966762] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU0] RECVMSG: thing=4 data_len=44 caps_len=0 node=Pointer { addr: 0xffffffffb00d90d0, metadata: DynMetadata(0xffffffff802c6ec0) }
[38620219968] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: SERVICE_READY from instance_id=0x13370001
[38622134727] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Service 'display' reported ready
[38624129445] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[38625756411] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[38653121001] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[38655386352] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[38656390113] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/pci_stubd' tid=7
[38816943858] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[38819441958] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[38820490896] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ps2_kbd' tid=7
[38941561659] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[38943133911] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[38944557036] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[38945920992] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[38947577922] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0 mounted=true, service_ready=true)...
[38950500765] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/bloom' (len=462464, base=0x200000)
[38951150271] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[38952400344] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[38953197657] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[38955359223] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [29, 44, 24, 60, 48, 8b, 44, 24]
[38995852170] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[38998268430] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[38999338554] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ps2_mouse' tid=7
[39010043589] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=253000 exec=false
[39015394176] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=25a000 exec=false
[39017767503] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x25d000 filesz=0 memsz=0 align=1
[39029409210] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb0137b80 arg=0xffffffffb00d9160
[39031062081] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[39031572360] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=10
[39032128905] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[39036237306] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 10 → task '/bin/bloom' (pid=10 from boot module)
[39055826634] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[39058848510] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[39061234674] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[39063612489] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[39065783658] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[39109073190] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[39110811663] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=10 entry_pc=200000 user_sp=800000
[39116943888] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=0
[39119279694] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=100
[39123705093] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[39125217252] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[39128208339] [[34mDEBUG[0m] [bloom] [CPU1] bloom: connect try 0...
[39129524148] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/dev/display/card0' tid=10
[39132067920] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=Lookup len=4 tid=10
[39148641906] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=Lookup
[39150354210] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: vfs_lookup path=''
[39153547587] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=Lookup req_id=1
[39161134815] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=Lookup req_id=1
[39267625881] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=Lookup id=1 -> OK(8)
[39285208281] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=48 tid=10
[39294429042] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[39296806197] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[39297811278] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/rtc_cmos' tid=7
[39309833574] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[39311479218] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_GET_INFO requested
[39312875613] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: Returning dimensions 1920x1080
[39314505021] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=2
[39318987840] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=2
[39386413638] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=2 -> OK(48)
[39392051325] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[39407166447] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[39409662138] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[39411210828] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[39413146476] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[39415009029] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[39416956920] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[39419403738] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[39420824784] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[39425086800] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1920x1080 @ 60000mHz
[39426100890] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[39426760758] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[39427594239] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[39431571729] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=65536
[39432261000] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(6)
[39433012707] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=5 port_id=PortId(6) mode=Write
[39433689900] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=6 port_id=PortId(6) mode=Read
[39440521791] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/services/bloom' tid=10
[39448233792] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[39459037299] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/components' tid=10
[39462652812] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[39463857708] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/status' tid=10
[39467100915] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[39469223343] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/lib/libpistil.so' tid=10
[39607272474] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[39609623163] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[39610654215] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/rtl8168d' tid=7
[39737465064] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[39739055136] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[39740455359] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[39742009824] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[39743412621] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[39745135584] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[39747081627] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[39749480496] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[39767506680] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[39769746687] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[39770645211] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/virtio_netd' tid=7
[40040852544] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[40068894888] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[40070516739] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[40071996327] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[40073552508] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[40075079187] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[40076773176] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[40078884087] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[40080579759] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[40162075635] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[40165022601] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[40166716128] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/virtio_sound' tid=7
[40279172406] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[40280455809] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[40281491085] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[40282322751] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil nine-slice shadow renderer loaded
[40283240316] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/desktop/theme' tid=10
[40285999974] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/desktop/theme' tid=10
[40290172560] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[40292390952] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Solarized Warm
[40400853636] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[40403025465] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[40404773244] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[40406190627] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[40407854487] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[40409420040] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[40411646847] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[40414000803] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[40697104701] [[34mDEBUG[0m] [kernel::syscall::handlers::memory] [CPU1] sys_vm_map: large backing file mapping offset=0 len=8294400 total_size=8294400
[40733256498] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[40735612698] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[40739731461] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[40742421621] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[40745070960] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[40747928331] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[40750746729] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[40753629609] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[40878669414] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[40881870645] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[40884863712] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[40886417451] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/device_snapshot' tid=7
[40888778238] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='device_snapshot'
[40896007383] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[40902386844] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:00.0 kind=pci_device vendor=0x8086 device=0x29c0 class=0x060000 present=true
[40905416112] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:01.0 kind=pci_device vendor=0x1af4 device=0x1050 class=0x030000 present=true
[40908212928] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:02.0 kind=pci_device vendor=0x1af4 device=0x1000 class=0x020000 present=true
[40909896456] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: isolation mode skipping driver '/drivers/virtio_netd' class=Net for pci-0000:00:02.0
[40911135771] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:03.0 kind=pci_device vendor=0x1af4 device=0x1059 class=0x040100 present=true
[40912944864] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: isolation mode skipping driver '/drivers/virtio_sound' class=Audio for pci-0000:00:03.0
[40914204705] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:1f.0 kind=pci_device vendor=0x8086 device=0x2918 class=0x060100 present=true
[40915442007] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:1f.2 kind=pci_device vendor=0x8086 device=0x2922 class=0x010601 present=true
[40918121112] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.2/status' tid=7
[40919420718] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/status'
[40920964887] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/status'
[40922954424] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[40937918076] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: page cache miss for '/drivers/ahci_disk', caching 53784 bytes
[40939432512] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'ahci_disk' (len=53784, base=0x200000)
[40940257611] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[40941434358] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[40942107789] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=200000 exec=true
[40944443100] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   COPIED at 0x201420: [24, 0c, e8, 59, 0e, 00, 00, bf]
[40947428115] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=207000 exec=false
[40948926546] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=208000 exec=false
[40950942450] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x209000 filesz=0 memsz=0 align=1
[40962007482] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[40963437669] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/ahci_disk
[40965459777] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb0164da0 arg=0xffffffffb01241e0
[40985482197] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=11, applying inserts
[40986340395] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[40987154868] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=11
[40988149521] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[40988693427] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[40991262939] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[40997873994] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 11
[41001452745] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 11 woken, restoring IRQs
[41002099578] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/ahci_disk
[41003014404] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=11 PID=11
[41003821221] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x3
[41004456504] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=11 entry_pc=201000 user_sp=800000
[41005472838] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=11 buf_len=0
[41006689614] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=11 buf_len=100
[41010097689] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Starting AHCI VFS driver
[41018423325] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=88 tid=10
[41027407014] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/ahci_disk for pci-0000:00:1f.2 (entry='thingos_driver_start_safe', pid=11)
[41033284710] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: sent DRIVER_READY to Sprout (pid=5) for driver pid=11
[41035014042] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:1f.3 kind=pci_device vendor=0x8086 device=0x2930 class=0x0c0500 present=true
[41036584248] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=isa-0070 kind=dev.rtc.Cmos vendor=0x0000 device=0x0000 class=0x000000 present=true
[41037381099] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[41038637574] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_IMPORT_BUFFER requested: memfd=10, size=1920x1080
[41040079674] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0070/status' tid=7
[41040150426] [[34mDEBUG[0m] [kernel::syscall::handlers::memory] [CPU3] sys_vm_map: large backing file mapping offset=0 len=8294400 total_size=8294400
[41040165672] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU0] RECVMSG: thing=4 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00d90d0, metadata: DynMetadata(0xffffffff802c6ec0) }
[41042431419] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/status'
[41044998126] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/status'
[41048103756] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=4
[41057326992] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: DRIVER_READY from PID 11 — no matching ManagedTask (already exited?)
[41059489152] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[41061018108] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 'pci-0000:00:1f.2' (handle 1)
[41063384703] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: page cache miss for '/drivers/rtc_cmos', caching 45904 bytes
[41064004080] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[41065080705] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: mapping BAR5 (phys=0x80880000, size=0x1000) for task 11
[41065347840] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'rtc_cmos' (len=45904, base=0x200000)
[41066307975] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[41066482809] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10009000
[41068049979] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[41068977609] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=200000 exec=true
[41070803664] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: DMA alloc 1 pages phys=0x35a8000 -> user_va=0x1000a000
[41072259789] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   COPIED at 0x201420: [0f, 84, ea, 03, 00, 00, b8, 00]
[41076037002] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=206000 exec=false
[41077584801] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=207000 exec=false
[41078394489] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=65536
[41079369309] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x208000 filesz=0 memsz=0 align=1
[41085568293] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(7)
[41086879779] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=1 port_id=PortId(7) mode=Write
[41087837769] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=2 port_id=PortId(7) mode=Read
[41091848160] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[41092887297] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/rtc_cmos
[41094476742] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb01a9800 arg=0xffffffffb016d420
[41096076120] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=262400
[41096921514] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(8)
[41097826077] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=12, applying inserts
[41098114002] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(8) mode=Write
[41098587717] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[41103467988] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=12
[41104316913] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[41105021034] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[41107598268] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] vfs: mounted userland provider at /dev/storage/atapi2 (flags: 0x0) port=0xffffffffb0189590
[41108094027] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[41109885729] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Mounted atapi2 at /dev/storage/atapi2
[41111271366] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Provider loop online at /dev/storage/atapi2
[41113310040] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 12
[41116872885] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 12 woken, restoring IRQs
[41117544006] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/rtc_cmos
[41118623337] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/rtc_cmos' TID=12 PID=12
[41123184630] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/rtc_cmos for isa-0070 (entry='thingos_driver_start_safe', pid=12)
[41127243564] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: sent DRIVER_READY to Sprout (pid=5) for driver pid=12
[41130374175] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=isa-0060 kind=drv.Ps2Keyboard vendor=0x0000 device=0x0000 class=0x000000 present=true
[41131854819] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0060/status' tid=7
[41133923424] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/status'
[41135491518] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/status'
[41137761060] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=5
[41141320110] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU0] RECVMSG: thing=4 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00d90d0, metadata: DynMetadata(0xffffffff802c6ec0) }
[41146730295] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: DRIVER_READY from PID 12 — no matching ManagedTask (already exited?)
[41149003764] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[41151160941] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: page cache miss for '/drivers/ps2_kbd', caching 72264 bytes
[41152408341] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'ps2_kbd' (len=72264, base=0x200000)
[41153014287] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[41154079230] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[41154725700] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=200000 exec=true
[41156857236] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   COPIED at 0x201420: [83, ff, 06, 77, 46, 48, 8d, 15]
[41160790374] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=209000 exec=false
[41162881023] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=20c000 exec=false
[41164131129] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x20d000 filesz=0 memsz=0 align=1
[41169615366] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[41172936750] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/ps2_kbd
[41173910283] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb01e5b20 arg=0xffffffffb01c3e40
[41175758052] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=13, applying inserts
[41176384590] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[41176930740] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=13
[41177615391] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[41178157482] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[41181481803] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[41186806881] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 13
[41189932773] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 13 woken, restoring IRQs
[41190604752] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/ps2_kbd
[41191330818] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ps2_kbd' TID=13 PID=13
[41194655403] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x5
[41195543796] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=13 entry_pc=201000 user_sp=800000
[41197364274] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=13 buf_len=0
[41198633883] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=13 buf_len=100
[41202227286] [[34mDEBUG[0m] [ps2_kbd] [CPU1] ps2_kbd: online — waiting for bristle pid
[41203449738] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/run/bristle/pid' tid=13
[41206466928] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/run/bristle/pid' tid=13
[41209603050] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[41213395245] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: bristle pid=8
[41217509025] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: task subscribed to vector 0x21
[41219664552] [[34mDEBUG[0m] [ps2_kbd] [CPU1] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[41221082496] [[34mDEBUG[0m] [ps2_kbd] [CPU1] ps2_kbd: using interrupt-driven loop (IRQ vector 0x21)
[41225898054] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/ps2_kbd for isa-0060 (entry='thingos_driver_start_safe', pid=13)
[41229332331] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: sent DRIVER_READY to Sprout (pid=5) for driver pid=13
[41230355628] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=isa-0064 kind=drv.Ps2Mouse vendor=0x0000 device=0x0000 class=0x000000 present=true
[41231219898] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0064/status' tid=7
[41232431856] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0064/status'
[41234226066] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0064/status'
[41235858081] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x4
[41235902136] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU0] RECVMSG: thing=4 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00d90d0, metadata: DynMetadata(0xffffffff802c6ec0) }
[41236348164] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=6
[41236685655] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=12 entry_pc=201000 user_sp=800000
[41239456863] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=12 buf_len=0
[41241346674] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=12 buf_len=100
[41245260870] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: DRIVER_READY from PID 13 — no matching ManagedTask (already exited?)
[41247033894] [[34mDEBUG[0m] [rtc_cmos] [CPU3] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffef0 rip=0x201c27 rflags=0x206
[41248519719] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[41248749267] [[34mDEBUG[0m] [rtc_cmos] [CPU3] Starting... arg=4
[41250652542] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: page cache miss for '/drivers/ps2_mouse', caching 71096 bytes
[41252395536] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'ps2_mouse' (len=71096, base=0x200000)
[41252851233] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[41254299603] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[41255100348] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=200000 exec=true
[41259014544] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   COPIED at 0x201420: [83, f8, 02, 75, 1a, b8, 0b, 10]
[41263671471] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=209000 exec=false
[41265469047] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=20c000 exec=false
[41266770237] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x20d000 filesz=0 memsz=0 align=1
[41273110989] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[41273980209] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/ps2_mouse
[41274947406] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb0258560 arg=0xffffffffb01ef100
[41277053268] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=14, applying inserts
[41277618360] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[41278157118] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=14
[41278764417] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[41279188632] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[41281552653] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[41286727251] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 14
[41288262411] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 14 woken, restoring IRQs
[41288685966] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/ps2_mouse
[41289406917] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ps2_mouse' TID=14 PID=14
[41294698005] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: task 12 claimed device 'isa-0070' (handle 2)
[41290389624] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x6
[41296397967] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=14 entry_pc=201000 user_sp=800000
[41297021007] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/ps2_mouse for isa-0064 (entry='thingos_driver_start_safe', pid=14)
[41297239632] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: claimed /sys/devices/isa-0070 (handle=2)
[41298016848] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=14 buf_len=0
[41299362126] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: sent DRIVER_READY to Sprout (pid=5) for driver pid=14
[41299557750] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=14 buf_len=100
[41300228772] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=isa-01f0 kind=dev.storage.ata vendor=0x0000 device=0x0000 class=0x000000 present=true
[41303051526] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/proc/13/job_observer' tid=7
[41303947839] [[34mDEBUG[0m] [ps2_mouse] [CPU2] ps2_mouse: online — waiting for bristle pid
[41314225722] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/run/bristle/pid' tid=14
[41314400094] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU0] RECVMSG: thing=4 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00d90d0, metadata: DynMetadata(0xffffffff802c6ec0) }
[41319851166] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: DRIVER_READY from PID 14 — no matching ManagedTask (already exited?)
[41321743386] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[41321804469] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[41321710254] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/run/bristle/pid' tid=14
[41324742294] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/proc/14/job_observer' tid=7
[41331070374] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=8
[41335326516] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[41335401426] [[32mINFO [0m] [ps2_mouse] [CPU2] ps2_mouse: bristle pid=8
[41336939457] [[34mDEBUG[0m] [ps2_mouse] [CPU2] ps2_mouse: enabling aux port
[41337512865] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/proc/12/job_observer' tid=7
[41350131537] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[41352694119] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/proc/11/job_observer' tid=7
[41352751407] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: 2026-05-01 01:02:16 = 1777597336 unix_secs
[41354619966] [[32mINFO [0m] [kernel::time] [CPU3] System clock anchored: unix_secs=1777597336, mono_ns=20677145148, offset=1777597315322854852ns
[41358132057] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: System clock anchored to 1777597336 unix_secs
[41359380117] [[34mDEBUG[0m] [rtc_cmos] [CPU3] RTC: Entering maintenance loop.
[41361366981] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 01:02:16.002567053 unix_secs=1777597336.002567053
[41361876105] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] DISP: imported buffer as ID=1
[41362958439] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x47 (is_aux=false)
[41363003649] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[66959045076] [
```
</details>
