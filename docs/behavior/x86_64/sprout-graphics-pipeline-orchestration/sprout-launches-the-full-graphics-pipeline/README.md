# ❌ Scenario: sprout launches the full graphics pipeline

> Last run: 2026-04-27 14:11:39

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 9335ms | - [📜](./01/serial.log) - |
| 2 | Then the serial output should contain "SPROUT: Starting full pipeline (graphics + input)" within 60s | ✅ | 104ms | - [📜](./02/serial.log) - |
| 3 | And the serial output should contain "SPROUT: Spawned bristle" within 60s | ✅ | 105ms | - [📜](./03/serial.log) - |
| 4 | And the serial output should contain "display_virtio_gpu: frame pool ready" within 120s | ❌ | 121515ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[28470882396] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[28737417951] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[28758254085] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[28759097037] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] enter
[28759554747] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] set expand hook
[28759973286] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] expand hook ok
[28760443866] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock begin
[28760992986] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock ok
[28792356021] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[28815272574] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] devfs: set_boot_fb width=1920 height=1080 pitch=7680 resource_id=0xfb000000
[28818644448] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs set_boot_fb elapsed_ticks=2502720 elapsed_us=1251
[28825951143] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs FbNode::new elapsed_ticks=6103812 elapsed_us=3051
[28830940116] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs register fb0 elapsed_ticks=4242018 elapsed_us=2121
[28831456005] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[28852659429] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[28854418791] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[28896209067] [[34mDEBUG[0m] [kernel] [CPU0] Seeding entropy pool...
[28900556256] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[28901469333] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[28903282815] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] entropy::seed_from_hardware elapsed_ticks=5117772 elapsed_us=2558
[28903907175] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[28928343147] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[28966368123] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80078080 kstack_top=0xffffffffb0020660 arg=0x0
[28969024359] [[34mDEBUG[0m] [kernel::sched::spawn::task] [CPU0] SCHED: Task 1 assigned to CPU 0
[28993008627] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[29024250156] [[34mDEBUG[0m] [kernel] [CPU0] Initializing VFS...
[29030309088] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='set_cmdline' elapsed_ticks=4730550 elapsed_us=2365 total_ticks=5778432 total_us=2889
[29062805508] [[34mDEBUG[0m] [kernel::vfs] [CPU0] VFS: Created /dev/display directory
[29064205797] [[34mDEBUG[0m] [kernel::vfs] [CPU0] VFS: Created /dev/storage directory
[29127604572] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted union filesystem at / (root)
[29131827318] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted devfs at /dev
[29133626313] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted procfs at /proc
[29135036271] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted sysfs at /sys
[29142368673] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /tmp
[29143642605] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /run
[29144717910] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /services
[29145859677] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /session
[29147046192] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /data
[29147492781] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='vfs_init' elapsed_ticks=116223129 elapsed_us=58111 total_ticks=123427293 total_us=61713
[29148096846] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[29172505395] [[34mDEBUG[0m] [kernel] [CPU0] Scanning PCI bus...
[29177661282] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x29c0 at 00:00.0 class=060000 id=1
[29199584007] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1050 at 00:01.0 class=030000 id=2
[29225708061] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1000 at 00:02.0 class=020000 id=3
[29240288253] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1059 at 00:03.0 class=040100 id=4
[29241795033] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2918 at 00:1f.0 class=060100 id=5
[29248254618] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2922 at 00:1f.2 class=010601 id=6
[29252928012] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2930 at 00:1f.3 class=0c0500 id=7
[29255688033] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='scan_pci' elapsed_ticks=82628799 elapsed_us=41314 total_ticks=231611754 total_us=115805
[29256323679] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[29291574246] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='register_legacy_devices' elapsed_ticks=1157376 elapsed_us=578 total_ticks=267462855 total_us=133731
[29292800922] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[29325100530] [[34mDEBUG[0m] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[29334190248] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[29334707754] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[29335699800] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[29336478138] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[29340952905] [[34mDEBUG[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[29345071437] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[29346009924] [[34mDEBUG[0m] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[29346764304] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[29355140133] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Registers initialized
[29356678857] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[29358181116] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[29359325919] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: All pins masked
[29361377463] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Init complete
[29396198568] [[34mDEBUG[0m] [bran::arch::x86_64::ioapic] [CPU0] LAPIC: calibrated 63306500 ticks/sec (delta=633065, ok=true) -> init_cnt=633065
[29398184970] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (63306500 ticks/sec), init_cnt=633065 for 100Hz
[29399242389] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='setup_preemption_timer' elapsed_ticks=73456086 elapsed_us=36728 total_ticks=375130470 total_us=187565
[29400220344] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[29425116831] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[29426339448] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[29427551406] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[29429868600] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[29461019445] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[29462432241] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[29463950175] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] SMP: CPU 1 initializing preemption timer (vec=32 init_cnt=633065)
[29466081117] [[34mDEBUG[0m] [kernel] [CPU1] SMP: kernel_secondary_entry arg_cpu=1 runtime_cpu=1
[29466951525] [[34mDEBUG[0m] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[29472847404] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80078080 kstack_top=0xffffffffb0035a40 arg=0x1
[29474105925] [[34mDEBUG[0m] [kernel::sched::spawn::task] [CPU1] SCHED: Task 2 assigned to CPU 1
[29481439911] [[34mDEBUG[0m] [kernel::sched::lifecycle] [CPU1] SMP: Secondary CPU 1 online!
[29482449975] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: run_scheduler entry on CPU 1
[29483344506] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: bootstrap_cpu start on CPU 1
[29484971340] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle thread 2 and is now schedulable
[29488518279] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[29488448979] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] SMP: CPU 2 initializing preemption timer (vec=32 init_cnt=633065)
[29489412315] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[29489966847] [[34mDEBUG[0m] [kernel] [CPU2] SMP: kernel_secondary_entry arg_cpu=2 runtime_cpu=2
[29490664698] [[34mDEBUG[0m] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[29510856474] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU2] INIT KERNEL CTX: entry=0xffffffff80078080 kstack_top=0xffffffffb00471c0 arg=0x2
[29512092621] [[34mDEBUG[0m] [kernel::sched::spawn::task] [CPU2] SCHED: Task 3 assigned to CPU 2
[29514719751] [[34mDEBUG[0m] [kernel::sched::lifecycle] [CPU2] SMP: Secondary CPU 2 online!
[29515351074] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: run_scheduler entry on CPU 2
[29515939728] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: bootstrap_cpu start on CPU 2
[29518618338] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[29519407698] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[29519273817] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] SMP: CPU 3 initializing preemption timer (vec=32 init_cnt=633065)
[29520132345] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[29520538971] [[34mDEBUG[0m] [kernel] [CPU3] SMP: kernel_secondary_entry arg_cpu=3 runtime_cpu=3
[29520841812] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='start_secondary_cpus' elapsed_ticks=96161439 elapsed_us=48080 total_ticks=496761375 total_us=248380
[29521218573] [[34mDEBUG[0m] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[29521815708] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[29523956715] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80078080 kstack_top=0xffffffffb0058940 arg=0x3
[29525809203] [[34mDEBUG[0m] [kernel::sched::spawn::task] [CPU3] SCHED: Task 4 assigned to CPU 3
[29530824906] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle thread 3 and is now schedulable
[29530915392] [[34mDEBUG[0m] [kernel::sched::lifecycle] [CPU3] SMP: Secondary CPU 3 online!
[29535974853] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: run_scheduler entry on CPU 3
[29536587465] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: bootstrap_cpu start on CPU 3
[29537231592] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle thread 4 and is now schedulable
[29583426147] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[29654905335] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Enumerating 112 boot modules...
[29656167156] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[29847990051] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[29849328201] [[34mDEBUG[0m] [kernel] [CPU0] Found init module: /bin/sprout (cmdline: 'init'), loading...
[29851074000] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/sprout' (len=110408, base=0x200000)
[29866051710] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[29892374160] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x215000 filesz=0 memsz=0 align=1
[29904970491] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='load_init_module' elapsed_ticks=54214149 elapsed_us=27107 total_ticks=880871706 total_us=440435
[29907773214] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='prepare_boot_registry' elapsed_ticks=1708410 elapsed_us=854 total_ticks=883703040 total_us=441851
[29908999989] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='map_boot_registry' elapsed_ticks=273801 elapsed_us=136 total_ticks=884937141 total_us=442468
[29910163437] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='activate_init_aspace' elapsed_ticks=175593 elapsed_us=87 total_ticks=886061715 total_us=443030
[29910945108] [[34mDEBUG[0m] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[29911415853] [[34mDEBUG[0m] [kernel] [CPU0] Spawning init process...
[29913286161] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff8004aa00 kstack_top=0xffffffffb0068e40 arg=0xffffffffb0020a20
[29935676199] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='spawn_init_task' elapsed_ticks=24282225 elapsed_us=12141 total_ticks=911578866 total_us=455789
[29947043808] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[30110019423] [[34mDEBUG[0m] [kernel::sched::lifecycle] [CPU0] SCHED: early-boot bringup complete; resuming steady-state SMP scheduling
[30110946525] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='end_bringup' elapsed_ticks=1287594 elapsed_us=643 total_ticks=1086869487 total_us=543434
[30111613620] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[30417819960] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1393528323 elapsed_us=696764
[30418470159] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[30436276497] [[34mDEBUG[0m] [kernel::sched::spawn::task] [CPU0] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[30437197494] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] ENTER_USER: TID=5 entry_pc=200000 user_sp=800000
[30482412609] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=0
[30487867311] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=20
[30500237823] [[32mINFO [0m] [sprout] [CPU0] SPROUT: ENTERING MAIN (arg0=6291456)
[30503615604] [[34mDEBUG[0m] [sprout] [CPU0] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7fef70 rip=0x206395 rflags=0x202
[30505408032] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[30506834952] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: About to create Supervisor...
[30508013448] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Listing /bin directory...
[30548829597] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Initializing Supervisor...
[30579834318] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Parsed command line: '    '
[30584642154] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=4096
[30586113426] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(0)
[30593972937] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=1 port_id=PortId(0) mode=Write
[30595307391] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=2 port_id=PortId(0) mode=Read
[30598151892] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Supervisor created, calling run_forever...
[30601742226] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (FULL PIPELINE MODE)
[30611538771] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[30613041294] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[30658181169] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN: page cache miss for '/bin/sh', caching 127976 bytes
[30664677021] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'sh' (len=127976, base=0x200000)
[30665841855] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[30682217049] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x218000 filesz=0 memsz=0 align=1
[30687405507] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /bin/sh
[30689038215] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff8004aa00 kstack_top=0xffffffffb00a34e0 arg=0xffffffffb0073c80
[30692246244] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=6, applying inserts
[30697102656] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[30707266755] [[34mDEBUG[0m] [kernel::sched::spawn::stdio] [CPU0] SETUP_STDIO: stdin=Fd(3) stdout=Fd(4) stderr=Fd(5)
[30719989278] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 6
[30730466250] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Task 6 woken, restoring IRQs
[30731053518] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Done for /bin/sh
[30732612537] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[30733365300] [[34mDEBUG[0m] [kernel::sched::spawn::task] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[30734707245] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=6 entry_pc=200000 user_sp=800000
[30741279756] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[30744804288] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[30806576064] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[30808319157] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[30813136926] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/cambium' (len=140272, base=0x200000)
[30813942192] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[30832253067] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x21a000 filesz=0 memsz=0 align=1
[30840225867] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff8004aa00 kstack_top=0xffffffffb00bc7a0 arg=0xffffffffb00739e0
[30851019738] [[34mDEBUG[0m] [kernel::sched::spawn::boot] [CPU0] SCHED: TID 7 → task '/bin/cambium' (pid=7 from boot module)
[30855317922] [[34mDEBUG[0m] [kernel::sched::spawn::task] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[30856402137] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=7 entry_pc=200000 user_sp=800000
[30857572317] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=0
[30858501168] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned cambium (PID=7)
[30859026396] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=100
[30862889739] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: main started
[30862827501] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[30879806892] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN: page cache miss for '/bin/iso9660d', caching 78584 bytes
[30884069964] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'iso9660d' (len=78584, base=0x200000)
[30884878728] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[30893937558] [[32mINFO [0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[30900189012] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20e000 filesz=0 memsz=0 align=1
[30904842276] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /bin/iso9660d
[30905679024] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff8004aa00 kstack_top=0xffffffffb00e9d40 arg=0xffffffffb00c5560
[30907950183] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=8, applying inserts
[30908916423] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[30911747922] [[34mDEBUG[0m] [kernel::sched::spawn::stdio] [CPU0] SETUP_STDIO: stdin=Null stdout=Inherit stderr=Inherit
[30916808571] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 8
[30917840547] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Task 8 woken, restoring IRQs
[30918316638] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Done for /bin/iso9660d
[30918792102] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/iso9660d' TID=8 PID=8
[30920207769] [[34mDEBUG[0m] [kernel::sched::spawn::task] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[30921036300] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=8 entry_pc=200000 user_sp=800000
[30922334454] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=0
[30923771307] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=100
[30929685303] [[32mINFO [0m] [iso9660d] [CPU2] ISO9660D: Starting VFS provider
[30939924675] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned iso9660d (PID=8)
[30942624240] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[30944674662] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Deferring netd until /dev/net/virtio0/rx is ready...
[30946429800] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[30948191439] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting full pipeline (graphics + input)
[30950064585] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Entering supervisor service loop (tick=100ms)
[30968763903] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN: page cache miss for '/bin/bristle', caching 68312 bytes
[30970784691] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'bristle' (len=68312, base=0x200000)
[30971869467] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[30982736631] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20c000 filesz=0 memsz=0 align=1
[30988052502] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /bin/bristle
[30988928223] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff8004aa00 kstack_top=0xffffffffb0113b60 arg=0xffffffffb00d8940
[30991325409] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=9, applying inserts
[30992255349] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[31010852532] [[34mDEBUG[0m] [kernel::sched::spawn::stdio] [CPU0] SETUP_STDIO: stdin=Null stdout=Inherit stderr=Inherit
[31017566877] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 9
[31019857440] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Task 9 woken, restoring IRQs
[31021014156] [[34mDEBUG[0m] [kernel::sched::spawn::path] [CPU0] SPAWN_FROM_PATH: Done for /bin/bristle
[31021764840] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/bristle' TID=9 PID=9
[31030486839] [[34mDEBUG[0m] [kernel::sched::spawn::task] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[31032299232] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=9 entry_pc=200000 user_sp=800000
[31035295698] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=0
[31036640514] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=100
[31038783270] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=9)
[31044127983] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Starting display pipeline setup
[31046525664] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Probing /dev/fb0 for boot framebuffer metadata
[31048764021] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::descriptors] [CPU0] sys_fs_open: path='/dev/fb0' len=8 flags=0x0
[31052083656] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] devfs: dynamic registry hit path='fb0'
[31054740717] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::descriptors] [CPU0] sys_fs_open: resolved node for /dev/fb0 mode=0o20666 size=32 ino=4
[31055729661] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::descriptors] [CPU0] sys_fs_open: process info present for /dev/fb0
[31056799917] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::descriptors] [CPU0] sys_fs_open: handle_table.open('/dev/fb0') -> 3
[31071006978] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::io] [CPU3] sys_fs_write: tid=9 fd=3 starting copyin
[31072817952] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::io] [CPU3] sys_fs_write: tid=9 fd=3 copyin ok
[31084639377] [[34mDEBUG[0m] [bristle] [CPU3] bristle: active_ui set to 'bloom'
[31094790540] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: /dev/fb0 ready width=1920 height=1080 stride=7680 format=2
[31110139599] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Selected display driver '/drivers/display_virtio_gpu'
[31112755872] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display request port
[31115392044] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=4096
[31116499590] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(1)
[31117501140] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=3 port_id=PortId(1) mode=Write
[31118241066] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=4 port_id=PortId(1) mode=Read
[31124630427] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display response port
[31128480735] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=4096
[31129107636] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(2)
[31129874391] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=5 port_id=PortId(2) mode=Write
[31130572737] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=6 port_id=PortId(2) mode=Read
[31141584408] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Creating display bootstrap memfd
[31149514308] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Mapping display bootstrap memfd
[31159152552] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::io] [CPU3] sys_fs_write: tid=9 fd=3 starting copyin
[31160060877] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::io] [CPU3] sys_fs_write: tid=9 fd=3 copyin ok
[31158029001] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Launching display driver '/drivers/display_virtio_gpu' (boot_fd=3, bind_id=322371585)
[31163369787] [[32mINFO [0m] [bristle] [CPU3] bristle: published pid 9 to /run/bristle/pid
[31169229993] [[34mDEBUG[0m] [kernel::ipc] [CPU3] CREATE_PORT: capacity=4096
[31169926920] [[34mDEBUG[0m] [kernel::ipc] [CPU3] CREATE_PORT: appended id=PortId(3)
[31170835872] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU3] ALLOC_HANDLE: handle=3 port_id=PortId(3) mode=Write
[31171624968] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU3] ALLOC_HANDLE: handle=4 port_id=PortId(3) mode=Read
[31173335556] [[34mDEBUG[0m] [kernel::ipc] [CPU3] CREATE_PORT: capacity=4096
[31174121319] [[34mDEBUG[0m] [kernel::ipc] [CPU3] CREATE_PORT: appended id=PortId(4)
[31174893123] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU3] ALLOC_HANDLE: handle=5 port_id=PortId(4) mode=Write
[31175621994] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU3] ALLOC_HANDLE: handle=6 port_id=PortId(4) mode=Read
[31184771046] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::io] [CPU3] sys_fs_write: tid=9 fd=3 starting copyin
[31185393063] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::io] [CPU3] sys_fs_write: tid=9 fd=3 copyin ok
[31191893997] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::io] [CPU3] sys_fs_write: tid=9 fd=3 starting copyin
[31192473279] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs::io] [CPU3] sys_fs_write: tid=9 fd=3 copyin ok
[31195011738] [[32mINFO [0m] [bristle] [CPU3] bristle: published device handles kbd_in=3 mouse_in=5
[31205157027] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU3] BRIDGE: handle=4 -> fd=4 node=0xffffffffb0020a30 port=0xffffffffb0221c30
[31209393534] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU3] BRIDGE: handle=6 -> fd=5 node=0xffffffffb00f1650 port=0xffffffffb02239b0
[31212013536] [[32mINFO [0m] [bristle] [CPU3] bristle: online (kbd_tok=Some(WaitToken(2)), mouse_tok=Some(WaitToken(3)))
[31330511619] [[32mINFO [0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[31540054623] [[32mINFO [0m] [
```
</details>
