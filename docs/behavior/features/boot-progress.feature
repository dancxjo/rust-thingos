Feature: Boot progress milestone text reporting

  The kernel emits a structured INFO log line for every boot milestone so that
  the current stage is always visible on the serial console and can be
  verified programmatically by automated tests.

  @smoke
  @timeout.30s
  Scenario: Boot milestones are emitted to the serial console
    Given the machine is started
    When I wait for the system to boot
    Then the serial output should contain "boot_progress: milestone="

  @smoke
  @timeout.30s
  Scenario: Memory milestone is reported before VFS milestone
    Given the machine is started
    When I wait for the system to boot
    Then the serial log shows "boot_progress: milestone=\"Memory Map OK\"" after "kernel:start"
    And  the serial log shows "boot_progress: milestone=\"VFS Root Ready\"" after "boot_progress: milestone=\"Memory Map OK\""

  @smoke
  @timeout.30s
  Scenario: Final boot milestone is reported before entering the scheduler loop
    Given the machine is started
    When I wait for the system to boot
    Then the serial log shows "boot_progress: milestone=\"Entering Scheduler\"" after "boot_progress: milestone=\"Spawning Sprout\""
    And  the serial log shows "Entering scheduler loop." after "boot_progress: milestone=\"Entering Scheduler\""
