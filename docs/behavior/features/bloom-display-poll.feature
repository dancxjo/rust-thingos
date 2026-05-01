Feature: Bloom display metadata polling

  Bloom polls display metadata to detect host-driven output changes. That poll
  uses synchronous display DeviceCall RPC, so it must remain rate-limited while
  pointer and client events are flowing through the compositor loop.

  Scenario: pointer motion does not poll display metadata per event
    Given the machine is booted
    Then the serial output should contain "bloom: registered bristle pointer sink" within 60s
    And the serial output should contain "ps2_mouse: bristle pid=" within 60s
    And the serial output should contain "ps2_mouse: sample rate set to" within 60s
    When I wait for the shell prompt
    And I type "loglevel 5" on the serial console
    Then the latest serial output should contain "Log level set to 5"
    When I wait for 1 seconds
    And I type "echo poll-baseline" on the serial console
    And I move the mouse 20 times
    And I wait for 1 seconds
    Then the latest serial output should contain "bloom: pointer moved"
    And the latest serial output should contain "display_virtio_gpu: rpc enter seq="
    And the latest serial output should contain "display_virtio_gpu: rpc exit seq="
    And the latest serial output should contain at most 8 occurrences of "DISP: DISPLAY_OP_GET_INFO requested"
    And the latest serial output should contain at most 12 occurrences of "VFS: sys_fs_open path='/session/wayland/"

  Scenario: desktop display watchdog reports runtime state
    Given the machine is booted
    Then the serial output should contain "First frame rendered" within 60s
    And the serial output should contain "display_virtio_gpu: watchdog rpc_enter=" within 90s
