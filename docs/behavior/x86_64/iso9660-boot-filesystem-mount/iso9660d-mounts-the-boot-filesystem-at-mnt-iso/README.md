# ❌ Scenario: iso9660d mounts the boot filesystem at /mnt/iso

> Last run: 2026-04-22 08:47:33

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 12297ms | - [📜](./01/serial.log) - |
| 2 | Then the log should match pattern "iso9660d: found ISO9660 on device (atapi|ata_|ahci)" | ❌ | 1013ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[37743639021] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] enter
[37755977655] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] phys_memory_map ok
[37761075858] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] modules ok
[37763194887] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] phys_to_virt_offset ok
[37765112451] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] memory map logging done
[37804388424] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] boot_frame_alloc init ok
[38132872668] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] frame allocator build ok
[38135659914] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] frame allocator log ok
[38137984665] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] FRAME_ALLOCATOR init ok
[38140197711] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] tasking init ok
[38146366929] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[38148535491] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] enter
[38150688840] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] set expand hook
[38152612707] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] expand hook ok
[38154494499] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock begin
[38164332723] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock ok
[38167497093] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] reserve_region begin
[38227419978] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] reserve_region ok
[38230107762] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] inner allocator init begin
[38236109472] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] inner allocator init ok
[38244900408] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] heap top store ok
[38247191136] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] init done
[38258863863] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] set_boot_fb begin
[38263654077] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] devfs: set_boot_fb width=1920 height=1080 pitch=7680 resource_id=0xfb000000
[38269586388] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] set_boot_fb ok
[38272350171] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs set_boot_fb elapsed_ticks=12754269 elapsed_us=6377
[38284125561] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs FbNode::new elapsed_ticks=9083514 elapsed_us=4541
[38286615279] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] register begin
[38295379551] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] register ok
[38297413374] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs register fb0 elapsed_ticks=10683321 elapsed_us=5341
[38305790589] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[38321880993] [[34mDEBUG[0m] [kernel] [CPU0] Seeding entropy pool...
[38336941302] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed begin
[38339388747] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] fill_entropy done
[38344699239] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] add_sample(timer) ok
[38346778173] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] mark_seeded(timer) ok
[38356333620] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[38358424104] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[38360533398] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed done
[38363314671] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] entropy::seed_from_hardware elapsed_ticks=25556685 elapsed_us=12778
[38372227377] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[38391864258] [[35mTRACE[0m] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[38398289490] [[35mTRACE[0m] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[38400393933] [[35mTRACE[0m] [kernel::sched] [CPU0]   Allocating scheduler...
[38408623671] [[35mTRACE[0m] [kernel::sched] [CPU0]   Leaking scheduler...
[38418232248] [[35mTRACE[0m] [kernel::sched] [CPU0]   Initializing boot task...
[38424211485] [[35mTRACE[0m] [kernel::sched] [CPU0]   Creating boot task...
[38439822234] [[35mTRACE[0m] [kernel::sched] [CPU0]   Creating idle tasks...
[38442840117] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80016cc0 kstack_top=0xffffffffb001f980 arg=0x0
[38448264294] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[38489274549] [[35mTRACE[0m] [kernel::sched] [CPU0]   Boot task initialized
[38491131822] [[35mTRACE[0m] [kernel::sched] [CPU0]   Storing scheduler pointer...
[38497113501] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[38506487547] [[34mDEBUG[0m] [kernel] [CPU0] Initializing VFS...
[38520786315] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='set_cmdline' elapsed_ticks=4795791 elapsed_us=2397 total_ticks=13914912 total_us=6957
[38559533727] [[34mDEBUG[0m] [kernel::vfs] [CPU0] VFS: Created /dev/display directory
[38589660978] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted union filesystem at / (root)
[38602564275] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted devfs at /dev
[38606878299] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted procfs at /proc
[38612105268] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted sysfs at /sys
[38619946167] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /tmp
[38622184326] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /run
[38624274942] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /services
[38626633089] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /session
[38628836829] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /data
[38630470197] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='vfs_init' elapsed_ticks=102331581 elapsed_us=51165 total_ticks=124109964 total_us=62054
[38639729865] [[34mDEBUG[0m] [kernel] [CPU0] Scanning PCI bus...
[38647697352] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x29c0 at 00:00.0 class=060000 id=1
[38657562900] [[35mTRACE[0m] [kernel] [CPU0]   BAR0 (MEM32): 0x80000000 (size 0x800000)
[38679296634] [[35mTRACE[0m] [kernel] [CPU0]   BAR2 (MEM64): 0xc000000000 (size 0x4000)
[38685694245] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (MEM32): 0x80882000 (size 0x1000)
[38688489774] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1050 at 00:01.0 class=030000 id=2
[38699251404] [[35mTRACE[0m] [kernel] [CPU0]   BAR0 (I/O):  0x6060 (size 0x20)
[38703894405] [[35mTRACE[0m] [kernel] [CPU0]   BAR1 (MEM32): 0x80881000 (size 0x1000)
[38716886175] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (MEM64): 0xc000004000 (size 0x4000)
[38719145718] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1000 at 00:02.0 class=020000 id=3
[38722232109] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2918 at 00:1f.0 class=060100 id=4
[38729010474] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (I/O):  0x6040 (size 0x20)
[38736917637] [[35mTRACE[0m] [kernel] [CPU0]   BAR5 (MEM32): 0x80880000 (size 0x1000)
[38739244170] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2922 at 00:1f.2 class=010601 id=5
[38743793583] [[35mTRACE[0m] [kernel] [CPU0]   BAR4 (I/O):  0x6000 (size 0x40)
[38745907497] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2930 at 00:1f.3 class=0c0500 id=6
[38750978145] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='scan_pci' elapsed_ticks=108795687 elapsed_us=54397 total_ticks=244623918 total_us=122311
[38761927677] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='register_legacy_devices' elapsed_ticks=1230108 elapsed_us=615 total_ticks=255572361 total_us=127786
[38774262747] [[34mDEBUG[0m] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[38786256036] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[38789488782] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[38791958832] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[38794281273] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[38802252291] [[34mDEBUG[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[38805910407] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[38808450153] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[38814551556] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[38816635671] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[38820905871] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[38823820398] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[38826786042] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[38828943945] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[38831040600] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[38833308657] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[38835467352] [[35mTRACE[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[38838227043] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[38840561991] [[34mDEBUG[0m] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[38842770351] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[38845716030] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Registers initialized
[38851429947] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[38854187394] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[38857048164] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: All pins masked
[38860301766] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> NMI
[38862380997] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[38863877316] [[35mTRACE[0m] [bran::arch] [CPU0] IOAPIC: IRQ4 -> GSI 4 -> 0x24
[38865786333] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Init complete
[38901778872] [[34mDEBUG[0m] [bran::arch::x86_64::ioapic] [CPU0] LAPIC: calibrated 62436400 ticks/sec (delta=624364, ok=true) -> init_cnt=624364
[38907909282] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62436400 ticks/sec), init_cnt=624364 for 100Hz
[38911150212] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='setup_preemption_timer' elapsed_ticks=134753784 elapsed_us=67376 total_ticks=404788692 total_us=202394
[38925226560] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[38930726043] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[38937379305] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[38944074675] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[38983621974] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[38989965498] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] SMP: CPU 1 initializing preemption timer (vec=32 init_cnt=624364)
[38993478018] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[38995181643] [[34mDEBUG[0m] [kernel] [CPU1] SMP: kernel_secondary_entry arg_cpu=1 runtime_cpu=1
[38998335750] [[34mDEBUG[0m] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[39001837182] [[34mDEBUG[0m] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[39009872451] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80016cc0 kstack_top=0xffffffffb0032e40 arg=0x1
[39013943265] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Task 2 assigned to CPU 1
[39028080135] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] SMP: CPU 2 initializing preemption timer (vec=32 init_cnt=624364)
[39038573112] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[39041600862] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[39046913301] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Applying 2 deferred inserts
[39055717734] [[34mDEBUG[0m] [kernel] [CPU2] SMP: kernel_secondary_entry arg_cpu=2 runtime_cpu=2
[39058070832] [[34mDEBUG[0m] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[39063884508] [[34mDEBUG[0m] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[39066157845] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=1
[39070899483] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserting TID=2
[39074043261] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[39077023986] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[39090115680] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[39092021727] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='start_secondary_cpus' elapsed_ticks=167162919 elapsed_us=83581 total_ticks=585651363 total_us=292825
[39101309280] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] SMP: CPU 3 initializing preemption timer (vec=32 init_cnt=624364)
[39106260336] [[34mDEBUG[0m] [kernel] [CPU3] SMP: kernel_secondary_entry arg_cpu=3 runtime_cpu=3
[39117929928] [[34mDEBUG[0m] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[39140765433] [[34mDEBUG[0m] [kernel::sched] [CPU1] REGISTRY: Inserts applied
[39151152942] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU2] INIT KERNEL CTX: entry=0xffffffff80016cc0 kstack_top=0xffffffffb00445a0 arg=0x2
[39156125151] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] SCHED: Task 3 assigned to CPU 2
[39162058452] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Applying 1 deferred inserts
[39164699574] [[34mDEBUG[0m] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[39169015974] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Enumerating 108 boot modules...
[39174989238] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[39178914687] [[35mTRACE[0m] [kernel] [CPU0]   Module[0]: name='/bin/sprout' cmdline='init' size=85144 bytes
[39185757303] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80016cc0 kstack_top=0xffffffffb0055d00 arg=0x3
[39190141122] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: run_scheduler entry on CPU 1
[39197859162] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserting TID=3
[39207310263] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SCHED: Task 4 assigned to CPU 3
[39213224886] [[35mTRACE[0m] [kernel] [CPU0]   Module[1]: name='/bin/bristle' cmdline='' size=45888 bytes
[39216435060] [[34mDEBUG[0m] [kernel::sched] [CPU2] REGISTRY: Inserts applied
[39217998633] [[35mTRACE[0m] [kernel] [CPU0]   Module[2]: name='/drivers/rtc_cmos' cmdline='init' size=39744 bytes
[39229877478] [[35mTRACE[0m] [kernel] [CPU0]   Module[3]: name='/drivers/ps2_kbd' cmdline='' size=38760 bytes
[39232066566] [[34mDEBUG[0m] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[39239229414] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: bootstrap_cpu start on CPU 1
[39241706196] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: run_scheduler entry on CPU 2
[39243504795] [[34mDEBUG[0m] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle thread 2 and is now schedulable
[39250306491] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Applying 1 deferred inserts
[39252113769] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: bootstrap_cpu start on CPU 2
[39254573457] [[35mTRACE[0m] [kernel] [CPU0]   Module[4]: name='/bin/sh' cmdline='init' size=121736 bytes
[39260396868] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserting TID=4
[39262267836] [[34mDEBUG[0m] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle thread 3 and is now schedulable
[39273036066] [[35mTRACE[0m] [kernel] [CPU0]   Module[5]: name='/bin/ls' cmdline='init' size=62192 bytes
[39281547822] [[35mTRACE[0m] [kernel] [CPU0]   Module[6]: name='/bin/ln' cmdline='init' size=47624 bytes
[39288082086] [[35mTRACE[0m] [kernel] [CPU0]   Module[7]: name='/bin/ps' cmdline='init' size=50608 bytes
[39292965228] [[35mTRACE[0m] [kernel] [CPU0]   Module[8]: name='/bin/kill' cmdline='init' size=51552 bytes
[39296970603] [[35mTRACE[0m] [kernel] [CPU0]   Module[9]: name='/bin/killall' cmdline='init' size=67712 bytes
[39301390194] [[35mTRACE[0m] [kernel] [CPU0]   Module[10]: name='/bin/cat' cmdline='init' size=46168 bytes
[39309049461] [[34mDEBUG[0m] [kernel::sched] [CPU3] REGISTRY: Inserts applied
[39314413050] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[39317268144] [[35mTRACE[0m] [kernel] [CPU0]   Module[11]: name='/bin/head' cmdline='init' size=56168 bytes
[39322173165] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: run_scheduler entry on CPU 3
[39325152174] [[35mTRACE[0m] [kernel] [CPU0]   Module[12]: name='/bin/tail' cmdline='init' size=52048 bytes
[39330346110] [[35mTRACE[0m] [kernel] [CPU0]   Module[13]: name='/bin/top' cmdline='init' size=65544 bytes
[39334349736] [[35mTRACE[0m] [kernel] [CPU0]   Module[14]: name='/bin/wc' cmdline='init' size=56312 bytes
[39347439483] [[35mTRACE[0m] [kernel] [CPU0]   Module[15]: name='/bin/yes' cmdline='init' size=40480 bytes
[39353165082] [[35mTRACE[0m] [kernel] [CPU0]   Module[16]: name='/bin/which' cmdline='init' size=46408 bytes
[39363739998] [[35mTRACE[0m] [kernel] [CPU0]   Module[17]: name='/bin/cp' cmdline='init' size=46376 bytes
[39369908985] [[35mTRACE[0m] [kernel] [CPU0]   Module[18]: name='/bin/mv' cmdline='init' size=47104 bytes
[39374298348] [[35mTRACE[0m] [kernel] [CPU0]   Module[19]: name='/bin/rm' cmdline='init' size=47184 bytes
[39379713351] [[35mTRACE[0m] [kernel] [CPU0]   Module[20]: name='/bin/rmdir' cmdline='init' size=47208 bytes
[39383275998] [[35mTRACE[0m] [kernel] [CPU0]   Module[21]: name='/bin/mkdir' cmdline='init' size=47200 bytes
[39391273746] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: bootstrap_cpu start on CPU 3
[39396298458] [[35mTRACE[0m] [kernel] [CPU0]   Module[22]: name='/bin/mount' cmdline='init' size=57752 bytes
[39404897730] [[35mTRACE[0m] [kernel] [CPU0]   Module[23]: name='/bin/echo' cmdline='init' size=40584 bytes
[39407851824] [[34mDEBUG[0m] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle thread 4 and is now schedulable
[39415276989] [[35mTRACE[0m] [kernel] [CPU0]   Module[24]: name='/bin/grep' cmdline='init' size=78216 bytes
[39417673152] [[35mTRACE[0m] [kernel] [CPU0]   Module[25]: name='/bin/pwd' cmdline='init' size=35144 bytes
[39424947804] [[35mTRACE[0m] [kernel] [CPU0]   Module[26]: name='/bin/touch' cmdline='init' size=46216 bytes
[39428274831] [[35mTRACE[0m] [kernel] [CPU0]   Module[27]: name='/bin/setshell' cmdline='init' size=47104 bytes
[39430420128] [[35mTRACE[0m] [kernel] [CPU0]   Module[28]: name='/bin/dmesg' cmdline='init' size=39392 bytes
[39435121308] [[35mTRACE[0m] [kernel] [CPU0]   Module[29]: name='/bin/stat' cmdline='init' size=46264 bytes
[39441219048] [[35mTRACE[0m] [kernel] [CPU0]   Module[30]: name='/bin/file' cmdline='init' size=51160 bytes
[39445305603] [[35mTRACE[0m] [kernel] [CPU0]   Module[31]: name='/bin/dirname' cmdline='init' size=50160 bytes
[39448505217] [[35mTRACE[0m] [kernel] [CPU0]   Module[32]: name='/bin/basename' cmdline='init' size=50168 bytes
[39451185972] [[35mTRACE[0m] [kernel] [CPU0]   Module[33]: name='/bin/sleep' cmdline='init' size=56880 bytes
[39457423533] [[35mTRACE[0m] [kernel] [CPU0]   Module[34]: name='/bin/sort' cmdline='init' size=66400 bytes
[39459881538] [[35mTRACE[0m] [kernel] [CPU0]   Module[35]: name='/bin/env' cmdline='init' size=41792 bytes
[39462093594] [[35mTRACE[0m] [kernel] [CPU0]   Module[36]: name='/bin/uname' cmdline='' size=40600 bytes
[39464276016] [[35mTRACE[0m] [kernel] [CPU0]   Module[37]: name='/bin/true' cmdline='' size=20736 bytes
[39468284757] [[35mTRACE[0m] [kernel] [CPU0]   Module[38]: name='/bin/false' cmdline='' size=20736 bytes
[39473415234] [[35mTRACE[0m] [kernel] [CPU0]   Module[39]: name='/bin/input_echo' cmdline='' size=43096 bytes
[39475654218] [[35mTRACE[0m] [kernel] [CPU0]   Module[40]: name='/drivers/ps2_mouse' cmdline='' size=39312 bytes
[39477916863] [[35mTRACE[0m] [kernel] [CPU0]   Module[41]: name='/drivers/display_bootfb' cmdline='' size=81384 bytes
[39480258312] [[35mTRACE[0m] [kernel] [CPU0]   Module[42]: name='/drivers/display_virtio_gpu' cmdline='' size=128072 bytes
[39482665134] [[35mTRACE[0m] [kernel] [CPU0]   Module[43]: name='/bin/cambium' cmdline='' size=124728 bytes
[39484894284] [[35mTRACE[0m] [kernel] [CPU0]   Module[44]: name='/drivers/virtio_netd' cmdline='' size=106272 bytes
[39488926917] [[35mTRACE[0m] [kernel] [CPU0]   Module[45]: name='/drivers/rtl8168d' cmdline='' size=57360 bytes
[39492709311] [[35mTRACE[0m] [kernel] [CPU0]   Module[46]: name='/bin/netd' cmdline='' size=10813352 bytes
[39495058779] [[35mTRACE[0m] [kernel] [CPU0]   Module[47]: name='/bin/mesocarp' cmdline='' size=119480 bytes
[39497362938] [[35mTRACE[0m] [kernel] [CPU0]   Module[48]: name='/bin/mdns' cmdline='' size=119480 bytes
[39499881135] [[35mTRACE[0m] [kernel] [CPU0]   Module[49]: name='/bin/mdnsd' cmdline='' size=119480 bytes
[39506331513] [[35mTRACE[0m] [kernel] [CPU0]   Module[50]: name='/bin/fetchd' cmdline='' size=72720 bytes
[39510775491] [[35mTRACE[0m] [kernel] [CPU0]   Module[51]: name='/bin/httpsd' cmdline='' size=526160 bytes
[39512816673] [[35mTRACE[0m] [kernel] [CPU0]   Module[52]: name='/bin/find' cmdline='' size=57960 bytes
[39514861716] [[35mTRACE[0m] [kernel] [CPU0]   Module[53]: name='/bin/ip' cmdline='' size=55528 bytes
[39516877059] [[35mTRACE[0m] [kernel] [CPU0]   Module[54]: name='/bin/iso_reader' cmdline='' size=37504 bytes
[39519254676] [[35mTRACE[0m] [kernel] [CPU0]   Module[55]: name='/bin/ping' cmdline='' size=61144 bytes
[39522805575] [[35mTRACE[0m] [kernel] [CPU0]   Module[56]: name='/bin/nslookup' cmdline='' size=56480 bytes
[39525320307] [[35mTRACE[0m] [kernel] [CPU0]   Module[57]: name='/drivers/ahci_disk' cmdline='' size=74072 bytes
[39527643507] [[35mTRACE[0m] [kernel] [CPU0]   Module[58]: name='/drivers/ata_disk' cmdline='' size=61712 bytes
[39530431215] [[35mTRACE[0m] [kernel] [CPU0]   Module[59]: name='/bin/iso9660d' cmdline='' size=80648 bytes
[39532207275] [[35mTRACE[0m] [kernel] [CPU0]   Module[60]: name='/drivers/virtio_sound' cmdline='' size=95232 bytes
[39535598388] [[35mTRACE[0m] [kernel] [CPU0]   Module[61]: name='/drivers/hdaudio' cmdline='' size=67600 bytes
[39538690653] [[35mTRACE[0m] [kernel] [CPU0]   Module[62]: name='/drivers/pci_stubd' cmdline='' size=58224 bytes
[39540991281] [[35mTRACE[0m] [kernel] [CPU0]   Module[63]: name='/drivers/chime' cmdline='' size=55496 bytes
[39543262275] [[35mTRACE[0m] [kernel] [CPU0]   Module[64]: name='/bin/vfs_hello' cmdline='' size=35552 bytes
[39545492250] [[35mTRACE[0m] [kernel] [CPU0]   Module[65]: name='/bin/show_args' cmdline='' size=42568 bytes
[39547710873] [[35mTRACE[0m] [kernel] [CPU0]   Module[66]: name='/bin/env_roundtrip' cmdline='' size=44216 bytes
[39549986586] [[35mTRACE[0m] [kernel] [CPU0]   Module[67]: name='/bin/cwd_test' cmdline='' size=43776 bytes
[39552261144] [[35mTRACE[0m] [kernel] [CPU0]   Module[68]: name='/bin/date' cmdline='' size=62992 bytes
[39554467194] [[35mTRACE[0m] [kernel] [CPU0]   Module[69]: name='/bin/wayland_hello' cmdline='' size=65952 bytes
[39561956709] [[35mTRACE[0m] [kernel] [CPU0]   Module[70]: name='/bin/terminal' cmdline='' size=85376 bytes
[39564235128] [[35mTRACE[0m] [kernel] [CPU0]   Module[71]: name='/bin/tee' cmdline='' size=45032 bytes
[39566463684] [[35mTRACE[0m] [kernel] [CPU0]   Module[72]: name='/bin/xargs' cmdline='' size=55080 bytes
[39570266868] [[35mTRACE[0m] [kernel] [CPU0]   Module[73]: name='/bin/placed' cmdline='' size=38136 bytes
[39572445462] [[35mTRACE[0m] [kernel] [CPU0]   Module[74]: name='/bin/bloom' cmdline='' size=112872 bytes
[39574595742] [[35mTRACE[0m] [kernel] [CPU0]   Module[75]: name='/bin/clear' cmdline='' size=20864 bytes
[39576803079] [[35mTRACE[0m] [kernel] [CPU0]   Module[76]: name='/bin/loglevel' cmdline='' size=41600 bytes
[39579273789] [[35mTRACE[0m] [kernel] [CPU0]   Module[77]: name='/bin/poll_mux' cmdline='' size=39056 bytes
[39581479113] [[35mTRACE[0m] [kernel] [CPU0]   Module[78]: name='/bin/ipc_service_demo' cmdline='' size=47552 bytes
[39583771194] [[35mTRACE[0m] [kernel] [CPU0]   Module[79]: name='/bin/ipc_pipe_demo' cmdline='' size=44016 bytes
[39586021926] [[35mTRACE[0m] [kernel] [CPU0]   Module[80]: name='/bin/ipc_provider_demo' cmdline='' size=67008 bytes
[39590382447] [[35mTRACE[0m] [kernel] [CPU0]   Module[81]: name='/bin/ipc_memfd_demo' cmdline='' size=34216 bytes
[39592684725] [[35mTRACE[0m] [kernel] [CPU0]   Module[82]: name='/bin/test_exec' cmdline='' size=62504 bytes
[39594395643] [[35mTRACE[0m] [kernel] [CPU0]   Module[83]: name='/bin/test_vm_protect' cmdline='' size=34216 bytes
[39595908891] [[35mTRACE[0m] [kernel] [CPU0]   Module[84]: name='/bin/test_exec_env' cmdline='' size=54864 bytes
[39598362804] [[35mTRACE[0m] [kernel] [CPU0]   Module[85]: name='/bin/test_threads' cmdline='' size=78336 bytes
[39603176976] [[35mTRACE[0m] [kernel] [CPU0]   Module[86]: name='/bin/test_futex' cmdline='' size=48952 bytes
[39605499384] [[35mTRACE[0m] [kernel] [CPU0]   Module[87]: name='/bin/ld_so' cmdline='' size=378656 bytes
[39607685007] [[35mTRACE[0m] [kernel] [CPU0]   Module[88]: name='/bin/test_dyn_loader' cmdline='' size=53728 bytes
[39609961314] [[35mTRACE[0m] [kernel] [CPU0]   Module[89]: name='/bin/test_dlopen' cmdline='' size=60232 bytes
[39612507561] [[35mTRACE[0m] [kernel] [CPU0]   Module[90]: name='/bin/reboot' cmdline='' size=32136 bytes
[39614740110] [[35mTRACE[0m] [kernel] [CPU0]   Module[91]: name='/bin/shutdown' cmdline='' size=32136 bytes
[39616959393] [[35mTRACE[0m] [kernel] [CPU0]   Module[92]: name='/bin/attr_list' cmdline='' size=47456 bytes
[39619194945] [[35mTRACE[0m] [kernel] [CPU0]   Module[93]: name='/bin/attr_get' cmdline='' size=47552 bytes
[39623697201] [[35mTRACE[0m] [kernel] [CPU0]   Module[94]: name='/bin/attr_set' cmdline='' size=69656 bytes
[39625996641] [[35mTRACE[0m] [kernel] [CPU0]   Module[95]: name='/bin/attr_rm' cmdline='' size=47072 bytes
[39628285620] [[35mTRACE[0m] [kernel] [CPU0]   Module[96]: name='/lib/libpistil.so' cmdline='' size=67704 bytes
[39630610107] [[35mTRACE[0m] [kernel] [CPU0]   Module[97]: name='/share/wallpapers/clouds.bmp' cmdline='' size=786486 bytes
[39633196878] [[35mTRACE[0m] [kernel] [CPU0]   Module[98]: name='/share/wallpapers/flower.bmp' cmdline='' size=8386614 bytes
[39636288747] [[35mTRACE[0m] [kernel] [CPU0]   Module[99]: name='/share/wallpapers/leather.bmp' cmdline='' size=1179702 bytes
[39638051607] [[35mTRACE[0m] [kernel] [CPU0]   Module[100]: name='/share/wallpapers/linen.bmp' cmdline='' size=4718646 bytes
[39640595610] [[35mTRACE[0m] [kernel] [CPU0]   Module[101]: name='/share/fonts/NotoSans-Regular.ttf' cmdline='' size=569208 bytes
[39643176870] [[35mTRACE[0m] [kernel] [CPU0]   Module[102]: name='/share/fonts/unifont.hex' cmdline='' size=8355852 bytes
[39645640848] [[35mTRACE[0m] [kernel] [CPU0]   Module[103]: name='/etc/locale.conf' cmdline='' size=85 bytes
[39647949396] [[35mTRACE[0m] [kernel] [CPU0]   Module[104]: name='/etc/profile' cmdline='' size=68 bytes
[39650181879] [[35mTRACE[0m] [kernel] [CPU0]   Module[105]: name='/etc/motd' cmdline='' size=610 bytes
[39652763601] [[35mTRACE[0m] [kernel] [CPU0]   Module[106]: name='/etc/fstab' cmdline='' size=123 bytes
[39656380896] [[35mTRACE[0m] [kernel] [CPU0]   Module[107]: name='/etc/hostname' cmdline='' size=8 bytes
[39673381770] [[34mDEBUG[0m] [kernel] [CPU0] Found init module: /bin/sprout (cmdline: 'init'), loading...
[39681248244] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/sprout' (len=85144, base=0x200000)
[39684706215] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[39703321053] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[39707073054] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[39713105454] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [00, 00, 00, e8, e0, 7b, 00, 00]
[39723528339] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20c000 exec=false
[39726240378] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20e000 exec=false
[39731630928] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x20f000 filesz=0 memsz=0 align=1
[39742188915] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='load_init_module' elapsed_ticks=61380660 elapsed_us=30690 total_ticks=1235836404 total_us=617918
[39746857755] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='prepare_boot_registry' elapsed_ticks=1184403 elapsed_us=592 total_ticks=1240516959 total_us=620258
[39750722715] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='map_boot_registry' elapsed_ticks=207900 elapsed_us=103 total_ticks=1244381787 total_us=622190
[39755333112] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='activate_init_aspace' elapsed_ticks=278190 elapsed_us=139 total_ticks=1248914997 total_us=624457
[39758680863] [[34mDEBUG[0m] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[39760009839] [[34mDEBUG[0m] [kernel] [CPU0] Spawning init process...
[39764264463] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb00661e0 arg=0xffffffffb001fd20
[39792764154] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[39794926017] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=5
[39796166454] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[39797505792] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='spawn_init_task' elapsed_ticks=37588782 elapsed_us=18794 total_ticks=1291159452 total_us=645579
[39866333430] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED: early-boot bringup complete; resuming steady-state SMP scheduling
[39868407777] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='end_bringup' elapsed_ticks=3604029 elapsed_us=1802 total_ticks=1362059325 total_us=681029
[39928260669] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1421545620 elapsed_us=710772
[39930962313] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[39957726204] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[39960815466] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] ENTER_USER: TID=5 entry_pc=200000 user_sp=800000
[40039551453] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpus_online=4
[40054726503] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=0 curr=Some(5) runq=1 runq_avg=2 runq_samples=1 ctxsw=1 idle2busy=0 tick=1 ipi=0 enq=3 deq=1 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[40063494174] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=1 curr=Some(2) runq=0 runq_avg=0 runq_samples=21 ctxsw=0 idle2busy=0 tick=15 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[40075363449] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=2 curr=Some(3) runq=0 runq_avg=0 runq_samples=21 ctxsw=0 idle2busy=0 tick=11 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[40089553053] [[35mTRACE[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=3 curr=Some(4) runq=0 runq_avg=0 runq_samples=21 ctxsw=0 idle2busy=0 tick=17 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[40118591502] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=0
[40128274131] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=5 buf_len=20
[40147894446] [[34mDEBUG[0m] [sprout] [CPU0] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7fef70 rip=0x20414c rflags=0x202
[40150804485] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[40153019445] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: About to create Supervisor...
[40155249387] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Listing /bin directory...
[40159915455] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/bin' tid=5
[40324005513] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Parsed command line: '    '
[40330576473] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: FORCING display=bootfb for diagnostic test!
[40333400613] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Supervisor created, calling run_forever...
[40335873072] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[40338394140] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[40340861649] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[40342844454] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/run/sprout/shell' tid=5
[40356084351] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/etc/default/shell' tid=5
[40371123012] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/bin/sh' tid=5
[40374648204] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU0] BootFs: EXACT match for 'bin/sh' at index 4
[40380956517] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[40392530442] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[40397299767] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=5
[40415724096] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU0] BootFs: EXACT match for 'bin/sh' at index 4
[40443832242] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'sh' (len=121736, base=0x200000)
[40445419641] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[40448279058] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[40449905859] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[40452794547] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [9c, b9, 00, 00, 48, 8d, 7c, 24]
[40465963263] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[40469807235] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=216000 exec=false
[40473061860] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x217000 filesz=0 memsz=0 align=1
[40482489069] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /bin/sh
[40485881931] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80050890 kstack_top=0xffffffffb009d500 arg=0xffffffffb006f660
[40494394050] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=6, applying inserts
[40496735994] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[40498577658] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=6
[40500427077] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[40502622237] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[40513206459] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SETUP_STDIO: stdin=Fd(3) stdout=Fd(4) stderr=Fd(5)
[40531386654] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 6
[40536274878] [[35mTRACE[0m] [bran::arch::x86_64] [CPU0] SMP: send_ipi cpu_index=3 apic_id=3 vector=0x30
[40538381961] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Task 6 woken, restoring IRQs
[40540616919] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Done for /bin/sh
[40547004960] [[35mTRACE[0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[40560645774] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[40566810438] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[40568705331] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] ENTER_USER: TID=6 entry_pc=200000 user_sp=800000
[40573216662] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=0
[40580725713] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 250ms so the prompt can take the foreground
[40582955490] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU3] sys_auxv_get: tid=6 buf_len=100
[40589212983] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=19
[40593067878] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[40595642241] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[40601860860] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=19
DEBUG: sh starting
[40627871361] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=33
[40631348043] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[40635147960] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[40641596589] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=33
DEBUG: sh sig handlers installed
[40648145934] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/run/motd' tid=6
[40659229644] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/etc/motd' tid=6
[40665915180] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'etc/motd' at index 105
[40680316479] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=610
[40682981724] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[40688935122] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[40692900534] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=610
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
[40706370243] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=23
[40708636947] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[40710783465] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[40715488803] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=23
DEBUG: sh motd printed
[40721487048] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=31
[40724154108] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[40727022039] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[40729643427] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=31
DEBUG: sh shell object created
[40733149215] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] VFS: sys_fs_open path='/etc/profile' tid=6
[40739889828] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'etc/profile' at index 104
[40787276343] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=25
[40790335839] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[40792836612] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[40795569177] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=25
DEBUG: sh profile loaded
[40811722314] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=97
[40814509197] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[40817001621] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[40819651290] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=97
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [40833720840] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 len=6
[40836435717] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 starting copyin
[40844080464] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU3] sys_fs_write: tid=6 fd=1 copyin ok
[40847290176] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console write by PID=6 len=6
[?25h[40853511732] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU3] vfs: /dev/console read by PID=6 len=1
[43218534051] [[32mINFO [0m] [kernel] [C
```
</details>
