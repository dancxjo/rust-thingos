# ❌ Scenario: clicking the minimize chrome button minimizes through compositor chrome

> Last run: 2026-04-30 18:01:45

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the client has an xdg_toplevel | ✅ | 1500ms | - [📜](./01/serial.log) - |
| 2 | Then the Wayland hello client should be visible | ✅ | 3957ms | - [📜](./02/serial.log) - |
| 3 | When I click the Wayland hello minimize button | ✅ | 1936ms | - - - |
| 4 | Then the compositor should minimize the toplevel window | ❌ | 31053ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[35833782336] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[35868764151] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] enter
[35872044549] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] phys_memory_map ok
[35873007423] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] modules ok
[35873859021] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] phys_to_virt_offset ok
[35874514896] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] memory map logging done
[35899639050] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] boot_frame_alloc init ok
[36106139619] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] frame allocator build ok
[36107145921] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] frame allocator log ok
[36108249903] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] FRAME_ALLOCATOR init ok
[36109140045] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] tasking init ok
[36109804368] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[36131440092] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[36132099861] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] enter
[36132669210] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] set expand hook
[36133279545] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] expand hook ok
[36133796754] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock begin
[36134638188] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock ok
[36135172656] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] reserve_region begin
[36164502165] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] reserve_region ok
[36165141045] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] inner allocator init begin
[36168220935] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] inner allocator init ok
[36168984918] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] heap top store ok
[36169557930] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] init done
[36170254329] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[36192585264] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] devfs: set_boot_fb width=1920 height=1080 pitch=7680 resource_id=0xfb000000 phys=0x80000000
[36194792964] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs set_boot_fb elapsed_ticks=2120217 elapsed_us=1060
[36200820843] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs FbNode::new elapsed_ticks=5108499 elapsed_us=2554
[36201572385] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] register begin
[36206806977] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] register ok
[36207507996] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs register fb0 elapsed_ticks=5901027 elapsed_us=2950
[36208129353] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[36228621363] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[36230531898] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[36268499388] [[34mDEBUG[0m] [kernel] [CPU0] Seeding entropy pool...
[36269925285] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed begin
[36271114506] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] fill_entropy done
[36273693819] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] add_sample(timer) ok
[36274380780] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] mark_seeded(timer) ok
[36275019858] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[36275741964] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[36276438363] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed done
[36277881618] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] entropy::seed_from_hardware elapsed_ticks=7577361 elapsed_us=3788
[36278754600] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[36299746659] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[36309139647] [[35mTRACE[0m] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[36310353321] [[35mTRACE[0m] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[36311146047] [[35mTRACE[0m] [kernel::sched] [CPU0]   Allocating scheduler...
[36315286590] [[35mTRACE[0m] [kernel::sched] [CPU0]   Leaking scheduler...
[36315980349] [[35mTRACE[0m] [kernel::sched] [CPU0]   Initializing boot task...
[36318271770] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED: allocating CpuScheduler for cpu0 (total=1)
[36319175904] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[36319773105] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[36320800923] [[35mTRACE[0m] [kernel::sched] [CPU0]   Creating boot task...
[36333579711] [[35mTRACE[0m] [kernel::sched] [CPU0]   Creating idle tasks...
[36335004156] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff8008b390 kstack_top=0xffffffffb0020660 arg=0x0
[36337553769] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[36358852497] [[35mTRACE[0m] [kernel::sched] [CPU0]   Boot task initialized
[36361427520] [[34mDEBUG[0m] [kernel::sched::state] [CPU0] SCHED[cpu0]: current=Some(0) idle=Some(1) runnable=0 need_resched=true
[36362793291] [[35mTRACE[0m] [kernel::sched] [CPU0]   Storing scheduler pointer...
[36364146126] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[36365241759] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[36386760927] [[34mDEBUG[0m] [kernel] [CPU0] Initializing VFS...
[36390732774] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='set_cmdline' elapsed_ticks=3013956 elapsed_us=1506 total_ticks=3796815 total_us=1898
[36411021042] [[34mDEBUG[0m] [kernel::vfs] [CPU0] VFS: Created /dev/display directory
[36412061301] [[34mDEBUG[0m] [kernel::vfs] [CPU0] VFS: Created /dev/storage directory
[36454792935] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted union filesystem at / (root)
[36457633773] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted devfs at /dev
[36458940672] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted procfs at /proc
[36460053630] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted sysfs at /sys
[36464371878] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /tmp
[36465331947] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /run
[36466240998] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /services
[36467108733] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /session
[36468074973] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /data
[36468698805] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='vfs_init' elapsed_ticks=77149644 elapsed_us=38574 total_ticks=82028496 total_us=41014
[36469508262] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[36492616611] [[34mDEBUG[0m] [kernel] [CPU0] Scanning PCI bus...
[36496898790] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x29c0 at 00:00.0 class=060000 id=1
[36502546146] [[35mTRACE[0m] [kernel] [CPU0]   BAR0 (MEM32): 0x80000000 (size 0x800000)
[36515448783] [[35mTRACE[0m] [kernel] [CPU0]   BAR2 (MEM64): 0xc000000000 (size 0x4000)
[36519361725] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (MEM32): 0x80883000 (size 0x1000)
[36520166430] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1050 at 00:01.0 class=030000 id=2
[36525426267] [[35mTRACE[0m] [kernel] [CPU0]   BAR0 (I/O):  0x6060 (size 0x20)
[36530529222] [[35mTRACE[0m] [kernel] [CPU0]   BAR1 (MEM32): 0x80882000 (size 0x1000)
[36547353546] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (MEM64): 0xc000004000 (size 0x4000)
[36548411625] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1000 at 00:02.0 class=020000 id=3
[36552335028] [[35mTRACE[0m] [kernel] [CPU0]   BAR1 (MEM32): 0x80881000 (size 0x1000)
[36564799392] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (MEM64): 0xc000008000 (size 0x4000)
[36565488366] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1059 at 00:03.0 class=040100 id=4
[36566293203] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2918 at 00:1f.0 class=060100 id=5
[36569954949] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (I/O):  0x6040 (size 0x20)
[36573674445] [[35mTRACE[0m] [kernel] [CPU0]   BAR5 (MEM32): 0x80880000 (size 0x1000)
[36578291475] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2922 at 00:1f.2 class=010601 id=6
[36581219697] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (I/O):  0x6000 (size 0x40)
[36581864352] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2930 at 00:1f.3 class=0c0500 id=7
[36583840755] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='scan_pci' elapsed_ticks=90437061 elapsed_us=45218 total_ticks=197170248 total_us=98585
[36584538903] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[36614524023] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='register_legacy_devices' elapsed_ticks=1169685 elapsed_us=584 total_ticks=227825268 total_us=113912
[36615949854] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[36646774857] [[34mDEBUG[0m] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[36657800388] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[36658726566] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[36660280074] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[36661645911] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[36667793283] [[34mDEBUG[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[36670191756] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[36671584521] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[36672398631] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[36673145817] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[36673935012] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[36675197625] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[36676431660] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[36677238147] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[36678017541] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[36678787134] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[36679560423] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[36680882304] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[36682315494] [[34mDEBUG[0m] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[36683333082] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[36685432311] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Registers initialized
[36687441450] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[36689270013] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[36690547245] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: All pins masked
[36692087619] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> NMI
[36692865528] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[36693739764] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ4 -> GSI 4 -> 0x24
[36694297530] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Init complete
[36728612118] [[34mDEBUG[0m] [bran::arch::x86_64::ioapic] [CPU0] LAPIC: calibrated 62524400 ticks/sec (delta=625244, ok=true) -> init_cnt=625244
[36730202553] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62524400 ticks/sec), init_cnt=625244 for 100Hz
[36730902186] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='setup_preemption_timer' elapsed_ticks=83088291 elapsed_us=41544 total_ticks=344237256 total_us=172118
[36731619342] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[36753088977] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[36753870186] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[36755564043] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[36758278920] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[36787887312] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[36788900247] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[36799240269] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] SMP: CPU 1 initializing preemption timer (vec=32 init_cnt=625244)
[36801800574] [[34mDEBUG[0m] [kernel] [CPU1] SMP: kernel_secondary_entry arg_cpu=1 runtime_cpu=1
[36803007186] [[34mDEBUG[0m] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[36805022628] [[34mDEBUG[0m] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[36810628866] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff8008b390 kstack_top=0xffffffffb00375a0 arg=0x1
[36811771656] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Task 2 assigned to CPU 1
[36812951010] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] SMP: CPU 2 initializing preemption timer (vec=32 init_cnt=625244)
[36813115548] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[36814121124] [[34mDEBUG[0m] [kernel] [CPU2] SMP: kernel_secondary_entry arg_cpu=2 runtime_cpu=2
[36814251045] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[36814915500] [[34mDEBUG[0m] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[36815689449] [[34mDEBUG[0m] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[36815982918] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 2 deferred inserts
[36817147983] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU2] INIT KERNEL CTX: entry=0xffffffff8008b390 kstack_top=0xffffffffb0048d20 arg=0x2
[36817446204] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=1
[36818059938] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] SCHED: Task 3 assigned to CPU 2
[36818394558] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=2
[36819959715] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Applying 1 deferred inserts
[36820057395] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[36821239356] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserting TID=3
[36821661327] [[34mDEBUG[0m] [kernel::sched::lifecycle] [CPU1] SMP: Secondary CPU 1 online!
[36822795339] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: run_scheduler entry on CPU 1
[36823136328] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserts applied
[36823893744] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: bootstrap_cpu start on CPU 1
[36823913742] [[34mDEBUG[0m] [kernel::sched::lifecycle] [CPU2] SMP: Secondary CPU 2 online!
[36824983239] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: run_scheduler entry on CPU 2
[36825373992] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle thread 2 and is now schedulable
[36825682641] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: bootstrap_cpu start on CPU 2
[36827208825] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle thread 3 and is now schedulable
[36839833866] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[36840160764] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] SMP: CPU 3 initializing preemption timer (vec=32 init_cnt=625244)
[36840593394] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[36841537293] [[34mDEBUG[0m] [kernel] [CPU3] SMP: kernel_secondary_entry arg_cpu=3 runtime_cpu=3
[36842216862] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[36842262402] [[34mDEBUG[0m] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[36842829111] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='start_secondary_cpus' elapsed_ticks=89950476 elapsed_us=44975 total_ticks=456166392 total_us=228083
[36843809904] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[36844365690] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[36845619393] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff8008b390 kstack_top=0xffffffffb005a4a0 arg=0x3
[36846419379] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SCHED: Task 4 assigned to CPU 3
[36859932945] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[36860625846] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=4
[36861302577] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[36861708708] [[34mDEBUG[0m] [kernel::sched::lifecycle] [CPU3] SMP: Secondary CPU 3 online!
[36862114872] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: run_scheduler entry on CPU 3
[36862493514] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: bootstrap_cpu start on CPU 3
[36862901064] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle thread 4 and is now schedulable
[36898984386] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[36995036166] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Enumerating 214 boot modules...
[36996239973] [[35mTRACE[0m] [kernel] [CPU0]   Module[0]: name='/bin/sprout' cmdline='init' size=89912 bytes
[36997022832] [[35mTRACE[0m] [kernel] [CPU0]   Module[1]: name='/bin/bristle' cmdline='' size=68744 bytes
[36997784736] [[35mTRACE[0m] [kernel] [CPU0]   Module[2]: name='/drivers/rtc_cmos' cmdline='init' size=45904 bytes
[36998413353] [[35mTRACE[0m] [kernel] [CPU0]   Module[3]: name='/drivers/ps2_kbd' cmdline='' size=72264 bytes
[36999039297] [[35mTRACE[0m] [kernel] [CPU0]   Module[4]: name='/bin/sh' cmdline='init' size=127864 bytes
[36999705798] [[35mTRACE[0m] [kernel] [CPU0]   Module[5]: name='/bin/ls' cmdline='init' size=62136 bytes
[37000503672] [[35mTRACE[0m] [kernel] [CPU0]   Module[6]: name='/bin/ln' cmdline='init' size=47568 bytes
[37001322237] [[35mTRACE[0m] [kernel] [CPU0]   Module[7]: name='/bin/ps' cmdline='init' size=50544 bytes
[37007057703] [[35mTRACE[0m] [kernel] [CPU0]   Module[8]: name='/bin/kill' cmdline='init' size=55584 bytes
[37008517689] [[35mTRACE[0m] [kernel] [CPU0]   Module[9]: name='/bin/killall' cmdline='init' size=67656 bytes
[37009342326] [[35mTRACE[0m] [kernel] [CPU0]   Module[10]: name='/bin/cat' cmdline='init' size=50832 bytes
[37010020278] [[35mTRACE[0m] [kernel] [CPU0]   Module[11]: name='/bin/head' cmdline='init' size=56112 bytes
[37010679783] [[35mTRACE[0m] [kernel] [CPU0]   Module[12]: name='/bin/tail' cmdline='init' size=56088 bytes
[37011312690] [[35mTRACE[0m] [kernel] [CPU0]   Module[13]: name='/bin/top' cmdline='init' size=65488 bytes
[37011959952] [[35mTRACE[0m] [kernel] [CPU0]   Module[14]: name='/bin/wc' cmdline='init' size=56248 bytes
[37013007570] [[35mTRACE[0m] [kernel] [CPU0]   Module[15]: name='/bin/yes' cmdline='init' size=44520 bytes
[37014162636] [[35mTRACE[0m] [kernel] [CPU0]   Module[16]: name='/bin/which' cmdline='init' size=50448 bytes
[37026882519] [[35mTRACE[0m] [kernel] [CPU0]   Module[17]: name='/bin/cp' cmdline='init' size=46312 bytes
[37027472361] [[35mTRACE[0m] [kernel] [CPU0]   Module[18]: name='/bin/mv' cmdline='init' size=47048 bytes
[37028090187] [[35mTRACE[0m] [kernel] [CPU0]   Module[19]: name='/bin/rm' cmdline='init' size=47128 bytes
[37028704152] [[35mTRACE[0m] [kernel] [CPU0]   Module[20]: name='/bin/rmdir' cmdline='init' size=47144 bytes
[37029284160] [[35mTRACE[0m] [kernel] [CPU0]   Module[21]: name='/bin/mkdir' cmdline='init' size=47144 bytes
[37029875586] [[35mTRACE[0m] [kernel] [CPU0]   Module[22]: name='/bin/mount' cmdline='init' size=57688 bytes
[37030446750] [[35mTRACE[0m] [kernel] [CPU0]   Module[23]: name='/bin/echo' cmdline='init' size=44616 bytes
[37031032764] [[35mTRACE[0m] [kernel] [CPU0]   Module[24]: name='/bin/printf' cmdline='init' size=93952 bytes
[37031579607] [[35mTRACE[0m] [kernel] [CPU0]   Module[25]: name='/bin/grep' cmdline='init' size=85240 bytes
[37032092493] [[35mTRACE[0m] [kernel] [CPU0]   Module[26]: name='/bin/pwd' cmdline='init' size=39184 bytes
[37032645903] [[35mTRACE[0m] [kernel] [CPU0]   Module[27]: name='/bin/touch' cmdline='init' size=46160 bytes
[37033171725] [[35mTRACE[0m] [kernel] [CPU0]   Module[28]: name='/bin/setshell' cmdline='init' size=47048 bytes
[37033892346] [[35mTRACE[0m] [kernel] [CPU0]   Module[29]: name='/bin/dmesg' cmdline='init' size=39328 bytes
[37034807700] [[35mTRACE[0m] [kernel] [CPU0]   Module[30]: name='/bin/stat' cmdline='init' size=46200 bytes
[37035360846] [[35mTRACE[0m] [kernel] [CPU0]   Module[31]: name='/bin/file' cmdline='init' size=51104 bytes
[37035904422] [[35mTRACE[0m] [kernel] [CPU0]   Module[32]: name='/bin/dirname' cmdline='init' size=50096 bytes
[37036422423] [[35mTRACE[0m] [kernel] [CPU0]   Module[33]: name='/bin/basename' cmdline='init' size=50104 bytes
[37036939434] [[35mTRACE[0m] [kernel] [CPU0]   Module[34]: name='/bin/sleep' cmdline='init' size=56824 bytes
[37037449251] [[35mTRACE[0m] [kernel] [CPU0]   Module[35]: name='/bin/sort' cmdline='init' size=66336 bytes
[37037958012] [[35mTRACE[0m] [kernel] [CPU0]   Module[36]: name='/bin/env' cmdline='init' size=45832 bytes
[37038484758] [[35mTRACE[0m] [kernel] [CPU0]   Module[37]: name='/bin/uname' cmdline='' size=44632 bytes
[37039020414] [[35mTRACE[0m] [kernel] [CPU0]   Module[38]: name='/bin/true' cmdline='' size=20744 bytes
[37039634313] [[35mTRACE[0m] [kernel] [CPU0]   Module[39]: name='/bin/false' cmdline='' size=20744 bytes
[37040154360] [[35mTRACE[0m] [kernel] [CPU0]   Module[40]: name='/bin/input_echo' cmdline='' size=43016 bytes
[37040859735] [[35mTRACE[0m] [kernel] [CPU0]   Module[41]: name='/drivers/ps2_mouse' cmdline='' size=71096 bytes
[37041638172] [[35mTRACE[0m] [kernel] [CPU0]   Module[42]: name='/drivers/display_bootfb' cmdline='' size=91736 bytes
[37042191879] [[35mTRACE[0m] [kernel] [CPU0]   Module[43]: name='/drivers/display_virtio_gpu' cmdline='' size=147560 bytes
[37042805646] [[35mTRACE[0m] [kernel] [CPU0]   Module[44]: name='/bin/cambium' cmdline='' size=144768 bytes
[37043360046] [[35mTRACE[0m] [kernel] [CPU0]   Module[45]: name='/drivers/virtio_netd' cmdline='' size=123920 bytes
[37044055356] [[35mTRACE[0m] [kernel] [CPU0]   Module[46]: name='/drivers/rtl8168d' cmdline='' size=61392 bytes
[37044833925] [[35mTRACE[0m] [kernel] [CPU0]   Module[47]: name='/bin/netd' cmdline='' size=10819752 bytes
[37045428321] [[35mTRACE[0m] [kernel] [CPU0]   Module[48]: name='/bin/mesocarp' cmdline='' size=123496 bytes
[37046149140] [[35mTRACE[0m] [kernel] [CPU0]   Module[49]: name='/bin/mdns' cmdline='' size=123496 bytes
[37047807918] [[35mTRACE[0m] [kernel] [CPU0]   Module[50]: name='/bin/mdnsd' cmdline='' size=123496 bytes
[37060023099] [[35mTRACE[0m] [kernel] [CPU0]   Module[51]: name='/bin/fetchd' cmdline='' size=76752 bytes
[37060572384] [[35mTRACE[0m] [kernel] [CPU0]   Module[52]: name='/bin/httpsd' cmdline='' size=536128 bytes
[37061099691] [[35mTRACE[0m] [kernel] [CPU0]   Module[53]: name='/bin/find' cmdline='' size=57896 bytes
[37061605878] [[35mTRACE[0m] [kernel] [CPU0]   Module[54]: name='/bin/ip' cmdline='' size=55472 bytes
[37062182058] [[35mTRACE[0m] [kernel] [CPU0]   Module[55]: name='/bin/iso_reader' cmdline='' size=37424 bytes
[37062700785] [[35mTRACE[0m] [kernel] [CPU0]   Module[56]: name='/bin/ping' cmdline='' size=65184 bytes
[37063204860] [[35mTRACE[0m] [kernel] [CPU0]   Module[57]: name='/bin/nslookup' cmdline='' size=56424 bytes
[37063709232] [[35mTRACE[0m] [kernel] [CPU0]   Module[58]: name='/drivers/ahci_disk' cmdline='' size=53784 bytes
[37064227398] [[35mTRACE[0m] [kernel] [CPU0]   Module[59]: name='/drivers/ata_disk' cmdline='' size=64448 bytes
[37064758434] [[35mTRACE[0m] [kernel] [CPU0]   Module[60]: name='/bin/iso9660d' cmdline='' size=78544 bytes
[37065346329] [[35mTRACE[0m] [kernel] [CPU0]   Module[61]: name='/drivers/virtio_sound' cmdline='' size=99296 bytes
[37065877992] [[35mTRACE[0m] [kernel] [CPU0]   Module[62]: name='/drivers/hdaudio' cmdline='' size=67696 bytes
[37066564491] [[35mTRACE[0m] [kernel] [CPU0]   Module[63]: name='/drivers/pci_stubd' cmdline='' size=58168 bytes
[37067364279] [[35mTRACE[0m] [kernel] [CPU0]   Module[64]: name='/drivers/chime' cmdline='' size=59536 bytes
[37067922705] [[35mTRACE[0m] [kernel] [CPU0]   Module[65]: name='/bin/vfs_hello' cmdline='' size=39560 bytes
[37068442290] [[35mTRACE[0m] [kernel] [CPU0]   Module[66]: name='/bin/show_args' cmdline='' size=42488 bytes
[37068990750] [[35mTRACE[0m] [kernel] [CPU0]   Module[67]: name='/bin/clock' cmdline='' size=82936 bytes
[37069497201] [[35mTRACE[0m] [kernel] [CPU0]   Module[68]: name='/bin/env_roundtrip' cmdline='' size=48232 bytes
[37070012925] [[35mTRACE[0m] [kernel] [CPU0]   Module[69]: name='/bin/cwd_test' cmdline='' size=43696 bytes
[37070520300] [[35mTRACE[0m] [kernel] [CPU0]   Module[70]: name='/bin/date' cmdline='' size=67032 bytes
[37071021603] [[35mTRACE[0m] [kernel] [CPU0]   Module[71]: name='/bin/wayland_hello' cmdline='' size=82360 bytes
[37071538218] [[35mTRACE[0m] [kernel] [CPU0]   Module[72]: name='/bin/terminal' cmdline='' size=91056 bytes
[37072181124] [[35mTRACE[0m] [kernel] [CPU0]   Module[73]: name='/bin/tee' cmdline='' size=44968 bytes
[37072830168] [[35mTRACE[0m] [kernel] [CPU0]   Module[74]: name='/bin/xargs' cmdline='' size=55016 bytes
[37073728362] [[35mTRACE[0m] [kernel] [CPU0]   Module[75]: name='/bin/placed' cmdline='' size=38048 bytes
[37074667740] [[35mTRACE[0m] [kernel] [CPU0]   Module[76]: name='/bin/bloom' cmdline='' size=462464 bytes
[37076333316] [[35mTRACE[0m] [kernel] [CPU0]   Module[77]: name='/bin/clear' cmdline='' size=20872 bytes
[37076880555] [[35mTRACE[0m] [kernel] [CPU0]   Module[78]: name='/bin/loglevel' cmdline='' size=41536 bytes
[37077410106] [[35mTRACE[0m] [kernel] [CPU0]   Module[79]: name='/bin/poll_mux' cmdline='' size=39056 bytes
[37078041957] [[35mTRACE[0m] [kernel] [CPU0]   Module[80]: name='/bin/ipc_service_demo' cmdline='' size=47592 bytes
[37078976913] [[35mTRACE[0m] [kernel] [CPU0]   Module[81]: name='/bin/ipc_pipe_demo' cmdline='' size=43936 bytes
[37079885403] [[35mTRACE[0m] [kernel] [CPU0]   Module[82]: name='/bin/ipc_provider_demo' cmdline='' size=84440 bytes
[37085837679] [[35mTRACE[0m] [kernel] [CPU0]   Module[83]: name='/bin/vfs_test_provider' cmdline='' size=70648 bytes
[37086404256] [[35mTRACE[0m] [kernel] [CPU0]   Module[84]: name='/bin/ipc_memfd_demo' cmdline='' size=38232 bytes
[37086928428] [[35mTRACE[0m] [kernel] [CPU0]   Module[85]: name='/bin/test_exec' cmdline='' size=62592 bytes
[37087550874] [[35mTRACE[0m] [kernel] [CPU0]   Module[86]: name='/bin/test_vm_protect' cmdline='' size=38224 bytes
[37088097519] [[35mTRACE[0m] [kernel] [CPU0]   Module[87]: name='/bin/test_exec_env' cmdline='' size=58904 bytes
[37088655219] [[35mTRACE[0m] [kernel] [CPU0]   Module[88]: name='/bin/test_threads' cmdline='' size=82376 bytes
[37089171075] [[35mTRACE[0m] [kernel] [CPU0]   Module[89]: name='/bin/test_futex' cmdline='' size=52992 bytes
[37089683235] [[35mTRACE[0m] [kernel] [CPU0]   Module[90]: name='/bin/ld_so' cmdline='' size=378600 bytes
[37090185792] [[35mTRACE[0m] [kernel] [CPU0]   Module[91]: name='/bin/test_dyn_loader' cmdline='' size=57760 bytes
[37090702539] [[35mTRACE[0m] [kernel] [CPU0]   Module[92]: name='/bin/test_dlopen' cmdline='' size=64688 bytes
[37091215194] [[35mTRACE[0m] [kernel] [CPU0]   Module[93]: name='/bin/reboot' cmdline='' size=32048 bytes
[37091745306] [[35mTRACE[0m] [kernel] [CPU0]   Module[94]: name='/bin/shutdown' cmdline='' size=32048 bytes
[37092256245] [[35mTRACE[0m] [kernel] [CPU0]   Module[95]: name='/bin/attr_list' cmdline='' size=47400 bytes
[37092823647] [[35mTRACE[0m] [kernel] [CPU0]   Module[96]: name='/bin/attr_get' cmdline='' size=47488 bytes
[37093341450] [[35mTRACE[0m] [kernel] [CPU0]   Module[97]: name='/bin/attr_set' cmdline='' size=69600 bytes
[37093852686] [[35mTRACE[0m] [kernel] [CPU0]   Module[98]: name='/bin/attr_rm' cmdline='' size=47008 bytes
[37094357322] [[35mTRACE[0m] [kernel] [CPU0]   Module[99]: name='/lib/libpistil.so' cmdline='' size=826992 bytes
[37094899314] [[35mTRACE[0m] [kernel] [CPU0]   Module[100]: name='/share/wallpapers/clouds.bmp' cmdline='' size=786486 bytes
[37095448566] [[35mTRACE[0m] [kernel] [CPU0]   Module[101]: name='/share/wallpapers/flower.bmp' cmdline='' size=8386614 bytes
[37096023426] [[35mTRACE[0m] [kernel] [CPU0]   Module[102]: name='/share/wallpapers/flower.png' cmdline='' size=2652468 bytes
[37096581687] [[35mTRACE[0m] [kernel] [CPU0]   Module[103]: name='/share/wallpapers/leather.bmp' cmdline='' size=1179702 bytes
[37097136780] [[35mTRACE[0m] [kernel] [CPU0]   Module[104]: name='/share/wallpapers/linen.bmp' cmdline='' size=4718646 bytes
[37097711673] [[35mTRACE[0m] [kernel] [CPU0]   Module[105]: name='/share/fonts/DSEG7Classic-Regular.ttf' cmdline='' size=23272 bytes
[37098423318] [[35mTRACE[0m] [kernel] [CPU0]   Module[106]: name='/share/fonts/Hack-Regular.ttf' cmdline='' size=309408 bytes
[37099052166] [[35mTRACE[0m] [kernel] [CPU0]   Module[107]: name='/share/fonts/Inter-Regular.ttf' cmdline='' size=876576 bytes
[37099833507] [[35mTRACE[0m] [kernel] [CPU0]   Module[108]: name='/share/fonts/Iosevka-Regular.ttf' cmdline='' size=10457376 bytes
[37101112059] [[35mTRACE[0m] [kernel] [CPU0]   Module[109]: name='/share/fonts/JetBrainsMono-Regular.ttf' cmdline='' size=270224 bytes
[37101834363] [[35mTRACE[0m] [kernel] [CPU0]   Module[110]: name='/share/fonts/NotoSans-Regular.ttf' cmdline='' size=569208 bytes
[37102586796] [[35mTRACE[0m] [kernel] [CPU0]   Module[111]: name='/share/fonts/NotoSansSymbol-Regular.ttf' cmdline='' size=258156 bytes
[37103247456] [[35mTRACE[0m] [kernel] [CPU0]   Module[112]: name='/share/fonts/NotoSansSymbol2-Regular.ttf' cmdline='' size=656852 bytes
[37103906235] [[35mTRACE[0m] [kernel] [CPU0]   Module[113]: name='/share/fonts/NotoSerif-Regular.ttf' cmdline='' size=616196 bytes
[37104621147] [[35mTRACE[0m] [kernel] [CPU0]   Module[114]: name='/share/themes/solarized_warm.toml' cmdline='' size=1832 bytes
[37105339986] [[35mTRACE[0m] [kernel] [CPU0]   Module[115]: name='/share/cursors/future/alias.svg' cmdline='' size=9033 bytes
[37106008962] [[35mTRACE[0m] [kernel] [CPU0]   Module[116]: name='/share/cursors/future/all-scroll.svg' cmdline='' size=5170 bytes
[37106924382] [[35mTRACE[0m] [kernel] [CPU0]   Module[117]: name='/share/cursors/future/bottom_left_corner.svg' cmdline='' size=1441 bytes
[37108033677] [[35mTRACE[0m] [kernel] [CPU0]   Module[118]: name='/share/cursors/future/bottom_right_corner.svg' cmdline='' size=3037 bytes
[37108735752] [[35mTRACE[0m] [kernel] [CPU0]   Module[119]: name='/share/cursors/future/bottom_side.svg' cmdline='' size=5432 bytes
[37109387799] [[35mTRACE[0m] [kernel] [CPU0]   Module[120]: name='/share/cursors/future/cell.svg' cmdline='' size=3707 bytes
[37110040077] [[35mTRACE[0m] [kernel] [CPU0]   Module[121]: name='/share/cursors/future/center_ptr.svg' cmdline='' size=9766 bytes
[37110701298] [[35mTRACE[0m] [kernel] [CPU0]   Module[122]: name='/share/cursors/future/col-resize.svg' cmdline='' size=16463 bytes
[37111429773] [[35mTRACE[0m] [kernel] [CPU0]   Module[123]: name='/share/cursors/future/color-picker.svg' cmdline='' size=6296 bytes
[37111955364] [[35mTRACE[0m] [kernel] [CPU0]   Module[124]: name='/share/cursors/future/context-menu.svg' cmdline='' size=9011 bytes
[37113155640] [[35mTRACE[0m] [kernel] [CPU0]   Module[125]: name='/share/cursors/future/copy.svg' cmdline='' size=3527 bytes
[37117357860] [[35mTRACE[0m] [kernel] [CPU0]   Module[126]: name='/share/cursors/future/crosshair.svg' cmdline='' size=16918 bytes
[37118051157] [[35mTRACE[0m] [kernel] [CPU0]   Module[127]: name='/share/cursors/future/default.svg' cmdline='' size=3079 bytes
[37118700729] [[35mTRACE[0m] [kernel] [CPU0]   Module[128]: name='/share/cursors/future/dnd-move.svg' cmdline='' size=3569 bytes
[37119368847] [[35mTRACE[0m] [kernel] [CPU0]   Module[129]: name='/share/cursors/future/dnd-no-drop.svg' cmdline='' size=4753 bytes
[37120028154] [[35mTRACE[0m] [kernel] [CPU0]   Module[130]: name='/share/cursors/future/down-arrow.svg' cmdline='' size=1820 bytes
[37120689705] [[35mTRACE[0m] [kernel] [CPU0]   Module[131]: name='/share/cursors/future/draft.svg' cmdline='' size=3136 bytes
[37121382408] [[35mTRACE[0m] [kernel] [CPU0]   Module[132]: name='/share/cursors/future/fleur.svg' cmdline='' size=28469 bytes
[37122027063] [[35mTRACE[0m] [kernel] [CPU0]   Module[133]: name='/share/cursors/future/help.svg' cmdline='' size=5873 bytes
[37122748146] [[35mTRACE[0m] [kernel] [CPU0]   Module[134]: name='/share/cursors/future/left-arrow.svg' cmdline='' size=1785 bytes
[37123334028] [[35mTRACE[0m] [kernel] [CPU0]   Module[135]: name='/share/cursors/future/left_side.svg' cmdline='' size=3848 bytes
[37123963206] [[35mTRACE[0m] [kernel] [CPU0]   Module[136]: name='/share/cursors/future/no-drop.svg' cmdline='' size=3861 bytes
[37124553642] [[35mTRACE[0m] [kernel] [CPU0]   Module[137]: name='/share/cursors/future/not-allowed.svg' cmdline='' size=1567 bytes
[37125110022] [[35mTRACE[0m] [kernel] [CPU0]   Module[138]: name='/share/cursors/future/openhand.svg' cmdline='' size=3435 bytes
[37125737451] [[35mTRACE[0m] [kernel] [CPU0]   Module[139]: name='/share/cursors/future/pencil.svg' cmdline='' size=6527 bytes
[37126383723] [[35mTRACE[0m] [kernel] [CPU0]   Module[140]: name='/share/cursors/future/pirate.svg' cmdline='' size=6663 bytes
[37127026431] [[35mTRACE[0m] [kernel] [CPU0]   Module[141]: name='/share/cursors/future/pointer.svg' cmdline='' size=2922 bytes
[37127754609] [[35mTRACE[0m] [kernel] [CPU0]   Module[142]: name='/share/cursors/future/progress-01.svg' cmdline='' size=11030 bytes
[37128426951] [[35mTRACE[0m] [kernel] [CPU0]   Module[143]: name='/share/cursors/future/progress-02.svg' cmdline='' size=12198 bytes
[37129095498] [[35mTRACE[0m] [kernel] [CPU0]   Module[144]: name='/share/cursors/future/progress-03.svg' cmdline='' size=12733 bytes
[37129768269] [[35mTRACE[0m] [kernel] [CPU0]   Module[145]: name='/share/cursors/future/progress-04.svg' cmdline='' size=13777 bytes
[37130428104] [[35mTRACE[0m] [kernel] [CPU0]   Module[146]: name='/share/cursors/future/progress-05.svg' cmdline='' size=13835 bytes
[37131146052] [[35mTRACE[0m] [kernel] [CPU0]   Module[147]: name='/share/cursors/future/progress-06.svg' cmdline='' size=14907 bytes
[37132023027] [[35mTRACE[0m] [kernel] [CPU0]   Module[148]: name='/share/cursors/future/progress-07.svg' cmdline='' size=14805 bytes
[37133581122] [[35mTRACE[0m] [kernel] [CPU0]   Module[149]: name='/share/cursors/future/progress-08.svg' cmdline='' size=15996 bytes
[37134245115] [[35mTRACE[0m] [kernel] [CPU0]   Module[150]: name='/share/cursors/future/progress-09.svg' cmdline='' size=16010 bytes
[37134936465] [[35mTRACE[0m] [kernel] [CPU0]   Module[151]: name='/share/cursors/future/progress-10.svg' cmdline='' size=17080 bytes
[37135621545] [[35mTRACE[0m] [kernel] [CPU0]   Module[152]: name='/share/cursors/future/progress-11.svg' cmdline='' size=17096 bytes
[37136314248] [[35mTRACE[0m] [kernel] [CPU0]   Module[153]: name='/share/cursors/future/progress-12.svg' cmdline='' size=17079 bytes
[37136998701] [[35mTRACE[0m] [kernel] [CPU0]   Module[154]: name='/share/cursors/future/progress-13.svg' cmdline='' size=15900 bytes
[37137684639] [[35mTRACE[0m] [kernel] [CPU0]   Module[155]: name='/share/cursors/future/progress-14.svg' cmdline='' size=16001 bytes
[37138341207] [[35mTRACE[0m] [kernel] [CPU0]   Module[156]: name='/share/cursors/future/progress-15.svg' cmdline='' size=14931 bytes
[37139265504] [[35mTRACE[0m] [kernel] [CPU0]   Module[157]: name='/share/cursors/future/progress-16.svg' cmdline='' size=14873 bytes
[37141062090] [[35mTRACE[0m] [kernel] [CPU0]   Module[158]: name='/share/cursors/future/progress-17.svg' cmdline='' size=13846 bytes
[37141906131] [[35mTRACE[0m] [kernel] [CPU0]   Module[159]: name='/share/cursors/future/progress-18.svg' cmdline='' size=13835 bytes
[37142740008] [[35mTRACE[0m] [kernel] [CPU0]   Module[160]: name='/share/cursors/future/progress-19.svg' cmdline='' size=12645 bytes
[37143611274] [[35mTRACE[0m] [kernel] [CPU0]   Module[161]: name='/share/cursors/future/progress-20.svg' cmdline='' size=12741 bytes
[37144502076] [[35mTRACE[0m] [kernel] [CPU0]   Module[162]: name='/share/cursors/future/progress-21.svg' cmdline='' size=11660 bytes
[37145424558] [[35mTRACE[0m] [kernel] [CPU0]   Module[163]: name='/share/cursors/future/progress-22.svg' cmdline='' size=11612 bytes
[37146258369] [[35mTRACE[0m] [kernel] [CPU0]   Module[164]: name='/share/cursors/future/progress-23.svg' cmdline='' size=11674 bytes
[37147028523] [[35mTRACE[0m] [kernel] [CPU0]   Module[165]: name='/share/cursors/future/progress.svg' cmdline='' size=11642 bytes
[37148096832] [[35mTRACE[0m] [kernel] [CPU0]   Module[166]: name='/share/cursors/future/right-arrow.svg' cmdline='' size=3432 bytes
[37149310836] [[35mTRACE[0m] [kernel] [CPU0]   Module[167]: name='/share/cursors/future/right_ptr.svg' cmdline='' size=5155 bytes
[37150021227] [[35mTRACE[0m] [kernel] [CPU0]   Module[168]: name='/share/cursors/future/right_side.svg' cmdline='' size=6447 bytes
[37150728945] [[35mTRACE[0m] [kernel] [CPU0]   Module[169]: name='/share/cursors/future/row-resize.svg' cmdline='' size=15825 bytes
[37151466990] [[35mTRACE[0m] [kernel] [CPU0]   Module[170]: name='/share/cursors/future/size_bdiag.svg' cmdline='' size=16323 bytes
[37156941591] [[35mTRACE[0m] [kernel] [CPU0]   Module[171]: name='/share/cursors/future/size_fdiag.svg' cmdline='' size=16589 bytes
[37157858628] [[35mTRACE[0m] [kernel] [CPU0]   Module[172]: name='/share/cursors/future/size_hor.svg' cmdline='' size=16255 bytes
[37158776391] [[35mTRACE[0m] [kernel] [CPU0]   Module[173]: name='/share/cursors/future/size_ver.svg' cmdline='' size=15822 bytes
[37159646436] [[35mTRACE[0m] [kernel] [CPU0]   Module[174]: name='/share/cursors/future/text.svg' cmdline='' size=5584 bytes
[37160486649] [[35mTRACE[0m] [kernel] [CPU0]   Module[175]: name='/share/cursors/future/top_left_corner.svg' cmdline='' size=7121 bytes
[37161394611] [[35mTRACE[0m] [kernel] [CPU0]   Module[176]: name='/share/cursors/future/top_right_corner.svg' cmdline='' size=7139 bytes
[37162278120] [[35mTRACE[0m] [kernel] [CPU0]   Module[177]: name='/share/cursors/future/top_side.svg' cmdline='' size=10651 bytes
[37163165028] [[35mTRACE[0m] [kernel] [CPU0]   Module[178]: name='/share/cursors/future/up-arrow.svg' cmdline='' size=7653 bytes
[37164064245] [[35mTRACE[0m] [kernel] [CPU0]   Module[179]: name='/share/cursors/future/vertical-text.svg' cmdline='' size=5621 bytes
[37164640161] [[35mTRACE[0m] [kernel] [CPU0]   Module[180]: name='/share/cursors/future/wait-01.svg' cmdline='' size=15308 bytes
[37165206441] [[35mTRACE[0m] [kernel] [CPU0]   Module[181]: name='/share/cursors/future/wait-02.svg' cmdline='' size=6672 bytes
[37165940955] [[35mTRACE[0m] [kernel] [CPU0]   Module[182]: name='/share/cursors/future/wait-03.svg' cmdline='' size=6686 bytes
[37167386520] [[35mTRACE[0m] [kernel] [CPU0]   Module[183]: name='/share/cursors/future/wait-04.svg' cmdline='' size=7827 bytes
[37168451232] [[35mTRACE[0m] [kernel] [CPU0]   Module[184]: name='/share/cursors/future/wait-05.svg' cmdline='' size=7835 bytes
[37169132187] [[35mTRACE[0m] [kernel] [CPU0]   Module[185]: name='/share/cursors/future/wait-06.svg' cmdline='' size=8976 bytes
[37169758659] [[35mTRACE[0m] [kernel] [CPU0]   Module[186]: name='/share/cursors/future/wait-07.svg' cmdline='' size=8989 bytes
[37170356751] [[35mTRACE[0m] [kernel] [CPU0]   Module[187]: name='/share/cursors/future/wait-08.svg' cmdline='' size=10126 bytes
[37170967119] [[35mTRACE[0m] [kernel] [CPU0]   Module[188]: name='/share/cursors/future/wait-09.svg' cmdline='' size=10140 bytes
[37171713117] [[35mTRACE[0m] [kernel] [CPU0]   Module[189]: name='/share/cursors/future/wait-10.svg' cmdline='' size=11277 bytes
[37172281212] [[35mTRACE[0m] [kernel] [CPU0]   Module[190]: name='/share/cursors/future/wait-11.svg' cmdline='' size=11291 bytes
[37173335100] [[35mTRACE[0m] [kernel] [CPU0]   Module[191]: name='/share/cursors/future/wait-12.svg' cmdline='' size=11277 bytes
[37174841517] [[35mTRACE[0m] [kernel] [CPU0]   Module[192]: name='/share/cursors/future/wait-13.svg' cmdline='' size=10140 bytes
[37175416806] [[35mTRACE[0m] [kernel] [CPU0]   Module[193]: name='/share/cursors/future/wait-14.svg' cmdline='' size=10126 bytes
[37175964210] [[35mTRACE[0m] [kernel] [CPU0]   Module[194]: name='/share/cursors/future/wait-15.svg' cmdline='' size=8991 bytes
[37176506796] [[35mTRACE[0m] [kernel] [CPU0]   Module[195]: name='/share/cursors/future/wait-16.svg' cmdline='' size=8978 bytes
[37177088685] [[35mTRACE[0m] [kernel] [CPU0]   Module[196]: name='/share/cursors/future/wait-17.svg' cmdline='' size=7842 bytes
[37177968465] [[35mTRACE[0m] [kernel] [CPU0]   Module[197]: name='/share/cursors/future/wait-18.svg' cmdline='' size=7829 bytes
[37178953944] [[35mTRACE[0m] [kernel] [CPU0]   Module[198]: name='/share/cursors/future/wait-19.svg' cmdline='' size=6822 bytes
[37181138577] [[35mTRACE[0m] [kernel] [CPU0]   Module[199]: name='/share/cursors/future/wait-20.svg' cmdline='' size=6679 bytes
[37181800293] [[35mTRACE[0m] [kernel] [CPU0]   Module[200]: name='/share/cursors/future/wait-21.svg' cmdline='' size=5542 bytes
[37182352218] [[35mTRACE[0m] [kernel] [CPU0]   Module[201]: name='/share/cursors/future/wait-22.svg' cmdline='' size=5528 bytes
[37182906222] [[35mTRACE[0m] [kernel] [CPU0]   Module[202]: name='/share/cursors/future/wait-23.svg' cmdline='' size=5546 bytes
[37183485603] [[35mTRACE[0m] [kernel] [CPU0]   Module[203]: name='/share/cursors/future/wait.svg' cmdline='' size=5525 bytes
[37184116101] [[35mTRACE[0m] [kernel] [CPU0]   Module[204]: name='/share/cursors/future/wayland-cursor.svg' cmdline='' size=8222 bytes
[37184696769] [[35mTRACE[0m] [kernel] [CPU0]   Module[205]: name='/share/cursors/future/x-cursor.svg' cmdline='' size=6822 bytes
[37185244602] [[35mTRACE[0m] [kernel] [CPU0]   Module[206]: name='/share/cursors/future/zoom-in.svg' cmdline='' size=5441 bytes
[37185787881] [[35mTRACE[0m] [kernel] [CPU0]   Module[207]: name='/share/cursors/future/zoom-out.svg' cmdline='' size=5403 bytes
[37186329642] [[35mTRACE[0m] [kernel] [CPU0]   Module[208]: name='/share/fonts/unifont.hex' cmdline='' size=8355852 bytes
[37186861866] [[35mTRACE[0m] [kernel] [CPU0]   Module[209]: name='/etc/locale.conf' cmdline='' size=85 bytes
[37187412141] [[35mTRACE[0m] [kernel] [CPU0]   Module[210]: name='/etc/profile' cmdline='' size=68 bytes
[37187940405] [[35mTRACE[0m] [kernel] [CPU0]   Module[211]: name='/etc/motd' cmdline='' size=610 bytes
[37188442797] [[35mTRACE[0m] [kernel] [CPU0]   Module[212]: name='/etc/fstab' cmdline='' size=127 bytes
[37188948093] [[35mTRACE[0m] [kernel] [CPU0]   Module[213]: name='/etc/hostname' cmdline='' size=8 bytes
[37189661883] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[37552911231] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[37554863841] [[34mDEBUG[0m] [kernel] [CPU0] Found init module: /bin/sprout (cmdline: 'init'), loading...
[37556799654] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/sprout' (len=89912, base=0x200000)
[37557655839] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[37587701349] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[37589349666] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[37599732159] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [49, 3b, 6e, 18, 75, 13, 49, 8d]
[37606955529] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20c000 exec=false
[37609089606] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20f000 exec=false
[37613245395] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x210000 filesz=0 memsz=0 align=1
[37619993004] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='load_init_module' elapsed_ticks=63460782 elapsed_us=31730 total_ticks=1233323652 total_us=616661
[37621955184] [[34mDEBUG[0m] [kernel] [CPU0] Warning: Module registry page overflow, truncating list.
[37622789919] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='prepare_boot_registry' elapsed_ticks=1941093 elapsed_us=970 total_ticks=1236122580 total_us=618061
[37623785595] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='map_boot_registry' elapsed_ticks=191961 elapsed_us=95 total_ticks=1237130730 total_us=618565
[37625152092] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='activate_init_aspace' elapsed_ticks=483681 elapsed_us=241 total_ticks=1238407995 total_us=619203
[37626030717] [[34mDEBUG[0m] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[37626826974] [[34mDEBUG[0m] [kernel] [CPU0] Spawning init process...
[37633744104] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb006a9a0 arg=0xffffffffb0020a20
[37665678039] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[37667531517] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=5
[37668494325] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[37669320183] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='spawn_init_task' elapsed_ticks=42535581 elapsed_us=21267 total_ticks=1282655583 total_us=641327
[37670029947] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[37816606872] [[34mDEBUG[0m] [kernel::sched::lifecycle] [CPU0] SCHED: early-boot bringup complete; resuming steady-state SMP scheduling
[37817612646] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='end_bringup' elapsed_ticks=1533246 elapsed_us=766 total_ticks=1430944053 total_us=715472
[37818334818] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[38087127540] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1700214153 elapsed_us=850107
[38088046755] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[38110243215] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[38111322018] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] ENTER_USER: TID=5 entry_pc=200000 user_sp=800000
[38168987439] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpus_online=4
[38179146093] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=0 curr=Some(5) runq=1 runq_avg=2 runq_samples=1 ctxsw=1 idle2busy=0 tick=1 ipi=0 enq=3 deq=1 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[38180681286] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=1 curr=Some(2) runq=0 runq_avg=0 runq_samples=44 ctxsw=0 idle2busy=0 tick=38 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[38181813483] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=2 curr=Some(3) runq=0 runq_avg=0 runq_samples=44 ctxsw=0 idle2busy=0 tick=38 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[38182878723] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=3 curr=Some(4) runq=0 runq_avg=0 runq_samples=42 ctxsw=0 idle2busy=0 tick=36 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[38194631079] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=0
[38200987605] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=20
[38212900572] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[38216534697] [[34mDEBUG[0m] [sprout] [CPU0] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7fef70 rip=0x2032e5 rflags=0x202
[38219429556] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[38220958809] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: About to create Supervisor...
[38222275410] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Listing /bin directory...
[38225624745] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/bin' tid=5
[38260901646] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=3
[38268859497] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[38301662487] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Parsed command line: '    '
[38304649944] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Supervisor created, calling run_forever...
[38306465703] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[38322442290] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[38325229008] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[38327644377] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/run/sprout/shell' tid=5
[38334587346] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/etc/default/shell' tid=5
[38338750824] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/bin/sh' tid=5
[38390416515] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=3
[38393170860] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[38398833132] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[38402061060] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[38432293746] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN: page cache miss for '/bin/sh', caching 127864 bytes
[38437340040] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'sh' (len=127864, base=0x200000)
[38437967403] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[38439035778] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[38439649677] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[38441871963] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [0f, 84, 32, 06, 00, 00, 48, 8b]
[38459137794] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=214000 exec=false
[38462112810] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=217000 exec=false
[38465737398] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x218000 filesz=0 memsz=0 align=1
[38473578363] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /bin/sh
[38475644889] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb00a3d40 arg=0xffffffffb0074560
[38480563770] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=6, applying inserts
[38481505953] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[38482329501] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=6
[38483542944] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[38486807106] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[38502330306] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SETUP_STDIO: stdin=Fd(3) stdout=Fd(4) stderr=Fd(5)
[38520299532] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 6
[38533760463] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Task 6 woken, restoring IRQs
[38534989086] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Done for /bin/sh
[38536954335] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[38542740456] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[38544749991] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=3
[38545505196] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[38546430252] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=6 entry_pc=200000 user_sp=800000
[38546911986] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=4
[38549324121] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=0
[38549958348] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=5
[38551432755] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=100
[38554179708] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[38557455849] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[38563054662] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/run/motd' tid=6
[38564536626] [[34mDEBUG[0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=31515
[38568011097] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/etc/motd' tid=6
[38572912554] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=6
[38579804670] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=610
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
[38586702264] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/etc/profile' tid=6
[38618604057] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=6
[38630895831] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=97
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [38637294894] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=6
[?25h[38640217341] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[38727671136] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[38731930116] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[38747604291] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/cambium' (len=144768, base=0x200000)
[38751495948] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[38755824525] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[38756758425] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[38765114322] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [8d, 3c, f0, 48, 83, c7, 70, 48]
[38786879439] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=217000 exec=false
[38789014110] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[38790553956] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x21b000 filesz=0 memsz=0 align=1
[38799849627] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb00bd780 arg=0xffffffffb00ad560
[38802052476] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[38802882294] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=7
[38803998354] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[38819003718] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 7 ��� task '/bin/cambium' (pid=7 from boot module)
[38826572037] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[38826724860] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned cambium (PID=7)
[38827749444] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=7 entry_pc=200000 user_sp=800000
[38829323610] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=0
[38831252955] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=100
[38834521968] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[38835264402] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[38838054684] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Deferring netd until /dev/net/virtio0/rx is ready...
[38847519414] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[38850572937] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[38853464727] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[38856465087] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[38858726016] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[38870825466] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[38872356930] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers' tid=7
[38872604100] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN: page cache miss for '/bin/bristle', caching 68744 bytes
[38875771077] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_readdir: fd=3 len=4096
[38876727714] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'bristle' (len=68744, base=0x200000)
[38877341712] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[38878022139] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_readdir: natural phase begin offset=0
[38878597626] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[38879387943] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[38881886967] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [cc, fc, ff, ff, 45, 31, ff, e9]
[38887259664] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=209000 exec=false
[38889774528] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20b000 exec=false
[38894095185] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20c000 filesz=0 memsz=0 align=1
[38907527373] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /bin/bristle
[38908224201] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb00e9360 arg=0xffffffffb00d78e0
[38910112626] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=8, applying inserts
[38910794010] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[38911404906] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=8
[38911990722] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[38912475624] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[38914283397] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SETUP_STDIO: stdin=Null stdout=Inherit stderr=Inherit
[38919192609] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 8
[38920859802] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[38921225805] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Task 8 woken, restoring IRQs
[38922747006] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Done for /bin/bristle
[38923633287] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/bristle' TID=8 PID=8
[38924167821] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[38925049878] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=8 entry_pc=200000 user_sp=800000
[38925049878] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ahci_disk' tid=7
[38927088057] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=0
[38928796071] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=100
[38942870175] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[38946232611] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[38947537596] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/session/active_ui' tid=8
[38948246403] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[38951764533] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[38954576199] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/fb0' tid=5
[38955398196] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_open: path='/dev/fb0' len=8 flags=0x0
[38956938570] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] devfs: lookup entry path='fb0' len=3
[38958160692] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] devfs: dynamic registry hit path='fb0'
[38960204283] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_open: resolved node for /dev/fb0 mode=0o20666 size=32 ino=4
[38961127392] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_open: process info present for /dev/fb0
[38961187353] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=3
[38962167354] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_open: handle_table.open('/dev/fb0') -> 3
[38965739670] [[35mTRACE[0m] [bristle] [CPU2] bristle: active_ui set to 'bloom'
[38971332807] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/run/bristle/pid' tid=8
[38978154963] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] FbNode::read: off=0 n=32 buf_len=32 total=32
[38981000520] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=3
[38984725527] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[38990180526] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/drivers/display_virtio_gpu' tid=5
[38997776499] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=3
[39000320667] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[39002975715] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[39009985740] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=4096
[39011933895] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(0)
[39016466181] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=3
[39018595374] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=1 port_id=PortId(0) mode=Write
[39019344738] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[39023153862] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=2 port_id=PortId(0) mode=Read
[39025587216] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=4096
[39026985822] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(1)
[39028150293] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=1 port_id=PortId(1) mode=Write
[39028495704] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[39029187615] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=2 port_id=PortId(1) mode=Read
[39030186360] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=4096
[39030850980] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(2)
[39031747392] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=4096
[39032442438] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(3)
[39033201702] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=3 port_id=PortId(2) mode=Write
[39033446628] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(3) mode=Write
[39034565361] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=4 port_id=PortId(3) mode=Read
[39034592157] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=4 port_id=PortId(2) mode=Read
[39039550671] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/run/bristle/kbd_in' tid=8
[39047013522] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=3
[39048865548] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/run/bristle/mouse_in' tid=8
[39049684047] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[39054002724] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=3
[39056833827] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[39057767199] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/proc/self/inbox' tid=8
[39062427987] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[39068174937] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=2 -> fd=4 node=0xffffffffb00d79b0 port=0xffffffffb00d8f10
[39071375211] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=4 -> fd=5 node=0xffffffffb01f93f0 port=0xffffffffb00f8f70
[39073733886] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[39073961421] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[39106077318] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN: page cache miss for '/drivers/display_virtio_gpu', caching 147560 bytes
[39108245583] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'display_virtio_gpu' (len=147560, base=0x200000)
[39109186446] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[39110975970] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[39111911916] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[39115261812] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [00, 00, 00, ba, 12, 00, 00, 00]
[39140019006] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=216000 exec=false
[39144378801] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[39146666328] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x21c000 filesz=0 memsz=0 align=1
[39154955466] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/display_virtio_gpu
[39155752482] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb022daa0 arg=0xffffffffb021d880
[39157376346] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=9, applying inserts
[39157958004] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[39158508642] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=9
[39159069939] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[39159589161] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[39162240216] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SETUP_STDIO: stdin=Null stdout=Inherit stderr=Inherit
[39166305948] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 9
[39167466987] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Task 9 woken, restoring IRQs
[39168277566] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Done for /drivers/display_virtio_gpu
[39169405572] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/drivers/display_virtio_gpu' TID=9 PID=9
[39170634756] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x3
[39171616572] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=9 entry_pc=200000 user_sp=800000
[39173234166] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=0
[39175096389] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=100
[39181884819] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[39185388231] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Starting VFS-native VirtIO GPU driver...
[39187095849] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: boot_arg=3
[39189017472] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Mapping bootstrap memfd 3 size=4096...
[39192977406] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: vm_map success at 0x400000001000
[39194806794] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Recovered handles: req_read=2, resp_write=3, svc=0, id=322371585
[39196702479] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3, svc=0, id=322371585)
[39198106101] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Scanning /sys/devices for PCI GPU...
[39199382145] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices' tid=9
[39202256346] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices'
[39206015607] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[39207549480] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[39207872847] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_readdir: fd=4 len=4096
[39209452359] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_readdir: natural phase begin offset=0
[39212016888] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[39214746714] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[39214755129] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU3] sysfs: readdir found 12 slots
[39214705926] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0 mounted=false, service_ready=false)...
[39216460635] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ata_disk' tid=7
[39223980477] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready for bloom yet
[39224099013] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU3] sysfs: readdir wrote 173 bytes
[39227800062] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[39229108281] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[39231263544] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[39233645616] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[39236949378] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/proc/self/inbox' tid=5
[39243649566] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[39268317066] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/platform-fb000000/vendor' tid=9
[39271243407] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/platform-fb000000/vendor'
[39275898090] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'platform-fb000000/vendor'
[39282184590] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[39285872373] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/platform-fb000000/device' tid=9
[39288049878] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/platform-fb000000/device'
[39289893192] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'platform-fb000000/device'
[39293129040] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[39294992847] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/platform-fb000000/class' tid=9
[39297131577] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/platform-fb000000/class'
[39299197278] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'platform-fb000000/class'
[39302355411] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[39304292544] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/vendor' tid=9
[39306397581] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:00.0/vendor'
[39308224923] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:00.0/vendor'
[39314025927] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[39315832050] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/device' tid=9
[39317910390] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:00.0/device'
[39319651800] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:00.0/device'
[39322811814] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[39324542994] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/class' tid=9
[39326692185] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:00.0/class'
[39328397955] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:00.0/class'
[39331483290] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[39333144510] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/vendor' tid=9
[39335203578] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/vendor'
[39338861430] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:01.0/vendor'
[39341865453] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[39343496610] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/device' tid=9
[39345477699] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/device'
[39347349426] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:01.0/device'
[39351976290] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[39353056479] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/class' tid=9
[39354451059] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/class'
[39355425186] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:01.0/class'
[39357204183] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[39363677100] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: task 9 claimed device 'pci-0000:00:01.0' (handle 0)
[39365169360] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/virtio/common_bar' tid=9
[39366351255] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/common_bar'
[39372859647] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[39375282177] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/virtio/common_offset' tid=9
[39376656594] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/common_offset'
[39381142779] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[39382761891] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/virtio/notify_bar' tid=9
[39384037407] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/notify_bar'
[39387782379] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[39388873458] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/virtio/notify_offset' tid=9
[39389146368] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[39390841248] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/notify_offset'
[39391846725] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[39392844249] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/chime' tid=7
[39394528932] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[39395610639] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/virtio/notify_multiplier' tid=9
[39396829263] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/notify_multiplier'
[39399981555] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[39401650728] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[39404720817] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: mapping BAR2 (phys=0xc000000000, size=0x4000) for task 9
[39407364381] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10000000
[39410546934] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: DMA alloc 1 pages phys=0x2329000 -> user_va=0x10004000
[39416585241] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[39422039415] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: DMA alloc 4 pages phys=0x232a000 -> user_va=0x10005000
[39424392282] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[39425185239] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Virgl 3D not supported, using 2D only
[39425898897] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/firmware/framebuffer' tid=9
[39427149366] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='firmware/framebuffer'
[39432084681] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[39436134969] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=1
[39564877869] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[39567747549] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[39569282148] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/display_bootfb' tid=7
[39581963355] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[39583578309] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[39585528741] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[39587056080] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[39588571209] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0 mounted=false, service_ready=false)...
[39590500191] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready for bloom yet
[39591960672] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[39593387724] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[39595308588] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[39596676141] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[39761144115] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[39764797083] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[39766351944] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/display_virtio_gpu' tid=7
[39932311188] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[39934937427] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[39939226767] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[39941638770] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[39943984740] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0 mounted=false, service_ready=false)...
[39947118090] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready for bloom yet
[39949417629] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[39951757626] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[39952554411] [[34mDEBUG[0m] [kernel::syscall::handlers::memory] [CPU3] sys_vm_map: large backing file mapping offset=0 len=8294400 total_size=8294400
[39955338984] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[39957343140] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[39959297961] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[39962300961] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[39963952083] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/hdaudio' tid=7
[40047909231] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: frame pool ready (1 buffer)
[40053067527] [[34mDEBUG[0m] [kernel::ipc] [CPU3] CREATE_PORT: capacity=65536
[40054141512] [[34mDEBUG[0m] [kernel::ipc] [CPU3] CREATE_PORT: appended id=PortId(4)
[40055137584] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU3] ALLOC_HANDLE: handle=5 port_id=PortId(4) mode=Write
[40056060759] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU3] ALLOC_HANDLE: handle=6 port_id=PortId(4) mode=Read
[40057985121] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU3] BRIDGE: handle=5 -> fd=5 node=0xffffffffb00ad570 port=0xffffffffb010a1f0
[40060943604] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/proc/5/inbox' tid=9
[40092205626] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU3] SENDMSG: thing=6 data_len=44 caps_len=1 node=Pointer { addr: 0xffffffffb010a510, metadata: DynMetadata(0xffffffff802c6ec0) }
[40100231556] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_BIND_READY (ID: 322371585)
[40101483939] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Waiting for BIND_ASSIGNED...
[40110843168] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU0] RECVMSG: thing=4 data_len=44 caps_len=1 node=Pointer { addr: 0xffffffffb00d78f0, metadata: DynMetadata(0xffffffff802c6ec0) }
[40121117058] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: BIND_READY from instance_id=0x13370001
[40123519821] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Registering DISPLAY_CARD at /dev/display/card0 (handle=5)
[40136929569] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=262400
[40137989364] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[40138082820] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(5)
[40140297879] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[40141191915] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=7 port_id=PortId(5) mode=Write
[40141351998] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/pci_stubd' tid=7
[40154908596] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] vfs: mounted userland provider at /dev/display/card0 (flags: 0x0) port=0xffffffffb010a1f0
[40159760355] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[40163108799] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Handshake complete — task 'display' marked ready
[40172697576] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Sent MSG_BIND_ASSIGNED to display pid 9 (res=Ok(()))
[40174468224] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[40176116574] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU3] SENDMSG: thing=6 data_len=44 caps_len=0 node=Pointer { addr: 0xffffffffb010a510, metadata: DynMetadata(0xffffffff802c6ec0) }
[40178082483] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_SERVICE_READY.
[40178987475] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU3] BRIDGE: handle=2 -> fd=7 node=0xffffffffb00f9590 port=0xffffffffb00f3870
[40179920880] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU3] BRIDGE: handle=3 -> fd=8 node=0xffffffffb011b3f0 port=0xffffffffb00f71f0
[40180793829] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU3] BRIDGE: handle=6 -> fd=9 node=0xffffffffb011b4b0 port=0xffffffffb010a1f0
[40181454225] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[40182863424] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[40190221302] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU0] RECVMSG: thing=4 data_len=44 caps_len=0 node=Pointer { addr: 0xffffffffb00d78f0, metadata: DynMetadata(0xffffffff802c6ec0) }
[40221024525] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpus_online=4
[40223161968] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=0 curr=Some(5) runq=1 runq_avg=1 runq_samples=84 ctxsw=17 idle2busy=0 tick=48 ipi=1 enq=90 deq=88 wake=5 lock_miss=0 lock_pending=0 lock_blocked=0
[40226900835] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=1 curr=Some(7) runq=0 runq_avg=0 runq_samples=71 ctxsw=1 idle2busy=1 tick=66 ipi=1 enq=6 deq=5 wake=2 lock_miss=0 lock_pending=0 lock_blocked=0
[40228263636] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=2 curr=Some(3) runq=0 runq_avg=0 runq_samples=109 ctxsw=2 idle2busy=1 tick=87 ipi=1 enq=3 deq=2 wake=2 lock_miss=0 lock_pending=0 lock_blocked=0
[40229561988] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=3 curr=Some(9) runq=0 runq_avg=0 runq_samples=182 ctxsw=5 idle2busy=3 tick=76 ipi=3 enq=109 deq=108 wake=6 lock_miss=0 lock_pending=0 lock_blocked=0
[40242512574] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: SERVICE_READY from instance_id=0x13370001
[40245289392] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Service 'display' reported ready
[40247717136] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[40250155044] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[40339898841] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[40342341138] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[40343415783] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ps2_kbd' tid=7
[40529335836] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[40532800407] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[40534435194] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ps2_mouse' tid=7
[40560416787] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[40562689497] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[40564907031] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[40566782916] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[40568268972] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0 mounted=true, service_ready=true)...
[40571652891] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/bloom' (len=462464, base=0x200000)
[40572275205] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[40573457199] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[40574206827] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[40576633383] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [29, 44, 24, 60, 48, 8b, 44, 24]
[40624700985] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=253000 exec=false
[40628952672] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=25a000 exec=false
[40631531556] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x25d000 filesz=0 memsz=0 align=1
[40640613651] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb013b660 arg=0xffffffffb00f91e0
[40643054892] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[40643872170] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=10
[40644777492] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[40650727359] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 10 �� task '/bin/bloom' (pid=10 from boot module)
[40661340951] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[40664875581] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[40668741135] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[40670984970] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[40672974507] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[40752969741] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[40771001073] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[40773061725] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=10 entry_pc=200000 user_sp=800000
[40777718421] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=0
[40779864345] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=100
[40784480154] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[40786128702] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[40790679435] [[34mDEBUG[0m] [bloom] [CPU1] bloom: connect try 0...
[40791986235] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/dev/display/card0' tid=10
[40795706391] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=Lookup len=4 tid=10
[40813215894] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[40815013008] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/rtc_cmos' tid=7
[40827552414] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=Lookup
[40830897360] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: vfs_lookup path=''
[40833005070] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=Lookup req_id=1
[40838056710] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=Lookup req_id=1
[40986912450] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=Lookup id=1 -> OK(8)
[40987654389] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[40990501068] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[40992215583] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[40994138394] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[40996353750] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[40998451461] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[41000077338] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[41001667872] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[41017161768] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=48 tid=10
[41025286368] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[41029049919] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_GET_INFO requested
[41030590887] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: Returning dimensions 1920x1080
[41031999690] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=2
[41035169571] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=2
[41045015550] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=2 -> OK(48)
[41052034815] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[41082480681] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1920x1080 @ 60000mHz
[41084137017] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[41086029303] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[41087541297] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[41093714178] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=65536
[41094590031] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(6)
[41095752291] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=5 port_id=PortId(6) mode=Write
[41096794332] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=6 port_id=PortId(6) mode=Read
[41101534320] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/services/bloom' tid=10
[41109418680] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[41197386648] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[41200574811] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[41202410799] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/rtl8168d' tid=7
[41310399504] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/components' tid=10
[41315194734] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[41318794836] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[41319011745] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/status' tid=10
[41322413220] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[41324962965] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[41325153210] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[41327143077] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/lib/libpistil.so' tid=10
[41330122911] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[41332389747] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[41334892434] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[41337360471] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[41339578137] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[41607457617] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[41659482645] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[41661986784] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[41664338265] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[41667267939] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[41669549889] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[41672135208] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[41677253409] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[41679728508] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[41681997291] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[41763929130] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[41766571803] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[41768013045] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/virtio_netd' tid=7
[41962944804] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[41966129733] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[41967753663] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/virtio_sound' tid=7
[42008203743] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[42010780152] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[42013133679] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[42015521592] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[42017901948] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[42020482680] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[42022917816] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[42025047933] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[42163188738] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[42166569060] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[42169347066] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[42170971260] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/device_snapshot' tid=7
[42173329440] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='device_snapshot'
[42184009296] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[42192496665] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:00.0 kind=pci_device vendor=0x8086 device=0x29c0 class=0x060000 present=true
[42195523359] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:01.0 kind=pci_device vendor=0x1af4 device=0x1050 class=0x030000 present=true
[42198198735] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:02.0 kind=pci_device vendor=0x1af4 device=0x1000 class=0x020000 present=true
[42199980537] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: isolation mode skipping driver '/drivers/virtio_netd' class=Net for pci-0000:00:02.0
[42201130587] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:03.0 kind=pci_device vendor=0x1af4 device=0x1059 class=0x040100 present=true
[42203060097] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: isolation mode skipping driver '/drivers/virtio_sound' class=Audio for pci-0000:00:03.0
[42204443655] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:1f.0 kind=pci_device vendor=0x8086 device=0x2918 class=0x060100 present=true
[42207478401] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:1f.2 kind=pci_device vendor=0x8086 device=0x2922 class=0x010601 present=true
[42210289407] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.2/status' tid=7
[42212551953] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/status'
[42215352861] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/status'
[42218470734] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[42238977297] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: page cache miss for '/drivers/ahci_disk', caching 53784 bytes
[42240882915] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'ahci_disk' (len=53784, base=0x200000)
[42241843050] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[42243574164] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[42244633431] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=200000 exec=true
[42249266862] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   COPIED at 0x201420: [24, 0c, e8, 59, 0e, 00, 00, bf]
[42253691997] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=207000 exec=false
[42255185610] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=208000 exec=false
[42257100468] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x209000 filesz=0 memsz=0 align=1
[42267873252] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[42268688121] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/ahci_disk
[42269908395] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb0168880 arg=0xffffffffb0127700
[42288435717] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=11, applying inserts
[42289549797] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[42290417400] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=11
[42291527619] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[42292310841] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[42296484582] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[42302027757] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 11
[42303770718] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 11 woken, restoring IRQs
[42304701285] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/ahci_disk
[42305983038] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=11 PID=11
[42306102531] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x3
[42307630860] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=11 entry_pc=201000 user_sp=800000
[42309156087] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=11 buf_len=0
[42312033984] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/ahci_disk for pci-0000:00:1f.2 (entry='thingos_driver_start_safe', pid=11)
[42314499414] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=11 buf_len=100
[42317500533] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Starting AHCI VFS driver
[42317670087] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: sent DRIVER_READY to Sprout (pid=5) for driver pid=11
[42319217853] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:1f.3 kind=pci_device vendor=0x8086 device=0x2930 class=0x0c0500 present=true
[42320524158] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=isa-0070 kind=dev.rtc.Cmos vendor=0x0000 device=0x0000 class=0x000000 present=true
[42323913588] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0070/status' tid=7
[42326353575] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/status'
[42328420002] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU0] RECVMSG: thing=4 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00d78f0, metadata: DynMetadata(0xffffffff802c6ec0) }
[42328984797] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/status'
[42332240181] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=4
[42337243740] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: DRIVER_READY from PID 11 — no matching ManagedTask (already exited?)
[42340143879] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[42344712663] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: page cache miss for '/drivers/rtc_cmos', caching 45904 bytes
[42346302174] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'rtc_cmos' (len=45904, base=0x200000)
[42347087013] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[42348544623] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[42349161162] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=200000 exec=true
[42351111396] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 'pci-0000:00:1f.2' (handle 1)
[42351506868] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   COPIED at 0x201420: [0f, 84, ea, 03, 00, 00, b8, 00]
[42354367539] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=206000 exec=false
[42355447068] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=207000 exec=false
[42357301767] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x208000 filesz=0 memsz=0 align=1
[42358281768] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[42359247711] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: mapping BAR5 (phys=0x80880000, size=0x1000) for task 11
[42360097758] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10009000
[42362465244] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: DMA alloc 1 pages phys=0x2dcc000 -> user_va=0x1000a000
[42366026307] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[42366891633] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/rtc_cmos
[42367619283] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=65536
[42367788375] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb01abc60 arg=0xffffffffb016fae0
[42368312712] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(7)
[42369145335] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=1 port_id=PortId(7) mode=Write
[42369702672] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=2 port_id=PortId(7) mode=Read
[42370587204] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=12, applying inserts
[42371271360] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[42372617298] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=12
[42373263174] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[42373646667] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[42375603336] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[42376271256] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=262400
[42377187402] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(8)
[42379343754] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(8) mode=Write
[42381947157] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 12
[42384373284] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 12 woken, restoring IRQs
[42385081761] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] vfs: mounted userland provider at /dev/storage/atapi2 (flags: 0x0) port=0xffffffffb017c870
[42386239632] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x4
[42385266957] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/rtc_cmos
[42402704751] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=12 entry_pc=201000 user_sp=800000
[42403585620] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/rtc_cmos' TID=12 PID=12
[42404623503] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=12 buf_len=0
[42405466917] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Mounted atapi2 at /dev/storage/atapi2
[42406424874] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=12 buf_len=100
[42406758339] [[32mINFO [0m] [ahci_disk] [CPU2] AHCI: Provider loop online at /dev/storage/atapi2
[42408859845] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[42410524761] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[42415386816] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil nine-slice shadow renderer loaded
[42417088626] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/desktop/theme' tid=10
[42421414761] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/desktop/theme' tid=10
[42425856264] [[34mDEBUG[0m] [rtc_cmos] [CPU3] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffef0 rip=0x201c27 rflags=0x206
[42427470690] [[34mDEBUG[0m] [rtc_cmos] [CPU3] Starting... arg=4
[42428150952] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[42431071221] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Solarized Warm
[42485155416] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: task 12 claimed device 'isa-0070' (handle 2)
[42487707768] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: claimed /sys/devices/isa-0070 (handle=2)
[42535847376] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: 2026-05-01 01:14:05 = 1777598045 unix_secs
[42537715638] [[32mINFO [0m] [kernel::time] [CPU3] System clock anchored: unix_secs=1777598045, mono_ns=21268631076, offset=1777598023731368924ns
[42539193708] [[32mINFO [0m] [rtc_cmos] [CPU3] RTC: System clock anchored to 1777598045 unix_secs
[42540872088] [[34mDEBUG[0m] [rtc_cmos] [CPU3] RTC: Entering maintenance loop.
[42564267273] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 01:14:05.012653734 unix_secs=1777598045.012653734
[42667594893] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[42670251261] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[42672629340] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[42675171627] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[42677529147] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[42682583427] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/wayland_hello' (len=82360, base=0x200000)
[42683566233] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[42685397403] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[42686361696] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[42689728059] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [48, 83, c4, 58, c3, cc, cc, cc]
[42698063958] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20c000 exec=false
[42700042374] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20e000 exec=false
[42702684915] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x210000 filesz=0 memsz=0 align=1
[42718366284] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb01dbc60 arg=0xffffffffb016fa00
[42720911805] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[42721776570] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=13
[42722721558] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[42729432504] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 13 → task '/bin/wayland_hello' (pid=13 from boot module)
[42741140937] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=13)
[42744616662] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[42748376418] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/clock' (len=82936, base=0x200000)
[42749359587] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[42751161255] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[42752185377] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[42760511079] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [48, 83, c4, 58, c3, cc, cc, cc]
[42770239182] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20c000 exec=false
[42772422858] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20e000 exec=false
[42778443939] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x210000 filesz=0 memsz=0 align=1
[42787333545] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb01f09c0 arg=0xffffffffb018bbe0
[42789765381] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[42790636482] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=14
[42791689743] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[42798193713] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 14 → task '/bin/clock' (pid=14 from boot module)
[42802433421] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[42803322276] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=14 entry_pc=200000 user_sp=800000
[42804989040] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=14 buf_len=0
[42806936106] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=14 buf_len=100
[42814394238] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=14)
[42816847029] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=6
[42817708230] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[42820846827] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[42897788406] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[42898955088] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=13 entry_pc=200000 user_sp=800000
[42902206512] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=13 buf_len=0
[42904225353] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=13 buf_len=100
[42907426254] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=6
[42910902738] [[34mDEBUG[0m] [kernel::syscall::handlers::memory] [CPU1] sys_vm_map: large backing file mapping offset=0 len=8294400 total_size=8294400
[42977964909] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=6
[42995829195] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=88 tid=10
[43001775729] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[43002659568] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/rtc_cmos for isa-0070 (entry='thingos_driver_start_safe', pid=12)
[43003319799] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_IMPORT_BUFFER requested: memfd=10, size=1920x1080
[43004927031] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: sent DRIVER_READY to Sprout (pid=5) for driver pid=12
[43005644220] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=isa-0060 kind=drv.Ps2Keyboard vendor=0x0000 device=0x0000 class=0x000000 present=true
[43005973197] [[34mDEBUG[0m] [kernel::syscall::handlers::memory] [CPU3] sys_vm_map: large backing file mapping offset=0 len=8294400 total_size=8294400
[43006586832] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0060/status' tid=7
[43008005799] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/status'
[43010027082] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/status'
[43012826175] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=5
[43019569791] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU0] RECVMSG: thing=4 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00d78f0, metadata: DynMetadata(0xffffffff802c6ec0) }
[43024899423] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: page cache miss for '/drivers/ps2_kbd', caching 72264 bytes
[43026351984] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'ps2_kbd' (len=72264, base=0x200000)
[43027157811] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[43028896086] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[43029983073] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=200000 exec=true
[43033295646] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   COPIED at 0x201420: [83, ff, 06, 77, 46, 48, 8d, 15]
[43039426584] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=209000 exec=false
[43042000155] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: DRIVER_READY from PID 12 �� no matching ManagedTask (already exited?)
[43042117140] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=20c000 exec=false
[43044316194] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x20d000 filesz=0 memsz=0 align=1
[43047549303] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[43053669483] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[43054712844] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/ps2_kbd
[43056643245] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb025ca00 arg=0xffffffffb01f6560
[43058984859] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=15, applying inserts
[43059709803] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[43060485138] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=15
[43061227770] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[43061798076] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[43066605087] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[43073320257] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 15
[43075049292] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 15 woken, restoring IRQs
[43075648110] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/ps2_kbd
[43076560263] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ps2_kbd' TID=15 PID=15
[43081266855] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=6
[43085205405] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/ps2_kbd for isa-0060 (entry='thingos_driver_start_safe', pid=15)
[43087733271] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: sent DRIVER_READY to Sprout (pid=5) for driver pid=15
[43088533686] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=isa-0064 kind=drv.Ps2Mouse vendor=0x0000 device=0x0000 class=0x000000 present=true
[43089582987] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0064/status' tid=7
[43090788345] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0064/status'
[43092450159] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0064/status'
[43097299476] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=6
[43115479671] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: page cache miss for '/drivers/ps2_mouse', caching 71096 bytes
[43117367997] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'ps2_mouse' (len=71096, base=0x200000)
[43118306649] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[43119904278] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[43120819962] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=200000 exec=true
[43124260971] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   COPIED at 0x201420: [83, f8, 02, 75, 1a, b8, 0b, 10]
[43130288883] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=209000 exec=false
[43130541003] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU0] RECVMSG: thing=4 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00d78f0, metadata: DynMetadata(0xffffffff802c6ec0) }
[43130869617] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=6
[43133014089] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=20c000 exec=false
[43134698739] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x20d000 filesz=0 memsz=0 align=1
[43140435921] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: DRIVER_READY from PID 15 — no matching ManagedTask (already exited?)
[43143584286] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[43144164327] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[43145191023] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/ps2_mouse
[43146291375] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb02850c0 arg=0xffffffffb01f5c20
[43149354633] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=16, applying inserts
[43150569726] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[43151452773] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=16
[43152663477] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[43153316481] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[43157304861] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[43164053295] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 16
[43166385603] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 16 woken, restoring IRQs
[43167189417] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/ps2_mouse
[43168100217] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ps2_mouse' TID=16 PID=16
[43170319830] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/ps2_mouse for isa-0064 (entry='thingos_driver_start_safe', pid=16)
[43173511095] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: sent DRIVER_READY to Sprout (pid=5) for driver pid=16
[43174827531] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=isa-01f0 kind=dev.storage.ata vendor=0x0000 device=0x0000 class=0x000000 present=true
[43178610090] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/proc/15/job_observer' tid=7
[43194229683] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU0] RECVMSG: thing=4 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb00d78f0, metadata: DynMetadata(0xffffffff802c6ec0) }
[43199926935] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: DRIVER_READY from PID 16 — no matching ManagedTask (already exited?)
[43203038868] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[43206409587] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[43206475719] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x5
[43208099121] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=15 entry_pc=201000 user_sp=800000
[43209082884] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/proc/16/job_observer' tid=7
[43210628802] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=15 buf_len=0
[43217760663] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=15 buf_len=100
[43222803063] [[34mDEBUG[0m] [ps2_kbd] [CPU3] ps2_kbd: online — waiting for bristle pid
[43223118345] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[43224208071] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/run/bristle/pid' tid=15
[43225470420] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/proc/12/job_observer' tid=7
[43233512289] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/run/bristle/pid' tid=15
[43239968739] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[43241424336] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/proc/11/job_observer' tid=7
[43242920292] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] DISP: imported buffer as ID=1
[43247852538] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=3
[43250141220] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=3
[43252464024] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[43254336015] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/proc/self/inbox' tid=7
[43259118210] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices'
[43263739497] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=7
[43266539580] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x6
[43266824238] [[32mINFO [0m] [ps2_kbd] [CPU3] ps2_kbd: bristle pid=8
[43267714644] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=16 entry_pc=201000 user_sp=800000
[43269030354] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=16 buf_len=0
[43269939273] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x21
[43270951812] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=16 buf_len=100
[43271449386] [[34mDEBUG[0m] [ps2_kbd] [CPU3] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[43272458097] [[34mDEBUG[0m] [ps2_kbd] [CPU3] ps2_kbd: using interrupt-driven loop (IRQ vector 0x21)
[43276260984] [[34mDEBUG[0m] [ps2_mouse] [CPU1] ps2_mouse: online — waiting for bristle pid
[43277658996] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/run/bristle/pid' tid=16
[43282544778] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/run/bristle/pid' tid=16
[43286243088] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=8
[43289016408] [[32mINFO [0m] [ps2_mouse] [CPU1] ps2_mouse: bristle pid=8
[43290239751] [[34mDEBUG[0m] [ps2_mouse] [CPU1] ps2_mouse: enabling aux port
[43292573544] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=6
[43300158033] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=6
[43323840153] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=3 -> OK(12)
[43327861005] [[34mDEBUG[0m] [bloom::display] [CPU1] bloom: imported buffer 1920x1080 as ID=1
[43329300168] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/desktop/wallpaper' tid=10
[43331817276] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/desktop/wallpaper' tid=10
[43336112424] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=8
[43337526177] [[32mINFO [0m] [bloom] [CPU1] bloom: initial wallpaper configured /share/wallpapers/flower.png
[43348395288] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/share/cursors/future/default.svg' tid=10
[43365518493] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=9
[43459119825] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=6
[43496376528] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=6
[43524052242] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=88 tid=10
[43524707721] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[43528103751] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[43530512619] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[43532844333] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[43533406455] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[43534776813] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_IMPORT_BUFFER requested: memfd=11, size=96x96
[43535161890] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[43538264385] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[43538993223] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] DISP: imported buffer as ID=2
[43540548216] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x47 (is_aux=false)
[43541373777] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=4
[43543509372] [[34mDEBUG[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 keyboard NMI fired (count=1)
[43544157525] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0x47 (is_aux=false)
[43544471058] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=4
[43546414989] [[34mDEBUG[0m] [ps2_mouse] [CPU1] ps2_mouse: controller cfg already correct (0x47)
[43547525835] [[34mDEBUG[0m] [ps2_mouse] [CPU1] ps2_mouse: sending RESET (0xFF)
[43551462636] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xfa (is_aux=true)
[43552217874] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xaa (is_aux=true)
[43552610013] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0xfa (is_aux=true)
[43552865070] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x00 (is_aux=true)
[43553950143] [[34mDEBUG[0m] [ps2_mouse] [CPU1] ps2_mouse: reset ACK received (0xfa)
[43558132299] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0xaa (is_aux=true)
[43558979640] [[35mTRACE[0m] [ps2_kbd] [CPU3] ps2_kbd: yield on AUX data (mouse packet)
[43566873504] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=4 -> OK(12)
[43568759454] [[34mDEBUG[0m] [bloom::display] [CPU1] bloom: imported buffer 96x96 as ID=2
[43570510731] [[32mINFO [0m] [bloom::render] [CPU1] bloom: cursor ready buffer=2 size=96x96 hotspot=21,12
[43570790109] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[43575898608] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[43577121918] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=65536
[43577696910] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(9)
[43578374301] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=7 port_id=PortId(9) mode=Write
[43578849039] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=8 port_id=PortId(9) mode=Read
[43581732051] [[32mINFO [0m] [bloom] [CPU1] bloom: watching wallpaper config /session/desktop/wallpaper
[43583661792] [[32mINFO [0m] [bloom] [CPU1] bloom: watching theme config /session/desktop/theme
[43585491576] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU1] BRIDGE: handle=6 -> fd=11 node=0xffffffffb028ed90 port=0xffffffffb01263f0
[43588803357] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU1] BRIDGE: handle=8 -> fd=12 node=0xffffffffb01f6dd0 port=0xffffffffb028f9f0
[43593858033] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/desktop/wallpaper' tid=10
[43598589243] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=13
[43601598348] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/desktop/theme' tid=10
[43605142911] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=13
[43611089940] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=65536
[43611713640] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(10)
[43612265004] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=9 port_id=PortId(10) mode=Write
[43612740204] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=10 port_id=PortId(10) mode=Read
[43617009942] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=65536
[43617593448] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(11)
[43618218732] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=11 port_id=PortId(11) mode=Write
[43618872858] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=12 port_id=PortId(11) mode=Read
[43620118245] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU1] BRIDGE: handle=10 -> fd=13 node=0xffffffffb01f6df0 port=0xffffffffb028ffd0
[43621174311] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU1] BRIDGE: handle=12 -> fd=14 node=0xffffffffb028fe90 port=0xffffffffb02a19f0
[43623893115] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=6
[43637057442] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Task 17 (user thread) assigned to CPU 2
[43643174157] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[43644085749] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=17
[43645028394] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[43647254772] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Sending post-unlock Resched IPI to CPU 2 for task 17
[43649225565] [[32mINFO [0m] [bloom] [CPU1] bloom: Wayland server thread spawned
[43651746468] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: service loop started
[43652413167] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: starting
[43653112866] [[32mINFO [0m] [bloom::loop_types] [CPU1] bloom: output0 1920x1080 @ 60000mHz ready
[43653660039] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/proc/self/inbox' tid=17
[43665439983] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=6
[43671621741] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/desktop/wallpaper' tid=10
[43673807100] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: listening on /run/wayland-0
[43675962792] [[32mINFO [0m] [bloom::wayland] [CPU2] wayland-server: advertising globals wl_compositor wl_shm xdg_wm_base wl_seat wl_output wl_subcompositor wl_data_device_manager zwp_linux_dmabuf_v1
[43676749380] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=17
[43678148019] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/desktop/wallpaper' tid=10
[43682987502] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=17
[43685265261] [[32mINFO [0m] [bloom::services::wallpaper] [CPU1] bloom: preparing background /share/wallpapers/flower.png
[43798303197] [[32mINFO [0m] [clock] [CPU2] clock: connected to /run/wayland-0
[43800834000] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/lib/libpistil.so' tid=14
[43916512728] [[34mDEBUG[0m] [bloom::wayland] [CPU2] wayland-server: new client fd=17
[43921170216] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[43923349371] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[43925707386] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[43928019135] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[43929933927] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[43932112983] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[43933691934] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[43935032163] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[44155975083] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=7
[44198252208] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: connected to /run/wayland-0
[44200131195] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/lib/libpistil.so' tid=13
[44232210726] [[32mINFO [0m] [clock] [CPU2] clock: pistil DSEG7 text renderer loaded with /share/fonts/DSEG7Classic-Regular.ttf
[44249039340] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[44251448604] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[44253725340] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[44256055998] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[44258379363] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[44261144268] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[44263774533] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[44266034637] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[44271841416] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpus_online=4
[44272762017] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=0 curr=Some(0) runq=0 runq_avg=1 runq_samples=313 ctxsw=48 idle2busy=0 tick=157 ipi=3 enq=320 deq=319 wake=23 lock_miss=0 lock_pending=0 lock_blocked=0
[44274145014] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=1 curr=Some(13) runq=2 runq_avg=0 runq_samples=109 ctxsw=32 idle2busy=1 tick=106 ipi=3 enq=54 deq=51 wake=24 lock_miss=0 lock_pending=0 lock_blocked=0
[44275449306] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=2 curr=Some(14) runq=1 runq_avg=0 runq_samples=259 ctxsw=30 idle2busy=12 tick=188 ipi=11 enq=25 deq=23 wake=29 lock_miss=0 lock_pending=0 lock_blocked=0
[44276780757] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=3 curr=Some(4) runq=0 runq_avg=0 runq_samples=546 ctxsw=225 idle2busy=107 tick=183 ipi=97 enq=230 deq=229 wake=220 lock_miss=0 lock_pending=0 lock_blocked=0
[44281081251] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/etc/locale.conf' tid=14
[44287678578] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=7
[44298963357] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 17:14:05 UTC-8 system_unix=1777598045.877060074
[44306302920] [[34mDEBUG[0m] [kernel::syscall::handlers::memory] [CPU1] sys_vm_map: large backing file mapping offset=0 len=8294400 total_size=8294400
[44376510915] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/share/wallpapers/flower.png' tid=10
[44378992944] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=64
[44379884010] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(12)
[44380919781] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=13 port_id=PortId(12) mode=Write
[44381805402] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=14 port_id=PortId(12) mode=Read
[44577820551] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[44580220245] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[44582562948] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[44584808565] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[44587188294] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[44589548817] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[44591639961] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[44593255839] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[44907520581] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[44910068511] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[44912410884] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[44914707684] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[44916983892] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[44919313593] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[44921581254] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[44923700943] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[45194062518] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[45238153125] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[45240690891] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[45243042570] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[45245339502] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[45247670160] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[45250190007] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[45252396981] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[45254471493] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[45408200970] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[45567091713] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[45569716995] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[45571646010] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[45574191696] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[45576767577] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[45579285906] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[45581662269] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[45583855119] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[45896327499] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[45898994724] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[45901442169] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[45904248786] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[45906713952] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[45909369693] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[45911320917] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[45913581120] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[46227495006] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[46229972877] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[46232173317] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[46234503975] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[46237004451] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[46239557100] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[46241771202] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[46243852182] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[46555699575] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[46558081119] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[46560312249] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[46562720721] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[46564980759] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[46567228158] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[46568754276] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[46571516211] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[46644586098] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: pistil text renderer loaded with default /share/fonts/Inter-Regular.ttf
[46653058287] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0x00 (is_aux=true)
[46656490155] [[34mDEBUG[0m] [ps2_mouse] [CPU1] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[46658049735] [[34mDEBUG[0m] [ps2_mouse] [CPU1] ps2_mouse: requesting sample rate 60 Hz
[46663040688] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xfa (is_aux=true)
[46885276152] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[46887790653] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[46890128868] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[46892720028] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[46895039367] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[46897562052] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[46899904920] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[46902035928] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[46954106661] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0xfa (is_aux=true)
[46957875525] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xfa (is_aux=true)
[47019933246] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0xfa (is_aux=true)
[47021652018] [[32mINFO [0m] [ps2_mouse] [CPU1] ps2_mouse: sample rate set to 60 Hz
[47022658650] [[34mDEBUG[0m] [ps2_mouse] [CPU1] ps2_mouse: setting resolution (3)
[47025917730] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xfa (is_aux=true)
[47214890778] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[47217372576] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[47219806557] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[47222222454] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[47224505823] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[47226993957] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[47229295641] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[47231472420] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[47349431118] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0xfa (is_aux=true)
[47354137974] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xfa (is_aux=true)
[47545584999] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[47548139463] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[47550557835] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[47552897370] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[47555329569] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[47557766025] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[47560026954] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[47562166872] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[47728397079] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0xfa (is_aux=true)
[47732884815] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xfa (is_aux=true)
[47733904515] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x00 (is_aux=true)
[47734443207] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0xfa (is_aux=true)
[47735095881] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x03 (is_aux=true)
[47736017835] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0x3c (is_aux=true)
[47737731459] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0x00 (is_aux=true)
[47740572330] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0x03 (is_aux=true)
[47746644924] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0x3c (is_aux=true)
[47748951987] [[34mDEBUG[0m] [ps2_mouse] [CPU1] ps2_mouse: status result = Some(0) Some(3) Some(60)
[47749907700] [[34mDEBUG[0m] [ps2_mouse] [CPU1] ps2_mouse: sending enable command (0xF4)
[47753990460] [[35mTRACE[0m] [bran::arch::x86_64::idt] [CPU0] PS/2 byte received: 0xfa (is_aux=true)
[47758673919] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=88 tid=10
[47765150961] [[32mINFO [0m] [kernel::irq::ps2] [CPU1] PS/2 take_scancode: popped 0xfa (is_aux=true)
[47766568905] [[34mDEBUG[0m] [ps2_mouse] [CPU1] ps2_mouse: enable ACK received (0xfa)
[47786653299] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[47801331270] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_IMPORT_BUFFER requested: memfd=12, size=1920x1080
[47813887407] [[34mDEBUG[0m] [kernel::syscall::handlers::memory] [CPU3] sys_vm_map: large backing file mapping offset=0 len=8294400 total_size=8294400
[47872589226] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[47875077393] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[47876862759] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[47878525101] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[47880564930] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[47882925090] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[47885494437] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[47887967985] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[47919763254] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] DISP: imported buffer as ID=3
[47921009862] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=5
[47925297618] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=5
[47930393709] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=5 -> OK(12)
[47932486800] [[34mDEBUG[0m] [bloom::display] [CPU1] bloom: imported buffer 1920x1080 as ID=3
[47934287181] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=52 tid=10
[47940320637] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[47943720990] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_RELEASE_BUFFER requested: id=1 size=8294400
[47997066612] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=10
[47999452776] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=6
[48002308827] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=6
[48007552956] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=6 -> OK(8)
[48089049690] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=200 tid=10
[48113396067] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[48115479621] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_COMMIT requested: planes=2
[48204412542] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[48207115275] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[48209549256] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[48212137149] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[48215457213] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[48217792920] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[48220053453] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[48222267588] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[48399455577] [[34mDEBUG[0m] [ps2_mouse] [CPU1] ps2_mouse: init done
[48403070001] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: task subscribed to vector 0x2c
[48404680896] [[34mDEBUG[0m] [ps2_mouse] [CPU1] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[48406498899] [[34mDEBUG[0m] [ps2_mouse] [CPU1] ps2_mouse: using IRQ-assisted loop (IRQ vector 0x2c, poll=4ms)
[48533444982] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[48535114155] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[48536717163] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[48538455471] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[48540441246] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[48542777811] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[48545026299] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[48547126947] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[48569066370] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT transfer begin seq=1 res_id=1 damage=1920x1080+0,0
[48599780229] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 01:14:08.031152168 unix_secs=1777598048.031152168
[48605604498] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT transfer end seq=1 res_id=1
[48608348514] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT flush begin seq=1 res_id=1
[48611771010] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT flush end seq=1 res_id=1
[48613687947] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: first commit copied buffer=3 rect=1920x1080 src_px=0xff40422a dst_px=0xff40422a damage=1920x1080+0,0 res_id=1
[48615482751] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: cursor plane blended buffer=2 at 958,539 src_px=0x20181818 dst_before=0xff880604 dst_after=0xff7a0807
[48616986990] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT complete (seq=1)
[48618556041] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=7
[48621832809] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=7
[48627055389] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=7 -> OK(8)
[48640381152] [[34mDEBUG[0m] [bloom::world] [CPU1] bloom: presented cursor buffer=2 at 939,528 size=96x96
[48642154506] [[32mINFO [0m] [bloom::loop_types] [CPU1] First frame rendered
[48647271123] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=30 frame=6
[48648244788] [[34mDEBUG[0m] [bloom::services::wayland_cmd] [CPU1] wayland-cmd: created surface bloom_id=1
[48653799447] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=13 -> fd=19 node=0xffffffffb01e09b0 port=0xffffffffb02a8270
[48655147101] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=19
[48657717174] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=14 -> fd=19 node=0xffffffffb01e09b0 port=0xffffffffb02a8270
[48659132082] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=19
[48661705521] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/components' tid=10
[48664861344] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_surface obj=11 created for surface=1
[48668856093] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[48670772898] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU2] wayland-server: surface 1 titlebar height 30 frame 6
[48675077781] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[48675577566] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_surface.configure serial=1 sent to obj=11
[48676344552] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/status' tid=10
[48681067116] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[48681432921] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 title="Clock"
[48684174924] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/index' tid=10
[48684725397] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 app_id="thingos.clock"
[48689188020] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[48690897684] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/index' tid=10
[48701626776] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[48703555560] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/events/latest' tid=10
[48708672903] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[48713138496] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/info' tid=10
[48716979036] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[48718327548] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/geometry' tid=10
[48723840957] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[48725993844] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/state' tid=10
[48738437847] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[48740694387] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/title' tid=10
[48746106354] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[48757996914] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/run/bristle/pid' tid=10
[48764057727] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[48769425507] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=5 port_id=PortId(9) mode=Write
[48773024784] [[32mINFO [0m] [bloom::services::input_service] [CPU1] bloom: registered bristle pointer sink
[48777966171] [[34mDEBUG[0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=1 height=30 frame=6
[48778096752] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU2] RECVMSG: thing=3 data_len=22 caps_len=0 node=Pointer { addr: 0xffffffffb01f9450, metadata: DynMetadata(0xffffffff802c6ec0) }
[48781980423] [[32mINFO [0m] [bristle] [CPU2] bristle: bloom sink registered (handle=5 mask=0x03)
[48789439248] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/components' tid=10
[48792029385] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[48793048953] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/status' tid=10
[48805052208] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[48806381877] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/index' tid=10
[48809704746] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[48810685836] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/index' tid=10
[48812746092] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[48813885648] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/events/latest' tid=10
[48816796908] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[48819201981] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/info' tid=10
[48821713776] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[48822679818] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/geometry' tid=10
[48825310842] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[48826340574] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/state' tid=10
[48837249021] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[48838498368] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/title' tid=10
[48841599675] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[48857337573] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/components' tid=10
[48862565334] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[48868534572] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[48870905754] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[48873225753] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[48875750319] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[48878281188] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[48878670819] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/status' tid=10
[48881910528] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[48884218284] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[48884284647] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[48887184060] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/index' tid=10
[48887324079] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[48891841614] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[48893505705] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/index' tid=10
[48900225198] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[48902253543] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/events/latest' tid=10
[48906852027] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[48911186181] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/info' tid=10
[48915913761] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[48917714142] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/geometry' tid=10
[48922053213] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[48923810298] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/state' tid=10
[48940471008] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[48943009137] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/title' tid=10
[48948173703] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=19
[48960232035] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU2] SENDMSG: thing=6 data_len=16 caps_len=1 node=Pointer { addr: 0xffffffffb02a2770, metadata: DynMetadata(0xffffffff802d8c60) }
[48977657322] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU2] RECVMSG: thing=17 data_len=16 caps_len=1 node=Pointer { addr: 0xffffffffb02a2810, metadata: DynMetadata(0xffffffff802d9518) }
[48987771624] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/components' tid=10
[48989228178] [[34mDEBUG[0m] [pistil::font] [CPU2] pistil: opening font /share/fonts/DSEG7Classic-Regular.ttf
[48990895041] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/share/fonts/DSEG7Classic-Regular.ttf' tid=14
[48997225695] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=20
[48998794977] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/status' tid=10
[48998746863] [[34mDEBUG[0m] [pistil::font] [CPU2] pistil: stat font /share/fonts/DSEG7Classic-Regular.ttf
[49002373530] [[34mDEBUG[0m] [pistil::font] [CPU2] pistil: reading font 23272 bytes from /share/fonts/DSEG7Classic-Regular.ttf
[49003560078] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=20
[49005462363] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/index' tid=10
[49012205319] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=20
[49014116316] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/index' tid=10
[49017986985] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=20
[49019868546] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/events/latest' tid=10
[49024449111] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=20
[49030226223] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/info' tid=10
[49035251661] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=20
[49037353365] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/geometry' tid=10
[49042752000] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=20
[49042905747] [[34mDEBUG[0m] [pistil::font] [CPU2] pistil: font read returned 23272 of 23272 bytes
[49044315276] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=8
[49044470937] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/state' tid=10
[49046105493] [[34mDEBUG[0m] [pistil::font] [CPU2] pistil: parsing font /share/fonts/DSEG7Classic-Regular.ttf
[49049278773] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=20
[49050746151] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/title' tid=10
[49055006682] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=20
[49069271658] [[35mTRACE[0m] [bloom::display] [CPU1] bloom: committing bounded damage rects=1
[49070619510] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=200 tid=10
[49127435214] [[34mDEBUG[0m] [pistil::font] [CPU2] pistil: parsed font /share/fonts/DSEG7Classic-Regular.ttf
[49140492885] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[49141506018] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_COMMIT requested: planes=2
[49142665374] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT transfer begin seq=2 res_id=1 damage=20x24+0,0
[49143876837] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT transfer end seq=2 res_id=1
[49144887759] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT flush begin seq=2 res_id=1
[49145816808] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT flush end seq=2 res_id=1
[49146608940] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT complete (seq=2)
[49147272471] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=8
[49149220230] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=8
[49154249430] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=8 -> OK(8)
[49157917182] [[35mTRACE[0m] [bloom::frame_clock] [CPU1] bloom: frame interval 257458872ns (target 16666666ns)
[49203620367] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_surface.ack_configure serial=1 accepted
[49206951387] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU2] wayland-server: wl_surface obj=10 registered frame callback cb=1000
[49208519844] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU2] wayland-server: surface 1 ready for mapping
[49217203464] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=88 tid=10
[49230393267] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[49233367359] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_IMPORT_BUFFER requested: memfd=10, size=520x220
[49241200272] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] DISP: imported buffer as ID=4
[49242819021] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=9
[49247034639] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=9
[49250263689] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=9 -> OK(12)
[49252085883] [[34mDEBUG[0m] [bloom::display] [CPU1] bloom: imported buffer 520x220 as ID=4
[49259717628] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[49262239917] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[49264628259] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[49266660531] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[49268925420] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[49271514006] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[49273225551] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[49274595909] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[49274708868] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/components' tid=10
[49279068531] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=20
[49280881056] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/status' tid=10
[49284818550] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=20
[49287046446] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/index' tid=10
[49291835175] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=20
[49293962058] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/index' tid=10
[49298051649] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=20
[49299296970] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/events/latest' tid=10
[49301984127] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=20
[49305805263] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/info' tid=10
[49308315969] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=20
[49309379592] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/geometry' tid=10
[49311929832] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=20
[49312944120] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/state' tid=10
[49315344738] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=20
[49316326323] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/title' tid=10
[49318822839] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=20
[49321942296] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/info' tid=10
[49331427387] [[34mDEBUG[0m] [bloom::wayland] [CPU2] wayland-server: new client fd=21
[49337394249] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=20
[49339708440] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/geometry' tid=10
[49344828885] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=20
[49346923791] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/state' tid=10
[49352347770] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=20
[49354230849] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/title' tid=10
[49372254327] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: using zwp_linux_dmabuf_v1 buffers
[49373815260] [[32mINFO [0m] [wayland_hello] [CPU1] wayland_hello: bound wp_presentation
[49388270382] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=20
[49401469557] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU2] SENDMSG: thing=21 data_len=16 caps_len=0 node=Pointer { addr: 0xffffffffb02a4e90, metadata: DynMetadata(0xffffffff802d9518) }
[49406939571] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=64
[49408016394] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(13)
[49409177994] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=15 port_id=PortId(13) mode=Write
[49409977518] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=16 port_id=PortId(13) mode=Read
[49586951964] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[49589501478] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[49591896750] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[49594346637] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[49596707061] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[49599155925] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[49601629869] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[49603983033] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[49906271646] [[34mDEBUG[0m] [kernel::syscall::handlers::memory] [CPU1] sys_vm_map: large backing file mapping offset=0 len=8294400 total_size=8294400
[49963974126] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[49966718373] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[49969656891] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[49972137534] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[49974763542] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[49980778122] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[49983722646] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[49986208140] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[49995861564] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=88 tid=10
[50023563810] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[50025133653] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_IMPORT_BUFFER requested: memfd=13, size=1920x1080
[50026543215] [[34mDEBUG[0m] [kernel::syscall::handlers::memory] [CPU3] sys_vm_map: large backing file mapping offset=0 len=8294400 total_size=8294400
[50105587389] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] DISP: imported buffer as ID=5
[50107292466] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=10
[50110249629] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=10
[50114361693] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=10 -> OK(12)
[50117159334] [[34mDEBUG[0m] [bloom::display] [CPU1] bloom: imported buffer 1920x1080 as ID=5
[50118902394] [[32mINFO [0m] [bloom::render] [CPU1] bloom: chrome overlay ready buffer=5 size=1920x1080
[50311420401] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[50314843986] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[50317345650] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[50320226781] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[50322600537] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[50325100386] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[50327549349] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[50329890996] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[50335361373] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpus_online=4
[50336307780] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=0 curr=Some(0) runq=0 runq_avg=1 runq_samples=646 ctxsw=84 idle2busy=0 tick=322 ipi=4 enq=653 deq=652 wake=42 lock_miss=0 lock_pending=0 lock_blocked=0
[50337764400] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=1 curr=Some(10) runq=0 runq_avg=0 runq_samples=258 ctxsw=142 idle2busy=31 tick=190 ipi=34 enq=142 deq=141 wake=118 lock_miss=0 lock_pending=0 lock_blocked=0
[50339209734] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=2 curr=Some(3) runq=0 runq_avg=0 runq_samples=553 ctxsw=108 idle2busy=45 tick=344 ipi=36 enq=69 deq=68 wake=105 lock_miss=0 lock_pending=0 lock_blocked=0
[50340507327] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=3 curr=Some(4) runq=0 runq_avg=0 runq_samples=1054 ctxsw=525 idle2busy=257 tick=340 ipi=214 enq=386 deq=385 wake=515 lock_miss=0 lock_pending=0 lock_blocked=0
[50509472508] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 17:14:08 UTC-8 system_unix=1777598048.985711900
[50622413193] [[34mDEBUG[0m] [kernel::syscall::handlers::memory] [CPU1] sys_vm_map: large backing file mapping offset=0 len=8294400 total_size=8294400
[50692581588] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[50695220169] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[50696194527] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=88 tid=10
[50698794597] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[50703596691] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[50705127099] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_IMPORT_BUFFER requested: memfd=14, size=1920x1080
[50705996418] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[50706942462] [[34mDEBUG[0m] [kernel::syscall::handlers::memory] [CPU3] sys_vm_map: large backing file mapping offset=0 len=8294400 total_size=8294400
[50709512304] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[50711386902] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[50712884013] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[50714463096] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[50817446592] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] DISP: imported buffer as ID=6
[50819094744] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=11
[50822526546] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=11
[50825778003] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=11 -> OK(12)
[50827667121] [[34mDEBUG[0m] [bloom::display] [CPU1] bloom: imported buffer 1920x1080 as ID=6
[50829247359] [[32mINFO [0m] [bloom::render] [CPU1] bloom: shadow overlay ready buffer=6 size=1920x1080
[50877580017] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/share/icons/lucide/chevron-up.svg' tid=10
[50882324427] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/share/icons/lucide/fullscreen.svg' tid=10
[50887655148] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/share/icons/lucide/x.svg' tid=10
[50893562709] [[34mDEBUG[0m] [pistil::font] [CPU1] pistil: opening font /share/fonts/Inter-Regular.ttf
[50895139944] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/share/fonts/Inter-Regular.ttf' tid=10
[50898405228] [[34mDEBUG[0m] [pistil::font] [CPU1] pistil: stat font /share/fonts/Inter-Regular.ttf
[50900481621] [[34mDEBUG[0m] [pistil::font] [CPU1] pistil: reading font 876576 bytes from /share/fonts/Inter-Regular.ttf
[51036421755] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[51038974866] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[51041327106] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[51043685253] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[51046014525] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[51048386037] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[51050723724] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[51052890537] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[51167457660] [[34mDEBUG[0m] [pistil::font] [CPU1] pistil: font read returned 876576 of 876576 bytes
[51170159238] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=23
[51172760100] [[34mDEBUG[0m] [pistil::font] [CPU1] pistil: parsing font /share/fonts/Inter-Regular.ttf
[51372662583] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[51375262191] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[51377659245] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[51380444049] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[51382935978] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[51385549314] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[51388018869] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[51390294549] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[51728806734] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[51731528805] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[51734004399] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[51736582029] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[51739227771] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[51742539486] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[51744962808] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[51747337422] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[52058926953] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[52061299653] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[52063737264] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[52066390068] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[52068588429] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[52071006702] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[52073268588] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[52075287528] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[52389173826] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[52391680011] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[52394219856] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[52396704657] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[52399073397] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[52402043298] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[52404431343] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[52406417844] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[52717861251] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[52720493034] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[52723144782] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[52725715911] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[52728225363] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[52730745210] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[52732970499] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[52735163844] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[53043921117] [[34mDEBUG[0m] [pistil::font] [CPU1] pistil: parsed font /share/fonts/Inter-Regular.ttf
[53047821453] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[53050164354] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[53052451881] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[53054995092] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[53058956016] [[35mTRACE[0m] [bloom::display] [CPU1] bloom: committing bounded damage rects=2
[53060760159] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=372 tid=10
[53063796258] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[53066263404] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[53068615083] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[53070619008] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[53088685617] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[53090307897] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_COMMIT requested: planes=5
[53302887231] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT transfer begin seq=3 res_id=1 damage=540x244+38,46
[53304551916] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT transfer end seq=3 res_id=1
[53305202148] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT flush begin seq=3 res_id=1
[53306067408] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT flush end seq=3 res_id=1
[53306728860] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: COMMIT complete (seq=3)
[53307378168] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=12
[53309911347] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=12
[53315769672] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=12 -> OK(8)
[53320954797] [[35mTRACE[0m] [bloom::frame_clock] [CPU1] bloom: frame interval 2081776257ns (target 16666666ns)
[53326175529] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=30 frame=6
[53327522358] [[34mDEBUG[0m] [bloom::services::wayland_cmd] [CPU1] wayland-cmd: created surface bloom_id=2
[53332303167] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=15 -> fd=23 node=0xffffffffb0187350 port=0xffffffffb02abbb0
[53333578419] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=23
[53335482882] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=16 -> fd=23 node=0xffffffffb0187350 port=0xffffffffb02abbb0
[53336706621] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=23
[53338639167] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_surface obj=11 created for surface=2
[53343738228] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 assigned to xdg_surface=11
[53345580222] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU2] wayland-server: surface 2 titlebar height 30 frame 6
[53348806104] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_surface.configure serial=1 sent to obj=11
[53351321628] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 title="Thing-OS Wayland Lab"
[53358919383] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_toplevel obj=12 app_id="thingos.wayland_hello"
[53377535904] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[53379955035] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[53382116898] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[53384393073] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[53386509264] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[53388779037] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[53390844606] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[53392781772] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[53430670161] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU1] SENDMSG: thing=6 data_len=76 caps_len=1 node=Pointer { addr: 0xffffffffb02a4c10, metadata: DynMetadata(0xffffffff802d8c60) }
[53439156705] [[34mDEBUG[0m] [pistil::font] [CPU1] pistil: opening font /share/fonts/Inter-Regular.ttf
[53440769646] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/share/fonts/Inter-Regular.ttf' tid=13
[53440961145] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU2] RECVMSG: thing=21 data_len=76 caps_len=1 node=Pointer { addr: 0xffffffffb02a4e90, metadata: DynMetadata(0xffffffff802d9518) }
[53448988131] [[32mINFO [0m] [bloom::wayland::dispatch] [CPU2] wayland-server: imported dmabuf wl_buffer=111 480x320 format=0x34325241
[53451963015] [[34mDEBUG[0m] [pistil::font] [CPU1] pistil: stat font /share/fonts/Inter-Regular.ttf
[53455439466] [[34mDEBUG[0m] [pistil::font] [CPU1] pistil: reading font 876576 bytes from /share/fonts/Inter-Regular.ttf
[53707218378] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[53709815973] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[53712205536] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[53714372316] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[53716502829] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[53718822564] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[53720908725] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[53722912881] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[53780530881] [[34mDEBUG[0m] [pistil::font] [CPU1] pistil: font read returned 876576 of 876576 bytes
[53782130193] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=8
[53784055611] [[34mDEBUG[0m] [pistil::font] [CPU1] pistil: parsing font /share/fonts/Inter-Regular.ttf
[53951187741] [[35mTRACE[0m] [bloom::wayland] [CPU2] wayland-server: frame callback done surface=1 ts=26659ms callbacks=1
[53960974749] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU2] wayland-server: wl_surface obj=10 registered frame callback cb=1001
[53965833504] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU2] wayland-server: wl_surface obj=10 registered frame callback cb=1002
[54011543223] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/components' tid=10
[54017255160] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[54019076859] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/status' tid=10
[54024843147] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[54026739789] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/index' tid=10
[54032337810] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[54034412949] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/index' tid=10
[54039871281] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[54042150459] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[54044774223] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[54047826987] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[54050154114] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[54052487082] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[54054774312] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[54057062202] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[54366507855] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[54369198477] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[54371931603] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[54374789766] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[54377044227] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[54379522692] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[54381761082] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[54383976405] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[54389432757] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpus_online=4
[54390340917] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=0 curr=Some(0) runq=0 runq_avg=1 runq_samples=825 ctxsw=108 idle2busy=0 tick=432 ipi=5 enq=832 deq=831 wake=56 lock_miss=0 lock_pending=0 lock_blocked=0
[54391663194] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=1 curr=Some(13) runq=2 runq_avg=1 runq_samples=428 ctxsw=299 idle2busy=44 tick=265 ipi=89 enq=298 deq=295 wake=261 lock_miss=0 lock_pending=0 lock_blocked=0
[54392985801] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=2 curr=Some(3) runq=0 runq_avg=0 runq_samples=801 ctxsw=191 idle2busy=85 tick=452 ipi=67 enq=112 deq=111 wake=191 lock_miss=0 lock_pending=0 lock_blocked=0
[54394283856] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=3 curr=Some(4) runq=0 runq_avg=0 runq_samples=1412 ctxsw=745 idle2busy=367 tick=441 ipi=273 enq=501 deq=500 wake=733 lock_miss=0 lock_pending=0 lock_blocked=0
[54580356600] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU2] wayland-server: wl_surface obj=10 registered frame callback cb=1003
[54622759422] [[32mINFO [0m] [kernel::time] [CPU0] System clock tick: utc=2026-05-01 01:14:11.042681694 unix_secs=1777598051.042681694
[54695301672] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[54697929429] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[54700745286] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[54702745746] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[54705435774] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[54708096762] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[54710050560] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[54712136787] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[55023785586] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[55026348696] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[55027924017] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[55031454951] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[55033944504] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[55036447059] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[55038113724] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[55039733991] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[55354211055] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[55356950451] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[55359658431] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[55362352947] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[55364667930] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[55367117124] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[55369338090] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[55371468042] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[55689706677] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[55692474387] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[55694984994] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[55697693931] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[55700002545] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[55702308585] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[55704438801] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[55706544861] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[55759294239] [[34mDEBUG[0m] [pistil::font] [CPU1] pistil: parsed font /share/fonts/Inter-Regular.ttf
[55785843300] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_surface.ack_configure serial=1 accepted
[55787575272] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU2] wayland-server: wl_surface obj=10 registered frame callback cb=1000
[55789881444] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU2] wayland-server: wp_presentation.feedback surface_obj=10 fb=1001
[55791352650] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU2] wayland-server: surface 2 ready for mapping
[55796594601] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55797656541] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=64
[55798403661] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/events/latest' tid=10
[55798657893] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(14)
[55799745210] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=17 port_id=PortId(14) mode=Write
[55800552423] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=18 port_id=PortId(14) mode=Read
[55805011383] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55807825524] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/info' tid=10
[55811610393] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55814059917] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/geometry' tid=10
[55822592496] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55823785974] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/state' tid=10
[55828827879] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55829941827] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/title' tid=10
[55832285586] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55834528662] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/info' tid=10
[55837199220] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55838628681] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/geometry' tid=10
[55841193144] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55842260694] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/state' tid=10
[55844706126] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55846159710] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/title' tid=10
[55850769117] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55856142441] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/info' tid=10
[55861905297] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55863550446] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/geometry' tid=10
[55867945881] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55869034089] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/state' tid=10
[55872091638] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55873105299] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/title' tid=10
[55876253301] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55892442309] [[34mDEBUG[0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=2 height=30 frame=6
[55899550245] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/components' tid=10
[55901962743] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55902994950] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/status' tid=10
[55905229413] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55906250961] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/index' tid=10
[55908754473] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55909919967] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/index' tid=10
[55919225340] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55921063176] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/events/latest' tid=10
[55924683342] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55927943148] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/info' tid=10
[55931478273] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55933014885] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/geometry' tid=10
[55936596111] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55938095202] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/state' tid=10
[55948858284] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55951100271] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/title' tid=10
[55955286882] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55958459997] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/info' tid=10
[55961962419] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55963516719] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/geometry' tid=10
[55967165892] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55968739695] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/state' tid=10
[55972094805] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55973560236] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/title' tid=10
[55982945271] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55985863362] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/info' tid=10
[55988688195] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55989872994] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/geometry' tid=10
[55993260774] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55994703105] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/state' tid=10
[55997610174] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[55999075770] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/title' tid=10
[56002037817] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56010822780] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[56012596068] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[56014182807] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[56015629197] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[56017132512] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[56018968599] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[56020656153] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[56022082710] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[56025933249] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/components' tid=10
[56028937140] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56030377524] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/status' tid=10
[56033260173] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56034711579] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/index' tid=10
[56037500772] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56038977456] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/index' tid=10
[56042041836] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56043288774] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/events/latest' tid=10
[56052147261] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56055037566] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/info' tid=10
[56058622818] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56060117718] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/geometry' tid=10
[56063371023] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56064906117] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/state' tid=10
[56068064349] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56106922179] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/title' tid=10
[56121572529] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56126307699] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/info' tid=10
[56130668187] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56132316669] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/geometry' tid=10
[56138244195] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56140038207] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/state' tid=10
[56154751224] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56156829432] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/title' tid=10
[56161239486] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56165109000] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/info' tid=10
[56169265383] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56170892415] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/geometry' tid=10
[56175309036] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56186708556] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/state' tid=10
[56191943379] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56193764286] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/title' tid=10
[56198159721] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56229651951] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/components' tid=10
[56233506780] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56235114969] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/status' tid=10
[56239282506] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56242003224] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/index' tid=10
[56256703965] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56258161740] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/index' tid=10
[56262235392] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56263224270] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/events/latest' tid=10
[56265969903] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56268582348] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/info' tid=10
[56271144204] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56281079910] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/geometry' tid=10
[56285503890] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56287169004] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/state' tid=10
[56291962617] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56293586316] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/title' tid=10
[56297082435] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56300198691] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/info' tid=10
[56304326331] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56313004176] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/geometry' tid=10
[56317230717] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56318853624] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/state' tid=10
[56322347235] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56323972452] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/title' tid=10
[56327403528] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56330523975] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/info' tid=10
[56334132921] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56335669665] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/geometry' tid=10
[56340393549] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56345511321] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[56347716612] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[56350519170] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[56352737100] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/state' tid=10
[56352932757] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[56356937208] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[56357343372] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56358582588] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/title' tid=10
[56361131970] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56361185430] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[56364072402] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[56365974687] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[56369878257] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=88 tid=10
[56388520353] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[56390188107] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_IMPORT_BUFFER requested: memfd=15, size=520x220
[56395885953] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] DISP: imported buffer as ID=7
[56396953371] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=13
[56399336961] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=13
[56402362302] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=13 -> OK(12)
[56403892776] [[34mDEBUG[0m] [bloom::display] [CPU1] bloom: imported buffer 520x220 as ID=7
[56404116120] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpus_online=4
[56405052858] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=0 curr=Some(0) runq=0 runq_avg=1 runq_samples=971 ctxsw=120 idle2busy=0 tick=490 ipi=6 enq=978 deq=977 wake=63 lock_miss=0 lock_pending=0 lock_blocked=0
[56406194625] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=1 curr=Some(10) runq=0 runq_avg=1 runq_samples=532 ctxsw=401 idle2busy=46 tick=314 ipi=124 enq=398 deq=397 wake=354 lock_miss=0 lock_pending=0 lock_blocked=0
[56407373418] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=2 curr=Some(3) runq=0 runq_avg=0 runq_samples=895 ctxsw=212 idle2busy=95 tick=503 ipi=71 enq=124 deq=123 wake=212 lock_miss=0 lock_pending=0 lock_blocked=0
[56408553135] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=3 curr=Some(9) runq=0 runq_avg=0 runq_samples=1605 ctxsw=864 idle2busy=427 tick=491 ipi=314 enq=562 deq=561 wake=853 lock_miss=0 lock_pending=0 lock_blocked=0
[56411902206] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=52 tid=10
[56445953751] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[56448713112] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_RELEASE_BUFFER requested: id=4 size=457600
[56453925990] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=10
[56455802700] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=14
[56459098971] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=14
[56463675444] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=14 -> OK(8)
[56486073666] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/components' tid=10
[56490388581] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56492096496] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/status' tid=10
[56495897865] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56497631949] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/index' tid=10
[56500683855] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56501758863] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/index' tid=10
[56505901749] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56508359061] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/events/latest' tid=10
[56512350873] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56515896525] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/info' tid=10
[56518756734] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56520352284] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/geometry' tid=10
[56523858567] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56525213811] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/state' tid=10
[56527688943] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56528709402] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/title' tid=10
[56531111769] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56533312440] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/info' tid=10
[56535944256] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56537610459] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/geometry' tid=10
[56541023781] [[32mINFO [0m] [clock] [CPU2] clock: tick local=2026-04-30 17:14:12 UTC-8 system_unix=1777598052.001253930
[56553640869] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56555590839] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/state' tid=10
[56559985515] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56561587797] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/title' tid=10
[56565367617] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56569456944] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/info' tid=10
[56581740237] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56583027996] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/geometry' tid=10
[56586477948] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56587597770] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/state' tid=10
[56589924897] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56590974561] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/title' tid=10
[56593480482] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56601483114] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=88 tid=10
[56609192871] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[56610659127] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_IMPORT_BUFFER requested: memfd=10, size=520x220
[56617160226] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] DISP: imported buffer as ID=8
[56618378520] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=15
[56620753497] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=15
[56624753031] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=15 -> OK(12)
[56626666404] [[34mDEBUG[0m] [bloom::display] [CPU1] bloom: imported buffer 520x220 as ID=8
[56628013068] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=52 tid=10
[56635095627] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[56636429883] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_RELEASE_BUFFER requested: id=7 size=457600
[56641559667] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=15
[56643726777] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=16
[56646832572] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=16
[56650727430] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=16 -> OK(8)
[56663825955] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/components' tid=10
[56666485656] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56667601683] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/status' tid=10
[56674272666] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[56676605436] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[56679404298] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[56682055089] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[56682202962] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56684390037] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/index' tid=10
[56685066174] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[56688647499] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56690069799] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/index' tid=10
[56690232456] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[56693930172] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[56694017094] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56695887270] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/events/latest' tid=10
[56699636268] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56703234423] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[56713058325] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/info' tid=10
[56718434586] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56720220183] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/geometry' tid=10
[56726287893] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56728052238] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/state' tid=10
[56744232138] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56746523691] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/title' tid=10
[56751296415] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56755224702] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/info' tid=10
[56759361417] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56761001583] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/geometry' tid=10
[56765242149] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56767308312] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/state' tid=10
[56780620413] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56782540056] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/title' tid=10
[56787119862] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56791159161] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/info' tid=10
[56795270433] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56797118829] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/geometry' tid=10
[56811715950] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56813957805] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/state' tid=10
[56818689939] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56820331458] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/title' tid=10
[56824484277] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56846360340] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=88 tid=10
[56880209364] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[56882003541] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_IMPORT_BUFFER requested: memfd=15, size=520x220
[56891750091] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] DISP: imported buffer as ID=9
[56893318977] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=17
[56896897530] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=17
[56902004874] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=17 -> OK(12)
[56904257751] [[34mDEBUG[0m] [bloom::display] [CPU1] bloom: imported buffer 520x220 as ID=9
[56920202196] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=52 tid=10
[56945578800] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[56947498080] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_RELEASE_BUFFER requested: id=8 size=457600
[56952929121] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=10
[56954391252] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=18
[56956503714] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=18
[56961189417] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=18 -> OK(8)
[56983930245] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/components' tid=10
[56988445701] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56990187540] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/status' tid=10
[56995731870] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[56997907659] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/index' tid=10
[57011502966] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57013644567] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/index' tid=10
[57018410493] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57019988256] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/events/latest' tid=10
[57023503845] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57026844204] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/info' tid=10
[57030524265] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57031922244] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/geometry' tid=10
[57039148551] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[57041837622] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[57046394130] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[57048449865] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57049215861] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[57050492994] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/state' tid=10
[57053898561] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[57055569219] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57056489820] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[57057257499] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/title' tid=10
[57061419063] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57066099717] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/info' tid=10
[57067325733] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[57071417106] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[57082003506] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57083375184] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/geometry' tid=10
[57086222754] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57087262980] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/state' tid=10
[57089797446] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57091493547] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/title' tid=10
[57095441568] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57109158678] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/info' tid=10
[57114313146] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57116309250] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/geometry' tid=10
[57120444315] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57122120517] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/state' tid=10
[57126206775] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57127901358] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/title' tid=10
[57133258413] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57145849233] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=88 tid=10
[57169399881] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[57170991207] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_IMPORT_BUFFER requested: memfd=10, size=480x320
[57178481316] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] DISP: imported buffer as ID=10
[57179605065] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=19
[57182592258] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=19
[57187068741] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=19 -> OK(12)
[57188954790] [[34mDEBUG[0m] [bloom::display] [CPU1] bloom: imported buffer 480x320 as ID=10
[57202946427] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/components' tid=10
[57207526992] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57209266818] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/status' tid=10
[57213137751] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57214860087] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/index' tid=10
[57219001785] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57220683300] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/index' tid=10
[57226572447] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57228236241] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/events/latest' tid=10
[57235724040] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57239966949] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/info' tid=10
[57244397694] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57246032580] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/geometry' tid=10
[57250087455] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57251667396] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/state' tid=10
[57255723525] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57257323101] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/1/title' tid=10
[57261453546] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57266570460] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/info' tid=10
[57270850758] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57272607381] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/geometry' tid=10
[57276677337] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57278257179] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/state' tid=10
[57282409998] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57284110983] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/1/title' tid=10
[57288384615] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57292266273] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/info' tid=10
[57296610162] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57298880463] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/geometry' tid=10
[57303183003] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57305009784] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/state' tid=10
[57309157818] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57310684365] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/surfaces/2/title' tid=10
[57314874969] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57319310136] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/2/info' tid=10
[57325040619] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57326685273] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/2/geometry' tid=10
[57334498122] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57336265800] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/2/state' tid=10
[57341774061] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57343506561] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/windows/2/title' tid=10
[57348830517] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57350897175] [[32mINFO [0m] [bloom::services::wayland_cmd] [CPU1] bloom: registered titlebar drag zone surface=3 height=30 frame=6
[57352225524] [[34mDEBUG[0m] [bloom::services::wayland_cmd] [CPU1] wayland-cmd: created surface bloom_id=3
[57358069560] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=17 -> fd=24 node=0xffffffffb02aae10 port=0xffffffffb02ac110
[57359488758] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=24
[57361676757] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=18 -> fd=24 node=0xffffffffb02ab470 port=0xffffffffb02ac110
[57362672829] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=24
[57366999096] [[34mDEBUG[0m] [bloom::wayland::dispatch] [CPU2] wayland-server: xdg_surface obj=21 created for surface=3
[57373092018] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/components' tid=10
[57377610477] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57379441020] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/status' tid=10
[57383896911] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=24
[57385950072] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/s
```
</details>
