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

  @smoke
  @timeout-30s
  Scenario: Debug-only messages are suppressed at default INFO log level
    # Runs at the default BDD_LOGLEVEL=3 (INFO).  VFS mount detail lines are
    # kdebug! and must not appear in the INFO serial stream.
    Given the machine is started
    When I wait for the system to boot
    Then the serial output should contain "Entering scheduler loop."
    And the latest serial output should not contain "vfs: mounted devfs at /dev"
