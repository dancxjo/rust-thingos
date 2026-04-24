Feature: Per-CPU scheduler allocation and ownership

  Each logical CPU has an exclusive CpuScheduler that owns the local run queue,
  the currently running task, the idle task, and scheduling counters.  No other
  CPU may directly mutate a CPU's local scheduler state; cross-CPU effects go
  through explicit delivery mechanisms (mailboxes, IPIs).

  @smoke
  @timeout-30s
  Scenario: Per-CPU scheduler is allocated for every CPU at boot
    Given the machine is started
    When I wait for the system to boot
    Then the serial output should contain "per-CPU scheduler(s) allocated"

  @smoke
  @timeout-30s
  Scenario: System boots and schedules correctly with per-CPU schedulers
    Given the machine is started
    When I wait for the system to boot
    Then the serial output should contain "Scheduler initialized"
    And the serial log shows "per-CPU scheduler(s) allocated" after "Allocating"
    And the serial log shows "Scheduler initialized" after "per-CPU scheduler(s) allocated"

  @smoke
  @timeout-30s
  Scenario: Per-CPU preemption is initialized without a global preemption lock
    Given the machine is started
    When I wait for the system to boot
    Then the serial output should contain "per-CPU preemption initialized"
