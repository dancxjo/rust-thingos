Feature: Audio Subsystem
  The audio subsystem probes for early-boot audio without blocking the rest of
  supervisor bring-up.

  @timeout.30s
  Scenario: Early boot audio probe does not block supervisor bring-up
    Given the machine is started
    When I wait for the serial output to contain "SPROUT: Starting early audio stack"
    Then the serial output should contain "SPROUT: Audio stack worker running"
    And the serial output should contain "SPROUT: Early audio device"
    And the serial output should contain "SPROUT: Entering supervisor service loop"

  @timeout.30s
  Scenario: Audio drivers are available for hardware-backed boots
    Given the machine is started
    When I wait for the serial output to contain "DEVD CATALOG: registered driver '/drivers/virtio_sound'"
    Then the serial output should contain "DEVD CATALOG: registered driver '/drivers/chime'"
    And the serial output should contain "DEVD CATALOG: registered driver '/drivers/hdaudio'"
    And the serial output should not contain "KERNEL PAGE FAULT"

  @timeout.30s
  Scenario: Startup chime opens the audio output through the audio provider
    Given the machine is started
    When I wait for the serial output to contain "chime: Opened /dev/audio/card0/out0"
    Then the serial output should contain "SPROUT: Audio stack launched chime"
    And the serial output should not contain "DISP: vfs_lookup path='out0'"
    And the serial output should not contain "KERNEL PAGE FAULT"
