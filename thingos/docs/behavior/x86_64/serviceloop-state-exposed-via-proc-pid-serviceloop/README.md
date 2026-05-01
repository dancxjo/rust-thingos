# Feature: ServiceLoop state exposed via /proc/<pid>/serviceloop/

> Last run: 2026-05-01 15:28:03

## Scenarios

| Scenario | Steps | Status | Link |
|----------|-------|--------|------|
| /proc/self/serviceloop directory is listed for a running daemon | 0/1 | ❌ | [View Details](proc-self-serviceloop-directory-is-listed-for-a-running-daemon/README.md) |
| /proc/<pid>/serviceloop/state transitions between waiting and dispatching | 0/1 | ❌ | [View Details](proc-pid-serviceloop-state-transitions-between-waiting-and-dispatching/README.md) |
| /proc/<pid>/serviceloop/name reflects the loop name set by the daemon | 0/1 | ❌ | [View Details](proc-pid-serviceloop-name-reflects-the-loop-name-set-by-the-daemon/README.md) |
| /proc/<pid>/serviceloop/wakeups increases over time | 0/1 | ❌ | [View Details](proc-pid-serviceloop-wakeups-increases-over-time/README.md) |
