# ✅ Scenario: Launch ps and verify reaping

> Last run: 2026-04-22 08:46:56

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8722ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 2214ms | - [📜](./02/serial.log) - |
| 3 | And I type "ps" on the serial console | ✅ | 1170ms | - [📜](./03/serial.log) - |
| 4 | Then the serial output should contain "PID" | ✅ | 13ms | - - - |
| 5 | And the serial output should contain "sh" | ✅ | 12ms | - - - |
| 6 | And the serial output should contain "ps" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[26917752873] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] enter
[26929544169] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] phys_memory_map ok
[26934640359] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] modules ok
[26936702529] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] phys_to_virt_offset ok
[26938699755] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] memory map logging done
[26967830307] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] boot_frame_alloc init ok
[27223517508] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] frame allocator build ok
[27225699732] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] frame allocator log ok
[27228222384] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] FRAME_ALLOCATOR init ok
[27230413980] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] tasking init ok
[27236464497] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[27238499541] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] enter
[27241178613] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] set expand hook
[27243284112] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] expand hook ok
[27245081952] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock begin
[27247506825] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock ok
[27248788380] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] reserve_region begin
[27290109462] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] reserve_region ok
[27292350096] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] inner allocator init begin
[27298140870] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] inner allocator init ok
[27300540729] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] heap top store ok
[27302626131] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] init done
[27314102739] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] set_boot_fb begin
[27316460391] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] devfs: set_boot_fb width=1920 height=1080 pitch=7680 resource_id=0xfb000000
[27320429994] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] set_boot_fb ok
[27323277366] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs set_boot_fb elapsed_ticks=8446812 elapsed_us=4223
[27332239737] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs FbNode::new elapsed_ticks=6217464 elapsed_us=3108
[27334791462] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] register begin
[27340942629] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] register ok
[27342885405] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs register fb0 elapsed_ticks=8023224 elapsed_us=4011
[27349432935] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[27365206077] [[34mDEBUG[0m] [kernel] [CPU0] Seeding entropy pool...
[27381629583] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed begin
[27383520747] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] fill_entropy done
[27386356503] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] add_sample(timer) ok
[27387500547] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] mark_seeded(timer) ok
[27388598193] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[27389894235] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[27391305810] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed done
[27392980098] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] entropy::seed_from_hardware elapsed_ticks=11126445 elapsed_us=5563
[27401642598] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[27416582523] [[35mTRACE[0m] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[27419178006] [[35mTRACE[0m] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[27421260306] [[35mTRACE[0m] [kernel::sched] [CPU0]   Allocating scheduler...
[27428748270] [[35mTRACE[0m] [kernel::sched] [CPU0]   Leaking scheduler...
[27430468494] [[35mTRACE[0m] [kernel::sched] [CPU0]   Initializing boot task...
[27435435291] [[35mTRACE[0m] [kernel::sched] [CPU0]   Creating boot task...
[27453292515] [[35mTRACE[0m] [kernel::sched] [CPU0]   Creating idle tasks...
[27455245422] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80016cc0 kstack_top=0xffffffffb001f980 arg=0x0
[27458706693] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[27481607934] [[35mTRACE[0m] [kernel::sched] [CPU0]   Boot task initialized
[27482769369] [[35mTRACE[0m] [kernel::sched] [CPU0]   Storing scheduler pointer...
[27484515300] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[27490976733] [[34mDEBUG[0m] [kernel] [CPU0] Initializing VFS...
[27495006264] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='set_cmdline' elapsed_ticks=2847009 elapsed_us=1423 total_ticks=3846612 total_us=1923
[27516798474] [[34mDEBUG[0m] [kernel::vfs] [CPU0] VFS: Created /dev/display directory
[27534208713] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted union filesystem at / (root)
[27537287910] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted devfs at /dev
[27538967379] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted procfs at /proc
[27540663678] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted sysfs at /sys
[27546057759] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /tmp
[27547467123] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /run
[27548689905] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /services
[27549979644] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /session
[27551295585] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /data
[27552282846] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='vfs_init' elapsed_ticks=55378818 elapsed_us=27689 total_ticks=61395972 total_us=30697
[27558147540] [[34mDEBUG[0m] [kernel] [CPU0] Scanning PCI bus...
[27562568385] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x29c0 at 00:00.0 class=060000 id=1
[27569039850] [[35mTRACE[0m] [kernel] [CPU0]   BAR0 (MEM32): 0x80000000 (size 0x800000)
[27582646509] [[35mTRACE[0m] [kernel] [CPU0]   BAR2 (MEM64): 0xc000000000 (size 0x4000)
[27586866351] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (MEM32): 0x80882000 (size 0x1000)
[27588179751] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1050 at 00:01.0 class=030000 id=2
[27592978974] [[35mTRACE[0m] [kernel] [CPU0]   BAR0 (I/O):  0x6060 (size 0x20)
[27596826741] [[35mTRACE[0m] [kernel] [CPU0]   BAR1 (MEM32): 0x80881000 (size 0x1000)
[27608899362] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (MEM64): 0xc000004000 (size 0x4000)
[27610290774] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1000 at 00:02.0 class=020000 id=3
[27612579852] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2918 at 00:1f.0 class=060100 id=4
[27616228761] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (I/O):  0x6040 (size 0x20)
[27619089960] [[35mTRACE[0m] [kernel] [CPU0]   BAR5 (MEM32): 0x80880000 (size 0x1000)
[27620402106] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2922 at 00:1f.2 class=010601 id=5
[27624314289] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (I/O):  0x6000 (size 0x40)
[27625917264] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2930 at 00:1f.3 class=0c0500 id=6
[27629838258] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='scan_pci' elapsed_ticks=70674681 elapsed_us=35337 total_ticks=138939273 total_us=69469
[27636744729] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='register_legacy_devices' elapsed_ticks=805497 elapsed_us=402 total_ticks=145861353 total_us=72930
[27642998361] [[34mDEBUG[0m] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[27650479989] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[27651809757] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[27653646042] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[27655210506] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[27659950098] [[34mDEBUG[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[27662104866] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[27663690681] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[27665054538] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[27666951345] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[27668355891] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[27670045986] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[27671658729] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[27673372650] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[27675570285] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[27677836725] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[27680107257] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[27686771409] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[27689113848] [[34mDEBUG[0m] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[27691224924] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[27694574127] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Registers initialized
[27696936003] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[27699572307] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[27702479079] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: All pins masked
[27705975198] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> NMI
[27708168939] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[27710419143] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ4 -> GSI 4 -> 0x24
[27712189824] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Init complete
[27748151772] [[34mDEBUG[0m] [bran::arch::x86_64::ioapic] [CPU0] LAPIC: calibrated 62466600 ticks/sec (delta=624666, ok=true) -> init_cnt=624666
[27752292447] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62466600 ticks/sec), init_cnt=624666 for 100Hz
[27754876611] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='setup_preemption_timer' elapsed_ticks=110668437 elapsed_us=55334 total_ticks=263968188 total_us=131984
[27764969331] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[27767322660] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[27770515047] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[27774809601] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[27810596319] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[27812998818] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[27814780950] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] SMP: CPU 1 initializing preemption timer (vec=32 init_cnt=624666)
[27819080091] [[34mDEBUG[0m] [kernel] [CPU1] SMP: kernel_secondary_entry arg_cpu=1 runtime_cpu=1
[27821654025] [[34mDEBUG[0m] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[27824826348] [[34mDEBUG[0m] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[27831559305] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80016cc0 kstack_top=0xffffffffb0032e40 arg=0x1
[27834730572] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Task 2 assigned to CPU 1
[27841032648] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 2 deferred inserts
[27843984036] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=1
[27845929023] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] SMP: CPU 2 initializing preemption timer (vec=32 init_cnt=624666)
[27849022212] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[27851120121] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=2
[27853049895] [[34mDEBUG[0m] [kernel] [CPU2] SMP: kernel_secondary_entry arg_cpu=2 runtime_cpu=2
[27855320196] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[27857387085] [[34mDEBUG[0m] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[27859282638] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[27861101928] [[34mDEBUG[0m] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[27863238117] [[34mDEBUG[0m] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[27865015794] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU2] INIT KERNEL CTX: entry=0xffffffff80016cc0 kstack_top=0xffffffffb00445a0 arg=0x2
[27867930057] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: run_scheduler entry on CPU 1
[27869790003] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] SCHED: Task 3 assigned to CPU 2
[27871816137] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: bootstrap_cpu start on CPU 1
[27873610710] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Applying 1 deferred inserts
[27875532597] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle thread 2 and is now schedulable
[27878090394] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserting TID=3
[27879723795] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserts applied
[27881079501] [[34mDEBUG[0m] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[27882350496] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: run_scheduler entry on CPU 2
[27883656240] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: bootstrap_cpu start on CPU 2
[27885229317] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle thread 3 and is now schedulable
[27897122880] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[27899164557] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] SMP: CPU 3 initializing preemption timer (vec=32 init_cnt=624666)
[27905173560] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[27906543720] [[34mDEBUG[0m] [kernel] [CPU3] SMP: kernel_secondary_entry arg_cpu=3 runtime_cpu=3
[27908696046] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[27910068021] [[34mDEBUG[0m] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[27913260639] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='start_secondary_cpus' elapsed_ticks=146169870 elapsed_us=73084 total_ticks=419908863 total_us=209954
[27920309604] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[27923662569] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80016cc0 kstack_top=0xffffffffb0055d00 arg=0x3
[27927212280] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SCHED: Task 4 assigned to CPU 3
[27931050477] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[27933389385] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=4
[27936892995] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[27938944836] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[27941044395] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: run_scheduler entry on CPU 3
[27943110657] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: bootstrap_cpu start on CPU 3
[27945283905] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle thread 4 and is now schedulable
[27949109793] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Enumerating 108 boot modules...
[27952852785] [[35mTRACE[0m] [kernel] [CPU0]   Module[0]: name='/bin/sprout' cmdline='init' size=85144 bytes
[27955193871] [[35mTRACE[0m] [kernel] [CPU0]   Module[1]: name='/bin/bristle' cmdline='' size=45888 bytes
[27957413154] [[35mTRACE[0m] [kernel] [CPU0]   Module[2]: name='/drivers/rtc_cmos' cmdline='init' size=39744 bytes
[27959763975] [[35mTRACE[0m] [kernel] [CPU0]   Module[3]: name='/drivers/ps2_kbd' cmdline='' size=38760 bytes
[27961765029] [[35mTRACE[0m] [kernel] [CPU0]   Module[4]: name='/bin/sh' cmdline='init' size=121736 bytes
[27963153240] [[35mTRACE[0m] [kernel] [CPU0]   Module[5]: name='/bin/ls' cmdline='init' size=62192 bytes
[27964602996] [[35mTRACE[0m] [kernel] [CPU0]   Module[6]: name='/bin/ln' cmdline='init' size=47624 bytes
[27965973849] [[35mTRACE[0m] [kernel] [CPU0]   Module[7]: name='/bin/ps' cmdline='init' size=50608 bytes
[27967444626] [[35mTRACE[0m] [kernel] [CPU0]   Module[8]: name='/bin/kill' cmdline='init' size=51552 bytes
[27969522405] [[35mTRACE[0m] [kernel] [CPU0]   Module[9]: name='/bin/killall' cmdline='init' size=67712 bytes
[27971862501] [[35mTRACE[0m] [kernel] [CPU0]   Module[10]: name='/bin/cat' cmdline='init' size=46168 bytes
[27974169069] [[35mTRACE[0m] [kernel] [CPU0]   Module[11]: name='/bin/head' cmdline='init' size=56168 bytes
[27976424454] [[35mTRACE[0m] [kernel] [CPU0]   Module[12]: name='/bin/tail' cmdline='init' size=52048 bytes
[27980800386] [[35mTRACE[0m] [kernel] [CPU0]   Module[13]: name='/bin/top' cmdline='init' size=65544 bytes
[27986201067] [[35mTRACE[0m] [kernel] [CPU0]   Module[14]: name='/bin/wc' cmdline='init' size=56312 bytes
[27987861132] [[35mTRACE[0m] [kernel] [CPU0]   Module[15]: name='/bin/yes' cmdline='init' size=40480 bytes
[27990108531] [[35mTRACE[0m] [kernel] [CPU0]   Module[16]: name='/bin/which' cmdline='init' size=46408 bytes
[27992013390] [[35mTRACE[0m] [kernel] [CPU0]   Module[17]: name='/bin/cp' cmdline='init' size=46376 bytes
[27993433578] [[35mTRACE[0m] [kernel] [CPU0]   Module[18]: name='/bin/mv' cmdline='init' size=47104 bytes
[27994864458] [[35mTRACE[0m] [kernel] [CPU0]   Module[19]: name='/bin/rm' cmdline='init' size=47184 bytes
[27997347312] [[35mTRACE[0m] [kernel] [CPU0]   Module[20]: name='/bin/rmdir' cmdline='init' size=47208 bytes
[27998817495] [[35mTRACE[0m] [kernel] [CPU0]   Module[21]: name='/bin/mkdir' cmdline='init' size=47200 bytes
[28000231941] [[35mTRACE[0m] [kernel] [CPU0]   Module[22]: name='/bin/mount' cmdline='init' size=57752 bytes
[28001644572] [[35mTRACE[0m] [kernel] [CPU0]   Module[23]: name='/bin/echo' cmdline='init' size=40584 bytes
[28003059051] [[35mTRACE[0m] [kernel] [CPU0]   Module[24]: name='/bin/grep' cmdline='init' size=78216 bytes
[28004482341] [[35mTRACE[0m] [kernel] [CPU0]   Module[25]: name='/bin/pwd' cmdline='init' size=35144 bytes
[28006039116] [[35mTRACE[0m] [kernel] [CPU0]   Module[26]: name='/bin/touch' cmdline='init' size=46216 bytes
[28008489894] [[35mTRACE[0m] [kernel] [CPU0]   Module[27]: name='/bin/setshell' cmdline='init' size=47104 bytes
[28010754189] [[35mTRACE[0m] [kernel] [CPU0]   Module[28]: name='/bin/dmesg' cmdline='init' size=39392 bytes
[28012210149] [[35mTRACE[0m] [kernel] [CPU0]   Module[29]: name='/bin/stat' cmdline='init' size=46264 bytes
[28013582982] [[35mTRACE[0m] [kernel] [CPU0]   Module[30]: name='/bin/file' cmdline='init' size=51160 bytes
[28015735539] [[35mTRACE[0m] [kernel] [CPU0]   Module[31]: name='/bin/dirname' cmdline='init' size=50160 bytes
[28017968880] [[35mTRACE[0m] [kernel] [CPU0]   Module[32]: name='/bin/basename' cmdline='init' size=50168 bytes
[28019347917] [[35mTRACE[0m] [kernel] [CPU0]   Module[33]: name='/bin/sleep' cmdline='init' size=56880 bytes
[28020811038] [[35mTRACE[0m] [kernel] [CPU0]   Module[34]: name='/bin/sort' cmdline='init' size=66400 bytes
[28022930991] [[35mTRACE[0m] [kernel] [CPU0]   Module[35]: name='/bin/env' cmdline='init' size=41792 bytes
[28024325967] [[35mTRACE[0m] [kernel] [CPU0]   Module[36]: name='/bin/uname' cmdline='' size=40600 bytes
[28025660025] [[35mTRACE[0m] [kernel] [CPU0]   Module[37]: name='/bin/true' cmdline='' size=20736 bytes
[28027055925] [[35mTRACE[0m] [kernel] [CPU0]   Module[38]: name='/bin/false' cmdline='' size=20736 bytes
[28028962896] [[35mTRACE[0m] [kernel] [CPU0]   Module[39]: name='/bin/input_echo' cmdline='' size=43096 bytes
[28030789248] [[35mTRACE[0m] [kernel] [CPU0]   Module[40]: name='/drivers/ps2_mouse' cmdline='' size=39312 bytes
[28032194355] [[35mTRACE[0m] [kernel] [CPU0]   Module[41]: name='/drivers/display_bootfb' cmdline='' size=81384 bytes
[28033648566] [[35mTRACE[0m] [kernel] [CPU0]   Module[42]: name='/drivers/display_virtio_gpu' cmdline='' size=128072 bytes
[28035245601] [[35mTRACE[0m] [kernel] [CPU0]   Module[43]: name='/bin/cambium' cmdline='' size=124728 bytes
[28037102643] [[35mTRACE[0m] [kernel] [CPU0]   Module[44]: name='/drivers/virtio_netd' cmdline='' size=106272 bytes
[28039059081] [[35mTRACE[0m] [kernel] [CPU0]   Module[45]: name='/drivers/rtl8168d' cmdline='' size=57360 bytes
[28041278628] [[35mTRACE[0m] [kernel] [CPU0]   Module[46]: name='/bin/netd' cmdline='' size=10813352 bytes
[28042735875] [[35mTRACE[0m] [kernel] [CPU0]   Module[47]: name='/bin/mesocarp' cmdline='' size=119480 bytes
[28044137814] [[35mTRACE[0m] [kernel] [CPU0]   Module[48]: name='/bin/mdns' cmdline='' size=119480 bytes
[28045467186] [[35mTRACE[0m] [kernel] [CPU0]   Module[49]: name='/bin/mdnsd' cmdline='' size=119480 bytes
[28046841174] [[35mTRACE[0m] [kernel] [CPU0]   Module[50]: name='/bin/fetchd' cmdline='' size=72720 bytes
[28049239086] [[35mTRACE[0m] [kernel] [CPU0]   Module[51]: name='/bin/httpsd' cmdline='' size=526160 bytes
[28051213212] [[35mTRACE[0m] [kernel] [CPU0]   Module[52]: name='/bin/find' cmdline='' size=57960 bytes
[28052572845] [[35mTRACE[0m] [kernel] [CPU0]   Module[53]: name='/bin/ip' cmdline='' size=55528 bytes
[28053955314] [[35mTRACE[0m] [kernel] [CPU0]   Module[54]: name='/bin/iso_reader' cmdline='' size=37504 bytes
[28055362599] [[35mTRACE[0m] [kernel] [CPU0]   Module[55]: name='/bin/ping' cmdline='' size=61144 bytes
[28056708537] [[35mTRACE[0m] [kernel] [CPU0]   Module[56]: name='/bin/nslookup' cmdline='' size=56480 bytes
[28058152815] [[35mTRACE[0m] [kernel] [CPU0]   Module[57]: name='/drivers/ahci_disk' cmdline='' size=74072 bytes
[28059558483] [[35mTRACE[0m] [kernel] [CPU0]   Module[58]: name='/drivers/ata_disk' cmdline='' size=61712 bytes
[28061604153] [[35mTRACE[0m] [kernel] [CPU0]   Module[59]: name='/bin/iso9660d' cmdline='' size=80648 bytes
[28063108128] [[35mTRACE[0m] [kernel] [CPU0]   Module[60]: name='/drivers/virtio_sound' cmdline='' size=95232 bytes
[28065050277] [[35mTRACE[0m] [kernel] [CPU0]   Module[61]: name='/drivers/hdaudio' cmdline='' size=67600 bytes
[28066643517] [[35mTRACE[0m] [kernel] [CPU0]   Module[62]: name='/drivers/pci_stubd' cmdline='' size=58224 bytes
[28068117066] [[35mTRACE[0m] [kernel] [CPU0]   Module[63]: name='/drivers/chime' cmdline='' size=55496 bytes
[28069500492] [[35mTRACE[0m] [kernel] [CPU0]   Module[64]: name='/bin/vfs_hello' cmdline='' size=35552 bytes
[28071036972] [[35mTRACE[0m] [kernel] [CPU0]   Module[65]: name='/bin/show_args' cmdline='' size=42568 bytes
[28073310441] [[35mTRACE[0m] [kernel] [CPU0]   Module[66]: name='/bin/env_roundtrip' cmdline='' size=44216 bytes
[28074736008] [[35mTRACE[0m] [kernel] [CPU0]   Module[67]: name='/bin/cwd_test' cmdline='' size=43776 bytes
[28076177448] [[35mTRACE[0m] [kernel] [CPU0]   Module[68]: name='/bin/date' cmdline='' size=62992 bytes
[28077567573] [[35mTRACE[0m] [kernel] [CPU0]   Module[69]: name='/bin/wayland_hello' cmdline='' size=65952 bytes
[28079077290] [[35mTRACE[0m] [kernel] [CPU0]   Module[70]: name='/bin/terminal' cmdline='' size=85376 bytes
[28081274166] [[35mTRACE[0m] [kernel] [CPU0]   Module[71]: name='/bin/tee' cmdline='' size=45032 bytes
[28083615417] [[35mTRACE[0m] [kernel] [CPU0]   Module[72]: name='/bin/xargs' cmdline='' size=55080 bytes
[28085773584] [[35mTRACE[0m] [kernel] [CPU0]   Module[73]: name='/bin/placed' cmdline='' size=38136 bytes
[28087955544] [[35mTRACE[0m] [kernel] [CPU0]   Module[74]: name='/bin/bloom' cmdline='' size=112872 bytes
[28090061406] [[35mTRACE[0m] [kernel] [CPU0]   Module[75]: name='/bin/clear' cmdline='' size=20864 bytes
[28092203700] [[35mTRACE[0m] [kernel] [CPU0]   Module[76]: name='/bin/loglevel' cmdline='' size=41600 bytes
[28094359524] [[35mTRACE[0m] [kernel] [CPU0]   Module[77]: name='/bin/poll_mux' cmdline='' size=39056 bytes
[28096557522] [[35mTRACE[0m] [kernel] [CPU0]   Module[78]: name='/bin/ipc_service_demo' cmdline='' size=47552 bytes
[28098798222] [[35mTRACE[0m] [kernel] [CPU0]   Module[79]: name='/bin/ipc_pipe_demo' cmdline='' size=44016 bytes
[28101072087] [[35mTRACE[0m] [kernel] [CPU0]   Module[80]: name='/bin/ipc_provider_demo' cmdline='' size=67008 bytes
[28103415417] [[35mTRACE[0m] [kernel] [CPU0]   Module[81]: name='/bin/ipc_memfd_demo' cmdline='' size=34216 bytes
[28107554541] [[35mTRACE[0m] [kernel] [CPU0]   Module[82]: name='/bin/test_exec' cmdline='' size=62504 bytes
[28109753925] [[35mTRACE[0m] [kernel] [CPU0]   Module[83]: name='/bin/test_vm_protect' cmdline='' size=34216 bytes
[28112007000] [[35mTRACE[0m] [kernel] [CPU0]   Module[84]: name='/bin/test_exec_env' cmdline='' size=54864 bytes
[28114440354] [[35mTRACE[0m] [kernel] [CPU0]   Module[85]: name='/bin/test_threads' cmdline='' size=78336 bytes
[28117065933] [[35mTRACE[0m] [kernel] [CPU0]   Module[86]: name='/bin/test_futex' cmdline='' size=48952 bytes
[28119334419] [[35mTRACE[0m] [kernel] [CPU0]   Module[87]: name='/bin/ld_so' cmdline='' size=378656 bytes
[28121490375] [[35mTRACE[0m] [kernel] [CPU0]   Module[88]: name='/bin/test_dyn_loader' cmdline='' size=53728 bytes
[28123771698] [[35mTRACE[0m] [kernel] [CPU0]   Module[89]: name='/bin/test_dlopen' cmdline='' size=60232 bytes
[28126012464] [[35mTRACE[0m] [kernel] [CPU0]   Module[90]: name='/bin/reboot' cmdline='' size=32136 bytes
[28128166935] [[35mTRACE[0m] [kernel] [CPU0]   Module[91]: name='/bin/shutdown' cmdline='' size=32136 bytes
[28130350083] [[35mTRACE[0m] [kernel] [CPU0]   Module[92]: name='/bin/attr_list' cmdline='' size=47456 bytes
[28132686054] [[35mTRACE[0m] [kernel] [CPU0]   Module[93]: name='/bin/attr_get' cmdline='' size=47552 bytes
[28134849336] [[35mTRACE[0m] [kernel] [CPU0]   Module[94]: name='/bin/attr_set' cmdline='' size=69656 bytes
[28137152967] [[35mTRACE[0m] [kernel] [CPU0]   Module[95]: name='/bin/attr_rm' cmdline='' size=47072 bytes
[28139874345] [[35mTRACE[0m] [kernel] [CPU0]   Module[96]: name='/lib/libpistil.so' cmdline='' size=67704 bytes
[28142078877] [[35mTRACE[0m] [kernel] [CPU0]   Module[97]: name='/share/wallpapers/clouds.bmp' cmdline='' size=786486 bytes
[28144453095] [[35mTRACE[0m] [kernel] [CPU0]   Module[98]: name='/share/wallpapers/flower.bmp' cmdline='' size=8386614 bytes
[28147040757] [[35mTRACE[0m] [kernel] [CPU0]   Module[99]: name='/share/wallpapers/leather.bmp' cmdline='' size=1179702 bytes
[28149832392] [[35mTRACE[0m] [kernel] [CPU0]   Module[100]: name='/share/wallpapers/linen.bmp' cmdline='' size=4718646 bytes
[28152337983] [[35mTRACE[0m] [kernel] [CPU0]   Module[101]: name='/share/fonts/NotoSans-Regular.ttf' cmdline='' size=569208 bytes
[28154872449] [[35mTRACE[0m] [kernel] [CPU0]   Module[102]: name='/share/fonts/unifont.hex' cmdline='' size=8355852 bytes
[28157326692] [[35mTRACE[0m] [kernel] [CPU0]   Module[103]: name='/etc/locale.conf' cmdline='' size=85 bytes
[28159603494] [[35mTRACE[0m] [kernel] [CPU0]   Module[104]: name='/etc/profile' cmdline='' size=68 bytes
[28161846537] [[35mTRACE[0m] [kernel] [CPU0]   Module[105]: name='/etc/motd' cmdline='' size=610 bytes
[28164045393] [[35mTRACE[0m] [kernel] [CPU0]   Module[106]: name='/etc/fstab' cmdline='' size=123 bytes
[28166252235] [[35mTRACE[0m] [kernel] [CPU0]   Module[107]: name='/etc/hostname' cmdline='' size=8 bytes
[28180477446] [[34mDEBUG[0m] [kernel] [CPU0] Found init module: /bin/sprout (cmdline: 'init'), loading...
[28184490048] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/sprout' (len=85144, base=0x200000)
[28187239047] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[28205042382] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[28209129234] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[28216870275] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [00, 00, 00, e8, e0, 7b, 00, 00]
[28227359457] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20c000 exec=false
[28230864354] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20e000 exec=false
[28238143890] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20f000 filesz=0 memsz=0 align=1
[28249668513] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='load_init_module' elapsed_ticks=65598621 elapsed_us=32799 total_ticks=758769297 total_us=379384
[28252913502] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='prepare_boot_registry' elapsed_ticks=1098966 elapsed_us=549 total_ticks=762032139 total_us=381016
[28255455657] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='map_boot_registry' elapsed_ticks=261228 elapsed_us=130 total_ticks=764561853 total_us=382280
[28258752423] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='activate_init_aspace' elapsed_ticks=475563 elapsed_us=237 total_ticks=767780937 total_us=383890
[28261127697] [[34mDEBUG[0m] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[28262305434] [[34mDEBUG[0m] [kernel] [CPU0] Spawning init process...
[28264765518] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb00661e0 arg=0xffffffffb001fd20
[28288045566] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[28289936169] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=5
[28291242210] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[28292411895] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='spawn_init_task' elapsed_ticks=30157611 elapsed_us=15078 total_ticks=801525846 total_us=400762
[28337863719] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED: early-boot bringup complete; resuming steady-state SMP scheduling
[28339855302] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='end_bringup' elapsed_ticks=2427117 elapsed_us=1213 total_ticks=848943579 total_us=424471
[28384710915] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=893588289 elapsed_us=446794
[28386306828] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[28403500356] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[28405323408] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] ENTER_USER: TID=5 entry_pc=200000 user_sp=800000
[28456847133] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpus_online=4
[28467005589] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=0 curr=Some(5) runq=1 runq_avg=2 runq_samples=1 ctxsw=1 idle2busy=0 tick=1 ipi=0 enq=3 deq=1 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[28473884439] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=1 curr=Some(2) runq=0 runq_avg=0 runq_samples=18 ctxsw=0 idle2busy=0 tick=13 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[28480127379] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=2 curr=Some(3) runq=0 runq_avg=0 runq_samples=19 ctxsw=0 idle2busy=0 tick=15 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[28484514003] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=3 curr=Some(4) runq=0 runq_avg=0 runq_samples=17 ctxsw=0 idle2busy=0 tick=13 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[28495351137] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=0
[28500758253] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=20
[28515309570] [[34mDEBUG[0m] [sprout] [CPU0] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7fef70 rip=0x20414c rflags=0x202
[28517765562] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[28519122720] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: About to create Supervisor...
[28520389359] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Listing /bin directory...
[28523766183] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/bin' tid=5
[28606569486] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Parsed command line: '    '
[28610726595] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: FORCING display=bootfb for diagnostic test!
[28613455431] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Supervisor created, calling run_forever...
[28615958415] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[28618325670] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[28620737244] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[28623099879] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/run/sprout/shell' tid=5
[28635751584] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/etc/default/shell' tid=5
[28646214069] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/bin/sh' tid=5
[28649663922] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU0] BootFs: EXACT match for 'bin/sh' at index 4
[28654279731] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[28659945435] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[28663652094] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[28677068772] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU0] BootFs: EXACT match for 'bin/sh' at index 4
[28698170061] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'sh' (len=121736, base=0x200000)
[28700740299] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[28705642383] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[28708712835] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[28712441307] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [9c, b9, 00, 00, 48, 8d, 7c, 24]
[28724225475] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[28727764956] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=216000 exec=false
[28730722845] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x217000 filesz=0 memsz=0 align=1
[28738133028] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /bin/sh
[28740417750] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb009d500 arg=0xffffffffb006f660
[28745290365] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=6, applying inserts
[28746743190] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[28748005770] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=6
[28749258516] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[28750704147] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[28757158815] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SETUP_STDIO: stdin=Fd(3) stdout=Fd(4) stderr=Fd(5)
[28774535988] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 6
[28780796649] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[28784016756] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Task 6 woken, restoring IRQs
[28786166178] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Done for /bin/sh
[28792355064] [[35mTRACE[0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[28800655290] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[28805239188] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[28810092399] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=6 entry_pc=200000 user_sp=800000
[28820138853] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=0
[28823700180] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=100
[28829681463] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=19
[28832731224] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[28835179626] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[28842164010] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=19
DEBUG: sh starting
[28882135854] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=33
[28884988011] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[28889159475] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[28892018067] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=33
DEBUG: sh sig handlers installed
[28895618664] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 250ms so the prompt can take the foreground
[28897906224] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/run/motd' tid=6
[28908281193] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/etc/motd' tid=6
[28913291781] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'etc/motd' at index 105
[28920582207] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=610
[28923080637] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[28932238896] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[28936513815] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=610
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
[28964098416] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=23
[28969243677] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[28971937137] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[28974856086] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=23
DEBUG: sh motd printed
[28980228915] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=31
[28982571387] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[28984989858] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[28987581645] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=31
DEBUG: sh shell object created
[28990920783] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/etc/profile' tid=6
[28997633082] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'etc/profile' at index 104
[29041759065] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=25
[29044807242] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[29047603332] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[29050865514] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=25
DEBUG: sh profile loaded
[29067628854] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=97
[29070496851] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[29072990562] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[29075573241] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=97
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [29085980649] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=6
[29088468519] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[29090917185] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[29093437065] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=6
[?25h[29098992252] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[30777690141] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=1858769814
[30783325815] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpus_online=4
[30787866252] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=0 curr=Some(0) runq=0 runq_avg=1 runq_samples=12 ctxsw=4 idle2busy=0 tick=10 ipi=0 enq=13 deq=12 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[30793784571] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=1 curr=Some(2) runq=0 runq_avg=0 runq_samples=91 ctxsw=0 idle2busy=0 tick=47 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[30799431234] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=2 curr=Some(3) runq=0 runq_avg=0 runq_samples=92 ctxsw=0 idle2busy=0 tick=43 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[30805163433] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=3 curr=Some(6) runq=0 runq_avg=0 runq_samples=4486 ctxsw=1 idle2busy=1 tick=26 ipi=1 enq=4459 deq=4458 wake=1 lock_miss=0 lock_pending=0 lock_blocked=0
[31554076620] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=0 apic_id=0 vector=0x30
[31560244122] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[31562833533] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[31569939852] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=4096 port=0xffffffffb00715b0
[31574367924] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(0)
[31583464506] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=1 port_id=PortId(0) mode=Write
[31587780477] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=2 port_id=PortId(0) mode=Read
[31598506302] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/cambium' (len=124728, base=0x200000)
[31601188377] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[31606746270] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[31609596018] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[31614266937] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [8d, 3c, f0, 48, 83, c7, 70, 48]
[31630603851] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=214000 exec=false
[31634205273] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=216000 exec=false
[31639814382] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x217000 filesz=0 memsz=0 align=1
[31649939046] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb0081720 arg=0xffffffffb006f700
[31653526905] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[31654735134] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=7
[31656089685] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[31666457658] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 7 �� task '/bin/cambium' (pid=7 from boot module)
[31668568569] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=1 apic_id=1 vector=0x30
[31674588627] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned cambium (PID=7)
[31676141640] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[31678693992] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=7 entry_pc=200000 user_sp=800000
[31680417516] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[31681934823] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=0
[31684697154] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=100
[31686991908] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/iso9660d' (len=80648, base=0x200000)
[31689579735] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[31694559105] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[31697540193] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[31702206261] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [48, 89, 44, 24, 50, 48, 8d, 05]
[31710278952] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20b000 exec=false
[31713688017] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20d000 exec=false
[31716760647] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20e000 filesz=0 memsz=0 align=1
[31724201586] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb00b6440 arg=0xffffffffb006f700
[31729181484] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[31731335691] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[31733485476] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=8
[31735881243] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[31739147682] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers' tid=7
[31744529157] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 8 �� task '/bin/iso9660d' (pid=8 from boot module)
[31747312509] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[31751214462] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned iso9660d (PID=8)
[31753000653] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[31755633129] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Deferring netd until /dev/net/virtio0/rx is ready...
[31757922669] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=8 entry_pc=200000 user_sp=800000
[31760202342] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=0
[31762481949] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=100
[31783138926] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Task 9 (user thread) assigned to CPU 3
[31790731599] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[31792643454] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running ��� Runnable (cpu=3, next=9)
[31794590817] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=9
[31796670015] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[31800102576] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Sending post-unlock Resched IPI to CPU 3 for task 9
[31802648757] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=0
[31804983705] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[31807138209] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=20
[31809268458] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ahci_disk' tid=7
[31812289806] [[32mINFO [0m] [sprout::supervisor] [CPU3] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[31815349698] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[31817730285] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ahci_disk' at index 57
[31820046027] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Running registration + health supervision loop
[31822014543] [[34mDEBUG[0m] [iso9660d] [CPU2] iso9660d: starting ISO9660 VFS provider
[31824504921] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/services/storage' tid=8
[31828519503] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU0] BRIDGE: handle=2 -> fd=3 node=0xffffffffb008b930 port=0xffffffffb00715b0
[31833544446] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[31836775509] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Bridged resp_port 2 -> FD 3 for task 'cambium'
[32155229799] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[32160790893] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ata_disk' tid=7
[32167195071] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ata_disk' at index 58
[32194307310] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running → Runnable (cpu=3, next=9)
[32196841116] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=0 apic_id=0 vector=0x30
[32199861210] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[32201890083] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[32386645632] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.ata' start='thingos_driver_start_safe'
[32391070899] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/chime' tid=7
[32395159170] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/chime' at index 63
[32494738485] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[32496925923] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running �� Runnable (cpu=3, next=9)
[32502419829] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[32587274280] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[32590522008] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/display_bootfb' tid=7
[32596529262] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/display_bootfb' at index 41
[32823677304] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[32825892759] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running → Runnable (cpu=3, next=9)
[32830716897] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[32844132123] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[32848231845] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/display_virtio_gpu' tid=7
[32851684470] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/display_virtio_gpu' at index 42
[33050820396] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[33054980541] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/hdaudio' tid=7
[33059264997] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/hdaudio' at index 61
[33159066963] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[33161450520] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running �� Runnable (cpu=3, next=9)
[33166692735] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[33249874383] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[33255176460] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/pci_stubd' tid=7
[33259744518] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/pci_stubd' at index 62
[33481385817] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[33486103563] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ps2_kbd' tid=7
[33490522263] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ps2_kbd' at index 3
[33515261868] [[35mTRACE[0m] [bran::arch::x86_64] [CPU2] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[33517679250] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running �� Runnable (cpu=3, next=9)
[33520072080] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/services/storage' tid=8
[33522477483] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[33704891682] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[33707209404] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ps2_mouse' tid=7
[33710324967] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ps2_mouse' at index 40
[33844086903] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[33846411654] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running ��� Runnable (cpu=3, next=9)
[33851009445] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[33902073975] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[33906427434] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/rtc_cmos' tid=7
[33910021497] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/rtc_cmos' at index 2
[34099069818] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[34103111361] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/rtl8168d' tid=7
[34107618534] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/rtl8168d' at index 45
[34172811387] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[34174389579] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running ��� Runnable (cpu=3, next=9)
[34179386076] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[34334051268] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[34339038954] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/virtio_netd' tid=7
[34344434487] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/virtio_netd' at index 44
[34573349778] [[35mTRACE[0m] [bran::arch::x86_64] [CPU2] SMP: send_ipi cpu_index=0 apic_id=0 vector=0x30
[34576532430] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[34580945487] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running �� Runnable (cpu=3, next=9)
[34582951887] [[35mTRACE[0m] [bran::arch::x86_64] [CPU2] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[34585234926] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/virtio_sound' tid=7
[34588102923] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[34591049988] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/virtio_sound' at index 60
[34792400742] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[34797106740] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[34798875903] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpus_online=4
[34800603024] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices' tid=7
[34803012453] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=0 curr=Some(0) runq=0 runq_avg=1 runq_samples=169 ctxsw=16 idle2busy=0 tick=31 ipi=0 enq=170 deq=169 wake=6 lock_miss=0 lock_pending=0 lock_blocked=0
[34807087557] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices'
[34809368682] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=1 curr=Some(7) runq=0 runq_avg=0 runq_samples=128 ctxsw=1 idle2busy=1 tick=73 ipi=1 enq=11 deq=10 wake=1 lock_miss=0 lock_pending=0 lock_blocked=0
[34814827608] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=2 curr=Some(3) runq=0 runq_avg=0 runq_samples=218 ctxsw=4 idle2busy=2 tick=70 ipi=0 enq=3 deq=2 wake=2 lock_miss=0 lock_pending=0 lock_blocked=0
[34818459753] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: readdir found 10 slots
[34822879047] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=3 curr=Some(6) runq=0 runq_avg=0 runq_samples=14837 ctxsw=19 idle2busy=1 tick=32 ipi=8 enq=14810 deq=14809 wake=9 lock_miss=0 lock_pending=0 lock_blocked=0
[34828030248] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: readdir wrote 147 bytes
[34833925962] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/vendor' tid=7
[34837546029] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:00.0/vendor'
[34842877674] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:00.0/vendor'
[34849063854] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/device' tid=7
[34852086060] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:00.0/device'
[34855153938] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:00.0/device'
[34859091234] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/class' tid=7
[34862152512] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:00.0/class'
[34864925436] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:00.0/class'
[34869554907] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/status' tid=7
[34872462603] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:00.0/status'
[34875031026] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:00.0/status'
[34878884172] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/kind' tid=7
[34881870738] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:00.0/kind'
[34885491069] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:00.0/kind'
[34890497400] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/vendor' tid=7
[34894195644] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/vendor'
[34897923720] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:01.0/vendor'
[34903355487] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/device' tid=7
[34906764519] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/device'
[34909730163] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:01.0/device'
[34912934760] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/class' tid=7
[34915560075] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/class'
[34918127244] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:01.0/class'
[34921883700] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/status' tid=7
[34925353914] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/status'
[34929918408] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:01.0/status'
[34933810197] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/kind' tid=7
[34936831875] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/kind'
[34939827087] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:01.0/kind'
[34943968587] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/vendor' tid=7
[34947373296] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/vendor'
[34950463350] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/vendor'
[34954881786] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/device' tid=7
[34958354508] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/device'
[34961986752] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/device'
[34969243518] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[34971851508] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running → Runnable (cpu=3, next=9)
[34974461610] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/class' tid=7
[34977300864] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[34979756328] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/class'
[34983476913] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/class'
[34988048601] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/status' tid=7
[34990834956] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/status'
[34993829739] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/status'
[34997841384] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/kind' tid=7
[35001630411] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/kind'
[35005090329] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/kind'
[35010501867] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.0/vendor' tid=7
[35013870738] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.0/vendor'
[35016420087] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.0/vendor'
[35021215878] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.0/device' tid=7
[35024964216] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.0/device'
[35028718659] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.0/device'
[35034704001] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.0/class' tid=7
[35038840914] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.0/class'
[35042483454] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.0/class'
[35047865820] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.0/status' tid=7
[35052028440] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.0/status'
[35055489282] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.0/status'
[35060772417] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.0/kind' tid=7
[35064625266] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.0/kind'
[35068012617] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.0/kind'
[35073961527] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.2/vendor' tid=7
[35077802694] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/vendor'
[35080359303] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/vendor'
[35083947657] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.2/device' tid=7
[35086673325] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/device'
[35089371042] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/device'
[35093640252] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.2/class' tid=7
[35097790563] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/class'
[35101090266] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/class'
[35105662284] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.2/status' tid=7
[35108392605] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/status'
[35110854471] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/status'
[35114296239] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.2/kind' tid=7
[35118768201] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/kind'
[35123094996] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/kind'
[35129259561] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.3/vendor' tid=7
[35133449934] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.3/vendor'
[35137204509] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.3/vendor'
[35142543975] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.3/device' tid=7
[35145835428] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.3/device'
[35148486252] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.3/device'
[35152231818] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.3/class' tid=7
[35155688667] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.3/class'
[35158779579] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.3/class'
[35163819768] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[35167079409] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.3/status' tid=7
[35169529857] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/services/storage' tid=8
[35172290901] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.3/status'
[35176621920] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.3/status'
[35180148399] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.3/kind' tid=7
[35184016065] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.3/kind'
[35186632866] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.3/kind'
[35190592965] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0070/vendor' tid=7
[35193627348] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/vendor'
[35197641138] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/vendor'
[35202409605] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0070/device' tid=7
[35206070988] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/device'
[35210175099] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/device'
[35215318314] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0070/class' tid=7
[35218963230] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/class'
[35222602965] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/class'
[35227799739] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0070/status' tid=7
[35231629785] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/status'
[35234488509] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/status'
[35237368122] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0070/kind' tid=7
[35240808867] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/kind'
[35243981058] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/kind'
[35247887136] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0060/vendor' tid=7
[35251209246] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/vendor'
[35254466313] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/vendor'
[35258238609] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0060/device' tid=7
[35261894019] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/device'
[35265679449] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/device'
[35270307732] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0060/class' tid=7
[35273829228] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/class'
[35277321453] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/class'
[35282292705] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0060/status' tid=7
[35285927523] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/status'
[35289388530] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/status'
[35291657478] [[35mTRACE[0m] [bran::arch::x86_64] [CPU2] SMP: send_ipi cpu_index=0 apic_id=0 vector=0x30
[35295867288] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0060/kind' tid=7
[35298281271] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/kind'
[35300923416] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/kind'
[35304177414] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-01f0/vendor' tid=7
[35306863614] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-01f0/vendor'
[35310166287] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-01f0/vendor'
[35315213637] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-01f0/device' tid=7
[35318607819] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-01f0/device'
[35322109746] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-01f0/device'
[35326586427] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[35328696216] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running → Runnable (cpu=3, next=9)
[35330473794] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-01f0/class' tid=7
[35333139864] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[35335910049] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-01f0/class'
[35339836125] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-01f0/class'
[35345143911] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-01f0/status' tid=7
[35349171990] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-01f0/status'
[35353290786] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-01f0/status'
[35359019388] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-01f0/kind' tid=7
[35362825311] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-01f0/kind'
[35367088284] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-01f0/kind'
[35377183116] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/status' tid=7
[35381341842] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/status'
[35384984349] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/status'
[35399295162] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=3 len=144
[35401653705] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=3 starting copyin
[35403932223] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=3 copyin ok
[35414553339] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/virtio_netd' at index 44
[35433867216] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'virtio_netd' (len=106272, base=0x200000)
[35435940969] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[35440023729] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[35443085238] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=200000 exec=true
[35447537004] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   COPIED at 0x201420: [e9, 0f, 86, 41, 04, 00, 00, be]
[35457542967] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=20e000 exec=false
[35462221410] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=212000 exec=false
[35465854479] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x214000 filesz=0 memsz=0 align=1
[35478227136] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x20277c + bias 0x0 = PC 0x20277c
[35481837930] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/virtio_netd
[35484967815] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb00f8360 arg=0xffffffffb008d160
[35490427137] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=10, applying inserts
[35493289821] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[35495287674] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=10
[35497032747] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[35498718354] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[35501309250] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[35507195493] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 10
[35509465761] [[35mTRACE[0m] [kernel::sched::blocking] [CPU1] WAKE_TASK: ID=10 taking SCHEDULER lock
[35516553534] [[35mTRACE[0m] [kernel::sched::blocking] [CPU1] WAKE_TASK: ID=10 wake_task_locked returned IPI_CPU=None
[35519132682] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 10 woken, restoring IRQs
[35521359522] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/virtio_netd
[35525635926] [[35mTRACE[0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=10 PID=10
[35529491976] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/virtio_netd for pci-0000:00:02.0 (entry='thingos_driver_start_safe', pid=10)
[35532963609] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.2/status' tid=7
[35536038516] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/status'
[35538554370] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/status'
[35544129027] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=4 len=144
[35546678475] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=4 starting copyin
[35549361342] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=4 copyin ok
[35556363546] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ahci_disk' at index 57
[35570978058] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'ahci_disk' (len=74072, base=0x200000)
[35573226612] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[35577844533] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[35581086519] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=200000 exec=true
[35586094137] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   COPIED at 0x201420: [89, 85, 88, 05, 00, 00, 48, c7]
[35594888670] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=209000 exec=false
[35599041918] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=20b000 exec=false
[35603866320] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x20e000 filesz=0 memsz=0 align=1
[35614669530] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[35618309958] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/ahci_disk
[35622427104] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb0110160 arg=0xffffffffb00e07c0
[35644343823] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=11, applying inserts
[35647064970] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[35649200004] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=11
[35651498157] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[35654040147] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[35657891115] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[35662472109] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[35664361887] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running → Runnable (cpu=3, next=9)
[35666562624] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 11
[35669009739] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[35670631227] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[35673267696] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 11 woken, restoring IRQs
[35675279145] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x4
[35678010885] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/ahci_disk
[35680086321] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=11 entry_pc=201000 user_sp=800000
[35682538848] [[35mTRACE[0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=11 PID=11
[35684754732] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=11 buf_len=0
[35687341899] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/ahci_disk for pci-0000:00:1f.2 (entry='thingos_driver_start_safe', pid=11)
[35689907979] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=11 buf_len=100
[35695393041] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-01f0/status' tid=7
[35699526984] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-01f0/status'
[35701319808] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver (boot_fd=4)
[35703550146] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-01f0/status'
[35716833933] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=5 len=144
[35726606850] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=5 starting copyin
[35729824977] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=5 copyin ok
[35744580531] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ata_disk' at index 58
[35760187848] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'ata_disk' (len=61712, base=0x200000)
[35763479103] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[35769006405] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[35772128601] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=200000 exec=true
[35778956301] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   COPIED at 0x201420: [00, 80, 66, 66, 66, 66, 66, 2e]
[35782716156] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 'pci-0000:00:1f.2' (handle 0)
[35792074197] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Claimed PCI device '/sys/devices/pci-0000:00:1f.2' handle=0
[35794486167] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=208000 exec=false
[35797544277] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: mapping BAR5 (phys=0x80880000, size=0x1000) for task 11
[35801136558] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=20a000 exec=false
[35804886117] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x20b000 filesz=0 memsz=0 align=1
[35807788830] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[35812479450] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[35814669198] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[35818076085] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[35821391793] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/ata_disk
[35823730932] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[35825658264] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb0124160 arg=0xffffffffb00dda80
[35829344529] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: DMA virt=0x20c000
[35831581995] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=12, applying inserts
[35833924071] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: DMA phys=0x22fa000
[35835325515] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[35837309013] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 0...
[35839818102] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=12
[35842766388] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[35848439484] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[35852665134] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[35854920519] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[35857731690] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[35860039908] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[35864209326] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[35868145863] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 1...
[35869904532] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
[35872637460] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 12
p[35874601554] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[35876874561] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[35878645176] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 12 woken, restoring IRQs
[35880739620] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 2...
[35882336259] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/ata_disk
[35884206435] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[35886292068] [[35mTRACE[0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=12 PID=12
[35890057137] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=4096 port=0xffffffffb00d04f0
[35892156201] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running → Runnable (cpu=3, next=12)
[35894568204] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/ata_disk for isa-01f0 (entry='thingos_driver_start_safe', pid=12)
[35897408085] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(1)
[35899974462] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(1) mode=Write
[35902383264] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/proc/12/job_observer' tid=7
[35904637923] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x5
[35907741474] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=4 port_id=PortId(1) mode=Read
[35910261090] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=12 entry_pc=201000 user_sp=800000
[35918222208] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=12 buf_len=0
[35943883107] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=12 buf_len=100
[35951662791] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[35954685624] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[35959404987] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x20277c SP=0x800000 ARG0=0x3
[35962017300] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=10 entry_pc=20277c user_sp=800000
[35965649049] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=0
[35969070555] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/services/storage/atapi2' tid=11
[35972493084] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=100
[35984048133] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[35994000042] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=11 fd=5 len=1
[35998395048] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=11 fd=5 starting copyin
[36000628620] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=11 fd=5 copyin ok
[36009883998] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device port=2 model='                                        ' rpc_port=3
[36015796014] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 3...
[36018282564] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[36019978401] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
[36023597313] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 4...
[36025720434] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[36027823590] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 5...
[36029575131] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[36032281395] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Starting VirtIO-NET driver service...
[36037197141] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[36038807871] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Initializing hardware driver...
[36041329632] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[36043533834] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Claiming NIC device at /sys/devices/pci-0000:00:02.0
[36046069983] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=4 -> fd=5 node=0xffffffffb008d170 port=0xffffffffb00d04f0
[36051884121] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: device::new(/sys/devices/pci-0000:00:02.0)
[36055602759] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: claiming '/sys/devices/pci-0000:00:02.0'...
[36059308593] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 10 claimed device 'pci-0000:00:02.0' (handle 1)
[36063248859] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: claimed, handle=1
[36067538562] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/virtio/common_bar' tid=10
[36072373095] [[34mDEBUG[0m] [kernel::sched] [CPU2] SCHED: CPU 2 stole task 9 (prio 2) from CPU 3 (depth 2)
[36074870898] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[36076885218] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/common_bar'
[36079313952] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[36081690843] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[36084087699] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
[36088753602] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[36091456137] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[36093557874] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/common_bar -> '4' (n=2)
[36095753331] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[36098028054] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/virtio/common_offset' tid=10
[36100417947] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[36102640365] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/common_offset'
[36106739922] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
s[36110607192] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[36113078892] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/common_offset -> '0x0' (n=4)
[36115936296] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/virtio/notify_bar' tid=10
[36119605797] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/notify_bar'
[36126498012] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/notify_bar -> '4' (n=2)
[36129722442] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/virtio/notify_offset' tid=10
[36133923210] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/notify_offset'
[36141397710] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/notify_offset -> '0x3000' (n=7)
[36144919833] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/virtio/notify_multiplier' tid=10
[36149353482] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/notify_multiplier'
[36155136864] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/notify_multiplier -> '4' (n=2)
[36158889129] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[36162149859] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/virtio/device_bar' tid=10
[36165049470] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/device_bar'
[36170156187] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/device_bar -> '4' (n=2)
[36172968084] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/virtio/device_offset' tid=10
[36175487799] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/device_offset'
[36179998140] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/device_offset -> '0x2000' (n=7)
[36183739680] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[36186309918] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[36188826762] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: mapping common BAR4...
[36190631928] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[36192713700] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: mapping BAR4 (phys=0xc000004000, size=0x4000) for task 10
[36195697989] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1

[36198176949] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x10001000
[36201975381] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: common_cfg at 0x10001000
[36204579906] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: notify_cfg at 0x10004000
[36208051473] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: device_cfg = Some(268447744)
[36209807733] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ps'
[36211292040] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: allocating DMA command buffer...
[36213824031] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/bin/ps' tid=6
[36217351269] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/ps' at index 7
[36219185904] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 1 pages phys=0x23ba000 -> user_va=0x10005000
[36224275659] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: cmd_buf virt=0x10005000 phys=0x23ba000
[36226720497] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: device::new complete
[36230512428] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/ps' at index 7
[36232858101] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Device features 0x30bf8024
[36237009402] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: MAC 52:54:00:12:34:56
[36239662602] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Link UP
[36247405953] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: Loading module 'ps' (len=50608, base=0x200000)
[36249727173] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 4 pages phys=0x23bb000 -> user_va=0x10006000
[36254105679] [[35mTRACE[0m] [kernel::task::loader] [CPU3]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[36259538766] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[36262485204] [[35mTRACE[0m] [kernel::task::loader] [CPU3] Segment: vaddr=200000 exec=true
[36265196880] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: RX queue 0 setup (size=32)
[36267378312] [[35mTRACE[0m] [kernel::task::loader] [CPU3]   COPIED at 0x201420: [29, e8, 4b, 8d, 1c, 2c, 48, 83]
[36273497106] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 4 pages phys=0x23c5000 -> user_va=0x1000a000
[36276771498] [[35mTRACE[0m] [kernel::task::loader] [CPU3] Segment: vaddr=207000 exec=false
[36279487827] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: TX queue 1 setup (size=32)
[36281532243] [[35mTRACE[0m] [kernel::task::loader] [CPU3] Segment: vaddr=208000 exec=false
[36286939854] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 16 pages phys=0x23cf000 -> user_va=0x1000e000
[36289002255] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: TLS block: tp=0x209000 filesz=0 memsz=0 align=1
[36291485901] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[36293480454] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 1 pages phys=0x23e5000 -> user_va=0x1001e000
[36297805962] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Allocated TX buffer (1 page)
[36299421114] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Starting Phase 1 for /bin/ps
[36301576575] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: RX queue filled with 32 buffers
[36303403587] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb013b160 arg=0xffffffffb00d0ec0
[36306922047] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: DRIVER_OK set, device is live
[36308628675] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Phase 1 complete, ID=13, applying inserts
[36311303391] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Driver initialized successfully
[36312957186] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[36314879898] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Driver initialized successfully
[36316271145] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=13
[36317750667] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[36319004205] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[36320532864] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: MAC 52:54:00:12:34:56
[36322102839] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[36324500355] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Initial link state is UP
[36330319806] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Phase 2 complete, waking task 13
[36332750025] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=65536 port=0xffffffffb00dff50
[36334888788] [[35mTRACE[0m] [bran::arch::x86_64] [CPU3] SMP: send_ipi cpu_index=1 apic_id=1 vector=0x30
[36337336728] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(2)
[36338841528] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Task 13 woken, restoring IRQs
[36341258415] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=3 port_id=PortId(2) mode=Write
[36343308309] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Done for /bin/ps
[36345382425] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=4 port_id=PortId(2) mode=Read
[36347112450] [[35mTRACE[0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/ps' TID=13 PID=13
[36349830528] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Created VFS provider port
[36355249458] [[34mDEBUG[0m] [sh] [CPU3] sh: spawned '/bin/ps' pid=13 idx=0 background=false pending_pgid=0
[36361544736] [[34mDEBUG[0m] [sh] [CPU3] sh: setpgid leader pid=13 -> pgid=13
[36363750819] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Task 14 (user thread) assigned to CPU 2
[36366409827] [[34mDEBUG[0m] [sh] [CPU3] sh: foreground handoff start job_id=1 pgid=13 shell_pgid=5 cmd='ps'
[36368507274] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[36370326234] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=14
[36371764704] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[36373458132] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Sending post-unlock Resched IPI to CPU 2 for task 14
[36375291645] [[34mDEBUG[0m] [sh] [CPU3] sh: foreground handoff done job_id=1 pgid=13 cmd='ps'
[36377476014] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[36380256990] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=6
[36382528611] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=14 buf_len=0
[36384952791] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[36387762477] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=14 buf_len=100
[36390181872] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[36392292321] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[36395370693] [[34mDEBUG[0m] [virtio_netd] [CPU2] VIRTIO_NETD: Entering VFS provider service loop at /dev/net/virtio0
[36397763952] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=13 entry_pc=200000 user_sp=800000
[36400411608] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=6
[36402866082] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=13 buf_len=0
[?25l[36406544889] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=13 buf_len=100
[36410740707] [[35mTRACE[0m] [kernel::sched] [CPU3] waitpid: blocking parent_pid=6 parent_tid=6 target_pid=-1 flags=0x6 registered_children=1
[36414868083] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=13 fd=1 len=25
[36417385785] [[34mDEBUG[0m] [kernel::sched::blocking] [CPU3] SCHED[TID6]: blocked
[36419140824] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=13 fd=1 starting copyin
[36421760496] [[34mDEBUG[0m] [kernel::sched] [CPU3] SCHED: CPU 3 stole task 10 (prio 2) from CPU 1 (depth 2)
[36424383765] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=13 fd=1 copyin ok
[36427503189] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU1] vfs: /dev/console write by PID=13 len=25
  PID  PPID STAT COMMAND
[36431952084] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/proc' tid=13
[36437045238] [[34mDEBUG[0m] [kernel::ipc] [CPU3] CREATE_PORT: capacity=262400 port=0xffffffffb00d33b0
[36439613958] [[34mDEBUG[0m] [kernel::ipc] [CPU3] CREATE_PORT: appended id=PortId(3)
[36443486937] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU3] ALLOC_HANDLE: handle=5 port_id=PortId(3) mode=Write
[36452134422] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] vfs: mounted userland provider at /dev/net/virtio0 (flags: 0x0)
[36455361327] [[34mDEBUG[0m] [virtio_netd] [CPU3] VIRTIO_NETD: Mounted at /dev/net/virtio0
[36456993672] [[34mDEBUG[0m] [virtio_netd] [CPU3] VIRTIO_NETD: Provider thread live at /dev/net/virtio0
[36485353839] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[36
```
</details>
