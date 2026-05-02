Feature: Debug-level log output during boot

  # Policy enforced by these scenarios:
  #   INFO  – end-user visible; only obvious, high-level events.
  #   DEBUG – developer diagnostics; helpful but not called on every tick.
  #   TRACE – high-frequency; expected to flood at trace level.
  #
  # The @debug-level scenarios MUST be run with BDD_LOGLEVEL=4 (debug):
  #
  #   BDD_LOGLEVEL=4 cargo xtask bdd
  #
  # The @smoke scenarios run at the default INFO level (BDD_LOGLEVEL=3)
  # and confirm that debug-only messages are correctly suppressed.

  @debug-level
  @timeout-30s
  Scenario: VFS subsystem emits mount debug messages during boot
    Given the machine is started
    When I wait for the system to boot
    Then the serial output should contain "vfs: mounted devfs at /dev"
    And the serial output should contain "vfs: mounted tmpfs at /tmp"
    And the serial output should contain "vfs: mounted tmpfs at /run"

  @debug-level
  @timeout-30s
  Scenario: Entropy seeding is reported at debug level
    Given the machine is started
    When I wait for the system to boot
    Then the serial output should contain "ENTROPY:"

  @debug-level
  @timeout-30s
  Scenario: Scheduler task assignment is reported at debug level
    Given the machine is started
    When I wait for the system to boot
    Then the serial output should contain "SCHED:"

  @debug-level
  @timeout-30s
  Scenario: Kernel subsystem initialisation steps are visible at debug level
    Given the machine is started
    When I wait for the system to boot
    Then the serial output should contain "Initializing VFS..."
    And the serial output should contain "Seeding entropy pool..."

  @debug-level
  @timeout-30s
  Scenario: Spawn process syscall completion is reported at debug level
    # SYSCALL SPAWN_PROCESS_EX completion is kdebug! — it fires on every
    # process launch and must not appear in the default INFO stream.
    Given the machine is started
    When I wait for the system to boot
    Then the serial output should contain "SYSCALL SPAWN_PROCESS_EX:"

  @smoke
  @timeout-30s
  Scenario: Debug-only messages are suppressed at default INFO log level
    # Runs at the default BDD_LOGLEVEL=3 (INFO).  VFS mount detail lines are
    # kdebug! and must not appear in the INFO serial stream.
    Given the machine is started
    When I wait for the system to boot
    Then the serial output should contain "Entering scheduler loop."
    And the latest serial output should not contain "vfs: mounted devfs at /dev"

  @smoke
  @timeout-30s
  Scenario: Spawn process syscall messages are suppressed at INFO log level
    # SYSCALL SPAWN_PROCESS_EX is kdebug! and must not pollute the INFO stream,
    # reducing spawn-path serial latency at default log level.
    Given the machine is started
    When I wait for the system to boot
    Then the latest serial output should not contain "SYSCALL SPAWN_PROCESS_EX:"

  @smoke
  @kmsg
  @timeout-30s
  Scenario: Kernel message ring remains readable after boot
    Given the machine is started
    When I wait for the system to boot
    And the shell command "cat /dev/kmsg" succeeds
    Then the latest serial output should contain "Entering scheduler loop."

  @smoke
  @timeout-30s
  Scenario: F11 cycles the kernel log level from the low-level keyboard path
    Given the machine is started
    When I wait for the system to boot
    And I press f11
    Then the serial output should contain "F11 hotkey: log level set to 4 (DEBUG)"
