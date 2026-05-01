# ❌ Scenario: pointer animation releases imported display buffers cleanly

> Last run: 2026-04-30 18:01:45

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 17392ms | - [📜](./01/serial.log) - |
| 2 | Then the pointer debug overlay should update after mouse movement | ❌ | 61075ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[53317616715] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[53378341995] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] enter
[53384918169] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] phys_memory_map ok
[53386610607] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] modules ok
[53387936019] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] phys_to_virt_offset ok
[53388835203] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] memory map logging done
[53429799060] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] boot_frame_alloc init ok
[53772054468] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] frame allocator build ok
[53774745288] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] frame allocator log ok
[53776311996] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] FRAME_ALLOCATOR init ok
[53777573982] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] tasking init ok
[53778425712] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[53811200949] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[53814635391] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] enter
[53817454977] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] set expand hook
[53820152826] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] expand hook ok
[53823250173] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock begin
[53829371739] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock ok
[53832327054] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] reserve_region begin
[53885581596] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] reserve_region ok
[53886979773] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] inner allocator init begin
[53892358872] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] inner allocator init ok
[53896138296] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] heap top store ok
[53897146215] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] init done
[53898235974] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[53932700613] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] devfs: set_boot_fb width=1920 height=1080 pitch=7680 resource_id=0xfb000000 phys=0x80000000
[53939366118] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs set_boot_fb elapsed_ticks=6364776 elapsed_us=3182
[53952372573] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs FbNode::new elapsed_ticks=8946696 elapsed_us=4473
[53956829982] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] register begin
[53968071861] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] register ok
[53971678167] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs register fb0 elapsed_ticks=14738823 elapsed_us=7369
[53972825544] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[54002811687] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[54006884877] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[54073344864] [[34mDEBUG[0m] [kernel] [CPU0] Seeding entropy pool...
[54078291003] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed begin
[54081084684] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] fill_entropy done
[54085586709] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] add_sample(timer) ok
[54086899020] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] mark_seeded(timer) ok
[54087713394] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[54088657029] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[54089587728] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed done
[54094337418] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] entropy::seed_from_hardware elapsed_ticks=15653946 elapsed_us=7826
[54097512513] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[54129552576] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[54147400956] [[35mTRACE[0m] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[54152354124] [[35mTRACE[0m] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[54155759427] [[35mTRACE[0m] [kernel::sched] [CPU0]   Allocating scheduler...
[54164595474] [[35mTRACE[0m] [kernel::sched] [CPU0]   Leaking scheduler...
[54166405029] [[35mTRACE[0m] [kernel::sched] [CPU0]   Initializing boot task...
[54170634903] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED: allocating CpuScheduler for cpu0 (total=1)
[54174397728] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[54176337666] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[54178316049] [[35mTRACE[0m] [kernel::sched] [CPU0]   Creating boot task...
[54202695360] [[35mTRACE[0m] [kernel::sched] [CPU0]   Creating idle tasks...
[54207351000] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff8008b390 kstack_top=0xffffffffb0020660 arg=0x0
[54212596284] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[54244340535] [[35mTRACE[0m] [kernel::sched] [CPU0]   Boot task initialized
[54248031123] [[34mDEBUG[0m] [kernel::sched::state] [CPU0] SCHED[cpu0]: current=Some(0) idle=Some(1) runnable=0 need_resched=true
[54249812298] [[35mTRACE[0m] [kernel::sched] [CPU0]   Storing scheduler pointer...
[54252135729] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[54255079131] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[54280572720] [[34mDEBUG[0m] [kernel] [CPU0] Initializing VFS...
[54286221132] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='set_cmdline' elapsed_ticks=4403751 elapsed_us=2201 total_ticks=5307159 total_us=2653
[54313883811] [[34mDEBUG[0m] [kernel::vfs] [CPU0] VFS: Created /dev/display directory
[54315611460] [[34mDEBUG[0m] [kernel::vfs] [CPU0] VFS: Created /dev/storage directory
[54384439065] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted union filesystem at / (root)
[54389283201] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted devfs at /dev
[54391689594] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted procfs at /proc
[54393376587] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted sysfs at /sys
[54401128386] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /tmp
[54406324071] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /run
[54407881605] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /services
[54409637568] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /session
[54413709240] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /data
[54416857407] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='vfs_init' elapsed_ticks=129142068 elapsed_us=64571 total_ticks=136390782 total_us=68195
[54419015409] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[54460306890] [[34mDEBUG[0m] [kernel] [CPU0] Scanning PCI bus...
[54470808810] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x29c0 at 00:00.0 class=060000 id=1
[54482300301] [[35mTRACE[0m] [kernel] [CPU0]   BAR0 (MEM32): 0x80000000 (size 0x800000)
[54503263914] [[35mTRACE[0m] [kernel] [CPU0]   BAR2 (MEM64): 0xc000000000 (size 0x4000)
[54509137617] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (MEM32): 0x80883000 (size 0x1000)
[54510833091] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1050 at 00:01.0 class=030000 id=2
[54518707287] [[35mTRACE[0m] [kernel] [CPU0]   BAR0 (I/O):  0x6060 (size 0x20)
[54529290552] [[35mTRACE[0m] [kernel] [CPU0]   BAR1 (MEM32): 0x80882000 (size 0x1000)
[54543490254] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (MEM64): 0xc000004000 (size 0x4000)
[54544916877] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1000 at 00:02.0 class=020000 id=3
[54550104411] [[35mTRACE[0m] [kernel] [CPU0]   BAR1 (MEM32): 0x80881000 (size 0x1000)
[54568629819] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (MEM64): 0xc000008000 (size 0x4000)
[54569973084] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1059 at 00:03.0 class=040100 id=4
[54571131615] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2918 at 00:1f.0 class=060100 id=5
[54574986279] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (I/O):  0x6040 (size 0x20)
[54583756590] [[35mTRACE[0m] [kernel] [CPU0]   BAR5 (MEM32): 0x80880000 (size 0x1000)
[54587268021] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2922 at 00:1f.2 class=010601 id=6
[54592404669] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (I/O):  0x6000 (size 0x40)
[54594689655] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2930 at 00:1f.3 class=0c0500 id=7
[54599027340] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='scan_pci' elapsed_ticks=137428335 elapsed_us=68714 total_ticks=318553257 total_us=159276
[54601197387] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[54639478014] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='register_legacy_devices' elapsed_ticks=1217535 elapsed_us=608 total_ticks=358932882 total_us=179466
[54647234235] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[54687035436] [[34mDEBUG[0m] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[54699868707] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[54704100990] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[54707715447] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[54710984889] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[54719173740] [[34mDEBUG[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[54724459911] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[54728119017] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[54731351862] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[54732178710] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[54732978267] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[54734373672] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[54735638100] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[54736463463] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[54737287044] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[54738036870] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[54738849000] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[54740282289] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[54742320930] [[34mDEBUG[0m] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[54745064715] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[54749265549] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Registers initialized
[54752923566] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[54757536933] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[54761941542] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: All pins masked
[54767094261] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> NMI
[54770425446] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[54771898731] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ4 -> GSI 4 -> 0x24
[54772786266] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Init complete
[54807851472] [[34mDEBUG[0m] [bran::arch::x86_64::ioapic] [CPU0] LAPIC: calibrated 62804500 ticks/sec (delta=628045, ok=true) -> init_cnt=628045
[54810549123] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62804500 ticks/sec), init_cnt=628045 for 100Hz
[54811701087] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='setup_preemption_timer' elapsed_ticks=123268926 elapsed_us=61634 total_ticks=531241986 total_us=265620
[54812830842] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[54848250798] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[54852396060] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[54863120499] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[54870842466] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[54917844234] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[54930268866] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[54931133631] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] SMP: CPU 1 initializing preemption timer (vec=32 init_cnt=628045)
[54933697038] [[34mDEBUG[0m] [kernel] [CPU1] SMP: kernel_secondary_entry arg_cpu=1 runtime_cpu=1
[54936578070] [[34mDEBUG[0m] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[54940865199] [[34mDEBUG[0m] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[54949657389] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff8008b390 kstack_top=0xffffffffb00375a0 arg=0x1
[54951495522] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Task 2 assigned to CPU 1
[54957734040] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 2 deferred inserts
[54969415182] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[54973554933] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[54967647900] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] SMP: CPU 2 initializing preemption timer (vec=32 init_cnt=628045)
[54989817102] [[34mDEBUG[0m] [kernel] [CPU2] SMP: kernel_secondary_entry arg_cpu=2 runtime_cpu=2
[54975109266] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=1
[54990892770] [[34mDEBUG[0m] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[54994741857] [[34mDEBUG[0m] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[54996415881] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=2
[54996864186] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU2] INIT KERNEL CTX: entry=0xffffffff8008b390 kstack_top=0xffffffffb0048d20 arg=0x2
[54997957080] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] SCHED: Task 3 assigned to CPU 2
[54999869364] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[55000536558] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Applying 1 deferred inserts
[55001392446] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserting TID=3
[55002481347] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserts applied
[55001593977] [[34mDEBUG[0m] [kernel::sched::lifecycle] [CPU1] SMP: Secondary CPU 1 online!
[55003264008] [[34mDEBUG[0m] [kernel::sched::lifecycle] [CPU2] SMP: Secondary CPU 2 online!
[55006244568] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: run_scheduler entry on CPU 1
[55007445471] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: bootstrap_cpu start on CPU 1
[55012557765] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle thread 2 and is now schedulable
[55015516017] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] SMP: CPU 3 initializing preemption timer (vec=32 init_cnt=628045)
[55017055137] [[34mDEBUG[0m] [kernel] [CPU3] SMP: kernel_secondary_entry arg_cpu=3 runtime_cpu=3
[55017703521] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[55014210636] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: run_scheduler entry on CPU 2
[55019291943] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: bootstrap_cpu start on CPU 2
[55019395365] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[55020951810] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[55021655436] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle thread 3 and is now schedulable
[55021910823] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='start_secondary_cpus' elapsed_ticks=174087705 elapsed_us=87043 total_ticks=741461589 total_us=370730
[55017887826] [[34mDEBUG[0m] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[55033853325] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[55096161846] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[55099195305] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff8008b390 kstack_top=0xffffffffb005a4a0 arg=0x3
[55104766101] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SCHED: Task 4 assigned to CPU 3
[55096946355] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[55111122858] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[55111877172] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=4
[55112657622] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[55113487836] [[34mDEBUG[0m] [kernel::sched::lifecycle] [CPU3] SMP: Secondary CPU 3 online!
[55114354350] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: run_scheduler entry on CPU 3
[55115244228] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: bootstrap_cpu start on CPU 3
[55121961510] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle thread 4 and is now schedulable
[55182422394] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Enumerating 214 boot modules...
[55198526724] [[35mTRACE[0m] [kernel] [CPU0]   Module[0]: name='/bin/sprout' cmdline='init' size=89912 bytes
[55200303642] [[35mTRACE[0m] [kernel] [CPU0]   Module[1]: name='/bin/bristle' cmdline='' size=68744 bytes
[55204646805] [[35mTRACE[0m] [kernel] [CPU0]   Module[2]: name='/drivers/rtc_cmos' cmdline='init' size=45904 bytes
[55208335347] [[35mTRACE[0m] [kernel] [CPU0]   Module[3]: name='/drivers/ps2_kbd' cmdline='' size=72264 bytes
[55209127578] [[35mTRACE[0m] [kernel] [CPU0]   Module[4]: name='/bin/sh' cmdline='init' size=127864 bytes
[55209943635] [[35mTRACE[0m] [kernel] [CPU0]   Module[5]: name='/bin/ls' cmdline='init' size=62136 bytes
[55214563701] [[35mTRACE[0m] [kernel] [CPU0]   Module[6]: name='/bin/ln' cmdline='init' size=47568 bytes
[55215522978] [[35mTRACE[0m] [kernel] [CPU0]   Module[7]: name='/bin/ps' cmdline='init' size=50544 bytes
[55220195184] [[35mTRACE[0m] [kernel] [CPU0]   Module[8]: name='/bin/kill' cmdline='init' size=55584 bytes
[55221208119] [[35mTRACE[0m] [kernel] [CPU0]   Module[9]: name='/bin/killall' cmdline='init' size=67656 bytes
[55222245507] [[35mTRACE[0m] [kernel] [CPU0]   Module[10]: name='/bin/cat' cmdline='init' size=50832 bytes
[55223464395] [[35mTRACE[0m] [kernel] [CPU0]   Module[11]: name='/bin/head' cmdline='init' size=56112 bytes
[55224457035] [[35mTRACE[0m] [kernel] [CPU0]   Module[12]: name='/bin/tail' cmdline='init' size=56088 bytes
[55225395786] [[35mTRACE[0m] [kernel] [CPU0]   Module[13]: name='/bin/top' cmdline='init' size=65488 bytes
[55226307378] [[35mTRACE[0m] [kernel] [CPU0]   Module[14]: name='/bin/wc' cmdline='init' size=56248 bytes
[55228645758] [[35mTRACE[0m] [kernel] [CPU0]   Module[15]: name='/bin/yes' cmdline='init' size=44520 bytes
[55229602527] [[35mTRACE[0m] [kernel] [CPU0]   Module[16]: name='/bin/which' cmdline='init' size=50448 bytes
[55230504219] [[35mTRACE[0m] [kernel] [CPU0]   Module[17]: name='/bin/cp' cmdline='init' size=46312 bytes
[55231393404] [[35mTRACE[0m] [kernel] [CPU0]   Module[18]: name='/bin/mv' cmdline='init' size=47048 bytes
[55232324433] [[35mTRACE[0m] [kernel] [CPU0]   Module[19]: name='/bin/rm' cmdline='init' size=47128 bytes
[55233234507] [[35mTRACE[0m] [kernel] [CPU0]   Module[20]: name='/bin/rmdir' cmdline='init' size=47144 bytes
[55234355088] [[35mTRACE[0m] [kernel] [CPU0]   Module[21]: name='/bin/mkdir' cmdline='init' size=47144 bytes
[55235270343] [[35mTRACE[0m] [kernel] [CPU0]   Module[22]: name='/bin/mount' cmdline='init' size=57688 bytes
[55236185103] [[35mTRACE[0m] [kernel] [CPU0]   Module[23]: name='/bin/echo' cmdline='init' size=44616 bytes
[55239402999] [[35mTRACE[0m] [kernel] [CPU0]   Module[24]: name='/bin/printf' cmdline='init' size=93952 bytes
[55240840083] [[35mTRACE[0m] [kernel] [CPU0]   Module[25]: name='/bin/grep' cmdline='init' size=85240 bytes
[55258107498] [[35mTRACE[0m] [kernel] [CPU0]   Module[26]: name='/bin/pwd' cmdline='init' size=39184 bytes
[55264857582] [[35mTRACE[0m] [kernel] [CPU0]   Module[27]: name='/bin/touch' cmdline='init' size=46160 bytes
[55265693373] [[35mTRACE[0m] [kernel] [CPU0]   Module[28]: name='/bin/setshell' cmdline='init' size=47048 bytes
[55271021289] [[35mTRACE[0m] [kernel] [CPU0]   Module[29]: name='/bin/dmesg' cmdline='init' size=39328 bytes
[55272256017] [[35mTRACE[0m] [kernel] [CPU0]   Module[30]: name='/bin/stat' cmdline='init' size=46200 bytes
[55273161603] [[35mTRACE[0m] [kernel] [CPU0]   Module[31]: name='/bin/file' cmdline='init' size=51104 bytes
[55274188431] [[35mTRACE[0m] [kernel] [CPU0]   Module[32]: name='/bin/dirname' cmdline='init' size=50096 bytes
[55275009108] [[35mTRACE[0m] [kernel] [CPU0]   Module[33]: name='/bin/basename' cmdline='init' size=50104 bytes
[55276076724] [[35mTRACE[0m] [kernel] [CPU0]   Module[34]: name='/bin/sleep' cmdline='init' size=56824 bytes
[55276941225] [[35mTRACE[0m] [kernel] [CPU0]   Module[35]: name='/bin/sort' cmdline='init' size=66336 bytes
[55277785563] [[35mTRACE[0m] [kernel] [CPU0]   Module[36]: name='/bin/env' cmdline='init' size=45832 bytes
[55279122756] [[35mTRACE[0m] [kernel] [CPU0]   Module[37]: name='/bin/uname' cmdline='' size=44632 bytes
[55280227761] [[35mTRACE[0m] [kernel] [CPU0]   Module[38]: name='/bin/true' cmdline='' size=20744 bytes
[55280990754] [[35mTRACE[0m] [kernel] [CPU0]   Module[39]: name='/bin/false' cmdline='' size=20744 bytes
[55281806316] [[35mTRACE[0m] [kernel] [CPU0]   Module[40]: name='/bin/input_echo' cmdline='' size=43016 bytes
[55282770840] [[35mTRACE[0m] [kernel] [CPU0]   Module[41]: name='/drivers/ps2_mouse' cmdline='' size=71096 bytes
[55283627421] [[35mTRACE[0m] [kernel] [CPU0]   Module[42]: name='/drivers/display_bootfb' cmdline='' size=91736 bytes
[55291063245] [[35mTRACE[0m] [kernel] [CPU0]   Module[43]: name='/drivers/display_virtio_gpu' cmdline='' size=147560 bytes
[55291882536] [[35mTRACE[0m] [kernel] [CPU0]   Module[44]: name='/bin/cambium' cmdline='' size=144768 bytes
[55292669982] [[35mTRACE[0m] [kernel] [CPU0]   Module[45]: name='/drivers/virtio_netd' cmdline='' size=123920 bytes
[55293390900] [[35mTRACE[0m] [kernel] [CPU0]   Module[46]: name='/drivers/rtl8168d' cmdline='' size=61392 bytes
[55294095747] [[35mTRACE[0m] [kernel] [CPU0]   Module[47]: name='/bin/netd' cmdline='' size=10819752 bytes
[55310252811] [[35mTRACE[0m] [kernel] [CPU0]   Module[48]: name='/bin/mesocarp' cmdline='' size=123496 bytes
[55311141930] [[35mTRACE[0m] [kernel] [CPU0]   Module[49]: name='/bin/mdns' cmdline='' size=123496 bytes
[55313456715] [[35mTRACE[0m] [kernel] [CPU0]   Module[50]: name='/bin/mdnsd' cmdline='' size=123496 bytes
[55314310425] [[35mTRACE[0m] [kernel] [CPU0]   Module[51]: name='/bin/fetchd' cmdline='' size=76752 bytes
[55315164894] [[35mTRACE[0m] [kernel] [CPU0]   Module[52]: name='/bin/httpsd' cmdline='' size=536128 bytes
[55316277390] [[35mTRACE[0m] [kernel] [CPU0]   Module[53]: name='/bin/find' cmdline='' size=57896 bytes
[55317339858] [[35mTRACE[0m] [kernel] [CPU0]   Module[54]: name='/bin/ip' cmdline='' size=55472 bytes
[55318649364] [[35mTRACE[0m] [kernel] [CPU0]   Module[55]: name='/bin/iso_reader' cmdline='' size=37424 bytes
[55321661835] [[35mTRACE[0m] [kernel] [CPU0]   Module[56]: name='/bin/ping' cmdline='' size=65184 bytes
[55330445544] [[35mTRACE[0m] [kernel] [CPU0]   Module[57]: name='/bin/nslookup' cmdline='' size=56424 bytes
[55336709043] [[35mTRACE[0m] [kernel] [CPU0]   Module[58]: name='/drivers/ahci_disk' cmdline='' size=53784 bytes
[55339729830] [[35mTRACE[0m] [kernel] [CPU0]   Module[59]: name='/drivers/ata_disk' cmdline='' size=64448 bytes
[55348441500] [[35mTRACE[0m] [kernel] [CPU0]   Module[60]: name='/bin/iso9660d' cmdline='' size=78544 bytes
[55349382297] [[35mTRACE[0m] [kernel] [CPU0]   Module[61]: name='/drivers/virtio_sound' cmdline='' size=99296 bytes
[55350279402] [[35mTRACE[0m] [kernel] [CPU0]   Module[62]: name='/drivers/hdaudio' cmdline='' size=67696 bytes
[55355496306] [[35mTRACE[0m] [kernel] [CPU0]   Module[63]: name='/drivers/pci_stubd' cmdline='' size=58168 bytes
[55363807323] [[35mTRACE[0m] [kernel] [CPU0]   Module[64]: name='/drivers/chime' cmdline='' size=59536 bytes
[55364729145] [[35mTRACE[0m] [kernel] [CPU0]   Module[65]: name='/bin/vfs_hello' cmdline='' size=39560 bytes
[55365415809] [[35mTRACE[0m] [kernel] [CPU0]   Module[66]: name='/bin/show_args' cmdline='' size=42488 bytes
[55366126365] [[35mTRACE[0m] [kernel] [CPU0]   Module[67]: name='/bin/clock' cmdline='' size=82936 bytes
[55367948955] [[35mTRACE[0m] [kernel] [CPU0]   Module[68]: name='/bin/env_roundtrip' cmdline='' size=48232 bytes
[55368931959] [[35mTRACE[0m] [kernel] [CPU0]   Module[69]: name='/bin/cwd_test' cmdline='' size=43696 bytes
[55369766100] [[35mTRACE[0m] [kernel] [CPU0]   Module[70]: name='/bin/date' cmdline='' size=67032 bytes
[55370456493] [[35mTRACE[0m] [kernel] [CPU0]   Module[71]: name='/bin/wayland_hello' cmdline='' size=82360 bytes
[55378357320] [[35mTRACE[0m] [kernel] [CPU0]   Module[72]: name='/bin/terminal' cmdline='' size=91056 bytes
[55379082495] [[35mTRACE[0m] [kernel] [CPU0]   Module[73]: name='/bin/tee' cmdline='' size=44968 bytes
[55381706688] [[35mTRACE[0m] [kernel] [CPU0]   Module[74]: name='/bin/xargs' cmdline='' size=55016 bytes
[55382457834] [[35mTRACE[0m] [kernel] [CPU0]   Module[75]: name='/bin/placed' cmdline='' size=38048 bytes
[55383880959] [[35mTRACE[0m] [kernel] [CPU0]   Module[76]: name='/bin/bloom' cmdline='' size=462464 bytes
[55384805685] [[35mTRACE[0m] [kernel] [CPU0]   Module[77]: name='/bin/clear' cmdline='' size=20872 bytes
[55385695266] [[35mTRACE[0m] [kernel] [CPU0]   Module[78]: name='/bin/loglevel' cmdline='' size=41536 bytes
[55386558447] [[35mTRACE[0m] [kernel] [CPU0]   Module[79]: name='/bin/poll_mux' cmdline='' size=39056 bytes
[55387603689] [[35mTRACE[0m] [kernel] [CPU0]   Module[80]: name='/bin/ipc_service_demo' cmdline='' size=47592 bytes
[55388477628] [[35mTRACE[0m] [kernel] [CPU0]   Module[81]: name='/bin/ipc_pipe_demo' cmdline='' size=43936 bytes
[55389341634] [[35mTRACE[0m] [kernel] [CPU0]   Module[82]: name='/bin/ipc_provider_demo' cmdline='' size=84440 bytes
[55393553292] [[35mTRACE[0m] [kernel] [CPU0]   Module[83]: name='/bin/vfs_test_provider' cmdline='' size=70648 bytes
[55397119503] [[35mTRACE[0m] [kernel] [CPU0]   Module[84]: name='/bin/ipc_memfd_demo' cmdline='' size=38232 bytes
[55397886753] [[35mTRACE[0m] [kernel] [CPU0]   Module[85]: name='/bin/test_exec' cmdline='' size=62592 bytes
[55398575364] [[35mTRACE[0m] [kernel] [CPU0]   Module[86]: name='/bin/test_vm_protect' cmdline='' size=38224 bytes
[55399293708] [[35mTRACE[0m] [kernel] [CPU0]   Module[87]: name='/bin/test_exec_env' cmdline='' size=58904 bytes
[55399977567] [[35mTRACE[0m] [kernel] [CPU0]   Module[88]: name='/bin/test_threads' cmdline='' size=82376 bytes
[55400703798] [[35mTRACE[0m] [kernel] [CPU0]   Module[89]: name='/bin/test_futex' cmdline='' size=52992 bytes
[55404483420] [[35mTRACE[0m] [kernel] [CPU0]   Module[90]: name='/bin/ld_so' cmdline='' size=378600 bytes
[55405385838] [[35mTRACE[0m] [kernel] [CPU0]   Module[91]: name='/bin/test_dyn_loader' cmdline='' size=57760 bytes
[55406335347] [[35mTRACE[0m] [kernel] [CPU0]   Module[92]: name='/bin/test_dlopen' cmdline='' size=64688 bytes
[55407266244] [[35mTRACE[0m] [kernel] [CPU0]   Module[93]: name='/bin/reboot' cmdline='' size=32048 bytes
[55408121340] [[35mTRACE[0m] [kernel] [CPU0]   Module[94]: name='/bin/shutdown' cmdline='' size=32048 bytes
[55408997853] [[35mTRACE[0m] [kernel] [CPU0]   Module[95]: name='/bin/attr_list' cmdline='' size=47400 bytes
[55410690192] [[35mTRACE[0m] [kernel] [CPU0]   Module[96]: name='/bin/attr_get' cmdline='' size=47488 bytes
[55411692237] [[35mTRACE[0m] [kernel] [CPU0]   Module[97]: name='/bin/attr_set' cmdline='' size=69600 bytes
[55412548653] [[35mTRACE[0m] [kernel] [CPU0]   Module[98]: name='/bin/attr_rm' cmdline='' size=47008 bytes
[55413387381] [[35mTRACE[0m] [kernel] [CPU0]   Module[99]: name='/lib/libpistil.so' cmdline='' size=826992 bytes
[55414277985] [[35mTRACE[0m] [kernel] [CPU0]   Module[100]: name='/share/wallpapers/clouds.bmp' cmdline='' size=786486 bytes
[55415199081] [[35mTRACE[0m] [kernel] [CPU0]   Module[101]: name='/share/wallpapers/flower.bmp' cmdline='' size=8386614 bytes
[55416126249] [[35mTRACE[0m] [kernel] [CPU0]   Module[102]: name='/share/wallpapers/flower.png' cmdline='' size=2652468 bytes
[55417217295] [[35mTRACE[0m] [kernel] [CPU0]   Module[103]: name='/share/wallpapers/leather.bmp' cmdline='' size=1179702 bytes
[55418204952] [[35mTRACE[0m] [kernel] [CPU0]   Module[104]: name='/share/wallpapers/linen.bmp' cmdline='' size=4718646 bytes
[55419045165] [[35mTRACE[0m] [kernel] [CPU0]   Module[105]: name='/share/fonts/DSEG7Classic-Regular.ttf' cmdline='' size=23272 bytes
[55419982662] [[35mTRACE[0m] [kernel] [CPU0]   Module[106]: name='/share/fonts/Hack-Regular.ttf' cmdline='' size=309408 bytes
[55421111229] [[35mTRACE[0m] [kernel] [CPU0]   Module[107]: name='/share/fonts/Inter-Regular.ttf' cmdline='' size=876576 bytes
[55422212901] [[35mTRACE[0m] [kernel] [CPU0]   Module[108]: name='/share/fonts/Iosevka-Regular.ttf' cmdline='' size=10457376 bytes
[55439531862] [[35mTRACE[0m] [kernel] [CPU0]   Module[109]: name='/share/fonts/JetBrainsMono-Regular.ttf' cmdline='' size=270224 bytes
[55440494340] [[35mTRACE[0m] [kernel] [CPU0]   Module[110]: name='/share/fonts/NotoSans-Regular.ttf' cmdline='' size=569208 bytes
[55441225026] [[35mTRACE[0m] [kernel] [CPU0]   Module[111]: name='/share/fonts/NotoSansSymbol-Regular.ttf' cmdline='' size=258156 bytes
[55441942182] [[35mTRACE[0m] [kernel] [CPU0]   Module[112]: name='/share/fonts/NotoSansSymbol2-Regular.ttf' cmdline='' size=656852 bytes
[55442865786] [[35mTRACE[0m] [kernel] [CPU0]   Module[113]: name='/share/fonts/NotoSerif-Regular.ttf' cmdline='' size=616196 bytes
[55454107698] [[35mTRACE[0m] [kernel] [CPU0]   Module[114]: name='/share/themes/solarized_warm.toml' cmdline='' size=1832 bytes
[55458777891] [[35mTRACE[0m] [kernel] [CPU0]   Module[115]: name='/share/cursors/future/alias.svg' cmdline='' size=9033 bytes
[55459774557] [[35mTRACE[0m] [kernel] [CPU0]   Module[116]: name='/share/cursors/future/all-scroll.svg' cmdline='' size=5170 bytes
[55460725221] [[35mTRACE[0m] [kernel] [CPU0]   Module[117]: name='/share/cursors/future/bottom_left_corner.svg' cmdline='' size=1441 bytes
[55461671001] [[35mTRACE[0m] [kernel] [CPU0]   Module[118]: name='/share/cursors/future/bottom_right_corner.svg' cmdline='' size=3037 bytes
[55462551837] [[35mTRACE[0m] [kernel] [CPU0]   Module[119]: name='/share/cursors/future/bottom_side.svg' cmdline='' size=5432 bytes
[55463513094] [[35mTRACE[0m] [kernel] [CPU0]   Module[120]: name='/share/cursors/future/cell.svg' cmdline='' size=3707 bytes
[55471275024] [[35mTRACE[0m] [kernel] [CPU0]   Module[121]: name='/share/cursors/future/center_ptr.svg' cmdline='' size=9766 bytes
[55472683926] [[35mTRACE[0m] [kernel] [CPU0]   Module[122]: name='/share/cursors/future/col-resize.svg' cmdline='' size=16463 bytes
[55473682605] [[35mTRACE[0m] [kernel] [CPU0]   Module[123]: name='/share/cursors/future/color-picker.svg' cmdline='' size=6296 bytes
[55474543707] [[35mTRACE[0m] [kernel] [CPU0]   Module[124]: name='/share/cursors/future/context-menu.svg' cmdline='' size=9011 bytes
[55475487078] [[35mTRACE[0m] [kernel] [CPU0]   Module[125]: name='/share/cursors/future/copy.svg' cmdline='' size=3527 bytes
[55476596175] [[35mTRACE[0m] [kernel] [CPU0]   Module[126]: name='/share/cursors/future/crosshair.svg' cmdline='' size=16918 bytes
[55477575120] [[35mTRACE[0m] [kernel] [CPU0]   Module[127]: name='/share/cursors/future/default.svg' cmdline='' size=3079 bytes
[55478591454] [[35mTRACE[0m] [kernel] [CPU0]   Module[128]: name='/share/cursors/future/dnd-move.svg' cmdline='' size=3569 bytes
[55479494895] [[35mTRACE[0m] [kernel] [CPU0]   Module[129]: name='/share/cursors/future/dnd-no-drop.svg' cmdline='' size=4753 bytes
[55480427673] [[35mTRACE[0m] [kernel] [CPU0]   Module[130]: name='/share/cursors/future/down-arrow.svg' cmdline='' size=1820 bytes
[55481241684] [[35mTRACE[0m] [kernel] [CPU0]   Module[131]: name='/share/cursors/future/draft.svg' cmdline='' size=3136 bytes
[55484911350] [[35mTRACE[0m] [kernel] [CPU0]   Module[132]: name='/share/cursors/future/fleur.svg' cmdline='' size=28469 bytes
[55494717267] [[35mTRACE[0m] [kernel] [CPU0]   Module[133]: name='/share/cursors/future/help.svg' cmdline='' size=5873 bytes
[55495445181] [[35mTRACE[0m] [kernel] [CPU0]   Module[134]: name='/share/cursors/future/left-arrow.svg' cmdline='' size=1785 bytes
[55496221902] [[35mTRACE[0m] [kernel] [CPU0]   Module[135]: name='/share/cursors/future/left_side.svg' cmdline='' size=3848 bytes
[55499653572] [[35mTRACE[0m] [kernel] [CPU0]   Module[136]: name='/share/cursors/future/no-drop.svg' cmdline='' size=3861 bytes
[55500413793] [[35mTRACE[0m] [kernel] [CPU0]   Module[137]: name='/share/cursors/future/not-allowed.svg' cmdline='' size=1567 bytes
[55501124580] [[35mTRACE[0m] [kernel] [CPU0]   Module[138]: name='/share/cursors/future/openhand.svg' cmdline='' size=3435 bytes
[55501847313] [[35mTRACE[0m] [kernel] [CPU0]   Module[139]: name='/share/cursors/future/pencil.svg' cmdline='' size=6527 bytes
[55502565888] [[35mTRACE[0m] [kernel] [CPU0]   Module[140]: name='/share/cursors/future/pirate.svg' cmdline='' size=6663 bytes
[55503268194] [[35mTRACE[0m] [kernel] [CPU0]   Module[141]: name='/share/cursors/future/pointer.svg' cmdline='' size=2922 bytes
[55503993270] [[35mTRACE[0m] [kernel] [CPU0]   Module[142]: name='/share/cursors/future/progress-01.svg' cmdline='' size=11030 bytes
[55504722966] [[35mTRACE[0m] [kernel] [CPU0]   Module[143]: name='/share/cursors/future/progress-02.svg' cmdline='' size=12198 bytes
[55508398539] [[35mTRACE[0m] [kernel] [CPU0]   Module[144]: name='/share/cursors/future/progress-03.svg' cmdline='' size=12733 bytes
[55509256869] [[35mTRACE[0m] [kernel] [CPU0]   Module[145]: name='/share/cursors/future/progress-04.svg' cmdline='' size=13777 bytes
[55510010094] [[35mTRACE[0m] [kernel] [CPU0]   Module[146]: name='/share/cursors/future/progress-05.svg' cmdline='' size=13835 bytes
[55515853899] [[35mTRACE[0m] [kernel] [CPU0]   Module[147]: name='/share/cursors/future/progress-06.svg' cmdline='' size=14907 bytes
[55516734438] [[35mTRACE[0m] [kernel] [CPU0]   Module[148]: name='/share/cursors/future/progress-07.svg' cmdline='' size=14805 bytes
[55517404734] [[35mTRACE[0m] [kernel] [CPU0]   Module[149]: name='/share/cursors/future/progress-08.svg' cmdline='' size=15996 bytes
[55518065295] [[35mTRACE[0m] [kernel] [CPU0]   Module[150]: name='/share/cursors/future/progress-09.svg' cmdline='' size=16010 bytes
[55518787599] [[35mTRACE[0m] [kernel] [CPU0]   Module[151]: name='/share/cursors/future/progress-10.svg' cmdline='' size=17080 bytes
[55530513324] [[35mTRACE[0m] [kernel] [CPU0]   Module[152]: name='/share/cursors/future/progress-11.svg' cmdline='' size=17096 bytes
[55535064915] [[35mTRACE[0m] [kernel] [CPU0]   Module[153]: name='/share/cursors/future/progress-12.svg' cmdline='' size=17079 bytes
[55536090918] [[35mTRACE[0m] [kernel] [CPU0]   Module[154]: name='/share/cursors/future/progress-13.svg' cmdline='' size=15900 bytes
[55537265355] [[35mTRACE[0m] [kernel] [CPU0]   Module[155]: name='/share/cursors/future/progress-14.svg' cmdline='' size=16001 bytes
[55538390160] [[35mTRACE[0m] [kernel] [CPU0]   Module[156]: name='/share/cursors/future/progress-15.svg' cmdline='' size=14931 bytes
[55539468369] [[35mTRACE[0m] [kernel] [CPU0]   Module[157]: name='/share/cursors/future/progress-16.svg' cmdline='' size=14873 bytes
[55540531101] [[35mTRACE[0m] [kernel] [CPU0]   Module[158]: name='/share/cursors/future/progress-17.svg' cmdline='' size=13846 bytes
[55541468730] [[35mTRACE[0m] [kernel] [CPU0]   Module[159]: name='/share/cursors/future/progress-18.svg' cmdline='' size=13835 bytes
[55542562317] [[35mTRACE[0m] [kernel] [CPU0]   Module[160]: name='/share/cursors/future/progress-19.svg' cmdline='' size=12645 bytes
[55543492851] [[35mTRACE[0m] [kernel] [CPU0]   Module[161]: name='/share/cursors/future/progress-20.svg' cmdline='' size=12741 bytes
[55544381541] [[35mTRACE[0m] [kernel] [CPU0]   Module[162]: name='/share/cursors/future/progress-21.svg' cmdline='' size=11660 bytes
[55545346362] [[35mTRACE[0m] [kernel] [CPU0]   Module[163]: name='/share/cursors/future/progress-22.svg' cmdline='' size=11612 bytes
[55546515915] [[35mTRACE[0m] [kernel] [CPU0]   Module[164]: name='/share/cursors/future/progress-23.svg' cmdline='' size=11674 bytes
[55566300768] [[35mTRACE[0m] [kernel] [CPU0]   Module[165]: name='/share/cursors/future/progress.svg' cmdline='' size=11642 bytes
[55582835880] [[35mTRACE[0m] [kernel] [CPU0]   Module[166]: name='/share/cursors/future/right-arrow.svg' cmdline='' size=3432 bytes
[55583750211] [[35mTRACE[0m] [kernel] [CPU0]   Module[167]: name='/share/cursors/future/right_ptr.svg' cmdline='' size=5155 bytes
[55584465486] [[35mTRACE[0m] [kernel] [CPU0]   Module[168]: name='/share/cursors/future/right_side.svg' cmdline='' size=6447 bytes
[55585223826] [[35mTRACE[0m] [kernel] [CPU0]   Module[169]: name='/share/cursors/future/row-resize.svg' cmdline='' size=15825 bytes
[55585934349] [[35mTRACE[0m] [kernel] [CPU0]   Module[170]: name='/share/cursors/future/size_bdiag.svg' cmdline='' size=16323 bytes
[55594339812] [[35mTRACE[0m] [kernel] [CPU0]   Module[171]: name='/share/cursors/future/size_fdiag.svg' cmdline='' size=16589 bytes
[55595095875] [[35mTRACE[0m] [kernel] [CPU0]   Module[172]: name='/share/cursors/future/size_hor.svg' cmdline='' size=16255 bytes
[55595865732] [[35mTRACE[0m] [kernel] [CPU0]   Module[173]: name='/share/cursors/future/size_ver.svg' cmdline='' size=15822 bytes
[55596580743] [[35mTRACE[0m] [kernel] [CPU0]   Module[174]: name='/share/cursors/future/text.svg' cmdline='' size=5584 bytes
[55616843337] [[35mTRACE[0m] [kernel] [CPU0]   Module[175]: name='/share/cursors/future/top_left_corner.svg' cmdline='' size=7121 bytes
[55618020084] [[35mTRACE[0m] [kernel] [CPU0]   Module[176]: name='/share/cursors/future/top_right_corner.svg' cmdline='' size=7139 bytes
[55618960980] [[35mTRACE[0m] [kernel] [CPU0]   Module[177]: name='/share/cursors/future/top_side.svg' cmdline='' size=10651 bytes
[55619883759] [[35mTRACE[0m] [kernel] [CPU0]   Module[178]: name='/share/cursors/future/up-arrow.svg' cmdline='' size=7653 bytes
[55621268406] [[35mTRACE[0m] [kernel] [CPU0]   Module[179]: name='/share/cursors/future/vertical-text.svg' cmdline='' size=5621 bytes
[55621960548] [[35mTRACE[0m] [kernel] [CPU0]   Module[180]: name='/share/cursors/future/wait-01.svg' cmdline='' size=15308 bytes
[55622677770] [[35mTRACE[0m] [kernel] [CPU0]   Module[181]: name='/share/cursors/future/wait-02.svg' cmdline='' size=6672 bytes
[55623339288] [[35mTRACE[0m] [kernel] [CPU0]   Module[182]: name='/share/cursors/future/wait-03.svg' cmdline='' size=6686 bytes
[55628197416] [[35mTRACE[0m] [kernel] [CPU0]   Module[183]: name='/share/cursors/future/wait-04.svg' cmdline='' size=7827 bytes
[55629123000] [[35mTRACE[0m] [kernel] [CPU0]   Module[184]: name='/share/cursors/future/wait-05.svg' cmdline='' size=7835 bytes
[55629912162] [[35mTRACE[0m] [kernel] [CPU0]   Module[185]: name='/share/cursors/future/wait-06.svg' cmdline='' size=8976 bytes
[55630712313] [[35mTRACE[0m] [kernel] [CPU0]   Module[186]: name='/share/cursors/future/wait-07.svg' cmdline='' size=8989 bytes
[55634435571] [[35mTRACE[0m] [kernel] [CPU0]   Module[187]: name='/share/cursors/future/wait-08.svg' cmdline='' size=10126 bytes
[55638709038] [[35mTRACE[0m] [kernel] [CPU0]   Module[188]: name='/share/cursors/future/wait-09.svg' cmdline='' size=10140 bytes
[55639513083] [[35mTRACE[0m] [kernel] [CPU0]   Module[189]: name='/share/cursors/future/wait-10.svg' cmdline='' size=11277 bytes
[55640257497] [[35mTRACE[0m] [kernel] [CPU0]   Module[190]: name='/share/cursors/future/wait-11.svg' cmdline='' size=11291 bytes
[55641029499] [[35mTRACE[0m] [kernel] [CPU0]   Module[191]: name='/share/cursors/future/wait-12.svg' cmdline='' size=11277 bytes
[55641774243] [[35mTRACE[0m] [kernel] [CPU0]   Module[192]: name='/share/cursors/future/wait-13.svg' cmdline='' size=10140 bytes
[55642528722] [[35mTRACE[0m] [kernel] [CPU0]   Module[193]: name='/share/cursors/future/wait-14.svg' cmdline='' size=10126 bytes
[55643287656] [[35mTRACE[0m] [kernel] [CPU0]   Module[194]: name='/share/cursors/future/wait-15.svg' cmdline='' size=8991 bytes
[55643956962] [[35mTRACE[0m] [kernel] [CPU0]   Module[195]: name='/share/cursors/future/wait-16.svg' cmdline='' size=8978 bytes
[55644697383] [[35mTRACE[0m] [kernel] [CPU0]   Module[196]: name='/share/cursors/future/wait-17.svg' cmdline='' size=7842 bytes
[55645438101] [[35mTRACE[0m] [kernel] [CPU0]   Module[197]: name='/share/cursors/future/wait-18.svg' cmdline='' size=7829 bytes
[55646113347] [[35mTRACE[0m] [kernel] [CPU0]   Module[198]: name='/share/cursors/future/wait-19.svg' cmdline='' size=6822 bytes
[55646821263] [[35mTRACE[0m] [kernel] [CPU0]   Module[199]: name='/share/cursors/future/wait-20.svg' cmdline='' size=6679 bytes
[55647611019] [[35mTRACE[0m] [kernel] [CPU0]   Module[200]: name='/share/cursors/future/wait-21.svg' cmdline='' size=5542 bytes
[55648367610] [[35mTRACE[0m] [kernel] [CPU0]   Module[201]: name='/share/cursors/future/wait-22.svg' cmdline='' size=5528 bytes
[55649120604] [[35mTRACE[0m] [kernel] [CPU0]   Module[202]: name='/share/cursors/future/wait-23.svg' cmdline='' size=5546 bytes
[55649875809] [[35mTRACE[0m] [kernel] [CPU0]   Module[203]: name='/share/cursors/future/wait.svg' cmdline='' size=5525 bytes
[55660402578] [[35mTRACE[0m] [kernel] [CPU0]   Module[204]: name='/share/cursors/future/wayland-cursor.svg' cmdline='' size=8222 bytes
[55661350701] [[35mTRACE[0m] [kernel] [CPU0]   Module[205]: name='/share/cursors/future/x-cursor.svg' cmdline='' size=6822 bytes
[55662137322] [[35mTRACE[0m] [kernel] [CPU0]   Module[206]: name='/share/cursors/future/zoom-in.svg' cmdline='' size=5441 bytes
[55670263671] [[35mTRACE[0m] [kernel] [CPU0]   Module[207]: name='/share/cursors/future/zoom-out.svg' cmdline='' size=5403 bytes
[55679262969] [[35mTRACE[0m] [kernel] [CPU0]   Module[208]: name='/share/fonts/unifont.hex' cmdline='' size=8355852 bytes
[55680542907] [[35mTRACE[0m] [kernel] [CPU0]   Module[209]: name='/etc/locale.conf' cmdline='' size=85 bytes
[55681533303] [[35mTRACE[0m] [kernel] [CPU0]   Module[210]: name='/etc/profile' cmdline='' size=68 bytes
[55682411433] [[35mTRACE[0m] [kernel] [CPU0]   Module[211]: name='/etc/motd' cmdline='' size=610 bytes
[55683285108] [[35mTRACE[0m] [kernel] [CPU0]   Module[212]: name='/etc/fstab' cmdline='' size=127 bytes
[55684177065] [[35mTRACE[0m] [kernel] [CPU0]   Module[213]: name='/etc/hostname' cmdline='' size=8 bytes
[55685243262] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[56095705149] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[56097694158] [[34mDEBUG[0m] [kernel] [CPU0] Found init module: /bin/sprout (cmdline: 'init'), loading...
[56100036729] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/sprout' (len=89912, base=0x200000)
[56101214994] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[56131072668] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[56137575747] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[56151889101] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [49, 3b, 6e, 18, 75, 13, 49, 8d]
[56167292049] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20c000 exec=false
[56170923006] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20f000 exec=false
[56178232044] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x210000 filesz=0 memsz=0 align=1
[56191798377] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='load_init_module' elapsed_ticks=92095344 elapsed_us=46047 total_ticks=1911297861 total_us=955648
[56196586017] [[34mDEBUG[0m] [kernel] [CPU0] Warning: Module registry page overflow, truncating list.
[56197588293] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='prepare_boot_registry' elapsed_ticks=4309371 elapsed_us=2154 total_ticks=1917142788 total_us=958571
[56199014421] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='map_boot_registry' elapsed_ticks=309969 elapsed_us=154 total_ticks=1918586010 total_us=959293
[56201017884] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='activate_init_aspace' elapsed_ticks=716562 elapsed_us=358 total_ticks=1920496281 total_us=960248
[56202398076] [[34mDEBUG[0m] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[56203250103] [[34mDEBUG[0m] [kernel] [CPU0] Spawning init process...
[56206652469] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb006a9a0 arg=0xffffffffb0020a20
[56240734341] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[56253772377] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=5
[56254891011] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[56255897511] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='spawn_init_task' elapsed_ticks=52699482 elapsed_us=26349 total_ticks=1975442997 total_us=987721
[56257263678] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[56521724457] [[34mDEBUG[0m] [kernel::sched::lifecycle] [CPU0] SCHED: early-boot bringup complete; resuming steady-state SMP scheduling
[56525210346] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='end_bringup' elapsed_ticks=4240071 elapsed_us=2120 total_ticks=2244741180 total_us=1122370
[56528804211] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[56972109216] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=2691310677 elapsed_us=1345655
[56974837062] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[57008072451] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[57010295133] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] ENTER_USER: TID=5 entry_pc=200000 user_sp=800000
[57107686746] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=0
[57119080491] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=20
[57146172963] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[57150504213] [[34mDEBUG[0m] [sprout] [CPU0] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7fef70 rip=0x2032e5 rflags=0x202
[57154249581] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[57158345013] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: About to create Supervisor...
[57164562840] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Listing /bin directory...
[57169721235] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/bin' tid=5
[57233604780] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=3
[57247499859] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[57304222206] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Parsed command line: '    '
[57309569823] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Supervisor created, calling run_forever...
[57313552989] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[57338646849] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[57342301698] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[57345217677] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/run/sprout/shell' tid=5
[57354389829] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/etc/default/shell' tid=5
[57417644262] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/bin/sh' tid=5
[57434214453] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=3
[57438770796] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[57448953276] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[57456635115] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[57508749705] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN: page cache miss for '/bin/sh', caching 127864 bytes
[57517853778] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'sh' (len=127864, base=0x200000)
[57519149556] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[57520915320] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[57521916441] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[57525479253] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [0f, 84, 32, 06, 00, 00, 48, 8b]
[57542300442] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=214000 exec=false
[57552225555] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=217000 exec=false
[57558322008] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x218000 filesz=0 memsz=0 align=1
[57569278635] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /bin/sh
[57579406566] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb00a3d40 arg=0xffffffffb0074560
[57590245284] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=6, applying inserts
[57591558057] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[57592375632] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=6
[57593419125] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[57595074438] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[57620141931] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SETUP_STDIO: stdin=Fd(3) stdout=Fd(4) stderr=Fd(5)
[57649060425] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 6
[57664040280] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Task 6 woken, restoring IRQs
[57664940784] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Done for /bin/sh
[57674675586] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[57680296707] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[57686015904] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=6 entry_pc=200000 user_sp=800000
[57689534199] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=0
[57691831560] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=100
[57684439362] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[57701716017] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=3
[57702137262] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[57712231533] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=4
[57719126355] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/run/motd' tid=6
[57721162389] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=5
[57730008930] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[57741357168] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/etc/motd' tid=6
[57753298878] [[34mDEBUG[0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=36135
[57759548088] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=6
[57774230778] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=610
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
[57795092289] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/etc/profile' tid=6
[57855382464] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=6
[57875471445] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=97
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [57887615445] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=6
[?25h[57891171657] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[57921454140] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[57926994741] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[57937469634] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/cambium' (len=144768, base=0x200000)
[57938695122] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[57941035515] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[57942210645] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[57945732273] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [8d, 3c, f0, 48, 83, c7, 70, 48]
[57968475807] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=217000 exec=false
[57974507085] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[57977088873] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x21b000 filesz=0 memsz=0 align=1
[57987845124] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb00bd7a0 arg=0xffffffffb00ad440
[57991036389] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[57991952799] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=7
[57993017544] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[58009704027] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 7 → task '/bin/cambium' (pid=7 from boot module)
[58019736423] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned cambium (PID=7)
[58020960954] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[58021961085] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=7 entry_pc=200000 user_sp=800000
[58024599699] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=0
[58026239898] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=100
[58029884022] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[58038135969] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[58041852099] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Deferring netd until /dev/net/virtio0/rx is ready...
[58044570045] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[58047642048] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[58050371511] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[58053233667] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[58057899933] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[58067134059] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[58068930777] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers' tid=7
[58075774383] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_readdir: fd=3 len=4096
[58078284792] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_readdir: natural phase begin offset=0
[58082497308] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN: page cache miss for '/bin/bristle', caching 68744 bytes
[58088950392] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'bristle' (len=68744, base=0x200000)
[58090128756] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[58091890428] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[58092834492] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[58096196565] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [cc, fc, ff, ff, 45, 31, ff, e9]
[58103631663] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=209000 exec=false
[58105569027] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20b000 exec=false
[58107967533] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20c000 filesz=0 memsz=0 align=1
[58124762916] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /bin/bristle
[58128275106] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb00e9380 arg=0xffffffffb00d8e80
[58130797593] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=8, applying inserts
[58131787923] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[58134599952] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=8
[58139074587] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[58139985981] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[58145287332] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SETUP_STDIO: stdin=Null stdout=Inherit stderr=Inherit
[58149132393] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[58153343259] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 8
[58155441399] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ahci_disk' tid=7
[58155733449] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Task 8 woken, restoring IRQs
[58157034705] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Done for /bin/bristle
[58157996490] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/bristle' TID=8 PID=8
[58159110636] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[58160015364] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=8 entry_pc=200000 user_sp=800000
[58161980250] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=0
[58163993877] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=100
[58168091091] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[58184339400] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[58187702529] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[58191640221] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[58194308733] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/fb0' tid=5
[58195576692] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_open: path='/dev/fb0' len=8 flags=0x0
[58200299652] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] devfs: lookup entry path='fb0' len=3
[58202155341] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] devfs: dynamic registry hit path='fb0'
[58204361424] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/session/active_ui' tid=8
[58205786595] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_open: resolved node for /dev/fb0 mode=0o20666 size=32 ino=4
[58207240278] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_open: process info present for /dev/fb0
[58208561367] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_open: handle_table.open('/dev/fb0') -> 3
[58218186708] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] FbNode::read: off=0 n=32 buf_len=32 total=32
[58220814696] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=3
[58223486277] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=3
[58226199636] [[35mTRACE[0m] [bristle] [CPU2] bristle: active_ui set to 'bloom'
[58228501221] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[58232715255] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/drivers/display_virtio_gpu' tid=5
[58237910808] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/run/bristle/pid' tid=8
[58240003338] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_close: fd=3
[58243034619] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[58246352274] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[58253315802] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=4096
[58256702526] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(0)
[58266012519] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=1 port_id=PortId(0) mode=Write
[58268886225] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=2 port_id=PortId(0) mode=Read
[58274937798] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[58278566082] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=4096
[58280143482] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(1)
[58281240765] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=3
[58281377286] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=3 port_id=PortId(1) mode=Write
[58282783218] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=4 port_id=PortId(1) mode=Read
[58284491628] [[32mINFO [0m] [bristle] [CPU2] bristle: published pid 8 to /run/bristle/pid
[58287015930] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=4096
[58287730479] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(2)
[58288743084] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=1 port_id=PortId(2) mode=Write
[58289929995] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=2 port_id=PortId(2) mode=Read
[58292185413] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=4096
[58293111624] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(3)
[58294236825] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(3) mode=Write
[58295011632] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=4 port_id=PortId(3) mode=Read
[58305594204] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/run/bristle/kbd_in' tid=8
[58311282513] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[58314516909] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=3
[58316815392] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/run/bristle/mouse_in' tid=8
[58324909665] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=3
[58325557950] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[58328260782] [[32mINFO [0m] [bristle] [CPU2] bristle: published device handles kbd_in=1 mouse_in=3
[58330213854] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/proc/self/inbox' tid=8
[58339991820] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[58348579014] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=2 -> fd=4 node=0xffffffffb01f9330 port=0xffffffffb00f7390
[58354008636] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=4 -> fd=5 node=0xffffffffb021d530 port=0xffffffffb00f9110
[58357300848] [[32mINFO [0m] [bristle] [CPU2] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[58383616731] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN: page cache miss for '/drivers/display_virtio_gpu', caching 147560 bytes
[58386845319] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'display_virtio_gpu' (len=147560, base=0x200000)
[58392115980] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[58394093604] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[58395393210] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[58405772139] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [00, 00, 00, ba, 12, 00, 00, 00]
[58424150598] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=216000 exec=false
[58430900352] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[58434264867] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x21c000 filesz=0 memsz=0 align=1
[58442868891] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/display_virtio_gpu
[58444341384] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb022dac0 arg=0xffffffffb021d820
[58446973398] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=9, applying inserts
[58447952838] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[58448786022] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=9
[58450153806] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[58451123511] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[58454703450] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SETUP_STDIO: stdin=Null stdout=Inherit stderr=Inherit
[58462172439] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 9
[58467419439] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Task 9 woken, restoring IRQs
[58468232790] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Done for /drivers/display_virtio_gpu
[58469068086] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/drivers/display_virtio_gpu' TID=9 PID=9
[58471312053] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x3
[58472274729] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=9 entry_pc=200000 user_sp=800000
[58475220474] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=0
[58477800447] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=100
[58479142425] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[58480670919] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[58481977323] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0 mounted=false, service_ready=false)...
[58483339596] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready for bloom yet
[58484501130] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[58484684577] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[58486515054] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Starting VFS-native VirtIO GPU driver...
[58487616627] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[58487758131] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: boot_arg=3
[58489040610] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[58489682196] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Mapping bootstrap memfd 3 size=4096...
[58490603556] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/proc/self/inbox' tid=5
[58495174353] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: vm_map success at 0x400000001000
[58496767758] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[58497483429] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Recovered handles: req_read=2, resp_write=3, svc=0, id=322371585
[58499068122] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3, svc=0, id=322371585)
[58500490653] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Scanning /sys/devices for PCI GPU...
[58501763199] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices' tid=9
[58504563579] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices'
[58510634490] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_readdir: fd=4 len=4096
[58512104409] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_readdir: natural phase begin offset=0
[58517265312] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU3] sysfs: readdir found 12 slots
[58522355958] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU3] sysfs: readdir wrote 173 bytes
[58524501453] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[58524577221] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[58531605462] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[58533375714] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ata_disk' tid=7
[58565328096] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/platform-fb000000/vendor' tid=9
[58568584206] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/platform-fb000000/vendor'
[58573729467] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'platform-fb000000/vendor'
[58578931554] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[58582439883] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/platform-fb000000/device' tid=9
[58584945639] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/platform-fb000000/device'
[58586967549] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'platform-fb000000/device'
[58590520824] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[58594588008] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/platform-fb000000/class' tid=9
[58597415151] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/platform-fb000000/class'
[58599544872] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'platform-fb000000/class'
[58603198632] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[58605422766] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/vendor' tid=9
[58607756823] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:00.0/vendor'
[58609566312] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:00.0/vendor'
[58612972242] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[58614636498] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/device' tid=9
[58616877594] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:00.0/device'
[58618508718] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:00.0/device'
[58621771824] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[58623375591] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/class' tid=9
[58625644077] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:00.0/class'
[58627583289] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:00.0/class'
[58632257409] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[58633978557] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/vendor' tid=9
[58636185993] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/vendor'
[58637842197] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:01.0/vendor'
[58640839554] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[58642432992] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/device' tid=9
[58644539316] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/device'
[58646247627] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:01.0/device'
[58649364906] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[58651000914] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/class' tid=9
[58653123804] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/class'
[58655659557] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: matched device file 'pci-0000:00:01.0/class'
[58661033541] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[58672151142] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: task 9 claimed device 'pci-0000:00:01.0' (handle 0)
[58674753885] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/virtio/common_bar' tid=9
[58677452361] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/common_bar'
[58687812942] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[58690805778] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/virtio/common_offset' tid=9
[58693488678] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/common_offset'
[58700662449] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[58703104053] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/virtio/notify_bar' tid=9
[58705557900] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/notify_bar'
[58708648218] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[58710953268] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[58712056722] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.Ide' start='thingos_driver_start_safe'
[58712696559] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/virtio/notify_offset' tid=9
[58712957655] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/chime' tid=7
[58714972833] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/notify_offset'
[58720097073] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[58722590949] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/virtio/notify_multiplier' tid=9
[58724968368] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/notify_multiplier'
[58732397592] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[58734880644] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[58738649376] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: mapping BAR2 (phys=0xc000000000, size=0x4000) for task 9
[58742528757] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10000000
[58747725234] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: DMA alloc 1 pages phys=0x2329000 -> user_va=0x10004000
[58755489639] [[32mINFO [0m] [virtio_gpu] [CPU3] virtio_gpu: device features=0x30000002 virgl=false
[58762211178] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU3] DEVICE: DMA alloc 4 pages phys=0x232a000 -> user_va=0x10005000
[58767314628] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: GPU initialized successfully
[58768872855] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Virgl 3D not supported, using 2D only
[58770209850] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/sys/firmware/framebuffer' tid=9
[58772566743] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU3] sysfs: lookup path='firmware/framebuffer'
[58778585646] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=4
[58784450571] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=1
[58843499220] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[58845002997] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[58847497731] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[58855748391] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[58859779770] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0 mounted=false, service_ready=false)...
[58861516626] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready for bloom yet
[58862747163] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[58863886092] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[58864961331] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[58866591960] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[58904922780] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[58910718141] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[58912653624] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/display_bootfb' tid=7
[59116197987] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[59117877357] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpus_online=4
[59120024106] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[59119947678] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=0 curr=Some(0) runq=0 runq_avg=1 runq_samples=44 ctxsw=16 idle2busy=0 tick=44 ipi=0 enq=50 deq=49 wake=2 lock_miss=0 lock_pending=0 lock_blocked=0
[59121878607] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/display_virtio_gpu' tid=7
[59124248139] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=1 curr=Some(7) runq=0 runq_avg=0 runq_samples=93 ctxsw=1 idle2busy=1 tick=88 ipi=0 enq=4 deq=3 wake=2 lock_miss=0 lock_pending=0 lock_blocked=0
[59131084023] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=2 curr=Some(3) runq=0 runq_avg=0 runq_samples=123 ctxsw=2 idle2busy=1 tick=103 ipi=1 enq=2 deq=1 wake=2 lock_miss=0 lock_pending=0 lock_blocked=0
[59137664949] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=3 curr=Some(9) runq=0 runq_avg=0 runq_samples=101 ctxsw=3 idle2busy=2 tick=101 ipi=2 enq=4 deq=3 wake=4 lock_miss=0 lock_pending=0 lock_blocked=0
[59187202569] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[59188648365] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[59189819997] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[59190929523] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[59192152899] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0 mounted=false, service_ready=false)...
[59193348951] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready for bloom yet
[59194353471] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[59195377758] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[59196356934] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[59197346802] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[59373545154] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[59381886432] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[59384630976] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/hdaudio' tid=7
[59401722897] [[34mDEBUG[0m] [kernel::syscall::handlers::memory] [CPU3] sys_vm_map: large backing file mapping offset=0 len=8294400 total_size=8294400
[59518520391] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[59519943483] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[59521218504] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[59522375616] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[59523534873] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0 mounted=false, service_ready=false)...
[59524853454] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready for bloom yet
[59525997003] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[59527160880] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[59528271231] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[59529737421] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[59567045736] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: frame pool ready (1 buffer)
[59573825916] [[34mDEBUG[0m] [kernel::ipc] [CPU3] CREATE_PORT: capacity=65536
[59575000485] [[34mDEBUG[0m] [kernel::ipc] [CPU3] CREATE_PORT: appended id=PortId(4)
[59576144892] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU3] ALLOC_HANDLE: handle=5 port_id=PortId(4) mode=Write
[59580533133] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU3] ALLOC_HANDLE: handle=6 port_id=PortId(4) mode=Read
[59582849370] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU3] BRIDGE: handle=5 -> fd=5 node=0xffffffffb00d8730 port=0xffffffffb010a150
[59586191478] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/proc/5/inbox' tid=9
[59630806323] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[59637388404] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[59636125230] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU3] SENDMSG: thing=6 data_len=44 caps_len=1 node=Pointer { addr: 0xffffffffb010a5d0, metadata: DynMetadata(0xffffffff802c6ec0) }
[59639388270] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/pci_stubd' tid=7
[59648057601] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_BIND_READY (ID: 322371585)
[59649777792] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Waiting for BIND_ASSIGNED...
[59655242526] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU0] RECVMSG: thing=4 data_len=44 caps_len=1 node=Pointer { addr: 0xffffffffb02370b0, metadata: DynMetadata(0xffffffff802c6ec0) }
[59665146189] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: BIND_READY from instance_id=0x13370001
[59667009039] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Registering DISPLAY_CARD at /dev/display/card0 (handle=5)
[59680688562] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=262400
[59681693412] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(5)
[59684785908] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=7 port_id=PortId(5) mode=Write
[59693634000] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] vfs: mounted userland provider at /dev/display/card0 (flags: 0x0) port=0xffffffffb010a150
[59698628385] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Mounted /dev/display/card0 successfully
[59700999072] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Handshake complete — task 'display' marked ready
[59712988566] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[59715495906] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU3] SENDMSG: thing=6 data_len=44 caps_len=0 node=Pointer { addr: 0xffffffffb010a5d0, metadata: DynMetadata(0xffffffff802c6ec0) }
[59718644040] [[34mDEBUG[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: Sent MSG_SERVICE_READY.
[59720041788] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Sent MSG_BIND_ASSIGNED to display pid 9 (res=Ok(()))
[59720333772] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU3] BRIDGE: handle=2 -> fd=7 node=0xffffffffb00c6c50 port=0xffffffffb00f3890
[59721425148] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[59721964599] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU3] BRIDGE: handle=3 -> fd=8 node=0xffffffffb011b4f0 port=0xffffffffb00f5610
[59723483259] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU3] BRIDGE: handle=6 -> fd=9 node=0xffffffffb00c6c30 port=0xffffffffb010a150
[59726364060] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU0] RECVMSG: thing=4 data_len=44 caps_len=0 node=Pointer { addr: 0xffffffffb02370b0, metadata: DynMetadata(0xffffffff802c6ec0) }
[59726739666] [[32mINFO [0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS provider loop online
[59746557651] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: SERVICE_READY from instance_id=0x13370001
[59748430368] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Service 'display' reported ready
[59749687998] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display service is ready at /dev/display/card0
[59750931867] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[59967452523] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[59970697248] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[59972517792] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ps2_kbd' tid=7
[60252337992] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[60258110583] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[60260292444] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ps2_mouse' tid=7
[60414619848] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[60449882988] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[60463198653] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[60496471134] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[60500184624] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0 mounted=true, service_ready=true)...
[60533590788] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/bloom' (len=462464, base=0x200000)
[60547231800] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[60559051608] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[60560928648] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[60564483375] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [29, 44, 24, 60, 48, 8b, 44, 24]
[60606747597] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[60626900202] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[60636472809] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/rtc_cmos' tid=7
[60706505937] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=253000 exec=false
[60718650960] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=25a000 exec=false
[60724124736] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x25d000 filesz=0 memsz=0 align=1
[60731784003] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb012bb40 arg=0xffffffffb00c6b00
[60737498118] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[60740886426] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=10
[60746630109] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[60760148229] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 10 → task '/bin/bloom' (pid=10 from boot module)
[60764351043] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned bloom (PID=10)
[60810714888] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[60820949112] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[60831996060] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[60832176768] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=10 entry_pc=200000 user_sp=800000
[60845759436] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=0
[60851006964] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=100
[60855954819] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[60857341017] [[32mINFO [0m] [bloom] [CPU1] bloom: ENTERING MAIN
[60867072123] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[61029508254] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[61033394037] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[61035222600] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/rtl8168d' tid=7
[61134914577] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpus_online=4
[61142244108] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=0 curr=Some(0) runq=0 runq_avg=1 runq_samples=96 ctxsw=28 idle2busy=0 tick=80 ipi=0 enq=104 deq=103 wake=7 lock_miss=0 lock_pending=0 lock_blocked=0
[61143616743] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=1 curr=Some(7) runq=1 runq_avg=0 runq_samples=97 ctxsw=3 idle2busy=1 tick=101 ipi=0 enq=10 deq=8 wake=4 lock_miss=0 lock_pending=0 lock_blocked=0
[61144804314] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=2 curr=Some(3) runq=0 runq_avg=0 runq_samples=187 ctxsw=2 idle2busy=1 tick=148 ipi=1 enq=2 deq=1 wake=2 lock_miss=0 lock_pending=0 lock_blocked=0
[61145992149] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=3 curr=Some(9) runq=0 runq_avg=0 runq_samples=230 ctxsw=39 idle2busy=20 tick=138 ipi=14 enq=78 deq=77 wake=39 lock_miss=0 lock_pending=0 lock_blocked=0
[61216729035] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[61224770475] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[61229385888] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[61245011982] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[61250997852] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[61270872300] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[61283053260] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[61312909647] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[61314537009] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[61351344120] [[32mINFO [0m] [bloom] [CPU1] bloom: compositor service starting
[61368148644] [[34mDEBUG[0m] [bloom] [CPU1] bloom: connect try 0...
[61374736104] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/dev/display/card0' tid=10
[61388852283] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=Lookup len=4 tid=10
[61426809279] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=Lookup
[61436452539] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: vfs_lookup path=''
[61437452967] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[61458176175] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/virtio_netd' tid=7
[61469940741] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=Lookup req_id=1
[61494539271] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=Lookup req_id=1
[61737014361] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[61757795418] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[61782134832] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[61788132450] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[61792448322] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=Lookup id=1 -> OK(8)
[61802470059] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[61805809758] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[61808425338] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[61827137163] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[61863777360] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=48 tid=10
[61892519238] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[61895123895] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_GET_INFO requested
[61913258418] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: Returning dimensions 1920x1080
[61919415228] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response begin op=DeviceCall req_id=2
[61930054395] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC response sent op=DeviceCall req_id=2
[62027052714] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: response op=DeviceCall id=2 -> OK(48)
[62187120732] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[62204963073] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[62211166545] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[62214105063] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[62217337941] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[62220633123] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[62223814950] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[62227253583] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[62231402079] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[62237297628] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[62249286990] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/virtio_sound' tid=7
[62399847444] [[32mINFO [0m] [bloom] [CPU1] bloom: connected to /dev/display/card0 on try 0
[62455628565] [[32mINFO [0m] [bloom] [CPU1] bloom: output0 1920x1080 @ 60000mHz
[62466454215] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver does not support VBLANK
[62468054814] [[32mINFO [0m] [bloom] [CPU1] bloom: display driver supports linear dmabuf import
[62479041735] [[32mINFO [0m] [bloom] [CPU1] bloom: creating service port...
[62490013509] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=65536
[62490913782] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(6)
[62493315159] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=5 port_id=PortId(6) mode=Write
[62499535065] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=6 port_id=PortId(6) mode=Read
[62510786283] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/services/bloom' tid=10
[62556061755] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[62563454646] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[62611631478] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[62632294692] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/components' tid=10
[62690805078] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[62699863644] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[62696551599] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[62713641474] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/wayland/status' tid=10
[62738899245] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[62751469605] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/lib/libpistil.so' tid=10
[62788703175] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[62809231551] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[62830700724] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/clock' (len=82936, base=0x200000)
[62845317018] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[62857587045] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[62859200085] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[62878667214] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [48, 83, c4, 58, c3, cc, cc, cc]
[62912135088] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20c000 exec=false
[62914529700] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20e000 exec=false
[62917578438] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x210000 filesz=0 memsz=0 align=1
[62929201500] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb0165b60 arg=0xffffffffb00c6b00
[63050793333] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[63051712218] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=11
[63053080299] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[63059685942] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 11 → task '/bin/clock' (pid=11 from boot module)
[63159776394] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpus_online=4
[63162575388] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[63163885620] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=11 entry_pc=200000 user_sp=800000
[63174261744] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=0 curr=Some(5) runq=1 runq_avg=1 runq_samples=152 ctxsw=41 idle2busy=0 tick=123 ipi=0 enq=164 deq=162 wake=11 lock_miss=0 lock_pending=0 lock_blocked=0
[63191698977] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=1 curr=Some(10) runq=1 runq_avg=0 runq_samples=103 ctxsw=8 idle2busy=1 tick=113 ipi=1 enq=17 deq=15 wake=6 lock_miss=0 lock_pending=0 lock_blocked=0
[63206442123] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=2 curr=Some(11) runq=0 runq_avg=0 runq_samples=243 ctxsw=3 idle2busy=2 tick=185 ipi=1 enq=3 deq=2 wake=4 lock_miss=0 lock_pending=0 lock_blocked=0
[63207972432] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=3 curr=Some(4) runq=0 runq_avg=0 runq_samples=315 ctxsw=92 idle2busy=46 tick=174 ipi=32 enq=109 deq=108 wake=89 lock_miss=0 lock_pending=0 lock_blocked=0
[63197640693] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=11 buf_len=0
[63216206361] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=11 buf_len=100
[63225589383] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=6
[63290633769] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned clock (PID=11)
[63318403962] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[63327205986] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[63423053001] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=6
[63501196704] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[63507382257] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[63622470780] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=6
[63674991963] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[63705589266] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[63712439736] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[63717896649] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[63721064418] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[63726102429] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/wayland_hello' (len=82360, base=0x200000)
[63727440546] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[63729653394] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[63734409057] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[63745392645] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [48, 83, c4, 58, c3, cc, cc, cc]
[63755343234] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20c000 exec=false
[63764926764] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20e000 exec=false
[63774824520] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x210000 filesz=0 memsz=0 align=1
[63797414670] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb0175b60 arg=0xffffffffb00c6b00
[63809672124] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[63814262358] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=12
[63822578028] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[63829431831] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 12 → task '/bin/wayland_hello' (pid=12 from boot module)
[63838459839] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[63841937907] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=12 entry_pc=200000 user_sp=800000
[63852101082] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=12 buf_len=0
[63867197196] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=12 buf_len=100
[63910610379] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=6
[63917084913] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawned wayland_hello (PID=12)
[63984112533] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=6
[63990315510] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[63993570168] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[63996265212] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/device_snapshot' tid=7
[63996306132] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[63999007611] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='device_snapshot'
[64001390343] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[64010525238] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[64028732955] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:00.0 kind=pci_device vendor=0x8086 device=0x29c0 class=0x060000 present=true
[64040283153] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[64109243187] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=6
[64172894742] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=6
[64230109680] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:01.0 kind=pci_device vendor=0x1af4 device=0x1050 class=0x030000 present=true
[64239418188] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:02.0 kind=pci_device vendor=0x1af4 device=0x1000 class=0x020000 present=true
[64244538732] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: isolation mode skipping driver '/drivers/virtio_netd' class=Net for pci-0000:00:02.0
[64246741317] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:03.0 kind=pci_device vendor=0x1af4 device=0x1059 class=0x040100 present=true
[64248690726] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: isolation mode skipping driver '/drivers/virtio_sound' class=Audio for pci-0000:00:03.0
[64249892652] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:1f.0 kind=pci_device vendor=0x8086 device=0x2918 class=0x060100 present=true
[64251142758] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:1f.2 kind=pci_device vendor=0x8086 device=0x2922 class=0x010601 present=true
[64254260994] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.2/status' tid=7
[64256860866] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/status'
[64264317843] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/status'
[64287492852] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=6
[64296270588] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil background renderer loaded from /lib/libpistil.so
[64298627349] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil font text renderer loaded with default /share/fonts/Inter-Regular.ttf
[64303930911] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil symbol renderer loaded with /share/fonts/NotoSansSymbol2-Regular.ttf
[64305629685] [[32mINFO [0m] [bloom::render] [CPU1] bloom: pistil nine-slice shadow renderer loaded
[64308322386] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/desktop/theme' tid=10
[64318247598] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/session/desktop/theme' tid=10
[64326029955] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[64328078529] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=7
[64329182610] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[64330457730] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[64331061927] [[32mINFO [0m] [bloom] [CPU1] bloom: initial theme configured Solarized Warm
[64331557389] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[64332674835] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[64333821651] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[64334921343] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[64335964902] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[64365122052] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=6
[64457670486] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=6
[64523215944] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=6
[64650613335] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=6
[64686345471] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=6
[64684940562] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[64696268604] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[64704847647] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[64719038472] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[64724266926] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[64725663981] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[64726878876] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[64728037143] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[64819631085] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=6
[64857889899] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=6
[64975804806] [[34mDEBUG[0m] [kernel::syscall::handlers::memory] [CPU1] sys_vm_map: large backing file mapping offset=0 len=8294400 total_size=8294400
[64978799028] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=6
[65019552015] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=6
[65088774300] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor tick...
[65091906957] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bristle_if_needed
[65097208341] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_display_if_needed
[65100249357] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_bloom_if_ready
[65103043368] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_wayland_hello_if_ready
[65107121574] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: spawn_clock_if_ready
[65110179354] [[35mTRACE[0m] [sprout::supervisor] [CPU0] SPROUT: Loop iteration: tick complete
[65118000882] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[65136877080] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=3
[65150354148] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=6
[65159703246] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: page cache miss for '/drivers/ahci_disk', caching 53784 bytes
[65162299785] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'ahci_disk' (len=53784, base=0x200000)
[65165070927] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[65169582852] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[65173573179] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=200000 exec=true
[65176819488] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   COPIED at 0x201420: [24, 0c, e8, 59, 0e, 00, 00, bf]
[65181135228] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=207000 exec=false
[65182525584] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=208000 exec=false
[65184531258] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x209000 filesz=0 memsz=0 align=1
[65188774497] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=6
[65205422073] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[65207115963] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/ahci_disk
[65211402828] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb019cca0 arg=0xffffffffb017d060
[65216560200] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=13, applying inserts
[65217485025] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[65218265442] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=13
[65219327712] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[65224674273] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[65228207550] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[65238142497] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 13
[65243253009] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 13 woken, restoring IRQs
[65244299307] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/ahci_disk
[65249183010] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=13 PID=13
[65253329130] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/ahci_disk for pci-0000:00:1f.2 (entry='thingos_driver_start_safe', pid=13)
[65258629590] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: sent DRIVER_READY to Sprout (pid=5) for driver pid=13
[65263202598] [[35mTRACE[0m] [kernel::syscall::handlers::socket] [CPU0] RECVMSG: thing=4 data_len=40 caps_len=0 node=Pointer { addr: 0xffffffffb02370b0, metadata: DynMetadata(0xffffffff802c6ec0) }
[65264262954] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x3
[65265525039] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=13 entry_pc=201000 user_sp=800000
[65267130786] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=13 buf_len=0
[65269272024] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=13 buf_len=100
[65273843448] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Starting AHCI VFS driver
[65279792556] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: DRIVER_READY from PID 13 — no matching ManagedTask (already exited?)
[65283246567] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[65312500737] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=6
[65341251129] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 'pci-0000:00:1f.2' (handle 1)
[65346793215] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Claimed device /sys/devices/pci-0000:00:1f.2
[65346993162] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_close: fd=6
[65348084373] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: mapping BAR5 (phys=0x80880000, size=0x1000) for task 13
[65349528222] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10009000
[65354051466] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 1 pages phys=0x35ef000 -> user_va=0x1000a000
[65362579788] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=65536
[65363810886] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(7)
[65365150257] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=1 port_id=PortId(7) mode=Write
[65366012448] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=2 port_id=PortId(7) mode=Read
[65376571029] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=262400
[65379938844] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(8)
[65383084965] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=3 port_id=PortId(8) mode=Write
[65392289490] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] vfs: mounted userland provider at /dev/storage/atapi2 (flags: 0x0) port=0xffffffffb01a48b0
[65397841377] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Mounted atapi2 at /dev/storage/atapi2
[65399375712] [[32mINFO [0m] [ahci_disk] [CPU1] AHCI: Provider loop online at /dev/storage/atapi2
[65419110174] [[34mDEBUG[0m] [kernel::vfs::provider] [CPU1] VFS_RPC: request op=DeviceCall len=88 tid=10
[65427783762] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=pci-0000:00:1f.3 kind=pci_device vendor=0x8086 device=0x2930 class=0x0c0500 present=true
[65430029016] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered device slot=isa-0070 kind=dev.rtc.Cmos vendor=0x0000 device=0x0000 class=0x000000 present=true
[65434249188] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0070/status' tid=7
[65436102831] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/status'
[65437947630] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/status'
[65443121073] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_close: fd=4
[65455856763] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: page cache miss for '/drivers/rtc_cmos', caching 45904 bytes
[65457558903] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] display_virtio_gpu: VFS RPC op=DeviceCall
[65459478084] [[35mTRACE[0m] [display_virtio_gpu] [CPU3] DISP: DISPLAY_OP_IMPORT_BUFFER requested: memfd=10, size=1920x1080
[65459239527] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'rtc_cmos' (len=45904, base=0x200000)
[65462014101] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[65463996708] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[65462456631] [[34mDEBUG[0m] [kernel::syscall::handlers::memory] [CPU3] sys_vm_map: large backing file mapping offset=0 len=8294400 total_size=8294400
[65465017134] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=200000 exec=true
[65475670821] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   COPIED at 0x201420: [0f, 84, ea, 03, 00, 00, b8, 00]
[65485073049] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=206000 exec=false
[65486908608] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=207000 exec=false
[65489408820] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x208000 filesz=0 memsz=0 align=1
[65490529401] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_close: fd=6
[65498203749] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[65499239751] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/rtc_cmos
[65500803522] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff800532f0 kstack_top=0xffffffffb01f3ca0 arg=0xffffffffb01b1240
[65503732998] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=14, applying inserts
[65504507739] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[65505204501] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=14
[65506482393] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[65507322375] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[65511138957] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[65517014046] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 14
[65519328798] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 14 woken, restoring IRQs
[65521678233] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/rtc_cmos
[65522780763] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/rtc_cmos' TID=14 PID=14
[65524373640] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x4
[65525793564] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=14 entry_pc=201000 user_sp=800000
[65527767261] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=14 buf_len=0
[65530083828] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=14 buf_len=100
[65532933081] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/rtc_cmos for isa-0070 (entry='thingos_driver_start_safe', pid=14)
[65537788305] [[34mDEBUG[0m] [rtc_cmos] [CPU2] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffef0 rip=0x201c27 rflags=0x206
[65539459524] [[34mDEBUG[0m] [rtc_cmos] [CPU2] Starting... arg=4
[65542581753] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: sent DRIVER_READY to Sprout (pid=5) for driver pid=14
[65543900466] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: discovered d
```
</details>
