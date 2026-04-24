# ✅ Scenario: POSIX behavior - head -n and tail -n (expected to fail if not fully POSIX)

> Last run: 2026-04-23 20:38:49

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 9220ms | - - - |
| 2 | When I wait for the shell prompt | ✅ | 254ms | - [📜](./02/serial.log) - |
| 3 | And I type "echo -e '1\n2\n3' | head -n 2 | tail -n 1" on the serial console | ✅ | 837ms | - [📜](./03/serial.log) - |
| 4 | Then the command output should strictly be "2" | ✅ | 0ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[28522364343] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Memory Map OK"
[28552225185] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[28589740575] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Global Allocator"
[28626999522] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Display Registry"
[28650153807] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[28652977716] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[28694727897] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SIMD Ready"
[28718415429] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[28772412042] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[28774514868] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Tasking Initialized"
[28869365250] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="VFS Root Ready"
[28941395736] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="PCI Bus Scanned"
[28964285460] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Legacy Devices"
[29039849190] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="BSP Timer OK"
[29155660644] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="SMP Bring-up"
[29210121105] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Boot Info OK"
[29236209321] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Modules Scanned"
[29344917195] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: hint="Press F12 for a terminal"
[29397751812] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Spawning Sprout"
[29580044934] [[32mINFO [0m] [kernel::boot_progress] [CPU0] boot_progress: milestone="Entering Scheduler"
[29945240391] [[32mINFO [0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=1146389013 elapsed_us=573194
[29946049452] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[30070772589] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[30151317504] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[30152251536] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[30251057595] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=6 PID=6
[30254788080] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=6)
[30256891896] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 50ms so the prompt can take the foreground
[30267749292] [[32mINFO [0m] [sh] [CPU3] SH: starting v0.1.0-debug
[30274889634] [[32mINFO [0m] [kernel] [CPU0] deferred_bootfb_gradient elapsed_ticks=55605
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[30496693293] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Skipping early /hosts mount; mesocarp is disabled in init
[30499537200] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Waiting for /dev/net/virtio0/rx before spawning netd...
[30555746001] [[32mINFO [0m] [iso9660d] [CPU2] iso9660d: no ISO9660 filesystem found yet — retrying
echo -e '1\n2\n3' | head -n 2 | tail -n 1
[33258251730] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: set_handle_flags fd=6 flags=1
[33259893810] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: set_handle_flags fd=7 flags=1
[33265305315] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: set_handle_flags fd=8 flags=1
[33267190803] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: set_handle_flags fd=9 flags=1
[33271939536] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='echo'
[33278710476] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/echo' with argv=["/bin/echo", "-e", "1\\n2\\n3"]
[33308101002] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: close_on_exec closing fd 6
[33309366024] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: close_on_exec closing fd 7
[33310415325] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: close_on_exec closing fd 8
[33311058759] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: close_on_exec closing fd 9
[33325505367] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/echo' TID=9 PID=9
[33371831691] [[32mINFO [0m] [kernel::ipc::pipe] [CPU3] PIPE_ENQUEUE: tail=6 len=6 n=6
[33392704389] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='head'
[33397172028] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/head' with argv=["/bin/head", "-n", "2"]
[33423117585] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: close_on_exec closing fd 6
[33423803094] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: close_on_exec closing fd 7
[33424341027] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: close_on_exec closing fd 8
[33424859952] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: close_on_exec closing fd 9
[33430486287] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_EX: name='/bin/head' TID=10 PID=10
[33468114834] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='tail'
[33471225018] [[32mINFO [0m] [sh] [CPU3] sh: spawning '/bin/tail' with argv=["/bin/tail", "-n", "1"]
[33499636896] [[32mINFO [0m] [kernel::ipc::pipe] [CPU1] PIPE_ENQUEUE: tail=2 len=2 n=2
[33502174596] [[32mINFO [0m] [kernel::ipc::pipe] [CPU1] PIPE_ENQUEUE: tail=4 len=4 n=2
[33512501022] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: close_on_exec closing fd 6
[33513474159] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: close_on_exec closing fd 7
[33514345854] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: close_on_exec closing fd 8
[33515232300] [[32mINFO [0m] [kernel::vfs::handle_table] [CPU3] VFS: close_on_exec closing fd 9
[33523707558] [[32mINFO [0m] [kernel::syscall::handlers::process] [CPU3] SYSCALL SPAWN_PROCESS_E
```
</details>
