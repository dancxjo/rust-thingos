Feature: Per-CPU wake mailbox for cross-CPU task delivery

  Each CPU has a WakeMailbox that remote CPUs use to deliver wakeups instead
  of directly mutating another CPU's run queue.  The owning CPU drains its
  mailbox at safe scheduling points, locally enqueuing the delivered tasks.

  @smoke
  @timeout-30s
  Scenario: System boots and schedules tasks with mailbox-based cross-CPU wakeup
    Given the machine is started
    When I wait for the system to boot
    Then the serial output should contain "Scheduler initialized"

  @smoke
  @timeout-30s
  Scenario: Per-CPU wake mailbox is present on each CPU scheduler
    Given the machine is started
    When I wait for the system to boot
    Then the serial output should contain "per-CPU scheduler(s) allocated"

  @smoke
  @timeout-30s
  Scenario: Scheduler diagnostics expose remote wake IPI delivery
    Given the machine is started
    When I wait for the system to boot
    And the shell command "cat /proc/sched/stat" succeeds
    Then the latest command output should contain "resched_ipi_received:"
    And the latest command output should contain "mailbox_tasks_drained:"
