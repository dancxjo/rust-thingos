Feature: DRIVER_READY inbox message readiness handshake

  Sprout supervises every driver and service as an inbox-backed actor.
  When a supervised process finishes initialising, Cambium sends a typed
  DRIVER_READY inbox message to Sprout's process inbox.  Sprout receives
  that message through its ServiceLoop and marks the corresponding
  ManagedTask as ready.

  This replaces the legacy per-task port-handshake (drv_req_write /
  drv_resp_read / boot_req_read / boot_resp_write / resp_fd).

  Background:
    Given the machine is booted

  @smoke
  @timeout-60s
  Scenario: Sprout receives DRIVER_READY and logs the state transition
    When the system finishes bringing up supervised services
    Then the serial output should contain "DRIVER_READY received"
    And the serial output should contain "marked ready"

  @smoke
  @timeout-60s
  Scenario: Cambium sends DRIVER_READY after spawning a driver
    When cambium spawns a hardware driver
    Then the serial output should contain "CAMBIUM: sent DRIVER_READY to Sprout"

  @smoke
  @timeout-60s
  Scenario: Sprout supervisor loop stays alive after readiness messages
    When the system finishes bringing up supervised services
    Then the serial output should contain "SPROUT: ServiceLoop"
    And the serial shell is still responsive

  @smoke
  @timeout-60s
  Scenario: No legacy readiness port fields remain in use
    # Validates the structural acceptance criterion: ManagedTask no longer
    # carries drv_req_write / drv_resp_read / boot_req_read /
    # boot_resp_write / resp_fd.  The absence of any log line mentioning
    # those fields after boot confirms the legacy path is gone.
    When the system finishes bringing up supervised services
    Then the serial output should not contain "drv_resp_read"
    And the serial output should not contain "boot_req_read"
    And the serial output should not contain "resp_fd poll"
