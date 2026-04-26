Feature: Sprout graphics pipeline orchestration

  Sprout is responsible for bringing up the graphics and input stack during
  early boot. This feature verifies that sprout correctly launches bloom,
  bristle, and the necessary drivers, and that they reach a functional state.

  Scenario: sprout launches the full graphics pipeline
    Given the machine is booted
    Then the serial output should contain "SPROUT: Starting full pipeline (graphics + input)" within 60s
    And the serial output should contain "SPROUT: Spawned bristle" within 60s
    And the serial output should contain "SPROUT: Spawned bloom" within 120s
    And the serial output should contain "bloom: compositor service starting" within 120s
    And the serial output should contain "bloom: service loop started" within 180s
    And the serial output should contain "First frame rendered" within 180s
