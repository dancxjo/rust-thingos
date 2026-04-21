Feature: VFS Extended Attributes
  Verify that VFS providers can support extended attributes (xattrs)
  via the Attr device call protocol.

  Scenario: Round trip with mutable attributes
    Given the machine is booted
    When I wait for the shell prompt
    And I type "ls /bin" on the serial console
    And I wait for 2 seconds
    And I type "ipc_provider_demo &" on the serial console
    And I wait for 2 seconds
    And I type "attr_list /run/cookbook/hello.txt" on the serial console
    Then the serial output should contain "provider.name"
    And the serial output should contain "provider.is_demo"
    And the serial output should contain "provider.requests"
    When I type "attr_set /run/cookbook/hello.txt user.comment utf8 This_is_a_test" on the serial console
    And I wait for 1 seconds
    And I type "attr_get /run/cookbook/hello.txt user.comment" on the serial console
    Then the serial output should contain "type=Utf8 len=14"
    And the serial output should contain "value=This_is_a_test"
    When I type "attr_list /run/cookbook/hello.txt" on the serial console
    Then the serial output should contain "user.comment"
    When I type "attr_rm /run/cookbook/hello.txt user.comment" on the serial console
    And I wait for 1 seconds
    And I type "attr_list /run/cookbook/hello.txt" on the serial console
    Then the latest serial output should not contain "user.comment"
