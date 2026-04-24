Feature: Terminal parser and renderer threading

  The terminal splits ANSI parsing / input processing and framebuffer
  blitting into separate kernel-scheduled threads so that heavy glyph
  rendering cannot block further input processing.

  Scenario: Terminal renderer thread starts independently of the parser
    Given the machine is booted
    Then the log should match pattern "Terminal: Renderer thread spawned"

  Scenario: Terminal parser loop remains alive after renderer is started
    Given the machine is booted
    Then the log should match pattern "Terminal: Renderer thread spawned"
    And the log should match pattern "Terminal: Liveness check"

  Scenario: Terminal parser emits liveness signals under output load
    # The parser thread must continue writing liveness dots and status lines
    # even while the renderer is busy blitting dirty regions.  At least two
    # liveness check log lines must appear after the renderer has started.
    Given the machine is booted
    Then the log should match pattern "Terminal: Renderer thread spawned"
    And "Terminal: Liveness check" should appear at least 2 times

  Scenario: Terminal input processing remains responsive during rendering
    # The parser thread signals the renderer via a commit flag and immediately
    # continues processing.  This verifies the decoupled design: the log line
    # emitted by the parser appears independently of render completion.
    Given the machine is booted
    Then the log should match pattern "Terminal: Renderer thread spawned"
    And the log should match pattern "Terminal: Liveness check - frame 0"
    And the log should match pattern "Terminal: Liveness check - frame 60"
