Feature: Scheduler entry latency instrumentation

  @smoke
  Scenario: Scheduler loop entry is reached before deferred boot framebuffer paint
    Given the machine is started
    When I wait for the system to boot
    Then the serial output should contain "scheduler-entry total elapsed_ticks="
    And I should see "Entering scheduler loop." after "Scheduler initialized"
    And I should see "deferred_bootfb_gradient elapsed_ticks=" after "Entering scheduler loop."
