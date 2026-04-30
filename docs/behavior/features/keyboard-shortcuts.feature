Feature: Keyboard Shortcut Relocation
 
  Scenario: F1 cycles log levels in the kernel
    Given the machine is booted
    When I wait for the shell prompt
    And I press f1
    Then the serial output should contain "F1 hotkey: log level set to 4 (Debug)" within 10s
 
  Scenario: F11 toggles fullscreen in Bloom
    Given the machine is booted
    When I wait for the shell prompt
    # Bloom should be focused by default.
    And I press f11
    Then the serial output should contain "BLOOM_FULLSCREEN_TOGGLE_TRIGGERED" within 10s
 
  Scenario: Alt+F no longer toggles fullscreen in Bloom
    Given the machine is booted
    When I wait for the shell prompt
    And I press alt+f
    Then the serial output should not contain "BLOOM_FULLSCREEN_TOGGLE_TRIGGERED" within 5s
