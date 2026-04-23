Feature: Bloom compositor threading and responsiveness

  The bloom compositor separates I/O event handling from frame rendering
  using a dedicated I/O thread.  Wallpaper changes are decoded in a
  background worker thread so the render loop is never blocked by image
  decoding.

  Scenario: bloom compositor service starts and publishes its port
    Given the machine is booted
    Then the log should match pattern "bloom: compositor service starting"
    And the log should match pattern "bloom: output0 [0-9]+x[0-9]+"

  Scenario: bloom compositor remains observable after startup
    Given the machine is booted
    Then the log should match pattern "bloom: output0 [0-9]+x[0-9]+"
    And the log should match pattern "bloom: creating service port"

  Scenario: bloom compositor reacts to wallpaper watch path
    Given the machine is booted
    When I wait for the shell prompt
    And I type "echo /share/wallpapers/flower.bmp > /session/desktop/wallpaper" on the serial console
    Then the log should match pattern "bloom: reacting to wallpaper change"

  Scenario: bloom I/O thread starts independently of the render loop
    Given the machine is booted
    Then the log should match pattern "bloom: I/O thread started"

  Scenario: bloom input ingestion continues under render pressure
    # Verifies that the I/O thread queues events independently of the render
    # loop: two rapid wallpaper-change writes are both ingested and reacted to
    # even if the render thread is busy presenting a frame.
    Given the machine is booted
    When I wait for the shell prompt
    And I type "echo /share/wallpapers/flower.bmp > /session/desktop/wallpaper" on the serial console
    And I type "echo /share/wallpapers/flower.bmp > /session/desktop/wallpaper" on the serial console
    Then the log should match pattern "bloom: I/O thread started"
    And "bloom: reacting to wallpaper change" should appear at least 2 times
