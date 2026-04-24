# ❌ Scenario: grep filters piped input correctly

> Last run: 2026-04-23 20:38:49

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 10038ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 253ms | - [📜](./02/serial.log) - |
| 3 | And I type "echo hello | grep hello | wc -l" on the serial console | ✅ | 10536ms | - [📜](./03/serial.log) - |
| 4 | Then the command output should strictly be "1" | ❌ | 6082ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[31059945741] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[31092293661] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[31135558773] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[31174169499] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[31197183369] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[31200412551] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[31243970967] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[31267166898] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[31322366988] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[31323558156] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[31446897702] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[31539303147] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[31564156437] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[31653158064] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[31777318089] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[31829598042] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[31867366278] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[31995158349] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[32060896725] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[32252831424] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[32579159646] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1229979729 elapsed_us=614989
[32580022200] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[32692289817] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[32778283263] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[32779320453] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[32890575597] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[32895458970] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[32899299279] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[32910114567] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[32915735688] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=42042
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[33202591026] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[33204295575] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[33278261082] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet ��� retrying
echo hello | grep hello | wc -l
[35384682465] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: set_handle_flags fd=6 flags=1
[35386207989] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: set_handle_flags fd=7 flags=1
[35391280254] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: set_handle_flags fd=8 flags=1
[35392653285] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: set_handle_flags fd=9 flags=1
[35396200191] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='echo'
[35402711685] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/echo' with argv=["/bin/echo", "hello"]
[35434376802] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: close_on_exec closing fd 6
[35435724456] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: close_on_exec closing fd 7
[35437063497] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: close_on_exec closing fd 8
[35437822365] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: close_on_exec closing fd 9
[35449858620] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/echo' TID=9 PID=9
[35502462402] [[32mINFO [0m] [kernel::ipc::pipe] [CPU3] PIPE_ENQUEUE: tail=6 len=6 n=6
[35528153166] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='grep'
[35531639814] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/grep' with argv=["/bin/grep", "hello"]
[35567801049] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: close_on_exec closing fd 6
[35568470652] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: close_on_exec closing fd 7
[35569009344] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: close_on_exec closing fd 8
[35569536057] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: close_on_exec closing fd 9
[35575565916] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/grep' TID=10 PID=10
[35613013491] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='wc'
[35616053286] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/wc' with argv=["/bin/wc", "-l"]
[35650544127] [[32mINFO [0m] [grep] [CPU1] grep: main started
[] [[32mINFO [0m] [kernel::vfs::handl3567e_table] [CPU31699833] VFS: close_on_exec closing fd 6
[35675685078] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: close_on_exec closing fd 7
[35676672207] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: close_on_exec closing fd 8
[35677639536] [[32mINFO [
```
</details>
