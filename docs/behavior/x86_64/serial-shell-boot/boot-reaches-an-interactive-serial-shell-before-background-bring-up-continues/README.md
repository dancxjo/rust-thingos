# ❌ Scenario: Boot reaches an interactive serial shell before background bring-up continues

> Last run: 2026-04-22 08:47:12

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8824ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 2208ms | - [📜](./02/serial.log) - |
| 3 | And I type "echo shell-ready" on the serial console | ✅ | 1881ms | - [📜](./03/serial.log) - |
| 4 | Then the serial output should contain "shell-ready" | ❌ | 301083ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[27317989215] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] enter
[27328928187] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] phys_memory_map ok
[27333904191] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] modules ok
[27335609103] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] phys_to_virt_offset ok
[27336840795] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] memory map logging done
[27366046125] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] boot_frame_alloc init ok
[27664653522] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] frame allocator build ok
[27666826077] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] frame allocator log ok
[27668869998] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] FRAME_ALLOCATOR init ok
[27671022522] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] tasking init ok
[27676902759] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[27678989052] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] enter
[27681089799] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] set expand hook
[27683130816] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] expand hook ok
[27685151010] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock begin
[27687568623] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock ok
[27689637558] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] reserve_region begin
[27727135953] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] reserve_region ok
[27729318903] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] inner allocator init begin
[27733911942] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] inner allocator init ok
[27736199502] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] heap top store ok
[27738310875] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] init done
[27746438808] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] set_boot_fb begin
[27748662612] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] devfs: set_boot_fb width=1920 height=1080 pitch=7680 resource_id=0xfb000000
[27752532258] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] set_boot_fb ok
[27754346730] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs set_boot_fb elapsed_ticks=7410744 elapsed_us=3705
[27761819250] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs FbNode::new elapsed_ticks=5735268 elapsed_us=2867
[27764104599] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] register begin
[27769500165] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] register ok
[27770809011] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs register fb0 elapsed_ticks=6662733 elapsed_us=3331
[27778292949] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[27791762196] [[34mDEBUG[0m] [kernel] [CPU0] Seeding entropy pool...
[27801271509] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed begin
[27803482839] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] fill_entropy done
[27807854118] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] add_sample(timer) ok
[27809077560] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] mark_seeded(timer) ok
[27810199692] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[27811537578] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[27812825238] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed done
[27814559949] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] entropy::seed_from_hardware elapsed_ticks=12837066 elapsed_us=6418
[27820783188] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[27830324181] [[35mTRACE[0m] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[27832912239] [[35mTRACE[0m] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[27834534915] [[35mTRACE[0m] [kernel::sched] [CPU0]   Allocating scheduler...
[27839380767] [[35mTRACE[0m] [kernel::sched] [CPU0]   Leaking scheduler...
[27841103268] [[35mTRACE[0m] [kernel::sched] [CPU0]   Initializing boot task...
[27844964004] [[35mTRACE[0m] [kernel::sched] [CPU0]   Creating boot task...
[27858323592] [[35mTRACE[0m] [kernel::sched] [CPU0]   Creating idle tasks...
[27861672696] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80016cc0 kstack_top=0xffffffffb001f980 arg=0x0
[27866275074] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[27899525148] [[35mTRACE[0m] [kernel::sched] [CPU0]   Boot task initialized
[27901368825] [[35mTRACE[0m] [kernel::sched] [CPU0]   Storing scheduler pointer...
[27904173627] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[27913325550] [[34mDEBUG[0m] [kernel] [CPU0] Initializing VFS...
[27918905850] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='set_cmdline' elapsed_ticks=3367584 elapsed_us=1683 total_ticks=5496744 total_us=2748
[27941038092] [[34mDEBUG[0m] [kernel::vfs] [CPU0] VFS: Created /dev/display directory
[27959092722] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted union filesystem at / (root)
[27962284779] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted devfs at /dev
[27963876765] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted procfs at /proc
[27965522541] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted sysfs at /sys
[27971233224] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /tmp
[27972677634] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /run
[27974269389] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /services
[27975640341] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /session
[27977132007] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /data
[27978235527] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='vfs_init' elapsed_ticks=57366705 elapsed_us=28683 total_ticks=65099958 total_us=32549
[27984334884] [[34mDEBUG[0m] [kernel] [CPU0] Scanning PCI bus...
[27988804965] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x29c0 at 00:00.0 class=060000 id=1
[27997060377] [[35mTRACE[0m] [kernel] [CPU0]   BAR0 (MEM32): 0x80000000 (size 0x800000)
[28010096697] [[35mTRACE[0m] [kernel] [CPU0]   BAR2 (MEM64): 0xc000000000 (size 0x4000)
[28014606972] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (MEM32): 0x80882000 (size 0x1000)
[28016057487] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1050 at 00:01.0 class=030000 id=2
[28020775101] [[35mTRACE[0m] [kernel] [CPU0]   BAR0 (I/O):  0x6060 (size 0x20)
[28025763084] [[35mTRACE[0m] [kernel] [CPU0]   BAR1 (MEM32): 0x80881000 (size 0x1000)
[28039701162] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (MEM64): 0xc000004000 (size 0x4000)
[28042018290] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1000 at 00:02.0 class=020000 id=3
[28044665847] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2918 at 00:1f.0 class=060100 id=4
[28048542258] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (I/O):  0x6040 (size 0x20)
[28053212649] [[35mTRACE[0m] [kernel] [CPU0]   BAR5 (MEM32): 0x80880000 (size 0x1000)
[28055376393] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2922 at 00:1f.2 class=010601 id=5
[28060864854] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (I/O):  0x6000 (size 0x40)
[28062856503] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2930 at 00:1f.3 class=0c0500 id=6
[28067455350] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='scan_pci' elapsed_ticks=82150662 elapsed_us=41075 total_ticks=154313016 total_us=77156
[28074161049] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='register_legacy_devices' elapsed_ticks=773784 elapsed_us=386 total_ticks=161033070 total_us=80516
[28080389733] [[34mDEBUG[0m] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[28088999829] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[28090851393] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[28093087968] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[28095105588] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[28100226198] [[34mDEBUG[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[28103213754] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[28105911273] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[28108216983] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[28110479199] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[28112760786] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[28115412666] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[28117966404] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[28120256340] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[28122492783] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[28124740545] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[28126982730] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[28129626294] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[28131720309] [[34mDEBUG[0m] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[28138731918] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[28141899951] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Registers initialized
[28144085739] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[28146512658] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[28149213378] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: All pins masked
[28152117642] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> NMI
[28154193540] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[28155992139] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ4 -> GSI 4 -> 0x24
[28157032431] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Init complete
[28192636197] [[34mDEBUG[0m] [bran::arch::x86_64::ioapic] [CPU0] LAPIC: calibrated 62909300 ticks/sec (delta=629093, ok=true) -> init_cnt=629093
[28196914746] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62909300 ticks/sec), init_cnt=629093 for 100Hz
[28199793105] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='setup_preemption_timer' elapsed_ticks=117314835 elapsed_us=58657 total_ticks=286617540 total_us=143308
[28208358585] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[28210717359] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[28213490613] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[28216982574] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[28249350756] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[28251776091] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[28253866245] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] SMP: CPU 1 initializing preemption timer (vec=32 init_cnt=629093)
[28257329859] [[34mDEBUG[0m] [kernel] [CPU1] SMP: kernel_secondary_entry arg_cpu=1 runtime_cpu=1
[28260068991] [[34mDEBUG[0m] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[28263344241] [[34mDEBUG[0m] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[28267950975] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80016cc0 kstack_top=0xffffffffb0032e40 arg=0x1
[28271031657] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Task 2 assigned to CPU 1
[28276059174] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 2 deferred inserts
[28279134411] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=1
[28281440649] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=2
[28283572647] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] SMP: CPU 2 initializing preemption timer (vec=32 init_cnt=629093)
[28286474172] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[28288399326] [[34mDEBUG[0m] [kernel] [CPU2] SMP: kernel_secondary_entry arg_cpu=2 runtime_cpu=2
[28290655239] [[34mDEBUG[0m] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[28292178486] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[28294548942] [[34mDEBUG[0m] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[28296696417] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: run_scheduler entry on CPU 1
[28298466108] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[28300771290] [[34mDEBUG[0m] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[28303123695] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: bootstrap_cpu start on CPU 1
[28304958528] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU2] INIT KERNEL CTX: entry=0xffffffff80016cc0 kstack_top=0xffffffffb00445a0 arg=0x2
[28308529656] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] SCHED: Task 3 assigned to CPU 2
[28312408179] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Applying 1 deferred inserts
[28314513909] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle thread 2 and is now schedulable
[28317095961] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserting TID=3
[28319388405] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserts applied
[28321421667] [[34mDEBUG[0m] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[28323404538] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: run_scheduler entry on CPU 2
[28325492910] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: bootstrap_cpu start on CPU 2
[28332748818] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[28334741688] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] SMP: CPU 3 initializing preemption timer (vec=32 init_cnt=629093)
[28337627637] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[28339627668] [[34mDEBUG[0m] [kernel] [CPU3] SMP: kernel_secondary_entry arg_cpu=3 runtime_cpu=3
[28341773856] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[28343334888] [[34mDEBUG[0m] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[28351275942] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='start_secondary_cpus' elapsed_ticks=135543441 elapsed_us=67771 total_ticks=430480050 total_us=215240
[28367729511] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle thread 3 and is now schedulable
[28372418811] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Enumerating 108 boot modules...
[28374152763] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[28376358945] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80016cc0 kstack_top=0xffffffffb0055d00 arg=0x3
[28378564929] [[35mTRACE[0m] [kernel] [CPU0]   Module[0]: name='/bin/sprout' cmdline='init' size=85144 bytes
[28382880240] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SCHED: Task 4 assigned to CPU 3
[28388126382] [[35mTRACE[0m] [kernel] [CPU0]   Module[1]: name='/bin/bristle' cmdline='' size=45888 bytes
[28390668537] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[28393135188] [[35mTRACE[0m] [kernel] [CPU0]   Module[2]: name='/drivers/rtc_cmos' cmdline='init' size=39744 bytes
[28396827426] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=4
[28398666846] [[35mTRACE[0m] [kernel] [CPU0]   Module[3]: name='/drivers/ps2_kbd' cmdline='' size=38760 bytes
[28400451255] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[28402071522] [[35mTRACE[0m] [kernel] [CPU0]   Module[4]: name='/bin/sh' cmdline='init' size=121736 bytes
[28405001790] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[28406699937] [[35mTRACE[0m] [kernel] [CPU0]   Module[5]: name='/bin/ls' cmdline='init' size=62192 bytes
[28409005086] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: run_scheduler entry on CPU 3
[28410689472] [[35mTRACE[0m] [kernel] [CPU0]   Module[6]: name='/bin/ln' cmdline='init' size=47624 bytes
[28412882058] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: bootstrap_cpu start on CPU 3
[28414524468] [[35mTRACE[0m] [kernel] [CPU0]   Module[7]: name='/bin/ps' cmdline='init' size=50608 bytes
[28416787080] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle thread 4 and is now schedulable
[28418960526] [[35mTRACE[0m] [kernel] [CPU0]   Module[8]: name='/bin/kill' cmdline='init' size=51552 bytes
[28421655438] [[35mTRACE[0m] [kernel] [CPU0]   Module[9]: name='/bin/killall' cmdline='init' size=67712 bytes
[28425418692] [[35mTRACE[0m] [kernel] [CPU0]   Module[10]: name='/bin/cat' cmdline='init' size=46168 bytes
[28426788390] [[35mTRACE[0m] [kernel] [CPU0]   Module[11]: name='/bin/head' cmdline='init' size=56168 bytes
[28428295566] [[35mTRACE[0m] [kernel] [CPU0]   Module[12]: name='/bin/tail' cmdline='init' size=52048 bytes
[28430466075] [[35mTRACE[0m] [kernel] [CPU0]   Module[13]: name='/bin/top' cmdline='init' size=65544 bytes
[28432667274] [[35mTRACE[0m] [kernel] [CPU0]   Module[14]: name='/bin/wc' cmdline='init' size=56312 bytes
[28434761817] [[35mTRACE[0m] [kernel] [CPU0]   Module[15]: name='/bin/yes' cmdline='init' size=40480 bytes
[28436453925] [[35mTRACE[0m] [kernel] [CPU0]   Module[16]: name='/bin/which' cmdline='init' size=46408 bytes
[28437878403] [[35mTRACE[0m] [kernel] [CPU0]   Module[17]: name='/bin/cp' cmdline='init' size=46376 bytes
[28440310008] [[35mTRACE[0m] [kernel] [CPU0]   Module[18]: name='/bin/mv' cmdline='init' size=47104 bytes
[28441694556] [[35mTRACE[0m] [kernel] [CPU0]   Module[19]: name='/bin/rm' cmdline='init' size=47184 bytes
[28443324426] [[35mTRACE[0m] [kernel] [CPU0]   Module[20]: name='/bin/rmdir' cmdline='init' size=47208 bytes
[28444912485] [[35mTRACE[0m] [kernel] [CPU0]   Module[21]: name='/bin/mkdir' cmdline='init' size=47200 bytes
[28446449823] [[35mTRACE[0m] [kernel] [CPU0]   Module[22]: name='/bin/mount' cmdline='init' size=57752 bytes
[28447910106] [[35mTRACE[0m] [kernel] [CPU0]   Module[23]: name='/bin/echo' cmdline='init' size=40584 bytes
[28449610200] [[35mTRACE[0m] [kernel] [CPU0]   Module[24]: name='/bin/grep' cmdline='init' size=78216 bytes
[28451277327] [[35mTRACE[0m] [kernel] [CPU0]   Module[25]: name='/bin/pwd' cmdline='init' size=35144 bytes
[28453812585] [[35mTRACE[0m] [kernel] [CPU0]   Module[26]: name='/bin/touch' cmdline='init' size=46216 bytes
[28457492613] [[35mTRACE[0m] [kernel] [CPU0]   Module[27]: name='/bin/setshell' cmdline='init' size=47104 bytes
[28460153271] [[35mTRACE[0m] [kernel] [CPU0]   Module[28]: name='/bin/dmesg' cmdline='init' size=39392 bytes
[28462639128] [[35mTRACE[0m] [kernel] [CPU0]   Module[29]: name='/bin/stat' cmdline='init' size=46264 bytes
[28465049811] [[35mTRACE[0m] [kernel] [CPU0]   Module[30]: name='/bin/file' cmdline='init' size=51160 bytes
[28467456600] [[35mTRACE[0m] [kernel] [CPU0]   Module[31]: name='/bin/dirname' cmdline='init' size=50160 bytes
[28469963313] [[35mTRACE[0m] [kernel] [CPU0]   Module[32]: name='/bin/basename' cmdline='init' size=50168 bytes
[28472025252] [[35mTRACE[0m] [kernel] [CPU0]   Module[33]: name='/bin/sleep' cmdline='init' size=56880 bytes
[28473879786] [[35mTRACE[0m] [kernel] [CPU0]   Module[34]: name='/bin/sort' cmdline='init' size=66400 bytes
[28475626575] [[35mTRACE[0m] [kernel] [CPU0]   Module[35]: name='/bin/env' cmdline='init' size=41792 bytes
[28477103193] [[35mTRACE[0m] [kernel] [CPU0]   Module[36]: name='/bin/uname' cmdline='' size=40600 bytes
[28478491404] [[35mTRACE[0m] [kernel] [CPU0]   Module[37]: name='/bin/true' cmdline='' size=20736 bytes
[28479897402] [[35mTRACE[0m] [kernel] [CPU0]   Module[38]: name='/bin/false' cmdline='' size=20736 bytes
[28481289309] [[35mTRACE[0m] [kernel] [CPU0]   Module[39]: name='/bin/input_echo' cmdline='' size=43096 bytes
[28482710322] [[35mTRACE[0m] [kernel] [CPU0]   Module[40]: name='/drivers/ps2_mouse' cmdline='' size=39312 bytes
[28484196576] [[35mTRACE[0m] [kernel] [CPU0]   Module[41]: name='/drivers/display_bootfb' cmdline='' size=81384 bytes
[28485676428] [[35mTRACE[0m] [kernel] [CPU0]   Module[42]: name='/drivers/display_virtio_gpu' cmdline='' size=128072 bytes
[28487354643] [[35mTRACE[0m] [kernel] [CPU0]   Module[43]: name='/bin/cambium' cmdline='' size=124728 bytes
[28489488060] [[35mTRACE[0m] [kernel] [CPU0]   Module[44]: name='/drivers/virtio_netd' cmdline='' size=106272 bytes
[28491536370] [[35mTRACE[0m] [kernel] [CPU0]   Module[45]: name='/drivers/rtl8168d' cmdline='' size=57360 bytes
[28493206566] [[35mTRACE[0m] [kernel] [CPU0]   Module[46]: name='/bin/netd' cmdline='' size=10813352 bytes
[28494809541] [[35mTRACE[0m] [kernel] [CPU0]   Module[47]: name='/bin/mesocarp' cmdline='' size=119480 bytes
[28496397072] [[35mTRACE[0m] [kernel] [CPU0]   Module[48]: name='/bin/mdns' cmdline='' size=119480 bytes
[28497760797] [[35mTRACE[0m] [kernel] [CPU0]   Module[49]: name='/bin/mdnsd' cmdline='' size=119480 bytes
[28499295099] [[35mTRACE[0m] [kernel] [CPU0]   Module[50]: name='/bin/fetchd' cmdline='' size=72720 bytes
[28500673113] [[35mTRACE[0m] [kernel] [CPU0]   Module[51]: name='/bin/httpsd' cmdline='' size=526160 bytes
[28502054328] [[35mTRACE[0m] [kernel] [CPU0]   Module[52]: name='/bin/find' cmdline='' size=57960 bytes
[28503448512] [[35mTRACE[0m] [kernel] [CPU0]   Module[53]: name='/bin/ip' cmdline='' size=55528 bytes
[28505016078] [[35mTRACE[0m] [kernel] [CPU0]   Module[54]: name='/bin/iso_reader' cmdline='' size=37504 bytes
[28506450522] [[35mTRACE[0m] [kernel] [CPU0]   Module[55]: name='/bin/ping' cmdline='' size=61144 bytes
[28507852527] [[35mTRACE[0m] [kernel] [CPU0]   Module[56]: name='/bin/nslookup' cmdline='' size=56480 bytes
[28509216054] [[35mTRACE[0m] [kernel] [CPU0]   Module[57]: name='/drivers/ahci_disk' cmdline='' size=74072 bytes
[28510829094] [[35mTRACE[0m] [kernel] [CPU0]   Module[58]: name='/drivers/ata_disk' cmdline='' size=61712 bytes
[28512243408] [[35mTRACE[0m] [kernel] [CPU0]   Module[59]: name='/bin/iso9660d' cmdline='' size=80648 bytes
[28513642509] [[35mTRACE[0m] [kernel] [CPU0]   Module[60]: name='/drivers/virtio_sound' cmdline='' size=95232 bytes
[28515082068] [[35mTRACE[0m] [kernel] [CPU0]   Module[61]: name='/drivers/hdaudio' cmdline='' size=67600 bytes
[28516578288] [[35mTRACE[0m] [kernel] [CPU0]   Module[62]: name='/drivers/pci_stubd' cmdline='' size=58224 bytes
[28518429786] [[35mTRACE[0m] [kernel] [CPU0]   Module[63]: name='/drivers/chime' cmdline='' size=55496 bytes
[28520376258] [[35mTRACE[0m] [kernel] [CPU0]   Module[64]: name='/bin/vfs_hello' cmdline='' size=35552 bytes
[28523870892] [[35mTRACE[0m] [kernel] [CPU0]   Module[65]: name='/bin/show_args' cmdline='' size=42568 bytes
[28525275108] [[35mTRACE[0m] [kernel] [CPU0]   Module[66]: name='/bin/env_roundtrip' cmdline='' size=44216 bytes
[28526781690] [[35mTRACE[0m] [kernel] [CPU0]   Module[67]: name='/bin/cwd_test' cmdline='' size=43776 bytes
[28528228674] [[35mTRACE[0m] [kernel] [CPU0]   Module[68]: name='/bin/date' cmdline='' size=62992 bytes
[28529619624] [[35mTRACE[0m] [kernel] [CPU0]   Module[69]: name='/bin/wayland_hello' cmdline='' size=65952 bytes
[28531312293] [[35mTRACE[0m] [kernel] [CPU0]   Module[70]: name='/bin/terminal' cmdline='' size=85376 bytes
[28532786073] [[35mTRACE[0m] [kernel] [CPU0]   Module[71]: name='/bin/tee' cmdline='' size=45032 bytes
[28534287804] [[35mTRACE[0m] [kernel] [CPU0]   Module[72]: name='/bin/xargs' cmdline='' size=55080 bytes
[28535729508] [[35mTRACE[0m] [kernel] [CPU0]   Module[73]: name='/bin/placed' cmdline='' size=38136 bytes
[28537206225] [[35mTRACE[0m] [kernel] [CPU0]   Module[74]: name='/bin/bloom' cmdline='' size=112872 bytes
[28539977433] [[35mTRACE[0m] [kernel] [CPU0]   Module[75]: name='/bin/clear' cmdline='' size=20864 bytes
[28541495565] [[35mTRACE[0m] [kernel] [CPU0]   Module[76]: name='/bin/loglevel' cmdline='' size=41600 bytes
[28543010760] [[35mTRACE[0m] [kernel] [CPU0]   Module[77]: name='/bin/poll_mux' cmdline='' size=39056 bytes
[28544635845] [[35mTRACE[0m] [kernel] [CPU0]   Module[78]: name='/bin/ipc_service_demo' cmdline='' size=47552 bytes
[28546499058] [[35mTRACE[0m] [kernel] [CPU0]   Module[79]: name='/bin/ipc_pipe_demo' cmdline='' size=44016 bytes
[28548100185] [[35mTRACE[0m] [kernel] [CPU0]   Module[80]: name='/bin/ipc_provider_demo' cmdline='' size=67008 bytes
[28549598847] [[35mTRACE[0m] [kernel] [CPU0]   Module[81]: name='/bin/ipc_memfd_demo' cmdline='' size=34216 bytes
[28551088698] [[35mTRACE[0m] [kernel] [CPU0]   Module[82]: name='/bin/test_exec' cmdline='' size=62504 bytes
[28552823145] [[35mTRACE[0m] [kernel] [CPU0]   Module[83]: name='/bin/test_vm_protect' cmdline='' size=34216 bytes
[28556753610] [[35mTRACE[0m] [kernel] [CPU0]   Module[84]: name='/bin/test_exec_env' cmdline='' size=54864 bytes
[28558418130] [[35mTRACE[0m] [kernel] [CPU0]   Module[85]: name='/bin/test_threads' cmdline='' size=78336 bytes
[28560057933] [[35mTRACE[0m] [kernel] [CPU0]   Module[86]: name='/bin/test_futex' cmdline='' size=48952 bytes
[28561527753] [[35mTRACE[0m] [kernel] [CPU0]   Module[87]: name='/bin/ld_so' cmdline='' size=378656 bytes
[28562929692] [[35mTRACE[0m] [kernel] [CPU0]   Module[88]: name='/bin/test_dyn_loader' cmdline='' size=53728 bytes
[28564466337] [[35mTRACE[0m] [kernel] [CPU0]   Module[89]: name='/bin/test_dlopen' cmdline='' size=60232 bytes
[28566067035] [[35mTRACE[0m] [kernel] [CPU0]   Module[90]: name='/bin/reboot' cmdline='' size=32136 bytes
[28567469733] [[35mTRACE[0m] [kernel] [CPU0]   Module[91]: name='/bin/shutdown' cmdline='' size=32136 bytes
[28568891241] [[35mTRACE[0m] [kernel] [CPU0]   Module[92]: name='/bin/attr_list' cmdline='' size=47456 bytes
[28570354362] [[35mTRACE[0m] [kernel] [CPU0]   Module[93]: name='/bin/attr_get' cmdline='' size=47552 bytes
[28572964233] [[35mTRACE[0m] [kernel] [CPU0]   Module[94]: name='/bin/attr_set' cmdline='' size=69656 bytes
[28574495532] [[35mTRACE[0m] [kernel] [CPU0]   Module[95]: name='/bin/attr_rm' cmdline='' size=47072 bytes
[28576136424] [[35mTRACE[0m] [kernel] [CPU0]   Module[96]: name='/lib/libpistil.so' cmdline='' size=67704 bytes
[28577835660] [[35mTRACE[0m] [kernel] [CPU0]   Module[97]: name='/share/wallpapers/clouds.bmp' cmdline='' size=786486 bytes
[28579439328] [[35mTRACE[0m] [kernel] [CPU0]   Module[98]: name='/share/wallpapers/flower.bmp' cmdline='' size=8386614 bytes
[28581011283] [[35mTRACE[0m] [kernel] [CPU0]   Module[99]: name='/share/wallpapers/leather.bmp' cmdline='' size=1179702 bytes
[28582567959] [[35mTRACE[0m] [kernel] [CPU0]   Module[100]: name='/share/wallpapers/linen.bmp' cmdline='' size=4718646 bytes
[28584175818] [[35mTRACE[0m] [kernel] [CPU0]   Module[101]: name='/share/fonts/NotoSans-Regular.ttf' cmdline='' size=569208 bytes
[28586329299] [[35mTRACE[0m] [kernel] [CPU0]   Module[102]: name='/share/fonts/unifont.hex' cmdline='' size=8355852 bytes
[28589213400] [[35mTRACE[0m] [kernel] [CPU0]   Module[103]: name='/etc/locale.conf' cmdline='' size=85 bytes
[28590764037] [[35mTRACE[0m] [kernel] [CPU0]   Module[104]: name='/etc/profile' cmdline='' size=68 bytes
[28592233296] [[35mTRACE[0m] [kernel] [CPU0]   Module[105]: name='/etc/motd' cmdline='' size=610 bytes
[28593745059] [[35mTRACE[0m] [kernel] [CPU0]   Module[106]: name='/etc/fstab' cmdline='' size=123 bytes
[28595179305] [[35mTRACE[0m] [kernel] [CPU0]   Module[107]: name='/etc/hostname' cmdline='' size=8 bytes
[28605978423] [[34mDEBUG[0m] [kernel] [CPU0] Found init module: /bin/sprout (cmdline: 'init'), loading...
[28608564831] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/sprout' (len=85144, base=0x200000)
[28610709039] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[28624084863] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[28627083342] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[28633621896] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [00, 00, 00, e8, e0, 7b, 00, 00]
[28641218925] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20c000 exec=false
[28643564697] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20e000 exec=false
[28648594887] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20f000 filesz=0 memsz=0 align=1
[28657424367] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='load_init_module' elapsed_ticks=49106574 elapsed_us=24553 total_ticks=744272100 total_us=372136
[28662631965] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='prepare_boot_registry' elapsed_ticks=1541793 elapsed_us=770 total_ticks=749490126 total_us=374745
[28665223191] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='map_boot_registry' elapsed_ticks=212124 elapsed_us=106 total_ticks=752094783 total_us=376047
[28667969517] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='activate_init_aspace' elapsed_ticks=203742 elapsed_us=101 total_ticks=754795305 total_us=377397
[28670654595] [[34mDEBUG[0m] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[28672337166] [[34mDEBUG[0m] [kernel] [CPU0] Spawning init process...
[28675098639] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb00661e0 arg=0xffffffffb001fd20
[28697308134] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[28699498641] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=5
[28700629683] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[28701811743] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='spawn_init_task' elapsed_ticks=29559915 elapsed_us=14779 total_ticks=788677857 total_us=394338
[28744791273] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED: early-boot bringup complete; resuming steady-state SMP scheduling
[28746436950] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='end_bringup' elapsed_ticks=1971882 elapsed_us=985 total_ticks=833303328 total_us=416651
[28791990777] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=878485146 elapsed_us=439242
[28794555735] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[28819066419] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[28822125156] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] ENTER_USER: TID=5 entry_pc=200000 user_sp=800000
[28867891404] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpus_online=4
[28879640625] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=0 curr=Some(5) runq=1 runq_avg=2 runq_samples=1 ctxsw=1 idle2busy=0 tick=1 ipi=0 enq=3 deq=1 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[28884870564] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=1 curr=Some(2) runq=0 runq_avg=0 runq_samples=17 ctxsw=0 idle2busy=0 tick=14 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[28891217982] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=2 curr=Some(3) runq=0 runq_avg=0 runq_samples=16 ctxsw=0 idle2busy=0 tick=11 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[28897544247] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=3 curr=Some(4) runq=0 runq_avg=0 runq_samples=16 ctxsw=0 idle2busy=0 tick=13 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[28923391992] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=0
[28931057760] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=20
[28944513741] [[34mDEBUG[0m] [sprout] [CPU0] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7fef70 rip=0x20414c rflags=0x202
[28946903502] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[28948357614] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: About to create Supervisor...
[28949773446] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Listing /bin directory...
[28953288540] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/bin' tid=5
[29051230296] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Parsed command line: '    '
[29054494887] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: FORCING display=bootfb for diagnostic test!
[29057673513] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Supervisor created, calling run_forever...
[29064175767] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[29066941398] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[29069787021] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[29071731018] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/run/sprout/shell' tid=5
[29078950593] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/etc/default/shell' tid=5
[29084489544] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/bin/sh' tid=5
[29086943523] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU0] BootFs: EXACT match for 'bin/sh' at index 4
[29090899101] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[29097669084] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[29100668421] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[29114539212] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU0] BootFs: EXACT match for 'bin/sh' at index 4
[29141861100] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'sh' (len=121736, base=0x200000)
[29144346000] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[29149229538] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[29151459348] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[29155304970] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [9c, b9, 00, 00, 48, 8d, 7c, 24]
[29165518008] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[29168046666] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=216000 exec=false
[29170150944] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x217000 filesz=0 memsz=0 align=1
[29176827339] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /bin/sh
[29179825389] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb009d500 arg=0xffffffffb006f660
[29186097963] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=6, applying inserts
[29187976389] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[29189709351] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=6
[29191091193] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[29192269029] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[29200132698] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SETUP_STDIO: stdin=Fd(3) stdout=Fd(4) stderr=Fd(5)
[29214703452] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 6
[29221435749] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[29224761291] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Task 6 woken, restoring IRQs
[29227032978] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Done for /bin/sh
[29233527609] [[35mTRACE[0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[29237338944] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[29239217238] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=6 entry_pc=200000 user_sp=800000
[29242062663] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[29243346099] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=0
[29246284947] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=100
[29247668076] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 250ms so the prompt can take the foreground
[29249416251] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=19
[29252054733] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[29254335924] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[29259917016] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=19
DEBUG: sh starting
[29271630267] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=33
[29273457873] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[29275182090] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[29277061143] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=33
DEBUG: sh sig handlers installed
[29279801397] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/run/motd' tid=6
[29287605765] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/etc/motd' tid=6
[29293612392] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'etc/motd' at index 105
[29301839490] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=610
[29303739366] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[29305535556] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[29307602181] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=610
[1;32m
        .-.
       /   \        [1;36mTHING-OS[1;32m
      |     |       [0;36m"People, places, things."[1;32m
       \   /        
        `-'        
       /   \        v0.1  ��  
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
[29318229435] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=23
[29320963452] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[29323556559] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[29326439967] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=23
DEBUG: sh motd printed
[29330822103] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=31
[29333497413] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[29336128899] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[29338822458] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=31
DEBUG: sh shell object created
[29343515553] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/etc/profile' tid=6
[29350360017] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'etc/profile' at index 104
[29387406873] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=25
[29389394595] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[29391172239] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[29393034330] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=25
DEBUG: sh profile loaded
[29406673065] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=97
[29408682567] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[29411373123] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[29414066385] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=97
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [29423908767] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=6
[29426361855] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[29428746996] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[29431317762] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=6
[?25h[29435333202] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[31096203666] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=1831504587
[31101083508] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpus_online=4
[31104944376] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=0 curr=Some(0) runq=0 runq_avg=1 runq_samples=3 ctxsw=2 idle2busy=0 tick=8 ipi=0 enq=4 deq=3 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[31111435641] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=1 curr=Some(2) runq=0 runq_avg=0 runq_samples=87 ctxsw=0 idle2busy=0 tick=46 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[31117723758] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=2 curr=Some(3) runq=0 runq_avg=0 runq_samples=85 ctxsw=0 idle2busy=0 tick=41 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[31124136648] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=3 curr=Some(6) runq=0 runq_avg=0 runq_samples=4212 ctxsw=1 idle2busy=1 tick=29 ipi=1 enq=4186 deq=4185 wake=1 lock_miss=0 lock_pending=0 lock_blocked=0
[31921685334] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Continuing supervisor startup
[31924249137] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawning cambium for driver discovery...
[31929734298] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: capacity=4096 port=0xffffffffb0071470
[31933622391] [[34mDEBUG[0m] [kernel::ipc] [CPU0] CREATE_PORT: appended id=PortId(0)
[31940910507] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=1 port_id=PortId(0) mode=Write
[31944632115] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU0] ALLOC_HANDLE: handle=2 port_id=PortId(0) mode=Read
[31954064373] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/cambium' (len=124728, base=0x200000)
[31956996720] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[31962120333] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[31965211509] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[31969156362] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [8d, 3c, f0, 48, 83, c7, 70, 48]
[31982596206] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=214000 exec=false
[31985768793] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=216000 exec=false
[31988777238] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x217000 filesz=0 memsz=0 align=1
[31996678890] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb0081820 arg=0xffffffffb0071600
[32001991758] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[32004342546] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=7
[32006430258] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[32015987916] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 7 → task '/bin/cambium' (pid=7 from boot module)
[32017846443] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=1 apic_id=1 vector=0x30
[32020780935] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned cambium (PID=7)
[32023166076] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawning iso9660d...
[32024403477] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[32027135184] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/iso9660d' (len=80648, base=0x200000)
[32029096341] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=7 entry_pc=200000 user_sp=800000
[32031267972] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[32036974728] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=0
[32039759433] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[32042326800] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=7 buf_len=100
[32044939740] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[32047942773] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [48, 89, 44, 24, 50, 48, 8d, 05]
[32054755755] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20b000 exec=false
[32057337213] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20d000 exec=false
[32060269164] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20e000 filesz=0 memsz=0 align=1
[32068980372] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb00b6440 arg=0xffffffffb0071600
[32074008846] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[32075439726] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=8
[32077089792] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[32082177963] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: TID 8 �� task '/bin/iso9660d' (pid=8 from boot module)
[32084796414] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[32088229998] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Spawned iso9660d (PID=8)
[32090321736] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[32093066346] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Deferring netd until /dev/net/virtio0/rx is ready...
[32095367370] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=8 entry_pc=200000 user_sp=800000
[32100252855] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=0
[32103697824] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=8 buf_len=100
[32107792332] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[32111043921] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers' tid=7
[32123400936] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Task 9 (user thread) assigned to CPU 3
[32129775612] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[32131951863] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running → Runnable (cpu=3, next=9)
[32134240149] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=9
[32136317499] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[32139735705] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Sending post-unlock Resched IPI to CPU 3 for task 9
[32142633006] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[32147101371] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Running registration + health supervision loop
[32154311145] [[34mDEBUG[0m] [kernel::handle::bridge] [CPU0] BRIDGE: handle=2 -> fd=3 node=0xffffffffb0069330 port=0xffffffffb0071470
[32159013051] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Bridged resp_port 2 -> FD 3 for task 'cambium'
[32169061947] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ahci_disk' tid=7
[32177175063] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=0
[32180194365] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=9 buf_len=20
[32182480902] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ahci_disk' at index 57
[32184917325] [[32mINFO [0m] [sprout::supervisor] [CPU3] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[32188156902] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[32213028012] [[34mDEBUG[0m] [iso9660d] [CPU2] iso9660d: starting ISO9660 VFS provider
[32215462422] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/services/storage' tid=8
[32223932037] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
[32499880380] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[32505628056] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ata_disk' tid=7
[32510447871] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ata_disk' at index 58
[32533533747] [[35mTRACE[0m] [bran::arch::x86_64] [CPU2] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[32536235457] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running → Runnable (cpu=3, next=9)
[32542065237] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[32690205438] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ata_disk' name='ata_disk' class=Block kind='dev.storage.ata' start='thingos_driver_start_safe'
[32693255925] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/chime' tid=7
[32696832993] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/chime' at index 63
[32869496946] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[32873489880] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/display_bootfb' tid=7
[32878788855] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/display_bootfb' at index 41
[32898267963] [[35mTRACE[0m] [bran::arch::x86_64] [CPU2] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[32912606067] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running → Runnable (cpu=3, next=9)
[32925595857] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[33103872747] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpus_online=4
[33107002830] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=0 curr=Some(0) runq=0 runq_avg=1 runq_samples=69 ctxsw=6 idle2busy=0 tick=20 ipi=0 enq=70 deq=69 wake=2 lock_miss=0 lock_pending=0 lock_blocked=0
[33112565046] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=1 curr=Some(7) runq=0 runq_avg=0 runq_samples=118 ctxsw=1 idle2busy=1 tick=55 ipi=0 enq=3 deq=2 wake=1 lock_miss=0 lock_pending=0 lock_blocked=0
[33119226954] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=2 curr=Some(3) runq=0 runq_avg=0 runq_samples=146 ctxsw=2 idle2busy=1 tick=57 ipi=0 enq=2 deq=1 wake=1 lock_miss=0 lock_pending=0 lock_blocked=0
[33123974202] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=3 curr=Some(6) runq=0 runq_avg=0 runq_samples=8929 ctxsw=7 idle2busy=1 tick=31 ipi=3 enq=8903 deq=8902 wake=3 lock_miss=0 lock_pending=0 lock_blocked=0
[33136747908] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[33142482351] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/display_virtio_gpu' tid=7
[33148047240] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/display_virtio_gpu' at index 42
[33238436352] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[33240650883] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running → Runnable (cpu=3, next=9)
[33246023877] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[33378649392] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[33383676678] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/hdaudio' tid=7
[33390647070] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/hdaudio' at index 61
[33608708430] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[33613248867] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/pci_stubd' tid=7
[33619103793] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/pci_stubd' at index 62
[33702495684] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[33713613912] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running ��� Runnable (cpu=3, next=9)
[33724885656] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[33819702081] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[33824337063] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ps2_kbd' tid=7
[33828903240] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ps2_kbd' at index 3
[33965415462] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[33974525409] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/services/storage' tid=8
[34079448744] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[34081732476] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running → Runnable (cpu=3, next=9)
[34087848795] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[34097722164] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[34104279429] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/ps2_mouse' tid=7
[34109813694] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ps2_mouse' at index 40
[34317701682] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[34321934130] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/rtc_cmos' tid=7
[34325298810] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/rtc_cmos' at index 2
[34432184259] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[34435689948] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running → Runnable (cpu=3, next=9)
[34447510284] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[34522999236] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[34526922111] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/rtl8168d' tid=7
[34532937120] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/rtl8168d' at index 45
[34735229991] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtl8168d' name='rtl8168d' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[34739679645] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/virtio_netd' tid=7
[34745332314] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/virtio_netd' at index 44
[34764212670] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[34766481156] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running → Runnable (cpu=3, next=9)
[34772703141] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[34993633785] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_netd' name='virtio_netd' class=Net kind='dev.net.Nic' start='thingos_driver_start_safe'
[34999743702] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/drivers/virtio_sound' tid=7
[35005382016] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/virtio_sound' at index 60
[35118488097] [[35mTRACE[0m] [bran::arch::x86_64] [CPU2] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[35120777340] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running �� Runnable (cpu=3, next=9)
[35130442941] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[35133829500] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpus_online=4
[35136405678] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=0 curr=Some(0) runq=0 runq_avg=1 runq_samples=154 ctxsw=14 idle2busy=0 tick=35 ipi=0 enq=155 deq=154 wake=6 lock_miss=0 lock_pending=0 lock_blocked=0
[35142829524] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=1 curr=Some(7) runq=0 runq_avg=0 runq_samples=123 ctxsw=1 idle2busy=1 tick=61 ipi=0 enq=8 deq=7 wake=1 lock_miss=0 lock_pending=0 lock_blocked=0
[35148611091] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=2 curr=Some(3) runq=0 runq_avg=0 runq_samples=210 ctxsw=4 idle2busy=2 tick=75 ipi=0 enq=3 deq=2 wake=2 lock_miss=0 lock_pending=0 lock_blocked=0
[35154798624] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=3 curr=Some(9) runq=1 runq_avg=0 runq_samples=12827 ctxsw=18 idle2busy=1 tick=35 ipi=7 enq=12802 deq=12800 wake=9 lock_miss=0 lock_pending=0 lock_blocked=0
[35229193131] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/virtio_sound' name='virtio_sound' class=Audio kind='dev.sound.Virtio' start='thingos_driver_start_safe'
[35234893485] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: 13 driver(s) found
[35237744322] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices' tid=7
[35242082898] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices'
[35254252110] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: readdir found 10 slots
[35263288368] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: readdir wrote 147 bytes
[35272451808] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/vendor' tid=7
[35276470350] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:00.0/vendor'
[35282636961] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:00.0/vendor'
[35293641867] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/device' tid=7
[35297982159] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:00.0/device'
[35301462339] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:00.0/device'
[35307734616] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/class' tid=7
[35310585288] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:00.0/class'
[35313752463] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:00.0/class'
[35319793212] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/status' tid=7
[35323611180] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:00.0/status'
[35326949493] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:00.0/status'
[35332968495] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:00.0/kind' tid=7
[35337114450] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:00.0/kind'
[35341007295] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:00.0/kind'
[35346210306] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/vendor' tid=7
[35350213866] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/vendor'
[35354607255] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:01.0/vendor'
[35362821780] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/device' tid=7
[35367040764] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/device'
[35370450390] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:01.0/device'
[35375755272] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/class' tid=7
[35379508956] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/class'
[35382402924] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:01.0/class'
[35391272466] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/status' tid=7
[35395599426] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/status'
[35398416768] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:01.0/status'
[35401934073] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:01.0/kind' tid=7
[35404985484] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:01.0/kind'
[35408177805] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:01.0/kind'
[35413534332] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/vendor' tid=7
[35417809119] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/vendor'
[35420469744] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/vendor'
[35424564615] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/device' tid=7
[35428819800] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/device'
[35432260215] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/device'
[35437797813] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/class' tid=7
[35440760421] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/class'
[35443007127] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/class'
[35446788234] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/status' tid=7
[35450542974] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/status'
[35453863434] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/status'
[35458909761] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/kind' tid=7
[35463024663] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/kind'
[35466542199] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/kind'
[35472898230] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.0/vendor' tid=7
[35477283501] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.0/vendor'
[35481251289] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.0/vendor'
[35489748954] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=0 apic_id=0 vector=0x30
[35494400601] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running �� Runnable (cpu=3, next=9)
[35497726308] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[35500711884] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[35503429005] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.0/device' tid=7
[35507296011] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.0/device'
[35510815989] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.0/device'
[35517162945] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.0/class' tid=7
[35521623786] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.0/class'
[35525169537] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.0/class'
[35532883716] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.0/status' tid=7
[35537641722] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.0/status'
[35540815266] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.0/status'
[35545268088] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.0/kind' tid=7
[35549696424] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.0/kind'
[35553019887] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.0/kind'
[35566411716] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.2/vendor' tid=7
[35571116988] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/vendor'
[35576121669] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/vendor'
[35583515880] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.2/device' tid=7
[35588347839] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/device'
[35592983976] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/device'
[35598931302] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.2/class' tid=7
[35602145271] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/class'
[35605833780] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/class'
[35612435595] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.2/status' tid=7
[35616989496] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/status'
[35621130171] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/status'
[35629223850] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.2/kind' tid=7
[35637516189] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/kind'
[35641572681] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/kind'
[35646428466] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.3/vendor' tid=7
[35651683782] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.3/vendor'
[35655452382] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.3/vendor'
[35661731787] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.3/device' tid=7
[35667107916] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.3/device'
[35671373562] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.3/device'
[35677424508] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.3/class' tid=7
[35681580165] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.3/class'
[35685002100] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.3/class'
[35690947182] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.3/status' tid=7
[35694263814] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.3/status'
[35696836956] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.3/status'
[35701726533] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.3/kind' tid=7
[35705787018] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.3/kind'
[35709189384] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.3/kind'
[35714601087] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0070/vendor' tid=7
[35729599191] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/vendor'
[35743953069] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/vendor'
[35746596666] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[35760055089] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0070/device' tid=7
[35765636346] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU2] VFS: sys_fs_open path='/services/storage' tid=8
[35768451345] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/device'
[35772271590] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/device'
[35778079722] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0070/class' tid=7
[35792146995] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/class'
[35795530914] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/class'
[35802076233] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0070/status' tid=7
[35806036860] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/status'
[35812590396] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/status'
[35820157857] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0070/kind' tid=7
[35825448417] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0070/kind'
[35829804549] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0070/kind'
[35836311225] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0060/vendor' tid=7
[35839913967] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/vendor'
[35842432956] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/vendor'
[35847489150] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0060/device' tid=7
[35851659162] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/device'
[35854398228] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/device'
[35861401191] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0060/class' tid=7
[35865061947] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/class'
[35869083789] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/class'
[35874327687] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0060/status' tid=7
[35878714410] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/status'
[35885055294] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/status'
[35889693708] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-0060/kind' tid=7
[35893104984] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-0060/kind'
[35897186655] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-0060/kind'
[35900837874] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-01f0/vendor' tid=7
[35904264660] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-01f0/vendor'
[35907683724] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-01f0/vendor'
[35911790310] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-01f0/device' tid=7
[35915350152] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-01f0/device'
[35918229567] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-01f0/device'
[35922288171] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-01f0/class' tid=7
[35924959422] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-01f0/class'
[35928817815] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-01f0/class'
[35933224041] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-01f0/status' tid=7
[35936679438] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-01f0/status'
[35940066657] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-01f0/status'
[35943856410] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/isa-01f0/kind' tid=7
[35947152549] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/isa-01f0/kind'
[35950879239] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'isa-01f0/kind'
[35956539465] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[35958725814] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running �� Runnable (cpu=3, next=9)
[35961292818] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/status' tid=7
[35964313506] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[35966892159] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/status'
[35976170736] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:02.0/status'
[35992499730] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=3 len=144
[35995425708] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=3 starting copyin
[36000303834] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=3 copyin ok
[36011314152] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/virtio_netd' at index 44
[36030517875] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: Loading module 'virtio_netd' (len=106272, base=0x200000)
[36033953439] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[36039157968] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[36042541953] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=200000 exec=true
[36047122122] [[35mTRACE[0m] [kernel::task::loader] [CPU1]   COPIED at 0x201420: [e9, 0f, 86, 41, 04, 00, 00, be]
[36059541045] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=20e000 exec=false
[36064998849] [[35mTRACE[0m] [kernel::task::loader] [CPU1] Segment: vaddr=212000 exec=false
[36068997030] [[34mDEBUG[0m] [kernel::task::loader] [CPU1] LOADER: TLS block: tp=0x214000 filesz=0 memsz=0 align=1
[36089731491] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN: driver entrypoint override 'thingos_driver_start_safe' => VA 0x20277c + bias 0x0 = PC 0x20277c
[36093582657] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Starting Phase 1 for /drivers/virtio_netd
[36099186750] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb00f8360 arg=0xffffffffb008d0a0
[36103870737] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 1 complete, ID=10, applying inserts
[36106102923] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[36109123017] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=10
[36111489645] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[36113432322] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[36118113702] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SETUP_STDIO: stdin=Inherit stdout=Inherit stderr=Inherit
[36128769963] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Phase 2 complete, waking task 10
[36133411248] [[35mTRACE[0m] [kernel::sched::blocking] [CPU1] WAKE_TASK: ID=10 taking SCHEDULER lock
[36141700089] [[35mTRACE[0m] [kernel::sched::blocking] [CPU1] WAKE_TASK: ID=10 wake_task_locked returned IPI_CPU=None
[36145841721] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Task 10 woken, restoring IRQs
[36147514161] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SPAWN_FROM_PATH: Done for /drivers/virtio_netd
[36152980281] [[35mTRACE[0m] [kernel::syscall::handlers::process] [CPU1] SYSCALL SPAWN_PROCESS_EX: name='/drivers/virtio_netd' TID=10 PID=10
[36157423038] [[34mDEBUG[0m] [cambium::spawn] [CPU1] CAMBIUM: launched driver /drivers/virtio_netd for pci-0000:00:02.0 (entry='thingos_driver_start_safe', pid=10)
[36161607240] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:1f.2/status' tid=7
[36165509787] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:1f.2/status'
[36168843843] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: matched device file 'pci-0000:00:1f.2/status'
[36177383385] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=4 len=144
[36180604284] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=4 starting copyin
[36183242040] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[36186165477] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[36188219925] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_write: tid=7 fd=4 copyin ok
[36190942194] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[36194091153] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
e[36197565162] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[36207150474] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x20277c SP=0x800000 ARG0=0x3
[36210917292] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=10 entry_pc=20277c user_sp=800000
[36216165513] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=0
[36219527388] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU1] sys_auxv_get: tid=10 buf_len=100
[36257410134] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Starting VirtIO-NET driver service...
[36260693238] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Initializing hardware driver...
[36264355644] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Claiming NIC device at /sys/devices/pci-0000:00:02.0
[36267818928] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: device::new(/sys/devices/pci-0000:00:02.0)
[36270535191] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: claiming '/sys/devices/pci-0000:00:02.0'...
[36279055758] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 10 claimed device 'pci-0000:00:02.0' (handle 0)
[36286779672] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: claimed, handle=0
[36291360237] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/virtio/common_bar' tid=10
[36294834411] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/common_bar'
[36309483903] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/common_bar -> '4' (n=2)
[36316835016] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/virtio/common_offset' tid=10
[36319790100] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/common_offset'
[36325953708] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/common_offset -> '0x0' (n=4)
[36328921233] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/virtio/notify_bar' tid=10
[36332060028] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/notify_bar'
[36336620595] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/notify_bar -> '4' (n=2)
[36339216441] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[36341614419] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/virtio/notify_offset' tid=10
[36344636262] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[36346558578] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[36348403410] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/notify_offset'
[36353502339] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
c[36356751783] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[36360073299] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running ��� Runnable (cpu=3, next=9)
[36362406432] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/notify_offset -> '0x3000' (n=7)
[36365019141] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[36367582020] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/virtio/notify_multiplier' tid=10
[36372264885] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/notify_multiplier'
[36380059617] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/notify_multiplier -> '4' (n=2)
[36383871645] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[36386969751] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/virtio/device_bar' tid=10
[36390209625] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/device_bar'
[36397656735] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/device_bar -> '4' (n=2)
[36401288913] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU1] VFS: sys_fs_open path='/sys/devices/pci-0000:00:02.0/virtio/device_offset' tid=10
[36404929539] [[35mTRACE[0m] [kernel::vfs::sysfs] [CPU1] sysfs: lookup path='devices/pci-0000:00:02.0/virtio/device_offset'
[36409869144] [[34mDEBUG[0m] [virtio::device] [CPU1] READ_SYS: /sys/devices/pci-0000:00:02.0/virtio/device_offset -> '0x2000' (n=7)
[36415851483] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: mapping common BAR4...
[36420450165] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: mapping BAR4 (phys=0xc000004000, size=0x4000) for task 10
[36426479826] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x10000000
[36430823319] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: common_cfg at 0x10000000
[36433443222] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: notify_cfg at 0x10003000
[36436928220] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: device_cfg = Some(268443648)
[36439524066] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: allocating DMA command buffer...
[36445519011] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 1 pages phys=0x232f000 -> user_va=0x10004000
[36454045485] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: cmd_buf virt=0x10004000 phys=0x232f000
[36457377594] [[34mDEBUG[0m] [virtio::device] [CPU1] VirtIO: device::new complete
[36460929450] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Device features 0x30bf8024
[36464619246] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: MAC 52:54:00:12:34:56
[36467367618] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Link UP
[36475358568] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 4 pages phys=0x2330000 -> user_va=0x10005000
[36484675161] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: RX queue 0 setup (size=32)
[36493301526] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 4 pages phys=0x2334000 -> user_va=0x10009000
[36497760552] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: TX queue 1 setup (size=32)
[36505007220] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 16 pages phys=0x2338000 -> user_va=0x1000d000
[36507857100] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[36509676456] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[36512445024] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[36514811289] [[34mDEBUG[0m] [kernel::syscall::handlers::device] [CPU1] DEVICE: DMA alloc 1 pages phys=0x2348000 -> user_va=0x1001d000
[36516989157] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[36518585202] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Allocated TX buffer (1 page)
[36520036179] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
[36522011559] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: RX queue filled with 32 buffers
h[36525346803] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[36527843121] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: DRIVER_OK set, device is live
[36530078706] [[34mDEBUG[0m] [virtio_netd::driver] [CPU1] VirtIO-NET: Driver initialized successfully
[36533036001] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Driver initialized successfully
[36535788630] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: MAC 52:54:00:12:34:56
[36538406883] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Initial link state is UP
[36544682064] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=65536 port=0xffffffffb00df0f0
[36547185807] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(1)
[36549857784] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=3 port_id=PortId(1) mode=Write
[36552741951] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=4 port_id=PortId(1) mode=Read
[36556501806] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Created VFS provider port
[36572157072] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Task 11 (user thread) assigned to CPU 2
[36594241431] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 1 deferred inserts
[36596450517] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=11
[36598375308] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[36600339864] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Sending post-unlock Resched IPI to CPU 2 for task 11
[36602889873] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=2 apic_id=2 vector=0x30
[36607247226] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=11 buf_len=0
[36610987479] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU2] sys_auxv_get: tid=11 buf_len=100
[36615136866] [[34mDEBUG[0m] [virtio_netd] [CPU2] VIRTIO_NETD: Entering VFS provider service loop at /dev/net/virtio0
[36630322344] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=262400 port=0xffffffffb00e21b0
[36633283698] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(2)
[36637641315] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=5 port_id=PortId(2) mode=Write
[36644721861] [[35mTRACE[0m] [bran::arch::x86_64] [CPU2] SMP: send_ipi cpu_index=0 apic_id=0 vector=0x30
[36647490693] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] vfs: mounted userland provider at /dev/net/virtio0 (flags: 0x0)
[36652268862] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Mounted at /dev/net/virtio0
[36654890910] [[34mDEBUG[0m] [virtio_netd] [CPU1] VIRTIO_NETD: Provider thread live at /dev/net/virtio0
[36675979989] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=1
[36678766146] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[36681364698] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[36684489039] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=1
o[36689622915] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[36693197244] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running → Runnable (cpu=3, next=9)
[36697775202] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/dev/net/virtio0/rx' tid=9
[36705758199] [[35mTRACE[0m] [kernel::ipc::port] [CPU3] PORT: Port written 13 bytes from TID 9
[36708574452] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 13 bytes from TID 11
[36711361137] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU2] sys_port_recv: read 13 bytes from port PortId(1)
[36716999055] [[35mTRACE[0m] [virtio_netd] [CPU2] VIRTIO_NETD: dispatch begin op=Lookup resp_port=5 payload_len=6
[36720483096] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU2] VIRTIO_NETD: rpc op=Lookup payload_len=6
[36723741219] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU2] VIRTIO_NETD: lookup 'rx'
[36727501569] [[34mDEBUG[0m] [virtio_netd::vfs_provider] [CPU2] VIRTIO_NETD: lookup 'rx' -> handle=5
[36731361843] [[35mTRACE[0m] [virtio_netd] [CPU2] VIRTIO_NETD: dispatch end op=Lookup resp_port=5 status=0 resp_payload_len=8
[36734606964] [[35mTRACE[0m] [virtio_netd] [CPU2] VIRTIO_NETD: send_response begin op=Lookup resp_port=5
[36741719025] [[35mTRACE[0m] [bran::arch::x86_64] [CPU2] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[36745168779] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU2] sys_port_send_all: wrote 9 bytes to port PortId(2)
[36747832110] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running �� Runnable (cpu=3, next=9)
[36751976184] [[35mTRACE[0m] [kernel::ipc::port] [CPU3] PORT: Port read 9 bytes from TID 9
[36754185534] [[35mTRACE[0m] [virtio_netd] [CPU2] VIRTIO_NETD: send_response end op=Lookup resp_port=5
[36766852782] [[35mTRACE[0m] [kernel::ipc::port] [CPU3] PORT: Port written 15 bytes from TID 9
[36768918285] [[35mTRACE[0m] [kernel::ipc::port] [CPU2] PORT: Port read 15 bytes from TID 11
[36771111861] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU2] sys_port_recv: read 15 bytes from port PortId(1)
[36775814130] [[35mTRACE[0m] [virtio_netd] [CPU2] VIRTIO_NETD: dispatch begin op=Close resp_port=5 payload_len=8
[36778738755] [[35mTRACE[0m] [virtio_netd::vfs_provider] [CPU2] VIRTIO_NETD: rpc op=Close payload_len=8
[36781634142] [[35mTRACE[0m] [virtio_netd] [CPU2] VIRTIO_NETD: dispatch end op=Close resp_port=5 status=0 resp_payload_len=0
[36787008654] [[35mTRACE[0m] [virtio_netd] [CPU2] VIRTIO_NETD: send_response begin op=Close resp_port=5
[36790149264] [[35mTRACE[0m] [bran::arch::x86_64] [CPU2] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[36792837081] [[35mTRACE[0m] [kernel::sched] [CPU3] SCHED[TID6]: Running → Runnable (cpu=3, next=9)
[36795202191] [[35mTRACE[0m] [kernel::syscall::handlers::port] [CPU2] sys_port_send_all: wrote 1 bytes to port PortId(2)
[36798385239] [[35mTRACE[0m] [kernel::ipc::port] [CPU3] PORT: Port read 1 bytes from TID 9
[36800698506] [[35mTRACE[0m] [virtio_netd] [CPU2] VIRTIO_NETD: send_response end op=Close resp_port=5
[36805172877] [[32mINFO [0m] [sprout::supervisor] [CPU3] SPROUT: /dev/net/virtio0/rx is ready; spawning netd.
[36809120535] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: Loading module '/bin/netd' (len=10813352, base=0x200000)
[36811975233] [[35mTRACE[0m] [kernel::task::loader] [CPU3]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[36817383339] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[36820082343] [[35mTRACE[0m] [kernel::task::loader] [CPU3] Segment: vaddr=200000 exec=true
[36823566351] [[35mTRACE[0m] [kernel::task::loader] [CPU3]   COPIED at 0x201420: [48, 8b, 06, 48, 8b, 80, 70, 01]
[36853750263] [[35mTRACE[0m] [kernel::task::loader] [CPU3] Segment: vaddr=234000 exec=false
[36858630864] [[35mTRACE[0m] [kernel::task::loader] [CPU3] Segment: vaddr=23a000 exec=false
[37011438354] [[35mTRACE[0m] [bran::arch::x86_64] [CPU1] SMP: send_ipi cpu_index=0 apic_id=0 vector=0x30
[37018781118] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ahci_disk' at index 57

```
</details>
