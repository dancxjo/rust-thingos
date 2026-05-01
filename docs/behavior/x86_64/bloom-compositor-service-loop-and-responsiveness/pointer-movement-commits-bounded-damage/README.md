# ❌ Scenario: pointer movement commits bounded damage

> Last run: 2026-04-30 18:01:45

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ❌ | 31751ms | - [📜](./01/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[101829786630] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Framebuffer Initialized"
[101889303714] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] enter
[101896566255] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] phys_memory_map ok
[101900075409] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] modules ok
[101902491372] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] phys_to_virt_offset ok
[101903443092] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] memory map logging done
[101942884923] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] boot_frame_alloc init ok
[102276484827] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] frame allocator build ok
[102280838154] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] frame allocator log ok
[102283825083] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] FRAME_ALLOCATOR init ok
[102285233820] [[35mTRACE[0m] [kernel::memory] [CPU0] [kernel:mem:init] tasking init ok
[102286039944] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[102320034201] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[102325559424] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] enter
[102328353237] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] set expand hook
[102330298554] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] expand hook ok
[102331099695] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock begin
[102332390094] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock ok
[102333229251] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] reserve_region begin
[102392912160] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] reserve_region ok
[102396190017] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] inner allocator init begin
[102401963433] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] inner allocator init ok
[102404116188] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] heap top store ok
[102405070515] [[35mTRACE[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] init done
[102406189776] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[102455368488] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] devfs: set_boot_fb width=1920 height=1080 pitch=7680 resource_id=0xfb000000 phys=0x80000000
[102463057389] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs set_boot_fb elapsed_ticks=6791565 elapsed_us=3395
[102475154727] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs FbNode::new elapsed_ticks=9192843 elapsed_us=4596
[102477540429] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] register begin
[102487268037] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] register ok
[102491925921] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs register fb0 elapsed_ticks=14263194 elapsed_us=7131
[102494083362] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[102527839029] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[102532932414] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[102598295844] [[34mDEBUG[0m] [kernel] [CPU0] Seeding entropy pool...
[102604705137] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed begin
[102608834493] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] fill_entropy done
[102613709682] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] add_sample(timer) ok
[102615009255] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] mark_seeded(timer) ok
[102615856431] [[33mWARN [0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (weak entropy)
[102617711163] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[102621332748] [[35mTRACE[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed done
[102625816557] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] entropy::seed_from_hardware elapsed_ticks=20793498 elapsed_us=10396
[102629509092] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[102662598126] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[102681831945] [[35mTRACE[0m] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[102686685849] [[35mTRACE[0m] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[102689542758] [[35mTRACE[0m] [kernel::sched] [CPU0]   Allocating scheduler...
[102697047750] [[35mTRACE[0m] [kernel::sched] [CPU0]   Leaking scheduler...
[102700603104] [[35mTRACE[0m] [kernel::sched] [CPU0]   Initializing boot task...
[102707373912] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED: allocating CpuScheduler for cpu0 (total=1)
[102710364999] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: 1 per-CPU scheduler(s) allocated
[102711731364] [[32mINFO [0m] [kernel::sched] [CPU0] SCHED: per-CPU preemption initialized (1 independent preemption domains, no global preemption lock)
[102714431523] [[35mTRACE[0m] [kernel::sched] [CPU0]   Creating boot task...
[102740897391] [[35mTRACE[0m] [kernel::sched] [CPU0]   Creating idle tasks...
[102744206070] [[34mDEBUG[0m] [bran::arch::x86_64::task] [CPU0] INIT KERNEL CTX: entry=0xffffffff8008b390 kstack_top=0xffffffffb0020660 arg=0x0
[102748950282] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[102790928031] [[35mTRACE[0m] [kernel::sched] [CPU0]   Boot task initialized
[102798927396] [[34mDEBUG[0m] [kernel::sched::state] [CPU0] SCHED[cpu0]: current=Some(0) idle=Some(1) runnable=0 need_resched=true
[102803790903] [[35mTRACE[0m] [kernel::sched] [CPU0]   Storing scheduler pointer...
[102807256398] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[102809175975] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[102844183035] [[34mDEBUG[0m] [kernel] [CPU0] Initializing VFS...
[102854430162] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='set_cmdline' elapsed_ticks=5879874 elapsed_us=2939 total_ticks=9945309 total_u
```
</details>
