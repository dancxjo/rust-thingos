Feature: Signal Handling
  The kernel supports POSIX-style signals for asynchronous task notification.

  Scenario: Default signal action terminates task
    Given a task is running
    When I send signal SIGTERM to the task
    Then the task should exit with status corresponding to SIGTERM

  Scenario: Signal masking prevents delivery
    Given a task has masked SIGUSR1
    When I send signal SIGUSR1 to the task
    Then the task should remain running
    And the signal should be marked as pending in "/proc/self/status"
