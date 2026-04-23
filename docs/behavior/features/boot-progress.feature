Feature: Boot progress milestone text reporting

  The kernel emits a structured INFO log line for every boot milestone so that
  the current stage is always visible on the serial console and can be
  verified programmatically by automated tests.

  @smoke
  @timeout-30s
  Scenario: Boot milestones are emitted to the serial console
    Given the machine is started
    When I wait for the system to boot
    Then the serial output should contain "boot_progress: milestone="

  @smoke
  @timeout-30s
  Scenario: Memory milestone is reported before VFS milestone
    Given the machine is started
    When I wait for the system to boot
    Then the serial output should contain "Memory Map OK"
    And the serial log shows "VFS Root Ready" after "Memory Map OK"

  @smoke
  @timeout-30s
  Scenario: Final boot milestone is reported before entering the scheduler loop
    Given the machine is started
    When I wait for the system to boot
    Then the serial log shows "Entering Scheduler" after "Spawning Sprout"
    And the serial log shows "Entering scheduler loop." after "Entering Scheduler"

  @smoke
  @timeout-30s
  Scenario: Terminal hint is reported during initialization
    Given the machine is started
    When I wait for the system to boot
    Then the serial output should contain "Press F12 for a terminal"
