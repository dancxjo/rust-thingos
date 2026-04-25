Feature: Scheduler diagnostics exposed via /proc/sched

  The kernel exposes per-CPU scheduler counters through the procfs
  virtual filesystem at /proc/sched.  This allows user-space tools and
  operators to observe load distribution, context-switch rates, and idle
  time without attaching a debugger.

  @smoke
  @timeout-30s
  Scenario: /proc/sched directory is visible in the root procfs listing
    Given the machine is started
    When I wait for the system to boot
    And the shell command "ls /proc" succeeds
    Then the output contains "sched"

  @smoke
  @timeout-30s
  Scenario: /proc/sched/stat is readable and contains online_cpus
    Given the machine is started
    When I wait for the system to boot
    And the shell command "cat /proc/sched/stat" succeeds
    Then the output contains "online_cpus:"

  @smoke
  @timeout-30s
  Scenario: /proc/sched/stat contains per-CPU counters
    Given the machine is started
    When I wait for the system to boot
    And the shell command "cat /proc/sched/stat" succeeds
    Then the output contains "context_switches:"
    And the output contains "wakeups:"
    And the output contains "runnable_count:"

  @smoke
  @timeout-30s
  Scenario: /proc/sched/cpu0 is readable and contains cpu-0 counters
    Given the machine is started
    When I wait for the system to boot
    And the shell command "cat /proc/sched/cpu0" succeeds
    Then the output contains "cpu: 0"
    And the output contains "context_switches:"
