Feature: Alt+Tab window cycling
  
  Scenario: Alt+Tab cycles through multiple windows
    Given the machine is booted
    When I wait for the shell prompt
    And I type "clock &" on the serial console
    And I wait for 2 seconds
    And I type "clock &" on the serial console
    And I wait for 2 seconds
    And I type "clock &" on the serial console
    And I wait for 5 seconds
    Then the serial output should contain "First frame rendered" within 60s
    And the serial output should contain "bloom: registered bristle pointer sink" within 60s
    
    # Cycle 1: First focus
    When I press alt+tab
    Then the latest output should contain "bloom: focus cycled from None" within 10s
    
    # Cycle 2: Move to next
    When I press alt+tab
    Then the latest output should contain "bloom: focus cycled from Some" within 10s
    
    # Backward Cycle: Move back
    When I press shift+alt+tab
    Then the latest output should contain "bloom: focus cycled from Some" within 10s
    And the output contains "forward=false"
