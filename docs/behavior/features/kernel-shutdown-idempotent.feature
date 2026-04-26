Feature: Kernel shutdown is idempotent and rejects new spawns during teardown
  # Regression spec for the "17-second delay before /bin/shutdown main() executes" issue.
  #
  # Root cause: a second /bin/shutdown process was spawned while a first was already
  # executing stem::shutdown().  The kernel had no single-owner shutdown state, so
  # two processes raced through teardown simultaneously, causing scheduler starvation
  # and a ~17-second delay.
  #
  # Fix: introduce an atomic kernel shutdown state machine (Running → ShutdownRequested).
  # The first caller to SYS_REBOOT wins and performs teardown; every subsequent caller
  # exits cleanly without re-entering the teardown path.  SYS_SPAWN_PROCESS_EX is
  # rejected with EBUSY once shutdown has been initiated.

  Background:
    Given the machine is booted

  Scenario: Duplicate shutdown call exits cleanly without 17-second delay
    # Regression test: spawning /bin/shutdown a second time while teardown is
    # already in progress must not stall the system for ~17 seconds.
    Given the system is running normally
    When /bin/shutdown is invoked
    Then the kernel log should contain "SYSCALL SHUTDOWN: system shutdown requested"
    And the kernel log should contain "Shutting down..."
    And QEMU exits within 5 seconds

  Scenario: Second shutdown syscall during teardown logs a duplicate warning
    # The kernel must log that a duplicate caller was detected and exited.
    Given the system is running normally
    When /bin/shutdown is invoked twice concurrently
    Then the kernel log should contain "shutdown already in progress, duplicate caller exiting"

  Scenario: Spawning a new process during shutdown is rejected
    # Once SYS_REBOOT has been accepted, SYS_SPAWN_PROCESS_EX must return EBUSY
    # so that the shell cannot create new processes that race with teardown.
    Given the system is running normally
    When /bin/shutdown is invoked
    And a process spawn is attempted after shutdown begins
    Then the spawn should fail with EBUSY
