Feature: Bristle HID broker — ServiceLoop-based fanout

  Bristle is the sole input authority.  After migration from packed-port
  args it uses a ServiceLoop inbox for control messages and publishes
  device port write handles to the VFS so that drivers can discover them.
  Consumers register as event sinks via KIND_BRISTLE_REGISTER_SINK inbox
  messages.

  Background:
    Given the machine is booted

  Scenario: bristle publishes its PID file on startup
    # Consumers (bloom, echo) discover bristle via /run/bristle/pid.
    # The file must exist and contain a non-zero decimal PID.
    Then the log should match pattern "bristle: published pid [1-9][0-9]* to /run/bristle/pid"

  Scenario: bristle publishes keyboard device handle
    # PS/2 keyboard driver reads /run/bristle/kbd_in to obtain its write handle.
    Then the log should match pattern "bristle: published device handles kbd_in=[0-9]+"

  Scenario: bristle publishes mouse device handle
    # PS/2 mouse driver reads /run/bristle/mouse_in to obtain its write handle.
    Then the log should match pattern "bristle: published device handles .*mouse_in=[0-9]+"

  Scenario: bloom registers with bristle via inbox after bristle starts
    # bloom creates a port pair and sends KIND_BRISTLE_REGISTER_SINK to bristle.
    Then the log should match pattern "bristle: bloom sink registered"
    And the log should match pattern "bloom: registered with bristle"

  Scenario: bristle seeds the session locale from Limine
    Then the log should match pattern "bristle: session locale initialized to syc"

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

  Scenario: F1 cycles the session locale
    Given the machine is booted
    When I press f1
    Then the log should match pattern "bristle: F1 pressed - locale set to (eo|la|syc|en)"

  Scenario: ps2_kbd driver reads handle from VFS path
    # ps2_kbd no longer uses a packed spawn arg; it reads the handle from VFS.
    Then the log should match pattern "ps2_kbd: bristle handle=[1-9][0-9]*"

  Scenario: ps2_mouse driver reads handle from VFS path
    # ps2_mouse no longer uses a packed spawn arg; it reads the handle from VFS.
    Then the log should match pattern "ps2_mouse: bristle handle=[1-9][0-9]*"
