Feature: ServiceLoop — inbox-backed control plane for ThingOS services
  # Executable spec for stem::service_loop::ServiceLoop.
  #
  # Implements the contract from docs/ipc/service_loop.md and the fifth
  # guardrail in docs/concepts/thingos-guardrails.md:
  #
  #   "A ThingOS service is an inbox-backed actor.  It may additionally wait
  #    on FDs, IRQs, child exits, or provider ports, but its control plane
  #    is its inbox."
  #
  # These scenarios drive a small in-tree harness (added when the first
  # consumer migrates — virtio_gpu in the follow-up PR) that constructs a
  # ServiceLoop, registers secondary sources, and prints structured event
  # tags to the serial console for the BDD runner to assert against.

  Background:
    Given the machine is booted
    And the serviceloop test harness is available

  Scenario: inbox messages are dispatched as control-plane events
    When the harness creates a ServiceLoop with max_payload 4096
    And another task sends a typed message with kind "test.ping" to the harness inbox
    Then the harness should observe a "Message" event with kind "test.ping" within 5s
    And the harness should not have observed any "Ready" event before the message

  Scenario: finite timeout waits remain event-driven
    # A ServiceLoop with a periodic timeout must stay registered in WaitSet
    # while it waits.  Inbox delivery should wake the loop as an event, not
    # wait for a userspace poll/sleep timeout cycle.
    When the harness creates a ServiceLoop with max_payload 4096
    And another task sends a typed message with kind "test.timeout-wake" to the harness inbox
    Then the harness should observe a "Message" event with kind "test.timeout-wake" within 5s

  Scenario: when the inbox and a pipe both fire, the inbox wins
    # Resolves the design question in docs/ipc/service_loop.md §3.1:
    # control-plane priority is part of the contract, not an implementation
    # accident.
    When the harness creates a ServiceLoop with max_payload 4096
    And the harness registers a pipe read-end as a secondary FD
    And the pipe write-end is filled with 8 bytes of data
    And another task sends a typed message with kind "test.priority" to the harness inbox
    Then the next ServiceLoop event observed by the harness should be a "Message" event with kind "test.priority"
    And the subsequent ServiceLoop event observed by the harness should be a "Ready" event for the pipe token

  Scenario: secondary FD readiness still dispatches when the inbox is idle
    When the harness creates a ServiceLoop with max_payload 4096
    And the harness registers a pipe read-end as a secondary FD
    And the pipe write-end is filled with 8 bytes of data
    Then the harness should observe a "Ready" event for the pipe token within 5s

  Scenario: an IRQ-registered service wakes from add_irq
    # Mirrors the existing WaitSet IRQ test, but exercises the ServiceLoop
    # wrapper so the dispatch invariant (inbox-first) is also tested under
    # IRQ wakes.
    When the harness creates a ServiceLoop with max_payload 4096
    And the harness subscribes to a synthetic test IRQ and registers it as a secondary source
    And the synthetic test IRQ is fired
    Then the harness should observe a "Ready" event with the IRQ flag set within 5s

  Scenario: closing the inbox surfaces a clean shutdown event
    # Critical: the loop must NOT spin on EOF.  InboxClosed is latched, so
    # callers can break their loop on a single match arm.
    When the harness creates a ServiceLoop with max_payload 4096
    And the harness's inbox is closed by the supervisor
    Then the harness should observe an "InboxClosed" event within 5s
    And subsequent calls to next_event should keep returning "InboxClosed"
    And the harness should not consume more than 1% CPU after the inbox closes

  Scenario: drain_inbox batches multiple messages in one wake
    # The default contract is one message per next_event; drain_inbox is the
    # explicit batch escape hatch documented in docs/ipc/service_loop.md.
    When the harness creates a ServiceLoop with max_payload 4096
    And another task sends 4 typed messages with kind "test.batch" to the harness inbox
    And the harness calls next_event followed by drain_inbox
    Then the harness should report 4 messages observed in total
    And exactly 1 of those messages should have come from next_event
    And exactly 3 of those messages should have come from drain_inbox

  Scenario: removing a secondary token does not detach the inbox
    # The inbox is structural; remove(inbox_token) must be a no-op.
    When the harness creates a ServiceLoop with max_payload 4096
    And the harness registers a pipe read-end as a secondary FD
    And the harness attempts to remove the inbox token
    Then the remove call should report "false"
    And the ServiceLoop should still have the inbox registered

  Scenario: run_until_shutdown calls the shutdown hook on inbox close
    # Verifies the canonical one-shot shutdown pattern.  The hook must be
    # called exactly once, and the helper must return cleanly so the caller
    # can proceed to exit(0).
    When the harness creates a ServiceLoop with max_payload 4096
    And the harness arms a run_until_shutdown hook that records "shutdown-called"
    And the harness's inbox is closed by the supervisor
    Then the harness should observe "shutdown-called" in the hook output within 5s
    And run_until_shutdown should have returned to the caller

  Scenario: run_until_shutdown calls the shutdown hook on handler Break
    When the harness creates a ServiceLoop with max_payload 4096
    And the harness arms a run_until_shutdown hook that records "shutdown-called"
    And the handler returns Break on the next Message event
    And another task sends a typed message with kind "test.break" to the harness inbox
    Then the harness should observe "shutdown-called" in the hook output within 5s
    And run_until_shutdown should have returned to the caller
