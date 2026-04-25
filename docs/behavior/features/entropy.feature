Feature: Entropy and Random Number Generation
  The kernel provides a secure random number source via hardware RNG (RDRAND/RDSEED)
  or a timer-based entropy fallback.

  Scenario: Hardware entropy is detected
    Given the machine is started
    When the entropy subsystem initializes
    Then the serial output should contain "ENTROPY: Using hardware RNG"

  Scenario: Timer fallback is used when hardware RNG is missing
    Given a machine without RDRAND support
    When the entropy subsystem initializes
    Then the serial output should contain "ENTROPY: no hardware RNG available, using timer fallback"
