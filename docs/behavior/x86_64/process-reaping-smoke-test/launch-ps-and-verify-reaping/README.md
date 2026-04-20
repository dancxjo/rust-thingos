# ✅ Scenario: Launch ps and verify reaping

> Last run: 2026-04-19 19:23:16

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8807ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 202ms | - [📜](./02/serial.log) - |
| 3 | And I type "ps" on the serial console | ✅ | 503ms | - [📜](./03/serial.log) - |
| 4 | Then the serial output should contain "PID" | ✅ | 0ms | - - - |
| 5 | And the serial output should contain "sh" | ✅ | 0ms | - - - |
| 6 | And the serial output should contain "ps" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[25450043751] [[32mINFO [0m] [bran] [CPU0] Limine: FRAMEBUFFER_REQUEST fulfilled.
[25467190188] [[32mINFO [0m] [bran] [CPU0] BOOTFB: limine width=1920 height=1080 pitch=7680 bpp=32 model=RGB -> bpp_bytes=4 stride=7680
[25471308126] [[32mINFO [0m] [kernel] [CPU0] INIT_RUNTIME: type=bran::runtime::Runtime<bran::arch::x86_64::X86_64Runtime>
[25474707852] [[34mDEBUG[0m] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[25759017834] [[32mINFO [0m] [kernel] [CPU0] thing-os kernel starting...
[25762707927] [[34mDEBUG[0m] [bran::requests] [CPU0] Limine: Found 97 boot modules
[25766551503] [[34mDEBUG[0m] [kernel::memory] [CPU0] Memory map has 64 entries
[25768989213] [[34mDEBUG[0m] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[26010216441] [[34mDEBUG[0m] [kernel::memory] [CPU0] Frame allocator initialized with 479349 free frames
[26022738093] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[26031438675] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[26033674689] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[26035282020] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[26040236046] [[34mDEBUG[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[26044445262] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[26046261615] [[34mDEBUG[0m] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[26047923297] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[26050506603] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Registers initialized
[26052462843] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[26054390010] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[26057130825] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: All pins masked
[26060595330] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Init complete
[26062065282] [[32mINFO [0m] [bran::arch::x86_64] [CPU0] [SERIAL] UART COM1 initialized (115200 8N1 FIFO-1 IRQ-on)
[26064066171] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26096366175] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 16 MiB bootstrap)
[26102728773] [[32mINFO [0m] [kernel::vfs::devfs] [CPU0] devfs: set_boot_fb width=1920 height=1080 pitch=7680 resource_id=0xfb000000
[27982204503] [[32mINFO [0m] [kernel] [CPU0] Seeding entropy pool...
[27985606176] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[27987651912] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[27989519679] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[27992082855] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[28007659845] [[34mDEBUG[0m] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[28009479399] [[34mDEBUG[0m] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[28011353403] [[34mDEBUG[0m] [kernel::sched] [CPU0]   Allocating scheduler...
[28014976671] [[34mDEBUG[0m] [kernel::sched] [CPU0]   Leaking scheduler...
[28016272977] [[34mDEBUG[0m] [kernel::sched] [CPU0]   Initializing boot task...
[28020110877] [[34mDEBUG[0m] [kernel::sched] [CPU0]   Creating boot task...
[28032549006] [[34mDEBUG[0m] [kernel::sched] [CPU0]   Creating idle tasks...
[28034614410] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff800539a0 kstack_top=0xffffffffb080ac20 arg=0x0
[28038405747] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[28057427079] [[34mDEBUG[0m] [kernel::sched] [CPU0]   Boot task initialized
[28058764701] [[34mDEBUG[0m] [kernel::sched] [CPU0]   Storing scheduler pointer...
[28060679493] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[28062396153] [[32mINFO [0m] [kernel] [CPU0] Initializing VFS...
[28080965880] [[34mDEBUG[0m] [kernel::vfs] [CPU0] VFS: Created /dev/display directory
[28092512910] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted union filesystem at / (root)
[28095817200] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted devfs at /dev
[28097633619] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted procfs at /proc
[28099409514] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted sysfs at /sys
[28104236193] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /tmp
[28105807785] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /run
[28107291597] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /services
[28108808079] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /session
[28110426135] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /data
[28111733925] [[34mDEBUG[0m] [kernel] [CPU0] Scanning PCI bus...
[28116172821] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x29c0 at 00:00.0 class=060000 id=1
[28134411162] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1050 at 00:01.0 class=030000 id=2
[28153938813] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1000 at 00:02.0 class=020000 id=3
[28160601150] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2918 at 00:1f.0 class=060100 id=4
[28165727865] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2922 at 00:1f.2 class=010601 id=5
[28169116635] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2930 at 00:1f.3 class=0c0500 id=6
[28172730432] [[34mDEBUG[0m] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
LAPIC: calibrated 63241600 ticks/sec (delta=632416, ok=true) -> init_cnt=632416
[28212073362] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (63241600 ticks/sec), init_cnt=632416 for 100Hz
[28214586279] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[28216309143] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[28218469290] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[28224708402] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[28264392123] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[28266777363] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[28268483133] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] SMP: CPU 1 initializing preemption timer (vec=32 init_cnt=632416)
[28271627637] [[34mDEBUG[0m] [kernel] [CPU1] SMP: kernel_secondary_entry arg_cpu=1 runtime_cpu=1
[28273576155] [[34mDEBUG[0m] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[28276114713] [[34mDEBUG[0m] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[28280022672] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff800539a0 kstack_top=0xffffffffb081cb40 arg=0x1
[28283874234] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Task 2 assigned to CPU 1
[28290496278] [[34mDEBUG[0m] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[28292438526] [[32mINFO [0m] [kernel::task] [CPU1] SMP: run_scheduler entry on CPU 1
[28294484790] [[32mINFO [0m] [kernel::task] [CPU1] SMP: bootstrap_cpu start on CPU 1
[28298463072] [[32mINFO [0m] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle thread 2 and is now schedulable
[28306442868] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] SMP: CPU 2 initializing preemption timer (vec=32 init_cnt=632416)
[28311835926] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[28313797215] [[34mDEBUG[0m] [kernel] [CPU2] SMP: kernel_secondary_entry arg_cpu=2 runtime_cpu=2
[28315650660] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[28317155328] [[34mDEBUG[0m] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[28322825091] [[34mDEBUG[0m] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[28325116710] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU2] INIT KERNEL CTX: entry=0xffffffff800539a0 kstack_top=0xffffffffb082cea0 arg=0x2
[28327813602] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] SCHED: Task 3 assigned to CPU 2
[28330671303] [[34mDEBUG[0m] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[28332319158] [[32mINFO [0m] [kernel::task] [CPU2] SMP: run_scheduler entry on CPU 2
[28334120628] [[32mINFO [0m] [kernel::task] [CPU2] SMP: bootstrap_cpu start on CPU 2
[28335853095] [[32mINFO [0m] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle thread 3 and is now schedulable
[28348604559] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[28351164765] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] SMP: CPU 3 initializing preemption timer (vec=32 init_cnt=632416)
[28354873965] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[28357446414] [[34mDEBUG[0m] [kernel] [CPU3] SMP: kernel_secondary_entry arg_cpu=3 runtime_cpu=3
[28360638702] [[34mDEBUG[0m] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[28363143303] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[28365072912] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[28368922758] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff800539a0 kstack_top=0xffffffffb083d200 arg=0x3
[28371551868] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SCHED: Task 4 assigned to CPU 3
[28374289845] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[28375879455] [[32mINFO [0m] [kernel::task] [CPU3] SMP: run_scheduler entry on CPU 3
[28377678450] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Enumerating 97 boot modules...
[28379502393] [[32mINFO [0m] [kernel::task] [CPU3] SMP: bootstrap_cpu start on CPU 3
[28382405799] [[32mINFO [0m] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle thread 4 and is now schedulable
[28386713487] [[34mDEBUG[0m] [kernel] [CPU0] Found init module: /bin/sprout (cmdline: 'init'), loading...
[28389758364] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/sprout' (len=100760, base=0x200000)
[28399866099] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[28424512974] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x212000 filesz=0 memsz=0 align=1
[28434567909] [[34mDEBUG[0m] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[28436545830] [[32mINFO [0m] [kernel] [CPU0] Spawning init process...
[28440059010] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80012e40 kstack_top=0xffffffffb084d6e0 arg=0xffffffffb080afc0
[28464258240] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Sending post-unlock Resched IPI to CPU 2 for task 5
[28467713208] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[28472448807] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED-DBG: cpus_online=4
[28474000500] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[28476581001] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=5 entry_pc=200000 user_sp=800000
[28478372307] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=0 curr=Some(0) runq=0 runq_avg=1 runq_samples=1 ctxsw=0 idle2busy=0 tick=1 ipi=0 enq=2 deq=1 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[28482792096] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=1 curr=Some(2) runq=0 runq_avg=0 runq_samples=6 ctxsw=0 idle2busy=0 tick=1 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[28486577229] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=2 curr=Some(5) runq=0 runq_avg=0 runq_samples=5 ctxsw=1 idle2busy=1 tick=3 ipi=0 enq=2 deq=1 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[28490799744] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=3 curr=Some(4) runq=0 runq_avg=0 runq_samples=4 ctxsw=0 idle2busy=0 tick=2 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[28513647657] [[34mDEBUG[0m] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7fef60 rip=0x20403e rflags=0x206
[28551752229] [[32mINFO [0m] [sprout] [CPU2] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[28554240132] [[34mDEBUG[0m] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (VFS-native)
[28556497728] [[34mDEBUG[0m] [sprout::devtree] [CPU2] SPROUT: Reading HHDM offset from /sys/firmware/hhdm
[28594162080] [[34mDEBUG[0m] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[28606263609] [[34mDEBUG[0m] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context (graph discovery eradicated)
[28609206582] [[34mDEBUG[0m] [sprout::devtree] [CPU2] SPROUT: build() called (VFS-native, graph building skipped)
[28612099560] [[34mDEBUG[0m] [sprout] [CPU2] SPROUT: About to create Supervisor...
[28615635675] [[34mDEBUG[0m] [sprout] [CPU2] SPROUT: Listing /bin directory...
[28652388732] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Parsed command line: '    '
[28655877162] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: FORCING display=bootfb for diagnostic test!
[28658877456] [[34mDEBUG[0m] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[28661682456] [[34mDEBUG[0m] [sprout::supervisor] [CPU2] SPROUT: Supervisor session started (Sovereign mode)
[28664256357] [[34mDEBUG[0m] [sprout::supervisor] [CPU2] SPROUT: Discovery loop disabled in favor of cambium.
[28673053959] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=4096 port=0xffffffffb08571f0
[28678436028] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(0)
[28686460110] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=1 port_id=PortId(0) mode=Write
[28690421892] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=2 port_id=PortId(0) mode=Read
[28696388820] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Launching early serial shell...
[28712889909] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] SCHED: Task 6 (user thread) assigned to CPU 3
[28719924849] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] SCHED: Sending post-unlock Resched IPI to CPU 3 for task 6
[28725612696] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Launching cambium...
[28727634738] [[34mDEBUG[0m] [sprout::pipelines] [CPU3] SPROUT: Setting up serial shell on /dev/console...
[28730021100] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=4096 port=0xffffffffb0869350
[28732521114] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(1)
[28734689742] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(1) mode=Write
[28737262092] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=4 port_id=PortId(1) mode=Read
[28743867108] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/sh' at index 4
[28746056988] [[34mDEBUG[0m] [kernel::task::loader] [CPU2] LOADER: Loading module '/bin/cambium' (len=109840, base=0x200000)
[28748994186] [[34mDEBUG[0m] [kernel::task::loader] [CPU2] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[28759487295] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/sh' at index 4
[28763791518] [[34mDEBUG[0m] [kernel::task::loader] [CPU2] LOADER: TLS block: tp=0x214000 filesz=0 memsz=0 align=1
[28772959809] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU2] INIT KERNEL CTX: entry=0xffffffff80012e40 kstack_top=0xffffffffb08971c0 arg=0xffffffffb0869500
[28778949837] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: Loading module 'sh' (len=121088, base=0x200000)
[28782162849] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[28793701398] [[32mINFO [0m] [kernel::sched::spawn] [CPU2] SCHED: TID 7 �� task '/bin/cambium' (pid=7 from boot module)
[28797380931] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: TLS block: tp=0x217000 filesz=0 memsz=0 align=1
[28805029638] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80012e40 kstack_top=0xffffffffb08af1e0 arg=0xffffffffb08695a0
[28808242089] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Spawned cambium (PID=7)
[28811162226] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Scheduling netd launch...
[28816725927] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[28820049654] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user thread) assigned to CPU 3
[28822125354] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=7 entry_pc=200000 user_sp=800000
[28826108982] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: poll_mux verification test disabled
[28828608270] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Bringing up graphics stack...
[28830952128] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_open: path='/dev/fb0' len=8 flags=0x0
[28834723269] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU2] devfs: lookup entry path='fb0' len=3
[28837201305] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU2] devfs: dynamic registry hit path='fb0'
[28840436724] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_open: resolved node for /dev/fb0 mode=0o20666 size=32 ino=4
[28843411080] [[34mDEBUG[0m] [sprout::pipelines] [CPU3] SPROUT: Spawned serial shell '/bin/sh' (PID=8)
[28845492984] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_open: process info present for /dev/fb0
[28847743881] [[34mDEBUG[0m] [kernel::sched] [CPU3] SCHED[TID6]: exited (code=0)
[28849419984] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_open: handle_table.open('/dev/fb0') -> 3
[28856624841] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[28872330960] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[28880298777] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=8 entry_pc=200000 user_sp=800000
[28895609787] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU2] BootFs: EXACT match for 'etc/motd' at index 95
[28902014097] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ahci_disk' at index 52
  +--------------------------------------------------------------+
  | THING-OS  v0.1  (thingos ACT IV)                 2026-04-16  |
  |                                                              |
  | People, places, things                                       |
  |                                                              |
  |                                                              |
  | Try: ls /bin, ps, cat /version                               |
  +--------------------------------------------------------------+
[28953470700] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU2] BootFs: EXACT match for 'etc/profile' at index 94
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m���[0m [28995478215] [[32mINFO [0m] [kernel::vfs::devfs] [CPU2] FbNode::read: off=0 n=32 buf_len=32 total=32
[29000142996] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=4096 port=0xffffffffb0869790
[29002376865] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(2)
[29004216879] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=5 port_id=PortId(2) mode=Write
[29006754975] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=6 port_id=PortId(2) mode=Read
[29009692305] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=4096 port=0xffffffffb086a710
[29011859085] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(3)
[29013679761] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=7 port_id=PortId(3) mode=Write
[29015845947] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=8 port_id=PortId(3) mode=Read
[29026226361] [[32mINFO [0m] [sprout::pipelines] [CPU2] SPROUT: Launching display driver '/drivers/display_bootfb' (boot_fd=3, bind_id=45057)
[29030280246] [[34mDEBUG[0m] [kernel::task::loader] [CPU2] LOADER: Loading module '/drivers/display_bootfb' (len=81704, base=0x200000)
[29033320338] [[34mDEBUG[0m] [kernel::task::loader] [CPU2] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[29044745961] [[34mDEBUG[0m] [kernel::task::loader] [CPU2] LOADER: TLS block: tp=0x20e000 filesz=0 memsz=0 align=1
[29051744667] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU2] INIT KERNEL CTX: entry=0xffffffff80012e40 kstack_top=0xffffffffb087dae0 arg=0xffffffffb0869980
[29059204713] [[32mINFO [0m] [kernel::sched::spawn] [CPU2] SCHED: TID 10 ��� task '/drivers/display_bootfb' (pid=10 from boot module)
[29071897932] [[34mDEBUG[0m] [sprout::supervisor] [CPU2] SPROUT: Waiting for display driver registration...
[29078122260] [[32mINFO [0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=4 -> fd=4 node=0xffffffffb086a950 port=0xffffffffb0869350
[29083596828] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Bridged resp_port 4 -> FD 4 for task 'cambium'
[29086998270] [[32mINFO [0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=8 -> fd=5 node=0xffffffffb086ad50 port=0xffffffffb086a710
[29090497260] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Bridged resp_port 8 -> FD 5 for task 'display'
[29136375147] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[29143097577] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/chime' at index 57
[29304995610] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[29311021641] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/display_bootfb' at index 40
[29448288408] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Still waiting for display (step 0)...
[29451478056] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Health check: Loop still running, tasks=3
[29478319860] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[29483844918] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/display_virtio_gpu' at index 41
[29655030999] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x3
[29658160884] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=10 entry_pc=200000 user_sp=800000
[29665352937] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: Starting VFS-native bootfb driver (v0.4.1) TID=10 PID=10
[29669227203] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: boot_arg=3
[29671793877] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: Mapping bootstrap memfd 3 size=4096...
[29675351739] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: vm_map success at 0x400000403000
[29677870497] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: Recovered handles: req_read=6, resp_write=7, svc=1, id=45057
[29681014638] [[32mINFO [0m] [display_bootfb::driver] [CPU1] display_bootfb: probing /dev/fb0...
[29683203000] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_open: path='/dev/fb0' len=8 flags=0x2
[29686349418] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU1] devfs: lookup entry path='fb0' len=3
[29688629487] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU1] devfs: dynamic registry hit path='fb0'
[29690957505] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_open: resolved node for /dev/fb0 mode=0o20666 size=32 ino=4
[29694168966] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_open: process info present for /dev/fb0
[29696859357] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_open: handle_table.open('/dev/fb0') -> 4
[29699620863] [[32mINFO [0m] [kernel::vfs::devfs] [CPU1] FbNode::read: off=0 n=32 buf_len=32 total=32
[29702246013] [[32mINFO [0m] [kernel::syscall::handlers::memory] [CPU1] sys_vm_map: large backing file mapping offset=0 len=8294400 total_size=8294400
[29720264541] [[34mDEBUG[0m] [display_bootfb] [CPU1] display_bootfb: Driver initialized successfully (1920x1080)
[29723916387] [[34mDEBUG[0m] [display_bootfb] [CPU1] display_bootfb: Mapping framebuffer (backing fd=3)...
[29729965353] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=65536 port=0xffffffffb0886750
[29732063064] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(4)
[29734057518] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=9 port_id=PortId(4) mode=Write
[29736497109] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=10 port_id=PortId(4) mode=Read
[29739899343] [[32mINFO [0m] [kernel::handle::bridge] [CPU1] BRIDGE: handle=7 -> fd=4 node=0xffffffffb080afd0 port=0xffffffffb086a710
[29744325567] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: Sending MSG_BIND_READY handshake (class_mask=0x3)...
[29753037798] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: Sent MSG_BIND_READY (result=Ok(())), waiting for MSG_BIND_ASSIGNED...
[29763906612] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[29770726623] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/hdaudio' at index 55
[29855346015] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Recvmsg SUCCESS from display: n=28, nfds=1
[29858382972] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: BIND_READY from display (bundle_fd=6)
[29867776686] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=262400 port=0xffffffffb0886bd0
[29870020191] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(5)
[29872742262] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=11 port_id=PortId(5) mode=Write
[29879761956] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] vfs: mounted userland provider at /dev/display/card0 (flags: 0x0)
[29883996318] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Sovereign mount success: display -> /dev/display/card0
[29944124694] [[34mDEBUG[0m] [display_bootfb] [CPU1] display_bootfb: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[29948370837] [[34mDEBUG[0m] [display_bootfb] [CPU1] display_bootfb: Sent MSG_SERVICE_READY.
[29950487556] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: entering VFS provider service loop
[29977955238] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[29984230683] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/pci_stubd' at index 56
[30141751563] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[30147112380] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ps2_kbd' at index 3
[30321849822] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Display card0 detected. Proceeding.
[30323865033] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_kbd' name='ps2_kbd' class=Input kind='drv.Ps2Keyboard' start='thingos_driver_start_safe'
[30327134739] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Launching UI services...
[30329123022] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ps2_mouse' at index 39
[30331176843] [[32mINFO [0m] [sprout::pipelines] [CPU2] SPROUT: Input driver startup is disabled (network-only mode)
[30334511658] [[34mDEBUG[0m] [kernel::task::loader] [CPU2] LOADER: Loading module '/bin/bloom' (len=108600, base=0x200000)
[30337296858] [[34mDEBUG[0m] [kernel::task::loader] [CPU2] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[30352075710] [[34mDEBUG[0m] [kernel::task::loader] [CPU2] LOADER: TLS block: tp=0x214000 filesz=0 memsz=0 align=1
[30358728741] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU2] INIT KERNEL CTX: entry=0xffffffff80012e40 kstack_top=0xffffffffb08f7560 arg=0xffffffffb08695a0
[30379511547] [[32mINFO [0m] [kernel::sched::spawn] [CPU2] SCHED: TID 11 �� task '/bin/bloom' (pid=11 from boot module)
[30384641661] [[32mINFO [0m] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=11)
[30390070194] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Recvmsg SUCCESS from display: n=28, nfds=0
[30391930833] [[34mDEBUG[0m] [kernel::sched] [CPU3] SCHED: CPU 3 stole task 8 (prio 2) from CPU 2 (depth 3)
[30399118827] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[30401816247] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=11 entry_pc=200000 user_sp=800000
[30407607516] [[32mINFO [0m] [bloom] [CPU2] bloom: compositor service starting
[30482653146] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ps2_mouse' name='ps2_mouse' class=Input kind='drv.Ps2Mouse' start='thingos_driver_start_safe'
[30487632714] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/rtc_cmos' at index 2
[30517099536] [[32mINFO [0m] [sprout::pipelines] [CPU3] SPROUT: Waiting for /dev/net/virtio0/rx before applying /etc/fstab mounts...
[30660060684] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/rtc_cmos' name='rtc_cmos' class=Other kind='dev.rtc.Cmos' start='thingos_driver_start_safe'
[30665272605] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/rtl8168d' at index 44
[30822045276] [[34m
```
</details>
