Feature: Audio Subsystem
  The audio subsystem provides early-boot diagnostic chimes, beeper support,
  and full PCM audio via VirtIO Sound.

  @timeout.30s
  Scenario: Early boot audio starts without blocking supervisor bring-up
    Given the machine is started
    When I wait for the serial output to contain "SPROUT: Starting early audio stack"
    Then the serial output should contain "SPROUT: Audio stack worker running"
    And the serial output should contain "SPROUT: Audio stack launched chime"
    And the serial log shows "SPROUT: Continuing supervisor startup" after "SPROUT: Starting early audio stack"

  @timeout.30s
  Scenario: VirtIO Sound driver initializes successfully
    Given the machine is started
    When I wait for the serial output to contain "SPROUT: Early audio device"
    Then the serial output should contain "SPROUT: Early audio device"
    And the serial output should contain "chime: Opened /dev/audio/card0/out0"
    And the serial output should not contain "KERNEL PAGE FAULT"

  @timeout.30s
  Scenario: Startup chime uses the VFS audio stream
    Given the machine is started
    When I wait for the serial output to contain "chime: Opened /dev/audio/card0/out0"
    Then the serial output should contain "SND: Mounted at /dev/audio/card0"
    And the serial output should contain "SPROUT: Startup chime task completed"
