Feature: Petals description style layout loop

  Petals declares UI structure, Stile resolves description-based rules, and
  Taffy computes geometry before any Pistil rendering is required.

  Scenario: Petals demo computes a styled layout tree
    Given the machine is booted
    When I wait for the shell prompt
    And I type "petals_demo" on the serial console
    Then the serial output should contain "petals_demo: layout tree" within 60s
    And the serial output should contain "petals_demo: node=1" within 60s
    And the serial output should contain "box=(0, 0) 120x40" within 60s
    And the serial output should contain "petals_demo: clock petal time= 9:41 PM date=May 2, 2026" within 60s
    And the serial output should contain "petals_demo: clock root box=" within 60s
    And the serial output should contain "petals_demo: clock petal PASS" within 60s
    And the serial output should contain "petals_demo: calc expression=12 + 7 * 3 result=33" within 60s
    And the serial output should contain "Three pressed. Expression: 12 + 7 * 3. Result: 33" within 60s
    And the serial output should contain "petals_demo: calc first key box=" within 60s
    And the serial output should contain "petals_demo: calc petal PASS" within 60s
    And the serial output should contain "petals_demo: PASS" within 60s
