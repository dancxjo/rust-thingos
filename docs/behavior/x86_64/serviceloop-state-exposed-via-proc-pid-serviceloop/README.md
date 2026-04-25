# Feature: ServiceLoop state exposed via /proc/<pid>/serviceloop/

> Last run: 2026-04-25 13:10:40

## Scenarios

| Scenario | Steps | Status | Link |
|----------|-------|--------|------|
| /proc/self/serviceloop directory is listed for a running daemon | 2/3 | ✅ | [View Details](proc-self-serviceloop-directory-is-listed-for-a-running-daemon/README.md) |
| /proc/<pid>/serviceloop/state transitions between waiting and dispatching | 2/3 | ✅ | [View Details](proc-pid-serviceloop-state-transitions-between-waiting-and-dispatching/README.md) |
| /proc/<pid>/serviceloop/name reflects the loop name set by the daemon | 2/3 | ✅ | [View Details](proc-pid-serviceloop-name-reflects-the-loop-name-set-by-the-daemon/README.md) |
| /proc/<pid>/serviceloop/wakeups increases over time | 2/3 | ✅ | [View Details](proc-pid-serviceloop-wakeups-increases-over-time/README.md) |
