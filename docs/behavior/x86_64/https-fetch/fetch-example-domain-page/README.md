# ❌ Scenario: Fetch Example Domain page

> Last run: 2026-04-22 10:07:03

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 7713ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 2107ms | - [📜](./02/serial.log) - |
| 3 | And I wait for the serial output to contain "NETD: Network ready" | ❌ | 301024ms | - [📜](./03/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[24265543170] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] enter
[24275700570] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] phys_memory_map ok
[24279612819] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] modules ok
[24280773528] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] phys_to_virt_offset ok
[24281752011] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] memory map logging done
[24305191680] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] boot_frame_alloc init ok
[24502478715] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] frame allocator build ok
[24503709384] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] frame allocator log ok
[24505105482] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] FRAME_ALLOCATOR init ok
[24506334171] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] tasking init ok
[24510847185] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[24512104947] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] enter
[24513096894] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] set expand hook
[24514337067] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] expand hook ok
[24515374092] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock begin
[24516861699] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock ok
[24518011716] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] reserve_region begin
[24547916580] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] reserve_region ok
[24549165663] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] inner allocator init begin
[24552558591] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] inner allocator init ok
[24553873971] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] heap top store ok
[24554930301] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] init done
[24561561585] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] set_boot_fb begin
[24562790571] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] devfs: set_boot_fb width=1920 height=1080 pitch=7680 resource_id=0xfb000000
[24564927552] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] set_boot_fb ok
[24566806737] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs set_boot_fb elapsed_ticks=4593270 elapsed_us=2296
[24574485276] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs FbNode::new elapsed_ticks=5435925 elapsed_us=2717
[24575921403] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] register begin
[24580848501] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] register ok
[24581924730] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs register fb0 elapsed_ticks=5938581 elapsed_us=2969
[24586822590] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[24596495319] [[34mDEBUG[0m] [kernel] [CPU0] Seeding entropy pool...
[24605213655] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed begin
[24606525141] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] fill_entropy done
[24609111285] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] add_sample(timer) ok
[24610181706] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] mark_seeded(timer) ok
[24611152764] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[24612345219] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[24613510779] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed done
[24614924433] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] entropy::seed_from_hardware elapsed_ticks=9503934 elapsed_us=4751
[24619841301] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[24646338255] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80016ce0 kstack_top=0xffffffffb001f980 arg=0x0
[24649927335] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[24670672488] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[24675765873] [[34mDEBUG[0m] [kernel] [CPU0] Initializing VFS...
[24679623144] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='set_cmdline' elapsed_ticks=2655180 elapsed_us=1327 total_ticks=3672042 total_us=1836
[24698492115] [[34mDEBUG[0m] [kernel::vfs] [CPU0] VFS: Created /dev/display directory
[24713881566] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted union filesystem at / (root)
[24716765601] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted devfs at /dev
[24718244463] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted procfs at /proc
[24719666466] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted sysfs at /sys
[24724064838] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /tmp
[24725287521] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /run
[24726386355] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /services
[24727505253] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /session
[24728725758] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /data
[24729603129] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='vfs_init' elapsed_ticks=48276063 elapsed_us=24138 total_ticks=53919657 total_us=26959
[24734820660] [[34mDEBUG[0m] [kernel] [CPU0] Scanning PCI bus...
[24738746142] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x29c0 at 00:00.0 class=060000 id=1
[24755656068] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1050 at 00:01.0 class=030000 id=2
[24773949585] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1000 at 00:02.0 class=020000 id=3
[24775745016] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2918 at 00:1f.0 class=060100 id=4
[24784437975] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2922 at 00:1f.2 class=010601 id=5
[24787534827] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2930 at 00:1f.3 class=0c0500 id=6
[24790541259] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='scan_pci' elapsed_ticks=54916323 elapsed_us=27458 total_ticks=114846831 total_us=57423
[24796562571] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='register_legacy_devices' elapsed_ticks=781473 elapsed_us=390 total_ticks=120880518 total_us=60440
[24801774393] [[34mDEBUG[0m] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[24808732278] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[24809691522] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[24811046931] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[24812232093] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[24816449526] [[34mDEBUG[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[24819547698] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[24820876278] [[34mDEBUG[0m] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[24822123678] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[24823976067] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Registers initialized
[24825292404] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[24826723812] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[24828415887] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: All pins masked
[24830831652] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Init complete
[24865472379] [[34mDEBUG[0m] [bran::arch::x86_64::ioapic] [CPU0] LAPIC: calibrated 62595300 ticks/sec (delta=625953, ok=true) -> init_cnt=625953
[24878432733] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62595300 ticks/sec), init_cnt=625953 for 100Hz
[24880783851] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='setup_preemption_timer' elapsed_ticks=77959596 elapsed_us=38979 total_ticks=205096947 total_us=102548
[24886364712] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[24887599275] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[24889310094] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[24891732855] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[24922785888] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[24924335832] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[24926247489] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] SMP: CPU 1 initializing preemption timer (vec=32 init_cnt=625953)
[24928602006] [[34mDEBUG[0m] [kernel] [CPU1] SMP: kernel_secondary_entry arg_cpu=1 runtime_cpu=1
[24930054633] [[34mDEBUG[0m] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[24932102943] [[34mDEBUG[0m] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[24935943681] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80016ce0 kstack_top=0xffffffffb0032e40 arg=0x1
[24937819962] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Task 2 assigned to CPU 1
[24941474448] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 2 deferred inserts
[24943189590] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=1
[24944463291] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=2
[24946469922] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[24947881365] [[34mDEBUG[0m] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[24949229811] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: run_scheduler entry on CPU 1
[24950673957] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: bootstrap_cpu start on CPU 1
[24951853311] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[24953152455] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle thread 2 and is now schedulable
[24954640392] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[24955839216] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] SMP: CPU 2 initializing preemption timer (vec=32 init_cnt=625953)
[24957590592] [[34mDEBUG[0m] [kernel] [CPU2] SMP: kernel_secondary_entry arg_cpu=2 runtime_cpu=2
[24958877724] [[34mDEBUG[0m] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[24967218705] [[34mDEBUG[0m] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[24968956386] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU2] INIT KERNEL CTX: entry=0xffffffff80016ce0 kstack_top=0xffffffffb00445a0 arg=0x2
[24970842831] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] SCHED: Task 3 assigned to CPU 2
[24972745314] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Applying 1 deferred inserts
[24974392806] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserting TID=3
[24976349178] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserts applied
[24977456790] [[34mDEBUG[0m] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[24978614199] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: run_scheduler entry on CPU 2
[24979862391] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: bootstrap_cpu start on CPU 2
[24982695606] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[24984002670] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle thread 3 and is now schedulable
[24985527567] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[24986762031] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] SMP: CPU 3 initializing preemption timer (vec=32 init_cnt=625953)
[24988985703] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[24990023520] [[34mDEBUG[0m] [kernel] [CPU3] SMP: kernel_secondary_entry arg_cpu=3 runtime_cpu=3
[24991759551] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='start_secondary_cpus' elapsed_ticks=104492454 elapsed_us=52246 total_ticks=314983977 total_us=157491
[24994060443] [[34mDEBUG[0m] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[24999948237] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[25002752082] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80016ce0 kstack_top=0xffffffffb0055d00 arg=0x3
[25005790524] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SCHED: Task 4 assigned to CPU 3
[25008513750] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[25009844607] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=4
[25012049568] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[25013104908] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[25014197736] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: run_scheduler entry on CPU 3
[25015335147] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: bootstrap_cpu start on CPU 3
[25016483547] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle thread 4 and is now schedulable
[25018794339] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Enumerating 108 boot modules...
[25027707507] [[34mDEBUG[0m] [kernel] [CPU0] Found init module: /bin/sprout (cmdline: 'init'), loading...
[25029930321] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/sprout' (len=85144, base=0x200000)
[25038552231] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[25054745067] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20f000 filesz=0 memsz=0 align=1
[25063016088] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='load_init_module' elapsed_ticks=33315447 elapsed_us=16657 total_ticks=387321198 total_us=193660
[25065921045] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='prepare_boot_registry' elapsed_ticks=1009107 elapsed_us=504 total_ticks=390238728 total_us=195119
[25067981400] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='map_boot_registry' elapsed_ticks=149325 elapsed_us=74 total_ticks=392301723 total_us=196150
[25070040369] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='activate_init_aspace' elapsed_ticks=144969 elapsed_us=72 total_ticks=394319376 total_us=197159
[25072023471] [[34mDEBUG[0m] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[25073054061] [[34mDEBUG[0m] [kernel] [CPU0] Spawning init process...
[25075153422] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800508b0 kstack_top=0xffffffffb00661e0 arg=0xffffffffb001fd20
[25089796446] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[25091728497] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=5
[25092695298] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[25093886499] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='spawn_init_task' elapsed_ticks=20876130 elapsed_us=10438 total_ticks=418203984 total_us=209101
[25132022619] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED: early-boot bringup complete; resuming steady-state SMP scheduling
[25133451156] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='end_bringup' elapsed_ticks=1702470 elapsed_us=851 total_ticks=457749435 total_us=228874
[25173399075] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=497516349 elapsed_us=248758
[25174758939] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[25190711898] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[25192413048] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] ENTER_USER: TID=5 entry_pc=200000 user_sp=800000
[25238753034] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=0
[25244481240] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=20
[25255979859] [[34mDEBUG[0m] [sprout] [CPU0] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7fef70 rip=0x20414c rflags=0x202
[25257784926] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[25259240655] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: About to create Supervisor...
[25260499374] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Listing /bin directory...
[25327790466] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Parsed command line: '    '
[25329830922] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: FORCING display=bootfb for diagnostic test!
[25331314503] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Supervisor created, calling run_forever...
[25332700074] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[25333984434] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[25335907377] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[25350296037] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU0] BootFs: EXACT match for 'bin/sh' at index 4
[25367136465] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU0] BootFs: EXACT match for 'bin/sh' at index 4
[25385787141] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'sh' (len=121736, base=0x200000)
[25387889736] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[25401027861] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x217000 filesz=0 memsz=0 align=1
[25406551896] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /bin/sh
[25408525725] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800508b0 kstack_top=0xffffffffb009d500 arg=0xffffffffb006f660
[25412518527] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=6, applying inserts
[25414079196] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[25415146680] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=6
[25416179745] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[25417245645] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[25423343253] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SETUP_STDIO: stdin=Fd(3) stdout=Fd(4) stderr=Fd(5)
[25435492236] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 6
[25439926512] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Task 6 woken, restoring IRQs
[25441323006] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Done for /bin/sh
[25449066357] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[25451578218] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 250ms so the prompt can take the foreground
[25453208121] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[25455034638] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=6 entry_pc=200000 user_sp=800000
[25457378892] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=0
[25459678530] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=100
[25462876296] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[25464510852] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
DEBUG: sh starting
[25476882552] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[25478411607] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
DEBUG: sh sig handlers installed
[25490478552] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'etc/motd' at index 105
[25494545373] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[25496116437] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
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
[25503396006] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[25504849953] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
DEBUG: sh motd printed
[25508017755] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[25509540804] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
DEBUG: sh shell object created
[25513376262] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'etc/profile' at index 104
[25557370608] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[25559087994] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
DEBUG: sh profile loaded
[25568350896] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[25569843618] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [25576185789] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[25577689236] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[?25h[26837501592] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=1374356676
[27680362050] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[27683245986] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[27687509883] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=4096 port=0xffffffffb0071530
[27690197733] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(0)
[27696228747] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=1 port_id=PortId(0) mode=Write
[27698922339] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=2 port_id=PortId(0) mode=Read
[27703978929] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/cambium' (len=124728, base=0x200000)
[27705796635] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[27718625748] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x217000 filesz=0 memsz=0 align=1
[27724869579] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800508b0 kstack_top=0xffffffffb00818e0 arg=0xffffffffb00716c0
[27727919373] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[27729297882] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=7
[27730713153] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[27738348330] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 7 → task '/bin/cambium' (pid=7 from boot module)
[27741067992] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned cambium (PID=7)
[27742250580] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[27745055547] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[27746119533] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=7 entry_pc=200000 user_sp=800000
[27747916548] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=0
[27749515794] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/iso9660d' (len=80648, base=0x200000)
[27752105832] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=100
[27753549912] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[27763446084] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20e000 filesz=0 memsz=0 align=1
[27768697968] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800508b0 kstack_top=0xffffffffb00b6300 arg=0xffffffffb00716c0
[27771622626] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[27772862568] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=8
[27774094986] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[27778101978] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 8 �� task '/bin/iso9660d' (pid=8 from boot module)
[27781471047] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned iso9660d (PID=8)
[27783341157] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[27785904465] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=8 entry_pc=200000 user_sp=800000
[27787974093] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Deferring netd until /dev/net/virtio0/rx is ready...
[27790086522] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=0
[27792058899] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[27793394343] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=100
[27808223586] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Task 9 (user thread) assigned to CPU 3
[27812731287] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[27814151838] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=9
[27815475435] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[27817477314] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Sending post-unlock Resched IPI to CPU 3 for task 9
[27819948057] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Running registration + health supervision loop
[27824101404] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU0] BRIDGE: handle=2 -> fd=3 node=0xffffffffb0069330 port=0xffffffffb0071530
[27827590758] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Bridged resp_port 2 -> FD 3 for task 'cambium'
[27837054630] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ahci_disk' at index 57
[27838521711] [[34mDEBUG[0m] [iso9660d] [CPU2] iso9660d: starting ISO9660 VFS provider
[27844167978] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[28112071680] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[28117302048] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ata_disk' at index 58
[28279445469] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.ata' start='thingos_driver_start_safe'
[28283406294] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/chime' at index 63
[28453734903] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[28458302004] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/display_bootfb' at index 41
[28625731464] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[28631782509] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/display_virtio_gpu' at index 42
[28806083262] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[28810211958] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/hdaudio' at index 61
[28973387850] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[28977343626] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/pci_stubd' at index 62
[29140228755] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[29143793316] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ps2_kbd' at index 3
[29301450486] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[29305286604] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ps2_mouse' at index 40
[29483200230] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[29486490957] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/rtc_cmos' at index 2
[29648613885] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[29653115712] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/rtl8168d' at index 45
[29814920190] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[29818754394] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/virtio_netd' at index 44
[29989897146] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[29994252519] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/virtio_sound' at index 60
[30164546742] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[30167812521] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[30306543399] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=3 starting copyin
[30309548049] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=3 copyin ok
[30319071750] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/virtio_netd' at index 44
[30334783413] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'virtio_netd' (len=106272, base=0x200000)
[30336755856] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[30349499796] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x214000 filesz=0 memsz=0 align=1
[30360822822] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x20277c + bias 0x0 = PC 0x20277c
[30363207765] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/virtio_netd
[30365410647] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff800508b0 kstack_top=0xffffffffb00f8220 arg=0xffffffffb008ca40
[30368702067] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=10, applying inserts
[30370398531] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[30371842908] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=10
[30373182807] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[30374379783] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[30376574283] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[30381845736] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 10
[30388199457] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 10 woken, restoring IRQs
[30389727225] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/virtio_netd
[30394407846] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/virtio_netd for pci-0000:00:02.0 (entry='thingos_driver_start_safe', pid=10)
[30401105658] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=4 starting copyin
[30403026456] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=4 copyin ok
[30407002164] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ahci_disk' at index 57
[30418991229] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'ahci_disk' (len=74072, base=0x200000)
[30421147284] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[30431474832] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x20e000 filesz=0 memsz=0 align=1
[30438789249] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[30441036714] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/ahci_disk
[30442837722] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff800508b0 kstack_top=0xffffffffb0110020 arg=0xffffffffb00be2a0
[30458529189] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=11, applying inserts
[30460101903] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[30461365770] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=11
[30462738603] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[30464056062] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[30466734012] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[30471899931] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 11
[30473727405] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 11 woken, restoring IRQs
[30475337904] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/ahci_disk
[30476691135] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x4
[30479472738] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=11 entry_pc=201000 user_sp=800000
[30480931800] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/ahci_disk for pci-0000:00:1f.2 (entry='thingos_driver_start_safe', pid=11)
[30482715285] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=11 buf_len=0
[30484944930] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=11 buf_len=100
[30487747554] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=5 starting copyin
[30489175167] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver (boot_fd=4)
[30490374684] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=5 copyin ok
[30494248290] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ata_disk' at index 58
[30503778558] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'ata_disk' (len=61712, base=0x200000)
[30505829541] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[30514214148] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x20b000 filesz=0 memsz=0 align=1
[30521014788] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[30523867209] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/ata_disk
[30526164603] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff800508b0 kstack_top=0xffffffffb0124020 arg=0xffffffffb00dd6a0
[30529797375] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=12, applying inserts
[30531504795] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[30533150736] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=12
[30534664248] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[30536130405] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[30538648503] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[30540517128] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 'pci-0000:00:1f.2' (handle 0)
[30545372187] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Claimed PCI device '/sys/devices/pci-0000:00:1f.2' handle=0
[30546883488] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 12
[30548699313] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: mapping BAR5 (phys=0x80880000, size=0x1000) for task 11
[30550646610] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 12 woken, restoring IRQs
[30552359607] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/ata_disk
[30553597470] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x5
[30555817314] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[30557630004] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/ata_disk for isa-01f0 (entry='thingos_driver_start_safe', pid=12)
[30559526811] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[30560741145] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=12 entry_pc=201000 user_sp=800000
[30563250861] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[30567185385] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=12 buf_len=0
[30583107654] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=12 buf_len=100
[30585168702] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[30586751349] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: DMA virt=0x20c000
[30588298950] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[30589898955] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: DMA phys=0x22fa000
[30590969178] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[30592629903] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 0...
[30594028674] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[30595545552] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 1...
[30596833146] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[30598171659] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 2...
[30599714244] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[30602778954] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=4096 port=0xffffffffb00ce690
[30604671867] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(1)
[30607371300] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(1) mode=Write
[30609191910] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=4 port_id=PortId(1) mode=Read
[30611758452] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x20277c SP=0x800000 ARG0=0x3
[30619667331] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=10 entry_pc=20277c user_sp=800000
[30623786655] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=0
[30625921722] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=100
[30627365637] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
[30632061339] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=11 fd=5 starting copyin
[30635162085] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=11 fd=5 copyin ok
[30643358856] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device port=2 model='                                        ' rpc_port=3
[30646350108] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 3...
[30647759241] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[30648995982] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 4...
[30650283147] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[30651597438] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 5...
[30652878366] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[30654378249] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[30655796292] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[30657284229] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[30658669536] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=4 -> fd=5 node=0xffffffffb008ca50 port=0xffffffffb00ce690
[30660362205] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[30662095134] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
[30663835257] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Starting VirtIO-NET driver service...
[30665834034] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Initializing hardware driver...
[30668208648] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Claiming NIC device at /sys/devices/pci-0000:00:02.0
[30670620222] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: device::new(/sys/devices/pci-0000:00:02.0)
[30672208083] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: claiming '/sys/devices/pci-0000:00:02.0'...
[30674357208] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 10 claimed device 'pci-0000:00:02.0' (handle 1)
[30677016018] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: claimed, handle=1
[30687283176] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/common_bar -> '4' (n=2)
[30692610366] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/common_offset -> '0x0' (n=4)
[30697499415] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/notify_bar -> '4' (n=2)
[30704079483] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/notify_offset -> '0x3000' (n=7)
[30708747795] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/notify_multiplier -> '4' (n=2)
[30711469305] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[30716448939] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/device_bar -> '4' (n=2)
[30721180281] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/device_offset -> '0x2000' (n=7)
[30723132198] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: mapping common BAR4...
[30724644819] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: mapping BAR4 (phys=0xc000004000, size=0x4000) for task 10
[30728074641] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x10001000
[30730298115] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: common_cfg at 0x10001000
[30731952570] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: notify_cfg at 0x10004000
[30734085261] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: device_cfg = Some(268447744)
[30735877194] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: allocating DMA command buffer...
[30739734201] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 1 pages phys=0x23ba000 -> user_va=0x10005000
[30744009153] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: cmd_buf virt=0x10005000 phys=0x23ba000
[30745995291] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: device::new complete
[30748755774] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Device features 0x30bf8024
[30751607568] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: MAC 52:54:00:12:34:56
[30753558495] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Link UP
[30758755797] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 4 pages phys=0x23bb000 -> user_va=0x10006000
[30762628578] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: RX queue 0 setup (size=32)
[30768112155] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 4 pages phys=0x23bf000 -> user_va=0x1000a000
[30770837922] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: TX queue 1 setup (size=32)
[30777432213] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 16 pages phys=0x23c3000 -> user_va=0x1000e000
[30779984400] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[30781645257] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 1 pages phys=0x23d3000 -> user_va=0x1001e000
[30784384686] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Allocated TX buffer (1 page)
[30786904632] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: RX queue filled with 32 buffers
[30788661717] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: DRIVER_OK set, device is live
[30790502688] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Driver initialized successfully
[30792292905] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Driver initialized successfully
[30793891491] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: MAC 52:54:00:12:34:56
[30795704379] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Initial link state is UP
[30800727210] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=65536 port=0xffffffffb00d15f0
[30802366551] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(2)
[30803850099] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=3 port_id=PortId(2) mode=Write
[30805665132] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=4 port_id=PortId(2) mode=Read
[30807779475] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Created VFS provider port
[30816960405] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Task 13 (user thread) assigned to CPU 1
[30819459132] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[30820843581] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=13
[30822309474] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[30825918618] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=13 buf_len=0
[30827636136] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=13 buf_len=100
[30829975275] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Entering VFS provider service loop at /dev/net/virtio0
[30832012530] [[34mDEBUG[0m] [kernel::sched] [CPU2] SCHED: CPU 2 stole task 7 (prio 2) from CPU 1 (depth 2)
[30836351337] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=7 fd=6 starting copyin
[30839137890] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=7 fd=6 copyin ok
[30854034222] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=7 fd=6 starting copyin
[30855776325] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=7 fd=6 copyin ok
[30863609601] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=7 fd=6 starting copyin
[30865291908] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=7 fd=6 copyin ok
[30897074175] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=262400 port=0xffffffffb00d2bb0
[30898785819] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(3)
[30901465782] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=5 port_id=PortId(3) mode=Write
[30907203063] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] vfs: mounted userland provider at /dev/net/virtio0 (flags: 0x0)
[30909935793] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Mounted at /dev/net/virtio0
[30911560416] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Provider thread live at /dev/net/virtio0
[32628275196] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=65536 port=0xffffffffb00d3c10
[32631234405] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(4)
[32634022047] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(4) mode=Write
[32637158862] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=4 port_id=PortId(4) mode=Read
[33060970173] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED: CPU 0 resched try_lock actionable misses (excluding idle timer-only misses) reached 100 in 2s (timer=96 ipi=4 pending=4 idle_timer=0 last_trigger=timer_tick suppressing until window reset)
[34755542976] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=65536 port=0xffffffffb00d4af0
[34757351079] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(5)
[34758875019] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=5 port_id=PortId(5) mode=Write
[34761012891] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=6 port_id=PortId(5) mode=Read
[36244566501] [[34mDEBUG[0m] [kernel::sched] [CPU1] SCHED: CPU 1 resched try_lock actionable misses (excluding idle timer-only misses) reached 100 in 2s (timer=100 ipi=0 pending=0 idle_timer=0 last_trigger=timer_tick suppressing until window reset)
[44276839101] [[31;1mERROR[0m] [bran] [CPU3] panicked at thingos/bran/src/arch/x86_64/idt.rs:1327:5:
PAGE FAULT at 0x23 RIP=0x23 CS=0x8 ERR=0x10
[2286277466
```
</details>
