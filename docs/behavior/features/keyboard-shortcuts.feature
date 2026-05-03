@timeout=30s
Feature: Keyboard Shortcut Relocation

  Scenario: F1 cycles log levels in the kernel
    Given the machine is booted
    When I wait for the shell prompt
    And I press f1
    Then the serial output should contain "PS/2 hotkey F1 detected; cycling log level" within 10s

  Scenario: F11 toggles fullscreen in Bloom
    Given the machine is booted
    When I wait for the shell prompt
    And I wait for the serial output to contain "bloom: registered bristle pointer sink"
    And I wait for 2 seconds
    And I click at 500, 500
    And I press f11
    Then the serial output should contain "BLOOM_FULLSCREEN_TOGGLE_TRIGGERED" within 10s

  Scenario: Alt+F no longer toggles fullscreen in Bloom
    Given the machine is booted
    When I wait for the shell prompt
    And I wait for the serial output to contain "bloom: registered bristle pointer sink"
    And I wait for 2 seconds
    And I click at 500, 500
    And I press alt+f
    Then the serial output should NOT contain "BLOOM_FULLSCREEN_TOGGLE_TRIGGERED" within 5s

  Scenario: Super+R opens the Run dialog
    Given the machine is booted
    When I wait for the shell prompt
    And I wait for the serial output to contain "bloom: registered bristle pointer sink"
    And I press super+r
    Then the serial output should contain "Run dialog. Input field focused" within 10s

  Scenario: Super+L opens the application launcher
    Given the machine is booted
    When I wait for the shell prompt
    And I wait for the serial output to contain "bloom: registered bristle pointer sink"
    And I press super+l
    Then the serial output should contain "Application launcher opened" within 10s
    And the serial output should contain "Application launcher overlay ready" within 10s
