Feature: Low-level keyboard hotkeys

  @smoke
  @timeout-180s
  Scenario: Ctrl Alt Delete reboots from the bran keyboard trap
    Given the machine is started
    When I wait for the system to boot
    And I press ctrl+alt+delete
    Then the serial output should contain "PS/2 hotkey Ctrl+Alt+Del detected; forcing immediate reboot"
    And the machine should reboot
