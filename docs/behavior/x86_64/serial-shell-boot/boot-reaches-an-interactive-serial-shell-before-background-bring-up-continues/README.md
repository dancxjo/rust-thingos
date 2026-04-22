# ❌ Scenario: Boot reaches an interactive serial shell before background bring-up continues

> Last run: 2026-04-22 16:43:24

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ❌ | 1001ms | - [📜](./01/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[24054546813] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] enter
[24064289898] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] phys_memory_map ok
[24068198781] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] modules ok
[24069405063] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] phys_to_virt_offset ok
[24070380477] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] memory map logging done
[24095507568] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] boot_frame_alloc init ok
[24289293435] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] frame allocator build ok
[24290513478] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] frame allocator log ok
[24291861495] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] FRAME_ALLOCATOR init ok
[24293101833] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] tasking init ok
[24294154170] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[24318459660] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[24319613406] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] enter
[24320581692] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] set expand hook
[24321766062] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] expand hook ok
[24322837242] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock begin
[24324182883] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock ok
[24325274754] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] reserve_region begin
[24355067517] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] reserve_region ok
[24356231031] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] inner allocator init begin
[24359609901] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] inner allocator init ok
[24360863868] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] heap top store ok
[24361952010] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] init done
[24363071436] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[24389532618] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] set_boot_fb begin
[24390710850] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] devfs: set_boot_fb width=1920 height=1080 pitch=7680 resource_id=0xfb000000
[24393451599] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] set_boot_fb ok
[24395027349] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs set_boot_fb elapsed_ticks=5027187 elapsed_us=2513
[24401253756] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs FbNode::new elapsed_ticks=4760151 elapsed_us=2380
[24402698034] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] register begin
[24407691924] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] register ok
[24408819666] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs register fb0 elapsed_ticks=6058239 elapsed_us=3029
[24410099439] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[24434521221] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[24437327475] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[24473539992] [[34mDEBUG[0m] [kernel] [CPU0] Seeding entropy pool...
[24474979617] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed begin
[24476241537] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] fill_entropy done
[24478947108] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] add_sample(timer) ok
[24480274302] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] mark_seeded(timer) ok
[24481863285] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[24483397851] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[24484594761] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed done
[24486172260] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] entropy::seed_from_hardware elapsed_ticks=10861752 elapsed_us=5430
[24487541364] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[24519570141] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[24550097847] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80016f50 kstack_top=0xffffffffb001f980 arg=0x0
[24553150248] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[24575171511] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[24576641595] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[24604279227] [[34mDEBUG[0m] [kernel] [CPU0] Initializing VFS...
[24608057727] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='set_cmdline' elapsed_ticks=2707914 elapsed_us=1353 total_ticks=3612543 total_us=1806
[24627614121] [[34mDEBUG[0m] [kernel::vfs] [CPU0] VFS: Created /dev/display directory
[24642507219] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted union filesystem at / (root)
[24645296412] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted devfs at /dev
[24646743198] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted procfs at /proc
[24648161472] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted sysfs at /sys
[24652557996] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /tmp
[24653771637] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /run
[24654879051] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /services
[24656076621] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /session
[24657364545] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /data
[24658263168] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='vfs_init' elapsed_ticks=48506106 elapsed_us=24253 total_ticks=54078321 total_us=27039
[24659975109] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[24686801172] [[34mDEBUG[0m] [kernel] [CPU0] Scanning PCI bus...
[24690774108] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x29c0 at 00:00.0 class=060000 id=1
[24709428909] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1050 at 00:01.0 class=030000 id=2
[24727735065] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1000 at 00:02.0 class=020000 id=3
[24729461856] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2918 at 00:1f.0 class=060100 id=4
[24740243847] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2922 at 00:1f.2 class=010601 id=5
[24743061948] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2930 at 00:1f.3 class=0c0500 id=6
[24746072637] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='scan_pci' elapsed_ticks=58442835 elapsed_us=29221 total_ticks=141875646 total_us=70937
[24747944100] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[24776239686] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='register_legacy_devices' elapsed_ticks=954294 elapsed_us=477 total_ticks=172039791 total_us=86019
[24778108410] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[24804338097] [[34mDEBUG[0m] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[24811314264] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[24812248296] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[24813585291] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[24814757583] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[24823827402] [[34mDEBUG[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[24826908744] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[24828174393] [[34mDEBUG[0m] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[24829327776] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[24831671997] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Registers initialized
[24833032686] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[24834489702] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[24836130033] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: All pins masked
[24838499070] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Init complete
[24873088548] [[34mDEBUG[0m] [bran::arch::x86_64::ioapic] [CPU0] LAPIC: calibrated 62462600 ticks/sec (delta=624626, ok=true) -> init_cnt=624626
[24875823324] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62462600 ticks/sec), init_cnt=624626 for 100Hz
[24877191207] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='setup_preemption_timer' elapsed_ticks=71782887 elapsed_us=35891 total_ticks=273000750 total_us=136500
[24878955420] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[24905415216] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[24906720663] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[24908475306] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[24911048118] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[24941069373] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[24942548697] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[24945894006] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] SMP: CPU 1 initializing preemption timer (vec=32 init_cnt=624626)
[24948381678] [[34mDEBUG[0m] [kernel] [CPU1] SMP: kernel_secondary_entry arg_cpu=1 runtime_cpu=1
[24949969902] [[34mDEBUG[0m] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[24951836481] [[34mDEBUG[0m] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[24955905216] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80016f50 kstack_top=0xffffffffb0032e40 arg=0x1
[24957829347] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Task 2 assigned to CPU 1
[24961762386] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 2 deferred inserts
[24963488088] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=1
[24964748160] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=2
[24967111092] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[24968352486] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[24969579261] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] SMP: CPU 2 initializing preemption timer (vec=32 init_cnt=624626)
[24972874410] [[34mDEBUG[0m] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[24973928529] [[34mDEBUG[0m] [kernel] [CPU2] SMP: kernel_secondary_entry arg_cpu=2 runtime_cpu=2
[24975131379] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: run_scheduler entry on CPU 1
[24976223976] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[24977349738] [[34mDEBUG[0m] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[24979268787] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: bootstrap_cpu start on CPU 1
[24980323335] [[34mDEBUG[0m] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[24982154769] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU2] INIT KERNEL CTX: entry=0xffffffff80016f50 kstack_top=0xffffffffb00445a0 arg=0x2
[24984003000] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] SCHED: Task 3 assigned to CPU 2
[24986061738] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Applying 1 deferred inserts
[24987189744] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle thread 2 and is now schedulable
[24988610460] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserting TID=3
[24989857761] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserts applied
[24990942834] [[34mDEBUG[0m] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[24992154957] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: run_scheduler entry on CPU 2
[24993901911] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: bootstrap_cpu start on CPU 2
[25007156229] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[25008595557] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] SMP: CPU 3 initializing preemption timer (vec=32 init_cnt=624626)
[25011940206] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[25013029998] [[34mDEBUG[0m] [kernel] [CPU3] SMP: kernel_secondary_entry arg_cpu=3 runtime_cpu=3
[25014270963] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[25015397847] [[34mDEBUG[0m] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[25016523873] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='start_secondary_cpus' elapsed_ticks=111266859 elapsed_us=55633 total_ticks=412262136 total_us=206131
[25018754442] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle thread 3 and is now schedulable
[25020777177] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[25023277389] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[25026306459] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80016f50 kstack_top=0xffffffffb0055d00 arg=0x3
[25028425323] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SCHED: Task 4 assigned to CPU 3
[25030552866] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[25032118419] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=4
[25034890419] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[25035947409] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[25037046969] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: run_scheduler entry on CPU 3
[25038156396] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: bootstrap_cpu start on CPU 3
[25039834479] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle thread 4 and is now schedulable
[25074291231] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[25105356012] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Enumerating 108 boot modules...
[25107683766] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[25139984661] [[34mDEBUG[0m] [kernel] [CPU0] Found init module: /bin/sprout (cmdline: 'init'), loading...
[25142158074] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/sprout' (len=85144, base=0x200000)
[25151304288] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[25166228769] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20f000 filesz=0 memsz=0 align=1
[25173447189] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='load_init_module' elapsed_ticks=31516749 elapsed_us=15758 total_ticks=569252607 total_us=284626
[25176183516] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='prepare_boot_registry' elapsed_ticks=924561 elapsed_us=462 total_ticks=571999164 total_us=285999
[25178320365] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='map_boot_registry' elapsed_ticks=153450 elapsed_us=76 total_ticks=574137729 total_us=287068
[25180962774] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='activate_init_aspace' elapsed_ticks=222519 elapsed_us=111 total_ticks=576736974 total_us=288368
[25183038837] [[34mDEBUG[0m] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[25184061408] [[34mDEBUG[0m] [kernel] [CPU0] Spawning init process...
[25186781862] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80050990 kstack_top=0xffffffffb00661e0 arg=0xffffffffb001fd20
[25202902230] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[25204085676] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=5
[25205123427] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[25206708714] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='spawn_init_task' elapsed_ticks=22688523 elapsed_us=11344 total_ticks=602517762 total_us=301258
[25208525991] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[25317710847] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED: early-boot bringup complete; resuming steady-state SMP scheduling
[25319159481] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='end_bringup' elapsed_ticks=1811304 elapsed_us=905 total_ticks=714968331 total_us=357484
[25320926301] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[25496329815] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=891927960 elapsed_us=445963
[25497741522] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[25515729954] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[25517356986] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] ENTER_USER: TID=5 entry_pc=200000 user_sp=800000
[25564412445] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=0
[25570923576] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=20
[25582875714] [[34mDEBUG[0m] [sprout] [CPU0] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7fef70 rip=0x20414c rflags=0x202
[25585167465] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[25586460966] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: About to create Supervisor...
[25587605901] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Listing /bin directory...
[25668834345] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Parsed command line: '    '
[25672108836] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: FORCING display=bootfb for diagnostic test!
[25673964096] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Supervisor created, calling run_forever...
[25675454409] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[25676876676] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[25678297161] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[25690605930] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU0] BootFs: EXACT match for 'bin/sh' at index 4
[25715181723] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU0] BootFs: EXACT match for 'bin/sh' at index 4
[25736379471] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'sh' (len=117640, base=0x200000)
[25739488896] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[25752470337] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x216000 filesz=0 memsz=0 align=1
[25758214119] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /bin/sh
[25760114325] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80050990 kstack_top=0xffffffffb009c500 arg=0xffffffffb006f660
[25765463559] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=6, applying inserts
[25767743727] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[25768824939] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=6
[25770369735] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[25772097549] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[25778861592] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SETUP_STDIO: stdin=Fd(3) stdout=Fd(4) stderr=Fd(5)
[25793414559] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 6
[25799196060] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Task 6 woken, restoring IRQs
[25801275390] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Done for /bin/sh
[25811069196] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[25813966662] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 250ms so the prompt can take the foreground
[25815346656] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[25816938345] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=6 entry_pc=200000 user_sp=800000
[25819085655] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=0
[25821232800] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=100
[25825975956] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=19899
[25832638194] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'etc/motd' at index 105
[25852716054] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[25854747699] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
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
[25875225882] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'etc/profile' at index 104
[25913034444] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[25915022859] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [25921952529] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[25923490989] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[?25h[26641565577] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[26645634642] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[26649725817] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=4096 port=0xffffffffb0071470
[26652300411] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(0)
[26658073761] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=1 port_id=PortId(0) mode=Write
[26660636013] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=2 port_id=PortId(0) mode=Read
[26665523412] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/cambium' (len=124728, base=0x200000)
[26667328908] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[26680134822] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x217000 filesz=0 memsz=0 align=1
[26686420101] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80050990 kstack_top=0xffffffffb0081820 arg=0xffffffffb0071600
[26689333605] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[26690512827] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=7
[26691733827] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[26699452395] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 7 → task '/bin/cambium' (pid=7 from boot module)
[26702352369] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned cambium (PID=7)
[26703969666] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[26705500998] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[26706515253] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=7 entry_pc=200000 user_sp=800000
[26708420343] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=0
[26709608376] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/iso9660d' (len=80648, base=0x200000)
[26711011602] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=100
[26712509406] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[26721420231] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20e000 filesz=0 memsz=0 align=1
[26727588756] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80050990 kstack_top=0xffffffffb00b5440 arg=0xffffffffb0071600
[26730288552] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[26731510674] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=8
[26732785596] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[26736682005] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 8 → task '/bin/iso9660d' (pid=8 from boot module)
[26739547758] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned iso9660d (PID=8)
[26741241483] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[26742951576] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=8 entry_pc=200000 user_sp=800000
[26744648238] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Deferring netd until /dev/net/virtio0/rx is ready...
[26746728228] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=0
[26748824685] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=100
[26756069967] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[26765443947] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Task 9 (user thread) assigned to CPU 3
[26770050813] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[26771341377] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=9
[26772564555] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[26774470767] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Sending post-unlock Resched IPI to CPU 3 for task 9
[26776814922] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Running registration + health supervision loop
[26781036843] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU0] BRIDGE: handle=2 -> fd=3 node=0xffffffffb00cdb10 port=0xffffffffb0071470
[26784605463] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Bridged resp_port 2 -> FD 3 for task 'cambium'
[26794152000] [[34mDEBUG[0m] [iso9660d] [CPU2] iso9660d: starting ISO9660 VFS provider
[26798225619] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ahci_disk' at index 57
[26800127277] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[27064736886] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[27070876998] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ata_disk' at index 58
[27240717801] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.ata' start='thingos_driver_start_safe'
[27245130066] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/chime' at index 63
[27404914515] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[27408735189] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/display_bootfb' at index 41
[27579475209] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[27583863549] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/display_virtio_gpu' at index 42
[27763831095] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[27768641538] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/hdaudio' at index 61
[27931704933] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[27936323547] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/pci_stubd' at index 62
[28097796408] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[28101575370] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ps2_kbd' at index 3
[28262092815] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[28266572730] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ps2_mouse' at index 40
[28458090606] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[28461234549] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/rtc_cmos' at index 2
[28626434991] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[28631625858] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/rtl8168d' at index 45
[28799369412] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[28803476097] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/virtio_netd' at index 44
[28974547272] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[28978992900] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/virtio_sound' at index 60
[29152892109] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[29157939657] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[29312634780] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=3 starting copyin
[29314395792] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=3 copyin ok
[29322740469] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/virtio_netd' at index 44
[29338172754] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'virtio_netd' (len=106272, base=0x200000)
[29340458631] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[29353342260] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x214000 filesz=0 memsz=0 align=1
[29365205496] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x20277c + bias 0x0 = PC 0x20277c
[29367834870] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/virtio_netd
[29369765469] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80050990 kstack_top=0xffffffffb00f7ae0 arg=0xffffffffb00cca60
[29373783549] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=10, applying inserts
[29375795394] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[29377464600] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=10
[29378959995] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[29380251318] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[29382755556] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[29388909924] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 10
[29395930905] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 10 woken, restoring IRQs
[29397631857] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/virtio_netd
[29403108603] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/virtio_netd for pci-0000:00:02.0 (entry='thingos_driver_start_safe', pid=10)
[29411061966] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=4 starting copyin
[29413056750] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=4 copyin ok
[29417021733] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ahci_disk' at index 57
[29429127024] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'ahci_disk' (len=74072, base=0x200000)
[29431232226] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[29442423450] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x20e000 filesz=0 memsz=0 align=1
[29449630056] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[29452479573] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/ahci_disk
[29455176696] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80050990 kstack_top=0xffffffffb01102c0 arg=0xffffffffb00cd4a0
[29474038407] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=11, applying inserts
[29476300425] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[29478247689] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=11
[29480583132] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[29482780668] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[29487292263] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[29493814944] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 11
[29496154611] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 11 woken, restoring IRQs
[29497923411] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/ahci_disk
[29499778044] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x4
[29502471207] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/ahci_disk for pci-0000:00:1f.2 (entry='thingos_driver_start_safe', pid=11)
[29504843577] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=11 entry_pc=201000 user_sp=800000
[29507444406] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=11 buf_len=0
[29509769025] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=11 buf_len=100
[29512107966] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=5 starting copyin
[29514215445] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=5 copyin ok
[29516152743] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver (boot_fd=4)
[29520087102] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ata_disk' at index 58
[29530903347] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'ata_disk' (len=61712, base=0x200000)
[29533444479] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[29542793907] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x20b000 filesz=0 memsz=0 align=1
[29550491718] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[29553823233] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/ata_disk
[29556374001] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80050990 kstack_top=0xffffffffb01242c0 arg=0xffffffffb00dcf60
[29560373040] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=12, applying inserts
[29562684756] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[29564444679] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=12
[29566161735] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[29567867967] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[29570825988] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[29576556405] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 12
[29578395363] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 12 woken, restoring IRQs
[29580176472] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/ata_disk
[29581680744] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x5
[29584593753] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=12 entry_pc=201000 user_sp=800000
[29588459835] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=12 buf_len=0
[29591138709] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 'pci-0000:00:1f.2' (handle 0)
[29593184808] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=12 buf_len=100
[29595564108] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Claimed PCI device '/sys/devices/pci-0000:00:1f.2' handle=0
[29600821503] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: mapping BAR5 (phys=0x80880000, size=0x1000) for task 11
[29602958121] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[29605775166] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x20277c SP=0x800000 ARG0=0x3
[29612419188] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[29614428459] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[29616965598] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=10 entry_pc=20277c user_sp=800000
[29619353346] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[29622787326] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[29624363241] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=0
[29625929883] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[29626955325] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=100
[29629039209] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: DMA virt=0x20c000
[29632693398] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: DMA phys=0x22f9000
[29634898722] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 0...
[29637134472] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[29639416389] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 1...
[29641695501] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[29643753876] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 2...
[29646204852] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[29651586921] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=4096 port=0xffffffffb00cde10
[29654429706] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(1)
[29656059741] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(1) mode=Write
[29657660373] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
[29659535136] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=4 port_id=PortId(1) mode=Read
[29671611420] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Starting VirtIO-NET driver service...
[29674760148] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Initializing hardware driver...
[29678583957] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Claiming NIC device at /sys/devices/pci-0000:00:02.0
[29682165711] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: device::new(/sys/devices/pci-0000:00:02.0)
[29685093801] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: claiming '/sys/devices/pci-0000:00:02.0'...
[29688379677] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 10 claimed device 'pci-0000:00:02.0' (handle 1)
[29691971760] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=11 fd=5 starting copyin
[29693848173] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[29695187511] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=11 fd=5 copyin ok
[29696698251] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: claimed, handle=1
[29698072668] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[29699743953] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
[29701009965] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device port=2 model='                                        ' rpc_port=3
[29704531131] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 3...
[29706662106] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[29708835585] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 4...
[29710575114] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/common_bar -> '4' (n=2)
[29712518715] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[29715092814] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 5...
[29716872966] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[29718936753] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/common_offset -> '0x0' (n=4)
[29720851875] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[29723119437] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[29725842993] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=4 -> fd=5 node=0xffffffffb00cca70 port=0xffffffffb00cde10
[29728751118] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/notify_bar -> '4' (n=2)
[29735305875] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/notify_offset -> '0x3000' (n=7)
[29740928283] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/notify_multiplier -> '4' (n=2)
[29743742952] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[29749870689] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/device_bar -> '4' (n=2)
[29756094885] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/device_offset -> '0x2000' (n=7)
[29758523718] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: mapping common BAR4...
[29760264765] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: mapping BAR4 (phys=0xc000004000, size=0x4000) for task 10
[29763594498] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x10001000
[29766851796] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: common_cfg at 0x10001000
[29768835393] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: notify_cfg at 0x10004000
[29771510802] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: device_cfg = Some(268447744)
[29773227363] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: allocating DMA command buffer...
[29777257818] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 1 pages phys=0x23b9000 -> user_va=0x10005000
[29781592038] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: cmd_buf virt=0x10005000 phys=0x23b9000
[29783687505] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: device::new complete
[29788459041] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Device features 0x30bf8024
[29792750724] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: MAC 52:54:00:12:34:56
[29795910408] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Link UP
[29802312474] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 4 pages phys=0x23ba000 -> user_va=0x10006000
[29806748037] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: RX queue 0 setup (size=32)
[29813794428] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 4 pages phys=0x23be000 -> user_va=0x1000a000
[29817397632] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: TX queue 1 setup (size=32)
[29827326276] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 16 pages phys=0x23c2000 -> user_va=0x1000e000
[29830580439] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[29832918423] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 1 pages phys=0x23d2000 -> user_va=0x1001e000
[29836249212] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Allocated TX buffer (1 page)
[29839188291] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: RX queue filled with 32 buffers
[29841034212] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: DRIVER_OK set, device is live
[29844289530] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Driver initialized successfully
[29847448257] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Driver initialized successfully
[29850263520] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: MAC 52:54:00:12:34:56
[29853267642] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Initial link state is UP
[29859427719] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=65536 port=0xffffffffb00d0e30
[29862204306] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(2)
[29864900967] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=3 port_id=PortId(2) mode=Write
[29868016893] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=4 port_id=PortId(2) mode=Read
[29870928978] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Created VFS provider port
[29881562106] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Task 13 (user thread) assigned to CPU 1
[29886273351] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[29888629584] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=13
[29890876983] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[29897177409] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=13 buf_len=0
[29900358906] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=13 buf_len=100
[29903673492] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Entering VFS provider service loop at /dev/net/virtio0
[29926697823] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=262400 port=0xffffffffb00d1f10
[29929295814] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(3)
[29934065106] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=5 port_id=PortId(3) mode=Write
[29940042561] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] vfs: mounted userland provider at /dev/net/virtio0 (flags: 0x0)
[29942849409] [[34mDEBUG[0m] [virtio_netd] [CPU2] VIRTIO_NETD: Mounted at /dev/net/virtio0
[29945253063] [[34mDEBUG[0m] [virtio_netd] [CPU2] VIRTIO_NETD: Provider thread live at /dev/net/virtio0
[3046129686
```
</details>
