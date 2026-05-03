Feature: Audio Subsystem
  The audio subsystem starts the boot chime without blocking the rest of
  supervisor bring-up.

  @timeout.30s
  Scenario: Startup chime waits for audio without blocking supervisor bring-up
    Given the machine is started
    When I wait for the serial output to contain "SPROUT: spawned startup chime"
    Then the serial output should contain "chime: Waiting for /dev/audio/card0/out0"
    And the serial output should contain "chime: open /dev/audio/card0/out0 attempt 1 failed"
    And the serial output should contain "SPROUT: Continuing supervisor startup"

  @timeout.30s
  Scenario: Audio drivers are available for hardware-backed boots
    Given the machine is started
    When I wait for the serial output to contain "DEVD CATALOG: registered driver '/drivers/virtio_sound'"
    Then the serial output should contain "DEVD CATALOG: registered driver '/drivers/hdaudio'"
    And the serial output should contain "CAMBIUM: audio driver PID"
    And the serial output should contain "priority set to realtime"
    And the serial output should not contain "KERNEL PAGE FAULT"

  @timeout.60s
  Scenario: Startup chime plays once audio appears and exits
    Given the machine is started
    When I wait for the serial output to contain "chime: Opened /dev/audio/card0/out0"
    Then the serial output should contain "SPROUT: spawned startup chime"
    And the serial output should contain "SPROUT: startup chime priority set to realtime"
    And the serial output should contain "chime: Playback started"
    And the serial output should contain "chime: Finished"
    And the serial output should not contain "DISP: vfs_lookup path='out0'"
    And the serial output should not contain "KERNEL PAGE FAULT"
