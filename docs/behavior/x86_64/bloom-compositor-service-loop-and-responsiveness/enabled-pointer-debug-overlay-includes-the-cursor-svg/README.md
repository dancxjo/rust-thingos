# ❌ Scenario: enabled pointer debug overlay includes the cursor svg

> Last run: 2026-04-30 18:01:45

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 11063ms | - [📜](./01/serial.log) - |
| 2 | Then the pointer debug overlay should include the cursor svg | ❌ | 61114ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[34093413420] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[34132060512] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] enter
[34135487892] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] phys_memory_map ok
[34136305434] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] modules ok
[34137116277] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] phys_to_virt_offset ok
[34137633057] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] memory map logging done
[34162314021] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] boot_frame_alloc init ok
[34364238447] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] frame allocator build ok
[34365196074] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] frame allocator log ok
[34366216830] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] FRAME_ALLOCATOR init ok
[34367028828] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] tasking init ok
[34367584152] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[34388180772] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[34389041280] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] enter
[34389833676] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] set expand hook
[34390375602] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] expand hook ok
[34390891557] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock begin
[34391775462] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock ok
[34392342963] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] reserve_region begin
[34424427642] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] reserve_region ok
[34425245052] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] inner allocator init begin
[34428224259] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] inner allocator init ok
[34428982401] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] heap top store ok
[34429594947] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] init done
[34430233728] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[34452507474] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] devfs: set_boot_fb width=1920 height=1080 pitch=7680 resource_id=0xfb000000 phys=0x80000000
[34454632806] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs set_boot_fb elapsed_ticks=2059398 elapsed_us=1029
[34460442555] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs FbNode::new elapsed_ticks=5013261 elapsed_us=2506
[34461279171] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] register begin
[34465929069] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] register ok
[34466668236] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs register fb0 elapsed_ticks=5353491 elapsed_us=2676
[34467260850] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[34487469918] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[34489447938] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[34527604188] [[34mDEBUG[0m] [kernel] [CPU0] Seeding entropy pool...
[34529267916] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed begin
[34530440802] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] fill_entropy done
[34532866467] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] add_sample(timer) ok
[34533475317] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] mark_seeded(timer) ok
[34534024305] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[34534610847] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[34535160693] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed done
[34536272661] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] entropy::seed_from_hardware elapsed_ticks=6877431 elapsed_us=3438
[34536921177] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[34558176576] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[34568491056] [[35mTRACE[0m] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[34569771819] [[35mTRACE[0m] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[34570451850] [[35mTRACE[0m] [kernel::sched] [CPU0]   Allocating scheduler...
[34575373767] [[35mTRACE[0m] [kernel::sched] [CPU0]   Leaking scheduler...
[34576045779] [[35mTRACE[0m] [kernel::sched] [CPU0]   Initializing boot task...
[34578655122] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED: allocating CpuScheduler for cpu0 (total=1)
[34579896384] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[34580578989] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[34582287531] [[35mTRACE[0m] [kernel::sched] [CPU0]   Creating boot task...
[34597096347] [[35mTRACE[0m] [kernel::sched] [CPU0]   Creating idle tasks...
[34598433936] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff8008b390 kstack_top=0xffffffffb0020660 arg=0x0
[34600618701] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[34625300391] [[35mTRACE[0m] [kernel::sched] [CPU0]   Boot task initialized
[34627707147] [[34mDEBUG[0m] [kernel::sched::state] [CPU0] SCHED[cpu0]: current=Some(0) idle=Some(1) runnable=0 need_resched=true
[34628976030] [[35mTRACE[0m] [kernel::sched] [CPU0]   Storing scheduler pointer...
[34630185018] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[34631228016] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[34652626371] [[34mDEBUG[0m] [kernel] [CPU0] Initializing VFS...
[34656618282] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='set_cmdline' elapsed_ticks=2981583 elapsed_us=1490 total_ticks=3805758 total_us=1902
[34680214998] [[34mDEBUG[0m] [kernel::vfs] [CPU0] VFS: Created /dev/display directory
[34681272846] [[34mDEBUG[0m] [kernel::vfs] [CPU0] VFS: Created /dev/storage directory
[34724717676] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted union filesystem at / (root)
[34727265144] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted devfs at /dev
[34728396054] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted procfs at /proc
[34729384437] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted sysfs at /sys
[34733874648] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /tmp
[34734787989] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /run
[34735601637] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /services
[34736372781] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /session
[34737209892] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /data
[34737751653] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='vfs_init' elapsed_ticks=80358894 elapsed_us=40179 total_ticks=85234611 total_us=42617
[34738442079] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[34760645898] [[34mDEBUG[0m] [kernel] [CPU0] Scanning PCI bus...
[34765083474] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x29c0 at 00:00.0 class=060000 id=1
[34770988197] [[35mTRACE[0m] [kernel] [CPU0]   BAR0 (MEM32): 0x80000000 (size 0x800000)
[34786912677] [[35mTRACE[0m] [kernel] [CPU0]   BAR2 (MEM64): 0xc000000000 (size 0x4000)
[34791667086] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (MEM32): 0x80883000 (size 0x1000)
[34792458426] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1050 at 00:01.0 class=030000 id=2
[34796563065] [[35mTRACE[0m] [kernel] [CPU0]   BAR0 (I/O):  0x6060 (size 0x20)
[34800546528] [[35mTRACE[0m] [kernel] [CPU0]   BAR1 (MEM32): 0x80882000 (size 0x1000)
[34815951027] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (MEM64): 0xc000004000 (size 0x4000)
[34817397681] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1000 at 00:02.0 class=020000 id=3
[34820422098] [[35mTRACE[0m] [kernel] [CPU0]   BAR1 (MEM32): 0x80881000 (size 0x1000)
[34831063641] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (MEM64): 0xc000008000 (size 0x4000)
[34832402715] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1059 at 00:03.0 class=040100 id=4
[34833750963] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2918 at 00:1f.0 class=060100 id=5
[34839386010] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (I/O):  0x6040 (size 0x20)
[34847765337] [[35mTRACE[0m] [kernel] [CPU0]   BAR5 (MEM32): 0x80880000 (size 0x1000)
[34848667854] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2922 at 00:1f.2 class=010601 id=6
[34851161532] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (I/O):  0x6000 (size 0x40)
[34851926604] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2930 at 00:1f.3 class=0c0500 id=7
[34854215748] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='scan_pci' elapsed_ticks=92780589 elapsed_us=46390 total_ticks=201693525 total_us=100846
[34854945543] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[34878395046] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='register_legacy_devices' elapsed_ticks=736890 elapsed_us=368 total_ticks=225860547 total_us=112930
[34879378743] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[34901592957] [[34mDEBUG[0m] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[34909173420] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[34909974198] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[34911077883] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[34911962448] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[34917815526] [[34mDEBUG[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[34919627292] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[34920445725] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[34920961812] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[34921453215] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[34921939734] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[34922770080] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[34923556470] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[34924060479] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[34924548714] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[34925042427] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[34925557458] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[34926391896] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[34927298538] [[34mDEBUG[0m] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[34927946427] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[34929546630] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Registers initialized
[34930644342] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[34931668563] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[34932843891] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: All pins masked
[34934286288] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> NMI
[34935081093] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[34935902991] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ4 -> GSI 4 -> 0x24
[34936438218] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Init complete
[34971047199] [[34mDEBUG[0m] [bran::arch::x86_64::ioapic] [CPU0] LAPIC: calibrated 62705200 ticks/sec (delta=627052, ok=true) -> init_cnt=627052
[34972765344] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62705200 ticks/sec), init_cnt=627052 for 100Hz
[34973493918] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='setup_preemption_timer' elapsed_ticks=70990128 elapsed_us=35495 total_ticks=320975061 total_us=160487
[34974237837] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[34996743837] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[34997773272] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[34999271340] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[35001407232] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[35036498739] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[35037803823] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[35039287800] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] SMP: CPU 1 initializing preemption timer (vec=32 init_cnt=627052)
[35040913677] [[34mDEBUG[0m] [kernel] [CPU1] SMP: kernel_secondary_entry arg_cpu=1 runtime_cpu=1
[35041635024] [[34mDEBUG[0m] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[35042984757] [[34mDEBUG[0m] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[35046400356] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff8008b390 kstack_top=0xffffffffb00375a0 arg=0x1
[35047153548] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Task 2 assigned to CPU 1
[35051002371] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 2 deferred inserts
[35052164466] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=1
[35052881226] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=2
[35054332698] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[35055138624] [[34mDEBUG[0m] [kernel::sched::lifecycle] [CPU1] SMP: Secondary CPU 1 online!
[35055843207] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: run_scheduler entry on CPU 1
[35056501920] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: bootstrap_cpu start on CPU 1
[35057635470] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle thread 2 and is now schedulable
[35062615566] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[35062524816] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] SMP: CPU 2 initializing preemption timer (vec=32 init_cnt=627052)
[35063220324] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[35064031464] [[34mDEBUG[0m] [kernel] [CPU2] SMP: kernel_secondary_entry arg_cpu=2 runtime_cpu=2
[35064755649] [[34mDEBUG[0m] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[35069759604] [[34mDEBUG[0m] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[35071029015] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU2] INIT KERNEL CTX: entry=0xffffffff8008b390 kstack_top=0xffffffffb0048d20 arg=0x2
[35072902755] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] SCHED: Task 3 assigned to CPU 2
[35074706238] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Applying 1 deferred inserts
[35075425407] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserting TID=3
[35076166851] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserts applied
[35076784710] [[34mDEBUG[0m] [kernel::sched::lifecycle] [CPU2] SMP: Secondary CPU 2 online!
[35077454940] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: run_scheduler entry on CPU 2
[35078055144] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: bootstrap_cpu start on CPU 2
[35082783813] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle thread 3 and is now schedulable
[35089330617] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] SMP: CPU 3 initializing preemption timer (vec=32 init_cnt=627052)
[35091064041] [[34mDEBUG[0m] [kernel] [CPU3] SMP: kernel_secondary_entry arg_cpu=3 runtime_cpu=3
[35089174560] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[35091730641] [[34mDEBUG[0m] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[35093716350] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[35093818353] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[35094649458] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[35095110501] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff8008b390 kstack_top=0xffffffffb005a4a0 arg=0x3
[35095206300] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='start_secondary_cpus' elapsed_ticks=98860344 elapsed_us=49430 total_ticks=442694604 total_us=221347
[35095862868] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SCHED: Task 4 assigned to CPU 3
[35096348826] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[35097700110] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[35098608534] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=4
[35099654106] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[35100508608] [[34mDEBUG[0m] [kernel::sched::lifecycle] [CPU3] SMP: Secondary CPU 3 online!
[35101347105] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: run_scheduler entry on CPU 3
[35103092904] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: bootstrap_cpu start on CPU 3
[35103886059] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle thread 4 and is now schedulable
[35186434932] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[35248551954] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Enumerating 214 boot modules...
[35250273696] [[35mTRACE[0m] [kernel] [CPU0]   Module[0]: name='/bin/sprout' cmdline='init' size=89912 bytes
[35251218090] [[35mTRACE[0m] [kernel] [CPU0]   Module[1]: name='/bin/bristle' cmdline='' size=68744 bytes
[35252057082] [[35mTRACE[0m] [kernel] [CPU0]   Module[2]: name='/drivers/rtc_cmos' cmdline='init' size=45904 bytes
[35252596665] [[35mTRACE[0m] [kernel] [CPU0]   Module[3]: name='/drivers/ps2_kbd' cmdline='' size=72264 bytes
[35253114435] [[35mTRACE[0m] [kernel] [CPU0]   Module[4]: name='/bin/sh' cmdline='init' size=127864 bytes
[35253638244] [[35mTRACE[0m] [kernel] [CPU0]   Module[5]: name='/bin/ls' cmdline='init' size=62136 bytes
[35254146543] [[35mTRACE[0m] [kernel] [CPU0]   Module[6]: name='/bin/ln' cmdline='init' size=47568 bytes
[35254660353] [[35mTRACE[0m] [kernel] [CPU0]   Module[7]: name='/bin/ps' cmdline='init' size=50544 bytes
[35255232210] [[35mTRACE[0m] [kernel] [CPU0]   Module[8]: name='/bin/kill' cmdline='init' size=55584 bytes
[35255799414] [[35mTRACE[0m] [kernel] [CPU0]   Module[9]: name='/bin/killall' cmdline='init' size=67656 bytes
[35256354771] [[35mTRACE[0m] [kernel] [CPU0]   Module[10]: name='/bin/cat' cmdline='init' size=50832 bytes
[35256866766] [[35mTRACE[0m] [kernel] [CPU0]   Module[11]: name='/bin/head' cmdline='init' size=56112 bytes
[35257375131] [[35mTRACE[0m] [kernel] [CPU0]   Module[12]: name='/bin/tail' cmdline='init' size=56088 bytes
[35258191584] [[35mTRACE[0m] [kernel] [CPU0]   Module[13]: name='/bin/top' cmdline='init' size=65488 bytes
[35261141949] [[35mTRACE[0m] [kernel] [CPU0]   Module[14]: name='/bin/wc' cmdline='init' size=56248 bytes
[35261681103] [[35mTRACE[0m] [kernel] [CPU0]   Module[15]: name='/bin/yes' cmdline='init' size=44520 bytes
[35262194649] [[35mTRACE[0m] [kernel] [CPU0]   Module[16]: name='/bin/which' cmdline='init' size=50448 bytes
[35262951240] [[35mTRACE[0m] [kernel] [CPU0]   Module[17]: name='/bin/cp' cmdline='init' size=46312 bytes
[35263996020] [[35mTRACE[0m] [kernel] [CPU0]   Module[18]: name='/bin/mv' cmdline='init' size=47048 bytes
[35264732514] [[35mTRACE[0m] [kernel] [CPU0]   Module[19]: name='/bin/rm' cmdline='init' size=47128 bytes
[35265265497] [[35mTRACE[0m] [kernel] [CPU0]   Module[20]: name='/bin/rmdir' cmdline='init' size=47144 bytes
[35265776634] [[35mTRACE[0m] [kernel] [CPU0]   Module[21]: name='/bin/mkdir' cmdline='init' size=47144 bytes
[35266282359] [[35mTRACE[0m] [kernel] [CPU0]   Module[22]: name='/bin/mount' cmdline='init' size=57688 bytes
[35266790823] [[35mTRACE[0m] [kernel] [CPU0]   Module[23]: name='/bin/echo' cmdline='init' size=44616 bytes
[35267316183] [[35mTRACE[0m] [kernel] [CPU0]   Module[24]: name='/bin/printf' cmdline='init' size=93952 bytes
[35267843358] [[35mTRACE[0m] [kernel] [CPU0]   Module[25]: name='/bin/grep' cmdline='init' size=85240 bytes
[35268375978] [[35mTRACE[0m] [kernel] [CPU0]   Module[26]: name='/bin/pwd' cmdline='init' size=39184 bytes
[35269043139] [[35mTRACE[0m] [kernel] [CPU0]   Module[27]: name='/bin/touch' cmdline='init' size=46160 bytes
[35269574967] [[35mTRACE[0m] [kernel] [CPU0]   Module[28]: name='/bin/setshell' cmdline='init' size=47048 bytes
[35270134680] [[35mTRACE[0m] [kernel] [CPU0]   Module[29]: name='/bin/dmesg' cmdline='init' size=39328 bytes
[35270742738] [[35mTRACE[0m] [kernel] [CPU0]   Module[30]: name='/bin/stat' cmdline='init' size=46200 bytes
[35271387294] [[35mTRACE[0m] [kernel] [CPU0]   Module[31]: name='/bin/file' cmdline='init' size=51104 bytes
[35271908727] [[35mTRACE[0m] [kernel] [CPU0]   Module[32]: name='/bin/dirname' cmdline='init' size=50096 bytes
[35272436331] [[35mTRACE[0m] [kernel] [CPU0]   Module[33]: name='/bin/basename' cmdline='init' size=50104 bytes
[35273035743] [[35mTRACE[0m] [kernel] [CPU0]   Module[34]: name='/bin/sleep' cmdline='init' size=56824 bytes
[35273998617] [[35mTRACE[0m] [kernel] [CPU0]   Module[35]: name='/bin/sort' cmdline='init' size=66336 bytes
[35277101442] [[35mTRACE[0m] [kernel] [CPU0]   Module[36]: name='/bin/env' cmdline='init' size=45832 bytes
[35277624426] [[35mTRACE[0m] [kernel] [CPU0]   Module[37]: name='/bin/uname' cmdline='' size=44632 bytes
[35278237863] [[35mTRACE[0m] [kernel] [CPU0]   Module[38]: name='/bin/true' cmdline='' size=20744 bytes
[35278762860] [[35mTRACE[0m] [kernel] [CPU0]   Module[39]: name='/bin/false' cmdline='' size=20744 bytes
[35279283732] [[35mTRACE[0m] [kernel] [CPU0]   Module[40]: name='/bin/input_echo' cmdline='' size=43016 bytes
[35279803977] [[35mTRACE[0m] [kernel] [CPU0]   Module[41]: name='/drivers/ps2_mouse' cmdline='' size=71096 bytes
[35280330525] [[35mTRACE[0m] [kernel] [CPU0]   Module[42]: name='/drivers/display_bootfb' cmdline='' size=91736 bytes
[35280894957] [[35mTRACE[0m] [kernel] [CPU0]   Module[43]: name='/drivers/display_virtio_gpu' cmdline='' size=147560 bytes
[35281445001] [[35mTRACE[0m] [kernel] [CPU0]   Module[44]: name='/bin/cambium' cmdline='' size=144768 bytes
[35281953333] [[35mTRACE[0m] [kernel] [CPU0]   Module[45]: name='/drivers/virtio_netd' cmdline='' size=123920 bytes
[35282509779] [[35mTRACE[0m] [kernel] [CPU0]   Module[46]: name='/drivers/rtl8168d' cmdline='' size=61392 bytes
[35283040782] [[35mTRACE[0m] [kernel] [CPU0]   Module[47]: name='/bin/netd' cmdline='' size=10819752 bytes
[35283563997] [[35mTRACE[0m] [kernel] [CPU0]   Module[48]: name='/bin/mesocarp' cmdline='' size=123496 bytes
[35284093086] [[35mTRACE[0m] [kernel] [CPU0]   Module[49]: name='/bin/mdns' cmdline='' size=123496 bytes
[35284618347] [[35mTRACE[0m] [kernel] [CPU0]   Module[50]: name='/bin/mdnsd' cmdline='' size=123496 bytes
[35285129088] [[35mTRACE[0m] [kernel] [CPU0]   Module[51]: name='/bin/fetchd' cmdline='' size=76752 bytes
[35285639697] [[35mTRACE[0m] [kernel] [CPU0]   Module[52]: name='/bin/httpsd' cmdline='' size=536128 bytes
[35286227526] [[35mTRACE[0m] [kernel] [CPU0]   Module[53]: name='/bin/find' cmdline='' size=57896 bytes
[35286968805] [[35mTRACE[0m] [kernel] [CPU0]   Module[54]: name='/bin/ip' cmdline='' size=55472 bytes
[35287546008] [[35mTRACE[0m] [kernel] [CPU0]   Module[55]: name='/bin/iso_reader' cmdline='' size=37424 bytes
[35288106447] [[35mTRACE[0m] [kernel] [CPU0]   Module[56]: name='/bin/ping' cmdline='' size=65184 bytes
[35288628540] [[35mTRACE[0m] [kernel] [CPU0]   Module[57]: name='/bin/nslookup' cmdline='' size=56424 bytes
[35289137829] [[35mTRACE[0m] [kernel] [CPU0]   Module[58]: name='/drivers/ahci_disk' cmdline='' size=53784 bytes
[35289802647] [[35mTRACE[0m] [kernel] [CPU0]   Module[59]: name='/drivers/ata_disk' cmdline='' size=64448 bytes
[35290341966] [[35mTRACE[0m] [kernel] [CPU0]   Module[60]: name='/bin/iso9660d' cmdline='' size=78544 bytes
[35291987313] [[35mTRACE[0m] [kernel] [CPU0]   Module[61]: name='/drivers/virtio_sound' cmdline='' size=99296 bytes
[35307762303] [[35mTRACE[0m] [kernel] [CPU0]   Module[62]: name='/drivers/hdaudio' cmdline='' size=67696 bytes
[35308497147] [[35mTRACE[0m] [kernel] [CPU0]   Module[63]: name='/drivers/pci_stubd' cmdline='' size=58168 bytes
[35309334918] [[35mTRACE[0m] [kernel] [CPU0]   Module[64]: name='/drivers/chime' cmdline='' size=59536 bytes
[35310192654] [[35mTRACE[0m] [kernel] [CPU0]   Module[65]: name='/bin/vfs_hello' cmdline='' size=39560 bytes
[35311100022] [[35mTRACE[0m] [kernel] [CPU0]   Module[66]: name='/bin/show_args' cmdline='' size=42488 bytes
[35311954557] [[35mTRACE[0m] [kernel] [CPU0]   Module[67]: name='/bin/clock' cmdline='' size=82936 bytes
[35312829024] [[35mTRACE[0m] [kernel] [CPU0]   Module[68]: name='/bin/env_roundtrip' cmdline='' size=48232 bytes
[35313734643] [[35mTRACE[0m] [kernel] [CPU0]   Module[69]: name='/bin/cwd_test' cmdline='' size=43696 bytes
[35314589508] [[35mTRACE[0m] [kernel] [CPU0]   Module[70]: name='/bin/date' cmdline='' size=67032 bytes
[35315378274] [[35mTRACE[0m] [kernel] [CPU0]   Module[71]: name='/bin/wayland_hello' cmdline='' size=82360 bytes
[35316168261] [[35mTRACE[0m] [kernel] [CPU0]   Module[72]: name='/bin/terminal' cmdline='' size=91056 bytes
[35316968049] [[35mTRACE[0m] [kernel] [CPU0]   Module[73]: name='/bin/tee' cmdline='' size=44968 bytes
[35317764702] [[35mTRACE[0m] [kernel] [CPU0]   Module[74]: name='/bin/xargs' cmdline='' size=55016 bytes
[35318339661] [[35mTRACE[0m] [kernel] [CPU0]   Module[75]: name='/bin/placed' cmdline='' size=38048 bytes
[35318841096] [[35mTRACE[0m] [kernel] [CPU0]   Module[76]: name='/bin/bloom' cmdline='' size=462464 bytes
[35319340980] [[35mTRACE[0m] [kernel] [CPU0]   Module[77]: name='/bin/clear' cmdline='' size=20872 bytes
[35319842085] [[35mTRACE[0m] [kernel] [CPU0]   Module[78]: name='/bin/loglevel' cmdline='' size=41536 bytes
[35320364310] [[35mTRACE[0m] [kernel] [CPU0]   Module[79]: name='/bin/poll_mux' cmdline='' size=39056 bytes
[35320892310] [[35mTRACE[0m] [kernel] [CPU0]   Module[80]: name='/bin/ipc_service_demo' cmdline='' size=47592 bytes
[35321413380] [[35mTRACE[0m] [kernel] [CPU0]   Module[81]: name='/bin/ipc_pipe_demo' cmdline='' size=43936 bytes
[35321924748] [[35mTRACE[0m] [kernel] [CPU0]   Module[82]: name='/bin/ipc_provider_demo' cmdline='' size=84440 bytes
[35322457830] [[35mTRACE[0m] [kernel] [CPU0]   Module[83]: name='/bin/vfs_test_provider' cmdline='' size=70648 bytes
[35323088526] [[35mTRACE[0m] [kernel] [CPU0]   Module[84]: name='/bin/ipc_memfd_demo' cmdline='' size=38232 bytes
[35323628934] [[35mTRACE[0m] [kernel] [CPU0]   Module[85]: name='/bin/test_exec' cmdline='' size=62592 bytes
[35324496372] [[35mTRACE[0m] [kernel] [CPU0]   Module[86]: name='/bin/test_vm_protect' cmdline='' size=38224 bytes
[35325045624] [[35mTRACE[0m] [kernel] [CPU0]   Module[87]: name='/bin/test_exec_env' cmdline='' size=58904 bytes
[35325825282] [[35mTRACE[0m] [kernel] [CPU0]   Module[88]: name='/bin/test_threads' cmdline='' size=82376 bytes
[35329589724] [[35mTRACE[0m] [kernel] [CPU0]   Module[89]: name='/bin/test_futex' cmdline='' size=52992 bytes
[35337053136] [[35mTRACE[0m] [kernel] [CPU0]   Module[90]: name='/bin/ld_so' cmdline='' size=378600 bytes
[35337616182] [[35mTRACE[0m] [kernel] [CPU0]   Module[91]: name='/bin/test_dyn_loader' cmdline='' size=57760 bytes
[35338167744] [[35mTRACE[0m] [kernel] [CPU0]   Module[92]: name='/bin/test_dlopen' cmdline='' size=64688 bytes
[35338694226] [[35mTRACE[0m] [kernel] [CPU0]   Module[93]: name='/bin/reboot' cmdline='' size=32048 bytes
[35339393133] [[35mTRACE[0m] [kernel] [CPU0]   Module[94]: name='/bin/shutdown' cmdline='' size=32048 bytes
[35339924697] [[35mTRACE[0m] [kernel] [CPU0]   Module[95]: name='/bin/attr_list' cmdline='' size=47400 bytes
[35341456920] [[35mTRACE[0m] [kernel] [CPU0]   Module[96]: name='/bin/attr_get' cmdline='' size=47488 bytes
[35344530276] [[35mTRACE[0m] [kernel] [CPU0]   Module[97]: name='/bin/attr_set' cmdline='' size=69600 bytes
[35345061510] [[35mTRACE[0m] [kernel] [CPU0]   Module[98]: name='/bin/attr_rm' cmdline='' size=47008 bytes
[35345570931] [[35mTRACE[0m] [kernel] [CPU0]   Module[99]: name='/lib/libpistil.so' cmdline='' size=826992 bytes
[35346090912] [[35mTRACE[0m] [kernel] [CPU0]   Module[100]: name='/share/wallpapers/clouds.bmp' cmdline='' size=786486 bytes
[35346648744] [[35mTRACE[0m] [kernel] [CPU0]   Module[101]: name='/share/wallpapers/flower.bmp' cmdline='' size=8386614 bytes
[35347240368] [[35mTRACE[0m] [kernel] [CPU0]   Module[102]: name='/share/wallpapers/flower.png' cmdline='' size=2652468 bytes
[35347801929] [[35mTRACE[0m] [kernel] [CPU0]   Module[103]: name='/share/wallpapers/leather.bmp' cmdline='' size=1179702 bytes
[35348343591] [[35mTRACE[0m] [kernel] [CPU0]   Module[104]: name='/share/wallpapers/linen.bmp' cmdline='' size=4718646 bytes
[35348881821] [[35mTRACE[0m] [kernel] [CPU0]   Module[105]: name='/share/fonts/DSEG7Classic-Regular.ttf' cmdline='' size=23272 bytes
[35349446451] [[35mTRACE[0m] [kernel] [CPU0]   Module[106]: name='/share/fonts/Hack-Regular.ttf' cmdline='' size=309408 bytes
[35350065861] [[35mTRACE[0m] [kernel] [CPU0]   Module[107]: name='/share/fonts/Inter-Regular.ttf' cmdline='' size=876576 bytes
[35350772589] [[35mTRACE[0m] [kernel] [CPU0]   Module[108]: name='/share/fonts/Iosevka-Regular.ttf' cmdline='' size=10457376 bytes
[35351464863] [[35mTRACE[0m] [kernel] [CPU0]   Module[109]: name='/share/fonts/JetBrainsMono-Regular.ttf' cmdline='' size=270224 bytes
[35352062196] [[35mTRACE[0m] [kernel] [CPU0]   Module[110]: name='/share/fonts/NotoSans-Regular.ttf' cmdline='' size=569208 bytes
[35352613263] [[35mTRACE[0m] [kernel] [CPU0]   Module[111]: name='/share/fonts/NotoSansSymbol-Regular.ttf' cmdline='' size=258156 bytes
[35353162317] [[35mTRACE[0m] [kernel] [CPU0]   Module[112]: name='/share/fonts/NotoSansSymbol2-Regular.ttf' cmdline='' size=656852 bytes
[35353782222] [[35mTRACE[0m] [kernel] [CPU0]   Module[113]: name='/share/fonts/NotoSerif-Regular.ttf' cmdline='' size=616196 bytes
[35354342958] [[35mTRACE[0m] [kernel] [CPU0]   Module[114]: name='/share/themes/solarized_warm.toml' cmdline='' size=1832 bytes
[35354884818] [[35mTRACE[0m] [kernel] [CPU0]   Module[115]: name='/share/cursors/future/alias.svg' cmdline='' size=9033 bytes
[35355424830] [[35mTRACE[0m] [kernel] [CPU0]   Module[116]: name='/share/cursors/future/all-scroll.svg' cmdline='' size=5170 bytes
[35356010976] [[35mTRACE[0m] [kernel] [CPU0]   Module[117]: name='/share/cursors/future/bottom_left_corner.svg' cmdline='' size=1441 bytes
[35356758723] [[35mTRACE[0m] [kernel] [CPU0]   Module[118]: name='/share/cursors/future/bottom_right_corner.svg' cmdline='' size=3037 bytes
[35357733114] [[35mTRACE[0m] [kernel] [CPU0]   Module[119]: name='/share/cursors/future/bottom_side.svg' cmdline='' size=5432 bytes
[35358310251] [[35mTRACE[0m] [kernel] [CPU0]   Module[120]: name='/share/cursors/future/cell.svg' cmdline='' size=3707 bytes
[35358864354] [[35mTRACE[0m] [kernel] [CPU0]   Module[121]: name='/share/cursors/future/center_ptr.svg' cmdline='' size=9766 bytes
[35359424100] [[35mTRACE[0m] [kernel] [CPU0]   Module[122]: name='/share/cursors/future/col-resize.svg' cmdline='' size=16463 bytes
[35359969458] [[35mTRACE[0m] [kernel] [CPU0]   Module[123]: name='/share/cursors/future/color-picker.svg' cmdline='' size=6296 bytes
[35360541381] [[35mTRACE[0m] [kernel] [CPU0]   Module[124]: name='/share/cursors/future/context-menu.svg' cmdline='' size=9011 bytes
[35361153366] [[35mTRACE[0m] [kernel] [CPU0]   Module[125]: name='/share/cursors/future/copy.svg' cmdline='' size=3527 bytes
[35361817689] [[35mTRACE[0m] [kernel] [CPU0]   Module[126]: name='/share/cursors/future/crosshair.svg' cmdline='' size=16918 bytes
[35362559595] [[35mTRACE[0m] [kernel] [CPU0]   Module[127]: name='/share/cursors/future/default.svg' cmdline='' size=3079 bytes
[35363226360] [[35mTRACE[0m] [kernel] [CPU0]   Module[128]: name='/share/cursors/future/dnd-move.svg' cmdline='' size=3569 bytes
[35363870157] [[35mTRACE[0m] [kernel] [CPU0]   Module[129]: name='/share/cursors/future/dnd-no-drop.svg' cmdline='' size=4753 bytes
[35365373538] [[35mTRACE[0m] [kernel] [CPU0]   Module[130]: name='/share/cursors/future/down-arrow.svg' cmdline='' size=1820 bytes
[35366138973] [[35mTRACE[0m] [kernel] [CPU0]   Module[131]: name='/share/cursors/future/draft.svg' cmdline='' size=3136 bytes
[35366692878] [[35mTRACE[0m] [kernel] [CPU0]   Module[132]: name='/share/cursors/future/fleur.svg' cmdline='' size=28469 bytes
[35367267144] [[35mTRACE[0m] [kernel] [CPU0]   Module[133]: name='/share/cursors/future/help.svg' cmdline='' size=5873 bytes
[35367827418] [[35mTRACE[0m] [kernel] [CPU0]   Module[134]: name='/share/cursors/future/left-arrow.svg' cmdline='' size=1785 bytes
[35368373667] [[35mTRACE[0m] [kernel] [CPU0]   Module[135]: name='/share/cursors/future/left_side.svg' cmdline='' size=3848 bytes
[35369193684] [[35mTRACE[0m] [kernel] [CPU0]   Module[136]: name='/share/cursors/future/no-drop.svg' cmdline='' size=3861 bytes
[35369760096] [[35mTRACE[0m] [kernel] [CPU0]   Module[137]: name='/share/cursors/future/not-allowed.svg' cmdline='' size=1567 bytes
[35370336243] [[35mTRACE[0m] [kernel] [CPU0]   Module[138]: name='/share/cursors/future/openhand.svg' cmdline='' size=3435 bytes
[35370884010] [[35mTRACE[0m] [kernel] [CPU0]   Module[139]: name='/share/cursors/future/pencil.svg' cmdline='' size=6527 bytes
[35371578297] [[35mTRACE[0m] [kernel] [CPU0]   Module[140]: name='/share/cursors/future/pirate.svg' cmdline='' size=6663 bytes
[35372126526] [[35mTRACE[0m] [kernel] [CPU0]   Module[141]: name='/share/cursors/future/pointer.svg' cmdline='' size=2922 bytes
[35372886252] [[35mTRACE[0m] [kernel] [CPU0]   Module[142]: name='/share/cursors/future/progress-01.svg' cmdline='' size=11030 bytes
[35373834144] [[35mTRACE[0m] [kernel] [CPU0]   Module[143]: name='/share/cursors/future/progress-02.svg' cmdline='' size=12198 bytes
[35374636572] [[35mTRACE[0m] [kernel] [CPU0]   Module[144]: name='/share/cursors/future/progress-03.svg' cmdline='' size=12733 bytes
[35375198694] [[35mTRACE[0m] [kernel] [CPU0]   Module[145]: name='/share/cursors/future/progress-04.svg' cmdline='' size=13777 bytes
[35375768043] [[35mTRACE[0m] [kernel] [CPU0]   Module[146]: name='/share/cursors/future/progress-05.svg' cmdline='' size=13835 bytes
[35376360195] [[35mTRACE[0m] [kernel] [CPU0]   Module[147]: name='/share/cursors/future/progress-06.svg' cmdline='' size=14907 bytes
[35376968418] [[35mTRACE[0m] [kernel] [CPU0]   Module[148]: name='/share/cursors/future/progress-07.svg' cmdline='' size=14805 bytes
[35377648911] [[35mTRACE[0m] [kernel] [CPU0]   Module[149]: name='/share/cursors/future/progress-08.svg' cmdline='' size=15996 bytes
[35378251491] [[35mTRACE[0m] [kernel] [CPU0]   Module[150]: name='/share/cursors/future/progress-09.svg' cmdline='' size=16010 bytes
[35378850309] [[35mTRACE[0m] [kernel] [CPU0]   Module[151]: name='/share/cursors/future/progress-10.svg' cmdline='' size=17080 bytes
[35379501201] [[35mTRACE[0m] [kernel] [CPU0]   Module[152]: name='/share/cursors/future/progress-11.svg' cmdline='' size=17096 bytes
[35380087842] [[35mTRACE[0m] [kernel] [CPU0]   Module[153]: name='/share/cursors/future/progress-12.svg' cmdline='' size=17079 bytes
[35380676958] [[35mTRACE[0m] [kernel] [CPU0]   Module[154]: name='/share/cursors/future/progress-13.svg' cmdline='' size=15900 bytes
[35381234031] [[35mTRACE[0m] [kernel] [CPU0]   Module[155]: name='/share/cursors/future/progress-14.svg' cmdline='' size=16001 bytes
[35381779422] [[35mTRACE[0m] [kernel] [CPU0]   Module[156]: name='/share/cursors/future/progress-15.svg' cmdline='' size=14931 bytes
[35382322074] [[35mTRACE[0m] [kernel] [CPU0]   Module[157]: name='/share/cursors/future/progress-16.svg' cmdline='' size=14873 bytes
[35382870072] [[35mTRACE[0m] [kernel] [CPU0]   Module[158]: name='/share/cursors/future/progress-17.svg' cmdline='' size=13846 bytes
[35383477107] [[35mTRACE[0m] [kernel] [CPU0]   Module[159]: name='/share/cursors/future/progress-18.svg' cmdline='' size=13835 bytes
[35384060184] [[35mTRACE[0m] [kernel] [CPU0]   Module[160]: name='/share/cursors/future/progress-19.svg' cmdline='' size=12645 bytes
[35384639400] [[35mTRACE[0m] [kernel] [CPU0]   Module[161]: name='/share/cursors/future/progress-20.svg' cmdline='' size=12741 bytes
[35385228186] [[35mTRACE[0m] [kernel] [CPU0]   Module[162]: name='/share/cursors/future/progress-21.svg' cmdline='' size=11660 bytes
[35385800637] [[35mTRACE[0m] [kernel] [CPU0]   Module[163]: name='/share/cursors/future/progress-22.svg' cmdline='' size=11612 bytes
[35386372428] [[35mTRACE[0m] [kernel] [CPU0]   Module[164]: name='/share/cursors/future/progress-23.svg' cmdline='' size=11674 bytes
[35386967781] [[35mTRACE[0m] [kernel] [CPU0]   Module[165]: name='/share/cursors/future/progress.svg' cmdline='' size=11642 bytes
[35387531619] [[35mTRACE[0m] [kernel] [CPU0]   Module[166]: name='/share/cursors/future/right-arrow.svg' cmdline='' size=3432 bytes
[35388161655] [[35mTRACE[0m] [kernel] [CPU0]   Module[167]: name='/share/cursors/future/right_ptr.svg' cmdline='' size=5155 bytes
[35388872871] [[35mTRACE[0m] [kernel] [CPU0]   Module[168]: name='/share/cursors/future/right_side.svg' cmdline='' size=6447 bytes
[35389479345] [[35mTRACE[0m] [kernel] [CPU0]   Module[169]: name='/share/cursors/future/row-resize.svg' cmdline='' size=15825 bytes
[35390473173] [[35mTRACE[0m] [kernel] [CPU0]   Module[170]: name='/share/cursors/future/size_bdiag.svg' cmdline='' size=16323 bytes
[35391827064] [[35mTRACE[0m] [kernel] [CPU0]   Module[171]: name='/share/cursors/future/size_fdiag.svg' cmdline='' size=16589 bytes
[35392435419] [[35mTRACE[0m] [kernel] [CPU0]   Module[172]: name='/share/cursors/future/size_hor.svg' cmdline='' size=16255 bytes
[35392995594] [[35mTRACE[0m] [kernel] [CPU0]   Module[173]: name='/share/cursors/future/size_ver.svg' cmdline='' size=15822 bytes
[35393593587] [[35mTRACE[0m] [kernel] [CPU0]   Module[174]: name='/share/cursors/future/text.svg' cmdline='' size=5584 bytes
[35394133896] [[35mTRACE[0m] [kernel] [CPU0]   Module[175]: name='/share/cursors/future/top_left_corner.svg' cmdline='' size=7121 bytes
[35394739809] [[35mTRACE[0m] [kernel] [CPU0]   Module[176]: name='/share/cursors/future/top_right_corner.svg' cmdline='' size=7139 bytes
[35395451586] [[35mTRACE[0m] [kernel] [CPU0]   Module[177]: name='/share/cursors/future/top_side.svg' cmdline='' size=10651 bytes
[35396595135] [[35mTRACE[0m] [kernel] [CPU0]   Module[178]: name='/share/cursors/future/up-arrow.svg' cmdline='' size=7653 bytes
[35397192996] [[35mTRACE[0m] [kernel] [CPU0]   Module[179]: name='/share/cursors/future/vertical-text.svg' cmdline='' size=5621 bytes
[35397766965] [[35mTRACE[0m] [kernel] [CPU0]   Module[180]: name='/share/cursors/future/wait-01.svg' cmdline='' size=15308 bytes
[35398314270] [[35mTRACE[0m] [kernel] [CPU0]   Module[181]: name='/share/cursors/future/wait-02.svg' cmdline='' size=6672 bytes
[35398898766] [[35mTRACE[0m] [kernel] [CPU0]   Module[182]: name='/share/cursors/future/wait-03.svg' cmdline='' size=6686 bytes
[35399505372] [[35mTRACE[0m] [kernel] [CPU0]   Module[183]: name='/share/cursors/future/wait-04.svg' cmdline='' size=7827 bytes
[35400081618] [[35mTRACE[0m] [kernel] [CPU0]   Module[184]: name='/share/cursors/future/wait-05.svg' cmdline='' size=7835 bytes
[35400628494] [[35mTRACE[0m] [kernel] [CPU0]   Module[185]: name='/share/cursors/future/wait-06.svg' cmdline='' size=8976 bytes
[35401174908] [[35mTRACE[0m] [kernel] [CPU0]   Module[186]: name='/share/cursors/future/wait-07.svg' cmdline='' size=8989 bytes
[35401745841] [[35mTRACE[0m] [kernel] [CPU0]   Module[187]: name='/share/cursors/future/wait-08.svg' cmdline='' size=10126 bytes
[35402366076] [[35mTRACE[0m] [kernel] [CPU0]   Module[188]: name='/share/cursors/future/wait-09.svg' cmdline='' size=10140 bytes
[35402926020] [[35mTRACE[0m] [kernel] [CPU0]   Module[189]: name='/share/cursors/future/wait-10.svg' cmdline='' size=11277 bytes
[35403539391] [[35mTRACE[0m] [kernel] [CPU0]   Module[190]: name='/share/cursors/future/wait-11.svg' cmdline='' size=11291 bytes
[35404109334] [[35mTRACE[0m] [kernel] [CPU0]   Module[191]: name='/share/cursors/future/wait-12.svg' cmdline='' size=11277 bytes
[35404715973] [[35mTRACE[0m] [kernel] [CPU0]   Module[192]: name='/share/cursors/future/wait-13.svg' cmdline='' size=10140 bytes
[35405278260] [[35mTRACE[0m] [kernel] [CPU0]   Module[193]: name='/share/cursors/future/wait-14.svg' cmdline='' size=10126 bytes
[35406158271] [[35mTRACE[0m] [kernel] [CPU0]   Module[194]: name='/share/cursors/future/wait-15.svg' cmdline='' size=8991 bytes
[35407652577] [[35mTRACE[0m] [kernel] [CPU0]   Module[195]: name='/share/cursors/future/wait-16.svg' cmdline='' size=8978 bytes
[35416886439] [[35mTRACE[0m] [kernel] [CPU0]   Module[196]: name='/share/cursors/future/wait-17.svg' cmdline='' size=7842 bytes
[35420843040] [[35mTRACE[0m] [kernel] [CPU0]   Module[197]: name='/share/cursors/future/wait-18.svg' cmdline='' size=7829 bytes
[35421556665] [[35mTRACE[0m] [kernel] [CPU0]   Module[198]: name='/share/cursors/future/wait-19.svg' cmdline='' size=6822 bytes
[35422136904] [[35mTRACE[0m] [kernel] [CPU0]   Module[199]: name='/share/cursors/future/wait-20.svg' cmdline='' size=6679 bytes
[35422682493] [[35mTRACE[0m] [kernel] [CPU0]   Module[200]: name='/share/cursors/future/wait-21.svg' cmdline='' size=5542 bytes
[35423514522] [[35mTRACE[0m] [kernel] [CPU0]   Module[201]: name='/share/cursors/future/wait-22.svg' cmdline='' size=5528 bytes
[35424128883] [[35mTRACE[0m] [kernel] [CPU0]   Module[202]: name='/share/cursors/future/wait-23.svg' cmdline='' size=5546 bytes
[35428247976] [[35mTRACE[0m] [kernel] [CPU0]   Module[203]: name='/share/cursors/future/wait.svg' cmdline='' size=5525 bytes
[35429163000] [[35mTRACE[0m] [kernel] [CPU0]   Module[204]: name='/share/cursors/future/wayland-cursor.svg' cmdline='' size=8222 bytes
[35431664994] [[35mTRACE[0m] [kernel] [CPU0]   Module[205]: name='/share/cursors/future/x-cursor.svg' cmdline='' size=6822 bytes
[35435193189] [[35mTRACE[0m] [kernel] [CPU0]   Module[206]: name='/share/cursors/future/zoom-in.svg' cmdline='' size=5441 bytes
[35435839296] [[35mTRACE[0m] [kernel] [CPU0]   Module[207]: name='/share/cursors/future/zoom-out.svg' cmdline='' size=5403 bytes
[35436571929] [[35mTRACE[0m] [kernel] [CPU0]   Module[208]: name='/share/fonts/unifont.hex' cmdline='' size=8355852 bytes
[35437249914] [[35mTRACE[0m] [kernel] [CPU0]   Module[209]: name='/etc/locale.conf' cmdline='' size=85 bytes
[35437796592] [[35mTRACE[0m] [kernel] [CPU0]   Module[210]: name='/etc/profile' cmdline='' size=68 bytes
[35438545296] [[35mTRACE[0m] [kernel] [CPU0]   Module[211]: name='/etc/motd' cmdline='' size=610 bytes
[35439061680] [[35mTRACE[0m] [kernel] [CPU0]   Module[212]: name='/etc/fstab' cmdline='' size=127 bytes
[35439709833] [[35mTRACE[0m] [kernel] [CPU0]   Module[213]: name='/etc/hostname' cmdline='' size=8 bytes
[35441905884] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[35674506318] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[35675773947] [[34mDEBUG[0m] [kernel] [CPU0] Found init module: /bin/sprout (cmdline: 'init'), loading...
[35677382763] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/sprout' (len=89912, base=0x200000)
[35678319138] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[35688037803] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[35690061561] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[35712974352] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [49, 3b, 6e, 18, 75, 13, 49, 8d]
[35718935208] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20c000 exec=false
[35721158055] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20f000 exec=false
[35743707945] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x210000 filesz=0 memsz=0 align=1
[35751103674] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='load_init_module' elapsed_ticks=73962669 elapsed_us=36981 total_ticks=1098581682 total_us=549290
[35752888479] [[34mDEBUG[0m] [kernel] [CPU0] Warning: Module registry page overflow, truncating list.
[35753712753] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='prepare_boot_registry' elapsed_ticks=1830609 elapsed_us=915 total_ticks=1101207657 total_us=550603
[35756862438] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='map_boot_registry' elapsed_ticks=866745 elapsed_us=433 total_ticks=1104186270 total_us=552093
[35763495834] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='activate_init_aspace' elapsed_ticks=1828398 elapsed_us=914 total_ticks=1110171315 total_us=555085
[35768843154] [[34mDEBUG[0m] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[35769448638] [[34mDEBUG[0m] [kernel] [CPU0] Spawning init process...
[35772152064] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb006a9a0 arg=0xffffffffb0020a20
[35787337146] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[35793341133] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=5
[35801783094] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[35802915126] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='spawn_init_task' elapsed_ticks=33495297 elapsed_us=16747 total_ticks=1150385247 total_us=575192
[35803705245] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[35963305983] [[34mDEBUG[0m] [kernel::sched::lifecycle] [CPU0] SCHED: early-boot bringup complete; resuming steady-state SMP scheduling
[35964373071] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='end_bringup' elapsed_ticks=1661352 elapsed_us=830 total_ticks=1311848670 total_us=655924
[35965190349] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[36240046788] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1587309273 elapsed_us=793654
[36240911124] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[36260909883] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[36262581630] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] ENTER_USER: TID=5 entry_pc=200000 user_sp=800000
[36344700249] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpus_online=4
[36355558998] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=0 curr=Some(5) runq=1 runq_avg=2 runq_samples=1 ctxsw=1 idle2busy=0 tick=1 ipi=0 enq=3 deq=1 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[36356951895] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=1 curr=Some(2) runq=0 runq_avg=0 runq_samples=42 ctxsw=0 idle2busy=0 tick=38 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[36357824679] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=2 curr=Some(3) runq=0 runq_avg=0 runq_samples=42 ctxsw=0 idle2busy=0 tick=36 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[36358651032] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=3 curr=Some(4) runq=0 runq_avg=0 runq_samples=40 ctxsw=0 idle2busy=0 tick=34 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[36372908352] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=0
[36378114135] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=20
[36389797224] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[36391984662] [[34mDEBUG[0m] [sprout] [CPU0] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7fef70 rip=0x2032e5 rflags=0x202
[36395061978] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[36397214997] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: About to create Supervisor...
[36399082632] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Listing /bin directory...
[36401669040] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/bin' tid=5
[36437748039] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=3
[36448110435] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[36483437595] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Parsed command line: '    '
[36486066441] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Supervisor created, calling run_forever...
[36488464023] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[36499786818] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[36501643563] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[36503234493] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/run/sprout/shell' tid=5
[36507593562] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/etc/default/shell' tid=5
[36510038202] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/bin/sh' tid=5
[36515391825] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=3
[36518283483] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[36523945425] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[36528882951] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[36562810878] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN: page cache miss for '/bin/sh', caching 127864 bytes
[36569291946] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'sh' (len=127864, base=0x200000)
[36569940693] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[36571033257] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[36571645869] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[36573800868] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [0f, 84, 32, 06, 00, 00, 48, 8b]
[36591401946] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=214000 exec=false
[36593458506] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=217000 exec=false
[36595465401] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x218000 filesz=0 memsz=0 align=1
[36601142490] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /bin/sh
[36602829681] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb00a3d40 arg=0xffffffffb0074560
[36606374079] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=6, applying inserts
[36607049259] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[36607551321] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=6
[36608146113] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[36608740113] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[36627235920] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SETUP_STDIO: stdin=Fd(3) stdout=Fd(4) stderr=Fd(5)
[36643155186] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 6
[36655384326] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Task 6 woken, restoring IRQs
[36656773659] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Done for /bin/sh
[36659160021] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[36665268651] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[36666200274] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=6 entry_pc=200000 user_sp=800000
[36668886969] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[36668920497] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=0
[36671335074] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=3
[36671507598] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=100
[36673651542] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=4
[36674574420] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[36674932371] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=5
[36677269992] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/run/motd' tid=6
[36679159539] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[36683636682] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/etc/motd' tid=6
[36687892329] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=6
[36690890181] [[34mDEBUG[0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=20559
[36696142164] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=610
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
[36701709363] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/etc/profile' tid=6
[36731977095] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=6
[36742671702] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=97
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [36751676577] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=6
[?25h[36753718419] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[36838424172] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[36840241086] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[36844547256] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/cambium' (len=144768, base=0x200000)
[36845462181] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[36846764427] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[36847365489] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[36849745284] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [8d, 3c, f0, 48, 83, c7, 70, 48]
[36866304453] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=217000 exec=false
[36868076124] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[36869435295] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x21b000 filesz=0 memsz=0 align=1
[36875227026] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb00bd7a0 arg=0xffffffffb00ad2c0
[36877641471] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[36880945530] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=7
[36884041095] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[36891324921] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 7 �� task '/bin/cambium' (pid=7 from boot module)
[36896076492] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[36897046890] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned cambium (PID=7)
[36897122196] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=7 entry_pc=200000 user_sp=800000
[36899720616] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=0
[36901106220] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=100
[36901278282] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[36903818886] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Deferring netd until /dev/net/virtio0/rx is ready...
[36904344576] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[36905966328] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[36907626690] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[36909270618] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[36910849536] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[36912165213] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[36925369107] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN: page cache miss for '/bin/bristle', caching 68744 bytes
[36929799654] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'bristle' (len=68744, base=0x200000)
[36930985014] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[36932445561] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[36933103977] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[36935375037] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [cc, fc, ff, ff, 45, 31, ff, e9]
[36935778858] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[36937133838] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers' tid=7
[36939472977] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=209000 exec=false
[36940358136] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_readdir: fd=3 len=4096
[36940952928] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20b000 exec=false
[36942215409] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20c000 filesz=0 memsz=0 align=1
[36942327180] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_readdir: natural phase begin offset=0
[36957869982] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /bin/bristle
[36958765833] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb00e9080 arg=0xffffffffb00d7a80
[36960338910] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=8, applying inserts
[36961045044] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[36961687521] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=8
[36962423289] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[36963037122] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[36964967886] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SETUP_STDIO: stdin=Null stdout=Inherit stderr=Inherit
[36969785523] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 8
[36971388597] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Task 8 woken, restoring IRQs
[36972109515] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Done for /bin/bristle
[36972901152] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/bristle' TID=8 PID=8
[36975729615] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[36976614972] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=8 entry_pc=200000 user_sp=800000
[36978280746] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=0
[36978302229] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[36979807953] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=100
[36985069176] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ahci_disk' tid=7
[37000569144] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/session/active_ui' tid=8
[37010017209] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=3
[37015673508] [[35mTRACE[0m] [bristle] [CPU2] bristle: active_ui set to 'bloom'
[37020398184] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/run/bristle/pid' tid=8
[37040924877] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[37044586326] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[37046391360] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[37048252032] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[37050453825] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/fb0' tid=5
[37051574934] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_open: path='/dev/fb0' len=8 flags=0x0
[37053380793] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] devfs: lookup entry path='fb0' len=3
[37054581036] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] devfs: dynamic registry hit path='fb0'
[37056628818] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_open: resolved node for /dev/fb0 mode=0o20666 size=32 ino=4
[37057506750] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_open: process info present for /dev/fb0
[37058351781] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_open: handle_table.open('/dev/fb0') -> 3
[37064166645] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=3
[37066125228] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[37071048828] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=4096
[37072633719] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(0)
[37074686814] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] FbNode::read: off=0 n=32 buf_len=32 total=32
[37076686548] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=3
[37077036051] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=1 port_id=PortId(0) mode=Write
[37078575171] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=2 port_id=PortId(0) mode=Read
[37080451122] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=4096
[37081923285] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(1)
[37082769108] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(1) mode=Write
[37083433299] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=4 port_id=PortId(1) mode=Read
[37085661657] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[37087561203] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/run/bristle/kbd_in' tid=8
[37091342211] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/drivers/display_virtio_gpu' tid=5
[37092594561] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=3
[37094374185] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/run/bristle/mouse_in' tid=8
[37098047679] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=3
[37099320390] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[37100070414] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/proc/self/inbox' tid=8
[37102913529] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=3
[37105882143] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[37108067766] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[37108130994] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=2 -> fd=4 node=0xffffffffb00c6b50 port=0xffffffffb00d8430
[37111250286] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=4 -> fd=5 node=0xffffffffb00d7f30 port=0xffffffffb00d85b0
[37112913783] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[37115064657] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=4096
[37115601270] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(2)
[37116320142] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=1 port_id=PortId(2) mode=Write
[37117016772] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=2 port_id=PortId(2) mode=Read
[37124293371] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[37126126125] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=4096
[37126649274] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(3)
[37127250006] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=3 port_id=PortId(3) mode=Write
[37127932347] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=4 port_id=PortId(3) mode=Read
[37130958645] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[37136191554] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[37141450269] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[37170490500] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN: page cache miss for '/drivers/display_virtio_gpu', caching 147560 bytes
[37172168748] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'display_virtio_gpu' (len=147560, base=0x200000)
[37172791755] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[37174002393] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[37174610550] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[37185260640] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [00, 00, 00, ba, 12, 00, 00, 00]
[37196022963] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=216000 exec=false
[37198534791] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[37200755031] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x21c000 filesz=0 memsz=0 align=1
[37205224848] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/display_virtio_gpu
[37205895144] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb022da20 arg=0xffffffffb01f97c0
[37208455548] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=9, applying inserts
[37209268371] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[37209934080] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=9
[37210583355] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[37211124588] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[37213556325] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SETUP_STDIO: stdin=Null stdout=Inherit stderr=Inherit
[37219565427] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 9
[37220802663] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Task 9 woken, restoring IRQs
[37221452796] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Done for /drivers/display_virtio_gpu
[37222064154] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/drivers/display_virtio_gpu' TID=9 PID=9
[37223282382] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x3
[37224033561] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=9 entry_pc=200000 user_sp=800000
[37225370589] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=0
[37226871099] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=100
[37231041144] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[37232584422] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Starting VFS-native VirtIO GPU driver...
[37233701868] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: boot_arg=3
[37234797996] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Mapping bootstrap memfd 3 size=4096...
[37237962861] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: vm_map success at 0x400000001000
[37242099972] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Recovered handles: req_read=2, resp_write=3, svc=0, id=322371585
[37244373870] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[37244394957] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3, svc=0, id=322371585)
[37246273812] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Scanning /sys/devices for PCI GPU...
[37247686641] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices' tid=9
[37248464583] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[37250673603] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices'
[37252411350] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0 mounted=false, service_ready=false)...
[37255306869] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_readdir: fd=4 len=4096
[37256017623] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready for bloom yet
[37256401182] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_readdir: natural phase begin offset=0
[37260457905] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU3] sysfs: readdir found 12 slots
[37261210470] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[37263421800] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[37265308377] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[37265624253] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU3] sysfs: readdir wrote 173 bytes
[37267354872] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37268838288] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/proc/self/inbox' tid=5
[37274054400] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[37295676099] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[37303494228] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/platform-fb000000/vendor' tid=9
[37304105982] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[37305890391] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ata_disk' tid=7
[37305882900] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/platform-fb000000/vendor'
[37313827122] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'platform-fb000000/vendor'
[37318544307] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37321092072] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/platform-fb000000/device' tid=9
[37322574894] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/platform-fb000000/device'
[37323909711] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'platform-fb000000/device'
[37326312837] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37327885947] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/platform-fb000000/class' tid=9
[37329269406] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/platform-fb000000/class'
[37330577493] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'platform-fb000000/class'
[37333384704] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37335119349] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/vendor' tid=9
[37336946394] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:00.0/vendor'
[37338918045] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:00.0/vendor'
[37343289258] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37345189167] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/device' tid=9
[37348473294] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:00.0/device'
[37349575659] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:00.0/device'
[37352378514] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37354593870] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/class' tid=9
[37357408077] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:00.0/class'
[37359170541] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:00.0/class'
[37362194727] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37363897527] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/vendor' tid=9
[37365960654] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/vendor'
[37368100341] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:01.0/vendor'
[37370307843] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37372803765] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/device' tid=9
[37374436770] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/device'
[37375583322] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:01.0/device'
[37377639717] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37378889922] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/class' tid=9
[37380622785] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/class'
[37381897839] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:01.0/class'
[37383761646] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37389763950] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: task 9 claimed device 'pci-0000:00:01.0' (handle 0)
[37391163612] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/virtio/common_bar' tid=9
[37392597330] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/common_bar'
[37400983884] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37403167626] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/virtio/common_offset' tid=9
[37404787266] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/common_offset'
[37409768451] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37412051886] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/virtio/notify_bar' tid=9
[37413769371] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/notify_bar'
[37417822101] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37419173583] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/virtio/notify_offset' tid=9
[37420607400] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/notify_offset'
[37426495854] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37428195387] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/virtio/notify_multiplier' tid=9
[37430337648] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/notify_multiplier'
[37435915737] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37440053904] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[37444319022] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: mapping BAR2 (phys=0xc000000000, size=0x4000) for task 9
[37447640472] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10000000
[37451370330] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: DMA alloc 1 pages phys=0x2329000 -> user_va=0x10004000
[37456135200] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[37460717250] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: DMA alloc 4 pages phys=0x232a000 -> user_va=0x10005000
[37463735067] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[37464741699] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Virgl 3D not supported, using 2D only
[37465705266] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/firmware/framebuffer' tid=9
[37467436413] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='firmware/framebuffer'
[37473179040] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[37473617082] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[37476381129] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[37477989318] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/chime' tid=7
[37478814351] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=1
[37600307349] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[37602953850] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[37604692059] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[37606176663] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[37608266751] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0 mounted=false, service_ready=false)...
[37611265329] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready for bloom yet
[37613254635] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[37615551204] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[37618852326] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[37621006599] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[37642954470] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[37645211604] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[37646315322] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/display_bootfb' tid=7
[37819022505] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[37822139784] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[37823209413] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/display_virtio_gpu' tid=7
[37926467931] [[34mDEBUG[0m] [kernel::syscall::handlers::memory] [CPU3] sys_vm_map: large backing file mapping offset=0 len=8294400 total_size=8294400
[38011201239] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[38015486124] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[38017436094] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/hdaudio' tid=7
[38032505280] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: frame pool ready (1 buffer)
[38035357767] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[38036756538] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[38038368621] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[38039571438] [[34mDEBUG[0m] [kernel::ipc] [CPU3] CREATE_PORT: capacity=65536
[38040248070] [[34mDEBUG[0m] [kernel::ipc] [CPU3] CREATE_PORT: appended id=PortId(4)
[38040766170] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[38041057890] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU3] ALLOC_HANDLE: handle=5 port_id=PortId(4) mode=Write
[38041770360] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU3] ALLOC_HANDLE: handle=6 port_id=PortId(4) mode=Read
[38043078645] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU3] BRIDGE: handle=5 -> fd=5 node=0xffffffffb00ad2d0 port=0xffffffffb01120f0
[38045181933] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0 mounted=false, service_ready=false)...
[38046129330] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/proc/5/inbox' tid=9
[38049552882] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready for bloom yet
[38065493103] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[38067713046] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[38069484486] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[38070975624] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[38079341586] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU3] SENDMSG: thing=6 data_len=44 caps_len=1 node=Pointer { addr: 0xffffffffb0112390, metadata: DynMetadata(0xffffffff802c6ec0) }
[38088154830] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_BIND_READY (ID: 322371585)
[38089251123] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Waiting for BIND_ASSIGNED...
[38095133274] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU0] RECVMSG: thing=4 data_len=44 caps_len=1 node=Pointer { addr: 0xffffffffb0101250, metadata: DynMetadata(0xffffffff802c6ec0) }
[38103444885] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: BIND_READY from instance_id=0x13370001
[38106072378] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Registering DISPLAY_CARD at /dev/display/card0 (handle=5)
[38115346665] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=262400
[38115962412] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(5)
[38117934987] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=7 port_id=PortId(5) mode=Write
[38124030747] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] vfs: mounted userland provider at /dev/display/card0 (flags: 0x0) port=0xffffffffb01120f0
[38129083179] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[38131748457] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Handshake complete — task 'display' marked ready
[38139001560] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Sent MSG_BIND_ASSIGNED to display pid 9 (res=Ok(()))
[38140632519] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[38140725546] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[38142115209] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU3] SENDMSG: thing=6 data_len=44 caps_len=0 node=Pointer { addr: 0xffffffffb0112390, metadata: DynMetadata(0xffffffff802c6ec0) }
[38143661358] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_SERVICE_READY.
[38144573247] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU3] BRIDGE: handle=2 -> fd=7 node=0xffffffffb0112310 port=0xffffffffb00d8990
[38145521106] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU3] BRIDGE: handle=3 -> fd=8 node=0xffffffffb01233d0 port=0xffffffffb00d8db0
[38146421676] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU3] BRIDGE: handle=6 -> fd=9 node=0xffffffffb0123490 port=0xffffffffb01120f0
[38148622380] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[38156483310] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU0] RECVMSG: thing=4 data_len=44 caps_len=0 node=Pointer { addr: 0xffffffffb0101250, metadata: DynMetadata(0xffffffff802c6ec0) }
[38160051336] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: SERVICE_READY from instance_id=0x13370001
[38162330481] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Service 'display' reported ready
[38164318071] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[38166389514] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[38200959225] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[38203389147] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[38207547939] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/pci_stubd' tid=7
[38366884083] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[38369332485] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[38370459039] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ps2_kbd' tid=7
[38485077840] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[38487467832] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[38488904190] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[38490561780] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[38492607087] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0 mounted=true, service_ready=true)...
[38495261442] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/bloom' (len=462464, base=0x200000)
[38495867124] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[38497775976] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[38498363772] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[38500497189] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [29, 44, 24, 60, 48, 8b, 44, 24]
[38534189925] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[38536645554] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[38537763198] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ps2_mouse' tid=7
[38546011218] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=253000 exec=false
[38551035171] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=25a000 exec=false
[38553577920] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x25d000 filesz=0 memsz=0 align=1
[38558233362] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb0143640 arg=0xffffffffb00c6c00
[38560058361] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[38560766178] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=10
[38561650908] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[38575494375] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 10 → task '/bin/bloom' (pid=10 from boot module)
[38583656760] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[38587684707] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[38589817959] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[38591606031] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[38593113372] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[38633825901] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[38635536555] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=10 entry_pc=200000 user_sp=800000
[38639731317] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=0
[38641305747] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=100
[38645197272] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[38646236211] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[38648219709] [[34mDEBUG[0m] [bloom] [CPU1] bloom: connect try 0...
[38649027945] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/dev/display/card0' tid=10
[38651590362] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=Lookup len=4 tid=10
[38685369162] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=Lookup
[38687405757] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: vfs_lookup path=''
[38689540659] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=Lookup req_id=1
[38698288596] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=Lookup req_id=1
[38776406922] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=Lookup id=1 -> OK(8)
[38795458911] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=48 tid=10
[38804786229] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[38807191731] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[38808882882] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/rtc_cmos' tid=7
[38817481692] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[38818896633] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_GET_INFO requested
[38819837430] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: Returning dimensions 1920x1080
[38820860166] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=2
[38823321735] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=2
[38910436125] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[38912170836] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[38913586800] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[38915024610] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[38916606861] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[38918466939] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[38919998106] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[38921380245] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[38977183476] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=2 -> OK(48)
[38982963855] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[39002967861] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1920x1080 @ 60000mHz
[39004056069] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[39004980894] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[39005801142] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[39010551789] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=65536
[39011224230] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(6)
[39011971251] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=5 port_id=PortId(6) mode=Write
[39012646233] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=6 port_id=PortId(6) mode=Read
[39015490569] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/services/bloom' tid=10
[39020322264] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[39031284831] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/components' tid=10
[39034844475] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[39035924466] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/status' tid=10
[39039150810] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[39043251093] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[39045214626] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[39046182945] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/rtl8168d' tid=7
[39203439429] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/lib/libpistil.so' tid=10
[39239576838] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[39241534761] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[39243211326] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[39245413152] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[39247547130] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[39249368301] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[39251046813] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[39252442251] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[39450366747] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[39509863239] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[39512610126] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[39513592008] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/virtio_netd' tid=7
[39570817407] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[39572575548] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[39574054014] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[39575787174] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[39577347183] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[39579130140] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[39581182509] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[39582725523] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[39679138323] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[39682558212] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[39684015426] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/virtio_sound' tid=7
[39848189964] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[39851143035] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[39853634007] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[39854915826] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/device_snapshot' tid=7
[39857057262] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='device_snapshot'
[39863358810] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[39869538423] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:00.0 kind=pci_device vendor=0x8086 device=0x29c0 class=0x060000 present=true
[39872065365] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:01.0 kind=pci_device vendor=0x1af4 device=0x1050 class=0x030000 present=true
[39874437075] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:02.0 kind=pci_device vendor=0x1af4 device=0x1000 class=0x020000 present=true
[39876136377] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: isolation mode skipping driver '/drivers/virtio_netd' class=Net for pci-0000:00:02.0
[39877461525] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:03.0 kind=pci_device vendor=0x1af4 device=0x1059 class=0x040100 present=true
[39879209271] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: isolation mode skipping driver '/drivers/virtio_sound' class=Audio for pci-0000:00:03.0
[39880558542] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:1f.0 kind=pci_device vendor=0x8086 device=0x2918 class=0x060100 present=true
[39881832672] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:1f.2 kind=pci_device vendor=0x8086 device=0x2922 class=0x010601 present=true
[39884175012] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.2/status' tid=7
[39885366378] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/status'
[39886658361] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/status'
[39888911040] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[39901759392] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[39903470442] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[39903908451] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: page cache miss for '/drivers/ahci_disk', caching 53784 bytes
[39905838984] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[39905837994] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'ahci_disk' (len=53784, base=0x200000)
[39906733482] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[39907374012] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[39907832283] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[39908348403] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[39908462022] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=200000 exec=true
[39910947087] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   COPIED at 0x201420: [24, 0c, e8, 59, 0e, 00, 00, bf]
[39914077731] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=207000 exec=false
[39914648367] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[39915148614] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=208000 exec=false
[39916716378] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x209000 filesz=0 memsz=0 align=1
[39917539464] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[39922935096] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[39927878826] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[39928764678] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/ahci_disk
[39929432631] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb0170860 arg=0xffffffffb012ebe0
[39945126969] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=11, applying inserts
[39945819672] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[39946330512] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=11
[39947019321] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[39947532240] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[39950502240] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[39955292058] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 11
[39956760525] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 11 woken, restoring IRQs
[39957306807] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/ahci_disk
[39958016604] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=11 PID=11
[39959308521] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x3
[39960363993] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/ahci_disk for pci-0000:00:1f.2 (entry='thingos_driver_start_safe', pid=11)
[39960526254] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=11 entry_pc=201000 user_sp=800000
[39968497767] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=11 buf_len=0
[39970161396] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=11 buf_len=100
[39974526174] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Starting AHCI VFS driver
[39975473538] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[39976600422] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[39977339127] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[39978104892] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil nine-slice shadow renderer loaded
[39979234152] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/desktop/theme' tid=10
[39981977574] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/desktop/theme' tid=10
[39985798545] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[39988196391] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Solarized Warm
[40014253884] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 'pci-0000:00:1f.2' (handle 1)
[40017462441] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[40019090364] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: mapping BAR5 (phys=0x80880000, size=0x1000) for task 11
[40020289782] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10009000
[40024424550] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: DMA alloc 1 pages phys=0x359c000 -> user_va=0x1000a000
[40029728013] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=65536
[40030389003] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(7)
[40031268684] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=1 port_id=PortId(7) mode=Write
[40031914131] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=2 port_id=PortId(7) mode=Read
[40038078267] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=262400
[40038998208] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(8)
[40039882278] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(8) mode=Write
[40043519736] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] vfs: mounted userland provider at /dev/storage/atapi2 (flags: 0x0) port=0xffffffffb01787f0
[40044906825] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Mounted atapi2 at /dev/storage/atapi2
[40045636950] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Provider loop online at /dev/storage/atapi2
[40233297159] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[40236071964] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[40238409552] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[40240814724] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[40242807957] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[40244419809] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[40246252992] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[40248350736] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[40401576237] [[34mDEBUG[0m] [kernel::syscall::handlers::memory] [CPU1] sys_vm_map: large backing file mapping offset=0 len=8294400 total_size=8294400
[40471134693] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: sent DRIVER_READY to Sprout (pid=5) for driver pid=11
[40472369916] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:1f.3 kind=pci_device vendor=0x8086 device=0x2930 class=0x0c0500 present=true
[40473293784] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=isa-0070 kind=dev.rtc.Cmos vendor=0x0000 device=0x0000 class=0x000000 present=true
[40475768322] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0070/status' tid=7
[40476177324] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU0] RECVMSG: thing=4 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb0101250, metadata: DynMetadata(0xffffffff802c6ec0) }
[40477270383] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/status'
[40478893059] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/status'
[40481300970] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=4
[40483943544] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: DRIVER_READY from PID 11 �� no matching ManagedTask (already exited?)
[40486336374] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[40490448933] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: page cache miss for '/drivers/rtc_cmos', caching 45904 bytes
[40491680163] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'rtc_cmos' (len=45904, base=0x200000)
[40492208658] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[40493273040] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[40494004155] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=200000 exec=true
[40497058767] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   COPIED at 0x201420: [0f, 84, ea, 03, 00, 00, b8, 00]
[40500927555] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=206000 exec=false
[40502315337] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=207000 exec=false
[40503676059] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x208000 filesz=0 memsz=0 align=1
[40509861678] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[40510877451] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/rtc_cmos
[40512586026] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb01c82a0 arg=0xffffffffb0185320
[40514717661] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=12, applying inserts
[40515465969] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[40515853488] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=12
[40516553187] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[40517041059] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[40519712838] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[40524056826] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 12
[40525525095] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 12 woken, restoring IRQs
[40526118996] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/rtc_cmos
[40526985774] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/rtc_cmos' TID=12 PID=12
[40528702038] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x4
[40528979469] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/rtc_cmos for isa-0070 (entry='thingos_driver_start_safe', pid=12)
[40529714016] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=12 entry_pc=201000 user_sp=800000
[40537815285] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=12 buf_len=0
[40539211779] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=12 buf_len=100
[40540803567] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: sent DRIVER_READY to Sprout (pid=5) for driver pid=12
[40541996319] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=isa-0060 kind=drv.Ps2Keyboard vendor=0x0000 device=0x0000 class=0x000000 present=true
[40543107528] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0060/status' tid=7
[40543504815] [[34mDEBUG[0m] [rtc_cmos] [CPU3] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffef0 rip=0x201c27 rflags=0x206
[40544084328] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU0] RECVMSG: thing=4 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb0101250, metadata: DynMetadata(0xffffffff802c6ec0) }
[40545127227] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/status'
[40545556887] [[34mDEBUG[0m] [rtc_cmos] [CPU3] Starting... arg=4
[40546966515] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: DRIVER_READY from PID 12 ��� no matching ManagedTask (already exited?)
[40547470161] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/status'
[40548224970] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[40550458410] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=5
[40568460537] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: page cache miss for '/drivers/ps2_kbd', caching 72264 bytes
[40570501290] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'ps2_kbd' (len=72264, base=0x200000)
[40571595702] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[40572435750] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: task 12 claimed device 'isa-0070' (handle 2)
[40573556430] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[40573873494] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: claimed /sys/devices/isa-0070 (handle=2)
[40574630844] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=200000 exec=true
[40577917083] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   COPIED at 0x201420: [83, ff, 06, 77, 46, 48, 8d, 15]
[40583569290] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=209000 exec=false
[40586279217] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=20c000 exec=false
[40587416364] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x20d000 filesz=0 memsz=0 align=1
[40593958746] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[40596612243] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/ps2_kbd
[40597466844] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb01f1740 arg=0xffffffffb0187120
[40599103248] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=13, applying inserts
[40599682431] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[40600146543] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=13
[40600883532] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[40601308572] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[40603565739] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[40606674504] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: 2026-05-01 01:03:28 = 1777597408 unix_secs
[40607954145] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 13
[40608292923] [[32mINFO [0m] [kernel::time] [CPU3] System clock anchored: unix_secs=1777597408, mono_ns=20303856870, offset=1777597387696143130ns
[40609365456] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 13 woken, restoring IRQs
[40610047500] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: System clock anchored to 1777597408 unix_secs
[40612103598] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/ps2_kbd
[40613020767] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ps2_kbd' TID=13 PID=13
[40613736933] [[34mDEBUG[0m] [rtc_cmos] [CPU3] RTC: Entering maintenance loop.
[40616642418] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x5
[40617615555] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=13 entry_pc=201000 user_sp=800000
[40619633010] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=13 buf_len=0
[40621044420] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=13 buf_len=100
[40624020426] [[34mDEBUG[0m] [ps2_kbd] [CPU1] ps2_kbd: online — waiting for bristle pid
[40624890900] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/run/bristle/pid' tid=13
[40625619804] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 01:03:28.007938645 unix_secs=1777597408.007938645
[40628382762] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/run/bristle/pid' tid=13
[40631429883] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[40634679624] [[32mINFO [0m] [ps2_kbd] [CPU1] ps2_kbd: bristle pid=8
[40638676122] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: task subscribed to vector 0x21
[40640637741] [[34mDEBUG[0m] [ps2_kbd] [CPU1] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[40641695490] [[34mDEBUG[0m] [ps2_kbd] [CPU1] ps2_kbd: using interrupt-driven loop (IRQ vector 0x21)
[40654431213] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=88 tid=10
[40663373454] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/ps2_kbd for isa-0060 (entry='thingos_driver_start_safe', pid=13)
[40667293623] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[40668411663] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: sent DRIVER_READY to Sprout (pid=5) for driver pid=13
[40669269465] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_IMPORT_BUFFER requested: memfd=10, size=1920x1080
[40670133603] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=isa-0064 kind=drv.Ps2Mouse vendor=0x0000 device=0x0000 class=0x000000 present=true
[40671246990] [[34mDEBUG[0m] [kernel::syscall::handlers::memory] [CPU3] sys_vm_map: large backing file mapping offset=0 len=8294400 total_size=8294400
[40671736545] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0064/status' tid=7
[40674432975] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0064/status'
[40677493923] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0064/status'
[40678690965] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU0] RECVMSG: thing=4 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb0101250, metadata: DynMetadata(0xffffffff802c6ec0) }
[40679950806] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=6
[40688237964] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: DRIVER_READY from PID 13 ��� no matching ManagedTask (already exited?)
[40696973229] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: page cache miss for '/drivers/ps2_mouse', caching 71096 bytes
[40697421171] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[40699214952] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'ps2_mouse' (len=71096, base=0x200000)
[40699892838] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[40701401433] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[40702328634] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=200000 exec=true
[40704597615] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   COPIED at 0x201420: [83, f8, 02, 75, 1a, b8, 0b, 10]
[40708813662] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=209000 exec=false
[40710745416] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=20c000 exec=false
[40712245398] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x20d000 filesz=0 memsz=0 align=1
[40720349604] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[40721734977] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/ps2_mouse
[40722931161] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb0252fe0 arg=0xffffffffb01f6c00
[40726029531] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=14, applying inserts
[40726704678] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[40727160375] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=14
[40727845587] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[40728812652] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[40731541488] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[40736734137] [[34mDEBUG[0m]
```
</details>
