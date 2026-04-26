# BDD Test Results: x86_64

> Last run: 2026-04-25 20:10:26

## Features

| Feature | Scenarios | Status |
|---------|-----------|--------|
| [Boot progress milestone text reporting](boot-progress-milestone-text-reporting/README.md) | ✅ 4 / ⏭️ 0 / ❌ 0 | ✅ |
| [Scheduler entry latency instrumentation](scheduler-entry-latency-instrumentation/README.md) | ✅ 0 / ⏭️ 0 / ❌ 1 | ❌ |
| [Debug-level log output during boot](debug-level-log-output-during-boot/README.md) | ✅ 2 / ⏭️ 0 / ❌ 0 | ✅ |
| [DRIVER_READY inbox message readiness handshake](driver-ready-inbox-message-readiness-handshake/README.md) | ✅ 0 / ⏭️ 1 / ❌ 3 | ❌ |
| [Per-CPU scheduler allocation and ownership](per-cpu-scheduler-allocation-and-ownership/README.md) | ✅ 0 / ⏭️ 0 / ❌ 3 | ❌ |
| [Process Reaping Smoke Test](process-reaping-smoke-test/README.md) | ✅ 1 / ⏭️ 0 / ❌ 0 | ✅ |
| [Scheduler diagnostics exposed via /proc/sched](scheduler-diagnostics-exposed-via-proc-sched/README.md) | ✅ 0 / ⏭️ 0 / ❌ 4 | ❌ |
| [Serial Shell Boot](serial-shell-boot/README.md) | ✅ 0 / ⏭️ 0 / ❌ 1 | ❌ |
| [ServiceLoop state exposed via /proc/<pid>/serviceloop/](serviceloop-state-exposed-via-proc-pid-serviceloop/README.md) | ✅ 4 / ⏭️ 0 / ❌ 0 | ✅ |
| [vfs_test_provider — synthetic VFS provider for ServiceLoop stress and fault testing](vfs-test-provider-synthetic-vfs-provider-for-serviceloop-stress-and-fault-testing/README.md) | ✅ 0 / ⏭️ 6 / ❌ 0 | ⏭️ |
| [virtio_netd uses ServiceProviderLoop instead of a hand-rolled busy loop](virtio-netd-uses-serviceproviderloop-instead-of-a-hand-rolled-busy-loop/README.md) | ✅ 0 / ⏭️ 4 / ❌ 0 | ⏭️ |
| [Per-CPU wake mailbox for cross-CPU task delivery](per-cpu-wake-mailbox-for-cross-cpu-task-delivery/README.md) | ✅ 2 / ⏭️ 0 / ❌ 0 | ✅ |
