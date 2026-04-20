# ✅ Scenario: Launch ps and verify reaping

> Last run: 2026-04-19 19:31:21

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 9712ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 202ms | - [📜](./02/serial.log) - |
| 3 | And I type "ps" on the serial console | ✅ | 501ms | - [📜](./03/serial.log) - |
| 4 | Then the serial output should contain "PID" | ✅ | 0ms | - - - |
| 5 | And the serial output should contain "sh" | ✅ | 0ms | - - - |
| 6 | And the serial output should contain "ps" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[27537673944] [[32mINFO [0m] [bran] [CPU0] Limine: FRAMEBUFFER_REQUEST fulfilled.
[27564073812] [[32mINFO [0m] [bran] [CPU0] BOOTFB: limine width=1920 height=1080 pitch=7680 bpp=32 model=RGB -> bpp_bytes=4 stride=7680
[27570227025] [[32mINFO [0m] [kernel] [CPU0] INIT_RUNTIME: type=bran::runtime::Runtime<bran::arch::x86_64::X86_64Runtime>
[27575566062] [[34mDEBUG[0m] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[27894694641] [[32mINFO [0m] [kernel] [CPU0] thing-os kernel starting...
[27898707672] [[34mDEBUG[0m] [bran::requests] [CPU0] Limine: Found 97 boot modules
[27902664240] [[34mDEBUG[0m] [kernel::memory] [CPU0] Memory map has 64 entries
[27905183130] [[34mDEBUG[0m] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[28238121780] [[34mDEBUG[0m] [kernel::memory] [CPU0] Frame allocator initialized with 479345 free frames
[28256698668] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[28270114917] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[28273621596] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[28276269978] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[28284068340] [[34mDEBUG[0m] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[28290446085] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[28293336456] [[34mDEBUG[0m] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[28295976984] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[28300200027] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Registers initialized
[28303082478] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[28306180254] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[28310425869] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: All pins masked
[28315031580] [[34mDEBUG[0m] [bran::arch] [CPU0] IOAPIC: Init complete
[28317373227] [[32mINFO [0m] [bran::arch::x86_64] [CPU0] [SERIAL] UART COM1 initialized (115200 8N1 FIFO-1 IRQ-on)
[28320631845] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[28376675349] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 16 MiB bootstrap)
[28386621549] [[32mINFO [0m] [kernel::vfs::devfs] [CPU0] devfs: set_boot_fb width=1920 height=1080 pitch=7680 resource_id=0xfb000000
[30714074226] [[32mINFO [0m] [kernel] [CPU0] Seeding entropy pool...
[30718551072] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[30722074515] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[30725364582] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[30728725302] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[30747965094] [[34mDEBUG[0m] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[30749953410] [[34mDEBUG[0m] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[30751620108] [[34mDEBUG[0m] [kernel::sched] [CPU0]   Allocating scheduler...
[30754942251] [[34mDEBUG[0m] [kernel::sched] [CPU0]   Leaking scheduler...
[30756781935] [[34mDEBUG[0m] [kernel::sched] [CPU0]   Initializing boot task...
[30760878753] [[34mDEBUG[0m] [kernel::sched] [CPU0]   Creating boot task...
[30777255960] [[34mDEBUG[0m] [kernel::sched] [CPU0]   Creating idle tasks...
[30780591567] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80053fb0 kstack_top=0xffffffffb080ac20 arg=0x0
[30786565326] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[30809336019] [[34mDEBUG[0m] [kernel::sched] [CPU0]   Boot task initialized
[30810853887] [[34mDEBUG[0m] [kernel::sched] [CPU0]   Storing scheduler pointer...
[30813749571] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[30816399999] [[32mINFO [0m] [kernel] [CPU0] Initializing VFS...
[30843697038] [[34mDEBUG[0m] [kernel::vfs] [CPU0] VFS: Created /dev/display directory
[30859292079] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted union filesystem at / (root)
[30863872347] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted devfs at /dev
[30866491689] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted procfs at /proc
[30868970847] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted sysfs at /sys
[30876088419] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /tmp
[30878350470] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /run
[30880442934] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /services
[30882635949] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /session
[30885152991] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /data
[30887092896] [[34mDEBUG[0m] [kernel] [CPU0] Scanning PCI bus...
[30892762329] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x29c0 at 00:00.0 class=060000 id=1
[30916094748] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1050 at 00:01.0 class=030000 id=2
[30940154124] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x1af4:0x1000 at 00:02.0 class=020000 id=3
[30943424919] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2918 at 00:1f.0 class=060100 id=4
[30951117186] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2922 at 00:1f.2 class=010601 id=5
[30956466717] [[34mDEBUG[0m] [kernel] [CPU0] PCI: Discovered 0x8086:0x2930 at 00:1f.3 class=0c0500 id=6
[30962858916] [[34mDEBUG[0m] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
LAPIC: calibrated 62835900 ticks/sec (delta=628359, ok=true) -> init_cnt=628359
[31009458150] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62835900 ticks/sec), init_cnt=628359 for 100Hz
[31013246880] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[31016127780] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[31019697027] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[31029907656] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[31065819807] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[31068517656] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] SMP: CPU 1 initializing preemption timer (vec=32 init_cnt=628359)
[31072595994] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[31074861708] [[34mDEBUG[0m] [kernel] [CPU1] SMP: kernel_secondary_entry arg_cpu=1 runtime_cpu=1
[31078418184] [[34mDEBUG[0m] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[31082454381] [[34mDEBUG[0m] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[31086623403] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU1] INIT KERNEL CTX: entry=0xffffffff80053fb0 kstack_top=0xffffffffb081cb40 arg=0x1
[31091163576] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] SCHED: Task 2 assigned to CPU 1
[31101416940] [[34mDEBUG[0m] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[31103394201] [[32mINFO [0m] [kernel::task] [CPU1] SMP: run_scheduler entry on CPU 1
[31106429508] [[32mINFO [0m] [kernel::task] [CPU1] SMP: bootstrap_cpu start on CPU 1
[31112980470] [[32mINFO [0m] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle thread 2 and is now schedulable
[31116627135] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] SMP: CPU 2 initializing preemption timer (vec=32 init_cnt=628359)
[31120503447] [[34mDEBUG[0m] [kernel] [CPU2] SMP: kernel_secondary_entry arg_cpu=2 runtime_cpu=2
[31123067349] [[34mDEBUG[0m] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[31125616434] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[31130356389] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[31149040593] [[34mDEBUG[0m] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[31153016862] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU2] INIT KERNEL CTX: entry=0xffffffff80053fb0 kstack_top=0xffffffffb082cea0 arg=0x2
[31157956764] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] SCHED: Task 3 assigned to CPU 2
[31162045728] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU3] SMP: CPU 3 initializing preemption timer (vec=32 init_cnt=628359)
[31166381631] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[31169463237] [[34mDEBUG[0m] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[31172216493] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[31175909589] [[34mDEBUG[0m] [kernel] [CPU3] SMP: kernel_secondary_entry arg_cpu=3 runtime_cpu=3
[31178756499] [[34mDEBUG[0m] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[31181181735] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[31183934628] [[32mINFO [0m] [kernel::task] [CPU2] SMP: run_scheduler entry on CPU 2
[31186525227] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[31189796583] [[32mINFO [0m] [kernel::task] [CPU2] SMP: bootstrap_cpu start on CPU 2
[31192237989] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80053fb0 kstack_top=0xffffffffb083d200 arg=0x3
[31196434764] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU3] SCHED: Task 4 assigned to CPU 3
[31203246360] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Enumerating 97 boot modules...
[31205692848] [[32mINFO [0m] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle thread 3 and is now schedulable
[31209743961] [[34mDEBUG[0m] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[31212381024] [[32mINFO [0m] [kernel::task] [CPU3] SMP: run_scheduler entry on CPU 3
[31215014556] [[34mDEBUG[0m] [kernel] [CPU0] Found init module: /bin/sprout (cmdline: 'init'), loading...
[31217236347] [[32mINFO [0m] [kernel::task] [CPU3] SMP: bootstrap_cpu start on CPU 3
[31219689402] [[32mINFO [0m] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle thread 4 and is now schedulable
[31223846346] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/sprout' (len=100760, base=0x200000)
[31243721619] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[31264933326] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: TLS block: tp=0x212000 filesz=0 memsz=0 align=1
[31276704261] [[34mDEBUG[0m] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[31279519425] [[32mINFO [0m] [kernel] [CPU0] Spawning init process...
[31283052603] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff80012eb0 kstack_top=0xffffffffb084d6e0 arg=0xffffffffb080afc0
[31307662287] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Sending post-unlock Resched IPI to CPU 2 for task 5
[31313338749] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[31321493577] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED-DBG: cpus_online=4
[31324111038] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[31327943163] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=0 curr=Some(0) runq=0 runq_avg=1 runq_samples=1 ctxsw=0 idle2busy=0 tick=1 ipi=0 enq=2 deq=1 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[31335786570] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=5 entry_pc=200000 user_sp=800000
[31342974465] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=1 curr=Some(2) runq=0 runq_avg=0 runq_samples=7 ctxsw=0 idle2busy=0 tick=4 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[31355711706] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=2 curr=Some(5) runq=0 runq_avg=0 runq_samples=5 ctxsw=1 idle2busy=1 tick=2 ipi=0 enq=2 deq=1 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[31361244948] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED-DBG: cpu=3 curr=Some(4) runq=0 runq_avg=0 runq_samples=4 ctxsw=0 idle2busy=0 tick=2 ipi=0 enq=1 deq=0 wake=0 lock_miss=0 lock_pending=0 lock_blocked=0
[31382683464] [[34mDEBUG[0m] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7fef60 rip=0x20403e rflags=0x206
[31386382929] [[32mINFO [0m] [sprout] [CPU2] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[31389273597] [[34mDEBUG[0m] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (VFS-native)
[31391787438] [[34mDEBUG[0m] [sprout::devtree] [CPU2] SPROUT: Reading HHDM offset from /sys/firmware/hhdm
[31496607417] [[34mDEBUG[0m] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[31515939444] [[34mDEBUG[0m] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context (graph discovery eradicated)
[31519518096] [[34mDEBUG[0m] [sprout::devtree] [CPU2] SPROUT: build() called (VFS-native, graph building skipped)
[31522412361] [[34mDEBUG[0m] [sprout] [CPU2] SPROUT: About to create Supervisor...
[31524859641] [[34mDEBUG[0m] [sprout] [CPU2] SPROUT: Listing /bin directory...
[31579160514] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Parsed command line: '    '
[31583327325] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: FORCING display=bootfb for diagnostic test!
[31588012038] [[34mDEBUG[0m] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[31591768923] [[34mDEBUG[0m] [sprout::supervisor] [CPU2] SPROUT: Supervisor session started (Sovereign mode)
[31595784429] [[34mDEBUG[0m] [sprout::supervisor] [CPU2] SPROUT: Discovery loop disabled in favor of cambium.
[31604148642] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=4096 port=0xffffffffb08571f0
[31609178370] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(0)
[31616609145] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=1 port_id=PortId(0) mode=Write
[31621119057] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=2 port_id=PortId(0) mode=Read
[31626939432] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Launching early serial shell...
[31646292678] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] SCHED: Task 6 (user thread) assigned to CPU 3
[31654806150] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] SCHED: Sending post-unlock Resched IPI to CPU 3 for task 6
[31660594647] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Launching cambium...
[31662956655] [[34mDEBUG[0m] [sprout::pipelines] [CPU3] SPROUT: Setting up serial shell on /dev/console...
[31666575468] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=4096 port=0xffffffffb0869350
[31670649747] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(1)
[31673494776] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=3 port_id=PortId(1) mode=Write
[31676597667] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=4 port_id=PortId(1) mode=Read
[31685562777] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/sh' at index 4
[31689497334] [[34mDEBUG[0m] [kernel::task::loader] [CPU2] LOADER: Loading module '/bin/cambium' (len=109840, base=0x200000)
[31694443869] [[34mDEBUG[0m] [kernel::task::loader] [CPU2] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[31716781701] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU3] BootFs: EXACT match for 'bin/sh' at index 4
[31719933135] [[34mDEBUG[0m] [kernel::task::loader] [CPU2] LOADER: TLS block: tp=0x214000 filesz=0 memsz=0 align=1
[31732532997] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU2] INIT KERNEL CTX: entry=0xffffffff80012eb0 kstack_top=0xffffffffb0897100 arg=0xffffffffb0869500
[31746720885] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: Loading module 'sh' (len=121088, base=0x200000)
[31756737837] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[31763321667] [[32mINFO [0m] [kernel::sched::spawn] [CPU2] SCHED: TID 7 ��� task '/bin/cambium' (pid=7 from boot module)
[31778628849] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Spawned cambium (PID=7)
[31781474307] [[34mDEBUG[0m] [kernel::task::loader] [CPU3] LOADER: TLS block: tp=0x217000 filesz=0 memsz=0 align=1
[31784940759] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[31787769354] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=7 entry_pc=200000 user_sp=800000
[31790697411] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Scheduling netd launch...
[31800132441] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU3] INIT KERNEL CTX: entry=0xffffffff80012eb0 kstack_top=0xffffffffb08af300 arg=0xffffffffb0869500
[31810620798] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user thread) assigned to CPU 3
[31816193673] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: poll_mux verification test disabled
[31820716257] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Bringing up graphics stack...
[31824509706] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_open: path='/dev/fb0' len=8 flags=0x0
[31828177986] [[34mDEBUG[0m] [cambium] [CPU1] CAMBIUM: starting device discovery manager (daemon mode)
[31831607742] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU2] devfs: lookup entry path='fb0' len=3
[31835678820] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU2] devfs: dynamic registry hit path='fb0'
[31838544210] [[34mDEBUG[0m] [sprout::pipelines] [CPU3] SPROUT: Spawned serial shell '/bin/sh' (PID=8)
[31842148536] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_open: resolved node for /dev/fb0 mode=0o20666 size=32 ino=4
[31846787379] [[34mDEBUG[0m] [kernel::sched] [CPU3] SCHED[TID6]: exited (code=0)
[31853341212] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_open: process info present for /dev/fb0
[31857721830] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] sys_fs_open: handle_table.open('/dev/fb0') -> 3
[31863021399] [[32mINFO [0m] [kernel::vfs::devfs] [CPU2] FbNode::read: off=0 n=32 buf_len=32 total=32
[31868488212] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=4096 port=0xffffffffb086ccd0
[31871754651] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(2)
[31874786493] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=5 port_id=PortId(2) mode=Write
[31878494109] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=6 port_id=PortId(2) mode=Read
[31883201163] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=4096 port=0xffffffffb086a8d0
[31886363916] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(3)
[31889000154] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ahci_disk' at index 52
[31891123638] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=7 port_id=PortId(3) mode=Write
[31894665528] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=8 port_id=PortId(3) mode=Read
[31908757254] [[32mINFO [0m] [sprout::pipelines] [CPU2] SPROUT: Launching display driver '/drivers/display_bootfb' (boot_fd=3, bind_id=45057)
[31914574461] [[34mDEBUG[0m] [kernel::task::loader] [CPU2] LOADER: Loading module '/drivers/display_bootfb' (len=81704, base=0x200000)
[31919393319] [[34mDEBUG[0m] [kernel::task::loader] [CPU2] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[31935419241] [[34mDEBUG[0m] [kernel::task::loader] [CPU2] LOADER: TLS block: tp=0x20e000 filesz=0 memsz=0 align=1
[31946232747] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU2] INIT KERNEL CTX: entry=0xffffffff80012eb0 kstack_top=0xffffffffb087da40 arg=0xffffffffb086aa40
[31955226996] [[32mINFO [0m] [kernel::sched::spawn] [CPU2] SCHED: TID 10 ��� task '/drivers/display_bootfb' (pid=10 from boot module)
[31988422752] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU2] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[31995019749] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU1] USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x3
[32000323047] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU2] ENTER_USER: TID=8 entry_pc=200000 user_sp=800000
[32003998290] [[34mDEBUG[0m] [bran::arch::x86_64] [CPU1] ENTER_USER: TID=10 entry_pc=200000 user_sp=800000
[32015606172] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: Starting VFS-native bootfb driver (v0.4.1) TID=10 PID=10
[32019695004] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: boot_arg=3
[32023950552] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: Mapping bootstrap memfd 3 size=4096...
[32028745419] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU2] BootFs: EXACT match for 'etc/motd' at index 95
[32032342980] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: vm_map success at 0x400000403000
  +--------------------------------------------------------------+
  | THING-OS  v0.1  (thingos ACT IV)                 2026-04-16  |
  |                                                              |
  | People, places, things                                       |
  |                                                              |
  |                                                              |
  | Try: ls /bin, ps, cat /version                               |
  +--------------------------------------------------------------+
[32116362498] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU2] BootFs: EXACT match for 'etc/profile' at index 94
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m⚡[0m [32168537247] [[34mDEBUG[0m] [sprout::supervisor] [CPU2] SPROUT: Waiting for display driver registration...
[32177473251] [[32mINFO [0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=4 -> fd=4 node=0xffffffffb086aa50 port=0xffffffffb0869350
[32185010748] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Bridged resp_port 4 -> FD 4 for task 'cambium'
[32189597781] [[32mINFO [0m] [kernel::handle::bridge] [CPU2] BRIDGE: handle=8 -> fd=5 node=0xffffffffb086b070 port=0xffffffffb086a8d0
[32194327011] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Bridged resp_port 8 -> FD 5 for task 'display'
[32256250191] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/ahci_disk' name='ahci_disk' class=Block kind='dev.storage.Ahci' start='thingos_driver_start_safe'
[32267251500] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/chime' at index 57
[32485520265] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/chime' name='chime' class=Audio kind='dev.sound.Chime' start='thingos_driver_start_safe'
[32495121417] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/display_bootfb' at index 40
[32650963125] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: Recovered handles: req_read=6, resp_write=7, svc=1, id=45057
[32656495641] [[32mINFO [0m] [display_bootfb::driver] [CPU1] display_bootfb: probing /dev/fb0...
[32659215798] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_open: path='/dev/fb0' len=8 flags=0x2
[32664042873] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU1] devfs: lookup entry path='fb0' len=3
[32667438441] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU1] devfs: dynamic registry hit path='fb0'
[32670948123] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_open: resolved node for /dev/fb0 mode=0o20666 size=32 ino=4
[32675372400] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Still waiting for display (step 0)...
[32678828259] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_open: process info present for /dev/fb0
[32682782913] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Health check: Loop still running, tasks=3
[32686405059] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU1] sys_fs_open: handle_table.open('/dev/fb0') -> 4
[32691220353] [[32mINFO [0m] [kernel::vfs::devfs] [CPU1] FbNode::read: off=0 n=32 buf_len=32 total=32
[32696137650] [[32mINFO [0m] [kernel::syscall::handlers::memory] [CPU1] sys_vm_map: large backing file mapping offset=0 len=8294400 total_size=8294400
[32824757394] [[34mDEBUG[0m] [display_bootfb] [CPU1] display_bootfb: Driver initialized successfully (1920x1080)
[32827635390] [[34mDEBUG[0m] [display_bootfb] [CPU1] display_bootfb: Mapping framebuffer (backing fd=3)...
[32834584398] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: capacity=65536 port=0xffffffffb08866f0
[32837098635] [[34mDEBUG[0m] [kernel::ipc] [CPU1] CREATE_PORT: appended id=PortId(4)
[32839211823] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=9 port_id=PortId(4) mode=Write
[32841960327] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU1] ALLOC_HANDLE: handle=10 port_id=PortId(4) mode=Read
[32844961314] [[32mINFO [0m] [kernel::handle::bridge] [CPU1] BRIDGE: handle=7 -> fd=4 node=0xffffffffb080afd0 port=0xffffffffb086a8d0
[32848901019] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: Sending MSG_BIND_READY handshake (class_mask=0x3)...
[32857569822] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: Sent MSG_BIND_READY (result=Ok(())), waiting for MSG_BIND_ASSIGNED...
[32868533049] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_bootfb' name='display_bootfb' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[32874295971] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/display_virtio_gpu' at index 41
[33053132211] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Recvmsg SUCCESS from display: n=28, nfds=1
[33056328492] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/display_virtio_gpu' name='display_virtio_gpu' class=Display kind='dev.display.Gpu' start='thingos_driver_start_safe'
[33060645948] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: BIND_READY from display (bundle_fd=6)
[33064339341] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/hdaudio' at index 55
[33070564659] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: capacity=262400 port=0xffffffffb0886bb0
[33073024677] [[34mDEBUG[0m] [kernel::ipc] [CPU2] CREATE_PORT: appended id=PortId(5)
[33076064703] [[34mDEBUG[0m] [kernel::ipc::handles] [CPU2] ALLOC_HANDLE: handle=11 port_id=PortId(5) mode=Write
[33084374103] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU2] vfs: mounted userland provider at /dev/display/card0 (flags: 0x0)
[33090296448] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Sovereign mount success: display -> /dev/display/card0
[33179700048] [[34mDEBUG[0m] [display_bootfb] [CPU1] display_bootfb: Sovereign registration COMPLETE. Assigned: /dev/display/card0
[33186067101] [[34mDEBUG[0m] [display_bootfb] [CPU1] display_bootfb: Sent MSG_SERVICE_READY.
[33190282851] [[32mINFO [0m] [display_bootfb] [CPU1] display_bootfb: entering VFS provider service loop
[33304637322] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/hdaudio' name='hdaudio' class=Audio kind='dev.sound.Hda' start='thingos_driver_start_safe'
[33314794227] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/pci_stubd' at index 56
[33487830585] [[34mDEBUG[0m] [cambium::catalog] [CPU1] DEVD CATALOG: registered driver '/drivers/pci_stubd' name='pci_stubd' class=Other kind='drv.PciStubd' start='thingos_driver_start_safe'
[33492742932] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU1] BootFs: EXACT match for 'drivers/ps2_kbd' at index 3
[33590597403] [[32mINFO [0m] [sprout::pipelines] [CPU3] SPROUT: Waiting for /dev/net/virtio0/rx before applying /etc/fstab mounts...
[33595302048] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Display card0 detected. Proceeding.
[33599133282] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Launching UI services...
[33602527761] [[32mINFO [0m] [sprout::pipelines] [CPU2] SPROUT: Input driver startup is disabled (network-only mode)
[33608455353] [[34mDEBUG[0m] [kernel::task::loader] [CPU2] LOADER: Loading module '/bin/bloom' (len=108600, base=0x200000)
[33612706479] [[34mDEBUG[0m] [kernel::task::loader] [CPU2] LOADER: ELF info: entry=200000, min_vaddr=200000, bias=0, entry_pc=200000
[33633173838] [[34mDEBUG[0m] [kernel::task::loader] [CPU2] LOADER: TLS block: tp=0x214000 filesz=0 memsz=0 align=1
[33642788355] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU2] INIT KERNEL CTX: entry=0xffffffff80012eb0 kstack_top=0xffffffffb09075c0 arg=0xffffffffb08695a0
[33
```
</details>
