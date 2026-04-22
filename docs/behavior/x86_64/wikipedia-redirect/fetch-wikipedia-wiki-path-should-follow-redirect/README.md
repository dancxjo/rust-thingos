# ❌ Scenario: Fetch Wikipedia /wiki path should follow redirect

> Last run: 2026-04-22 08:46:14

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8012ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 2105ms | - [📜](./02/serial.log) - |
| 3 | And I type "mount -t https en.wikipedia.org /https/wp" on the serial console | ✅ | 3168ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "mounted type=https" | ❌ | 301027ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[24511102077] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] enter
[24522203838] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] phys_memory_map ok
[24528573927] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] modules ok
[24530155683] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] phys_to_virt_offset ok
[24531315303] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] memory map logging done
[24558037746] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] boot_frame_alloc init ok
[24792951315] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] frame allocator build ok
[24794409486] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] frame allocator log ok
[24795934218] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] FRAME_ALLOCATOR init ok
[24797308701] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] tasking init ok
[24802576656] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[24803859003] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] enter
[24805013904] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] set expand hook
[24806164746] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] expand hook ok
[24807348885] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock begin
[24809131974] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock ok
[24810553482] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] reserve_region begin
[24849269577] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] reserve_region ok
[24850768536] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] inner allocator init begin
[24854782062] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] inner allocator init ok
[24856265346] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] heap top store ok
[24857532315] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] init done
[24864744795] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] set_boot_fb begin
[24866297511] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] devfs: set_boot_fb width=1920 height=1080 pitch=7680 resource_id=0xfb000000
[24869235006] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] set_boot_fb ok
[24872068419] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs set_boot_fb elapsed_ticks=6503673 elapsed_us=3251
[24880995942] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs FbNode::new elapsed_ticks=7173540 elapsed_us=3586
[24882682407] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] register begin
[24887847963] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] register ok
[24889058073] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs register fb0 elapsed_ticks=6302439 elapsed_us=3151
[24894550032] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[24904685190] [[34mDEBUG[0m] [kernel] [CPU0] Seeding entropy pool...
[24914261295] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed begin
[24915757746] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] fill_entropy done
[24918472458] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] add_sample(timer) ok
[24919626138] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] mark_seeded(timer) ok
[24920680950] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[24921947424] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[24923256831] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed done
[24925447305] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] entropy::seed_from_hardware elapsed_ticks=10418166 elapsed_us=5209
[24931382025] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[24940937901] [[35mTRACE[0m] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[24943063233] [[35mTRACE[0m] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[24944430918] [[35mTRACE[0m] [kernel::sched] [CPU0]   Allocating scheduler...
[24950213112] [[35mTRACE[0m] [kernel::sched] [CPU0]   Leaking scheduler...
[24951480015] [[35mTRACE[0m] [kernel::sched] [CPU0]   Initializing boot task...
[24955002732] [[35mTRACE[0m] [kernel::sched] [CPU0]   Creating boot task...
[24968889264] [[35mTRACE[0m] [kernel::sched] [CPU0]   Creating idle tasks...
[24971000901] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80016cc0 kstack_top=0xffffffffb001f980 arg=0x0
[24974222163] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[24998309952] [[35mTRACE[0m] [kernel::sched] [CPU0]   Boot task initialized
[24999603552] [[35mTRACE[0m] [kernel::sched] [CPU0]   Storing scheduler pointer...
[25001462277] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[25007783196] [[34mDEBUG[0m] [kernel] [CPU0] Initializing VFS...
[25012535724] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='set_cmdline' elapsed_ticks=3538293 elapsed_us=1769 total_ticks=4582182 total_us=2291
[25035785742] [[34mDEBUG[0m] [kernel::vfs] [CPU0] VFS: Created /dev/display directory
[25054619865] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted union filesystem at / (root)
[25057841589] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted devfs at /dev
[25059500334] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted procfs at /proc
[25061973519] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted sysfs at /sys
[25067528112] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /tmp
[25069586949] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /run
[25071546357] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /services
[25073584734] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /session
[25075645056] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /data
[25077362970] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='vfs_init' elapsed_ticks=62489328 elapsed_us=31244 total_ticks=69684021 total_us=34842
[25084535256] [[34mDEBUG[0m] [kernel] [CPU0] Scanning PCI bus...
[25089633426] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x29c0 at 00:00.0 class=060000 id=1
[25097972163] [[35mTRACE[0m] [kernel] [CPU0]   BAR0 (MEM32): 0x80000000 (size 0x800000)
[25112094711] [[35mTRACE[0m] [kernel] [CPU0]   BAR2 (MEM64): 0xc000000000 (size 0x4000)
[25117270497] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (MEM32): 0x80882000 (size 0x1000)
[25119497601] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1050 at 00:01.0 class=030000 id=2
[25125403644] [[35mTRACE[0m] [kernel] [CPU0]   BAR0 (I/O):  0x6060 (size 0x20)
[25130063970] [[35mTRACE[0m] [kernel] [CPU0]   BAR1 (MEM32): 0x80881000 (size 0x1000)
[25141239915] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (MEM64): 0xc000004000 (size 0x4000)
[25143236448] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1000 at 00:02.0 class=020000 id=3
[25146267201] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2918 at 00:1f.0 class=060100 id=4
[25150426389] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (I/O):  0x6040 (size 0x20)
[25154453346] [[35mTRACE[0m] [kernel] [CPU0]   BAR5 (MEM32): 0x80880000 (size 0x1000)
[25156713681] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2922 at 00:1f.2 class=010601 id=5
[25161687375] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (I/O):  0x6000 (size 0x40)
[25163093274] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2930 at 00:1f.3 class=0c0500 id=6
[25166548935] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='scan_pci' elapsed_ticks=80486043 elapsed_us=40243 total_ticks=158863320 total_us=79431
[25173522330] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='register_legacy_devices' elapsed_ticks=866745 elapsed_us=433 total_ticks=165834372 total_us=82917
[25179492459] [[34mDEBUG[0m] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[25187457075] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[25188567096] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[25190240658] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[25191586794] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[25196170065] [[34mDEBUG[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[25199353608] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[25201626615] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[25203021096] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[25204385943] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[25205688948] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[25207401087] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[25209083790] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[25210372572] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[25211828565] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[25213180344] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[25214551494] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[25216245780] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[25217773152] [[34mDEBUG[0m] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[25219179381] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[25221342333] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Registers initialized
[25222888086] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[25224744468] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[25226679456] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: All pins masked
[25229102745] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> NMI
[25230565206] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[25231972029] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ4 -> GSI 4 -> 0x24
[25233017535] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Init complete
[25268964798] [[34mDEBUG[0m] [bran::arch::x86_64::ioapic] [CPU0] LAPIC: calibrated 64541800 ticks/sec (delta=645418, ok=true) -> init_cnt=645418
[25275725541] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (64541800 ticks/sec), init_cnt=645418 for 100Hz
[25277618916] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='setup_preemption_timer' elapsed_ticks=96880113 elapsed_us=48440 total_ticks=269938218 total_us=134969
[25285147866] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[25287009561] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[25290146343] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[25294452051] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[25325627547] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[25327458486] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[25329649719] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] SMP: CPU 1 initializing preemption timer (vec=32 init_cnt=645418)
[25334722314] [[34mDEBUG[0m] [kernel] [CPU1] SMP: kernel_secondary_entry arg_cpu=1 runtime_cpu=1
[25337583018] [[34mDEBUG[0m] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[25340257140] [[34mDEBUG[0m] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[25345134012] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80016cc0 kstack_top=0xffffffffb0032e40 arg=0x1
[25347902052] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Task 2 assigned to CPU 1
[25354515945] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 2 deferred inserts
[25356732390] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[25358738823] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] SMP: CPU 2 initializing preemption timer (vec=32 init_cnt=645418)
[25361160297] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=1
[25362859995] [[34mDEBUG[0m] [kernel] [CPU2] SMP: kernel_secondary_entry arg_cpu=2 runtime_cpu=2
[25365230781] [[34mDEBUG[0m] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[25367120526] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[25368660009] [[34mDEBUG[0m] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[25370970537] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=2
[25372816359] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU2] INIT KERNEL CTX: entry=0xffffffff80016cc0 kstack_top=0xffffffffb00445a0 arg=0x2
[25375449726] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[25376890242] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] SCHED: Task 3 assigned to CPU 2
[25378466520] [[34mDEBUG[0m] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[25379863476] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Applying 1 deferred inserts
[25381370025] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: run_scheduler entry on CPU 1
[25382778531] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserting TID=3
[25383945774] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: bootstrap_cpu start on CPU 1
[25385276070] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserts applied
[25386518454] [[34mDEBUG[0m] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[25388510730] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle thread 2 and is now schedulable
[25391188350] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: run_scheduler entry on CPU 2
[25393926492] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: bootstrap_cpu start on CPU 2
[25407911364] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[25409585289] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] SMP: CPU 3 initializing preemption timer (vec=32 init_cnt=645418)
[25412336730] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[25414592940] [[34mDEBUG[0m] [kernel] [CPU3] SMP: kernel_secondary_entry arg_cpu=3 runtime_cpu=3
[25419471297] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[25421453475] [[34mDEBUG[0m] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[25423481688] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='start_secondary_cpus' elapsed_ticks=137704677 elapsed_us=68852 total_ticks=414931143 total_us=207465
[25432987767] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[25437081681] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80016cc0 kstack_top=0xffffffffb0055d00 arg=0x3
[25443424710] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SCHED: Task 4 assigned to CPU 3
[25447402332] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[25451960094] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle thread 3 and is now schedulable
[25454714934] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=4
[25457059947] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[25459359354] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[25461412911] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: run_scheduler entry on CPU 3
[25464360207] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Enumerating 108 boot modules...
[25466921403] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: bootstrap_cpu start on CPU 3
[25469062014] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle thread 4 and is now schedulable
[25473348483] [[35mTRACE[0m] [kernel] [CPU0]   Module[0]: name='/bin/sprout' cmdline='init' size=85144 bytes
[25476615747] [[35mTRACE[0m] [kernel] [CPU0]   Module[1]: name='/bin/bristle' cmdline='' size=45888 bytes
[25478825757] [[35mTRACE[0m] [kernel] [CPU0]   Module[2]: name='/drivers/rtc_cmos' cmdline='init' size=39744 bytes
[25481193903] [[35mTRACE[0m] [kernel] [CPU0]   Module[3]: name='/drivers/ps2_kbd' cmdline='' size=38760 bytes
[25482953133] [[35mTRACE[0m] [kernel] [CPU0]   Module[4]: name='/bin/sh' cmdline='init' size=121736 bytes
[25484357052] [[35mTRACE[0m] [kernel] [CPU0]   Module[5]: name='/bin/ls' cmdline='init' size=62192 bytes
[25486349097] [[35mTRACE[0m] [kernel] [CPU0]   Module[6]: name='/bin/ln' cmdline='init' size=47624 bytes
[25488508320] [[35mTRACE[0m] [kernel] [CPU0]   Module[7]: name='/bin/ps' cmdline='init' size=50608 bytes
[25490656983] [[35mTRACE[0m] [kernel] [CPU0]   Module[8]: name='/bin/kill' cmdline='init' size=51552 bytes
[25492821255] [[35mTRACE[0m] [kernel] [CPU0]   Module[9]: name='/bin/killall' cmdline='init' size=67712 bytes
[25495150362] [[35mTRACE[0m] [kernel] [CPU0]   Module[10]: name='/bin/cat' cmdline='init' size=46168 bytes
[25497351990] [[35mTRACE[0m] [kernel] [CPU0]   Module[11]: name='/bin/head' cmdline='init' size=56168 bytes
[25499979879] [[35mTRACE[0m] [kernel] [CPU0]   Module[12]: name='/bin/tail' cmdline='init' size=52048 bytes
[25503540678] [[35mTRACE[0m] [kernel] [CPU0]   Module[13]: name='/bin/top' cmdline='init' size=65544 bytes
[25506007527] [[35mTRACE[0m] [kernel] [CPU0]   Module[14]: name='/bin/wc' cmdline='init' size=56312 bytes
[25509257565] [[35mTRACE[0m] [kernel] [CPU0]   Module[15]: name='/bin/yes' cmdline='init' size=40480 bytes
[25510954128] [[35mTRACE[0m] [kernel] [CPU0]   Module[16]: name='/bin/which' cmdline='init' size=46408 bytes
[25513325409] [[35mTRACE[0m] [kernel] [CPU0]   Module[17]: name='/bin/cp' cmdline='init' size=46376 bytes
[25514739591] [[35mTRACE[0m] [kernel] [CPU0]   Module[18]: name='/bin/mv' cmdline='init' size=47104 bytes
[25516381539] [[35mTRACE[0m] [kernel] [CPU0]   Module[19]: name='/bin/rm' cmdline='init' size=47184 bytes
[25517769816] [[35mTRACE[0m] [kernel] [CPU0]   Module[20]: name='/bin/rmdir' cmdline='init' size=47208 bytes
[25519358238] [[35mTRACE[0m] [kernel] [CPU0]   Module[21]: name='/bin/mkdir' cmdline='init' size=47200 bytes
[25520801163] [[35mTRACE[0m] [kernel] [CPU0]   Module[22]: name='/bin/mount' cmdline='init' size=57752 bytes
[25522222044] [[35mTRACE[0m] [kernel] [CPU0]   Module[23]: name='/bin/echo' cmdline='init' size=40584 bytes
[25523644443] [[35mTRACE[0m] [kernel] [CPU0]   Module[24]: name='/bin/grep' cmdline='init' size=78216 bytes
[25525122084] [[35mTRACE[0m] [kernel] [CPU0]   Module[25]: name='/bin/pwd' cmdline='init' size=35144 bytes
[25526713509] [[35mTRACE[0m] [kernel] [CPU0]   Module[26]: name='/bin/touch' cmdline='init' size=46216 bytes
[25528270581] [[35mTRACE[0m] [kernel] [CPU0]   Module[27]: name='/bin/setshell' cmdline='init' size=47104 bytes
[25530355323] [[35mTRACE[0m] [kernel] [CPU0]   Module[28]: name='/bin/dmesg' cmdline='init' size=39392 bytes
[25531933383] [[35mTRACE[0m] [kernel] [CPU0]   Module[29]: name='/bin/stat' cmdline='init' size=46264 bytes
[25533539427] [[35mTRACE[0m] [kernel] [CPU0]   Module[30]: name='/bin/file' cmdline='init' size=51160 bytes
[25535589750] [[35mTRACE[0m] [kernel] [CPU0]   Module[31]: name='/bin/dirname' cmdline='init' size=50160 bytes
[25537022049] [[35mTRACE[0m] [kernel] [CPU0]   Module[32]: name='/bin/basename' cmdline='init' size=50168 bytes
[25538452698] [[35mTRACE[0m] [kernel] [CPU0]   Module[33]: name='/bin/sleep' cmdline='init' size=56880 bytes
[25540792266] [[35mTRACE[0m] [kernel] [CPU0]   Module[34]: name='/bin/sort' cmdline='init' size=66400 bytes
[25542178695] [[35mTRACE[0m] [kernel] [CPU0]   Module[35]: name='/bin/env' cmdline='init' size=41792 bytes
[25543542618] [[35mTRACE[0m] [kernel] [CPU0]   Module[36]: name='/bin/uname' cmdline='' size=40600 bytes
[25544909841] [[35mTRACE[0m] [kernel] [CPU0]   Module[37]: name='/bin/true' cmdline='' size=20736 bytes
[25546270893] [[35mTRACE[0m] [kernel] [CPU0]   Module[38]: name='/bin/false' cmdline='' size=20736 bytes
[25547782788] [[35mTRACE[0m] [kernel] [CPU0]   Module[39]: name='/bin/input_echo' cmdline='' size=43096 bytes
[25549278843] [[35mTRACE[0m] [kernel] [CPU0]   Module[40]: name='/drivers/ps2_mouse' cmdline='' size=39312 bytes
[25551160437] [[35mTRACE[0m] [kernel] [CPU0]   Module[41]: name='/drivers/display_bootfb' cmdline='' size=81384 bytes
[25552595904] [[35mTRACE[0m] [kernel] [CPU0]   Module[42]: name='/drivers/display_virtio_gpu' cmdline='' size=128072 bytes
[25554989097] [[35mTRACE[0m] [kernel] [CPU0]   Module[43]: name='/bin/cambium' cmdline='' size=124728 bytes
[25557451524] [[35mTRACE[0m] [kernel] [CPU0]   Module[44]: name='/drivers/virtio_netd' cmdline='' size=106272 bytes
[25559904414] [[35mTRACE[0m] [kernel] [CPU0]   Module[45]: name='/drivers/rtl8168d' cmdline='' size=57360 bytes
[25561846167] [[35mTRACE[0m] [kernel] [CPU0]   Module[46]: name='/bin/netd' cmdline='' size=10813352 bytes
[25563209298] [[35mTRACE[0m] [kernel] [CPU0]   Module[47]: name='/bin/mesocarp' cmdline='' size=119480 bytes
[25564741620] [[35mTRACE[0m] [kernel] [CPU0]   Module[48]: name='/bin/mdns' cmdline='' size=119480 bytes
[25566523620] [[35mTRACE[0m] [kernel] [CPU0]   Module[49]: name='/bin/mdnsd' cmdline='' size=119480 bytes
[25568539821] [[35mTRACE[0m] [kernel] [CPU0]   Module[50]: name='/bin/fetchd' cmdline='' size=72720 bytes
[25569896319] [[35mTRACE[0m] [kernel] [CPU0]   Module[51]: name='/bin/httpsd' cmdline='' size=526160 bytes
[25571575095] [[35mTRACE[0m] [kernel] [CPU0]   Module[52]: name='/bin/find' cmdline='' size=57960 bytes
[25573029669] [[35mTRACE[0m] [kernel] [CPU0]   Module[53]: name='/bin/ip' cmdline='' size=55528 bytes
[25575130350] [[35mTRACE[0m] [kernel] [CPU0]   Module[54]: name='/bin/iso_reader' cmdline='' size=37504 bytes
[25576515756] [[35mTRACE[0m] [kernel] [CPU0]   Module[55]: name='/bin/ping' cmdline='' size=61144 bytes
[25577923998] [[35mTRACE[0m] [kernel] [CPU0]   Module[56]: name='/bin/nslookup' cmdline='' size=56480 bytes
[25579301682] [[35mTRACE[0m] [kernel] [CPU0]   Module[57]: name='/drivers/ahci_disk' cmdline='' size=74072 bytes
[25580916009] [[35mTRACE[0m] [kernel] [CPU0]   Module[58]: name='/drivers/ata_disk' cmdline='' size=61712 bytes
[25582907889] [[35mTRACE[0m] [kernel] [CPU0]   Module[59]: name='/bin/iso9660d' cmdline='' size=80648 bytes
[25585241550] [[35mTRACE[0m] [kernel] [CPU0]   Module[60]: name='/drivers/virtio_sound' cmdline='' size=95232 bytes
[25587288111] [[35mTRACE[0m] [kernel] [CPU0]   Module[61]: name='/drivers/hdaudio' cmdline='' size=67600 bytes
[25589313618] [[35mTRACE[0m] [kernel] [CPU0]   Module[62]: name='/drivers/pci_stubd' cmdline='' size=58224 bytes
[25591343184] [[35mTRACE[0m] [kernel] [CPU0]   Module[63]: name='/drivers/chime' cmdline='' size=55496 bytes
[25593311238] [[35mTRACE[0m] [kernel] [CPU0]   Module[64]: name='/bin/vfs_hello' cmdline='' size=35552 bytes
[25595290611] [[35mTRACE[0m] [kernel] [CPU0]   Module[65]: name='/bin/show_args' cmdline='' size=42568 bytes
[25597283514] [[35mTRACE[0m] [kernel] [CPU0]   Module[66]: name='/bin/env_roundtrip' cmdline='' size=44216 bytes
[25599369015] [[35mTRACE[0m] [kernel] [CPU0]   Module[67]: name='/bin/cwd_test' cmdline='' size=43776 bytes
[25601859129] [[35mTRACE[0m] [kernel] [CPU0]   Module[68]: name='/bin/date' cmdline='' size=62992 bytes
[25604028582] [[35mTRACE[0m] [kernel] [CPU0]   Module[69]: name='/bin/wayland_hello' cmdline='' size=65952 bytes
[25606364850] [[35mTRACE[0m] [kernel] [CPU0]   Module[70]: name='/bin/terminal' cmdline='' size=85376 bytes
[25608635613] [[35mTRACE[0m] [kernel] [CPU0]   Module[71]: name='/bin/tee' cmdline='' size=45032 bytes
[25610248224] [[35mTRACE[0m] [kernel] [CPU0]   Module[72]: name='/bin/xargs' cmdline='' size=55080 bytes
[25611684219] [[35mTRACE[0m] [kernel] [CPU0]   Module[73]: name='/bin/placed' cmdline='' size=38136 bytes
[25613055666] [[35mTRACE[0m] [kernel] [CPU0]   Module[74]: name='/bin/bloom' cmdline='' size=112872 bytes
[25614613563] [[35mTRACE[0m] [kernel] [CPU0]   Module[75]: name='/bin/clear' cmdline='' size=20864 bytes
[25617128196] [[35mTRACE[0m] [kernel] [CPU0]   Module[76]: name='/bin/loglevel' cmdline='' size=41600 bytes
[25618633821] [[35mTRACE[0m] [kernel] [CPU0]   Module[77]: name='/bin/poll_mux' cmdline='' size=39056 bytes
[25620585276] [[35mTRACE[0m] [kernel] [CPU0]   Module[78]: name='/bin/ipc_service_demo' cmdline='' size=47552 bytes
[25622643156] [[35mTRACE[0m] [kernel] [CPU0]   Module[79]: name='/bin/ipc_pipe_demo' cmdline='' size=44016 bytes
[25624069911] [[35mTRACE[0m] [kernel] [CPU0]   Module[80]: name='/bin/ipc_provider_demo' cmdline='' size=67008 bytes
[25625473467] [[35mTRACE[0m] [kernel] [CPU0]   Module[81]: name='/bin/ipc_memfd_demo' cmdline='' size=34216 bytes
[25626882831] [[35mTRACE[0m] [kernel] [CPU0]   Module[82]: name='/bin/test_exec' cmdline='' size=62504 bytes
[25628294076] [[35mTRACE[0m] [kernel] [CPU0]   Module[83]: name='/bin/test_vm_protect' cmdline='' size=34216 bytes
[25629776073] [[35mTRACE[0m] [kernel] [CPU0]   Module[84]: name='/bin/test_exec_env' cmdline='' size=54864 bytes
[25631307207] [[35mTRACE[0m] [kernel] [CPU0]   Module[85]: name='/bin/test_threads' cmdline='' size=78336 bytes
[25632683076] [[35mTRACE[0m] [kernel] [CPU0]   Module[86]: name='/bin/test_futex' cmdline='' size=48952 bytes
[25634373633] [[35mTRACE[0m] [kernel] [CPU0]   Module[87]: name='/bin/ld_so' cmdline='' size=378656 bytes
[25636432371] [[35mTRACE[0m] [kernel] [CPU0]   Module[88]: name='/bin/test_dyn_loader' cmdline='' size=53728 bytes
[25638236019] [[35mTRACE[0m] [kernel] [CPU0]   Module[89]: name='/bin/test_dlopen' cmdline='' size=60232 bytes
[25640509356] [[35mTRACE[0m] [kernel] [CPU0]   Module[90]: name='/bin/reboot' cmdline='' size=32136 bytes
[25642776225] [[35mTRACE[0m] [kernel] [CPU0]   Module[91]: name='/bin/shutdown' cmdline='' size=32136 bytes
[25644945348] [[35mTRACE[0m] [kernel] [CPU0]   Module[92]: name='/bin/attr_list' cmdline='' size=47456 bytes
[25647257724] [[35mTRACE[0m] [kernel] [CPU0]   Module[93]: name='/bin/attr_get' cmdline='' size=47552 bytes
[25648656396] [[35mTRACE[0m] [kernel] [CPU0]   Module[94]: name='/bin/attr_set' cmdline='' size=69656 bytes
[25650580692] [[35mTRACE[0m] [kernel] [CPU0]   Module[95]: name='/bin/attr_rm' cmdline='' size=47072 bytes
[25652866767] [[35mTRACE[0m] [kernel] [CPU0]   Module[96]: name='/lib/libpistil.so' cmdline='' size=67704 bytes
[25654543959] [[35mTRACE[0m] [kernel] [CPU0]   Module[97]: name='/share/wallpapers/clouds.bmp' cmdline='' size=786486 bytes
[25656020577] [[35mTRACE[0m] [kernel] [CPU0]   Module[98]: name='/share/wallpapers/flower.bmp' cmdline='' size=8386614 bytes
[25657564020] [[35mTRACE[0m] [kernel] [CPU0]   Module[99]: name='/share/wallpapers/leather.bmp' cmdline='' size=1179702 bytes
[25659065784] [[35mTRACE[0m] [kernel] [CPU0]   Module[100]: name='/share/wallpapers/linen.bmp' cmdline='' size=4718646 bytes
[25660717368] [[35mTRACE[0m] [kernel] [CPU0]   Module[101]: name='/share/fonts/NotoSans-Regular.ttf' cmdline='' size=569208 bytes
[25662230022] [[35mTRACE[0m] [kernel] [CPU0]   Module[102]: name='/share/fonts/unifont.hex' cmdline='' size=8355852 bytes
[25664078814] [[35mTRACE[0m] [kernel] [CPU0]   Module[103]: name='/etc/locale.conf' cmdline='' size=85 bytes
[25666215366] [[35mTRACE[0m] [kernel] [CPU0]   Module[104]: name='/etc/profile' cmdline='' size=68 bytes
[25668418578] [[35mTRACE[0m] [kernel] [CPU0]   Module[105]: name='/etc/motd' cmdline='' size=610 bytes
[25669804149] [[35mTRACE[0m] [kernel] [CPU0]   Module[106]: name='/etc/fstab' cmdline='' size=123 bytes
[25671629346] [[35mTRACE[0m] [kernel] [CPU0]   Module[107]: name='/etc/hostname' cmdline='' size=8 bytes
[25681330092] [[34mDEBUG[0m] [kernel] [CPU0] Found init module: /bin/sprout (cmdline: 'init'), loading...
[25683834858] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/sprout' (len=85144, base=0x200000)
[25686821358] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[25704952383] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[25709083587] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[25714310457] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [00, 00, 00, e8, e0, 7b, 00, 00]
[25722370575] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20c000 exec=false
[25726225767] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20e000 exec=false
[25734370035] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20f000 filesz=0 memsz=0 align=1
[25748539344] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='load_init_module' elapsed_ticks=64919052 elapsed_us=32459 total_ticks=740781129 total_us=370390
[25754735655] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='prepare_boot_registry' elapsed_ticks=2441043 elapsed_us=1220 total_ticks=747027435 total_us=373513
[25757401659] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='map_boot_registry' elapsed_ticks=197472 elapsed_us=98 total_ticks=749728221 total_us=374864
[25759703046] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='activate_init_aspace' elapsed_ticks=163845 elapsed_us=81 total_ticks=751982847 total_us=375991
[25761966186] [[34mDEBUG[0m] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[25763080497] [[34mDEBUG[0m] [kernel] [CPU0] Spawning init process...
[25765839825] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb00661e0 arg=0xffffffffb001fd20
[25791323184] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[25793761620] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=5
[25795986975] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[25797909027] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='spawn_init_task' elapsed_ticks=34854930 elapsed_us=17427 total_ticks=790216680 total_us=395108
[25848435657] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED: early-boot bringup complete; resuming steady-state SMP scheduling
[25850033550] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='end_bringup' elapsed_ticks=2012769 elapsed_us=1006 total_ticks=842356350 total_us=421178
[25896458478] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=888420027 elapsed_us=444210
[25898973771] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[25919198646] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[25921995396] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] ENTER_USER: TID=5 entry_pc=200000 user_sp=800000
[25974490344] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpus_online=4
[25987101360] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=0 curr=Some(5) runq=1 runq_avg=2 runq_samples=1 ctxsw=1 idle2busy=0 tick=1 ipi=0 enq=3 deq=1 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[25992428418] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=1 curr=Some(2) runq=0 runq_avg=0 runq_samples=18 ctxsw=0 idle2busy=0 tick=13 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[25999359012] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=2 curr=Some(3) runq=0 runq_avg=0 runq_samples=17 ctxsw=0 idle2busy=0 tick=13 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[26005823382] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=3 curr=Some(4) runq=0 runq_avg=0 runq_samples=17 ctxsw=0 idle2busy=0 tick=13 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[26017195842] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=0
[26057495343] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=20
[26071344618] [[34mDEBUG[0m] [sprout] [CPU0] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7fef70 rip=0x20414c rflags=0x202
[26073608946] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[26075417577] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: About to create Supervisor...
[26077602738] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Listing /bin directory...
[26081145486] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/bin' tid=5
[26157228009] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Parsed command line: '    '
[26164559223] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: FORCING display=bootfb for diagnostic test!
[26166455601] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Supervisor created, calling run_forever...
[26168095206] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[26169688215] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[26172187107] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[26173960593] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/run/sprout/shell' tid=5
[26182643487] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/etc/default/shell' tid=5
[26190310839] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/bin/sh' tid=5
[26194521243] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU0] BootFs: EXACT match for 'bin/sh' at index 4
[26206538061] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[26233460484] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[26243161560] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[26261503455] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU0] BootFs: EXACT match for 'bin/sh' at index 4
[26286798714] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'sh' (len=121736, base=0x200000)
[26288597709] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[26291986314] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[26294671557] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[26299187508] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [9c, b9, 00, 00, 48, 8d, 7c, 24]
[26312081631] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[26315470632] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=216000 exec=false
[26318617347] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x217000 filesz=0 memsz=0 align=1
[26330424351] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /bin/sh
[26335358511] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb009d500 arg=0xffffffffb006f660
[26343111729] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=6, applying inserts
[26345623392] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[26347555146] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=6
[26349775023] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[26351766771] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[26361090129] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SETUP_STDIO: stdin=Fd(3) stdout=Fd(4) stderr=Fd(5)
[26376368271] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 6
[26381669952] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[26384573391] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Task 6 woken, restoring IRQs
[26385937974] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Done for /bin/sh
[26390676543] [[35mTRACE[0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[26395767585] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[26397812298] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[26399836749] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=6 entry_pc=200000 user_sp=800000
[26401991253] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=0
[26404959240] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=100
[26407400778] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 250ms so the prompt can take the foreground
[26410056684] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=19
[26412846339] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[26415619956] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[26420824980] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=19
DEBUG: sh starting
[26437491828] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=33
[26440308807] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[26442962535] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[26445484692] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=33
DEBUG: sh sig handlers installed
[26449718790] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/run/motd' tid=6
[26457870846] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/etc/motd' tid=6
[26463940899] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'etc/motd' at index 105
[26470400682] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=610
[26473274652] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[26476691670] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[26478971937] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=610
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
[26489427558] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=23
[26491770393] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[26493764847] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[26495898726] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=23
DEBUG: sh motd printed
[26499926046] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=31
[26501803713] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[26503315773] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[26504938119] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=31
DEBUG: sh shell object created
[26508166707] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/etc/profile' tid=6
[26513246628] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'etc/profile' at index 104
[26544360117] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=25
[26546446740] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[26548592730] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[26551153299] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=25
DEBUG: sh profile loaded
[26562762996] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=97
[26565981816] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[26568749130] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[26571610626] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=97
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [26581323549] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=6
[26583353016] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[26584839105] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[26586608895] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=6
[?25h[26590222032] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[28190321445] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=1765933026
[29052134595] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=0 apic_id=0 vector=0x30
[29057235273] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[29059794225] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[29066971263] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=4096 port=0xffffffffb0071530
[29071470582] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(0)
[29078121699] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=1 port_id=PortId(0) mode=Write
[29081124270] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=2 port_id=PortId(0) mode=Read
[29088466539] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/cambium' (len=124728, base=0x200000)
[29090269923] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[29093910417] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[29095990308] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[29099286348] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [8d, 3c, f0, 48, 83, c7, 70, 48]
[29109820509] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=214000 exec=false
[29113965441] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=216000 exec=false
[29117377212] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x217000 filesz=0 memsz=0 align=1
[29125028196] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb00818e0 arg=0xffffffffb00716c0
[29128415910] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[29129612556] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=7
[29130839331] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[29138926905] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 7 �� task '/bin/cambium' (pid=7 from boot module)
[29140863708] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=1 apic_id=1 vector=0x30
[29145052002] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned cambium (PID=7)
[29147232147] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[29148678966] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[29151140997] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=7 entry_pc=200000 user_sp=800000
[29153292663] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/iso9660d' (len=80648, base=0x200000)
[29155655331] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[29158559034] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=0
[29160819072] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[29162527647] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=100
[29165058021] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[29169286047] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [48, 89, 44, 24, 50, 48, 8d, 05]
[29177262444] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20b000 exec=false
[29179759224] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20d000 exec=false
[29182017975] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20e000 filesz=0 memsz=0 align=1
[29187861417] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb00b6300 arg=0xffffffffb00716c0
[29193035124] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[29195301069] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=8
[29197360830] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[29203263210] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 8 → task '/bin/iso9660d' (pid=8 from boot module)
[29206354056] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[29209454274] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned iso9660d (PID=8)
[29211018606] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[29213512878] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Deferring netd until /dev/net/virtio0/rx is ready...
[29215016688] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=8 entry_pc=200000 user_sp=800000
[29217282435] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[29219488518] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=0
[29221772811] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers' tid=7
[29224171350] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=100
[29238803253] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Task 9 (user thread) assigned to CPU 3
[29244472884] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[29246170338] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running ��� Runnable (cpu=3, next=9)
[29248345797] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=9
[29249809908] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[29252360577] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Sending post-unlock Resched IPI to CPU 3 for task 9
[29254299789] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[29256042156] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=0
[29259297309] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Running registration + health supervision loop
[29260907841] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=20
[29265236022] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU0] BRIDGE: handle=2 -> fd=3 node=0xffffffffb008cc50 port=0xffffffffb0071530
[29269158138] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Bridged resp_port 2 -> FD 3 for task 'cambium'
[29270691351] [[32mINFO [0m] [sprout::supervisor] [CPU3] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[29273630694] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[29282764698] [[34mDEBUG[0m] [iso9660d] [CPU2] iso9660d: starting ISO9660 VFS provider
[29285426841] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/services/storage' tid=8
[29289703212] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ahci_disk' tid=7
[29295083664] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ahci_disk' at index 57
[29297758281] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet �� retrying
[29570948055] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[29574802818] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ata_disk' tid=7
[29578245345] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ata_disk' at index 58
[29597882358] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[29600526483] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running → Runnable (cpu=3, next=9)
[29604465396] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[29763138438] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.ata' start='thingos_driver_start_safe'
[29766376431] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/chime' tid=7
[29770173873] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/chime' at index 63
[29943143934] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[29946657345] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/display_bootfb' tid=7
[29950473729] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/display_bootfb' at index 41
[29954890317] [[35mTRACE[0m] [bran::arch::x86_64] [CPU2] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[29957592423] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running → Runnable (cpu=3, next=9)
[29962120716] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[30136677450] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[30141145254] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/display_virtio_gpu' tid=7
[30144967446] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/display_virtio_gpu' at index 42
[30208470732] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpus_online=4
[30212703345] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=0 curr=Some(0) runq=0 runq_avg=1 runq_samples=86 ctxsw=8 idle2busy=0 tick=22 ipi=0 enq=87 deq=86 wake=2 lock_miss=0 lock_pending=0 lock_blocked=0
[30216735351] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=1 curr=Some(7) runq=0 runq_avg=0 runq_samples=115 ctxsw=1 idle2busy=1 tick=46 ipi=0 enq=2 deq=1 wake=1 lock_miss=0 lock_pending=0 lock_blocked=0
[30221125803] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=2 curr=Some(3) runq=0 runq_avg=0 runq_samples=146 ctxsw=2 idle2busy=1 tick=55 ipi=0 enq=2 deq=1 wake=1 lock_miss=0 lock_pending=0 lock_blocked=0
[30224896119] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=3 curr=Some(6) runq=0 runq_avg=0 runq_samples=10368 ctxsw=7 idle2busy=1 tick=31 ipi=3 enq=10339 deq=10338 wake=3 lock_miss=0 lock_pending=0 lock_blocked=0
[30337394076] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[30340644741] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/hdaudio' tid=7
[30344453700] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/hdaudio' at index 61
[30347324502] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[30349758087] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running → Runnable (cpu=3, next=9)
[30352680402] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[30529876212] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[30533132124] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/pci_stubd' tid=7
[30536667249] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/pci_stubd' at index 62
[30703573032] [[35mTRACE[0m] [bran::arch::x86_64] [CPU2] SMP: send_ipi cpu_index=0 apic_id=0 vector=0x30
[30706004637] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running → Runnable (cpu=3, next=9)
[30708213360] [[35mTRACE[0m] [bran::arch::x86_64] [CPU2] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[30710907216] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[30717748017] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[30720640863] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ps2_kbd' tid=7
[30723166089] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ps2_kbd' at index 3
[30894584259] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[30897868122] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ps2_mouse' tid=7
[30901391103] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ps2_mouse' at index 40
[31045429305] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/services/storage' tid=8
[31068485712] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[31072296717] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/rtc_cmos' tid=7
[31075292457] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/rtc_cmos' at index 2
[31180917603] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[31183148403] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running → Runnable (cpu=3, next=9)
[31187094774] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[31283844768] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[31289188227] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/rtl8168d' tid=7
[31292540301] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/rtl8168d' at index 45
[31452436851] [[35mTRACE[0m] [bran::arch::x86_64] [CPU2] SMP: send_ipi cpu_index=0 apic_id=0 vector=0x30
[31469039778] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[31471990935] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/virtio_netd' tid=7
[31475089899] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/virtio_netd' at index 44
[31638441945] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[31641153753] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running ��� Runnable (cpu=3, next=9)
[31645122696] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[31657252341] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[31659959859] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/virtio_sound' tid=7
[31662997146] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/virtio_sound' at index 60
[31841978652] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[31845487938] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[31848203178] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices' tid=7
[31852193637] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices'
[31860095355] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: readdir found 10 slots
[31864784259] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: readdir wrote 147 bytes
[31870340601] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/vendor' tid=7
[31874839029] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:00.0/vendor'
[31879799391] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:00.0/vendor'
[31885897329] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/device' tid=7
[31888547724] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:00.0/device'
[31890797796] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:00.0/device'
[31895203824] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/class' tid=7
[31898744823] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:00.0/class'
[31902223254] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:00.0/class'
[31907428245] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/status' tid=7
[31910271987] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:00.0/status'
[31912576146] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:00.0/status'
[31916169186] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/kind' tid=7
[31918927029] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:00.0/kind'
[31921849113] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:00.0/kind'
[31926133470] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/vendor' tid=7
[31929131817] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/vendor'
[31931540817] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:01.0/vendor'
[31935102837] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/device' tid=7
[31937585691] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/device'
[31939738776] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:01.0/device'
[31943078145] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/class' tid=7
[31945657062] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/class'
[31948275810] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:01.0/class'
[31952135754] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/status' tid=7
[31954733844] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/status'
[31957285602] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:01.0/status'
[31960747632] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/kind' tid=7
[31963524549] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/kind'
[31965757131] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:01.0/kind'
[31969337235] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/vendor' tid=7
[31971896286] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/vendor'
[31974449067] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/vendor'
[31978970859] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/device' tid=7
[31981598220] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/device'
[31983728238] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/device'
[31987423611] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/class' tid=7
[31989854325] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/class'
[31992012393] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/class'
[31995214383] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/status' tid=7
[31997790858] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/status'
[32000031261] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/status'
[32004961098] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/kind' tid=7
[32007684621] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/kind'
[32009762499] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/kind'
[32013578190] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.0/vendor' tid=7
[32016017847] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.0/vendor'
[32018288016] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.0/vendor'
[32021364408] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.0/device' tid=7
[32023804725] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.0/device'
[32025956127] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.0/device'
[32029515045] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.0/class' tid=7
[32033118777] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.0/class'
[32035496757] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.0/class'
[32039201700] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.0/status' tid=7
[32041767054] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.0/status'
[32044199088] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.0/status'
[32047467903] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.0/kind' tid=7
[32050070217] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.0/kind'
[32052556107] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.0/kind'
[32055975369] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.2/vendor' tid=7
[32059746147] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/vendor'
[32062245006] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/vendor'
[32066452341] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.2/device' tid=7
[32069362941] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/device'
[32071684755] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/device'
[32075195625] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.2/class' tid=7
[32077953501] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/class'
[32080410384] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/class'
[32083893633] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.2/status' tid=7
[32088415458] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/status'
[32092504521] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/status'
[32098255959] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.2/kind' tid=7
[32102123196] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/kind'
[32104904040] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/kind'
[32108666370] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.3/vendor' tid=7
[32111473086] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.3/vendor'
[32115594522] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.3/vendor'
[32118881553] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.3/device' tid=7
[32122719981] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.3/device'
[32125187721] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.3/device'
[32128561839] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.3/class' tid=7
[32131294437] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.3/class'
[32133944700] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.3/class'
[32137160616] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.3/status' tid=7
[32140570044] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.3/status'
[32143886379] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.3/status'
[32147559114] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.3/kind' tid=7
[32150863833] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.3/kind'
[32153330253] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.3/kind'
[32156581677] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0070/vendor' tid=7
[32159146437] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/vendor'
[32161517916] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/vendor'
[32164697466] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0070/device' tid=7
[32166930015] [[35mTRACE[0m] [bran::arch::x86_64] [CPU2] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[32169403035] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running → Runnable (cpu=3, next=9)
[32171721054] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/device'
[32174052702] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/device'
[32175796257] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[32179210635] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0070/class' tid=7
[32181632109] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/class'
[32184167796] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/class'
[32187270720] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0070/status' tid=7
[32189820135] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/status'
[32192413209] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/status'
[32196057432] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0070/kind' tid=7
[32198678985] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/kind'
[32201016837] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/kind'
[32204968455] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0060/vendor' tid=7
[32207215293] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/vendor'
[32209598883] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/vendor'
[32212696659] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0060/device' tid=7
[32215086354] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/device'
[32217537858] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/device'
[32220753246] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0060/class' tid=7
[32224408161] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/class'
[32228277543] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/class'
[32233511343] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0060/status' tid=7
[32237257404] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/status'
[32241150645] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/status'
[32246462259] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0060/kind' tid=7
[32250459318] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/kind'
[32254307382] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/kind'
[32257672623] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-01f0/vendor' tid=7
[32260654833] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-01f0/vendor'
[32263319550] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-01f0/vendor'
[32266272852] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-01f0/device' tid=7
[32268933609] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-01f0/device'
[32271515067] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-01f0/device'
[32274934857] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-01f0/class' tid=7
[32277781437] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-01f0/class'
[32280711441] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-01f0/class'
[32283884721] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-01f0/status' tid=7
[32287010217] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-01f0/status'
[32289400407] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-01f0/status'
[32292479175] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-01f0/kind' tid=7
[32294801418] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-01f0/kind'
[32297122374] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-01f0/kind'
[32303558991] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/status' tid=7
[32306276079] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/status'
[32308641354] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/status'
[32319026157] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=3 len=144
[32320882077] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=3 starting copyin
[32322798915] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=3 copyin ok
[32332027629] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/virtio_netd' at index 44
[32348964153] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'virtio_netd' (len=106272, base=0x200000)
[32351388927] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[32355170529] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[32357454228] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=200000 exec=true
[32361088419] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   COPIED at 0x201420: [e9, 0f, 86, 41, 04, 00, 00, be]
[32370862260] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=20e000 exec=false
[32374702173] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=212000 exec=false
[32377340259] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x214000 filesz=0 memsz=0 align=1
[32389912302] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x20277c + bias 0x0 = PC 0x20277c
[32392450299] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/virtio_netd
[32394452739] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb00f8220 arg=0xffffffffb008cfa0
[32398522563] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=10, applying inserts
[32400386106] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[32401752174] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=10
[32403293439] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[32404560870] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[32407275912] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[32413392297] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 10
[32416757934] [[35mTRACE[0m] [kernel::sched::blocking] [CPU1] WAKE_TASK: ID=10 taking SCHEDULER lock
[32424299721] [[35mTRACE[0m] [kernel::sched::blocking] [CPU1] WAKE_TASK: ID=10 wake_task_locked returned IPI_CPU=None
[32427920910] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 10 woken, restoring IRQs
[32430656445] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/virtio_netd
[32435475072] [[35mTRACE[0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=10 PID=10
[32440906410] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/virtio_netd for pci-0000:00:02.0 (entry='thingos_driver_start_safe', pid=10)
[32445510273] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.2/status' tid=7
[32448118164] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/status'
[32451023814] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/status'
[32455328334] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=4 len=144
[32457104625] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=4 starting copyin
[32458668693] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=4 copyin ok
[32463653112] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ahci_disk' at index 57
[32476736358] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'ahci_disk' (len=74072, base=0x200000)
[32480013984] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[32485427337] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[32488905900] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=200000 exec=true
[32493162339] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   COPIED at 0x201420: [89, 85, 88, 05, 00, 00, 48, c7]
[32500378053] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=209000 exec=false
[32504233080] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=20b000 exec=false
[32508615645] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x20e000 filesz=0 memsz=0 align=1
[32516781297] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[32520639360] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/ahci_disk
[32523626553] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb0110020 arg=0xffffffffb00be2e0
[32542257033] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=11, applying inserts
[32543998740] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[32545580298] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=11
[32547078366] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[32548477104] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[32551877292] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[32556084462] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[32557432017] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running → Runnable (cpu=3, next=9)
[32558794719] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 11
[32560570185] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[32562492402] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 11 woken, restoring IRQs
[32564072937] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[32566513815] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x4
[32568877737] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/ahci_disk
[32571155001] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=11 entry_pc=201000 user_sp=800000
[32573438898] [[35mTRACE[0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ahci_disk' TID=11 PID=11
[32576416554] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=11 buf_len=0
[32578658376] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/ahci_disk for pci-0000:00:1f.2 (entry='thingos_driver_start_safe', pid=11)
[32581693155] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=11 buf_len=100
[32584208547] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-01f0/status' tid=7
[32586818616] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver (boot_fd=4)
[32588380209] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-01f0/status'
[32592617145] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-01f0/status'
[32599038219] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=5 len=144
[32601616344] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=5 starting copyin
[32604288750] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=5 copyin ok
[32610908319] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ata_disk' at index 58
[32621941077] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'ata_disk' (len=61712, base=0x200000)
[32623960776] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[32627629386] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[32629764189] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=200000 exec=true
[32633414715] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   COPIED at 0x201420: [00, 80, 66, 66, 66, 66, 66, 2e]
[32636093160] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 'pci-0000:00:1f.2' (handle 0)
[32639125728] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Claimed PCI device '/sys/devices/pci-0000:00:1f.2' handle=0
[32640530307] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=208000 exec=false
[32642836875] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: mapping BAR5 (phys=0x80880000, size=0x1000) for task 11
[32644797669] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=20a000 exec=false
[32647079289] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x20b000 filesz=0 memsz=0 align=1
[32649017247] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[32653462479] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[32655402879] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x201000 + bias 0x0 = PC 0x201000
[32657883918] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[32659901505] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/ata_disk
[32662072377] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[32663750361] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb0124020 arg=0xffffffffb00dd820
[32666373465] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: DMA virt=0x20c000
[32667906315] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=12, applying inserts
[32670259512] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[32672194038] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: DMA phys=0x22fa000
[32673890073] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=12
[32675556177] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[32677146117] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[32679401139] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 0...
[32681516604] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[32683768194] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[32686492476] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 1...
[32688823563] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[32690497719] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 12
[32692856757] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 2...
[32693952093] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[32696275689] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[32698201701] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running → Runnable (cpu=3, next=12)
[32700787053] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 12 woken, restoring IRQs
[32703020922] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=4096 port=0xffffffffb00e7670
[32705849055] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/ata_disk
[32707992174] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(1)
[32710612869] [[35mTRACE[0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/ata_disk' TID=12 PID=12
[32713673949] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(1) mode=Write
[32716582239] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x201000 SP=0x800000 ARG0=0x5
[32719089678] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=4 port_id=PortId(1) mode=Read
[32721675360] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=12 entry_pc=201000 user_sp=800000
[32725305195] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=12 buf_len=0
[32727515139] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x20277c SP=0x800000 ARG0=0x3
[32729444583] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=12 buf_len=100
[32731901730] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=10 entry_pc=20277c user_sp=800000
[32735616573] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Starting ATA disk driver v1 (with ATAPI support)
[32737741575] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/services/storage/atapi2' tid=11
[32740543143] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing primary port (0x1F0)...
[32742555648] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=0
[32746421202] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=100
[32764962120] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/services/storage' tid=8
[32778833571] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/services/storage/atapi2' tid=8
[32784721893] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Probing secondary port (0x170)...
[32790473364] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=11 fd=5 len=1
[32793376110] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=11 fd=5 starting copyin
[32795761581] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Starting VirtIO-NET driver service...
[32797870908] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=11 fd=5 copyin ok
[32800308090] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Initializing hardware driver...
[32803523709] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Claiming NIC device at /sys/devices/pci-0000:00:02.0
[32805219645] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device port=2 model='                                        ' rpc_port=3
[32807255250] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: device::new(/sys/devices/pci-0000:00:02.0)
[32809566999] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 3...
[32810723649] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: claiming '/sys/devices/pci-0000:00:02.0'...
[32813283294] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 10 claimed device 'pci-0000:00:02.0' (handle 1)
[32815651935] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[32817292464] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: claimed, handle=1
[32818598967] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 4...
[32820194946] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[32821322985] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/virtio/common_bar' tid=10
[32823404658] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Probing port 5...
[32824926288] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/common_bar'
[32829259980] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[32830938921] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[32832581859] [[34mDEBUG[0m] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[32834842689] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=4 -> fd=5 node=0xffffffffb00ce710 port=0xffffffffb00e7670
[32837964060] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/common_bar -> '4' (n=2)
[32841172551] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Found 0 ATA disk(s), 0 ATAPI device(s)
[32843016822] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/virtio/common_offset' tid=10
[32846557458] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: Entering RPC service loop
[32848399320] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/common_offset'
[32851423341] [[32mINFO [0m] [ata_disk] [CPU3] ATA_DISK: No active devices to service
[32856351132] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/common_offset -> '0x0' (n=4)
[32859889656] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/virtio/notify_bar' tid=10
[32862834906] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/notify_bar'
[32867376729] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/notify_bar -> '4' (n=2)
[32869634853] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/virtio/notify_offset' tid=10
[32872397349] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/notify_offset'
[32876917359] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/notify_offset -> '0x3000' (n=7)
[32879218614] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/virtio/notify_multiplier' tid=10
[32883024768] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/notify_multiplier'
[32887792707] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/notify_multiplier -> '4' (n=2)
[32890854315] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[32893845996] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/virtio/device_bar' tid=10
[32897711583] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/device_bar'
[32903036826] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/device_bar -> '4' (n=2)
[32906034579] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/virtio/device_offset' tid=10
[32909909703] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/device_offset'
[32918612562] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/device_offset -> '0x2000' (n=7)
[32922395418] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: mapping common BAR4...
[32926133229] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: mapping BAR4 (phys=0xc000004000, size=0x4000) for task 10
[32930852196] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[32933405439] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running �� Runnable (cpu=3, next=9)
[32938002702] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x10001000
[32940854628] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[32943630522] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: common_cfg at 0x10001000
[32946109746] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: notify_cfg at 0x10004000
[32949810729] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: device_cfg = Some(268447744)
[32951533560] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: allocating DMA command buffer...
[32957264835] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 1 pages phys=0x23ba000 -> user_va=0x10005000
[32961773493] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: cmd_buf virt=0x10005000 phys=0x23ba000
[32964578889] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: device::new complete
[32968687059] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Device features 0x30bf8024
[32972518326] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: MAC 52:54:00:12:34:56
[32974332138] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Link UP
[32979804858] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 4 pages phys=0x23bb000 -> user_va=0x10006000
[32984723805] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: RX queue 0 setup (size=32)
[32990965227] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 4 pages phys=0x23bf000 -> user_va=0x1000a000
[32996632845] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: TX queue 1 setup (size=32)
[33004407678] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 16 pages phys=0x23c3000 -> user_va=0x1000e000
[33008551224] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[33011891451] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 1 pages phys=0x23d3000 -> user_va=0x1001e000
[33016664901] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Allocated TX buffer (1 page)
[33020584146] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: RX queue filled with 32 buffers
[33023912493] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: DRIVER_OK set, device is live
[33026769897] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Driver initialized successfully
[33029751150] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Driver initialized successfully
[33031736133] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: MAC 52:54:00:12:34:56
[33033719202] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Initial link state is UP
[33039195651] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=65536 port=0xffffffffb00cfa70
[33040986165] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(2)
[33042764469] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=3 port_id=PortId(2) mode=Write
[33045658371] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=4 port_id=PortId(2) mode=Read
[33049248573] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Created VFS provider port
[33058547808] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Task 13 (user thread) assigned to CPU 1
[33061335978] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[33062811672] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=13
[33064340430] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[33068038872] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=13 buf_len=0
[33069888126] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=13 buf_len=100
[33072427245] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Entering VFS provider service loop at /dev/net/virtio0
[33085365027] [[34mDEBUG[0m] [kernel::sched] [CPU2] SCHED: CPU 2 stole task 7 (prio 2) from CPU 1 (depth 2)
[33090202332] [[34mDEBUG[0m] [cambium::spawn] [CPU2] CAMBIUM: launched driver /drivers/ata_disk for isa-01f0 (entry='thingos_driver_start_safe', pid=12)
[33095498238] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/proc/12/job_observer' tid=7
[33130832856] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=7 fd=6 len=1
[33132830049] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=7 fd=6 starting copyin
[33134604987] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=7 fd=6 copyin ok
[33139807965] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/proc/10/job_observer' tid=7
[33147022590] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=7 fd=6 len=1
[33148800432] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=7 fd=6 starting copyin
[33150544482] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=7 fd=6 copyin ok
[33154107723] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/proc/11/job_observer' tid=7
[33162011157] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=7 fd=6 len=1
[33164819325] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=7 fd=6 starting copyin
[33167560503] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=7 fd=6 copyin ok
[33171296565] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=262400 port=0xffffffffb00d0b50
[33173015304] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/proc/self/inbox' tid=7
[33174647055] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(3)
[33177558447] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=5 port_id=PortId(3) mode=Write
[33180631539] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU2] sysfs: lookup path='devices'
[33184290315] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] vfs: mounted userland provider at /dev/net/virtio0 (flags: 0x0)
[33187536228] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Mounted at /dev/net/virtio0
[33189533850] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Provider thread live at /dev/net/virtio0
[33196336041] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[33198306306] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[33199986864] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[33201855621] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
m[33205414737] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[33289122768] [[35mTRACE[0m] [bran::arch::x86_64] [CPU2] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[33291713268] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running �� Runnable (cpu=3, next=9)
[33297743721] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[33305992368] [[35mTRACE[0m] [kernel::ipc::port] [CPU3] PORT: Port written 13 bytes from TID 9
[33308279004] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 13 bytes from TID 13
[33311101923] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 13 bytes from port PortId(2)
[33316027965] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Lookup resp_port=5 payload_len=6
[33318988626] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Lookup payload_len=6
[33322366143] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'rx'
[33324437355] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'rx' -> handle=5
[33326699571] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Lookup resp_port=5 status=0 resp_payload_len=8
[33329909811] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Lookup resp_port=5
[33334624983] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[33336690189] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 9 bytes to port PortId(3)
[33338727477] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running → Runnable (cpu=3, next=9)
[33340455060] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Lookup resp_port=5
[33342195480] [[35mTRACE[0m] [kernel::ipc::port] [CPU3] PORT: Port read 9 bytes from TID 9
[33353228568] [[35mTRACE[0m] [kernel::ipc::port] [CPU3] PORT: Port written 15 bytes from TID 9
[33354729243] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 15 bytes from TID 13
[33356909355] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 15 bytes from port PortId(2)
[33361209024] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[33362821932] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Close resp_port=5 payload_len=8
[33364359765] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[33365986137] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Close payload_len=8
[33367677420] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[33369157668] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Close resp_port=5 status=0 resp_payload_len=0
[33370900563] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
[33372610689] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Close resp_port=5
o[33377391564] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[33379371894] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[33381506895] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 1 bytes to port PortId(3)
[33383805873] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running �� Runnable (cpu=3, next=9)
[33386027235] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Close resp_port=5
[33387804417] [[35mTRACE[0m] [kernel::ipc::port] [CPU3] PORT: Port read 1 bytes from TID 9
[33393576381] [[32mINFO [0m] [sprout::supervisor] [CPU3] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[33396144474] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: Loading module '/bin/netd' (len=10813352, base=0x200000)
[33398035143] [[35mTRACE[0m] [kernel::task::loader] [CPU3]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[33401497107] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[33403841130] [[35mTRACE[0m] [kernel::task::loader] [CPU3] Segment: vaddr=200000 exec=true
[33407284845] [[35mTRACE[0m] [kernel::task::loader] [CPU3]   COPIED at 0x201420: [48, 8b, 06, 48, 8b, 80, 70, 01]
[33436176477] [[35mTRACE[0m] [kernel::task::loader] [CPU3] Segment: vaddr=234000 exec=false
[33441059388] [[35mTRACE[0m] [kernel::task::loader] [CPU3] Segment: vaddr=23a000 exec=false
[34222601292] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpus_online=4
[34224675012] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=0 curr=Some(0) runq=0 runq_avg=1 runq_samples=516 ctxsw=24 idle2busy=0 tick=70 ipi=1 enq=517 deq=516 wake=10 lock_miss=0 lock_pending=0 lock_blocked=0
[34230041934] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=1 curr=Some(13) runq=1 runq_avg=1 runq_samples=1066 ctxsw=25 idle2busy=1 tick=83 ipi=0 enq=955 deq=953 wake=2 lock_miss=0 lock_pending=0 lock_blocked=0
[34235567982] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=2 curr=Some(3) runq=0 runq_avg=0 runq_samples=264 ctxsw=10 idle2busy=4 tick=93 ipi=0 enq=6 deq=5 wake=4 lock_miss=0 lock_pending=0 lock_blocked=0
[34241267313] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=3 curr=Some(9) runq=1 runq_avg=0 runq_samples=17418 ctxsw=28 idle2busy=1 tick=42 ipi=11 enq=17390 deq=17388 wake=14 lock_miss=0 lock_pending=0 lock_blocked=0
[34461326031] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[34464696684] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/services/storage' tid=8
[34472148150] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/services/storage/atapi2' tid=8
[34485844338] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=65536 port=0xffffffffb00d2170
[34488468861] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(4)
[34490936667] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(4) mode=Write
[34493725629] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=4 port_id=PortId(4) mode=Read
[34498770702] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 21 bytes from TID 8
[34502315991] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 21 bytes from TID 8
[34504011993] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU2] sys_port_recv: read 21 bytes from port PortId(4)
[34520270367] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=0 apic_id=0 vector=0x30
[34832601738] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: TLS block: tp=0xc3e000 filesz=0 memsz=0 align=1
[34838979516] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb016c020 arg=0xffffffffb001fd20
[34842579519] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[34844069535] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=14
[34845458109] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[34850051412] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SCHED: TID 14 → task '/bin/netd' (pid=14 from boot module)
[34852211493] [[35mTRACE[0m] [bran::arch::x86_64] [CPU3] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[34855009695] [[34mDEBUG[0m] [sprout::supervisor] [CPU3] SPROUT: Spawned netd (PID=14)
[34857310026] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[34860056220] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=14 entry_pc=200000 user_sp=800000
[34863193266] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=14 buf_len=0
[34865414562] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=14 buf_len=100
[34866927282] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[34868943648] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[34870885005] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[34872726867] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[34874293971] [[34mDEBUG[0m] [netd] [CPU2] NETD: binary v2 (with heap storage) starting...
[34876716798] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
u[34878768078] [[34mDEBUG[0m] [netd] [CPU2] NETD: main entry point, arg=0
[34880373957] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[34882575156] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[34884488463] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[34886359332] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[34888452423] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
n[34890416088] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[34892284977] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[34894094664] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[34895833731] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[34897811652] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
t[34899650049] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[34901346744] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[34903129734] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[34904798907] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[34906444452] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
 [34908438081] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[34910426958] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[34912535130] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[34914369633] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[34916169387] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
-[34918307028] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[34920173640] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[34921756155] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[34923457833] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[34925176968] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
[34926644115] [[34mDEBUG[0m] [netd] [CPU2] NETD: get_args starting...
t[34928866830] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[34930126011] [[34mDEBUG[0m] [netd] [CPU2] NETD: argv_get len=17
[34931858907] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[34933450695] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[34935042813] [[34mDEBUG[0m] [netd] [CPU2] NETD: get_args returning 0 args
[34936360074] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[34937922723] [[34mDEBUG[0m] [netd] [CPU2] NETD: Starting network service (Phase 3 �� /net/ VFS provider)
[34939709838] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
[34941640008] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
 [34944401151] [[34mDEBUG[0m] [netd] [CPU2] NETD: NIC probe loop starting (round=0)
[34946239845] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[34948185063] [[34mDEBUG[0m] [netd] [CPU2] NETD: Probing /dev/net/virtio0...
[34949417844] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[34950616998] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=14
[34952417478] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[34954810473] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 13 bytes from TID 14
[34956249834] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[34959130041] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
[34961409912] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 13 bytes from TID 13
h[34963181187] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 13 bytes from port PortId(2)
[34965045852] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[34967129076] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[34968852369] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Lookup resp_port=5 payload_len=6
[34971076569] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[34972466661] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Lookup payload_len=6
[34973971494] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[34975464018] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'rx'
[34976752074] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
[34978226910] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'rx' -> handle=5
t[34981090683] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Lookup resp_port=5 status=0 resp_payload_len=8
[34983381576] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[34984687716] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Lookup resp_port=5
[34987976199] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[34990210398] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 9 bytes from TID 14
[34992566268] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 9 bytes to port PortId(3)
[34996379286] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Lookup resp_port=5
[35000732250] [[34mDEBUG[0m] [netd] [CPU2] NETD: Found /dev/net/virtio0/rx, opening others...
[35003737659] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/dev/net/virtio0/tx' tid=14
[35008271397] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 13 bytes from TID 14
[35045057289] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[35046923637] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[35049187800] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[35051639964] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
t[35054023125] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[35071265889] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 13 bytes from TID 13
[35072962188] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 13 bytes from port PortId(2)
[35076837972] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Lookup resp_port=5 payload_len=6
[35079187638] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Lookup payload_len=6
[35081150577] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'tx'
[35082949605] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'tx' -> handle=6
[35084701212] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Lookup resp_port=5 status=0 resp_payload_len=8
[35086832781] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Lookup resp_port=5
[35089431498] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[35091525414] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 9 bytes to port PortId(3)
[35093477595] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 9 bytes from TID 14
[35095807758] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Lookup resp_port=5
[35102898996] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/dev/net/virtio0/events' tid=14
[35107943739] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 17 bytes from TID 14
[35110217076] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 17 bytes from TID 13
[35112820776] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 17 bytes from port PortId(2)
[35117767212] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Lookup resp_port=5 payload_len=10
[35119823178] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Lookup payload_len=10
[35122583529] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'events'
[35125356486] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'events' -> handle=8
[35128006353] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Lookup resp_port=5 status=0 resp_payload_len=8
[35131383672] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Lookup resp_port=5
[35134779438] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[35137567905] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 9 bytes to port PortId(3)
[35139802005] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 9 bytes from TID 14
[35141280768] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Lookup resp_port=5
[35144580504] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/dev/net/virtio0/mac' tid=14
[35147424576] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 14 bytes from TID 14
[35148839352] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 14 bytes from TID 13
[35150555616] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 14 bytes from port PortId(2)
[35154271020] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Lookup resp_port=5 payload_len=7
[35156363088] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Lookup payload_len=7
[35159091132] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'mac'
[35161613454] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'mac' -> handle=3
[35163250287] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Lookup resp_port=5 status=0 resp_payload_len=8
[35165584179] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Lookup resp_port=5
[35167681758] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[35169313476] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 9 bytes to port PortId(3)
[35171552691] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 9 bytes from TID 14
[35173866849] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Lookup resp_port=5
[35179558161] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[35181556608] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[35183374050] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[35187763182] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[35189982069] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[35192356254] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[35194472808] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[35196597051] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[35198770002] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[35201301729] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[35204555958] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[35208384552] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 27 bytes from TID 14
[35211324984] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 27 bytes from TID 13
[35213390850] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[35215752924] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 27 bytes from port PortId(2)
[35217607788] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[35219881191] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Read resp_port=5 payload_len=20
[35221318440] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[35223600390] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Read payload_len=20
[35225698167] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
[35227835973] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: read mac
p[35229861975] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[35231887020] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Read resp_port=5 status=0 resp_payload_len=22
[35234237577] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Read resp_port=5
[35236618824] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[35238532692] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 23 bytes to port PortId(3)
[35241763128] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 23 bytes from TID 14
[35244149556] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Read resp_port=5
[35247617493] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 15 bytes from TID 14
[35250759423] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 15 bytes from TID 13
[35252649696] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 15 bytes from port PortId(2)
[35256471690] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Close resp_port=5 payload_len=8
[35258357673] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Close payload_len=8
[35260077138] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Close resp_port=5 status=0 resp_payload_len=0
[35262136140] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Close resp_port=5
[35264191512] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[35266042779] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 1 bytes to port PortId(3)
[35269315554] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Close resp_port=5
[35270937372] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 1 bytes from TID 14
[35277789228] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/dev/net/virtio0/mtu' tid=14
[35281438500] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 14 bytes from TID 14
[35283452160] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 14 bytes from TID 13
[35285164332] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 14 bytes from port PortId(2)
[35288760573] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Lookup resp_port=5 payload_len=7
[35290817727] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Lookup payload_len=7
[35292683184] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'mtu'
[35294730174] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'mtu' -> handle=4
[35297299059] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Lookup resp_port=5 status=0 resp_payload_len=8
[35300143329] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Lookup resp_port=5
[35303303706] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[35305824246] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 9 bytes to port PortId(3)
[35308316109] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 9 bytes from TID 14
[35311127049] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Lookup resp_port=5
[35314273434] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[35318095626] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[35320307517] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[35324804031] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[35326733838] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[35328740997] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[35330901111] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[35332994103] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[35334782802] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[35337182133] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[35339342874] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[35341679670] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 27 bytes from TID 14
[35345547963] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 27 bytes from TID 13
[35347139850] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 27 bytes from port PortId(2)
[35351385828] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Read resp_port=5 payload_len=20
[35353546701] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Read payload_len=20
[35355249270] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: read mtu
[35357223759] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Read resp_port=5 status=0 resp_payload_len=9
[35359289262] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Read resp_port=5
[35361405618] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[35363698128] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 10 bytes to port PortId(3)
[35365635756] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 10 bytes from TID 14
[35368128774] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Read resp_port=5
[35371316871] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 15 bytes from TID 14
[35390591049] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[35393211711] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[35395809405] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[35399427855] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
s[35402608494] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[35410436358] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 15 bytes from TID 13
[35412162060] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 15 bytes from port PortId(2)
[35416583235] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Close resp_port=5 payload_len=8
[35418741204] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Close payload_len=8
[35420777040] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Close resp_port=5 status=0 resp_payload_len=0
[35422884486] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Close resp_port=5
[35425053477] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[35426973714] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 1 bytes to port PortId(3)
[35429561376] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 1 bytes from TID 14
[35431366377] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Close resp_port=5
[35434599948] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/dev/net/virtio0/status' tid=14
[35438026932] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 17 bytes from TID 14
[35439474939] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 17 bytes from TID 13
[35441188299] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 17 bytes from port PortId(2)
[35445197403] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Lookup resp_port=5 payload_len=10
[35447394378] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Lookup payload_len=10
[35449181889] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'status'
[35451036687] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'status' -> handle=2
[35453049522] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Lookup resp_port=5 status=0 resp_payload_len=8
[35455385658] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Lookup resp_port=5
[35457615171] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[35459947578] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 9 bytes to port PortId(3)
[35462453763] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 9 bytes from TID 14
[35465519166] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Lookup resp_port=5
[35468434782] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[35474144871] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[35476779162] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[35483285838] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[35486328504] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[35489680908] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[35492819373] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[35496283614] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[35499434190] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[35501940078] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[35503416432] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[35505711813] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 27 bytes from TID 14
[35559069513] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[35562135972] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[35564868801] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[35567494083] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
 [35570595093] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[35725759608] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[35727652026] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[35729461218] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[35731415940] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
e[35734028088] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[35893586850] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[35896413168] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[35899219917] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[35902050063] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
n[35905351878] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[35983138950] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=0 apic_id=0 vector=0x30
[35991928698] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 27 bytes from TID 13
[35993899821] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 27 bytes from port PortId(2)
[35997778872] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Read resp_port=5 payload_len=20
[35999957664] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Read payload_len=20
[36001721052] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: read status
[36003708840] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Read resp_port=5 status=0 resp_payload_len=56
[36005676267] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Read resp_port=5
[36008069493] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[36010579506] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 57 bytes to port PortId(3)
[36012548352] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 57 bytes from TID 14
[36014987019] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Read resp_port=5
[36017357409] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 15 bytes from TID 14
[36021576096] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 15 bytes from TID 13
[36023422446] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 15 bytes from port PortId(2)
[36027379806] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Close resp_port=5 payload_len=8
[36029229852] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Close payload_len=8
[36031008684] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Close resp_port=5 status=0 resp_payload_len=0
[36032891136] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Close resp_port=5
[36035033100] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[36037148895] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 1 bytes to port PortId(3)
[36039769062] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 1 bytes from TID 14
[36041641746] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Close resp_port=5
[36046375299] [[34mDEBUG[0m] [netd] [CPU2] NETD: Opened VFS NIC device at /dev/net/virtio0 (rx=4, tx=5, events=6, mtu=1500, link=up)
[36052076676] [[34mDEBUG[0m] [netd] [CPU2] NETD: Driver online at /dev/net/virtio0 �� MAC 52:54:00:12:34:56  MTU 1500
[36062525169] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=65536 port=0xffffffffb00dc350
[36064688748] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[36067356204] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(5)
[36069332739] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[36072010161] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(5) mode=Write
[36074796351] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[36077506179] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=4 port_id=PortId(5) mode=Read
[36080122947] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
.[36083041698] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[36088330839] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=262400 port=0xffffffffb00dd0d0
[36090784290] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(6)
[36093182730] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(6) mode=Write
[36100381515] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] vfs: mounted userland provider at /net (flags: 0x0)
[36104112363] [[34mDEBUG[0m] [netd::vfs_provider] [CPU2] NetVfsProvider: mounted at /net (port w=3 r=4)
[36107491860] [[34mDEBUG[0m] [netd] [CPU2] NETD: Running DHCP...
[36110766351] [[34mDEBUG[0m] [netd::dhcp] [CPU2] DHCP: Starting discovery...
[36114522807] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[36121562664] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[36123506661] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[36127592325] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[36129551502] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[36131746794] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[36133822725] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[36136112562] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[36138019863] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[36140926932] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[36142515816] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[36149742057] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[36156162636] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[36157941600] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[36161847447] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[36163820484] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[36165872721] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[36168045408] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[36170276901] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[36172204563] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[36174618810] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[36176946663] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[36187936884] [[35mTRACE[0m] [netd::vfs_device] [CPU2] VFS_NIC: sending frame len=304 (IPv4 0.0.0.0 -> 255.255.255.255, proto=17)
[36191181378] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=14 fd=5 len=308
[36193653540] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=14 fd=5 starting copyin
[36195575163] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=14 fd=5 copyin ok
[36197573808] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[36231358218] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[36233909283] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[36236377914] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[36238879743] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
w[36241754373] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[36400652376] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[36403627821] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[36405364710] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[36407501526] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
i[36409438164] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[36567415104] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[36569389296] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[36571388337] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[36573342927] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
k[36577510431] [[35mTRACE[0m] [bran::arch::x86_64] [CPU3] SMP: send_ipi cpu_index=0 apic_id=0 vector=0x30
[36579566628] [[35mTRACE[0m] [bran::arch::x86_64] [CPU3] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[36583011432] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[36586167618] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/services/storage' tid=8
[36595594233] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/services/storage/atapi2' tid=8
[36604260759] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=65536 port=0xffffffffb00dd430
[36606082722] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(7)
[36607841952] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=5 port_id=PortId(7) mode=Write
[36609793935] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=6 port_id=PortId(7) mode=Read
[36612472512] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 21 bytes from TID 8
[36740537625] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[36743234319] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[36745712223] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[36748198311] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
i[36751174086] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[36909280122] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[36913414758] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[36916510851] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[36920037759] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[36922235328] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[36924934002] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[36927579777] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[36930474009] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
[36934559508] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
p[36937778658] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[36939975006] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[36943730010] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[36946572201] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[36952371753] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[36957586215] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[36965217564] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 15 bytes from TID 14
[36969573102] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 15 bytes from TID 13
[36971717805] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 15 bytes from port PortId(2)
[36976297215] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Stat resp_port=5 payload_len=8
[36978587646] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Stat payload_len=8
[36980783598] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Stat resp_port=5 status=0 resp_payload_len=20
[36983646447] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Stat resp_port=5
[36986709837] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[36989445570] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 21 bytes to port PortId(3)
[36992049996] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 21 bytes from TID 14
[36995451471] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Stat resp_port=5
[37000284783] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 335 bytes from TID 14
[37006774728] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 335 bytes from TID 13
[37010079777] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 335 bytes from port PortId(2)
[37015662717] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Write resp_port=5 payload_len=328
[37018568202] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Write payload_len=328
[37021531602] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: write tx len=308
[37024681947] [[35mTRACE[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: TX 304 bytes
[37027043229] [[35mTRACE[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: TX total_len=314
[37029346266] [[35mTRACE[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: TX queue add begin
[37031675637] [[35mTRACE[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: TX queue add end
[37033902180] [[35mTRACE[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: TX notify begin
[37043047602] [[35mTRACE[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: TX notify end
[37046423997] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: TX complete after 0 iterations
[37048943547] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Write resp_port=5 status=0 resp_payload_len=4
[37053969942] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Write resp_port=5
[37063960494] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[37066397049] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[37069127040] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[37071876270] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Write resp_port=5
[37075107597] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 15 bytes from TID 14
[37077882996] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[37081930050] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[37084277901] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: RX frame! desc=0 len=600
[37086585360] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[37088715081] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
e[37092602184] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[37098221028] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 15 bytes from TID 13
[37099863570] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 15 bytes from port PortId(2)
[37104550461] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Stat resp_port=5 payload_len=8
[37107751065] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Stat payload_len=8
[37109564679] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Stat resp_port=5 status=0 resp_payload_len=20
[37111890684] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Stat resp_port=5
[37114380732] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[37116989877] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 21 bytes to port PortId(3)
[37120135767] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 21 bytes from TID 14
[37122282285] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Stat resp_port=5
[37125784377] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[37130046657] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[37132371837] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[37136627814] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[37139019390] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[37140956655] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[37143544416] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[37146243024] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[37148075514] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[37150494579] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[37152340434] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[37156038645] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[37158055341] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[37159867932] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[37164114636] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[37166554920] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[37168216932] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[37170246927] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[37173074565] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[37174964838] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[37177267149] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[37179656217] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[37186390329] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 27 bytes from TID 14
[37191828729] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 27 bytes from TID 13
[37193712303] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 27 bytes from port PortId(2)
[37197238089] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Read resp_port=5 payload_len=20
[37199468130] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Read payload_len=20
[37201638144] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: read rx queued=1
[37204091925] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Read resp_port=5 status=0 resp_payload_len=598
[37206400044] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Read resp_port=5
[37208982888] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[37210844055] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 599 bytes to port PortId(3)
[37212993576] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 599 bytes from TID 14
[37214520321] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Read resp_port=5
[37218602982] [[35mTRACE[0m] [netd::vfs_device] [CPU2] VfsNicDevice: RX vfs_read(fd=4) returned 594 bytes
[37220779662] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[37222289379] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[37224062964] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[37227577794] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[37229828328] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[37231639863] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[37233650322] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[37235916234] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[37237848186] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[37240006584] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[37242360243] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[37243868607] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[37246267443] [[35mTRACE[0m] [netd::vfs_device] [CPU2] VFS_NIC: received frame len=590 (IPv4 10.0.2.2 -> 255.255.255.255, proto=17)
[37248608199] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[37250359014] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[37252275720] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
d[37255018119] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[37256414283] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[37259908653] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[37261958910] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[37266959004] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[37269956724] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[37272721563] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[37276470990] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[37280111781] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[37282928892] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[37285662216] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[37287828534] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[37290558360] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[37297292505] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[37299751962] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[37305161718] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[37309750269] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[37312983510] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[37316308260] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[37318755309] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[37320735375] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[37323208659] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[37324755369] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[37331327088] [[35mTRACE[0m] [netd::vfs_device] [CPU2] VFS_NIC: sending frame len=316 (IPv4 0.0.0.0 -> 255.255.255.255, proto=17)
[37334606496] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=14 fd=5 len=320
[37337420670] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=14 fd=5 starting copyin
[37340312559] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_write: tid=14 fd=5 copyin ok
[37343943318] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[37413717231] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[37415729505] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[37417618392] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[37419557340] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
i[37421703330] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[37583515068] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[37585679934] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[37587752697] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[37589191299] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[37591072167] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[37592870865] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
a[37594849314] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[37596477204] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[37598296593] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[37599991044] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[37602320019] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[37604480595] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[37606074363] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[37608106272] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[37609389015] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[37611188736] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 15 bytes from TID 14
[37615504014] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 15 bytes from TID 13
[37616958192] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 15 bytes from port PortId(2)
[37621874697] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Stat resp_port=5 payload_len=8
[37624925118] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Stat payload_len=8
[37627605180] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Stat resp_port=5 status=0 resp_payload_len=20
[37630769814] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Stat resp_port=5
[37633787796] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[37636776672] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 21 bytes to port PortId(3)
[37639869498] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 21 bytes from TID 14
[37642546194] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Stat resp_port=5
[37644661230] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 347 bytes from TID 14
[37648779894] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 347 bytes from TID 13
[37651188630] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 347 bytes from port PortId(2)
[37657407414] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Write resp_port=5 payload_len=340
[37660559541] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Write payload_len=340
[37663345896] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: write tx len=320
[37666269828] [[35mTRACE[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: TX 316 bytes
[37668799377] [[35mTRACE[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: TX total_len=326
[37670425089] [[35mTRACE[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: TX queue add begin
[37672060965] [[35mTRACE[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: TX queue add end
[37673893026] [[35mTRACE[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: TX notify begin
[37675699875] [[35mTRACE[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: TX notify end
[37677282423] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: TX complete after 0 iterations
[37679269452] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Write resp_port=5 status=0 resp_payload_len=4
[37681105473] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Write resp_port=5
[37684470384] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[37686436689] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[37688912844] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Write resp_port=5
[37690425036] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[37692614289] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: RX frame! desc=1 len=600
[37694999529] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 15 bytes from TID 14
[37698172149] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 15 bytes from TID 13
[37699771989] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 15 bytes from port PortId(2)
[37703072583] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Stat resp_port=5 payload_len=8
[37705049811] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Stat payload_len=8
[37706718720] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Stat resp_port=5 status=0 resp_payload_len=20
[37708825044] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Stat resp_port=5
[37711570611] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[37713404916] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 21 bytes to port PortId(3)
[37715644923] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 21 bytes from TID 14
[37717966242] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Stat resp_port=5
[37721181597] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[37723630164] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[37725135855] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[37728285441] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[37730160666] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[37731832050] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[37733670843] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[37735731165] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[37737849270] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[37740035124] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[37742204808] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[37746466692] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[37748908989] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[37750420455] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[37752918159] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[37754626998] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[37757274687] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[37760312040] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
.[37764607485] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[37785818961] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[37787964093] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[37789778598] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[37791837600] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[37794905181] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[37796853336] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[37798877160] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[37800449742] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[37805680275] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 27 bytes from TID 14
[37807152141] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 27 bytes from TID 13
[37808892594] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 27 bytes from port PortId(2)
[37812340731] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Read resp_port=5 payload_len=20
[37814905029] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Read payload_len=20
[37816925883] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: read rx queued=1
[37820585517] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Read resp_port=5 status=0 resp_payload_len=598
[37823499681] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Read resp_port=5
[37826225316] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[37828559439] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 599 bytes to port PortId(3)
[37830988272] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Read resp_port=5
[37832613093] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 599 bytes from TID 14
[37839176001] [[35mTRACE[0m] [netd::vfs_device] [CPU2] VfsNicDevice: RX vfs_read(fd=4) returned 594 bytes
[37842044658] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[37918362867] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[37920314916] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[37922032170] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[37923883272] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
o[37925925213] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[38087439324] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[38089481133] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[38091151197] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[38092847298] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
r[38095595076] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[38098437135] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[38100988365] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[38105805375] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[38108997333] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[38111915424] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[38115165429] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[38118164337] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[38120000160] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[38122628643] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[38124888483] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[38127654807] [[35mTRACE[0m] [netd::vfs_device] [CPU2] VFS_NIC: received frame len=590 (IPv4 10.0.2.2 -> 255.255.255.255, proto=17)
[38133347703] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[38136373011] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[38138008788] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[38141251929] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[38143498635] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[38145401811] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[38147404053] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[38149945779] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[38151782460] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[38154138858] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[38155747245] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[38160960453] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[38163025989] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[38164629063] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[38169197418] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[38171259324] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[38173326378] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[38175520548] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[38178080688] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[38180674026] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[38183191596] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[38185351281] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[38188655901] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[38255398401] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[38257229670] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[38259407340] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[38262047439] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
g[38264818020] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[38423504922] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[38425781625] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[38427535707] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[38429236692] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
 [38431350078] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[38433976647] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=0 apic_id=0 vector=0x30
[38443540905] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[38446009635] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[38450480937] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[38453377578] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[38455911120] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[38458678830] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[38461560489] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[38464117989] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[38466678063] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[38469028884] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[38471538864] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[38591464362] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[38593311108] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[38595081063] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[38596724463] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
/[38598571473] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[38674515627] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[38678278155] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[38684123544] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[38687307252] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[38690256759] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[38692881645] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[38695308960] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[38698138578] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[38701200813] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[38702827317] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[38706296541] [[34mDEBUG[0m] [netd::dhcp] [CPU2] DHCP: Configuration received
[38708667525] [[34mDEBUG[0m] [netd] [CPU2] NETD: DHCP — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[38710656171] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
[38712275052] [[34mDEBUG[0m] [netd] [CPU2] NETD: entering VFS service loop
[38713830375] [[34mDEBUG[0m] [netd] [CPU2] NETD: creating SocketApi...
[38715629469] [[34mDEBUG[0m] [netd] [CPU2] NETD: allocating sockets_storage...
[38717493969] [[34mDEBUG[0m] [netd] [CPU2] NETD: pushing socket storage 0...
[38719182447] [[34mDEBUG[0m] [netd] [CPU2] NETD: pushing socket storage 64...
[38720944647] [[34mDEBUG[0m] [netd] [CPU2] NETD: pushing socket storage 128...
[38722630947] [[34mDEBUG[0m] [netd] [CPU2] NETD: pushing socket storage 192...
[38724429546] [[34mDEBUG[0m] [netd] [CPU2] NETD: creating SocketSet...
[38726918406] [[34mDEBUG[0m] [netd] [CPU2] NETD: getting link state...
[38728530324] [[34mDEBUG[0m] [netd] [CPU2] NETD: scanning NIC units...
[38730556887] [[34mDEBUG[0m] [netd] [CPU2] NETD: scanning for registered NIC units...
[38732433960] [[35mTRACE[0m] [netd] [CPU2] NETD: checking nic unit 0 at /dev/net/virtio0/rx
[38734176756] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=14
[38737378218] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 13 bytes from TID 14
[38743372932] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 13 bytes from TID 13
[38745771372] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 13 bytes from port PortId(2)
[38750150274] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Lookup resp_port=5 payload_len=6
[38752872147] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Lookup payload_len=6
[38755473867] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'rx'
[38757496074] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[38759808087] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: lookup 'rx' -> handle=5
[38761702023] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[38764312752] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Lookup resp_port=5 status=0 resp_payload_len=8
[38767024989] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[38769344691] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Lookup resp_port=5
[38771428872] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
h[38774508630] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=0 apic_id=0 vector=0x30
[38776967691] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[38779368441] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[38782232874] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 9 bytes to port PortId(3)
[38785118724] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 9 bytes from TID 14
[38786910327] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Lookup resp_port=5
[38790195345] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 15 bytes from TID 14
[38928931602] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[38930799699] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[38932658292] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[38934696603] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
t[38936520678] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[39097902591] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[39099946842] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[39101525727] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[39103207176] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
t[39105377718] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[39267308619] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[39270004851] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[39272445894] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[39274922049] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
p[39277699560] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[39286403277] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 15 bytes from TID 13
[39288033312] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 15 bytes from port PortId(2)
[39291459933] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Close resp_port=5 payload_len=8
[39293324169] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Close payload_len=8
[39295069539] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Close resp_port=5 status=0 resp_payload_len=0
[39297929220] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Close resp_port=5
[39301201896] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[39303782001] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 1 bytes to port PortId(3)
[39306242514] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 1 bytes from TID 14
[39308500473] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Close resp_port=5
[39312088695] [[35mTRACE[0m] [netd] [CPU2] NETD: checking nic unit 1 at /dev/net/virtio1/rx
[39314581383] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/dev/net/virtio1/rx' tid=14
[39325325292] [[35mTRACE[0m] [netd] [CPU2] NETD: checking nic unit 2 at /dev/net/virtio2/rx
[39327720135] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/dev/net/virtio2/rx' tid=14
[39337804605] [[35mTRACE[0m] [netd] [CPU2] NETD: checking nic unit 3 at /dev/net/virtio3/rx
[39340055172] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/dev/net/virtio3/rx' tid=14
[39349185084] [[35mTRACE[0m] [netd] [CPU2] NETD: checking nic unit 4 at /dev/net/virtio4/rx
[39351177195] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/dev/net/virtio4/rx' tid=14
[39357189003] [[35mTRACE[0m] [netd] [CPU2] NETD: checking nic unit 5 at /dev/net/virtio5/rx
[39359583549] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/dev/net/virtio5/rx' tid=14
[39365250771] [[35mTRACE[0m] [netd] [CPU2] NETD: checking nic unit 6 at /dev/net/virtio6/rx
[39367041813] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/dev/net/virtio6/rx' tid=14
[39373153578] [[35mTRACE[0m] [netd] [CPU2] NETD: checking nic unit 7 at /dev/net/virtio7/rx
[39376029000] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/dev/net/virtio7/rx' tid=14
[39382534323] [[35mTRACE[0m] [netd] [CPU2] NETD: checking nic unit 8 at /dev/net/virtio8/rx
[39385377075] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/dev/net/virtio8/rx' tid=14
[39391807950] [[35mTRACE[0m] [netd] [CPU2] NETD: checking nic unit 9 at /dev/net/virtio9/rx
[39393947373] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/dev/net/virtio9/rx' tid=14
[39399342708] [[35mTRACE[0m] [netd] [CPU2] NETD: checking nic unit 10 at /dev/net/virtio10/rx
[39401052306] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/dev/net/virtio10/rx' tid=14
[39406957359] [[35mTRACE[0m] [netd] [CPU2] NETD: checking nic unit 11 at /dev/net/virtio11/rx
[39408606534] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/dev/net/virtio11/rx' tid=14
[39414716781] [[35mTRACE[0m] [netd] [CPU2] NETD: checking nic unit 12 at /dev/net/virtio12/rx
[39416690115] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/dev/net/virtio12/rx' tid=14
[39421970511] [[35mTRACE[0m] [netd] [CPU2] NETD: checking nic unit 13 at /dev/net/virtio13/rx
[39423842964] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/dev/net/virtio13/rx' tid=14
[39430538664] [[35mTRACE[0m] [netd] [CPU2] NETD: checking nic unit 14 at /dev/net/virtio14/rx
[39432780024] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[39434434842] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/dev/net/virtio14/rx' tid=14
[39436968417] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[39439158231] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[39441677154] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
[39443457801] [[35mTRACE[0m] [netd] [CPU2] NETD: checking nic unit 15 at /dev/net/virtio15/rx
s[39445794168] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[39447136410] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/dev/net/virtio15/rx' tid=14
[39454936125] [[34mDEBUG[0m] [netd] [CPU2] NETD: NIC scan complete: [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]
[39458748384] [[34mDEBUG[0m] [netd] [CPU2] NETD: NIC units scanned.
[39460863288] [[34mDEBUG[0m] [netd] [CPU2] NETD: bridging request port to fd...
[39463436892] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=4 -> fd=7 node=0xffffffffb006f670 port=0xffffffffb00dc350
[39467077914] [[34mDEBUG[0m] [netd] [CPU2] NETD: request fd=7
[39469154406] [[34mDEBUG[0m] [netd] [CPU2] NETD: setting up /dev/net watch...
[39474617325] [[34mDEBUG[0m] [netd] [CPU2] NETD: watch fd=Some(8)
[39476666427] [[35mTRACE[0m] [netd] [CPU2] NETD: main loop iteration
[39479410344] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[39482217555] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[39484698297] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[39491008161] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[39494213154] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[39497241630] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[39499844208] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[39504220833] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[39507572445] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[39511154991] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[39514520133] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[39517952100] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[39522376575] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[39524351427] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[39528815700] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[39531224634] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[39533690823] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[39535784178] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[39538113780] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[39539966103] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[39542086353] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[39544311774] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[39553984866] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=0 apic_id=0 vector=0x30
[39555637704] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[39602964390] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[39605033655] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[39607050219] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[39609189477] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
/[39611177727] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[39661783821] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[39663616575] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[39667356135] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[39669361974] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[39671278119] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[39673446384] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[39675710679] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[39677534259] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[39679968966] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[39682136802] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[39685499172] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[39689248104] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[39690901272] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[39695554272] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[39697748211] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[39699893937] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[39702111537] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[39704461533] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[39706369527] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[39708647121] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[39711097173] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[39771365271] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[39773107275] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[39774714837] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[39776669922] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
w[39778760769] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[39937135524] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[39940042428] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[39943010976] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[39945980613] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
p[39949106505] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[40107375594] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[40110374766] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[40113160098] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[40116001068] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1

[40127076297] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='mount'
[40129711776] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/bin/mount' tid=6
[40132505292] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=0 apic_id=0 vector=0x30
[40134088104] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/mount' at index 22
[40135634187] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[40139873334] [[35mTRACE[0m] [netd] [CPU2] NETD: main loop iteration
[40142468322] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[40144533627] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[40146275697] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[40149174912] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/mount' at index 22
[40151276352] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[40153689609] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[40155766794] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[40157535099] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[40159854966] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[40161580470] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[40163673792] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: Loading module 'mount' (len=57752, base=0x200000)
[40166691675] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[40168229904] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[40170064143] [[35mTRACE[0m] [kernel::task::loader] [CPU3]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[40175249136] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[40176814557] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[40179983778] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[40182385452] [[35mTRACE[0m] [kernel::task::loader] [CPU3] Segment: vaddr=200000 exec=true
[40184227083] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[40187327961] [[35mTRACE[0m] [kernel::task::loader] [CPU3]   COPIED at 0x201420: [7c, 24, 28, 49, 8b, 17, 49, 8b]
[40190521701] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[40193095536] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[40195065636] [[35mTRACE[0m] [kernel::task::loader] [CPU3] Segment: vaddr=208000 exec=false
[40197336729] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[40199588847] [[35mTRACE[0m] [kernel::task::loader] [CPU3] Segment: vaddr=209000 exec=false
[40201207662] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[40203063021] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: TLS block: tp=0x20a000 filesz=0 memsz=0 align=1
[40205498157] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[40207726614] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[40210526796] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[40212788616] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[40214894874] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Starting Phase 1 for /bin/mount
[40217503887] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb01dcc60 arg=0xffffffffb00df780
[40221187347] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Phase 1 complete, ID=15, applying inserts
[40222935984] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[40225224105] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[40226720853] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=15
[40228281456] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[40229644158] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[40232143908] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[40238184921] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Phase 2 complete, waking task 15
[40240696518] [[35mTRACE[0m] [kernel::sched::blocking] [CPU3] WAKE_TASK: ID=15 taking SCHEDULER lock
[40243599099] [[35mTRACE[0m] [kernel::sched::blocking] [CPU3] WAKE_TASK: ID=15 wake_task_locked returned IPI_CPU=None
[40246254972] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Task 15 woken, restoring IRQs
[40248577248] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Done for /bin/mount
[40251931104] [[35mTRACE[0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/mount' TID=15 PID=15
[40256869719] [[34mDEBUG[0m] [sh] [CPU3] sh: spawned '/bin/mount' pid=15 idx=0 background=false pending_pgid=0
[40260704583] [[34mDEBUG[0m] [sh] [CPU3] sh: setpgid leader pid=15 -> pgid=15
[40265068998] [[34mDEBUG[0m] [sh] [CPU3] sh: foreground handoff start job_id=1 pgid=15 shell_pgid=5 cmd='mount -t https en.wikipedia.org /https/wp'
[40269830601] [[34mDEBUG[0m] [sh] [CPU3] sh: foreground handoff done job_id=1 pgid=15 cmd='mount -t https en.wikipedia.org /https/wp'
[40272601875] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=6
[40274881053] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[40277236857] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[40278856365] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[40280889825] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[40282863852] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=6
[?25l[40284928992] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[40286696043] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running → Runnable (cpu=3, next=15)
[40288562193] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[40290411612] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[40292846913] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[40295387055] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=15 entry_pc=200000 user_sp=800000
[40297090581] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[40298636994] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=15 buf_len=0
[40300444140] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[40303214193] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=15 buf_len=100
[40305049950] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[40308216630] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[40310337210] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[40312609260] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[40342046811] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[40345212303] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[40350201573] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[40352192529] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[40353629382] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/bin/https' tid=15
[40356193383] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[40358597895] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[40360867470] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[40362717780] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[40364576109] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/bin/httpsd' tid=15
[40367346327] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[40368971709] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[40370784168] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/httpsd' at index 51
[40378435218] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/httpsd' at index 51
[40381126599] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[40385708220] [[35mTRACE[0m] [netd] [CPU2] NETD: main loop iteration
[40388551071] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[40391679108] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[40393151634] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[40396536048] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[40399300491] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[40401863172] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[40439397273] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[40442676813] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[40445689614] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[40448397099] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[40449951696] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[40452112701] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[40456120683] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[40457670528] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: Loading module 'httpsd' (len=526160, base=0x200000)
[40460250666] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[40462556937] [[35mTRACE[0m] [kernel::task::loader] [CPU3]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[40466683752] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[40469116083] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[40471302927] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[40473460302] [[35mTRACE[0m] [kernel::task::loader] [CPU3] Segment: vaddr=200000 exec=true
[40476030540] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[40478047665] [[35mTRACE[0m] [kernel::task::loader] [CPU3]   COPIED at 0x201420: [3f, 06, 00, 48, 89, 44, 24, 08]
[40480912263] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[40483739604] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[40485613938] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[40488021783] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[40489814904] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[40500612240] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[40511871114] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[40514019645] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[40517800323] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[40519695876] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[40522346469] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[40525431276] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[40528884990] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[40531467438] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[40534003884] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[40535641905] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[40541658597] [[35mTRACE[0m] [kernel::task::loader] [CPU3] Segment: vaddr=25b000 exec=false
[40544005128] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[40548707628] [[35mTRACE[0m] [kernel::task::loader] [CPU3] Segment: vaddr=263000 exec=false
[40550495601] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[40553041353] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[40556187870] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: TLS block: tp=0x266000 filesz=0 memsz=0 align=1
[40558737153] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[40561553208] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[40564165356] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[40566365763] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Starting Phase 1 for /bin/httpsd
[40568356257] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response begin op=Poll resp_port=5
[40571020842] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb026d3c0 arg=0xffffffffb019c0a0
[40572989424] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[40575632031] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Phase 1 complete, ID=16, applying inserts
[40578172371] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_send_all: wrote 5 bytes to port PortId(3)
[40580764983] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[40582539063] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 5 bytes from TID 14
[40584703071] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: send_response end op=Poll resp_port=5
[40586226714] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=16
[40588516122] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[40590463122] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[40593843642] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[40602734139] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Phase 2 complete, waking task 16
[40604627250] [[35mTRACE[0m] [bran::arch::x86_64] [CPU3] SMP: send_ipi cpu_index=1 apic_id=1 vector=0x30
[40606410306] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Task 16 woken, restoring IRQs
[40608269988] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SPAWN_FROM_PATH: Done for /bin/httpsd
[40610479008] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[40612257345] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=16 entry_pc=200000 user_sp=800000
[40614635028] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=16 buf_len=0
[40617145998] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=16 buf_len=100
[40618960668] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[40620876813] [[35mTRACE[0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/httpsd' TID=16 PID=16
[40624240371] [[35mTRACE[0m] [netd] [CPU2] NETD: main loop iteration
[40626309438] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/https/wp' tid=15
[40629192021] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port written 19 bytes from TID 14
[40636212276] [[34mDEBUG[0m] [kernel::sched] [CPU2] SCHED: CPU 2 stole task 10 (prio 1) from CPU 1 (depth 2)
[40639012920] [[35mTRACE[0m] [kernel::sched] [CPU3] waitpid: blocking parent_pid=6 parent_tid=6 target_pid=-1 flags=0x6 registered_children=1
[40643663445] [[34mDEBUG[0m] [kernel::sched::blocking] [CPU3] SCHED[TID6]: blocked
[40671720804] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=32768 port=0xffffffffb01a6010
[40673803863] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(8)
[40676075682] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=1 port_id=PortId(8) mode=Write
[40678166298] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=2 port_id=PortId(8) mode=Read
[40686847311] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=262400 port=0xffffffffb01a6d90
[40689587928] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(9)
[40692094113] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=3 port_id=PortId(9) mode=Write
[40695880797] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] vfs: mounted userland provider at /run/httpsd/cache (flags: 0x0)
[40710238140] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Task 17 (user thread) assigned to CPU 2
[40714032843] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[40715592720] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=17
[40716964332] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[40718829723] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Sending post-unlock Resched IPI to CPU 2 for task 17
[40721437779] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[40725895650] [[34mDEBUG[0m] [httpsd] [CPU1] httpsd: cache mount started at /run/httpsd/cache
[40730436615] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=32768 port=0xffffffffb01a7f50
[40732242342] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(10)
[40733840631] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=4 port_id=PortId(10) mode=Write
[40735859043] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=5 port_id=PortId(10) mode=Read
[40743476202] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=262400 port=0xffffffffb01a8d30
[40745119371] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(11)
[40746750792] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=6 port_id=PortId(11) mode=Write
[40749190944] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] vfs: mounted userland provider at /https/wp (flags: 0x0)
[40752759201] [[34mDEBUG[0m] [httpsd] [CPU1] httpsd: mounted at /https/wp
[40755673002] [[32mINFO [0m] [httpsd] [CPU1] httpsd: entering main RPC loop for /https/wp
[40767217722] [[35mTRACE[0m] [kernel::ipc::port] [CPU1] PORT: Port read 19 bytes from TID 13
[40769597055] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU1] sys_port_recv: read 19 bytes from port PortId(2)
[40774131387] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch begin op=Poll resp_port=5 payload_len=12
[40776856362] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU1] VIRTIO_NETD: rpc op=Poll payload_len=12
[40779302355] [[35mTRACE[0m] [virtio_netd] [CPU1] VIRTIO_NETD: dispatch end op=Poll resp_port=5 status=0 resp_payload_len=4
[4078116824
```
</details>
