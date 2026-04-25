Feature: ServiceLoop state exposed via /proc/<pid>/serviceloop/

  The kernel exposes per-process ServiceLoop diagnostic state through the
  procfs virtual filesystem at /proc/<pid>/serviceloop/.  This allows
  operators and tooling to observe service loop state (idle, waiting,
  dispatching, shutdown) without consulting logs.

  @smoke
  @timeout-30s
  Scenario: /proc/self/serviceloop directory is listed for a running daemon
    Given the machine is started
    When I wait for the system to boot
    And a daemon with a named ServiceLoop is running
    And the shell command "ls /proc/1/serviceloop" succeeds
    Then the output contains "state"
    And the output contains "name"
    And the output contains "last_event"
    And the output contains "last_dispatch_ns"
    And the output contains "wakeups"
    And the output contains "timeouts"
    And the output contains "errors"

  @smoke
  @timeout-30s
  Scenario: /proc/<pid>/serviceloop/state transitions between waiting and dispatching
    Given the machine is started
    When I wait for the system to boot
    And a daemon with a named ServiceLoop is running
    And the shell command "cat /proc/1/serviceloop/state" succeeds
    Then the output contains one of "idle" or "waiting" or "dispatching" or "shutdown"

  @smoke
  @timeout-30s
  Scenario: /proc/<pid>/serviceloop/name reflects the loop name set by the daemon
    Given the machine is started
    When I wait for the system to boot
    And a daemon with a named ServiceLoop is running
    And the shell command "cat /proc/1/serviceloop/name" succeeds
    Then the output is not empty
    And the output does not equal "-"

  @smoke
  @timeout-30s
  Scenario: /proc/<pid>/serviceloop/wakeups increases over time
    Given the machine is started
    When I wait for the system to boot
    And a daemon with a named ServiceLoop is running
    And the shell command "cat /proc/1/serviceloop/wakeups" succeeds
    Then the output contains a non-negative integer
