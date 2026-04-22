# ✅ Scenario: Scheduler loop entry is reached before deferred boot framebuffer paint

> Last run: 2026-04-22 08:44:15

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 0ms | - - - |
| 2 | When I wait for the system to boot | ✅ | 8117ms | - [📜](./02/serial.log) - |
| 3 | Then the serial output should contain "scheduler-entry total elapsed_ticks=" | ✅ | 0ms | - - - |
| 4 | And the serial log shows "Entering scheduler loop." after "Scheduler initialized" | ✅ | 504ms | - [📜](./04/serial.log) - |
| 5 | And the serial log shows "deferred_bootfb_gradient elapsed_ticks=" after "Entering scheduler loop." | ✅ | 503ms | - [📜](./05/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[25231384992] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] enter
[25242479460] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] phys_memory_map ok
[25246828959] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] modules ok
[25248344418] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] phys_to_virt_offset ok
[25249547235] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] memory map logging done
[25277334522] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] boot_frame_alloc init ok
[25497467193] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] frame allocator build ok
[25498825407] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] frame allocator log ok
[25500408417] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] FRAME_ALLOCATOR init ok
[25501712115] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] tasking init ok
[25506868398] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[25508206548] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] enter
[25509516549] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] set expand hook
[25510763553] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] expand hook ok
[25512114309] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock begin
[25513733520] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock ok
[25515052167] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] reserve_region begin
[25548975342] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] reserve_region ok
[25550413548] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] inner allocator init begin
[25554054669] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] inner allocator init ok
[25555758129] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] heap top store ok
[25556979261] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] init done
[25564250283] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] set_boot_fb begin
[25565641860] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] devfs: set_boot_fb width=1920 height=1080 pitch=7680 resource_id=0xfb000000
[25568155998] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] set_boot_fb ok
[25569865596] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs set_boot_fb elapsed_ticks=5167932 elapsed_us=2583
[25577058540] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs FbNode::new elapsed_ticks=5563701 elapsed_us=2781
[25578644883] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] register begin
[25583885712] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] register ok
[25585058433] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs register fb0 elapsed_ticks=6349299 elapsed_us=3174
[25590661932] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[25601246517] [[34mDEBUG[0m] [kernel] [CPU0] Seeding entropy pool...
[25610247894] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed begin
[25611811467] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] fill_entropy done
[25614717216] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] add_sample(timer) ok
[25616062725] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] mark_seeded(timer) ok
[25617156675] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[25618492152] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[25619953359] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed done
[25621749186] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] entropy::seed_from_hardware elapsed_ticks=10966956 elapsed_us=5483
[25627160460] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[25636761051] [[35mTRACE[0m] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[25638411183] [[35mTRACE[0m] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[25639629180] [[35mTRACE[0m] [kernel::sched] [CPU0]   Allocating scheduler...
[25644173742] [[35mTRACE[0m] [kernel::sched] [CPU0]   Leaking scheduler...
[25645228785] [[35mTRACE[0m] [kernel::sched] [CPU0]   Initializing boot task...
[25648356888] [[35mTRACE[0m] [kernel::sched] [CPU0]   Creating boot task...
[25662015126] [[35mTRACE[0m] [kernel::sched] [CPU0]   Creating idle tasks...
[25663892232] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80016cc0 kstack_top=0xffffffffb001f980 arg=0x0
[25666988358] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[25689080472] [[35mTRACE[0m] [kernel::sched] [CPU0]   Boot task initialized
[25690341435] [[35mTRACE[0m] [kernel::sched] [CPU0]   Storing scheduler pointer...
[25692197190] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[25697868900] [[34mDEBUG[0m] [kernel] [CPU0] Initializing VFS...
[25702001886] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='set_cmdline' elapsed_ticks=2974158 elapsed_us=1487 total_ticks=3929145 total_us=1964
[25728306912] [[34mDEBUG[0m] [kernel::vfs] [CPU0] VFS: Created /dev/display directory
[25745227035] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted union filesystem at / (root)
[25748369757] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted devfs at /dev
[25749921186] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted procfs at /proc
[25751478819] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted sysfs at /sys
[25756148154] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /tmp
[25757783700] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /run
[25759185012] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /services
[25760495904] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /session
[25761828576] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /data
[25762840125] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='vfs_init' elapsed_ticks=58743333 elapsed_us=29371 total_ticks=65040525 total_us=32520
[25768948458] [[34mDEBUG[0m] [kernel] [CPU0] Scanning PCI bus...
[25773445467] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x29c0 at 00:00.0 class=060000 id=1
[25779490407] [[35mTRACE[0m] [kernel] [CPU0]   BAR0 (MEM32): 0x80000000 (size 0x800000)
[25791844683] [[35mTRACE[0m] [kernel] [CPU0]   BAR2 (MEM64): 0xc000000000 (size 0x4000)
[25796167386] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (MEM32): 0x80882000 (size 0x1000)
[25797551703] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1050 at 00:01.0 class=030000 id=2
[25803185826] [[35mTRACE[0m] [kernel] [CPU0]   BAR0 (I/O):  0x6060 (size 0x20)
[25807618056] [[35mTRACE[0m] [kernel] [CPU0]   BAR1 (MEM32): 0x80881000 (size 0x1000)
[25819899501] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (MEM64): 0xc000004000 (size 0x4000)
[25821372588] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1000 at 00:02.0 class=020000 id=3
[25824029154] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2918 at 00:1f.0 class=060100 id=4
[25828175406] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (I/O):  0x6040 (size 0x20)
[25831182102] [[35mTRACE[0m] [kernel] [CPU0]   BAR5 (MEM32): 0x80880000 (size 0x1000)
[25832531373] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2922 at 00:1f.2 class=010601 id=5
[25836316737] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (I/O):  0x6000 (size 0x40)
[25837522722] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2930 at 00:1f.3 class=0c0500 id=6
[25840985511] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='scan_pci' elapsed_ticks=71041608 elapsed_us=35520 total_ticks=143173305 total_us=71586
[25847514396] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='register_legacy_devices' elapsed_ticks=764841 elapsed_us=382 total_ticks=149720010 total_us=74860
[25853497428] [[34mDEBUG[0m] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[25860852039] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[25861927641] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[25863528075] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[25864831509] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[25869358383] [[34mDEBUG[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[25871774280] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[25875295842] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[25877052630] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[25878425100] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[25879741404] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[25881455754] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[25883078100] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[25884421959] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[25886407371] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[25887718692] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[25889104263] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[25890826236] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[25892293284] [[34mDEBUG[0m] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[25893538044] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[25895751321] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Registers initialized
[25897178340] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[25898806131] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[25900500087] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: All pins masked
[25902640170] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> NMI
[25904235456] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[25905715671] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ4 -> GSI 4 -> 0x24
[25913115855] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Init complete
[25948106250] [[34mDEBUG[0m] [bran::arch::x86_64::ioapic] [CPU0] LAPIC: calibrated 62547500 ticks/sec (delta=625475, ok=true) -> init_cnt=625475
[25950931875] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62547500 ticks/sec), init_cnt=625475 for 100Hz
[25952875839] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='setup_preemption_timer' elapsed_ticks=98182689 elapsed_us=49091 total_ticks=255068682 total_us=127534
[25959435315] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[25960886556] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[25963034229] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[25965820452] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[25996152039] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[25997727558] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[25999274796] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] SMP: CPU 1 initializing preemption timer (vec=32 init_cnt=625475)
[26002066662] [[34mDEBUG[0m] [kernel] [CPU1] SMP: kernel_secondary_entry arg_cpu=1 runtime_cpu=1
[26003808567] [[34mDEBUG[0m] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[26005967757] [[34mDEBUG[0m] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[26010048372] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80016cc0 kstack_top=0xffffffffb0032e40 arg=0x1
[26012075892] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Task 2 assigned to CPU 1
[26016495417] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 2 deferred inserts
[26018521122] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=1
[26019819573] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=2
[26021898573] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[26023358592] [[34mDEBUG[0m] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[26024804388] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: run_scheduler entry on CPU 1
[26026107492] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[26027691987] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] SMP: CPU 2 initializing preemption timer (vec=32 init_cnt=625475)
[26029952784] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[26031385644] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: bootstrap_cpu start on CPU 1
[26032684161] [[34mDEBUG[0m] [kernel] [CPU2] SMP: kernel_secondary_entry arg_cpu=2 runtime_cpu=2
[26033974560] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle thread 2 and is now schedulable
[26035602351] [[34mDEBUG[0m] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[26037144936] [[34mDEBUG[0m] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[26039124045] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU2] INIT KERNEL CTX: entry=0xffffffff80016cc0 kstack_top=0xffffffffb00445a0 arg=0x2
[26041249971] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] SCHED: Task 3 assigned to CPU 2
[26044883700] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Applying 1 deferred inserts
[26046233697] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserting TID=3
[26047628574] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserts applied
[26048748429] [[34mDEBUG[0m] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[26050069254] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: run_scheduler entry on CPU 2
[26051252832] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: bootstrap_cpu start on CPU 2
[26058351528] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle thread 3 and is now schedulable
[26062073697] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[26064448542] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] SMP: CPU 3 initializing preemption timer (vec=32 init_cnt=625475)
[26066126988] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[26067321060] [[34mDEBUG[0m] [kernel] [CPU3] SMP: kernel_secondary_entry arg_cpu=3 runtime_cpu=3
[26069181996] [[34mDEBUG[0m] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[26071231164] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[26073406392] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='start_secondary_cpus' elapsed_ticks=114188976 elapsed_us=57094 total_ticks=375589665 total_us=187794
[26076900300] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[26079950457] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80016cc0 kstack_top=0xffffffffb0055d00 arg=0x3
[26083266396] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SCHED: Task 4 assigned to CPU 3
[26086012491] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[26087428785] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=4
[26088683973] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[26089815279] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[26090968167] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: run_scheduler entry on CPU 3
[26092109010] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: bootstrap_cpu start on CPU 3
[26093341395] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle thread 4 and is now schedulable
[26101166883] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Enumerating 108 boot modules...
[26103542190] [[35mTRACE[0m] [kernel] [CPU0]   Module[0]: name='/bin/sprout' cmdline='init' size=85144 bytes
[26105032173] [[35mTRACE[0m] [kernel] [CPU0]   Module[1]: name='/bin/bristle' cmdline='' size=45888 bytes
[26106447213] [[35mTRACE[0m] [kernel] [CPU0]   Module[2]: name='/drivers/rtc_cmos' cmdline='init' size=39744 bytes
[26107928715] [[35mTRACE[0m] [kernel] [CPU0]   Module[3]: name='/drivers/ps2_kbd' cmdline='' size=38760 bytes
[26109348573] [[35mTRACE[0m] [kernel] [CPU0]   Module[4]: name='/bin/sh' cmdline='init' size=121736 bytes
[26110725960] [[35mTRACE[0m] [kernel] [CPU0]   Module[5]: name='/bin/ls' cmdline='init' size=62192 bytes
[26112077310] [[35mTRACE[0m] [kernel] [CPU0]   Module[6]: name='/bin/ln' cmdline='init' size=47624 bytes
[26113530201] [[35mTRACE[0m] [kernel] [CPU0]   Module[7]: name='/bin/ps' cmdline='init' size=50608 bytes
[26114903562] [[35mTRACE[0m] [kernel] [CPU0]   Module[8]: name='/bin/kill' cmdline='init' size=51552 bytes
[26116373382] [[35mTRACE[0m] [kernel] [CPU0]   Module[9]: name='/bin/killall' cmdline='init' size=67712 bytes
[26117775222] [[35mTRACE[0m] [kernel] [CPU0]   Module[10]: name='/bin/cat' cmdline='init' size=46168 bytes
[26119179438] [[35mTRACE[0m] [kernel] [CPU0]   Module[11]: name='/bin/head' cmdline='init' size=56168 bytes
[26120605335] [[35mTRACE[0m] [kernel] [CPU0]   Module[12]: name='/bin/tail' cmdline='init' size=52048 bytes
[26122022751] [[35mTRACE[0m] [kernel] [CPU0]   Module[13]: name='/bin/top' cmdline='init' size=65544 bytes
[26123458944] [[35mTRACE[0m] [kernel] [CPU0]   Module[14]: name='/bin/wc' cmdline='init' size=56312 bytes
[26124823395] [[35mTRACE[0m] [kernel] [CPU0]   Module[15]: name='/bin/yes' cmdline='init' size=40480 bytes
[26126236026] [[35mTRACE[0m] [kernel] [CPU0]   Module[16]: name='/bin/which' cmdline='init' size=46408 bytes
[26127617901] [[35mTRACE[0m] [kernel] [CPU0]   Module[17]: name='/bin/cp' cmdline='init' size=46376 bytes
[26129039640] [[35mTRACE[0m] [kernel] [CPU0]   Module[18]: name='/bin/mv' cmdline='init' size=47104 bytes
[26130636642] [[35mTRACE[0m] [kernel] [CPU0]   Module[19]: name='/bin/rm' cmdline='init' size=47184 bytes
[26134187475] [[35mTRACE[0m] [kernel] [CPU0]   Module[20]: name='/bin/rmdir' cmdline='init' size=47208 bytes
[26135833746] [[35mTRACE[0m] [kernel] [CPU0]   Module[21]: name='/bin/mkdir' cmdline='init' size=47200 bytes
[26137361580] [[35mTRACE[0m] [kernel] [CPU0]   Module[22]: name='/bin/mount' cmdline='init' size=57752 bytes
[26138918883] [[35mTRACE[0m] [kernel] [CPU0]   Module[23]: name='/bin/echo' cmdline='init' size=40584 bytes
[26140426059] [[35mTRACE[0m] [kernel] [CPU0]   Module[24]: name='/bin/grep' cmdline='init' size=78216 bytes
[26141818329] [[35mTRACE[0m] [kernel] [CPU0]   Module[25]: name='/bin/pwd' cmdline='init' size=35144 bytes
[26143338474] [[35mTRACE[0m] [kernel] [CPU0]   Module[26]: name='/bin/touch' cmdline='init' size=46216 bytes
[26144748102] [[35mTRACE[0m] [kernel] [CPU0]   Module[27]: name='/bin/setshell' cmdline='init' size=47104 bytes
[26146219704] [[35mTRACE[0m] [kernel] [CPU0]   Module[28]: name='/bin/dmesg' cmdline='init' size=39392 bytes
[26147630916] [[35mTRACE[0m] [kernel] [CPU0]   Module[29]: name='/bin/stat' cmdline='init' size=46264 bytes
[26149155714] [[35mTRACE[0m] [kernel] [CPU0]   Module[30]: name='/bin/file' cmdline='init' size=51160 bytes
[26150548809] [[35mTRACE[0m] [kernel] [CPU0]   Module[31]: name='/bin/dirname' cmdline='init' size=50160 bytes
[26151971175] [[35mTRACE[0m] [kernel] [CPU0]   Module[32]: name='/bin/basename' cmdline='init' size=50168 bytes
[26153459970] [[35mTRACE[0m] [kernel] [CPU0]   Module[33]: name='/bin/sleep' cmdline='init' size=56880 bytes
[26154923091] [[35mTRACE[0m] [kernel] [CPU0]   Module[34]: name='/bin/sort' cmdline='init' size=66400 bytes
[26156383176] [[35mTRACE[0m] [kernel] [CPU0]   Module[35]: name='/bin/env' cmdline='init' size=41792 bytes
[26157777525] [[35mTRACE[0m] [kernel] [CPU0]   Module[36]: name='/bin/uname' cmdline='' size=40600 bytes
[26159188935] [[35mTRACE[0m] [kernel] [CPU0]   Module[37]: name='/bin/true' cmdline='' size=20736 bytes
[26160525633] [[35mTRACE[0m] [kernel] [CPU0]   Module[38]: name='/bin/false' cmdline='' size=20736 bytes
[26161899720] [[35mTRACE[0m] [kernel] [CPU0]   Module[39]: name='/bin/input_echo' cmdline='' size=43096 bytes
[26163461346] [[35mTRACE[0m] [kernel] [CPU0]   Module[40]: name='/drivers/ps2_mouse' cmdline='' size=39312 bytes
[26164879488] [[35mTRACE[0m] [kernel] [CPU0]   Module[41]: name='/drivers/display_bootfb' cmdline='' size=81384 bytes
[26166620370] [[35mTRACE[0m] [kernel] [CPU0]   Module[42]: name='/drivers/display_virtio_gpu' cmdline='' size=128072 bytes
[26169700689] [[35mTRACE[0m] [kernel] [CPU0]   Module[43]: name='/bin/cambium' cmdline='' size=124728 bytes
[26171942841] [[35mTRACE[0m] [kernel] [CPU0]   Module[44]: name='/drivers/virtio_netd' cmdline='' size=106272 bytes
[26174304354] [[35mTRACE[0m] [kernel] [CPU0]   Module[45]: name='/drivers/rtl8168d' cmdline='' size=57360 bytes
[26176624221] [[35mTRACE[0m] [kernel] [CPU0]   Module[46]: name='/bin/netd' cmdline='' size=10813352 bytes
[26178825981] [[35mTRACE[0m] [kernel] [CPU0]   Module[47]: name='/bin/mesocarp' cmdline='' size=119480 bytes
[26181100803] [[35mTRACE[0m] [kernel] [CPU0]   Module[48]: name='/bin/mdns' cmdline='' size=119480 bytes
[26183562999] [[35mTRACE[0m] [kernel] [CPU0]   Module[49]: name='/bin/mdnsd' cmdline='' size=119480 bytes
[26185740471] [[35mTRACE[0m] [kernel] [CPU0]   Module[50]: name='/bin/fetchd' cmdline='' size=72720 bytes
[26187763965] [[35mTRACE[0m] [kernel] [CPU0]   Module[51]: name='/bin/httpsd' cmdline='' size=526160 bytes
[26189379678] [[35mTRACE[0m] [kernel] [CPU0]   Module[52]: name='/bin/find' cmdline='' size=57960 bytes
[26190773895] [[35mTRACE[0m] [kernel] [CPU0]   Module[53]: name='/bin/ip' cmdline='' size=55528 bytes
[26192168706] [[35mTRACE[0m] [kernel] [CPU0]   Module[54]: name='/bin/iso_reader' cmdline='' size=37504 bytes
[26193594669] [[35mTRACE[0m] [kernel] [CPU0]   Module[55]: name='/bin/ping' cmdline='' size=61144 bytes
[26194967271] [[35mTRACE[0m] [kernel] [CPU0]   Module[56]: name='/bin/nslookup' cmdline='' size=56480 bytes
[26197005318] [[35mTRACE[0m] [kernel] [CPU0]   Module[57]: name='/drivers/ahci_disk' cmdline='' size=74072 bytes
[26199919449] [[35mTRACE[0m] [kernel] [CPU0]   Module[58]: name='/drivers/ata_disk' cmdline='' size=61712 bytes
[26201859783] [[35mTRACE[0m] [kernel] [CPU0]   Module[59]: name='/bin/iso9660d' cmdline='' size=80648 bytes
[26203308945] [[35mTRACE[0m] [kernel] [CPU0]   Module[60]: name='/drivers/virtio_sound' cmdline='' size=95232 bytes
[26204778699] [[35mTRACE[0m] [kernel] [CPU0]   Module[61]: name='/drivers/hdaudio' cmdline='' size=67600 bytes
[26206661019] [[35mTRACE[0m] [kernel] [CPU0]   Module[62]: name='/drivers/pci_stubd' cmdline='' size=58224 bytes
[26208121566] [[35mTRACE[0m] [kernel] [CPU0]   Module[63]: name='/drivers/chime' cmdline='' size=55496 bytes
[26209508358] [[35mTRACE[0m] [kernel] [CPU0]   Module[64]: name='/bin/vfs_hello' cmdline='' size=35552 bytes
[26210894589] [[35mTRACE[0m] [kernel] [CPU0]   Module[65]: name='/bin/show_args' cmdline='' size=42568 bytes
[26212302435] [[35mTRACE[0m] [kernel] [CPU0]   Module[66]: name='/bin/env_roundtrip' cmdline='' size=44216 bytes
[26213727903] [[35mTRACE[0m] [kernel] [CPU0]   Module[67]: name='/bin/cwd_test' cmdline='' size=43776 bytes
[26215209702] [[35mTRACE[0m] [kernel] [CPU0]   Module[68]: name='/bin/date' cmdline='' size=62992 bytes
[26216551581] [[35mTRACE[0m] [kernel] [CPU0]   Module[69]: name='/bin/wayland_hello' cmdline='' size=65952 bytes
[26217996420] [[35mTRACE[0m] [kernel] [CPU0]   Module[70]: name='/bin/terminal' cmdline='' size=85376 bytes
[26219388459] [[35mTRACE[0m] [kernel] [CPU0]   Module[71]: name='/bin/tee' cmdline='' size=45032 bytes
[26220724695] [[35mTRACE[0m] [kernel] [CPU0]   Module[72]: name='/bin/xargs' cmdline='' size=55080 bytes
[26222087430] [[35mTRACE[0m] [kernel] [CPU0]   Module[73]: name='/bin/placed' cmdline='' size=38136 bytes
[26223447492] [[35mTRACE[0m] [kernel] [CPU0]   Module[74]: name='/bin/bloom' cmdline='' size=112872 bytes
[26224826793] [[35mTRACE[0m] [kernel] [CPU0]   Module[75]: name='/bin/clear' cmdline='' size=20864 bytes
[26226276912] [[35mTRACE[0m] [kernel] [CPU0]   Module[76]: name='/bin/loglevel' cmdline='' size=41600 bytes
[26227664067] [[35mTRACE[0m] [kernel] [CPU0]   Module[77]: name='/bin/poll_mux' cmdline='' size=39056 bytes
[26230783722] [[35mTRACE[0m] [kernel] [CPU0]   Module[78]: name='/bin/ipc_service_demo' cmdline='' size=47552 bytes
[26233124445] [[35mTRACE[0m] [kernel] [CPU0]   Module[79]: name='/bin/ipc_pipe_demo' cmdline='' size=44016 bytes
[26235291918] [[35mTRACE[0m] [kernel] [CPU0]   Module[80]: name='/bin/ipc_provider_demo' cmdline='' size=67008 bytes
[26237063721] [[35mTRACE[0m] [kernel] [CPU0]   Module[81]: name='/bin/ipc_memfd_demo' cmdline='' size=34216 bytes
[26238536115] [[35mTRACE[0m] [kernel] [CPU0]   Module[82]: name='/bin/test_exec' cmdline='' size=62504 bytes
[26240183871] [[35mTRACE[0m] [kernel] [CPU0]   Module[83]: name='/bin/test_vm_protect' cmdline='' size=34216 bytes
[26241909342] [[35mTRACE[0m] [kernel] [CPU0]   Module[84]: name='/bin/test_exec_env' cmdline='' size=54864 bytes
[26243317584] [[35mTRACE[0m] [kernel] [CPU0]   Module[85]: name='/bin/test_threads' cmdline='' size=78336 bytes
[26244737541] [[35mTRACE[0m] [kernel] [CPU0]   Module[86]: name='/bin/test_futex' cmdline='' size=48952 bytes
[26246119482] [[35mTRACE[0m] [kernel] [CPU0]   Module[87]: name='/bin/ld_so' cmdline='' size=378656 bytes
[26247712821] [[35mTRACE[0m] [kernel] [CPU0]   Module[88]: name='/bin/test_dyn_loader' cmdline='' size=53728 bytes
[26250052851] [[35mTRACE[0m] [kernel] [CPU0]   Module[89]: name='/bin/test_dlopen' cmdline='' size=60232 bytes
[26251717239] [[35mTRACE[0m] [kernel] [CPU0]   Module[90]: name='/bin/reboot' cmdline='' size=32136 bytes
[26253157920] [[35mTRACE[0m] [kernel] [CPU0]   Module[91]: name='/bin/shutdown' cmdline='' size=32136 bytes
[26254568010] [[35mTRACE[0m] [kernel] [CPU0]   Module[92]: name='/bin/attr_list' cmdline='' size=47456 bytes
[26257101882] [[35mTRACE[0m] [kernel] [CPU0]   Module[93]: name='/bin/attr_get' cmdline='' size=47552 bytes
[26259537678] [[35mTRACE[0m] [kernel] [CPU0]   Module[94]: name='/bin/attr_set' cmdline='' size=69656 bytes
[26262105408] [[35mTRACE[0m] [kernel] [CPU0]   Module[95]: name='/bin/attr_rm' cmdline='' size=47072 bytes
[26264886054] [[35mTRACE[0m] [kernel] [CPU0]   Module[96]: name='/lib/libpistil.so' cmdline='' size=67704 bytes
[26266975218] [[35mTRACE[0m] [kernel] [CPU0]   Module[97]: name='/share/wallpapers/clouds.bmp' cmdline='' size=786486 bytes
[26268528858] [[35mTRACE[0m] [kernel] [CPU0]   Module[98]: name='/share/wallpapers/flower.bmp' cmdline='' size=8386614 bytes
[26270090682] [[35mTRACE[0m] [kernel] [CPU0]   Module[99]: name='/share/wallpapers/leather.bmp' cmdline='' size=1179702 bytes
[26272027023] [[35mTRACE[0m] [kernel] [CPU0]   Module[100]: name='/share/wallpapers/linen.bmp' cmdline='' size=4718646 bytes
[26273794998] [[35mTRACE[0m] [kernel] [CPU0]   Module[101]: name='/share/fonts/NotoSans-Regular.ttf' cmdline='' size=569208 bytes
[26275464171] [[35mTRACE[0m] [kernel] [CPU0]   Module[102]: name='/share/fonts/unifont.hex' cmdline='' size=8355852 bytes
[26276947851] [[35mTRACE[0m] [kernel] [CPU0]   Module[103]: name='/etc/locale.conf' cmdline='' size=85 bytes
[26278445589] [[35mTRACE[0m] [kernel] [CPU0]   Module[104]: name='/etc/profile' cmdline='' size=68 bytes
[26279818884] [[35mTRACE[0m] [kernel] [CPU0]   Module[105]: name='/etc/motd' cmdline='' size=610 bytes
[26281359984] [[35mTRACE[0m] [kernel] [CPU0]   Module[106]: name='/etc/fstab' cmdline='' size=123 bytes
[26283227487] [[35mTRACE[0m] [kernel] [CPU0]   Module[107]: name='/etc/hostname' cmdline='' size=8 bytes
[26293459368] [[34mDEBUG[0m] [kernel] [CPU0] Found init module: /bin/sprout (cmdline: 'init'), loading...
[26296081020] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/sprout' (len=85144, base=0x200000)
[26298881136] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[26309992566] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[26312579601] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[26317765914] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [00, 00, 00, e8, e0, 7b, 00, 00]
[26324726406] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20c000 exec=false
[26327027331] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20e000 exec=false
[26333127117] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20f000 filesz=0 memsz=0 align=1
[26341891686] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='load_init_module' elapsed_ticks=46055658 elapsed_us=23027 total_ticks=644079051 total_us=322039
[26346657975] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='prepare_boot_registry' elapsed_ticks=1247532 elapsed_us=623 total_ticks=648856230 total_us=324428
[26350347639] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='map_boot_registry' elapsed_ticks=187341 elapsed_us=93 total_ticks=652551042 total_us=326275
[26354069907] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='activate_init_aspace' elapsed_ticks=162921 elapsed_us=81 total_ticks=656219817 total_us=328109
[26357724558] [[34mDEBUG[0m] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[26359668027] [[34mDEBUG[0m] [kernel] [CPU0] Spawning init process...
[26363458638] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb00661e0 arg=0xffffffffb001fd20
[26382055755] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[26383487262] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=5
[26384720736] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[26385875043] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='spawn_init_task' elapsed_ticks=26263446 elapsed_us=13131 total_ticks=688068447 total_us=344034
[26427205530] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED: early-boot bringup complete; resuming steady-state SMP scheduling
[26430277269] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='end_bringup' elapsed_ticks=3893670 elapsed_us=1946 total_ticks=732460212 total_us=366230
[26474023026] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=775953288 elapsed_us=387976
[26475687315] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[26492250972] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[26494367163] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] ENTER_USER: TID=5 entry_pc=200000 user_sp=800000
[26537528787] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpus_online=4
[26551222995] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=0 curr=Some(5) runq=1 runq_avg=2 runq_samples=1 ctxsw=1 idle2busy=0 tick=1 ipi=0 enq=3 deq=1 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[26556343506] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=1 curr=Some(2) runq=0 runq_avg=0 runq_samples=17 ctxsw=0 idle2busy=0 tick=10 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[26562644493] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=2 curr=Some(3) runq=0 runq_avg=0 runq_samples=17 ctxsw=0 idle2busy=0 tick=14 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[26569009401] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=3 curr=Some(4) runq=0 runq_avg=0 runq_samples=16 ctxsw=0 idle2busy=0 tick=11 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[26580595338] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=0
[26587237710] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=20
[26603432262] [[34mDEBUG[0m] [sprout] [CPU0] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7fef70 rip=0x20414c rflags=0x202
[26605838457] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[26608064472] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: About to create Supervisor...
[26609331936] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Listing /bin directory...
[26613256296] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/bin' tid=5
[26685114423] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Parsed command line: '    '
[26687319153] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: FORCING display=bootfb for diagnostic test!
[26689042743] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Supervisor created, calling run_forever...
[26690844180] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[26692326078] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[26694744351] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[26696264727] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/run/sprout/shell' tid=5
[26704071966] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/etc/default/shell' tid=5
[26708697576] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/bin/sh' tid=5
[26710904979] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU0] BootFs: EXACT match for 'bin/sh' at index 4
[26715804258] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[26721156825] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[26724626016] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[26737349529] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU0] BootFs: EXACT match for 'bin/sh' at index 4
[26757325485] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'sh' (len=121736, base=0x200000)
[26759314560] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[26762795631] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[26764448634] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[26767341018] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [9c, b9, 00, 00, 48, 8d, 7c, 24]
[26777369982] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[26779826040] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=216000 exec=false
[26781807459] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x217000 filesz=0 memsz=0 align=1
[26787791976] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /bin/sh
[26790090723] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb009d500 arg=0xffffffffb006f660
[26796863247] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=6, applying inserts
[26799939045] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[26801983824] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=6
[26803822914] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[26804928414] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[26811830991] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SETUP_STDIO: stdin=Fd(3) stdout=Fd(4) stderr=Fd(5)
[26827044783] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 6
[26831228424] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[26833425069] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Task 6 woken, restoring IRQs
[26835619569] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Done for /bin/sh
[26841192180] [[35mTRACE[0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[26845484589] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[26847239694] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=6 entry_pc=200000 user_sp=800000
[26848670673] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[26849812407] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=0
[26852105973] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=100
[26853440922] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 250ms so the prompt can take the foreground
[26855134185] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=19
[26858180019] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[26860054848] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[26865467871] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=19
DEBUG: sh starting
[26877646818] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=33
[26879399019] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[26881149339] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[26882845176] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=33
DEBUG: sh sig handlers installed
[26885282259] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/run/motd' tid=6
[26892689208] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/etc/motd' tid=6
[26896886940] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'etc/motd' at index 105
[26900966433] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=610
[26902674975] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[26904405825] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[26906325996] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=610
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
[26914699779] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=23
[26916410697] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[26918144451] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[26920003275] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=23
DEBUG: sh motd printed
[26924824410] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=31
[26926713297] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[26928627396] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[26930512653] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=31
DEBUG: sh shell object created
[26932968744] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/etc/profile' tid=6
[26936707215] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'etc/profile' at index 104
[26972173536] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=25
[26974838385] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[26976795285] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[26978864781] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=25
DEBUG: sh profile loaded
[26991837576] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=97
[26994350691] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[26995973697] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[26997659172] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=97
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [27005138292] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=6
[27006781428] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[27008312595] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[27009919497] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=6
[?25h[27012488085] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[28402473429] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=1536391263
[29228406207] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[29230199757] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[29235193284] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=4096 port=0xffffffffb0071470
[29237899416] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(0)
[29243805492] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=1 port_id=PortId(0) mode=Write
[29246237328] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=2 port_id=PortId(0) mode=Read
[29252185842] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/cambium' (len=124728, base=0x200000)
[29254310646] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[29257409907] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[29259075186] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[29262439173] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [8d, 3c, f0, 48, 83, c7, 70, 48]
[29275377615] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=214000 exec=false
[29277648642] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=216000 exec=false
[29279785689] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x217000 filesz=0 memsz=0 align=1
[29287304871] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb0081820 arg=0xffffffffb0071600
[29291183790] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[29292480261] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=7
[29293947474] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[29302823484] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 7 → task '/bin/cambium' (pid=7 from boot module)
[29304823614] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=1 apic_id=1 vector=0x30
[29307901557] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned cambium (PID=7)
[29310460146] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[29311734375] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[29314088331] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=7 entry_pc=200000 user_sp=800000
[29316002760] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/iso9660d' (len=80648, base=0x200000)
[29318111427] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=0
[29320293189] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[29322850194] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=100
[29325186759] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[29327435478] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[29332714686] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [48, 89, 44, 24, 50, 48, 8d, 05]
[29343825390] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20b000 exec=false
[29347845780] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20d000 exec=false
[29351622399] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20e000 filesz=0 memsz=0 align=1
[29361269751] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb00b6440 arg=0xffffffffb0071600
[29366108838] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[29368318254] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[29370149424] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers' tid=7
[29372350920] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=8
[29374540833] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[29382263328] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 8 → task '/bin/iso9660d' (pid=8 from boot module)
[29385251412] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[29389089114] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned iso9660d (PID=8)
[29390765118] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[29394042447] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=8 entry_pc=200000 user_sp=800000
[29396935821] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=0
[29399104779] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Deferring netd until /dev/net/virtio0/rx is ready...
[29401081776] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=100
[29421690573] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Task 9 (user thread) assigned to CPU 3
[29427343506] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[29428927638] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running → Runnable (cpu=3, next=9)
[29430575361] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=9
[29432023038] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[29434331124] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Sending post-unlock Resched IPI to CPU 3 for task 9
[29436826122] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[29439520473] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Running registration + health supervision loop
[29442830076] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ahci_disk' tid=7
[29446702131] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU0] BRIDGE: handle=2 -> fd=3 node=0xffffffffb0071610 port=0xffffffffb0071470
[29449344375] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ahci_disk' at index 57
[29453225571] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Bridged resp_port 2 -> FD 3 for task 'cambium'
[29458203852] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=0
[29462145768] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=20
[29466709767] [[32mINFO [0m] [sprout::supervisor] [CPU3] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[29469726165] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[29487621075] [[34mDEBUG[0m] [iso9660d] [CPU2] iso9660d: starting ISO9660 VFS provider
[29490942591] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/services/storage' tid=8
[29503292247] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[297534832
```
</details>
