# ❌ Scenario: sprout launches the full graphics pipeline

> Last run: 2026-04-27 13:59:43

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8925ms | - [📜](./01/serial.log) - |
| 2 | Then the serial output should contain "SPROUT: Starting full pipeline (graphics + input)" within 60s | ✅ | 210ms | - [📜](./02/serial.log) - |
| 3 | And the serial output should contain "SPROUT: Spawned bristle" within 60s | ✅ | 0ms | - - - |
| 4 | And the serial output should contain "display_virtio_gpu: frame pool ready" within 120s | ❌ | 121601ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[27435254022] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[27792758070] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[27813252093] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[27814112040] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] enter
[27814580409] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] set expand hook
[27815003304] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] expand hook ok
[27815419962] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock begin
[27816288555] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock ok
[27847914270] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[27871069248] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] devfs: set_boot_fb width=1920 height=1080 pitch=7680 resource_id=0xfb000000
[27872997339] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs set_boot_fb elapsed_ticks=1757580 elapsed_us=878
[27878717097] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs FbNode::new elapsed_ticks=5020719 elapsed_us=2510
[27884374683] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs register fb0 elapsed_ticks=4900797 elapsed_us=2450
[27884970828] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[27905097924] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[27906831645] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[27941492205] [[34mDEBUG[0m] [kernel] [CPU0] Seeding entropy pool...
[27945104517] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[27945931596] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[27947072010] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] entropy::seed_from_hardware elapsed_ticks=4222647 elapsed_us=2111
[27947580012] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[27967575702] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[27994175319] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80078080 kstack_top=0xffffffffb0020660 arg=0x0
[27996497166] [[34mDEBUG[0m] [kernel::sched::spawn::task] [CPU0] SCHED: Task 1 assigned to CPU 0
[28014188532] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[28034802180] [[34mDEBUG[0m] [kernel] [CPU0] Initializing VFS...
[28038138414] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='set_cmdline' elapsed_ticks=2705571 elapsed_us=1352 total_ticks=3158364 total_us=1579
[28056984978] [[34mDEBUG[0m] [kernel::vfs] [CPU0] VFS: Created /dev/display directory
[28057668804] [[34mDEBUG[0m] [kernel::vfs] [CPU0] VFS: Created /dev/storage directory
[28093298904] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted union filesystem at / (root)
[28095389487] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted devfs at /dev
[28096322892] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted procfs at /proc
[28097119380] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted sysfs at /sys
[28101400767] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /tmp
[28102184352] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /run
[28102885569] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /services
[28103494188] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /session
[28104165738] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /data
[28104576951] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='vfs_init' elapsed_ticks=65858166 elapsed_us=32929 total_ticks=69854070 total_us=34927
[28105128744] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[28125903828] [[34mDEBUG[0m] [kernel] [CPU0] Scanning PCI bus...
[28129293093] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x29c0 at 00:00.0 class=060000 id=1
[28149378642] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1050 at 00:01.0 class=030000 id=2
[28169489667] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1000 at 00:02.0 class=020000 id=3
[28182257631] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1059 at 00:03.0 class=040100 id=4
[28183040754] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2918 at 00:1f.0 class=060100 id=5
[28187070351] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2922 at 00:1f.2 class=010601 id=6
[28189487238] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2930 at 00:1f.3 class=0c0500 id=7
[28191797700] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='scan_pci' elapsed_ticks=65501304 elapsed_us=32750 total_ticks=157063203 total_us=78531
[28192362858] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[28214798436] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='register_legacy_devices' elapsed_ticks=846549 elapsed_us=423 total_ticks=180075192 total_us=90037
[28215385176] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[28235985063] [[34mDEBUG[0m] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[28243056897] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[28243588923] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[28244462862] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[28245266874] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[28248808368] [[34mDEBUG[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[28251284424] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[28252069626] [[34mDEBUG[0m] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[28252610166] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[28253909343] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Registers initialized
[28254759456] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[28255643988] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[28256702859] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: All pins masked
[28258658835] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Init complete
[28298191053] [[34mDEBUG[0m] [bran::arch::x86_64::ioapic] [CPU0] LAPIC: calibrated 62457800 ticks/sec (delta=624578, ok=true) -> init_cnt=624578
[28299549597] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62457800 ticks/sec), init_cnt=624578 for 100Hz
[28300078290] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='setup_preemption_timer' elapsed_ticks=63656571 elapsed_us=31828 total_ticks=265355178 total_us=132677
[28300675260] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[28321940691] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[28322628378] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[28323751434] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[28325752950] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[28354512582] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[28355333457] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[28358365992] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] SMP: CPU 1 initializing preemption timer (vec=32 init_cnt=624578)
[28359854688] [[34mDEBUG[0m] [kernel] [CPU1] SMP: kernel_secondary_entry arg_cpu=1 runtime_cpu=1
[28360390278] [[34mDEBUG[0m] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[28369883289] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80078080 kstack_top=0xffffffffb0035a40 arg=0x1
[28370692614] [[34mDEBUG[0m] [kernel::sched::spawn::task] [CPU1] SCHED: Task 2 assigned to CPU 1
[28374316311] [[34mDEBUG[0m] [kernel::sched::lifecycle] [CPU1] SMP: Secondary CPU 1 online!
[28374891930] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: run_scheduler entry on CPU 1
[28375410162] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: bootstrap_cpu start on CPU 1
[28376354490] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle thread 2 and is now schedulable
[28379179554] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[28379064516] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] SMP: CPU 2 initializing preemption timer (vec=32 init_cnt=624578)
[28379657328] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[28380325347] [[34mDEBUG[0m] [kernel] [CPU2] SMP: kernel_secondary_entry arg_cpu=2 runtime_cpu=2
[28381109592] [[34mDEBUG[0m] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[28388508984] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU2] INIT KERNEL CTX: entry=0xffffffff80078080 kstack_top=0xffffffffb00471c0 arg=0x2
[28389098694] [[34mDEBUG[0m] [kernel::sched::spawn::task] [CPU2] SCHED: Task 3 assigned to CPU 2
[28390659693] [[34mDEBUG[0m] [kernel::sched::lifecycle] [CPU2] SMP: Secondary CPU 2 online!
[28391676225] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: run_scheduler entry on CPU 2
[28392042096] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: bootstrap_cpu start on CPU 2
[28392419484] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle thread 3 and is now schedulable
[28404859659] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] SMP: CPU 3 initializing preemption timer (vec=32 init_cnt=624578)
[28406275458] [[34mDEBUG[0m] [kernel] [CPU3] SMP: kernel_secondary_entry arg_cpu=3 runtime_cpu=3
[28406426301] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[28406769171] [[34mDEBUG[0m] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[28407332052] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[28407853023] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[28407967731] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80078080 kstack_top=0xffffffffb0058940 arg=0x3
[28408275753] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='start_secondary_cpus' elapsed_ticks=86553786 elapsed_us=43276 total_ticks=373542576 total_us=186771
[28408640304] [[34mDEBUG[0m] [kernel::sched::spawn::task] [CPU3] SCHED: Task 4 assigned to CPU 3
[28408865166] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[28410253839] [[34mDEBUG[0m] [kernel::sched::lifecycle] [CPU3] SMP: Secondary CPU 3 online!
[28410720360] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: run_scheduler entry on CPU 3
[28411132662] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: bootstrap_cpu start on CPU 3
[28411540410] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle thread 4 and is now schedulable
[28498545108] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[28537821576] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Enumerating 112 boot modules...
[28538880579] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[28658513862] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[28659688365] [[34mDEBUG[0m] [kernel] [CPU0] Found init module: /bin/sprout (cmdline: 'init'), loading...
[28661236692] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/sprout' (len=110200, base=0x200000)
[28669545036] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[28697118912] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x215000 filesz=0 memsz=0 align=1
[28706372673] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='load_init_module' elapsed_ticks=45358566 elapsed_us=22679 total_ticks=671616330 total_us=335808
[28708151109] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='prepare_boot_registry' elapsed_ticks=1031349 elapsed_us=515 total_ticks=673429614 total_us=336714
[28709086758] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='map_boot_registry' elapsed_ticks=199155 elapsed_us=99 total_ticks=674365824 total_us=337182
[28710308022] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='activate_init_aspace' elapsed_ticks=403821 elapsed_us=201 total_ticks=675525543 total_us=337762
[28711424313] [[34mDEBUG[0m] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[28712106522] [[34mDEBUG[0m] [kernel] [CPU0] Spawning init process...
[28716005439] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff8004aa00 kstack_top=0xffffffffb0068e40 arg=0xffffffffb0020a20
[28731830457] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='spawn_init_task' elapsed_ticks=19753041 elapsed_us=9876 total_ticks=697089393 total_us=348544
[28732691889] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[28895228637] [[34mDEBUG[0m] [kernel::sched::lifecycle] [CPU0] SCHED: early-boot bringup complete; resuming steady-state SMP scheduling
[28896041460] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='end_bringup' elapsed_ticks=1132131 elapsed_us=566 total_ticks=861312309 total_us=430656
[28896943449] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[29183943855] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1148995980 elapsed_us=574497
[29184591315] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[29201159097] [[34mDEBUG[0m] [kernel::sched::spawn::task] [CPU0] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[29202098673] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] ENTER_USER: TID=5 entry_pc=200000 user_sp=800000
[29238276276] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=0
[29244904062] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=20
[29256110103] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[29260144320] [[34mDEBUG[0m] [sprout] [CPU0] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7fef70 rip=0x206395 rflags=0x202
[29261832798] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[29263427919] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: About to create Supervisor...
[29265614268] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Listing /bin directory...
[29305407780] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[29335738311] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Parsed command line: '    '
[29340435168] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=4096
[29342071176] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(0)
[29347265706] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=1 port_id=PortId(0) mode=Write
[29349090870] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=2 port_id=PortId(0) mode=Read
[29355541248] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Supervisor created, calling run_forever...
[29357602692] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[29367342444] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[29369116689] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[29409289338] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN: page cache miss for '/bin/sh', caching 127952 bytes
[29416471524] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'sh' (len=127952, base=0x200000)
[29417326983] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[29432719206] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x218000 filesz=0 memsz=0 align=1
[29439276306] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /bin/sh
[29440492818] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff8004aa00 kstack_top=0xffffffffb00a34c0 arg=0xffffffffb0073c80
[29443544295] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=6, applying inserts
[29444366325] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[29456635593] [[34mDEBUG[0m] [kernel::sched::spawn::stdio] [CPU0] SETUP_STDIO: stdin=Fd(3) stdout=Fd(4) stderr=Fd(5)
[29471184633] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 6
[29481210330] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Task 6 woken, restoring IRQs
[29482624941] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Done for /bin/sh
[29484423012] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[29486683776] [[34mDEBUG[0m] [kernel::sched::spawn::task] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[29487245040] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=6 entry_pc=200000 user_sp=800000
[29488013280] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=0
[29489228571] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=100
[29491953381] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[29492166198] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[29495628690] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[29500793685] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::io] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[29501575092] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::io] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
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
[29567966670] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[29570572746] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[29577812187] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/cambium' (len=140064, base=0x200000)
[29579265309] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[29581288737] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::io] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[29581933656] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::io] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [29587712121] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::io] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[29588240055] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::io] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[?25h[29602471404] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x21a000 filesz=0 memsz=0 align=1
[29608836972] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff8004aa00 kstack_top=0xffffffffb00bcec0 arg=0xffffffffb00ac440
[29620348461] [[34mDEBUG[0m] [kernel::sched::spawn::boot] [CPU0] SCHED: TID 7 → task '/bin/cambium' (pid=7 from boot module)
[29626853850] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned cambium (PID=7)
[29626803624] [[34mDEBUG[0m] [kernel::sched::spawn::task] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[29627597340] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=7 entry_pc=200000 user_sp=800000
[29628710958] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=0
[29629797846] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=100
[29632635879] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[29634393723] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[29648404005] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN: page cache miss for '/bin/iso9660d', caching 78576 bytes
[29651827986] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'iso9660d' (len=78576, base=0x200000)
[29652953154] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[29656345488] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[29665636704] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20e000 filesz=0 memsz=0 align=1
[29670010194] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /bin/iso9660d
[29670586704] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff8004aa00 kstack_top=0xffffffffb00ea6c0 arg=0xffffffffb00d9180
[29671859778] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=8, applying inserts
[29672403288] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[29674140012] [[34mDEBUG[0m] [kernel::sched::spawn::stdio] [CPU0] SETUP_STDIO: stdin=Null stdout=Inherit stderr=Inherit
[29678512809] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 8
[29679749979] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Task 8 woken, restoring IRQs
[29680200561] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Done for /bin/iso9660d
[29680737306] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/iso9660d' TID=8 PID=8
[29682349224] [[34mDEBUG[0m] [kernel::sched::spawn::task] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[29682987543] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=8 entry_pc=200000 user_sp=800000
[29683849107] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=0
[29684890323] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=100
[29687930382] [[32mINFO [0m] [iso9660d] [CPU2] ISO9660D: Starting VFS provider
[29698681221] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned iso9660d (PID=8)
[29701392534] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[29703533937] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Deferring netd until /dev/net/virtio0/rx is ready...
[29705093022] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[29706774933] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[29708619864] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[29723679480] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN: page cache miss for '/bin/bristle', caching 64016 bytes
[29724878304] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'bristle' (len=64016, base=0x200000)
[29726301825] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[29736507900] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20b000 filesz=0 memsz=0 align=1
[29741324646] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /bin/bristle
[29741919042] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff8004aa00 kstack_top=0xffffffffb0113240 arg=0xffffffffb00d9020
[29743351374] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=9, applying inserts
[29743925970] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[29745775785] [[34mDEBUG[0m] [kernel::sched::spawn::stdio] [CPU0] SETUP_STDIO: stdin=Null stdout=Inherit stderr=Inherit
[29749411659] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 9
[29750833035] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Task 9 woken, restoring IRQs
[29751378723] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Done for /bin/bristle
[29751902202] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/bristle' TID=9 PID=9
[29753023971] [[34mDEBUG[0m] [kernel::sched::spawn::task] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[29753756637] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=9 entry_pc=200000 user_sp=800000
[29754778317] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=0
[29757292323] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=100
[29758056405] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=9)
[29760777453] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[29761749963] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[29762592915] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::descriptors] [CPU0] sys_fs_open: path='/dev/fb0' len=8 flags=0x0
[29765303865] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] devfs: dynamic registry hit path='fb0'
[29784283122] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::descriptors] [CPU0] sys_fs_open: resolved node for /dev/fb0 mode=0o20666 size=32 ino=4
[29785609359] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::descriptors] [CPU0] sys_fs_open: process info present for /dev/fb0
[29786179665] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::io] [CPU3] sys_fs_write: tid=9 fd=3 starting copyin
[29786364927] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::descriptors] [CPU0] sys_fs_open: handle_table.open('/dev/fb0') -> 3
[29786699448] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::io] [CPU3] sys_fs_write: tid=9 fd=3 copyin ok
[29797314228] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[29797926543] [[34mDEBUG[0m] [bristle] [CPU3] bristle: active_ui set to 'bloom'
[29802516975] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[29804104572] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[29805741570] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=4096
[29806474434] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(1)
[29807052264] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=3 port_id=PortId(1) mode=Write
[29807541720] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=4 port_id=PortId(1) mode=Read
[29810780538] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[29812542177] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=4096
[29812929300] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(2)
[29813380113] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=5 port_id=PortId(2) mode=Write
[29813877060] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=6 port_id=PortId(2) mode=Read
[29818306386] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[29823895365] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[29827242753] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::io] [CPU3] sys_fs_write: tid=9 fd=3 starting copyin
[29827870413] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::io] [CPU3] sys_fs_write: tid=9 fd=3 copyin ok
[29830069764] [[32mINFO [0m] [bristle] [CPU3] bristle: published pid 9 to /run/bristle/pid
[29831402733] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[29831473353] [[34mDEBUG[0m] [kernel::ipc] [CPU3] CREATE_PORT: capacity=4096
[29832220737] [[34mDEBUG[0m] [kernel::ipc] [CPU3] CREATE_PORT: appended id=PortId(3)
[29833066395] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU3] ALLOC_HANDLE: handle=3 port_id=PortId(3) mode=Write
[29833735701] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU3] ALLOC_HANDLE: handle=4 port_id=PortId(3) mode=Read
[29834927859] [[34mDEBUG[0m] [kernel::ipc] [CPU3] CREATE_PORT: capacity=4096
[29835415137] [[34mDEBUG[0m] [kernel::ipc] [CPU3] CREATE_PORT: appended id=PortId(4)
[29835873804] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU3] ALLOC_HANDLE: handle=5 port_id=PortId(4) mode=Write
[29836329336] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU3] ALLOC_HANDLE: handle=6 port_id=PortId(4) mode=Read
[29841587985] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::io] [CPU3] sys_fs_write: tid=9 fd=3 starting copyin
[29842116513] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::io] [CPU3] sys_fs_write: tid=9 fd=3 copyin ok
[29846831982] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::io] [CPU3] sys_fs_write: tid=9 fd=3 starting copyin
[29847533562] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::io] [CPU3] sys_fs_write: tid=9 fd=3 copyin ok
[29849940648] [[32mINFO [0m] [bristle] [CPU3] bristle: published device handles kbd_in=3 mouse_in=5
[29852648760] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN: page cache miss for '/drivers/display_virtio_gpu', caching 133344 bytes
[29853998856] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'display_virtio_gpu' (len=133344, base=0x200000)
[29854945758] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[29859775770] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU3] BRIDGE: handle=4 -> fd=4 node=0xffffffffb0020a30 port=0xffffffffb0221310
[29862038217] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU3] BRIDGE: handle=6 -> fd=5 node=0xffffffffb0243b70 port=0xffffffffb0223090
[29863794477] [[32mINFO [0m] [bristle] [CPU3] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[29887949850] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x219000 filesz=0 memsz=0 align=1
[29893872723] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/display_virtio_gpu
[29894493222] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff8004aa00 kstack_top=0xffffffffb012bb80 arg=0xffffffffb00f2d20
[29895826983] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=10, applying inserts
[29899690326] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[29901639438] [[34mDEBUG[0m] [kernel::sched::spawn::stdio] [CPU0] SETUP_STDIO: stdin=Null stdout=Inherit stderr=Inherit
[29905714641] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 10
[29906804862] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Task 10 woken, restoring IRQs
[29907234621] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Done for /drivers/display_virtio_gpu
[29907772059] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/drivers/display_virtio_gpu' TID=10 PID=10
[29916813696] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Display pipeline initialized (backend=virtio_gpu)
[29918878209] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[29920627440] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready yet
[29929305978] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU0] BRIDGE: handle=2 -> fd=5 node=0xffffffffb00d9030 port=0xffffffffb0073870
[29931452760] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[29992109202] [[34mDEBUG[0m] [kernel::sched::spawn::task] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x3
[30001593105] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=10 entry_pc=200000 user_sp=800000
[30008004180] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=0
[30009249930] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=100
[30013055259] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting v0.4.1 (boot_arg=3)
[30014083242] [[34mDEBUG[0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: Starting VFS-native VirtIO GPU driver...
[30014794029] [[34mDEBUG[0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: boot_arg=3
[30015701166] [[34mDEBUG[0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: Mapping bootstrap memfd 3 size=4096...
[30017860323] [[34mDEBUG[0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: vm_map success at 0x400000001000
[30018856791] [[34mDEBUG[0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: Recovered handles: req_read=4, resp_write=5, svc=1, id=322371585
[30019780164] [[34mDEBUG[0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=4, drv_resp_w=5, svc=1, id=322371585)
[30021814812] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices'
[30069205881] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/platform-fb000000/vendor'
[30072102588] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'platform-fb000000/vendor'
[30079655232] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/platform-fb000000/device'
[30080733243] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'platform-fb000000/device'
[30083600778] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/platform-fb000000/class'
[30084925431] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'platform-fb000000/class'
[30089497482] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:00.0/vendor'
[30090638094] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:00.0/vendor'
[30093687789] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:00.0/device'
[30094622349] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:00.0/device'
[30097120614] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:00.0/class'
[30098328216] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:00.0/class'
[30101363886] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/vendor'
[30102355668] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:01.0/vendor'
[30107193897] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/device'
[30108291642] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:01.0/device'
[30111936987] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/class'
[30113090799] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:01.0/class'
[30120457521] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 10 claimed device 'pci-0000:00:01.0' (handle 0)
[30122933082] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/common_bar'
[30131114904] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/common_offset'
[30136725201] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/notify_bar'
[30144530427] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/notify_offset'
[30148547979] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/virtio/notify_multiplier'
[30152459997] [[32mINFO [0m] [virtio_gpu] [CPU1] virtio_gpu: caps from sysfs - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[30155258364] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Checking if bloom is ready to spawn (/dev/display/card0)...
[30157049010] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: display card not ready yet
[30157639974] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: mapping BAR2 (phys=0xc000000000, size=0x4000) for task 10
[30160309674] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10000000
[30162357555] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: ServiceLoop waiting for next event...
[30163727385] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 1 pages phys=0x234d000 -> user_va=0x10004000
[30168505587] [[32mINFO [0m] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[30173344740] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 4 pages phys=0x234e000 -> user_va=0x10005000
[30178523265] [[32mINFO [0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[30179478681] [[34mDEBUG[0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[30181172571] [[34mDEBUG[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='firmware/framebuffer'
[30187340073] [[34mDEBUG[0m] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=1
[30625819422] [[34mDEBUG[0m] [
```
</details>
