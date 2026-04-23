# ❌ Scenario: Scheduler loop entry is reached before deferred boot framebuffer paint

> Last run: 2026-04-22 08:38:40

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 0ms | - - - |
| 2 | When I wait for the system to boot | ✅ | 11567ms | - [📜](./02/serial.log) - |
| 3 | Then the serial output should contain "scheduler-entry total elapsed_ticks=" | ✅ | 0ms | - - - |
| 4 | And the serial log shows "Entering scheduler loop." after "Scheduler initialized" | ✅ | 503ms | - [📜](./04/serial.log) - |
| 5 | And the serial log shows "deferred_bootfb_gradient elapsed_ticks=" after "Entering scheduler loop." | ❌ | 1510ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
UEFI firmware (version  built at 23:58:55 on Oct  8 2025)
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;031;100t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI Misc Device" from PciRoot(0x0)/Pci(0x3,0x0)
BdsDxe: starting Boot0002 "UEFI Misc Device" from PciRoot(0x0)/Pci(0x3,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01HKMAIN
[633915975] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] enter
[634142631] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] phys_memory_map ok
[634400482] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] modules ok
[634463454] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] phys_to_virt_offset ok
[634525990] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] memory map logging done
[644622832] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] boot_frame_alloc init ok
[674538103] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] frame allocator build ok
[674600830] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] frame allocator log ok
[674665634] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] FRAME_ALLOCATOR init ok
[674726248] [[34mDEBUG[0m] [kernel::memory] [CPU0] [kernel:mem:init] tasking init ok
[675134914] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[675196188] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] enter
[675256265] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] set expand hook
[675318737] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] expand hook ok
[675380020] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock begin
[675448728] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] kernel_heap lock ok
[675514780] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] reserve_region begin
[679785855] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] reserve_region ok
[679864739] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] inner allocator init begin
[679968667] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] inner allocator init ok
[680037378] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] heap top store ok
[680101530] [[34mDEBUG[0m] [kernel::memory::global_alloc] [CPU0] [kernel:global_alloc] init done
[680587280] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] set_boot_fb begin
[680650698] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] devfs: set_boot_fb width=800 height=600 pitch=3200 resource_id=0xfb000000
[680756430] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] set_boot_fb ok
[680826487] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs set_boot_fb elapsed_ticks=230329 elapsed_us=3685
[681003646] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs FbNode::new elapsed_ticks=95255 elapsed_us=1524
[681083691] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] register begin
[681218923] [[34mDEBUG[0m] [kernel::vfs::devfs] [CPU0] [kernel:devfs] register ok
[681279876] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] framebuffer/devfs register fb0 elapsed_ticks=196273 elapsed_us=3140
[681678364] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[682054645] [[34mDEBUG[0m] [kernel] [CPU0] Seeding entropy pool...
[682111052] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed begin
[682170223] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] fill_entropy done
[682266696] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] add_sample(timer) ok
[682327279] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] mark_seeded(timer) ok
[682386619] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[682452731] [[34mDEBUG[0m] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[682517603] [[34mDEBUG[0m] [kernel::entropy] [CPU0] [kernel:entropy] seed done
[682577379] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] entropy::seed_from_hardware elapsed_ticks=465086 elapsed_us=7441
[683006915] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[683105745] [[35mTRACE[0m] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[683177984] [[35mTRACE[0m] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[683241913] [[35mTRACE[0m] [kernel::sched] [CPU0]   Allocating scheduler...
[683390448] [[35mTRACE[0m] [kernel::sched] [CPU0]   Leaking scheduler...
[683448660] [[35mTRACE[0m] [kernel::sched] [CPU0]   Initializing boot task...
[683557091] [[35mTRACE[0m] [kernel::sched] [CPU0]   Creating boot task...
[683996734] [[35mTRACE[0m] [kernel::sched] [CPU0]   Creating idle tasks...
[684099897] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[684667471] [[35mTRACE[0m] [kernel::sched] [CPU0]   Boot task initialized
[684725397] [[35mTRACE[0m] [kernel::sched] [CPU0]   Storing scheduler pointer...
[684797706] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[685225136] [[34mDEBUG[0m] [kernel] [CPU0] Initializing VFS...
[685317139] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='set_cmdline' elapsed_ticks=14463 elapsed_us=231 total_ticks=89536 total_us=1432
[685963790] [[34mDEBUG[0m] [kernel::vfs] [CPU0] VFS: Created /dev/display directory
[686526954] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted union filesystem at / (root)
[686636633] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted devfs at /dev
[686719470] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted procfs at /proc
[686798885] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted sysfs at /sys
[686957209] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /tmp
[687046088] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /run
[687138217] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /services
[687232512] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /session
[687340021] [[34mDEBUG[0m] [kernel::vfs] [CPU0] vfs: mounted tmpfs at /data
[687398489] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='vfs_init' elapsed_ticks=1944783 elapsed_us=31116 total_ticks=2175776 total_us=34812
[687833819] [[34mDEBUG[0m] [kernel] [CPU0] Scanning PCI bus...
[687952739] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='scan_pci' elapsed_ticks=64114 elapsed_us=1025 total_ticks=2729918 total_us=43678
[688414478] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='register_legacy_devices' elapsed_ticks=29465 elapsed_us=471 total_ticks=3191655 total_us=51066
[688854756] [[34mDEBUG[0m] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[688921093] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='setup_preemption_timer' elapsed_ticks=1913 elapsed_us=30 total_ticks=3698415 total_us=59174
[689382422] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Detected 1 CPU.
[689445176] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='start_secondary_cpus' elapsed_ticks=65911 elapsed_us=1054 total_ticks=4222459 total_us=67559
[690198999] [[34mDEBUG[0m] [kernel] [CPU0] Kernel: Enumerating 108 boot modules...
[690278644] [[35mTRACE[0m] [kernel] [CPU0]   Module[0]: name='/bin/sprout' cmdline='init' size=155320 bytes
[690367560] [[35mTRACE[0m] [kernel] [CPU0]   Module[1]: name='/bin/bristle' cmdline='' size=107176 bytes
[690459300] [[35mTRACE[0m] [kernel] [CPU0]   Module[2]: name='/drivers/rtc_cmos' cmdline='init' size=98560 bytes
[690547287] [[35mTRACE[0m] [kernel] [CPU0]   Module[3]: name='/drivers/ps2_kbd' cmdline='' size=98568 bytes
[690648822] [[35mTRACE[0m] [kernel] [CPU0]   Module[4]: name='/bin/sh' cmdline='init' size=180592 bytes
[690735311] [[35mTRACE[0m] [kernel] [CPU0]   Module[5]: name='/bin/ls' cmdline='init' size=120504 bytes
[690821190] [[35mTRACE[0m] [kernel] [CPU0]   Module[6]: name='/bin/ln' cmdline='init' size=109264 bytes
[690907267] [[35mTRACE[0m] [kernel] [CPU0]   Module[7]: name='/bin/ps' cmdline='init' size=111096 bytes
[690994482] [[35mTRACE[0m] [kernel] [CPU0]   Module[8]: name='/bin/kill' cmdline='init' size=114112 bytes
[691083210] [[35mTRACE[0m] [kernel] [CPU0]   Module[9]: name='/bin/killall' cmdline='init' size=130992 bytes
[691180543] [[35mTRACE[0m] [kernel] [CPU0]   Module[10]: name='/bin/cat' cmdline='init' size=106232 bytes
[691266632] [[35mTRACE[0m] [kernel] [CPU0]   Module[11]: name='/bin/head' cmdline='init' size=117016 bytes
[691352741] [[35mTRACE[0m] [kernel] [CPU0]   Module[12]: name='/bin/tail' cmdline='init' size=117072 bytes
[691438920] [[35mTRACE[0m] [kernel] [CPU0]   Module[13]: name='/bin/top' cmdline='init' size=122808 bytes
[691524464] [[35mTRACE[0m] [kernel] [CPU0]   Module[14]: name='/bin/wc' cmdline='init' size=117224 bytes
[691609841] [[35mTRACE[0m] [kernel] [CPU0]   Module[15]: name='/bin/yes' cmdline='init' size=100040 bytes
[691699284] [[35mTRACE[0m] [kernel] [CPU0]   Module[16]: name='/bin/which' cmdline='init' size=106392 bytes
[691785112] [[35mTRACE[0m] [kernel] [CPU0]   Module[17]: name='/bin/cp' cmdline='init' size=106496 bytes
[691870688] [[35mTRACE[0m] [kernel] [CPU0]   Module[18]: name='/bin/mv' cmdline='init' size=108496 bytes
[691955967] [[35mTRACE[0m] [kernel] [CPU0]   Module[19]: name='/bin/rm' cmdline='init' size=108576 bytes
[692041629] [[35mTRACE[0m] [kernel] [CPU0]   Module[20]: name='/bin/rmdir' cmdline='init' size=108592 bytes
[692127605] [[35mTRACE[0m] [kernel] [CPU0]   Module[21]: name='/bin/mkdir' cmdline='init' size=108592 bytes
[692216461] [[35mTRACE[0m] [kernel] [CPU0]   Module[22]: name='/bin/mount' cmdline='init' size=115792 bytes
[692302991] [[35mTRACE[0m] [kernel] [CPU0]   Module[23]: name='/bin/echo' cmdline='init' size=104288 bytes
[692388817] [[35mTRACE[0m] [kernel] [CPU0]   Module[24]: name='/bin/grep' cmdline='init' size=139728 bytes
[692474471] [[35mTRACE[0m] [kernel] [CPU0]   Module[25]: name='/bin/pwd' cmdline='init' size=98448 bytes
[692560100] [[35mTRACE[0m] [kernel] [CPU0]   Module[26]: name='/bin/touch' cmdline='init' size=106288 bytes
[692646262] [[35mTRACE[0m] [kernel] [CPU0]   Module[27]: name='/bin/setshell' cmdline='init' size=107232 bytes
[692736758] [[35mTRACE[0m] [kernel] [CPU0]   Module[28]: name='/bin/dmesg' cmdline='init' size=98648 bytes
[692822595] [[35mTRACE[0m] [kernel] [CPU0]   Module[29]: name='/bin/stat' cmdline='init' size=107016 bytes
[692908268] [[35mTRACE[0m] [kernel] [CPU0]   Module[30]: name='/bin/file' cmdline='init' size=122968 bytes
[692997675] [[35mTRACE[0m] [kernel] [CPU0]   Module[31]: name='/bin/dirname' cmdline='init' size=99928 bytes
[693084075] [[35mTRACE[0m] [kernel] [CPU0]   Module[32]: name='/bin/basename' cmdline='init' size=110464 bytes
[693173713] [[35mTRACE[0m] [kernel] [CPU0]   Module[33]: name='/bin/sleep' cmdline='init' size=117824 bytes
[693275225] [[35mTRACE[0m] [kernel] [CPU0]   Module[34]: name='/bin/sort' cmdline='init' size=127952 bytes
[693361406] [[35mTRACE[0m] [kernel] [CPU0]   Module[35]: name='/bin/env' cmdline='init' size=105824 bytes
[693447237] [[35mTRACE[0m] [kernel] [CPU0]   Module[36]: name='/bin/uname' cmdline='' size=100288 bytes
[693532526] [[35mTRACE[0m] [kernel] [CPU0]   Module[37]: name='/bin/true' cmdline='' size=78056 bytes
[693617599] [[35mTRACE[0m] [kernel] [CPU0]   Module[38]: name='/bin/false' cmdline='' size=78056 bytes
[693702502] [[35mTRACE[0m] [kernel] [CPU0]   Module[39]: name='/bin/input_echo' cmdline='' size=101464 bytes
[693795427] [[35mTRACE[0m] [kernel] [CPU0]   Module[40]: name='/drivers/ps2_mouse' cmdline='' size=99864 bytes
[693882728] [[35mTRACE[0m] [kernel] [CPU0]   Module[41]: name='/drivers/display_bootfb' cmdline='' size=148088 bytes
[693971027] [[35mTRACE[0m] [kernel] [CPU0]   Module[42]: name='/drivers/display_virtio_gpu' cmdline='' size=194672 bytes
[694060508] [[35mTRACE[0m] [kernel] [CPU0]   Module[43]: name='/bin/cambium' cmdline='' size=184320 bytes
[694146123] [[35mTRACE[0m] [kernel] [CPU0]   Module[44]: name='/drivers/virtio_netd' cmdline='' size=177184 bytes
[694236378] [[35mTRACE[0m] [kernel] [CPU0]   Module[45]: name='/drivers/rtl8168d' cmdline='' size=119776 bytes
[694325589] [[35mTRACE[0m] [kernel] [CPU0]   Module[46]: name='/bin/netd' cmdline='' size=398520 bytes
[694410614] [[35mTRACE[0m] [kernel] [CPU0]   Module[47]: name='/bin/mesocarp' cmdline='' size=180992 bytes
[694496954] [[35mTRACE[0m] [kernel] [CPU0]   Module[48]: name='/bin/mdns' cmdline='' size=180992 bytes
[694585155] [[35mTRACE[0m] [kernel] [CPU0]   Module[49]: name='/bin/mdnsd' cmdline='' size=180992 bytes
[694673458] [[35mTRACE[0m] [kernel] [CPU0]   Module[50]: name='/bin/fetchd' cmdline='' size=241424 bytes
[694761357] [[35mTRACE[0m] [kernel] [CPU0]   Module[51]: name='/bin/httpsd' cmdline='' size=567848 bytes
[694847720] [[35mTRACE[0m] [kernel] [CPU0]   Module[52]: name='/bin/find' cmdline='' size=120264 bytes
[694933087] [[35mTRACE[0m] [kernel] [CPU0]   Module[53]: name='/bin/ip' cmdline='' size=112416 bytes
[695017596] [[35mTRACE[0m] [kernel] [CPU0]   Module[54]: name='/bin/iso_reader' cmdline='' size=96080 bytes
[695104094] [[35mTRACE[0m] [kernel] [CPU0]   Module[55]: name='/bin/ping' cmdline='' size=127296 bytes
[695189152] [[35mTRACE[0m] [kernel] [CPU0]   Module[56]: name='/bin/nslookup' cmdline='' size=117696 bytes
[695279092] [[35mTRACE[0m] [kernel] [CPU0]   Module[57]: name='/drivers/ahci_disk' cmdline='' size=130176 bytes
[695367071] [[35mTRACE[0m] [kernel] [CPU0]   Module[58]: name='/drivers/ata_disk' cmdline='' size=128496 bytes
[695454060] [[35mTRACE[0m] [kernel] [CPU0]   Module[59]: name='/bin/iso9660d' cmdline='' size=149864 bytes
[695540082] [[35mTRACE[0m] [kernel] [CPU0]   Module[60]: name='/drivers/virtio_sound' cmdline='' size=166696 bytes
[695628460] [[35mTRACE[0m] [kernel] [CPU0]   Module[61]: name='/drivers/hdaudio' cmdline='' size=131240 bytes
[695715315] [[35mTRACE[0m] [kernel] [CPU0]   Module[62]: name='/drivers/pci_stubd' cmdline='' size=121120 bytes
[695805471] [[35mTRACE[0m] [kernel] [CPU0]   Module[63]: name='/drivers/chime' cmdline='' size=117152 bytes
[695922722] [[35mTRACE[0m] [kernel] [CPU0]   Module[64]: name='/bin/vfs_hello' cmdline='' size=99776 bytes
[696010853] [[35mTRACE[0m] [kernel] [CPU0]   Module[65]: name='/bin/show_args' cmdline='' size=101552 bytes
[696100635] [[35mTRACE[0m] [kernel] [CPU0]   Module[66]: name='/bin/env_roundtrip' cmdline='' size=109000 bytes
[696188754] [[35mTRACE[0m] [kernel] [CPU0]   Module[67]: name='/bin/cwd_test' cmdline='' size=108232 bytes
[696276970] [[35mTRACE[0m] [kernel] [CPU0]   Module[68]: name='/bin/date' cmdline='' size=129704 bytes
[696368983] [[35mTRACE[0m] [kernel] [CPU0]   Module[69]: name='/bin/wayland_hello' cmdline='' size=133128 bytes
[696456661] [[35mTRACE[0m] [kernel] [CPU0]   Module[70]: name='/bin/terminal' cmdline='' size=145112 bytes
[696553563] [[35mTRACE[0m] [kernel] [CPU0]   Module[71]: name='/bin/tee' cmdline='' size=104968 bytes
[696639126] [[35mTRACE[0m] [kernel] [CPU0]   Module[72]: name='/bin/xargs' cmdline='' size=115392 bytes
[696724327] [[35mTRACE[0m] [kernel] [CPU0]   Module[73]: name='/bin/placed' cmdline='' size=98256 bytes
[696810039] [[35mTRACE[0m] [kernel] [CPU0]   Module[74]: name='/bin/bloom' cmdline='' size=174912 bytes
[696901444] [[35mTRACE[0m] [kernel] [CPU0]   Module[75]: name='/bin/clear' cmdline='' size=78264 bytes
[696986965] [[35mTRACE[0m] [kernel] [CPU0]   Module[76]: name='/bin/loglevel' cmdline='' size=102656 bytes
[697076090] [[35mTRACE[0m] [kernel] [CPU0]   Module[77]: name='/bin/poll_mux' cmdline='' size=100168 bytes
[697162153] [[35mTRACE[0m] [kernel] [CPU0]   Module[78]: name='/bin/ipc_service_demo' cmdline='' size=109280 bytes
[697250846] [[35mTRACE[0m] [kernel] [CPU0]   Module[79]: name='/bin/ipc_pipe_demo' cmdline='' size=108688 bytes
[697338192] [[35mTRACE[0m] [kernel] [CPU0]   Module[80]: name='/bin/ipc_provider_demo' cmdline='' size=130480 bytes
[697429601] [[35mTRACE[0m] [kernel] [CPU0]   Module[81]: name='/bin/ipc_memfd_demo' cmdline='' size=94000 bytes
[697516943] [[35mTRACE[0m] [kernel] [CPU0]   Module[82]: name='/bin/test_exec' cmdline='' size=125584 bytes
[697603199] [[35mTRACE[0m] [kernel] [CPU0]   Module[83]: name='/bin/test_vm_protect' cmdline='' size=94136 bytes
[697693688] [[35mTRACE[0m] [kernel] [CPU0]   Module[84]: name='/bin/test_exec_env' cmdline='' size=123064 bytes
[697784450] [[35mTRACE[0m] [kernel] [CPU0]   Module[85]: name='/bin/test_threads' cmdline='' size=143408 bytes
[697872324] [[35mTRACE[0m] [kernel] [CPU0]   Module[86]: name='/bin/test_futex' cmdline='' size=111792 bytes
[697961422] [[35mTRACE[0m] [kernel] [CPU0]   Module[87]: name='/bin/ld_so' cmdline='' size=112624 bytes
[698047276] [[35mTRACE[0m] [kernel] [CPU0]   Module[88]: name='/bin/test_dyn_loader' cmdline='' size=121080 bytes
[698135207] [[35mTRACE[0m] [kernel] [CPU0]   Module[89]: name='/bin/test_dlopen' cmdline='' size=127488 bytes
[698222497] [[35mTRACE[0m] [kernel] [CPU0]   Module[90]: name='/bin/reboot' cmdline='' size=90016 bytes
[698308667] [[35mTRACE[0m] [kernel] [CPU0]   Module[91]: name='/bin/shutdown' cmdline='' size=90024 bytes
[698394646] [[35mTRACE[0m] [kernel] [CPU0]   Module[92]: name='/bin/attr_list' cmdline='' size=109168 bytes
[698548923] [[35mTRACE[0m] [kernel] [CPU0]   Module[93]: name='/bin/attr_get' cmdline='' size=109280 bytes
[698650132] [[35mTRACE[0m] [kernel] [CPU0]   Module[94]: name='/bin/attr_set' cmdline='' size=131728 bytes
[698738750] [[35mTRACE[0m] [kernel] [CPU0]   Module[95]: name='/bin/attr_rm' cmdline='' size=108464 bytes
[698827874] [[35mTRACE[0m] [kernel] [CPU0]   Module[96]: name='/lib/libpistil.so' cmdline='' size=124544 bytes
[698915253] [[35mTRACE[0m] [kernel] [CPU0]   Module[97]: name='/share/wallpapers/clouds.bmp' cmdline='' size=786486 bytes
[699005505] [[35mTRACE[0m] [kernel] [CPU0]   Module[98]: name='/share/wallpapers/flower.bmp' cmdline='' size=8386614 bytes
[699095287] [[35mTRACE[0m] [kernel] [CPU0]   Module[99]: name='/share/wallpapers/leather.bmp' cmdline='' size=1179702 bytes
[699185485] [[35mTRACE[0m] [kernel] [CPU0]   Module[100]: name='/share/wallpapers/linen.bmp' cmdline='' size=4718646 bytes
[699275542] [[35mTRACE[0m] [kernel] [CPU0]   Module[101]: name='/share/fonts/NotoSans-Regular.ttf' cmdline='' size=569208 bytes
[699367017] [[35mTRACE[0m] [kernel] [CPU0]   Module[102]: name='/share/fonts/unifont.hex' cmdline='' size=8355852 bytes
[699456577] [[35mTRACE[0m] [kernel] [CPU0]   Module[103]: name='/etc/locale.conf' cmdline='' size=85 bytes
[699542995] [[35mTRACE[0m] [kernel] [CPU0]   Module[104]: name='/etc/profile' cmdline='' size=68 bytes
[699628534] [[35mTRACE[0m] [kernel] [CPU0]   Module[105]: name='/etc/motd' cmdline='' size=610 bytes
[699713079] [[35mTRACE[0m] [kernel] [CPU0]   Module[106]: name='/etc/fstab' cmdline='' size=123 bytes
[699798707] [[35mTRACE[0m] [kernel] [CPU0]   Module[107]: name='/etc/hostname' cmdline='' size=8 bytes
[700237038] [[34mDEBUG[0m] [kernel] [CPU0] Found init module: /bin/sprout (cmdline: 'init'), loading...
[700368152] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] make_user_address_space: created aspace phys=0x41008000
[700450729] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module '/bin/sprout' (len=155320, base=0x200000)
[700540303] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[700892718] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=2064b4, min_vaddr=200000, bias=0, entry_pc=2064b4
[700999072] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[701441633] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [f7, 33, 40, f9, e8, 03, 0a, aa]
[702459430] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20b000 exec=false
[702782501] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20d008 exec=false
[702864701] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Overlap at 20d000: merging perms to r=true w=true x=false
[702963342] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=20e000 exec=false
[703898153] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='load_init_module' elapsed_ticks=3451125 elapsed_us=55218 total_ticks=18674643 total_us=298794
[704069184] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='prepare_boot_registry' elapsed_ticks=51132 elapsed_us=818 total_ticks=18846349 total_us=301541
[704183453] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='map_boot_registry' elapsed_ticks=6740 elapsed_us=107 total_ticks=18960647 total_us=303370
[704297067] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] activate_address_space: setting TTBR0 to 0x41008000
[704373866] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='activate_init_aspace' elapsed_ticks=78422 elapsed_us=1254 total_ticks=19149943 total_us=306399
[704491628] [[34mDEBUG[0m] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[704553306] [[34mDEBUG[0m] [kernel] [CPU0] Spawning init process...
[705639653] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 2 deferred inserts
[705716039] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=1
[705785050] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=2
[705864943] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[705926060] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='spawn_init_task' elapsed_ticks=1373300 elapsed_us=21972 total_ticks=20702815 total_us=331245
[709427240] [[34mDEBUG[0m] [kernel::sched] [CPU0] SCHED: early-boot bringup complete; resuming steady-state SMP scheduling
[709513338] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry step='end_bringup' elapsed_ticks=95423 elapsed_us=1526 total_ticks=24288873 total_us=388621
[713445296] [[34mDEBUG[0m] [kernel] [CPU0] [kernel:start] scheduler-entry total elapsed_ticks=28217290 elapsed_us=451476
[713524460] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[713994676] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] activate_address_space: setting TTBR0 to 0x41008000
[714126385] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] USER_TRAMPOLINE: PC=0x2064b4 SP=0x800000 ARG0=0x600000
[714227741] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] enter_user: TTBR0=0x41008000 entry_pc=0x2064b4 user_sp=0x800000 tls_base=0x0
[714371297] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=2 buf_len=0
[714499242] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=2 buf_len=20
[714775192] [[34mDEBUG[0m] [sprout] [CPU0] [sprout] whoami: cs=0x0 ss=0x0 cpl=0 rsp=0x0 rip=0x0 rflags=0x0
[714874739] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[714951790] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: About to create Supervisor...
[715026123] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Listing /bin directory...
[715122453] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/bin' tid=2
[719398761] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Parsed command line: '    '
[719490105] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: FORCING display=bootfb for diagnostic test!
[719572470] [[34mDEBUG[0m] [sprout] [CPU0] SPROUT: Supervisor created, calling run_forever...
[719653252] [[34mDEBUG[0m] [sprout::supervisor] [CPU0] SPROUT: Supervisor session started (MINIMAL MODE)
[719733181] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Launching serial shell...
[719854983] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Setting up serial shell on /dev/console...
[719930984] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/run/sprout/shell' tid=2
[720535278] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/etc/default/shell' tid=2
[721078739] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/bin/sh' tid=2
[721304100] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU0] BootFs: EXACT match for 'bin/sh' at index 4
[721638406] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=2
[722030710] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=2
[722373291] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/dev/console' tid=2
[723034428] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU0] BootFs: EXACT match for 'bin/sh' at index 4
[728677856] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] make_user_address_space: created aspace phys=0x4106e000
[728791139] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: Loading module 'sh' (len=180592, base=0x200000)
[728875541] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[729108237] [[34mDEBUG[0m] [kernel::task::loader] [CPU0] LOADER: ELF info: entry=20b03c, min_vaddr=200000, bias=0, entry_pc=20b03c
[729199309] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[729599656] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   COPIED at 0x201420: [3f, 29, 00, 71, c3, 05, 00, 54]
[731172028] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=210000 exec=false
[731528821] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=212708 exec=false
[731607595] [[35mTRACE[0m] [kernel::task::loader] [CPU0]   Overlap at 212000: merging perms to r=true w=true x=false
[731699280] [[35mTRACE[0m] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[732611637] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Starting Phase 1 for /bin/sh
[732852752] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 1 complete, ID=3, applying inserts
[732927484] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Applying 1 deferred inserts
[732995463] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserting TID=3
[733063712] [[34mDEBUG[0m] [kernel::sched] [CPU0] REGISTRY: Inserts applied
[733122482] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Inserts applied, entering Phase 2
[734134735] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SETUP_STDIO: stdin=Fd(3) stdout=Fd(4) stderr=Fd(5)
[734944569] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Phase 2 complete, waking task 3
[735036007] [[35mTRACE[0m] [kernel::sched::blocking] [CPU0] WAKE_TASK: ID=3 taking SCHEDULER lock
[735242160] [[35mTRACE[0m] [kernel::sched::blocking] [CPU0] WAKE_TASK: ID=3 wake_task_locked returned IPI_CPU=None
[735334795] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Task 3 woken, restoring IRQs
[735430888] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] SPAWN_FROM_PATH: Done for /bin/sh
[736059721] [[35mTRACE[0m] [kernel::syscall::handlers::process] [CPU0] SYSCALL SPAWN_PROCESS_EX: name='/bin/sh' TID=3 PID=3
[736184118] [[32mINFO [0m] [sprout::pipelines] [CPU0] SPROUT: Spawned serial shell '/bin/sh' (PID=3)
[736314543] [[32mINFO [0m] [sprout::supervisor] [CPU0] SPROUT: Serial shell launched; yielding 250ms so the prompt can take the foreground
[736591290] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] activate_address_space: setting TTBR0 to 0x4106e000
[736677642] [[34mDEBUG[0m] [kernel::sched::spawn] [CPU0] USER_TRAMPOLINE: PC=0x20b03c SP=0x800000 ARG0=0x0
[736768800] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] enter_user: TTBR0=0x4106e000 entry_pc=0x20b03c user_sp=0x800000 tls_base=0x0
[736880333] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=3 buf_len=0
[736974370] [[34mDEBUG[0m] [kernel::syscall::handlers::process] [CPU0] sys_auxv_get: tid=3 buf_len=100
[737094606] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_write: tid=3 fd=1 len=19
[737180472] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_write: tid=3 fd=1 starting copyin
[737262043] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_write: tid=3 fd=1 copyin ok
[737387576] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] vfs: /dev/console write by PID=3 len=19
DEBUG: sh starting
[737686897] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_write: tid=3 fd=1 len=33
[737768010] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_write: tid=3 fd=1 starting copyin
[737845154] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_write: tid=3 fd=1 copyin ok
[737932128] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] vfs: /dev/console write by PID=3 len=33
DEBUG: sh sig handlers installed
[738072685] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/run/motd' tid=3
[738657540] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/etc/motd' tid=3
[738973546] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU0] BootFs: EXACT match for 'etc/motd' at index 105
[739213397] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_write: tid=3 fd=1 len=610
[739302298] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_write: tid=3 fd=1 starting copyin
[739388279] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_write: tid=3 fd=1 copyin ok
[739475438] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] vfs: /dev/console write by PID=3 len=610
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
[739673477] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_write: tid=3 fd=1 len=23
[739754904] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_write: tid=3 fd=1 starting copyin
[739831639] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_write: tid=3 fd=1 copyin ok
[739920186] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] vfs: /dev/console write by PID=3 len=23
DEBUG: sh motd printed
[740038554] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_write: tid=3 fd=1 len=31
[740119934] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_write: tid=3 fd=1 starting copyin
[740197169] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_write: tid=3 fd=1 copyin ok
[740283884] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] vfs: /dev/console write by PID=3 len=31
DEBUG: sh shell object created
[740382821] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] VFS: sys_fs_open path='/etc/profile' tid=3
[740772374] [[34mDEBUG[0m] [kernel::vfs::bootfs] [CPU0] BootFs: EXACT match for 'etc/profile' at index 104
[744041459] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_write: tid=3 fd=1 len=25
[744125289] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_write: tid=3 fd=1 starting copyin
[744204955] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_write: tid=3 fd=1 copyin ok
[744300404] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] vfs: /dev/console write by PID=3 len=25
DEBUG: sh profile loaded
[744535002] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_write: tid=3 fd=1 len=97
[744616857] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_write: tid=3 fd=1 starting copyin
[744701775] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_write: tid=3 fd=1 copyin ok
[744791146] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] vfs: /dev/console write by PID=3 len=97
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [744983560] [[35mTRACE[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_write: tid=3 fd=1 len=6
[745064650] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_write: tid=3 fd=1 starting copyin
[745143031] [[34mDEBUG[0m] [kernel::syscall::handlers::vfs] [CPU0] sys_fs_write: tid=3 fd=1 copyin ok
[745234701] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] vfs: /dev/console write by PID=3 len=6
[?25h[745343054] [[35mTRACE[0m] [kernel::vfs::devfs] [CPU0] vfs: /dev/console read by PID=3 len=1
[745548296] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] activate_address_space: setting TTBR0 to 0xb6d6a000
[745916225] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] activate_address_space: setting TTBR0 to 0x4106e000
[746312645] [[32mINFO [0m] [bran::arch::aarch64] [CPU0] activate_address_space: setting TTBR0 to 0xb6d6a000

```
</details>
