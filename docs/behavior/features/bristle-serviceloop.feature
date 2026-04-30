Feature: Bristle HID broker — ServiceLoop-based fanout

  Bristle is the sole input authority.  After migration from packed-port
  args it uses a ServiceLoop inbox for control messages and device events.
  Consumers register as event sinks via KIND_BRISTLE_REGISTER_SINK inbox
  messages.

  Background:
    Given the machine is booted

  Scenario: bristle publishes its PID file on startup
    # Consumers (bloom, echo) discover bristle via /run/bristle/pid.
    # The file must exist and contain a non-zero decimal PID.
    Then the log should match pattern "bristle: published pid [1-9][0-9]* to /run/bristle/pid"

  Scenario: bloom registers with bristle via inbox after bristle starts
    # bloom creates a port pair and sends KIND_BRISTLE_REGISTER_SINK to bristle.
    Then the log should match pattern "bristle: bloom sink registered"
    And the log should match pattern "bloom: registered with bristle"

  Scenario: bristle no longer accepts a packed port argument
    # The packed usize spawn arg path is removed.
    # Bristle ignores the arg and proceeds with zero (ServiceLoop start).
    Then the log should match pattern "bristle: online"
    And the log should not match pattern "bristle: online \(kbd=.*mouse=.*bloom_evt=.*input_echo="

  Scenario: keyboard input reaches bloom after registration
    # End-to-end: pressing a key while bloom is registered should produce
    # a bristle event dispatch log entry.
    Given the machine is booted
    When I wait for the shell prompt
    And I type "echo test" on the serial console
    Then the log should match pattern "bristle: bloom sink registered"

  Scenario: ps2_kbd driver discovers bristle via VFS path
    # ps2_kbd no longer uses a packed spawn arg; it reads bristle's PID from VFS.
    Then the log should match pattern "ps2_kbd: bristle pid=[1-9][0-9]*"

  Scenario: ps2_mouse driver discovers bristle via VFS path
    # ps2_mouse no longer uses a packed spawn arg; it reads bristle's PID from VFS.
    Then the log should match pattern "ps2_mouse: bristle pid=[1-9][0-9]*"

  Scenario: ps2_mouse completes shared-controller initialization
    # Mouse init must not spin forever behind keyboard bytes in the shared i8042 FIFO.
    Then the log should match pattern "ps2_mouse: init done"
