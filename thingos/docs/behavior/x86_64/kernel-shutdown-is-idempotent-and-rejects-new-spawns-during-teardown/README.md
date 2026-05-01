# Feature: Kernel shutdown is idempotent and rejects new spawns during teardown

> Last run: 2026-05-01 15:28:03

## Scenarios

| Scenario | Steps | Status | Link |
|----------|-------|--------|------|
| Duplicate shutdown call exits cleanly without 17-second delay | 0/0 | ⏭️ | [View Details](duplicate-shutdown-call-exits-cleanly-without-17-second-delay/README.md) |
| Second shutdown syscall during teardown logs a duplicate warning | 0/0 | ⏭️ | [View Details](second-shutdown-syscall-during-teardown-logs-a-duplicate-warning/README.md) |
| Spawning a new process during shutdown is rejected | 0/0 | ⏭️ | [View Details](spawning-a-new-process-during-shutdown-is-rejected/README.md) |
