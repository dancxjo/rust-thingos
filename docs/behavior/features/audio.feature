Feature: Audio Subsystem
  The audio subsystem provides early-boot diagnostic chimes, beeper support,
  and full PCM audio via VirtIO Sound.

  Scenario: Early boot chime is emitted
    Given the machine is started
    When I wait for the system to initialize audio
    Then the serial output should contain "AUDIO: Emitting boot chime"
    And the serial output should contain "CHIME: A-Major chord generated"

  Scenario: VirtIO Sound driver initializes successfully
    Given the machine is started
    When the PCI bus is scanned
    Then the serial output should contain "VIRTIO_SOUND: Found VirtIO sound device"
    And the serial output should contain "VIRTIO_SOUND: Mounted at /dev/audio/pcm0"

  Scenario: Beeper service is available
    Given the machine is booted
    Then the path "/dev/beep" should exist
    When I write "440,100" to "/dev/beep"
    Then the serial output should contain "BEEP: 440Hz for 100ms"
